#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;

#[inline]
fn scalar_limexp(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX * (1.0 + arg - 80.0) }
}

#[inline]
fn scalar_limexp_derivative(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX }
}

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
    v2: f64,
    v4: f64,
    v5: f64,
    v6: f64,
    v7: f64,
    v8: f64,
    v9: f64,
    v10: f64,
    v11: f64,
    v14: f64,
    v15: f64,
    v32: f64,
    v48: f64,
    v60: f64,
    v91: f64,
    v431: f64,
    v444: f64,
    v446: f64,
    v448: f64,
    v452: f64,
    v455: f64,
    v563: f64,
    v569: f64,
    v772: f64,
    v777: bool,
    v779: f64,
    v782: f64,
    v790: f64,
    v797: f64,
    v801: f64,
    v803: bool,
    v805: f64,
    v806: f64,
    v814: bool,
    v826: bool,
    v828: f64,
    v829: f64,
    v843: bool,
    v851: f64,
    v855: f64,
    v906: bool,
    v915: f64,
    v918: f64,
    v927: f64,
    v946: f64,
    v948: f64,
    v953: bool,
    v955: f64,
    v964: f64,
    v971: f64,
    v976: bool,
    v978: f64,
    v979: f64,
    v987: bool,
    v997: bool,
    v999: f64,
    v1000: f64,
    v1014: bool,
    v1022: f64,
    v1026: f64,
    v1077: bool,
    v1085: f64,
    v1088: f64,
    v1097: f64,
    v1116: f64,
    v1118: bool,
    v1119: f64,
    v1123: f64,
    v1128: f64,
    v1133: bool,
    v1135: f64,
    v1136: f64,
    v1144: bool,
    v1154: bool,
    v1156: f64,
    v1157: f64,
    v1171: bool,
    v1179: f64,
    v1183: f64,
    v1230: bool,
    v1238: f64,
    v1241: f64,
    v1250: f64,
    v1269: bool,
    v1270: f64,
    v1274: f64,
    v1279: f64,
    v1284: bool,
    v1286: f64,
    v1287: f64,
    v1295: bool,
    v1305: bool,
    v1307: f64,
    v1308: f64,
    v1322: bool,
    v1330: f64,
    v1334: f64,
    v1381: bool,
    v1389: f64,
    v1392: f64,
    v1401: f64,
    v1491: f64,
    v1492: bool,
    v1506: f64,
    v1509: f64,
    v1518: f64,
    v1538: f64,
    v1540: bool,
    v1554: f64,
    v1557: f64,
    v1566: f64,
    v1667: f64,
    v1740: f64,
    v1895: f64,
    v1897: f64,
    v1924: f64,
    v1992: bool,
    v1993: bool,
    v1994: f64,
    v1998: f64,
    v2003: f64,
    v2008: bool,
    v2010: f64,
    v2011: f64,
    v2019: bool,
    v2029: bool,
    v2031: f64,
    v2032: f64,
    v2046: bool,
    v2054: f64,
    v2058: f64,
    v2102: bool,
    v2110: f64,
    v2113: f64,
    v2122: f64,
    v2399: f64,
    v2400: f64,
    v2407: f64,
    v2408: f64,
    v2417: f64,
    v2419: f64,
    v2428: f64,
    v2429: f64,
    v2430: f64,
    v2431: f64,
    v2433: f64,
    v2435: f64,
    v2450: f64,
    v2477: f64,
    v2481: f64,
    v2482: f64,
    v2486: f64,
    v2490: f64,
    v2591: f64,
    v2600: f64,
    v2796: f64,
    v2802: f64,
    v2812: f64,
    v2824: f64,
    v2825: f64,
    v2826: f64,
    v2881: f64,
    v2882: f64,
    v2883: f64,
    v2945: f64,
    v2946: f64,
    v2947: f64,
    v2960: f64,
    v2961: f64,
    v2962: f64,
    v3134: f64,
    v3135: f64,
    v3136: f64,
    v3143: f64,
    v3144: f64,
    v3145: f64,
    v3185: f64,
    v3186: f64,
    v3187: f64,
    v3246: f64,
    v3248: f64,
    v3254: f64,
    v3264: f64,
    v3276: f64,
    v3277: f64,
    v3278: f64,
    v3279: f64,
    v3347: f64,
    v3348: f64,
    v3349: f64,
    v3350: f64,
    v3427: f64,
    v3428: f64,
    v3429: f64,
    v3430: f64,
    v3446: f64,
    v3447: f64,
    v3448: f64,
    v3449: f64,
    v3671: f64,
    v3672: f64,
    v3673: f64,
    v3674: f64,
    v3683: f64,
    v3684: f64,
    v3685: f64,
    v3686: f64,
    v3737: f64,
    v3738: f64,
    v3739: f64,
    v3740: f64,
    v3818: f64,
    v3822: f64,
    v3828: f64,
    v3840: f64,
    v3841: f64,
    v3842: f64,
    v3843: f64,
    v3911: f64,
    v3912: f64,
    v3913: f64,
    v3914: f64,
    v3991: f64,
    v3992: f64,
    v3993: f64,
    v3994: f64,
    v4010: f64,
    v4011: f64,
    v4012: f64,
    v4013: f64,
    v4235: f64,
    v4236: f64,
    v4237: f64,
    v4238: f64,
    v4247: f64,
    v4248: f64,
    v4249: f64,
    v4250: f64,
    v4301: f64,
    v4302: f64,
    v4303: f64,
    v4304: f64,
    v4384: f64,
    v4390: f64,
    v4402: f64,
    v4403: f64,
    v4404: f64,
    v4405: f64,
    v4473: f64,
    v4474: f64,
    v4475: f64,
    v4476: f64,
    v4553: f64,
    v4554: f64,
    v4555: f64,
    v4556: f64,
    v4572: f64,
    v4573: f64,
    v4574: f64,
    v4575: f64,
    v4797: f64,
    v4798: f64,
    v4799: f64,
    v4800: f64,
    v4809: f64,
    v4810: f64,
    v4811: f64,
    v4812: f64,
    v4863: f64,
    v4864: f64,
    v4865: f64,
    v4866: f64,
    v5215: f64,
    v5216: f64,
    v5217: f64,
    v5218: f64,
    v5252: f64,
    v5253: f64,
    v5254: f64,
    v5255: f64,
    v5256: f64,
    v5267: f64,
    v5268: f64,
    v5269: f64,
    v5270: f64,
    v5271: f64,
    v5333: f64,
    v5334: f64,
    v5335: f64,
    v5336: f64,
    v5337: f64,
    v5434: f64,
    v5439: f64,
    v5440: f64,
    v5441: f64,
    v5442: f64,
    v5476: f64,
    v5477: f64,
    v5478: f64,
    v5479: f64,
    v5480: f64,
    v5491: f64,
    v5492: f64,
    v5493: f64,
    v5494: f64,
    v5495: f64,
    v5557: f64,
    v5558: f64,
    v5559: f64,
    v5560: f64,
    v5561: f64,
    v5878: f64,
    v5879: f64,
    v5880: f64,
    v5952: f64,
    v6890: f64,
    v6894: f64,
    v6898: f64,
    v6902: f64,
    v6905: f64,
    v6906: f64,
    v6907: f64,
    v6908: f64,
    v6909: f64,
    v6910: f64,
    v7071: f64,
    v7072: f64,
    v7073: f64,
    v7074: f64,
    v7075: f64,
    v7230: f64,
    v7236: f64,
    v7248: f64,
    v7249: f64,
    v7250: f64,
    v7251: f64,
    v7319: f64,
    v7320: f64,
    v7321: f64,
    v7322: f64,
    v7399: f64,
    v7400: f64,
    v7401: f64,
    v7402: f64,
    v7418: f64,
    v7419: f64,
    v7420: f64,
    v7421: f64,
    v7631: f64,
    v7632: f64,
    v7633: f64,
    v7634: f64,
    v7635: f64,
    v7646: f64,
    v7647: f64,
    v7648: f64,
    v7649: f64,
    v7650: f64,
    v7712: f64,
    v7713: f64,
    v7714: f64,
    v7715: f64,
    v7716: f64,
    v8817: f64,
    v8818: f64,
    v8819: f64,
    v8820: f64,
    v8821: f64,
    v8852: f64,
    v8853: f64,
    v8854: f64,
    v8855: f64,
    v8856: f64,
    v8891: f64,
    v8892: f64,
    v8893: f64,
    v8894: f64,
    v8895: f64,
    v8902: f64,
    v8903: f64,
    v8904: f64,
    v8905: f64,
    v8906: f64,
    v8931: f64,
    v8932: f64,
    v8933: f64,
    v8934: f64,
    v8935: f64,
    v8936: f64,
    v8937: f64,
    v8938: f64,
    v8939: f64,
    v8940: f64,
    v8950: f64,
    v8951: f64,
    v8952: f64,
    v8953: f64,
    v8954: f64,
    v8958: f64,
    v8959: f64,
    v8960: f64,
    v8961: f64,
    v8962: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=ctx.node_voltage(nodes[1]);
        let v2=ctx.node_voltage(nodes[5]);
        let v4=(self.scalar_static_f64[0]*(v1-v2));
        let v5=ctx.node_voltage(nodes[6]);
        let v6=(v5-v2);
        let v7=(self.scalar_static_f64[0]*v6);
        let v8=ctx.node_voltage(nodes[7]);
        let v9=(v5-v8);
        let v10=(self.scalar_static_f64[0]*v9);
        let v11=(v10-v7);
        let v14=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[3])-v2));
        let v15=ctx.node_voltage(nodes[2]);
        let v26=1.3806226e-23;
        let v28=1.602176462e-19;
        let v32=0.5;
        let v43=3.0;
        let v48=1.0;
        let v60=0.0;
        let v72=173.14999999999998;
        let v75=600.0;
        let v91=2.0;
        let v107=4.0;
        let v431=ctx.node_voltage(nodes[4]);
        let v433=(if self.scalar_static_bool[14]{(self.scalar_static_f64[277]+v431)}else{self.scalar_static_f64[279]});
        let v434=(v433<v72);
        let v435=(self.scalar_static_bool[14]&&v434);
        let v436=(if v435{v72}else{v433});
        let v440=((v436>v75)&&(self.scalar_static_bool[14]&&(!v434)));
        let v441=(if v440{v75}else{v436});
        let v444=(if self.scalar_static_bool[14]{((v26*v441)/v28)}else{self.scalar_static_f64[281]});
        let v446=(if self.scalar_static_bool[14]{(v48/v444)}else{self.scalar_static_f64[282]});
        let v448=(if self.scalar_static_bool[14]{(v441-self.scalar_static_f64[2])}else{self.scalar_static_f64[283]});
        let v450=(if self.scalar_static_bool[14]{(v441/self.scalar_static_f64[2])}else{self.scalar_static_f64[284]});
        let v452=(if self.scalar_static_bool[14]{(v450).ln()}else{self.scalar_static_f64[285]});
        let v453=(v450-v48);
        let v455=(if self.scalar_static_bool[14]{(v446*v453)}else{self.scalar_static_f64[287]});
        let v465=(v48-v450);
        let v466=(self.scalar_static_f64[10]*v465);
        let v468=(self.scalar_static_f64[20]*v444);
        let v469=(v452*v468);
        let v471=(if self.scalar_static_bool[14]{(((v450*self.scalar_static_f64[150])+v466)-v469)}else{self.scalar_static_f64[448]});
        let v472=(v91*v444);
        let v473=(-v471);
        let v475=((v446*v473)).exp();
        let v478=((v48+(v107*v475))).sqrt();
        let v480=(v32*(v48+v478));
        let v481=(v480).ln();
        let v484=(if self.scalar_static_bool[14]{(v471+(v472*v481))}else{self.scalar_static_f64[306]});
        let v485=(self.scalar_static_f64[36]/v484);
        let v488=((self.scalar_static_f64[46]*(v485).ln())).exp();
        let v490=(if self.scalar_static_bool[14]{(self.scalar_static_f64[30]*v488)}else{self.scalar_static_f64[311]});
        let v493=(if self.scalar_static_bool[14]{((self.scalar_static_f64[47]*v484)/self.scalar_static_f64[36])}else{self.scalar_static_f64[313]});
        let v505=(if self.scalar_static_bool[14]{((v466+(v450*self.scalar_static_f64[158]))-v469)}else{v471});
        let v506=(-v505);
        let v508=((v446*v506)).exp();
        let v511=((v48+(v107*v508))).sqrt();
        let v513=(v32*(v48+v511));
        let v514=(v513).ln();
        let v517=(if self.scalar_static_bool[14]{(v505+(v472*v514))}else{self.scalar_static_f64[327]});
        let v518=(self.scalar_static_f64[48]/v517);
        let v521=((self.scalar_static_f64[57]*(v518).ln())).exp();
        let v523=(if self.scalar_static_bool[14]{(self.scalar_static_f64[30]*v521)}else{self.scalar_static_f64[332]});
        let v526=(if self.scalar_static_bool[14]{((self.scalar_static_f64[58]*v517)/self.scalar_static_f64[48])}else{self.scalar_static_f64[334]});
        let v548=(self.scalar_static_f64[13]*v465);
        let v551=(if self.scalar_static_bool[14]{(((v450*self.scalar_static_f64[166])+v548)-v469)}else{v505});
        let v552=(-v551);
        let v554=((v446*v552)).exp();
        let v557=((v48+(v107*v554))).sqrt();
        let v559=(v32*(v48+v557));
        let v560=(v559).ln();
        let v563=(if self.scalar_static_bool[14]{(v551+(v472*v560))}else{self.scalar_static_f64[359]});
        let v564=(self.scalar_static_f64[63]/v563);
        let v567=((self.scalar_static_f64[72]*(v564).ln())).exp();
        let v569=(if self.scalar_static_bool[14]{(self.scalar_static_f64[32]*v567)}else{self.scalar_static_f64[364]});
        let v579=(((self.scalar_static_f64[26]*v452)+(self.scalar_static_f64[7]*v455))).exp();
        let v581=(if self.scalar_static_bool[14]{(self.scalar_static_f64[74]*v579)}else{self.scalar_static_f64[374]});
        let v585=(((self.scalar_static_f64[76]*v452)-(self.scalar_static_f64[77]*v455))).exp();
        let v587=(if self.scalar_static_bool[14]{(self.scalar_static_f64[75]*v585)}else{self.scalar_static_f64[379]});
        let v589=((self.scalar_static_f64[79]*v452)).exp();
        let v591=(if self.scalar_static_bool[14]{(self.scalar_static_f64[78]*v589)}else{self.scalar_static_f64[382]});
        let v593=((self.scalar_static_f64[22]*v452)).exp();
        let v595=(if self.scalar_static_bool[14]{(self.scalar_static_f64[80]*v593)}else{self.scalar_static_f64[385]});
        let v597=(if self.scalar_static_bool[14]{(v48/v595)}else{self.scalar_static_f64[386]});
        let v600=(self.scalar_static_f64[81]*(v48+(self.scalar_static_f64[82]*v448)));
        let v613=(self.scalar_static_f64[87]*v448);
        let v617=(if self.scalar_static_bool[14]{(self.scalar_static_f64[85]*((v48+(self.scalar_static_f64[86]*v448))+(v448*v613)))}else{self.scalar_static_f64[402]});
        let v620=(self.scalar_static_f64[29]*v455);
        let v622=(((self.scalar_static_f64[28]*v452)-v620)).exp();
        let v626=(if self.scalar_static_bool[18]{self.scalar_static_f64[89]}else{(if self.scalar_static_bool[17]{(self.scalar_static_f64[89]*v622)}else{self.scalar_static_f64[409]})});
        let v628=((self.scalar_static_f64[91]*v452)).exp();
        let v630=(if self.scalar_static_bool[14]{(self.scalar_static_f64[90]*v628)}else{self.scalar_static_f64[412]});
        let v658=(if self.scalar_static_bool[14]{((v548+(v450*self.scalar_static_f64[174]))-v469)}else{v551});
        let v659=(-v658);
        let v661=((v446*v659)).exp();
        let v664=((v48+(v107*v661))).sqrt();
        let v666=(v32*(v48+v664));
        let v667=(v666).ln();
        let v670=(if self.scalar_static_bool[14]{(v658+(v472*v667))}else{self.scalar_static_f64[439]});
        let v671=(self.scalar_static_f64[97]/v670);
        let v674=((self.scalar_static_f64[107]*(v671).ln())).exp();
        let v676=(if self.scalar_static_bool[14]{(self.scalar_static_f64[106]*v674)}else{self.scalar_static_f64[444]});
        let v689=(if self.scalar_static_bool[14]{(((v450*self.scalar_static_f64[182])+(self.scalar_static_f64[16]*v465))-v469)}else{v658});
        let v690=(-v689);
        let v692=((v446*v690)).exp();
        let v695=((v48+(v107*v692))).sqrt();
        let v697=(v32*(v48+v695));
        let v698=(v697).ln();
        let v701=(if self.scalar_static_bool[14]{(v689+(v472*v698))}else{self.scalar_static_f64[459]});
        let v702=(self.scalar_static_f64[108]/v701);
        let v705=((self.scalar_static_f64[118]*(v702).ln())).exp();
        let v707=(if self.scalar_static_bool[14]{(self.scalar_static_f64[117]*v705)}else{self.scalar_static_f64[464]});
        let v719=((self.scalar_static_f64[122]*v452)).exp();
        let v721=(if self.scalar_static_bool[14]{(self.scalar_static_f64[121]*v719)}else{self.scalar_static_f64[475]});
        let v722=(self.scalar_static_f64[77]*v446);
        let v724=((self.scalar_static_f64[124]*v452)).exp();
        let v725=(v724-v48);
        let v727=((v722*v725)).exp();
        let v729=(if self.scalar_static_bool[14]{(self.scalar_static_f64[123]/v727)}else{self.scalar_static_f64[482]});
        let v732=(self.scalar_static_f64[126]+(self.scalar_static_f64[127]*v448));
        let v738=((self.scalar_static_f64[128]*v452)).exp();
        let v739=(if self.scalar_static_bool[22]{v738}else{(if self.scalar_static_bool[21]{(v48+(v448*v732))}else{self.scalar_static_f64[490]})});
        let v741=(if self.scalar_static_bool[14]{(self.scalar_static_f64[129]*v739)}else{self.scalar_static_f64[491]});
        let v742=(self.scalar_static_f64[130]*v739);
        let v743=(v620).exp();
        let v745=(if self.scalar_static_bool[14]{(v742*v743)}else{self.scalar_static_f64[494]});
        let v766=(v676<=1e-30);
        let v772=(if v766{(v569*self.scalar_static_f64[184])}else{v60});
        let v775=(v772>v60);
        let v776=(v766&&self.scalar_static_bool[23]);
        let v777=(v775&&v776);
        let v779=(if v777{self.scalar_static_f64[186]}else{v60});
        let v780=(self.scalar_static_f64[185]-v563);
        let v781=(if v777{v780}else{v60});
        let v782=2.4;
        let v787=(v563*self.scalar_static_f64[189]);
        let v788=(if v777{v787}else{v60});
        let v790=(if v777{(v772*v782)}else{v60});
        let v791=(v779-self.scalar_static_f64[72]);
        let v792=(self.scalar_static_f64[185]/v563);
        let v793=(v792).ln();
        let v795=((v791*v793)).exp();
        let v797=(if v777{(v772*v795)}else{v60});
        let v798=(v788-v4);
        let v800=(if v777{(v446*v798)}else{v60});
        let v801=80.0;
        let v802=(v800<v801);
        let v803=(v777&&v802);
        let v804=(v800).exp();
        let v805=(if v803{v804}else{v60});
        let v806=(v48+v805);
        let v809=(v806).ln();
        let v814=(v777&&(!v802));
        let v816=(if v814{v4}else{(if v803{(v788-(v444*v809))}else{v60})});
        let v817=0.1;
        let v819=(v107*v444);
        let v821=(if v777{((v781*v817)+v819)}else{v60});
        let v822=(v781+v816);
        let v824=(if v777{(v822/v821)}else{v60});
        let v825=(v824<v801);
        let v826=(v777&&v825);
        let v827=(v824).exp();
        let v828=(if v826{v827}else{v805});
        let v829=(v48+v828);
        let v835=(-(v781+v788));
        let v837=((v835/v821)).exp();
        let v838=((v829).ln()-v837);
        let v843=(v777&&(!v825));
        let v845=(if v843{v816}else{(if v826{((-v781)+(v821*v838))}else{v60})});
        let v847=(if v777{(v4-v816)}else{v60});
        let v849=(v48-(v816/v563));
        let v851=(if v777{(v849).ln()}else{v60});
        let v853=(v48-(v845/v563));
        let v855=(if v777{(v853).ln()}else{v60});
        let v857=(if v777{self.scalar_static_f64[190]}else{v60});
        let v859=(if v777{(v48-v779)}else{v60});
        let v878=((v855*v857)).exp();
        let v879=(v48-v878);
        let v882=(if v777{((v772*v879)/v857)}else{v60});
        let v884=((v851*v859)).exp();
        let v885=(v48-v884);
        let v888=(if v777{((v797*v885)/v859)}else{v60});
        let v890=((v855*v859)).exp();
        let v891=(v48-v890);
        let v894=(if v777{((v797*v891)/v859)}else{v60});
        let v896=((v882+v888)-v894);
        let v901=(!v775);
        let v902=(v776&&v901);
        let v905=(v766&&self.scalar_static_bool[24]);
        let v906=(v775&&v905);
        let v907=(if v906{v787}else{v60});
        let v908=(v907-v4);
        let v910=(if v906{(v446*v908)}else{v60});
        let v912=1.921812;
        let v914=(((v910*v910)+v912)).sqrt();
        let v915=(if v906{v914}else{v60});
        let v918=(if v906{(v32*(v910+v915))}else{v60});
        let v921=(if v906{(v907-(v444*v918))}else{v60});
        let v925=(v48-(v921/v563));
        let v927=(if v906{(v925).ln()}else{v60});
        let v933=((self.scalar_static_f64[190]*v927)).exp();
        let v934=(v48-v933);
        let v937=(if v906{((v563*v934)/self.scalar_static_f64[190])}else{v60});
        let v940=(v937+(v782*(v4-v921)));
        let v943=(v901&&v905);
        let v945=(!v766);
        let v946=(if v945{v569}else{(if v766{(v569*self.scalar_static_f64[183])}else{v60})});
        let v948=(if v945{(v676*self.scalar_static_f64[183])}else{v60});
        let v951=(v948>v60);
        let v952=(v945&&self.scalar_static_bool[25]);
        let v953=(v951&&v952);
        let v955=(if v953{self.scalar_static_f64[193]}else{v779});
        let v956=(self.scalar_static_f64[192]-v670);
        let v957=(if v953{v956}else{v781});
        let v961=(v670*self.scalar_static_f64[196]);
        let v962=(if v953{v961}else{v788});
        let v964=(if v953{(v782*v948)}else{v790});
        let v965=(v955-self.scalar_static_f64[107]);
        let v966=(self.scalar_static_f64[192]/v670);
        let v967=(v966).ln();
        let v969=((v965*v967)).exp();
        let v971=(if v953{(v948*v969)}else{v797});
        let v972=(v962-v7);
        let v974=(if v953{(v446*v972)}else{v800});
        let v975=(v974<v801);
        let v976=(v953&&v975);
        let v977=(v974).exp();
        let v978=(if v976{v977}else{v828});
        let v979=(v48+v978);
        let v982=(v979).ln();
        let v987=(v953&&(!v975));
        let v989=(if v987{v7}else{(if v976{(v962-(v444*v982))}else{v816})});
        let v992=(if v953{(v819+(v817*v957))}else{v821});
        let v993=(v957+v989);
        let v995=(if v953{(v993/v992)}else{v824});
        let v996=(v995<v801);
        let v997=(v953&&v996);
        let v998=(v995).exp();
        let v999=(if v997{v998}else{v978});
        let v1000=(v48+v999);
        let v1006=(-(v957+v962));
        let v1008=((v1006/v992)).exp();
        let v1009=((v1000).ln()-v1008);
        let v1014=(v953&&(!v996));
        let v1016=(if v1014{v989}else{(if v997{((-v957)+(v992*v1009))}else{v845})});
        let v1018=(if v953{(v7-v989)}else{v847});
        let v1020=(v48-(v989/v670));
        let v1022=(if v953{(v1020).ln()}else{v851});
        let v1024=(v48-(v1016/v670));
        let v1026=(if v953{(v1024).ln()}else{v855});
        let v1028=(if v953{self.scalar_static_f64[197]}else{v857});
        let v1030=(if v953{(v48-v955)}else{v859});
        let v1049=((v1026*v1028)).exp();
        let v1050=(v48-v1049);
        let v1053=(if v953{((v948*v1050)/v1028)}else{v882});
        let v1055=((v1022*v1030)).exp();
        let v1056=(v48-v1055);
        let v1059=(if v953{((v971*v1056)/v1030)}else{v888});
        let v1061=((v1026*v1030)).exp();
        let v1062=(v48-v1061);
        let v1065=(if v953{((v971*v1062)/v1030)}else{v894});
        let v1067=((v1053+v1059)-v1065);
        let v1072=(!v951);
        let v1073=(v952&&v1072);
        let v1076=(v945&&self.scalar_static_bool[26]);
        let v1077=(v951&&v1076);
        let v1078=(if v1077{v961}else{v907});
        let v1079=(v1078-v7);
        let v1081=(if v1077{(v446*v1079)}else{v910});
        let v1084=((v912+(v1081*v1081))).sqrt();
        let v1085=(if v1077{v1084}else{v915});
        let v1088=(if v1077{(v32*(v1081+v1085))}else{v918});
        let v1091=(if v1077{(v1078-(v444*v1088))}else{v921});
        let v1095=(v48-(v1091/v670));
        let v1097=(if v1077{(v1095).ln()}else{v927});
        let v1103=((self.scalar_static_f64[197]*v1097)).exp();
        let v1104=(v48-v1103);
        let v1107=(if v1077{((v670*v1104)/self.scalar_static_f64[197])}else{v937});
        let v1110=(v1107+(v782*(v7-v1091)));
        let v1113=(v1072&&v1076);
        let v1116=(if v945{(v676*self.scalar_static_f64[184])}else{v772});
        let v1117=(v1116>v60);
        let v1118=(v952&&v1117);
        let v1119=(if v1118{self.scalar_static_f64[193]}else{v955});
        let v1120=(if v1118{v956}else{v957});
        let v1121=(if v1118{v961}else{v962});
        let v1123=(if v1118{(v782*v1116)}else{v964});
        let v1124=(v1119-self.scalar_static_f64[107]);
        let v1126=((v967*v1124)).exp();
        let v1128=(if v1118{(v1116*v1126)}else{v971});
        let v1129=(v1121-v4);
        let v1131=(if v1118{(v446*v1129)}else{v974});
        let v1132=(v1131<v801);
        let v1133=(v1118&&v1132);
        let v1134=(v1131).exp();
        let v1135=(if v1133{v1134}else{v999});
        let v1136=(v48+v1135);
        let v1139=(v1136).ln();
        let v1144=(v1118&&(!v1132));
        let v1146=(if v1144{v4}else{(if v1133{(v1121-(v444*v1139))}else{v989})});
        let v1149=(if v1118{(v819+(v817*v1120))}else{v992});
        let v1150=(v1120+v1146);
        let v1152=(if v1118{(v1150/v1149)}else{v995});
        let v1153=(v1152<v801);
        let v1154=(v1118&&v1153);
        let v1155=(v1152).exp();
        let v1156=(if v1154{v1155}else{v1135});
        let v1157=(v48+v1156);
        let v1163=(-(v1120+v1121));
        let v1165=((v1163/v1149)).exp();
        let v1166=((v1157).ln()-v1165);
        let v1171=(v1118&&(!v1153));
        let v1173=(if v1171{v1146}else{(if v1154{((-v1120)+(v1149*v1166))}else{v1016})});
        let v1175=(if v1118{(v4-v1146)}else{v1018});
        let v1177=(v48-(v1146/v670));
        let v1179=(if v1118{(v1177).ln()}else{v1022});
        let v1181=(v48-(v1173/v670));
        let v1183=(if v1118{(v1181).ln()}else{v1026});
        let v1184=(if v1118{self.scalar_static_f64[197]}else{v1028});
        let v1186=(if v1118{(v48-v1119)}else{v1030});
        let v1204=((v1183*v1184)).exp();
        let v1205=(v48-v1204);
        let v1208=(if v1118{((v1116*v1205)/v1184)}else{v1053});
        let v1210=((v1179*v1186)).exp();
        let v1211=(v48-v1210);
        let v1214=(if v1118{((v1128*v1211)/v1186)}else{v1059});
        let v1216=((v1183*v1186)).exp();
        let v1217=(v48-v1216);
        let v1220=(if v1118{((v1128*v1217)/v1186)}else{v1065});
        let v1222=((v1208+v1214)-v1220);
        let v1227=(!v1117);
        let v1228=(v952&&v1227);
        let v1230=(v1076&&v1117);
        let v1231=(if v1230{v961}else{v1078});
        let v1232=(v1231-v4);
        let v1234=(if v1230{(v446*v1232)}else{v1081});
        let v1237=((v912+(v1234*v1234))).sqrt();
        let v1238=(if v1230{v1237}else{v1085});
        let v1241=(if v1230{(v32*(v1234+v1238))}else{v1088});
        let v1244=(if v1230{(v1231-(v444*v1241))}else{v1091});
        let v1248=(v48-(v1244/v670));
        let v1250=(if v1230{(v1248).ln()}else{v1097});
        let v1256=((self.scalar_static_f64[197]*v1250)).exp();
        let v1257=(v48-v1256);
        let v1260=(if v1230{((v670*v1257)/self.scalar_static_f64[197])}else{v1107});
        let v1263=(v1260+(v782*(v4-v1244)));
        let v1266=(v1076&&v1227);
        let v1268=(v946>v60);
        let v1269=(self.scalar_static_bool[23]&&v1268);
        let v1270=(if v1269{self.scalar_static_f64[186]}else{v1119});
        let v1271=(if v1269{v780}else{v1120});
        let v1272=(if v1269{v787}else{v1121});
        let v1273=(v782*v946);
        let v1274=(if v1269{v1273}else{v1123});
        let v1275=(v1270-self.scalar_static_f64[72]);
        let v1277=((v793*v1275)).exp();
        let v1279=(if v1269{(v946*v1277)}else{v1128});
        let v1280=(v1272-v7);
        let v1282=(if v1269{(v446*v1280)}else{v1131});
        let v1283=(v1282<v801);
        let v1284=(v1269&&v1283);
        let v1285=(v1282).exp();
        let v1286=(if v1284{v1285}else{v1156});
        let v1287=(v48+v1286);
        let v1290=(v1287).ln();
        let v1295=(v1269&&(!v1283));
        let v1297=(if v1295{v7}else{(if v1284{(v1272-(v444*v1290))}else{v1146})});
        let v1300=(if v1269{(v819+(v817*v1271))}else{v1149});
        let v1301=(v1271+v1297);
        let v1303=(if v1269{(v1301/v1300)}else{v1152});
        let v1304=(v1303<v801);
        let v1305=(v1269&&v1304);
        let v1306=(v1303).exp();
        let v1307=(if v1305{v1306}else{v1286});
        let v1308=(v48+v1307);
        let v1314=(-(v1271+v1272));
        let v1316=((v1314/v1300)).exp();
        let v1317=((v1308).ln()-v1316);
        let v1322=(v1269&&(!v1304));
        let v1324=(if v1322{v1297}else{(if v1305{((-v1271)+(v1300*v1317))}else{v1173})});
        let v1326=(if v1269{(v7-v1297)}else{v1175});
        let v1328=(v48-(v1297/v563));
        let v1330=(if v1269{(v1328).ln()}else{v1179});
        let v1332=(v48-(v1324/v563));
        let v1334=(if v1269{(v1332).ln()}else{v1183});
        let v1335=(if v1269{self.scalar_static_f64[190]}else{v1184});
        let v1337=(if v1269{(v48-v1270)}else{v1186});
        let v1355=((v1334*v1335)).exp();
        let v1356=(v48-v1355);
        let v1359=(if v1269{((v946*v1356)/v1335)}else{v1208});
        let v1361=((v1330*v1337)).exp();
        let v1362=(v48-v1361);
        let v1365=(if v1269{((v1279*v1362)/v1337)}else{v1214});
        let v1367=((v1334*v1337)).exp();
        let v1368=(v48-v1367);
        let v1371=(if v1269{((v1279*v1368)/v1337)}else{v1220});
        let v1373=((v1359+v1365)-v1371);
        let v1378=(!v1268);
        let v1379=(self.scalar_static_bool[23]&&v1378);
        let v1381=(self.scalar_static_bool[24]&&v1268);
        let v1382=(if v1381{v787}else{v1231});
        let v1383=(v1382-v7);
        let v1385=(if v1381{(v446*v1383)}else{v1234});
        let v1388=((v912+(v1385*v1385))).sqrt();
        let v1389=(if v1381{v1388}else{v1238});
        let v1392=(if v1381{(v32*(v1385+v1389))}else{v1241});
        let v1395=(if v1381{(v1382-(v444*v1392))}else{v1244});
        let v1399=(v48-(v1395/v563));
        let v1401=(if v1381{(v1399).ln()}else{v1250});
        let v1407=((self.scalar_static_f64[190]*v1401)).exp();
        let v1408=(v48-v1407);
        let v1411=(if v1381{((v563*v1408)/self.scalar_static_f64[190])}else{v1260});
        let v1414=(v1411+(v782*(v7-v1395)));
        let v1417=(self.scalar_static_bool[24]&&v1378);
        let v1418=(if v1417{v60}else{(if v1381{(v946*v1414)}else{(if v1379{v60}else{(if v1269{((v563*v1373)+(v1274*v1326))}else{v60})})})});
        let v1420=(if v1268{v787}else{v60});
        let v1421=(v1420-v7);
        let v1423=(if v1268{(v446*v1421)}else{v60});
        let v1426=((v912+(v1423*v1423))).sqrt();
        let v1427=(if v1268{v1426}else{v60});
        let v1430=(if v1268{(v32*(v1423+v1427))}else{v60});
        let v1433=(if v1268{(v1420-(v444*v1430))}else{v60});
        let v1435=(if v1268{(v1430/v1427)}else{v60});
        let v1437=(v48-(v1433/v563));
        let v1440=((self.scalar_static_f64[191]*(v1437).ln())).exp();
        let v1441=(v946*v1440);
        let v1443=(v48-v1435);
        let v1447=(if v1378{v60}else{(if v1268{((v1435*v1441)+(v1273*v1443))}else{v60})});
        let v1451=(if self.scalar_static_bool[5]{(v11-(if self.scalar_static_bool[16]{v600}else{(if self.scalar_static_bool[15]{self.scalar_static_f64[81]}else{(if self.scalar_static_bool[14]{v600}else{self.scalar_static_f64[395]})})}))}else{(if self.scalar_static_bool[4]{((if self.scalar_static_bool[16]{self.scalar_static_f64[83]}else{(if self.scalar_static_bool[15]{(self.scalar_static_f64[83]*(v48-(self.scalar_static_f64[84]*v448)))}else{self.scalar_static_f64[396]})})-v7)}else{v60})});
        let v1453=((v446*v1451)-v48);
        let v1456=((v912+(v1453*v1453))).sqrt();
        let v1459=(v48+((v1453+v1456)/v91));
        let v1460=(v444*v1459);
        let v1461=(v1460/v591);
        let v1462=(v597*v1460);
        let v1466=((self.scalar_static_f64[199]*(v1461).ln())).exp();
        let v1467=(v48+v1466);
        let v1470=(((v1467).ln()/self.scalar_static_f64[199])).exp();
        let v1471=(v1462/v1470);
        let v1474=((v1460-v591)/self.scalar_static_f64[200]);
        let v1478=(((v1474*v1474)+self.scalar_static_f64[201])).sqrt();
        let v1481=(v48+(v32*(v1474+v1478)));
        let v1482=(v1471*v1481);
        let v1484=(v1268&&(v1447>v60));
        let v1489=(!v1484);
        let v1490=(if v1489{v48}else{(if v1484{(v946/v1447)}else{v60})});
        let v1491=(if v1489{v60}else{(if v1484{(v1418/v946)}else{v1418})});
        let v1492=(v490>v60);
        let v1496=(((-(v493).ln())/self.scalar_static_f64[46])).exp();
        let v1497=(v48-v1496);
        let v1499=(if v1492{(v484*v1497)}else{v1382});
        let v1500=(v1499-v10);
        let v1502=(if v1492{(v446*v1500)}else{v1385});
        let v1505=((v912+(v1502*v1502))).sqrt();
        let v1506=(if v1492{v1505}else{v1389});
        let v1509=(if v1492{(v32*(v1502+v1506))}else{v1392});
        let v1512=(if v1492{(v1499-(v444*v1509))}else{v1395});
        let v1516=(v48-(v1512/v484));
        let v1518=(if v1492{(v1516).ln()}else{v1401});
        let v1526=((v1518*self.scalar_static_f64[203])).exp();
        let v1527=(v48-v1526);
        let v1530=(if v1492{((v484*v1527)/self.scalar_static_f64[203])}else{v1411});
        let v1531=(v10-v1512);
        let v1533=(v1530+(v493*v1531));
        let v1536=(!v1492);
        let v1537=(if v1536{v60}else{(if v1492{(v490*v1533)}else{v60})});
        let v1538=(v1537/v490);
        let v1539=(v523>v60);
        let v1540=(self.scalar_static_bool[10]&&v1539);
        let v1544=(((-(v526).ln())/self.scalar_static_f64[57])).exp();
        let v1545=(v48-v1544);
        let v1547=(if v1540{(v517*v1545)}else{v1499});
        let v1548=(v1547-v10);
        let v1550=(if v1540{(v446*v1548)}else{v1502});
        let v1553=((v912+(v1550*v1550))).sqrt();
        let v1554=(if v1540{v1553}else{v1506});
        let v1557=(if v1540{(v32*(v1550+v1554))}else{v1509});
        let v1560=(if v1540{(v1547-(v444*v1557))}else{v1512});
        let v1564=(v48-(v1560/v517));
        let v1566=(if v1540{(v1564).ln()}else{v1518});
        let v1574=((v1566*self.scalar_static_f64[205])).exp();
        let v1575=(v48-v1574);
        let v1578=(if v1540{((v517*v1575)/self.scalar_static_f64[205])}else{v1530});
        let v1579=(v10-v1560);
        let v1581=(v1578+(v526*v1579));
        let v1585=(self.scalar_static_bool[10]&&(!v1539));
        let v1586=(if v1585{v60}else{(if v1540{(v523*v1581)}else{v60})});
        let v1591=(if self.scalar_static_bool[11]{v1538}else{(if self.scalar_static_bool[10]{(v1586/v523)}else{v60})});
        let v1592=(if self.scalar_static_bool[11]{v484}else{(if self.scalar_static_bool[10]{v517}else{v60})});
        let v1599=(if self.scalar_static_bool[28]{(v444*self.scalar_static_f64[209])}else{v60});
        let v1600=(v1592-v10);
        let v1602=(if self.scalar_static_bool[28]{(v1600/v1599)}else{v60});
        let v1605=((v912+(v1602*v1602))).sqrt();
        let v1606=(v1602+v1605);
        let v1610=(if self.scalar_static_bool[28]{(v1592-(v32*(v1599*v1606)))}else{v60});
        let v1612=(v48-(v1610/v1592));
        let v1615=((self.scalar_static_f64[207]*(v1612).ln())).exp();
        let v1616=(v48-v1615);
        let v1618=(if self.scalar_static_bool[28]{(v721*v1616)}else{v60});
        let v1621=((v1618).abs()>=0.001);
        let v1622=(self.scalar_static_bool[28]&&v1621);
        let v1623=(v1618).exp();
        let v1624=(v1623-v48);
        let v1628=(self.scalar_static_bool[28]&&(!v1621));
        let v1631=(if v1628{(v48+(v32*v1618))}else{(if v1622{(v1624/v1618)}else{self.scalar_static_f64[208]})});
        let v1632=(v1591*v1631);
        let v1638=20.0;
        let v1640=((((v48+(v1632/v729))+(v1491/self.scalar_static_f64[210]))*v1638)-v48);
        let v1641=0.025;
        let v1644=((v912+(v1640*v1640))).sqrt();
        let v1648=(v1641*(v48+((v1640+v1644)/v91)));
        let v1657=((v617+(self.scalar_static_f64[211]*(v1490-v48)))+(self.scalar_static_f64[212]*((v48/v1490)-v48)));
        let v1663=(v48+(if self.scalar_static_bool[29]{((v1657/v617)-v48)}else{v60}));
        let v1667=(if self.scalar_static_bool[30]{v587}else{(if self.scalar_static_bool[29]{(v587/v1663)}else{v60})});
        let v1670=(v444*self.scalar_static_f64[215]);
        let v1671=(v10/v1670);
        let v1672=(v1671>v801);
        let v1676=(if v1672{v801}else{v1671});
        let v1677=(!v1672);
        let v1678=(if v1677{v48}else{(if v1672{(v48+(v1671-v801))}else{v60})});
        let v1679=scalar_limexp(v1676);
        let v1680=(v1678*v1679);
        let v1681=(v581*v1680);
        let v1683=(v444*self.scalar_static_f64[216]);
        let v1684=(v7/v1683);
        let v1685=(v1684>v801);
        let v1689=(if v1685{v801}else{v1684});
        let v1690=(!v1685);
        let v1691=(if v1690{v48}else{(if v1685{(v48+(v1684-v801))}else{v60})});
        let v1692=scalar_limexp(v1689);
        let v1693=(v1691*v1692);
        let v1694=(v581*v1693);
        let v1698=((v1681/v1667)+(v1694/self.scalar_static_f64[214]));
        let v1699=0.6666;
        let v1700=(v1681/v1482);
        let v1701=(v1681*v1700);
        let v1702=(v745/v741);
        let v1703=(v1701*v1702);
        let v1706=((v1699*(v1703).ln())).exp();
        let v1709=(v1681/v741);
        let v1710=(v1698+v1709);
        let v1714=(if self.scalar_static_bool[32]{v1698}else{(if self.scalar_static_bool[31]{(v1698+v1706)}else{v60})});
        let v1715=(if self.scalar_static_bool[32]{v1710}else{(if self.scalar_static_bool[31]{(v1706+v1710)}else{v60})});
        let v1716=(v1648*v1648);
        let v1718=((v1714+v1716)).sqrt();
        let v1719=(v1648+v1718);
        let v1721=((v1715+v1716)).sqrt();
        let v1726=(((v1715-v1714)).abs()>1e-8);
        let v1728=(v1482/self.scalar_static_f64[217]);
        let v1729=(v1728/v1681);
        let v1732=(if v1726{(v48-(v1719*v1729))}else{v60});
        let v1733=((v1648+v1721)-v1719);
        let v1736=(if v1726{(v48+(v1729*v1733))}else{v60});
        let v1738=(if v1726{(v1732/v1736)}else{v60});
        let v1740=0.01;
        let v1742=(((v1738*v1738)+v1740)).sqrt();
        let v1744=2.004987562112089;
        let v1747=(!v1726);
        let v1748=(if v1747{v60}else{(if v1726{((v1738+v1742)/v1744)}else{v60})});
        let v1752=(v1709*v1748);
        let v1754=(v1698+(v1748*v1752));
        let v1760=((v1716+(if self.scalar_static_bool[35]{v1754}else{(if self.scalar_static_bool[34]{(v1706+v1754)}else{v60})}))).sqrt();
        let v1766=-2.0;
        let v1768=(if self.scalar_static_bool[36]{(v1648*v1766)}else{v60});
        let v1776=(if self.scalar_static_bool[41]{(-v1754)}else{v60});
        let v1777=(-v1681);
        let v1778=(v1681*v1777);
        let v1779=(v1778/v1482);
        let v1780=(v745*v1779);
        let v1784=(if self.scalar_static_bool[36]{(v1768*v1768)}else{v60});
        let v1787=(if self.scalar_static_bool[36]{(v1776-(self.scalar_static_f64[219]*v1784))}else{v60});
        let v1788=(v91*v1768);
        let v1790=27.0;
        let v1796=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{(v1780/v741)}else{v60})+(((v1784*v1788)/v1790)-(self.scalar_static_f64[219]*(v1768*v1776))))}else{v60});
        let v1798=0.25;
        let v1800=(v1787*v1787);
        let v1801=(v1787*v1800);
        let v1804=(if self.scalar_static_bool[36]{(((v1796*v1796)*v1798)+(v1801/v1790))}else{v60});
        let v1807=((v1804).abs()<1e-10);
        let v1808=(self.scalar_static_bool[36]&&v1807);
        let v1809=(v43*v1796);
        let v1811=(self.scalar_static_f64[219]*v1768);
        let v1814=(v1804>v60);
        let v1816=(self.scalar_static_bool[36]&&(!v1807));
        let v1817=(v1814&&v1816);
        let v1819=(v32*(-v1796));
        let v1820=(if v1817{v1819}else{v60});
        let v1821=(v1804).sqrt();
        let v1822=(if v1817{v1821}else{v60});
        let v1824=(if v1817{(v1820+v1822)}else{v1784});
        let v1825=(v1824>v60);
        let v1826=(v1817&&v1825);
        let v1829=((self.scalar_static_f64[219]*(v1824).ln())).exp();
        let v1832=(v1817&&(!v1825));
        let v1833=(-v1824);
        let v1836=((self.scalar_static_f64[219]*(v1833).ln())).exp();
        let v1840=(if v1817{(v1820-v1822)}else{v1824});
        let v1841=(v1840>v60);
        let v1842=(v1817&&v1841);
        let v1845=((self.scalar_static_f64[219]*(v1840).ln())).exp();
        let v1848=(v1817&&(!v1841));
        let v1849=(-v1840);
        let v1852=((self.scalar_static_f64[219]*(v1849).ln())).exp();
        let v1859=(v1816&&(!v1814));
        let v1860=-27.0;
        let v1862=((v1860/v1801)).sqrt();
        let v1864=(if v1859{(v1819*v1862)}else{v1840});
        let v1866=(if v1859{(v1864*v1864)}else{v1820});
        let v1867=(v1864>=v60);
        let v1868=(v1859&&v1867);
        let v1869=1.5707963267948966;
        let v1870=(v48-v1866);
        let v1872=((v1866/v1870)).sqrt();
        let v1873=(v1872).atan();
        let v1877=(v1859&&(!v1867));
        let v1879=(if v1877{(v1869+v1873)}else{(if v1868{(v1869-v1873)}else{v1864})});
        let v1880=-4.0;
        let v1883=((self.scalar_static_f64[219]*(v1787*v1880))).sqrt();
        let v1884=(self.scalar_static_f64[219]*v1879);
        let v1885=(v1884).cos();
        let v1890=(if self.scalar_static_bool[36]{(if v1859{(if v1859{((v1883*v1885)-v1811)}else{v1879})}else{(if v1817{(((if v1832{(-v1836)}else{(if v1826{v1829}else{v60})})+(if v1848{(-v1852)}else{(if v1842{v1845}else{v60})}))-v1811)}else{(if v1808{((v1809/v1787)-v1811)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v1648+v1760)}else{v60})});
        let v1891=1e-20;
        let v1892=(v1890<v1891);
        let v1893=(if v1892{v1891}else{v1890});
        let v1894=(v1681/v1893);
        let v1895=(v1694/v1893);
        let v1896=(v1894<v1891);
        let v1897=(if v1896{v1891}else{v1894});
        let v1901=(v48-(v1482/v1897));
        let v1905=(((v1901*v1901)+self.scalar_static_f64[220])).sqrt();
        let v1910=((v1901+v1905)/self.scalar_static_f64[223]);
        let v1911=(v630*v1910);
        let v1912=(v1910*v1911);
        let v1915=(v1897/v1482);
        let v1918=((self.scalar_static_f64[224]*(v1915).ln())).exp();
        let v1919=(v626*v1918);
        let v1924=((v1897*v1912)+((v1657*v1897)+((v1897*v1919)/self.scalar_static_f64[225])));
        let v1992=(v569>v60);
        let v1993=(self.scalar_static_bool[23]&&v1992);
        let v1994=(if v1993{self.scalar_static_f64[186]}else{v1270});
        let v1995=(if v1993{v780}else{v1271});
        let v1996=(if v1993{v787}else{v1272});
        let v1998=(if v1993{(v569*v782)}else{v1274});
        let v1999=(v1994-self.scalar_static_f64[72]);
        let v2001=((v793*v1999)).exp();
        let v2003=(if v1993{(v569*v2001)}else{v1279});
        let v2004=(v1996-v7);
        let v2006=(if v1993{(v446*v2004)}else{v1282});
        let v2007=(v2006<v801);
        let v2008=(v1993&&v2007);
        let v2009=(v2006).exp();
        let v2010=(if v2008{v2009}else{v1307});
        let v2011=(v48+v2010);
        let v2014=(v2011).ln();
        let v2019=(v1993&&(!v2007));
        let v2021=(if v2019{v7}else{(if v2008{(v1996-(v444*v2014))}else{v1297})});
        let v2024=(if v1993{(v819+(v817*v1995))}else{v1300});
        let v2025=(v1995+v2021);
        let v2027=(if v1993{(v2025/v2024)}else{v1303});
        let v2028=(v2027<v801);
        let v2029=(v1993&&v2028);
        let v2030=(v2027).exp();
        let v2031=(if v2029{v2030}else{v2010});
        let v2032=(v48+v2031);
        let v2038=(-(v1995+v1996));
        let v2040=((v2038/v2024)).exp();
        let v2041=((v2032).ln()-v2040);
        let v2046=(v1993&&(!v2028));
        let v2048=(if v2046{v2021}else{(if v2029{((-v1995)+(v2024*v2041))}else{v1324})});
        let v2052=(v48-(v2021/v563));
        let v2054=(if v1993{(v2052).ln()}else{v1330});
        let v2056=(v48-(v2048/v563));
        let v2058=(if v1993{(v2056).ln()}else{v1334});
        let v2059=(if v1993{self.scalar_static_f64[190]}else{v1335});
        let v2061=(if v1993{(v48-v1994)}else{v1337});
        let v2082=((v2058*v2059)).exp();
        let v2083=(v48-v2082);
        let v2088=((v2054*v2061)).exp();
        let v2089=(v48-v2088);
        let v2094=((v2058*v2061)).exp();
        let v2095=(v48-v2094);
        let v2102=(self.scalar_static_bool[24]&&v1992);
        let v2103=(if v2102{v787}else{v1547});
        let v2104=(v2103-v7);
        let v2106=(if v2102{(v446*v2104)}else{v1550});
        let v2109=((v912+(v2106*v2106))).sqrt();
        let v2110=(if v2102{v2109}else{v1554});
        let v2113=(if v2102{(v32*(v2106+v2110))}else{v1557});
        let v2116=(if v2102{(v2103-(v444*v2113))}else{v1560});
        let v2120=(v48-(v2116/v563));
        let v2122=(if v2102{(v2120).ln()}else{v1566});
        let v2133=((self.scalar_static_f64[190]*v2122)).exp();
        let v2134=(v48-v2133);
        let v2256=(v707>v60);
        let v2257=(self.scalar_static_bool[52]&&v2256);
        let v2259=(if v2257{self.scalar_static_f64[236]}else{v1994});
        let v2261=(if v2257{(self.scalar_static_f64[235]-v701)}else{v1995});
        let v2265=(v701*self.scalar_static_f64[239]);
        let v2266=(if v2257{v2265}else{v1996});
        let v2268=(if v2257{(v707*v782)}else{v1998});
        let v2269=(v2259-self.scalar_static_f64[118]);
        let v2270=(self.scalar_static_f64[235]/v701);
        let v2273=((v2269*(v2270).ln())).exp();
        let v2275=(if v2257{(v707*v2273)}else{v2003});
        let v2276=(v2266-v14);
        let v2278=(if v2257{(v446*v2276)}else{v2006});
        let v2279=(v2278<v801);
        let v2280=(v2257&&v2279);
        let v2281=(v2278).exp();
        let v2282=(if v2280{v2281}else{v2031});
        let v2283=(v48+v2282);
        let v2284=(v2283).ln();
        let v2289=(v2257&&(!v2279));
        let v2290=(if v2289{v14}else{(if v2280{(v2266-(v444*v2284))}else{v2021})});
        let v2293=(if v2257{(v819+(v817*v2261))}else{v2024});
        let v2294=(v2261+v2290);
        let v2296=(if v2257{(v2294/v2293)}else{v2027});
        let v2297=(v2296<v801);
        let v2298=(v2257&&v2297);
        let v2299=(v2296).exp();
        let v2301=(v48+(if v2298{v2299}else{v2282}));
        let v2305=(-(v2261+v2266));
        let v2307=((v2305/v2293)).exp();
        let v2308=((v2301).ln()-v2307);
        let v2313=(v2257&&(!v2297));
        let v2314=(if v2313{v2290}else{(if v2298{((-v2261)+(v2293*v2308))}else{v2048})});
        let v2316=(if v2257{(v14-v2290)}else{(if v1993{(v7-v2021)}else{v1326})});
        let v2318=(v48-(v2290/v701));
        let v2322=(v48-(v2314/v701));
        let v2324=(if v2257{(v2322).ln()}else{v2058});
        let v2326=(if v2257{self.scalar_static_f64[240]}else{v2059});
        let v2328=(if v2257{(v48-v2259)}else{v2061});
        let v2330=((v2324*v2326)).exp();
        let v2331=(v48-v2330);
        let v2336=(((if v2257{(v2318).ln()}else{v2054})*v2328)).exp();
        let v2337=(v48-v2336);
        let v2342=((v2324*v2328)).exp();
        let v2343=(v48-v2342);
        let v2348=(((if v2257{((v707*v2331)/v2326)}else{(if v1993{((v569*v2083)/v2059)}else{v1359})})+(if v2257{((v2275*v2337)/v2328)}else{(if v1993{((v2003*v2089)/v2061)}else{v1365})}))-(if v2257{((v2275*v2343)/v2328)}else{(if v1993{((v2003*v2095)/v2061)}else{v1371})}));
        let v2353=(!v2256);
        let v2354=(self.scalar_static_bool[52]&&v2353);
        let v2357=(v2256&&self.scalar_static_bool[53]);
        let v2358=(if v2357{v2265}else{v2103});
        let v2359=(v2358-v14);
        let v2361=(if v2357{(v446*v2359)}else{v2106});
        let v2364=((v912+(v2361*v2361))).sqrt();
        let v2368=(if v2357{(v32*(v2361+(if v2357{v2364}else{v2110})))}else{v2113});
        let v2371=(if v2357{(v2358-(v444*v2368))}else{v2116});
        let v2373=(v48-(v2371/v701));
        let v2377=((self.scalar_static_f64[240]*(if v2357{(v2373).ln()}else{v2122}))).exp();
        let v2378=(v48-v2377);
        let v2384=((if v2357{((v701*v2378)/self.scalar_static_f64[240])}else{(if v2102{((v563*v2134)/self.scalar_static_f64[190])}else{v1578})})+(v782*(v14-v2371)));
        let v2387=(v2353&&self.scalar_static_bool[53]);
        let v2399=ctx.node_voltage(nodes[8]);
        let v2400=(if self.scalar_static_bool[58]{v2399}else{v1924});
        let v2407=ctx.node_voltage(nodes[9]);
        let v2408=(if self.scalar_static_bool[58]{v2407}else{v1897});
        let v2417=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(v2400*self.scalar_static_f64[242]))}else{v60})});
        let v2419=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(v2408*self.scalar_static_f64[243]))}else{v60})});
        let v2428=(self.scalar_static_f64[0]*(if v2387{v60}else{(if v2357{(v707*v2384)}else{(if v2354{v60}else{(if v2257{((v701*v2348)+(v2268*v2316))}else{v60})})})}));
        let v2429=(self.scalar_static_f64[0]*(if v1266{v60}else{(if v1230{(v1116*v1263)}else{(if v1228{v60}else{(if v1118{((v670*v1222)+(v1123*v1175))}else{(if v943{v60}else{(if v906{(v772*v940)}else{(if v902{v60}else{(if v777{((v563*v896)+(v790*v847))}else{v60})})})})})})})}));
        let v2430=(self.scalar_static_f64[0]*(v4*self.scalar_static_f64[244]));
        let v2431=(self.scalar_static_f64[0]*((self.scalar_static_f64[0]*(v1-v15))*self.scalar_static_f64[245]));
        let v2433=(self.scalar_static_f64[0]*(((if v1113{v60}else{(if v1077{(v948*v1110)}else{(if v1073{v60}else{(if v953{((v670*v1067)+(v964*v1018))}else{v60})})})})+v1418)+(v1895*self.scalar_static_f64[226])));
        let v2435=(self.scalar_static_f64[0]*(v1537+v2400));
        let v2450=(v431*self.scalar_static_f64[246]);
        let v2474=(if v440{v60}else{(if v435{v60}else{self.scalar_static_f64[249]})});
        let v2477=(if self.scalar_static_bool[14]{((v26*v2474)/v28)}else{v60});
        let v2481=(if self.scalar_static_bool[14]{((-v2477)/(v444*v444))}else{v60});
        let v2482=(if self.scalar_static_bool[14]{v2474}else{v60});
        let v2484=(if self.scalar_static_bool[14]{(v2474/self.scalar_static_f64[2])}else{v60});
        let v2486=(if self.scalar_static_bool[14]{(v2484/v450)}else{v60});
        let v2490=(if self.scalar_static_bool[14]{((v453*v2481)+(v446*v2484))}else{v60});
        let v2492=(-v2484);
        let v2493=(self.scalar_static_f64[10]*v2492);
        let v2498=((v468*v2486)+(v452*(self.scalar_static_f64[20]*v2477)));
        let v2500=(if self.scalar_static_bool[14]{(((self.scalar_static_f64[150]*v2484)+v2493)-v2498)}else{v60});
        let v2501=(v91*v2477);
        let v2516=(if self.scalar_static_bool[14]{(v2500+((v481*v2501)+(v472*((v32*((v107*(v475*((v473*v2481)+(v446*(-v2500)))))/(v91*v478)))/v480))))}else{v60});
        let v2519=(v484*v484);
        let v2525=(if self.scalar_static_bool[14]{(self.scalar_static_f64[30]*(v488*(self.scalar_static_f64[46]*(((-(self.scalar_static_f64[36]*v2516))/v2519)/v485))))}else{v60});
        let v2528=(if self.scalar_static_bool[14]{((self.scalar_static_f64[47]*v2516)/self.scalar_static_f64[36])}else{v60});
        let v2532=(if self.scalar_static_bool[14]{((v2493+(self.scalar_static_f64[158]*v2484))-v2498)}else{v2500});
        let v2547=(if self.scalar_static_bool[14]{(v2532+((v514*v2501)+(v472*((v32*((v107*(v508*((v506*v2481)+(v446*(-v2532)))))/(v91*v511)))/v513))))}else{v60});
        let v2550=(v517*v517);
        let v2556=(if self.scalar_static_bool[14]{(self.scalar_static_f64[30]*(v521*(self.scalar_static_f64[57]*(((-(self.scalar_static_f64[48]*v2547))/v2550)/v518))))}else{v60});
        let v2559=(if self.scalar_static_bool[14]{((self.scalar_static_f64[58]*v2547)/self.scalar_static_f64[48])}else{v60});
        let v2573=(self.scalar_static_f64[13]*v2492);
        let v2576=(if self.scalar_static_bool[14]{(((self.scalar_static_f64[166]*v2484)+v2573)-v2498)}else{v2532});
        let v2591=(if self.scalar_static_bool[14]{(v2576+((v560*v2501)+(v472*((v32*((v107*(v554*((v552*v2481)+(v446*(-v2576)))))/(v91*v557)))/v559))))}else{v60});
        let v2594=(v563*v563);
        let v2600=(if self.scalar_static_bool[14]{(self.scalar_static_f64[32]*(v567*(self.scalar_static_f64[72]*(((-(self.scalar_static_f64[63]*v2591))/v2594)/v564))))}else{v60});
        let v2612=(if self.scalar_static_bool[14]{(self.scalar_static_f64[74]*(v579*((self.scalar_static_f64[26]*v2486)+(self.scalar_static_f64[7]*v2490))))}else{v60});
        let v2618=(if self.scalar_static_bool[14]{(self.scalar_static_f64[75]*(v585*((self.scalar_static_f64[76]*v2486)-(self.scalar_static_f64[77]*v2490))))}else{v60});
        let v2622=(if self.scalar_static_bool[14]{(self.scalar_static_f64[78]*(v589*(self.scalar_static_f64[79]*v2486)))}else{v60});
        let v2632=(self.scalar_static_f64[81]*(self.scalar_static_f64[82]*v2482));
        let v2648=(if self.scalar_static_bool[14]{(self.scalar_static_f64[85]*((self.scalar_static_f64[86]*v2482)+((v613*v2482)+(v448*(self.scalar_static_f64[87]*v2482)))))}else{v60});
        let v2650=(self.scalar_static_f64[29]*v2490);
        let v2677=(if self.scalar_static_bool[14]{((v2573+(self.scalar_static_f64[174]*v2484))-v2498)}else{v2576});
        let v2692=(if self.scalar_static_bool[14]{(v2677+((v667*v2501)+(v472*((v32*((v107*(v661*((v659*v2481)+(v446*(-v2677)))))/(v91*v664)))/v666))))}else{v60});
        let v2695=(v670*v670);
        let v2701=(if self.scalar_static_bool[14]{(self.scalar_static_f64[106]*(v674*(self.scalar_static_f64[107]*(((-(self.scalar_static_f64[97]*v2692))/v2695)/v671))))}else{v60});
        let v2706=(if self.scalar_static_bool[14]{(((self.scalar_static_f64[182]*v2484)+(self.scalar_static_f64[16]*v2492))-v2498)}else{v2677});
        let v2721=(if self.scalar_static_bool[14]{(v2706+((v698*v2501)+(v472*((v32*((v107*(v692*((v690*v2481)+(v446*(-v2706)))))/(v91*v695)))/v697))))}else{v60});
        let v2724=(v701*v701);
        let v2730=(if self.scalar_static_bool[14]{(self.scalar_static_f64[117]*(v705*(self.scalar_static_f64[118]*(((-(self.scalar_static_f64[108]*v2721))/v2724)/v702))))}else{v60});
        let v2764=(if self.scalar_static_bool[22]{(v738*(self.scalar_static_f64[128]*v2486))}else{(if self.scalar_static_bool[21]{((v732*v2482)+(v448*(self.scalar_static_f64[127]*v2482)))}else{v60})});
        let v2766=(if self.scalar_static_bool[14]{(self.scalar_static_f64[129]*v2764)}else{v60});
        let v2772=(if self.scalar_static_bool[14]{((v743*(self.scalar_static_f64[130]*v2764))+(v742*(v743*v2650)))}else{v60});
        let v2796=(if v766{(self.scalar_static_f64[184]*v2600)}else{v60});
        let v2797=(-v2591);
        let v2798=(if v777{v2797}else{v60});
        let v2799=(self.scalar_static_f64[189]*v2591);
        let v2800=(if v777{v2799}else{v60});
        let v2802=(if v777{(v782*v2796)}else{v60});
        let v2806=(((-(self.scalar_static_f64[185]*v2591))/v2594)/v792);
        let v2812=(if v777{((v795*v2796)+(v772*(v795*(v791*v2806))))}else{v60});
        let v2813=(v446*self.scalar_static_f64[247]);
        let v2817=(self.scalar_static_f64[0]*v446);
        let v2818=(if v777{v2813}else{v60});
        let v2819=(if v777{((v798*v2481)+(v446*v2800))}else{v60});
        let v2820=(if v777{v2817}else{v60});
        let v2824=(if v803{(v804*v2818)}else{v60});
        let v2825=(if v803{(v804*v2819)}else{v60});
        let v2826=(if v803{(v804*v2820)}else{v60});
        let v2860=(if v814{self.scalar_static_f64[0]}else{(if v803{(-(v444*(v2824/v806)))}else{v60})});
        let v2861=(if v814{v60}else{(if v803{(v2800-((v809*v2477)+(v444*(v2825/v806))))}else{v60})});
        let v2862=(if v814{self.scalar_static_f64[247]}else{(if v803{(-(v444*(v2826/v806)))}else{v60})});
        let v2864=(v107*v2477);
        let v2866=(if v777{((v817*v2798)+v2864)}else{v60});
        let v2872=(v821*v821);
        let v2875=(if v777{(v2860/v821)}else{v60});
        let v2876=(if v777{(((v821*(v2798+v2861))-(v822*v2866))/v2872)}else{v60});
        let v2877=(if v777{(v2862/v821)}else{v60});
        let v2881=(if v826{(v827*v2875)}else{v2824});
        let v2882=(if v826{(v827*v2876)}else{v2825});
        let v2883=(if v826{(v827*v2877)}else{v2826});
        let v2924=(if v843{v2860}else{(if v826{(v821*(v2881/v829))}else{v60})});
        let v2925=(if v843{v2861}else{(if v826{((-v2798)+((v838*v2866)+(v821*((v2882/v829)-(v837*(((v821*(-(v2798+v2800)))-(v835*v2866))/v2872))))))}else{v60})});
        let v2926=(if v843{v2862}else{(if v826{(v821*(v2883/v829))}else{v60})});
        let v2930=(if v777{(self.scalar_static_f64[0]-v2860)}else{v60});
        let v2931=(if v777{(-v2861)}else{v60});
        let v2932=(if v777{(self.scalar_static_f64[247]-v2862)}else{v60});
        let v2945=(if v777{((-(v2860/v563))/v849)}else{v60});
        let v2946=(if v777{((-(((v563*v2861)-(v816*v2591))/v2594))/v849)}else{v60});
        let v2947=(if v777{((-(v2862/v563))/v849)}else{v60});
        let v2960=(if v777{((-(v2924/v563))/v853)}else{v60});
        let v2961=(if v777{((-(((v563*v2925)-(v845*v2591))/v2594))/v853)}else{v60});
        let v2962=(if v777{((-(v2926/v563))/v853)}else{v60});
        let v3049=(if v777{((v772*(-(v878*(v857*v2960))))/v857)}else{v60});
        let v3050=(if v777{(((v879*v2796)+(v772*(-(v878*(v857*v2961)))))/v857)}else{v60});
        let v3051=(if v777{((v772*(-(v878*(v857*v2962))))/v857)}else{v60});
        let v3069=(if v777{((v797*(-(v884*(v859*v2945))))/v859)}else{v60});
        let v3070=(if v777{(((v885*v2812)+(v797*(-(v884*(v859*v2946)))))/v859)}else{v60});
        let v3071=(if v777{((v797*(-(v884*(v859*v2947))))/v859)}else{v60});
        let v3089=(if v777{((v797*(-(v890*(v859*v2960))))/v859)}else{v60});
        let v3090=(if v777{(((v891*v2812)+(v797*(-(v890*(v859*v2961)))))/v859)}else{v60});
        let v3091=(if v777{((v797*(-(v890*(v859*v2962))))/v859)}else{v60});
        let v3117=(if v906{v2799}else{v60});
        let v3121=(if v906{v2813}else{v60});
        let v3122=(if v906{((v908*v2481)+(v446*v3117))}else{v60});
        let v3123=(if v906{v2817}else{v60});
        let v3124=(v910*v3121);
        let v3126=(v910*v3122);
        let v3128=(v910*v3123);
        let v3130=(v91*v914);
        let v3134=(if v906{((v3124+v3124)/v3130)}else{v60});
        let v3135=(if v906{((v3126+v3126)/v3130)}else{v60});
        let v3136=(if v906{((v3128+v3128)/v3130)}else{v60});
        let v3143=(if v906{(v32*(v3121+v3134))}else{v60});
        let v3144=(if v906{(v32*(v3122+v3135))}else{v60});
        let v3145=(if v906{(v32*(v3123+v3136))}else{v60});
        let v3154=(if v906{(-(v444*v3143))}else{v60});
        let v3155=(if v906{(v3117-((v918*v2477)+(v444*v3144)))}else{v60});
        let v3156=(if v906{(-(v444*v3145))}else{v60});
        let v3185=(if v906{((-(v3154/v563))/v925)}else{v60});
        let v3186=(if v906{((-(((v563*v3155)-(v921*v2591))/v2594))/v925)}else{v60});
        let v3187=(if v906{((-(v3156/v563))/v925)}else{v60});
        let v3223=(if v906{((v563*(-(v933*(self.scalar_static_f64[190]*v3185))))/self.scalar_static_f64[190])}else{v60});
        let v3224=(if v906{(((v934*v2591)+(v563*(-(v933*(self.scalar_static_f64[190]*v3186)))))/self.scalar_static_f64[190])}else{v60});
        let v3225=(if v906{((v563*(-(v933*(self.scalar_static_f64[190]*v3187))))/self.scalar_static_f64[190])}else{v60});
        let v3246=(if v945{v2600}else{(if v766{(self.scalar_static_f64[183]*v2600)}else{v60})});
        let v3248=(if v945{(self.scalar_static_f64[183]*v2701)}else{v60});
        let v3249=(-v2692);
        let v3250=(if v953{v3249}else{v2798});
        let v3251=(self.scalar_static_f64[196]*v2692);
        let v3252=(if v953{v3251}else{v2800});
        let v3254=(if v953{(v782*v3248)}else{v2802});
        let v3258=(((-(self.scalar_static_f64[192]*v2692))/v2695)/v966);
        let v3264=(if v953{((v969*v3248)+(v948*(v969*(v965*v3258))))}else{v2812});
        let v3268=(if v953{v60}else{v2818});
        let v3269=(if v953{((v972*v2481)+(v446*v3252))}else{v2819});
        let v3270=(if v953{v2817}else{v2820});
        let v3271=(if v953{v2813}else{v60});
        let v3276=(if v976{(v977*v3268)}else{v2881});
        let v3277=(if v976{(v977*v3269)}else{v2882});
        let v3278=(if v976{(v977*v3270)}else{v2883});
        let v3279=(if v976{(v977*v3271)}else{v60});
        let v3323=(if v987{v60}else{(if v976{(-(v444*(v3276/v979)))}else{v2860})});
        let v3324=(if v987{v60}else{(if v976{(v3252-((v982*v2477)+(v444*(v3277/v979))))}else{v2861})});
        let v3325=(if v987{self.scalar_static_f64[247]}else{(if v976{(-(v444*(v3278/v979)))}else{v2862})});
        let v3326=(if v987{self.scalar_static_f64[0]}else{(if v976{(-(v444*(v3279/v979)))}else{v60})});
        let v3329=(if v953{(v2864+(v817*v3250))}else{v2866});
        let v3335=(v992*v992);
        let v3339=(if v953{(v3323/v992)}else{v2875});
        let v3340=(if v953{(((v992*(v3250+v3324))-(v993*v3329))/v3335)}else{v2876});
        let v3341=(if v953{(v3325/v992)}else{v2877});
        let v3342=(if v953{(v3326/v992)}else{v60});
        let v3347=(if v997{(v998*v3339)}else{v3276});
        let v3348=(if v997{(v998*v3340)}else{v3277});
        let v3349=(if v997{(v998*v3341)}else{v3278});
        let v3350=(if v997{(v998*v3342)}else{v3279});
        let v3400=(if v1014{v3323}else{(if v997{(v992*(v3347/v1000))}else{v2924})});
        let v3401=(if v1014{v3324}else{(if v997{((-v3250)+((v1009*v3329)+(v992*((v3348/v1000)-(v1008*(((v992*(-(v3250+v3252)))-(v1006*v3329))/v3335))))))}else{v2925})});
        let v3402=(if v1014{v3325}else{(if v997{(v992*(v3349/v1000))}else{v2926})});
        let v3403=(if v1014{v3326}else{(if v997{(v992*(v3350/v1000))}else{v60})});
        let v3408=(if v953{(-v3323)}else{v2930});
        let v3409=(if v953{(-v3324)}else{v2931});
        let v3410=(if v953{(self.scalar_static_f64[247]-v3325)}else{v2932});
        let v3411=(if v953{(self.scalar_static_f64[0]-v3326)}else{v60});
        let v3427=(if v953{((-(v3323/v670))/v1020)}else{v2945});
        let v3428=(if v953{((-(((v670*v3324)-(v989*v2692))/v2695))/v1020)}else{v2946});
        let v3429=(if v953{((-(v3325/v670))/v1020)}else{v2947});
        let v3430=(if v953{((-(v3326/v670))/v1020)}else{v60});
        let v3446=(if v953{((-(v3400/v670))/v1024)}else{v2960});
        let v3447=(if v953{((-(((v670*v3401)-(v1016*v2692))/v2695))/v1024)}else{v2961});
        let v3448=(if v953{((-(v3402/v670))/v1024)}else{v2962});
        let v3449=(if v953{((-(v3403/v670))/v1024)}else{v60});
        let v3562=(if v953{((v948*(-(v1049*(v1028*v3446))))/v1028)}else{v3049});
        let v3563=(if v953{(((v1050*v3248)+(v948*(-(v1049*(v1028*v3447)))))/v1028)}else{v3050});
        let v3564=(if v953{((v948*(-(v1049*(v1028*v3448))))/v1028)}else{v3051});
        let v3565=(if v953{((v948*(-(v1049*(v1028*v3449))))/v1028)}else{v60});
        let v3588=(if v953{((v971*(-(v1055*(v1030*v3427))))/v1030)}else{v3069});
        let v3589=(if v953{(((v1056*v3264)+(v971*(-(v1055*(v1030*v3428)))))/v1030)}else{v3070});
        let v3590=(if v953{((v971*(-(v1055*(v1030*v3429))))/v1030)}else{v3071});
        let v3591=(if v953{((v971*(-(v1055*(v1030*v3430))))/v1030)}else{v60});
        let v3614=(if v953{((v971*(-(v1061*(v1030*v3446))))/v1030)}else{v3089});
        let v3615=(if v953{(((v1062*v3264)+(v971*(-(v1061*(v1030*v3447)))))/v1030)}else{v3090});
        let v3616=(if v953{((v971*(-(v1061*(v1030*v3448))))/v1030)}else{v3091});
        let v3617=(if v953{((v971*(-(v1061*(v1030*v3449))))/v1030)}else{v60});
        let v3650=(if v1077{v3251}else{v3117});
        let v3654=(if v1077{v60}else{v3121});
        let v3655=(if v1077{((v1079*v2481)+(v446*v3650))}else{v3122});
        let v3656=(if v1077{v2817}else{v3123});
        let v3657=(if v1077{v2813}else{v60});
        let v3658=(v1081*v3654);
        let v3660=(v1081*v3655);
        let v3662=(v1081*v3656);
        let v3664=(v1081*v3657);
        let v3666=(v91*v1084);
        let v3671=(if v1077{((v3658+v3658)/v3666)}else{v3134});
        let v3672=(if v1077{((v3660+v3660)/v3666)}else{v3135});
        let v3673=(if v1077{((v3662+v3662)/v3666)}else{v3136});
        let v3674=(if v1077{((v3664+v3664)/v3666)}else{v60});
        let v3683=(if v1077{(v32*(v3654+v3671))}else{v3143});
        let v3684=(if v1077{(v32*(v3655+v3672))}else{v3144});
        let v3685=(if v1077{(v32*(v3656+v3673))}else{v3145});
        let v3686=(if v1077{(v32*(v3657+v3674))}else{v60});
        let v3697=(if v1077{(-(v444*v3683))}else{v3154});
        let v3698=(if v1077{(v3650-((v1088*v2477)+(v444*v3684)))}else{v3155});
        let v3699=(if v1077{(-(v444*v3685))}else{v3156});
        let v3700=(if v1077{(-(v444*v3686))}else{v60});
        let v3737=(if v1077{((-(v3697/v670))/v1095)}else{v3185});
        let v3738=(if v1077{((-(((v670*v3698)-(v1091*v2692))/v2695))/v1095)}else{v3186});
        let v3739=(if v1077{((-(v3699/v670))/v1095)}else{v3187});
        let v3740=(if v1077{((-(v3700/v670))/v1095)}else{v60});
        let v3787=(if v1077{((v670*(-(v1103*(self.scalar_static_f64[197]*v3737))))/self.scalar_static_f64[197])}else{v3223});
        let v3788=(if v1077{(((v1104*v2692)+(v670*(-(v1103*(self.scalar_static_f64[197]*v3738)))))/self.scalar_static_f64[197])}else{v3224});
        let v3789=(if v1077{((v670*(-(v1103*(self.scalar_static_f64[197]*v3739))))/self.scalar_static_f64[197])}else{v3225});
        let v3790=(if v1077{((v670*(-(v1103*(self.scalar_static_f64[197]*v3740))))/self.scalar_static_f64[197])}else{v60});
        let v3818=(if v945{(self.scalar_static_f64[184]*v2701)}else{v2796});
        let v3819=(if v1118{v3249}else{v3250});
        let v3820=(if v1118{v3251}else{v3252});
        let v3822=(if v1118{(v782*v3818)}else{v3254});
        let v3828=(if v1118{((v1126*v3818)+(v1116*(v1126*(v1124*v3258))))}else{v3264});
        let v3832=(if v1118{v2813}else{v3268});
        let v3833=(if v1118{((v1129*v2481)+(v446*v3820))}else{v3269});
        let v3834=(if v1118{v2817}else{v3270});
        let v3835=(if v1118{v60}else{v3271});
        let v3840=(if v1133{(v1134*v3832)}else{v3347});
        let v3841=(if v1133{(v1134*v3833)}else{v3348});
        let v3842=(if v1133{(v1134*v3834)}else{v3349});
        let v3843=(if v1133{(v1134*v3835)}else{v3350});
        let v3887=(if v1144{self.scalar_static_f64[0]}else{(if v1133{(-(v444*(v3840/v1136)))}else{v3323})});
        let v3888=(if v1144{v60}else{(if v1133{(v3820-((v1139*v2477)+(v444*(v3841/v1136))))}else{v3324})});
        let v3889=(if v1144{self.scalar_static_f64[247]}else{(if v1133{(-(v444*(v3842/v1136)))}else{v3325})});
        let v3890=(if v1144{v60}else{(if v1133{(-(v444*(v3843/v1136)))}else{v3326})});
        let v3893=(if v1118{(v2864+(v817*v3819))}else{v3329});
        let v3899=(v1149*v1149);
        let v3903=(if v1118{(v3887/v1149)}else{v3339});
        let v3904=(if v1118{(((v1149*(v3819+v3888))-(v1150*v3893))/v3899)}else{v3340});
        let v3905=(if v1118{(v3889/v1149)}else{v3341});
        let v3906=(if v1118{(v3890/v1149)}else{v3342});
        let v3911=(if v1154{(v1155*v3903)}else{v3840});
        let v3912=(if v1154{(v1155*v3904)}else{v3841});
        let v3913=(if v1154{(v1155*v3905)}else{v3842});
        let v3914=(if v1154{(v1155*v3906)}else{v3843});
        let v3964=(if v1171{v3887}else{(if v1154{(v1149*(v3911/v1157))}else{v3400})});
        let v3965=(if v1171{v3888}else{(if v1154{((-v3819)+((v1166*v3893)+(v1149*((v3912/v1157)-(v1165*(((v1149*(-(v3819+v3820)))-(v1163*v3893))/v3899))))))}else{v3401})});
        let v3966=(if v1171{v3889}else{(if v1154{(v1149*(v3913/v1157))}else{v3402})});
        let v3967=(if v1171{v3890}else{(if v1154{(v1149*(v3914/v1157))}else{v3403})});
        let v3972=(if v1118{(self.scalar_static_f64[0]-v3887)}else{v3408});
        let v3973=(if v1118{(-v3888)}else{v3409});
        let v3974=(if v1118{(self.scalar_static_f64[247]-v3889)}else{v3410});
        let v3975=(if v1118{(-v3890)}else{v3411});
        let v3991=(if v1118{((-(v3887/v670))/v1177)}else{v3427});
        let v3992=(if v1118{((-(((v670*v3888)-(v1146*v2692))/v2695))/v1177)}else{v3428});
        let v3993=(if v1118{((-(v3889/v670))/v1177)}else{v3429});
        let v3994=(if v1118{((-(v3890/v670))/v1177)}else{v3430});
        let v4010=(if v1118{((-(v3964/v670))/v1181)}else{v3446});
        let v4011=(if v1118{((-(((v670*v3965)-(v1173*v2692))/v2695))/v1181)}else{v3447});
        let v4012=(if v1118{((-(v3966/v670))/v1181)}else{v3448});
        let v4013=(if v1118{((-(v3967/v670))/v1181)}else{v3449});
        let v4126=(if v1118{((v1116*(-(v1204*(v1184*v4010))))/v1184)}else{v3562});
        let v4127=(if v1118{(((v1205*v3818)+(v1116*(-(v1204*(v1184*v4011)))))/v1184)}else{v3563});
        let v4128=(if v1118{((v1116*(-(v1204*(v1184*v4012))))/v1184)}else{v3564});
        let v4129=(if v1118{((v1116*(-(v1204*(v1184*v4013))))/v1184)}else{v3565});
        let v4152=(if v1118{((v1128*(-(v1210*(v1186*v3991))))/v1186)}else{v3588});
        let v4153=(if v1118{(((v1211*v3828)+(v1128*(-(v1210*(v1186*v3992)))))/v1186)}else{v3589});
        let v4154=(if v1118{((v1128*(-(v1210*(v1186*v3993))))/v1186)}else{v3590});
        let v4155=(if v1118{((v1128*(-(v1210*(v1186*v3994))))/v1186)}else{v3591});
        let v4178=(if v1118{((v1128*(-(v1216*(v1186*v4010))))/v1186)}else{v3614});
        let v4179=(if v1118{(((v1217*v3828)+(v1128*(-(v1216*(v1186*v4011)))))/v1186)}else{v3615});
        let v4180=(if v1118{((v1128*(-(v1216*(v1186*v4012))))/v1186)}else{v3616});
        let v4181=(if v1118{((v1128*(-(v1216*(v1186*v4013))))/v1186)}else{v3617});
        let v4214=(if v1230{v3251}else{v3650});
        let v4218=(if v1230{v2813}else{v3654});
        let v4219=(if v1230{((v1232*v2481)+(v446*v4214))}else{v3655});
        let v4220=(if v1230{v2817}else{v3656});
        let v4221=(if v1230{v60}else{v3657});
        let v4222=(v1234*v4218);
        let v4224=(v1234*v4219);
        let v4226=(v1234*v4220);
        let v4228=(v1234*v4221);
        let v4230=(v91*v1237);
        let v4235=(if v1230{((v4222+v4222)/v4230)}else{v3671});
        let v4236=(if v1230{((v4224+v4224)/v4230)}else{v3672});
        let v4237=(if v1230{((v4226+v4226)/v4230)}else{v3673});
        let v4238=(if v1230{((v4228+v4228)/v4230)}else{v3674});
        let v4247=(if v1230{(v32*(v4218+v4235))}else{v3683});
        let v4248=(if v1230{(v32*(v4219+v4236))}else{v3684});
        let v4249=(if v1230{(v32*(v4220+v4237))}else{v3685});
        let v4250=(if v1230{(v32*(v4221+v4238))}else{v3686});
        let v4261=(if v1230{(-(v444*v4247))}else{v3697});
        let v4262=(if v1230{(v4214-((v1241*v2477)+(v444*v4248)))}else{v3698});
        let v4263=(if v1230{(-(v444*v4249))}else{v3699});
        let v4264=(if v1230{(-(v444*v4250))}else{v3700});
        let v4301=(if v1230{((-(v4261/v670))/v1248)}else{v3737});
        let v4302=(if v1230{((-(((v670*v4262)-(v1244*v2692))/v2695))/v1248)}else{v3738});
        let v4303=(if v1230{((-(v4263/v670))/v1248)}else{v3739});
        let v4304=(if v1230{((-(v4264/v670))/v1248)}else{v3740});
        let v4351=(if v1230{((v670*(-(v1256*(self.scalar_static_f64[197]*v4301))))/self.scalar_static_f64[197])}else{v3787});
        let v4352=(if v1230{(((v1257*v2692)+(v670*(-(v1256*(self.scalar_static_f64[197]*v4302)))))/self.scalar_static_f64[197])}else{v3788});
        let v4353=(if v1230{((v670*(-(v1256*(self.scalar_static_f64[197]*v4303))))/self.scalar_static_f64[197])}else{v3789});
        let v4354=(if v1230{((v670*(-(v1256*(self.scalar_static_f64[197]*v4304))))/self.scalar_static_f64[197])}else{v3790});
        let v4381=(if v1269{v2797}else{v3819});
        let v4382=(if v1269{v2799}else{v3820});
        let v4383=(v782*v3246);
        let v4384=(if v1269{v4383}else{v3822});
        let v4390=(if v1269{((v1277*v3246)+(v946*(v1277*(v1275*v2806))))}else{v3828});
        let v4394=(if v1269{v60}else{v3832});
        let v4395=(if v1269{((v1280*v2481)+(v446*v4382))}else{v3833});
        let v4396=(if v1269{v2817}else{v3834});
        let v4397=(if v1269{v2813}else{v3835});
        let v4402=(if v1284{(v1285*v4394)}else{v3911});
        let v4403=(if v1284{(v1285*v4395)}else{v3912});
        let v4404=(if v1284{(v1285*v4396)}else{v3913});
        let v4405=(if v1284{(v1285*v4397)}else{v3914});
        let v4449=(if v1295{v60}else{(if v1284{(-(v444*(v4402/v1287)))}else{v3887})});
        let v4450=(if v1295{v60}else{(if v1284{(v4382-((v1290*v2477)+(v444*(v4403/v1287))))}else{v3888})});
        let v4451=(if v1295{self.scalar_static_f64[247]}else{(if v1284{(-(v444*(v4404/v1287)))}else{v3889})});
        let v4452=(if v1295{self.scalar_static_f64[0]}else{(if v1284{(-(v444*(v4405/v1287)))}else{v3890})});
        let v4455=(if v1269{(v2864+(v817*v4381))}else{v3893});
        let v4461=(v1300*v1300);
        let v4465=(if v1269{(v4449/v1300)}else{v3903});
        let v4466=(if v1269{(((v1300*(v4381+v4450))-(v1301*v4455))/v4461)}else{v3904});
        let v4467=(if v1269{(v4451/v1300)}else{v3905});
        let v4468=(if v1269{(v4452/v1300)}else{v3906});
        let v4473=(if v1305{(v1306*v4465)}else{v4402});
        let v4474=(if v1305{(v1306*v4466)}else{v4403});
        let v4475=(if v1305{(v1306*v4467)}else{v4404});
        let v4476=(if v1305{(v1306*v4468)}else{v4405});
        let v4526=(if v1322{v4449}else{(if v1305{(v1300*(v4473/v1308))}else{v3964})});
        let v4527=(if v1322{v4450}else{(if v1305{((-v4381)+((v1317*v4455)+(v1300*((v4474/v1308)-(v1316*(((v1300*(-(v4381+v4382)))-(v1314*v4455))/v4461))))))}else{v3965})});
        let v4528=(if v1322{v4451}else{(if v1305{(v1300*(v4475/v1308))}else{v3966})});
        let v4529=(if v1322{v4452}else{(if v1305{(v1300*(v4476/v1308))}else{v3967})});
        let v4534=(if v1269{(-v4449)}else{v3972});
        let v4535=(if v1269{(-v4450)}else{v3973});
        let v4536=(if v1269{(self.scalar_static_f64[247]-v4451)}else{v3974});
        let v4537=(if v1269{(self.scalar_static_f64[0]-v4452)}else{v3975});
        let v4553=(if v1269{((-(v4449/v563))/v1328)}else{v3991});
        let v4554=(if v1269{((-(((v563*v4450)-(v1297*v2591))/v2594))/v1328)}else{v3992});
        let v4555=(if v1269{((-(v4451/v563))/v1328)}else{v3993});
        let v4556=(if v1269{((-(v4452/v563))/v1328)}else{v3994});
        let v4572=(if v1269{((-(v4526/v563))/v1332)}else{v4010});
        let v4573=(if v1269{((-(((v563*v4527)-(v1324*v2591))/v2594))/v1332)}else{v4011});
        let v4574=(if v1269{((-(v4528/v563))/v1332)}else{v4012});
        let v4575=(if v1269{((-(v4529/v563))/v1332)}else{v4013});
        let v4688=(if v1269{((v946*(-(v1355*(v1335*v4572))))/v1335)}else{v4126});
        let v4689=(if v1269{(((v1356*v3246)+(v946*(-(v1355*(v1335*v4573)))))/v1335)}else{v4127});
        let v4690=(if v1269{((v946*(-(v1355*(v1335*v4574))))/v1335)}else{v4128});
        let v4691=(if v1269{((v946*(-(v1355*(v1335*v4575))))/v1335)}else{v4129});
        let v4714=(if v1269{((v1279*(-(v1361*(v1337*v4553))))/v1337)}else{v4152});
        let v4715=(if v1269{(((v1362*v4390)+(v1279*(-(v1361*(v1337*v4554)))))/v1337)}else{v4153});
        let v4716=(if v1269{((v1279*(-(v1361*(v1337*v4555))))/v1337)}else{v4154});
        let v4717=(if v1269{((v1279*(-(v1361*(v1337*v4556))))/v1337)}else{v4155});
        let v4740=(if v1269{((v1279*(-(v1367*(v1337*v4572))))/v1337)}else{v4178});
        let v4741=(if v1269{(((v1368*v4390)+(v1279*(-(v1367*(v1337*v4573)))))/v1337)}else{v4179});
        let v4742=(if v1269{((v1279*(-(v1367*(v1337*v4574))))/v1337)}else{v4180});
        let v4743=(if v1269{((v1279*(-(v1367*(v1337*v4575))))/v1337)}else{v4181});
        let v4776=(if v1381{v2799}else{v4214});
        let v4780=(if v1381{v60}else{v4218});
        let v4781=(if v1381{((v1383*v2481)+(v446*v4776))}else{v4219});
        let v4782=(if v1381{v2817}else{v4220});
        let v4783=(if v1381{v2813}else{v4221});
        let v4784=(v1385*v4780);
        let v4786=(v1385*v4781);
        let v4788=(v1385*v4782);
        let v4790=(v1385*v4783);
        let v4792=(v91*v1388);
        let v4797=(if v1381{((v4784+v4784)/v4792)}else{v4235});
        let v4798=(if v1381{((v4786+v4786)/v4792)}else{v4236});
        let v4799=(if v1381{((v4788+v4788)/v4792)}else{v4237});
        let v4800=(if v1381{((v4790+v4790)/v4792)}else{v4238});
        let v4809=(if v1381{(v32*(v4780+v4797))}else{v4247});
        let v4810=(if v1381{(v32*(v4781+v4798))}else{v4248});
        let v4811=(if v1381{(v32*(v4782+v4799))}else{v4249});
        let v4812=(if v1381{(v32*(v4783+v4800))}else{v4250});
        let v4823=(if v1381{(-(v444*v4809))}else{v4261});
        let v4824=(if v1381{(v4776-((v1392*v2477)+(v444*v4810)))}else{v4262});
        let v4825=(if v1381{(-(v444*v4811))}else{v4263});
        let v4826=(if v1381{(-(v444*v4812))}else{v4264});
        let v4863=(if v1381{((-(v4823/v563))/v1399)}else{v4301});
        let v4864=(if v1381{((-(((v563*v4824)-(v1395*v2591))/v2594))/v1399)}else{v4302});
        let v4865=(if v1381{((-(v4825/v563))/v1399)}else{v4303});
        let v4866=(if v1381{((-(v4826/v563))/v1399)}else{v4304});
        let v4913=(if v1381{((v563*(-(v1407*(self.scalar_static_f64[190]*v4863))))/self.scalar_static_f64[190])}else{v4351});
        let v4914=(if v1381{(((v1408*v2591)+(v563*(-(v1407*(self.scalar_static_f64[190]*v4864)))))/self.scalar_static_f64[190])}else{v4352});
        let v4915=(if v1381{((v563*(-(v1407*(self.scalar_static_f64[190]*v4865))))/self.scalar_static_f64[190])}else{v4353});
        let v4916=(if v1381{((v563*(-(v1407*(self.scalar_static_f64[190]*v4866))))/self.scalar_static_f64[190])}else{v4354});
        let v4939=(if v1417{v60}else{(if v1381{(v946*(v4913+(v782*(-v4823))))}else{(if v1379{v60}else{(if v1269{((v563*((v4688+v4714)-v4740))+(v1274*v4534))}else{v60})})})});
        let v4940=(if v1417{v60}else{(if v1381{((v1414*v3246)+(v946*(v4914+(v782*(-v4824)))))}else{(if v1379{v60}else{(if v1269{(((v1373*v2591)+(v563*((v4689+v4715)-v4741)))+((v1326*v4384)+(v1274*v4535)))}else{v60})})})});
        let v4941=(if v1417{v60}else{(if v1381{(v946*(v4915+(v782*(self.scalar_static_f64[247]-v4825))))}else{(if v1379{v60}else{(if v1269{((v563*((v4690+v4716)-v4742))+(v1274*v4536))}else{v60})})})});
        let v4942=(if v1417{v60}else{(if v1381{(v946*(v4916+(v782*(self.scalar_static_f64[0]-v4826))))}else{(if v1379{v60}else{(if v1269{((v563*((v4691+v4717)-v4743))+(v1274*v4537))}else{v60})})})});
        let v4947=(if v1268{v2799}else{v60});
        let v4951=(if v1268{((v1421*v2481)+(v446*v4947))}else{v60});
        let v4952=(if v1268{v2817}else{v60});
        let v4953=(if v1268{v2813}else{v60});
        let v4954=(v1423*v4951);
        let v4956=(v1423*v4952);
        let v4958=(v1423*v4953);
        let v4960=(v91*v1426);
        let v4964=(if v1268{((v4954+v4954)/v4960)}else{v60});
        let v4965=(if v1268{((v4956+v4956)/v4960)}else{v60});
        let v4966=(if v1268{((v4958+v4958)/v4960)}else{v60});
        let v4973=(if v1268{(v32*(v4951+v4964))}else{v60});
        let v4974=(if v1268{(v32*(v4952+v4965))}else{v60});
        let v4975=(if v1268{(v32*(v4953+v4966))}else{v60});
        let v4990=(v1427*v1427);
        let v5000=(if v1268{(((v1427*v4973)-(v1430*v4964))/v4990)}else{v60});
        let v5001=(if v1268{(((v1427*v4974)-(v1430*v4965))/v4990)}else{v60});
        let v5002=(if v1268{(((v1427*v4975)-(v1430*v4966))/v4990)}else{v60});
        let v5062=((v1451*v2481)+(v446*(if self.scalar_static_bool[5]{(-(if self.scalar_static_bool[16]{v2632}else{(if self.scalar_static_bool[15]{v60}else{(if self.scalar_static_bool[14]{v2632}else{v60})})}))}else{(if self.scalar_static_bool[4]{(if self.scalar_static_bool[16]{v60}else{(if self.scalar_static_bool[15]{(self.scalar_static_f64[83]*(-(self.scalar_static_f64[84]*v2482)))}else{v60})})}else{v60})})));
        let v5063=(v446*self.scalar_static_f64[252]);
        let v5064=(v446*self.scalar_static_f64[253]);
        let v5065=(v446*self.scalar_static_f64[254]);
        let v5066=(v1453*v5062);
        let v5068=(v1453*v5063);
        let v5070=(v1453*v5064);
        let v5072=(v1453*v5065);
        let v5074=(v91*v1456);
        let v5089=((v1459*v2477)+(v444*((v5062+((v5066+v5066)/v5074))/v91)));
        let v5090=(v444*((v5063+((v5068+v5068)/v5074))/v91));
        let v5091=(v444*((v5064+((v5070+v5070)/v5074))/v91));
        let v5092=(v444*((v5065+((v5072+v5072)/v5074))/v91));
        let v5134=(v1470*v1470);
        let v5149=((v5089-v2622)/self.scalar_static_f64[200]);
        let v5150=(v5090/self.scalar_static_f64[200]);
        let v5151=(v5091/self.scalar_static_f64[200]);
        let v5152=(v5092/self.scalar_static_f64[200]);
        let v5153=(v1474*v5149);
        let v5155=(v1474*v5150);
        let v5157=(v1474*v5151);
        let v5159=(v1474*v5152);
        let v5161=(v91*v1478);
        let v5176=((v1481*(((v1470*((v1460*(if self.scalar_static_bool[14]{((-(if self.scalar_static_bool[14]{(self.scalar_static_f64[80]*(v593*(self.scalar_static_f64[22]*v2486)))}else{v60}))/(v595*v595))}else{v60}))+(v597*v5089)))-(v1462*(v1470*(((v1466*(self.scalar_static_f64[199]*((((v591*v5089)-(v1460*v2622))/(v591*v591))/v1461)))/v1467)/self.scalar_static_f64[199]))))/v5134))+(v1471*(v32*(v5149+((v5153+v5153)/v5161)))));
        let v5179=((v1481*(((v1470*(v597*v5090))-(v1462*(v1470*(((v1466*(self.scalar_static_f64[199]*((v5090/v591)/v1461)))/v1467)/self.scalar_static_f64[199]))))/v5134))+(v1471*(v32*(v5150+((v5155+v5155)/v5161)))));
        let v5182=((v1481*(((v1470*(v597*v5091))-(v1462*(v1470*(((v1466*(self.scalar_static_f64[199]*((v5091/v591)/v1461)))/v1467)/self.scalar_static_f64[199]))))/v5134))+(v1471*(v32*(v5151+((v5157+v5157)/v5161)))));
        let v5185=((v1481*(((v1470*(v597*v5092))-(v1462*(v1470*(((v1466*(self.scalar_static_f64[199]*((v5092/v591)/v1461)))/v1467)/self.scalar_static_f64[199]))))/v5134))+(v1471*(v32*(v5152+((v5159+v5159)/v5161)))));
        let v5189=(v1447*v1447);
        let v5212=(if v1489{v60}else{(if v1484{(((v1447*v3246)-(v946*(if v1378{v60}else{(if v1268{(((v1441*v5000)+(v1435*((v1440*v3246)+(v946*(v1440*(self.scalar_static_f64[191]*((-(((v563*(if v1268{(v4947-((v1430*v2477)+(v444*v4973)))}else{v60}))-(v1433*v2591))/v2594))/v1437)))))))+((v1443*v4383)+(v1273*(-v5000))))}else{v60})})))/v5189)}else{v60})});
        let v5213=(if v1489{v60}else{(if v1484{((-(v946*(if v1378{v60}else{(if v1268{(((v1441*v5001)+(v1435*(v946*(v1440*(self.scalar_static_f64[191]*((-((if v1268{(-(v444*v4974))}else{v60})/v563))/v1437))))))+(v1273*(-v5001)))}else{v60})})))/v5189)}else{v60})});
        let v5214=(if v1489{v60}else{(if v1484{((-(v946*(if v1378{v60}else{(if v1268{(((v1441*v5002)+(v1435*(v946*(v1440*(self.scalar_static_f64[191]*((-((if v1268{(-(v444*v4975))}else{v60})/v563))/v1437))))))+(v1273*(-v5002)))}else{v60})})))/v5189)}else{v60})});
        let v5215=(if v1489{v60}else{(if v1484{(v4939/v946)}else{v4939})});
        let v5216=(if v1489{v60}else{(if v1484{(((v946*v4940)-(v1418*v3246))/(v946*v946))}else{v4940})});
        let v5217=(if v1489{v60}else{(if v1484{(v4941/v946)}else{v4941})});
        let v5218=(if v1489{v60}else{(if v1484{(v4942/v946)}else{v4942})});
        let v5227=(if v1492{((v1497*v2516)+(v484*(-(v1496*((-(v2528/v493))/self.scalar_static_f64[46])))))}else{v4776});
        let v5231=(if v1492{v60}else{v4780});
        let v5232=(if v1492{((v1500*v2481)+(v446*v5227))}else{v4781});
        let v5233=(if v1492{v60}else{v4782});
        let v5234=(if v1492{v2813}else{v4783});
        let v5235=(if v1492{v2817}else{v60});
        let v5236=(v1502*v5231);
        let v5238=(v1502*v5232);
        let v5240=(v1502*v5233);
        let v5242=(v1502*v5234);
        let v5244=(v1502*v5235);
        let v5246=(v91*v1505);
        let v5252=(if v1492{((v5236+v5236)/v5246)}else{v4797});
        let v5253=(if v1492{((v5238+v5238)/v5246)}else{v4798});
        let v5254=(if v1492{((v5240+v5240)/v5246)}else{v4799});
        let v5255=(if v1492{((v5242+v5242)/v5246)}else{v4800});
        let v5256=(if v1492{((v5244+v5244)/v5246)}else{v60});
        let v5267=(if v1492{(v32*(v5231+v5252))}else{v4809});
        let v5268=(if v1492{(v32*(v5232+v5253))}else{v4810});
        let v5269=(if v1492{(v32*(v5233+v5254))}else{v4811});
        let v5270=(if v1492{(v32*(v5234+v5255))}else{v4812});
        let v5271=(if v1492{(v32*(v5235+v5256))}else{v60});
        let v5284=(if v1492{(-(v444*v5267))}else{v4823});
        let v5285=(if v1492{(v5227-((v1509*v2477)+(v444*v5268)))}else{v4824});
        let v5286=(if v1492{(-(v444*v5269))}else{v4825});
        let v5287=(if v1492{(-(v444*v5270))}else{v4826});
        let v5288=(if v1492{(-(v444*v5271))}else{v60});
        let v5333=(if v1492{((-(v5284/v484))/v1516)}else{v4863});
        let v5334=(if v1492{((-(((v484*v5285)-(v1512*v2516))/v2519))/v1516)}else{v4864});
        let v5335=(if v1492{((-(v5286/v484))/v1516)}else{v4865});
        let v5336=(if v1492{((-(v5287/v484))/v1516)}else{v4866});
        let v5337=(if v1492{((-(v5288/v484))/v1516)}else{v60});
        let v5395=(if v1492{((v484*(-(v1526*(self.scalar_static_f64[203]*v5333))))/self.scalar_static_f64[203])}else{v4913});
        let v5396=(if v1492{(((v1527*v2516)+(v484*(-(v1526*(self.scalar_static_f64[203]*v5334)))))/self.scalar_static_f64[203])}else{v4914});
        let v5397=(if v1492{((v484*(-(v1526*(self.scalar_static_f64[203]*v5335))))/self.scalar_static_f64[203])}else{v4915});
        let v5398=(if v1492{((v484*(-(v1526*(self.scalar_static_f64[203]*v5336))))/self.scalar_static_f64[203])}else{v4916});
        let v5399=(if v1492{((v484*(-(v1526*(self.scalar_static_f64[203]*v5337))))/self.scalar_static_f64[203])}else{v60});
        let v5429=(if v1536{v60}else{(if v1492{(v490*(v5395+(v493*(-v5284))))}else{v60})});
        let v5430=(if v1536{v60}else{(if v1492{((v1533*v2525)+(v490*(v5396+((v1531*v2528)+(v493*(-v5285))))))}else{v60})});
        let v5431=(if v1536{v60}else{(if v1492{(v490*(v5397+(v493*(-v5286))))}else{v60})});
        let v5432=(if v1536{v60}else{(if v1492{(v490*(v5398+(v493*(self.scalar_static_f64[0]-v5287))))}else{v60})});
        let v5433=(if v1536{v60}else{(if v1492{(v490*(v5399+(v493*(self.scalar_static_f64[247]-v5288))))}else{v60})});
        let v5434=(v5429/v490);
        let v5439=(((v490*v5430)-(v1537*v2525))/(v490*v490));
        let v5440=(v5431/v490);
        let v5441=(v5432/v490);
        let v5442=(v5433/v490);
        let v5451=(if v1540{((v1545*v2547)+(v517*(-(v1544*((-(v2559/v526))/self.scalar_static_f64[57])))))}else{v5227});
        let v5455=(if v1540{v60}else{v5231});
        let v5456=(if v1540{((v1548*v2481)+(v446*v5451))}else{v5232});
        let v5457=(if v1540{v60}else{v5233});
        let v5458=(if v1540{v2813}else{v5234});
        let v5459=(if v1540{v2817}else{v5235});
        let v5460=(v1550*v5455);
        let v5462=(v1550*v5456);
        let v5464=(v1550*v5457);
        let v5466=(v1550*v5458);
        let v5468=(v1550*v5459);
        let v5470=(v91*v1553);
        let v5476=(if v1540{((v5460+v5460)/v5470)}else{v5252});
        let v5477=(if v1540{((v5462+v5462)/v5470)}else{v5253});
        let v5478=(if v1540{((v5464+v5464)/v5470)}else{v5254});
        let v5479=(if v1540{((v5466+v5466)/v5470)}else{v5255});
        let v5480=(if v1540{((v5468+v5468)/v5470)}else{v5256});
        let v5491=(if v1540{(v32*(v5455+v5476))}else{v5267});
        let v5492=(if v1540{(v32*(v5456+v5477))}else{v5268});
        let v5493=(if v1540{(v32*(v5457+v5478))}else{v5269});
        let v5494=(if v1540{(v32*(v5458+v5479))}else{v5270});
        let v5495=(if v1540{(v32*(v5459+v5480))}else{v5271});
        let v5508=(if v1540{(-(v444*v5491))}else{v5284});
        let v5509=(if v1540{(v5451-((v1557*v2477)+(v444*v5492)))}else{v5285});
        let v5510=(if v1540{(-(v444*v5493))}else{v5286});
        let v5511=(if v1540{(-(v444*v5494))}else{v5287});
        let v5512=(if v1540{(-(v444*v5495))}else{v5288});
        let v5557=(if v1540{((-(v5508/v517))/v1564)}else{v5333});
        let v5558=(if v1540{((-(((v517*v5509)-(v1560*v2547))/v2550))/v1564)}else{v5334});
        let v5559=(if v1540{((-(v5510/v517))/v1564)}else{v5335});
        let v5560=(if v1540{((-(v5511/v517))/v1564)}else{v5336});
        let v5561=(if v1540{((-(v5512/v517))/v1564)}else{v5337});
        let v5619=(if v1540{((v517*(-(v1574*(self.scalar_static_f64[205]*v5557))))/self.scalar_static_f64[205])}else{v5395});
        let v5620=(if v1540{(((v1575*v2547)+(v517*(-(v1574*(self.scalar_static_f64[205]*v5558)))))/self.scalar_static_f64[205])}else{v5396});
        let v5621=(if v1540{((v517*(-(v1574*(self.scalar_static_f64[205]*v5559))))/self.scalar_static_f64[205])}else{v5397});
        let v5622=(if v1540{((v517*(-(v1574*(self.scalar_static_f64[205]*v5560))))/self.scalar_static_f64[205])}else{v5398});
        let v5623=(if v1540{((v517*(-(v1574*(self.scalar_static_f64[205]*v5561))))/self.scalar_static_f64[205])}else{v5399});
        let v5678=(if self.scalar_static_bool[11]{v2516}else{(if self.scalar_static_bool[10]{v2547}else{v60})});
        let v5680=(if self.scalar_static_bool[28]{(self.scalar_static_f64[209]*v2477)}else{v60});
        let v5688=(if self.scalar_static_bool[28]{(((v1599*v5678)-(v1600*v5680))/(v1599*v1599))}else{v60});
        let v5689=(if self.scalar_static_bool[28]{(self.scalar_static_f64[247]/v1599)}else{v60});
        let v5690=(if self.scalar_static_bool[28]{(self.scalar_static_f64[0]/v1599)}else{v60});
        let v5691=(v1602*v5688);
        let v5693=(v1602*v5689);
        let v5695=(v1602*v5690);
        let v5697=(v91*v1605);
        let v5745=(if self.scalar_static_bool[28]{((v1616*(if self.scalar_static_bool[14]{(self.scalar_static_f64[121]*(v719*(self.scalar_static_f64[122]*v2486)))}else{v60}))+(v721*(-(v1615*(self.scalar_static_f64[207]*((-(((v1592*(if self.scalar_static_bool[28]{(v5678-(v32*((v1606*v5680)+(v1599*(v5688+((v5691+v5691)/v5697))))))}else{v60}))-(v1610*v5678))/(v1592*v1592)))/v1612))))))}else{v60});
        let v5746=(if self.scalar_static_bool[28]{(v721*(-(v1615*(self.scalar_static_f64[207]*((-((if self.scalar_static_bool[28]{(-(v32*(v1599*(v5689+((v5693+v5693)/v5697)))))}else{v60})/v1592))/v1612)))))}else{v60});
        let v5747=(if self.scalar_static_bool[28]{(v721*(-(v1615*(self.scalar_static_f64[207]*((-((if self.scalar_static_bool[28]{(-(v32*(v1599*(v5690+((v5695+v5695)/v5697)))))}else{v60})/v1592))/v1612)))))}else{v60});
        let v5754=(v1618*v1618);
        let v5801=(v1638*(((v1631*(if self.scalar_static_bool[11]{v5434}else{(if self.scalar_static_bool[10]{((if v1585{v60}else{(if v1540{(v523*(v5619+(v526*(-v5508))))}else{v60})})/v523)}else{v60})}))/v729)+(v5215/self.scalar_static_f64[210])));
        let v5802=(v1638*((((v729*((v1631*(if self.scalar_static_bool[11]{v5439}else{(if self.scalar_static_bool[10]{(((v523*(if v1585{v60}else{(if v1540{((v1581*v2556)+(v523*(v5620+((v1579*v2559)+(v526*(-v5509))))))}else{v60})}))-(v1586*v2556))/(v523*v523))}else{v60})}))+(v1591*(if v1628{(v32*v5745)}else{(if v1622{(((v1618*(v1623*v5745))-(v1624*v5745))/v5754)}else{v60})}))))-(v1632*(if self.scalar_static_bool[14]{((-(self.scalar_static_f64[123]*(v727*((v725*(self.scalar_static_f64[77]*v2481))+(v722*(v724*(self.scalar_static_f64[124]*v2486)))))))/(v727*v727))}else{v60})))/(v729*v729))+(v5216/self.scalar_static_f64[210])));
        let v5803=(v1638*(((v1631*(if self.scalar_static_bool[11]{v5440}else{(if self.scalar_static_bool[10]{((if v1585{v60}else{(if v1540{(v523*(v5621+(v526*(-v5510))))}else{v60})})/v523)}else{v60})}))/v729)+(v5217/self.scalar_static_f64[210])));
        let v5804=(v1638*((((v1631*(if self.scalar_static_bool[11]{v5441}else{(if self.scalar_static_bool[10]{((if v1585{v60}else{(if v1540{(v523*(v5622+(v526*(self.scalar_static_f64[0]-v5511))))}else{v60})})/v523)}else{v60})}))+(v1591*(if v1628{(v32*v5746)}else{(if v1622{(((v1618*(v1623*v5746))-(v1624*v5746))/v5754)}else{v60})})))/v729)+(v5218/self.scalar_static_f64[210])));
        let v5805=(v1638*(((v1631*(if self.scalar_static_bool[11]{v5442}else{(if self.scalar_static_bool[10]{((if v1585{v60}else{(if v1540{(v523*(v5623+(v526*(self.scalar_static_f64[247]-v5512))))}else{v60})})/v523)}else{v60})}))+(v1591*(if v1628{(v32*v5747)}else{(if v1622{(((v1618*(v1623*v5747))-(v1624*v5747))/v5754)}else{v60})})))/v729));
        let v5806=(v1640*v5801);
        let v5808=(v1640*v5802);
        let v5810=(v1640*v5803);
        let v5812=(v1640*v5804);
        let v5814=(v1640*v5805);
        let v5816=(v91*v1644);
        let v5832=(v1641*((v5801+((v5806+v5806)/v5816))/v91));
        let v5833=(v1641*((v5802+((v5808+v5808)/v5816))/v91));
        let v5834=(v1641*((v5803+((v5810+v5810)/v5816))/v91));
        let v5835=(v1641*((v5804+((v5812+v5812)/v5816))/v91));
        let v5836=(v1641*((v5805+((v5814+v5814)/v5816))/v91));
        let v5842=(v1490*v1490);
        let v5851=((v2648+(self.scalar_static_f64[211]*v5212))+(self.scalar_static_f64[212]*((-v5212)/v5842)));
        let v5852=((self.scalar_static_f64[211]*v5213)+(self.scalar_static_f64[212]*((-v5213)/v5842)));
        let v5853=((self.scalar_static_f64[211]*v5214)+(self.scalar_static_f64[212]*((-v5214)/v5842)));
        let v5867=(v1663*v1663);
        let v5878=(if self.scalar_static_bool[30]{v2618}else{(if self.scalar_static_bool[29]{(((v1663*v2618)-(v587*(if self.scalar_static_bool[29]{(((v617*v5851)-(v1657*v2648))/(v617*v617))}else{v60})))/v5867)}else{v60})});
        let v5879=(if self.scalar_static_bool[30]{v60}else{(if self.scalar_static_bool[29]{((-(v587*(if self.scalar_static_bool[29]{(v5852/v617)}else{v60})))/v5867)}else{v60})});
        let v5880=(if self.scalar_static_bool[30]{v60}else{(if self.scalar_static_bool[29]{((-(v587*(if self.scalar_static_bool[29]{(v5853/v617)}else{v60})))/v5867)}else{v60})});
        let v5885=((-(v10*(self.scalar_static_f64[215]*v2477)))/(v1670*v1670));
        let v5886=(self.scalar_static_f64[0]/v1670);
        let v5887=(self.scalar_static_f64[247]/v1670);
        let v5897=scalar_limexp_derivative(v1676);
        let v5912=((v1680*v2612)+(v581*((v1679*(if v1677{v60}else{(if v1672{v5885}else{v60})}))+(v1678*((if v1672{v60}else{v5885})*v5897)))));
        let v5913=(v581*((v1679*(if v1677{v60}else{(if v1672{v5886}else{v60})}))+(v1678*((if v1672{v60}else{v5886})*v5897))));
        let v5914=(v581*((v1679*(if v1677{v60}else{(if v1672{v5887}else{v60})}))+(v1678*((if v1672{v60}else{v5887})*v5897))));
        let v5919=((-(v7*(self.scalar_static_f64[216]*v2477)))/(v1683*v1683));
        let v5920=(self.scalar_static_f64[247]/v1683);
        let v5921=(self.scalar_static_f64[0]/v1683);
        let v5931=scalar_limexp_derivative(v1689);
        let v5946=((v1693*v2612)+(v581*((v1692*(if v1690{v60}else{(if v1685{v5919}else{v60})}))+(v1691*((if v1685{v60}else{v5919})*v5931)))));
        let v5947=(v581*((v1692*(if v1690{v60}else{(if v1685{v5920}else{v60})}))+(v1691*((if v1685{v60}else{v5920})*v5931))));
        let v5948=(v581*((v1692*(if v1690{v60}else{(if v1685{v5921}else{v60})}))+(v1691*((if v1685{v60}else{v5921})*v5931))));
        let v5952=(v1667*v1667);
        let v5961=(v5914/v1667);
        let v5965=((((v1667*v5912)-(v1681*v5878))/v5952)+(v5946/self.scalar_static_f64[214]));
        let v5966=(((-(v1681*v5879))/v5952)+(v5947/self.scalar_static_f64[214]));
        let v5967=((((v1667*v5913)-(v1681*v5880))/v5952)+(v5948/self.scalar_static_f64[214]));
        let v5971=(v1482*v1482);
        let v5997=(v741*v741);
        let v6013=(v1706*(v1699*(((v1702*((v1700*v5912)+(v1681*(((v1482*v5912)-(v1681*v5176))/v5971))))+(v1701*(((v741*v2772)-(v745*v2766))/v5997)))/v1703)));
        let v6014=(v1706*(v1699*((v1702*(v1681*((-(v1681*v5179))/v5971)))/v1703)));
        let v6015=(v1706*(v1699*((v1702*((v1700*v5913)+(v1681*(((v1482*v5913)-(v1681*v5182))/v5971))))/v1703)));
        let v6016=(v1706*(v1699*((v1702*((v1700*v5914)+(v1681*(((v1482*v5914)-(v1681*v5185))/v5971))))/v1703)));
        let v6028=(((v741*v5912)-(v1681*v2766))/v5997);
        let v6029=(v5913/v741);
        let v6030=(v5914/v741);
        let v6031=(v5965+v6028);
        let v6032=(v5967+v6029);
        let v6033=(v5961+v6030);
        let v6047=(v1648*v5832);
        let v6048=(v6047+v6047);
        let v6049=(v1648*v5833);
        let v6050=(v6049+v6049);
        let v6051=(v1648*v5834);
        let v6052=(v6051+v6051);
        let v6053=(v1648*v5835);
        let v6054=(v6053+v6053);
        let v6055=(v1648*v5836);
        let v6056=(v6055+v6055);
        let v6058=((if self.scalar_static_bool[32]{v5966}else{(if self.scalar_static_bool[31]{(v5966+v6014)}else{v60})})+v6052);
        let v6061=(v91*v1718);
        let v6067=(v5832+(v6048/v6061));
        let v6068=(v5833+(((if self.scalar_static_bool[32]{v5965}else{(if self.scalar_static_bool[31]{(v5965+v6013)}else{v60})})+v6050)/v6061));
        let v6069=(v5834+(v6058/v6061));
        let v6070=(v5835+(((if self.scalar_static_bool[32]{v5967}else{(if self.scalar_static_bool[31]{(v5967+v6015)}else{v60})})+v6054)/v6061));
        let v6071=(v5836+(((if self.scalar_static_bool[32]{v5961}else{(if self.scalar_static_bool[31]{(v5961+v6016)}else{v60})})+v6056)/v6061));
        let v6075=(v91*v1721);
        let v6093=(v1681*v1681);
        let v6094=(((v1681*(v5176/self.scalar_static_f64[217]))-(v1728*v5912))/v6093);
        let v6095=((v5179/self.scalar_static_f64[217])/v1681);
        let v6099=(((v1681*(v5182/self.scalar_static_f64[217]))-(v1728*v5913))/v6093);
        let v6103=(((v1681*(v5185/self.scalar_static_f64[217]))-(v1728*v5914))/v6093);
        let v6153=(v1736*v1736);
        let v6171=(if v1726{(((v1736*(if v1726{(-(v1729*v6067))}else{v60}))-(v1732*(if v1726{(v1729*((v5832+(v6048/v6075))-v6067))}else{v60})))/v6153)}else{v60});
        let v6172=(if v1726{(((v1736*(if v1726{(-((v1729*v6068)+(v1719*v6094)))}else{v60}))-(v1732*(if v1726{((v1733*v6094)+(v1729*((v5833+(((if self.scalar_static_bool[32]{v6031}else{(if self.scalar_static_bool[31]{(v6013+v6031)}else{v60})})+v6050)/v6075))-v6068)))}else{v60})))/v6153)}else{v60});
        let v6173=(if v1726{(((v1736*(if v1726{(-((v1729*v6069)+(v1719*v6095)))}else{v60}))-(v1732*(if v1726{((v1733*v6095)+(v1729*((v5834+(v6058/v6075))-v6069)))}else{v60})))/v6153)}else{v60});
        let v6174=(if v1726{(((v1736*(if v1726{(-((v1729*v6070)+(v1719*v6099)))}else{v60}))-(v1732*(if v1726{((v1733*v6099)+(v1729*((v5835+(((if self.scalar_static_bool[32]{v6032}else{(if self.scalar_static_bool[31]{(v6015+v6032)}else{v60})})+v6054)/v6075))-v6070)))}else{v60})))/v6153)}else{v60});
        let v6175=(if v1726{(((v1736*(if v1726{(-((v1729*v6071)+(v1719*v6103)))}else{v60}))-(v1732*(if v1726{((v1733*v6103)+(v1729*((v5836+(((if self.scalar_static_bool[32]{v6033}else{(if self.scalar_static_bool[31]{(v6016+v6033)}else{v60})})+v6056)/v6075))-v6071)))}else{v60})))/v6153)}else{v60});
        let v6176=(v1738*v6171);
        let v6178=(v1738*v6172);
        let v6180=(v1738*v6173);
        let v6182=(v1738*v6174);
        let v6184=(v1738*v6175);
        let v6186=(v91*v1742);
        let v6207=(if v1747{v60}else{(if v1726{((v6171+((v6176+v6176)/v6186))/v1744)}else{v60})});
        let v6208=(if v1747{v60}else{(if v1726{((v6172+((v6178+v6178)/v6186))/v1744)}else{v60})});
        let v6209=(if v1747{v60}else{(if v1726{((v6173+((v6180+v6180)/v6186))/v1744)}else{v60})});
        let v6210=(if v1747{v60}else{(if v1726{((v6174+((v6182+v6182)/v6186))/v1744)}else{v60})});
        let v6211=(if v1747{v60}else{(if v1726{((v6175+((v6184+v6184)/v6186))/v1744)}else{v60})});
        let v6225=((v1752*v6207)+(v1748*(v1709*v6207)));
        let v6238=(v5965+((v1752*v6208)+(v1748*((v1748*v6028)+(v1709*v6208)))));
        let v6239=(v5966+((v1752*v6209)+(v1748*(v1709*v6209))));
        let v6240=(v5967+((v1752*v6210)+(v1748*((v1748*v6029)+(v1709*v6210)))));
        let v6241=(v5961+((v1752*v6211)+(v1748*((v1748*v6030)+(v1709*v6211)))));
        let v6261=(v91*v1760);
        let v6282=(if self.scalar_static_bool[36]{(v1766*v5832)}else{v60});
        let v6283=(if self.scalar_static_bool[36]{(v1766*v5833)}else{v60});
        let v6284=(if self.scalar_static_bool[36]{(v1766*v5834)}else{v60});
        let v6285=(if self.scalar_static_bool[36]{(v1766*v5835)}else{v60});
        let v6286=(if self.scalar_static_bool[36]{(v1766*v5836)}else{v60});
        let v6292=(if self.scalar_static_bool[41]{(-v6225)}else{v60});
        let v6293=(if self.scalar_static_bool[41]{(-v6238)}else{v60});
        let v6294=(if self.scalar_static_bool[41]{(-v6239)}else{v60});
        let v6295=(if self.scalar_static_bool[41]{(-v6240)}else{v60});
        let v6296=(if self.scalar_static_bool[41]{(-v6241)}else{v60});
        let v6341=(v1768*v6282);
        let v6343=(v1768*v6283);
        let v6345=(v1768*v6284);
        let v6347=(v1768*v6285);
        let v6349=(v1768*v6286);
        let v6351=(if self.scalar_static_bool[36]{(v6341+v6341)}else{v60});
        let v6352=(if self.scalar_static_bool[36]{(v6343+v6343)}else{v60});
        let v6353=(if self.scalar_static_bool[36]{(v6345+v6345)}else{v60});
        let v6354=(if self.scalar_static_bool[36]{(v6347+v6347)}else{v60});
        let v6355=(if self.scalar_static_bool[36]{(v6349+v6349)}else{v60});
        let v6366=(if self.scalar_static_bool[36]{(v6292-(self.scalar_static_f64[219]*v6351))}else{v60});
        let v6367=(if self.scalar_static_bool[36]{(v6293-(self.scalar_static_f64[219]*v6352))}else{v60});
        let v6368=(if self.scalar_static_bool[36]{(v6294-(self.scalar_static_f64[219]*v6353))}else{v60});
        let v6369=(if self.scalar_static_bool[36]{(v6295-(self.scalar_static_f64[219]*v6354))}else{v60});
        let v6370=(if self.scalar_static_bool[36]{(v6296-(self.scalar_static_f64[219]*v6355))}else{v60});
        let v6425=(if self.scalar_static_bool[36]{((((v1788*v6351)+(v1784*(v91*v6282)))/v1790)-(self.scalar_static_f64[219]*((v1776*v6282)+(v1768*v6292))))}else{v60});
        let v6426=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{(((v741*((v1779*v2772)+(v745*(((v1482*((v1777*v5912)+(v1681*(-v5912))))-(v1778*v5176))/v5971))))-(v1780*v2766))/v5997)}else{v60})+((((v1788*v6352)+(v1784*(v91*v6283)))/v1790)-(self.scalar_static_f64[219]*((v1776*v6283)+(v1768*v6293)))))}else{v60});
        let v6427=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v745*((-(v1778*v5179))/v5971))/v741)}else{v60})+((((v1788*v6353)+(v1784*(v91*v6284)))/v1790)-(self.scalar_static_f64[219]*((v1776*v6284)+(v1768*v6294)))))}else{v60});
        let v6428=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v745*(((v1482*((v1777*v5913)+(v1681*(-v5913))))-(v1778*v5182))/v5971))/v741)}else{v60})+((((v1788*v6354)+(v1784*(v91*v6285)))/v1790)-(self.scalar_static_f64[219]*((v1776*v6285)+(v1768*v6295)))))}else{v60});
        let v6429=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v745*(((v1482*((v1777*v5914)+(v1681*(-v5914))))-(v1778*v5185))/v5971))/v741)}else{v60})+((((v1788*v6355)+(v1784*(v91*v6286)))/v1790)-(self.scalar_static_f64[219]*((v1776*v6286)+(v1768*v6296)))))}else{v60});
        let v6430=(v1796*v6425);
        let v6432=(v1796*v6426);
        let v6434=(v1796*v6427);
        let v6436=(v1796*v6428);
        let v6438=(v1796*v6429);
        let v6445=(v1787*v6366);
        let v6447=(v1787*v6367);
        let v6449=(v1787*v6368);
        let v6451=(v1787*v6369);
        let v6453=(v1787*v6370);
        let v6457=((v1800*v6366)+(v1787*(v6445+v6445)));
        let v6460=((v1800*v6367)+(v1787*(v6447+v6447)));
        let v6463=((v1800*v6368)+(v1787*(v6449+v6449)));
        let v6466=((v1800*v6369)+(v1787*(v6451+v6451)));
        let v6469=((v1800*v6370)+(v1787*(v6453+v6453)));
        let v6510=(self.scalar_static_f64[219]*v6282);
        let v6511=(self.scalar_static_f64[219]*v6283);
        let v6512=(self.scalar_static_f64[219]*v6284);
        let v6513=(self.scalar_static_f64[219]*v6285);
        let v6514=(self.scalar_static_f64[219]*v6286);
        let v6530=(v32*(-v6425));
        let v6531=(v32*(-v6426));
        let v6532=(v32*(-v6427));
        let v6533=(v32*(-v6428));
        let v6534=(v32*(-v6429));
        let v6535=(if v1817{v6530}else{v60});
        let v6536=(if v1817{v6531}else{v60});
        let v6537=(if v1817{v6532}else{v60});
        let v6538=(if v1817{v6533}else{v60});
        let v6539=(if v1817{v6534}else{v60});
        let v6540=(v91*v1821);
        let v6546=(if v1817{((if self.scalar_static_bool[36]{((v1798*(v6430+v6430))+(v6457/v1790))}else{v60})/v6540)}else{v60});
        let v6547=(if v1817{((if self.scalar_static_bool[36]{((v1798*(v6432+v6432))+(v6460/v1790))}else{v60})/v6540)}else{v60});
        let v6548=(if v1817{((if self.scalar_static_bool[36]{((v1798*(v6434+v6434))+(v6463/v1790))}else{v60})/v6540)}else{v60});
        let v6549=(if v1817{((if self.scalar_static_bool[36]{((v1798*(v6436+v6436))+(v6466/v1790))}else{v60})/v6540)}else{v60});
        let v6550=(if v1817{((if self.scalar_static_bool[36]{((v1798*(v6438+v6438))+(v6469/v1790))}else{v60})/v6540)}else{v60});
        let v6556=(if v1817{(v6535+v6546)}else{v6351});
        let v6557=(if v1817{(v6536+v6547)}else{v6352});
        let v6558=(if v1817{(v6537+v6548)}else{v6353});
        let v6559=(if v1817{(v6538+v6549)}else{v6354});
        let v6560=(if v1817{(v6539+v6550)}else{v6355});
        let v6616=(if v1817{(v6535-v6546)}else{v6556});
        let v6617=(if v1817{(v6536-v6547)}else{v6557});
        let v6618=(if v1817{(v6537-v6548)}else{v6558});
        let v6619=(if v1817{(v6538-v6549)}else{v6559});
        let v6620=(if v1817{(v6539-v6550)}else{v6560});
        let v6688=(v1801*v1801);
        let v6702=(v91*v1862);
        let v6723=(if v1859{((v1862*v6530)+(v1819*(((-(v1860*v6457))/v6688)/v6702)))}else{v6616});
        let v6724=(if v1859{((v1862*v6531)+(v1819*(((-(v1860*v6460))/v6688)/v6702)))}else{v6617});
        let v6725=(if v1859{((v1862*v6532)+(v1819*(((-(v1860*v6463))/v6688)/v6702)))}else{v6618});
        let v6726=(if v1859{((v1862*v6533)+(v1819*(((-(v1860*v6466))/v6688)/v6702)))}else{v6619});
        let v6727=(if v1859{((v1862*v6534)+(v1819*(((-(v1860*v6469))/v6688)/v6702)))}else{v6620});
        let v6728=(v1864*v6723);
        let v6730=(v1864*v6724);
        let v6732=(v1864*v6725);
        let v6734=(v1864*v6726);
        let v6736=(v1864*v6727);
        let v6738=(if v1859{(v6728+v6728)}else{v6535});
        let v6739=(if v1859{(v6730+v6730)}else{v6536});
        let v6740=(if v1859{(v6732+v6732)}else{v6537});
        let v6741=(if v1859{(v6734+v6734)}else{v6538});
        let v6742=(if v1859{(v6736+v6736)}else{v6539});
        let v6751=(v1870*v1870);
        let v6769=(v91*v1872);
        let v6776=(v48+(v1872*v1872));
        let v6777=(((((v1870*v6738)-(v1866*(-v6738)))/v6751)/v6769)/v6776);
        let v6778=(((((v1870*v6739)-(v1866*(-v6739)))/v6751)/v6769)/v6776);
        let v6779=(((((v1870*v6740)-(v1866*(-v6740)))/v6751)/v6769)/v6776);
        let v6780=(((((v1870*v6741)-(v1866*(-v6741)))/v6751)/v6769)/v6776);
        let v6781=(((((v1870*v6742)-(v1866*(-v6742)))/v6751)/v6769)/v6776);
        let v6792=(if v1877{v6777}else{(if v1868{(-v6777)}else{v6723})});
        let v6793=(if v1877{v6778}else{(if v1868{(-v6778)}else{v6724})});
        let v6794=(if v1877{v6779}else{(if v1868{(-v6779)}else{v6725})});
        let v6795=(if v1877{v6780}else{(if v1868{(-v6780)}else{v6726})});
        let v6796=(if v1877{v6781}else{(if v1868{(-v6781)}else{v6727})});
        let v6807=(v91*v1883);
        let v6818=(v1884).sin();
        let v6864=(if v1892{v60}else{(if self.scalar_static_bool[36]{(if v1859{(if v1859{(((v1885*((self.scalar_static_f64[219]*(v1880*v6366))/v6807))+(v1883*(-((self.scalar_static_f64[219]*v6792)*v6818))))-v6510)}else{v6792})}else{(if v1817{(((if v1832{(-(v1836*(self.scalar_static_f64[219]*((-v6556)/v1833))))}else{(if v1826{(v1829*(self.scalar_static_f64[219]*(v6556/v1824)))}else{v60})})+(if v1848{(-(v1852*(self.scalar_static_f64[219]*((-v6616)/v1849))))}else{(if v1842{(v1845*(self.scalar_static_f64[219]*(v6616/v1840)))}else{v60})}))-v6510)}else{(if v1808{((((v1787*(v43*v6425))-(v1809*v6366))/v1800)-v6510)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v5832+((v6048+(if self.scalar_static_bool[35]{v6225}else{(if self.scalar_static_bool[34]{v6225}else{v60})}))/v6261))}else{v60})})});
        let v6865=(if v1892{v60}else{(if self.scalar_static_bool[36]{(if v1859{(if v1859{(((v1885*((self.scalar_static_f64[219]*(v1880*v6367))/v6807))+(v1883*(-((self.scalar_static_f64[219]*v6793)*v6818))))-v6511)}else{v6793})}else{(if v1817{(((if v1832{(-(v1836*(self.scalar_static_f64[219]*((-v6557)/v1833))))}else{(if v1826{(v1829*(self.scalar_static_f64[219]*(v6557/v1824)))}else{v60})})+(if v1848{(-(v1852*(self.scalar_static_f64[219]*((-v6617)/v1849))))}else{(if v1842{(v1845*(self.scalar_static_f64[219]*(v6617/v1840)))}else{v60})}))-v6511)}else{(if v1808{((((v1787*(v43*v6426))-(v1809*v6367))/v1800)-v6511)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v5833+((v6050+(if self.scalar_static_bool[35]{v6238}else{(if self.scalar_static_bool[34]{(v6013+v6238)}else{v60})}))/v6261))}else{v60})})});
        let v6866=(if v1892{v60}else{(if self.scalar_static_bool[36]{(if v1859{(if v1859{(((v1885*((self.scalar_static_f64[219]*(v1880*v6368))/v6807))+(v1883*(-((self.scalar_static_f64[219]*v6794)*v6818))))-v6512)}else{v6794})}else{(if v1817{(((if v1832{(-(v1836*(self.scalar_static_f64[219]*((-v6558)/v1833))))}else{(if v1826{(v1829*(self.scalar_static_f64[219]*(v6558/v1824)))}else{v60})})+(if v1848{(-(v1852*(self.scalar_static_f64[219]*((-v6618)/v1849))))}else{(if v1842{(v1845*(self.scalar_static_f64[219]*(v6618/v1840)))}else{v60})}))-v6512)}else{(if v1808{((((v1787*(v43*v6427))-(v1809*v6368))/v1800)-v6512)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v5834+((v6052+(if self.scalar_static_bool[35]{v6239}else{(if self.scalar_static_bool[34]{(v6014+v6239)}else{v60})}))/v6261))}else{v60})})});
        let v6867=(if v1892{v60}else{(if self.scalar_static_bool[36]{(if v1859{(if v1859{(((v1885*((self.scalar_static_f64[219]*(v1880*v6369))/v6807))+(v1883*(-((self.scalar_static_f64[219]*v6795)*v6818))))-v6513)}else{v6795})}else{(if v1817{(((if v1832{(-(v1836*(self.scalar_static_f64[219]*((-v6559)/v1833))))}else{(if v1826{(v1829*(self.scalar_static_f64[219]*(v6559/v1824)))}else{v60})})+(if v1848{(-(v1852*(self.scalar_static_f64[219]*((-v6619)/v1849))))}else{(if v1842{(v1845*(self.scalar_static_f64[219]*(v6619/v1840)))}else{v60})}))-v6513)}else{(if v1808{((((v1787*(v43*v6428))-(v1809*v6369))/v1800)-v6513)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v5835+((v6054+(if self.scalar_static_bool[35]{v6240}else{(if self.scalar_static_bool[34]{(v6015+v6240)}else{v60})}))/v6261))}else{v60})})});
        let v6868=(if v1892{v60}else{(if self.scalar_static_bool[36]{(if v1859{(if v1859{(((v1885*((self.scalar_static_f64[219]*(v1880*v6370))/v6807))+(v1883*(-((self.scalar_static_f64[219]*v6796)*v6818))))-v6514)}else{v6796})}else{(if v1817{(((if v1832{(-(v1836*(self.scalar_static_f64[219]*((-v6560)/v1833))))}else{(if v1826{(v1829*(self.scalar_static_f64[219]*(v6560/v1824)))}else{v60})})+(if v1848{(-(v1852*(self.scalar_static_f64[219]*((-v6620)/v1849))))}else{(if v1842{(v1845*(self.scalar_static_f64[219]*(v6620/v1840)))}else{v60})}))-v6514)}else{(if v1808{((((v1787*(v43*v6429))-(v1809*v6370))/v1800)-v6514)}else{v60})})})}else{(if self.scalar_static_bool[33]{(v5836+((v6056+(if self.scalar_static_bool[35]{v6241}else{(if self.scalar_static_bool[34]{(v6016+v6241)}else{v60})}))/v6261))}else{v60})})});
        let v6871=(v1893*v1893);
        let v6890=((-(v1694*v6864))/v6871);
        let v6894=(((v1893*v5946)-(v1694*v6865))/v6871);
        let v6898=(((v1893*v5947)-(v1694*v6866))/v6871);
        let v6902=(((v1893*v5948)-(v1694*v6867))/v6871);
        let v6905=((-(v1694*v6868))/v6871);
        let v6906=(if v1896{v60}else{((-(v1681*v6864))/v6871)});
        let v6907=(if v1896{v60}else{(((v1893*v5912)-(v1681*v6865))/v6871)});
        let v6908=(if v1896{v60}else{((-(v1681*v6866))/v6871)});
        let v6909=(if v1896{v60}else{(((v1893*v5913)-(v1681*v6867))/v6871)});
        let v6910=(if v1896{v60}else{(((v1893*v5914)-(v1681*v6868))/v6871)});
        let v6929=(v1897*v1897);
        let v6931=(v1897*v5176);
        let v6932=(v1482*v6907);
        let v6935=(v1897*v5179);
        let v6936=(v1482*v6908);
        let v6939=(v1897*v5182);
        let v6940=(v1482*v6909);
        let v6943=(v1897*v5185);
        let v6944=(v1482*v6910);
        let v6947=(-((-(v1482*v6906))/v6929));
        let v6948=(-((v6931-v6932)/v6929));
        let v6949=(-((v6935-v6936)/v6929));
        let v6950=(-((v6939-v6940)/v6929));
        let v6951=(-((v6943-v6944)/v6929));
        let v6952=(v1901*v6947);
        let v6954=(v1901*v6948);
        let v6956=(v1901*v6949);
        let v6958=(v1901*v6950);
        let v6960=(v1901*v6951);
        let v6962=(v91*v1905);
        let v6973=((v6947+((v6952+v6952)/v6962))/self.scalar_static_f64[223]);
        let v6974=((v6948+((v6954+v6954)/v6962))/self.scalar_static_f64[223]);
        let v6975=((v6949+((v6956+v6956)/v6962))/self.scalar_static_f64[223]);
        let v6976=((v6950+((v6958+v6958)/v6962))/self.scalar_static_f64[223]);
        let v6977=((v6951+((v6960+v6960)/v6962))/self.scalar_static_f64[223]);
        let v7071=(((v1912*v6906)+(v1897*((v1911*v6973)+(v1910*(v630*v6973)))))+((v1657*v6906)+(((v1919*v6906)+(v1897*(v626*(v1918*(self.scalar_static_f64[224]*((v6906/v1482)/v1915))))))/self.scalar_static_f64[225])));
        let v7072=(((v1912*v6907)+(v1897*((v1911*v6974)+(v1910*((v1910*(if self.scalar_static_bool[14]{(self.scalar_static_f64[90]*(v628*(self.scalar_static_f64[91]*v2486)))}else{v60}))+(v630*v6974))))))+(((v1897*v5851)+(v1657*v6907))+(((v1919*v6907)+(v1897*((v1918*(if self.scalar_static_bool[18]{v60}else{(if self.scalar_static_bool[17]{(self.scalar_static_f64[89]*(v622*((self.scalar_static_f64[28]*v2486)-v2650)))}else{v60})}))+(v626*(v1918*(self.scalar_static_f64[224]*(((v6932-v6931)/v5971)/v1915)))))))/self.scalar_static_f64[225])));
        let v7073=(((v1912*v6908)+(v1897*((v1911*v6975)+(v1910*(v630*v6975)))))+(((v1897*v5852)+(v1657*v6908))+(((v1919*v6908)+(v1897*(v626*(v1918*(self.scalar_static_f64[224]*(((v6936-v6935)/v5971)/v1915))))))/self.scalar_static_f64[225])));
        let v7074=(((v1912*v6909)+(v1897*((v1911*v6976)+(v1910*(v630*v6976)))))+(((v1897*v5853)+(v1657*v6909))+(((v1919*v6909)+(v1897*(v626*(v1918*(self.scalar_static_f64[224]*(((v6940-v6939)/v5971)/v1915))))))/self.scalar_static_f64[225])));
        let v7075=(((v1912*v6910)+(v1897*((v1911*v6977)+(v1910*(v630*v6977)))))+((v1657*v6910)+(((v1919*v6910)+(v1897*(v626*(v1918*(self.scalar_static_f64[224]*(((v6944-v6943)/v5971)/v1915))))))/self.scalar_static_f64[225])));
        let v7227=(if v1993{v2797}else{v4381});
        let v7228=(if v1993{v2799}else{v4382});
        let v7230=(if v1993{(v782*v2600)}else{v4384});
        let v7236=(if v1993{((v2001*v2600)+(v569*(v2001*(v1999*v2806))))}else{v4390});
        let v7240=(if v1993{v60}else{v4394});
        let v7241=(if v1993{((v2004*v2481)+(v446*v7228))}else{v4395});
        let v7242=(if v1993{v2817}else{v4396});
        let v7243=(if v1993{v2813}else{v4397});
        let v7248=(if v2008{(v2009*v7240)}else{v4473});
        let v7249=(if v2008{(v2009*v7241)}else{v4474});
        let v7250=(if v2008{(v2009*v7242)}else{v4475});
        let v7251=(if v2008{(v2009*v7243)}else{v4476});
        let v7295=(if v2019{v60}else{(if v2008{(-(v444*(v7248/v2011)))}else{v4449})});
        let v7296=(if v2019{v60}else{(if v2008{(v7228-((v2014*v2477)+(v444*(v7249/v2011))))}else{v4450})});
        let v7297=(if v2019{self.scalar_static_f64[247]}else{(if v2008{(-(v444*(v7250/v2011)))}else{v4451})});
        let v7298=(if v2019{self.scalar_static_f64[0]}else{(if v2008{(-(v444*(v7251/v2011)))}else{v4452})});
        let v7301=(if v1993{(v2864+(v817*v7227))}else{v4455});
        let v7307=(v2024*v2024);
        let v7311=(if v1993{(v7295/v2024)}else{v4465});
        let v7312=(if v1993{(((v2024*(v7227+v7296))-(v2025*v7301))/v7307)}else{v4466});
        let v7313=(if v1993{(v7297/v2024)}else{v4467});
        let v7314=(if v1993{(v7298/v2024)}else{v4468});
        let v7319=(if v2029{(v2030*v7311)}else{v7248});
        let v7320=(if v2029{(v2030*v7312)}else{v7249});
        let v7321=(if v2029{(v2030*v7313)}else{v7250});
        let v7322=(if v2029{(v2030*v7314)}else{v7251});
        let v7372=(if v2046{v7295}else{(if v2029{(v2024*(v7319/v2032))}else{v4526})});
        let v7373=(if v2046{v7296}else{(if v2029{((-v7227)+((v2041*v7301)+(v2024*((v7320/v2032)-(v2040*(((v2024*(-(v7227+v7228)))-(v2038*v7301))/v7307))))))}else{v4527})});
        let v7374=(if v2046{v7297}else{(if v2029{(v2024*(v7321/v2032))}else{v4528})});
        let v7375=(if v2046{v7298}else{(if v2029{(v2024*(v7322/v2032))}else{v4529})});
        let v7399=(if v1993{((-(v7295/v563))/v2052)}else{v4553});
        let v7400=(if v1993{((-(((v563*v7296)-(v2021*v2591))/v2594))/v2052)}else{v4554});
        let v7401=(if v1993{((-(v7297/v563))/v2052)}else{v4555});
        let v7402=(if v1993{((-(v7298/v563))/v2052)}else{v4556});
        let v7418=(if v1993{((-(v7372/v563))/v2056)}else{v4572});
        let v7419=(if v1993{((-(((v563*v7373)-(v2048*v2591))/v2594))/v2056)}else{v4573});
        let v7420=(if v1993{((-(v7374/v563))/v2056)}else{v4574});
        let v7421=(if v1993{((-(v7375/v563))/v2056)}else{v4575});
        let v7606=(if v2102{v2799}else{v5451});
        let v7610=(if v2102{v60}else{v5455});
        let v7611=(if v2102{((v2104*v2481)+(v446*v7606))}else{v5456});
        let v7612=(if v2102{v2817}else{v5457});
        let v7613=(if v2102{v2813}else{v5458});
        let v7614=(if v2102{v60}else{v5459});
        let v7615=(v2106*v7610);
        let v7617=(v2106*v7611);
        let v7619=(v2106*v7612);
        let v7621=(v2106*v7613);
        let v7623=(v2106*v7614);
        let v7625=(v91*v2109);
        let v7631=(if v2102{((v7615+v7615)/v7625)}else{v5476});
        let v7632=(if v2102{((v7617+v7617)/v7625)}else{v5477});
        let v7633=(if v2102{((v7619+v7619)/v7625)}else{v5478});
        let v7634=(if v2102{((v7621+v7621)/v7625)}else{v5479});
        let v7635=(if v2102{((v7623+v7623)/v7625)}else{v5480});
        let v7646=(if v2102{(v32*(v7610+v7631))}else{v5491});
        let v7647=(if v2102{(v32*(v7611+v7632))}else{v5492});
        let v7648=(if v2102{(v32*(v7612+v7633))}else{v5493});
        let v7649=(if v2102{(v32*(v7613+v7634))}else{v5494});
        let v7650=(if v2102{(v32*(v7614+v7635))}else{v5495});
        let v7663=(if v2102{(-(v444*v7646))}else{v5508});
        let v7664=(if v2102{(v7606-((v2113*v2477)+(v444*v7647)))}else{v5509});
        let v7665=(if v2102{(-(v444*v7648))}else{v5510});
        let v7666=(if v2102{(-(v444*v7649))}else{v5511});
        let v7667=(if v2102{(-(v444*v7650))}else{v5512});
        let v7712=(if v2102{((-(v7663/v563))/v2120)}else{v5557});
        let v7713=(if v2102{((-(((v563*v7664)-(v2116*v2591))/v2594))/v2120)}else{v5558});
        let v7714=(if v2102{((-(v7665/v563))/v2120)}else{v5559});
        let v7715=(if v2102{((-(v7666/v563))/v2120)}else{v5560});
        let v7716=(if v2102{((-(v7667/v563))/v2120)}else{v5561});
        let v8300=(if v2257{(-v2721)}else{v7227});
        let v8301=(self.scalar_static_f64[239]*v2721);
        let v8302=(if v2257{v8301}else{v7228});
        let v8314=(if v2257{((v2273*v2730)+(v707*(v2273*(v2269*(((-(self.scalar_static_f64[235]*v2721))/v2724)/v2270)))))}else{v7236});
        let v8328=(if v2280{(v2281*(if v2257{v60}else{v7240}))}else{v7319});
        let v8329=(if v2280{(v2281*(if v2257{v2813}else{v60}))}else{v60});
        let v8330=(if v2280{(v2281*(if v2257{((v2276*v2481)+(v446*v8302))}else{v7241}))}else{v7320});
        let v8331=(if v2280{(v2281*(if v2257{v2817}else{v7242}))}else{v7321});
        let v8332=(if v2280{(v2281*(if v2257{v60}else{v7243}))}else{v7322});
        let v8355=(if v2289{v60}else{(if v2280{(-(v444*(v8328/v2283)))}else{v7295})});
        let v8356=(if v2289{self.scalar_static_f64[0]}else{(if v2280{(-(v444*(v8329/v2283)))}else{v60})});
        let v8357=(if v2289{v60}else{(if v2280{(v8302-((v2284*v2477)+(v444*(v8330/v2283))))}else{v7296})});
        let v8358=(if v2289{self.scalar_static_f64[247]}else{(if v2280{(-(v444*(v8331/v2283)))}else{v7297})});
        let v8359=(if v2289{v60}else{(if v2280{(-(v444*(v8332/v2283)))}else{v7298})});
        let v8362=(if v2257{(v2864+(v817*v8300))}else{v7301});
        let v8369=(v2293*v2293);
        let v8471=(if v2257{((-((if v2313{v8355}else{(if v2298{(v2293*((if v2298{(v2299*(if v2257{(v8355/v2293)}else{v7311}))}else{v8328})/v2301))}else{v7372})})/v701))/v2322)}else{v7418});
        let v8472=(if v2257{((-((if v2313{v8356}else{(if v2298{(v2293*((if v2298{(v2299*(if v2257{(v8356/v2293)}else{v60}))}else{v8329})/v2301))}else{v60})})/v701))/v2322)}else{v60});
        let v8473=(if v2257{((-(((v701*(if v2313{v8357}else{(if v2298{((-v8300)+((v2308*v8362)+(v2293*(((if v2298{(v2299*(if v2257{(((v2293*(v8300+v8357))-(v2294*v8362))/v8369)}else{v7312}))}else{v8330})/v2301)-(v2307*(((v2293*(-(v8300+v8302)))-(v2305*v8362))/v8369))))))}else{v7373})}))-(v2314*v2721))/v2724))/v2322)}else{v7419});
        let v8474=(if v2257{((-((if v2313{v8358}else{(if v2298{(v2293*((if v2298{(v2299*(if v2257{(v8358/v2293)}else{v7313}))}else{v8331})/v2301))}else{v7374})})/v701))/v2322)}else{v7420});
        let v8475=(if v2257{((-((if v2313{v8359}else{(if v2298{(v2293*((if v2298{(v2299*(if v2257{(v8359/v2293)}else{v7314}))}else{v8332})/v2301))}else{v7375})})/v701))/v2322)}else{v7421});
        let v8598=(((v2348*v2721)+(v701*(((if v2257{(((v2331*v2730)+(v707*(-(v2330*(v2326*v8473)))))/v2326)}else{(if v1993{(((v2083*v2600)+(v569*(-(v2082*(v2059*v7419)))))/v2059)}else{v4689})})+(if v2257{(((v2337*v8314)+(v2275*(-(v2336*(v2328*(if v2257{((-(((v701*v8357)-(v2290*v2721))/v2724))/v2318)}else{v7400}))))))/v2328)}else{(if v1993{(((v2089*v7236)+(v2003*(-(v2088*(v2061*v7400)))))/v2061)}else{v4715})}))-(if v2257{(((v2343*v8314)+(v2275*(-(v2342*(v2328*v8473)))))/v2328)}else{(if v1993{(((v2095*v7236)+(v2003*(-(v2094*(v2061*v7419)))))/v2061)}else{v4741})}))))+((v2316*(if v2257{(v782*v2730)}else{v7230}))+(v2268*(if v2257{(-v8357)}else{(if v1993{(-v7296)}else{v4535})}))));
        let v8611=(if v2357{v8301}else{v7606});
        let v8615=(if v2357{v60}else{v7610});
        let v8616=(if v2357{v2813}else{v60});
        let v8617=(if v2357{((v2359*v2481)+(v446*v8611))}else{v7611});
        let v8618=(if v2357{v2817}else{v7612});
        let v8619=(if v2357{v60}else{v7613});
        let v8620=(if v2357{v60}else{v7614});
        let v8621=(v2361*v8615);
        let v8623=(v2361*v8616);
        let v8625=(v2361*v8617);
        let v8627=(v2361*v8618);
        let v8629=(v2361*v8619);
        let v8631=(v2361*v8620);
        let v8633=(v91*v2364);
        let v8678=(if v2357{(-(v444*(if v2357{(v32*(v8615+(if v2357{((v8621+v8621)/v8633)}else{v7631})))}else{v7646})))}else{v7663});
        let v8679=(if v2357{(-(v444*(if v2357{(v32*(v8616+(if v2357{((v8623+v8623)/v8633)}else{v60})))}else{v60})))}else{v60});
        let v8680=(if v2357{(v8611-((v2368*v2477)+(v444*(if v2357{(v32*(v8617+(if v2357{((v8625+v8625)/v8633)}else{v7632})))}else{v7647}))))}else{v7664});
        let v8681=(if v2357{(-(v444*(if v2357{(v32*(v8618+(if v2357{((v8627+v8627)/v8633)}else{v7633})))}else{v7648})))}else{v7665});
        let v8682=(if v2357{(-(v444*(if v2357{(v32*(v8619+(if v2357{((v8629+v8629)/v8633)}else{v7634})))}else{v7649})))}else{v7666});
        let v8683=(if v2357{(-(v444*(if v2357{(v32*(v8620+(if v2357{((v8631+v8631)/v8633)}else{v7635})))}else{v7650})))}else{v7667});
        let v8775=(if v2357{(v707*((if v2357{((v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(v8678/v701))/v2373)}else{v7712})))))/self.scalar_static_f64[240])}else{(if v2102{((v563*(-(v2133*(self.scalar_static_f64[190]*v7712))))/self.scalar_static_f64[190])}else{v5619})})+(v782*(-v8678))))}else{(if v2354{v60}else{(if v2257{((v701*(((if v2257{((v707*(-(v2330*(v2326*v8471))))/v2326)}else{(if v1993{((v569*(-(v2082*(v2059*v7418))))/v2059)}else{v4688})})+(if v2257{((v2275*(-(v2336*(v2328*(if v2257{((-(v8355/v701))/v2318)}else{v7399})))))/v2328)}else{(if v1993{((v2003*(-(v2088*(v2061*v7399))))/v2061)}else{v4714})}))-(if v2257{((v2275*(-(v2342*(v2328*v8471))))/v2328)}else{(if v1993{((v2003*(-(v2094*(v2061*v7418))))/v2061)}else{v4740})})))+(v2268*(if v2257{(-v8355)}else{(if v1993{(-v7295)}else{v4534})})))}else{v60})})});
        let v8778=(if v2357{(v707*((if v2357{((v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(v8681/v701))/v2373)}else{v7714})))))/self.scalar_static_f64[240])}else{(if v2102{((v563*(-(v2133*(self.scalar_static_f64[190]*v7714))))/self.scalar_static_f64[190])}else{v5621})})+(v782*(self.scalar_static_f64[247]-v8681))))}else{(if v2354{v60}else{(if v2257{((v701*(((if v2257{((v707*(-(v2330*(v2326*v8474))))/v2326)}else{(if v1993{((v569*(-(v2082*(v2059*v7420))))/v2059)}else{v4690})})+(if v2257{((v2275*(-(v2336*(v2328*(if v2257{((-(v8358/v701))/v2318)}else{v7401})))))/v2328)}else{(if v1993{((v2003*(-(v2088*(v2061*v7401))))/v2061)}else{v4716})}))-(if v2257{((v2275*(-(v2342*(v2328*v8474))))/v2328)}else{(if v1993{((v2003*(-(v2094*(v2061*v7420))))/v2061)}else{v4742})})))+(v2268*(if v2257{(self.scalar_static_f64[247]-v8358)}else{(if v1993{(self.scalar_static_f64[247]-v7297)}else{v4536})})))}else{v60})})});
        let v8779=(if v2357{(v707*((if v2357{((v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(v8682/v701))/v2373)}else{v7715})))))/self.scalar_static_f64[240])}else{(if v2102{((v563*(-(v2133*(self.scalar_static_f64[190]*v7715))))/self.scalar_static_f64[190])}else{v5622})})+(v782*(-v8682))))}else{(if v2354{v60}else{(if v2257{((v701*(((if v2257{((v707*(-(v2330*(v2326*v8475))))/v2326)}else{(if v1993{((v569*(-(v2082*(v2059*v7421))))/v2059)}else{v4691})})+(if v2257{((v2275*(-(v2336*(v2328*(if v2257{((-(v8359/v701))/v2318)}else{v7402})))))/v2328)}else{(if v1993{((v2003*(-(v2088*(v2061*v7402))))/v2061)}else{v4717})}))-(if v2257{((v2275*(-(v2342*(v2328*v8475))))/v2328)}else{(if v1993{((v2003*(-(v2094*(v2061*v7421))))/v2061)}else{v4743})})))+(v2268*(if v2257{(-v8359)}else{(if v1993{(self.scalar_static_f64[0]-v7298)}else{v4537})})))}else{v60})})});
        let v8817=(if self.scalar_static_bool[58]{v60}else{v7071});
        let v8818=(if self.scalar_static_bool[58]{v60}else{v7072});
        let v8819=(if self.scalar_static_bool[58]{v60}else{v7073});
        let v8820=(if self.scalar_static_bool[58]{v60}else{v7074});
        let v8821=(if self.scalar_static_bool[58]{v60}else{v7075});
        let v8852=(if self.scalar_static_bool[58]{v60}else{v6906});
        let v8853=(if self.scalar_static_bool[58]{v60}else{v6907});
        let v8854=(if self.scalar_static_bool[58]{v60}else{v6908});
        let v8855=(if self.scalar_static_bool[58]{v60}else{v6909});
        let v8856=(if self.scalar_static_bool[58]{v60}else{v6910});
        let v8891=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[242]*v8817))}else{v60})});
        let v8892=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[242]*v8818))}else{v60})});
        let v8893=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[242]*v8819))}else{v60})});
        let v8894=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[242]*v8820))}else{v60})});
        let v8895=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[242]*v8821))}else{v60})});
        let v8902=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[243]*v8852))}else{v60})});
        let v8903=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[243]*v8853))}else{v60})});
        let v8904=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[243]*v8854))}else{v60})});
        let v8905=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[243]*v8855))}else{v60})});
        let v8906=(if self.scalar_static_bool[59]{v60}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[85]*(self.scalar_static_f64[243]*v8856))}else{v60})});
        let v8931=(self.scalar_static_f64[0]*(if v2387{v60}else{v8775}));
        let v8932=(self.scalar_static_f64[0]*(if v2387{v60}else{(if v2357{(v707*((if v2357{((v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(v8679/v701))/v2373)}else{v60})))))/self.scalar_static_f64[240])}else{v60})+(v782*(self.scalar_static_f64[0]-v8679))))}else{(if v2354{v60}else{(if v2257{((v701*(((if v2257{((v707*(-(v2330*(v2326*v8472))))/v2326)}else{v60})+(if v2257{((v2275*(-(v2336*(v2328*(if v2257{((-(v8356/v701))/v2318)}else{v60})))))/v2328)}else{v60}))-(if v2257{((v2275*(-(v2342*(v2328*v8472))))/v2328)}else{v60})))+(v2268*(if v2257{(self.scalar_static_f64[0]-v8356)}else{v60})))}else{v60})})})}));
        let v8933=(self.scalar_static_f64[0]*(if v2387{v60}else{(if v2357{((v2384*v2730)+(v707*((if v2357{(((v2378*v2721)+(v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(((v701*v8680)-(v2371*v2721))/v2724))/v2373)}else{v7713}))))))/self.scalar_static_f64[240])}else{(if v2102{(((v2134*v2591)+(v563*(-(v2133*(self.scalar_static_f64[190]*v7713)))))/self.scalar_static_f64[190])}else{v5620})})+(v782*(-v8680)))))}else{(if v2354{v60}else{(if v2257{v8598}else{v60})})})}));
        let v8934=(self.scalar_static_f64[0]*(if v2387{v60}else{v8778}));
        let v8935=(self.scalar_static_f64[0]*(if v2387{v60}else{v8779}));
        let v8936=(self.scalar_static_f64[0]*(if v2387{v60}else{(if v2357{(v707*((if v2357{((v701*(-(v2377*(self.scalar_static_f64[240]*(if v2357{((-(v8683/v701))/v2373)}else{v7716})))))/self.scalar_static_f64[240])}else{(if v2102{((v563*(-(v2133*(self.scalar_static_f64[190]*v7716))))/self.scalar_static_f64[190])}else{v5623})})+(v782*(-v8683))))}else{v60})}));
        let v8937=(self.scalar_static_f64[0]*(if v1266{v60}else{(if v1230{(v1116*(v4351+(v782*(self.scalar_static_f64[0]-v4261))))}else{(if v1228{v60}else{(if v1118{((v670*((v4126+v4152)-v4178))+(v1123*v3972))}else{(if v943{v60}else{(if v906{(v772*(v3223+(v782*(self.scalar_static_f64[0]-v3154))))}else{(if v902{v60}else{(if v777{((v563*((v3049+v3069)-v3089))+(v790*v2930))}else{v60})})})})})})})}));
        let v8938=(self.scalar_static_f64[0]*(if v1266{v60}else{(if v1230{((v1263*v3818)+(v1116*(v4352+(v782*(-v4262)))))}else{(if v1228{v60}else{(if v1118{(((v1222*v2692)+(v670*((v4127+v4153)-v4179)))+((v1175*v3822)+(v1123*v3973)))}else{(if v943{v60}else{(if v906{((v940*v2796)+(v772*(v3224+(v782*(-v3155)))))}else{(if v902{v60}else{(if v777{(((v896*v2591)+(v563*((v3050+v3070)-v3090)))+((v847*v2802)+(v790*v2931)))}else{v60})})})})})})})}));
        let v8939=(self.scalar_static_f64[0]*(if v1266{v60}else{(if v1230{(v1116*(v4353+(v782*(self.scalar_static_f64[247]-v4263))))}else{(if v1228{v60}else{(if v1118{((v670*((v4128+v4154)-v4180))+(v1123*v3974))}else{(if v943{v60}else{(if v906{(v772*(v3225+(v782*(self.scalar_static_f64[247]-v3156))))}else{(if v902{v60}else{(if v777{((v563*((v3051+v3071)-v3091))+(v790*v2932))}else{v60})})})})})})})}));
        let v8940=(self.scalar_static_f64[0]*(if v1266{v60}else{(if v1230{(v1116*(v4354+(v782*(-v4264))))}else{(if v1228{v60}else{(if v1118{((v670*((v4129+v4155)-v4181))+(v1123*v3975))}else{v60})})})}));
        let v8950=(self.scalar_static_f64[0]*(((if v1113{v60}else{(if v1077{(v948*(v3787+(v782*(-v3697))))}else{(if v1073{v60}else{(if v953{((v670*((v3562+v3588)-v3614))+(v964*v3408))}else{v60})})})})+v4939)+(self.scalar_static_f64[226]*v6890)));
        let v8951=(self.scalar_static_f64[0]*(((if v1113{v60}else{(if v1077{((v1110*v3248)+(v948*(v3788+(v782*(-v3698)))))}else{(if v1073{v60}else{(if v953{(((v1067*v2692)+(v670*((v3563+v3589)-v3615)))+((v1018*v3254)+(v964*v3409)))}else{v60})})})})+v4940)+(self.scalar_static_f64[226]*v6894)));
        let v8952=(self.scalar_static_f64[0]*(((if v1113{v60}else{(if v1077{(v948*(v3789+(v782*(self.scalar_static_f64[247]-v3699))))}else{(if v1073{v60}else{(if v953{((v670*((v3564+v3590)-v3616))+(v964*v3410))}else{v60})})})})+v4941)+(self.scalar_static_f64[226]*v6898)));
        let v8953=(self.scalar_static_f64[0]*(((if v1113{v60}else{(if v1077{(v948*(v3790+(v782*(self.scalar_static_f64[0]-v3700))))}else{(if v1073{v60}else{(if v953{((v670*((v3565+v3591)-v3617))+(v964*v3411))}else{v60})})})})+v4942)+(self.scalar_static_f64[226]*v6902)));
        let v8954=(self.scalar_static_f64[0]*(self.scalar_static_f64[226]*v6905));
        let v8958=(self.scalar_static_f64[0]*(v5429+v8817));
        let v8959=(self.scalar_static_f64[0]*(v5430+v8818));
        let v8960=(self.scalar_static_f64[0]*(v5431+v8819));
        let v8961=(self.scalar_static_f64[0]*(v5432+v8820));
        let v8962=(self.scalar_static_f64[0]*(v5433+v8821));

        CommonStampValues {
            v1,
            v2,
            v4,
            v5,
            v6,
            v7,
            v8,
            v9,
            v10,
            v11,
            v14,
            v15,
            v32,
            v48,
            v60,
            v91,
            v431,
            v444,
            v446,
            v448,
            v452,
            v455,
            v563,
            v569,
            v772,
            v777,
            v779,
            v782,
            v790,
            v797,
            v801,
            v803,
            v805,
            v806,
            v814,
            v826,
            v828,
            v829,
            v843,
            v851,
            v855,
            v906,
            v915,
            v918,
            v927,
            v946,
            v948,
            v953,
            v955,
            v964,
            v971,
            v976,
            v978,
            v979,
            v987,
            v997,
            v999,
            v1000,
            v1014,
            v1022,
            v1026,
            v1077,
            v1085,
            v1088,
            v1097,
            v1116,
            v1118,
            v1119,
            v1123,
            v1128,
            v1133,
            v1135,
            v1136,
            v1144,
            v1154,
            v1156,
            v1157,
            v1171,
            v1179,
            v1183,
            v1230,
            v1238,
            v1241,
            v1250,
            v1269,
            v1270,
            v1274,
            v1279,
            v1284,
            v1286,
            v1287,
            v1295,
            v1305,
            v1307,
            v1308,
            v1322,
            v1330,
            v1334,
            v1381,
            v1389,
            v1392,
            v1401,
            v1491,
            v1492,
            v1506,
            v1509,
            v1518,
            v1538,
            v1540,
            v1554,
            v1557,
            v1566,
            v1667,
            v1740,
            v1895,
            v1897,
            v1924,
            v1992,
            v1993,
            v1994,
            v1998,
            v2003,
            v2008,
            v2010,
            v2011,
            v2019,
            v2029,
            v2031,
            v2032,
            v2046,
            v2054,
            v2058,
            v2102,
            v2110,
            v2113,
            v2122,
            v2399,
            v2400,
            v2407,
            v2408,
            v2417,
            v2419,
            v2428,
            v2429,
            v2430,
            v2431,
            v2433,
            v2435,
            v2450,
            v2477,
            v2481,
            v2482,
            v2486,
            v2490,
            v2591,
            v2600,
            v2796,
            v2802,
            v2812,
            v2824,
            v2825,
            v2826,
            v2881,
            v2882,
            v2883,
            v2945,
            v2946,
            v2947,
            v2960,
            v2961,
            v2962,
            v3134,
            v3135,
            v3136,
            v3143,
            v3144,
            v3145,
            v3185,
            v3186,
            v3187,
            v3246,
            v3248,
            v3254,
            v3264,
            v3276,
            v3277,
            v3278,
            v3279,
            v3347,
            v3348,
            v3349,
            v3350,
            v3427,
            v3428,
            v3429,
            v3430,
            v3446,
            v3447,
            v3448,
            v3449,
            v3671,
            v3672,
            v3673,
            v3674,
            v3683,
            v3684,
            v3685,
            v3686,
            v3737,
            v3738,
            v3739,
            v3740,
            v3818,
            v3822,
            v3828,
            v3840,
            v3841,
            v3842,
            v3843,
            v3911,
            v3912,
            v3913,
            v3914,
            v3991,
            v3992,
            v3993,
            v3994,
            v4010,
            v4011,
            v4012,
            v4013,
            v4235,
            v4236,
            v4237,
            v4238,
            v4247,
            v4248,
            v4249,
            v4250,
            v4301,
            v4302,
            v4303,
            v4304,
            v4384,
            v4390,
            v4402,
            v4403,
            v4404,
            v4405,
            v4473,
            v4474,
            v4475,
            v4476,
            v4553,
            v4554,
            v4555,
            v4556,
            v4572,
            v4573,
            v4574,
            v4575,
            v4797,
            v4798,
            v4799,
            v4800,
            v4809,
            v4810,
            v4811,
            v4812,
            v4863,
            v4864,
            v4865,
            v4866,
            v5215,
            v5216,
            v5217,
            v5218,
            v5252,
            v5253,
            v5254,
            v5255,
            v5256,
            v5267,
            v5268,
            v5269,
            v5270,
            v5271,
            v5333,
            v5334,
            v5335,
            v5336,
            v5337,
            v5434,
            v5439,
            v5440,
            v5441,
            v5442,
            v5476,
            v5477,
            v5478,
            v5479,
            v5480,
            v5491,
            v5492,
            v5493,
            v5494,
            v5495,
            v5557,
            v5558,
            v5559,
            v5560,
            v5561,
            v5878,
            v5879,
            v5880,
            v5952,
            v6890,
            v6894,
            v6898,
            v6902,
            v6905,
            v6906,
            v6907,
            v6908,
            v6909,
            v6910,
            v7071,
            v7072,
            v7073,
            v7074,
            v7075,
            v7230,
            v7236,
            v7248,
            v7249,
            v7250,
            v7251,
            v7319,
            v7320,
            v7321,
            v7322,
            v7399,
            v7400,
            v7401,
            v7402,
            v7418,
            v7419,
            v7420,
            v7421,
            v7631,
            v7632,
            v7633,
            v7634,
            v7635,
            v7646,
            v7647,
            v7648,
            v7649,
            v7650,
            v7712,
            v7713,
            v7714,
            v7715,
            v7716,
            v8817,
            v8818,
            v8819,
            v8820,
            v8821,
            v8852,
            v8853,
            v8854,
            v8855,
            v8856,
            v8891,
            v8892,
            v8893,
            v8894,
            v8895,
            v8902,
            v8903,
            v8904,
            v8905,
            v8906,
            v8931,
            v8932,
            v8933,
            v8934,
            v8935,
            v8936,
            v8937,
            v8938,
            v8939,
            v8940,
            v8950,
            v8951,
            v8952,
            v8953,
            v8954,
            v8958,
            v8959,
            v8960,
            v8961,
            v8962,
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
        let v18=(common.v8-common.v15);
        let v20=(common.v2-ctx.node_voltage(nodes[0]));
        let v21=(common.v1-common.v5);
        let v530=(((self.scalar_static_f64[25]*common.v452)+(self.scalar_static_f64[8]*common.v455))).exp();
        let v532=(if self.scalar_static_bool[14]{(self.scalar_static_f64[59]*v530)}else{self.scalar_static_f64[339]});
        let v536=(((self.scalar_static_f64[61]*common.v452)+(self.scalar_static_f64[62]*common.v455))).exp();
        let v538=(if self.scalar_static_bool[14]{(self.scalar_static_f64[60]*v536)}else{self.scalar_static_f64[344]});
        let v571=(self.scalar_static_f64[11]*common.v455);
        let v573=(((self.scalar_static_f64[23]*common.v452)+v571)).exp();
        let v575=(if self.scalar_static_bool[14]{(self.scalar_static_f64[73]*v573)}else{self.scalar_static_f64[369]});
        let v633=((self.scalar_static_f64[92]*common.v448)).exp();
        let v637=((self.scalar_static_f64[94]*common.v448)).exp();
        let v641=(if self.scalar_static_bool[20]{self.scalar_static_f64[31]}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[31]*v633)}else{self.scalar_static_f64[421]})});
        let v642=(if self.scalar_static_bool[20]{self.scalar_static_f64[93]}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[93]*v637)}else{self.scalar_static_f64[422]})});
        let v644=((self.scalar_static_f64[96]*common.v452)).exp();
        let v646=(if self.scalar_static_bool[14]{(self.scalar_static_f64[95]*v644)}else{self.scalar_static_f64[425]});
        let v708=(self.scalar_static_f64[24]*common.v452);
        let v711=((v708+(self.scalar_static_f64[14]*common.v455))).exp();
        let v713=(if self.scalar_static_bool[14]{(self.scalar_static_f64[119]*v711)}else{self.scalar_static_f64[469]});
        let v715=((v571+v708)).exp();
        let v717=(if self.scalar_static_bool[14]{(self.scalar_static_f64[120]*v715)}else{self.scalar_static_f64[472]});
        let v747=((self.scalar_static_f64[133]*common.v452)).exp();
        let v749=(if self.scalar_static_bool[14]{(self.scalar_static_f64[132]*v747)}else{self.scalar_static_f64[497]});
        let v751=((self.scalar_static_f64[135]*common.v452)).exp();
        let v755=((self.scalar_static_f64[137]*common.v452)).exp();
        let v757=(if self.scalar_static_bool[14]{(self.scalar_static_f64[136]*v755)}else{self.scalar_static_f64[503]});
        let v759=((self.scalar_static_f64[139]*common.v452)).exp();
        let v760=(self.scalar_static_f64[138]*v759);
        let v762=(common.v48+(self.scalar_static_f64[140]*common.v448));
        let v764=(if self.scalar_static_bool[14]{(v760*v762)}else{self.scalar_static_f64[509]});
        let v815=(if common.v814{common.v48}else{(if common.v803{(common.v805/common.v806)}else{common.v60})});
        let v844=(if common.v843{common.v48}else{(if common.v826{(common.v828/common.v829)}else{common.v60})});
        let v862=((common.v855*self.scalar_static_f64[191])).exp();
        let v863=(common.v772*v862);
        let v864=(v815*v863);
        let v867=(-common.v779);
        let v869=((common.v851*v867)).exp();
        let v870=(common.v797*v869);
        let v871=(common.v48-v844);
        let v874=(common.v48-v815);
        let v923=(if common.v906{(common.v918/common.v915)}else{common.v60});
        let v929=((self.scalar_static_f64[191]*common.v927)).exp();
        let v988=(if common.v987{common.v48}else{(if common.v976{(common.v978/common.v979)}else{v815})});
        let v1015=(if common.v1014{common.v48}else{(if common.v997{(common.v999/common.v1000)}else{v844})});
        let v1033=((common.v1026*self.scalar_static_f64[198])).exp();
        let v1034=(common.v948*v1033);
        let v1035=(v988*v1034);
        let v1038=(-common.v955);
        let v1040=((common.v1022*v1038)).exp();
        let v1041=(common.v971*v1040);
        let v1042=(common.v48-v1015);
        let v1045=(common.v48-v988);
        let v1093=(if common.v1077{(common.v1088/common.v1085)}else{v923});
        let v1099=((self.scalar_static_f64[198]*common.v1097)).exp();
        let v1145=(if common.v1144{common.v48}else{(if common.v1133{(common.v1135/common.v1136)}else{v988})});
        let v1172=(if common.v1171{common.v48}else{(if common.v1154{(common.v1156/common.v1157)}else{v1015})});
        let v1188=((self.scalar_static_f64[198]*common.v1183)).exp();
        let v1189=(common.v1116*v1188);
        let v1190=(v1145*v1189);
        let v1193=(-common.v1119);
        let v1195=((common.v1179*v1193)).exp();
        let v1196=(common.v1128*v1195);
        let v1197=(common.v48-v1172);
        let v1200=(common.v48-v1145);
        let v1246=(if common.v1230{(common.v1241/common.v1238)}else{v1093});
        let v1252=((self.scalar_static_f64[198]*common.v1250)).exp();
        let v1296=(if common.v1295{common.v48}else{(if common.v1284{(common.v1286/common.v1287)}else{v1145})});
        let v1323=(if common.v1322{common.v48}else{(if common.v1305{(common.v1307/common.v1308)}else{v1172})});
        let v1339=((self.scalar_static_f64[191]*common.v1334)).exp();
        let v1340=(common.v946*v1339);
        let v1341=(v1296*v1340);
        let v1344=(-common.v1270);
        let v1346=((common.v1330*v1344)).exp();
        let v1347=(common.v1279*v1346);
        let v1348=(common.v48-v1323);
        let v1351=(common.v48-v1296);
        let v1397=(if common.v1381{(common.v1392/common.v1389)}else{v1246});
        let v1403=((self.scalar_static_f64[191]*common.v1401)).exp();
        let v1514=(if common.v1492{(common.v1509/common.v1506)}else{v1397});
        let v1521=((common.v1518*self.scalar_static_f64[202])).exp();
        let v1562=(if common.v1540{(common.v1557/common.v1554)}else{v1514});
        let v1569=((common.v1566*self.scalar_static_f64[204])).exp();
        let v1898=(common.v1897-common.v1895);
        let v1929=(common.v444*self.scalar_static_f64[227]);
        let v1931=(if self.scalar_static_bool[42]{(common.v10/v1929)}else{common.v60});
        let v1932=(v1931>common.v801);
        let v1933=(self.scalar_static_bool[42]&&v1932);
        let v1937=(if v1933{common.v801}else{v1931});
        let v1939=(self.scalar_static_bool[42]&&(!v1932));
        let v1940=(if v1939{common.v48}else{(if v1933{(common.v48+(v1931-common.v801))}else{common.v60})});
        let v1941=scalar_limexp(v1937);
        let v1943=((v1940*v1941)-common.v48);
        let v1950=(common.v444*self.scalar_static_f64[228]);
        let v1952=(if self.scalar_static_bool[44]{(common.v10/v1950)}else{v1937});
        let v1953=(v1952>common.v801);
        let v1954=(self.scalar_static_bool[44]&&v1953);
        let v1958=(if v1954{common.v801}else{v1952});
        let v1960=(self.scalar_static_bool[44]&&(!v1953));
        let v1961=(if v1960{common.v48}else{(if v1954{(common.v48+(v1952-common.v801))}else{v1940})});
        let v1962=scalar_limexp(v1958);
        let v1964=((v1961*v1962)-common.v48);
        let v1969=((if self.scalar_static_bool[43]{common.v60}else{(if self.scalar_static_bool[42]{(v532*v1943)}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if self.scalar_static_bool[44]{(v538*v1964)}else{common.v60})}));
        let v1972=(common.v444*self.scalar_static_f64[229]);
        let v1974=(if self.scalar_static_bool[46]{(common.v7/v1972)}else{v1958});
        let v1975=(v1974>common.v801);
        let v1976=(self.scalar_static_bool[46]&&v1975);
        let v1980=(if v1976{common.v801}else{v1974});
        let v1982=(self.scalar_static_bool[46]&&(!v1975));
        let v1983=(if v1982{common.v48}else{(if v1976{(common.v48+(v1974-common.v801))}else{v1961})});
        let v1984=scalar_limexp(v1980);
        let v1986=((v1983*v1984)-common.v48);
        let v1990=(if self.scalar_static_bool[47]{common.v60}else{(if self.scalar_static_bool[46]{(v575*v1986)}else{common.v60})});
        let v1991=(v1969+v1990);
        let v2020=(if common.v2019{common.v48}else{(if common.v2008{(common.v2010/common.v2011)}else{v1296})});
        let v2047=(if common.v2046{common.v48}else{(if common.v2029{(common.v2031/common.v2032)}else{v1323})});
        let v2063=((self.scalar_static_f64[191]*common.v2058)).exp();
        let v2064=(common.v569*v2063);
        let v2065=(v2020*v2064);
        let v2068=(-common.v1994);
        let v2070=((common.v2054*v2068)).exp();
        let v2071=(common.v2003*v2070);
        let v2072=(common.v48-v2047);
        let v2075=(common.v48-v2020);
        let v2099=(!common.v1992);
        let v2100=(self.scalar_static_bool[23]&&v2099);
        let v2118=(if common.v2102{(common.v2113/common.v2110)}else{v1562});
        let v2124=((self.scalar_static_f64[191]*common.v2122)).exp();
        let v2129=((if common.v2102{(v2118*v2124)}else{(if common.v1540{(v1562*v1569)}else{(if common.v1492{(v1514*v1521)}else{(if common.v1381{(v1397*v1403)}else{(if common.v1230{(v1246*v1252)}else{(if common.v1077{(v1093*v1099)}else{(if common.v906{(v923*v929)}else{common.v60})})})})})})})+(common.v782*(common.v48-v2118)));
        let v2138=(self.scalar_static_bool[24]&&v2099);
        let v2139=(if v2138{common.v60}else{(if common.v2102{(common.v569*v2129)}else{(if v2100{common.v60}else{(if common.v1993{((if common.v1993{(common.v1998*v2075)}else{(if common.v1269{(common.v1274*v1351)}else{(if common.v1118{(common.v1123*v1200)}else{(if common.v953{(common.v964*v1045)}else{(if common.v777{(common.v790*v874)}else{common.v60})})})})})+((if common.v1993{(v2047*v2065)}else{(if common.v1269{(v1323*v1341)}else{(if common.v1118{(v1172*v1190)}else{(if common.v953{(v1015*v1035)}else{(if common.v777{(v844*v864)}else{common.v60})})})})})+(if common.v1993{(v2071*v2072)}else{(if common.v1269{(v1347*v1348)}else{(if common.v1118{(v1196*v1197)}else{(if common.v953{(v1041*v1042)}else{(if common.v777{(v870*v871)}else{common.v60})})})})})))}else{common.v60})})})});
        let v2140=(common.v563-common.v7);
        let v2141=(if self.scalar_static_bool[8]{v2140}else{common.v60});
        let v2142=(v2141>common.v60);
        let v2143=(self.scalar_static_bool[8]&&v2142);
        let v2145=(if v2143{(v642/v2139)}else{common.v60});
        let v2147=(if v2143{(v642/common.v569)}else{common.v60});
        let v2148=(v2141>v2147);
        let v2149=(v2143&&v2148);
        let v2150=(-v2145);
        let v2152=((v2150/v2147)).exp();
        let v2154=(if v2149{(v641*v2152)}else{common.v60});
        let v2156=(common.v48+(v2145/v2147));
        let v2157=(v2141-v2147);
        let v2159=(v2147+(v2156*v2157));
        let v2163=(v2143&&(!v2148));
        let v2164=(v641*v2141);
        let v2166=((v2150/v2141)).exp();
        let v2168=(if v2163{(v2164*v2166)}else{(if v2149{(v2154*v2159)}else{common.v60})});
        let v2172=(self.scalar_static_bool[8]&&(!v2142));
        let v2173=(if v2172{common.v60}else{(if v2143{(common.v1897*v2168)}else{common.v60})});
        let v2174=(v646>common.v60);
        let v2185=(if v2174{((((common.v48+(common.v1538/self.scalar_static_f64[230]))+(common.v1491/self.scalar_static_f64[231]))+(common.v1897/common.v1667))+(common.v1895/self.scalar_static_f64[214]))}else{common.v60});
        let v2188=((common.v1740+(v2185*v2185))).sqrt();
        let v2191=(if v2174{(common.v32*(v2185+v2188))}else{common.v60});
        let v2193=(if v2174{(v646/v2191)}else{common.v60});
        let v2195=(v2174&&(v1991>common.v60));
        let v2197=(v2193*self.scalar_static_f64[232]);
        let v2198=(v1991*v2197);
        let v2200=(if v2195{(common.v446*v2198)}else{common.v60});
        let v2202=(v2200<1e-6);
        let v2203=(v2195&&v2202);
        let v2205=(common.v48-(common.v32*v2200));
        let v2207=(if v2203{(v2193*v2205)}else{v2193});
        let v2209=(v2195&&(!v2202));
        let v2210=(common.v48+v2200);
        let v2211=(v2210).ln();
        let v2212=(v2207*v2211);
        let v2215=(!v2174);
        let v2217=((if self.scalar_static_bool[14]{(self.scalar_static_f64[134]*v751)}else{self.scalar_static_f64[500]})+(if v2215{common.v60}else{(if v2209{(v2212/v2200)}else{v2207})}));
        let v2221=(if self.scalar_static_bool[48]{(common.v444*self.scalar_static_f64[233])}else{common.v60});
        let v2222=(common.v4/v2221);
        let v2225=(common.v14/v2221);
        let v2228=((if self.scalar_static_bool[48]{scalar_limexp(v2222)}else{common.v60})-(if self.scalar_static_bool[48]{scalar_limexp(v2225)}else{common.v60}));
        let v2235=(common.v444*self.scalar_static_f64[234]);
        let v2237=(if self.scalar_static_bool[50]{(common.v14/v2235)}else{v1980});
        let v2238=(v2237>common.v801);
        let v2239=(self.scalar_static_bool[50]&&v2238);
        let v2243=(if v2239{common.v801}else{v2237});
        let v2245=(self.scalar_static_bool[50]&&(!v2238));
        let v2246=(if v2245{common.v48}else{(if v2239{(common.v48+(v2237-common.v801))}else{v1983})});
        let v2247=scalar_limexp(v2243);
        let v2249=((v2246*v2247)-common.v48);
        let v2451=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2450);
        let v2455=-1.0;
        let v2602=(self.scalar_static_f64[11]*common.v2490);
        let v2668=(if self.scalar_static_bool[20]{common.v60}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[31]*(v633*(self.scalar_static_f64[92]*common.v2482)))}else{common.v60})});
        let v2669=(if self.scalar_static_bool[20]{common.v60}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[93]*(v637*(self.scalar_static_f64[94]*common.v2482)))}else{common.v60})});
        let v2731=(self.scalar_static_f64[24]*common.v2486);
        let v2830=(common.v806*common.v806);
        let v2857=(if common.v814{common.v60}else{(if common.v803{(((common.v806*common.v2824)-(common.v805*common.v2824))/v2830)}else{common.v60})});
        let v2858=(if common.v814{common.v60}else{(if common.v803{(((common.v806*common.v2825)-(common.v805*common.v2825))/v2830)}else{common.v60})});
        let v2859=(if common.v814{common.v60}else{(if common.v803{(((common.v806*common.v2826)-(common.v805*common.v2826))/v2830)}else{common.v60})});
        let v2887=(common.v829*common.v829);
        let v2921=(if common.v843{common.v60}else{(if common.v826{(((common.v829*common.v2881)-(common.v828*common.v2881))/v2887)}else{common.v60})});
        let v2922=(if common.v843{common.v60}else{(if common.v826{(((common.v829*common.v2882)-(common.v828*common.v2882))/v2887)}else{common.v60})});
        let v2923=(if common.v843{common.v60}else{(if common.v826{(((common.v829*common.v2883)-(common.v828*common.v2883))/v2887)}else{common.v60})});
        let v3160=(common.v915*common.v915);
        let v3170=(if common.v906{(((common.v915*common.v3143)-(common.v918*common.v3134))/v3160)}else{common.v60});
        let v3171=(if common.v906{(((common.v915*common.v3144)-(common.v918*common.v3135))/v3160)}else{common.v60});
        let v3172=(if common.v906{(((common.v915*common.v3145)-(common.v918*common.v3136))/v3160)}else{common.v60});
        let v3283=(common.v979*common.v979);
        let v3319=(if common.v987{common.v60}else{(if common.v976{(((common.v979*common.v3276)-(common.v978*common.v3276))/v3283)}else{v2857})});
        let v3320=(if common.v987{common.v60}else{(if common.v976{(((common.v979*common.v3277)-(common.v978*common.v3277))/v3283)}else{v2858})});
        let v3321=(if common.v987{common.v60}else{(if common.v976{(((common.v979*common.v3278)-(common.v978*common.v3278))/v3283)}else{v2859})});
        let v3322=(if common.v987{common.v60}else{(if common.v976{(((common.v979*common.v3279)-(common.v978*common.v3279))/v3283)}else{common.v60})});
        let v3354=(common.v1000*common.v1000);
        let v3396=(if common.v1014{common.v60}else{(if common.v997{(((common.v1000*common.v3347)-(common.v999*common.v3347))/v3354)}else{v2921})});
        let v3397=(if common.v1014{common.v60}else{(if common.v997{(((common.v1000*common.v3348)-(common.v999*common.v3348))/v3354)}else{v2922})});
        let v3398=(if common.v1014{common.v60}else{(if common.v997{(((common.v1000*common.v3349)-(common.v999*common.v3349))/v3354)}else{v2923})});
        let v3399=(if common.v1014{common.v60}else{(if common.v997{(((common.v1000*common.v3350)-(common.v999*common.v3350))/v3354)}else{common.v60})});
        let v3704=(common.v1085*common.v1085);
        let v3718=(if common.v1077{(((common.v1085*common.v3683)-(common.v1088*common.v3671))/v3704)}else{v3170});
        let v3719=(if common.v1077{(((common.v1085*common.v3684)-(common.v1088*common.v3672))/v3704)}else{v3171});
        let v3720=(if common.v1077{(((common.v1085*common.v3685)-(common.v1088*common.v3673))/v3704)}else{v3172});
        let v3721=(if common.v1077{(((common.v1085*common.v3686)-(common.v1088*common.v3674))/v3704)}else{common.v60});
        let v3847=(common.v1136*common.v1136);
        let v3883=(if common.v1144{common.v60}else{(if common.v1133{(((common.v1136*common.v3840)-(common.v1135*common.v3840))/v3847)}else{v3319})});
        let v3884=(if common.v1144{common.v60}else{(if common.v1133{(((common.v1136*common.v3841)-(common.v1135*common.v3841))/v3847)}else{v3320})});
        let v3885=(if common.v1144{common.v60}else{(if common.v1133{(((common.v1136*common.v3842)-(common.v1135*common.v3842))/v3847)}else{v3321})});
        let v3886=(if common.v1144{common.v60}else{(if common.v1133{(((common.v1136*common.v3843)-(common.v1135*common.v3843))/v3847)}else{v3322})});
        let v3918=(common.v1157*common.v1157);
        let v3960=(if common.v1171{common.v60}else{(if common.v1154{(((common.v1157*common.v3911)-(common.v1156*common.v3911))/v3918)}else{v3396})});
        let v3961=(if common.v1171{common.v60}else{(if common.v1154{(((common.v1157*common.v3912)-(common.v1156*common.v3912))/v3918)}else{v3397})});
        let v3962=(if common.v1171{common.v60}else{(if common.v1154{(((common.v1157*common.v3913)-(common.v1156*common.v3913))/v3918)}else{v3398})});
        let v3963=(if common.v1171{common.v60}else{(if common.v1154{(((common.v1157*common.v3914)-(common.v1156*common.v3914))/v3918)}else{v3399})});
        let v4268=(common.v1238*common.v1238);
        let v4282=(if common.v1230{(((common.v1238*common.v4247)-(common.v1241*common.v4235))/v4268)}else{v3718});
        let v4283=(if common.v1230{(((common.v1238*common.v4248)-(common.v1241*common.v4236))/v4268)}else{v3719});
        let v4284=(if common.v1230{(((common.v1238*common.v4249)-(common.v1241*common.v4237))/v4268)}else{v3720});
        let v4285=(if common.v1230{(((common.v1238*common.v4250)-(common.v1241*common.v4238))/v4268)}else{v3721});
        let v4409=(common.v1287*common.v1287);
        let v4445=(if common.v1295{common.v60}else{(if common.v1284{(((common.v1287*common.v4402)-(common.v1286*common.v4402))/v4409)}else{v3883})});
        let v4446=(if common.v1295{common.v60}else{(if common.v1284{(((common.v1287*common.v4403)-(common.v1286*common.v4403))/v4409)}else{v3884})});
        let v4447=(if common.v1295{common.v60}else{(if common.v1284{(((common.v1287*common.v4404)-(common.v1286*common.v4404))/v4409)}else{v3885})});
        let v4448=(if common.v1295{common.v60}else{(if common.v1284{(((common.v1287*common.v4405)-(common.v1286*common.v4405))/v4409)}else{v3886})});
        let v4480=(common.v1308*common.v1308);
        let v4522=(if common.v1322{common.v60}else{(if common.v1305{(((common.v1308*common.v4473)-(common.v1307*common.v4473))/v4480)}else{v3960})});
        let v4523=(if common.v1322{common.v60}else{(if common.v1305{(((common.v1308*common.v4474)-(common.v1307*common.v4474))/v4480)}else{v3961})});
        let v4524=(if common.v1322{common.v60}else{(if common.v1305{(((common.v1308*common.v4475)-(common.v1307*common.v4475))/v4480)}else{v3962})});
        let v4525=(if common.v1322{common.v60}else{(if common.v1305{(((common.v1308*common.v4476)-(common.v1307*common.v4476))/v4480)}else{v3963})});
        let v4830=(common.v1389*common.v1389);
        let v4844=(if common.v1381{(((common.v1389*common.v4809)-(common.v1392*common.v4797))/v4830)}else{v4282});
        let v4845=(if common.v1381{(((common.v1389*common.v4810)-(common.v1392*common.v4798))/v4830)}else{v4283});
        let v4846=(if common.v1381{(((common.v1389*common.v4811)-(common.v1392*common.v4799))/v4830)}else{v4284});
        let v4847=(if common.v1381{(((common.v1389*common.v4812)-(common.v1392*common.v4800))/v4830)}else{v4285});
        let v5292=(common.v1506*common.v1506);
        let v5310=(if common.v1492{(((common.v1506*common.v5267)-(common.v1509*common.v5252))/v5292)}else{v4844});
        let v5311=(if common.v1492{(((common.v1506*common.v5268)-(common.v1509*common.v5253))/v5292)}else{v4845});
        let v5312=(if common.v1492{(((common.v1506*common.v5269)-(common.v1509*common.v5254))/v5292)}else{v4846});
        let v5313=(if common.v1492{(((common.v1506*common.v5270)-(common.v1509*common.v5255))/v5292)}else{v4847});
        let v5314=(if common.v1492{(((common.v1506*common.v5271)-(common.v1509*common.v5256))/v5292)}else{common.v60});
        let v5516=(common.v1554*common.v1554);
        let v5534=(if common.v1540{(((common.v1554*common.v5491)-(common.v1557*common.v5476))/v5516)}else{v5310});
        let v5535=(if common.v1540{(((common.v1554*common.v5492)-(common.v1557*common.v5477))/v5516)}else{v5311});
        let v5536=(if common.v1540{(((common.v1554*common.v5493)-(common.v1557*common.v5478))/v5516)}else{v5312});
        let v5537=(if common.v1540{(((common.v1554*common.v5494)-(common.v1557*common.v5479))/v5516)}else{v5313});
        let v5538=(if common.v1540{(((common.v1554*common.v5495)-(common.v1557*common.v5480))/v5516)}else{v5314});
        let v7088=(if self.scalar_static_bool[42]{((-(common.v10*(self.scalar_static_f64[227]*common.v2477)))/(v1929*v1929))}else{common.v60});
        let v7089=(if self.scalar_static_bool[42]{(self.scalar_static_f64[0]/v1929)}else{common.v60});
        let v7090=(if self.scalar_static_bool[42]{(self.scalar_static_f64[247]/v1929)}else{common.v60});
        let v7094=(if v1933{common.v60}else{v7088});
        let v7095=(if v1933{common.v60}else{v7089});
        let v7096=(if v1933{common.v60}else{v7090});
        let v7097=(if v1939{common.v60}else{(if v1933{v7088}else{common.v60})});
        let v7098=(if v1939{common.v60}else{(if v1933{v7089}else{common.v60})});
        let v7099=(if v1939{common.v60}else{(if v1933{v7090}else{common.v60})});
        let v7100=scalar_limexp_derivative(v1937);
        let v7131=(if self.scalar_static_bool[44]{((-(common.v10*(self.scalar_static_f64[228]*common.v2477)))/(v1950*v1950))}else{v7094});
        let v7132=(if self.scalar_static_bool[44]{(self.scalar_static_f64[0]/v1950)}else{v7095});
        let v7133=(if self.scalar_static_bool[44]{(self.scalar_static_f64[247]/v1950)}else{v7096});
        let v7137=(if v1954{common.v60}else{v7131});
        let v7138=(if v1954{common.v60}else{v7132});
        let v7139=(if v1954{common.v60}else{v7133});
        let v7140=(if v1960{common.v60}else{(if v1954{v7131}else{v7097})});
        let v7141=(if v1960{common.v60}else{(if v1954{v7132}else{v7098})});
        let v7142=(if v1960{common.v60}else{(if v1954{v7133}else{v7099})});
        let v7143=scalar_limexp_derivative(v1958);
        let v7167=((if self.scalar_static_bool[43]{common.v60}else{(if self.scalar_static_bool[42]{((v1943*(if self.scalar_static_bool[14]{(self.scalar_static_f64[59]*(v530*((self.scalar_static_f64[25]*common.v2486)+(self.scalar_static_f64[8]*common.v2490))))}else{common.v60}))+(v532*((v1941*v7097)+(v1940*(v7094*v7100)))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if self.scalar_static_bool[44]{((v1964*(if self.scalar_static_bool[14]{(self.scalar_static_f64[60]*(v536*((self.scalar_static_f64[61]*common.v2486)+(self.scalar_static_f64[62]*common.v2490))))}else{common.v60}))+(v538*((v1962*v7140)+(v1961*(v7137*v7143)))))}else{common.v60})}));
        let v7168=((if self.scalar_static_bool[43]{common.v60}else{(if self.scalar_static_bool[42]{(v532*((v1941*v7098)+(v1940*(v7095*v7100))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if self.scalar_static_bool[44]{(v538*((v1962*v7141)+(v1961*(v7138*v7143))))}else{common.v60})}));
        let v7169=((if self.scalar_static_bool[43]{common.v60}else{(if self.scalar_static_bool[42]{(v532*((v1941*v7099)+(v1940*(v7096*v7100))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if self.scalar_static_bool[44]{(v538*((v1962*v7142)+(v1961*(v7139*v7143))))}else{common.v60})}));
        let v7177=(if self.scalar_static_bool[46]{((-(common.v7*(self.scalar_static_f64[229]*common.v2477)))/(v1972*v1972))}else{v7137});
        let v7178=(if self.scalar_static_bool[46]{(self.scalar_static_f64[247]/v1972)}else{common.v60});
        let v7179=(if self.scalar_static_bool[46]{(self.scalar_static_f64[0]/v1972)}else{v7138});
        let v7180=(if self.scalar_static_bool[46]{common.v60}else{v7139});
        let v7185=(if v1976{common.v60}else{v7177});
        let v7186=(if v1976{common.v60}else{v7178});
        let v7187=(if v1976{common.v60}else{v7179});
        let v7188=(if v1976{common.v60}else{v7180});
        let v7189=(if v1982{common.v60}else{(if v1976{v7177}else{v7140})});
        let v7190=(if v1982{common.v60}else{(if v1976{v7178}else{common.v60})});
        let v7191=(if v1982{common.v60}else{(if v1976{v7179}else{v7141})});
        let v7192=(if v1982{common.v60}else{(if v1976{v7180}else{v7142})});
        let v7193=scalar_limexp_derivative(v1980);
        let v7220=(if self.scalar_static_bool[47]{common.v60}else{(if self.scalar_static_bool[46]{((v1986*(if self.scalar_static_bool[14]{(self.scalar_static_f64[73]*(v573*((self.scalar_static_f64[23]*common.v2486)+v2602)))}else{common.v60}))+(v575*((v1984*v7189)+(v1983*(v7185*v7193)))))}else{common.v60})});
        let v7221=(if self.scalar_static_bool[47]{common.v60}else{(if self.scalar_static_bool[46]{(v575*((v1984*v7190)+(v1983*(v7186*v7193))))}else{common.v60})});
        let v7222=(if self.scalar_static_bool[47]{common.v60}else{(if self.scalar_static_bool[46]{(v575*((v1984*v7191)+(v1983*(v7187*v7193))))}else{common.v60})});
        let v7223=(if self.scalar_static_bool[47]{common.v60}else{(if self.scalar_static_bool[46]{(v575*((v1984*v7192)+(v1983*(v7188*v7193))))}else{common.v60})});
        let v7255=(common.v2011*common.v2011);
        let v7291=(if common.v2019{common.v60}else{(if common.v2008{(((common.v2011*common.v7248)-(common.v2010*common.v7248))/v7255)}else{v4445})});
        let v7292=(if common.v2019{common.v60}else{(if common.v2008{(((common.v2011*common.v7249)-(common.v2010*common.v7249))/v7255)}else{v4446})});
        let v7293=(if common.v2019{common.v60}else{(if common.v2008{(((common.v2011*common.v7250)-(common.v2010*common.v7250))/v7255)}else{v4447})});
        let v7294=(if common.v2019{common.v60}else{(if common.v2008{(((common.v2011*common.v7251)-(common.v2010*common.v7251))/v7255)}else{v4448})});
        let v7326=(common.v2032*common.v2032);
        let v7368=(if common.v2046{common.v60}else{(if common.v2029{(((common.v2032*common.v7319)-(common.v2031*common.v7319))/v7326)}else{v4522})});
        let v7369=(if common.v2046{common.v60}else{(if common.v2029{(((common.v2032*common.v7320)-(common.v2031*common.v7320))/v7326)}else{v4523})});
        let v7370=(if common.v2046{common.v60}else{(if common.v2029{(((common.v2032*common.v7321)-(common.v2031*common.v7321))/v7326)}else{v4524})});
        let v7371=(if common.v2046{common.v60}else{(if common.v2029{(((common.v2032*common.v7322)-(common.v2031*common.v7322))/v7326)}else{v4525})});
        let v7512=((if common.v1993{((v2065*v7368)+(v2047*((v2064*v7291)+(v2020*(common.v569*(v2063*(self.scalar_static_f64[191]*common.v7418)))))))}else{(if common.v1269{((v1341*v4522)+(v1323*((v1340*v4445)+(v1296*(common.v946*(v1339*(self.scalar_static_f64[191]*common.v4572)))))))}else{(if common.v1118{((v1190*v3960)+(v1172*((v1189*v3883)+(v1145*(common.v1116*(v1188*(self.scalar_static_f64[198]*common.v4010)))))))}else{(if common.v953{((v1035*v3396)+(v1015*((v1034*v3319)+(v988*(common.v948*(v1033*(self.scalar_static_f64[198]*common.v3446)))))))}else{(if common.v777{((v864*v2921)+(v844*((v863*v2857)+(v815*(common.v772*(v862*(self.scalar_static_f64[191]*common.v2960)))))))}else{common.v60})})})})})+(if common.v1993{((v2072*(common.v2003*(v2070*(v2068*common.v7399))))+(v2071*(-v7368)))}else{(if common.v1269{((v1348*(common.v1279*(v1346*(v1344*common.v4553))))+(v1347*(-v4522)))}else{(if common.v1118{((v1197*(common.v1128*(v1195*(v1193*common.v3991))))+(v1196*(-v3960)))}else{(if common.v953{((v1042*(common.v971*(v1040*(v1038*common.v3427))))+(v1041*(-v3396)))}else{(if common.v777{((v871*(common.v797*(v869*(v867*common.v2945))))+(v870*(-v2921)))}else{common.v60})})})})}));
        let v7513=((if common.v1993{((v2065*v7369)+(v2047*((v2064*v7292)+(v2020*((v2063*common.v2600)+(common.v569*(v2063*(self.scalar_static_f64[191]*common.v7419))))))))}else{(if common.v1269{((v1341*v4523)+(v1323*((v1340*v4446)+(v1296*((v1339*common.v3246)+(common.v946*(v1339*(self.scalar_static_f64[191]*common.v4573))))))))}else{(if common.v1118{((v1190*v3961)+(v1172*((v1189*v3884)+(v1145*((v1188*common.v3818)+(common.v1116*(v1188*(self.scalar_static_f64[198]*common.v4011))))))))}else{(if common.v953{((v1035*v3397)+(v1015*((v1034*v3320)+(v988*((v1033*common.v3248)+(common.v948*(v1033*(self.scalar_static_f64[198]*common.v3447))))))))}else{(if common.v777{((v864*v2922)+(v844*((v863*v2858)+(v815*((v862*common.v2796)+(common.v772*(v862*(self.scalar_static_f64[191]*common.v2961))))))))}else{common.v60})})})})})+(if common.v1993{((v2072*((v2070*common.v7236)+(common.v2003*(v2070*(v2068*common.v7400)))))+(v2071*(-v7369)))}else{(if common.v1269{((v1348*((v1346*common.v4390)+(common.v1279*(v1346*(v1344*common.v4554)))))+(v1347*(-v4523)))}else{(if common.v1118{((v1197*((v1195*common.v3828)+(common.v1128*(v1195*(v1193*common.v3992)))))+(v1196*(-v3961)))}else{(if common.v953{((v1042*((v1040*common.v3264)+(common.v971*(v1040*(v1038*common.v3428)))))+(v1041*(-v3397)))}else{(if common.v777{((v871*((v869*common.v2812)+(common.v797*(v869*(v867*common.v2946)))))+(v870*(-v2922)))}else{common.v60})})})})}));
        let v7514=((if common.v1993{((v2065*v7370)+(v2047*((v2064*v7293)+(v2020*(common.v569*(v2063*(self.scalar_static_f64[191]*common.v7420)))))))}else{(if common.v1269{((v1341*v4524)+(v1323*((v1340*v4447)+(v1296*(common.v946*(v1339*(self.scalar_static_f64[191]*common.v4574)))))))}else{(if common.v1118{((v1190*v3962)+(v1172*((v1189*v3885)+(v1145*(common.v1116*(v1188*(self.scalar_static_f64[198]*common.v4012)))))))}else{(if common.v953{((v1035*v3398)+(v1015*((v1034*v3321)+(v988*(common.v948*(v1033*(self.scalar_static_f64[198]*common.v3448)))))))}else{(if common.v777{((v864*v2923)+(v844*((v863*v2859)+(v815*(common.v772*(v862*(self.scalar_static_f64[191]*common.v2962)))))))}else{common.v60})})})})})+(if common.v1993{((v2072*(common.v2003*(v2070*(v2068*common.v7401))))+(v2071*(-v7370)))}else{(if common.v1269{((v1348*(common.v1279*(v1346*(v1344*common.v4555))))+(v1347*(-v4524)))}else{(if common.v1118{((v1197*(common.v1128*(v1195*(v1193*common.v3993))))+(v1196*(-v3962)))}else{(if common.v953{((v1042*(common.v971*(v1040*(v1038*common.v3429))))+(v1041*(-v3398)))}else{(if common.v777{((v871*(common.v797*(v869*(v867*common.v2947))))+(v870*(-v2923)))}else{common.v60})})})})}));
        let v7515=((if common.v1993{((v2065*v7371)+(v2047*((v2064*v7294)+(v2020*(common.v569*(v2063*(self.scalar_static_f64[191]*common.v7421)))))))}else{(if common.v1269{((v1341*v4525)+(v1323*((v1340*v4448)+(v1296*(common.v946*(v1339*(self.scalar_static_f64[191]*common.v4575)))))))}else{(if common.v1118{((v1190*v3963)+(v1172*((v1189*v3886)+(v1145*(common.v1116*(v1188*(self.scalar_static_f64[198]*common.v4013)))))))}else{(if common.v953{((v1035*v3399)+(v1015*((v1034*v3322)+(v988*(common.v948*(v1033*(self.scalar_static_f64[198]*common.v3449)))))))}else{common.v60})})})})+(if common.v1993{((v2072*(common.v2003*(v2070*(v2068*common.v7402))))+(v2071*(-v7371)))}else{(if common.v1269{((v1348*(common.v1279*(v1346*(v1344*common.v4556))))+(v1347*(-v4525)))}else{(if common.v1118{((v1197*(common.v1128*(v1195*(v1193*common.v3994))))+(v1196*(-v3963)))}else{(if common.v953{((v1042*(common.v971*(v1040*(v1038*common.v3430))))+(v1041*(-v3399)))}else{common.v60})})})}));
        let v7671=(common.v2110*common.v2110);
        let v7689=(if common.v2102{(((common.v2110*common.v7646)-(common.v2113*common.v7631))/v7671)}else{v5534});
        let v7690=(if common.v2102{(((common.v2110*common.v7647)-(common.v2113*common.v7632))/v7671)}else{v5535});
        let v7691=(if common.v2102{(((common.v2110*common.v7648)-(common.v2113*common.v7633))/v7671)}else{v5536});
        let v7692=(if common.v2102{(((common.v2110*common.v7649)-(common.v2113*common.v7634))/v7671)}else{v5537});
        let v7693=(if common.v2102{(((common.v2110*common.v7650)-(common.v2113*common.v7635))/v7671)}else{v5538});
        let v7769=(if common.v2102{(common.v569*((if common.v2102{((v2124*v7689)+(v2118*(v2124*(self.scalar_static_f64[191]*common.v7712))))}else{(if common.v1540{((v1569*v5534)+(v1562*(v1569*(self.scalar_static_f64[204]*common.v5557))))}else{(if common.v1492{((v1521*v5310)+(v1514*(v1521*(self.scalar_static_f64[202]*common.v5333))))}else{(if common.v1381{((v1403*v4844)+(v1397*(v1403*(self.scalar_static_f64[191]*common.v4863))))}else{(if common.v1230{((v1252*v4282)+(v1246*(v1252*(self.scalar_static_f64[198]*common.v4301))))}else{(if common.v1077{((v1099*v3718)+(v1093*(v1099*(self.scalar_static_f64[198]*common.v3737))))}else{(if common.v906{((v929*v3170)+(v923*(v929*(self.scalar_static_f64[191]*common.v3185))))}else{common.v60})})})})})})})+(common.v782*(-v7689))))}else{(if v2100{common.v60}else{(if common.v1993{((if common.v1993{(common.v1998*(-v7291))}else{(if common.v1269{(common.v1274*(-v4445))}else{(if common.v1118{(common.v1123*(-v3883))}else{(if common.v953{(common.v964*(-v3319))}else{(if common.v777{(common.v790*(-v2857))}else{common.v60})})})})})+v7512)}else{common.v60})})});
        let v7770=(if common.v2102{((v2129*common.v2600)+(common.v569*((if common.v2102{((v2124*v7690)+(v2118*(v2124*(self.scalar_static_f64[191]*common.v7713))))}else{(if common.v1540{((v1569*v5535)+(v1562*(v1569*(self.scalar_static_f64[204]*common.v5558))))}else{(if common.v1492{((v1521*v5311)+(v1514*(v1521*(self.scalar_static_f64[202]*common.v5334))))}else{(if common.v1381{((v1403*v4845)+(v1397*(v1403*(self.scalar_static_f64[191]*common.v4864))))}else{(if common.v1230{((v1252*v4283)+(v1246*(v1252*(self.scalar_static_f64[198]*common.v4302))))}else{(if common.v1077{((v1099*v3719)+(v1093*(v1099*(self.scalar_static_f64[198]*common.v3738))))}else{(if common.v906{((v929*v3171)+(v923*(v929*(self.scalar_static_f64[191]*common.v3186))))}else{common.v60})})})})})})})+(common.v782*(-v7690)))))}else{(if v2100{common.v60}else{(if common.v1993{((if common.v1993{((v2075*common.v7230)+(common.v1998*(-v7292)))}else{(if common.v1269{((v1351*common.v4384)+(common.v1274*(-v4446)))}else{(if common.v1118{((v1200*common.v3822)+(common.v1123*(-v3884)))}else{(if common.v953{((v1045*common.v3254)+(common.v964*(-v3320)))}else{(if common.v777{((v874*common.v2802)+(common.v790*(-v2858)))}else{common.v60})})})})})+v7513)}else{common.v60})})});
        let v7771=(if common.v2102{(common.v569*((if common.v2102{((v2124*v7691)+(v2118*(v2124*(self.scalar_static_f64[191]*common.v7714))))}else{(if common.v1540{((v1569*v5536)+(v1562*(v1569*(self.scalar_static_f64[204]*common.v5559))))}else{(if common.v1492{((v1521*v5312)+(v1514*(v1521*(self.scalar_static_f64[202]*common.v5335))))}else{(if common.v1381{((v1403*v4846)+(v1397*(v1403*(self.scalar_static_f64[191]*common.v4865))))}else{(if common.v1230{((v1252*v4284)+(v1246*(v1252*(self.scalar_static_f64[198]*common.v4303))))}else{(if common.v1077{((v1099*v3720)+(v1093*(v1099*(self.scalar_static_f64[198]*common.v3739))))}else{(if common.v906{((v929*v3172)+(v923*(v929*(self.scalar_static_f64[191]*common.v3187))))}else{common.v60})})})})})})})+(common.v782*(-v7691))))}else{(if v2100{common.v60}else{(if common.v1993{((if common.v1993{(common.v1998*(-v7293))}else{(if common.v1269{(common.v1274*(-v4447))}else{(if common.v1118{(common.v1123*(-v3885))}else{(if common.v953{(common.v964*(-v3321))}else{(if common.v777{(common.v790*(-v2859))}else{common.v60})})})})})+v7514)}else{common.v60})})});
        let v7811=(if self.scalar_static_bool[8]{common.v2591}else{common.v60});
        let v7816=(v2139*v2139);
        let v7827=((-(v642*(if v2138{common.v60}else{(if common.v2102{(common.v569*((if common.v2102{((v2124*v7692)+(v2118*(v2124*(self.scalar_static_f64[191]*common.v7715))))}else{(if common.v1540{((v1569*v5537)+(v1562*(v1569*(self.scalar_static_f64[204]*common.v5560))))}else{(if common.v1492{((v1521*v5313)+(v1514*(v1521*(self.scalar_static_f64[202]*common.v5336))))}else{(if common.v1381{((v1403*v4847)+(v1397*(v1403*(self.scalar_static_f64[191]*common.v4866))))}else{(if common.v1230{((v1252*v4285)+(v1246*(v1252*(self.scalar_static_f64[198]*common.v4304))))}else{(if common.v1077{((v1099*v3721)+(v1093*(v1099*(self.scalar_static_f64[198]*common.v3740))))}else{common.v60})})})})})})+(common.v782*(-v7692))))}else{(if v2100{common.v60}else{(if common.v1993{((if common.v1993{(common.v1998*(-v7294))}else{(if common.v1269{(common.v1274*(-v4448))}else{(if common.v1118{(common.v1123*(-v3886))}else{(if common.v953{(common.v964*(-v3322))}else{common.v60})})})})+v7515)}else{common.v60})})})})))/v7816);
        let v7831=(if v2143{((-(v642*(if v2138{common.v60}else{v7769})))/v7816)}else{common.v60});
        let v7832=(if v2143{(((v2139*v2669)-(v642*(if v2138{common.v60}else{v7770})))/v7816)}else{common.v60});
        let v7833=(if v2143{((-(v642*(if v2138{common.v60}else{v7771})))/v7816)}else{common.v60});
        let v7834=(if v2143{v7827}else{common.v60});
        let v7835=(if v2143{((-(v642*(if v2138{common.v60}else{(if common.v2102{(common.v569*((if common.v2102{((v2124*v7693)+(v2118*(v2124*(self.scalar_static_f64[191]*common.v7716))))}else{(if common.v1540{((v1569*v5538)+(v1562*(v1569*(self.scalar_static_f64[204]*common.v5561))))}else{(if common.v1492{((v1521*v5314)+(v1514*(v1521*(self.scalar_static_f64[202]*common.v5337))))}else{common.v60})})})+(common.v782*(-v7693))))}else{common.v60})})))/v7816)}else{common.v60});
        let v7841=(if v2143{(((common.v569*v2669)-(v642*common.v2600))/(common.v569*common.v569))}else{common.v60});
        let v7842=(-v7831);
        let v7843=(-v7832);
        let v7844=(-v7833);
        let v7845=(-v7834);
        let v7846=(-v7835);
        let v7851=(v2147*v2147);
        let v7923=(v2141*v2141);
        let v7975=(if v2172{common.v60}else{(if v2143{((v2168*common.v6906)+(common.v1897*(if v2163{(v2164*(v2166*(v7842/v2141)))}else{(if v2149{((v2159*(if v2149{(v641*(v2152*(v7842/v2147)))}else{common.v60}))+(v2154*(v2157*(v7831/v2147))))}else{common.v60})})))}else{common.v60})});
        let v7976=(if v2172{common.v60}else{(if v2143{((v2168*common.v6907)+(common.v1897*(if v2163{((v2166*((v2141*v2668)+(v641*v7811)))+(v2164*(v2166*(((v2141*v7843)-(v2150*v7811))/v7923))))}else{(if v2149{((v2159*(if v2149{((v2152*v2668)+(v641*(v2152*(((v2147*v7843)-(v2150*v7841))/v7851))))}else{common.v60}))+(v2154*(v7841+((v2157*(((v2147*v7832)-(v2145*v7841))/v7851))+(v2156*(v7811-v7841))))))}else{common.v60})})))}else{common.v60})});
        let v7977=(if v2172{common.v60}else{(if v2143{((v2168*common.v6908)+(common.v1897*(if v2163{((v2166*(v641*self.scalar_static_f64[255]))+(v2164*(v2166*(((v2141*v7844)-(v2150*self.scalar_static_f64[255]))/v7923))))}else{(if v2149{((v2159*(if v2149{(v641*(v2152*(v7844/v2147)))}else{common.v60}))+(v2154*((v2157*(v7833/v2147))+(v2156*self.scalar_static_f64[255]))))}else{common.v60})})))}else{common.v60})});
        let v7978=(if v2172{common.v60}else{(if v2143{((v2168*common.v6909)+(common.v1897*(if v2163{((v2166*(v641*self.scalar_static_f64[256]))+(v2164*(v2166*(((v2141*v7845)-(v2150*self.scalar_static_f64[256]))/v7923))))}else{(if v2149{((v2159*(if v2149{(v641*(v2152*(v7845/v2147)))}else{common.v60}))+(v2154*((v2157*(v7834/v2147))+(v2156*self.scalar_static_f64[256]))))}else{common.v60})})))}else{common.v60})});
        let v7979=(if v2172{common.v60}else{(if v2143{((v2168*common.v6910)+(common.v1897*(if v2163{(v2164*(v2166*(v7846/v2141)))}else{(if v2149{((v2159*(if v2149{(v641*(v2152*(v7846/v2147)))}else{common.v60}))+(v2154*(v2157*(v7835/v2147))))}else{common.v60})})))}else{common.v60})});
        let v8022=(if v2174{((((common.v5434/self.scalar_static_f64[230])+(common.v5215/self.scalar_static_f64[231]))+(common.v6906/common.v1667))+(common.v6890/self.scalar_static_f64[214]))}else{common.v60});
        let v8023=(if v2174{((((common.v5439/self.scalar_static_f64[230])+(common.v5216/self.scalar_static_f64[231]))+(((common.v1667*common.v6907)-(common.v1897*common.v5878))/common.v5952))+(common.v6894/self.scalar_static_f64[214]))}else{common.v60});
        let v8024=(if v2174{((((common.v5440/self.scalar_static_f64[230])+(common.v5217/self.scalar_static_f64[231]))+(((common.v1667*common.v6908)-(common.v1897*common.v5879))/common.v5952))+(common.v6898/self.scalar_static_f64[214]))}else{common.v60});
        let v8025=(if v2174{((((common.v5441/self.scalar_static_f64[230])+(common.v5218/self.scalar_static_f64[231]))+(((common.v1667*common.v6909)-(common.v1897*common.v5880))/common.v5952))+(common.v6902/self.scalar_static_f64[214]))}else{common.v60});
        let v8026=(if v2174{(((common.v5442/self.scalar_static_f64[230])+(common.v6910/common.v1667))+(common.v6905/self.scalar_static_f64[214]))}else{common.v60});
        let v8027=(v2185*v8022);
        let v8029=(v2185*v8023);
        let v8031=(v2185*v8024);
        let v8033=(v2185*v8025);
        let v8035=(v2185*v8026);
        let v8037=(common.v91*v2188);
        let v8060=(v2191*v2191);
        let v8075=(if v2174{((-(v646*(if v2174{(common.v32*(v8022+((v8027+v8027)/v8037)))}else{common.v60})))/v8060)}else{common.v60});
        let v8076=(if v2174{(((v2191*(if self.scalar_static_bool[14]{(self.scalar_static_f64[95]*(v644*(self.scalar_static_f64[96]*common.v2486)))}else{common.v60}))-(v646*(if v2174{(common.v32*(v8023+((v8029+v8029)/v8037)))}else{common.v60})))/v8060)}else{common.v60});
        let v8077=(if v2174{((-(v646*(if v2174{(common.v32*(v8024+((v8031+v8031)/v8037)))}else{common.v60})))/v8060)}else{common.v60});
        let v8078=(if v2174{((-(v646*(if v2174{(common.v32*(v8025+((v8033+v8033)/v8037)))}else{common.v60})))/v8060)}else{common.v60});
        let v8079=(if v2174{((-(v646*(if v2174{(common.v32*(v8026+((v8035+v8035)/v8037)))}else{common.v60})))/v8060)}else{common.v60});
        let v8105=(if v2195{(common.v446*(v1991*(self.scalar_static_f64[232]*v8075)))}else{common.v60});
        let v8106=(if v2195{((v2198*common.v2481)+(common.v446*((v2197*(v7167+v7220))+(v1991*(self.scalar_static_f64[232]*v8076)))))}else{common.v60});
        let v8107=(if v2195{(common.v446*((v2197*v7221)+(v1991*(self.scalar_static_f64[232]*v8077))))}else{common.v60});
        let v8108=(if v2195{(common.v446*((v2197*(v7168+v7222))+(v1991*(self.scalar_static_f64[232]*v8078))))}else{common.v60});
        let v8109=(if v2195{(common.v446*((v2197*(v7169+v7223))+(v1991*(self.scalar_static_f64[232]*v8079))))}else{common.v60});
        let v8135=(if v2203{((v2205*v8075)+(v2193*(-(common.v32*v8105))))}else{v8075});
        let v8136=(if v2203{((v2205*v8076)+(v2193*(-(common.v32*v8106))))}else{v8076});
        let v8137=(if v2203{((v2205*v8077)+(v2193*(-(common.v32*v8107))))}else{v8077});
        let v8138=(if v2203{((v2205*v8078)+(v2193*(-(common.v32*v8108))))}else{v8078});
        let v8139=(if v2203{((v2205*v8079)+(v2193*(-(common.v32*v8109))))}else{v8079});
        let v8163=(v2200*v2200);
        let v8193=(if self.scalar_static_bool[48]{(self.scalar_static_f64[233]*common.v2477)}else{common.v60});
        let v8194=(self.scalar_static_f64[0]/v2221);
        let v8197=(v2221*v2221);
        let v8199=(self.scalar_static_f64[247]/v2221);
        let v8200=scalar_limexp_derivative(v2222);
        let v8210=scalar_limexp_derivative(v2225);
        let v8241=(if self.scalar_static_bool[50]{(self.scalar_static_f64[0]/v2235)}else{common.v60});
        let v8242=(if self.scalar_static_bool[50]{((-(common.v14*(self.scalar_static_f64[234]*common.v2477)))/(v2235*v2235))}else{v7185});
        let v8243=(if self.scalar_static_bool[50]{(self.scalar_static_f64[247]/v2235)}else{v7186});
        let v8244=(if self.scalar_static_bool[50]{common.v60}else{v7187});
        let v8245=(if self.scalar_static_bool[50]{common.v60}else{v7188});
        let v8261=scalar_limexp_derivative(v2243);
        let v8977=-0.0;
        let v9002=(v2217*v2217);

        stamper.stamp_current_node1_local(
            Some(6),
            Some(7),
            multiplicity * ((common.v9*common.v60)),
            7,
            multiplicity * (v8977),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(5),
            multiplicity * ((common.v6*common.v60)),
            5,
            multiplicity * (v8977),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if self.scalar_static_bool[48]{(v717*v2228)}else{common.v60})}))),
            [1, 3, 4, 5],
            [(self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if self.scalar_static_bool[48]{(v717*(if self.scalar_static_bool[48]{(v8194*v8200)}else{common.v60}))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if self.scalar_static_bool[48]{(v717*(-(if self.scalar_static_bool[48]{(v8194*v8210)}else{common.v60})))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if self.scalar_static_bool[48]{((v2228*(if self.scalar_static_bool[14]{(self.scalar_static_f64[120]*(v715*(v2602+v2731)))}else{common.v60}))+(v717*((if self.scalar_static_bool[48]{(((-(common.v4*v8193))/v8197)*v8200)}else{common.v60})-(if self.scalar_static_bool[48]{(((-(common.v14*v8193))/v8197)*v8210)}else{common.v60}))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if self.scalar_static_bool[48]{(v717*((if self.scalar_static_bool[48]{(v8199*v8200)}else{common.v60})-(if self.scalar_static_bool[48]{(v8199*v8210)}else{common.v60})))}else{common.v60})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(5),
            multiplicity * ((self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{(v713*v2249)}else{common.v60})}))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{(v713*((v2247*(if v2245{common.v60}else{(if v2239{v8241}else{common.v60})}))+(v2246*((if v2239{common.v60}else{v8241})*v8261))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{((v2249*(if self.scalar_static_bool[14]{(self.scalar_static_f64[119]*(v711*(v2731+(self.scalar_static_f64[14]*common.v2490))))}else{common.v60}))+(v713*((v2247*(if v2245{common.v60}else{(if v2239{v8242}else{v7189})}))+(v2246*((if v2239{common.v60}else{v8242})*v8261)))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{(v713*((v2247*(if v2245{common.v60}else{(if v2239{v8243}else{v7190})}))+(v2246*((if v2239{common.v60}else{v8243})*v8261))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{(v713*((v2247*(if v2245{common.v60}else{(if v2239{v8244}else{v7191})}))+(v2246*((if v2239{common.v60}else{v8244})*v8261))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if self.scalar_static_bool[50]{(v713*((v2247*(if v2245{common.v60}else{(if v2239{v8245}else{v7192})}))+(v2246*((if v2239{common.v60}else{v8245})*v8261))))}else{common.v60})}))],
            [],
            [],
            multiplicity,
        );
        let v2428_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2428);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (v2428_ddt),
            [1, 3, 4, 5, 6, 7],
            [((common.v8931) * ddt_scale), ((common.v8932) * ddt_scale), ((common.v8933) * ddt_scale), ((common.v8934) * ddt_scale), ((common.v8935) * ddt_scale), ((common.v8936) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2429_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2429);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2429_ddt),
            [1, 4, 5, 6],
            [((common.v8937) * ddt_scale), ((common.v8938) * ddt_scale), ((common.v8939) * ddt_scale), ((common.v8940) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2430_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2430);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (v2430_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[272]) * ddt_scale)),
            5,
            multiplicity * (((self.scalar_static_f64[273]) * ddt_scale)),
        );
        let v2431_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2431);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2431_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[274]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[275]) * ddt_scale)),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(2),
            multiplicity * ((if self.scalar_static_bool[60]{(v18/v757)}else{common.v60})),
            2,
            multiplicity * ((if self.scalar_static_bool[60]{(v2455/v757)}else{common.v60})),
            4,
            multiplicity * ((if self.scalar_static_bool[60]{((-(v18*(if self.scalar_static_bool[14]{(self.scalar_static_f64[136]*(v755*(self.scalar_static_f64[137]*common.v2486)))}else{common.v60})))/(v757*v757))}else{common.v60})),
            7,
            multiplicity * ((if self.scalar_static_bool[60]{(common.v48/v757)}else{common.v60})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v60,
        );
        stamper.stamp_current_node3_local(
            Some(5),
            Some(0),
            multiplicity * ((if self.scalar_static_bool[61]{(v20/v749)}else{common.v60})),
            0,
            multiplicity * ((if self.scalar_static_bool[61]{(v2455/v749)}else{common.v60})),
            4,
            multiplicity * ((if self.scalar_static_bool[61]{((-(v20*(if self.scalar_static_bool[14]{(self.scalar_static_f64[132]*(v747*(self.scalar_static_f64[133]*common.v2486)))}else{common.v60})))/(v749*v749))}else{common.v60})),
            5,
            multiplicity * ((if self.scalar_static_bool[61]{(common.v48/v749)}else{common.v60})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(0),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v60,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(6),
            multiplicity * ((if self.scalar_static_bool[64]{(v21/v2217)}else{common.v60})),
            [1, 4, 5, 6, 7],
            [(if self.scalar_static_bool[64]{((v2217-(v21*(if v2215{common.v60}else{(if v2209{(((v2200*((v2211*v8135)+(v2207*(v8105/v2210))))-(v2212*v8105))/v8163)}else{v8135})})))/v9002)}else{common.v60}), (if self.scalar_static_bool[64]{((-(v21*((if self.scalar_static_bool[14]{(self.scalar_static_f64[134]*(v751*(self.scalar_static_f64[135]*common.v2486)))}else{common.v60})+(if v2215{common.v60}else{(if v2209{(((v2200*((v2211*v8136)+(v2207*(v8106/v2210))))-(v2212*v8106))/v8163)}else{v8136})}))))/v9002)}else{common.v60}), (if self.scalar_static_bool[64]{((-(v21*(if v2215{common.v60}else{(if v2209{(((v2200*((v2211*v8137)+(v2207*(v8107/v2210))))-(v2212*v8107))/v8163)}else{v8137})})))/v9002)}else{common.v60}), (if self.scalar_static_bool[64]{(((-v2217)-(v21*(if v2215{common.v60}else{(if v2209{(((v2200*((v2211*v8138)+(v2207*(v8108/v2210))))-(v2212*v8108))/v8163)}else{v8138})})))/v9002)}else{common.v60}), (if self.scalar_static_bool[64]{((-(v21*(if v2215{common.v60}else{(if v2209{(((v2200*((v2211*v8139)+(v2207*(v8109/v2210))))-(v2212*v8109))/v8163)}else{v8139})})))/v9002)}else{common.v60})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            common.v60,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * ((self.scalar_static_f64[0]*(v1990-v2173))),
            [1, 4, 5, 6, 7],
            [(self.scalar_static_f64[0]*(-v7975)), (self.scalar_static_f64[0]*(v7220-v7976)), (self.scalar_static_f64[0]*(v7221-v7977)), (self.scalar_static_f64[0]*(v7222-v7978)), (self.scalar_static_f64[0]*(v7223-v7979))],
            [],
            [],
            multiplicity,
        );
        let v2433_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2433);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2433_ddt),
            [1, 4, 5, 6, 7],
            [((common.v8950) * ddt_scale), ((common.v8951) * ddt_scale), ((common.v8952) * ddt_scale), ((common.v8953) * ddt_scale), ((common.v8954) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((self.scalar_static_f64[0]*v1969)),
            4,
            multiplicity * ((self.scalar_static_f64[0]*v7167)),
            6,
            multiplicity * ((self.scalar_static_f64[0]*v7168)),
            7,
            multiplicity * ((self.scalar_static_f64[0]*v7169)),
        );
        let v2435_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2435);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2435_ddt),
            [1, 4, 5, 6, 7, 8],
            [((common.v8958) * ddt_scale), ((common.v8959) * ddt_scale), ((common.v8960) * ddt_scale), ((common.v8961) * ddt_scale), ((common.v8962) * ddt_scale), ((self.scalar_static_f64[276]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[0]*(common.v2408-common.v1895))),
            [1, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[0]*(common.v8852-common.v6890)), (self.scalar_static_f64[0]*(common.v8853-common.v6894)), (self.scalar_static_f64[0]*(common.v8854-common.v6898)), (self.scalar_static_f64[0]*(common.v8855-common.v6902)), (self.scalar_static_f64[0]*(common.v8856-common.v6905)), self.scalar_static_f64[276]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            common.v60,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[71]{((common.v431/v764)-(if self.scalar_static_bool[55]{((common.v11*v1898)+(v2140*v2173))}else{common.v60}))}else{common.v60})),
            [1, 4, 5, 6, 7],
            [(if self.scalar_static_bool[71]{(-(if self.scalar_static_bool[55]{((common.v11*(common.v6906-common.v6890))+(v2140*v7975))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(((v764-(common.v431*(if self.scalar_static_bool[14]{((v762*(self.scalar_static_f64[138]*(v759*(self.scalar_static_f64[139]*common.v2486))))+(v760*(self.scalar_static_f64[140]*common.v2482)))}else{common.v60})))/(v764*v764))-(if self.scalar_static_bool[55]{((common.v11*(common.v6907-common.v6894))+((v2173*common.v2591)+(v2140*v7976)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if self.scalar_static_bool[55]{(((self.scalar_static_f64[0]*v1898)+(common.v11*(common.v6908-common.v6898)))+((self.scalar_static_f64[0]*v2173)+(v2140*v7977)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if self.scalar_static_bool[55]{(((v1898*self.scalar_static_f64[248])+(common.v11*(common.v6909-common.v6902)))+((v2173*self.scalar_static_f64[247])+(v2140*v7978)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if self.scalar_static_bool[55]{(((v1898*self.scalar_static_f64[247])+(common.v11*(common.v6910-common.v6905)))+(v2140*v7979))}else{common.v60}))}else{common.v60})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{v2451}else{common.v60})}else{common.v60})),
            4,
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{(self.scalar_static_f64[246]*ddt_scale)}else{common.v60})}else{common.v60})),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * ((if self.scalar_static_bool[59]{common.v2399}else{(if self.scalar_static_bool[58]{(common.v2400-common.v1924)}else{common.v60})})),
            [1, 4, 5, 6, 7, 8],
            [(if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8817-common.v7071)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8818-common.v7072)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8819-common.v7073)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8820-common.v7074)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8821-common.v7075)}else{common.v60})}), self.scalar_static_f64[265]],
            [],
            [],
            multiplicity,
        );
        let v2417_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2417);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v2417_ddt),
            [1, 4, 5, 6, 7, 8],
            [((common.v8891) * ddt_scale), ((common.v8892) * ddt_scale), ((common.v8893) * ddt_scale), ((common.v8894) * ddt_scale), ((common.v8895) * ddt_scale), ((self.scalar_static_f64[266]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * ((if self.scalar_static_bool[59]{common.v2407}else{(if self.scalar_static_bool[58]{(common.v2408-common.v1897)}else{common.v60})})),
            [1, 4, 5, 6, 7, 9],
            [(if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8852-common.v6906)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8853-common.v6907)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8854-common.v6908)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8855-common.v6909)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if self.scalar_static_bool[58]{(common.v8856-common.v6910)}else{common.v60})}), self.scalar_static_f64[265]],
            [],
            [],
            multiplicity,
        );
        let v2419_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2419);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * (v2419_ddt),
            [1, 4, 5, 6, 7, 9],
            [((common.v8902) * ddt_scale), ((common.v8903) * ddt_scale), ((common.v8904) * ddt_scale), ((common.v8905) * ddt_scale), ((common.v8906) * ddt_scale), ((self.scalar_static_f64[267]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(6),
            multiplicity * (common.v60),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(0),
            multiplicity * (common.v60),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(2),
            multiplicity * (common.v60),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (common.v60),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (common.v60),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (common.v60),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v2451=0.0;

        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[common.v8931, common.v8932, common.v8933, common.v8934, common.v8935, common.v8936],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6]],
            &[common.v8937, common.v8938, common.v8939, common.v8940],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[272]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[273]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[274]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[275]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[common.v8950, common.v8951, common.v8952, common.v8953, common.v8954],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[common.v8958, common.v8959, common.v8960, common.v8961, common.v8962, self.scalar_static_f64[276]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{(self.scalar_static_f64[246]*1.0)}else{common.v60})}else{common.v60})),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[common.v8891, common.v8892, common.v8893, common.v8894, common.v8895, self.scalar_static_f64[266]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[common.v8902, common.v8903, common.v8904, common.v8905, common.v8906, self.scalar_static_f64[267]],
            &[],
            &[],
            multiplicity,
        );
    }
}
