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
    v6: f64, v11: f64, v12: f64, v13: f64, v51: f64, v52: f64, 
    v84: f64, v182: f64, v185: f64, v209: f64, v247: f64, v304: f64, 
    v339: f64, v340: f64, v341: f64, v342: f64, v343: f64, v344: f64, 
    v345: f64, v346: f64, v348: f64, v349: f64, v362: f64, v835: f64, 
    v836: f64, v896: f64, v917: f64, v923: f64, v959: f64, v964: f64, 
    v966: f64, v968: f64, v970: f64, v972: f64, v980: f64, v1173: f64, 
    v1179: f64, v1180: f64, v1185: f64, v1188: f64, v1223: f64, v1239: f64, 
    v1310: f64, v1312: f64, v1319: f64, v1320: f64, v1323: f64, v1327: f64, 
    v1330: f64, v1333: f64, v1372: f64, v1373: f64, v1424: f64, v1466: f64, 
    v1654: f64, v3257: f64, v3258: f64, v3259: f64, v3260: f64, v3261: f64, 
    v3262: f64, v3263: f64, v3365: f64, v3366: f64, v3367: f64, v3386: f64, 
    v3387: f64, v3388: f64, v3499: f64, v3500: f64, v3501: f64, v3502: f64, 
    v3503: f64, v3507: f64, v3544: f64, v3547: f64, v3548: f64, v3549: f64, 
    v3550: f64, v3551: f64, v3552: f64, v3559: f64, v3560: f64, v3561: f64, 
    v3562: f64, v3563: f64, v3567: f64, v3572: f64, v3573: f64, v3574: f64, 
    v3593: f64, v3594: f64, v3595: f64, v3596: f64, v3597: f64, v4390: f64, 
    v4391: f64, v4392: f64, v4394: f64, v4395: f64, v4396: f64, v4410: f64, 
    v4411: f64, v4412: f64, v4419: f64, v4420: f64, v4421: f64, v4811: f64, 
    v4815: f64, v4824: f64, v4825: f64, v4826: f64, v4833: f64, v4834: f64, 
    v4835: f64, v4836: f64, v4841: f64, v4843: f64, v4853: f64, v4854: f64, 
    v4855: f64, v4856: f64, v4857: f64, v4858: f64, v4865: f64, v4872: f64, 
    v4873: f64, v4874: f64, v4875: f64, v4876: f64, v4879: f64, v4880: f64, 
    v4881: f64, v4882: f64, v4883: f64, v4887: f64, v4888: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v6=ctx.node_voltage(nodes[4]);
        let v7=(self.scalar_static_f64[321]+v6);
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
        let v356=(v341+v352);
        let v357=(if (self.scalar_static_f64[121]!=0.0){v356}else{v304});
        let v359=(if (v357>v304){v51}else{v304});
        let v360=((self.scalar_static_f64[121]!=0.0)&&(v359!=0.0));
        let v362=-1.0;
        let v365=(if v360{self.scalar_static_f64[124]}else{v304});
        let v368=(v51-(self.scalar_static_f64[122]*(self.scalar_static_f64[122]*v365)));
        let v374=(v357*self.scalar_static_f64[126]);
        let v376=(self.scalar_static_f64[122]+(v374/v220));
        let v381=((self.scalar_static_f64[121]!=0.0)&&(!(v359!=0.0)));
        let v383=(v51-(v341/v220));
        let v385=(v51-f64::powf(v383,self.scalar_static_f64[125]));
        let v388=(if v381{((v220*v385)/self.scalar_static_f64[125])}else{(if v360{((v220*v368)/self.scalar_static_f64[125])}else{v304})});
        let v389=(if v381{v304}else{(if v360{(v365*(v357*v376))}else{v304})});
        let v397=(((v352*v352)+self.scalar_static_f64[128])).sqrt();
        let v398=(if self.scalar_static_bool[10]{v397}else{v304});
        let v401=(if self.scalar_static_bool[10]{(v191*(v352+v398))}else{v304});
        let v403=(v51-(v401/v220));
        let v404=f64::powf(v403,self.scalar_static_f64[125]);
        let v407=(if self.scalar_static_bool[10]{((v350*v404)/self.scalar_static_f64[125])}else{v304});
        let v408=(if self.scalar_static_bool[10]{v356}else{v304});
        let v411=((self.scalar_static_f64[128]+(v408*v408))).sqrt();
        let v412=(if self.scalar_static_bool[10]{v411}else{v304});
        let v416=(if self.scalar_static_bool[10]{((v185*(v408-v412))-v352)}else{v304});
        let v418=(v51-(v416/v220));
        let v419=f64::powf(v418,self.scalar_static_f64[125]);
        let v422=(if self.scalar_static_bool[10]{((v350*v419)/self.scalar_static_f64[125])}else{v388});
        let v430=(if self.scalar_static_bool[10]{((v422+(self.scalar_static_f64[130]*(v401+(v341-v416))))-v407)}else{(if (self.scalar_static_f64[121]!=0.0){(v388+v389)}else{v304})});
        let v431=(v343+v352);
        let v432=(if (self.scalar_static_f64[121]!=0.0){v431}else{v357});
        let v434=(if (v432>v304){v51}else{v304});
        let v435=((self.scalar_static_f64[121]!=0.0)&&(v434!=0.0));
        let v436=(if v435{self.scalar_static_f64[124]}else{v365});
        let v439=(v51-(self.scalar_static_f64[122]*(self.scalar_static_f64[122]*v436)));
        let v443=(self.scalar_static_f64[126]*v432);
        let v445=(self.scalar_static_f64[122]+(v443/v220));
        let v450=((self.scalar_static_f64[121]!=0.0)&&(!(v434!=0.0)));
        let v452=(v51-(v343/v220));
        let v454=(v51-f64::powf(v452,self.scalar_static_f64[125]));
        let v457=(if v450{((v220*v454)/self.scalar_static_f64[125])}else{(if v435{((v220*v439)/self.scalar_static_f64[125])}else{v422})});
        let v458=(if v450{v304}else{(if v435{(v436*(v432*v445))}else{v389})});
        let v461=(if self.scalar_static_bool[10]{v397}else{v398});
        let v464=(if self.scalar_static_bool[10]{(v191*(v352+v461))}else{v401});
        let v466=(v51-(v464/v220));
        let v467=f64::powf(v466,self.scalar_static_f64[125]);
        let v470=(if self.scalar_static_bool[10]{((v350*v467)/self.scalar_static_f64[125])}else{v407});
        let v471=(if self.scalar_static_bool[10]{v431}else{v408});
        let v474=((self.scalar_static_f64[128]+(v471*v471))).sqrt();
        let v475=(if self.scalar_static_bool[10]{v474}else{v412});
        let v479=(if self.scalar_static_bool[10]{((v185*(v471-v475))-v352)}else{v416});
        let v481=(v51-(v479/v220));
        let v482=f64::powf(v481,self.scalar_static_f64[125]);
        let v485=(if self.scalar_static_bool[10]{((v350*v482)/self.scalar_static_f64[125])}else{v457});
        let v491=(if self.scalar_static_bool[10]{((v485+(self.scalar_static_f64[130]*(v464+(v343-v479))))-v470)}else{(if (self.scalar_static_f64[121]!=0.0){(v457+v458)}else{v304})});
        let v492=(-v247);
        let v493=(self.scalar_static_f64[119]*v492);
        let v497=(v345+v493);
        let v498=(if (self.scalar_static_f64[132]!=0.0){v497}else{v432});
        let v500=(if (v498>v304){v51}else{v304});
        let v501=((self.scalar_static_f64[132]!=0.0)&&(v500!=0.0));
        let v504=(if v501{self.scalar_static_f64[134]}else{v436});
        let v507=(v51-(self.scalar_static_f64[122]*(self.scalar_static_f64[122]*v504)));
        let v513=(v498*self.scalar_static_f64[136]);
        let v515=(self.scalar_static_f64[122]+(v513/v247));
        let v524=(if (self.scalar_static_bool[12]&&(v345<self.scalar_static_f64[138])){v51}else{v304});
        let v526=((self.scalar_static_f64[132]!=0.0)&&(!(v500!=0.0)));
        let v527=((v524!=0.0)&&v526);
        let v529=(v51+(self.scalar_static_f64[137]/v247));
        let v530=f64::powf(v529,self.scalar_static_f64[135]);
        let v532=(self.scalar_static_f64[135]*(v345+self.scalar_static_f64[137]));
        let v533=(v247+self.scalar_static_f64[137]);
        let v535=(v51-(v532/v533));
        let v537=(v51-(v530*v535));
        let v542=(v526&&(!(v524!=0.0)));
        let v544=(v51-(v345/v247));
        let v546=(v51-f64::powf(v544,self.scalar_static_f64[135]));
        let v549=(if v542{((v247*v546)/self.scalar_static_f64[135])}else{(if v527{((v247*v537)/self.scalar_static_f64[135])}else{(if v501{((v247*v507)/self.scalar_static_f64[135])}else{v485})})});
        let v550=(if v526{v304}else{(if v501{(v504*(v498*v515))}else{v458})});
        let v559=(v493+self.scalar_static_f64[137]);
        let v560=(self.scalar_static_f64[137]-v493);
        let v561=(v559/v560);
        let v562=(if self.scalar_static_bool[16]{v561}else{v304});
        let v563=(v182*v562);
        let v564=(v562-v51);
        let v569=(((v564*v564)+self.scalar_static_f64[142])).sqrt();
        let v570=(v51+v562);
        let v575=(((v570*v570)+self.scalar_static_f64[144])).sqrt();
        let v576=(v569+v575);
        let v578=(if self.scalar_static_bool[16]{(v563/v576)}else{v304});
        let v583=(if self.scalar_static_bool[16]{(v185*(((v560*v578)-self.scalar_static_f64[137])-v493))}else{v464});
        let v585=(v51-(v583/v247));
        let v587=(v51-f64::powf(v585,self.scalar_static_f64[135]));
        let v590=(if self.scalar_static_bool[16]{((v247*v587)/self.scalar_static_f64[135])}else{v304});
        let v593=(v493+(self.scalar_static_f64[137]+(v182*v345)));
        let v595=(if self.scalar_static_bool[16]{(v593/v560)}else{v304});
        let v596=(v182*v595);
        let v597=(v595-v51);
        let v600=((self.scalar_static_f64[142]+(v597*v597))).sqrt();
        let v601=(v51+v595);
        let v604=((self.scalar_static_f64[144]+(v601*v601))).sqrt();
        let v605=(v600+v604);
        let v607=(if self.scalar_static_bool[16]{(v596/v605)}else{v304});
        let v612=(if self.scalar_static_bool[16]{(v185*(((v560*v607)-self.scalar_static_f64[137])-v493))}else{v479});
        let v614=(v51-(v612/v247));
        let v616=(v51-f64::powf(v614,self.scalar_static_f64[135]));
        let v619=(if self.scalar_static_bool[16]{((v247*v616)/self.scalar_static_f64[135])}else{v549});
        let v622=(if self.scalar_static_bool[16]{(v185*(v51+v607))}else{v304});
        let v624=f64::powf(v529,self.scalar_static_f64[145]);
        let v625=(if self.scalar_static_bool[16]{v624}else{v304});
        let v627=(v51+(v493/v247));
        let v628=f64::powf(v627,self.scalar_static_f64[145]);
        let v629=(if self.scalar_static_bool[16]{v628}else{v304});
        let v630=(v51-v622);
        let v634=(if self.scalar_static_bool[16]{((v625*v630)+(v622*v629))}else{v304});
        let v636=(v583+(v345-v612));
        let v638=(if self.scalar_static_bool[16]{(v634*v636)}else{v304});
        let v646=((self.scalar_static_f64[142]+(v493*v493))).sqrt();
        let v647=(if self.scalar_static_bool[18]{v646}else{v461});
        let v650=(if self.scalar_static_bool[18]{(v191*(v493+v647))}else{v583});
        let v652=(v51-(v650/v247));
        let v653=f64::powf(v652,self.scalar_static_f64[135]);
        let v656=(if self.scalar_static_bool[18]{((v492*v653)/self.scalar_static_f64[135])}else{v470});
        let v657=(if self.scalar_static_bool[18]{v497}else{v471});
        let v660=((self.scalar_static_f64[142]+(v657*v657))).sqrt();
        let v661=(if self.scalar_static_bool[18]{v660}else{v475});
        let v665=(if self.scalar_static_bool[18]{((v185*(v657-v661))-v493)}else{v612});
        let v667=(v51-(v665/v247));
        let v668=f64::powf(v667,self.scalar_static_f64[135]);
        let v671=(if self.scalar_static_bool[18]{((v492*v668)/self.scalar_static_f64[135])}else{v619});
        let v678=(if self.scalar_static_bool[18]{((v671+(self.scalar_static_f64[146]*(v650+(v345-v665))))-v656)}else{(if self.scalar_static_bool[16]{((v619+v638)-v590)}else{(if (self.scalar_static_f64[132]!=0.0){(v549+v550)}else{v304})})});
        let v679=(v349+v493);
        let v680=(if (self.scalar_static_f64[132]!=0.0){v679}else{v498});
        let v682=(if (v680>v304){v51}else{v304});
        let v683=((self.scalar_static_f64[132]!=0.0)&&(v682!=0.0));
        let v684=(if v683{self.scalar_static_f64[134]}else{v504});
        let v687=(v51-(self.scalar_static_f64[122]*(self.scalar_static_f64[122]*v684)));
        let v691=(self.scalar_static_f64[136]*v680);
        let v693=(self.scalar_static_f64[122]+(v691/v247));
        let v699=(if (self.scalar_static_bool[12]&&(v349<self.scalar_static_f64[138])){v51}else{v304});
        let v701=((self.scalar_static_f64[132]!=0.0)&&(!(v682!=0.0)));
        let v702=((v699!=0.0)&&v701);
        let v704=(self.scalar_static_f64[135]*(v349+self.scalar_static_f64[137]));
        let v706=(v51-(v704/v533));
        let v708=(v51-(v530*v706));
        let v713=(v701&&(!(v699!=0.0)));
        let v715=(v51-(v349/v247));
        let v717=(v51-f64::powf(v715,self.scalar_static_f64[135]));
        let v720=(if v713{((v247*v717)/self.scalar_static_f64[135])}else{(if v702{((v247*v708)/self.scalar_static_f64[135])}else{(if v683{((v247*v687)/self.scalar_static_f64[135])}else{v671})})});
        let v721=(if v701{v304}else{(if v683{(v684*(v680*v693))}else{v550})});
        let v724=(if self.scalar_static_bool[16]{v561}else{v562});
        let v725=(v182*v724);
        let v726=(v724-v51);
        let v729=((self.scalar_static_f64[142]+(v726*v726))).sqrt();
        let v730=(v51+v724);
        let v733=((self.scalar_static_f64[144]+(v730*v730))).sqrt();
        let v734=(v729+v733);
        let v736=(if self.scalar_static_bool[16]{(v725/v734)}else{v578});
        let v741=(if self.scalar_static_bool[16]{(v185*(((v560*v736)-self.scalar_static_f64[137])-v493))}else{v650});
        let v743=(v51-(v741/v247));
        let v745=(v51-f64::powf(v743,self.scalar_static_f64[135]));
        let v751=(v493+(self.scalar_static_f64[137]+(v182*v349)));
        let v753=(if self.scalar_static_bool[16]{(v751/v560)}else{v595});
        let v754=(v182*v753);
        let v755=(v753-v51);
        let v758=((self.scalar_static_f64[142]+(v755*v755))).sqrt();
        let v759=(v51+v753);
        let v762=((self.scalar_static_f64[144]+(v759*v759))).sqrt();
        let v763=(v758+v762);
        let v765=(if self.scalar_static_bool[16]{(v754/v763)}else{v607});
        let v770=(if self.scalar_static_bool[16]{(v185*(((v560*v765)-self.scalar_static_f64[137])-v493))}else{v665});
        let v772=(v51-(v770/v247));
        let v774=(v51-f64::powf(v772,self.scalar_static_f64[135]));
        let v777=(if self.scalar_static_bool[16]{((v247*v774)/self.scalar_static_f64[135])}else{v720});
        let v780=(if self.scalar_static_bool[16]{(v185*(v51+v765))}else{v622});
        let v781=(if self.scalar_static_bool[16]{v624}else{v625});
        let v782=(if self.scalar_static_bool[16]{v628}else{v629});
        let v783=(v51-v780);
        let v787=(if self.scalar_static_bool[16]{((v781*v783)+(v780*v782))}else{v634});
        let v789=(v741+(v349-v770));
        let v795=(if self.scalar_static_bool[18]{v646}else{v647});
        let v798=(if self.scalar_static_bool[18]{(v191*(v493+v795))}else{v741});
        let v800=(v51-(v798/v247));
        let v801=f64::powf(v800,self.scalar_static_f64[135]);
        let v804=(if self.scalar_static_bool[18]{((v492*v801)/self.scalar_static_f64[135])}else{v656});
        let v805=(if self.scalar_static_bool[18]{v679}else{v657});
        let v808=((self.scalar_static_f64[142]+(v805*v805))).sqrt();
        let v809=(if self.scalar_static_bool[18]{v808}else{v661});
        let v813=(if self.scalar_static_bool[18]{((v185*(v805-v809))-v493)}else{v770});
        let v815=(v51-(v813/v247));
        let v816=f64::powf(v815,self.scalar_static_f64[135]);
        let v819=(if self.scalar_static_bool[18]{((v492*v816)/self.scalar_static_f64[135])}else{v777});
        let v825=(if self.scalar_static_bool[18]{((v819+(self.scalar_static_f64[146]*(v798+(v349-v813))))-v804)}else{(if self.scalar_static_bool[16]{((v777+(if self.scalar_static_bool[16]{(v787*v789)}else{v638}))-(if self.scalar_static_bool[16]{((v247*v745)/self.scalar_static_f64[135])}else{v590}))}else{(if (self.scalar_static_f64[132]!=0.0){(v720+v721)}else{v304})})});
        let v828=(-v274);
        let v830=(if (self.scalar_static_f64[147]!=0.0){(self.scalar_static_f64[119]*v828)}else{v493});
        let v835=ctx.node_voltage(nodes[11]);
        let v836=(v835-v348);
        let v837=(v830+v836);
        let v838=(if self.scalar_static_bool[21]{v837}else{v680});
        let v840=(if (v838>v304){v51}else{v304});
        let v841=(self.scalar_static_bool[21]&&(v840!=0.0));
        let v844=(if v841{self.scalar_static_f64[151]}else{v684});
        let v847=(v51-(self.scalar_static_f64[122]*(self.scalar_static_f64[122]*v844)));
        let v853=(v838*self.scalar_static_f64[153]);
        let v855=(self.scalar_static_f64[122]+(v853/v274));
        let v860=(self.scalar_static_bool[21]&&(!(v840!=0.0)));
        let v862=(v51-(v836/v274));
        let v864=(v51-f64::powf(v862,self.scalar_static_f64[152]));
        let v867=(if v860{((v274*v864)/self.scalar_static_f64[152])}else{(if v841{((v274*v847)/self.scalar_static_f64[152])}else{v819})});
        let v877=(((v830*v830)+self.scalar_static_f64[155])).sqrt();
        let v881=(if self.scalar_static_bool[23]{(v191*(v830+(if self.scalar_static_bool[23]{v877}else{v795})))}else{v798});
        let v883=(v51-(v881/v274));
        let v884=f64::powf(v883,self.scalar_static_f64[152]);
        let v888=(if self.scalar_static_bool[23]{v837}else{v805});
        let v891=((self.scalar_static_f64[155]+(v888*v888))).sqrt();
        let v896=(if self.scalar_static_bool[23]{((v185*(v888-(if self.scalar_static_bool[23]{v891}else{v809})))-v830)}else{v813});
        let v898=(v51-(v896/v274));
        let v899=f64::powf(v898,self.scalar_static_f64[152]);
        let v912=(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{(((if self.scalar_static_bool[23]{((v828*v899)/self.scalar_static_f64[152])}else{v867})+(self.scalar_static_f64[157]*(v881+(v836-v896))))-(if self.scalar_static_bool[23]{((v828*v884)/self.scalar_static_f64[152])}else{v804}))}else{(if self.scalar_static_bool[21]{(v867+(if v860{v304}else{(if v841{(v844*(v838*v855))}else{v721})}))}else{v304})})});
        let v913=(v11*v162);
        let v914=(v341/v913);
        let v916=(scalar_limexp(v914)-v51);
        let v917=(v60*v916);
        let v918=(v11*v163);
        let v919=(v345/v918);
        let v920=scalar_limexp(v919);
        let v921=(v60*v73);
        let v922=(v920-v51);
        let v923=(v921*v922);
        let v927=((v51+(self.scalar_static_f64[102]*v430))+(self.scalar_static_f64[99]*v678));
        let v928=0.0001;
        let v929=(v927-v928);
        let v933=(((v929*v929)+1e-8)).sqrt();
        let v937=(v928+(v185*((v927+v933)-v928)));
        let v947=(v209*((v314*v917)+(self.scalar_static_f64[105]*v923)));
        let v948=(f64::powf(v937,self.scalar_static_f64[161])+v947);
        let v954=(v185*v937);
        let v955=(v51+v947);
        let v957=(v51+f64::powf(v955,self.scalar_static_f64[160]));
        let v959=(if self.scalar_static_bool[26]{(v954*v957)}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v937+f64::powf(v948,self.scalar_static_f64[160])))}else{v304})});
        let v964=(v11*self.scalar_static_f64[34]);
        let v966=(if (self.scalar_static_f64[162]!=0.0){(v349/v964)}else{v919});
        let v968=(if (self.scalar_static_f64[162]!=0.0){scalar_limexp(v966)}else{v920});
        let v970=(if (self.scalar_static_f64[162]!=0.0){(v345/v964)}else{v304});
        let v972=(if (self.scalar_static_f64[162]!=0.0){scalar_limexp(v970)}else{v304});
        let v978=(((v968*self.scalar_static_f64[163])+(v972*self.scalar_static_f64[164]))-v51);
        let v980=(if (self.scalar_static_f64[162]!=0.0){(v84*v978)}else{v304});
        let v1173=ctx.node_voltage(nodes[0]);
        let v1179=(v345/v11);
        let v1180=scalar_limexp(v1179);
        let v1181=(v347/v11);
        let v1182=scalar_limexp(v1181);
        let v1185=((v51+(v294*v1180))).sqrt();
        let v1188=((v51+(v294*v1182))).sqrt();
        let v1223=ctx.node_voltage(nodes[1]);
        let v1239=ctx.node_voltage(nodes[2]);
        let v1284=(if (v917>v304){v51}else{v304});
        let v1286=(self.scalar_static_f64[117]*(v917*v1284));
        let v1287=(v51+v1286);
        let v1288=(v1286/v1287);
        let v1293=(self.scalar_static_f64[183]*(v51+(v937*self.scalar_static_f64[184])));
        let v1297=((self.scalar_static_f64[114]*v345)/1.44);
        let v1299=(self.scalar_static_f64[185]*scalar_limexp(v1297));
        let v1301=(self.scalar_static_f64[118]+(v1288*v1288));
        let v1304=(v51+(v1284*(v1299*v1301)));
        let v1305=(v1293*v1304);
        let v1308=(v917*v1305);
        let v1310=((self.scalar_static_f64[165]*(v279*v430))+(v1308/v959));
        let v1312=(self.scalar_static_f64[170]*(v279*v491));
        let v1319=(((v284*v678)+(v923*self.scalar_static_f64[186]))+(v1185*self.scalar_static_f64[187]));
        let v1320=(v1188*self.scalar_static_f64[187]);
        let v1323=((v286*v825)+((if self.scalar_static_bool[28]{v304}else{v980})*self.scalar_static_f64[186]));
        let v1327=((v291*v912)+(v836*self.scalar_static_f64[188]));
        let v1330=((v1223-v1239)*self.scalar_static_f64[189]);
        let v1333=((v1223-v1173)*self.scalar_static_f64[190]);
        let v1372=(v6*self.scalar_static_f64[193]);
        let v1373=8.617342301212761e-5;
        let v1418=(self.scalar_static_f64[194]*(self.scalar_static_f64[20]*f64::powf(v12,self.scalar_static_f64[203])));
        let v1424=(v11*v11);
        let v1426=(v55*(((v11*self.scalar_static_f64[205])-(v53*v1373))/v1424));
        let v1434=(self.scalar_static_f64[19]*(((v55*v1418)+(v48*v1426))*(self.scalar_static_f64[24]*f64::powf(v56,self.scalar_static_f64[206]))));
        let v1466=(self.scalar_static_f64[31]*(((v79*v1418)+(v48*(v79*(((v11*self.scalar_static_f64[210])-(v77*v1373))/v1424))))*(self.scalar_static_f64[35]*f64::powf(v80,self.scalar_static_f64[211]))));
        let v1572=(v182*(((v12*v1373)-(v11*self.scalar_static_f64[194]))/(v12*v12)));
        let v1597=((v202*0.00025852026903638284)+(v201*(self.scalar_static_f64[194]/v12)));
        let v1600=((((v198*self.scalar_static_f64[194])+(v12*((v197*v1572)+(v184*(((v190*(((v11*self.scalar_static_f64[230])-(v188*v1373))/v1424))-(v195*(((v11*self.scalar_static_f64[231])-(v193*v1373))/v1424)))/v196)))))-v1597)-self.scalar_static_f64[232]);
        let v1601=0.00017234684602425522;
        let v1616=(v1600+((v218*v1601)+(v208*((v185*((v209*(v212*(((v11*(-v1600))-(v210*v1373))/v1424)))/(v182*v215)))/v217))));
        let v1639=((((v232*self.scalar_static_f64[194])+(v12*((v231*v1572)+(v184*(((v225*(((v11*self.scalar_static_f64[233])-(v223*v1373))/v1424))-(v229*(((v11*self.scalar_static_f64[234])-(v227*v1373))/v1424)))/v230)))))-v1597)-self.scalar_static_f64[235]);
        let v1654=(v1639+((v245*v1601)+(v208*((v185*((v209*(v239*(((v11*(-v1639))-(v237*v1373))/v1424)))/(v182*v242)))/v244))));
        let v1677=((((v259*self.scalar_static_f64[194])+(v12*((v258*v1572)+(v184*(((v252*(((v11*self.scalar_static_f64[236])-(v250*v1373))/v1424))-(v256*(((v11*self.scalar_static_f64[237])-(v254*v1373))/v1424)))/v257)))))-v1597)-self.scalar_static_f64[238]);
        let v1692=(v1677+((v272*v1601)+(v208*((v185*((v209*(v266*(((v11*(-v1677))-(v264*v1373))/v1424)))/(v182*v269)))/v271))));
        let v1695=(v220*v220);
        let v1701=(self.scalar_static_f64[87]*(((-(self.scalar_static_f64[78]*v1616))/v1695)*(self.scalar_static_f64[88]*f64::powf(v276,self.scalar_static_f64[239]))));
        let v1704=(v247*v247);
        let v1708=(((-(self.scalar_static_f64[81]*v1654))/v1704)*(self.scalar_static_f64[90]*f64::powf(v281,self.scalar_static_f64[174])));
        let v1713=(v274*v274);
        let v1723=((v293*v1426)+(v55*(self.scalar_static_f64[94]*v1418)));
        let v1747=(-v1616);
        let v1748=(self.scalar_static_f64[119]*v1747);
        let v1749=(if (self.scalar_static_f64[121]!=0.0){v1748}else{v304});
        let v1762=(self.scalar_static_f64[244]/v220);
        let v1785=(-(v51/v220));
        let v1786=(-(v362/v220));
        let v1789=(self.scalar_static_f64[125]*f64::powf(v383,self.scalar_static_f64[246]));
        let v1804=(if v381{(((v385*v1616)+(v220*(-((-((-(v341*v1616))/v1695))*v1789))))/self.scalar_static_f64[125])}else{(if v360{((v368*v1616)/self.scalar_static_f64[125])}else{v304})});
        let v1805=(if v381{((v220*(-(v1785*v1789)))/self.scalar_static_f64[125])}else{v304});
        let v1806=(if v381{((v220*(-(v1786*v1789)))/self.scalar_static_f64[125])}else{v304});
        let v1807=(if v381{v304}else{(if v360{(v365*((v376*v1749)+(v357*(((v220*(self.scalar_static_f64[126]*v1749))-(v374*v1616))/v1695))))}else{v304})});
        let v1808=(if v381{v304}else{(if v360{(v365*((v376*self.scalar_static_f64[242])+(v357*v1762)))}else{v304})});
        let v1809=(if v381{v304}else{(if v360{(v365*((v376*self.scalar_static_f64[243])+(v357*(self.scalar_static_f64[245]/v220))))}else{v304})});
        let v1816=(v352*v1748);
        let v1819=((v1816+v1816)/(v182*v397));
        let v1820=(if self.scalar_static_bool[10]{v1819}else{v304});
        let v1823=(if self.scalar_static_bool[10]{(v191*(v1748+v1820))}else{v304});
        let v1836=(if self.scalar_static_bool[10]{(((v404*v1747)+(v350*((-(((v220*v1823)-(v401*v1616))/v1695))*(self.scalar_static_f64[125]*f64::powf(v403,self.scalar_static_f64[246])))))/self.scalar_static_f64[125])}else{v304});
        let v1837=(if self.scalar_static_bool[10]{v1748}else{v304});
        let v1840=(v408*v1837);
        let v1842=(v408*self.scalar_static_f64[247]);
        let v1844=(v408*self.scalar_static_f64[248]);
        let v1846=(v182*v411);
        let v1850=(if self.scalar_static_bool[10]{((v1840+v1840)/v1846)}else{v304});
        let v1851=(if self.scalar_static_bool[10]{((v1842+v1842)/v1846)}else{v304});
        let v1852=(if self.scalar_static_bool[10]{((v1844+v1844)/v1846)}else{v304});
        let v1860=(if self.scalar_static_bool[10]{((v185*(v1837-v1850))-v1748)}else{v304});
        let v1861=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[247]-v1851))}else{v304});
        let v1862=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[248]-v1852))}else{v304});
        let v1873=(self.scalar_static_f64[125]*f64::powf(v418,self.scalar_static_f64[246]));
        let v1885=(if self.scalar_static_bool[10]{(((v419*v1747)+(v350*((-(((v220*v1860)-(v416*v1616))/v1695))*v1873)))/self.scalar_static_f64[125])}else{v1804});
        let v1886=(if self.scalar_static_bool[10]{((v350*((-(v1861/v220))*v1873))/self.scalar_static_f64[125])}else{v1805});
        let v1887=(if self.scalar_static_bool[10]{((v350*((-(v1862/v220))*v1873))/self.scalar_static_f64[125])}else{v1806});
        let v1899=(if self.scalar_static_bool[10]{((v1885+(self.scalar_static_f64[130]*(v1823+(-v1860))))-v1836)}else{(if (self.scalar_static_f64[121]!=0.0){(v1804+v1807)}else{v304})});
        let v1900=(if self.scalar_static_bool[10]{(v1886+(self.scalar_static_f64[130]*(v51-v1861)))}else{(if (self.scalar_static_f64[121]!=0.0){(v1805+v1808)}else{v304})});
        let v1901=(if self.scalar_static_bool[10]{(v1887+(self.scalar_static_f64[130]*(v362-v1862)))}else{(if (self.scalar_static_f64[121]!=0.0){(v1806+v1809)}else{v304})});
        let v1902=(if (self.scalar_static_f64[121]!=0.0){v1748}else{v1749});
        let v1944=(self.scalar_static_f64[125]*f64::powf(v452,self.scalar_static_f64[246]));
        let v1959=(if v450{(((v454*v1616)+(v220*(-((-((-(v343*v1616))/v1695))*v1944))))/self.scalar_static_f64[125])}else{(if v435{((v439*v1616)/self.scalar_static_f64[125])}else{v1885})});
        let v1960=(if v450{((v220*(-(v1785*v1944)))/self.scalar_static_f64[125])}else{v304});
        let v1961=(if v450{v304}else{(if v435{v304}else{v1886})});
        let v1962=(if v450{((v220*(-(v1786*v1944)))/self.scalar_static_f64[125])}else{(if v435{v304}else{v1887})});
        let v1963=(if v450{v304}else{(if v435{(v436*((v445*v1902)+(v432*(((v220*(self.scalar_static_f64[126]*v1902))-(v443*v1616))/v1695))))}else{v1807})});
        let v1964=(if v450{v304}else{(if v435{(v436*((v445*self.scalar_static_f64[242])+(v432*v1762)))}else{v304})});
        let v1965=(if v450{v304}else{(if v435{(v436*((v445*self.scalar_static_f64[249])+(v432*(self.scalar_static_f64[251]/v220))))}else{v1808})});
        let v1966=(if v450{v304}else{(if v435{(v436*((v445*self.scalar_static_f64[250])+(v432*(self.scalar_static_f64[252]/v220))))}else{v1809})});
        let v1975=(if self.scalar_static_bool[10]{v1819}else{v1820});
        let v1978=(if self.scalar_static_bool[10]{(v191*(v1748+v1975))}else{v1823});
        let v1991=(if self.scalar_static_bool[10]{(((v467*v1747)+(v350*((-(((v220*v1978)-(v464*v1616))/v1695))*(self.scalar_static_f64[125]*f64::powf(v466,self.scalar_static_f64[246])))))/self.scalar_static_f64[125])}else{v1836});
        let v1992=(if self.scalar_static_bool[10]{v1748}else{v1837});
        let v1995=(v471*v1992);
        let v1997=(v471*self.scalar_static_f64[247]);
        let v1999=(v471*self.scalar_static_f64[253]);
        let v2001=(v471*self.scalar_static_f64[254]);
        let v2003=(v182*v474);
        let v2008=(if self.scalar_static_bool[10]{((v1995+v1995)/v2003)}else{v1850});
        let v2009=(if self.scalar_static_bool[10]{((v1997+v1997)/v2003)}else{v304});
        let v2010=(if self.scalar_static_bool[10]{((v1999+v1999)/v2003)}else{v1851});
        let v2011=(if self.scalar_static_bool[10]{((v2001+v2001)/v2003)}else{v1852});
        let v2021=(if self.scalar_static_bool[10]{((v185*(v1992-v2008))-v1748)}else{v1860});
        let v2022=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[247]-v2009))}else{v304});
        let v2023=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[253]-v2010))}else{v1861});
        let v2024=(if self.scalar_static_bool[10]{(v185*(self.scalar_static_f64[254]-v2011))}else{v1862});
        let v2037=(self.scalar_static_f64[125]*f64::powf(v481,self.scalar_static_f64[246]));
        let v2052=(if self.scalar_static_bool[10]{(((v482*v1747)+(v350*((-(((v220*v2021)-(v479*v1616))/v1695))*v2037)))/self.scalar_static_f64[125])}else{v1959});
        let v2053=(if self.scalar_static_bool[10]{((v350*((-(v2022/v220))*v2037))/self.scalar_static_f64[125])}else{v1960});
        let v2054=(if self.scalar_static_bool[10]{((v350*((-(v2023/v220))*v2037))/self.scalar_static_f64[125])}else{v1961});
        let v2055=(if self.scalar_static_bool[10]{((v350*((-(v2024/v220))*v2037))/self.scalar_static_f64[125])}else{v1962});
        let v2074=(-v1654);
        let v2075=(self.scalar_static_f64[119]*v2074);
        let v2076=(if (self.scalar_static_f64[132]!=0.0){v2075}else{v1902});
        let v2096=(self.scalar_static_f64[259]/v247);
        let v2127=((-(self.scalar_static_f64[137]*v1654))/v1704);
        let v2131=(v2127*(self.scalar_static_f64[135]*f64::powf(v529,self.scalar_static_f64[263])));
        let v2135=(v533*v533);
        let v2156=((v247*(-(v530*(-(self.scalar_static_f64[264]/v533)))))/self.scalar_static_f64[135]);
        let v2157=((v247*(-(v530*(-(self.scalar_static_f64[135]/v533)))))/self.scalar_static_f64[135]);
        let v2169=(-(v362/v247));
        let v2170=(-(v51/v247));
        let v2172=(self.scalar_static_f64[135]*f64::powf(v544,self.scalar_static_f64[263]));
        let v2187=(if v542{(((v546*v1654)+(v247*(-((-((-(v345*v1654))/v1704))*v2172))))/self.scalar_static_f64[135])}else{(if v527{(((v537*v1654)+(v247*(-((v535*v2131)+(v530*(-((-(v532*v1654))/v2135)))))))/self.scalar_static_f64[135])}else{(if v501{((v507*v1654)/self.scalar_static_f64[135])}else{v2052})})});
        let v2188=(if v542{((v247*(-(v2169*v2172)))/self.scalar_static_f64[135])}else{(if v527{v2156}else{v304})});
        let v2189=(if v542{v304}else{(if v527{v304}else{(if v501{v304}else{v2053})})});
        let v2190=(if v542{((v247*(-(v2170*v2172)))/self.scalar_static_f64[135])}else{(if v527{v2157}else{(if v501{v304}else{v2054})})});
        let v2191=(if v542{v304}else{(if v527{v304}else{(if v501{v304}else{v2055})})});
        let v2192=(if v526{v304}else{(if v501{(v504*((v515*v2076)+(v498*(((v247*(self.scalar_static_f64[136]*v2076))-(v513*v1654))/v1704))))}else{v1963})});
        let v2193=(if v526{v304}else{(if v501{(v504*((v515*self.scalar_static_f64[255])+(v498*v2096)))}else{v304})});
        let v2194=(if v526{v304}else{(if v501{(v504*((v515*self.scalar_static_f64[256])+(v498*(self.scalar_static_f64[260]/v247))))}else{v1964})});
        let v2195=(if v526{v304}else{(if v501{(v504*((v515*self.scalar_static_f64[257])+(v498*(self.scalar_static_f64[261]/v247))))}else{v1965})});
        let v2196=(if v526{v304}else{(if v501{(v504*((v515*self.scalar_static_f64[258])+(v498*(self.scalar_static_f64[262]/v247))))}else{v1966})});
        let v2207=(-v2075);
        let v2208=(v560*v2075);
        let v2211=(v560*v560);
        let v2212=((v2208-(v559*v2207))/v2211);
        let v2213=(if self.scalar_static_bool[16]{v2212}else{v304});
        let v2215=(v564*v2213);
        let v2219=(v570*v2213);
        let v2229=(if self.scalar_static_bool[16]{(((v576*(v182*v2213))-(v563*(((v2215+v2215)/(v182*v569))+((v2219+v2219)/(v182*v575)))))/(v576*v576))}else{v304});
        let v2235=(if self.scalar_static_bool[16]{(v185*(((v578*v2207)+(v560*v2229))-v2075))}else{v1978});
        let v2249=(if self.scalar_static_bool[16]{(((v587*v1654)+(v247*(-((-(((v247*v2235)-(v583*v1654))/v1704))*(self.scalar_static_f64[135]*f64::powf(v585,self.scalar_static_f64[263]))))))/self.scalar_static_f64[135])}else{v304});
        let v2256=(if self.scalar_static_bool[16]{((v2208-(v593*v2207))/v2211)}else{v304});
        let v2257=(if self.scalar_static_bool[16]{(-2.0/v560)}else{v304});
        let v2258=(if self.scalar_static_bool[16]{(v182/v560)}else{v304});
        let v2260=(v182*v2257);
        let v2261=(v182*v2258);
        let v2262=(v597*v2256);
        let v2264=(v597*v2257);
        let v2266=(v597*v2258);
        let v2268=(v182*v600);
        let v2272=(v601*v2256);
        let v2274=(v601*v2257);
        let v2276=(v601*v2258);
        let v2278=(v182*v604);
        let v2288=(v605*v605);
        let v2298=(if self.scalar_static_bool[16]{(((v605*(v182*v2256))-(v596*(((v2262+v2262)/v2268)+((v2272+v2272)/v2278))))/v2288)}else{v304});
        let v2299=(if self.scalar_static_bool[16]{(((v605*v2260)-(v596*(((v2264+v2264)/v2268)+((v2274+v2274)/v2278))))/v2288)}else{v304});
        let v2300=(if self.scalar_static_bool[16]{(((v605*v2261)-(v596*(((v2266+v2266)/v2268)+((v2276+v2276)/v2278))))/v2288)}else{v304});
        let v2310=(if self.scalar_static_bool[16]{(v185*(((v607*v2207)+(v560*v2298))-v2075))}else{v2021});
        let v2311=(if self.scalar_static_bool[16]{(v185*(v560*v2299))}else{v304});
        let v2312=(if self.scalar_static_bool[16]{v304}else{v2022});
        let v2313=(if self.scalar_static_bool[16]{(v185*(v560*v2300))}else{v2023});
        let v2314=(if self.scalar_static_bool[16]{v304}else{v2024});
        let v2329=(self.scalar_static_f64[135]*f64::powf(v614,self.scalar_static_f64[263]));
        let v2352=(if self.scalar_static_bool[16]{(((v616*v1654)+(v247*(-((-(((v247*v2310)-(v612*v1654))/v1704))*v2329))))/self.scalar_static_f64[135])}else{v2187});
        let v2353=(if self.scalar_static_bool[16]{((v247*(-((-(v2311/v247))*v2329)))/self.scalar_static_f64[135])}else{v2188});
        let v2354=(if self.scalar_static_bool[16]{((v247*(-((-(v2312/v247))*v2329)))/self.scalar_static_f64[135])}else{v2189});
        let v2355=(if self.scalar_static_bool[16]{((v247*(-((-(v2313/v247))*v2329)))/self.scalar_static_f64[135])}else{v2190});
        let v2356=(if self.scalar_static_bool[16]{((v247*(-((-(v2314/v247))*v2329)))/self.scalar_static_f64[135])}else{v2191});
        let v2360=(if self.scalar_static_bool[16]{(v185*v2298)}else{v304});
        let v2361=(if self.scalar_static_bool[16]{(v185*v2299)}else{v304});
        let v2362=(if self.scalar_static_bool[16]{(v185*v2300)}else{v304});
        let v2366=(v2127*(self.scalar_static_f64[145]*f64::powf(v529,self.scalar_static_f64[265])));
        let v2367=(if self.scalar_static_bool[16]{v2366}else{v304});
        let v2374=((((v247*v2075)-(v493*v1654))/v1704)*(self.scalar_static_f64[145]*f64::powf(v627,self.scalar_static_f64[265])));
        let v2375=(if self.scalar_static_bool[16]{v2374}else{v304});
        let v2392=(if self.scalar_static_bool[16]{(((v630*v2367)+(v625*(-v2360)))+((v629*v2360)+(v622*v2375)))}else{v304});
        let v2393=(if self.scalar_static_bool[16]{((v625*(-v2361))+(v629*v2361))}else{v304});
        let v2394=(if self.scalar_static_bool[16]{((v625*(-v2362))+(v629*v2362))}else{v304});
        let v2412=(if self.scalar_static_bool[16]{((v636*v2392)+(v634*(v2235+(-v2310))))}else{v304});
        let v2413=(if self.scalar_static_bool[16]{((v636*v2393)+(v634*(v362-v2311)))}else{v304});
        let v2414=(if self.scalar_static_bool[16]{(v634*(-v2312))}else{v304});
        let v2415=(if self.scalar_static_bool[16]{((v636*v2394)+(v634*(v51-v2313)))}else{v304});
        let v2416=(if self.scalar_static_bool[16]{(v634*(-v2314))}else{v304});
        let v2428=(v493*v2075);
        let v2431=((v2428+v2428)/(v182*v646));
        let v2432=(if self.scalar_static_bool[18]{v2431}else{v1975});
        let v2435=(if self.scalar_static_bool[18]{(v191*(v2075+v2432))}else{v2235});
        let v2448=(if self.scalar_static_bool[18]{(((v653*v2074)+(v492*((-(((v247*v2435)-(v650*v1654))/v1704))*(self.scalar_static_f64[135]*f64::powf(v652,self.scalar_static_f64[263])))))/self.scalar_static_f64[135])}else{v1991});
        let v2449=(if self.scalar_static_bool[18]{v2075}else{v1992});
        let v2454=(v657*v2449);
        let v2456=(v657*self.scalar_static_f64[266]);
        let v2458=(v657*self.scalar_static_f64[267]);
        let v2460=(v657*self.scalar_static_f64[268]);
        let v2462=(v657*self.scalar_static_f64[269]);
        let v2464=(v182*v660);
        let v2470=(if self.scalar_static_bool[18]{((v2454+v2454)/v2464)}else{v2008});
        let v2471=(if self.scalar_static_bool[18]{((v2456+v2456)/v2464)}else{v304});
        let v2472=(if self.scalar_static_bool[18]{((v2458+v2458)/v2464)}else{v2009});
        let v2473=(if self.scalar_static_bool[18]{((v2460+v2460)/v2464)}else{v2010});
        let v2474=(if self.scalar_static_bool[18]{((v2462+v2462)/v2464)}else{v2011});
        let v2486=(if self.scalar_static_bool[18]{((v185*(v2449-v2470))-v2075)}else{v2310});
        let v2487=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[266]-v2471))}else{v2311});
        let v2488=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[267]-v2472))}else{v2312});
        let v2489=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[268]-v2473))}else{v2313});
        let v2490=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[269]-v2474))}else{v2314});
        let v2505=(self.scalar_static_f64[135]*f64::powf(v667,self.scalar_static_f64[263]));
        let v2523=(if self.scalar_static_bool[18]{(((v668*v2074)+(v492*((-(((v247*v2486)-(v665*v1654))/v1704))*v2505)))/self.scalar_static_f64[135])}else{v2352});
        let v2524=(if self.scalar_static_bool[18]{((v492*((-(v2487/v247))*v2505))/self.scalar_static_f64[135])}else{v2353});
        let v2525=(if self.scalar_static_bool[18]{((v492*((-(v2488/v247))*v2505))/self.scalar_static_f64[135])}else{v2354});
        let v2526=(if self.scalar_static_bool[18]{((v492*((-(v2489/v247))*v2505))/self.scalar_static_f64[135])}else{v2355});
        let v2527=(if self.scalar_static_bool[18]{((v492*((-(v2490/v247))*v2505))/self.scalar_static_f64[135])}else{v2356});
        let v2545=(if self.scalar_static_bool[18]{((v2523+(self.scalar_static_f64[146]*(v2435+(-v2486))))-v2448)}else{(if self.scalar_static_bool[16]{((v2352+v2412)-v2249)}else{(if (self.scalar_static_f64[132]!=0.0){(v2187+v2192)}else{v304})})});
        let v2546=(if self.scalar_static_bool[18]{(v2524+(self.scalar_static_f64[146]*(v362-v2487)))}else{(if self.scalar_static_bool[16]{(v2353+v2413)}else{(if (self.scalar_static_f64[132]!=0.0){(v2188+v2193)}else{v304})})});
        let v2547=(if self.scalar_static_bool[18]{(v2525+(self.scalar_static_f64[146]*(-v2488)))}else{(if self.scalar_static_bool[16]{(v2354+v2414)}else{(if (self.scalar_static_f64[132]!=0.0){(v2189+v2194)}else{v304})})});
        let v2548=(if self.scalar_static_bool[18]{(v2526+(self.scalar_static_f64[146]*(v51-v2489)))}else{(if self.scalar_static_bool[16]{(v2355+v2415)}else{(if (self.scalar_static_f64[132]!=0.0){(v2190+v2195)}else{v304})})});
        let v2549=(if self.scalar_static_bool[18]{(v2527+(self.scalar_static_f64[146]*(-v2490)))}else{(if self.scalar_static_bool[16]{(v2356+v2416)}else{(if (self.scalar_static_f64[132]!=0.0){(v2191+v2196)}else{v304})})});
        let v2550=(if (self.scalar_static_f64[132]!=0.0){v2075}else{v2076});
        let v2628=(self.scalar_static_f64[135]*f64::powf(v715,self.scalar_static_f64[263]));
        let v2643=(if v713{(((v717*v1654)+(v247*(-((-((-(v349*v1654))/v1704))*v2628))))/self.scalar_static_f64[135])}else{(if v702{(((v708*v1654)+(v247*(-((v706*v2131)+(v530*(-((-(v704*v1654))/v2135)))))))/self.scalar_static_f64[135])}else{(if v683{((v687*v1654)/self.scalar_static_f64[135])}else{v2523})})});
        let v2644=(if v713{v304}else{(if v702{v304}else{(if v683{v304}else{v2524})})});
        let v2645=(if v713{((v247*(-(v2170*v2628)))/self.scalar_static_f64[135])}else{(if v702{v2157}else{(if v683{v304}else{v2525})})});
        let v2646=(if v713{v304}else{(if v702{v304}else{(if v683{v304}else{v2526})})});
        let v2647=(if v713{v304}else{(if v702{v304}else{(if v683{v304}else{v2527})})});
        let v2648=(if v713{((v247*(-(v2169*v2628)))/self.scalar_static_f64[135])}else{(if v702{v2156}else{v304})});
        let v2649=(if v701{v304}else{(if v683{(v684*((v693*v2550)+(v680*(((v247*(self.scalar_static_f64[136]*v2550))-(v691*v1654))/v1704))))}else{v2192})});
        let v2650=(if v701{v304}else{(if v683{(v684*((v693*self.scalar_static_f64[270])+(v680*(self.scalar_static_f64[274]/v247))))}else{v2193})});
        let v2651=(if v701{v304}else{(if v683{(v684*((v693*self.scalar_static_f64[271])+(v680*(self.scalar_static_f64[275]/v247))))}else{v2194})});
        let v2652=(if v701{v304}else{(if v683{(v684*((v693*self.scalar_static_f64[272])+(v680*(self.scalar_static_f64[276]/v247))))}else{v2195})});
        let v2653=(if v701{v304}else{(if v683{(v684*((v693*self.scalar_static_f64[273])+(v680*(self.scalar_static_f64[277]/v247))))}else{v2196})});
        let v2654=(if v701{v304}else{(if v683{(v684*((v693*self.scalar_static_f64[255])+(v680*v2096)))}else{v304})});
        let v2667=(if self.scalar_static_bool[16]{v2212}else{v2213});
        let v2669=(v726*v2667);
        let v2673=(v730*v2667);
        let v2689=(if self.scalar_static_bool[16]{(v185*(((v736*v2207)+(v560*(if self.scalar_static_bool[16]{(((v734*(v182*v2667))-(v725*(((v2669+v2669)/(v182*v729))+((v2673+v2673)/(v182*v733)))))/(v734*v734))}else{v2229})))-v2075))}else{v2435});
        let v2707=(if self.scalar_static_bool[16]{((v2208-(v751*v2207))/v2211)}else{v2256});
        let v2708=(if self.scalar_static_bool[16]{v304}else{v2257});
        let v2709=(if self.scalar_static_bool[16]{v304}else{v2258});
        let v2713=(v755*v2707);
        let v2715=(v755*v2708);
        let v2717=(v755*v2258);
        let v2719=(v755*v2709);
        let v2721=(v755*v2257);
        let v2723=(v182*v758);
        let v2729=(v759*v2707);
        let v2731=(v759*v2708);
        let v2733=(v759*v2258);
        let v2735=(v759*v2709);
        let v2737=(v759*v2257);
        let v2739=(v182*v762);
        let v2753=(v763*v763);
        let v2771=(if self.scalar_static_bool[16]{(((v763*(v182*v2707))-(v754*(((v2713+v2713)/v2723)+((v2729+v2729)/v2739))))/v2753)}else{v2298});
        let v2772=(if self.scalar_static_bool[16]{(((v763*(v182*v2708))-(v754*(((v2715+v2715)/v2723)+((v2731+v2731)/v2739))))/v2753)}else{v2299});
        let v2773=(if self.scalar_static_bool[16]{(((v763*v2261)-(v754*(((v2717+v2717)/v2723)+((v2733+v2733)/v2739))))/v2753)}else{v304});
        let v2774=(if self.scalar_static_bool[16]{(((v763*(v182*v2709))-(v754*(((v2719+v2719)/v2723)+((v2735+v2735)/v2739))))/v2753)}else{v2300});
        let v2775=(if self.scalar_static_bool[16]{(((v763*v2260)-(v754*(((v2721+v2721)/v2723)+((v2737+v2737)/v2739))))/v2753)}else{v304});
        let v2789=(if self.scalar_static_bool[16]{(v185*(((v765*v2207)+(v560*v2771))-v2075))}else{v2486});
        let v2790=(if self.scalar_static_bool[16]{(v185*(v560*v2772))}else{v2487});
        let v2791=(if self.scalar_static_bool[16]{(v185*(v560*v2773))}else{v2488});
        let v2792=(if self.scalar_static_bool[16]{(v185*(v560*v2774))}else{v2489});
        let v2793=(if self.scalar_static_bool[16]{v304}else{v2490});
        let v2794=(if self.scalar_static_bool[16]{(v185*(v560*v2775))}else{v304});
        let v2811=(self.scalar_static_f64[135]*f64::powf(v772,self.scalar_static_f64[263]));
        let v2838=(if self.scalar_static_bool[16]{(((v774*v1654)+(v247*(-((-(((v247*v2789)-(v770*v1654))/v1704))*v2811))))/self.scalar_static_f64[135])}else{v2643});
        let v2839=(if self.scalar_static_bool[16]{((v247*(-((-(v2790/v247))*v2811)))/self.scalar_static_f64[135])}else{v2644});
        let v2840=(if self.scalar_static_bool[16]{((v247*(-((-(v2791/v247))*v2811)))/self.scalar_static_f64[135])}else{v2645});
        let v2841=(if self.scalar_static_bool[16]{((v247*(-((-(v2792/v247))*v2811)))/self.scalar_static_f64[135])}else{v2646});
        let v2842=(if self.scalar_static_bool[16]{((v247*(-((-(v2793/v247))*v2811)))/self.scalar_static_f64[135])}else{v2647});
        let v2843=(if self.scalar_static_bool[16]{((v247*(-((-(v2794/v247))*v2811)))/self.scalar_static_f64[135])}else{v2648});
        let v2849=(if self.scalar_static_bool[16]{(v185*v2771)}else{v2360});
        let v2850=(if self.scalar_static_bool[16]{(v185*v2772)}else{v2361});
        let v2851=(if self.scalar_static_bool[16]{(v185*v2773)}else{v304});
        let v2852=(if self.scalar_static_bool[16]{(v185*v2774)}else{v2362});
        let v2853=(if self.scalar_static_bool[16]{(v185*v2775)}else{v304});
        let v2927=(if self.scalar_static_bool[18]{v2431}else{v2432});
        let v2930=(if self.scalar_static_bool[18]{(v191*(v2075+v2927))}else{v2689});
        let v2943=(if self.scalar_static_bool[18]{(((v801*v2074)+(v492*((-(((v247*v2930)-(v798*v1654))/v1704))*(self.scalar_static_f64[135]*f64::powf(v800,self.scalar_static_f64[263])))))/self.scalar_static_f64[135])}else{v2448});
        let v2944=(if self.scalar_static_bool[18]{v2075}else{v2449});
        let v2949=(v805*v2944);
        let v2951=(v805*self.scalar_static_f64[278]);
        let v2953=(v805*self.scalar_static_f64[279]);
        let v2955=(v805*self.scalar_static_f64[280]);
        let v2957=(v805*self.scalar_static_f64[281]);
        let v2959=(v805*self.scalar_static_f64[266]);
        let v2961=(v182*v808);
        let v2968=(if self.scalar_static_bool[18]{((v2949+v2949)/v2961)}else{v2470});
        let v2969=(if self.scalar_static_bool[18]{((v2951+v2951)/v2961)}else{v2471});
        let v2970=(if self.scalar_static_bool[18]{((v2953+v2953)/v2961)}else{v2472});
        let v2971=(if self.scalar_static_bool[18]{((v2955+v2955)/v2961)}else{v2473});
        let v2972=(if self.scalar_static_bool[18]{((v2957+v2957)/v2961)}else{v2474});
        let v2973=(if self.scalar_static_bool[18]{((v2959+v2959)/v2961)}else{v304});
        let v2987=(if self.scalar_static_bool[18]{((v185*(v2944-v2968))-v2075)}else{v2789});
        let v2988=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[278]-v2969))}else{v2790});
        let v2989=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[279]-v2970))}else{v2791});
        let v2990=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[280]-v2971))}else{v2792});
        let v2991=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[281]-v2972))}else{v2793});
        let v2992=(if self.scalar_static_bool[18]{(v185*(self.scalar_static_f64[266]-v2973))}else{v2794});
        let v3009=(self.scalar_static_f64[135]*f64::powf(v815,self.scalar_static_f64[263]));
        let v3030=(if self.scalar_static_bool[18]{(((v816*v2074)+(v492*((-(((v247*v2987)-(v813*v1654))/v1704))*v3009)))/self.scalar_static_f64[135])}else{v2838});
        let v3031=(if self.scalar_static_bool[18]{((v492*((-(v2988/v247))*v3009))/self.scalar_static_f64[135])}else{v2839});
        let v3032=(if self.scalar_static_bool[18]{((v492*((-(v2989/v247))*v3009))/self.scalar_static_f64[135])}else{v2840});
        let v3033=(if self.scalar_static_bool[18]{((v492*((-(v2990/v247))*v3009))/self.scalar_static_f64[135])}else{v2841});
        let v3034=(if self.scalar_static_bool[18]{((v492*((-(v2991/v247))*v3009))/self.scalar_static_f64[135])}else{v2842});
        let v3035=(if self.scalar_static_bool[18]{((v492*((-(v2992/v247))*v3009))/self.scalar_static_f64[135])}else{v2843});
        let v3062=(-v1692);
        let v3064=(if (self.scalar_static_f64[147]!=0.0){(self.scalar_static_f64[119]*v3062)}else{v2075});
        let v3065=(if self.scalar_static_bool[21]{v3064}else{v2550});
        let v3142=(self.scalar_static_f64[152]*f64::powf(v862,self.scalar_static_f64[294]));
        let v3157=(if v860{(((v864*v1692)+(v274*(-((-((-(v836*v1692))/v1713))*v3142))))/self.scalar_static_f64[152])}else{(if v841{((v847*v1692)/self.scalar_static_f64[152])}else{v3030})});
        let v3158=(if v860{v304}else{(if v841{v304}else{v3031})});
        let v3159=(if v860{v304}else{(if v841{v304}else{v3032})});
        let v3160=(if v860{v304}else{(if v841{v304}else{v3033})});
        let v3161=(if v860{v304}else{(if v841{v304}else{v3034})});
        let v3162=(if v860{((v274*(-((-(v362/v274))*v3142)))/self.scalar_static_f64[152])}else{(if v841{v304}else{v3035})});
        let v3163=(if v860{((v274*(-((-(v51/v274))*v3142)))/self.scalar_static_f64[152])}else{v304});
        let v3185=(v830*v3064);
        let v3192=(if self.scalar_static_bool[23]{(v191*(v3064+(if self.scalar_static_bool[23]{((v3185+v3185)/(v182*v877))}else{v2927})))}else{v2930});
        let v3206=(if self.scalar_static_bool[23]{v3064}else{v2944});
        let v3213=(v888*v3206);
        let v3215=(v888*self.scalar_static_f64[295]);
        let v3217=(v888*self.scalar_static_f64[296]);
        let v3219=(v888*self.scalar_static_f64[297]);
        let v3221=(v888*self.scalar_static_f64[298]);
        let v3223=(v888*self.scalar_static_f64[299]);
        let v3225=(v888*self.scalar_static_f64[300]);
        let v3227=(v182*v891);
        let v3257=(if self.scalar_static_bool[23]{((v185*(v3206-(if self.scalar_static_bool[23]{((v3213+v3213)/v3227)}else{v2968})))-v3064)}else{v2987});
        let v3258=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[295]-(if self.scalar_static_bool[23]{((v3215+v3215)/v3227)}else{v2969})))}else{v2988});
        let v3259=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[296]-(if self.scalar_static_bool[23]{((v3217+v3217)/v3227)}else{v2970})))}else{v2989});
        let v3260=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[297]-(if self.scalar_static_bool[23]{((v3219+v3219)/v3227)}else{v2971})))}else{v2990});
        let v3261=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[298]-(if self.scalar_static_bool[23]{((v3221+v3221)/v3227)}else{v2972})))}else{v2991});
        let v3262=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[299]-(if self.scalar_static_bool[23]{((v3223+v3223)/v3227)}else{v2973})))}else{v2992});
        let v3263=(if self.scalar_static_bool[23]{(v185*(self.scalar_static_f64[300]-(if self.scalar_static_bool[23]{((v3225+v3225)/v3227)}else{v304})))}else{v304});
        let v3282=(self.scalar_static_f64[152]*f64::powf(v898,self.scalar_static_f64[294]));
        let v3359=scalar_limexp_derivative(v914);
        let v3365=((v916*v1434)+(v60*(((-(v341*((v162*v1373)+(v11*self.scalar_static_f64[226]))))/(v913*v913))*v3359)));
        let v3366=(v60*((v51/v913)*v3359));
        let v3367=(v60*((v362/v913)*v3359));
        let v3374=((-(v345*((v163*v1373)+(v11*self.scalar_static_f64[227]))))/(v918*v918));
        let v3375=(v362/v918);
        let v3376=(v51/v918);
        let v3377=scalar_limexp_derivative(v919);
        let v3378=(v3374*v3377);
        let v3379=(v3375*v3377);
        let v3380=(v3376*v3377);
        let v3386=((v922*((v73*v1434)+(v60*(self.scalar_static_f64[25]*(((v68*(self.scalar_static_f64[194]*(self.scalar_static_f64[26]*f64::powf(v12,self.scalar_static_f64[207]))))+(v63*(v68*(((v11*self.scalar_static_f64[208])-(v66*v1373))/v1424))))*(self.scalar_static_f64[30]*f64::powf(v69,self.scalar_static_f64[209])))))))+(v921*v3378));
        let v3387=(v921*v3379);
        let v3388=(v921*v3380);
        let v3393=(self.scalar_static_f64[99]*v2546);
        let v3394=(self.scalar_static_f64[99]*v2547);
        let v3397=((self.scalar_static_f64[102]*v1899)+(self.scalar_static_f64[99]*v2545));
        let v3398=((self.scalar_static_f64[102]*v1900)+(self.scalar_static_f64[99]*v2548));
        let v3399=((self.scalar_static_f64[102]*v1901)+(self.scalar_static_f64[99]*v2549));
        let v3400=(v929*v3397);
        let v3402=(v929*v3393);
        let v3404=(v929*v3394);
        let v3406=(v929*v3398);
        let v3408=(v929*v3399);
        let v3410=(v182*v933);
        let v3421=(v185*(v3397+((v3400+v3400)/v3410)));
        let v3422=(v185*(v3393+((v3402+v3402)/v3410)));
        let v3423=(v185*(v3394+((v3404+v3404)/v3410)));
        let v3424=(v185*(v3398+((v3406+v3406)/v3410)));
        let v3425=(v185*(v3399+((v3408+v3408)/v3410)));
        let v3438=(self.scalar_static_f64[161]*f64::powf(v937,self.scalar_static_f64[301]));
        let v3444=(v209*(((v917*(if self.scalar_static_bool[2]{((-(self.scalar_static_f64[3]*(self.scalar_static_f64[194]*(self.scalar_static_f64[4]*f64::powf(v12,self.scalar_static_f64[195])))))/(v17*v17))}else{v304}))+(v314*v3365))+(self.scalar_static_f64[105]*v3386)));
        let v3445=(v209*(self.scalar_static_f64[105]*v3387));
        let v3446=(v209*((v314*v3366)+(self.scalar_static_f64[105]*v3388)));
        let v3447=(v209*(v314*v3367));
        let v3454=(self.scalar_static_f64[160]*f64::powf(v948,self.scalar_static_f64[302]));
        let v3481=(self.scalar_static_f64[160]*f64::powf(v955,self.scalar_static_f64[302]));
        let v3499=(if self.scalar_static_bool[26]{((v957*(v185*v3421))+(v954*(v3444*v3481)))}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v3421+(((v3421*v3438)+v3444)*v3454)))}else{v304})});
        let v3500=(if self.scalar_static_bool[26]{((v957*(v185*v3422))+(v954*(v3445*v3481)))}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v3422+(((v3422*v3438)+v3445)*v3454)))}else{v304})});
        let v3501=(if self.scalar_static_bool[26]{(v957*(v185*v3423))}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v3423+((v3423*v3438)*v3454)))}else{v304})});
        let v3502=(if self.scalar_static_bool[26]{((v957*(v185*v3424))+(v954*(v3446*v3481)))}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v3424+(((v3424*v3438)+v3446)*v3454)))}else{v304})});
        let v3503=(if self.scalar_static_bool[26]{((v957*(v185*v3425))+(v954*(v3447*v3481)))}else{(if (self.scalar_static_f64[159]!=0.0){(v185*(v3425+(((v3425*v3438)+v3447)*v3454)))}else{v304})});
        let v3507=(v959*v959);
        let v3544=(v964*v964);
        let v3547=(v362/v964);
        let v3548=(if (self.scalar_static_f64[162]!=0.0){((-(v349*self.scalar_static_f64[303]))/v3544)}else{v3374});
        let v3549=(if (self.scalar_static_f64[162]!=0.0){v304}else{v3375});
        let v3550=(if (self.scalar_static_f64[162]!=0.0){(v51/v964)}else{v304});
        let v3551=(if (self.scalar_static_f64[162]!=0.0){v304}else{v3376});
        let v3552=(if (self.scalar_static_f64[162]!=0.0){v3547}else{v304});
        let v3553=scalar_limexp_derivative(v966);
        let v3559=(if (self.scalar_static_f64[162]!=0.0){(v3548*v3553)}else{v3378});
        let v3560=(if (self.scalar_static_f64[162]!=0.0){(v3549*v3553)}else{v3379});
        let v3561=(if (self.scalar_static_f64[162]!=0.0){(v3550*v3553)}else{v304});
        let v3562=(if (self.scalar_static_f64[162]!=0.0){(v3551*v3553)}else{v3380});
        let v3563=(if (self.scalar_static_f64[162]!=0.0){(v3552*v3553)}else{v304});
        let v3567=(if (self.scalar_static_f64[162]!=0.0){((-(v345*self.scalar_static_f64[303]))/v3544)}else{v304});
        let v3568=scalar_limexp_derivative(v970);
        let v3572=(if (self.scalar_static_f64[162]!=0.0){(v3567*v3568)}else{v304});
        let v3573=(if (self.scalar_static_f64[162]!=0.0){(v3552*v3568)}else{v304});
        let v3574=(if (self.scalar_static_f64[162]!=0.0){(v3550*v3568)}else{v304});
        let v3593=(if (self.scalar_static_f64[162]!=0.0){((v978*v1466)+(v84*((self.scalar_static_f64[163]*v3559)+(self.scalar_static_f64[164]*v3572))))}else{v304});
        let v3594=(if (self.scalar_static_f64[162]!=0.0){(v84*((self.scalar_static_f64[163]*v3560)+(self.scalar_static_f64[164]*v3573)))}else{v304});
        let v3595=(if (self.scalar_static_f64[162]!=0.0){(v84*(self.scalar_static_f64[163]*v3561))}else{v304});
        let v3596=(if (self.scalar_static_f64[162]!=0.0){(v84*((self.scalar_static_f64[163]*v3562)+(self.scalar_static_f64[164]*v3574)))}else{v304});
        let v3597=(if (self.scalar_static_f64[162]!=0.0){(v84*(self.scalar_static_f64[163]*v3563))}else{v304});
        let v4390=((-(v345*v1373))/v1424);
        let v4391=(v362/v11);
        let v4392=(v51/v11);
        let v4393=scalar_limexp_derivative(v1179);
        let v4394=(v4390*v4393);
        let v4395=(v4391*v4393);
        let v4396=(v4392*v4393);
        let v4400=scalar_limexp_derivative(v1181);
        let v4409=(v182*v1185);
        let v4410=(((v1180*v1723)+(v294*v4394))/v4409);
        let v4411=((v294*v4395)/v4409);
        let v4412=((v294*v4396)/v4409);
        let v4418=(v182*v1188);
        let v4419=(((v1182*v1723)+(v294*(((-(v347*v1373))/v1424)*v4400)))/v4418);
        let v4420=((v294*(v4391*v4400))/v4418);
        let v4421=((v294*(v4392*v4400))/v4418);
        let v4722=(self.scalar_static_f64[117]*(v1284*v3365));
        let v4723=(self.scalar_static_f64[117]*(v1284*v3366));
        let v4724=(self.scalar_static_f64[117]*(v1284*v3367));
        let v4728=(v1287*v1287);
        let v4751=scalar_limexp_derivative(v1297);
        let v4756=(v1288*(((v1287*v4722)-(v1286*v4722))/v4728));
        let v4758=(v1288*(((v1287*v4723)-(v1286*v4723))/v4728));
        let v4760=(v1288*(((v1287*v4724)-(v1286*v4724))/v4728));
        let v4811=(((v959*(v917*((v1304*(self.scalar_static_f64[183]*(self.scalar_static_f64[184]*v3422)))+(v1293*(v1284*(v1301*(self.scalar_static_f64[185]*(self.scalar_static_f64[313]*v4751))))))))-(v1308*v3500))/v3507);
        let v4815=(((v959*(v917*(v1304*(self.scalar_static_f64[183]*(self.scalar_static_f64[184]*v3423)))))-(v1308*v3501))/v3507);
        let v4824=((self.scalar_static_f64[165]*((v430*v1701)+(v279*v1899)))+(((v959*((v1305*v3365)+(v917*((v1304*(self.scalar_static_f64[183]*(self.scalar_static_f64[184]*v3421)))+(v1293*(v1284*(v1299*(v4756+v4756))))))))-(v1308*v3499))/v3507));
        let v4825=((self.scalar_static_f64[165]*(v279*v1900))+(((v959*((v1305*v3366)+(v917*((v1304*(self.scalar_static_f64[183]*(self.scalar_static_f64[184]*v3424)))+(v1293*(v1284*((v1301*(self.scalar_static_f64[185]*(self.scalar_static_f64[314]*v4751)))+(v1299*(v4758+v4758)))))))))-(v1308*v3502))/v3507));
        let v4826=((self.scalar_static_f64[165]*(v279*v1901))+(((v959*((v1305*v3367)+(v917*((v1304*(self.scalar_static_f64[183]*(self.scalar_static_f64[184]*v3425)))+(v1293*(v1284*(v1299*(v4760+v4760))))))))-(v1308*v3503))/v3507));
        let v4833=(self.scalar_static_f64[170]*((v491*v1701)+(v279*(if self.scalar_static_bool[10]{((v2052+(self.scalar_static_f64[130]*(v1978+(-v2021))))-v1991)}else{(if (self.scalar_static_f64[121]!=0.0){(v1959+v1963)}else{v304})}))));
        let v4834=(self.scalar_static_f64[170]*(v279*(if self.scalar_static_bool[10]{(v2053+(self.scalar_static_f64[130]*(v51-v2022)))}else{(if (self.scalar_static_f64[121]!=0.0){(v1960+v1964)}else{v304})})));
        let v4835=(self.scalar_static_f64[170]*(v279*(if self.scalar_static_bool[10]{(v2054+(self.scalar_static_f64[130]*(-v2023)))}else{(if (self.scalar_static_f64[121]!=0.0){(v1961+v1965)}else{v304})})));
        let v4836=(self.scalar_static_f64[170]*(v279*(if self.scalar_static_bool[10]{(v2055+(self.scalar_static_f64[130]*(v362-v2024)))}else{(if (self.scalar_static_f64[121]!=0.0){(v1962+v1966)}else{v304})})));
        let v4841=(v284*v2547);
        let v4843=(v284*v2549);
        let v4853=((((v678*(self.scalar_static_f64[89]*v1708))+(v284*v2545))+(self.scalar_static_f64[186]*v3386))+(self.scalar_static_f64[187]*v4410));
        let v4854=(((v284*v2546)+(self.scalar_static_f64[186]*v3387))+(self.scalar_static_f64[187]*v4411));
        let v4855=(((v284*v2548)+(self.scalar_static_f64[186]*v3388))+(self.scalar_static_f64[187]*v4412));
        let v4856=(self.scalar_static_f64[187]*v4419);
        let v4857=(self.scalar_static_f64[187]*v4420);
        let v4858=(self.scalar_static_f64[187]*v4421);
        let v4865=(v286*(if self.scalar_static_bool[18]{(v3034+(self.scalar_static_f64[146]*(-v2991)))}else{(if self.scalar_static_bool[16]{(v2842+(if self.scalar_static_bool[16]{(v787*(-v2793))}else{v2416}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2647+v2653)}else{v304})})}));
        let v4872=(((v825*(self.scalar_static_f64[91]*v1708))+(v286*(if self.scalar_static_bool[18]{((v3030+(self.scalar_static_f64[146]*(v2930+(-v2987))))-v2943)}else{(if self.scalar_static_bool[16]{((v2838+(if self.scalar_static_bool[16]{((v789*(if self.scalar_static_bool[16]{(((v783*(if self.scalar_static_bool[16]{v2366}else{v2367}))+(v781*(-v2849)))+((v782*v2849)+(v780*(if self.scalar_static_bool[16]{v2374}else{v2375}))))}else{v2392}))+(v787*(v2689+(-v2789))))}else{v2412}))-(if self.scalar_static_bool[16]{(((v745*v1654)+(v247*(-((-(((v247*v2689)-(v741*v1654))/v1704))*(self.scalar_static_f64[135]*f64::powf(v743,self.scalar_static_f64[263]))))))/self.scalar_static_f64[135])}else{v2249}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2643+v2649)}else{v304})})})))+(self.scalar_static_f64[186]*(if self.scalar_static_bool[28]{v304}else{v3593})));
        let v4873=((v286*(if self.scalar_static_bool[18]{(v3031+(self.scalar_static_f64[146]*(-v2988)))}else{(if self.scalar_static_bool[16]{(v2839+(if self.scalar_static_bool[16]{((v789*(if self.scalar_static_bool[16]{((v781*(-v2850))+(v782*v2850))}else{v2393}))+(v787*(-v2790)))}else{v2413}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2644+v2650)}else{v304})})}))+(self.scalar_static_f64[186]*(if self.scalar_static_bool[28]{v304}else{v3594})));
        let v4874=((v286*(if self.scalar_static_bool[18]{(v3032+(self.scalar_static_f64[146]*(v51-v2989)))}else{(if self.scalar_static_bool[16]{(v2840+(if self.scalar_static_bool[16]{((v789*(if self.scalar_static_bool[16]{((v781*(-v2851))+(v782*v2851))}else{v304}))+(v787*(v51-v2791)))}else{v2414}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2645+v2651)}else{v304})})}))+(self.scalar_static_f64[186]*(if self.scalar_static_bool[28]{v304}else{v3595})));
        let v4875=((v286*(if self.scalar_static_bool[18]{(v3033+(self.scalar_static_f64[146]*(-v2990)))}else{(if self.scalar_static_bool[16]{(v2841+(if self.scalar_static_bool[16]{((v789*(if self.scalar_static_bool[16]{((v781*(-v2852))+(v782*v2852))}else{v2394}))+(v787*(-v2792)))}else{v2415}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2646+v2652)}else{v304})})}))+(self.scalar_static_f64[186]*(if self.scalar_static_bool[28]{v304}else{v3596})));
        let v4876=((v286*(if self.scalar_static_bool[18]{(v3035+(self.scalar_static_f64[146]*(v362-v2992)))}else{(if self.scalar_static_bool[16]{(v2843+(if self.scalar_static_bool[16]{((v789*(if self.scalar_static_bool[16]{((v781*(-v2853))+(v782*v2853))}else{v304}))+(v787*(v362-v2794)))}else{v304}))}else{(if (self.scalar_static_f64[132]!=0.0){(v2648+v2654)}else{v304})})}))+(self.scalar_static_f64[186]*(if self.scalar_static_bool[28]{v304}else{v3597})));
        let v4879=((v912*(self.scalar_static_f64[92]*(((-(self.scalar_static_f64[84]*v1692))/v1713)*(self.scalar_static_f64[93]*f64::powf(v288,self.scalar_static_f64[240])))))+(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{(((if self.scalar_static_bool[23]{(((v899*v3062)+(v828*((-(((v274*v3257)-(v896*v1692))/v1713))*v3282)))/self.scalar_static_f64[152])}else{v3157})+(self.scalar_static_f64[157]*(v3192+(-v3257))))-(if self.scalar_static_bool[23]{(((v884*v3062)+(v828*((-(((v274*v3192)-(v881*v1692))/v1713))*(self.scalar_static_f64[152]*f64::powf(v883,self.scalar_static_f64[294])))))/self.scalar_static_f64[152])}else{v2943}))}else{(if self.scalar_static_bool[21]{(v3157+(if v860{v304}else{(if v841{(v844*((v855*v3065)+(v838*(((v274*(self.scalar_static_f64[153]*v3065))-(v853*v1692))/v1713))))}else{v2649})}))}else{v304})})})));
        let v4880=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3258/v274))*v3282))/self.scalar_static_f64[152])}else{v3158})+(self.scalar_static_f64[157]*(-v3258)))}else{(if self.scalar_static_bool[21]{(v3158+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[282])+(v838*(self.scalar_static_f64[288]/v274))))}else{v2650})}))}else{v304})})}));
        let v4881=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3259/v274))*v3282))/self.scalar_static_f64[152])}else{v3159})+(self.scalar_static_f64[157]*(-v3259)))}else{(if self.scalar_static_bool[21]{(v3159+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[283])+(v838*(self.scalar_static_f64[289]/v274))))}else{v2651})}))}else{v304})})}));
        let v4882=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3260/v274))*v3282))/self.scalar_static_f64[152])}else{v3160})+(self.scalar_static_f64[157]*(-v3260)))}else{(if self.scalar_static_bool[21]{(v3160+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[284])+(v838*(self.scalar_static_f64[290]/v274))))}else{v2652})}))}else{v304})})}));
        let v4883=(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3261/v274))*v3282))/self.scalar_static_f64[152])}else{v3161})+(self.scalar_static_f64[157]*(-v3261)))}else{(if self.scalar_static_bool[21]{(v3161+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[285])+(v838*(self.scalar_static_f64[291]/v274))))}else{v2653})}))}else{v304})})}));
        let v4887=((v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3262/v274))*v3282))/self.scalar_static_f64[152])}else{v3162})+(self.scalar_static_f64[157]*(v362-v3262)))}else{(if self.scalar_static_bool[21]{(v3162+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[286])+(v838*(self.scalar_static_f64[292]/v274))))}else{v2654})}))}else{v304})})}))+self.scalar_static_f64[315]);
        let v4888=(self.scalar_static_f64[188]+(v291*(if self.scalar_static_bool[24]{v304}else{(if self.scalar_static_bool[23]{((if self.scalar_static_bool[23]{((v828*((-(v3263/v274))*v3282))/self.scalar_static_f64[152])}else{v3163})+(self.scalar_static_f64[157]*(v51-v3263)))}else{(if self.scalar_static_bool[21]{(v3163+(if v860{v304}else{(if v841{(v844*((v855*self.scalar_static_f64[287])+(v838*(self.scalar_static_f64[293]/v274))))}else{v304})}))}else{v304})})})));

        CommonStampValues {
            v6, v11, v12, v13, v51, v52, v84, v182, 
            v185, v209, v247, v304, v339, v340, v341, v342, 
            v343, v344, v345, v346, v348, v349, v362, v835, 
            v836, v896, v917, v923, v959, v964, v966, v968, 
            v970, v972, v980, v1173, v1179, v1180, v1185, v1188, 
            v1223, v1239, v1310, v1312, v1319, v1320, v1323, v1327, 
            v1330, v1333, v1372, v1373, v1424, v1466, v1654, v3257, 
            v3258, v3259, v3260, v3261, v3262, v3263, v3365, v3366, 
            v3367, v3386, v3387, v3388, v3499, v3500, v3501, v3502, 
            v3503, v3507, v3544, v3547, v3548, v3549, v3550, v3551, 
            v3552, v3559, v3560, v3561, v3562, v3563, v3567, v3572, 
            v3573, v3574, v3593, v3594, v3595, v3596, v3597, v4390, 
            v4391, v4392, v4394, v4395, v4396, v4410, v4411, v4412, 
            v4419, v4420, v4421, v4811, v4815, v4824, v4825, v4826, 
            v4833, v4834, v4835, v4836, v4841, v4843, v4853, v4854, 
            v4855, v4856, v4857, v4858, v4865, v4872, v4873, v4874, 
            v4875, v4876, v4879, v4880, v4881, v4882, v4883, v4887, 
            v4888, 
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
        let v960=(common.v923/common.v959);
        let v961=(common.v917/common.v959);
        let v985=((common.v51+(common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v980)}else{common.v304})))).sqrt();
        let v988=(if (self.scalar_static_f64[162]!=0.0){(common.v185*(common.v51+v985))}else{common.v304});
        let v990=(if (self.scalar_static_f64[162]!=0.0){(common.v836/common.v964)}else{common.v966});
        let v992=(if (self.scalar_static_f64[162]!=0.0){scalar_limexp(v990)}else{common.v968});
        let v993=(v992-common.v51);
        let v996=(common.v980-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v993)}else{common.v304}));
        let v1001=(if self.scalar_static_bool[28]{common.v51}else{v988});
        let v1002=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(v996/v988)}else{common.v304})});
        let v1006=(common.v11*self.scalar_static_f64[40]);
        let v1007=(common.v341/v1006);
        let v1008=(if (self.scalar_static_f64[166]!=0.0){v1007}else{v990});
        let v1010=(if (self.scalar_static_f64[166]!=0.0){scalar_limexp(v1008)}else{v992});
        let v1011=(common.v11*self.scalar_static_f64[46]);
        let v1012=(common.v341/v1011);
        let v1013=(if (self.scalar_static_f64[166]!=0.0){v1012}else{common.v304});
        let v1015=(if (self.scalar_static_f64[166]!=0.0){scalar_limexp(v1013)}else{common.v304});
        let v1019=(v299-common.v341);
        let v1020=(v1019/v300);
        let v1021=(if self.scalar_static_bool[31]{v1020}else{common.v970});
        let v1023=(if self.scalar_static_bool[31]{scalar_limexp(v1021)}else{common.v972});
        let v1024=(v1010-common.v51);
        let v1026=(v1015-common.v51);
        let v1028=((v97*v1024)+(v110*v1026));
        let v1042=(common.v343/v1006);
        let v1043=(if self.scalar_static_bool[36]{v1042}else{v1008});
        let v1045=(if self.scalar_static_bool[36]{scalar_limexp(v1043)}else{v1010});
        let v1046=(common.v343/v1011);
        let v1047=(if self.scalar_static_bool[36]{v1046}else{v1013});
        let v1049=(if self.scalar_static_bool[36]{scalar_limexp(v1047)}else{v1015});
        let v1051=(v299-common.v343);
        let v1052=(v1051/v300);
        let v1053=(if self.scalar_static_bool[37]{v1052}else{v1021});
        let v1055=(if self.scalar_static_bool[37]{scalar_limexp(v1053)}else{v1023});
        let v1056=(v1045-common.v51);
        let v1058=(v1049-common.v51);
        let v1060=((v97*v1056)+(v110*v1058));
        let v1069=(if self.scalar_static_bool[40]{v1007}else{v1043});
        let v1071=(if self.scalar_static_bool[40]{scalar_limexp(v1069)}else{v1045});
        let v1072=(if self.scalar_static_bool[40]{v1012}else{v1047});
        let v1074=(if self.scalar_static_bool[40]{scalar_limexp(v1072)}else{v1049});
        let v1076=(if self.scalar_static_bool[41]{v1020}else{v1053});
        let v1078=(if self.scalar_static_bool[41]{scalar_limexp(v1076)}else{v1055});
        let v1079=(v1071-common.v51);
        let v1081=(v1074-common.v51);
        let v1083=((v97*v1079)+(v110*v1081));
        let v1091=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v1083)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v1083-(self.scalar_static_f64[168]*(v1078-v302))))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v1028}else{(if self.scalar_static_bool[31]{(v1028-(self.scalar_static_f64[168]*(v1023-v302)))}else{common.v304})})})})});
        let v1092=(if self.scalar_static_bool[40]{v1042}else{v1069});
        let v1095=(if self.scalar_static_bool[40]{v1046}else{v1072});
        let v1098=(if self.scalar_static_bool[41]{v1052}else{v1076});
        let v1102=((if self.scalar_static_bool[40]{scalar_limexp(v1092)}else{v1071})-common.v51);
        let v1104=((if self.scalar_static_bool[40]{scalar_limexp(v1095)}else{v1074})-common.v51);
        let v1106=((v97*v1102)+(v110*v1104));
        let v1113=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v1106)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v1106-(self.scalar_static_f64[168]*((if self.scalar_static_bool[41]{scalar_limexp(v1098)}else{v1078})-v302))))}else{(if self.scalar_static_bool[38]{v1060}else{(if self.scalar_static_bool[37]{(v1060-(self.scalar_static_f64[168]*(v1055-v302)))}else{common.v304})})})});
        let v1114=(common.v11*self.scalar_static_f64[51]);
        let v1115=(common.v345/v1114);
        let v1116=scalar_limexp(v1115);
        let v1117=(common.v11*self.scalar_static_f64[56]);
        let v1118=(common.v345/v1117);
        let v1119=scalar_limexp(v1118);
        let v1120=(v1116-common.v51);
        let v1122=(v1119-common.v51);
        let v1124=((v121*v1120)+(v132*v1122));
        let v1130=(if (self.scalar_static_f64[171]!=0.0){(common.v349/v1114)}else{v1115});
        let v1134=(if (self.scalar_static_f64[171]!=0.0){(common.v349/v1117)}else{v1118});
        let v1136=(if (self.scalar_static_f64[171]!=0.0){scalar_limexp(v1134)}else{v1119});
        let v1137=((if (self.scalar_static_f64[171]!=0.0){scalar_limexp(v1130)}else{v1116})-common.v51);
        let v1139=(v1136-common.v51);
        let v1144=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){((v134*v1137)+(v136*v1139))}else{common.v304})});
        let v1148=(common.v247-common.v345);
        let v1150=0.01;
        let v1152=(((v1148*v1148)+v1150)).sqrt();
        let v1155=(if (self.scalar_static_f64[173]!=0.0){(common.v185*(v1148+v1152))}else{common.v896});
        let v1156=(self.scalar_static_f64[172]*v1155);
        let v1157=(-(self.scalar_static_f64[71]*(common.v51+(common.v13*self.scalar_static_f64[72]))));
        let v1159=f64::powf(v1155,self.scalar_static_f64[174]);
        let v1160=(v1157*v1159);
        let v1161=scalar_limexp(v1160);
        let v1163=(if (self.scalar_static_f64[173]!=0.0){(v1156*v1161)}else{common.v304});
        let v1164=(v961-v960);
        let v1165=(v1164-v1124);
        let v1170=(v1124-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){(v1163*v1165)}else{common.v304})}));
        let v1174=(common.v1173-common.v346);
        let v1178=(if self.scalar_static_bool[50]{common.v304}else{(if (self.scalar_static_f64[175]!=0.0){(v1174/v21)}else{common.v304})});
        let v1191=(common.v51+common.v1185);
        let v1192=(common.v51+common.v1188);
        let v1194=(if (self.scalar_static_f64[176]!=0.0){(v1191/v1192)}else{common.v304});
        let v1195=(common.v346-common.v344);
        let v1198=((common.v1185-common.v1188)-(v1194).ln());
        let v1200=(v1195+(common.v11*v1198));
        let v1202=(if (self.scalar_static_f64[176]!=0.0){(v1200/v25)}else{common.v304});
        let v1203=(v25*v325);
        let v1204=(v1202*v1203);
        let v1206=(self.scalar_static_f64[111]*(common.v185*v325));
        let v1209=((v1150+(v1195*v1195))).sqrt();
        let v1211=(common.v51+(v1206*v1209));
        let v1213=(if (self.scalar_static_f64[176]!=0.0){(v1204/v1211)}else{common.v304});
        let v1216=((common.v51+(v1213*v1213))).sqrt();
        let v1220=(if self.scalar_static_bool[52]{common.v304}else{(if (self.scalar_static_f64[176]!=0.0){(v1202/v1216)}else{common.v304})});
        let v1224=(common.v1223-common.v342);
        let v1228=(if self.scalar_static_bool[54]{common.v304}else{(if (self.scalar_static_f64[177]!=0.0){(v1224/v29)}else{common.v304})});
        let v1231=(common.v342-common.v339);
        let v1232=(common.v959*v1231);
        let v1236=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){(v1232/v33)}else{common.v304})});
        let v1240=(common.v1239-common.v340);
        let v1244=(if self.scalar_static_bool[58]{common.v304}else{(if (self.scalar_static_f64[179]!=0.0){(v1240/v37)}else{common.v304})});
        let v1247=(common.v348-common.v346);
        let v1248=(v1001*v1247);
        let v1252=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){(v1248/v45)}else{common.v304})});
        let v1257=(common.v11*self.scalar_static_f64[63]);
        let v1259=(if (self.scalar_static_f64[181]!=0.0){(common.v836/v1257)}else{common.v1179});
        let v1262=(common.v11*self.scalar_static_f64[68]);
        let v1264=(if (self.scalar_static_f64[181]!=0.0){(common.v836/v1262)}else{v1134});
        let v1267=((if (self.scalar_static_f64[181]!=0.0){scalar_limexp(v1259)}else{common.v1180})-common.v51);
        let v1269=((if (self.scalar_static_f64[181]!=0.0){scalar_limexp(v1264)}else{v1136})-common.v51);
        let v1274=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){((v147*v1267)+(v158*v1269))}else{common.v304})});
        let v1278=(ctx.node_voltage(nodes[3])-common.v835);
        let v1282=(if self.scalar_static_bool[66]{common.v304}else{(if (self.scalar_static_f64[182]!=0.0){(v1278/v41)}else{common.v304})});
        let v1337=(common.v344-common.v340);
        let v1348=(common.v342-common.v835);
        let v1389=(self.scalar_static_f64[7]*(self.scalar_static_f64[194]*(self.scalar_static_f64[8]*f64::powf(common.v12,self.scalar_static_f64[197]))));
        let v1470=(self.scalar_static_f64[194]*(self.scalar_static_f64[37]*f64::powf(common.v12,self.scalar_static_f64[212])));
        let v1484=(self.scalar_static_f64[36]*(((v92*v1470)+(v87*(v92*(((common.v11*self.scalar_static_f64[213])-(v90*common.v1373))/common.v1424))))*(self.scalar_static_f64[41]*f64::powf(v93,self.scalar_static_f64[214]))));
        let v1488=(self.scalar_static_f64[194]*(self.scalar_static_f64[43]*f64::powf(common.v12,self.scalar_static_f64[215])));
        let v1502=(self.scalar_static_f64[42]*(((v105*v1488)+(v100*(v105*(((common.v11*self.scalar_static_f64[216])-(v103*common.v1373))/common.v1424))))*(self.scalar_static_f64[47]*f64::powf(v106,self.scalar_static_f64[217]))));
        let v1515=(((v116*v1470)+(v87*(v116*(((common.v11*self.scalar_static_f64[218])-(v114*common.v1373))/common.v1424))))*(self.scalar_static_f64[52]*f64::powf(v117,self.scalar_static_f64[219])));
        let v1529=(((v127*v1488)+(v100*(v127*(((common.v11*self.scalar_static_f64[220])-(v125*common.v1373))/common.v1424))))*(self.scalar_static_f64[57]*f64::powf(v128,self.scalar_static_f64[221])));
        let v1732=((v181*common.v1373)+(common.v11*self.scalar_static_f64[229]));
        let v1733=(v300*(-(self.scalar_static_f64[73]*(v172+v173))));
        let v1736=(v300*v300);
        let v1738=(v302*((v1733-(v299*v1732))/v1736));
        let v1746=(if self.scalar_static_bool[5]{((-(self.scalar_static_f64[95]*(self.scalar_static_f64[194]*(self.scalar_static_f64[96]*f64::powf(common.v12,self.scalar_static_f64[241])))))/(v298*v298))}else{common.v304});
        let v3508=(((common.v959*common.v3386)-(common.v923*common.v3499))/common.v3507);
        let v3512=(((common.v959*common.v3387)-(common.v923*common.v3500))/common.v3507);
        let v3515=((-(common.v923*common.v3501))/common.v3507);
        let v3519=(((common.v959*common.v3388)-(common.v923*common.v3502))/common.v3507);
        let v3522=((-(common.v923*common.v3503))/common.v3507);
        let v3526=(((common.v959*common.v3365)-(common.v917*common.v3499))/common.v3507);
        let v3529=((-(common.v917*common.v3500))/common.v3507);
        let v3532=((-(common.v917*common.v3501))/common.v3507);
        let v3536=(((common.v959*common.v3366)-(common.v917*common.v3502))/common.v3507);
        let v3540=(((common.v959*common.v3367)-(common.v917*common.v3503))/common.v3507);
        let v3613=(common.v182*v985);
        let v3624=(if (self.scalar_static_f64[162]!=0.0){(common.v185*((common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v3593)}else{common.v304}))/v3613))}else{common.v304});
        let v3625=(if (self.scalar_static_f64[162]!=0.0){(common.v185*((common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v3594)}else{common.v304}))/v3613))}else{common.v304});
        let v3626=(if (self.scalar_static_f64[162]!=0.0){(common.v185*((common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v3595)}else{common.v304}))/v3613))}else{common.v304});
        let v3627=(if (self.scalar_static_f64[162]!=0.0){(common.v185*((common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v3596)}else{common.v304}))/v3613))}else{common.v304});
        let v3628=(if (self.scalar_static_f64[162]!=0.0){(common.v185*((common.v209*(if (self.scalar_static_f64[162]!=0.0){(self.scalar_static_f64[108]*common.v3597)}else{common.v304}))/v3613))}else{common.v304});
        let v3632=(if (self.scalar_static_f64[162]!=0.0){((-(common.v836*self.scalar_static_f64[303]))/common.v3544)}else{common.v3548});
        let v3633=(if (self.scalar_static_f64[162]!=0.0){common.v304}else{common.v3549});
        let v3634=(if (self.scalar_static_f64[162]!=0.0){common.v304}else{common.v3550});
        let v3635=(if (self.scalar_static_f64[162]!=0.0){common.v304}else{common.v3551});
        let v3636=(if (self.scalar_static_f64[162]!=0.0){common.v3547}else{common.v3552});
        let v3637=scalar_limexp_derivative(v990);
        let v3644=(if (self.scalar_static_f64[162]!=0.0){(v3632*v3637)}else{common.v3559});
        let v3645=(if (self.scalar_static_f64[162]!=0.0){(v3633*v3637)}else{common.v3560});
        let v3646=(if (self.scalar_static_f64[162]!=0.0){(v3634*v3637)}else{common.v3561});
        let v3647=(if (self.scalar_static_f64[162]!=0.0){(v3635*v3637)}else{common.v3562});
        let v3648=(if (self.scalar_static_f64[162]!=0.0){(v3636*v3637)}else{common.v3563});
        let v3649=(if (self.scalar_static_f64[162]!=0.0){(common.v3550*v3637)}else{common.v304});
        let v3673=(v988*v988);
        let v3708=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(((v988*(common.v3593-(if (self.scalar_static_f64[162]!=0.0){((v993*common.v1466)+(common.v84*v3644))}else{common.v304})))-(v996*v3624))/v3673)}else{common.v304})});
        let v3709=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(((v988*(common.v3594-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v3645)}else{common.v304})))-(v996*v3625))/v3673)}else{common.v304})});
        let v3710=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(((v988*(common.v3595-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v3646)}else{common.v304})))-(v996*v3626))/v3673)}else{common.v304})});
        let v3711=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(((v988*(common.v3596-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v3647)}else{common.v304})))-(v996*v3627))/v3673)}else{common.v304})});
        let v3712=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){(((v988*(common.v3597-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v3648)}else{common.v304})))-(v996*v3628))/v3673)}else{common.v304})});
        let v3713=(if self.scalar_static_bool[28]{common.v304}else{(if (self.scalar_static_f64[162]!=0.0){((-(if (self.scalar_static_f64[162]!=0.0){(common.v84*v3649)}else{common.v304}))/v988)}else{common.v304})});
        let v3717=(v1006*v1006);
        let v3718=((-(common.v341*self.scalar_static_f64[304]))/v3717);
        let v3719=(common.v51/v1006);
        let v3720=(common.v362/v1006);
        let v3721=(if (self.scalar_static_f64[166]!=0.0){v3718}else{v3632});
        let v3722=(if (self.scalar_static_f64[166]!=0.0){common.v304}else{v3633});
        let v3723=(if (self.scalar_static_f64[166]!=0.0){common.v304}else{v3634});
        let v3724=(if (self.scalar_static_f64[166]!=0.0){v3719}else{v3635});
        let v3725=(if (self.scalar_static_f64[166]!=0.0){v3720}else{common.v304});
        let v3726=(if (self.scalar_static_f64[166]!=0.0){common.v304}else{v3636});
        let v3727=(if (self.scalar_static_f64[166]!=0.0){common.v304}else{common.v3550});
        let v3728=scalar_limexp_derivative(v1008);
        let v3736=(if (self.scalar_static_f64[166]!=0.0){(v3721*v3728)}else{v3644});
        let v3737=(if (self.scalar_static_f64[166]!=0.0){(v3722*v3728)}else{v3645});
        let v3738=(if (self.scalar_static_f64[166]!=0.0){(v3723*v3728)}else{v3646});
        let v3739=(if (self.scalar_static_f64[166]!=0.0){(v3724*v3728)}else{v3647});
        let v3740=(if (self.scalar_static_f64[166]!=0.0){(v3725*v3728)}else{common.v304});
        let v3741=(if (self.scalar_static_f64[166]!=0.0){(v3726*v3728)}else{v3648});
        let v3742=(if (self.scalar_static_f64[166]!=0.0){(v3727*v3728)}else{v3649});
        let v3746=(v1011*v1011);
        let v3747=((-(common.v341*self.scalar_static_f64[305]))/v3746);
        let v3748=(common.v51/v1011);
        let v3749=(common.v362/v1011);
        let v3750=(if (self.scalar_static_f64[166]!=0.0){v3747}else{common.v304});
        let v3751=(if (self.scalar_static_f64[166]!=0.0){v3748}else{common.v304});
        let v3752=(if (self.scalar_static_f64[166]!=0.0){v3749}else{common.v304});
        let v3753=scalar_limexp_derivative(v1013);
        let v3757=(if (self.scalar_static_f64[166]!=0.0){(v3750*v3753)}else{common.v304});
        let v3758=(if (self.scalar_static_f64[166]!=0.0){(v3751*v3753)}else{common.v304});
        let v3759=(if (self.scalar_static_f64[166]!=0.0){(v3752*v3753)}else{common.v304});
        let v3762=((v1733-(v1019*v1732))/v1736);
        let v3763=(common.v362/v300);
        let v3764=(common.v51/v300);
        let v3765=(if self.scalar_static_bool[31]{v3762}else{common.v3567});
        let v3766=(if self.scalar_static_bool[31]{common.v304}else{common.v3552});
        let v3767=(if self.scalar_static_bool[31]{v3763}else{common.v3550});
        let v3768=(if self.scalar_static_bool[31]{v3764}else{common.v304});
        let v3769=scalar_limexp_derivative(v1021);
        let v3774=(if self.scalar_static_bool[31]{(v3765*v3769)}else{common.v3572});
        let v3775=(if self.scalar_static_bool[31]{(v3766*v3769)}else{common.v3573});
        let v3776=(if self.scalar_static_bool[31]{(v3767*v3769)}else{common.v3574});
        let v3777=(if self.scalar_static_bool[31]{(v3768*v3769)}else{common.v304});
        let v3781=(v97*v3737);
        let v3782=(v97*v3738);
        let v3785=(v97*v3741);
        let v3786=(v97*v3742);
        let v3792=(((v1024*v1484)+(v97*v3736))+((v1026*v1502)+(v110*v3757)));
        let v3793=((v97*v3739)+(v110*v3758));
        let v3794=((v97*v3740)+(v110*v3759));
        let v3827=((-(common.v343*self.scalar_static_f64[304]))/v3717);
        let v3828=(if self.scalar_static_bool[36]{v3827}else{v3721});
        let v3829=(if self.scalar_static_bool[36]{common.v304}else{v3722});
        let v3830=(if self.scalar_static_bool[36]{v3719}else{v3723});
        let v3831=(if self.scalar_static_bool[36]{common.v304}else{v3724});
        let v3832=(if self.scalar_static_bool[36]{v3720}else{v3725});
        let v3833=(if self.scalar_static_bool[36]{common.v304}else{v3726});
        let v3834=(if self.scalar_static_bool[36]{common.v304}else{v3727});
        let v3835=scalar_limexp_derivative(v1043);
        let v3843=(if self.scalar_static_bool[36]{(v3828*v3835)}else{v3736});
        let v3844=(if self.scalar_static_bool[36]{(v3829*v3835)}else{v3737});
        let v3845=(if self.scalar_static_bool[36]{(v3830*v3835)}else{v3738});
        let v3846=(if self.scalar_static_bool[36]{(v3831*v3835)}else{v3739});
        let v3847=(if self.scalar_static_bool[36]{(v3832*v3835)}else{v3740});
        let v3848=(if self.scalar_static_bool[36]{(v3833*v3835)}else{v3741});
        let v3849=(if self.scalar_static_bool[36]{(v3834*v3835)}else{v3742});
        let v3852=((-(common.v343*self.scalar_static_f64[305]))/v3746);
        let v3853=(if self.scalar_static_bool[36]{v3852}else{v3750});
        let v3854=(if self.scalar_static_bool[36]{v3748}else{common.v304});
        let v3855=(if self.scalar_static_bool[36]{common.v304}else{v3751});
        let v3856=(if self.scalar_static_bool[36]{v3749}else{v3752});
        let v3857=scalar_limexp_derivative(v1047);
        let v3862=(if self.scalar_static_bool[36]{(v3853*v3857)}else{v3757});
        let v3863=(if self.scalar_static_bool[36]{(v3854*v3857)}else{common.v304});
        let v3864=(if self.scalar_static_bool[36]{(v3855*v3857)}else{v3758});
        let v3865=(if self.scalar_static_bool[36]{(v3856*v3857)}else{v3759});
        let v3868=((v1733-(v1051*v1732))/v1736);
        let v3869=(if self.scalar_static_bool[37]{v3868}else{v3765});
        let v3870=(if self.scalar_static_bool[37]{common.v304}else{v3766});
        let v3871=(if self.scalar_static_bool[37]{v3763}else{common.v304});
        let v3872=(if self.scalar_static_bool[37]{common.v304}else{v3767});
        let v3873=(if self.scalar_static_bool[37]{v3764}else{v3768});
        let v3874=scalar_limexp_derivative(v1053);
        let v3880=(if self.scalar_static_bool[37]{(v3869*v3874)}else{v3774});
        let v3881=(if self.scalar_static_bool[37]{(v3870*v3874)}else{v3775});
        let v3882=(if self.scalar_static_bool[37]{(v3871*v3874)}else{common.v304});
        let v3883=(if self.scalar_static_bool[37]{(v3872*v3874)}else{v3776});
        let v3884=(if self.scalar_static_bool[37]{(v3873*v3874)}else{v3777});
        let v3888=(v97*v3844);
        let v3892=(v97*v3848);
        let v3893=(v97*v3849);
        let v3900=(((v1056*v1484)+(v97*v3843))+((v1058*v1502)+(v110*v3862)));
        let v3901=((v97*v3845)+(v110*v3863));
        let v3902=((v97*v3846)+(v110*v3864));
        let v3903=((v97*v3847)+(v110*v3865));
        let v3929=(if self.scalar_static_bool[40]{v3718}else{v3828});
        let v3930=(if self.scalar_static_bool[40]{common.v304}else{v3829});
        let v3931=(if self.scalar_static_bool[40]{common.v304}else{v3830});
        let v3932=(if self.scalar_static_bool[40]{v3719}else{v3831});
        let v3933=(if self.scalar_static_bool[40]{v3720}else{v3832});
        let v3934=(if self.scalar_static_bool[40]{common.v304}else{v3833});
        let v3935=(if self.scalar_static_bool[40]{common.v304}else{v3834});
        let v3936=scalar_limexp_derivative(v1069);
        let v3944=(if self.scalar_static_bool[40]{(v3929*v3936)}else{v3843});
        let v3945=(if self.scalar_static_bool[40]{(v3930*v3936)}else{v3844});
        let v3946=(if self.scalar_static_bool[40]{(v3931*v3936)}else{v3845});
        let v3947=(if self.scalar_static_bool[40]{(v3932*v3936)}else{v3846});
        let v3948=(if self.scalar_static_bool[40]{(v3933*v3936)}else{v3847});
        let v3949=(if self.scalar_static_bool[40]{(v3934*v3936)}else{v3848});
        let v3950=(if self.scalar_static_bool[40]{(v3935*v3936)}else{v3849});
        let v3951=(if self.scalar_static_bool[40]{v3747}else{v3853});
        let v3952=(if self.scalar_static_bool[40]{common.v304}else{v3854});
        let v3953=(if self.scalar_static_bool[40]{v3748}else{v3855});
        let v3954=(if self.scalar_static_bool[40]{v3749}else{v3856});
        let v3955=scalar_limexp_derivative(v1072);
        let v3960=(if self.scalar_static_bool[40]{(v3951*v3955)}else{v3862});
        let v3961=(if self.scalar_static_bool[40]{(v3952*v3955)}else{v3863});
        let v3962=(if self.scalar_static_bool[40]{(v3953*v3955)}else{v3864});
        let v3963=(if self.scalar_static_bool[40]{(v3954*v3955)}else{v3865});
        let v3964=(if self.scalar_static_bool[41]{v3762}else{v3869});
        let v3965=(if self.scalar_static_bool[41]{common.v304}else{v3870});
        let v3966=(if self.scalar_static_bool[41]{common.v304}else{v3871});
        let v3967=(if self.scalar_static_bool[41]{v3763}else{v3872});
        let v3968=(if self.scalar_static_bool[41]{v3764}else{v3873});
        let v3969=scalar_limexp_derivative(v1076);
        let v3975=(if self.scalar_static_bool[41]{(v3964*v3969)}else{v3880});
        let v3976=(if self.scalar_static_bool[41]{(v3965*v3969)}else{v3881});
        let v3977=(if self.scalar_static_bool[41]{(v3966*v3969)}else{v3882});
        let v3978=(if self.scalar_static_bool[41]{(v3967*v3969)}else{v3883});
        let v3979=(if self.scalar_static_bool[41]{(v3968*v3969)}else{v3884});
        let v3983=(v97*v3945);
        let v3995=(((v1079*v1484)+(v97*v3944))+((v1081*v1502)+(v110*v3960)));
        let v3996=((v97*v3946)+(v110*v3961));
        let v3997=((v97*v3947)+(v110*v3962));
        let v3998=((v97*v3948)+(v110*v3963));
        let v4015=(self.scalar_static_f64[165]*(v97*v3949));
        let v4016=(self.scalar_static_f64[165]*(v97*v3950));
        let v4029=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v3995)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v3995-(self.scalar_static_f64[168]*(v3975-v1738))))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3792}else{(if self.scalar_static_bool[31]{(v3792-(self.scalar_static_f64[168]*(v3774-v1738)))}else{common.v304})})})})});
        let v4030=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v3983)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v3983-(self.scalar_static_f64[168]*v3976)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3781}else{(if self.scalar_static_bool[31]{(v3781-(self.scalar_static_f64[168]*v3775))}else{common.v304})})})})});
        let v4031=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v3996)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v3996-(self.scalar_static_f64[168]*v3977)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3782}else{(if self.scalar_static_bool[31]{v3782}else{common.v304})})})})});
        let v4032=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v3997)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v3997-(self.scalar_static_f64[168]*v3978)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3793}else{(if self.scalar_static_bool[31]{(v3793-(self.scalar_static_f64[168]*v3776))}else{common.v304})})})})});
        let v4033=(if self.scalar_static_bool[42]{(self.scalar_static_f64[165]*v3998)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[165]*(v3998-(self.scalar_static_f64[168]*v3979)))}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3794}else{(if self.scalar_static_bool[31]{(v3794-(self.scalar_static_f64[168]*v3777))}else{common.v304})})})})});
        let v4034=(if self.scalar_static_bool[42]{v4015}else{(if self.scalar_static_bool[41]{v4015}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3785}else{(if self.scalar_static_bool[31]{v3785}else{common.v304})})})})});
        let v4035=(if self.scalar_static_bool[42]{v4016}else{(if self.scalar_static_bool[41]{v4016}else{(if self.scalar_static_bool[36]{common.v304}else{(if self.scalar_static_bool[33]{v3786}else{(if self.scalar_static_bool[31]{v3786}else{common.v304})})})})});
        let v4043=scalar_limexp_derivative(v1092);
        let v4062=scalar_limexp_derivative(v1095);
        let v4076=scalar_limexp_derivative(v1098);
        let v4090=(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3930})*v4043)}else{v3945}));
        let v4102=(((v1102*v1484)+(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3827}else{v3929})*v4043)}else{v3944})))+((v1104*v1502)+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3852}else{v3951})*v4062)}else{v3960}))));
        let v4103=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3719}else{v3931})*v4043)}else{v3946}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3748}else{v3952})*v4062)}else{v3961})));
        let v4104=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3932})*v4043)}else{v3947}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3953})*v4062)}else{v3962})));
        let v4105=((v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3720}else{v3933})*v4043)}else{v3948}))+(v110*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{v3749}else{v3954})*v4062)}else{v3963})));
        let v4122=(self.scalar_static_f64[170]*(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3934})*v4043)}else{v3949})));
        let v4123=(self.scalar_static_f64[170]*(v97*(if self.scalar_static_bool[40]{((if self.scalar_static_bool[40]{common.v304}else{v3935})*v4043)}else{v3950})));
        let v4136=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v4102)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v4102-(self.scalar_static_f64[168]*((if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3868}else{v3964})*v4076)}else{v3975})-v1738))))}else{(if self.scalar_static_bool[38]{v3900}else{(if self.scalar_static_bool[37]{(v3900-(self.scalar_static_f64[168]*(v3880-v1738)))}else{common.v304})})})});
        let v4137=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v4090)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v4090-(self.scalar_static_f64[168]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{common.v304}else{v3965})*v4076)}else{v3976}))))}else{(if self.scalar_static_bool[38]{v3888}else{(if self.scalar_static_bool[37]{(v3888-(self.scalar_static_f64[168]*v3881))}else{common.v304})})})});
        let v4138=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v4103)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v4103-(self.scalar_static_f64[168]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3763}else{v3966})*v4076)}else{v3977}))))}else{(if self.scalar_static_bool[38]{v3901}else{(if self.scalar_static_bool[37]{(v3901-(self.scalar_static_f64[168]*v3882))}else{common.v304})})})});
        let v4139=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v4104)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v4104-(self.scalar_static_f64[168]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{common.v304}else{v3967})*v4076)}else{v3978}))))}else{(if self.scalar_static_bool[38]{v3902}else{(if self.scalar_static_bool[37]{(v3902-(self.scalar_static_f64[168]*v3883))}else{common.v304})})})});
        let v4140=(if self.scalar_static_bool[42]{(self.scalar_static_f64[170]*v4105)}else{(if self.scalar_static_bool[41]{(self.scalar_static_f64[170]*(v4105-(self.scalar_static_f64[168]*(if self.scalar_static_bool[41]{((if self.scalar_static_bool[41]{v3764}else{v3968})*v4076)}else{v3979}))))}else{(if self.scalar_static_bool[38]{v3903}else{(if self.scalar_static_bool[37]{(v3903-(self.scalar_static_f64[168]*v3884))}else{common.v304})})})});
        let v4141=(if self.scalar_static_bool[42]{v4122}else{(if self.scalar_static_bool[41]{v4122}else{(if self.scalar_static_bool[38]{v3892}else{(if self.scalar_static_bool[37]{v3892}else{common.v304})})})});
        let v4142=(if self.scalar_static_bool[42]{v4123}else{(if self.scalar_static_bool[41]{v4123}else{(if self.scalar_static_bool[38]{v3893}else{(if self.scalar_static_bool[37]{v3893}else{common.v304})})})});
        let v4146=(v1114*v1114);
        let v4147=((-(common.v345*self.scalar_static_f64[306]))/v4146);
        let v4148=(common.v362/v1114);
        let v4149=(common.v51/v1114);
        let v4150=scalar_limexp_derivative(v1115);
        let v4151=(v4147*v4150);
        let v4152=(v4148*v4150);
        let v4153=(v4149*v4150);
        let v4157=(v1117*v1117);
        let v4158=((-(common.v345*self.scalar_static_f64[307]))/v4157);
        let v4159=(common.v362/v1117);
        let v4160=(common.v51/v1117);
        let v4161=scalar_limexp_derivative(v1118);
        let v4162=(v4158*v4161);
        let v4163=(v4159*v4161);
        let v4164=(v4160*v4161);
        let v4175=(((v1120*(self.scalar_static_f64[48]*v1515))+(v121*v4151))+((v1122*(self.scalar_static_f64[53]*v1529))+(v132*v4162)));
        let v4176=((v121*v4152)+(v132*v4163));
        let v4177=((v121*v4153)+(v132*v4164));
        let v4186=scalar_limexp_derivative(v1130);
        let v4200=(if (self.scalar_static_f64[171]!=0.0){((-(common.v349*self.scalar_static_f64[307]))/v4157)}else{v4158});
        let v4201=(if (self.scalar_static_f64[171]!=0.0){common.v304}else{v4159});
        let v4202=(if (self.scalar_static_f64[171]!=0.0){v4160}else{common.v304});
        let v4203=(if (self.scalar_static_f64[171]!=0.0){common.v304}else{v4160});
        let v4204=(if (self.scalar_static_f64[171]!=0.0){v4159}else{common.v304});
        let v4205=scalar_limexp_derivative(v1134);
        let v4211=(if (self.scalar_static_f64[171]!=0.0){(v4200*v4205)}else{v4162});
        let v4212=(if (self.scalar_static_f64[171]!=0.0){(v4201*v4205)}else{v4163});
        let v4213=(if (self.scalar_static_f64[171]!=0.0){(v4202*v4205)}else{common.v304});
        let v4214=(if (self.scalar_static_f64[171]!=0.0){(v4203*v4205)}else{v4164});
        let v4215=(if (self.scalar_static_f64[171]!=0.0){(v4204*v4205)}else{common.v304});
        let v4240=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){(((v1137*(self.scalar_static_f64[58]*v1515))+(v134*(if (self.scalar_static_f64[171]!=0.0){((if (self.scalar_static_f64[171]!=0.0){((-(common.v349*self.scalar_static_f64[306]))/v4146)}else{v4147})*v4186)}else{v4151})))+((v1139*(self.scalar_static_f64[59]*v1529))+(v136*v4211)))}else{common.v304})});
        let v4241=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){((v134*(if (self.scalar_static_f64[171]!=0.0){((if (self.scalar_static_f64[171]!=0.0){common.v304}else{v4148})*v4186)}else{v4152}))+(v136*v4212))}else{common.v304})});
        let v4242=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){((v134*(if (self.scalar_static_f64[171]!=0.0){((if (self.scalar_static_f64[171]!=0.0){v4149}else{common.v304})*v4186)}else{common.v304}))+(v136*v4213))}else{common.v304})});
        let v4243=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){((v134*(if (self.scalar_static_f64[171]!=0.0){((if (self.scalar_static_f64[171]!=0.0){common.v304}else{v4149})*v4186)}else{v4153}))+(v136*v4214))}else{common.v304})});
        let v4244=(if self.scalar_static_bool[46]{common.v304}else{(if (self.scalar_static_f64[171]!=0.0){((v134*(if (self.scalar_static_f64[171]!=0.0){((if (self.scalar_static_f64[171]!=0.0){v4148}else{common.v304})*v4186)}else{common.v304}))+(v136*v4215))}else{common.v304})});
        let v4245=(v1148*common.v1654);
        let v4248=(-v1148);
        let v4250=(common.v182*v1152);
        let v4260=(if (self.scalar_static_f64[173]!=0.0){(common.v185*(common.v1654+((v4245+v4245)/v4250)))}else{common.v3257});
        let v4261=(if (self.scalar_static_f64[173]!=0.0){(common.v185*(common.v51+((v1148+v1148)/v4250)))}else{common.v3258});
        let v4262=(if (self.scalar_static_f64[173]!=0.0){common.v304}else{common.v3259});
        let v4263=(if (self.scalar_static_f64[173]!=0.0){(common.v185*(common.v362+((v4248+v4248)/v4250)))}else{common.v3260});
        let v4264=(if (self.scalar_static_f64[173]!=0.0){common.v304}else{common.v3261});
        let v4265=(if (self.scalar_static_f64[173]!=0.0){common.v304}else{common.v3262});
        let v4266=(if (self.scalar_static_f64[173]!=0.0){common.v304}else{common.v3263});
        let v4277=(self.scalar_static_f64[174]*f64::powf(v1155,self.scalar_static_f64[309]));
        let v4294=scalar_limexp_derivative(v1160);
        let v4330=(v3526-v3508);
        let v4331=(v3529-v3512);
        let v4332=(v3532-v3515);
        let v4333=(v3536-v3519);
        let v4334=(v3540-v3522);
        let v4369=(v4175-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){((v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4260))+(v1156*(((v1159*self.scalar_static_f64[308])+(v1157*(v4260*v4277)))*v4294)))}else{common.v304}))+(v1163*(v4330-v4175)))}else{common.v304})}));
        let v4370=(v4176-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){((v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4261))+(v1156*((v1157*(v4261*v4277))*v4294)))}else{common.v304}))+(v1163*(v4331-v4176)))}else{common.v304})}));
        let v4371=(-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){((v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4262))+(v1156*((v1157*(v4262*v4277))*v4294)))}else{common.v304}))+(v1163*v4332))}else{common.v304})}));
        let v4372=(v4177-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){((v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4263))+(v1156*((v1157*(v4263*v4277))*v4294)))}else{common.v304}))+(v1163*(v4333-v4177)))}else{common.v304})}));
        let v4373=(-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){((v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4264))+(v1156*((v1157*(v4264*v4277))*v4294)))}else{common.v304}))+(v1163*v4334))}else{common.v304})}));
        let v4374=(-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){(v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4265))+(v1156*((v1157*(v4265*v4277))*v4294)))}else{common.v304}))}else{common.v304})}));
        let v4375=(-(if self.scalar_static_bool[48]{common.v304}else{(if (self.scalar_static_f64[173]!=0.0){(v1165*(if (self.scalar_static_f64[173]!=0.0){((v1161*(self.scalar_static_f64[172]*v4266))+(v1156*((v1157*(v4266*v4277))*v4294)))}else{common.v304}))}else{common.v304})}));
        let v4385=(if self.scalar_static_bool[50]{common.v304}else{(if (self.scalar_static_f64[175]!=0.0){(common.v51/v21)}else{common.v304})});
        let v4386=(if self.scalar_static_bool[50]{common.v304}else{(if (self.scalar_static_f64[175]!=0.0){((-(v1174*(self.scalar_static_f64[5]*(self.scalar_static_f64[194]*(self.scalar_static_f64[6]*f64::powf(common.v12,self.scalar_static_f64[196]))))))/(v21*v21))}else{common.v304})});
        let v4387=(if self.scalar_static_bool[50]{common.v304}else{(if (self.scalar_static_f64[175]!=0.0){(common.v362/v21)}else{common.v304})});
        let v4425=(v1192*v1192);
        let v4466=(if (self.scalar_static_f64[176]!=0.0){(((v25*((v1198*common.v1373)+(common.v11*((common.v4410-common.v4419)-((if (self.scalar_static_f64[176]!=0.0){(((v1192*common.v4410)-(v1191*common.v4419))/v4425)}else{common.v304})/v1194)))))-(v1200*v1389))/(v25*v25))}else{common.v304});
        let v4467=(if (self.scalar_static_f64[176]!=0.0){((common.v51+(common.v11*((-common.v4420)-((if (self.scalar_static_f64[176]!=0.0){((-(v1191*common.v4420))/v4425)}else{common.v304})/v1194))))/v25)}else{common.v304});
        let v4468=(if (self.scalar_static_f64[176]!=0.0){((common.v362+(common.v11*(common.v4411-((if (self.scalar_static_f64[176]!=0.0){(common.v4411/v1192)}else{common.v304})/v1194))))/v25)}else{common.v304});
        let v4469=(if (self.scalar_static_f64[176]!=0.0){((common.v11*((common.v4412-common.v4421)-((if (self.scalar_static_f64[176]!=0.0){(((v1192*common.v4412)-(v1191*common.v4421))/v4425)}else{common.v304})/v1194)))/v25)}else{common.v304});
        let v4482=(-v1195);
        let v4484=(common.v182*v1209);
        let v4493=(v1211*v1211);
        let v4508=(v1213*(if (self.scalar_static_f64[176]!=0.0){(((v1211*((v1203*v4466)+(v1202*((v325*v1389)+(v25*v1746)))))-(v1204*(v1209*(self.scalar_static_f64[111]*(common.v185*v1746)))))/v4493)}else{common.v304}));
        let v4510=(v1213*(if (self.scalar_static_f64[176]!=0.0){(((v1211*(v1203*v4467))-(v1204*(v1206*((v1195+v1195)/v4484))))/v4493)}else{common.v304}));
        let v4512=(v1213*(if (self.scalar_static_f64[176]!=0.0){(((v1211*(v1203*v4468))-(v1204*(v1206*((v4482+v4482)/v4484))))/v4493)}else{common.v304}));
        let v4514=(v1213*(if (self.scalar_static_f64[176]!=0.0){((v1203*v4469)/v1211)}else{common.v304}));
        let v4516=(common.v182*v1216);
        let v4524=(v1216*v1216);
        let v4542=(if self.scalar_static_bool[52]{common.v304}else{(if (self.scalar_static_f64[176]!=0.0){(((v1216*v4466)-(v1202*((v4508+v4508)/v4516)))/v4524)}else{common.v304})});
        let v4543=(if self.scalar_static_bool[52]{common.v304}else{(if (self.scalar_static_f64[176]!=0.0){(((v1216*v4467)-(v1202*((v4510+v4510)/v4516)))/v4524)}else{common.v304})});
        let v4544=(if self.scalar_static_bool[52]{common.v304}else{(if (self.scalar_static_f64[176]!=0.0){(((v1216*v4468)-(v1202*((v4512+v4512)/v4516)))/v4524)}else{common.v304})});
        let v4545=(if self.scalar_static_bool[52]{common.v304}else{(if (self.scalar_static_f64[176]!=0.0){(((v1216*v4469)-(v1202*((v4514+v4514)/v4516)))/v4524)}else{common.v304})});
        let v4555=(if self.scalar_static_bool[54]{common.v304}else{(if (self.scalar_static_f64[177]!=0.0){(common.v51/v29)}else{common.v304})});
        let v4556=(if self.scalar_static_bool[54]{common.v304}else{(if (self.scalar_static_f64[177]!=0.0){((-(v1224*(self.scalar_static_f64[9]*(self.scalar_static_f64[194]*(self.scalar_static_f64[10]*f64::powf(common.v12,self.scalar_static_f64[198]))))))/(v29*v29))}else{common.v304})});
        let v4557=(if self.scalar_static_bool[54]{common.v304}else{(if (self.scalar_static_f64[177]!=0.0){(common.v362/v29)}else{common.v304})});
        let v4580=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){(((v33*(v1231*common.v3499))-(v1232*(self.scalar_static_f64[11]*(self.scalar_static_f64[194]*(self.scalar_static_f64[12]*f64::powf(common.v12,self.scalar_static_f64[199]))))))/(v33*v33))}else{common.v304})});
        let v4581=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){((v1231*common.v3500)/v33)}else{common.v304})});
        let v4582=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){((common.v959+(v1231*common.v3501))/v33)}else{common.v304})});
        let v4583=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){(((v1231*common.v3502)+(-common.v959))/v33)}else{common.v304})});
        let v4584=(if self.scalar_static_bool[56]{common.v304}else{(if (self.scalar_static_f64[178]!=0.0){((v1231*common.v3503)/v33)}else{common.v304})});
        let v4594=(if self.scalar_static_bool[58]{common.v304}else{(if (self.scalar_static_f64[179]!=0.0){(common.v51/v37)}else{common.v304})});
        let v4595=(if self.scalar_static_bool[58]{common.v304}else{(if (self.scalar_static_f64[179]!=0.0){((-(v1240*(self.scalar_static_f64[13]*(self.scalar_static_f64[194]*(self.scalar_static_f64[14]*f64::powf(common.v12,self.scalar_static_f64[200]))))))/(v37*v37))}else{common.v304})});
        let v4596=(if self.scalar_static_bool[58]{common.v304}else{(if (self.scalar_static_f64[179]!=0.0){(common.v362/v37)}else{common.v304})});
        let v4620=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){(((v45*(v1247*(if self.scalar_static_bool[28]{common.v304}else{v3624})))-(v1248*(self.scalar_static_f64[17]*(self.scalar_static_f64[194]*(self.scalar_static_f64[18]*f64::powf(common.v12,self.scalar_static_f64[202]))))))/(v45*v45))}else{common.v304})});
        let v4621=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){((-v1001)/v45)}else{common.v304})});
        let v4622=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){((v1247*(if self.scalar_static_bool[28]{common.v304}else{v3625}))/v45)}else{common.v304})});
        let v4623=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){((v1247*(if self.scalar_static_bool[28]{common.v304}else{v3626}))/v45)}else{common.v304})});
        let v4624=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){((v1247*(if self.scalar_static_bool[28]{common.v304}else{v3627}))/v45)}else{common.v304})});
        let v4625=(if self.scalar_static_bool[60]{common.v304}else{(if (self.scalar_static_f64[180]!=0.0){((v1001+(v1247*(if self.scalar_static_bool[28]{common.v304}else{v3628})))/v45)}else{common.v304})});
        let v4638=scalar_limexp_derivative(v1259);
        let v4662=scalar_limexp_derivative(v1264);
        let v4701=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){(((v1267*(self.scalar_static_f64[60]*(((v142*v1470)+(v87*(v142*(((common.v11*self.scalar_static_f64[222])-(v140*common.v1373))/common.v1424))))*(self.scalar_static_f64[64]*f64::powf(v143,self.scalar_static_f64[223])))))+(v147*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){((-(common.v836*self.scalar_static_f64[310]))/(v1257*v1257))}else{common.v4390})*v4638)}else{common.v4394})))+((v1269*(self.scalar_static_f64[65]*(((v153*v1488)+(v100*(v153*(((common.v11*self.scalar_static_f64[224])-(v151*common.v1373))/common.v1424))))*(self.scalar_static_f64[69]*f64::powf(v154,self.scalar_static_f64[225])))))+(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){((-(common.v836*self.scalar_static_f64[311]))/(v1262*v1262))}else{v4200})*v4662)}else{v4211}))))}else{common.v304})});
        let v4702=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){((v147*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){common.v304}else{common.v4391})*v4638)}else{common.v4395}))+(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){common.v304}else{v4201})*v4662)}else{v4212})))}else{common.v304})});
        let v4703=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){common.v304}else{v4202})*v4662)}else{v4213}))}else{common.v304})});
        let v4704=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){((v147*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){common.v304}else{common.v4392})*v4638)}else{common.v4396}))+(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){common.v304}else{v4203})*v4662)}else{v4214})))}else{common.v304})});
        let v4705=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){((v147*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){(common.v362/v1257)}else{common.v304})*v4638)}else{common.v304}))+(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){(common.v362/v1262)}else{v4204})*v4662)}else{v4215})))}else{common.v304})});
        let v4706=(if self.scalar_static_bool[64]{common.v304}else{(if (self.scalar_static_f64[181]!=0.0){((v147*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){(common.v51/v1257)}else{common.v304})*v4638)}else{common.v304}))+(v158*(if (self.scalar_static_f64[181]!=0.0){((if (self.scalar_static_f64[181]!=0.0){(common.v51/v1262)}else{common.v304})*v4662)}else{common.v304})))}else{common.v304})});
        let v4716=(if self.scalar_static_bool[66]{common.v304}else{(if (self.scalar_static_f64[182]!=0.0){(common.v51/v41)}else{common.v304})});
        let v4717=(if self.scalar_static_bool[66]{common.v304}else{(if (self.scalar_static_f64[182]!=0.0){((-(v1278*(self.scalar_static_f64[15]*(self.scalar_static_f64[194]*(self.scalar_static_f64[16]*f64::powf(common.v12,self.scalar_static_f64[201]))))))/(v41*v41))}else{common.v304})});
        let v4718=(if self.scalar_static_bool[66]{common.v304}else{(if (self.scalar_static_f64[182]!=0.0){(common.v362/v41)}else{common.v304})});

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1091),
            [4, 6, 7, 8, 9, 10, 11],
            [v4029, v4030, v4031, v4032, v4033, v4034, v4035],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1113),
            [4, 6, 7, 8, 9, 10, 11],
            [v4136, v4137, v4138, v4139, v4140, v4141, v4142],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (v961),
            [4, 6, 7, 8, 9],
            [v3526, v3529, v3532, v3536, v3540],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (v960),
            [4, 6, 7, 8, 9],
            [v3508, v3512, v3515, v3519, v3522],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1170),
            [4, 6, 7, 8, 9, 10, 11],
            [v4369, v4370, v4371, v4372, v4373, v4374, v4375],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1144),
            [4, 6, 7, 8, 10],
            [v4240, v4241, v4242, v4243, v4244],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1178),
            0,
            multiplicity * (v4385),
            4,
            multiplicity * (v4386),
            5,
            multiplicity * (v4387),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1220),
            [4, 5, 6, 8],
            [v4542, v4543, v4544, v4545],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1228),
            1,
            multiplicity * (v4555),
            4,
            multiplicity * (v4556),
            7,
            multiplicity * (v4557),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1236),
            [4, 6, 7, 8, 9],
            [v4580, v4581, v4582, v4583, v4584],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1244),
            2,
            multiplicity * (v4594),
            4,
            multiplicity * (v4595),
            9,
            multiplicity * (v4596),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1252),
            [4, 5, 6, 7, 8, 10],
            [v4620, v4621, v4622, v4623, v4624, v4625],
            [],
            [],
            multiplicity,
        );
        let v1310_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v1310);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1310_ddt),
            [4, 6, 7, 8, 9],
            [((common.v4824) * ddt_scale), ((common.v4811) * ddt_scale), ((common.v4815) * ddt_scale), ((common.v4825) * ddt_scale), ((common.v4826) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1312_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v1312);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1312_ddt),
            [4, 7, 8, 9],
            [((common.v4833) * ddt_scale), ((common.v4834) * ddt_scale), ((common.v4835) * ddt_scale), ((common.v4836) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1319_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v1319);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1319_ddt),
            [4, 6, 7, 8, 9],
            [((common.v4853) * ddt_scale), ((common.v4854) * ddt_scale), ((common.v4841) * ddt_scale), ((common.v4855) * ddt_scale), ((common.v4843) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1320_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v1320);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v1320_ddt),
            4,
            multiplicity * (((common.v4856) * ddt_scale)),
            5,
            multiplicity * (((common.v4857) * ddt_scale)),
            8,
            multiplicity * (((common.v4858) * ddt_scale)),
        );
        let v1323_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v1323);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1323_ddt),
            [4, 6, 7, 8, 9, 10],
            [((common.v4872) * ddt_scale), ((common.v4873) * ddt_scale), ((common.v4874) * ddt_scale), ((common.v4875) * ddt_scale), ((common.v4865) * ddt_scale), ((common.v4876) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1330_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v1330);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v1330_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[189]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[316]) * ddt_scale)),
        );
        let v1333_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v1333);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v1333_ddt),
            0,
            multiplicity * (((self.scalar_static_f64[317]) * ddt_scale)),
            1,
            multiplicity * (((self.scalar_static_f64[190]) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1274),
            [4, 6, 7, 8, 10, 11],
            [v4701, v4702, v4703, v4704, v4705, v4706],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1002),
            [4, 6, 7, 8, 10, 11],
            [v3708, v3709, v3710, v3711, v3712, v3713],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1282),
            3,
            multiplicity * (v4716),
            4,
            multiplicity * (v4717),
            11,
            multiplicity * (v4718),
        );
        let v1327_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v1327);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1327_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v4879) * ddt_scale), ((common.v4880) * ddt_scale), ((common.v4881) * ddt_scale), ((common.v4882) * ddt_scale), ((common.v4883) * ddt_scale), ((common.v4887) * ddt_scale), ((common.v4888) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[68]{common.v304}else{(if (self.scalar_static_f64[192]!=0.0){(common.v6/self.scalar_static_f64[191])}else{common.v304})})),
            4,
            multiplicity * (self.scalar_static_f64[320]),
        );
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * ((-((((((((((((((common.v341*v1091)+(common.v345*v1170))+(v1164*v1337))+(common.v343*v1113))+(common.v349*v1144))+(v1278*v1282))+(common.v836*v1274))+(v1002*v1348))+(v1174*v1178))+(v1195*v1220))+(v1224*v1228))+(v1231*v1236))+(v1240*v1244))+(v1247*v1252)))),
            &[(-(v1178+(v1174*v4385))),(-(v1228+(v1224*v4555))),(-(v1244+(v1240*v4594))),(-(v1282+(v1278*v4716))),(-((((((((((((((common.v341*v4029)+(common.v345*v4369))+(v1337*v4330))+(common.v343*v4136))+(common.v349*v4240))+(v1278*v4717))+(common.v836*v4701))+(v1348*v3708))+(v1174*v4386))+(v1195*v4542))+(v1224*v4556))+(v1231*v4580))+(v1240*v4595))+(v1247*v4620))),(-((((-v1178)+(v1174*v4387))+(v1220+(v1195*v4543)))+((-v1252)+(v1247*v4621)))),(-((((((((((common.v341*v4030)+((-v1170)+(common.v345*v4370)))+(v1164+(v1337*v4331)))+(common.v343*v4137))+(common.v349*v4241))+(common.v836*v4702))+(v1348*v3709))+((-v1220)+(v1195*v4544)))+(v1231*v4581))+(v1247*v4622))),(-((((((((((common.v341*v4031)+(common.v345*v4371))+(v1337*v4332))+(v1113+(common.v343*v4138)))+(v1144+(common.v349*v4242)))+(common.v836*v4703))+(v1002+(v1348*v3710)))+((-v1228)+(v1224*v4557)))+(v1236+(v1231*v4582)))+(v1247*v4623))),(-((((((((((v1091+(common.v341*v4032))+(v1170+(common.v345*v4372)))+(v1337*v4333))+(common.v343*v4139))+(common.v349*v4243))+(common.v836*v4704))+(v1348*v3711))+(v1195*v4545))+((-v1236)+(v1231*v4583)))+(v1247*v4624))),(-(((((((-v1091)+(common.v341*v4033))+(common.v345*v4373))+((v1337*v4334)+(-v1164)))+((-v1113)+(common.v343*v4140)))+(v1231*v4584))+((-v1244)+(v1240*v4596)))),(-(((((((common.v341*v4034)+(common.v345*v4374))+(common.v343*v4141))+((-v1144)+(common.v349*v4244)))+((-v1274)+(common.v836*v4705)))+(v1348*v3712))+(v1252+(v1247*v4625)))),(-((((((common.v341*v4035)+(common.v345*v4375))+(common.v343*v4142))+((-v1282)+(v1278*v4718)))+(v1274+(common.v836*v4706)))+((v1348*v3713)+(-v1002))))],
            &[],
            multiplicity,
        );
        let v1372_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v1372);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v1372_ddt),
            4,
            multiplicity * (((self.scalar_static_f64[193]) * ddt_scale)),
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
            &[common.v4824, common.v4811, common.v4815, common.v4825, common.v4826],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &[nodes[4], nodes[7], nodes[8], nodes[9]],
            &[common.v4833, common.v4834, common.v4835, common.v4836],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[common.v4853, common.v4854, common.v4841, common.v4855, common.v4843],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (common.v4856),
            nodes[5],
            multiplicity * (common.v4857),
            nodes[8],
            multiplicity * (common.v4858),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[common.v4872, common.v4873, common.v4874, common.v4875, common.v4865, common.v4876],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[189]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[316]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (self.scalar_static_f64[317]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[190]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[common.v4879, common.v4880, common.v4881, common.v4882, common.v4883, common.v4887, common.v4888],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (self.scalar_static_f64[193]),
        );
    }
}
