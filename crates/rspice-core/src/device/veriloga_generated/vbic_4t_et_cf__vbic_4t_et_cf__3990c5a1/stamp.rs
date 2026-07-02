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
    v6: f64,
    v11: f64,
    v12: f64,
    v13: f64,
    v51: f64,
    v52: f64,
    v84: f64,
    v182: f64,
    v185: f64,
    v209: f64,
    v247: f64,
    v304: f64,
    v339: f64,
    v340: f64,
    v341: f64,
    v342: f64,
    v343: f64,
    v344: f64,
    v345: f64,
    v346: f64,
    v348: f64,
    v349: f64,
    v360: f64,
    v824: f64,
    v825: f64,
    v884: f64,
    v905: f64,
    v911: f64,
    v946: f64,
    v950: f64,
    v952: f64,
    v954: f64,
    v956: f64,
    v958: f64,
    v966: f64,
    v1153: f64,
    v1159: f64,
    v1160: f64,
    v1165: f64,
    v1168: f64,
    v1201: f64,
    v1215: f64,
    v1283: f64,
    v1285: f64,
    v1292: f64,
    v1293: f64,
    v1296: f64,
    v1300: f64,
    v1303: f64,
    v1306: f64,
    v1344: f64,
    v1345: f64,
    v1396: f64,
    v1438: f64,
    v1626: f64,
    v3229: f64,
    v3230: f64,
    v3231: f64,
    v3232: f64,
    v3233: f64,
    v3234: f64,
    v3235: f64,
    v3337: f64,
    v3338: f64,
    v3339: f64,
    v3358: f64,
    v3359: f64,
    v3360: f64,
    v3471: f64,
    v3472: f64,
    v3473: f64,
    v3474: f64,
    v3475: f64,
    v3479: f64,
    v3516: f64,
    v3519: f64,
    v3520: f64,
    v3521: f64,
    v3522: f64,
    v3523: f64,
    v3524: f64,
    v3531: f64,
    v3532: f64,
    v3533: f64,
    v3534: f64,
    v3535: f64,
    v3539: f64,
    v3544: f64,
    v3545: f64,
    v3546: f64,
    v3565: f64,
    v3566: f64,
    v3567: f64,
    v3568: f64,
    v3569: f64,
    v4362: f64,
    v4363: f64,
    v4364: f64,
    v4366: f64,
    v4367: f64,
    v4368: f64,
    v4382: f64,
    v4383: f64,
    v4384: f64,
    v4391: f64,
    v4392: f64,
    v4393: f64,
    v4783: f64,
    v4787: f64,
    v4796: f64,
    v4797: f64,
    v4798: f64,
    v4805: f64,
    v4806: f64,
    v4807: f64,
    v4808: f64,
    v4813: f64,
    v4815: f64,
    v4825: f64,
    v4826: f64,
    v4827: f64,
    v4828: f64,
    v4829: f64,
    v4830: f64,
    v4837: f64,
    v4844: f64,
    v4845: f64,
    v4846: f64,
    v4847: f64,
    v4848: f64,
    v4851: f64,
    v4852: f64,
    v4853: f64,
    v4854: f64,
    v4855: f64,
    v4859: f64,
    v4860: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v6=ctx.node_voltage(nodes[4]);
        let v7=(self.scalar_static_f64[300]+v6);
        let v11=((v7*1.3806503e-23)/1.602176462e-19);
        let v12=(v7/self.scalar_static_f64[1]);
        let v13=(v7-self.scalar_static_f64[1]);
        let v17=(self.scalar_static_f64[3]*f64::powf(v12,self.scalar_static_f64[4]));
        let v48=f64::powf(v12,self.scalar_static_f64[20]);
        let v51=1.0;
        let v52=(v51-v12);
        let v53=(self.scalar_static_f64[22]*v52);
        let v55=((v53/v11)).exp();
        let v56=(v48*v55);
        let v60=(self.scalar_static_f64[19]*f64::powf(v56,self.scalar_static_f64[24]));
        let v63=f64::powf(v12,self.scalar_static_f64[26]);
        let v66=(v52*self.scalar_static_f64[28]);
        let v68=((v66/v11)).exp();
        let v69=(v63*v68);
        let v73=(self.scalar_static_f64[25]*f64::powf(v69,self.scalar_static_f64[30]));
        let v77=(v52*self.scalar_static_f64[33]);
        let v79=((v77/v11)).exp();
        let v80=(v48*v79);
        let v84=(self.scalar_static_f64[31]*f64::powf(v80,self.scalar_static_f64[35]));
        let v161=(v51+(v13*self.scalar_static_f64[70]));
        let v162=(self.scalar_static_f64[23]*v161);
        let v163=(self.scalar_static_f64[29]*v161);
        let v182=2.0;
        let v184=(v182*(v11/v12));
        let v185=0.5;
        let v188=(v12*self.scalar_static_f64[79]);
        let v190=((v188/v11)).exp();
        let v191=-0.5;
        let v193=(v12*self.scalar_static_f64[80]);
        let v195=((v193/v11)).exp();
        let v196=(v190-v195);
        let v197=(v196).ln();
        let v198=(v184*v197);
        let v201=(v11*3.0);
        let v202=(v12).ln();
        let v203=(v201*v202);
        let v205=(v12-v51);
        let v207=(((v12*v198)-v203)-(self.scalar_static_f64[38]*v205));
        let v208=(v11*v182);
        let v209=4.0;
        let v210=(-v207);
        let v212=((v210/v11)).exp();
        let v215=((v51+(v209*v212))).sqrt();
        let v217=(v185*(v51+v215));
        let v218=(v217).ln();
        let v220=(v207+(v208*v218));
        let v223=(v12*self.scalar_static_f64[82]);
        let v225=((v223/v11)).exp();
        let v227=(v12*self.scalar_static_f64[83]);
        let v229=((v227/v11)).exp();
        let v230=(v225-v229);
        let v231=(v230).ln();
        let v232=(v184*v231);
        let v236=(((v12*v232)-v203)-(self.scalar_static_f64[49]*v205));
        let v237=(-v236);
        let v239=((v237/v11)).exp();
        let v242=((v51+(v209*v239))).sqrt();
        let v244=(v185*(v51+v242));
        let v245=(v244).ln();
        let v247=(v236+(v208*v245));
        let v250=(v12*self.scalar_static_f64[85]);
        let v252=((v250/v11)).exp();
        let v254=(v12*self.scalar_static_f64[86]);
        let v256=((v254/v11)).exp();
        let v257=(v252-v256);
        let v258=(v257).ln();
        let v259=(v184*v258);
        let v263=(((v12*v259)-v203)-(self.scalar_static_f64[61]*v205));
        let v264=(-v263);
        let v266=((v264/v11)).exp();
        let v269=((v51+(v209*v266))).sqrt();
        let v271=(v185*(v51+v269));
        let v272=(v271).ln();
        let v274=(v263+(v208*v272));
        let v276=(self.scalar_static_f64[78]/v220);
        let v279=(self.scalar_static_f64[87]*f64::powf(v276,self.scalar_static_f64[88]));
        let v281=(self.scalar_static_f64[81]/v247);
        let v283=f64::powf(v281,self.scalar_static_f64[90]);
        let v284=(self.scalar_static_f64[89]*v283);
        let v286=(v283*self.scalar_static_f64[91]);
        let v288=(self.scalar_static_f64[84]/v274);
        let v291=(self.scalar_static_f64[92]*f64::powf(v288,self.scalar_static_f64[93]));
        let v293=(v48*self.scalar_static_f64[94]);
        let v294=(v55*v293);
        let v304=0.0;
        let v314=(if self.scalar_static_bool[2]{(v51/v17)}else{v304});
        let v339=ctx.node_voltage(nodes[8]);
        let v340=ctx.node_voltage(nodes[9]);
        let v341=(v339-v340);
        let v342=ctx.node_voltage(nodes[7]);
        let v343=(v342-v340);
        let v344=ctx.node_voltage(nodes[6]);
        let v345=(v339-v344);
        let v346=ctx.node_voltage(nodes[5]);
        let v347=(v339-v346);
        let v348=ctx.node_voltage(nodes[10]);
        let v349=(v342-v348);
        let v350=(-v220);
        let v352=(v350*self.scalar_static_f64[119]);
        let v355=(v341+v352);
        let v356=(if self.scalar_static_bool[9]{v355}else{v304});
        let v357=(v356>v304);
        let v358=(self.scalar_static_bool[9]&&v357);
        let v360=-1.0;
        let v363=(if v358{self.scalar_static_f64[123]}else{v304});
        let v366=(v51-(self.scalar_static_f64[121]*(self.scalar_static_f64[121]*v363)));
        let v372=(v356*self.scalar_static_f64[125]);
        let v374=(self.scalar_static_f64[121]+(v372/v220));
        let v379=(self.scalar_static_bool[9]&&(!v357));
        let v381=(v51-(v341/v220));
        let v383=(v51-f64::powf(v381,self.scalar_static_f64[124]));
        let v386=(if v379{((v220*v383)/self.scalar_static_f64[124])}else{(if v358{((v220*v366)/self.scalar_static_f64[124])}else{v304})});
        let v387=(if v379{v304}else{(if v358{(v363*(v356*v374))}else{v304})});
        let v395=(((v352*v352)+self.scalar_static_f64[127])).sqrt();
        let v396=(if self.scalar_static_bool[10]{v395}else{v304});
        let v399=(if self.scalar_static_bool[10]{(v191*(v352+v396))}else{v304});
        let v401=(v51-(v399/v220));
        let v402=f64::powf(v401,self.scalar_static_f64[124]);
        let v405=(if self.scalar_static_bool[10]{((v350*v402)/self.scalar_static_f64[124])}else{v304});
        let v406=(if self.scalar_static_bool[10]{v355}else{v304});
        let v409=((self.scalar_static_f64[127]+(v406*v406))).sqrt();
        let v410=(if self.scalar_static_bool[10]{v409}else{v304});
        let v414=(if self.scalar_static_bool[10]{((v185*(v406-v410))-v352)}else{v304});
        let v416=(v51-(v414/v220));
        let v417=f64::powf(v416,self.scalar_static_f64[124]);
        let v420=(if self.scalar_static_bool[10]{((v350*v417)/self.scalar_static_f64[124])}else{v386});
        let v428=(if self.scalar_static_bool[10]{((v420+(self.scalar_static_f64[129]*(v399+(v341-v414))))-v405)}else{(if self.scalar_static_bool[9]{(v386+v387)}else{v304})});
        let v429=(v343+v352);
        let v430=(if self.scalar_static_bool[9]{v429}else{v356});
        let v431=(v430>v304);
        let v432=(self.scalar_static_bool[9]&&v431);
        let v433=(if v432{self.scalar_static_f64[123]}else{v363});
        let v436=(v51-(self.scalar_static_f64[121]*(self.scalar_static_f64[121]*v433)));
        let v440=(self.scalar_static_f64[125]*v430);
        let v442=(self.scalar_static_f64[121]+(v440/v220));
        let v447=(self.scalar_static_bool[9]&&(!v431));
        let v449=(v51-(v343/v220));
        let v451=(v51-f64::powf(v449,self.scalar_static_f64[124]));
        let v454=(if v447{((v220*v451)/self.scalar_static_f64[124])}else{(if v432{((v220*v436)/self.scalar_static_f64[124])}else{v420})});
        let v455=(if v447{v304}else{(if v432{(v433*(v430*v442))}else{v387})});
        let v458=(if self.scalar_static_bool[10]{v395}else{v396});
        let v461=(if self.scalar_static_bool[10]{(v191*(v352+v458))}else{v399});
        let v463=(v51-(v461/v220));
        let v464=f64::powf(v463,self.scalar_static_f64[124]);
        let v467=(if self.scalar_static_bool[10]{((v350*v464)/self.scalar_static_f64[124])}else{v405});
        let v468=(if self.scalar_static_bool[10]{v429}else{v406});
        let v471=((self.scalar_static_f64[127]+(v468*v468))).sqrt();
        let v472=(if self.scalar_static_bool[10]{v471}else{v410});
        let v476=(if self.scalar_static_bool[10]{((v185*(v468-v472))-v352)}else{v414});
        let v478=(v51-(v476/v220));
        let v479=f64::powf(v478,self.scalar_static_f64[124]);
        let v482=(if self.scalar_static_bool[10]{((v350*v479)/self.scalar_static_f64[124])}else{v454});
        let v488=(if self.scalar_static_bool[10]{((v482+(self.scalar_static_f64[129]*(v461+(v343-v476))))-v467)}else{(if self.scalar_static_bool[9]{(v454+v455)}else{v304})});
        let v489=(-v247);
        let v490=(self.scalar_static_f64[119]*v489);
        let v493=(v345+v490);
        let v494=(if self.scalar_static_bool[11]{v493}else{v430});
        let v495=(v494>v304);
        let v496=(self.scalar_static_bool[11]&&v495);
        let v499=(if v496{self.scalar_static_f64[132]}else{v433});
        let v502=(v51-(self.scalar_static_f64[121]*(self.scalar_static_f64[121]*v499)));
        let v508=(v494*self.scalar_static_f64[134]);
        let v510=(self.scalar_static_f64[121]+(v508/v247));
        let v518=(self.scalar_static_bool[12]&&(v345<self.scalar_static_f64[136]));
        let v520=(self.scalar_static_bool[11]&&(!v495));
        let v521=(v518&&v520);
        let v523=(v51+(self.scalar_static_f64[135]/v247));
        let v524=f64::powf(v523,self.scalar_static_f64[133]);
        let v526=(self.scalar_static_f64[133]*(v345+self.scalar_static_f64[135]));
        let v527=(v247+self.scalar_static_f64[135]);
        let v529=(v51-(v526/v527));
        let v531=(v51-(v524*v529));
        let v536=(v520&&(!v518));
        let v538=(v51-(v345/v247));
        let v540=(v51-f64::powf(v538,self.scalar_static_f64[133]));
        let v543=(if v536{((v247*v540)/self.scalar_static_f64[133])}else{(if v521{((v247*v531)/self.scalar_static_f64[133])}else{(if v496{((v247*v502)/self.scalar_static_f64[133])}else{v482})})});
        let v544=(if v520{v304}else{(if v496{(v499*(v494*v510))}else{v455})});
        let v552=(v490+self.scalar_static_f64[135]);
        let v553=(self.scalar_static_f64[135]-v490);
        let v554=(v552/v553);
        let v555=(if self.scalar_static_bool[16]{v554}else{v304});
        let v556=(v182*v555);
        let v557=(v555-v51);
        let v562=(((v557*v557)+self.scalar_static_f64[139])).sqrt();
        let v563=(v51+v555);
        let v568=(((v563*v563)+self.scalar_static_f64[141])).sqrt();
        let v569=(v562+v568);
        let v571=(if self.scalar_static_bool[16]{(v556/v569)}else{v304});
        let v576=(if self.scalar_static_bool[16]{(v185*(((v553*v571)-self.scalar_static_f64[135])-v490))}else{v461});
        let v578=(v51-(v576/v247));
        let v580=(v51-f64::powf(v578,self.scalar_static_f64[133]));
        let v583=(if self.scalar_static_bool[16]{((v247*v580)/self.scalar_static_f64[133])}else{v304});
        let v586=(v490+(self.scalar_static_f64[135]+(v182*v345)));
        let v588=(if self.scalar_static_bool[16]{(v586/v553)}else{v304});
        let v589=(v182*v588);
        let v590=(v588-v51);
        let v593=((self.scalar_static_f64[139]+(v590*v590))).sqrt();
        let v594=(v51+v588);
        let v597=((self.scalar_static_f64[141]+(v594*v594))).sqrt();
        let v598=(v593+v597);
        let v600=(if self.scalar_static_bool[16]{(v589/v598)}else{v304});
        let v605=(if self.scalar_static_bool[16]{(v185*(((v553*v600)-self.scalar_static_f64[135])-v490))}else{v476});
        let v607=(v51-(v605/v247));
        let v609=(v51-f64::powf(v607,self.scalar_static_f64[133]));
        let v612=(if self.scalar_static_bool[16]{((v247*v609)/self.scalar_static_f64[133])}else{v543});
        let v615=(if self.scalar_static_bool[16]{(v185*(v51+v600))}else{v304});
        let v617=f64::powf(v523,self.scalar_static_f64[142]);
        let v618=(if self.scalar_static_bool[16]{v617}else{v304});
        let v620=(v51+(v490/v247));
        let v621=f64::powf(v620,self.scalar_static_f64[142]);
        let v622=(if self.scalar_static_bool[16]{v621}else{v304});
        let v623=(v51-v615);
        let v627=(if self.scalar_static_bool[16]{((v618*v623)+(v615*v622))}else{v304});
        let v629=(v576+(v345-v605));
        let v631=(if self.scalar_static_bool[16]{(v627*v629)}else{v304});
        let v639=((self.scalar_static_f64[139]+(v490*v490))).sqrt();
        let v640=(if self.scalar_static_bool[18]{v639}else{v458});
        let v643=(if self.scalar_static_bool[18]{(v191*(v490+v640))}else{v576});
        let v645=(v51-(v643/v247));
        let v646=f64::powf(v645,self.scalar_static_f64[133]);
        let v649=(if self.scalar_static_bool[18]{((v489*v646)/self.scalar_static_f64[133])}else{v467});
        let v650=(if self.scalar_static_bool[18]{v493}else{v468});
        let v653=((self.scalar_static_f64[139]+(v650*v650))).sqrt();
        let v654=(if self.scalar_static_bool[18]{v653}else{v472});
        let v658=(if self.scalar_static_bool[18]{((v185*(v650-v654))-v490)}else{v605});
        let v660=(v51-(v658/v247));
        let v661=f64::powf(v660,self.scalar_static_f64[133]);
        let v664=(if self.scalar_static_bool[18]{((v489*v661)/self.scalar_static_f64[133])}else{v612});
        let v671=(if self.scalar_static_bool[18]{((v664+(self.scalar_static_f64[143]*(v643+(v345-v658))))-v649)}else{(if self.scalar_static_bool[16]{((v612+v631)-v583)}else{(if self.scalar_static_bool[11]{(v543+v544)}else{v304})})});
        let v672=(v349+v490);
        let v673=(if self.scalar_static_bool[11]{v672}else{v494});
        let v674=(v673>v304);
        let v675=(self.scalar_static_bool[11]&&v674);
        let v676=(if v675{self.scalar_static_f64[132]}else{v499});
        let v679=(v51-(self.scalar_static_f64[121]*(self.scalar_static_f64[121]*v676)));
        let v683=(self.scalar_static_f64[134]*v673);
        let v685=(self.scalar_static_f64[121]+(v683/v247));
        let v690=(self.scalar_static_bool[12]&&(v349<self.scalar_static_f64[136]));
        let v692=(self.scalar_static_bool[11]&&(!v674));
        let v693=(v690&&v692);
        let v695=(self.scalar_static_f64[133]*(v349+self.scalar_static_f64[135]));
        let v697=(v51-(v695/v527));
        let v699=(v51-(v524*v697));
        let v704=(v692&&(!v690));
        let v706=(v51-(v349/v247));
        let v708=(v51-f64::powf(v706,self.scalar_static_f64[133]));
        let v711=(if v704{((v247*v708)/self.scalar_static_f64[133])}else{(if v693{((v247*v699)/self.scalar_static_f64[133])}else{(if v675{((v247*v679)/self.scalar_static_f64[133])}else{v664})})});
        let v712=(if v692{v304}else{(if v675{(v676*(v673*v685))}else{v544})});
        let v715=(if self.scalar_static_bool[16]{v554}else{v555});
        let v716=(v182*v715);
        let v717=(v715-v51);
        let v720=((self.scalar_static_f64[139]+(v717*v717))).sqrt();
        let v721=(v51+v715);
        let v724=((self.scalar_static_f64[141]+(v721*v721))).sqrt();
        let v725=(v720+v724);
        let v727=(if self.scalar_static_bool[16]{(v716/v725)}else{v571});
        let v732=(if self.scalar_static_bool[16]{(v185*(((v553*v727)-self.scalar_static_f64[135])-v490))}else{v643});
        let v734=(v51-(v732/v247));
        let v736=(v51-f64::powf(v734,self.scalar_static_f64[133]));
        let v742=(v490+(self.scalar_static_f64[135]+(v182*v349)));
        let v744=(if self.scalar_static_bool[16]{(v742/v553)}else{v588});
        let v745=(v182*v744);
        let v746=(v744-v51);
        let v749=((self.scalar_static_f64[139]+(v746*v746))).sqrt();
        let v750=(v51+v744);
        let v753=((self.scalar_static_f64[141]+(v750*v750))).sqrt();
        let v754=(v749+v753);
        let v756=(if self.scalar_static_bool[16]{(v745/v754)}else{v600});
        let v761=(if self.scalar_static_bool[16]{(v185*(((v553*v756)-self.scalar_static_f64[135])-v490))}else{v658});
        let v763=(v51-(v761/v247));
        let v765=(v51-f64::powf(v763,self.scalar_static_f64[133]));
        let v768=(if self.scalar_static_bool[16]{((v247*v765)/self.scalar_static_f64[133])}else{v711});
        let v771=(if self.scalar_static_bool[16]{(v185*(v51+v756))}else{v615});
        let v772=(if self.scalar_static_bool[16]{v617}else{v618});
        let v773=(if self.scalar_static_bool[16]{v621}else{v622});
        let v774=(v51-v771);
        let v778=(if self.scalar_static_bool[16]{((v772*v774)+(v771*v773))}else{v627});
        let v780=(v732+(v349-v761));
        let v786=(if self.scalar_static_bool[18]{v639}else{v640});
        let v789=(if self.scalar_static_bool[18]{(v191*(v490+v786))}else{v732});
        let v791=(v51-(v789/v247));
        let v792=f64::powf(v791,self.scalar_static_f64[133]);
        let v795=(if self.scalar_static_bool[18]{((v489*v792)/self.scalar_static_f64[133])}else{v649});
        let v796=(if self.scalar_static_bool[18]{v672}else{v650});
        let v799=((self.scalar_static_f64[139]+(v796*v796))).sqrt();
        let v800=(if self.scalar_static_bool[18]{v799}else{v654});
        let v804=(if self.scalar_static_bool[18]{((v185*(v796-v800))-v490)}else{v761});
        let v806=(v51-(v804/v247));
        let v807=f64::powf(v806,self.scalar_static_f64[133]);
        let v810=(if self.scalar_static_bool[18]{((v489*v807)/self.scalar_static_f64[133])}else{v768});
        let v816=(if self.scalar_static_bool[18]{((v810+(self.scalar_static_f64[143]*(v789+(v349-v804))))-v795)}else{(if self.scalar_static_bool[16]{((v768+(if self.scalar_static_bool[16]{(v778*v780)}else{v631}))-(if self.scalar_static_bool[16]{((v247*v736)/self.scalar_static_f64[133])}else{v583}))}else{(if self.scalar_static_bool[11]{(v711+v712)}else{v304})})});
        let v818=(-v274);
        let v820=(if self.scalar_static_bool[19]{(self.scalar_static_f64[119]*v818)}else{v490});
        let v824=ctx.node_voltage(nodes[11]);
        let v825=(v824-v348);
        let v826=(v820+v825);
        let v827=(if self.scalar_static_bool[21]{v826}else{v673});
        let v828=(v827>v304);
        let v829=(self.scalar_static_bool[21]&&v828);
        let v832=(if v829{self.scalar_static_f64[146]}else{v676});
        let v835=(v51-(self.scalar_static_f64[121]*(self.scalar_static_f64[121]*v832)));
        let v841=(v827*self.scalar_static_f64[148]);
        let v843=(self.scalar_static_f64[121]+(v841/v274));
        let v848=(self.scalar_static_bool[21]&&(!v828));
        let v850=(v51-(v825/v274));
        let v852=(v51-f64::powf(v850,self.scalar_static_f64[147]));
        let v855=(if v848{((v274*v852)/self.scalar_static_f64[147])}else{(if v829{((v274*v835)/self.scalar_static_f64[147])}else{v810})});
        let v865=(((v820*v820)+self.scalar_static_f64[150])).sqrt();
        let v869=(if self.scalar_static_bool[23]{(v191*(v820+(if self.scalar_static_bool[23]{v865}else{v786})))}else{v789});
        let v871=(v51-(v869/v274));
        let v872=f64::powf(v871,self.scalar_static_f64[147]);
        let v876=(if self.scalar_static_bool[23]{v826}else{v796});
        let v879=((self.scalar_static_f64[150]+(v876*v876))).sqrt();
        let v884=(if self.scalar_static_bool[23]{((v185*(v876-(if self.scalar_static_bool[23]{v879}else{v800})))-v820)}else{v804});
        let v886=(v51-(v884/v274));
        let v887=f64::powf(v886,self.scalar_static_f64[147]);
        let v900=(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{(((if self.scalar_static_bool[23]{((v818*v887)/self.scalar_static_f64[147])}else{v855})+(self.scalar_static_f64[152]*(v869+(v825-v884))))-(if self.scalar_static_bool[23]{((v818*v872)/self.scalar_static_f64[147])}else{v795}))}else{(if self.scalar_static_bool[21]{(v855+(if v848{v304}else{(if v829{(v832*(v827*v843))}else{v712})}))}else{v304})})});
        let v901=(v11*v162);
        let v902=(v341/v901);
        let v904=(scalar_limexp(v902)-v51);
        let v905=(v60*v904);
        let v906=(v11*v163);
        let v907=(v345/v906);
        let v908=scalar_limexp(v907);
        let v909=(v60*v73);
        let v910=(v908-v51);
        let v911=(v909*v910);
        let v915=((v51+(self.scalar_static_f64[102]*v428))+(self.scalar_static_f64[99]*v671));
        let v916=0.0001;
        let v917=(v915-v916);
        let v921=(((v917*v917)+1e-8)).sqrt();
        let v925=(v916+(v185*((v915+v921)-v916)));
        let v934=(v209*((v314*v905)+(self.scalar_static_f64[105]*v911)));
        let v935=(f64::powf(v925,self.scalar_static_f64[155])+v934);
        let v941=(v185*v925);
        let v942=(v51+v934);
        let v944=(v51+f64::powf(v942,self.scalar_static_f64[154]));
        let v946=(if self.scalar_static_bool[26]{(v941*v944)}else{(if self.scalar_static_bool[25]{(v185*(v925+f64::powf(v935,self.scalar_static_f64[154])))}else{v304})});
        let v950=(v11*self.scalar_static_f64[34]);
        let v952=(if self.scalar_static_bool[27]{(v349/v950)}else{v907});
        let v954=(if self.scalar_static_bool[27]{scalar_limexp(v952)}else{v908});
        let v956=(if self.scalar_static_bool[27]{(v345/v950)}else{v304});
        let v958=(if self.scalar_static_bool[27]{scalar_limexp(v956)}else{v304});
        let v964=(((v954*self.scalar_static_f64[156])+(v958*self.scalar_static_f64[157]))-v51);
        let v966=(if self.scalar_static_bool[27]{(v84*v964)}else{v304});
        let v1153=ctx.node_voltage(nodes[0]);
        let v1159=(v345/v11);
        let v1160=scalar_limexp(v1159);
        let v1161=(v347/v11);
        let v1162=scalar_limexp(v1161);
        let v1165=((v51+(v294*v1160))).sqrt();
        let v1168=((v51+(v294*v1162))).sqrt();
        let v1201=ctx.node_voltage(nodes[1]);
        let v1215=ctx.node_voltage(nodes[2]);
        let v1257=(if (v905>v304){v51}else{v304});
        let v1259=(self.scalar_static_f64[117]*(v905*v1257));
        let v1260=(v51+v1259);
        let v1261=(v1259/v1260);
        let v1266=(self.scalar_static_f64[163]*(v51+(v925*self.scalar_static_f64[164])));
        let v1270=((self.scalar_static_f64[114]*v345)/1.44);
        let v1272=(self.scalar_static_f64[165]*scalar_limexp(v1270));
        let v1274=(self.scalar_static_f64[118]+(v1261*v1261));
        let v1277=(v51+(v1257*(v1272*v1274)));
        let v1278=(v1266*v1277);
        let v1281=(v905*v1278);
        let v1283=((self.scalar_static_f64[158]*(v279*v428))+(v1281/v946));
        let v1285=(self.scalar_static_f64[160]*(v279*v488));
        let v1292=(((v284*v671)+(v911*self.scalar_static_f64[166]))+(v1165*self.scalar_static_f64[167]));
        let v1293=(v1168*self.scalar_static_f64[167]);
        let v1296=((v286*v816)+((if self.scalar_static_bool[28]{v304}else{v966})*self.scalar_static_f64[166]));
        let v1300=((v291*v900)+(v825*self.scalar_static_f64[168]));
        let v1303=((v1201-v1215)*self.scalar_static_f64[169]);
        let v1306=((v1201-v1153)*self.scalar_static_f64[170]);
        let v1344=(v6*self.scalar_static_f64[172]);
        let v1345=8.617342301212761e-5;
        let v1390=(self.scalar_static_f64[173]*(self.scalar_static_f64[20]*f64::powf(v12,self.scalar_static_f64[182])));
        let v1396=(v11*v11);
        let v1398=(v55*(((v11*self.scalar_static_f64[184])-(v53*v1345))/v1396));
        let v1406=(self.scalar_static_f64[19]*(((v55*v1390)+(v48*v1398))*(self.scalar_static_f64[24]*f64::powf(v56,self.scalar_static_f64[185]))));
        let v1438=(self.scalar_static_f64[31]*(((v79*v1390)+(v48*(v79*(((v11*self.scalar_static_f64[189])-(v77*v1345))/v1396))))*(self.scalar_static_f64[35]*f64::powf(v80,self.scalar_static_f64[190]))));
        let v1544=(v182*(((v12*v1345)-(v11*self.scalar_static_f64[173]))/(v12*v12)));
        let v1569=((v202*0.00025852026903638284)+(v201*(self.scalar_static_f64[173]/v12)));
        let v1572=((((v198*self.scalar_static_f64[173])+(v12*((v197*v1544)+(v184*(((v190*(((v11*self.scalar_static_f64[209])-(v188*v1345))/v1396))-(v195*(((v11*self.scalar_static_f64[210])-(v193*v1345))/v1396)))/v196)))))-v1569)-self.scalar_static_f64[211]);
        let v1573=0.00017234684602425522;
        let v1588=(v1572+((v218*v1573)+(v208*((v185*((v209*(v212*(((v11*(-v1572))-(v210*v1345))/v1396)))/(v182*v215)))/v217))));
        let v1611=((((v232*self.scalar_static_f64[173])+(v12*((v231*v1544)+(v184*(((v225*(((v11*self.scalar_static_f64[212])-(v223*v1345))/v1396))-(v229*(((v11*self.scalar_static_f64[213])-(v227*v1345))/v1396)))/v230)))))-v1569)-self.scalar_static_f64[214]);
        let v1626=(v1611+((v245*v1573)+(v208*((v185*((v209*(v239*(((v11*(-v1611))-(v237*v1345))/v1396)))/(v182*v242)))/v244))));
        let v1649=((((v259*self.scalar_static_f64[173])+(v12*((v258*v1544)+(v184*(((v252*(((v11*self.scalar_static_f64[215])-(v250*v1345))/v1396))-(v256*(((v11*self.scalar_static_f64[216])-(v254*v1345))/v1396)))/v257)))))-v1569)-self.scalar_static_f64[217]);
        let v1664=(v1649+((v272*v1573)+(v208*((v185*((v209*(v266*(((v11*(-v1649))-(v264*v1345))/v1396)))/(v182*v269)))/v271))));
        let v1667=(v220*v220);
        let v1673=(self.scalar_static_f64[87]*(((-(self.scalar_static_f64[78]*v1588))/v1667)*(self.scalar_static_f64[88]*f64::powf(v276,self.scalar_static_f64[218]))));
        let v1676=(v247*v247);
        let v1680=(((-(self.scalar_static_f64[81]*v1626))/v1676)*(self.scalar_static_f64[90]*f64::powf(v281,self.scalar_static_f64[162])));
        let v1685=(v274*v274);
        let v1695=((v293*v1398)+(v55*(self.scalar_static_f64[94]*v1390)));
        let v1719=(-v1588);
        let v1720=(self.scalar_static_f64[119]*v1719);
        let v1721=(if self.scalar_static_bool[9]{v1720}else{v304});
        let v1734=(self.scalar_static_f64[223]/v220);
        let v1757=(-(v51/v220));
        let v1758=(-(v360/v220));
        let v1761=(self.scalar_static_f64[124]*f64::powf(v381,self.scalar_static_f64[225]));
        let v1776=(if v379{(((v383*v1588)+(v220*(-((-((-(v341*v1588))/v1667))*v1761))))/self.scalar_static_f64[124])}else{(if v358{((v366*v1588)/self.scalar_static_f64[124])}else{v304})});
        let v1777=(if v379{((v220*(-(v1757*v1761)))/self.scalar_static_f64[124])}else{v304});
        let v1778=(if v379{((v220*(-(v1758*v1761)))/self.scalar_static_f64[124])}else{v304});
        let v1779=(if v379{v304}else{(if v358{(v363*((v374*v1721)+(v356*(((v220*(self.scalar_static_f64[125]*v1721))-(v372*v1588))/v1667))))}else{v304})});
        let v1780=(if v379{v304}else{(if v358{(v363*((v374*self.scalar_static_f64[221])+(v356*v1734)))}else{v304})});
        let v1781=(if v379{v304}else{(if v358{(v363*((v374*self.scalar_static_f64[222])+(v356*(self.scalar_static_f64[224]/v220))))}else{v304})});
        let v1788=(v352*v1720);
        let v1791=((v1788+v1788)/(v182*v395));
        let v1792=(if self.scalar_static_bool[10]{v1791}else{v304});
        let v1795=(if self.scalar_static_bool[10]{(v191*(v1720+v1792))}else{v304});
        let v1808=(if self.scalar_static_bool[10]{(((v402*v1719)+(v350*((-(((v220*v1795)-(v399*v1588))/v1667))*(self.scalar_static_f64[124]*f64::powf(v401,self.scalar_static_f64[225])))))/self.scalar_static_f64[124])}else{v304});
        let v1809=(if self.scalar_static_bool[10]{v1720}else{v304});
        let v1812=(v406*v1809);
        let v1814=(v406*self.scalar_static_f64[226]);
        let v1816=(v406*self.scalar_static_f64[227]);
        let v1818=(v182*v409);
        let v1822=(if self.scalar_static_bool[10]{((v1812+v1812)/v1818)}else{v304});
        let v1823=(if self.scalar_static_bool[10]{((v1814+v1814)/v1818)}else{v304});
        let v1824=(if self.scalar_static_bool[10]{((v1816+v1816)/v1818)}else{v304});
        let v1832=(if self.scalar_static_bool[10]{((v185*(v1809-v1822))-v1720)}else{v304});
        let v1833=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[226]-v1823))}else{v304});
        let v1834=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[227]-v1824))}else{v304});
        let v1845=(self.scalar_static_f64[124]*f64::powf(v416,self.scalar_static_f64[225]));
        let v1857=(if self.scalar_static_bool[10]{(((v417*v1719)+(v350*((-(((v220*v1832)-(v414*v1588))/v1667))*v1845)))/self.scalar_static_f64[124])}else{v1776});
        let v1858=(if self.scalar_static_bool[10]{((v350*((-(v1833/v220))*v1845))/self.scalar_static_f64[124])}else{v1777});
        let v1859=(if self.scalar_static_bool[10]{((v350*((-(v1834/v220))*v1845))/self.scalar_static_f64[124])}else{v1778});
        let v1871=(if self.scalar_static_bool[10]{((v1857+(self.scalar_static_f64[129]*(v1795+(-v1832))))-v1808)}else{(if self.scalar_static_bool[9]{(v1776+v1779)}else{v304})});
        let v1872=(if self.scalar_static_bool[10]{(v1858+(self.scalar_static_f64[129]*(v51-v1833)))}else{(if self.scalar_static_bool[9]{(v1777+v1780)}else{v304})});
        let v1873=(if self.scalar_static_bool[10]{(v1859+(self.scalar_static_f64[129]*(v360-v1834)))}else{(if self.scalar_static_bool[9]{(v1778+v1781)}else{v304})});
        let v1874=(if self.scalar_static_bool[9]{v1720}else{v1721});
        let v1916=(self.scalar_static_f64[124]*f64::powf(v449,self.scalar_static_f64[225]));
        let v1931=(if v447{(((v451*v1588)+(v220*(-((-((-(v343*v1588))/v1667))*v1916))))/self.scalar_static_f64[124])}else{(if v432{((v436*v1588)/self.scalar_static_f64[124])}else{v1857})});
        let v1932=(if v447{((v220*(-(v1757*v1916)))/self.scalar_static_f64[124])}else{v304});
        let v1933=(if v447{v304}else{(if v432{v304}else{v1858})});
        let v1934=(if v447{((v220*(-(v1758*v1916)))/self.scalar_static_f64[124])}else{(if v432{v304}else{v1859})});
        let v1935=(if v447{v304}else{(if v432{(v433*((v442*v1874)+(v430*(((v220*(self.scalar_static_f64[125]*v1874))-(v440*v1588))/v1667))))}else{v1779})});
        let v1936=(if v447{v304}else{(if v432{(v433*((v442*self.scalar_static_f64[221])+(v430*v1734)))}else{v304})});
        let v1937=(if v447{v304}else{(if v432{(v433*((v442*self.scalar_static_f64[228])+(v430*(self.scalar_static_f64[230]/v220))))}else{v1780})});
        let v1938=(if v447{v304}else{(if v432{(v433*((v442*self.scalar_static_f64[229])+(v430*(self.scalar_static_f64[231]/v220))))}else{v1781})});
        let v1947=(if self.scalar_static_bool[10]{v1791}else{v1792});
        let v1950=(if self.scalar_static_bool[10]{(v191*(v1720+v1947))}else{v1795});
        let v1963=(if self.scalar_static_bool[10]{(((v464*v1719)+(v350*((-(((v220*v1950)-(v461*v1588))/v1667))*(self.scalar_static_f64[124]*f64::powf(v463,self.scalar_static_f64[225])))))/self.scalar_static_f64[124])}else{v1808});
        let v1964=(if self.scalar_static_bool[10]{v1720}else{v1809});
        let v1967=(v468*v1964);
        let v1969=(v468*self.scalar_static_f64[226]);
        let v1971=(v468*self.scalar_static_f64[232]);
        let v1973=(v468*self.scalar_static_f64[233]);
        let v1975=(v182*v471);
        let v1980=(if self.scalar_static_bool[10]{((v1967+v1967)/v1975)}else{v1822});
        let v1981=(if self.scalar_static_bool[10]{((v1969+v1969)/v1975)}else{v304});
        let v1982=(if self.scalar_static_bool[10]{((v1971+v1971)/v1975)}else{v1823});
        let v1983=(if self.scalar_static_bool[10]{((v1973+v1973)/v1975)}else{v1824});
        let v1993=(if self.scalar_static_bool[10]{((v185*(v1964-v1980))-v1720)}else{v1832});
        let v1994=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[226]-v1981))}else{v304});
        let v1995=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[232]-v1982))}else{v1833});
        let v1996=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[233]-v1983))}else{v1834});
        let v2009=(self.scalar_static_f64[124]*f64::powf(v478,self.scalar_static_f64[225]));
        let v2024=(if self.scalar_static_bool[10]{(((v479*v1719)+(v350*((-(((v220*v1993)-(v476*v1588))/v1667))*v2009)))/self.scalar_static_f64[124])}else{v1931});
        let v2025=(if self.scalar_static_bool[10]{((v350*((-(v1994/v220))*v2009))/self.scalar_static_f64[124])}else{v1932});
        let v2026=(if self.scalar_static_bool[10]{((v350*((-(v1995/v220))*v2009))/self.scalar_static_f64[124])}else{v1933});
        let v2027=(if self.scalar_static_bool[10]{((v350*((-(v1996/v220))*v2009))/self.scalar_static_f64[124])}else{v1934});
        let v2046=(-v1626);
        let v2047=(self.scalar_static_f64[119]*v2046);
        let v2048=(if self.scalar_static_bool[11]{v2047}else{v1874});
        let v2068=(self.scalar_static_f64[238]/v247);
        let v2099=((-(self.scalar_static_f64[135]*v1626))/v1676);
        let v2103=(v2099*(self.scalar_static_f64[133]*f64::powf(v523,self.scalar_static_f64[242])));
        let v2107=(v527*v527);
        let v2128=((v247*(-(v524*(-(self.scalar_static_f64[243]/v527)))))/self.scalar_static_f64[133]);
        let v2129=((v247*(-(v524*(-(self.scalar_static_f64[133]/v527)))))/self.scalar_static_f64[133]);
        let v2141=(-(v360/v247));
        let v2142=(-(v51/v247));
        let v2144=(self.scalar_static_f64[133]*f64::powf(v538,self.scalar_static_f64[242]));
        let v2159=(if v536{(((v540*v1626)+(v247*(-((-((-(v345*v1626))/v1676))*v2144))))/self.scalar_static_f64[133])}else{(if v521{(((v531*v1626)+(v247*(-((v529*v2103)+(v524*(-((-(v526*v1626))/v2107)))))))/self.scalar_static_f64[133])}else{(if v496{((v502*v1626)/self.scalar_static_f64[133])}else{v2024})})});
        let v2160=(if v536{((v247*(-(v2141*v2144)))/self.scalar_static_f64[133])}else{(if v521{v2128}else{v304})});
        let v2161=(if v536{v304}else{(if v521{v304}else{(if v496{v304}else{v2025})})});
        let v2162=(if v536{((v247*(-(v2142*v2144)))/self.scalar_static_f64[133])}else{(if v521{v2129}else{(if v496{v304}else{v2026})})});
        let v2163=(if v536{v304}else{(if v521{v304}else{(if v496{v304}else{v2027})})});
        let v2164=(if v520{v304}else{(if v496{(v499*((v510*v2048)+(v494*(((v247*(self.scalar_static_f64[134]*v2048))-(v508*v1626))/v1676))))}else{v1935})});
        let v2165=(if v520{v304}else{(if v496{(v499*((v510*self.scalar_static_f64[234])+(v494*v2068)))}else{v304})});
        let v2166=(if v520{v304}else{(if v496{(v499*((v510*self.scalar_static_f64[235])+(v494*(self.scalar_static_f64[239]/v247))))}else{v1936})});
        let v2167=(if v520{v304}else{(if v496{(v499*((v510*self.scalar_static_f64[236])+(v494*(self.scalar_static_f64[240]/v247))))}else{v1937})});
        let v2168=(if v520{v304}else{(if v496{(v499*((v510*self.scalar_static_f64[237])+(v494*(self.scalar_static_f64[241]/v247))))}else{v1938})});
        let v2179=(-v2047);
        let v2180=(v553*v2047);
        let v2183=(v553*v553);
        let v2184=((v2180-(v552*v2179))/v2183);
        let v2185=(if self.scalar_static_bool[16]{v2184}else{v304});
        let v2187=(v557*v2185);
        let v2191=(v563*v2185);
        let v2201=(if self.scalar_static_bool[16]{(((v569*(v182*v2185))-(v556*(((v2187+v2187)/(v182*v562))+((v2191+v2191)/(v182*v568)))))/(v569*v569))}else{v304});
        let v2207=(if self.scalar_static_bool[16]{(v185*(((v571*v2179)+(v553*v2201))-v2047))}else{v1950});
        let v2221=(if self.scalar_static_bool[16]{(((v580*v1626)+(v247*(-((-(((v247*v2207)-(v576*v1626))/v1676))*(self.scalar_static_f64[133]*f64::powf(v578,self.scalar_static_f64[242]))))))/self.scalar_static_f64[133])}else{v304});
        let v2228=(if self.scalar_static_bool[16]{((v2180-(v586*v2179))/v2183)}else{v304});
        let v2229=(if self.scalar_static_bool[16]{(-2.0/v553)}else{v304});
        let v2230=(if self.scalar_static_bool[16]{(v182/v553)}else{v304});
        let v2232=(v182*v2229);
        let v2233=(v182*v2230);
        let v2234=(v590*v2228);
        let v2236=(v590*v2229);
        let v2238=(v590*v2230);
        let v2240=(v182*v593);
        let v2244=(v594*v2228);
        let v2246=(v594*v2229);
        let v2248=(v594*v2230);
        let v2250=(v182*v597);
        let v2260=(v598*v598);
        let v2270=(if self.scalar_static_bool[16]{(((v598*(v182*v2228))-(v589*(((v2234+v2234)/v2240)+((v2244+v2244)/v2250))))/v2260)}else{v304});
        let v2271=(if self.scalar_static_bool[16]{(((v598*v2232)-(v589*(((v2236+v2236)/v2240)+((v2246+v2246)/v2250))))/v2260)}else{v304});
        let v2272=(if self.scalar_static_bool[16]{(((v598*v2233)-(v589*(((v2238+v2238)/v2240)+((v2248+v2248)/v2250))))/v2260)}else{v304});
        let v2282=(if self.scalar_static_bool[16]{(v185*(((v600*v2179)+(v553*v2270))-v2047))}else{v1993});
        let v2283=(if self.scalar_static_bool[16]{(v185*(v553*v2271))}else{v304});
        let v2284=(if self.scalar_static_bool[16]{v304}else{v1994});
        let v2285=(if self.scalar_static_bool[16]{(v185*(v553*v2272))}else{v1995});
        let v2286=(if self.scalar_static_bool[16]{v304}else{v1996});
        let v2301=(self.scalar_static_f64[133]*f64::powf(v607,self.scalar_static_f64[242]));
        let v2324=(if self.scalar_static_bool[16]{(((v609*v1626)+(v247*(-((-(((v247*v2282)-(v605*v1626))/v1676))*v2301))))/self.scalar_static_f64[133])}else{v2159});
        let v2325=(if self.scalar_static_bool[16]{((v247*(-((-(v2283/v247))*v2301)))/self.scalar_static_f64[133])}else{v2160});
        let v2326=(if self.scalar_static_bool[16]{((v247*(-((-(v2284/v247))*v2301)))/self.scalar_static_f64[133])}else{v2161});
        let v2327=(if self.scalar_static_bool[16]{((v247*(-((-(v2285/v247))*v2301)))/self.scalar_static_f64[133])}else{v2162});
        let v2328=(if self.scalar_static_bool[16]{((v247*(-((-(v2286/v247))*v2301)))/self.scalar_static_f64[133])}else{v2163});
        let v2332=(if self.scalar_static_bool[16]{(v185*v2270)}else{v304});
        let v2333=(if self.scalar_static_bool[16]{(v185*v2271)}else{v304});
        let v2334=(if self.scalar_static_bool[16]{(v185*v2272)}else{v304});
        let v2338=(v2099*(self.scalar_static_f64[142]*f64::powf(v523,self.scalar_static_f64[244])));
        let v2339=(if self.scalar_static_bool[16]{v2338}else{v304});
        let v2346=((((v247*v2047)-(v490*v1626))/v1676)*(self.scalar_static_f64[142]*f64::powf(v620,self.scalar_static_f64[244])));
        let v2347=(if self.scalar_static_bool[16]{v2346}else{v304});
        let v2364=(if self.scalar_static_bool[16]{(((v623*v2339)+(v618*(-v2332)))+((v622*v2332)+(v615*v2347)))}else{v304});
        let v2365=(if self.scalar_static_bool[16]{((v618*(-v2333))+(v622*v2333))}else{v304});
        let v2366=(if self.scalar_static_bool[16]{((v618*(-v2334))+(v622*v2334))}else{v304});
        let v2384=(if self.scalar_static_bool[16]{((v629*v2364)+(v627*(v2207+(-v2282))))}else{v304});
        let v2385=(if self.scalar_static_bool[16]{((v629*v2365)+(v627*(v360-v2283)))}else{v304});
        let v2386=(if self.scalar_static_bool[16]{(v627*(-v2284))}else{v304});
        let v2387=(if self.scalar_static_bool[16]{((v629*v2366)+(v627*(v51-v2285)))}else{v304});
        let v2388=(if self.scalar_static_bool[16]{(v627*(-v2286))}else{v304});
        let v2400=(v490*v2047);
        let v2403=((v2400+v2400)/(v182*v639));
        let v2404=(if self.scalar_static_bool[18]{v2403}else{v1947});
        let v2407=(if self.scalar_static_bool[18]{(v191*(v2047+v2404))}else{v2207});
        let v2420=(if self.scalar_static_bool[18]{(((v646*v2046)+(v489*((-(((v247*v2407)-(v643*v1626))/v1676))*(self.scalar_static_f64[133]*f64::powf(v645,self.scalar_static_f64[242])))))/self.scalar_static_f64[133])}else{v1963});
        let v2421=(if self.scalar_static_bool[18]{v2047}else{v1964});
        let v2426=(v650*v2421);
        let v2428=(v650*self.scalar_static_f64[245]);
        let v2430=(v650*self.scalar_static_f64[246]);
        let v2432=(v650*self.scalar_static_f64[247]);
        let v2434=(v650*self.scalar_static_f64[248]);
        let v2436=(v182*v653);
        let v2442=(if self.scalar_static_bool[18]{((v2426+v2426)/v2436)}else{v1980});
        let v2443=(if self.scalar_static_bool[18]{((v2428+v2428)/v2436)}else{v304});
        let v2444=(if self.scalar_static_bool[18]{((v2430+v2430)/v2436)}else{v1981});
        let v2445=(if self.scalar_static_bool[18]{((v2432+v2432)/v2436)}else{v1982});
        let v2446=(if self.scalar_static_bool[18]{((v2434+v2434)/v2436)}else{v1983});
        let v2458=(if self.scalar_static_bool[18]{((v185*(v2421-v2442))-v2047)}else{v2282});
        let v2459=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[245]-v2443))}else{v2283});
        let v2460=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[246]-v2444))}else{v2284});
        let v2461=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[247]-v2445))}else{v2285});
        let v2462=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[248]-v2446))}else{v2286});
        let v2477=(self.scalar_static_f64[133]*f64::powf(v660,self.scalar_static_f64[242]));
        let v2495=(if self.scalar_static_bool[18]{(((v661*v2046)+(v489*((-(((v247*v2458)-(v658*v1626))/v1676))*v2477)))/self.scalar_static_f64[133])}else{v2324});
        let v2496=(if self.scalar_static_bool[18]{((v489*((-(v2459/v247))*v2477))/self.scalar_static_f64[133])}else{v2325});
        let v2497=(if self.scalar_static_bool[18]{((v489*((-(v2460/v247))*v2477))/self.scalar_static_f64[133])}else{v2326});
        let v2498=(if self.scalar_static_bool[18]{((v489*((-(v2461/v247))*v2477))/self.scalar_static_f64[133])}else{v2327});
        let v2499=(if self.scalar_static_bool[18]{((v489*((-(v2462/v247))*v2477))/self.scalar_static_f64[133])}else{v2328});
        let v2517=(if self.scalar_static_bool[18]{((v2495+(self.scalar_static_f64[143]*(v2407+(-v2458))))-v2420)}else{(if self.scalar_static_bool[16]{((v2324+v2384)-v2221)}else{(if self.scalar_static_bool[11]{(v2159+v2164)}else{v304})})});
        let v2518=(if self.scalar_static_bool[18]{(v2496+(self.scalar_static_f64[143]*(v360-v2459)))}else{(if self.scalar_static_bool[16]{(v2325+v2385)}else{(if self.scalar_static_bool[11]{(v2160+v2165)}else{v304})})});
        let v2519=(if self.scalar_static_bool[18]{(v2497+(self.scalar_static_f64[143]*(-v2460)))}else{(if self.scalar_static_bool[16]{(v2326+v2386)}else{(if self.scalar_static_bool[11]{(v2161+v2166)}else{v304})})});
        let v2520=(if self.scalar_static_bool[18]{(v2498+(self.scalar_static_f64[143]*(v51-v2461)))}else{(if self.scalar_static_bool[16]{(v2327+v2387)}else{(if self.scalar_static_bool[11]{(v2162+v2167)}else{v304})})});
        let v2521=(if self.scalar_static_bool[18]{(v2499+(self.scalar_static_f64[143]*(-v2462)))}else{(if self.scalar_static_bool[16]{(v2328+v2388)}else{(if self.scalar_static_bool[11]{(v2163+v2168)}else{v304})})});
        let v2522=(if self.scalar_static_bool[11]{v2047}else{v2048});
        let v2600=(self.scalar_static_f64[133]*f64::powf(v706,self.scalar_static_f64[242]));
        let v2615=(if v704{(((v708*v1626)+(v247*(-((-((-(v349*v1626))/v1676))*v2600))))/self.scalar_static_f64[133])}else{(if v693{(((v699*v1626)+(v247*(-((v697*v2103)+(v524*(-((-(v695*v1626))/v2107)))))))/self.scalar_static_f64[133])}else{(if v675{((v679*v1626)/self.scalar_static_f64[133])}else{v2495})})});
        let v2616=(if v704{v304}else{(if v693{v304}else{(if v675{v304}else{v2496})})});
        let v2617=(if v704{((v247*(-(v2142*v2600)))/self.scalar_static_f64[133])}else{(if v693{v2129}else{(if v675{v304}else{v2497})})});
        let v2618=(if v704{v304}else{(if v693{v304}else{(if v675{v304}else{v2498})})});
        let v2619=(if v704{v304}else{(if v693{v304}else{(if v675{v304}else{v2499})})});
        let v2620=(if v704{((v247*(-(v2141*v2600)))/self.scalar_static_f64[133])}else{(if v693{v2128}else{v304})});
        let v2621=(if v692{v304}else{(if v675{(v676*((v685*v2522)+(v673*(((v247*(self.scalar_static_f64[134]*v2522))-(v683*v1626))/v1676))))}else{v2164})});
        let v2622=(if v692{v304}else{(if v675{(v676*((v685*self.scalar_static_f64[249])+(v673*(self.scalar_static_f64[253]/v247))))}else{v2165})});
        let v2623=(if v692{v304}else{(if v675{(v676*((v685*self.scalar_static_f64[250])+(v673*(self.scalar_static_f64[254]/v247))))}else{v2166})});
        let v2624=(if v692{v304}else{(if v675{(v676*((v685*self.scalar_static_f64[251])+(v673*(self.scalar_static_f64[255]/v247))))}else{v2167})});
        let v2625=(if v692{v304}else{(if v675{(v676*((v685*self.scalar_static_f64[252])+(v673*(self.scalar_static_f64[256]/v247))))}else{v2168})});
        let v2626=(if v692{v304}else{(if v675{(v676*((v685*self.scalar_static_f64[234])+(v673*v2068)))}else{v304})});
        let v2639=(if self.scalar_static_bool[16]{v2184}else{v2185});
        let v2641=(v717*v2639);
        let v2645=(v721*v2639);
        let v2661=(if self.scalar_static_bool[16]{(v185*(((v727*v2179)+(v553*(if self.scalar_static_bool[16]{(((v725*(v182*v2639))-(v716*(((v2641+v2641)/(v182*v720))+((v2645+v2645)/(v182*v724)))))/(v725*v725))}else{v2201})))-v2047))}else{v2407});
        let v2679=(if self.scalar_static_bool[16]{((v2180-(v742*v2179))/v2183)}else{v2228});
        let v2680=(if self.scalar_static_bool[16]{v304}else{v2229});
        let v2681=(if self.scalar_static_bool[16]{v304}else{v2230});
        let v2685=(v746*v2679);
        let v2687=(v746*v2680);
        let v2689=(v746*v2230);
        let v2691=(v746*v2681);
        let v2693=(v746*v2229);
        let v2695=(v182*v749);
        let v2701=(v750*v2679);
        let v2703=(v750*v2680);
        let v2705=(v750*v2230);
        let v2707=(v750*v2681);
        let v2709=(v750*v2229);
        let v2711=(v182*v753);
        let v2725=(v754*v754);
        let v2743=(if self.scalar_static_bool[16]{(((v754*(v182*v2679))-(v745*(((v2685+v2685)/v2695)+((v2701+v2701)/v2711))))/v2725)}else{v2270});
        let v2744=(if self.scalar_static_bool[16]{(((v754*(v182*v2680))-(v745*(((v2687+v2687)/v2695)+((v2703+v2703)/v2711))))/v2725)}else{v2271});
        let v2745=(if self.scalar_static_bool[16]{(((v754*v2233)-(v745*(((v2689+v2689)/v2695)+((v2705+v2705)/v2711))))/v2725)}else{v304});
        let v2746=(if self.scalar_static_bool[16]{(((v754*(v182*v2681))-(v745*(((v2691+v2691)/v2695)+((v2707+v2707)/v2711))))/v2725)}else{v2272});
        let v2747=(if self.scalar_static_bool[16]{(((v754*v2232)-(v745*(((v2693+v2693)/v2695)+((v2709+v2709)/v2711))))/v2725)}else{v304});
        let v2761=(if self.scalar_static_bool[16]{(v185*(((v756*v2179)+(v553*v2743))-v2047))}else{v2458});
        let v2762=(if self.scalar_static_bool[16]{(v185*(v553*v2744))}else{v2459});
        let v2763=(if self.scalar_static_bool[16]{(v185*(v553*v2745))}else{v2460});
        let v2764=(if self.scalar_static_bool[16]{(v185*(v553*v2746))}else{v2461});
        let v2765=(if self.scalar_static_bool[16]{v304}else{v2462});
        let v2766=(if self.scalar_static_bool[16]{(v185*(v553*v2747))}else{v304});
        let v2783=(self.scalar_static_f64[133]*f64::powf(v763,self.scalar_static_f64[242]));
        let v2810=(if self.scalar_static_bool[16]{(((v765*v1626)+(v247*(-((-(((v247*v2761)-(v761*v1626))/v1676))*v2783))))/self.scalar_static_f64[133])}else{v2615});
        let v2811=(if self.scalar_static_bool[16]{((v247*(-((-(v2762/v247))*v2783)))/self.scalar_static_f64[133])}else{v2616});
        let v2812=(if self.scalar_static_bool[16]{((v247*(-((-(v2763/v247))*v2783)))/self.scalar_static_f64[133])}else{v2617});
        let v2813=(if self.scalar_static_bool[16]{((v247*(-((-(v2764/v247))*v2783)))/self.scalar_static_f64[133])}else{v2618});
        let v2814=(if self.scalar_static_bool[16]{((v247*(-((-(v2765/v247))*v2783)))/self.scalar_static_f64[133])}else{v2619});
        let v2815=(if self.scalar_static_bool[16]{((v247*(-((-(v2766/v247))*v2783)))/self.scalar_static_f64[133])}else{v2620});
        let v2821=(if self.scalar_static_bool[16]{(v185*v2743)}else{v2332});
        let v2822=(if self.scalar_static_bool[16]{(v185*v2744)}else{v2333});
        let v2823=(if self.scalar_static_bool[16]{(v185*v2745)}else{v304});
        let v2824=(if self.scalar_static_bool[16]{(v185*v2746)}else{v2334});
        let v2825=(if self.scalar_static_bool[16]{(v185*v2747)}else{v304});
        let v2899=(if self.scalar_static_bool[18]{v2403}else{v2404});
        let v2902=(if self.scalar_static_bool[18]{(v191*(v2047+v2899))}else{v2661});
        let v2915=(if self.scalar_static_bool[18]{(((v792*v2046)+(v489*((-(((v247*v2902)-(v789*v1626))/v1676))*(self.scalar_static_f64[133]*f64::powf(v791,self.scalar_static_f64[242])))))/self.scalar_static_f64[133])}else{v2420});
        let v2916=(if self.scalar_static_bool[18]{v2047}else{v2421});
        let v2921=(v796*v2916);
        let v2923=(v796*self.scalar_static_f64[257]);
        let v2925=(v796*self.scalar_static_f64[258]);
        let v2927=(v796*self.scalar_static_f64[259]);
        let v2929=(v796*self.scalar_static_f64[260]);
        let v2931=(v796*self.scalar_static_f64[245]);
        let v2933=(v182*v799);
        let v2940=(if self.scalar_static_bool[18]{((v2921+v2921)/v2933)}else{v2442});
        let v2941=(if self.scalar_static_bool[18]{((v2923+v2923)/v2933)}else{v2443});
        let v2942=(if self.scalar_static_bool[18]{((v2925+v2925)/v2933)}else{v2444});
        let v2943=(if self.scalar_static_bool[18]{((v2927+v2927)/v2933)}else{v2445});
        let v2944=(if self.scalar_static_bool[18]{((v2929+v2929)/v2933)}else{v2446});
        let v2945=(if self.scalar_static_bool[18]{((v2931+v2931)/v2933)}else{v304});
        let v2959=(if self.scalar_static_bool[18]{((v185*(v2916-v2940))-v2047)}else{v2761});
        let v2960=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[257]-v2941))}else{v2762});
        let v2961=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[258]-v2942))}else{v2763});
        let v2962=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[259]-v2943))}else{v2764});
        let v2963=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[260]-v2944))}else{v2765});
        let v2964=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[245]-v2945))}else{v2766});
        let v2981=(self.scalar_static_f64[133]*f64::powf(v806,self.scalar_static_f64[242]));
        let v3002=(if self.scalar_static_bool[18]{(((v807*v2046)+(v489*((-(((v247*v2959)-(v804*v1626))/v1676))*v2981)))/self.scalar_static_f64[133])}else{v2810});
        let v3003=(if self.scalar_static_bool[18]{((v489*((-(v2960/v247))*v2981))/self.scalar_static_f64[133])}else{v2811});
        let v3004=(if self.scalar_static_bool[18]{((v489*((-(v2961/v247))*v2981))/self.scalar_static_f64[133])}else{v2812});
        let v3005=(if self.scalar_static_bool[18]{((v489*((-(v2962/v247))*v2981))/self.scalar_static_f64[133])}else{v2813});
        let v3006=(if self.scalar_static_bool[18]{((v489*((-(v2963/v247))*v2981))/self.scalar_static_f64[133])}else{v2814});
        let v3007=(if self.scalar_static_bool[18]{((v489*((-(v2964/v247))*v2981))/self.scalar_static_f64[133])}else{v2815});
        let v3034=(-v1664);
        let v3036=(if self.scalar_static_bool[19]{(self.scalar_static_f64[119]*v3034)}else{v2047});
        let v3037=(if self.scalar_static_bool[21]{v3036}else{v2522});
        let v3114=(self.scalar_static_f64[147]*f64::powf(v850,self.scalar_static_f64[273]));
        let v3129=(if v848{(((v852*v1664)+(v274*(-((-((-(v825*v1664))/v1685))*v3114))))/self.scalar_static_f64[147])}else{(if v829{((v835*v1664)/self.scalar_static_f64[147])}else{v3002})});
        let v3130=(if v848{v304}else{(if v829{v304}else{v3003})});
        let v3131=(if v848{v304}else{(if v829{v304}else{v3004})});
        let v3132=(if v848{v304}else{(if v829{v304}else{v3005})});
        let v3133=(if v848{v304}else{(if v829{v304}else{v3006})});
        let v3134=(if v848{((v274*(-((-(v360/v274))*v3114)))/self.scalar_static_f64[147])}else{(if v829{v304}else{v3007})});
        let v3135=(if v848{((v274*(-((-(v51/v274))*v3114)))/self.scalar_static_f64[147])}else{v304});
        let v3157=(v820*v3036);
        let v3164=(if self.scalar_static_bool[23]{(v191*(v3036+(if self.scalar_static_bool[23]{((v3157+v3157)/(v182*v865))}else{v2899})))}else{v2902});
        let v3178=(if self.scalar_static_bool[23]{v3036}else{v2916});
        let v3185=(v876*v3178);
        let v3187=(v876*self.scalar_static_f64[274]);
        let v3189=(v876*self.scalar_static_f64[275]);
        let v3191=(v876*self.scalar_static_f64[276]);
        let v3193=(v876*self.scalar_static_f64[277]);
        let v3195=(v876*self.scalar_static_f64[278]);
        let v3197=(v876*self.scalar_static_f64[279]);
        let v3199=(v182*v879);
        let v3229=(if self.scalar_static_bool[23]{((v185*(v3178-(if self.scalar_static_bool[23]{((v3185+v3185)/v3199)}else{v2940})))-v3036)}else{v2959});
        let v3230=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[274]-(if self.scalar_static_bool[23]{((v3187+v3187)/v3199)}else{v2941})))}else{v2960});
        let v3231=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[275]-(if self.scalar_static_bool[23]{((v3189+v3189)/v3199)}else{v2942})))}else{v2961});
        let v3232=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[276]-(if self.scalar_static_bool[23]{((v3191+v3191)/v3199)}else{v2943})))}else{v2962});
        let v3233=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[277]-(if self.scalar_static_bool[23]{((v3193+v3193)/v3199)}else{v2944})))}else{v2963});
        let v3234=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[278]-(if self.scalar_static_bool[23]{((v3195+v3195)/v3199)}else{v2945})))}else{v2964});
        let v3235=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[279]-(if self.scalar_static_bool[23]{((v3197+v3197)/v3199)}else{v304})))}else{v304});
        let v3254=(self.scalar_static_f64[147]*f64::powf(v886,self.scalar_static_f64[273]));
        let v3331=scalar_limexp_derivative(v902);
        let v3337=((v904*v1406)+(v60*(((-(v341*((v162*v1345)+(v11*self.scalar_static_f64[205]))))/(v901*v901))*v3331)));
        let v3338=(v60*((v51/v901)*v3331));
        let v3339=(v60*((v360/v901)*v3331));
        let v3346=((-(v345*((v163*v1345)+(v11*self.scalar_static_f64[206]))))/(v906*v906));
        let v3347=(v360/v906);
        let v3348=(v51/v906);
        let v3349=scalar_limexp_derivative(v907);
        let v3350=(v3346*v3349);
        let v3351=(v3347*v3349);
        let v3352=(v3348*v3349);
        let v3358=((v910*((v73*v1406)+(v60*(self.scalar_static_f64[25]*(((v68*(self.scalar_static_f64[173]*(self.scalar_static_f64[26]*f64::powf(v12,self.scalar_static_f64[186]))))+(v63*(v68*(((v11*self.scalar_static_f64[187])-(v66*v1345))/v1396))))*(self.scalar_static_f64[30]*f64::powf(v69,self.scalar_static_f64[188])))))))+(v909*v3350));
        let v3359=(v909*v3351);
        let v3360=(v909*v3352);
        let v3365=(self.scalar_static_f64[99]*v2518);
        let v3366=(self.scalar_static_f64[99]*v2519);
        let v3369=((self.scalar_static_f64[102]*v1871)+(self.scalar_static_f64[99]*v2517));
        let v3370=((self.scalar_static_f64[102]*v1872)+(self.scalar_static_f64[99]*v2520));
        let v3371=((self.scalar_static_f64[102]*v1873)+(self.scalar_static_f64[99]*v2521));
        let v3372=(v917*v3369);
        let v3374=(v917*v3365);
        let v3376=(v917*v3366);
        let v3378=(v917*v3370);
        let v3380=(v917*v3371);
        let v3382=(v182*v921);
        let v3393=(v185*(v3369+((v3372+v3372)/v3382)));
        let v3394=(v185*(v3365+((v3374+v3374)/v3382)));
        let v3395=(v185*(v3366+((v3376+v3376)/v3382)));
        let v3396=(v185*(v3370+((v3378+v3378)/v3382)));
        let v3397=(v185*(v3371+((v3380+v3380)/v3382)));
        let v3410=(self.scalar_static_f64[155]*f64::powf(v925,self.scalar_static_f64[280]));
        let v3416=(v209*(((v905*(if self.scalar_static_bool[2]{((-(self.scalar_static_f64[3]*(self.scalar_static_f64[173]*(self.scalar_static_f64[4]*f64::powf(v12,self.scalar_static_f64[174])))))/(v17*v17))}else{v304}))+(v314*v3337))+(self.scalar_static_f64[105]*v3358)));
        let v3417=(v209*(self.scalar_static_f64[105]*v3359));
        let v3418=(v209*((v314*v3338)+(self.scalar_static_f64[105]*v3360)));
        let v3419=(v209*(v314*v3339));
        let v3426=(self.scalar_static_f64[154]*f64::powf(v935,self.scalar_static_f64[281]));
        let v3453=(self.scalar_static_f64[154]*f64::powf(v942,self.scalar_static_f64[281]));
        let v3471=(if self.scalar_static_bool[26]{((v944*(v185*v3393))+(v941*(v3416*v3453)))}else{(if self.scalar_static_bool[25]{(v185*(v3393+(((v3393*v3410)+v3416)*v3426)))}else{v304})});
        let v3472=(if self.scalar_static_bool[26]{((v944*(v185*v3394))+(v941*(v3417*v3453)))}else{(if self.scalar_static_bool[25]{(v185*(v3394+(((v3394*v3410)+v3417)*v3426)))}else{v304})});
        let v3473=(if self.scalar_static_bool[26]{(v944*(v185*v3395))}else{(if self.scalar_static_bool[25]{(v185*(v3395+((v3395*v3410)*v3426)))}else{v304})});
        let v3474=(if self.scalar_static_bool[26]{((v944*(v185*v3396))+(v941*(v3418*v3453)))}else{(if self.scalar_static_bool[25]{(v185*(v3396+(((v3396*v3410)+v3418)*v3426)))}else{v304})});
        let v3475=(if self.scalar_static_bool[26]{((v944*(v185*v3397))+(v941*(v3419*v3453)))}else{(if self.scalar_static_bool[25]{(v185*(v3397+(((v3397*v3410)+v3419)*v3426)))}else{v304})});
        let v3479=(v946*v946);
        let v3516=(v950*v950);
        let v3519=(v360/v950);
        let v3520=(if self.scalar_static_bool[27]{((-(v349*self.scalar_static_f64[282]))/v3516)}else{v3346});
        let v3521=(if self.scalar_static_bool[27]{v304}else{v3347});
        let v3522=(if self.scalar_static_bool[27]{(v51/v950)}else{v304});
        let v3523=(if self.scalar_static_bool[27]{v304}else{v3348});
        let v3524=(if self.scalar_static_bool[27]{v3519}else{v304});
        let v3525=scalar_limexp_derivative(v952);
        let v3531=(if self.scalar_static_bool[27]{(v3520*v3525)}else{v3350});
        let v3532=(if self.scalar_static_bool[27]{(v3521*v3525)}else{v3351});
        let v3533=(if self.scalar_static_bool[27]{(v3522*v3525)}else{v304});
        let v3534=(if self.scalar_static_bool[27]{(v3523*v3525)}else{v3352});
        let v3535=(if self.scalar_static_bool[27]{(v3524*v3525)}else{v304});
        let v3539=(if self.scalar_static_bool[27]{((-(v345*self.scalar_static_f64[282]))/v3516)}else{v304});
        let v3540=scalar_limexp_derivative(v956);
        let v3544=(if self.scalar_static_bool[27]{(v3539*v3540)}else{v304});
        let v3545=(if self.scalar_static_bool[27]{(v3524*v3540)}else{v304});
        let v3546=(if self.scalar_static_bool[27]{(v3522*v3540)}else{v304});
        let v3565=(if self.scalar_static_bool[27]{((v964*v1438)+(v84*((self.scalar_static_f64[156]*v3531)+(self.scalar_static_f64[157]*v3544))))}else{v304});
        let v3566=(if self.scalar_static_bool[27]{(v84*((self.scalar_static_f64[156]*v3532)+(self.scalar_static_f64[157]*v3545)))}else{v304});
        let v3567=(if self.scalar_static_bool[27]{(v84*(self.scalar_static_f64[156]*v3533))}else{v304});
        let v3568=(if self.scalar_static_bool[27]{(v84*((self.scalar_static_f64[156]*v3534)+(self.scalar_static_f64[157]*v3546)))}else{v304});
        let v3569=(if self.scalar_static_bool[27]{(v84*(self.scalar_static_f64[156]*v3535))}else{v304});
        let v4362=((-(v345*v1345))/v1396);
        let v4363=(v360/v11);
        let v4364=(v51/v11);
        let v4365=scalar_limexp_derivative(v1159);
        let v4366=(v4362*v4365);
        let v4367=(v4363*v4365);
        let v4368=(v4364*v4365);
        let v4372=scalar_limexp_derivative(v1161);
        let v4381=(v182*v1165);
        let v4382=(((v1160*v1695)+(v294*v4366))/v4381);
        let v4383=((v294*v4367)/v4381);
        let v4384=((v294*v4368)/v4381);
        let v4390=(v182*v1168);
        let v4391=(((v1162*v1695)+(v294*(((-(v347*v1345))/v1396)*v4372)))/v4390);
        let v4392=((v294*(v4363*v4372))/v4390);
        let v4393=((v294*(v4364*v4372))/v4390);
        let v4694=(self.scalar_static_f64[117]*(v1257*v3337));
        let v4695=(self.scalar_static_f64[117]*(v1257*v3338));
        let v4696=(self.scalar_static_f64[117]*(v1257*v3339));
        let v4700=(v1260*v1260);
        let v4723=scalar_limexp_derivative(v1270);
        let v4728=(v1261*(((v1260*v4694)-(v1259*v4694))/v4700));
        let v4730=(v1261*(((v1260*v4695)-(v1259*v4695))/v4700));
        let v4732=(v1261*(((v1260*v4696)-(v1259*v4696))/v4700));
        let v4783=(((v946*(v905*((v1277*(self.scalar_static_f64[163]*(self.scalar_static_f64[164]*v3394)))+(v1266*(v1257*(v1274*(self.scalar_static_f64[165]*(self.scalar_static_f64[292]*v4723))))))))-(v1281*v3472))/v3479);
        let v4787=(((v946*(v905*(v1277*(self.scalar_static_f64[163]*(self.scalar_static_f64[164]*v3395)))))-(v1281*v3473))/v3479);
        let v4796=((self.scalar_static_f64[158]*((v428*v1673)+(v279*v1871)))+(((v946*((v1278*v3337)+(v905*((v1277*(self.scalar_static_f64[163]*(self.scalar_static_f64[164]*v3393)))+(v1266*(v1257*(v1272*(v4728+v4728))))))))-(v1281*v3471))/v3479));
        let v4797=((self.scalar_static_f64[158]*(v279*v1872))+(((v946*((v1278*v3338)+(v905*((v1277*(self.scalar_static_f64[163]*(self.scalar_static_f64[164]*v3396)))+(v1266*(v1257*((v1274*(self.scalar_static_f64[165]*(self.scalar_static_f64[293]*v4723)))+(v1272*(v4730+v4730)))))))))-(v1281*v3474))/v3479));
        let v4798=((self.scalar_static_f64[158]*(v279*v1873))+(((v946*((v1278*v3339)+(v905*((v1277*(self.scalar_static_f64[163]*(self.scalar_static_f64[164]*v3397)))+(v1266*(v1257*(v1272*(v4732+v4732))))))))-(v1281*v3475))/v3479));
        let v4805=(self.scalar_static_f64[160]*((v488*v1673)+(v279*(if self.scalar_static_bool[10]{((v2024+(self.scalar_static_f64[129]*(v1950+(-v1993))))-v1963)}else{(if self.scalar_static_bool[9]{(v1931+v1935)}else{v304})}))));
        let v4806=(self.scalar_static_f64[160]*(v279*(if self.scalar_static_bool[10]{(v2025+(self.scalar_static_f64[129]*(v51-v1994)))}else{(if self.scalar_static_bool[9]{(v1932+v1936)}else{v304})})));
        let v4807=(self.scalar_static_f64[160]*(v279*(if self.scalar_static_bool[10]{(v2026+(self.scalar_static_f64[129]*(-v1995)))}else{(if self.scalar_static_bool[9]{(v1933+v1937)}else{v304})})));
        let v4808=(self.scalar_static_f64[160]*(v279*(if self.scalar_static_bool[10]{(v2027+(self.scalar_static_f64[129]*(v360-v1996)))}else{(if self.scalar_static_bool[9]{(v1934+v1938)}else{v304})})));
        let v4813=(v284*v2519);
        let v4815=(v284*v2521);
        let v4825=((((v671*(self.scalar_static_f64[89]*v1680))+(v284*v2517))+(self.scalar_static_f64[166]*v3358))+(self.scalar_static_f64[167]*v4382));
        let v4826=(((v284*v2518)+(self.scalar_static_f64[166]*v3359))+(self.scalar_static_f64[167]*v4383));
        let v4827=(((v284*v2520)+(self.scalar_static_f64[166]*v3360))+(self.scalar_static_f64[167]*v4384));
        let v4828=(self.scalar_static_f64[167]*v4391);
        let v4829=(self.scalar_static_f64[167]*v4392);
        let v4830=(self.scalar_static_f64[167]*v4393);
        let v4837=(v286*(if self.scalar_static_bool[18]{(v3006+(self.scalar_static_f64[143]*(-v2963)))}else{(if self.scalar_static_bool[16]{(v2814+(if self.scalar_static_bool[16]{(v778*(-v2765))}else{v2388}))}else{(if self.scalar_static_bool[11]{(v2619+v2625)}else{v304})})}));
        let v4844=(((v816*(self.scalar_static_f64[91]*v1680))+(v286*(if self.scalar_static_bool[18]{((v3002+(self.scalar_static_f64[143]*(v2902+(-v2959))))-v2915)}else{(if self.scalar_static_bool[16]{((v2810+(if self.scalar_static_bool[16]{((v780*(if self.scalar_static_bool[16]{(((v774*(if self.scalar_static_bool[16]{v2338}else{v2339}))+(v772*(-v2821)))+((v773*v2821)+(v771*(if self.scalar_static_bool[16]{v2346}else{v2347}))))}else{v2364}))+(v778*(v2661+(-v2761))))}else{v2384}))-(if self.scalar_static_bool[16]{(((v736*v1626)+(v247*(-((-(((v247*v2661)-(v732*v1626))/v1676))*(self.scalar_static_f64[133]*f64::powf(v734,self.scalar_static_f64[242]))))))/self.scalar_static_f64[133])}else{v2221}))}else{(if self.scalar_static_bool[11]{(v2615+v2621)}else{v304})})})))+(self.scalar_static_f64[166]*(if self.scalar_static_bool[28]{v304}else{v3565})));
        let v4845=((v286*(if self.scalar_static_bool[18]{(v3003+(self.scalar_static_f64[143]*(-v2960)))}else{(if self.scalar_static_bool[16]{(v2811+(if self.scalar_static_bool[16]{((v780*(if self.scalar_static_bool[16]{((v772*(-v2822))+(v773*v2822))}else{v2365}))+(v778*(-v2762)))}else{v2385}))}else{(if self.scalar_static_bool[11]{(v2616+v2622)}else{v304})})}))+(self.scalar_static_f64[166]*(if self.scalar_static_bool[28]{v304}else{v3566})));
        let v4846=((v286*(if self.scalar_static_bool[18]{(v3004+(self.scalar_static_f64[143]*(v51-v2961)))}else{(if self.scalar_static_bool[16]{(v2812+(if self.scalar_static_bool[16]{((v780*(if self.scalar_static_bool[16]{((v772*(-v2823))+(v773*v2823))}else{v304}))+(v778*(v51-v2763)))}else{v2386}))}else{(if self.scalar_static_bool[11]{(v2617+v2623)}else{v304})})}))+(self.scalar_static_f64[166]*(if self.scalar_static_bool[28]{v304}else{v3567})));
        let v4847=((v286*(if self.scalar_static_bool[18]{(v3005+(self.scalar_static_f64[143]*(-v2962)))}else{(if self.scalar_static_bool[16]{(v2813+(if self.scalar_static_bool[16]{((v780*(if self.scalar_static_bool[16]{((v772*(-v2824))+(v773*v2824))}else{v2366}))+(v778*(-v2764)))}else{v2387}))}else{(if self.scalar_static_bool[11]{(v2618+v2624)}else{v304})})}))+(self.scalar_static_f64[166]*(if self.scalar_static_bool[28]{v304}else{v3568})));
        let v4848=((v286*(if self.scalar_static_bool[18]{(v3007+(self.scalar_static_f64[143]*(v360-v2964)))}else{(if self.scalar_static_bool[16]{(v2815+(if self.scalar_static_bool[16]{((v780*(if self.scalar_static_bool[16]{((v772*(-v2825))+(v773*v2825))}else{v304}))+(v778*(v360-v2766)))}else{v304}))}else{(if self.scalar_static_bool[11]{(v2620+v2626)}else{v304})})}))+(self.scalar_static_f64[166]*(if self.scalar_static_bool[28]{v304}else{v3569})));
        let v4851=((v900*(self.scalar_static_f64[92]*(((-(self.scalar_static_f64[84]*v1664))/v1685)*(self.scalar_static_f64[93]*f64::powf(v288,self.scalar_static_f64[219])))))+(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{(((if self.scalar_static_bool[23]{(((v887*v3034)+(v818*((-(((v274*v3229)-(v884*v1664))/v1685))*v3254)))/self.scalar_static_f64[147])}else{v3129})+(self.scalar_static_f64[152]*(v3164+(-v3229))))-(if self.scalar_static_bool[23]{(((v872*v3034)+(v818*((-(((v274*v3164)-(v869*v1664))/v1685))*(self.scalar_static_f64[147]*f64::powf(v871,self.scalar_static_f64[273])))))/self.scalar_static_f64[147])}else{v2915}))}else{(if self.scalar_static_bool[21]{(v3129+(if v848{v304}else{(if v829{(v832*((v843*v3037)+(v827*(((v274*(self.scalar_static_f64[148]*v3037))-(v841*v1664))/v1685))))}else{v2621})}))}else{v304})})})));
        let v4852=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3230/v274))*v3254))/self.scalar_static_f64[147])}else{v3130})+(self.scalar_static_f64[152]*(-v3230)))}else{(if self.scalar_static_bool[21]{(v3130+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[261])+(v827*(self.scalar_static_f64[267]/v274))))}else{v2622})}))}else{v304})})}));
        let v4853=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3231/v274))*v3254))/self.scalar_static_f64[147])}else{v3131})+(self.scalar_static_f64[152]*(-v3231)))}else{(if self.scalar_static_bool[21]{(v3131+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[262])+(v827*(self.scalar_static_f64[268]/v274))))}else{v2623})}))}else{v304})})}));
        let v4854=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3232/v274))*v3254))/self.scalar_static_f64[147])}else{v3132})+(self.scalar_static_f64[152]*(-v3232)))}else{(if self.scalar_static_bool[21]{(v3132+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[263])+(v827*(self.scalar_static_f64[269]/v274))))}else{v2624})}))}else{v304})})}));
        let v4855=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3233/v274))*v3254))/self.scalar_static_f64[147])}else{v3133})+(self.scalar_static_f64[152]*(-v3233)))}else{(if self.scalar_static_bool[21]{(v3133+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[264])+(v827*(self.scalar_static_f64[270]/v274))))}else{v2625})}))}else{v304})})}));
        let v4859=((v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3234/v274))*v3254))/self.scalar_static_f64[147])}else{v3134})+(self.scalar_static_f64[152]*(v360-v3234)))}else{(if self.scalar_static_bool[21]{(v3134+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[265])+(v827*(self.scalar_static_f64[271]/v274))))}else{v2626})}))}else{v304})})}))+self.scalar_static_f64[294]);
        let v4860=(self.scalar_static_f64[168]+(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v818*((-(v3235/v274))*v3254))/self.scalar_static_f64[147])}else{v3135})+(self.scalar_static_f64[152]*(v51-v3235)))}else{(if self.scalar_static_bool[21]{(v3135+(if v848{v304}else{(if v829{(v832*((v843*self.scalar_static_f64[266])+(v827*(self.scalar_static_f64[272]/v274))))}else{v304})}))}else{v304})})})));

        CommonStampValues {
            v6,
            v11,
            v12,
            v13,
            v51,
            v52,
            v84,
            v182,
            v185,
            v209,
            v247,
            v304,
            v339,
            v340,
            v341,
            v342,
            v343,
            v344,
            v345,
            v346,
            v348,
            v349,
            v360,
            v824,
            v825,
            v884,
            v905,
            v911,
            v946,
            v950,
            v952,
            v954,
            v956,
            v958,
            v966,
            v1153,
            v1159,
            v1160,
            v1165,
            v1168,
            v1201,
            v1215,
            v1283,
            v1285,
            v1292,
            v1293,
            v1296,
            v1300,
            v1303,
            v1306,
            v1344,
            v1345,
            v1396,
            v1438,
            v1626,
            v3229,
            v3230,
            v3231,
            v3232,
            v3233,
            v3234,
            v3235,
            v3337,
            v3338,
            v3339,
            v3358,
            v3359,
            v3360,
            v3471,
            v3472,
            v3473,
            v3474,
            v3475,
            v3479,
            v3516,
            v3519,
            v3520,
            v3521,
            v3522,
            v3523,
            v3524,
            v3531,
            v3532,
            v3533,
            v3534,
            v3535,
            v3539,
            v3544,
            v3545,
            v3546,
            v3565,
            v3566,
            v3567,
            v3568,
            v3569,
            v4362,
            v4363,
            v4364,
            v4366,
            v4367,
            v4368,
            v4382,
            v4383,
            v4384,
            v4391,
            v4392,
            v4393,
            v4783,
            v4787,
            v4796,
            v4797,
            v4798,
            v4805,
            v4806,
            v4807,
            v4808,
            v4813,
            v4815,
            v4825,
            v4826,
            v4827,
            v4828,
            v4829,
            v4830,
            v4837,
            v4844,
            v4845,
            v4846,
            v4847,
            v4848,
            v4851,
            v4852,
            v4853,
            v4854,
            v4855,
            v4859,
            v4860,
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
        let v21=(self.scalar_static_f64[5]*f64::powf(common.v12,self.scalar_static_f64[6]));
        let v25=(self.scalar_static_f64[7]*f64::powf(common.v12,self.scalar_static_f64[8]));
        let v29=(self.scalar_static_f64[9]*f64::powf(common.v12,self.scalar_static_f64[10]));
        let v33=(self.scalar_static_f64[11]*f64::powf(common.v12,self.scalar_static_f64[12]));
        let v37=(self.scalar_static_f64[13]*f64::powf(common.v12,self.scalar_static_f64[14]));
        let v41=(self.scalar_static_f64[15]*f64::powf(common.v12,self.scalar_static_f64[16]));
        let v45=(self.scalar_static_f64[17]*f64::powf(common.v12,self.scalar_static_f64[18]));
        let v87=f64::powf(common.v12,self.scalar_static_f64[37]);
        let v90=(common.v52*self.scalar_static_f64[39]);
        let v92=((v90/common.v11)).exp();
        let v93=(v87*v92);
        let v97=(self.scalar_static_f64[36]*f64::powf(v93,self.scalar_static_f64[41]));
        let v100=f64::powf(common.v12,self.scalar_static_f64[43]);
        let v103=(common.v52*self.scalar_static_f64[45]);
        let v105=((v103/common.v11)).exp();
        let v106=(v100*v105);
        let v110=(self.scalar_static_f64[42]*f64::powf(v106,self.scalar_static_f64[47]));
        let v114=(common.v52*self.scalar_static_f64[50]);
        let v116=((v114/common.v11)).exp();
        let v117=(v87*v116);
        let v120=f64::powf(v117,self.scalar_static_f64[52]);
        let v121=(self.scalar_static_f64[48]*v120);
        let v125=(common.v52*self.scalar_static_f64[55]);
        let v127=((v125/common.v11)).exp();
        let v128=(v100*v127);
        let v131=f64::powf(v128,self.scalar_static_f64[57]);
        let v132=(self.scalar_static_f64[53]*v131);
        let v134=(v120*self.scalar_static_f64[58]);
        let v136=(v131*self.scalar_static_f64[59]);
        let v140=(common.v52*self.scalar_static_f64[62]);
        let v142=((v140/common.v11)).exp();
        let v143=(v87*v142);
        let v147=(self.scalar_static_f64[60]*f64::powf(v143,self.scalar_static_f64[64]));
        let v151=(common.v52*self.scalar_static_f64[67]);
        let v153=((v151/common.v11)).exp();
        let v154=(v100*v153);
        let v158=(self.scalar_static_f64[65]*f64::powf(v154,self.scalar_static_f64[69]));
        let v172=(common.v13*self.scalar_static_f64[75]);
        let v173=(self.scalar_static_f64[74]+v172);
        let v181=(self.scalar_static_f64[76]*(common.v51+(common.v13*self.scalar_static_f64[77])));
        let v298=(self.scalar_static_f64[95]*f64::powf(common.v12,self.scalar_static_f64[96]));
        let v299=(-(self.scalar_static_f64[73]*(common.v51+(common.v13*v173))));
        let v300=(common.v11*v181);
        let v302=((v299/v300)).exp();
        let v325=(if self.scalar_static_bool[5]{(common.v51/v298)}else{common.v304});
        let v947=(common.v911/common.v946);
        let v948=(common.v905/common.v946);
        let v971=((common.v51+(common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v966)}else{common.v304})))).sqrt();
        let v974=(if self.scalar_static_bool[27]{(common.v185*(common.v51+v971))}else{common.v304});
        let v976=(if self.scalar_static_bool[27]{(common.v825/common.v950)}else{common.v952});
        let v978=(if self.scalar_static_bool[27]{scalar_limexp(v976)}else{common.v954});
        let v979=(v978-common.v51);
        let v982=(common.v966-(if self.scalar_static_bool[27]{(common.v84*v979)}else{common.v304}));
        let v987=(if self.scalar_static_bool[28]{common.v51}else{v974});
        let v988=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(v982/v974)}else{common.v304})});
        let v991=(common.v11*self.scalar_static_f64[40]);
        let v992=(common.v341/v991);
        let v993=(if self.scalar_static_bool[29]{v992}else{v976});
        let v995=(if self.scalar_static_bool[29]{scalar_limexp(v993)}else{v978});
        let v996=(common.v11*self.scalar_static_f64[46]);
        let v997=(common.v341/v996);
        let v998=(if self.scalar_static_bool[29]{v997}else{common.v304});
        let v1000=(if self.scalar_static_bool[29]{scalar_limexp(v998)}else{common.v304});
        let v1003=(v299-common.v341);
        let v1004=(v1003/v300);
        let v1005=(if self.scalar_static_bool[31]{v1004}else{common.v956});
        let v1007=(if self.scalar_static_bool[31]{scalar_limexp(v1005)}else{common.v958});
        let v1008=(v995-common.v51);
        let v1010=(v1000-common.v51);
        let v1012=((v97*v1008)+(v110*v1010));
        let v1025=(common.v343/v991);
        let v1026=(if self.scalar_static_bool[36]{v1025}else{v993});
        let v1028=(if self.scalar_static_bool[36]{scalar_limexp(v1026)}else{v995});
        let v1029=(common.v343/v996);
        let v1030=(if self.scalar_static_bool[36]{v1029}else{v998});
        let v1032=(if self.scalar_static_bool[36]{scalar_limexp(v1030)}else{v1000});
        let v1034=(v299-common.v343);
        let v1035=(v1034/v300);
        let v1036=(if self.scalar_static_bool[37]{v1035}else{v1005});
        let v1038=(if self.scalar_static_bool[37]{scalar_limexp(v1036)}else{v1007});
        let v1039=(v1028-common.v51);
        let v1041=(v1032-common.v51);
        let v1043=((v97*v1039)+(v110*v1041));
        let v1052=(if self.scalar_static_bool[40]{v992}else{v1026});
        let v1054=(if self.scalar_static_bool[40]{scalar_limexp(v1052)}else{v1028});
        let v1055=(if self.scalar_static_bool[40]{v997}else{v1030});
        let v1057=(if self.scalar_static_bool[40]{scalar_limexp(v1055)}else{v1032});
        let v1059=(if self.scalar_static_bool[41]{v1004}else{v1036});
        let v1061=(if self.scalar_static_bool[41]{scalar_limexp(v1059)}else{v1038});
        let v1062=(v1054-common.v51);
        let v1064=(v1057-common.v51);
        let v1066=((v97*v1062)+(v110*v1064));
        let v1074=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v1066)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v1066-(self.scalar_static_f64[159]*(v1061-v302))))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v1012}else{(if self.scalar_static_bool[31]{(v1012-(self.scalar_static_f64[159]*(v1007-v302)))}else{common.v304})})})})});
        let v1075=(if self.scalar_static_bool[40]{v1025}else{v1052});
        let v1078=(if self.scalar_static_bool[40]{v1029}else{v1055});
        let v1081=(if self.scalar_static_bool[41]{v1035}else{v1059});
        let v1085=((if self.scalar_static_bool[40]{scalar_limexp(v1075)}else{v1054})-common.v51);
        let v1087=((if self.scalar_static_bool[40]{scalar_limexp(v1078)}else{v1057})-common.v51);
        let v1089=((v97*v1085)+(v110*v1087));
        let v1096=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v1089)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v1089-(self.scalar_static_f64[159]*((if self.scalar_static_bool[41]{scalar_limexp(v1081)}else{v1061})-v302))))}else{(if self.scalar_static_bool[38]{v1043}else{(if self.scalar_static_bool[37]{(v1043-(self.scalar_static_f64[159]*(v1038-v302)))}else{common.v304})})})});
        let v1097=(common.v11*self.scalar_static_f64[51]);
        let v1098=(common.v345/v1097);
        let v1099=scalar_limexp(v1098);
        let v1100=(common.v11*self.scalar_static_f64[56]);
        let v1101=(common.v345/v1100);
        let v1102=scalar_limexp(v1101);
        let v1103=(v1099-common.v51);
        let v1105=(v1102-common.v51);
        let v1107=((v121*v1103)+(v132*v1105));
        let v1112=(if self.scalar_static_bool[45]{(common.v349/v1097)}else{v1098});
        let v1116=(if self.scalar_static_bool[45]{(common.v349/v1100)}else{v1101});
        let v1118=(if self.scalar_static_bool[45]{scalar_limexp(v1116)}else{v1102});
        let v1119=((if self.scalar_static_bool[45]{scalar_limexp(v1112)}else{v1099})-common.v51);
        let v1121=(v1118-common.v51);
        let v1126=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{((v134*v1119)+(v136*v1121))}else{common.v304})});
        let v1129=(common.v247-common.v345);
        let v1131=0.01;
        let v1133=(((v1129*v1129)+v1131)).sqrt();
        let v1136=(if self.scalar_static_bool[47]{(common.v185*(v1129+v1133))}else{common.v884});
        let v1137=(self.scalar_static_f64[161]*v1136);
        let v1138=(-(self.scalar_static_f64[71]*(common.v51+(common.v13*self.scalar_static_f64[72]))));
        let v1140=f64::powf(v1136,self.scalar_static_f64[162]);
        let v1141=(v1138*v1140);
        let v1142=scalar_limexp(v1141);
        let v1144=(if self.scalar_static_bool[47]{(v1137*v1142)}else{common.v304});
        let v1145=(v948-v947);
        let v1146=(v1145-v1107);
        let v1151=(v1107-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{(v1144*v1146)}else{common.v304})}));
        let v1154=(common.v1153-common.v346);
        let v1158=(if self.scalar_static_bool[50]{common.v304}else{(if self.scalar_static_bool[49]{(v1154/v21)}else{common.v304})});
        let v1170=(common.v51+common.v1165);
        let v1171=(common.v51+common.v1168);
        let v1173=(if self.scalar_static_bool[51]{(v1170/v1171)}else{common.v304});
        let v1174=(common.v346-common.v344);
        let v1177=((common.v1165-common.v1168)-(v1173).ln());
        let v1179=(v1174+(common.v11*v1177));
        let v1181=(if self.scalar_static_bool[51]{(v1179/v25)}else{common.v304});
        let v1182=(v25*v325);
        let v1183=(v1181*v1182);
        let v1185=(self.scalar_static_f64[111]*(common.v185*v325));
        let v1188=((v1131+(v1174*v1174))).sqrt();
        let v1190=(common.v51+(v1185*v1188));
        let v1192=(if self.scalar_static_bool[51]{(v1183/v1190)}else{common.v304});
        let v1195=((common.v51+(v1192*v1192))).sqrt();
        let v1199=(if self.scalar_static_bool[52]{common.v304}else{(if self.scalar_static_bool[51]{(v1181/v1195)}else{common.v304})});
        let v1202=(common.v1201-common.v342);
        let v1206=(if self.scalar_static_bool[54]{common.v304}else{(if self.scalar_static_bool[53]{(v1202/v29)}else{common.v304})});
        let v1208=(common.v342-common.v339);
        let v1209=(common.v946*v1208);
        let v1213=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{(v1209/v33)}else{common.v304})});
        let v1216=(common.v1215-common.v340);
        let v1220=(if self.scalar_static_bool[58]{common.v304}else{(if self.scalar_static_bool[57]{(v1216/v37)}else{common.v304})});
        let v1222=(common.v348-common.v346);
        let v1223=(v987*v1222);
        let v1227=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{(v1223/v45)}else{common.v304})});
        let v1231=(common.v11*self.scalar_static_f64[63]);
        let v1233=(if self.scalar_static_bool[63]{(common.v825/v1231)}else{common.v1159});
        let v1236=(common.v11*self.scalar_static_f64[68]);
        let v1238=(if self.scalar_static_bool[63]{(common.v825/v1236)}else{v1116});
        let v1241=((if self.scalar_static_bool[63]{scalar_limexp(v1233)}else{common.v1160})-common.v51);
        let v1243=((if self.scalar_static_bool[63]{scalar_limexp(v1238)}else{v1118})-common.v51);
        let v1248=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{((v147*v1241)+(v158*v1243))}else{common.v304})});
        let v1251=(ctx.node_voltage(nodes[3])-common.v824);
        let v1255=(if self.scalar_static_bool[66]{common.v304}else{(if self.scalar_static_bool[65]{(v1251/v41)}else{common.v304})});
        let v1310=(common.v344-common.v340);
        let v1321=(common.v342-common.v824);
        let v1361=(self.scalar_static_f64[7]*(self.scalar_static_f64[173]*(self.scalar_static_f64[8]*f64::powf(common.v12,self.scalar_static_f64[176]))));
        let v1442=(self.scalar_static_f64[173]*(self.scalar_static_f64[37]*f64::powf(common.v12,self.scalar_static_f64[191])));
        let v1456=(self.scalar_static_f64[36]*(((v92*v1442)+(v87*(v92*(((common.v11*self.scalar_static_f64[192])-(v90*common.v1345))/common.v1396))))*(self.scalar_static_f64[41]*f64::powf(v93,self.scalar_static_f64[193]))));
        let v1460=(self.scalar_static_f64[173]*(self.scalar_static_f64[43]*f64::powf(common.v12,self.scalar_static_f64[194])));
        let v1474=(self.scalar_static_f64[42]*(((v105*v1460)+(v100*(v105*(((common.v11*self.scalar_static_f64[195])-(v103*common.v1345))/common.v1396))))*(self.scalar_static_f64[47]*f64::powf(v106,self.scalar_static_f64[196]))));
        let v1487=(((v116*v1442)+(v87*(v116*(((common.v11*self.scalar_static_f64[197])-(v114*common.v1345))/common.v1396))))*(self.scalar_static_f64[52]*f64::powf(v117,self.scalar_static_f64[198])));
        let v1501=(((v127*v1460)+(v100*(v127*(((common.v11*self.scalar_static_f64[199])-(v125*common.v1345))/common.v1396))))*(self.scalar_static_f64[57]*f64::powf(v128,self.scalar_static_f64[200])));
        let v1704=((v181*common.v1345)+(common.v11*self.scalar_static_f64[208]));
        let v1705=(v300*(-(self.scalar_static_f64[73]*(v172+v173))));
        let v1708=(v300*v300);
        let v1710=(v302*((v1705-(v299*v1704))/v1708));
        let v1718=(if self.scalar_static_bool[5]{((-(self.scalar_static_f64[95]*(self.scalar_static_f64[173]*(self.scalar_static_f64[96]*f64::powf(common.v12,self.scalar_static_f64[220])))))/(v298*v298))}else{common.v304});
        let v3480=(((common.v946*common.v3358)-(common.v911*common.v3471))/common.v3479);
        let v3484=(((common.v946*common.v3359)-(common.v911*common.v3472))/common.v3479);
        let v3487=((-(common.v911*common.v3473))/common.v3479);
        let v3491=(((common.v946*common.v3360)-(common.v911*common.v3474))/common.v3479);
        let v3494=((-(common.v911*common.v3475))/common.v3479);
        let v3498=(((common.v946*common.v3337)-(common.v905*common.v3471))/common.v3479);
        let v3501=((-(common.v905*common.v3472))/common.v3479);
        let v3504=((-(common.v905*common.v3473))/common.v3479);
        let v3508=(((common.v946*common.v3338)-(common.v905*common.v3474))/common.v3479);
        let v3512=(((common.v946*common.v3339)-(common.v905*common.v3475))/common.v3479);
        let v3585=(common.v182*v971);
        let v3596=(if self.scalar_static_bool[27]{(common.v185*((common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v3565)}else{common.v304}))/v3585))}else{common.v304});
        let v3597=(if self.scalar_static_bool[27]{(common.v185*((common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v3566)}else{common.v304}))/v3585))}else{common.v304});
        let v3598=(if self.scalar_static_bool[27]{(common.v185*((common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v3567)}else{common.v304}))/v3585))}else{common.v304});
        let v3599=(if self.scalar_static_bool[27]{(common.v185*((common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v3568)}else{common.v304}))/v3585))}else{common.v304});
        let v3600=(if self.scalar_static_bool[27]{(common.v185*((common.v209*(if self.scalar_static_bool[27]{(self.scalar_static_f64[108]*common.v3569)}else{common.v304}))/v3585))}else{common.v304});
        let v3604=(if self.scalar_static_bool[27]{((-(common.v825*self.scalar_static_f64[282]))/common.v3516)}else{common.v3520});
        let v3605=(if self.scalar_static_bool[27]{common.v304}else{common.v3521});
        let v3606=(if self.scalar_static_bool[27]{common.v304}else{common.v3522});
        let v3607=(if self.scalar_static_bool[27]{common.v304}else{common.v3523});
        let v3608=(if self.scalar_static_bool[27]{common.v3519}else{common.v3524});
        let v3609=scalar_limexp_derivative(v976);
        let v3616=(if self.scalar_static_bool[27]{(v3604*v3609)}else{common.v3531});
        let v3617=(if self.scalar_static_bool[27]{(v3605*v3609)}else{common.v3532});
        let v3618=(if self.scalar_static_bool[27]{(v3606*v3609)}else{common.v3533});
        let v3619=(if self.scalar_static_bool[27]{(v3607*v3609)}else{common.v3534});
        let v3620=(if self.scalar_static_bool[27]{(v3608*v3609)}else{common.v3535});
        let v3621=(if self.scalar_static_bool[27]{(common.v3522*v3609)}else{common.v304});
        let v3645=(v974*v974);
        let v3680=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(((v974*(common.v3565-(if self.scalar_static_bool[27]{((v979*common.v1438)+(common.v84*v3616))}else{common.v304})))-(v982*v3596))/v3645)}else{common.v304})});
        let v3681=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(((v974*(common.v3566-(if self.scalar_static_bool[27]{(common.v84*v3617)}else{common.v304})))-(v982*v3597))/v3645)}else{common.v304})});
        let v3682=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(((v974*(common.v3567-(if self.scalar_static_bool[27]{(common.v84*v3618)}else{common.v304})))-(v982*v3598))/v3645)}else{common.v304})});
        let v3683=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(((v974*(common.v3568-(if self.scalar_static_bool[27]{(common.v84*v3619)}else{common.v304})))-(v982*v3599))/v3645)}else{common.v304})});
        let v3684=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{(((v974*(common.v3569-(if self.scalar_static_bool[27]{(common.v84*v3620)}else{common.v304})))-(v982*v3600))/v3645)}else{common.v304})});
        let v3685=(if self.scalar_static_bool[28]{common.v304}else{(if self.scalar_static_bool[27]{((-(if self.scalar_static_bool[27]{(common.v84*v3621)}else{common.v304}))/v974)}else{common.v304})});
        let v3689=(v991*v991);
        let v3690=((-(common.v341*self.scalar_static_f64[283]))/v3689);
        let v3691=(common.v51/v991);
        let v3692=(common.v360/v991);
        let v3693=(if self.scalar_static_bool[29]{v3690}else{v3604});
        let v3694=(if self.scalar_static_bool[29]{common.v304}else{v3605});
        let v3695=(if self.scalar_static_bool[29]{common.v304}else{v3606});
        let v3696=(if self.scalar_static_bool[29]{v3691}else{v3607});
        let v3697=(if self.scalar_static_bool[29]{v3692}else{common.v304});
        let v3698=(if self.scalar_static_bool[29]{common.v304}else{v3608});
        let v3699=(if self.scalar_static_bool[29]{common.v304}else{common.v3522});
        let v3700=scalar_limexp_derivative(v993);
        let v3708=(if self.scalar_static_bool[29]{(v3693*v3700)}else{v3616});
        let v3709=(if self.scalar_static_bool[29]{(v3694*v3700)}else{v3617});
        let v3710=(if self.scalar_static_bool[29]{(v3695*v3700)}else{v3618});
        let v3711=(if self.scalar_static_bool[29]{(v3696*v3700)}else{v3619});
        let v3712=(if self.scalar_static_bool[29]{(v3697*v3700)}else{common.v304});
        let v3713=(if self.scalar_static_bool[29]{(v3698*v3700)}else{v3620});
        let v3714=(if self.scalar_static_bool[29]{(v3699*v3700)}else{v3621});
        let v3718=(v996*v996);
        let v3719=((-(common.v341*self.scalar_static_f64[284]))/v3718);
        let v3720=(common.v51/v996);
        let v3721=(common.v360/v996);
        let v3722=(if self.scalar_static_bool[29]{v3719}else{common.v304});
        let v3723=(if self.scalar_static_bool[29]{v3720}else{common.v304});
        let v3724=(if self.scalar_static_bool[29]{v3721}else{common.v304});
        let v3725=scalar_limexp_derivative(v998);
        let v3729=(if self.scalar_static_bool[29]{(v3722*v3725)}else{common.v304});
        let v3730=(if self.scalar_static_bool[29]{(v3723*v3725)}else{common.v304});
        let v3731=(if self.scalar_static_bool[29]{(v3724*v3725)}else{common.v304});
        let v3734=((v1705-(v1003*v1704))/v1708);
        let v3735=(common.v360/v300);
        let v3736=(common.v51/v300);
        let v3737=(if self.scalar_static_bool[31]{v3734}else{common.v3539});
        let v3738=(if self.scalar_static_bool[31]{common.v304}else{common.v3524});
        let v3739=(if self.scalar_static_bool[31]{v3735}else{common.v3522});
        let v3740=(if self.scalar_static_bool[31]{v3736}else{common.v304});
        let v3741=scalar_limexp_derivative(v1005);
        let v3746=(if self.scalar_static_bool[31]{(v3737*v3741)}else{common.v3544});
        let v3747=(if self.scalar_static_bool[31]{(v3738*v3741)}else{common.v3545});
        let v3748=(if self.scalar_static_bool[31]{(v3739*v3741)}else{common.v3546});
        let v3749=(if self.scalar_static_bool[31]{(v3740*v3741)}else{common.v304});
        let v3753=(v97*v3709);
        let v3754=(v97*v3710);
        let v3757=(v97*v3713);
        let v3758=(v97*v3714);
        let v3764=(((v1008*v1456)+(v97*v3708))+((v1010*v1474)+(v110*v3729)));
        let v3765=((v97*v3711)+(v110*v3730));
        let v3766=((v97*v3712)+(v110*v3731));
        let v3799=((-(common.v343*self.scalar_static_f64[283]))/v3689);
        let v3800=(if self.scalar_static_bool[36]{v3799}else{v3693});
        let v3801=(if self.scalar_static_bool[36]{common.v304}else{v3694});
        let v3802=(if self.scalar_static_bool[36]{v3691}else{v3695});
        let v3803=(if self.scalar_static_bool[36]{common.v304}else{v3696});
        let v3804=(if self.scalar_static_bool[36]{v3692}else{v3697});
        let v3805=(if self.scalar_static_bool[36]{common.v304}else{v3698});
        let v3806=(if self.scalar_static_bool[36]{common.v304}else{v3699});
        let v3807=scalar_limexp_derivative(v1026);
        let v3815=(if self.scalar_static_bool[36]{(v3800*v3807)}else{v3708});
        let v3816=(if self.scalar_static_bool[36]{(v3801*v3807)}else{v3709});
        let v3817=(if self.scalar_static_bool[36]{(v3802*v3807)}else{v3710});
        let v3818=(if self.scalar_static_bool[36]{(v3803*v3807)}else{v3711});
        let v3819=(if self.scalar_static_bool[36]{(v3804*v3807)}else{v3712});
        let v3820=(if self.scalar_static_bool[36]{(v3805*v3807)}else{v3713});
        let v3821=(if self.scalar_static_bool[36]{(v3806*v3807)}else{v3714});
        let v3824=((-(common.v343*self.scalar_static_f64[284]))/v3718);
        let v3825=(if self.scalar_static_bool[36]{v3824}else{v3722});
        let v3826=(if self.scalar_static_bool[36]{v3720}else{common.v304});
        let v3827=(if self.scalar_static_bool[36]{common.v304}else{v3723});
        let v3828=(if self.scalar_static_bool[36]{v3721}else{v3724});
        let v3829=scalar_limexp_derivative(v1030);
        let v3834=(if self.scalar_static_bool[36]{(v3825*v3829)}else{v3729});
        let v3835=(if self.scalar_static_bool[36]{(v3826*v3829)}else{common.v304});
        let v3836=(if self.scalar_static_bool[36]{(v3827*v3829)}else{v3730});
        let v3837=(if self.scalar_static_bool[36]{(v3828*v3829)}else{v3731});
        let v3840=((v1705-(v1034*v1704))/v1708);
        let v3841=(if self.scalar_static_bool[37]{v3840}else{v3737});
        let v3842=(if self.scalar_static_bool[37]{common.v304}else{v3738});
        let v3843=(if self.scalar_static_bool[37]{v3735}else{common.v304});
        let v3844=(if self.scalar_static_bool[37]{common.v304}else{v3739});
        let v3845=(if self.scalar_static_bool[37]{v3736}else{v3740});
        let v3846=scalar_limexp_derivative(v1036);
        let v3852=(if self.scalar_static_bool[37]{(v3841*v3846)}else{v3746});
        let v3853=(if self.scalar_static_bool[37]{(v3842*v3846)}else{v3747});
        let v3854=(if self.scalar_static_bool[37]{(v3843*v3846)}else{common.v304});
        let v3855=(if self.scalar_static_bool[37]{(v3844*v3846)}else{v3748});
        let v3856=(if self.scalar_static_bool[37]{(v3845*v3846)}else{v3749});
        let v3860=(v97*v3816);
        let v3864=(v97*v3820);
        let v3865=(v97*v3821);
        let v3872=(((v1039*v1456)+(v97*v3815))+((v1041*v1474)+(v110*v3834)));
        let v3873=((v97*v3817)+(v110*v3835));
        let v3874=((v97*v3818)+(v110*v3836));
        let v3875=((v97*v3819)+(v110*v3837));
        let v3901=(if self.scalar_static_bool[40]{v3690}else{v3800});
        let v3902=(if self.scalar_static_bool[40]{common.v304}else{v3801});
        let v3903=(if self.scalar_static_bool[40]{common.v304}else{v3802});
        let v3904=(if self.scalar_static_bool[40]{v3691}else{v3803});
        let v3905=(if self.scalar_static_bool[40]{v3692}else{v3804});
        let v3906=(if self.scalar_static_bool[40]{common.v304}else{v3805});
        let v3907=(if self.scalar_static_bool[40]{common.v304}else{v3806});
        let v3908=scalar_limexp_derivative(v1052);
        let v3916=(if self.scalar_static_bool[40]{(v3901*v3908)}else{v3815});
        let v3917=(if self.scalar_static_bool[40]{(v3902*v3908)}else{v3816});
        let v3918=(if self.scalar_static_bool[40]{(v3903*v3908)}else{v3817});
        let v3919=(if self.scalar_static_bool[40]{(v3904*v3908)}else{v3818});
        let v3920=(if self.scalar_static_bool[40]{(v3905*v3908)}else{v3819});
        let v3921=(if self.scalar_static_bool[40]{(v3906*v3908)}else{v3820});
        let v3922=(if self.scalar_static_bool[40]{(v3907*v3908)}else{v3821});
        let v3923=(if self.scalar_static_bool[40]{v3719}else{v3825});
        let v3924=(if self.scalar_static_bool[40]{common.v304}else{v3826});
        let v3925=(if self.scalar_static_bool[40]{v3720}else{v3827});
        let v3926=(if self.scalar_static_bool[40]{v3721}else{v3828});
        let v3927=scalar_limexp_derivative(v1055);
        let v3932=(if self.scalar_static_bool[40]{(v3923*v3927)}else{v3834});
        let v3933=(if self.scalar_static_bool[40]{(v3924*v3927)}else{v3835});
        let v3934=(if self.scalar_static_bool[40]{(v3925*v3927)}else{v3836});
        let v3935=(if self.scalar_static_bool[40]{(v3926*v3927)}else{v3837});
        let v3936=(if self.scalar_static_bool[41]{v3734}else{v3841});
        let v3937=(if self.scalar_static_bool[41]{common.v304}else{v3842});
        let v3938=(if self.scalar_static_bool[41]{common.v304}else{v3843});
        let v3939=(if self.scalar_static_bool[41]{v3735}else{v3844});
        let v3940=(if self.scalar_static_bool[41]{v3736}else{v3845});
        let v3941=scalar_limexp_derivative(v1059);
        let v3947=(if self.scalar_static_bool[41]{(v3936*v3941)}else{v3852});
        let v3948=(if self.scalar_static_bool[41]{(v3937*v3941)}else{v3853});
        let v3949=(if self.scalar_static_bool[41]{(v3938*v3941)}else{v3854});
        let v3950=(if self.scalar_static_bool[41]{(v3939*v3941)}else{v3855});
        let v3951=(if self.scalar_static_bool[41]{(v3940*v3941)}else{v3856});
        let v3955=(v97*v3917);
        let v3967=(((v1062*v1456)+(v97*v3916))+((v1064*v1474)+(v110*v3932)));
        let v3968=((v97*v3918)+(v110*v3933));
        let v3969=((v97*v3919)+(v110*v3934));
        let v3970=((v97*v3920)+(v110*v3935));
        let v3987=(self.scalar_static_f64[158]*(v97*v3921));
        let v3988=(self.scalar_static_f64[158]*(v97*v3922));
        let v4001=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v3967)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v3967-(self.scalar_static_f64[159]*(v3947-v1710))))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3764}else{(if self.scalar_static_bool[31]{(v3764-(self.scalar_static_f64[159]*(v3746-v1710)))}else{common.v304})})})})});
        let v4002=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v3955)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v3955-(self.scalar_static_f64[159]*v3948)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3753}else{(if self.scalar_static_bool[31]{(v3753-(self.scalar_static_f64[159]*v3747))}else{common.v304})})})})});
        let v4003=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v3968)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v3968-(self.scalar_static_f64[159]*v3949)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3754}else{(if self.scalar_static_bool[31]{v3754}else{common.v304})})})})});
        let v4004=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v3969)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v3969-(self.scalar_static_f64[159]*v3950)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3765}else{(if self.scalar_static_bool[31]{(v3765-(self.scalar_static_f64[159]*v3748))}else{common.v304})})})})});
        let v4005=(if self.scalar_static_bool[42]{(self.scalar_static_f64[158]*v3970)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[158]*(v3970-(self.scalar_static_f64[159]*v3951)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3766}else{(if self.scalar_static_bool[31]{(v3766-(self.scalar_static_f64[159]*v3749))}else{common.v304})})})})});
        let v4006=(if self.scalar_static_bool[42]{v3987}else{(if self.scalar_static_bool[41]{v3987}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3757}else{(if self.scalar_static_bool[31]{v3757}else{common.v304})})})})});
        let v4007=(if self.scalar_static_bool[42]{v3988}else{(if self.scalar_static_bool[41]{v3988}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3758}else{(if self.scalar_static_bool[31]{v3758}else{common.v304})})})})});
        let v4015=scalar_limexp_derivative(v1075);
        let v4034=scalar_limexp_derivative(v1078);
        let v4048=scalar_limexp_derivative(v1081);
        let v4062=(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3902})*v4015)}else{v3917}));
        let v4074=(((v1085*v1456)+(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3799}else{v3901})*v4015)}else{v3916})))+((v1087*v1474)+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3824}else{v3923})*v4034)}else{v3932}))));
        let v4075=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3691}else{v3903})*v4015)}else{v3918}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3720}else{v3924})*v4034)}else{v3933})));
        let v4076=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3904})*v4015)}else{v3919}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3925})*v4034)}else{v3934})));
        let v4077=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3692}else{v3905})*v4015)}else{v3920}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3721}else{v3926})*v4034)}else{v3935})));
        let v4094=(self.scalar_static_f64[160]*(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3906})*v4015)}else{v3921})));
        let v4095=(self.scalar_static_f64[160]*(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3907})*v4015)}else{v3922})));
        let v4108=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v4074)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v4074-(self.scalar_static_f64[159]*((if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3840}else{v3936})*v4048)}else{v3947})-v1710))))}else{(if self.scalar_static_bool[38]{v3872}else{(if self.scalar_static_bool[37]{(v3872-(self.scalar_static_f64[159]*(v3852-v1710)))}else{common.v304})})})});
        let v4109=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v4062)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v4062-(self.scalar_static_f64[159]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{common.v304}else{v3937})*v4048)}else{v3948}))))}else{(if self.scalar_static_bool[38]{v3860}else{(if self.scalar_static_bool[37]{(v3860-(self.scalar_static_f64[159]*v3853))}else{common.v304})})})});
        let v4110=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v4075)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v4075-(self.scalar_static_f64[159]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3735}else{v3938})*v4048)}else{v3949}))))}else{(if self.scalar_static_bool[38]{v3873}else{(if self.scalar_static_bool[37]{(v3873-(self.scalar_static_f64[159]*v3854))}else{common.v304})})})});
        let v4111=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v4076)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v4076-(self.scalar_static_f64[159]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{common.v304}else{v3939})*v4048)}else{v3950}))))}else{(if self.scalar_static_bool[38]{v3874}else{(if self.scalar_static_bool[37]{(v3874-(self.scalar_static_f64[159]*v3855))}else{common.v304})})})});
        let v4112=(if self.scalar_static_bool[42]{(self.scalar_static_f64[160]*v4077)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[160]*(v4077-(self.scalar_static_f64[159]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3736}else{v3940})*v4048)}else{v3951}))))}else{(if self.scalar_static_bool[38]{v3875}else{(if self.scalar_static_bool[37]{(v3875-(self.scalar_static_f64[159]*v3856))}else{common.v304})})})});
        let v4113=(if self.scalar_static_bool[42]{v4094}else{(if self.scalar_static_bool[41]{v4094}else{(if self.scalar_static_bool[38]{v3864}else{(if self.scalar_static_bool[37]{v3864}else{common.v304})})})});
        let v4114=(if self.scalar_static_bool[42]{v4095}else{(if self.scalar_static_bool[41]{v4095}else{(if self.scalar_static_bool[38]{v3865}else{(if self.scalar_static_bool[37]{v3865}else{common.v304})})})});
        let v4118=(v1097*v1097);
        let v4119=((-(common.v345*self.scalar_static_f64[285]))/v4118);
        let v4120=(common.v360/v1097);
        let v4121=(common.v51/v1097);
        let v4122=scalar_limexp_derivative(v1098);
        let v4123=(v4119*v4122);
        let v4124=(v4120*v4122);
        let v4125=(v4121*v4122);
        let v4129=(v1100*v1100);
        let v4130=((-(common.v345*self.scalar_static_f64[286]))/v4129);
        let v4131=(common.v360/v1100);
        let v4132=(common.v51/v1100);
        let v4133=scalar_limexp_derivative(v1101);
        let v4134=(v4130*v4133);
        let v4135=(v4131*v4133);
        let v4136=(v4132*v4133);
        let v4147=(((v1103*(self.scalar_static_f64[48]*v1487))+(v121*v4123))+((v1105*(self.scalar_static_f64[53]*v1501))+(v132*v4134)));
        let v4148=((v121*v4124)+(v132*v4135));
        let v4149=((v121*v4125)+(v132*v4136));
        let v4158=scalar_limexp_derivative(v1112);
        let v4172=(if self.scalar_static_bool[45]{((-(common.v349*self.scalar_static_f64[286]))/v4129)}else{v4130});
        let v4173=(if self.scalar_static_bool[45]{common.v304}else{v4131});
        let v4174=(if self.scalar_static_bool[45]{v4132}else{common.v304});
        let v4175=(if self.scalar_static_bool[45]{common.v304}else{v4132});
        let v4176=(if self.scalar_static_bool[45]{v4131}else{common.v304});
        let v4177=scalar_limexp_derivative(v1116);
        let v4183=(if self.scalar_static_bool[45]{(v4172*v4177)}else{v4134});
        let v4184=(if self.scalar_static_bool[45]{(v4173*v4177)}else{v4135});
        let v4185=(if self.scalar_static_bool[45]{(v4174*v4177)}else{common.v304});
        let v4186=(if self.scalar_static_bool[45]{(v4175*v4177)}else{v4136});
        let v4187=(if self.scalar_static_bool[45]{(v4176*v4177)}else{common.v304});
        let v4212=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{(((v1119*(self.scalar_static_f64[58]*v1487))+(v134*(if self.scalar_static_bool[45]{((if self.scalar_static_bool[45]{((-(common.v349*self.scalar_static_f64[285]))/v4118)}else{v4119})*v4158)}else{v4123})))+((v1121*(self.scalar_static_f64[59]*v1501))+(v136*v4183)))}else{common.v304})});
        let v4213=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{((v134*(if self.scalar_static_bool[45]{((if self.scalar_static_bool[45]{common.v304}else{v4120})*v4158)}else{v4124}))+(v136*v4184))}else{common.v304})});
        let v4214=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{((v134*(if self.scalar_static_bool[45]{((if self.scalar_static_bool[45]{v4121}else{common.v304})*v4158)}else{common.v304}))+(v136*v4185))}else{common.v304})});
        let v4215=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{((v134*(if self.scalar_static_bool[45]{((if self.scalar_static_bool[45]{common.v304}else{v4121})*v4158)}else{v4125}))+(v136*v4186))}else{common.v304})});
        let v4216=(if self.scalar_static_bool[46]{common.v304}else{(if self.scalar_static_bool[45]{((v134*(if self.scalar_static_bool[45]{((if self.scalar_static_bool[45]{v4120}else{common.v304})*v4158)}else{common.v304}))+(v136*v4187))}else{common.v304})});
        let v4217=(v1129*common.v1626);
        let v4220=(-v1129);
        let v4222=(common.v182*v1133);
        let v4232=(if self.scalar_static_bool[47]{(common.v185*(common.v1626+((v4217+v4217)/v4222)))}else{common.v3229});
        let v4233=(if self.scalar_static_bool[47]{(common.v185*(common.v51+((v1129+v1129)/v4222)))}else{common.v3230});
        let v4234=(if self.scalar_static_bool[47]{common.v304}else{common.v3231});
        let v4235=(if self.scalar_static_bool[47]{(common.v185*(common.v360+((v4220+v4220)/v4222)))}else{common.v3232});
        let v4236=(if self.scalar_static_bool[47]{common.v304}else{common.v3233});
        let v4237=(if self.scalar_static_bool[47]{common.v304}else{common.v3234});
        let v4238=(if self.scalar_static_bool[47]{common.v304}else{common.v3235});
        let v4249=(self.scalar_static_f64[162]*f64::powf(v1136,self.scalar_static_f64[288]));
        let v4266=scalar_limexp_derivative(v1141);
        let v4302=(v3498-v3480);
        let v4303=(v3501-v3484);
        let v4304=(v3504-v3487);
        let v4305=(v3508-v3491);
        let v4306=(v3512-v3494);
        let v4341=(v4147-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{((v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4232))+(v1137*(((v1140*self.scalar_static_f64[287])+(v1138*(v4232*v4249)))*v4266)))}else{common.v304}))+(v1144*(v4302-v4147)))}else{common.v304})}));
        let v4342=(v4148-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{((v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4233))+(v1137*((v1138*(v4233*v4249))*v4266)))}else{common.v304}))+(v1144*(v4303-v4148)))}else{common.v304})}));
        let v4343=(-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{((v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4234))+(v1137*((v1138*(v4234*v4249))*v4266)))}else{common.v304}))+(v1144*v4304))}else{common.v304})}));
        let v4344=(v4149-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{((v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4235))+(v1137*((v1138*(v4235*v4249))*v4266)))}else{common.v304}))+(v1144*(v4305-v4149)))}else{common.v304})}));
        let v4345=(-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{((v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4236))+(v1137*((v1138*(v4236*v4249))*v4266)))}else{common.v304}))+(v1144*v4306))}else{common.v304})}));
        let v4346=(-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{(v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4237))+(v1137*((v1138*(v4237*v4249))*v4266)))}else{common.v304}))}else{common.v304})}));
        let v4347=(-(if self.scalar_static_bool[48]{common.v304}else{(if self.scalar_static_bool[47]{(v1146*(if self.scalar_static_bool[47]{((v1142*(self.scalar_static_f64[161]*v4238))+(v1137*((v1138*(v4238*v4249))*v4266)))}else{common.v304}))}else{common.v304})}));
        let v4357=(if self.scalar_static_bool[50]{common.v304}else{(if self.scalar_static_bool[49]{(common.v51/v21)}else{common.v304})});
        let v4358=(if self.scalar_static_bool[50]{common.v304}else{(if self.scalar_static_bool[49]{((-(v1154*(self.scalar_static_f64[5]*(self.scalar_static_f64[173]*(self.scalar_static_f64[6]*f64::powf(common.v12,self.scalar_static_f64[175]))))))/(v21*v21))}else{common.v304})});
        let v4359=(if self.scalar_static_bool[50]{common.v304}else{(if self.scalar_static_bool[49]{(common.v360/v21)}else{common.v304})});
        let v4397=(v1171*v1171);
        let v4438=(if self.scalar_static_bool[51]{(((v25*((v1177*common.v1345)+(common.v11*((common.v4382-common.v4391)-((if self.scalar_static_bool[51]{(((v1171*common.v4382)-(v1170*common.v4391))/v4397)}else{common.v304})/v1173)))))-(v1179*v1361))/(v25*v25))}else{common.v304});
        let v4439=(if self.scalar_static_bool[51]{((common.v51+(common.v11*((-common.v4392)-((if self.scalar_static_bool[51]{((-(v1170*common.v4392))/v4397)}else{common.v304})/v1173))))/v25)}else{common.v304});
        let v4440=(if self.scalar_static_bool[51]{((common.v360+(common.v11*(common.v4383-((if self.scalar_static_bool[51]{(common.v4383/v1171)}else{common.v304})/v1173))))/v25)}else{common.v304});
        let v4441=(if self.scalar_static_bool[51]{((common.v11*((common.v4384-common.v4393)-((if self.scalar_static_bool[51]{(((v1171*common.v4384)-(v1170*common.v4393))/v4397)}else{common.v304})/v1173)))/v25)}else{common.v304});
        let v4454=(-v1174);
        let v4456=(common.v182*v1188);
        let v4465=(v1190*v1190);
        let v4480=(v1192*(if self.scalar_static_bool[51]{(((v1190*((v1182*v4438)+(v1181*((v325*v1361)+(v25*v1718)))))-(v1183*(v1188*(self.scalar_static_f64[111]*(common.v185*v1718)))))/v4465)}else{common.v304}));
        let v4482=(v1192*(if self.scalar_static_bool[51]{(((v1190*(v1182*v4439))-(v1183*(v1185*((v1174+v1174)/v4456))))/v4465)}else{common.v304}));
        let v4484=(v1192*(if self.scalar_static_bool[51]{(((v1190*(v1182*v4440))-(v1183*(v1185*((v4454+v4454)/v4456))))/v4465)}else{common.v304}));
        let v4486=(v1192*(if self.scalar_static_bool[51]{((v1182*v4441)/v1190)}else{common.v304}));
        let v4488=(common.v182*v1195);
        let v4496=(v1195*v1195);
        let v4514=(if self.scalar_static_bool[52]{common.v304}else{(if self.scalar_static_bool[51]{(((v1195*v4438)-(v1181*((v4480+v4480)/v4488)))/v4496)}else{common.v304})});
        let v4515=(if self.scalar_static_bool[52]{common.v304}else{(if self.scalar_static_bool[51]{(((v1195*v4439)-(v1181*((v4482+v4482)/v4488)))/v4496)}else{common.v304})});
        let v4516=(if self.scalar_static_bool[52]{common.v304}else{(if self.scalar_static_bool[51]{(((v1195*v4440)-(v1181*((v4484+v4484)/v4488)))/v4496)}else{common.v304})});
        let v4517=(if self.scalar_static_bool[52]{common.v304}else{(if self.scalar_static_bool[51]{(((v1195*v4441)-(v1181*((v4486+v4486)/v4488)))/v4496)}else{common.v304})});
        let v4527=(if self.scalar_static_bool[54]{common.v304}else{(if self.scalar_static_bool[53]{(common.v51/v29)}else{common.v304})});
        let v4528=(if self.scalar_static_bool[54]{common.v304}else{(if self.scalar_static_bool[53]{((-(v1202*(self.scalar_static_f64[9]*(self.scalar_static_f64[173]*(self.scalar_static_f64[10]*f64::powf(common.v12,self.scalar_static_f64[177]))))))/(v29*v29))}else{common.v304})});
        let v4529=(if self.scalar_static_bool[54]{common.v304}else{(if self.scalar_static_bool[53]{(common.v360/v29)}else{common.v304})});
        let v4552=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{(((v33*(v1208*common.v3471))-(v1209*(self.scalar_static_f64[11]*(self.scalar_static_f64[173]*(self.scalar_static_f64[12]*f64::powf(common.v12,self.scalar_static_f64[178]))))))/(v33*v33))}else{common.v304})});
        let v4553=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{((v1208*common.v3472)/v33)}else{common.v304})});
        let v4554=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{((common.v946+(v1208*common.v3473))/v33)}else{common.v304})});
        let v4555=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{(((v1208*common.v3474)+(-common.v946))/v33)}else{common.v304})});
        let v4556=(if self.scalar_static_bool[56]{common.v304}else{(if self.scalar_static_bool[55]{((v1208*common.v3475)/v33)}else{common.v304})});
        let v4566=(if self.scalar_static_bool[58]{common.v304}else{(if self.scalar_static_bool[57]{(common.v51/v37)}else{common.v304})});
        let v4567=(if self.scalar_static_bool[58]{common.v304}else{(if self.scalar_static_bool[57]{((-(v1216*(self.scalar_static_f64[13]*(self.scalar_static_f64[173]*(self.scalar_static_f64[14]*f64::powf(common.v12,self.scalar_static_f64[179]))))))/(v37*v37))}else{common.v304})});
        let v4568=(if self.scalar_static_bool[58]{common.v304}else{(if self.scalar_static_bool[57]{(common.v360/v37)}else{common.v304})});
        let v4592=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{(((v45*(v1222*(if self.scalar_static_bool[28]{common.v304}else{v3596})))-(v1223*(self.scalar_static_f64[17]*(self.scalar_static_f64[173]*(self.scalar_static_f64[18]*f64::powf(common.v12,self.scalar_static_f64[181]))))))/(v45*v45))}else{common.v304})});
        let v4593=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{((-v987)/v45)}else{common.v304})});
        let v4594=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{((v1222*(if self.scalar_static_bool[28]{common.v304}else{v3597}))/v45)}else{common.v304})});
        let v4595=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{((v1222*(if self.scalar_static_bool[28]{common.v304}else{v3598}))/v45)}else{common.v304})});
        let v4596=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{((v1222*(if self.scalar_static_bool[28]{common.v304}else{v3599}))/v45)}else{common.v304})});
        let v4597=(if self.scalar_static_bool[60]{common.v304}else{(if self.scalar_static_bool[59]{((v987+(v1222*(if self.scalar_static_bool[28]{common.v304}else{v3600})))/v45)}else{common.v304})});
        let v4610=scalar_limexp_derivative(v1233);
        let v4634=scalar_limexp_derivative(v1238);
        let v4673=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{(((v1241*(self.scalar_static_f64[60]*(((v142*v1442)+(v87*(v142*(((common.v11*self.scalar_static_f64[201])-(v140*common.v1345))/common.v1396))))*(self.scalar_static_f64[64]*f64::powf(v143,self.scalar_static_f64[202])))))+(v147*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{((-(common.v825*self.scalar_static_f64[289]))/(v1231*v1231))}else{common.v4362})*v4610)}else{common.v4366})))+((v1243*(self.scalar_static_f64[65]*(((v153*v1460)+(v100*(v153*(((common.v11*self.scalar_static_f64[203])-(v151*common.v1345))/common.v1396))))*(self.scalar_static_f64[69]*f64::powf(v154,self.scalar_static_f64[204])))))+(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{((-(common.v825*self.scalar_static_f64[290]))/(v1236*v1236))}else{v4172})*v4634)}else{v4183}))))}else{common.v304})});
        let v4674=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{((v147*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{common.v304}else{common.v4363})*v4610)}else{common.v4367}))+(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{common.v304}else{v4173})*v4634)}else{v4184})))}else{common.v304})});
        let v4675=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{common.v304}else{v4174})*v4634)}else{v4185}))}else{common.v304})});
        let v4676=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{((v147*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{common.v304}else{common.v4364})*v4610)}else{common.v4368}))+(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{common.v304}else{v4175})*v4634)}else{v4186})))}else{common.v304})});
        let v4677=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{((v147*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{(common.v360/v1231)}else{common.v304})*v4610)}else{common.v304}))+(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{(common.v360/v1236)}else{v4176})*v4634)}else{v4187})))}else{common.v304})});
        let v4678=(if self.scalar_static_bool[64]{common.v304}else{(if self.scalar_static_bool[63]{((v147*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{(common.v51/v1231)}else{common.v304})*v4610)}else{common.v304}))+(v158*(if self.scalar_static_bool[63]{((if self.scalar_static_bool[63]{(common.v51/v1236)}else{common.v304})*v4634)}else{common.v304})))}else{common.v304})});
        let v4688=(if self.scalar_static_bool[66]{common.v304}else{(if self.scalar_static_bool[65]{(common.v51/v41)}else{common.v304})});
        let v4689=(if self.scalar_static_bool[66]{common.v304}else{(if self.scalar_static_bool[65]{((-(v1251*(self.scalar_static_f64[15]*(self.scalar_static_f64[173]*(self.scalar_static_f64[16]*f64::powf(common.v12,self.scalar_static_f64[180]))))))/(v41*v41))}else{common.v304})});
        let v4690=(if self.scalar_static_bool[66]{common.v304}else{(if self.scalar_static_bool[65]{(common.v360/v41)}else{common.v304})});

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1074),
            [4, 6, 7, 8, 9, 10, 11],
            [v4001, v4002, v4003, v4004, v4005, v4006, v4007],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1096),
            [4, 6, 7, 8, 9, 10, 11],
            [v4108, v4109, v4110, v4111, v4112, v4113, v4114],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (v948),
            [4, 6, 7, 8, 9],
            [v3498, v3501, v3504, v3508, v3512],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (v947),
            [4, 6, 7, 8, 9],
            [v3480, v3484, v3487, v3491, v3494],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1151),
            [4, 6, 7, 8, 9, 10, 11],
            [v4341, v4342, v4343, v4344, v4345, v4346, v4347],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1126),
            [4, 6, 7, 8, 10],
            [v4212, v4213, v4214, v4215, v4216],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1158),
            0,
            multiplicity * (v4357),
            4,
            multiplicity * (v4358),
            5,
            multiplicity * (v4359),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1199),
            [4, 5, 6, 8],
            [v4514, v4515, v4516, v4517],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1206),
            1,
            multiplicity * (v4527),
            4,
            multiplicity * (v4528),
            7,
            multiplicity * (v4529),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1213),
            [4, 6, 7, 8, 9],
            [v4552, v4553, v4554, v4555, v4556],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1220),
            2,
            multiplicity * (v4566),
            4,
            multiplicity * (v4567),
            9,
            multiplicity * (v4568),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1227),
            [4, 5, 6, 7, 8, 10],
            [v4592, v4593, v4594, v4595, v4596, v4597],
            [],
            [],
            multiplicity,
        );
        let v1283_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v1283);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1283_ddt),
            [4, 6, 7, 8, 9],
            [((common.v4796) * ddt_scale), ((common.v4783) * ddt_scale), ((common.v4787) * ddt_scale), ((common.v4797) * ddt_scale), ((common.v4798) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1285_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v1285);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1285_ddt),
            [4, 7, 8, 9],
            [((common.v4805) * ddt_scale), ((common.v4806) * ddt_scale), ((common.v4807) * ddt_scale), ((common.v4808) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1292_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v1292);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1292_ddt),
            [4, 6, 7, 8, 9],
            [((common.v4825) * ddt_scale), ((common.v4826) * ddt_scale), ((common.v4813) * ddt_scale), ((common.v4827) * ddt_scale), ((common.v4815) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1293_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v1293);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v1293_ddt),
            4,
            multiplicity * (((common.v4828) * ddt_scale)),
            5,
            multiplicity * (((common.v4829) * ddt_scale)),
            8,
            multiplicity * (((common.v4830) * ddt_scale)),
        );
        let v1296_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v1296);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1296_ddt),
            [4, 6, 7, 8, 9, 10],
            [((common.v4844) * ddt_scale), ((common.v4845) * ddt_scale), ((common.v4846) * ddt_scale), ((common.v4847) * ddt_scale), ((common.v4837) * ddt_scale), ((common.v4848) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1303_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v1303);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v1303_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[169]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[295]) * ddt_scale)),
        );
        let v1306_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v1306);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v1306_ddt),
            0,
            multiplicity * (((self.scalar_static_f64[296]) * ddt_scale)),
            1,
            multiplicity * (((self.scalar_static_f64[170]) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1248),
            [4, 6, 7, 8, 10, 11],
            [v4673, v4674, v4675, v4676, v4677, v4678],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (v988),
            [4, 6, 7, 8, 10, 11],
            [v3680, v3681, v3682, v3683, v3684, v3685],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1255),
            3,
            multiplicity * (v4688),
            4,
            multiplicity * (v4689),
            11,
            multiplicity * (v4690),
        );
        let v1300_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v1300);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1300_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v4851) * ddt_scale), ((common.v4852) * ddt_scale), ((common.v4853) * ddt_scale), ((common.v4854) * ddt_scale), ((common.v4855) * ddt_scale), ((common.v4859) * ddt_scale), ((common.v4860) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[68]{common.v304}else{(if self.scalar_static_bool[67]{(common.v6/self.scalar_static_f64[171])}else{common.v304})})),
            4,
            multiplicity * (self.scalar_static_f64[299]),
        );
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * ((-((((((((((((((common.v341*v1074)+(common.v345*v1151))+(v1145*v1310))+(common.v343*v1096))+(common.v349*v1126))+(v1251*v1255))+(common.v825*v1248))+(v988*v1321))+(v1154*v1158))+(v1174*v1199))+(v1202*v1206))+(v1208*v1213))+(v1216*v1220))+(v1222*v1227)))),
            &[(-(v1158+(v1154*v4357))),(-(v1206+(v1202*v4527))),(-(v1220+(v1216*v4566))),(-(v1255+(v1251*v4688))),(-((((((((((((((common.v341*v4001)+(common.v345*v4341))+(v1310*v4302))+(common.v343*v4108))+(common.v349*v4212))+(v1251*v4689))+(common.v825*v4673))+(v1321*v3680))+(v1154*v4358))+(v1174*v4514))+(v1202*v4528))+(v1208*v4552))+(v1216*v4567))+(v1222*v4592))),(-((((-v1158)+(v1154*v4359))+(v1199+(v1174*v4515)))+((-v1227)+(v1222*v4593)))),(-((((((((((common.v341*v4002)+((-v1151)+(common.v345*v4342)))+(v1145+(v1310*v4303)))+(common.v343*v4109))+(common.v349*v4213))+(common.v825*v4674))+(v1321*v3681))+((-v1199)+(v1174*v4516)))+(v1208*v4553))+(v1222*v4594))),(-((((((((((common.v341*v4003)+(common.v345*v4343))+(v1310*v4304))+(v1096+(common.v343*v4110)))+(v1126+(common.v349*v4214)))+(common.v825*v4675))+(v988+(v1321*v3682)))+((-v1206)+(v1202*v4529)))+(v1213+(v1208*v4554)))+(v1222*v4595))),(-((((((((((v1074+(common.v341*v4004))+(v1151+(common.v345*v4344)))+(v1310*v4305))+(common.v343*v4111))+(common.v349*v4215))+(common.v825*v4676))+(v1321*v3683))+(v1174*v4517))+((-v1213)+(v1208*v4555)))+(v1222*v4596))),(-(((((((-v1074)+(common.v341*v4005))+(common.v345*v4345))+((v1310*v4306)+(-v1145)))+((-v1096)+(common.v343*v4112)))+(v1208*v4556))+((-v1220)+(v1216*v4568)))),(-(((((((common.v341*v4006)+(common.v345*v4346))+(common.v343*v4113))+((-v1126)+(common.v349*v4216)))+((-v1248)+(common.v825*v4677)))+(v1321*v3684))+(v1227+(v1222*v4597)))),(-((((((common.v341*v4007)+(common.v345*v4347))+(common.v343*v4114))+((-v1255)+(v1251*v4690)))+(v1248+(common.v825*v4678)))+((v1321*v3685)+(-v988))))],
            &[],
            multiplicity,
        );
        let v1344_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v1344);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v1344_ddt),
            4,
            multiplicity * (((self.scalar_static_f64[172]) * ddt_scale)),
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
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[common.v4796, common.v4783, common.v4787, common.v4797, common.v4798],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &[nodes[4], nodes[7], nodes[8], nodes[9]],
            &[common.v4805, common.v4806, common.v4807, common.v4808],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[common.v4825, common.v4826, common.v4813, common.v4827, common.v4815],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (common.v4828),
            nodes[5],
            multiplicity * (common.v4829),
            nodes[8],
            multiplicity * (common.v4830),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[common.v4844, common.v4845, common.v4846, common.v4847, common.v4837, common.v4848],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[169]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[295]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (self.scalar_static_f64[296]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[170]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[common.v4851, common.v4852, common.v4853, common.v4854, common.v4855, common.v4859, common.v4860],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (self.scalar_static_f64[172]),
        );
    }
}
