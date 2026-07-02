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
    v94: f64,
    v439: f64,
    v454: f64,
    v456: f64,
    v458: f64,
    v462: f64,
    v465: f64,
    v573: f64,
    v579: f64,
    v783: f64,
    v790: bool,
    v792: f64,
    v795: f64,
    v803: f64,
    v810: f64,
    v814: f64,
    v817: bool,
    v819: f64,
    v820: f64,
    v828: bool,
    v841: bool,
    v843: f64,
    v844: f64,
    v858: bool,
    v866: f64,
    v870: f64,
    v921: bool,
    v930: f64,
    v933: f64,
    v942: f64,
    v961: f64,
    v963: f64,
    v970: bool,
    v972: f64,
    v981: f64,
    v988: f64,
    v994: bool,
    v996: f64,
    v997: f64,
    v1005: bool,
    v1016: bool,
    v1018: f64,
    v1019: f64,
    v1033: bool,
    v1041: f64,
    v1045: f64,
    v1096: bool,
    v1104: f64,
    v1107: f64,
    v1116: f64,
    v1135: f64,
    v1138: bool,
    v1139: f64,
    v1143: f64,
    v1148: f64,
    v1154: bool,
    v1156: f64,
    v1157: f64,
    v1165: bool,
    v1176: bool,
    v1178: f64,
    v1179: f64,
    v1193: bool,
    v1201: f64,
    v1205: f64,
    v1252: bool,
    v1260: f64,
    v1263: f64,
    v1272: f64,
    v1292: bool,
    v1293: f64,
    v1297: f64,
    v1302: f64,
    v1308: bool,
    v1310: f64,
    v1311: f64,
    v1319: bool,
    v1330: bool,
    v1332: f64,
    v1333: f64,
    v1347: bool,
    v1355: f64,
    v1359: f64,
    v1406: bool,
    v1414: f64,
    v1417: f64,
    v1426: f64,
    v1517: f64,
    v1519: f64,
    v1533: f64,
    v1536: f64,
    v1545: f64,
    v1565: f64,
    v1568: bool,
    v1582: f64,
    v1585: f64,
    v1594: f64,
    v1698: f64,
    v1775: f64,
    v1938: f64,
    v1941: f64,
    v1968: f64,
    v2043: f64,
    v2044: bool,
    v2045: f64,
    v2049: f64,
    v2054: f64,
    v2060: bool,
    v2062: f64,
    v2063: f64,
    v2071: bool,
    v2082: bool,
    v2084: f64,
    v2085: f64,
    v2099: bool,
    v2107: f64,
    v2111: f64,
    v2155: bool,
    v2163: f64,
    v2166: f64,
    v2175: f64,
    v2466: f64,
    v2467: f64,
    v2474: f64,
    v2475: f64,
    v2484: f64,
    v2486: f64,
    v2495: f64,
    v2496: f64,
    v2497: f64,
    v2498: f64,
    v2500: f64,
    v2502: f64,
    v2521: f64,
    v2549: f64,
    v2553: f64,
    v2554: f64,
    v2558: f64,
    v2562: f64,
    v2663: f64,
    v2672: f64,
    v2868: f64,
    v2874: f64,
    v2884: f64,
    v2896: f64,
    v2897: f64,
    v2898: f64,
    v2953: f64,
    v2954: f64,
    v2955: f64,
    v3017: f64,
    v3018: f64,
    v3019: f64,
    v3032: f64,
    v3033: f64,
    v3034: f64,
    v3206: f64,
    v3207: f64,
    v3208: f64,
    v3215: f64,
    v3216: f64,
    v3217: f64,
    v3257: f64,
    v3258: f64,
    v3259: f64,
    v3318: f64,
    v3320: f64,
    v3326: f64,
    v3336: f64,
    v3348: f64,
    v3349: f64,
    v3350: f64,
    v3351: f64,
    v3419: f64,
    v3420: f64,
    v3421: f64,
    v3422: f64,
    v3499: f64,
    v3500: f64,
    v3501: f64,
    v3502: f64,
    v3518: f64,
    v3519: f64,
    v3520: f64,
    v3521: f64,
    v3743: f64,
    v3744: f64,
    v3745: f64,
    v3746: f64,
    v3755: f64,
    v3756: f64,
    v3757: f64,
    v3758: f64,
    v3809: f64,
    v3810: f64,
    v3811: f64,
    v3812: f64,
    v3890: f64,
    v3894: f64,
    v3900: f64,
    v3912: f64,
    v3913: f64,
    v3914: f64,
    v3915: f64,
    v3983: f64,
    v3984: f64,
    v3985: f64,
    v3986: f64,
    v4063: f64,
    v4064: f64,
    v4065: f64,
    v4066: f64,
    v4082: f64,
    v4083: f64,
    v4084: f64,
    v4085: f64,
    v4307: f64,
    v4308: f64,
    v4309: f64,
    v4310: f64,
    v4319: f64,
    v4320: f64,
    v4321: f64,
    v4322: f64,
    v4373: f64,
    v4374: f64,
    v4375: f64,
    v4376: f64,
    v4456: f64,
    v4462: f64,
    v4474: f64,
    v4475: f64,
    v4476: f64,
    v4477: f64,
    v4545: f64,
    v4546: f64,
    v4547: f64,
    v4548: f64,
    v4625: f64,
    v4626: f64,
    v4627: f64,
    v4628: f64,
    v4644: f64,
    v4645: f64,
    v4646: f64,
    v4647: f64,
    v4869: f64,
    v4870: f64,
    v4871: f64,
    v4872: f64,
    v4881: f64,
    v4882: f64,
    v4883: f64,
    v4884: f64,
    v4935: f64,
    v4936: f64,
    v4937: f64,
    v4938: f64,
    v5287: f64,
    v5288: f64,
    v5289: f64,
    v5290: f64,
    v5324: f64,
    v5325: f64,
    v5326: f64,
    v5327: f64,
    v5328: f64,
    v5339: f64,
    v5340: f64,
    v5341: f64,
    v5342: f64,
    v5343: f64,
    v5405: f64,
    v5406: f64,
    v5407: f64,
    v5408: f64,
    v5409: f64,
    v5506: f64,
    v5511: f64,
    v5512: f64,
    v5513: f64,
    v5514: f64,
    v5548: f64,
    v5549: f64,
    v5550: f64,
    v5551: f64,
    v5552: f64,
    v5563: f64,
    v5564: f64,
    v5565: f64,
    v5566: f64,
    v5567: f64,
    v5629: f64,
    v5630: f64,
    v5631: f64,
    v5632: f64,
    v5633: f64,
    v5950: f64,
    v5951: f64,
    v5952: f64,
    v6024: f64,
    v6962: f64,
    v6966: f64,
    v6970: f64,
    v6974: f64,
    v6977: f64,
    v6978: f64,
    v6979: f64,
    v6980: f64,
    v6981: f64,
    v6982: f64,
    v7143: f64,
    v7144: f64,
    v7145: f64,
    v7146: f64,
    v7147: f64,
    v7302: f64,
    v7308: f64,
    v7320: f64,
    v7321: f64,
    v7322: f64,
    v7323: f64,
    v7391: f64,
    v7392: f64,
    v7393: f64,
    v7394: f64,
    v7471: f64,
    v7472: f64,
    v7473: f64,
    v7474: f64,
    v7490: f64,
    v7491: f64,
    v7492: f64,
    v7493: f64,
    v7703: f64,
    v7704: f64,
    v7705: f64,
    v7706: f64,
    v7707: f64,
    v7718: f64,
    v7719: f64,
    v7720: f64,
    v7721: f64,
    v7722: f64,
    v7784: f64,
    v7785: f64,
    v7786: f64,
    v7787: f64,
    v7788: f64,
    v8889: f64,
    v8890: f64,
    v8891: f64,
    v8892: f64,
    v8893: f64,
    v8924: f64,
    v8925: f64,
    v8926: f64,
    v8927: f64,
    v8928: f64,
    v8963: f64,
    v8964: f64,
    v8965: f64,
    v8966: f64,
    v8967: f64,
    v8974: f64,
    v8975: f64,
    v8976: f64,
    v8977: f64,
    v8978: f64,
    v9003: f64,
    v9004: f64,
    v9005: f64,
    v9006: f64,
    v9007: f64,
    v9008: f64,
    v9009: f64,
    v9010: f64,
    v9011: f64,
    v9012: f64,
    v9022: f64,
    v9023: f64,
    v9024: f64,
    v9025: f64,
    v9026: f64,
    v9030: f64,
    v9031: f64,
    v9032: f64,
    v9033: f64,
    v9034: f64,
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
        let v73=173.14999999999998;
        let v77=600.0;
        let v94=2.0;
        let v110=4.0;
        let v439=ctx.node_voltage(nodes[4]);
        let v441=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[303]+v439)}else{self.scalar_static_f64[307]});
        let v443=(if (v441<v73){v48}else{v60});
        let v444=((self.scalar_static_f64[148]!=0.0)&&(v443!=0.0));
        let v445=(if v444{v73}else{v441});
        let v450=(((if (v445>v77){v48}else{v60})!=0.0)&&((self.scalar_static_f64[148]!=0.0)&&(!(v443!=0.0))));
        let v451=(if v450{v77}else{v445});
        let v454=(if (self.scalar_static_f64[148]!=0.0){((v26*v451)/v28)}else{self.scalar_static_f64[309]});
        let v456=(if (self.scalar_static_f64[148]!=0.0){(v48/v454)}else{self.scalar_static_f64[310]});
        let v458=(if (self.scalar_static_f64[148]!=0.0){(v451-self.scalar_static_f64[2])}else{self.scalar_static_f64[311]});
        let v460=(if (self.scalar_static_f64[148]!=0.0){(v451/self.scalar_static_f64[2])}else{self.scalar_static_f64[312]});
        let v462=(if (self.scalar_static_f64[148]!=0.0){(v460).ln()}else{self.scalar_static_f64[313]});
        let v463=(v460-v48);
        let v465=(if (self.scalar_static_f64[148]!=0.0){(v456*v463)}else{self.scalar_static_f64[315]});
        let v475=(v48-v460);
        let v476=(self.scalar_static_f64[10]*v475);
        let v478=(self.scalar_static_f64[20]*v454);
        let v479=(v462*v478);
        let v481=(if (self.scalar_static_f64[148]!=0.0){(((v460*self.scalar_static_f64[156])+v476)-v479)}else{self.scalar_static_f64[476]});
        let v482=(v94*v454);
        let v483=(-v481);
        let v485=((v456*v483)).exp();
        let v488=((v48+(v110*v485))).sqrt();
        let v490=(v32*(v48+v488));
        let v491=(v490).ln();
        let v494=(if (self.scalar_static_f64[148]!=0.0){(v481+(v482*v491))}else{self.scalar_static_f64[334]});
        let v495=(self.scalar_static_f64[37]/v494);
        let v498=((self.scalar_static_f64[47]*(v495).ln())).exp();
        let v500=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[30]*v498)}else{self.scalar_static_f64[339]});
        let v503=(if (self.scalar_static_f64[148]!=0.0){((self.scalar_static_f64[48]*v494)/self.scalar_static_f64[37])}else{self.scalar_static_f64[341]});
        let v515=(if (self.scalar_static_f64[148]!=0.0){((v476+(v460*self.scalar_static_f64[164]))-v479)}else{v481});
        let v516=(-v515);
        let v518=((v456*v516)).exp();
        let v521=((v48+(v110*v518))).sqrt();
        let v523=(v32*(v48+v521));
        let v524=(v523).ln();
        let v527=(if (self.scalar_static_f64[148]!=0.0){(v515+(v482*v524))}else{self.scalar_static_f64[355]});
        let v528=(self.scalar_static_f64[49]/v527);
        let v531=((self.scalar_static_f64[58]*(v528).ln())).exp();
        let v533=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[30]*v531)}else{self.scalar_static_f64[360]});
        let v536=(if (self.scalar_static_f64[148]!=0.0){((self.scalar_static_f64[59]*v527)/self.scalar_static_f64[49])}else{self.scalar_static_f64[362]});
        let v558=(self.scalar_static_f64[13]*v475);
        let v561=(if (self.scalar_static_f64[148]!=0.0){(((v460*self.scalar_static_f64[172])+v558)-v479)}else{v515});
        let v562=(-v561);
        let v564=((v456*v562)).exp();
        let v567=((v48+(v110*v564))).sqrt();
        let v569=(v32*(v48+v567));
        let v570=(v569).ln();
        let v573=(if (self.scalar_static_f64[148]!=0.0){(v561+(v482*v570))}else{self.scalar_static_f64[387]});
        let v574=(self.scalar_static_f64[64]/v573);
        let v577=((self.scalar_static_f64[73]*(v574).ln())).exp();
        let v579=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[32]*v577)}else{self.scalar_static_f64[392]});
        let v589=(((self.scalar_static_f64[26]*v462)+(self.scalar_static_f64[7]*v465))).exp();
        let v591=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[75]*v589)}else{self.scalar_static_f64[402]});
        let v595=(((self.scalar_static_f64[77]*v462)-(self.scalar_static_f64[78]*v465))).exp();
        let v597=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[76]*v595)}else{self.scalar_static_f64[407]});
        let v599=((self.scalar_static_f64[80]*v462)).exp();
        let v601=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[79]*v599)}else{self.scalar_static_f64[410]});
        let v603=((self.scalar_static_f64[22]*v462)).exp();
        let v605=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[81]*v603)}else{self.scalar_static_f64[413]});
        let v607=(if (self.scalar_static_f64[148]!=0.0){(v48/v605)}else{self.scalar_static_f64[414]});
        let v610=(self.scalar_static_f64[82]*(v48+(self.scalar_static_f64[83]*v458)));
        let v623=(self.scalar_static_f64[89]*v458);
        let v627=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[87]*((v48+(self.scalar_static_f64[88]*v458))+(v458*v623)))}else{self.scalar_static_f64[430]});
        let v630=(self.scalar_static_f64[29]*v465);
        let v632=(((self.scalar_static_f64[28]*v462)-v630)).exp();
        let v636=(if self.scalar_static_bool[18]{self.scalar_static_f64[92]}else{(if self.scalar_static_bool[17]{(self.scalar_static_f64[92]*v632)}else{self.scalar_static_f64[437]})});
        let v638=((self.scalar_static_f64[94]*v462)).exp();
        let v640=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[93]*v638)}else{self.scalar_static_f64[440]});
        let v668=(if (self.scalar_static_f64[148]!=0.0){((v558+(v460*self.scalar_static_f64[180]))-v479)}else{v561});
        let v669=(-v668);
        let v671=((v456*v669)).exp();
        let v674=((v48+(v110*v671))).sqrt();
        let v676=(v32*(v48+v674));
        let v677=(v676).ln();
        let v680=(if (self.scalar_static_f64[148]!=0.0){(v668+(v482*v677))}else{self.scalar_static_f64[467]});
        let v681=(self.scalar_static_f64[101]/v680);
        let v684=((self.scalar_static_f64[111]*(v681).ln())).exp();
        let v686=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[110]*v684)}else{self.scalar_static_f64[472]});
        let v699=(if (self.scalar_static_f64[148]!=0.0){(((v460*self.scalar_static_f64[188])+(self.scalar_static_f64[16]*v475))-v479)}else{v668});
        let v700=(-v699);
        let v702=((v456*v700)).exp();
        let v705=((v48+(v110*v702))).sqrt();
        let v707=(v32*(v48+v705));
        let v708=(v707).ln();
        let v711=(if (self.scalar_static_f64[148]!=0.0){(v699+(v482*v708))}else{self.scalar_static_f64[487]});
        let v712=(self.scalar_static_f64[112]/v711);
        let v715=((self.scalar_static_f64[122]*(v712).ln())).exp();
        let v717=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[121]*v715)}else{self.scalar_static_f64[492]});
        let v729=((self.scalar_static_f64[126]*v462)).exp();
        let v731=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[125]*v729)}else{self.scalar_static_f64[503]});
        let v732=(self.scalar_static_f64[78]*v456);
        let v734=((self.scalar_static_f64[128]*v462)).exp();
        let v735=(v734-v48);
        let v737=((v732*v735)).exp();
        let v739=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[127]/v737)}else{self.scalar_static_f64[510]});
        let v742=(self.scalar_static_f64[131]+(self.scalar_static_f64[132]*v458));
        let v748=((self.scalar_static_f64[133]*v462)).exp();
        let v749=(if self.scalar_static_bool[22]{v748}else{(if self.scalar_static_bool[21]{(v48+(v458*v742))}else{self.scalar_static_f64[518]})});
        let v751=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[134]*v749)}else{self.scalar_static_f64[519]});
        let v752=(self.scalar_static_f64[135]*v749);
        let v753=(v630).exp();
        let v755=(if (self.scalar_static_f64[148]!=0.0){(v752*v753)}else{self.scalar_static_f64[522]});
        let v777=(if (v686<=1e-30){v48}else{v60});
        let v783=(if (v777!=0.0){(v579*self.scalar_static_f64[190])}else{v60});
        let v788=(if (v783>v60){v48}else{v60});
        let v789=((v777!=0.0)&&(self.scalar_static_f64[192]!=0.0));
        let v790=((v788!=0.0)&&v789);
        let v792=(if v790{self.scalar_static_f64[193]}else{v60});
        let v793=(self.scalar_static_f64[191]-v573);
        let v794=(if v790{v793}else{v60});
        let v795=2.4;
        let v800=(v573*self.scalar_static_f64[196]);
        let v801=(if v790{v800}else{v60});
        let v803=(if v790{(v783*v795)}else{v60});
        let v804=(v792-self.scalar_static_f64[73]);
        let v805=(self.scalar_static_f64[191]/v573);
        let v806=(v805).ln();
        let v808=((v804*v806)).exp();
        let v810=(if v790{(v783*v808)}else{v60});
        let v811=(v801-v4);
        let v813=(if v790{(v456*v811)}else{v60});
        let v814=80.0;
        let v816=(if (v813<v814){v48}else{v60});
        let v817=(v790&&(v816!=0.0));
        let v818=(v813).exp();
        let v819=(if v817{v818}else{v60});
        let v820=(v48+v819);
        let v823=(v820).ln();
        let v828=(v790&&(!(v816!=0.0)));
        let v830=(if v828{v4}else{(if v817{(v801-(v454*v823))}else{v60})});
        let v831=0.1;
        let v833=(v110*v454);
        let v835=(if v790{((v794*v831)+v833)}else{v60});
        let v836=(v794+v830);
        let v838=(if v790{(v836/v835)}else{v60});
        let v840=(if (v838<v814){v48}else{v60});
        let v841=(v790&&(v840!=0.0));
        let v842=(v838).exp();
        let v843=(if v841{v842}else{v819});
        let v844=(v48+v843);
        let v850=(-(v794+v801));
        let v852=((v850/v835)).exp();
        let v853=((v844).ln()-v852);
        let v858=(v790&&(!(v840!=0.0)));
        let v860=(if v858{v830}else{(if v841{((-v794)+(v835*v853))}else{v60})});
        let v862=(if v790{(v4-v830)}else{v60});
        let v864=(v48-(v830/v573));
        let v866=(if v790{(v864).ln()}else{v60});
        let v868=(v48-(v860/v573));
        let v870=(if v790{(v868).ln()}else{v60});
        let v872=(if v790{self.scalar_static_f64[197]}else{v60});
        let v874=(if v790{(v48-v792)}else{v60});
        let v893=((v870*v872)).exp();
        let v894=(v48-v893);
        let v897=(if v790{((v783*v894)/v872)}else{v60});
        let v899=((v866*v874)).exp();
        let v900=(v48-v899);
        let v903=(if v790{((v810*v900)/v874)}else{v60});
        let v905=((v870*v874)).exp();
        let v906=(v48-v905);
        let v909=(if v790{((v810*v906)/v874)}else{v60});
        let v911=((v897+v903)-v909);
        let v916=(!(v788!=0.0));
        let v917=(v789&&v916);
        let v920=((v777!=0.0)&&self.scalar_static_bool[24]);
        let v921=((v788!=0.0)&&v920);
        let v922=(if v921{v800}else{v60});
        let v923=(v922-v4);
        let v925=(if v921{(v456*v923)}else{v60});
        let v927=1.921812;
        let v929=(((v925*v925)+v927)).sqrt();
        let v930=(if v921{v929}else{v60});
        let v933=(if v921{(v32*(v925+v930))}else{v60});
        let v936=(if v921{(v922-(v454*v933))}else{v60});
        let v940=(v48-(v936/v573));
        let v942=(if v921{(v940).ln()}else{v60});
        let v948=((self.scalar_static_f64[197]*v942)).exp();
        let v949=(v48-v948);
        let v952=(if v921{((v573*v949)/self.scalar_static_f64[197])}else{v60});
        let v955=(v952+(v795*(v4-v936)));
        let v958=(v916&&v920);
        let v960=(!(v777!=0.0));
        let v961=(if v960{v579}else{(if (v777!=0.0){(v579*self.scalar_static_f64[189])}else{v60})});
        let v963=(if v960{(v686*self.scalar_static_f64[189])}else{v60});
        let v968=(if (v963>v60){v48}else{v60});
        let v969=(v960&&(self.scalar_static_f64[200]!=0.0));
        let v970=((v968!=0.0)&&v969);
        let v972=(if v970{self.scalar_static_f64[201]}else{v792});
        let v973=(self.scalar_static_f64[199]-v680);
        let v974=(if v970{v973}else{v794});
        let v978=(v680*self.scalar_static_f64[204]);
        let v979=(if v970{v978}else{v801});
        let v981=(if v970{(v795*v963)}else{v803});
        let v982=(v972-self.scalar_static_f64[111]);
        let v983=(self.scalar_static_f64[199]/v680);
        let v984=(v983).ln();
        let v986=((v982*v984)).exp();
        let v988=(if v970{(v963*v986)}else{v810});
        let v989=(v979-v7);
        let v991=(if v970{(v456*v989)}else{v813});
        let v993=(if (v991<v814){v48}else{v60});
        let v994=(v970&&(v993!=0.0));
        let v995=(v991).exp();
        let v996=(if v994{v995}else{v843});
        let v997=(v48+v996);
        let v1000=(v997).ln();
        let v1005=(v970&&(!(v993!=0.0)));
        let v1007=(if v1005{v7}else{(if v994{(v979-(v454*v1000))}else{v830})});
        let v1010=(if v970{(v833+(v831*v974))}else{v835});
        let v1011=(v974+v1007);
        let v1013=(if v970{(v1011/v1010)}else{v838});
        let v1015=(if (v1013<v814){v48}else{v60});
        let v1016=(v970&&(v1015!=0.0));
        let v1017=(v1013).exp();
        let v1018=(if v1016{v1017}else{v996});
        let v1019=(v48+v1018);
        let v1025=(-(v974+v979));
        let v1027=((v1025/v1010)).exp();
        let v1028=((v1019).ln()-v1027);
        let v1033=(v970&&(!(v1015!=0.0)));
        let v1035=(if v1033{v1007}else{(if v1016{((-v974)+(v1010*v1028))}else{v860})});
        let v1037=(if v970{(v7-v1007)}else{v862});
        let v1039=(v48-(v1007/v680));
        let v1041=(if v970{(v1039).ln()}else{v866});
        let v1043=(v48-(v1035/v680));
        let v1045=(if v970{(v1043).ln()}else{v870});
        let v1047=(if v970{self.scalar_static_f64[205]}else{v872});
        let v1049=(if v970{(v48-v972)}else{v874});
        let v1068=((v1045*v1047)).exp();
        let v1069=(v48-v1068);
        let v1072=(if v970{((v963*v1069)/v1047)}else{v897});
        let v1074=((v1041*v1049)).exp();
        let v1075=(v48-v1074);
        let v1078=(if v970{((v988*v1075)/v1049)}else{v903});
        let v1080=((v1045*v1049)).exp();
        let v1081=(v48-v1080);
        let v1084=(if v970{((v988*v1081)/v1049)}else{v909});
        let v1086=((v1072+v1078)-v1084);
        let v1091=(!(v968!=0.0));
        let v1092=(v969&&v1091);
        let v1095=(v960&&self.scalar_static_bool[26]);
        let v1096=((v968!=0.0)&&v1095);
        let v1097=(if v1096{v978}else{v922});
        let v1098=(v1097-v7);
        let v1100=(if v1096{(v456*v1098)}else{v925});
        let v1103=((v927+(v1100*v1100))).sqrt();
        let v1104=(if v1096{v1103}else{v930});
        let v1107=(if v1096{(v32*(v1100+v1104))}else{v933});
        let v1110=(if v1096{(v1097-(v454*v1107))}else{v936});
        let v1114=(v48-(v1110/v680));
        let v1116=(if v1096{(v1114).ln()}else{v942});
        let v1122=((self.scalar_static_f64[205]*v1116)).exp();
        let v1123=(v48-v1122);
        let v1126=(if v1096{((v680*v1123)/self.scalar_static_f64[205])}else{v952});
        let v1129=(v1126+(v795*(v7-v1110)));
        let v1132=(v1091&&v1095);
        let v1135=(if v960{(v686*self.scalar_static_f64[190])}else{v783});
        let v1137=(if (v1135>v60){v48}else{v60});
        let v1138=(v969&&(v1137!=0.0));
        let v1139=(if v1138{self.scalar_static_f64[201]}else{v972});
        let v1140=(if v1138{v973}else{v974});
        let v1141=(if v1138{v978}else{v979});
        let v1143=(if v1138{(v795*v1135)}else{v981});
        let v1144=(v1139-self.scalar_static_f64[111]);
        let v1146=((v984*v1144)).exp();
        let v1148=(if v1138{(v1135*v1146)}else{v988});
        let v1149=(v1141-v4);
        let v1151=(if v1138{(v456*v1149)}else{v991});
        let v1153=(if (v1151<v814){v48}else{v60});
        let v1154=(v1138&&(v1153!=0.0));
        let v1155=(v1151).exp();
        let v1156=(if v1154{v1155}else{v1018});
        let v1157=(v48+v1156);
        let v1160=(v1157).ln();
        let v1165=(v1138&&(!(v1153!=0.0)));
        let v1167=(if v1165{v4}else{(if v1154{(v1141-(v454*v1160))}else{v1007})});
        let v1170=(if v1138{(v833+(v831*v1140))}else{v1010});
        let v1171=(v1140+v1167);
        let v1173=(if v1138{(v1171/v1170)}else{v1013});
        let v1175=(if (v1173<v814){v48}else{v60});
        let v1176=(v1138&&(v1175!=0.0));
        let v1177=(v1173).exp();
        let v1178=(if v1176{v1177}else{v1156});
        let v1179=(v48+v1178);
        let v1185=(-(v1140+v1141));
        let v1187=((v1185/v1170)).exp();
        let v1188=((v1179).ln()-v1187);
        let v1193=(v1138&&(!(v1175!=0.0)));
        let v1195=(if v1193{v1167}else{(if v1176{((-v1140)+(v1170*v1188))}else{v1035})});
        let v1197=(if v1138{(v4-v1167)}else{v1037});
        let v1199=(v48-(v1167/v680));
        let v1201=(if v1138{(v1199).ln()}else{v1041});
        let v1203=(v48-(v1195/v680));
        let v1205=(if v1138{(v1203).ln()}else{v1045});
        let v1206=(if v1138{self.scalar_static_f64[205]}else{v1047});
        let v1208=(if v1138{(v48-v1139)}else{v1049});
        let v1226=((v1205*v1206)).exp();
        let v1227=(v48-v1226);
        let v1230=(if v1138{((v1135*v1227)/v1206)}else{v1072});
        let v1232=((v1201*v1208)).exp();
        let v1233=(v48-v1232);
        let v1236=(if v1138{((v1148*v1233)/v1208)}else{v1078});
        let v1238=((v1205*v1208)).exp();
        let v1239=(v48-v1238);
        let v1242=(if v1138{((v1148*v1239)/v1208)}else{v1084});
        let v1244=((v1230+v1236)-v1242);
        let v1249=(!(v1137!=0.0));
        let v1250=(v969&&v1249);
        let v1252=(v1095&&(v1137!=0.0));
        let v1253=(if v1252{v978}else{v1097});
        let v1254=(v1253-v4);
        let v1256=(if v1252{(v456*v1254)}else{v1100});
        let v1259=((v927+(v1256*v1256))).sqrt();
        let v1260=(if v1252{v1259}else{v1104});
        let v1263=(if v1252{(v32*(v1256+v1260))}else{v1107});
        let v1266=(if v1252{(v1253-(v454*v1263))}else{v1110});
        let v1270=(v48-(v1266/v680));
        let v1272=(if v1252{(v1270).ln()}else{v1116});
        let v1278=((self.scalar_static_f64[205]*v1272)).exp();
        let v1279=(v48-v1278);
        let v1282=(if v1252{((v680*v1279)/self.scalar_static_f64[205])}else{v1126});
        let v1285=(v1282+(v795*(v4-v1266)));
        let v1288=(v1095&&v1249);
        let v1290=(v961>v60);
        let v1291=(if v1290{v48}else{v60});
        let v1292=((self.scalar_static_f64[192]!=0.0)&&(v1291!=0.0));
        let v1293=(if v1292{self.scalar_static_f64[193]}else{v1139});
        let v1294=(if v1292{v793}else{v1140});
        let v1295=(if v1292{v800}else{v1141});
        let v1296=(v795*v961);
        let v1297=(if v1292{v1296}else{v1143});
        let v1298=(v1293-self.scalar_static_f64[73]);
        let v1300=((v806*v1298)).exp();
        let v1302=(if v1292{(v961*v1300)}else{v1148});
        let v1303=(v1295-v7);
        let v1305=(if v1292{(v456*v1303)}else{v1151});
        let v1307=(if (v1305<v814){v48}else{v60});
        let v1308=(v1292&&(v1307!=0.0));
        let v1309=(v1305).exp();
        let v1310=(if v1308{v1309}else{v1178});
        let v1311=(v48+v1310);
        let v1314=(v1311).ln();
        let v1319=(v1292&&(!(v1307!=0.0)));
        let v1321=(if v1319{v7}else{(if v1308{(v1295-(v454*v1314))}else{v1167})});
        let v1324=(if v1292{(v833+(v831*v1294))}else{v1170});
        let v1325=(v1294+v1321);
        let v1327=(if v1292{(v1325/v1324)}else{v1173});
        let v1329=(if (v1327<v814){v48}else{v60});
        let v1330=(v1292&&(v1329!=0.0));
        let v1331=(v1327).exp();
        let v1332=(if v1330{v1331}else{v1310});
        let v1333=(v48+v1332);
        let v1339=(-(v1294+v1295));
        let v1341=((v1339/v1324)).exp();
        let v1342=((v1333).ln()-v1341);
        let v1347=(v1292&&(!(v1329!=0.0)));
        let v1349=(if v1347{v1321}else{(if v1330{((-v1294)+(v1324*v1342))}else{v1195})});
        let v1351=(if v1292{(v7-v1321)}else{v1197});
        let v1353=(v48-(v1321/v573));
        let v1355=(if v1292{(v1353).ln()}else{v1201});
        let v1357=(v48-(v1349/v573));
        let v1359=(if v1292{(v1357).ln()}else{v1205});
        let v1360=(if v1292{self.scalar_static_f64[197]}else{v1206});
        let v1362=(if v1292{(v48-v1293)}else{v1208});
        let v1380=((v1359*v1360)).exp();
        let v1381=(v48-v1380);
        let v1384=(if v1292{((v961*v1381)/v1360)}else{v1230});
        let v1386=((v1355*v1362)).exp();
        let v1387=(v48-v1386);
        let v1390=(if v1292{((v1302*v1387)/v1362)}else{v1236});
        let v1392=((v1359*v1362)).exp();
        let v1393=(v48-v1392);
        let v1396=(if v1292{((v1302*v1393)/v1362)}else{v1242});
        let v1398=((v1384+v1390)-v1396);
        let v1403=(!(v1291!=0.0));
        let v1404=((self.scalar_static_f64[192]!=0.0)&&v1403);
        let v1406=(self.scalar_static_bool[24]&&(v1291!=0.0));
        let v1407=(if v1406{v800}else{v1253});
        let v1408=(v1407-v7);
        let v1410=(if v1406{(v456*v1408)}else{v1256});
        let v1413=((v927+(v1410*v1410))).sqrt();
        let v1414=(if v1406{v1413}else{v1260});
        let v1417=(if v1406{(v32*(v1410+v1414))}else{v1263});
        let v1420=(if v1406{(v1407-(v454*v1417))}else{v1266});
        let v1424=(v48-(v1420/v573));
        let v1426=(if v1406{(v1424).ln()}else{v1272});
        let v1432=((self.scalar_static_f64[197]*v1426)).exp();
        let v1433=(v48-v1432);
        let v1436=(if v1406{((v573*v1433)/self.scalar_static_f64[197])}else{v1282});
        let v1439=(v1436+(v795*(v7-v1420)));
        let v1442=(self.scalar_static_bool[24]&&v1403);
        let v1443=(if v1442{v60}else{(if v1406{(v961*v1439)}else{(if v1404{v60}else{(if v1292{((v573*v1398)+(v1297*v1351))}else{v60})})})});
        let v1445=(if (v1291!=0.0){v800}else{v60});
        let v1446=(v1445-v7);
        let v1448=(if (v1291!=0.0){(v456*v1446)}else{v60});
        let v1451=((v927+(v1448*v1448))).sqrt();
        let v1452=(if (v1291!=0.0){v1451}else{v60});
        let v1455=(if (v1291!=0.0){(v32*(v1448+v1452))}else{v60});
        let v1458=(if (v1291!=0.0){(v1445-(v454*v1455))}else{v60});
        let v1460=(if (v1291!=0.0){(v1455/v1452)}else{v60});
        let v1462=(v48-(v1458/v573));
        let v1465=((self.scalar_static_f64[198]*(v1462).ln())).exp();
        let v1466=(v961*v1465);
        let v1468=(v48-v1460);
        let v1472=(if v1403{v60}else{(if (v1291!=0.0){((v1460*v1466)+(v1296*v1468))}else{v60})});
        let v1476=(if self.scalar_static_bool[5]{(v11-(if self.scalar_static_bool[16]{v610}else{(if self.scalar_static_bool[15]{self.scalar_static_f64[82]}else{(if (self.scalar_static_f64[148]!=0.0){v610}else{self.scalar_static_f64[423]})})}))}else{(if (self.scalar_static_f64[85]!=0.0){((if self.scalar_static_bool[16]{self.scalar_static_f64[84]}else{(if self.scalar_static_bool[15]{(self.scalar_static_f64[84]*(v48-(self.scalar_static_f64[86]*v458)))}else{self.scalar_static_f64[424]})})-v7)}else{v60})});
        let v1478=((v456*v1476)-v48);
        let v1481=((v927+(v1478*v1478))).sqrt();
        let v1484=(v48+((v1478+v1481)/v94));
        let v1485=(v454*v1484);
        let v1486=(v1485/v601);
        let v1487=(v607*v1485);
        let v1491=((self.scalar_static_f64[207]*(v1486).ln())).exp();
        let v1492=(v48+v1491);
        let v1495=(((v1492).ln()/self.scalar_static_f64[207])).exp();
        let v1496=(v1487/v1495);
        let v1499=((v1485-v601)/self.scalar_static_f64[208]);
        let v1503=(((v1499*v1499)+self.scalar_static_f64[209])).sqrt();
        let v1506=(v48+(v32*(v1499+v1503)));
        let v1507=(v1496*v1506);
        let v1510=(if (v1290&&(v1472>v60)){v48}else{v60});
        let v1515=(!(v1510!=0.0));
        let v1516=(if v1515{v48}else{(if (v1510!=0.0){(v961/v1472)}else{v60})});
        let v1517=(if v1515{v60}else{(if (v1510!=0.0){(v1443/v961)}else{v1443})});
        let v1519=(if (v500>v60){v48}else{v60});
        let v1523=(((-(v503).ln())/self.scalar_static_f64[47])).exp();
        let v1524=(v48-v1523);
        let v1526=(if (v1519!=0.0){(v494*v1524)}else{v1407});
        let v1527=(v1526-v10);
        let v1529=(if (v1519!=0.0){(v456*v1527)}else{v1410});
        let v1532=((v927+(v1529*v1529))).sqrt();
        let v1533=(if (v1519!=0.0){v1532}else{v1414});
        let v1536=(if (v1519!=0.0){(v32*(v1529+v1533))}else{v1417});
        let v1539=(if (v1519!=0.0){(v1526-(v454*v1536))}else{v1420});
        let v1543=(v48-(v1539/v494));
        let v1545=(if (v1519!=0.0){(v1543).ln()}else{v1426});
        let v1553=((v1545*self.scalar_static_f64[211])).exp();
        let v1554=(v48-v1553);
        let v1557=(if (v1519!=0.0){((v494*v1554)/self.scalar_static_f64[211])}else{v1436});
        let v1558=(v10-v1539);
        let v1560=(v1557+(v503*v1558));
        let v1563=(!(v1519!=0.0));
        let v1564=(if v1563{v60}else{(if (v1519!=0.0){(v500*v1560)}else{v60})});
        let v1565=(v1564/v500);
        let v1567=(if (v533>v60){v48}else{v60});
        let v1568=((self.scalar_static_f64[130]!=0.0)&&(v1567!=0.0));
        let v1572=(((-(v536).ln())/self.scalar_static_f64[58])).exp();
        let v1573=(v48-v1572);
        let v1575=(if v1568{(v527*v1573)}else{v1526});
        let v1576=(v1575-v10);
        let v1578=(if v1568{(v456*v1576)}else{v1529});
        let v1581=((v927+(v1578*v1578))).sqrt();
        let v1582=(if v1568{v1581}else{v1533});
        let v1585=(if v1568{(v32*(v1578+v1582))}else{v1536});
        let v1588=(if v1568{(v1575-(v454*v1585))}else{v1539});
        let v1592=(v48-(v1588/v527));
        let v1594=(if v1568{(v1592).ln()}else{v1545});
        let v1602=((v1594*self.scalar_static_f64[213])).exp();
        let v1603=(v48-v1602);
        let v1606=(if v1568{((v527*v1603)/self.scalar_static_f64[213])}else{v1557});
        let v1607=(v10-v1588);
        let v1609=(v1606+(v536*v1607));
        let v1613=((self.scalar_static_f64[130]!=0.0)&&(!(v1567!=0.0)));
        let v1614=(if v1613{v60}else{(if v1568{(v533*v1609)}else{v60})});
        let v1619=(if self.scalar_static_bool[11]{v1565}else{(if (self.scalar_static_f64[130]!=0.0){(v1614/v533)}else{v60})});
        let v1620=(if self.scalar_static_bool[11]{v494}else{(if (self.scalar_static_f64[130]!=0.0){v527}else{v60})});
        let v1628=(if self.scalar_static_bool[28]{(v454*self.scalar_static_f64[218])}else{v60});
        let v1629=(v1620-v10);
        let v1631=(if self.scalar_static_bool[28]{(v1629/v1628)}else{v60});
        let v1634=((v927+(v1631*v1631))).sqrt();
        let v1635=(v1631+v1634);
        let v1639=(if self.scalar_static_bool[28]{(v1620-(v32*(v1628*v1635)))}else{v60});
        let v1641=(v48-(v1639/v1620));
        let v1644=((self.scalar_static_f64[215]*(v1641).ln())).exp();
        let v1645=(v48-v1644);
        let v1647=(if self.scalar_static_bool[28]{(v731*v1645)}else{v60});
        let v1651=(if ((v1647).abs()>=0.001){v48}else{v60});
        let v1652=(self.scalar_static_bool[28]&&(v1651!=0.0));
        let v1653=(v1647).exp();
        let v1654=(v1653-v48);
        let v1658=(self.scalar_static_bool[28]&&(!(v1651!=0.0)));
        let v1661=(if v1658{(v48+(v32*v1647))}else{(if v1652{(v1654/v1647)}else{self.scalar_static_f64[217]})});
        let v1662=(v1619*v1661);
        let v1668=20.0;
        let v1670=((((v48+(v1662/v739))+(v1517/self.scalar_static_f64[219]))*v1668)-v48);
        let v1671=0.025;
        let v1674=((v927+(v1670*v1670))).sqrt();
        let v1678=(v1671*(v48+((v1670+v1674)/v94)));
        let v1687=((v627+(self.scalar_static_f64[220]*(v1516-v48)))+(self.scalar_static_f64[221]*((v48/v1516)-v48)));
        let v1694=(v48+(if (self.scalar_static_f64[223]!=0.0){((v1687/v627)-v48)}else{v60}));
        let v1698=(if self.scalar_static_bool[30]{v597}else{(if (self.scalar_static_f64[223]!=0.0){(v597/v1694)}else{v60})});
        let v1701=(v454*self.scalar_static_f64[225]);
        let v1702=(v10/v1701);
        let v1704=(if (v1702>v814){v48}else{v60});
        let v1708=(if (v1704!=0.0){v814}else{v1702});
        let v1709=(!(v1704!=0.0));
        let v1710=(if v1709{v48}else{(if (v1704!=0.0){(v48+(v1702-v814))}else{v60})});
        let v1711=scalar_limexp(v1708);
        let v1712=(v1710*v1711);
        let v1713=(v591*v1712);
        let v1715=(v454*self.scalar_static_f64[226]);
        let v1716=(v7/v1715);
        let v1718=(if (v1716>v814){v48}else{v60});
        let v1722=(if (v1718!=0.0){v814}else{v1716});
        let v1723=(!(v1718!=0.0));
        let v1724=(if v1723{v48}else{(if (v1718!=0.0){(v48+(v1716-v814))}else{v60})});
        let v1725=scalar_limexp(v1722);
        let v1726=(v1724*v1725);
        let v1727=(v591*v1726);
        let v1732=((v1713/v1698)+(v1727/self.scalar_static_f64[224]));
        let v1733=0.6666;
        let v1734=(v1713/v1507);
        let v1735=(v1713*v1734);
        let v1736=(v755/v751);
        let v1737=(v1735*v1736);
        let v1740=((v1733*(v1737).ln())).exp();
        let v1743=(v1713/v751);
        let v1744=(v1732+v1743);
        let v1748=(if self.scalar_static_bool[32]{v1732}else{(if (self.scalar_static_f64[227]!=0.0){(v1732+v1740)}else{v60})});
        let v1749=(if self.scalar_static_bool[32]{v1744}else{(if (self.scalar_static_f64[227]!=0.0){(v1740+v1744)}else{v60})});
        let v1750=(v1678*v1678);
        let v1752=((v1748+v1750)).sqrt();
        let v1753=(v1678+v1752);
        let v1755=((v1749+v1750)).sqrt();
        let v1761=(if (((v1749-v1748)).abs()>1e-8){v48}else{v60});
        let v1763=(v1507/self.scalar_static_f64[228]);
        let v1764=(v1763/v1713);
        let v1767=(if (v1761!=0.0){(v48-(v1753*v1764))}else{v60});
        let v1768=((v1678+v1755)-v1753);
        let v1771=(if (v1761!=0.0){(v48+(v1764*v1768))}else{v60});
        let v1773=(if (v1761!=0.0){(v1767/v1771)}else{v60});
        let v1775=0.01;
        let v1777=(((v1773*v1773)+v1775)).sqrt();
        let v1779=2.004987562112089;
        let v1782=(!(v1761!=0.0));
        let v1783=(if v1782{v60}else{(if (v1761!=0.0){((v1773+v1777)/v1779)}else{v60})});
        let v1788=(v1743*v1783);
        let v1790=(v1732+(v1783*v1788));
        let v1796=((v1750+(if self.scalar_static_bool[35]{v1790}else{(if self.scalar_static_bool[34]{(v1740+v1790)}else{v60})}))).sqrt();
        let v1802=-2.0;
        let v1804=(if self.scalar_static_bool[36]{(v1678*v1802)}else{v60});
        let v1813=(if self.scalar_static_bool[41]{(-v1790)}else{v60});
        let v1814=(-v1713);
        let v1815=(v1713*v1814);
        let v1816=(v1815/v1507);
        let v1817=(v755*v1816);
        let v1821=(if self.scalar_static_bool[36]{(v1804*v1804)}else{v60});
        let v1824=(if self.scalar_static_bool[36]{(v1813-(self.scalar_static_f64[231]*v1821))}else{v60});
        let v1825=(v94*v1804);
        let v1827=27.0;
        let v1833=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{(v1817/v751)}else{v60})+(((v1821*v1825)/v1827)-(self.scalar_static_f64[231]*(v1804*v1813))))}else{v60});
        let v1835=0.25;
        let v1837=(v1824*v1824);
        let v1838=(v1824*v1837);
        let v1841=(if self.scalar_static_bool[36]{(((v1833*v1833)*v1835)+(v1838/v1827))}else{v60});
        let v1845=(if ((v1841).abs()<1e-10){v48}else{v60});
        let v1846=(self.scalar_static_bool[36]&&(v1845!=0.0));
        let v1847=(v43*v1833);
        let v1849=(self.scalar_static_f64[231]*v1804);
        let v1853=(if (v1841>v60){v48}else{v60});
        let v1855=(self.scalar_static_bool[36]&&(!(v1845!=0.0)));
        let v1856=((v1853!=0.0)&&v1855);
        let v1858=(v32*(-v1833));
        let v1859=(if v1856{v1858}else{v60});
        let v1860=(v1841).sqrt();
        let v1861=(if v1856{v1860}else{v60});
        let v1863=(if v1856{(v1859+v1861)}else{v1821});
        let v1865=(if (v1863>v60){v48}else{v60});
        let v1866=(v1856&&(v1865!=0.0));
        let v1869=((self.scalar_static_f64[231]*(v1863).ln())).exp();
        let v1872=(v1856&&(!(v1865!=0.0)));
        let v1873=(-v1863);
        let v1876=((self.scalar_static_f64[231]*(v1873).ln())).exp();
        let v1880=(if v1856{(v1859-v1861)}else{v1863});
        let v1882=(if (v1880>v60){v48}else{v60});
        let v1883=(v1856&&(v1882!=0.0));
        let v1886=((self.scalar_static_f64[231]*(v1880).ln())).exp();
        let v1889=(v1856&&(!(v1882!=0.0)));
        let v1890=(-v1880);
        let v1893=((self.scalar_static_f64[231]*(v1890).ln())).exp();
        let v1900=(v1855&&(!(v1853!=0.0)));
        let v1901=-27.0;
        let v1903=((v1901/v1838)).sqrt();
        let v1905=(if v1900{(v1858*v1903)}else{v1880});
        let v1907=(if v1900{(v1905*v1905)}else{v1859});
        let v1909=(if (v1905>=v60){v48}else{v60});
        let v1910=(v1900&&(v1909!=0.0));
        let v1911=1.5707963267948966;
        let v1912=(v48-v1907);
        let v1914=((v1907/v1912)).sqrt();
        let v1915=(v1914).atan();
        let v1919=(v1900&&(!(v1909!=0.0)));
        let v1921=(if v1919{(v1911+v1915)}else{(if v1910{(v1911-v1915)}else{v1905})});
        let v1922=-4.0;
        let v1925=((self.scalar_static_f64[231]*(v1824*v1922))).sqrt();
        let v1926=(self.scalar_static_f64[231]*v1921);
        let v1927=(v1926).cos();
        let v1932=(if self.scalar_static_bool[36]{(if v1900{(if v1900{((v1925*v1927)-v1849)}else{v1921})}else{(if v1856{(((if v1872{(-v1876)}else{(if v1866{v1869}else{v60})})+(if v1889{(-v1893)}else{(if v1883{v1886}else{v60})}))-v1849)}else{(if v1846{((v1847/v1824)-v1849)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v1678+v1796)}else{v60})});
        let v1933=1e-20;
        let v1935=(if (v1932<v1933){v48}else{v60});
        let v1936=(if (v1935!=0.0){v1933}else{v1932});
        let v1937=(v1713/v1936);
        let v1938=(v1727/v1936);
        let v1940=(if (v1937<v1933){v48}else{v60});
        let v1941=(if (v1940!=0.0){v1933}else{v1937});
        let v1945=(v48-(v1507/v1941));
        let v1949=(((v1945*v1945)+self.scalar_static_f64[233])).sqrt();
        let v1954=((v1945+v1949)/self.scalar_static_f64[236]);
        let v1955=(v640*v1954);
        let v1956=(v1954*v1955);
        let v1959=(v1941/v1507);
        let v1962=((self.scalar_static_f64[237]*(v1959).ln())).exp();
        let v1963=(v636*v1962);
        let v1968=((v1941*v1956)+((v1687*v1941)+((v1941*v1963)/self.scalar_static_f64[238])));
        let v2043=(if (v579>v60){v48}else{v60});
        let v2044=((self.scalar_static_f64[192]!=0.0)&&(v2043!=0.0));
        let v2045=(if v2044{self.scalar_static_f64[193]}else{v1293});
        let v2046=(if v2044{v793}else{v1294});
        let v2047=(if v2044{v800}else{v1295});
        let v2049=(if v2044{(v579*v795)}else{v1297});
        let v2050=(v2045-self.scalar_static_f64[73]);
        let v2052=((v806*v2050)).exp();
        let v2054=(if v2044{(v579*v2052)}else{v1302});
        let v2055=(v2047-v7);
        let v2057=(if v2044{(v456*v2055)}else{v1305});
        let v2059=(if (v2057<v814){v48}else{v60});
        let v2060=(v2044&&(v2059!=0.0));
        let v2061=(v2057).exp();
        let v2062=(if v2060{v2061}else{v1332});
        let v2063=(v48+v2062);
        let v2066=(v2063).ln();
        let v2071=(v2044&&(!(v2059!=0.0)));
        let v2073=(if v2071{v7}else{(if v2060{(v2047-(v454*v2066))}else{v1321})});
        let v2076=(if v2044{(v833+(v831*v2046))}else{v1324});
        let v2077=(v2046+v2073);
        let v2079=(if v2044{(v2077/v2076)}else{v1327});
        let v2081=(if (v2079<v814){v48}else{v60});
        let v2082=(v2044&&(v2081!=0.0));
        let v2083=(v2079).exp();
        let v2084=(if v2082{v2083}else{v2062});
        let v2085=(v48+v2084);
        let v2091=(-(v2046+v2047));
        let v2093=((v2091/v2076)).exp();
        let v2094=((v2085).ln()-v2093);
        let v2099=(v2044&&(!(v2081!=0.0)));
        let v2101=(if v2099{v2073}else{(if v2082{((-v2046)+(v2076*v2094))}else{v1349})});
        let v2105=(v48-(v2073/v573));
        let v2107=(if v2044{(v2105).ln()}else{v1355});
        let v2109=(v48-(v2101/v573));
        let v2111=(if v2044{(v2109).ln()}else{v1359});
        let v2112=(if v2044{self.scalar_static_f64[197]}else{v1360});
        let v2114=(if v2044{(v48-v2045)}else{v1362});
        let v2135=((v2111*v2112)).exp();
        let v2136=(v48-v2135);
        let v2141=((v2107*v2114)).exp();
        let v2142=(v48-v2141);
        let v2147=((v2111*v2114)).exp();
        let v2148=(v48-v2147);
        let v2155=(self.scalar_static_bool[24]&&(v2043!=0.0));
        let v2156=(if v2155{v800}else{v1575});
        let v2157=(v2156-v7);
        let v2159=(if v2155{(v456*v2157)}else{v1578});
        let v2162=((v927+(v2159*v2159))).sqrt();
        let v2163=(if v2155{v2162}else{v1582});
        let v2166=(if v2155{(v32*(v2159+v2163))}else{v1585});
        let v2169=(if v2155{(v2156-(v454*v2166))}else{v1588});
        let v2173=(v48-(v2169/v573));
        let v2175=(if v2155{(v2173).ln()}else{v1594});
        let v2186=((self.scalar_static_f64[197]*v2175)).exp();
        let v2187=(v48-v2186);
        let v2319=(if (v717>v60){v48}else{v60});
        let v2320=((self.scalar_static_f64[254]!=0.0)&&(v2319!=0.0));
        let v2322=(if v2320{self.scalar_static_f64[255]}else{v2045});
        let v2324=(if v2320{(self.scalar_static_f64[253]-v711)}else{v2046});
        let v2328=(v711*self.scalar_static_f64[258]);
        let v2329=(if v2320{v2328}else{v2047});
        let v2331=(if v2320{(v717*v795)}else{v2049});
        let v2332=(v2322-self.scalar_static_f64[122]);
        let v2333=(self.scalar_static_f64[253]/v711);
        let v2336=((v2332*(v2333).ln())).exp();
        let v2338=(if v2320{(v717*v2336)}else{v2054});
        let v2339=(v2329-v14);
        let v2341=(if v2320{(v456*v2339)}else{v2057});
        let v2343=(if (v2341<v814){v48}else{v60});
        let v2344=(v2320&&(v2343!=0.0));
        let v2345=(v2341).exp();
        let v2346=(if v2344{v2345}else{v2084});
        let v2347=(v48+v2346);
        let v2348=(v2347).ln();
        let v2353=(v2320&&(!(v2343!=0.0)));
        let v2354=(if v2353{v14}else{(if v2344{(v2329-(v454*v2348))}else{v2073})});
        let v2357=(if v2320{(v833+(v831*v2324))}else{v2076});
        let v2358=(v2324+v2354);
        let v2360=(if v2320{(v2358/v2357)}else{v2079});
        let v2362=(if (v2360<v814){v48}else{v60});
        let v2363=(v2320&&(v2362!=0.0));
        let v2364=(v2360).exp();
        let v2366=(v48+(if v2363{v2364}else{v2346}));
        let v2370=(-(v2324+v2329));
        let v2372=((v2370/v2357)).exp();
        let v2373=((v2366).ln()-v2372);
        let v2378=(v2320&&(!(v2362!=0.0)));
        let v2379=(if v2378{v2354}else{(if v2363{((-v2324)+(v2357*v2373))}else{v2101})});
        let v2381=(if v2320{(v14-v2354)}else{(if v2044{(v7-v2073)}else{v1351})});
        let v2383=(v48-(v2354/v711));
        let v2387=(v48-(v2379/v711));
        let v2389=(if v2320{(v2387).ln()}else{v2111});
        let v2391=(if v2320{self.scalar_static_f64[259]}else{v2112});
        let v2393=(if v2320{(v48-v2322)}else{v2114});
        let v2395=((v2389*v2391)).exp();
        let v2396=(v48-v2395);
        let v2401=(((if v2320{(v2383).ln()}else{v2107})*v2393)).exp();
        let v2402=(v48-v2401);
        let v2407=((v2389*v2393)).exp();
        let v2408=(v48-v2407);
        let v2413=(((if v2320{((v717*v2396)/v2391)}else{(if v2044{((v579*v2136)/v2112)}else{v1384})})+(if v2320{((v2338*v2402)/v2393)}else{(if v2044{((v2054*v2142)/v2114)}else{v1390})}))-(if v2320{((v2338*v2408)/v2393)}else{(if v2044{((v2054*v2148)/v2114)}else{v1396})}));
        let v2418=(!(v2319!=0.0));
        let v2419=((self.scalar_static_f64[254]!=0.0)&&v2418);
        let v2422=((v2319!=0.0)&&self.scalar_static_bool[53]);
        let v2423=(if v2422{v2328}else{v2156});
        let v2424=(v2423-v14);
        let v2426=(if v2422{(v456*v2424)}else{v2159});
        let v2429=((v927+(v2426*v2426))).sqrt();
        let v2433=(if v2422{(v32*(v2426+(if v2422{v2429}else{v2163})))}else{v2166});
        let v2436=(if v2422{(v2423-(v454*v2433))}else{v2169});
        let v2438=(v48-(v2436/v711));
        let v2442=((self.scalar_static_f64[259]*(if v2422{(v2438).ln()}else{v2175}))).exp();
        let v2443=(v48-v2442);
        let v2449=((if v2422{((v711*v2443)/self.scalar_static_f64[259])}else{(if v2155{((v573*v2187)/self.scalar_static_f64[197])}else{v1606})})+(v795*(v14-v2436)));
        let v2452=(v2418&&self.scalar_static_bool[53]);
        let v2466=ctx.node_voltage(nodes[8]);
        let v2467=(if (self.scalar_static_f64[262]!=0.0){v2466}else{v1968});
        let v2474=ctx.node_voltage(nodes[9]);
        let v2475=(if (self.scalar_static_f64[262]!=0.0){v2474}else{v1941});
        let v2484=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(v2467*self.scalar_static_f64[263]))}else{v60})});
        let v2486=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(v2475*self.scalar_static_f64[264]))}else{v60})});
        let v2495=(self.scalar_static_f64[0]*(if v2452{v60}else{(if v2422{(v717*v2449)}else{(if v2419{v60}else{(if v2320{((v711*v2413)+(v2331*v2381))}else{v60})})})}));
        let v2496=(self.scalar_static_f64[0]*(if v1288{v60}else{(if v1252{(v1135*v1285)}else{(if v1250{v60}else{(if v1138{((v680*v1244)+(v1143*v1197))}else{(if v958{v60}else{(if v921{(v783*v955)}else{(if v917{v60}else{(if v790{((v573*v911)+(v803*v862))}else{v60})})})})})})})}));
        let v2497=(self.scalar_static_f64[0]*(v4*self.scalar_static_f64[265]));
        let v2498=(self.scalar_static_f64[0]*((self.scalar_static_f64[0]*(v1-v15))*self.scalar_static_f64[266]));
        let v2500=(self.scalar_static_f64[0]*(((if v1132{v60}else{(if v1096{(v963*v1129)}else{(if v1092{v60}else{(if v970{((v680*v1086)+(v981*v1037))}else{v60})})})})+v1443)+(v1938*self.scalar_static_f64[239])));
        let v2502=(self.scalar_static_f64[0]*(v1564+v2467));
        let v2521=(v439*self.scalar_static_f64[270]);
        let v2546=(if v450{v60}else{(if v444{v60}else{self.scalar_static_f64[275]})});
        let v2549=(if (self.scalar_static_f64[148]!=0.0){((v26*v2546)/v28)}else{v60});
        let v2553=(if (self.scalar_static_f64[148]!=0.0){((-v2549)/(v454*v454))}else{v60});
        let v2554=(if (self.scalar_static_f64[148]!=0.0){v2546}else{v60});
        let v2556=(if (self.scalar_static_f64[148]!=0.0){(v2546/self.scalar_static_f64[2])}else{v60});
        let v2558=(if (self.scalar_static_f64[148]!=0.0){(v2556/v460)}else{v60});
        let v2562=(if (self.scalar_static_f64[148]!=0.0){((v463*v2553)+(v456*v2556))}else{v60});
        let v2564=(-v2556);
        let v2565=(self.scalar_static_f64[10]*v2564);
        let v2570=((v478*v2558)+(v462*(self.scalar_static_f64[20]*v2549)));
        let v2572=(if (self.scalar_static_f64[148]!=0.0){(((self.scalar_static_f64[156]*v2556)+v2565)-v2570)}else{v60});
        let v2573=(v94*v2549);
        let v2588=(if (self.scalar_static_f64[148]!=0.0){(v2572+((v491*v2573)+(v482*((v32*((v110*(v485*((v483*v2553)+(v456*(-v2572)))))/(v94*v488)))/v490))))}else{v60});
        let v2591=(v494*v494);
        let v2597=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[30]*(v498*(self.scalar_static_f64[47]*(((-(self.scalar_static_f64[37]*v2588))/v2591)/v495))))}else{v60});
        let v2600=(if (self.scalar_static_f64[148]!=0.0){((self.scalar_static_f64[48]*v2588)/self.scalar_static_f64[37])}else{v60});
        let v2604=(if (self.scalar_static_f64[148]!=0.0){((v2565+(self.scalar_static_f64[164]*v2556))-v2570)}else{v2572});
        let v2619=(if (self.scalar_static_f64[148]!=0.0){(v2604+((v524*v2573)+(v482*((v32*((v110*(v518*((v516*v2553)+(v456*(-v2604)))))/(v94*v521)))/v523))))}else{v60});
        let v2622=(v527*v527);
        let v2628=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[30]*(v531*(self.scalar_static_f64[58]*(((-(self.scalar_static_f64[49]*v2619))/v2622)/v528))))}else{v60});
        let v2631=(if (self.scalar_static_f64[148]!=0.0){((self.scalar_static_f64[59]*v2619)/self.scalar_static_f64[49])}else{v60});
        let v2645=(self.scalar_static_f64[13]*v2564);
        let v2648=(if (self.scalar_static_f64[148]!=0.0){(((self.scalar_static_f64[172]*v2556)+v2645)-v2570)}else{v2604});
        let v2663=(if (self.scalar_static_f64[148]!=0.0){(v2648+((v570*v2573)+(v482*((v32*((v110*(v564*((v562*v2553)+(v456*(-v2648)))))/(v94*v567)))/v569))))}else{v60});
        let v2666=(v573*v573);
        let v2672=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[32]*(v577*(self.scalar_static_f64[73]*(((-(self.scalar_static_f64[64]*v2663))/v2666)/v574))))}else{v60});
        let v2684=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[75]*(v589*((self.scalar_static_f64[26]*v2558)+(self.scalar_static_f64[7]*v2562))))}else{v60});
        let v2690=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[76]*(v595*((self.scalar_static_f64[77]*v2558)-(self.scalar_static_f64[78]*v2562))))}else{v60});
        let v2694=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[79]*(v599*(self.scalar_static_f64[80]*v2558)))}else{v60});
        let v2704=(self.scalar_static_f64[82]*(self.scalar_static_f64[83]*v2554));
        let v2720=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[87]*((self.scalar_static_f64[88]*v2554)+((v623*v2554)+(v458*(self.scalar_static_f64[89]*v2554)))))}else{v60});
        let v2722=(self.scalar_static_f64[29]*v2562);
        let v2749=(if (self.scalar_static_f64[148]!=0.0){((v2645+(self.scalar_static_f64[180]*v2556))-v2570)}else{v2648});
        let v2764=(if (self.scalar_static_f64[148]!=0.0){(v2749+((v677*v2573)+(v482*((v32*((v110*(v671*((v669*v2553)+(v456*(-v2749)))))/(v94*v674)))/v676))))}else{v60});
        let v2767=(v680*v680);
        let v2773=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[110]*(v684*(self.scalar_static_f64[111]*(((-(self.scalar_static_f64[101]*v2764))/v2767)/v681))))}else{v60});
        let v2778=(if (self.scalar_static_f64[148]!=0.0){(((self.scalar_static_f64[188]*v2556)+(self.scalar_static_f64[16]*v2564))-v2570)}else{v2749});
        let v2793=(if (self.scalar_static_f64[148]!=0.0){(v2778+((v708*v2573)+(v482*((v32*((v110*(v702*((v700*v2553)+(v456*(-v2778)))))/(v94*v705)))/v707))))}else{v60});
        let v2796=(v711*v711);
        let v2802=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[121]*(v715*(self.scalar_static_f64[122]*(((-(self.scalar_static_f64[112]*v2793))/v2796)/v712))))}else{v60});
        let v2836=(if self.scalar_static_bool[22]{(v748*(self.scalar_static_f64[133]*v2558))}else{(if self.scalar_static_bool[21]{((v742*v2554)+(v458*(self.scalar_static_f64[132]*v2554)))}else{v60})});
        let v2838=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[134]*v2836)}else{v60});
        let v2844=(if (self.scalar_static_f64[148]!=0.0){((v753*(self.scalar_static_f64[135]*v2836))+(v752*(v753*v2722)))}else{v60});
        let v2868=(if (v777!=0.0){(self.scalar_static_f64[190]*v2672)}else{v60});
        let v2869=(-v2663);
        let v2870=(if v790{v2869}else{v60});
        let v2871=(self.scalar_static_f64[196]*v2663);
        let v2872=(if v790{v2871}else{v60});
        let v2874=(if v790{(v795*v2868)}else{v60});
        let v2878=(((-(self.scalar_static_f64[191]*v2663))/v2666)/v805);
        let v2884=(if v790{((v808*v2868)+(v783*(v808*(v804*v2878))))}else{v60});
        let v2885=(v456*self.scalar_static_f64[273]);
        let v2889=(self.scalar_static_f64[0]*v456);
        let v2890=(if v790{v2885}else{v60});
        let v2891=(if v790{((v811*v2553)+(v456*v2872))}else{v60});
        let v2892=(if v790{v2889}else{v60});
        let v2896=(if v817{(v818*v2890)}else{v60});
        let v2897=(if v817{(v818*v2891)}else{v60});
        let v2898=(if v817{(v818*v2892)}else{v60});
        let v2932=(if v828{self.scalar_static_f64[0]}else{(if v817{(-(v454*(v2896/v820)))}else{v60})});
        let v2933=(if v828{v60}else{(if v817{(v2872-((v823*v2549)+(v454*(v2897/v820))))}else{v60})});
        let v2934=(if v828{self.scalar_static_f64[273]}else{(if v817{(-(v454*(v2898/v820)))}else{v60})});
        let v2936=(v110*v2549);
        let v2938=(if v790{((v831*v2870)+v2936)}else{v60});
        let v2944=(v835*v835);
        let v2947=(if v790{(v2932/v835)}else{v60});
        let v2948=(if v790{(((v835*(v2870+v2933))-(v836*v2938))/v2944)}else{v60});
        let v2949=(if v790{(v2934/v835)}else{v60});
        let v2953=(if v841{(v842*v2947)}else{v2896});
        let v2954=(if v841{(v842*v2948)}else{v2897});
        let v2955=(if v841{(v842*v2949)}else{v2898});
        let v2996=(if v858{v2932}else{(if v841{(v835*(v2953/v844))}else{v60})});
        let v2997=(if v858{v2933}else{(if v841{((-v2870)+((v853*v2938)+(v835*((v2954/v844)-(v852*(((v835*(-(v2870+v2872)))-(v850*v2938))/v2944))))))}else{v60})});
        let v2998=(if v858{v2934}else{(if v841{(v835*(v2955/v844))}else{v60})});
        let v3002=(if v790{(self.scalar_static_f64[0]-v2932)}else{v60});
        let v3003=(if v790{(-v2933)}else{v60});
        let v3004=(if v790{(self.scalar_static_f64[273]-v2934)}else{v60});
        let v3017=(if v790{((-(v2932/v573))/v864)}else{v60});
        let v3018=(if v790{((-(((v573*v2933)-(v830*v2663))/v2666))/v864)}else{v60});
        let v3019=(if v790{((-(v2934/v573))/v864)}else{v60});
        let v3032=(if v790{((-(v2996/v573))/v868)}else{v60});
        let v3033=(if v790{((-(((v573*v2997)-(v860*v2663))/v2666))/v868)}else{v60});
        let v3034=(if v790{((-(v2998/v573))/v868)}else{v60});
        let v3121=(if v790{((v783*(-(v893*(v872*v3032))))/v872)}else{v60});
        let v3122=(if v790{(((v894*v2868)+(v783*(-(v893*(v872*v3033)))))/v872)}else{v60});
        let v3123=(if v790{((v783*(-(v893*(v872*v3034))))/v872)}else{v60});
        let v3141=(if v790{((v810*(-(v899*(v874*v3017))))/v874)}else{v60});
        let v3142=(if v790{(((v900*v2884)+(v810*(-(v899*(v874*v3018)))))/v874)}else{v60});
        let v3143=(if v790{((v810*(-(v899*(v874*v3019))))/v874)}else{v60});
        let v3161=(if v790{((v810*(-(v905*(v874*v3032))))/v874)}else{v60});
        let v3162=(if v790{(((v906*v2884)+(v810*(-(v905*(v874*v3033)))))/v874)}else{v60});
        let v3163=(if v790{((v810*(-(v905*(v874*v3034))))/v874)}else{v60});
        let v3189=(if v921{v2871}else{v60});
        let v3193=(if v921{v2885}else{v60});
        let v3194=(if v921{((v923*v2553)+(v456*v3189))}else{v60});
        let v3195=(if v921{v2889}else{v60});
        let v3196=(v925*v3193);
        let v3198=(v925*v3194);
        let v3200=(v925*v3195);
        let v3202=(v94*v929);
        let v3206=(if v921{((v3196+v3196)/v3202)}else{v60});
        let v3207=(if v921{((v3198+v3198)/v3202)}else{v60});
        let v3208=(if v921{((v3200+v3200)/v3202)}else{v60});
        let v3215=(if v921{(v32*(v3193+v3206))}else{v60});
        let v3216=(if v921{(v32*(v3194+v3207))}else{v60});
        let v3217=(if v921{(v32*(v3195+v3208))}else{v60});
        let v3226=(if v921{(-(v454*v3215))}else{v60});
        let v3227=(if v921{(v3189-((v933*v2549)+(v454*v3216)))}else{v60});
        let v3228=(if v921{(-(v454*v3217))}else{v60});
        let v3257=(if v921{((-(v3226/v573))/v940)}else{v60});
        let v3258=(if v921{((-(((v573*v3227)-(v936*v2663))/v2666))/v940)}else{v60});
        let v3259=(if v921{((-(v3228/v573))/v940)}else{v60});
        let v3295=(if v921{((v573*(-(v948*(self.scalar_static_f64[197]*v3257))))/self.scalar_static_f64[197])}else{v60});
        let v3296=(if v921{(((v949*v2663)+(v573*(-(v948*(self.scalar_static_f64[197]*v3258)))))/self.scalar_static_f64[197])}else{v60});
        let v3297=(if v921{((v573*(-(v948*(self.scalar_static_f64[197]*v3259))))/self.scalar_static_f64[197])}else{v60});
        let v3318=(if v960{v2672}else{(if (v777!=0.0){(self.scalar_static_f64[189]*v2672)}else{v60})});
        let v3320=(if v960{(self.scalar_static_f64[189]*v2773)}else{v60});
        let v3321=(-v2764);
        let v3322=(if v970{v3321}else{v2870});
        let v3323=(self.scalar_static_f64[204]*v2764);
        let v3324=(if v970{v3323}else{v2872});
        let v3326=(if v970{(v795*v3320)}else{v2874});
        let v3330=(((-(self.scalar_static_f64[199]*v2764))/v2767)/v983);
        let v3336=(if v970{((v986*v3320)+(v963*(v986*(v982*v3330))))}else{v2884});
        let v3340=(if v970{v60}else{v2890});
        let v3341=(if v970{((v989*v2553)+(v456*v3324))}else{v2891});
        let v3342=(if v970{v2889}else{v2892});
        let v3343=(if v970{v2885}else{v60});
        let v3348=(if v994{(v995*v3340)}else{v2953});
        let v3349=(if v994{(v995*v3341)}else{v2954});
        let v3350=(if v994{(v995*v3342)}else{v2955});
        let v3351=(if v994{(v995*v3343)}else{v60});
        let v3395=(if v1005{v60}else{(if v994{(-(v454*(v3348/v997)))}else{v2932})});
        let v3396=(if v1005{v60}else{(if v994{(v3324-((v1000*v2549)+(v454*(v3349/v997))))}else{v2933})});
        let v3397=(if v1005{self.scalar_static_f64[273]}else{(if v994{(-(v454*(v3350/v997)))}else{v2934})});
        let v3398=(if v1005{self.scalar_static_f64[0]}else{(if v994{(-(v454*(v3351/v997)))}else{v60})});
        let v3401=(if v970{(v2936+(v831*v3322))}else{v2938});
        let v3407=(v1010*v1010);
        let v3411=(if v970{(v3395/v1010)}else{v2947});
        let v3412=(if v970{(((v1010*(v3322+v3396))-(v1011*v3401))/v3407)}else{v2948});
        let v3413=(if v970{(v3397/v1010)}else{v2949});
        let v3414=(if v970{(v3398/v1010)}else{v60});
        let v3419=(if v1016{(v1017*v3411)}else{v3348});
        let v3420=(if v1016{(v1017*v3412)}else{v3349});
        let v3421=(if v1016{(v1017*v3413)}else{v3350});
        let v3422=(if v1016{(v1017*v3414)}else{v3351});
        let v3472=(if v1033{v3395}else{(if v1016{(v1010*(v3419/v1019))}else{v2996})});
        let v3473=(if v1033{v3396}else{(if v1016{((-v3322)+((v1028*v3401)+(v1010*((v3420/v1019)-(v1027*(((v1010*(-(v3322+v3324)))-(v1025*v3401))/v3407))))))}else{v2997})});
        let v3474=(if v1033{v3397}else{(if v1016{(v1010*(v3421/v1019))}else{v2998})});
        let v3475=(if v1033{v3398}else{(if v1016{(v1010*(v3422/v1019))}else{v60})});
        let v3480=(if v970{(-v3395)}else{v3002});
        let v3481=(if v970{(-v3396)}else{v3003});
        let v3482=(if v970{(self.scalar_static_f64[273]-v3397)}else{v3004});
        let v3483=(if v970{(self.scalar_static_f64[0]-v3398)}else{v60});
        let v3499=(if v970{((-(v3395/v680))/v1039)}else{v3017});
        let v3500=(if v970{((-(((v680*v3396)-(v1007*v2764))/v2767))/v1039)}else{v3018});
        let v3501=(if v970{((-(v3397/v680))/v1039)}else{v3019});
        let v3502=(if v970{((-(v3398/v680))/v1039)}else{v60});
        let v3518=(if v970{((-(v3472/v680))/v1043)}else{v3032});
        let v3519=(if v970{((-(((v680*v3473)-(v1035*v2764))/v2767))/v1043)}else{v3033});
        let v3520=(if v970{((-(v3474/v680))/v1043)}else{v3034});
        let v3521=(if v970{((-(v3475/v680))/v1043)}else{v60});
        let v3634=(if v970{((v963*(-(v1068*(v1047*v3518))))/v1047)}else{v3121});
        let v3635=(if v970{(((v1069*v3320)+(v963*(-(v1068*(v1047*v3519)))))/v1047)}else{v3122});
        let v3636=(if v970{((v963*(-(v1068*(v1047*v3520))))/v1047)}else{v3123});
        let v3637=(if v970{((v963*(-(v1068*(v1047*v3521))))/v1047)}else{v60});
        let v3660=(if v970{((v988*(-(v1074*(v1049*v3499))))/v1049)}else{v3141});
        let v3661=(if v970{(((v1075*v3336)+(v988*(-(v1074*(v1049*v3500)))))/v1049)}else{v3142});
        let v3662=(if v970{((v988*(-(v1074*(v1049*v3501))))/v1049)}else{v3143});
        let v3663=(if v970{((v988*(-(v1074*(v1049*v3502))))/v1049)}else{v60});
        let v3686=(if v970{((v988*(-(v1080*(v1049*v3518))))/v1049)}else{v3161});
        let v3687=(if v970{(((v1081*v3336)+(v988*(-(v1080*(v1049*v3519)))))/v1049)}else{v3162});
        let v3688=(if v970{((v988*(-(v1080*(v1049*v3520))))/v1049)}else{v3163});
        let v3689=(if v970{((v988*(-(v1080*(v1049*v3521))))/v1049)}else{v60});
        let v3722=(if v1096{v3323}else{v3189});
        let v3726=(if v1096{v60}else{v3193});
        let v3727=(if v1096{((v1098*v2553)+(v456*v3722))}else{v3194});
        let v3728=(if v1096{v2889}else{v3195});
        let v3729=(if v1096{v2885}else{v60});
        let v3730=(v1100*v3726);
        let v3732=(v1100*v3727);
        let v3734=(v1100*v3728);
        let v3736=(v1100*v3729);
        let v3738=(v94*v1103);
        let v3743=(if v1096{((v3730+v3730)/v3738)}else{v3206});
        let v3744=(if v1096{((v3732+v3732)/v3738)}else{v3207});
        let v3745=(if v1096{((v3734+v3734)/v3738)}else{v3208});
        let v3746=(if v1096{((v3736+v3736)/v3738)}else{v60});
        let v3755=(if v1096{(v32*(v3726+v3743))}else{v3215});
        let v3756=(if v1096{(v32*(v3727+v3744))}else{v3216});
        let v3757=(if v1096{(v32*(v3728+v3745))}else{v3217});
        let v3758=(if v1096{(v32*(v3729+v3746))}else{v60});
        let v3769=(if v1096{(-(v454*v3755))}else{v3226});
        let v3770=(if v1096{(v3722-((v1107*v2549)+(v454*v3756)))}else{v3227});
        let v3771=(if v1096{(-(v454*v3757))}else{v3228});
        let v3772=(if v1096{(-(v454*v3758))}else{v60});
        let v3809=(if v1096{((-(v3769/v680))/v1114)}else{v3257});
        let v3810=(if v1096{((-(((v680*v3770)-(v1110*v2764))/v2767))/v1114)}else{v3258});
        let v3811=(if v1096{((-(v3771/v680))/v1114)}else{v3259});
        let v3812=(if v1096{((-(v3772/v680))/v1114)}else{v60});
        let v3859=(if v1096{((v680*(-(v1122*(self.scalar_static_f64[205]*v3809))))/self.scalar_static_f64[205])}else{v3295});
        let v3860=(if v1096{(((v1123*v2764)+(v680*(-(v1122*(self.scalar_static_f64[205]*v3810)))))/self.scalar_static_f64[205])}else{v3296});
        let v3861=(if v1096{((v680*(-(v1122*(self.scalar_static_f64[205]*v3811))))/self.scalar_static_f64[205])}else{v3297});
        let v3862=(if v1096{((v680*(-(v1122*(self.scalar_static_f64[205]*v3812))))/self.scalar_static_f64[205])}else{v60});
        let v3890=(if v960{(self.scalar_static_f64[190]*v2773)}else{v2868});
        let v3891=(if v1138{v3321}else{v3322});
        let v3892=(if v1138{v3323}else{v3324});
        let v3894=(if v1138{(v795*v3890)}else{v3326});
        let v3900=(if v1138{((v1146*v3890)+(v1135*(v1146*(v1144*v3330))))}else{v3336});
        let v3904=(if v1138{v2885}else{v3340});
        let v3905=(if v1138{((v1149*v2553)+(v456*v3892))}else{v3341});
        let v3906=(if v1138{v2889}else{v3342});
        let v3907=(if v1138{v60}else{v3343});
        let v3912=(if v1154{(v1155*v3904)}else{v3419});
        let v3913=(if v1154{(v1155*v3905)}else{v3420});
        let v3914=(if v1154{(v1155*v3906)}else{v3421});
        let v3915=(if v1154{(v1155*v3907)}else{v3422});
        let v3959=(if v1165{self.scalar_static_f64[0]}else{(if v1154{(-(v454*(v3912/v1157)))}else{v3395})});
        let v3960=(if v1165{v60}else{(if v1154{(v3892-((v1160*v2549)+(v454*(v3913/v1157))))}else{v3396})});
        let v3961=(if v1165{self.scalar_static_f64[273]}else{(if v1154{(-(v454*(v3914/v1157)))}else{v3397})});
        let v3962=(if v1165{v60}else{(if v1154{(-(v454*(v3915/v1157)))}else{v3398})});
        let v3965=(if v1138{(v2936+(v831*v3891))}else{v3401});
        let v3971=(v1170*v1170);
        let v3975=(if v1138{(v3959/v1170)}else{v3411});
        let v3976=(if v1138{(((v1170*(v3891+v3960))-(v1171*v3965))/v3971)}else{v3412});
        let v3977=(if v1138{(v3961/v1170)}else{v3413});
        let v3978=(if v1138{(v3962/v1170)}else{v3414});
        let v3983=(if v1176{(v1177*v3975)}else{v3912});
        let v3984=(if v1176{(v1177*v3976)}else{v3913});
        let v3985=(if v1176{(v1177*v3977)}else{v3914});
        let v3986=(if v1176{(v1177*v3978)}else{v3915});
        let v4036=(if v1193{v3959}else{(if v1176{(v1170*(v3983/v1179))}else{v3472})});
        let v4037=(if v1193{v3960}else{(if v1176{((-v3891)+((v1188*v3965)+(v1170*((v3984/v1179)-(v1187*(((v1170*(-(v3891+v3892)))-(v1185*v3965))/v3971))))))}else{v3473})});
        let v4038=(if v1193{v3961}else{(if v1176{(v1170*(v3985/v1179))}else{v3474})});
        let v4039=(if v1193{v3962}else{(if v1176{(v1170*(v3986/v1179))}else{v3475})});
        let v4044=(if v1138{(self.scalar_static_f64[0]-v3959)}else{v3480});
        let v4045=(if v1138{(-v3960)}else{v3481});
        let v4046=(if v1138{(self.scalar_static_f64[273]-v3961)}else{v3482});
        let v4047=(if v1138{(-v3962)}else{v3483});
        let v4063=(if v1138{((-(v3959/v680))/v1199)}else{v3499});
        let v4064=(if v1138{((-(((v680*v3960)-(v1167*v2764))/v2767))/v1199)}else{v3500});
        let v4065=(if v1138{((-(v3961/v680))/v1199)}else{v3501});
        let v4066=(if v1138{((-(v3962/v680))/v1199)}else{v3502});
        let v4082=(if v1138{((-(v4036/v680))/v1203)}else{v3518});
        let v4083=(if v1138{((-(((v680*v4037)-(v1195*v2764))/v2767))/v1203)}else{v3519});
        let v4084=(if v1138{((-(v4038/v680))/v1203)}else{v3520});
        let v4085=(if v1138{((-(v4039/v680))/v1203)}else{v3521});
        let v4198=(if v1138{((v1135*(-(v1226*(v1206*v4082))))/v1206)}else{v3634});
        let v4199=(if v1138{(((v1227*v3890)+(v1135*(-(v1226*(v1206*v4083)))))/v1206)}else{v3635});
        let v4200=(if v1138{((v1135*(-(v1226*(v1206*v4084))))/v1206)}else{v3636});
        let v4201=(if v1138{((v1135*(-(v1226*(v1206*v4085))))/v1206)}else{v3637});
        let v4224=(if v1138{((v1148*(-(v1232*(v1208*v4063))))/v1208)}else{v3660});
        let v4225=(if v1138{(((v1233*v3900)+(v1148*(-(v1232*(v1208*v4064)))))/v1208)}else{v3661});
        let v4226=(if v1138{((v1148*(-(v1232*(v1208*v4065))))/v1208)}else{v3662});
        let v4227=(if v1138{((v1148*(-(v1232*(v1208*v4066))))/v1208)}else{v3663});
        let v4250=(if v1138{((v1148*(-(v1238*(v1208*v4082))))/v1208)}else{v3686});
        let v4251=(if v1138{(((v1239*v3900)+(v1148*(-(v1238*(v1208*v4083)))))/v1208)}else{v3687});
        let v4252=(if v1138{((v1148*(-(v1238*(v1208*v4084))))/v1208)}else{v3688});
        let v4253=(if v1138{((v1148*(-(v1238*(v1208*v4085))))/v1208)}else{v3689});
        let v4286=(if v1252{v3323}else{v3722});
        let v4290=(if v1252{v2885}else{v3726});
        let v4291=(if v1252{((v1254*v2553)+(v456*v4286))}else{v3727});
        let v4292=(if v1252{v2889}else{v3728});
        let v4293=(if v1252{v60}else{v3729});
        let v4294=(v1256*v4290);
        let v4296=(v1256*v4291);
        let v4298=(v1256*v4292);
        let v4300=(v1256*v4293);
        let v4302=(v94*v1259);
        let v4307=(if v1252{((v4294+v4294)/v4302)}else{v3743});
        let v4308=(if v1252{((v4296+v4296)/v4302)}else{v3744});
        let v4309=(if v1252{((v4298+v4298)/v4302)}else{v3745});
        let v4310=(if v1252{((v4300+v4300)/v4302)}else{v3746});
        let v4319=(if v1252{(v32*(v4290+v4307))}else{v3755});
        let v4320=(if v1252{(v32*(v4291+v4308))}else{v3756});
        let v4321=(if v1252{(v32*(v4292+v4309))}else{v3757});
        let v4322=(if v1252{(v32*(v4293+v4310))}else{v3758});
        let v4333=(if v1252{(-(v454*v4319))}else{v3769});
        let v4334=(if v1252{(v4286-((v1263*v2549)+(v454*v4320)))}else{v3770});
        let v4335=(if v1252{(-(v454*v4321))}else{v3771});
        let v4336=(if v1252{(-(v454*v4322))}else{v3772});
        let v4373=(if v1252{((-(v4333/v680))/v1270)}else{v3809});
        let v4374=(if v1252{((-(((v680*v4334)-(v1266*v2764))/v2767))/v1270)}else{v3810});
        let v4375=(if v1252{((-(v4335/v680))/v1270)}else{v3811});
        let v4376=(if v1252{((-(v4336/v680))/v1270)}else{v3812});
        let v4423=(if v1252{((v680*(-(v1278*(self.scalar_static_f64[205]*v4373))))/self.scalar_static_f64[205])}else{v3859});
        let v4424=(if v1252{(((v1279*v2764)+(v680*(-(v1278*(self.scalar_static_f64[205]*v4374)))))/self.scalar_static_f64[205])}else{v3860});
        let v4425=(if v1252{((v680*(-(v1278*(self.scalar_static_f64[205]*v4375))))/self.scalar_static_f64[205])}else{v3861});
        let v4426=(if v1252{((v680*(-(v1278*(self.scalar_static_f64[205]*v4376))))/self.scalar_static_f64[205])}else{v3862});
        let v4453=(if v1292{v2869}else{v3891});
        let v4454=(if v1292{v2871}else{v3892});
        let v4455=(v795*v3318);
        let v4456=(if v1292{v4455}else{v3894});
        let v4462=(if v1292{((v1300*v3318)+(v961*(v1300*(v1298*v2878))))}else{v3900});
        let v4466=(if v1292{v60}else{v3904});
        let v4467=(if v1292{((v1303*v2553)+(v456*v4454))}else{v3905});
        let v4468=(if v1292{v2889}else{v3906});
        let v4469=(if v1292{v2885}else{v3907});
        let v4474=(if v1308{(v1309*v4466)}else{v3983});
        let v4475=(if v1308{(v1309*v4467)}else{v3984});
        let v4476=(if v1308{(v1309*v4468)}else{v3985});
        let v4477=(if v1308{(v1309*v4469)}else{v3986});
        let v4521=(if v1319{v60}else{(if v1308{(-(v454*(v4474/v1311)))}else{v3959})});
        let v4522=(if v1319{v60}else{(if v1308{(v4454-((v1314*v2549)+(v454*(v4475/v1311))))}else{v3960})});
        let v4523=(if v1319{self.scalar_static_f64[273]}else{(if v1308{(-(v454*(v4476/v1311)))}else{v3961})});
        let v4524=(if v1319{self.scalar_static_f64[0]}else{(if v1308{(-(v454*(v4477/v1311)))}else{v3962})});
        let v4527=(if v1292{(v2936+(v831*v4453))}else{v3965});
        let v4533=(v1324*v1324);
        let v4537=(if v1292{(v4521/v1324)}else{v3975});
        let v4538=(if v1292{(((v1324*(v4453+v4522))-(v1325*v4527))/v4533)}else{v3976});
        let v4539=(if v1292{(v4523/v1324)}else{v3977});
        let v4540=(if v1292{(v4524/v1324)}else{v3978});
        let v4545=(if v1330{(v1331*v4537)}else{v4474});
        let v4546=(if v1330{(v1331*v4538)}else{v4475});
        let v4547=(if v1330{(v1331*v4539)}else{v4476});
        let v4548=(if v1330{(v1331*v4540)}else{v4477});
        let v4598=(if v1347{v4521}else{(if v1330{(v1324*(v4545/v1333))}else{v4036})});
        let v4599=(if v1347{v4522}else{(if v1330{((-v4453)+((v1342*v4527)+(v1324*((v4546/v1333)-(v1341*(((v1324*(-(v4453+v4454)))-(v1339*v4527))/v4533))))))}else{v4037})});
        let v4600=(if v1347{v4523}else{(if v1330{(v1324*(v4547/v1333))}else{v4038})});
        let v4601=(if v1347{v4524}else{(if v1330{(v1324*(v4548/v1333))}else{v4039})});
        let v4606=(if v1292{(-v4521)}else{v4044});
        let v4607=(if v1292{(-v4522)}else{v4045});
        let v4608=(if v1292{(self.scalar_static_f64[273]-v4523)}else{v4046});
        let v4609=(if v1292{(self.scalar_static_f64[0]-v4524)}else{v4047});
        let v4625=(if v1292{((-(v4521/v573))/v1353)}else{v4063});
        let v4626=(if v1292{((-(((v573*v4522)-(v1321*v2663))/v2666))/v1353)}else{v4064});
        let v4627=(if v1292{((-(v4523/v573))/v1353)}else{v4065});
        let v4628=(if v1292{((-(v4524/v573))/v1353)}else{v4066});
        let v4644=(if v1292{((-(v4598/v573))/v1357)}else{v4082});
        let v4645=(if v1292{((-(((v573*v4599)-(v1349*v2663))/v2666))/v1357)}else{v4083});
        let v4646=(if v1292{((-(v4600/v573))/v1357)}else{v4084});
        let v4647=(if v1292{((-(v4601/v573))/v1357)}else{v4085});
        let v4760=(if v1292{((v961*(-(v1380*(v1360*v4644))))/v1360)}else{v4198});
        let v4761=(if v1292{(((v1381*v3318)+(v961*(-(v1380*(v1360*v4645)))))/v1360)}else{v4199});
        let v4762=(if v1292{((v961*(-(v1380*(v1360*v4646))))/v1360)}else{v4200});
        let v4763=(if v1292{((v961*(-(v1380*(v1360*v4647))))/v1360)}else{v4201});
        let v4786=(if v1292{((v1302*(-(v1386*(v1362*v4625))))/v1362)}else{v4224});
        let v4787=(if v1292{(((v1387*v4462)+(v1302*(-(v1386*(v1362*v4626)))))/v1362)}else{v4225});
        let v4788=(if v1292{((v1302*(-(v1386*(v1362*v4627))))/v1362)}else{v4226});
        let v4789=(if v1292{((v1302*(-(v1386*(v1362*v4628))))/v1362)}else{v4227});
        let v4812=(if v1292{((v1302*(-(v1392*(v1362*v4644))))/v1362)}else{v4250});
        let v4813=(if v1292{(((v1393*v4462)+(v1302*(-(v1392*(v1362*v4645)))))/v1362)}else{v4251});
        let v4814=(if v1292{((v1302*(-(v1392*(v1362*v4646))))/v1362)}else{v4252});
        let v4815=(if v1292{((v1302*(-(v1392*(v1362*v4647))))/v1362)}else{v4253});
        let v4848=(if v1406{v2871}else{v4286});
        let v4852=(if v1406{v60}else{v4290});
        let v4853=(if v1406{((v1408*v2553)+(v456*v4848))}else{v4291});
        let v4854=(if v1406{v2889}else{v4292});
        let v4855=(if v1406{v2885}else{v4293});
        let v4856=(v1410*v4852);
        let v4858=(v1410*v4853);
        let v4860=(v1410*v4854);
        let v4862=(v1410*v4855);
        let v4864=(v94*v1413);
        let v4869=(if v1406{((v4856+v4856)/v4864)}else{v4307});
        let v4870=(if v1406{((v4858+v4858)/v4864)}else{v4308});
        let v4871=(if v1406{((v4860+v4860)/v4864)}else{v4309});
        let v4872=(if v1406{((v4862+v4862)/v4864)}else{v4310});
        let v4881=(if v1406{(v32*(v4852+v4869))}else{v4319});
        let v4882=(if v1406{(v32*(v4853+v4870))}else{v4320});
        let v4883=(if v1406{(v32*(v4854+v4871))}else{v4321});
        let v4884=(if v1406{(v32*(v4855+v4872))}else{v4322});
        let v4895=(if v1406{(-(v454*v4881))}else{v4333});
        let v4896=(if v1406{(v4848-((v1417*v2549)+(v454*v4882)))}else{v4334});
        let v4897=(if v1406{(-(v454*v4883))}else{v4335});
        let v4898=(if v1406{(-(v454*v4884))}else{v4336});
        let v4935=(if v1406{((-(v4895/v573))/v1424)}else{v4373});
        let v4936=(if v1406{((-(((v573*v4896)-(v1420*v2663))/v2666))/v1424)}else{v4374});
        let v4937=(if v1406{((-(v4897/v573))/v1424)}else{v4375});
        let v4938=(if v1406{((-(v4898/v573))/v1424)}else{v4376});
        let v4985=(if v1406{((v573*(-(v1432*(self.scalar_static_f64[197]*v4935))))/self.scalar_static_f64[197])}else{v4423});
        let v4986=(if v1406{(((v1433*v2663)+(v573*(-(v1432*(self.scalar_static_f64[197]*v4936)))))/self.scalar_static_f64[197])}else{v4424});
        let v4987=(if v1406{((v573*(-(v1432*(self.scalar_static_f64[197]*v4937))))/self.scalar_static_f64[197])}else{v4425});
        let v4988=(if v1406{((v573*(-(v1432*(self.scalar_static_f64[197]*v4938))))/self.scalar_static_f64[197])}else{v4426});
        let v5011=(if v1442{v60}else{(if v1406{(v961*(v4985+(v795*(-v4895))))}else{(if v1404{v60}else{(if v1292{((v573*((v4760+v4786)-v4812))+(v1297*v4606))}else{v60})})})});
        let v5012=(if v1442{v60}else{(if v1406{((v1439*v3318)+(v961*(v4986+(v795*(-v4896)))))}else{(if v1404{v60}else{(if v1292{(((v1398*v2663)+(v573*((v4761+v4787)-v4813)))+((v1351*v4456)+(v1297*v4607)))}else{v60})})})});
        let v5013=(if v1442{v60}else{(if v1406{(v961*(v4987+(v795*(self.scalar_static_f64[273]-v4897))))}else{(if v1404{v60}else{(if v1292{((v573*((v4762+v4788)-v4814))+(v1297*v4608))}else{v60})})})});
        let v5014=(if v1442{v60}else{(if v1406{(v961*(v4988+(v795*(self.scalar_static_f64[0]-v4898))))}else{(if v1404{v60}else{(if v1292{((v573*((v4763+v4789)-v4815))+(v1297*v4609))}else{v60})})})});
        let v5019=(if (v1291!=0.0){v2871}else{v60});
        let v5023=(if (v1291!=0.0){((v1446*v2553)+(v456*v5019))}else{v60});
        let v5024=(if (v1291!=0.0){v2889}else{v60});
        let v5025=(if (v1291!=0.0){v2885}else{v60});
        let v5026=(v1448*v5023);
        let v5028=(v1448*v5024);
        let v5030=(v1448*v5025);
        let v5032=(v94*v1451);
        let v5036=(if (v1291!=0.0){((v5026+v5026)/v5032)}else{v60});
        let v5037=(if (v1291!=0.0){((v5028+v5028)/v5032)}else{v60});
        let v5038=(if (v1291!=0.0){((v5030+v5030)/v5032)}else{v60});
        let v5045=(if (v1291!=0.0){(v32*(v5023+v5036))}else{v60});
        let v5046=(if (v1291!=0.0){(v32*(v5024+v5037))}else{v60});
        let v5047=(if (v1291!=0.0){(v32*(v5025+v5038))}else{v60});
        let v5062=(v1452*v1452);
        let v5072=(if (v1291!=0.0){(((v1452*v5045)-(v1455*v5036))/v5062)}else{v60});
        let v5073=(if (v1291!=0.0){(((v1452*v5046)-(v1455*v5037))/v5062)}else{v60});
        let v5074=(if (v1291!=0.0){(((v1452*v5047)-(v1455*v5038))/v5062)}else{v60});
        let v5134=((v1476*v2553)+(v456*(if self.scalar_static_bool[5]{(-(if self.scalar_static_bool[16]{v2704}else{(if self.scalar_static_bool[15]{v60}else{(if (self.scalar_static_f64[148]!=0.0){v2704}else{v60})})}))}else{(if (self.scalar_static_f64[85]!=0.0){(if self.scalar_static_bool[16]{v60}else{(if self.scalar_static_bool[15]{(self.scalar_static_f64[84]*(-(self.scalar_static_f64[86]*v2554)))}else{v60})})}else{v60})})));
        let v5135=(v456*self.scalar_static_f64[278]);
        let v5136=(v456*self.scalar_static_f64[279]);
        let v5137=(v456*self.scalar_static_f64[280]);
        let v5138=(v1478*v5134);
        let v5140=(v1478*v5135);
        let v5142=(v1478*v5136);
        let v5144=(v1478*v5137);
        let v5146=(v94*v1481);
        let v5161=((v1484*v2549)+(v454*((v5134+((v5138+v5138)/v5146))/v94)));
        let v5162=(v454*((v5135+((v5140+v5140)/v5146))/v94));
        let v5163=(v454*((v5136+((v5142+v5142)/v5146))/v94));
        let v5164=(v454*((v5137+((v5144+v5144)/v5146))/v94));
        let v5206=(v1495*v1495);
        let v5221=((v5161-v2694)/self.scalar_static_f64[208]);
        let v5222=(v5162/self.scalar_static_f64[208]);
        let v5223=(v5163/self.scalar_static_f64[208]);
        let v5224=(v5164/self.scalar_static_f64[208]);
        let v5225=(v1499*v5221);
        let v5227=(v1499*v5222);
        let v5229=(v1499*v5223);
        let v5231=(v1499*v5224);
        let v5233=(v94*v1503);
        let v5248=((v1506*(((v1495*((v1485*(if (self.scalar_static_f64[148]!=0.0){((-(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[81]*(v603*(self.scalar_static_f64[22]*v2558)))}else{v60}))/(v605*v605))}else{v60}))+(v607*v5161)))-(v1487*(v1495*(((v1491*(self.scalar_static_f64[207]*((((v601*v5161)-(v1485*v2694))/(v601*v601))/v1486)))/v1492)/self.scalar_static_f64[207]))))/v5206))+(v1496*(v32*(v5221+((v5225+v5225)/v5233)))));
        let v5251=((v1506*(((v1495*(v607*v5162))-(v1487*(v1495*(((v1491*(self.scalar_static_f64[207]*((v5162/v601)/v1486)))/v1492)/self.scalar_static_f64[207]))))/v5206))+(v1496*(v32*(v5222+((v5227+v5227)/v5233)))));
        let v5254=((v1506*(((v1495*(v607*v5163))-(v1487*(v1495*(((v1491*(self.scalar_static_f64[207]*((v5163/v601)/v1486)))/v1492)/self.scalar_static_f64[207]))))/v5206))+(v1496*(v32*(v5223+((v5229+v5229)/v5233)))));
        let v5257=((v1506*(((v1495*(v607*v5164))-(v1487*(v1495*(((v1491*(self.scalar_static_f64[207]*((v5164/v601)/v1486)))/v1492)/self.scalar_static_f64[207]))))/v5206))+(v1496*(v32*(v5224+((v5231+v5231)/v5233)))));
        let v5261=(v1472*v1472);
        let v5284=(if v1515{v60}else{(if (v1510!=0.0){(((v1472*v3318)-(v961*(if v1403{v60}else{(if (v1291!=0.0){(((v1466*v5072)+(v1460*((v1465*v3318)+(v961*(v1465*(self.scalar_static_f64[198]*((-(((v573*(if (v1291!=0.0){(v5019-((v1455*v2549)+(v454*v5045)))}else{v60}))-(v1458*v2663))/v2666))/v1462)))))))+((v1468*v4455)+(v1296*(-v5072))))}else{v60})})))/v5261)}else{v60})});
        let v5285=(if v1515{v60}else{(if (v1510!=0.0){((-(v961*(if v1403{v60}else{(if (v1291!=0.0){(((v1466*v5073)+(v1460*(v961*(v1465*(self.scalar_static_f64[198]*((-((if (v1291!=0.0){(-(v454*v5046))}else{v60})/v573))/v1462))))))+(v1296*(-v5073)))}else{v60})})))/v5261)}else{v60})});
        let v5286=(if v1515{v60}else{(if (v1510!=0.0){((-(v961*(if v1403{v60}else{(if (v1291!=0.0){(((v1466*v5074)+(v1460*(v961*(v1465*(self.scalar_static_f64[198]*((-((if (v1291!=0.0){(-(v454*v5047))}else{v60})/v573))/v1462))))))+(v1296*(-v5074)))}else{v60})})))/v5261)}else{v60})});
        let v5287=(if v1515{v60}else{(if (v1510!=0.0){(v5011/v961)}else{v5011})});
        let v5288=(if v1515{v60}else{(if (v1510!=0.0){(((v961*v5012)-(v1443*v3318))/(v961*v961))}else{v5012})});
        let v5289=(if v1515{v60}else{(if (v1510!=0.0){(v5013/v961)}else{v5013})});
        let v5290=(if v1515{v60}else{(if (v1510!=0.0){(v5014/v961)}else{v5014})});
        let v5299=(if (v1519!=0.0){((v1524*v2588)+(v494*(-(v1523*((-(v2600/v503))/self.scalar_static_f64[47])))))}else{v4848});
        let v5303=(if (v1519!=0.0){v60}else{v4852});
        let v5304=(if (v1519!=0.0){((v1527*v2553)+(v456*v5299))}else{v4853});
        let v5305=(if (v1519!=0.0){v60}else{v4854});
        let v5306=(if (v1519!=0.0){v2885}else{v4855});
        let v5307=(if (v1519!=0.0){v2889}else{v60});
        let v5308=(v1529*v5303);
        let v5310=(v1529*v5304);
        let v5312=(v1529*v5305);
        let v5314=(v1529*v5306);
        let v5316=(v1529*v5307);
        let v5318=(v94*v1532);
        let v5324=(if (v1519!=0.0){((v5308+v5308)/v5318)}else{v4869});
        let v5325=(if (v1519!=0.0){((v5310+v5310)/v5318)}else{v4870});
        let v5326=(if (v1519!=0.0){((v5312+v5312)/v5318)}else{v4871});
        let v5327=(if (v1519!=0.0){((v5314+v5314)/v5318)}else{v4872});
        let v5328=(if (v1519!=0.0){((v5316+v5316)/v5318)}else{v60});
        let v5339=(if (v1519!=0.0){(v32*(v5303+v5324))}else{v4881});
        let v5340=(if (v1519!=0.0){(v32*(v5304+v5325))}else{v4882});
        let v5341=(if (v1519!=0.0){(v32*(v5305+v5326))}else{v4883});
        let v5342=(if (v1519!=0.0){(v32*(v5306+v5327))}else{v4884});
        let v5343=(if (v1519!=0.0){(v32*(v5307+v5328))}else{v60});
        let v5356=(if (v1519!=0.0){(-(v454*v5339))}else{v4895});
        let v5357=(if (v1519!=0.0){(v5299-((v1536*v2549)+(v454*v5340)))}else{v4896});
        let v5358=(if (v1519!=0.0){(-(v454*v5341))}else{v4897});
        let v5359=(if (v1519!=0.0){(-(v454*v5342))}else{v4898});
        let v5360=(if (v1519!=0.0){(-(v454*v5343))}else{v60});
        let v5405=(if (v1519!=0.0){((-(v5356/v494))/v1543)}else{v4935});
        let v5406=(if (v1519!=0.0){((-(((v494*v5357)-(v1539*v2588))/v2591))/v1543)}else{v4936});
        let v5407=(if (v1519!=0.0){((-(v5358/v494))/v1543)}else{v4937});
        let v5408=(if (v1519!=0.0){((-(v5359/v494))/v1543)}else{v4938});
        let v5409=(if (v1519!=0.0){((-(v5360/v494))/v1543)}else{v60});
        let v5467=(if (v1519!=0.0){((v494*(-(v1553*(self.scalar_static_f64[211]*v5405))))/self.scalar_static_f64[211])}else{v4985});
        let v5468=(if (v1519!=0.0){(((v1554*v2588)+(v494*(-(v1553*(self.scalar_static_f64[211]*v5406)))))/self.scalar_static_f64[211])}else{v4986});
        let v5469=(if (v1519!=0.0){((v494*(-(v1553*(self.scalar_static_f64[211]*v5407))))/self.scalar_static_f64[211])}else{v4987});
        let v5470=(if (v1519!=0.0){((v494*(-(v1553*(self.scalar_static_f64[211]*v5408))))/self.scalar_static_f64[211])}else{v4988});
        let v5471=(if (v1519!=0.0){((v494*(-(v1553*(self.scalar_static_f64[211]*v5409))))/self.scalar_static_f64[211])}else{v60});
        let v5501=(if v1563{v60}else{(if (v1519!=0.0){(v500*(v5467+(v503*(-v5356))))}else{v60})});
        let v5502=(if v1563{v60}else{(if (v1519!=0.0){((v1560*v2597)+(v500*(v5468+((v1558*v2600)+(v503*(-v5357))))))}else{v60})});
        let v5503=(if v1563{v60}else{(if (v1519!=0.0){(v500*(v5469+(v503*(-v5358))))}else{v60})});
        let v5504=(if v1563{v60}else{(if (v1519!=0.0){(v500*(v5470+(v503*(self.scalar_static_f64[0]-v5359))))}else{v60})});
        let v5505=(if v1563{v60}else{(if (v1519!=0.0){(v500*(v5471+(v503*(self.scalar_static_f64[273]-v5360))))}else{v60})});
        let v5506=(v5501/v500);
        let v5511=(((v500*v5502)-(v1564*v2597))/(v500*v500));
        let v5512=(v5503/v500);
        let v5513=(v5504/v500);
        let v5514=(v5505/v500);
        let v5523=(if v1568{((v1573*v2619)+(v527*(-(v1572*((-(v2631/v536))/self.scalar_static_f64[58])))))}else{v5299});
        let v5527=(if v1568{v60}else{v5303});
        let v5528=(if v1568{((v1576*v2553)+(v456*v5523))}else{v5304});
        let v5529=(if v1568{v60}else{v5305});
        let v5530=(if v1568{v2885}else{v5306});
        let v5531=(if v1568{v2889}else{v5307});
        let v5532=(v1578*v5527);
        let v5534=(v1578*v5528);
        let v5536=(v1578*v5529);
        let v5538=(v1578*v5530);
        let v5540=(v1578*v5531);
        let v5542=(v94*v1581);
        let v5548=(if v1568{((v5532+v5532)/v5542)}else{v5324});
        let v5549=(if v1568{((v5534+v5534)/v5542)}else{v5325});
        let v5550=(if v1568{((v5536+v5536)/v5542)}else{v5326});
        let v5551=(if v1568{((v5538+v5538)/v5542)}else{v5327});
        let v5552=(if v1568{((v5540+v5540)/v5542)}else{v5328});
        let v5563=(if v1568{(v32*(v5527+v5548))}else{v5339});
        let v5564=(if v1568{(v32*(v5528+v5549))}else{v5340});
        let v5565=(if v1568{(v32*(v5529+v5550))}else{v5341});
        let v5566=(if v1568{(v32*(v5530+v5551))}else{v5342});
        let v5567=(if v1568{(v32*(v5531+v5552))}else{v5343});
        let v5580=(if v1568{(-(v454*v5563))}else{v5356});
        let v5581=(if v1568{(v5523-((v1585*v2549)+(v454*v5564)))}else{v5357});
        let v5582=(if v1568{(-(v454*v5565))}else{v5358});
        let v5583=(if v1568{(-(v454*v5566))}else{v5359});
        let v5584=(if v1568{(-(v454*v5567))}else{v5360});
        let v5629=(if v1568{((-(v5580/v527))/v1592)}else{v5405});
        let v5630=(if v1568{((-(((v527*v5581)-(v1588*v2619))/v2622))/v1592)}else{v5406});
        let v5631=(if v1568{((-(v5582/v527))/v1592)}else{v5407});
        let v5632=(if v1568{((-(v5583/v527))/v1592)}else{v5408});
        let v5633=(if v1568{((-(v5584/v527))/v1592)}else{v5409});
        let v5691=(if v1568{((v527*(-(v1602*(self.scalar_static_f64[213]*v5629))))/self.scalar_static_f64[213])}else{v5467});
        let v5692=(if v1568{(((v1603*v2619)+(v527*(-(v1602*(self.scalar_static_f64[213]*v5630)))))/self.scalar_static_f64[213])}else{v5468});
        let v5693=(if v1568{((v527*(-(v1602*(self.scalar_static_f64[213]*v5631))))/self.scalar_static_f64[213])}else{v5469});
        let v5694=(if v1568{((v527*(-(v1602*(self.scalar_static_f64[213]*v5632))))/self.scalar_static_f64[213])}else{v5470});
        let v5695=(if v1568{((v527*(-(v1602*(self.scalar_static_f64[213]*v5633))))/self.scalar_static_f64[213])}else{v5471});
        let v5750=(if self.scalar_static_bool[11]{v2588}else{(if (self.scalar_static_f64[130]!=0.0){v2619}else{v60})});
        let v5752=(if self.scalar_static_bool[28]{(self.scalar_static_f64[218]*v2549)}else{v60});
        let v5760=(if self.scalar_static_bool[28]{(((v1628*v5750)-(v1629*v5752))/(v1628*v1628))}else{v60});
        let v5761=(if self.scalar_static_bool[28]{(self.scalar_static_f64[273]/v1628)}else{v60});
        let v5762=(if self.scalar_static_bool[28]{(self.scalar_static_f64[0]/v1628)}else{v60});
        let v5763=(v1631*v5760);
        let v5765=(v1631*v5761);
        let v5767=(v1631*v5762);
        let v5769=(v94*v1634);
        let v5817=(if self.scalar_static_bool[28]{((v1645*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[125]*(v729*(self.scalar_static_f64[126]*v2558)))}else{v60}))+(v731*(-(v1644*(self.scalar_static_f64[215]*((-(((v1620*(if self.scalar_static_bool[28]{(v5750-(v32*((v1635*v5752)+(v1628*(v5760+((v5763+v5763)/v5769))))))}else{v60}))-(v1639*v5750))/(v1620*v1620)))/v1641))))))}else{v60});
        let v5818=(if self.scalar_static_bool[28]{(v731*(-(v1644*(self.scalar_static_f64[215]*((-((if self.scalar_static_bool[28]{(-(v32*(v1628*(v5761+((v5765+v5765)/v5769)))))}else{v60})/v1620))/v1641)))))}else{v60});
        let v5819=(if self.scalar_static_bool[28]{(v731*(-(v1644*(self.scalar_static_f64[215]*((-((if self.scalar_static_bool[28]{(-(v32*(v1628*(v5762+((v5767+v5767)/v5769)))))}else{v60})/v1620))/v1641)))))}else{v60});
        let v5826=(v1647*v1647);
        let v5873=(v1668*(((v1661*(if self.scalar_static_bool[11]{v5506}else{(if (self.scalar_static_f64[130]!=0.0){((if v1613{v60}else{(if v1568{(v533*(v5691+(v536*(-v5580))))}else{v60})})/v533)}else{v60})}))/v739)+(v5287/self.scalar_static_f64[219])));
        let v5874=(v1668*((((v739*((v1661*(if self.scalar_static_bool[11]{v5511}else{(if (self.scalar_static_f64[130]!=0.0){(((v533*(if v1613{v60}else{(if v1568{((v1609*v2628)+(v533*(v5692+((v1607*v2631)+(v536*(-v5581))))))}else{v60})}))-(v1614*v2628))/(v533*v533))}else{v60})}))+(v1619*(if v1658{(v32*v5817)}else{(if v1652{(((v1647*(v1653*v5817))-(v1654*v5817))/v5826)}else{v60})}))))-(v1662*(if (self.scalar_static_f64[148]!=0.0){((-(self.scalar_static_f64[127]*(v737*((v735*(self.scalar_static_f64[78]*v2553))+(v732*(v734*(self.scalar_static_f64[128]*v2558)))))))/(v737*v737))}else{v60})))/(v739*v739))+(v5288/self.scalar_static_f64[219])));
        let v5875=(v1668*(((v1661*(if self.scalar_static_bool[11]{v5512}else{(if (self.scalar_static_f64[130]!=0.0){((if v1613{v60}else{(if v1568{(v533*(v5693+(v536*(-v5582))))}else{v60})})/v533)}else{v60})}))/v739)+(v5289/self.scalar_static_f64[219])));
        let v5876=(v1668*((((v1661*(if self.scalar_static_bool[11]{v5513}else{(if (self.scalar_static_f64[130]!=0.0){((if v1613{v60}else{(if v1568{(v533*(v5694+(v536*(self.scalar_static_f64[0]-v5583))))}else{v60})})/v533)}else{v60})}))+(v1619*(if v1658{(v32*v5818)}else{(if v1652{(((v1647*(v1653*v5818))-(v1654*v5818))/v5826)}else{v60})})))/v739)+(v5290/self.scalar_static_f64[219])));
        let v5877=(v1668*(((v1661*(if self.scalar_static_bool[11]{v5514}else{(if (self.scalar_static_f64[130]!=0.0){((if v1613{v60}else{(if v1568{(v533*(v5695+(v536*(self.scalar_static_f64[273]-v5584))))}else{v60})})/v533)}else{v60})}))+(v1619*(if v1658{(v32*v5819)}else{(if v1652{(((v1647*(v1653*v5819))-(v1654*v5819))/v5826)}else{v60})})))/v739));
        let v5878=(v1670*v5873);
        let v5880=(v1670*v5874);
        let v5882=(v1670*v5875);
        let v5884=(v1670*v5876);
        let v5886=(v1670*v5877);
        let v5888=(v94*v1674);
        let v5904=(v1671*((v5873+((v5878+v5878)/v5888))/v94));
        let v5905=(v1671*((v5874+((v5880+v5880)/v5888))/v94));
        let v5906=(v1671*((v5875+((v5882+v5882)/v5888))/v94));
        let v5907=(v1671*((v5876+((v5884+v5884)/v5888))/v94));
        let v5908=(v1671*((v5877+((v5886+v5886)/v5888))/v94));
        let v5914=(v1516*v1516);
        let v5923=((v2720+(self.scalar_static_f64[220]*v5284))+(self.scalar_static_f64[221]*((-v5284)/v5914)));
        let v5924=((self.scalar_static_f64[220]*v5285)+(self.scalar_static_f64[221]*((-v5285)/v5914)));
        let v5925=((self.scalar_static_f64[220]*v5286)+(self.scalar_static_f64[221]*((-v5286)/v5914)));
        let v5939=(v1694*v1694);
        let v5950=(if self.scalar_static_bool[30]{v2690}else{(if (self.scalar_static_f64[223]!=0.0){(((v1694*v2690)-(v597*(if (self.scalar_static_f64[223]!=0.0){(((v627*v5923)-(v1687*v2720))/(v627*v627))}else{v60})))/v5939)}else{v60})});
        let v5951=(if self.scalar_static_bool[30]{v60}else{(if (self.scalar_static_f64[223]!=0.0){((-(v597*(if (self.scalar_static_f64[223]!=0.0){(v5924/v627)}else{v60})))/v5939)}else{v60})});
        let v5952=(if self.scalar_static_bool[30]{v60}else{(if (self.scalar_static_f64[223]!=0.0){((-(v597*(if (self.scalar_static_f64[223]!=0.0){(v5925/v627)}else{v60})))/v5939)}else{v60})});
        let v5957=((-(v10*(self.scalar_static_f64[225]*v2549)))/(v1701*v1701));
        let v5958=(self.scalar_static_f64[0]/v1701);
        let v5959=(self.scalar_static_f64[273]/v1701);
        let v5969=scalar_limexp_derivative(v1708);
        let v5984=((v1712*v2684)+(v591*((v1711*(if v1709{v60}else{(if (v1704!=0.0){v5957}else{v60})}))+(v1710*((if (v1704!=0.0){v60}else{v5957})*v5969)))));
        let v5985=(v591*((v1711*(if v1709{v60}else{(if (v1704!=0.0){v5958}else{v60})}))+(v1710*((if (v1704!=0.0){v60}else{v5958})*v5969))));
        let v5986=(v591*((v1711*(if v1709{v60}else{(if (v1704!=0.0){v5959}else{v60})}))+(v1710*((if (v1704!=0.0){v60}else{v5959})*v5969))));
        let v5991=((-(v7*(self.scalar_static_f64[226]*v2549)))/(v1715*v1715));
        let v5992=(self.scalar_static_f64[273]/v1715);
        let v5993=(self.scalar_static_f64[0]/v1715);
        let v6003=scalar_limexp_derivative(v1722);
        let v6018=((v1726*v2684)+(v591*((v1725*(if v1723{v60}else{(if (v1718!=0.0){v5991}else{v60})}))+(v1724*((if (v1718!=0.0){v60}else{v5991})*v6003)))));
        let v6019=(v591*((v1725*(if v1723{v60}else{(if (v1718!=0.0){v5992}else{v60})}))+(v1724*((if (v1718!=0.0){v60}else{v5992})*v6003))));
        let v6020=(v591*((v1725*(if v1723{v60}else{(if (v1718!=0.0){v5993}else{v60})}))+(v1724*((if (v1718!=0.0){v60}else{v5993})*v6003))));
        let v6024=(v1698*v1698);
        let v6033=(v5986/v1698);
        let v6037=((((v1698*v5984)-(v1713*v5950))/v6024)+(v6018/self.scalar_static_f64[224]));
        let v6038=(((-(v1713*v5951))/v6024)+(v6019/self.scalar_static_f64[224]));
        let v6039=((((v1698*v5985)-(v1713*v5952))/v6024)+(v6020/self.scalar_static_f64[224]));
        let v6043=(v1507*v1507);
        let v6069=(v751*v751);
        let v6085=(v1740*(v1733*(((v1736*((v1734*v5984)+(v1713*(((v1507*v5984)-(v1713*v5248))/v6043))))+(v1735*(((v751*v2844)-(v755*v2838))/v6069)))/v1737)));
        let v6086=(v1740*(v1733*((v1736*(v1713*((-(v1713*v5251))/v6043)))/v1737)));
        let v6087=(v1740*(v1733*((v1736*((v1734*v5985)+(v1713*(((v1507*v5985)-(v1713*v5254))/v6043))))/v1737)));
        let v6088=(v1740*(v1733*((v1736*((v1734*v5986)+(v1713*(((v1507*v5986)-(v1713*v5257))/v6043))))/v1737)));
        let v6100=(((v751*v5984)-(v1713*v2838))/v6069);
        let v6101=(v5985/v751);
        let v6102=(v5986/v751);
        let v6103=(v6037+v6100);
        let v6104=(v6039+v6101);
        let v6105=(v6033+v6102);
        let v6119=(v1678*v5904);
        let v6120=(v6119+v6119);
        let v6121=(v1678*v5905);
        let v6122=(v6121+v6121);
        let v6123=(v1678*v5906);
        let v6124=(v6123+v6123);
        let v6125=(v1678*v5907);
        let v6126=(v6125+v6125);
        let v6127=(v1678*v5908);
        let v6128=(v6127+v6127);
        let v6130=((if self.scalar_static_bool[32]{v6038}else{(if (self.scalar_static_f64[227]!=0.0){(v6038+v6086)}else{v60})})+v6124);
        let v6133=(v94*v1752);
        let v6139=(v5904+(v6120/v6133));
        let v6140=(v5905+(((if self.scalar_static_bool[32]{v6037}else{(if (self.scalar_static_f64[227]!=0.0){(v6037+v6085)}else{v60})})+v6122)/v6133));
        let v6141=(v5906+(v6130/v6133));
        let v6142=(v5907+(((if self.scalar_static_bool[32]{v6039}else{(if (self.scalar_static_f64[227]!=0.0){(v6039+v6087)}else{v60})})+v6126)/v6133));
        let v6143=(v5908+(((if self.scalar_static_bool[32]{v6033}else{(if (self.scalar_static_f64[227]!=0.0){(v6033+v6088)}else{v60})})+v6128)/v6133));
        let v6147=(v94*v1755);
        let v6165=(v1713*v1713);
        let v6166=(((v1713*(v5248/self.scalar_static_f64[228]))-(v1763*v5984))/v6165);
        let v6167=((v5251/self.scalar_static_f64[228])/v1713);
        let v6171=(((v1713*(v5254/self.scalar_static_f64[228]))-(v1763*v5985))/v6165);
        let v6175=(((v1713*(v5257/self.scalar_static_f64[228]))-(v1763*v5986))/v6165);
        let v6225=(v1771*v1771);
        let v6243=(if (v1761!=0.0){(((v1771*(if (v1761!=0.0){(-(v1764*v6139))}else{v60}))-(v1767*(if (v1761!=0.0){(v1764*((v5904+(v6120/v6147))-v6139))}else{v60})))/v6225)}else{v60});
        let v6244=(if (v1761!=0.0){(((v1771*(if (v1761!=0.0){(-((v1764*v6140)+(v1753*v6166)))}else{v60}))-(v1767*(if (v1761!=0.0){((v1768*v6166)+(v1764*((v5905+(((if self.scalar_static_bool[32]{v6103}else{(if (self.scalar_static_f64[227]!=0.0){(v6085+v6103)}else{v60})})+v6122)/v6147))-v6140)))}else{v60})))/v6225)}else{v60});
        let v6245=(if (v1761!=0.0){(((v1771*(if (v1761!=0.0){(-((v1764*v6141)+(v1753*v6167)))}else{v60}))-(v1767*(if (v1761!=0.0){((v1768*v6167)+(v1764*((v5906+(v6130/v6147))-v6141)))}else{v60})))/v6225)}else{v60});
        let v6246=(if (v1761!=0.0){(((v1771*(if (v1761!=0.0){(-((v1764*v6142)+(v1753*v6171)))}else{v60}))-(v1767*(if (v1761!=0.0){((v1768*v6171)+(v1764*((v5907+(((if self.scalar_static_bool[32]{v6104}else{(if (self.scalar_static_f64[227]!=0.0){(v6087+v6104)}else{v60})})+v6126)/v6147))-v6142)))}else{v60})))/v6225)}else{v60});
        let v6247=(if (v1761!=0.0){(((v1771*(if (v1761!=0.0){(-((v1764*v6143)+(v1753*v6175)))}else{v60}))-(v1767*(if (v1761!=0.0){((v1768*v6175)+(v1764*((v5908+(((if self.scalar_static_bool[32]{v6105}else{(if (self.scalar_static_f64[227]!=0.0){(v6088+v6105)}else{v60})})+v6128)/v6147))-v6143)))}else{v60})))/v6225)}else{v60});
        let v6248=(v1773*v6243);
        let v6250=(v1773*v6244);
        let v6252=(v1773*v6245);
        let v6254=(v1773*v6246);
        let v6256=(v1773*v6247);
        let v6258=(v94*v1777);
        let v6279=(if v1782{v60}else{(if (v1761!=0.0){((v6243+((v6248+v6248)/v6258))/v1779)}else{v60})});
        let v6280=(if v1782{v60}else{(if (v1761!=0.0){((v6244+((v6250+v6250)/v6258))/v1779)}else{v60})});
        let v6281=(if v1782{v60}else{(if (v1761!=0.0){((v6245+((v6252+v6252)/v6258))/v1779)}else{v60})});
        let v6282=(if v1782{v60}else{(if (v1761!=0.0){((v6246+((v6254+v6254)/v6258))/v1779)}else{v60})});
        let v6283=(if v1782{v60}else{(if (v1761!=0.0){((v6247+((v6256+v6256)/v6258))/v1779)}else{v60})});
        let v6297=((v1788*v6279)+(v1783*(v1743*v6279)));
        let v6310=(v6037+((v1788*v6280)+(v1783*((v1783*v6100)+(v1743*v6280)))));
        let v6311=(v6038+((v1788*v6281)+(v1783*(v1743*v6281))));
        let v6312=(v6039+((v1788*v6282)+(v1783*((v1783*v6101)+(v1743*v6282)))));
        let v6313=(v6033+((v1788*v6283)+(v1783*((v1783*v6102)+(v1743*v6283)))));
        let v6333=(v94*v1796);
        let v6354=(if self.scalar_static_bool[36]{(v1802*v5904)}else{v60});
        let v6355=(if self.scalar_static_bool[36]{(v1802*v5905)}else{v60});
        let v6356=(if self.scalar_static_bool[36]{(v1802*v5906)}else{v60});
        let v6357=(if self.scalar_static_bool[36]{(v1802*v5907)}else{v60});
        let v6358=(if self.scalar_static_bool[36]{(v1802*v5908)}else{v60});
        let v6364=(if self.scalar_static_bool[41]{(-v6297)}else{v60});
        let v6365=(if self.scalar_static_bool[41]{(-v6310)}else{v60});
        let v6366=(if self.scalar_static_bool[41]{(-v6311)}else{v60});
        let v6367=(if self.scalar_static_bool[41]{(-v6312)}else{v60});
        let v6368=(if self.scalar_static_bool[41]{(-v6313)}else{v60});
        let v6413=(v1804*v6354);
        let v6415=(v1804*v6355);
        let v6417=(v1804*v6356);
        let v6419=(v1804*v6357);
        let v6421=(v1804*v6358);
        let v6423=(if self.scalar_static_bool[36]{(v6413+v6413)}else{v60});
        let v6424=(if self.scalar_static_bool[36]{(v6415+v6415)}else{v60});
        let v6425=(if self.scalar_static_bool[36]{(v6417+v6417)}else{v60});
        let v6426=(if self.scalar_static_bool[36]{(v6419+v6419)}else{v60});
        let v6427=(if self.scalar_static_bool[36]{(v6421+v6421)}else{v60});
        let v6438=(if self.scalar_static_bool[36]{(v6364-(self.scalar_static_f64[231]*v6423))}else{v60});
        let v6439=(if self.scalar_static_bool[36]{(v6365-(self.scalar_static_f64[231]*v6424))}else{v60});
        let v6440=(if self.scalar_static_bool[36]{(v6366-(self.scalar_static_f64[231]*v6425))}else{v60});
        let v6441=(if self.scalar_static_bool[36]{(v6367-(self.scalar_static_f64[231]*v6426))}else{v60});
        let v6442=(if self.scalar_static_bool[36]{(v6368-(self.scalar_static_f64[231]*v6427))}else{v60});
        let v6497=(if self.scalar_static_bool[36]{((((v1825*v6423)+(v1821*(v94*v6354)))/v1827)-(self.scalar_static_f64[231]*((v1813*v6354)+(v1804*v6364))))}else{v60});
        let v6498=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{(((v751*((v1816*v2844)+(v755*(((v1507*((v1814*v5984)+(v1713*(-v5984))))-(v1815*v5248))/v6043))))-(v1817*v2838))/v6069)}else{v60})+((((v1825*v6424)+(v1821*(v94*v6355)))/v1827)-(self.scalar_static_f64[231]*((v1813*v6355)+(v1804*v6365)))))}else{v60});
        let v6499=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v755*((-(v1815*v5251))/v6043))/v751)}else{v60})+((((v1825*v6425)+(v1821*(v94*v6356)))/v1827)-(self.scalar_static_f64[231]*((v1813*v6356)+(v1804*v6366)))))}else{v60});
        let v6500=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v755*(((v1507*((v1814*v5985)+(v1713*(-v5985))))-(v1815*v5254))/v6043))/v751)}else{v60})+((((v1825*v6426)+(v1821*(v94*v6357)))/v1827)-(self.scalar_static_f64[231]*((v1813*v6357)+(v1804*v6367)))))}else{v60});
        let v6501=(if self.scalar_static_bool[36]{((if self.scalar_static_bool[36]{((v755*(((v1507*((v1814*v5986)+(v1713*(-v5986))))-(v1815*v5257))/v6043))/v751)}else{v60})+((((v1825*v6427)+(v1821*(v94*v6358)))/v1827)-(self.scalar_static_f64[231]*((v1813*v6358)+(v1804*v6368)))))}else{v60});
        let v6502=(v1833*v6497);
        let v6504=(v1833*v6498);
        let v6506=(v1833*v6499);
        let v6508=(v1833*v6500);
        let v6510=(v1833*v6501);
        let v6517=(v1824*v6438);
        let v6519=(v1824*v6439);
        let v6521=(v1824*v6440);
        let v6523=(v1824*v6441);
        let v6525=(v1824*v6442);
        let v6529=((v1837*v6438)+(v1824*(v6517+v6517)));
        let v6532=((v1837*v6439)+(v1824*(v6519+v6519)));
        let v6535=((v1837*v6440)+(v1824*(v6521+v6521)));
        let v6538=((v1837*v6441)+(v1824*(v6523+v6523)));
        let v6541=((v1837*v6442)+(v1824*(v6525+v6525)));
        let v6582=(self.scalar_static_f64[231]*v6354);
        let v6583=(self.scalar_static_f64[231]*v6355);
        let v6584=(self.scalar_static_f64[231]*v6356);
        let v6585=(self.scalar_static_f64[231]*v6357);
        let v6586=(self.scalar_static_f64[231]*v6358);
        let v6602=(v32*(-v6497));
        let v6603=(v32*(-v6498));
        let v6604=(v32*(-v6499));
        let v6605=(v32*(-v6500));
        let v6606=(v32*(-v6501));
        let v6607=(if v1856{v6602}else{v60});
        let v6608=(if v1856{v6603}else{v60});
        let v6609=(if v1856{v6604}else{v60});
        let v6610=(if v1856{v6605}else{v60});
        let v6611=(if v1856{v6606}else{v60});
        let v6612=(v94*v1860);
        let v6618=(if v1856{((if self.scalar_static_bool[36]{((v1835*(v6502+v6502))+(v6529/v1827))}else{v60})/v6612)}else{v60});
        let v6619=(if v1856{((if self.scalar_static_bool[36]{((v1835*(v6504+v6504))+(v6532/v1827))}else{v60})/v6612)}else{v60});
        let v6620=(if v1856{((if self.scalar_static_bool[36]{((v1835*(v6506+v6506))+(v6535/v1827))}else{v60})/v6612)}else{v60});
        let v6621=(if v1856{((if self.scalar_static_bool[36]{((v1835*(v6508+v6508))+(v6538/v1827))}else{v60})/v6612)}else{v60});
        let v6622=(if v1856{((if self.scalar_static_bool[36]{((v1835*(v6510+v6510))+(v6541/v1827))}else{v60})/v6612)}else{v60});
        let v6628=(if v1856{(v6607+v6618)}else{v6423});
        let v6629=(if v1856{(v6608+v6619)}else{v6424});
        let v6630=(if v1856{(v6609+v6620)}else{v6425});
        let v6631=(if v1856{(v6610+v6621)}else{v6426});
        let v6632=(if v1856{(v6611+v6622)}else{v6427});
        let v6688=(if v1856{(v6607-v6618)}else{v6628});
        let v6689=(if v1856{(v6608-v6619)}else{v6629});
        let v6690=(if v1856{(v6609-v6620)}else{v6630});
        let v6691=(if v1856{(v6610-v6621)}else{v6631});
        let v6692=(if v1856{(v6611-v6622)}else{v6632});
        let v6760=(v1838*v1838);
        let v6774=(v94*v1903);
        let v6795=(if v1900{((v1903*v6602)+(v1858*(((-(v1901*v6529))/v6760)/v6774)))}else{v6688});
        let v6796=(if v1900{((v1903*v6603)+(v1858*(((-(v1901*v6532))/v6760)/v6774)))}else{v6689});
        let v6797=(if v1900{((v1903*v6604)+(v1858*(((-(v1901*v6535))/v6760)/v6774)))}else{v6690});
        let v6798=(if v1900{((v1903*v6605)+(v1858*(((-(v1901*v6538))/v6760)/v6774)))}else{v6691});
        let v6799=(if v1900{((v1903*v6606)+(v1858*(((-(v1901*v6541))/v6760)/v6774)))}else{v6692});
        let v6800=(v1905*v6795);
        let v6802=(v1905*v6796);
        let v6804=(v1905*v6797);
        let v6806=(v1905*v6798);
        let v6808=(v1905*v6799);
        let v6810=(if v1900{(v6800+v6800)}else{v6607});
        let v6811=(if v1900{(v6802+v6802)}else{v6608});
        let v6812=(if v1900{(v6804+v6804)}else{v6609});
        let v6813=(if v1900{(v6806+v6806)}else{v6610});
        let v6814=(if v1900{(v6808+v6808)}else{v6611});
        let v6823=(v1912*v1912);
        let v6841=(v94*v1914);
        let v6848=(v48+(v1914*v1914));
        let v6849=(((((v1912*v6810)-(v1907*(-v6810)))/v6823)/v6841)/v6848);
        let v6850=(((((v1912*v6811)-(v1907*(-v6811)))/v6823)/v6841)/v6848);
        let v6851=(((((v1912*v6812)-(v1907*(-v6812)))/v6823)/v6841)/v6848);
        let v6852=(((((v1912*v6813)-(v1907*(-v6813)))/v6823)/v6841)/v6848);
        let v6853=(((((v1912*v6814)-(v1907*(-v6814)))/v6823)/v6841)/v6848);
        let v6864=(if v1919{v6849}else{(if v1910{(-v6849)}else{v6795})});
        let v6865=(if v1919{v6850}else{(if v1910{(-v6850)}else{v6796})});
        let v6866=(if v1919{v6851}else{(if v1910{(-v6851)}else{v6797})});
        let v6867=(if v1919{v6852}else{(if v1910{(-v6852)}else{v6798})});
        let v6868=(if v1919{v6853}else{(if v1910{(-v6853)}else{v6799})});
        let v6879=(v94*v1925);
        let v6890=(v1926).sin();
        let v6936=(if (v1935!=0.0){v60}else{(if self.scalar_static_bool[36]{(if v1900{(if v1900{(((v1927*((self.scalar_static_f64[231]*(v1922*v6438))/v6879))+(v1925*(-((self.scalar_static_f64[231]*v6864)*v6890))))-v6582)}else{v6864})}else{(if v1856{(((if v1872{(-(v1876*(self.scalar_static_f64[231]*((-v6628)/v1873))))}else{(if v1866{(v1869*(self.scalar_static_f64[231]*(v6628/v1863)))}else{v60})})+(if v1889{(-(v1893*(self.scalar_static_f64[231]*((-v6688)/v1890))))}else{(if v1883{(v1886*(self.scalar_static_f64[231]*(v6688/v1880)))}else{v60})}))-v6582)}else{(if v1846{((((v1824*(v43*v6497))-(v1847*v6438))/v1837)-v6582)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v5904+((v6120+(if self.scalar_static_bool[35]{v6297}else{(if self.scalar_static_bool[34]{v6297}else{v60})}))/v6333))}else{v60})})});
        let v6937=(if (v1935!=0.0){v60}else{(if self.scalar_static_bool[36]{(if v1900{(if v1900{(((v1927*((self.scalar_static_f64[231]*(v1922*v6439))/v6879))+(v1925*(-((self.scalar_static_f64[231]*v6865)*v6890))))-v6583)}else{v6865})}else{(if v1856{(((if v1872{(-(v1876*(self.scalar_static_f64[231]*((-v6629)/v1873))))}else{(if v1866{(v1869*(self.scalar_static_f64[231]*(v6629/v1863)))}else{v60})})+(if v1889{(-(v1893*(self.scalar_static_f64[231]*((-v6689)/v1890))))}else{(if v1883{(v1886*(self.scalar_static_f64[231]*(v6689/v1880)))}else{v60})}))-v6583)}else{(if v1846{((((v1824*(v43*v6498))-(v1847*v6439))/v1837)-v6583)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v5905+((v6122+(if self.scalar_static_bool[35]{v6310}else{(if self.scalar_static_bool[34]{(v6085+v6310)}else{v60})}))/v6333))}else{v60})})});
        let v6938=(if (v1935!=0.0){v60}else{(if self.scalar_static_bool[36]{(if v1900{(if v1900{(((v1927*((self.scalar_static_f64[231]*(v1922*v6440))/v6879))+(v1925*(-((self.scalar_static_f64[231]*v6866)*v6890))))-v6584)}else{v6866})}else{(if v1856{(((if v1872{(-(v1876*(self.scalar_static_f64[231]*((-v6630)/v1873))))}else{(if v1866{(v1869*(self.scalar_static_f64[231]*(v6630/v1863)))}else{v60})})+(if v1889{(-(v1893*(self.scalar_static_f64[231]*((-v6690)/v1890))))}else{(if v1883{(v1886*(self.scalar_static_f64[231]*(v6690/v1880)))}else{v60})}))-v6584)}else{(if v1846{((((v1824*(v43*v6499))-(v1847*v6440))/v1837)-v6584)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v5906+((v6124+(if self.scalar_static_bool[35]{v6311}else{(if self.scalar_static_bool[34]{(v6086+v6311)}else{v60})}))/v6333))}else{v60})})});
        let v6939=(if (v1935!=0.0){v60}else{(if self.scalar_static_bool[36]{(if v1900{(if v1900{(((v1927*((self.scalar_static_f64[231]*(v1922*v6441))/v6879))+(v1925*(-((self.scalar_static_f64[231]*v6867)*v6890))))-v6585)}else{v6867})}else{(if v1856{(((if v1872{(-(v1876*(self.scalar_static_f64[231]*((-v6631)/v1873))))}else{(if v1866{(v1869*(self.scalar_static_f64[231]*(v6631/v1863)))}else{v60})})+(if v1889{(-(v1893*(self.scalar_static_f64[231]*((-v6691)/v1890))))}else{(if v1883{(v1886*(self.scalar_static_f64[231]*(v6691/v1880)))}else{v60})}))-v6585)}else{(if v1846{((((v1824*(v43*v6500))-(v1847*v6441))/v1837)-v6585)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v5907+((v6126+(if self.scalar_static_bool[35]{v6312}else{(if self.scalar_static_bool[34]{(v6087+v6312)}else{v60})}))/v6333))}else{v60})})});
        let v6940=(if (v1935!=0.0){v60}else{(if self.scalar_static_bool[36]{(if v1900{(if v1900{(((v1927*((self.scalar_static_f64[231]*(v1922*v6442))/v6879))+(v1925*(-((self.scalar_static_f64[231]*v6868)*v6890))))-v6586)}else{v6868})}else{(if v1856{(((if v1872{(-(v1876*(self.scalar_static_f64[231]*((-v6632)/v1873))))}else{(if v1866{(v1869*(self.scalar_static_f64[231]*(v6632/v1863)))}else{v60})})+(if v1889{(-(v1893*(self.scalar_static_f64[231]*((-v6692)/v1890))))}else{(if v1883{(v1886*(self.scalar_static_f64[231]*(v6692/v1880)))}else{v60})}))-v6586)}else{(if v1846{((((v1824*(v43*v6501))-(v1847*v6442))/v1837)-v6586)}else{v60})})})}else{(if (self.scalar_static_f64[230]!=0.0){(v5908+((v6128+(if self.scalar_static_bool[35]{v6313}else{(if self.scalar_static_bool[34]{(v6088+v6313)}else{v60})}))/v6333))}else{v60})})});
        let v6943=(v1936*v1936);
        let v6962=((-(v1727*v6936))/v6943);
        let v6966=(((v1936*v6018)-(v1727*v6937))/v6943);
        let v6970=(((v1936*v6019)-(v1727*v6938))/v6943);
        let v6974=(((v1936*v6020)-(v1727*v6939))/v6943);
        let v6977=((-(v1727*v6940))/v6943);
        let v6978=(if (v1940!=0.0){v60}else{((-(v1713*v6936))/v6943)});
        let v6979=(if (v1940!=0.0){v60}else{(((v1936*v5984)-(v1713*v6937))/v6943)});
        let v6980=(if (v1940!=0.0){v60}else{((-(v1713*v6938))/v6943)});
        let v6981=(if (v1940!=0.0){v60}else{(((v1936*v5985)-(v1713*v6939))/v6943)});
        let v6982=(if (v1940!=0.0){v60}else{(((v1936*v5986)-(v1713*v6940))/v6943)});
        let v7001=(v1941*v1941);
        let v7003=(v1941*v5248);
        let v7004=(v1507*v6979);
        let v7007=(v1941*v5251);
        let v7008=(v1507*v6980);
        let v7011=(v1941*v5254);
        let v7012=(v1507*v6981);
        let v7015=(v1941*v5257);
        let v7016=(v1507*v6982);
        let v7019=(-((-(v1507*v6978))/v7001));
        let v7020=(-((v7003-v7004)/v7001));
        let v7021=(-((v7007-v7008)/v7001));
        let v7022=(-((v7011-v7012)/v7001));
        let v7023=(-((v7015-v7016)/v7001));
        let v7024=(v1945*v7019);
        let v7026=(v1945*v7020);
        let v7028=(v1945*v7021);
        let v7030=(v1945*v7022);
        let v7032=(v1945*v7023);
        let v7034=(v94*v1949);
        let v7045=((v7019+((v7024+v7024)/v7034))/self.scalar_static_f64[236]);
        let v7046=((v7020+((v7026+v7026)/v7034))/self.scalar_static_f64[236]);
        let v7047=((v7021+((v7028+v7028)/v7034))/self.scalar_static_f64[236]);
        let v7048=((v7022+((v7030+v7030)/v7034))/self.scalar_static_f64[236]);
        let v7049=((v7023+((v7032+v7032)/v7034))/self.scalar_static_f64[236]);
        let v7143=(((v1956*v6978)+(v1941*((v1955*v7045)+(v1954*(v640*v7045)))))+((v1687*v6978)+(((v1963*v6978)+(v1941*(v636*(v1962*(self.scalar_static_f64[237]*((v6978/v1507)/v1959))))))/self.scalar_static_f64[238])));
        let v7144=(((v1956*v6979)+(v1941*((v1955*v7046)+(v1954*((v1954*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[93]*(v638*(self.scalar_static_f64[94]*v2558)))}else{v60}))+(v640*v7046))))))+(((v1941*v5923)+(v1687*v6979))+(((v1963*v6979)+(v1941*((v1962*(if self.scalar_static_bool[18]{v60}else{(if self.scalar_static_bool[17]{(self.scalar_static_f64[92]*(v632*((self.scalar_static_f64[28]*v2558)-v2722)))}else{v60})}))+(v636*(v1962*(self.scalar_static_f64[237]*(((v7004-v7003)/v6043)/v1959)))))))/self.scalar_static_f64[238])));
        let v7145=(((v1956*v6980)+(v1941*((v1955*v7047)+(v1954*(v640*v7047)))))+(((v1941*v5924)+(v1687*v6980))+(((v1963*v6980)+(v1941*(v636*(v1962*(self.scalar_static_f64[237]*(((v7008-v7007)/v6043)/v1959))))))/self.scalar_static_f64[238])));
        let v7146=(((v1956*v6981)+(v1941*((v1955*v7048)+(v1954*(v640*v7048)))))+(((v1941*v5925)+(v1687*v6981))+(((v1963*v6981)+(v1941*(v636*(v1962*(self.scalar_static_f64[237]*(((v7012-v7011)/v6043)/v1959))))))/self.scalar_static_f64[238])));
        let v7147=(((v1956*v6982)+(v1941*((v1955*v7049)+(v1954*(v640*v7049)))))+((v1687*v6982)+(((v1963*v6982)+(v1941*(v636*(v1962*(self.scalar_static_f64[237]*(((v7016-v7015)/v6043)/v1959))))))/self.scalar_static_f64[238])));
        let v7299=(if v2044{v2869}else{v4453});
        let v7300=(if v2044{v2871}else{v4454});
        let v7302=(if v2044{(v795*v2672)}else{v4456});
        let v7308=(if v2044{((v2052*v2672)+(v579*(v2052*(v2050*v2878))))}else{v4462});
        let v7312=(if v2044{v60}else{v4466});
        let v7313=(if v2044{((v2055*v2553)+(v456*v7300))}else{v4467});
        let v7314=(if v2044{v2889}else{v4468});
        let v7315=(if v2044{v2885}else{v4469});
        let v7320=(if v2060{(v2061*v7312)}else{v4545});
        let v7321=(if v2060{(v2061*v7313)}else{v4546});
        let v7322=(if v2060{(v2061*v7314)}else{v4547});
        let v7323=(if v2060{(v2061*v7315)}else{v4548});
        let v7367=(if v2071{v60}else{(if v2060{(-(v454*(v7320/v2063)))}else{v4521})});
        let v7368=(if v2071{v60}else{(if v2060{(v7300-((v2066*v2549)+(v454*(v7321/v2063))))}else{v4522})});
        let v7369=(if v2071{self.scalar_static_f64[273]}else{(if v2060{(-(v454*(v7322/v2063)))}else{v4523})});
        let v7370=(if v2071{self.scalar_static_f64[0]}else{(if v2060{(-(v454*(v7323/v2063)))}else{v4524})});
        let v7373=(if v2044{(v2936+(v831*v7299))}else{v4527});
        let v7379=(v2076*v2076);
        let v7383=(if v2044{(v7367/v2076)}else{v4537});
        let v7384=(if v2044{(((v2076*(v7299+v7368))-(v2077*v7373))/v7379)}else{v4538});
        let v7385=(if v2044{(v7369/v2076)}else{v4539});
        let v7386=(if v2044{(v7370/v2076)}else{v4540});
        let v7391=(if v2082{(v2083*v7383)}else{v7320});
        let v7392=(if v2082{(v2083*v7384)}else{v7321});
        let v7393=(if v2082{(v2083*v7385)}else{v7322});
        let v7394=(if v2082{(v2083*v7386)}else{v7323});
        let v7444=(if v2099{v7367}else{(if v2082{(v2076*(v7391/v2085))}else{v4598})});
        let v7445=(if v2099{v7368}else{(if v2082{((-v7299)+((v2094*v7373)+(v2076*((v7392/v2085)-(v2093*(((v2076*(-(v7299+v7300)))-(v2091*v7373))/v7379))))))}else{v4599})});
        let v7446=(if v2099{v7369}else{(if v2082{(v2076*(v7393/v2085))}else{v4600})});
        let v7447=(if v2099{v7370}else{(if v2082{(v2076*(v7394/v2085))}else{v4601})});
        let v7471=(if v2044{((-(v7367/v573))/v2105)}else{v4625});
        let v7472=(if v2044{((-(((v573*v7368)-(v2073*v2663))/v2666))/v2105)}else{v4626});
        let v7473=(if v2044{((-(v7369/v573))/v2105)}else{v4627});
        let v7474=(if v2044{((-(v7370/v573))/v2105)}else{v4628});
        let v7490=(if v2044{((-(v7444/v573))/v2109)}else{v4644});
        let v7491=(if v2044{((-(((v573*v7445)-(v2101*v2663))/v2666))/v2109)}else{v4645});
        let v7492=(if v2044{((-(v7446/v573))/v2109)}else{v4646});
        let v7493=(if v2044{((-(v7447/v573))/v2109)}else{v4647});
        let v7678=(if v2155{v2871}else{v5523});
        let v7682=(if v2155{v60}else{v5527});
        let v7683=(if v2155{((v2157*v2553)+(v456*v7678))}else{v5528});
        let v7684=(if v2155{v2889}else{v5529});
        let v7685=(if v2155{v2885}else{v5530});
        let v7686=(if v2155{v60}else{v5531});
        let v7687=(v2159*v7682);
        let v7689=(v2159*v7683);
        let v7691=(v2159*v7684);
        let v7693=(v2159*v7685);
        let v7695=(v2159*v7686);
        let v7697=(v94*v2162);
        let v7703=(if v2155{((v7687+v7687)/v7697)}else{v5548});
        let v7704=(if v2155{((v7689+v7689)/v7697)}else{v5549});
        let v7705=(if v2155{((v7691+v7691)/v7697)}else{v5550});
        let v7706=(if v2155{((v7693+v7693)/v7697)}else{v5551});
        let v7707=(if v2155{((v7695+v7695)/v7697)}else{v5552});
        let v7718=(if v2155{(v32*(v7682+v7703))}else{v5563});
        let v7719=(if v2155{(v32*(v7683+v7704))}else{v5564});
        let v7720=(if v2155{(v32*(v7684+v7705))}else{v5565});
        let v7721=(if v2155{(v32*(v7685+v7706))}else{v5566});
        let v7722=(if v2155{(v32*(v7686+v7707))}else{v5567});
        let v7735=(if v2155{(-(v454*v7718))}else{v5580});
        let v7736=(if v2155{(v7678-((v2166*v2549)+(v454*v7719)))}else{v5581});
        let v7737=(if v2155{(-(v454*v7720))}else{v5582});
        let v7738=(if v2155{(-(v454*v7721))}else{v5583});
        let v7739=(if v2155{(-(v454*v7722))}else{v5584});
        let v7784=(if v2155{((-(v7735/v573))/v2173)}else{v5629});
        let v7785=(if v2155{((-(((v573*v7736)-(v2169*v2663))/v2666))/v2173)}else{v5630});
        let v7786=(if v2155{((-(v7737/v573))/v2173)}else{v5631});
        let v7787=(if v2155{((-(v7738/v573))/v2173)}else{v5632});
        let v7788=(if v2155{((-(v7739/v573))/v2173)}else{v5633});
        let v8372=(if v2320{(-v2793)}else{v7299});
        let v8373=(self.scalar_static_f64[258]*v2793);
        let v8374=(if v2320{v8373}else{v7300});
        let v8386=(if v2320{((v2336*v2802)+(v717*(v2336*(v2332*(((-(self.scalar_static_f64[253]*v2793))/v2796)/v2333)))))}else{v7308});
        let v8400=(if v2344{(v2345*(if v2320{v60}else{v7312}))}else{v7391});
        let v8401=(if v2344{(v2345*(if v2320{v2885}else{v60}))}else{v60});
        let v8402=(if v2344{(v2345*(if v2320{((v2339*v2553)+(v456*v8374))}else{v7313}))}else{v7392});
        let v8403=(if v2344{(v2345*(if v2320{v2889}else{v7314}))}else{v7393});
        let v8404=(if v2344{(v2345*(if v2320{v60}else{v7315}))}else{v7394});
        let v8427=(if v2353{v60}else{(if v2344{(-(v454*(v8400/v2347)))}else{v7367})});
        let v8428=(if v2353{self.scalar_static_f64[0]}else{(if v2344{(-(v454*(v8401/v2347)))}else{v60})});
        let v8429=(if v2353{v60}else{(if v2344{(v8374-((v2348*v2549)+(v454*(v8402/v2347))))}else{v7368})});
        let v8430=(if v2353{self.scalar_static_f64[273]}else{(if v2344{(-(v454*(v8403/v2347)))}else{v7369})});
        let v8431=(if v2353{v60}else{(if v2344{(-(v454*(v8404/v2347)))}else{v7370})});
        let v8434=(if v2320{(v2936+(v831*v8372))}else{v7373});
        let v8441=(v2357*v2357);
        let v8543=(if v2320{((-((if v2378{v8427}else{(if v2363{(v2357*((if v2363{(v2364*(if v2320{(v8427/v2357)}else{v7383}))}else{v8400})/v2366))}else{v7444})})/v711))/v2387)}else{v7490});
        let v8544=(if v2320{((-((if v2378{v8428}else{(if v2363{(v2357*((if v2363{(v2364*(if v2320{(v8428/v2357)}else{v60}))}else{v8401})/v2366))}else{v60})})/v711))/v2387)}else{v60});
        let v8545=(if v2320{((-(((v711*(if v2378{v8429}else{(if v2363{((-v8372)+((v2373*v8434)+(v2357*(((if v2363{(v2364*(if v2320{(((v2357*(v8372+v8429))-(v2358*v8434))/v8441)}else{v7384}))}else{v8402})/v2366)-(v2372*(((v2357*(-(v8372+v8374)))-(v2370*v8434))/v8441))))))}else{v7445})}))-(v2379*v2793))/v2796))/v2387)}else{v7491});
        let v8546=(if v2320{((-((if v2378{v8430}else{(if v2363{(v2357*((if v2363{(v2364*(if v2320{(v8430/v2357)}else{v7385}))}else{v8403})/v2366))}else{v7446})})/v711))/v2387)}else{v7492});
        let v8547=(if v2320{((-((if v2378{v8431}else{(if v2363{(v2357*((if v2363{(v2364*(if v2320{(v8431/v2357)}else{v7386}))}else{v8404})/v2366))}else{v7447})})/v711))/v2387)}else{v7493});
        let v8670=(((v2413*v2793)+(v711*(((if v2320{(((v2396*v2802)+(v717*(-(v2395*(v2391*v8545)))))/v2391)}else{(if v2044{(((v2136*v2672)+(v579*(-(v2135*(v2112*v7491)))))/v2112)}else{v4761})})+(if v2320{(((v2402*v8386)+(v2338*(-(v2401*(v2393*(if v2320{((-(((v711*v8429)-(v2354*v2793))/v2796))/v2383)}else{v7472}))))))/v2393)}else{(if v2044{(((v2142*v7308)+(v2054*(-(v2141*(v2114*v7472)))))/v2114)}else{v4787})}))-(if v2320{(((v2408*v8386)+(v2338*(-(v2407*(v2393*v8545)))))/v2393)}else{(if v2044{(((v2148*v7308)+(v2054*(-(v2147*(v2114*v7491)))))/v2114)}else{v4813})}))))+((v2381*(if v2320{(v795*v2802)}else{v7302}))+(v2331*(if v2320{(-v8429)}else{(if v2044{(-v7368)}else{v4607})}))));
        let v8683=(if v2422{v8373}else{v7678});
        let v8687=(if v2422{v60}else{v7682});
        let v8688=(if v2422{v2885}else{v60});
        let v8689=(if v2422{((v2424*v2553)+(v456*v8683))}else{v7683});
        let v8690=(if v2422{v2889}else{v7684});
        let v8691=(if v2422{v60}else{v7685});
        let v8692=(if v2422{v60}else{v7686});
        let v8693=(v2426*v8687);
        let v8695=(v2426*v8688);
        let v8697=(v2426*v8689);
        let v8699=(v2426*v8690);
        let v8701=(v2426*v8691);
        let v8703=(v2426*v8692);
        let v8705=(v94*v2429);
        let v8750=(if v2422{(-(v454*(if v2422{(v32*(v8687+(if v2422{((v8693+v8693)/v8705)}else{v7703})))}else{v7718})))}else{v7735});
        let v8751=(if v2422{(-(v454*(if v2422{(v32*(v8688+(if v2422{((v8695+v8695)/v8705)}else{v60})))}else{v60})))}else{v60});
        let v8752=(if v2422{(v8683-((v2433*v2549)+(v454*(if v2422{(v32*(v8689+(if v2422{((v8697+v8697)/v8705)}else{v7704})))}else{v7719}))))}else{v7736});
        let v8753=(if v2422{(-(v454*(if v2422{(v32*(v8690+(if v2422{((v8699+v8699)/v8705)}else{v7705})))}else{v7720})))}else{v7737});
        let v8754=(if v2422{(-(v454*(if v2422{(v32*(v8691+(if v2422{((v8701+v8701)/v8705)}else{v7706})))}else{v7721})))}else{v7738});
        let v8755=(if v2422{(-(v454*(if v2422{(v32*(v8692+(if v2422{((v8703+v8703)/v8705)}else{v7707})))}else{v7722})))}else{v7739});
        let v8847=(if v2422{(v717*((if v2422{((v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(v8750/v711))/v2438)}else{v7784})))))/self.scalar_static_f64[259])}else{(if v2155{((v573*(-(v2186*(self.scalar_static_f64[197]*v7784))))/self.scalar_static_f64[197])}else{v5691})})+(v795*(-v8750))))}else{(if v2419{v60}else{(if v2320{((v711*(((if v2320{((v717*(-(v2395*(v2391*v8543))))/v2391)}else{(if v2044{((v579*(-(v2135*(v2112*v7490))))/v2112)}else{v4760})})+(if v2320{((v2338*(-(v2401*(v2393*(if v2320{((-(v8427/v711))/v2383)}else{v7471})))))/v2393)}else{(if v2044{((v2054*(-(v2141*(v2114*v7471))))/v2114)}else{v4786})}))-(if v2320{((v2338*(-(v2407*(v2393*v8543))))/v2393)}else{(if v2044{((v2054*(-(v2147*(v2114*v7490))))/v2114)}else{v4812})})))+(v2331*(if v2320{(-v8427)}else{(if v2044{(-v7367)}else{v4606})})))}else{v60})})});
        let v8850=(if v2422{(v717*((if v2422{((v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(v8753/v711))/v2438)}else{v7786})))))/self.scalar_static_f64[259])}else{(if v2155{((v573*(-(v2186*(self.scalar_static_f64[197]*v7786))))/self.scalar_static_f64[197])}else{v5693})})+(v795*(self.scalar_static_f64[273]-v8753))))}else{(if v2419{v60}else{(if v2320{((v711*(((if v2320{((v717*(-(v2395*(v2391*v8546))))/v2391)}else{(if v2044{((v579*(-(v2135*(v2112*v7492))))/v2112)}else{v4762})})+(if v2320{((v2338*(-(v2401*(v2393*(if v2320{((-(v8430/v711))/v2383)}else{v7473})))))/v2393)}else{(if v2044{((v2054*(-(v2141*(v2114*v7473))))/v2114)}else{v4788})}))-(if v2320{((v2338*(-(v2407*(v2393*v8546))))/v2393)}else{(if v2044{((v2054*(-(v2147*(v2114*v7492))))/v2114)}else{v4814})})))+(v2331*(if v2320{(self.scalar_static_f64[273]-v8430)}else{(if v2044{(self.scalar_static_f64[273]-v7369)}else{v4608})})))}else{v60})})});
        let v8851=(if v2422{(v717*((if v2422{((v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(v8754/v711))/v2438)}else{v7787})))))/self.scalar_static_f64[259])}else{(if v2155{((v573*(-(v2186*(self.scalar_static_f64[197]*v7787))))/self.scalar_static_f64[197])}else{v5694})})+(v795*(-v8754))))}else{(if v2419{v60}else{(if v2320{((v711*(((if v2320{((v717*(-(v2395*(v2391*v8547))))/v2391)}else{(if v2044{((v579*(-(v2135*(v2112*v7493))))/v2112)}else{v4763})})+(if v2320{((v2338*(-(v2401*(v2393*(if v2320{((-(v8431/v711))/v2383)}else{v7474})))))/v2393)}else{(if v2044{((v2054*(-(v2141*(v2114*v7474))))/v2114)}else{v4789})}))-(if v2320{((v2338*(-(v2407*(v2393*v8547))))/v2393)}else{(if v2044{((v2054*(-(v2147*(v2114*v7493))))/v2114)}else{v4815})})))+(v2331*(if v2320{(-v8431)}else{(if v2044{(self.scalar_static_f64[0]-v7370)}else{v4609})})))}else{v60})})});
        let v8889=(if (self.scalar_static_f64[262]!=0.0){v60}else{v7143});
        let v8890=(if (self.scalar_static_f64[262]!=0.0){v60}else{v7144});
        let v8891=(if (self.scalar_static_f64[262]!=0.0){v60}else{v7145});
        let v8892=(if (self.scalar_static_f64[262]!=0.0){v60}else{v7146});
        let v8893=(if (self.scalar_static_f64[262]!=0.0){v60}else{v7147});
        let v8924=(if (self.scalar_static_f64[262]!=0.0){v60}else{v6978});
        let v8925=(if (self.scalar_static_f64[262]!=0.0){v60}else{v6979});
        let v8926=(if (self.scalar_static_f64[262]!=0.0){v60}else{v6980});
        let v8927=(if (self.scalar_static_f64[262]!=0.0){v60}else{v6981});
        let v8928=(if (self.scalar_static_f64[262]!=0.0){v60}else{v6982});
        let v8963=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[263]*v8889))}else{v60})});
        let v8964=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[263]*v8890))}else{v60})});
        let v8965=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[263]*v8891))}else{v60})});
        let v8966=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[263]*v8892))}else{v60})});
        let v8967=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[263]*v8893))}else{v60})});
        let v8974=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[264]*v8924))}else{v60})});
        let v8975=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[264]*v8925))}else{v60})});
        let v8976=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[264]*v8926))}else{v60})});
        let v8977=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[264]*v8927))}else{v60})});
        let v8978=(if self.scalar_static_bool[59]{v60}else{(if (self.scalar_static_f64[262]!=0.0){(self.scalar_static_f64[87]*(self.scalar_static_f64[264]*v8928))}else{v60})});
        let v9003=(self.scalar_static_f64[0]*(if v2452{v60}else{v8847}));
        let v9004=(self.scalar_static_f64[0]*(if v2452{v60}else{(if v2422{(v717*((if v2422{((v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(v8751/v711))/v2438)}else{v60})))))/self.scalar_static_f64[259])}else{v60})+(v795*(self.scalar_static_f64[0]-v8751))))}else{(if v2419{v60}else{(if v2320{((v711*(((if v2320{((v717*(-(v2395*(v2391*v8544))))/v2391)}else{v60})+(if v2320{((v2338*(-(v2401*(v2393*(if v2320{((-(v8428/v711))/v2383)}else{v60})))))/v2393)}else{v60}))-(if v2320{((v2338*(-(v2407*(v2393*v8544))))/v2393)}else{v60})))+(v2331*(if v2320{(self.scalar_static_f64[0]-v8428)}else{v60})))}else{v60})})})}));
        let v9005=(self.scalar_static_f64[0]*(if v2452{v60}else{(if v2422{((v2449*v2802)+(v717*((if v2422{(((v2443*v2793)+(v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(((v711*v8752)-(v2436*v2793))/v2796))/v2438)}else{v7785}))))))/self.scalar_static_f64[259])}else{(if v2155{(((v2187*v2663)+(v573*(-(v2186*(self.scalar_static_f64[197]*v7785)))))/self.scalar_static_f64[197])}else{v5692})})+(v795*(-v8752)))))}else{(if v2419{v60}else{(if v2320{v8670}else{v60})})})}));
        let v9006=(self.scalar_static_f64[0]*(if v2452{v60}else{v8850}));
        let v9007=(self.scalar_static_f64[0]*(if v2452{v60}else{v8851}));
        let v9008=(self.scalar_static_f64[0]*(if v2452{v60}else{(if v2422{(v717*((if v2422{((v711*(-(v2442*(self.scalar_static_f64[259]*(if v2422{((-(v8755/v711))/v2438)}else{v7788})))))/self.scalar_static_f64[259])}else{(if v2155{((v573*(-(v2186*(self.scalar_static_f64[197]*v7788))))/self.scalar_static_f64[197])}else{v5695})})+(v795*(-v8755))))}else{v60})}));
        let v9009=(self.scalar_static_f64[0]*(if v1288{v60}else{(if v1252{(v1135*(v4423+(v795*(self.scalar_static_f64[0]-v4333))))}else{(if v1250{v60}else{(if v1138{((v680*((v4198+v4224)-v4250))+(v1143*v4044))}else{(if v958{v60}else{(if v921{(v783*(v3295+(v795*(self.scalar_static_f64[0]-v3226))))}else{(if v917{v60}else{(if v790{((v573*((v3121+v3141)-v3161))+(v803*v3002))}else{v60})})})})})})})}));
        let v9010=(self.scalar_static_f64[0]*(if v1288{v60}else{(if v1252{((v1285*v3890)+(v1135*(v4424+(v795*(-v4334)))))}else{(if v1250{v60}else{(if v1138{(((v1244*v2764)+(v680*((v4199+v4225)-v4251)))+((v1197*v3894)+(v1143*v4045)))}else{(if v958{v60}else{(if v921{((v955*v2868)+(v783*(v3296+(v795*(-v3227)))))}else{(if v917{v60}else{(if v790{(((v911*v2663)+(v573*((v3122+v3142)-v3162)))+((v862*v2874)+(v803*v3003)))}else{v60})})})})})})})}));
        let v9011=(self.scalar_static_f64[0]*(if v1288{v60}else{(if v1252{(v1135*(v4425+(v795*(self.scalar_static_f64[273]-v4335))))}else{(if v1250{v60}else{(if v1138{((v680*((v4200+v4226)-v4252))+(v1143*v4046))}else{(if v958{v60}else{(if v921{(v783*(v3297+(v795*(self.scalar_static_f64[273]-v3228))))}else{(if v917{v60}else{(if v790{((v573*((v3123+v3143)-v3163))+(v803*v3004))}else{v60})})})})})})})}));
        let v9012=(self.scalar_static_f64[0]*(if v1288{v60}else{(if v1252{(v1135*(v4426+(v795*(-v4336))))}else{(if v1250{v60}else{(if v1138{((v680*((v4201+v4227)-v4253))+(v1143*v4047))}else{v60})})})}));
        let v9022=(self.scalar_static_f64[0]*(((if v1132{v60}else{(if v1096{(v963*(v3859+(v795*(-v3769))))}else{(if v1092{v60}else{(if v970{((v680*((v3634+v3660)-v3686))+(v981*v3480))}else{v60})})})})+v5011)+(self.scalar_static_f64[239]*v6962)));
        let v9023=(self.scalar_static_f64[0]*(((if v1132{v60}else{(if v1096{((v1129*v3320)+(v963*(v3860+(v795*(-v3770)))))}else{(if v1092{v60}else{(if v970{(((v1086*v2764)+(v680*((v3635+v3661)-v3687)))+((v1037*v3326)+(v981*v3481)))}else{v60})})})})+v5012)+(self.scalar_static_f64[239]*v6966)));
        let v9024=(self.scalar_static_f64[0]*(((if v1132{v60}else{(if v1096{(v963*(v3861+(v795*(self.scalar_static_f64[273]-v3771))))}else{(if v1092{v60}else{(if v970{((v680*((v3636+v3662)-v3688))+(v981*v3482))}else{v60})})})})+v5013)+(self.scalar_static_f64[239]*v6970)));
        let v9025=(self.scalar_static_f64[0]*(((if v1132{v60}else{(if v1096{(v963*(v3862+(v795*(self.scalar_static_f64[0]-v3772))))}else{(if v1092{v60}else{(if v970{((v680*((v3637+v3663)-v3689))+(v981*v3483))}else{v60})})})})+v5014)+(self.scalar_static_f64[239]*v6974)));
        let v9026=(self.scalar_static_f64[0]*(self.scalar_static_f64[239]*v6977));
        let v9030=(self.scalar_static_f64[0]*(v5501+v8889));
        let v9031=(self.scalar_static_f64[0]*(v5502+v8890));
        let v9032=(self.scalar_static_f64[0]*(v5503+v8891));
        let v9033=(self.scalar_static_f64[0]*(v5504+v8892));
        let v9034=(self.scalar_static_f64[0]*(v5505+v8893));

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
            v94,
            v439,
            v454,
            v456,
            v458,
            v462,
            v465,
            v573,
            v579,
            v783,
            v790,
            v792,
            v795,
            v803,
            v810,
            v814,
            v817,
            v819,
            v820,
            v828,
            v841,
            v843,
            v844,
            v858,
            v866,
            v870,
            v921,
            v930,
            v933,
            v942,
            v961,
            v963,
            v970,
            v972,
            v981,
            v988,
            v994,
            v996,
            v997,
            v1005,
            v1016,
            v1018,
            v1019,
            v1033,
            v1041,
            v1045,
            v1096,
            v1104,
            v1107,
            v1116,
            v1135,
            v1138,
            v1139,
            v1143,
            v1148,
            v1154,
            v1156,
            v1157,
            v1165,
            v1176,
            v1178,
            v1179,
            v1193,
            v1201,
            v1205,
            v1252,
            v1260,
            v1263,
            v1272,
            v1292,
            v1293,
            v1297,
            v1302,
            v1308,
            v1310,
            v1311,
            v1319,
            v1330,
            v1332,
            v1333,
            v1347,
            v1355,
            v1359,
            v1406,
            v1414,
            v1417,
            v1426,
            v1517,
            v1519,
            v1533,
            v1536,
            v1545,
            v1565,
            v1568,
            v1582,
            v1585,
            v1594,
            v1698,
            v1775,
            v1938,
            v1941,
            v1968,
            v2043,
            v2044,
            v2045,
            v2049,
            v2054,
            v2060,
            v2062,
            v2063,
            v2071,
            v2082,
            v2084,
            v2085,
            v2099,
            v2107,
            v2111,
            v2155,
            v2163,
            v2166,
            v2175,
            v2466,
            v2467,
            v2474,
            v2475,
            v2484,
            v2486,
            v2495,
            v2496,
            v2497,
            v2498,
            v2500,
            v2502,
            v2521,
            v2549,
            v2553,
            v2554,
            v2558,
            v2562,
            v2663,
            v2672,
            v2868,
            v2874,
            v2884,
            v2896,
            v2897,
            v2898,
            v2953,
            v2954,
            v2955,
            v3017,
            v3018,
            v3019,
            v3032,
            v3033,
            v3034,
            v3206,
            v3207,
            v3208,
            v3215,
            v3216,
            v3217,
            v3257,
            v3258,
            v3259,
            v3318,
            v3320,
            v3326,
            v3336,
            v3348,
            v3349,
            v3350,
            v3351,
            v3419,
            v3420,
            v3421,
            v3422,
            v3499,
            v3500,
            v3501,
            v3502,
            v3518,
            v3519,
            v3520,
            v3521,
            v3743,
            v3744,
            v3745,
            v3746,
            v3755,
            v3756,
            v3757,
            v3758,
            v3809,
            v3810,
            v3811,
            v3812,
            v3890,
            v3894,
            v3900,
            v3912,
            v3913,
            v3914,
            v3915,
            v3983,
            v3984,
            v3985,
            v3986,
            v4063,
            v4064,
            v4065,
            v4066,
            v4082,
            v4083,
            v4084,
            v4085,
            v4307,
            v4308,
            v4309,
            v4310,
            v4319,
            v4320,
            v4321,
            v4322,
            v4373,
            v4374,
            v4375,
            v4376,
            v4456,
            v4462,
            v4474,
            v4475,
            v4476,
            v4477,
            v4545,
            v4546,
            v4547,
            v4548,
            v4625,
            v4626,
            v4627,
            v4628,
            v4644,
            v4645,
            v4646,
            v4647,
            v4869,
            v4870,
            v4871,
            v4872,
            v4881,
            v4882,
            v4883,
            v4884,
            v4935,
            v4936,
            v4937,
            v4938,
            v5287,
            v5288,
            v5289,
            v5290,
            v5324,
            v5325,
            v5326,
            v5327,
            v5328,
            v5339,
            v5340,
            v5341,
            v5342,
            v5343,
            v5405,
            v5406,
            v5407,
            v5408,
            v5409,
            v5506,
            v5511,
            v5512,
            v5513,
            v5514,
            v5548,
            v5549,
            v5550,
            v5551,
            v5552,
            v5563,
            v5564,
            v5565,
            v5566,
            v5567,
            v5629,
            v5630,
            v5631,
            v5632,
            v5633,
            v5950,
            v5951,
            v5952,
            v6024,
            v6962,
            v6966,
            v6970,
            v6974,
            v6977,
            v6978,
            v6979,
            v6980,
            v6981,
            v6982,
            v7143,
            v7144,
            v7145,
            v7146,
            v7147,
            v7302,
            v7308,
            v7320,
            v7321,
            v7322,
            v7323,
            v7391,
            v7392,
            v7393,
            v7394,
            v7471,
            v7472,
            v7473,
            v7474,
            v7490,
            v7491,
            v7492,
            v7493,
            v7703,
            v7704,
            v7705,
            v7706,
            v7707,
            v7718,
            v7719,
            v7720,
            v7721,
            v7722,
            v7784,
            v7785,
            v7786,
            v7787,
            v7788,
            v8889,
            v8890,
            v8891,
            v8892,
            v8893,
            v8924,
            v8925,
            v8926,
            v8927,
            v8928,
            v8963,
            v8964,
            v8965,
            v8966,
            v8967,
            v8974,
            v8975,
            v8976,
            v8977,
            v8978,
            v9003,
            v9004,
            v9005,
            v9006,
            v9007,
            v9008,
            v9009,
            v9010,
            v9011,
            v9012,
            v9022,
            v9023,
            v9024,
            v9025,
            v9026,
            v9030,
            v9031,
            v9032,
            v9033,
            v9034,
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
        let v540=(((self.scalar_static_f64[25]*common.v462)+(self.scalar_static_f64[8]*common.v465))).exp();
        let v542=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[60]*v540)}else{self.scalar_static_f64[367]});
        let v546=(((self.scalar_static_f64[62]*common.v462)+(self.scalar_static_f64[63]*common.v465))).exp();
        let v548=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[61]*v546)}else{self.scalar_static_f64[372]});
        let v581=(self.scalar_static_f64[11]*common.v465);
        let v583=(((self.scalar_static_f64[23]*common.v462)+v581)).exp();
        let v585=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[74]*v583)}else{self.scalar_static_f64[397]});
        let v643=((self.scalar_static_f64[96]*common.v458)).exp();
        let v647=((self.scalar_static_f64[98]*common.v458)).exp();
        let v651=(if self.scalar_static_bool[20]{self.scalar_static_f64[31]}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[31]*v643)}else{self.scalar_static_f64[449]})});
        let v652=(if self.scalar_static_bool[20]{self.scalar_static_f64[97]}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[97]*v647)}else{self.scalar_static_f64[450]})});
        let v654=((self.scalar_static_f64[100]*common.v462)).exp();
        let v656=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[99]*v654)}else{self.scalar_static_f64[453]});
        let v718=(self.scalar_static_f64[24]*common.v462);
        let v721=((v718+(self.scalar_static_f64[14]*common.v465))).exp();
        let v723=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[123]*v721)}else{self.scalar_static_f64[497]});
        let v725=((v581+v718)).exp();
        let v727=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[124]*v725)}else{self.scalar_static_f64[500]});
        let v757=((self.scalar_static_f64[138]*common.v462)).exp();
        let v759=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[137]*v757)}else{self.scalar_static_f64[525]});
        let v761=((self.scalar_static_f64[140]*common.v462)).exp();
        let v765=((self.scalar_static_f64[142]*common.v462)).exp();
        let v767=(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[141]*v765)}else{self.scalar_static_f64[531]});
        let v769=((self.scalar_static_f64[144]*common.v462)).exp();
        let v770=(self.scalar_static_f64[143]*v769);
        let v772=(common.v48+(self.scalar_static_f64[145]*common.v458));
        let v774=(if (self.scalar_static_f64[148]!=0.0){(v770*v772)}else{self.scalar_static_f64[537]});
        let v829=(if common.v828{common.v48}else{(if common.v817{(common.v819/common.v820)}else{common.v60})});
        let v859=(if common.v858{common.v48}else{(if common.v841{(common.v843/common.v844)}else{common.v60})});
        let v877=((common.v870*self.scalar_static_f64[198])).exp();
        let v878=(common.v783*v877);
        let v879=(v829*v878);
        let v882=(-common.v792);
        let v884=((common.v866*v882)).exp();
        let v885=(common.v810*v884);
        let v886=(common.v48-v859);
        let v889=(common.v48-v829);
        let v938=(if common.v921{(common.v933/common.v930)}else{common.v60});
        let v944=((self.scalar_static_f64[198]*common.v942)).exp();
        let v1006=(if common.v1005{common.v48}else{(if common.v994{(common.v996/common.v997)}else{v829})});
        let v1034=(if common.v1033{common.v48}else{(if common.v1016{(common.v1018/common.v1019)}else{v859})});
        let v1052=((common.v1045*self.scalar_static_f64[206])).exp();
        let v1053=(common.v963*v1052);
        let v1054=(v1006*v1053);
        let v1057=(-common.v972);
        let v1059=((common.v1041*v1057)).exp();
        let v1060=(common.v988*v1059);
        let v1061=(common.v48-v1034);
        let v1064=(common.v48-v1006);
        let v1112=(if common.v1096{(common.v1107/common.v1104)}else{v938});
        let v1118=((self.scalar_static_f64[206]*common.v1116)).exp();
        let v1166=(if common.v1165{common.v48}else{(if common.v1154{(common.v1156/common.v1157)}else{v1006})});
        let v1194=(if common.v1193{common.v48}else{(if common.v1176{(common.v1178/common.v1179)}else{v1034})});
        let v1210=((self.scalar_static_f64[206]*common.v1205)).exp();
        let v1211=(common.v1135*v1210);
        let v1212=(v1166*v1211);
        let v1215=(-common.v1139);
        let v1217=((common.v1201*v1215)).exp();
        let v1218=(common.v1148*v1217);
        let v1219=(common.v48-v1194);
        let v1222=(common.v48-v1166);
        let v1268=(if common.v1252{(common.v1263/common.v1260)}else{v1112});
        let v1274=((self.scalar_static_f64[206]*common.v1272)).exp();
        let v1320=(if common.v1319{common.v48}else{(if common.v1308{(common.v1310/common.v1311)}else{v1166})});
        let v1348=(if common.v1347{common.v48}else{(if common.v1330{(common.v1332/common.v1333)}else{v1194})});
        let v1364=((self.scalar_static_f64[198]*common.v1359)).exp();
        let v1365=(common.v961*v1364);
        let v1366=(v1320*v1365);
        let v1369=(-common.v1293);
        let v1371=((common.v1355*v1369)).exp();
        let v1372=(common.v1302*v1371);
        let v1373=(common.v48-v1348);
        let v1376=(common.v48-v1320);
        let v1422=(if common.v1406{(common.v1417/common.v1414)}else{v1268});
        let v1428=((self.scalar_static_f64[198]*common.v1426)).exp();
        let v1541=(if (common.v1519!=0.0){(common.v1536/common.v1533)}else{v1422});
        let v1548=((common.v1545*self.scalar_static_f64[210])).exp();
        let v1590=(if common.v1568{(common.v1585/common.v1582)}else{v1541});
        let v1597=((common.v1594*self.scalar_static_f64[212])).exp();
        let v1942=(common.v1941-common.v1938);
        let v1974=(common.v454*self.scalar_static_f64[241]);
        let v1976=(if (self.scalar_static_f64[240]!=0.0){(common.v10/v1974)}else{common.v60});
        let v1978=(if (v1976>common.v814){common.v48}else{common.v60});
        let v1979=((self.scalar_static_f64[240]!=0.0)&&(v1978!=0.0));
        let v1983=(if v1979{common.v814}else{v1976});
        let v1985=((self.scalar_static_f64[240]!=0.0)&&(!(v1978!=0.0)));
        let v1986=(if v1985{common.v48}else{(if v1979{(common.v48+(v1976-common.v814))}else{common.v60})});
        let v1987=scalar_limexp(v1983);
        let v1989=((v1986*v1987)-common.v48);
        let v1997=(common.v454*self.scalar_static_f64[243]);
        let v1999=(if (self.scalar_static_f64[242]!=0.0){(common.v10/v1997)}else{v1983});
        let v2001=(if (v1999>common.v814){common.v48}else{common.v60});
        let v2002=((self.scalar_static_f64[242]!=0.0)&&(v2001!=0.0));
        let v2006=(if v2002{common.v814}else{v1999});
        let v2008=((self.scalar_static_f64[242]!=0.0)&&(!(v2001!=0.0)));
        let v2009=(if v2008{common.v48}else{(if v2002{(common.v48+(v1999-common.v814))}else{v1986})});
        let v2010=scalar_limexp(v2006);
        let v2012=((v2009*v2010)-common.v48);
        let v2017=((if self.scalar_static_bool[43]{common.v60}else{(if (self.scalar_static_f64[240]!=0.0){(v542*v1989)}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if (self.scalar_static_f64[242]!=0.0){(v548*v2012)}else{common.v60})}));
        let v2021=(common.v454*self.scalar_static_f64[245]);
        let v2023=(if (self.scalar_static_f64[244]!=0.0){(common.v7/v2021)}else{v2006});
        let v2025=(if (v2023>common.v814){common.v48}else{common.v60});
        let v2026=((self.scalar_static_f64[244]!=0.0)&&(v2025!=0.0));
        let v2030=(if v2026{common.v814}else{v2023});
        let v2032=((self.scalar_static_f64[244]!=0.0)&&(!(v2025!=0.0)));
        let v2033=(if v2032{common.v48}else{(if v2026{(common.v48+(v2023-common.v814))}else{v2009})});
        let v2034=scalar_limexp(v2030);
        let v2036=((v2033*v2034)-common.v48);
        let v2040=(if self.scalar_static_bool[47]{common.v60}else{(if (self.scalar_static_f64[244]!=0.0){(v585*v2036)}else{common.v60})});
        let v2041=(v2017+v2040);
        let v2072=(if common.v2071{common.v48}else{(if common.v2060{(common.v2062/common.v2063)}else{v1320})});
        let v2100=(if common.v2099{common.v48}else{(if common.v2082{(common.v2084/common.v2085)}else{v1348})});
        let v2116=((self.scalar_static_f64[198]*common.v2111)).exp();
        let v2117=(common.v579*v2116);
        let v2118=(v2072*v2117);
        let v2121=(-common.v2045);
        let v2123=((common.v2107*v2121)).exp();
        let v2124=(common.v2054*v2123);
        let v2125=(common.v48-v2100);
        let v2128=(common.v48-v2072);
        let v2152=(!(common.v2043!=0.0));
        let v2153=((self.scalar_static_f64[192]!=0.0)&&v2152);
        let v2171=(if common.v2155{(common.v2166/common.v2163)}else{v1590});
        let v2177=((self.scalar_static_f64[198]*common.v2175)).exp();
        let v2182=((if common.v2155{(v2171*v2177)}else{(if common.v1568{(v1590*v1597)}else{(if (common.v1519!=0.0){(v1541*v1548)}else{(if common.v1406{(v1422*v1428)}else{(if common.v1252{(v1268*v1274)}else{(if common.v1096{(v1112*v1118)}else{(if common.v921{(v938*v944)}else{common.v60})})})})})})})+(common.v795*(common.v48-v2171)));
        let v2191=(self.scalar_static_bool[24]&&v2152);
        let v2192=(if v2191{common.v60}else{(if common.v2155{(common.v579*v2182)}else{(if v2153{common.v60}else{(if common.v2044{((if common.v2044{(common.v2049*v2128)}else{(if common.v1292{(common.v1297*v1376)}else{(if common.v1138{(common.v1143*v1222)}else{(if common.v970{(common.v981*v1064)}else{(if common.v790{(common.v803*v889)}else{common.v60})})})})})+((if common.v2044{(v2100*v2118)}else{(if common.v1292{(v1348*v1366)}else{(if common.v1138{(v1194*v1212)}else{(if common.v970{(v1034*v1054)}else{(if common.v790{(v859*v879)}else{common.v60})})})})})+(if common.v2044{(v2124*v2125)}else{(if common.v1292{(v1372*v1373)}else{(if common.v1138{(v1218*v1219)}else{(if common.v970{(v1060*v1061)}else{(if common.v790{(v885*v886)}else{common.v60})})})})})))}else{common.v60})})})});
        let v2193=(common.v573-common.v7);
        let v2194=(if (self.scalar_static_f64[95]!=0.0){v2193}else{common.v60});
        let v2196=(if (v2194>common.v60){common.v48}else{common.v60});
        let v2197=((self.scalar_static_f64[95]!=0.0)&&(v2196!=0.0));
        let v2199=(if v2197{(v652/v2192)}else{common.v60});
        let v2201=(if v2197{(v652/common.v579)}else{common.v60});
        let v2203=(if (v2194>v2201){common.v48}else{common.v60});
        let v2204=(v2197&&(v2203!=0.0));
        let v2205=(-v2199);
        let v2207=((v2205/v2201)).exp();
        let v2209=(if v2204{(v651*v2207)}else{common.v60});
        let v2211=(common.v48+(v2199/v2201));
        let v2212=(v2194-v2201);
        let v2214=(v2201+(v2211*v2212));
        let v2218=(v2197&&(!(v2203!=0.0)));
        let v2219=(v651*v2194);
        let v2221=((v2205/v2194)).exp();
        let v2223=(if v2218{(v2219*v2221)}else{(if v2204{(v2209*v2214)}else{common.v60})});
        let v2227=((self.scalar_static_f64[95]!=0.0)&&(!(v2196!=0.0)));
        let v2228=(if v2227{common.v60}else{(if v2197{(common.v1941*v2223)}else{common.v60})});
        let v2230=(if (v656>common.v60){common.v48}else{common.v60});
        let v2241=(if (v2230!=0.0){((((common.v48+(common.v1565/self.scalar_static_f64[246]))+(common.v1517/self.scalar_static_f64[247]))+(common.v1941/common.v1698))+(common.v1938/self.scalar_static_f64[224]))}else{common.v60});
        let v2244=((common.v1775+(v2241*v2241))).sqrt();
        let v2247=(if (v2230!=0.0){(common.v32*(v2241+v2244))}else{common.v60});
        let v2249=(if (v2230!=0.0){(v656/v2247)}else{common.v60});
        let v2252=((v2230!=0.0)&&((if (v2041>common.v60){common.v48}else{common.v60})!=0.0));
        let v2254=(v2249*self.scalar_static_f64[248]);
        let v2255=(v2041*v2254);
        let v2257=(if v2252{(common.v456*v2255)}else{common.v60});
        let v2260=(if (v2257<1e-6){common.v48}else{common.v60});
        let v2261=(v2252&&(v2260!=0.0));
        let v2263=(common.v48-(common.v32*v2257));
        let v2265=(if v2261{(v2249*v2263)}else{v2249});
        let v2267=(v2252&&(!(v2260!=0.0)));
        let v2268=(common.v48+v2257);
        let v2269=(v2268).ln();
        let v2270=(v2265*v2269);
        let v2273=(!(v2230!=0.0));
        let v2275=((if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[139]*v761)}else{self.scalar_static_f64[528]})+(if v2273{common.v60}else{(if v2267{(v2270/v2257)}else{v2265})}));
        let v2280=(if (self.scalar_static_f64[249]!=0.0){(common.v454*self.scalar_static_f64[250])}else{common.v60});
        let v2281=(common.v4/v2280);
        let v2284=(common.v14/v2280);
        let v2287=((if (self.scalar_static_f64[249]!=0.0){scalar_limexp(v2281)}else{common.v60})-(if (self.scalar_static_f64[249]!=0.0){scalar_limexp(v2284)}else{common.v60}));
        let v2295=(common.v454*self.scalar_static_f64[252]);
        let v2297=(if (self.scalar_static_f64[251]!=0.0){(common.v14/v2295)}else{v2030});
        let v2299=(if (v2297>common.v814){common.v48}else{common.v60});
        let v2300=((self.scalar_static_f64[251]!=0.0)&&(v2299!=0.0));
        let v2304=(if v2300{common.v814}else{v2297});
        let v2306=((self.scalar_static_f64[251]!=0.0)&&(!(v2299!=0.0)));
        let v2307=(if v2306{common.v48}else{(if v2300{(common.v48+(v2297-common.v814))}else{v2033})});
        let v2308=scalar_limexp(v2304);
        let v2310=((v2307*v2308)-common.v48);
        let v2522=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2521);
        let v2527=-1.0;
        let v2674=(self.scalar_static_f64[11]*common.v2562);
        let v2740=(if self.scalar_static_bool[20]{common.v60}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[31]*(v643*(self.scalar_static_f64[96]*common.v2554)))}else{common.v60})});
        let v2741=(if self.scalar_static_bool[20]{common.v60}else{(if self.scalar_static_bool[19]{(self.scalar_static_f64[97]*(v647*(self.scalar_static_f64[98]*common.v2554)))}else{common.v60})});
        let v2803=(self.scalar_static_f64[24]*common.v2558);
        let v2902=(common.v820*common.v820);
        let v2929=(if common.v828{common.v60}else{(if common.v817{(((common.v820*common.v2896)-(common.v819*common.v2896))/v2902)}else{common.v60})});
        let v2930=(if common.v828{common.v60}else{(if common.v817{(((common.v820*common.v2897)-(common.v819*common.v2897))/v2902)}else{common.v60})});
        let v2931=(if common.v828{common.v60}else{(if common.v817{(((common.v820*common.v2898)-(common.v819*common.v2898))/v2902)}else{common.v60})});
        let v2959=(common.v844*common.v844);
        let v2993=(if common.v858{common.v60}else{(if common.v841{(((common.v844*common.v2953)-(common.v843*common.v2953))/v2959)}else{common.v60})});
        let v2994=(if common.v858{common.v60}else{(if common.v841{(((common.v844*common.v2954)-(common.v843*common.v2954))/v2959)}else{common.v60})});
        let v2995=(if common.v858{common.v60}else{(if common.v841{(((common.v844*common.v2955)-(common.v843*common.v2955))/v2959)}else{common.v60})});
        let v3232=(common.v930*common.v930);
        let v3242=(if common.v921{(((common.v930*common.v3215)-(common.v933*common.v3206))/v3232)}else{common.v60});
        let v3243=(if common.v921{(((common.v930*common.v3216)-(common.v933*common.v3207))/v3232)}else{common.v60});
        let v3244=(if common.v921{(((common.v930*common.v3217)-(common.v933*common.v3208))/v3232)}else{common.v60});
        let v3355=(common.v997*common.v997);
        let v3391=(if common.v1005{common.v60}else{(if common.v994{(((common.v997*common.v3348)-(common.v996*common.v3348))/v3355)}else{v2929})});
        let v3392=(if common.v1005{common.v60}else{(if common.v994{(((common.v997*common.v3349)-(common.v996*common.v3349))/v3355)}else{v2930})});
        let v3393=(if common.v1005{common.v60}else{(if common.v994{(((common.v997*common.v3350)-(common.v996*common.v3350))/v3355)}else{v2931})});
        let v3394=(if common.v1005{common.v60}else{(if common.v994{(((common.v997*common.v3351)-(common.v996*common.v3351))/v3355)}else{common.v60})});
        let v3426=(common.v1019*common.v1019);
        let v3468=(if common.v1033{common.v60}else{(if common.v1016{(((common.v1019*common.v3419)-(common.v1018*common.v3419))/v3426)}else{v2993})});
        let v3469=(if common.v1033{common.v60}else{(if common.v1016{(((common.v1019*common.v3420)-(common.v1018*common.v3420))/v3426)}else{v2994})});
        let v3470=(if common.v1033{common.v60}else{(if common.v1016{(((common.v1019*common.v3421)-(common.v1018*common.v3421))/v3426)}else{v2995})});
        let v3471=(if common.v1033{common.v60}else{(if common.v1016{(((common.v1019*common.v3422)-(common.v1018*common.v3422))/v3426)}else{common.v60})});
        let v3776=(common.v1104*common.v1104);
        let v3790=(if common.v1096{(((common.v1104*common.v3755)-(common.v1107*common.v3743))/v3776)}else{v3242});
        let v3791=(if common.v1096{(((common.v1104*common.v3756)-(common.v1107*common.v3744))/v3776)}else{v3243});
        let v3792=(if common.v1096{(((common.v1104*common.v3757)-(common.v1107*common.v3745))/v3776)}else{v3244});
        let v3793=(if common.v1096{(((common.v1104*common.v3758)-(common.v1107*common.v3746))/v3776)}else{common.v60});
        let v3919=(common.v1157*common.v1157);
        let v3955=(if common.v1165{common.v60}else{(if common.v1154{(((common.v1157*common.v3912)-(common.v1156*common.v3912))/v3919)}else{v3391})});
        let v3956=(if common.v1165{common.v60}else{(if common.v1154{(((common.v1157*common.v3913)-(common.v1156*common.v3913))/v3919)}else{v3392})});
        let v3957=(if common.v1165{common.v60}else{(if common.v1154{(((common.v1157*common.v3914)-(common.v1156*common.v3914))/v3919)}else{v3393})});
        let v3958=(if common.v1165{common.v60}else{(if common.v1154{(((common.v1157*common.v3915)-(common.v1156*common.v3915))/v3919)}else{v3394})});
        let v3990=(common.v1179*common.v1179);
        let v4032=(if common.v1193{common.v60}else{(if common.v1176{(((common.v1179*common.v3983)-(common.v1178*common.v3983))/v3990)}else{v3468})});
        let v4033=(if common.v1193{common.v60}else{(if common.v1176{(((common.v1179*common.v3984)-(common.v1178*common.v3984))/v3990)}else{v3469})});
        let v4034=(if common.v1193{common.v60}else{(if common.v1176{(((common.v1179*common.v3985)-(common.v1178*common.v3985))/v3990)}else{v3470})});
        let v4035=(if common.v1193{common.v60}else{(if common.v1176{(((common.v1179*common.v3986)-(common.v1178*common.v3986))/v3990)}else{v3471})});
        let v4340=(common.v1260*common.v1260);
        let v4354=(if common.v1252{(((common.v1260*common.v4319)-(common.v1263*common.v4307))/v4340)}else{v3790});
        let v4355=(if common.v1252{(((common.v1260*common.v4320)-(common.v1263*common.v4308))/v4340)}else{v3791});
        let v4356=(if common.v1252{(((common.v1260*common.v4321)-(common.v1263*common.v4309))/v4340)}else{v3792});
        let v4357=(if common.v1252{(((common.v1260*common.v4322)-(common.v1263*common.v4310))/v4340)}else{v3793});
        let v4481=(common.v1311*common.v1311);
        let v4517=(if common.v1319{common.v60}else{(if common.v1308{(((common.v1311*common.v4474)-(common.v1310*common.v4474))/v4481)}else{v3955})});
        let v4518=(if common.v1319{common.v60}else{(if common.v1308{(((common.v1311*common.v4475)-(common.v1310*common.v4475))/v4481)}else{v3956})});
        let v4519=(if common.v1319{common.v60}else{(if common.v1308{(((common.v1311*common.v4476)-(common.v1310*common.v4476))/v4481)}else{v3957})});
        let v4520=(if common.v1319{common.v60}else{(if common.v1308{(((common.v1311*common.v4477)-(common.v1310*common.v4477))/v4481)}else{v3958})});
        let v4552=(common.v1333*common.v1333);
        let v4594=(if common.v1347{common.v60}else{(if common.v1330{(((common.v1333*common.v4545)-(common.v1332*common.v4545))/v4552)}else{v4032})});
        let v4595=(if common.v1347{common.v60}else{(if common.v1330{(((common.v1333*common.v4546)-(common.v1332*common.v4546))/v4552)}else{v4033})});
        let v4596=(if common.v1347{common.v60}else{(if common.v1330{(((common.v1333*common.v4547)-(common.v1332*common.v4547))/v4552)}else{v4034})});
        let v4597=(if common.v1347{common.v60}else{(if common.v1330{(((common.v1333*common.v4548)-(common.v1332*common.v4548))/v4552)}else{v4035})});
        let v4902=(common.v1414*common.v1414);
        let v4916=(if common.v1406{(((common.v1414*common.v4881)-(common.v1417*common.v4869))/v4902)}else{v4354});
        let v4917=(if common.v1406{(((common.v1414*common.v4882)-(common.v1417*common.v4870))/v4902)}else{v4355});
        let v4918=(if common.v1406{(((common.v1414*common.v4883)-(common.v1417*common.v4871))/v4902)}else{v4356});
        let v4919=(if common.v1406{(((common.v1414*common.v4884)-(common.v1417*common.v4872))/v4902)}else{v4357});
        let v5364=(common.v1533*common.v1533);
        let v5382=(if (common.v1519!=0.0){(((common.v1533*common.v5339)-(common.v1536*common.v5324))/v5364)}else{v4916});
        let v5383=(if (common.v1519!=0.0){(((common.v1533*common.v5340)-(common.v1536*common.v5325))/v5364)}else{v4917});
        let v5384=(if (common.v1519!=0.0){(((common.v1533*common.v5341)-(common.v1536*common.v5326))/v5364)}else{v4918});
        let v5385=(if (common.v1519!=0.0){(((common.v1533*common.v5342)-(common.v1536*common.v5327))/v5364)}else{v4919});
        let v5386=(if (common.v1519!=0.0){(((common.v1533*common.v5343)-(common.v1536*common.v5328))/v5364)}else{common.v60});
        let v5588=(common.v1582*common.v1582);
        let v5606=(if common.v1568{(((common.v1582*common.v5563)-(common.v1585*common.v5548))/v5588)}else{v5382});
        let v5607=(if common.v1568{(((common.v1582*common.v5564)-(common.v1585*common.v5549))/v5588)}else{v5383});
        let v5608=(if common.v1568{(((common.v1582*common.v5565)-(common.v1585*common.v5550))/v5588)}else{v5384});
        let v5609=(if common.v1568{(((common.v1582*common.v5566)-(common.v1585*common.v5551))/v5588)}else{v5385});
        let v5610=(if common.v1568{(((common.v1582*common.v5567)-(common.v1585*common.v5552))/v5588)}else{v5386});
        let v7160=(if (self.scalar_static_f64[240]!=0.0){((-(common.v10*(self.scalar_static_f64[241]*common.v2549)))/(v1974*v1974))}else{common.v60});
        let v7161=(if (self.scalar_static_f64[240]!=0.0){(self.scalar_static_f64[0]/v1974)}else{common.v60});
        let v7162=(if (self.scalar_static_f64[240]!=0.0){(self.scalar_static_f64[273]/v1974)}else{common.v60});
        let v7166=(if v1979{common.v60}else{v7160});
        let v7167=(if v1979{common.v60}else{v7161});
        let v7168=(if v1979{common.v60}else{v7162});
        let v7169=(if v1985{common.v60}else{(if v1979{v7160}else{common.v60})});
        let v7170=(if v1985{common.v60}else{(if v1979{v7161}else{common.v60})});
        let v7171=(if v1985{common.v60}else{(if v1979{v7162}else{common.v60})});
        let v7172=scalar_limexp_derivative(v1983);
        let v7203=(if (self.scalar_static_f64[242]!=0.0){((-(common.v10*(self.scalar_static_f64[243]*common.v2549)))/(v1997*v1997))}else{v7166});
        let v7204=(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[0]/v1997)}else{v7167});
        let v7205=(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[273]/v1997)}else{v7168});
        let v7209=(if v2002{common.v60}else{v7203});
        let v7210=(if v2002{common.v60}else{v7204});
        let v7211=(if v2002{common.v60}else{v7205});
        let v7212=(if v2008{common.v60}else{(if v2002{v7203}else{v7169})});
        let v7213=(if v2008{common.v60}else{(if v2002{v7204}else{v7170})});
        let v7214=(if v2008{common.v60}else{(if v2002{v7205}else{v7171})});
        let v7215=scalar_limexp_derivative(v2006);
        let v7239=((if self.scalar_static_bool[43]{common.v60}else{(if (self.scalar_static_f64[240]!=0.0){((v1989*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[60]*(v540*((self.scalar_static_f64[25]*common.v2558)+(self.scalar_static_f64[8]*common.v2562))))}else{common.v60}))+(v542*((v1987*v7169)+(v1986*(v7166*v7172)))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if (self.scalar_static_f64[242]!=0.0){((v2012*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[61]*(v546*((self.scalar_static_f64[62]*common.v2558)+(self.scalar_static_f64[63]*common.v2562))))}else{common.v60}))+(v548*((v2010*v7212)+(v2009*(v7209*v7215)))))}else{common.v60})}));
        let v7240=((if self.scalar_static_bool[43]{common.v60}else{(if (self.scalar_static_f64[240]!=0.0){(v542*((v1987*v7170)+(v1986*(v7167*v7172))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if (self.scalar_static_f64[242]!=0.0){(v548*((v2010*v7213)+(v2009*(v7210*v7215))))}else{common.v60})}));
        let v7241=((if self.scalar_static_bool[43]{common.v60}else{(if (self.scalar_static_f64[240]!=0.0){(v542*((v1987*v7171)+(v1986*(v7168*v7172))))}else{common.v60})})+(if self.scalar_static_bool[45]{common.v60}else{(if (self.scalar_static_f64[242]!=0.0){(v548*((v2010*v7214)+(v2009*(v7211*v7215))))}else{common.v60})}));
        let v7249=(if (self.scalar_static_f64[244]!=0.0){((-(common.v7*(self.scalar_static_f64[245]*common.v2549)))/(v2021*v2021))}else{v7209});
        let v7250=(if (self.scalar_static_f64[244]!=0.0){(self.scalar_static_f64[273]/v2021)}else{common.v60});
        let v7251=(if (self.scalar_static_f64[244]!=0.0){(self.scalar_static_f64[0]/v2021)}else{v7210});
        let v7252=(if (self.scalar_static_f64[244]!=0.0){common.v60}else{v7211});
        let v7257=(if v2026{common.v60}else{v7249});
        let v7258=(if v2026{common.v60}else{v7250});
        let v7259=(if v2026{common.v60}else{v7251});
        let v7260=(if v2026{common.v60}else{v7252});
        let v7261=(if v2032{common.v60}else{(if v2026{v7249}else{v7212})});
        let v7262=(if v2032{common.v60}else{(if v2026{v7250}else{common.v60})});
        let v7263=(if v2032{common.v60}else{(if v2026{v7251}else{v7213})});
        let v7264=(if v2032{common.v60}else{(if v2026{v7252}else{v7214})});
        let v7265=scalar_limexp_derivative(v2030);
        let v7292=(if self.scalar_static_bool[47]{common.v60}else{(if (self.scalar_static_f64[244]!=0.0){((v2036*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[74]*(v583*((self.scalar_static_f64[23]*common.v2558)+v2674)))}else{common.v60}))+(v585*((v2034*v7261)+(v2033*(v7257*v7265)))))}else{common.v60})});
        let v7293=(if self.scalar_static_bool[47]{common.v60}else{(if (self.scalar_static_f64[244]!=0.0){(v585*((v2034*v7262)+(v2033*(v7258*v7265))))}else{common.v60})});
        let v7294=(if self.scalar_static_bool[47]{common.v60}else{(if (self.scalar_static_f64[244]!=0.0){(v585*((v2034*v7263)+(v2033*(v7259*v7265))))}else{common.v60})});
        let v7295=(if self.scalar_static_bool[47]{common.v60}else{(if (self.scalar_static_f64[244]!=0.0){(v585*((v2034*v7264)+(v2033*(v7260*v7265))))}else{common.v60})});
        let v7327=(common.v2063*common.v2063);
        let v7363=(if common.v2071{common.v60}else{(if common.v2060{(((common.v2063*common.v7320)-(common.v2062*common.v7320))/v7327)}else{v4517})});
        let v7364=(if common.v2071{common.v60}else{(if common.v2060{(((common.v2063*common.v7321)-(common.v2062*common.v7321))/v7327)}else{v4518})});
        let v7365=(if common.v2071{common.v60}else{(if common.v2060{(((common.v2063*common.v7322)-(common.v2062*common.v7322))/v7327)}else{v4519})});
        let v7366=(if common.v2071{common.v60}else{(if common.v2060{(((common.v2063*common.v7323)-(common.v2062*common.v7323))/v7327)}else{v4520})});
        let v7398=(common.v2085*common.v2085);
        let v7440=(if common.v2099{common.v60}else{(if common.v2082{(((common.v2085*common.v7391)-(common.v2084*common.v7391))/v7398)}else{v4594})});
        let v7441=(if common.v2099{common.v60}else{(if common.v2082{(((common.v2085*common.v7392)-(common.v2084*common.v7392))/v7398)}else{v4595})});
        let v7442=(if common.v2099{common.v60}else{(if common.v2082{(((common.v2085*common.v7393)-(common.v2084*common.v7393))/v7398)}else{v4596})});
        let v7443=(if common.v2099{common.v60}else{(if common.v2082{(((common.v2085*common.v7394)-(common.v2084*common.v7394))/v7398)}else{v4597})});
        let v7584=((if common.v2044{((v2118*v7440)+(v2100*((v2117*v7363)+(v2072*(common.v579*(v2116*(self.scalar_static_f64[198]*common.v7490)))))))}else{(if common.v1292{((v1366*v4594)+(v1348*((v1365*v4517)+(v1320*(common.v961*(v1364*(self.scalar_static_f64[198]*common.v4644)))))))}else{(if common.v1138{((v1212*v4032)+(v1194*((v1211*v3955)+(v1166*(common.v1135*(v1210*(self.scalar_static_f64[206]*common.v4082)))))))}else{(if common.v970{((v1054*v3468)+(v1034*((v1053*v3391)+(v1006*(common.v963*(v1052*(self.scalar_static_f64[206]*common.v3518)))))))}else{(if common.v790{((v879*v2993)+(v859*((v878*v2929)+(v829*(common.v783*(v877*(self.scalar_static_f64[198]*common.v3032)))))))}else{common.v60})})})})})+(if common.v2044{((v2125*(common.v2054*(v2123*(v2121*common.v7471))))+(v2124*(-v7440)))}else{(if common.v1292{((v1373*(common.v1302*(v1371*(v1369*common.v4625))))+(v1372*(-v4594)))}else{(if common.v1138{((v1219*(common.v1148*(v1217*(v1215*common.v4063))))+(v1218*(-v4032)))}else{(if common.v970{((v1061*(common.v988*(v1059*(v1057*common.v3499))))+(v1060*(-v3468)))}else{(if common.v790{((v886*(common.v810*(v884*(v882*common.v3017))))+(v885*(-v2993)))}else{common.v60})})})})}));
        let v7585=((if common.v2044{((v2118*v7441)+(v2100*((v2117*v7364)+(v2072*((v2116*common.v2672)+(common.v579*(v2116*(self.scalar_static_f64[198]*common.v7491))))))))}else{(if common.v1292{((v1366*v4595)+(v1348*((v1365*v4518)+(v1320*((v1364*common.v3318)+(common.v961*(v1364*(self.scalar_static_f64[198]*common.v4645))))))))}else{(if common.v1138{((v1212*v4033)+(v1194*((v1211*v3956)+(v1166*((v1210*common.v3890)+(common.v1135*(v1210*(self.scalar_static_f64[206]*common.v4083))))))))}else{(if common.v970{((v1054*v3469)+(v1034*((v1053*v3392)+(v1006*((v1052*common.v3320)+(common.v963*(v1052*(self.scalar_static_f64[206]*common.v3519))))))))}else{(if common.v790{((v879*v2994)+(v859*((v878*v2930)+(v829*((v877*common.v2868)+(common.v783*(v877*(self.scalar_static_f64[198]*common.v3033))))))))}else{common.v60})})})})})+(if common.v2044{((v2125*((v2123*common.v7308)+(common.v2054*(v2123*(v2121*common.v7472)))))+(v2124*(-v7441)))}else{(if common.v1292{((v1373*((v1371*common.v4462)+(common.v1302*(v1371*(v1369*common.v4626)))))+(v1372*(-v4595)))}else{(if common.v1138{((v1219*((v1217*common.v3900)+(common.v1148*(v1217*(v1215*common.v4064)))))+(v1218*(-v4033)))}else{(if common.v970{((v1061*((v1059*common.v3336)+(common.v988*(v1059*(v1057*common.v3500)))))+(v1060*(-v3469)))}else{(if common.v790{((v886*((v884*common.v2884)+(common.v810*(v884*(v882*common.v3018)))))+(v885*(-v2994)))}else{common.v60})})})})}));
        let v7586=((if common.v2044{((v2118*v7442)+(v2100*((v2117*v7365)+(v2072*(common.v579*(v2116*(self.scalar_static_f64[198]*common.v7492)))))))}else{(if common.v1292{((v1366*v4596)+(v1348*((v1365*v4519)+(v1320*(common.v961*(v1364*(self.scalar_static_f64[198]*common.v4646)))))))}else{(if common.v1138{((v1212*v4034)+(v1194*((v1211*v3957)+(v1166*(common.v1135*(v1210*(self.scalar_static_f64[206]*common.v4084)))))))}else{(if common.v970{((v1054*v3470)+(v1034*((v1053*v3393)+(v1006*(common.v963*(v1052*(self.scalar_static_f64[206]*common.v3520)))))))}else{(if common.v790{((v879*v2995)+(v859*((v878*v2931)+(v829*(common.v783*(v877*(self.scalar_static_f64[198]*common.v3034)))))))}else{common.v60})})})})})+(if common.v2044{((v2125*(common.v2054*(v2123*(v2121*common.v7473))))+(v2124*(-v7442)))}else{(if common.v1292{((v1373*(common.v1302*(v1371*(v1369*common.v4627))))+(v1372*(-v4596)))}else{(if common.v1138{((v1219*(common.v1148*(v1217*(v1215*common.v4065))))+(v1218*(-v4034)))}else{(if common.v970{((v1061*(common.v988*(v1059*(v1057*common.v3501))))+(v1060*(-v3470)))}else{(if common.v790{((v886*(common.v810*(v884*(v882*common.v3019))))+(v885*(-v2995)))}else{common.v60})})})})}));
        let v7587=((if common.v2044{((v2118*v7443)+(v2100*((v2117*v7366)+(v2072*(common.v579*(v2116*(self.scalar_static_f64[198]*common.v7493)))))))}else{(if common.v1292{((v1366*v4597)+(v1348*((v1365*v4520)+(v1320*(common.v961*(v1364*(self.scalar_static_f64[198]*common.v4647)))))))}else{(if common.v1138{((v1212*v4035)+(v1194*((v1211*v3958)+(v1166*(common.v1135*(v1210*(self.scalar_static_f64[206]*common.v4085)))))))}else{(if common.v970{((v1054*v3471)+(v1034*((v1053*v3394)+(v1006*(common.v963*(v1052*(self.scalar_static_f64[206]*common.v3521)))))))}else{common.v60})})})})+(if common.v2044{((v2125*(common.v2054*(v2123*(v2121*common.v7474))))+(v2124*(-v7443)))}else{(if common.v1292{((v1373*(common.v1302*(v1371*(v1369*common.v4628))))+(v1372*(-v4597)))}else{(if common.v1138{((v1219*(common.v1148*(v1217*(v1215*common.v4066))))+(v1218*(-v4035)))}else{(if common.v970{((v1061*(common.v988*(v1059*(v1057*common.v3502))))+(v1060*(-v3471)))}else{common.v60})})})}));
        let v7743=(common.v2163*common.v2163);
        let v7761=(if common.v2155{(((common.v2163*common.v7718)-(common.v2166*common.v7703))/v7743)}else{v5606});
        let v7762=(if common.v2155{(((common.v2163*common.v7719)-(common.v2166*common.v7704))/v7743)}else{v5607});
        let v7763=(if common.v2155{(((common.v2163*common.v7720)-(common.v2166*common.v7705))/v7743)}else{v5608});
        let v7764=(if common.v2155{(((common.v2163*common.v7721)-(common.v2166*common.v7706))/v7743)}else{v5609});
        let v7765=(if common.v2155{(((common.v2163*common.v7722)-(common.v2166*common.v7707))/v7743)}else{v5610});
        let v7841=(if common.v2155{(common.v579*((if common.v2155{((v2177*v7761)+(v2171*(v2177*(self.scalar_static_f64[198]*common.v7784))))}else{(if common.v1568{((v1597*v5606)+(v1590*(v1597*(self.scalar_static_f64[212]*common.v5629))))}else{(if (common.v1519!=0.0){((v1548*v5382)+(v1541*(v1548*(self.scalar_static_f64[210]*common.v5405))))}else{(if common.v1406{((v1428*v4916)+(v1422*(v1428*(self.scalar_static_f64[198]*common.v4935))))}else{(if common.v1252{((v1274*v4354)+(v1268*(v1274*(self.scalar_static_f64[206]*common.v4373))))}else{(if common.v1096{((v1118*v3790)+(v1112*(v1118*(self.scalar_static_f64[206]*common.v3809))))}else{(if common.v921{((v944*v3242)+(v938*(v944*(self.scalar_static_f64[198]*common.v3257))))}else{common.v60})})})})})})})+(common.v795*(-v7761))))}else{(if v2153{common.v60}else{(if common.v2044{((if common.v2044{(common.v2049*(-v7363))}else{(if common.v1292{(common.v1297*(-v4517))}else{(if common.v1138{(common.v1143*(-v3955))}else{(if common.v970{(common.v981*(-v3391))}else{(if common.v790{(common.v803*(-v2929))}else{common.v60})})})})})+v7584)}else{common.v60})})});
        let v7842=(if common.v2155{((v2182*common.v2672)+(common.v579*((if common.v2155{((v2177*v7762)+(v2171*(v2177*(self.scalar_static_f64[198]*common.v7785))))}else{(if common.v1568{((v1597*v5607)+(v1590*(v1597*(self.scalar_static_f64[212]*common.v5630))))}else{(if (common.v1519!=0.0){((v1548*v5383)+(v1541*(v1548*(self.scalar_static_f64[210]*common.v5406))))}else{(if common.v1406{((v1428*v4917)+(v1422*(v1428*(self.scalar_static_f64[198]*common.v4936))))}else{(if common.v1252{((v1274*v4355)+(v1268*(v1274*(self.scalar_static_f64[206]*common.v4374))))}else{(if common.v1096{((v1118*v3791)+(v1112*(v1118*(self.scalar_static_f64[206]*common.v3810))))}else{(if common.v921{((v944*v3243)+(v938*(v944*(self.scalar_static_f64[198]*common.v3258))))}else{common.v60})})})})})})})+(common.v795*(-v7762)))))}else{(if v2153{common.v60}else{(if common.v2044{((if common.v2044{((v2128*common.v7302)+(common.v2049*(-v7364)))}else{(if common.v1292{((v1376*common.v4456)+(common.v1297*(-v4518)))}else{(if common.v1138{((v1222*common.v3894)+(common.v1143*(-v3956)))}else{(if common.v970{((v1064*common.v3326)+(common.v981*(-v3392)))}else{(if common.v790{((v889*common.v2874)+(common.v803*(-v2930)))}else{common.v60})})})})})+v7585)}else{common.v60})})});
        let v7843=(if common.v2155{(common.v579*((if common.v2155{((v2177*v7763)+(v2171*(v2177*(self.scalar_static_f64[198]*common.v7786))))}else{(if common.v1568{((v1597*v5608)+(v1590*(v1597*(self.scalar_static_f64[212]*common.v5631))))}else{(if (common.v1519!=0.0){((v1548*v5384)+(v1541*(v1548*(self.scalar_static_f64[210]*common.v5407))))}else{(if common.v1406{((v1428*v4918)+(v1422*(v1428*(self.scalar_static_f64[198]*common.v4937))))}else{(if common.v1252{((v1274*v4356)+(v1268*(v1274*(self.scalar_static_f64[206]*common.v4375))))}else{(if common.v1096{((v1118*v3792)+(v1112*(v1118*(self.scalar_static_f64[206]*common.v3811))))}else{(if common.v921{((v944*v3244)+(v938*(v944*(self.scalar_static_f64[198]*common.v3259))))}else{common.v60})})})})})})})+(common.v795*(-v7763))))}else{(if v2153{common.v60}else{(if common.v2044{((if common.v2044{(common.v2049*(-v7365))}else{(if common.v1292{(common.v1297*(-v4519))}else{(if common.v1138{(common.v1143*(-v3957))}else{(if common.v970{(common.v981*(-v3393))}else{(if common.v790{(common.v803*(-v2931))}else{common.v60})})})})})+v7586)}else{common.v60})})});
        let v7883=(if (self.scalar_static_f64[95]!=0.0){common.v2663}else{common.v60});
        let v7888=(v2192*v2192);
        let v7899=((-(v652*(if v2191{common.v60}else{(if common.v2155{(common.v579*((if common.v2155{((v2177*v7764)+(v2171*(v2177*(self.scalar_static_f64[198]*common.v7787))))}else{(if common.v1568{((v1597*v5609)+(v1590*(v1597*(self.scalar_static_f64[212]*common.v5632))))}else{(if (common.v1519!=0.0){((v1548*v5385)+(v1541*(v1548*(self.scalar_static_f64[210]*common.v5408))))}else{(if common.v1406{((v1428*v4919)+(v1422*(v1428*(self.scalar_static_f64[198]*common.v4938))))}else{(if common.v1252{((v1274*v4357)+(v1268*(v1274*(self.scalar_static_f64[206]*common.v4376))))}else{(if common.v1096{((v1118*v3793)+(v1112*(v1118*(self.scalar_static_f64[206]*common.v3812))))}else{common.v60})})})})})})+(common.v795*(-v7764))))}else{(if v2153{common.v60}else{(if common.v2044{((if common.v2044{(common.v2049*(-v7366))}else{(if common.v1292{(common.v1297*(-v4520))}else{(if common.v1138{(common.v1143*(-v3958))}else{(if common.v970{(common.v981*(-v3394))}else{common.v60})})})})+v7587)}else{common.v60})})})})))/v7888);
        let v7903=(if v2197{((-(v652*(if v2191{common.v60}else{v7841})))/v7888)}else{common.v60});
        let v7904=(if v2197{(((v2192*v2741)-(v652*(if v2191{common.v60}else{v7842})))/v7888)}else{common.v60});
        let v7905=(if v2197{((-(v652*(if v2191{common.v60}else{v7843})))/v7888)}else{common.v60});
        let v7906=(if v2197{v7899}else{common.v60});
        let v7907=(if v2197{((-(v652*(if v2191{common.v60}else{(if common.v2155{(common.v579*((if common.v2155{((v2177*v7765)+(v2171*(v2177*(self.scalar_static_f64[198]*common.v7788))))}else{(if common.v1568{((v1597*v5610)+(v1590*(v1597*(self.scalar_static_f64[212]*common.v5633))))}else{(if (common.v1519!=0.0){((v1548*v5386)+(v1541*(v1548*(self.scalar_static_f64[210]*common.v5409))))}else{common.v60})})})+(common.v795*(-v7765))))}else{common.v60})})))/v7888)}else{common.v60});
        let v7913=(if v2197{(((common.v579*v2741)-(v652*common.v2672))/(common.v579*common.v579))}else{common.v60});
        let v7914=(-v7903);
        let v7915=(-v7904);
        let v7916=(-v7905);
        let v7917=(-v7906);
        let v7918=(-v7907);
        let v7923=(v2201*v2201);
        let v7995=(v2194*v2194);
        let v8047=(if v2227{common.v60}else{(if v2197{((v2223*common.v6978)+(common.v1941*(if v2218{(v2219*(v2221*(v7914/v2194)))}else{(if v2204{((v2214*(if v2204{(v651*(v2207*(v7914/v2201)))}else{common.v60}))+(v2209*(v2212*(v7903/v2201))))}else{common.v60})})))}else{common.v60})});
        let v8048=(if v2227{common.v60}else{(if v2197{((v2223*common.v6979)+(common.v1941*(if v2218{((v2221*((v2194*v2740)+(v651*v7883)))+(v2219*(v2221*(((v2194*v7915)-(v2205*v7883))/v7995))))}else{(if v2204{((v2214*(if v2204{((v2207*v2740)+(v651*(v2207*(((v2201*v7915)-(v2205*v7913))/v7923))))}else{common.v60}))+(v2209*(v7913+((v2212*(((v2201*v7904)-(v2199*v7913))/v7923))+(v2211*(v7883-v7913))))))}else{common.v60})})))}else{common.v60})});
        let v8049=(if v2227{common.v60}else{(if v2197{((v2223*common.v6980)+(common.v1941*(if v2218{((v2221*(v651*self.scalar_static_f64[281]))+(v2219*(v2221*(((v2194*v7916)-(v2205*self.scalar_static_f64[281]))/v7995))))}else{(if v2204{((v2214*(if v2204{(v651*(v2207*(v7916/v2201)))}else{common.v60}))+(v2209*((v2212*(v7905/v2201))+(v2211*self.scalar_static_f64[281]))))}else{common.v60})})))}else{common.v60})});
        let v8050=(if v2227{common.v60}else{(if v2197{((v2223*common.v6981)+(common.v1941*(if v2218{((v2221*(v651*self.scalar_static_f64[282]))+(v2219*(v2221*(((v2194*v7917)-(v2205*self.scalar_static_f64[282]))/v7995))))}else{(if v2204{((v2214*(if v2204{(v651*(v2207*(v7917/v2201)))}else{common.v60}))+(v2209*((v2212*(v7906/v2201))+(v2211*self.scalar_static_f64[282]))))}else{common.v60})})))}else{common.v60})});
        let v8051=(if v2227{common.v60}else{(if v2197{((v2223*common.v6982)+(common.v1941*(if v2218{(v2219*(v2221*(v7918/v2194)))}else{(if v2204{((v2214*(if v2204{(v651*(v2207*(v7918/v2201)))}else{common.v60}))+(v2209*(v2212*(v7907/v2201))))}else{common.v60})})))}else{common.v60})});
        let v8094=(if (v2230!=0.0){((((common.v5506/self.scalar_static_f64[246])+(common.v5287/self.scalar_static_f64[247]))+(common.v6978/common.v1698))+(common.v6962/self.scalar_static_f64[224]))}else{common.v60});
        let v8095=(if (v2230!=0.0){((((common.v5511/self.scalar_static_f64[246])+(common.v5288/self.scalar_static_f64[247]))+(((common.v1698*common.v6979)-(common.v1941*common.v5950))/common.v6024))+(common.v6966/self.scalar_static_f64[224]))}else{common.v60});
        let v8096=(if (v2230!=0.0){((((common.v5512/self.scalar_static_f64[246])+(common.v5289/self.scalar_static_f64[247]))+(((common.v1698*common.v6980)-(common.v1941*common.v5951))/common.v6024))+(common.v6970/self.scalar_static_f64[224]))}else{common.v60});
        let v8097=(if (v2230!=0.0){((((common.v5513/self.scalar_static_f64[246])+(common.v5290/self.scalar_static_f64[247]))+(((common.v1698*common.v6981)-(common.v1941*common.v5952))/common.v6024))+(common.v6974/self.scalar_static_f64[224]))}else{common.v60});
        let v8098=(if (v2230!=0.0){(((common.v5514/self.scalar_static_f64[246])+(common.v6982/common.v1698))+(common.v6977/self.scalar_static_f64[224]))}else{common.v60});
        let v8099=(v2241*v8094);
        let v8101=(v2241*v8095);
        let v8103=(v2241*v8096);
        let v8105=(v2241*v8097);
        let v8107=(v2241*v8098);
        let v8109=(common.v94*v2244);
        let v8132=(v2247*v2247);
        let v8147=(if (v2230!=0.0){((-(v656*(if (v2230!=0.0){(common.v32*(v8094+((v8099+v8099)/v8109)))}else{common.v60})))/v8132)}else{common.v60});
        let v8148=(if (v2230!=0.0){(((v2247*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[99]*(v654*(self.scalar_static_f64[100]*common.v2558)))}else{common.v60}))-(v656*(if (v2230!=0.0){(common.v32*(v8095+((v8101+v8101)/v8109)))}else{common.v60})))/v8132)}else{common.v60});
        let v8149=(if (v2230!=0.0){((-(v656*(if (v2230!=0.0){(common.v32*(v8096+((v8103+v8103)/v8109)))}else{common.v60})))/v8132)}else{common.v60});
        let v8150=(if (v2230!=0.0){((-(v656*(if (v2230!=0.0){(common.v32*(v8097+((v8105+v8105)/v8109)))}else{common.v60})))/v8132)}else{common.v60});
        let v8151=(if (v2230!=0.0){((-(v656*(if (v2230!=0.0){(common.v32*(v8098+((v8107+v8107)/v8109)))}else{common.v60})))/v8132)}else{common.v60});
        let v8177=(if v2252{(common.v456*(v2041*(self.scalar_static_f64[248]*v8147)))}else{common.v60});
        let v8178=(if v2252{((v2255*common.v2553)+(common.v456*((v2254*(v7239+v7292))+(v2041*(self.scalar_static_f64[248]*v8148)))))}else{common.v60});
        let v8179=(if v2252{(common.v456*((v2254*v7293)+(v2041*(self.scalar_static_f64[248]*v8149))))}else{common.v60});
        let v8180=(if v2252{(common.v456*((v2254*(v7240+v7294))+(v2041*(self.scalar_static_f64[248]*v8150))))}else{common.v60});
        let v8181=(if v2252{(common.v456*((v2254*(v7241+v7295))+(v2041*(self.scalar_static_f64[248]*v8151))))}else{common.v60});
        let v8207=(if v2261{((v2263*v8147)+(v2249*(-(common.v32*v8177))))}else{v8147});
        let v8208=(if v2261{((v2263*v8148)+(v2249*(-(common.v32*v8178))))}else{v8148});
        let v8209=(if v2261{((v2263*v8149)+(v2249*(-(common.v32*v8179))))}else{v8149});
        let v8210=(if v2261{((v2263*v8150)+(v2249*(-(common.v32*v8180))))}else{v8150});
        let v8211=(if v2261{((v2263*v8151)+(v2249*(-(common.v32*v8181))))}else{v8151});
        let v8235=(v2257*v2257);
        let v8265=(if (self.scalar_static_f64[249]!=0.0){(self.scalar_static_f64[250]*common.v2549)}else{common.v60});
        let v8266=(self.scalar_static_f64[0]/v2280);
        let v8269=(v2280*v2280);
        let v8271=(self.scalar_static_f64[273]/v2280);
        let v8272=scalar_limexp_derivative(v2281);
        let v8282=scalar_limexp_derivative(v2284);
        let v8313=(if (self.scalar_static_f64[251]!=0.0){(self.scalar_static_f64[0]/v2295)}else{common.v60});
        let v8314=(if (self.scalar_static_f64[251]!=0.0){((-(common.v14*(self.scalar_static_f64[252]*common.v2549)))/(v2295*v2295))}else{v7257});
        let v8315=(if (self.scalar_static_f64[251]!=0.0){(self.scalar_static_f64[273]/v2295)}else{v7258});
        let v8316=(if (self.scalar_static_f64[251]!=0.0){common.v60}else{v7259});
        let v8317=(if (self.scalar_static_f64[251]!=0.0){common.v60}else{v7260});
        let v8333=scalar_limexp_derivative(v2304);
        let v9049=-0.0;
        let v9074=(v2275*v2275);

        stamper.stamp_current_node1_local(
            Some(6),
            Some(7),
            multiplicity * ((common.v9*common.v60)),
            7,
            multiplicity * (v9049),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(5),
            multiplicity * ((common.v6*common.v60)),
            5,
            multiplicity * (v9049),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if (self.scalar_static_f64[249]!=0.0){(v727*v2287)}else{common.v60})}))),
            [1, 3, 4, 5],
            [(self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if (self.scalar_static_f64[249]!=0.0){(v727*(if (self.scalar_static_f64[249]!=0.0){(v8266*v8272)}else{common.v60}))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if (self.scalar_static_f64[249]!=0.0){(v727*(-(if (self.scalar_static_f64[249]!=0.0){(v8266*v8282)}else{common.v60})))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if (self.scalar_static_f64[249]!=0.0){((v2287*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[124]*(v725*(v2674+v2803)))}else{common.v60}))+(v727*((if (self.scalar_static_f64[249]!=0.0){(((-(common.v4*v8265))/v8269)*v8272)}else{common.v60})-(if (self.scalar_static_f64[249]!=0.0){(((-(common.v14*v8265))/v8269)*v8282)}else{common.v60}))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[49]{common.v60}else{(if (self.scalar_static_f64[249]!=0.0){(v727*((if (self.scalar_static_f64[249]!=0.0){(v8271*v8272)}else{common.v60})-(if (self.scalar_static_f64[249]!=0.0){(v8271*v8282)}else{common.v60})))}else{common.v60})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(5),
            multiplicity * ((self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){(v723*v2310)}else{common.v60})}))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){(v723*((v2308*(if v2306{common.v60}else{(if v2300{v8313}else{common.v60})}))+(v2307*((if v2300{common.v60}else{v8313})*v8333))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){((v2310*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[123]*(v721*(v2803+(self.scalar_static_f64[14]*common.v2562))))}else{common.v60}))+(v723*((v2308*(if v2306{common.v60}else{(if v2300{v8314}else{v7261})}))+(v2307*((if v2300{common.v60}else{v8314})*v8333)))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){(v723*((v2308*(if v2306{common.v60}else{(if v2300{v8315}else{v7262})}))+(v2307*((if v2300{common.v60}else{v8315})*v8333))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){(v723*((v2308*(if v2306{common.v60}else{(if v2300{v8316}else{v7263})}))+(v2307*((if v2300{common.v60}else{v8316})*v8333))))}else{common.v60})})), (self.scalar_static_f64[0]*(if self.scalar_static_bool[51]{common.v60}else{(if (self.scalar_static_f64[251]!=0.0){(v723*((v2308*(if v2306{common.v60}else{(if v2300{v8317}else{v7264})}))+(v2307*((if v2300{common.v60}else{v8317})*v8333))))}else{common.v60})}))],
            [],
            [],
            multiplicity,
        );
        let v2495_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2495);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (v2495_ddt),
            [1, 3, 4, 5, 6, 7],
            [((common.v9003) * ddt_scale), ((common.v9004) * ddt_scale), ((common.v9005) * ddt_scale), ((common.v9006) * ddt_scale), ((common.v9007) * ddt_scale), ((common.v9008) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2496_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2496);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2496_ddt),
            [1, 4, 5, 6],
            [((common.v9009) * ddt_scale), ((common.v9010) * ddt_scale), ((common.v9011) * ddt_scale), ((common.v9012) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2497_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2497);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (v2497_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[298]) * ddt_scale)),
            5,
            multiplicity * (((self.scalar_static_f64[299]) * ddt_scale)),
        );
        let v2498_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2498);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2498_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[300]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[301]) * ddt_scale)),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(2),
            multiplicity * ((if (self.scalar_static_f64[267]!=0.0){(v18/v767)}else{common.v60})),
            2,
            multiplicity * ((if (self.scalar_static_f64[267]!=0.0){(v2527/v767)}else{common.v60})),
            4,
            multiplicity * ((if (self.scalar_static_f64[267]!=0.0){((-(v18*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[141]*(v765*(self.scalar_static_f64[142]*common.v2558)))}else{common.v60})))/(v767*v767))}else{common.v60})),
            7,
            multiplicity * ((if (self.scalar_static_f64[267]!=0.0){(common.v48/v767)}else{common.v60})),
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
            multiplicity * ((if (self.scalar_static_f64[268]!=0.0){(v20/v759)}else{common.v60})),
            0,
            multiplicity * ((if (self.scalar_static_f64[268]!=0.0){(v2527/v759)}else{common.v60})),
            4,
            multiplicity * ((if (self.scalar_static_f64[268]!=0.0){((-(v20*(if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[137]*(v757*(self.scalar_static_f64[138]*common.v2558)))}else{common.v60})))/(v759*v759))}else{common.v60})),
            5,
            multiplicity * ((if (self.scalar_static_f64[268]!=0.0){(common.v48/v759)}else{common.v60})),
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
            multiplicity * ((if (self.scalar_static_f64[269]!=0.0){(v21/v2275)}else{common.v60})),
            [1, 4, 5, 6, 7],
            [(if (self.scalar_static_f64[269]!=0.0){((v2275-(v21*(if v2273{common.v60}else{(if v2267{(((v2257*((v2269*v8207)+(v2265*(v8177/v2268))))-(v2270*v8177))/v8235)}else{v8207})})))/v9074)}else{common.v60}), (if (self.scalar_static_f64[269]!=0.0){((-(v21*((if (self.scalar_static_f64[148]!=0.0){(self.scalar_static_f64[139]*(v761*(self.scalar_static_f64[140]*common.v2558)))}else{common.v60})+(if v2273{common.v60}else{(if v2267{(((v2257*((v2269*v8208)+(v2265*(v8178/v2268))))-(v2270*v8178))/v8235)}else{v8208})}))))/v9074)}else{common.v60}), (if (self.scalar_static_f64[269]!=0.0){((-(v21*(if v2273{common.v60}else{(if v2267{(((v2257*((v2269*v8209)+(v2265*(v8179/v2268))))-(v2270*v8179))/v8235)}else{v8209})})))/v9074)}else{common.v60}), (if (self.scalar_static_f64[269]!=0.0){(((-v2275)-(v21*(if v2273{common.v60}else{(if v2267{(((v2257*((v2269*v8210)+(v2265*(v8180/v2268))))-(v2270*v8180))/v8235)}else{v8210})})))/v9074)}else{common.v60}), (if (self.scalar_static_f64[269]!=0.0){((-(v21*(if v2273{common.v60}else{(if v2267{(((v2257*((v2269*v8211)+(v2265*(v8181/v2268))))-(v2270*v8181))/v8235)}else{v8211})})))/v9074)}else{common.v60})],
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
            multiplicity * ((self.scalar_static_f64[0]*(v2040-v2228))),
            [1, 4, 5, 6, 7],
            [(self.scalar_static_f64[0]*(-v8047)), (self.scalar_static_f64[0]*(v7292-v8048)), (self.scalar_static_f64[0]*(v7293-v8049)), (self.scalar_static_f64[0]*(v7294-v8050)), (self.scalar_static_f64[0]*(v7295-v8051))],
            [],
            [],
            multiplicity,
        );
        let v2500_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2500);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2500_ddt),
            [1, 4, 5, 6, 7],
            [((common.v9022) * ddt_scale), ((common.v9023) * ddt_scale), ((common.v9024) * ddt_scale), ((common.v9025) * ddt_scale), ((common.v9026) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((self.scalar_static_f64[0]*v2017)),
            4,
            multiplicity * ((self.scalar_static_f64[0]*v7239)),
            6,
            multiplicity * ((self.scalar_static_f64[0]*v7240)),
            7,
            multiplicity * ((self.scalar_static_f64[0]*v7241)),
        );
        let v2502_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2502);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2502_ddt),
            [1, 4, 5, 6, 7, 8],
            [((common.v9030) * ddt_scale), ((common.v9031) * ddt_scale), ((common.v9032) * ddt_scale), ((common.v9033) * ddt_scale), ((common.v9034) * ddt_scale), ((self.scalar_static_f64[302]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[0]*(common.v2475-common.v1938))),
            [1, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[0]*(common.v8924-common.v6962)), (self.scalar_static_f64[0]*(common.v8925-common.v6966)), (self.scalar_static_f64[0]*(common.v8926-common.v6970)), (self.scalar_static_f64[0]*(common.v8927-common.v6974)), (self.scalar_static_f64[0]*(common.v8928-common.v6977)), self.scalar_static_f64[302]],
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
            multiplicity * ((if self.scalar_static_bool[71]{((common.v439/v774)-(if (self.scalar_static_f64[260]!=0.0){((common.v11*v1942)+(v2193*v2228))}else{common.v60}))}else{common.v60})),
            [1, 4, 5, 6, 7],
            [(if self.scalar_static_bool[71]{(-(if (self.scalar_static_f64[260]!=0.0){((common.v11*(common.v6978-common.v6962))+(v2193*v8047))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(((v774-(common.v439*(if (self.scalar_static_f64[148]!=0.0){((v772*(self.scalar_static_f64[143]*(v769*(self.scalar_static_f64[144]*common.v2558))))+(v770*(self.scalar_static_f64[145]*common.v2554)))}else{common.v60})))/(v774*v774))-(if (self.scalar_static_f64[260]!=0.0){((common.v11*(common.v6979-common.v6966))+((v2228*common.v2663)+(v2193*v8048)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if (self.scalar_static_f64[260]!=0.0){(((self.scalar_static_f64[0]*v1942)+(common.v11*(common.v6980-common.v6970)))+((self.scalar_static_f64[0]*v2228)+(v2193*v8049)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if (self.scalar_static_f64[260]!=0.0){(((v1942*self.scalar_static_f64[274])+(common.v11*(common.v6981-common.v6974)))+((v2228*self.scalar_static_f64[273])+(v2193*v8050)))}else{common.v60}))}else{common.v60}), (if self.scalar_static_bool[71]{(-(if (self.scalar_static_f64[260]!=0.0){(((v1942*self.scalar_static_f64[273])+(common.v11*(common.v6982-common.v6977)))+(v2193*v8051))}else{common.v60}))}else{common.v60})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{v2522}else{common.v60})}else{common.v60})),
            4,
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{(self.scalar_static_f64[270]*ddt_scale)}else{common.v60})}else{common.v60})),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * ((if self.scalar_static_bool[59]{common.v2466}else{(if (self.scalar_static_f64[262]!=0.0){(common.v2467-common.v1968)}else{common.v60})})),
            [1, 4, 5, 6, 7, 8],
            [(if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8889-common.v7143)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8890-common.v7144)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8891-common.v7145)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8892-common.v7146)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8893-common.v7147)}else{common.v60})}), self.scalar_static_f64[291]],
            [],
            [],
            multiplicity,
        );
        let v2484_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2484);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v2484_ddt),
            [1, 4, 5, 6, 7, 8],
            [((common.v8963) * ddt_scale), ((common.v8964) * ddt_scale), ((common.v8965) * ddt_scale), ((common.v8966) * ddt_scale), ((common.v8967) * ddt_scale), ((self.scalar_static_f64[292]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * ((if self.scalar_static_bool[59]{common.v2474}else{(if (self.scalar_static_f64[262]!=0.0){(common.v2475-common.v1941)}else{common.v60})})),
            [1, 4, 5, 6, 7, 9],
            [(if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8924-common.v6978)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8925-common.v6979)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8926-common.v6980)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8927-common.v6981)}else{common.v60})}), (if self.scalar_static_bool[59]{common.v60}else{(if (self.scalar_static_f64[262]!=0.0){(common.v8928-common.v6982)}else{common.v60})}), self.scalar_static_f64[291]],
            [],
            [],
            multiplicity,
        );
        let v2486_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2486);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * (v2486_ddt),
            [1, 4, 5, 6, 7, 9],
            [((common.v8974) * ddt_scale), ((common.v8975) * ddt_scale), ((common.v8976) * ddt_scale), ((common.v8977) * ddt_scale), ((common.v8978) * ddt_scale), ((self.scalar_static_f64[293]) * ddt_scale)],
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
        let v2522=0.0;

        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[common.v9003, common.v9004, common.v9005, common.v9006, common.v9007, common.v9008],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6]],
            &[common.v9009, common.v9010, common.v9011, common.v9012],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[298]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[299]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[300]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[301]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[common.v9022, common.v9023, common.v9024, common.v9025, common.v9026],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[common.v9030, common.v9031, common.v9032, common.v9033, common.v9034, self.scalar_static_f64[302]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if self.scalar_static_bool[71]{(if self.scalar_static_bool[68]{(self.scalar_static_f64[270]*1.0)}else{common.v60})}else{common.v60})),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[common.v8963, common.v8964, common.v8965, common.v8966, common.v8967, self.scalar_static_f64[292]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[common.v8974, common.v8975, common.v8976, common.v8977, common.v8978, self.scalar_static_f64[293]],
            &[],
            &[],
            multiplicity,
        );
    }
}
