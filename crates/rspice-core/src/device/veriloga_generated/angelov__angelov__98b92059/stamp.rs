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

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
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
        let v0=ctx.node_voltage(nodes[8]);
        let v1=ctx.node_voltage(nodes[5]);
        let v2=(v0-v1);
        let v3=ctx.node_voltage(nodes[4]);
        let v4=ctx.node_voltage(nodes[3]);
        let v5=(v3-v4);
        let v6=(-v5);
        let v7=(v4-v1);
        let v8=ctx.node_voltage(nodes[7]);
        let v9=(v8-v4);
        let v10=ctx.node_voltage(nodes[13]);
        let v11=0.0;
        let v30=ctx.node_voltage(nodes[11]);
        let v33=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[137]+(v30).abs())}else{self.scalar_static_f64[137]});
        let v37=((v33-self.scalar_static_f64[9])).abs();
        let v41=((v37>v11)||self.scalar_static_bool[2]);
        let v42=1.0;
        let v112=(!v41);
        let v113=(if v112{self.scalar_static_f64[12]}else{(if v41{(self.scalar_static_f64[12]*(v42+(v37*self.scalar_static_f64[13])))}else{v11})});
        let v114=(if v112{self.scalar_static_f64[14]}else{(if v41{(self.scalar_static_f64[14]*(v42+(v37*self.scalar_static_f64[15])))}else{v11})});
        let v115=(if v112{self.scalar_static_f64[16]}else{(if v41{(self.scalar_static_f64[16]*(v42+(v37*self.scalar_static_f64[17])))}else{v11})});
        let v116=(if v112{self.scalar_static_f64[18]}else{(if v41{(self.scalar_static_f64[18]*(v42+(v37*self.scalar_static_f64[19])))}else{v11})});
        let v117=(if v112{self.scalar_static_f64[20]}else{(if v41{(self.scalar_static_f64[20]*(v42+(v37*self.scalar_static_f64[21])))}else{v11})});
        let v118=(if v112{self.scalar_static_f64[22]}else{(if v41{(self.scalar_static_f64[22]*(v42+(v37*self.scalar_static_f64[23])))}else{v11})});
        let v119=(if v112{self.scalar_static_f64[24]}else{(if v41{(self.scalar_static_f64[24]*(v42+(v37*self.scalar_static_f64[25])))}else{v11})});
        let v121=(if v112{self.scalar_static_f64[28]}else{(if v41{(self.scalar_static_f64[28]+(v37*self.scalar_static_f64[30]))}else{v11})});
        let v122=(if v112{self.scalar_static_f64[31]}else{(if v41{(self.scalar_static_f64[31]+(v37*self.scalar_static_f64[33]))}else{v11})});
        let v123=(if v112{self.scalar_static_f64[34]}else{(if v41{(self.scalar_static_f64[34]+(v37*self.scalar_static_f64[35]))}else{v11})});
        let v124=(if v112{self.scalar_static_f64[36]}else{(if v41{(self.scalar_static_f64[36]+(v37*self.scalar_static_f64[37]))}else{v11})});
        let v129=0.5;
        let v136=(if self.scalar_static_bool[5]{self.scalar_static_f64[42]}else{(if self.scalar_static_bool[4]{(self.scalar_static_f64[41]/(v33*8.617333262145179e-5))}else{v11})});
        let v138=(v7*self.scalar_static_f64[43]);
        let v139=(v138).cosh();
        let v141=(v139*v139);
        let v144=(v114*(v42+(self.scalar_static_f64[44]/v141)));
        let v149=((v7*self.scalar_static_f64[46])).tanh();
        let v154=(self.scalar_static_f64[47]*(v6-self.scalar_static_f64[36]));
        let v155=(v6-v124);
        let v157=((((if v112{self.scalar_static_f64[26]}else{(if v41{(self.scalar_static_f64[26]+(v37*self.scalar_static_f64[27]))}else{v11})})-self.scalar_static_f64[45])+(self.scalar_static_f64[45]*v149))-(v154*v155));
        let v158=(v2-v157);
        let v159=(v158*v158);
        let v165=(v158*self.scalar_static_f64[49]);
        let v167=(((v144*v158)+(v159*self.scalar_static_f64[48]))+(v159*v165));
        let v168=(v167).tanh();
        let v169=(v42+v168);
        let v171=(-v167);
        let v175=((v129*(scalar_limexp(v167)-scalar_limexp(v171)))).tanh();
        let v179=(self.scalar_static_f64[50]+(self.scalar_static_f64[46]*v169));
        let v181=((v7*v179)).tanh();
        let v189=(v113*v169);
        let v190=(v181*v189);
        let v195=(v115*scalar_limexp(v155));
        let v196=((v42+(v7*self.scalar_static_f64[52]))+v195);
        let v201=(v5-v157);
        let v202=(if self.scalar_static_bool[11]{v201}else{v139});
        let v204=(if self.scalar_static_bool[11]{(v202*v202)}else{v158});
        let v206=(if self.scalar_static_bool[11]{(v202*v204)}else{v159});
        let v212=(if self.scalar_static_bool[11]{(((v144*v202)+(self.scalar_static_f64[48]*v204))+(self.scalar_static_f64[49]*v206))}else{v11});
        let v213=(v212).tanh();
        let v215=(if self.scalar_static_bool[11]{(v42+v213)}else{v11});
        let v218=(if self.scalar_static_bool[11]{(self.scalar_static_f64[50]+(self.scalar_static_f64[46]*v215))}else{v11});
        let v222=(if self.scalar_static_bool[11]{(self.scalar_static_f64[52]+(v169*self.scalar_static_f64[53]))}else{v11});
        let v223=(v42+v181);
        let v224=(v189*v223);
        let v227=(v7-v124);
        let v229=(v115*scalar_limexp(v227));
        let v230=((v42+(v7*v222))+v229);
        let v232=(if self.scalar_static_bool[11]{(v224*v230)}else{v11});
        let v235=(if self.scalar_static_bool[11]{(self.scalar_static_f64[52]+(v215*self.scalar_static_f64[53]))}else{v11});
        let v237=((v7*v218)).tanh();
        let v239=(v113*v215);
        let v240=(v42-(if self.scalar_static_bool[11]{v237}else{v11}));
        let v241=(v239*v240);
        let v243=(v42-(v7*v235));
        let v245=(if self.scalar_static_bool[11]{(v241*v243)}else{v11});
        let v252=(if self.scalar_static_bool[14]{v158}else{v202});
        let v254=(if self.scalar_static_bool[14]{(v252*v252)}else{v204});
        let v257=(self.scalar_static_f64[49]*v254);
        let v259=((v252+(self.scalar_static_f64[48]*v254))+(v252*v257));
        let v261=(if self.scalar_static_bool[14]{(v144*v259)}else{v167});
        let v263=(-v261);
        let v267=((v129*(scalar_limexp(v261)-scalar_limexp(v263)))).tanh();
        let v269=(if self.scalar_static_bool[14]{(v42+v267)}else{(v42+v175)});
        let v272=(if self.scalar_static_bool[14]{(self.scalar_static_f64[50]+(self.scalar_static_f64[46]*v269))}else{v11});
        let v274=((v7*v272)).tanh();
        let v275=(if self.scalar_static_bool[14]{v274}else{v11});
        let v278=(if self.scalar_static_bool[14]{(self.scalar_static_f64[52]+(self.scalar_static_f64[53]*v269))}else{v222});
        let v279=(v113*v269);
        let v280=(v275*v279);
        let v283=(v195+(v42+(v7*v278)));
        let v289=(if self.scalar_static_bool[17]{v158}else{v252});
        let v291=(if self.scalar_static_bool[17]{(v289*v289)}else{v254});
        let v294=(self.scalar_static_f64[49]*v291);
        let v296=((v289+(self.scalar_static_f64[48]*v291))+(v289*v294));
        let v298=(if self.scalar_static_bool[17]{(v144*v296)}else{v261});
        let v299=(if self.scalar_static_bool[17]{v201}else{v206});
        let v301=(if self.scalar_static_bool[17]{(v299*v299)}else{v11});
        let v304=(self.scalar_static_f64[49]*v299);
        let v306=((v299+(self.scalar_static_f64[48]*v301))+(v301*v304));
        let v308=(if self.scalar_static_bool[17]{(v144*v306)}else{v212});
        let v310=(-v298);
        let v314=((v129*(scalar_limexp(v298)-scalar_limexp(v310)))).tanh();
        let v316=(if self.scalar_static_bool[17]{(v42+v314)}else{v269});
        let v318=(-v308);
        let v322=((v129*(scalar_limexp(v308)-scalar_limexp(v318)))).tanh();
        let v324=(if self.scalar_static_bool[17]{(v42+v322)}else{v11});
        let v327=(if self.scalar_static_bool[17]{(self.scalar_static_f64[50]+(self.scalar_static_f64[46]*v316))}else{v272});
        let v330=(if self.scalar_static_bool[17]{(self.scalar_static_f64[50]+(self.scalar_static_f64[46]*v324))}else{v11});
        let v332=((v7*v327)).tanh();
        let v335=((v7*v330)).tanh();
        let v339=(if self.scalar_static_bool[17]{(self.scalar_static_f64[52]+(self.scalar_static_f64[53]*v324))}else{v11});
        let v342=(if self.scalar_static_bool[17]{(self.scalar_static_f64[52]+(self.scalar_static_f64[53]*v316))}else{v11});
        let v343=(v113*v316);
        let v344=(v42+(if self.scalar_static_bool[17]{v332}else{v275}));
        let v345=(v343*v344);
        let v348=(v229+(v42+(v7*v342)));
        let v351=(v113*v324);
        let v352=(v42-(if self.scalar_static_bool[17]{v335}else{v11}));
        let v353=(v351*v352);
        let v355=(v42-(v7*v339));
        let v362=(v42+v169);
        let v368=(v169*self.scalar_static_f64[56]);
        let v374=(v42+v316);
        let v377=(if self.scalar_static_bool[13]{(self.scalar_static_f64[54]+(v118/v374))}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[54]+(v118/v362))}else{v11})});
        let v378=(v316*self.scalar_static_f64[56]);
        let v380=(if self.scalar_static_bool[13]{(self.scalar_static_f64[55]+v378)}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[55]+v368)}else{v11})});
        let v382=(if self.scalar_static_bool[13]{(self.scalar_static_f64[57]+v378)}else{(if self.scalar_static_bool[12]{(v368+self.scalar_static_f64[57])}else{v11})});
        let v383=((v37!=0.0)||self.scalar_static_bool[2]);
        let v386=(v42+(v37*self.scalar_static_f64[58]));
        let v391=(!v383);
        let v392=(if v391{v380}else{(if v383{(v380*v386)}else{v11})});
        let v393=(if v391{v382}else{(if v383{(v382*v386)}else{v11})});
        let v396=-1.0;
        let v402=(v2-v123);
        let v404=(v9-v123);
        let v410=(if self.scalar_static_bool[19]{scalar_limexp((v123*(-v136)))}else{(if self.scalar_static_bool[18]{scalar_limexp((v136*((-v123)).tanh()))}else{v289})});
        let v413=(v402).tanh();
        let v415=(v404).tanh();
        let v422=(v136*(if self.scalar_static_bool[23]{v402}else{(if self.scalar_static_bool[21]{v413}else{(if self.scalar_static_bool[18]{v402}else{v11})})}));
        let v425=(self.scalar_static_f64[60]*(scalar_limexp(v422)-v410));
        let v426=(v136*(if self.scalar_static_bool[23]{v404}else{(if self.scalar_static_bool[21]{v415}else{(if self.scalar_static_bool[18]{v404}else{v11})})}));
        let v433=(v7*self.scalar_static_f64[61]);
        let v434=((v121+(v2*self.scalar_static_f64[29]))+v433);
        let v435=(v434).tanh();
        let v441=((self.scalar_static_f64[62]+(v7*self.scalar_static_f64[63]))).tanh();
        let v442=(v42+v441);
        let v447=((self.scalar_static_f64[64]-(v7*self.scalar_static_f64[65]))).tanh();
        let v449=((v42+v447)-self.scalar_static_f64[61]);
        let v452=((v122+(v9*self.scalar_static_f64[32]))-v433);
        let v453=(v452).tanh();
        let v454=(v42+v453);
        let v465=(v116*(v42+v435));
        let v479=(if self.scalar_static_bool[31]{(v442-self.scalar_static_f64[61])}else{v442});
        let v480=(v121+v433);
        let v482=(if self.scalar_static_bool[31]{(v480).cosh()}else{v11});
        let v486=(if self.scalar_static_bool[31]{(v434).cosh()}else{v11});
        let v492=((v434+(if self.scalar_static_bool[31]{(v486).ln()}else{v11}))-(if self.scalar_static_bool[31]{(v480+(if self.scalar_static_bool[31]{(v482).ln()}else{v11}))}else{v11}));
        let v501=(v122-v433);
        let v503=(if self.scalar_static_bool[31]{(v501).cosh()}else{v482});
        let v507=(if self.scalar_static_bool[31]{(v452).cosh()}else{v486});
        let v513=((v452+(if self.scalar_static_bool[31]{(v507).ln()}else{v11}))-(if self.scalar_static_bool[31]{(v501+(if self.scalar_static_bool[31]{(v503).ln()}else{v11}))}else{v11}));
        let v1716=(v434).sinh();
        let v1722=(if self.scalar_static_bool[31]{(self.scalar_static_f64[29]*v1716)}else{v11});
        let v1759=(if self.scalar_static_bool[31]{(self.scalar_static_f64[67]+(v116*(self.scalar_static_f64[71]+((v479*(self.scalar_static_f64[29]+(if self.scalar_static_bool[31]{(v1722/v486)}else{v11})))/self.scalar_static_f64[29]))))}else{v11});
        let v522=v1759;
        let v523=(if self.scalar_static_bool[31]{v522}else{(if self.scalar_static_bool[28]{(self.scalar_static_f64[67]+(v442*v465))}else{self.scalar_static_f64[68]})});
        let v1769=(v452).sinh();
        let v1817=(if self.scalar_static_bool[31]{(self.scalar_static_f64[69]+(v117*(self.scalar_static_f64[71]+((v449*(self.scalar_static_f64[32]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[32]*v1769)}else{v11})/v507)}else{v11})))/self.scalar_static_f64[32]))))}else{v11});
        let v524=v1817;
        let v525=(if self.scalar_static_bool[31]{v524}else{(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]+(v117*((v449*v454)+self.scalar_static_f64[71])))}else{self.scalar_static_f64[70]})});
        let v560=(if self.scalar_static_bool[47]{((v116*((v33*5.5226012e-23)*self.scalar_static_f64[82]))*self.scalar_static_f64[84])}else{v11});
        let v564=(if self.scalar_static_bool[47]{((v42-(v560*v560))).sqrt()}else{v11});
        let v566=3.141592653589793;
        let v568=(if self.scalar_static_bool[47]{((-v560)*v566)}else{v11});
        let v579=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (if self.scalar_static_bool[31]{((v117*(((v449*v513)/self.scalar_static_f64[32])+(v9*self.scalar_static_f64[71])))+(v9*self.scalar_static_f64[69]))}else{v11}));
        let v581=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (if self.scalar_static_bool[31]{((v116*(((v479*v492)/self.scalar_static_f64[29])+(v2*self.scalar_static_f64[71])))+(v2*self.scalar_static_f64[67]))}else{v11}));
        let v585=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (v9*v525));
        let v588=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v2*v523));
        let v596=ctx.node_voltage(nodes[10]);
        let v599=(v596-v1);
        let v603=ctx.node_voltage(nodes[9]);
        let v620=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (self.scalar_static_f64[76]*ctx.branch_current(branches[6])));
        let v626=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, (self.scalar_static_f64[76]*ctx.branch_current(branches[8])));
        let v628=ctx.branch_current(branches[10]);
        let v634=ctx.branch_current(branches[14]);
        let v639=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, (self.scalar_static_f64[77]*ctx.branch_current(branches[15])));
        let v645=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, (self.scalar_static_f64[77]*ctx.branch_current(branches[17])));
        let v647=ctx.node_voltage(nodes[14]);
        let v648=(if self.scalar_static_bool[47]{v647}else{v11});
        let v649=ctx.node_voltage(nodes[15]);
        let v655=(-(if self.scalar_static_bool[47]{(v560*v566)}else{v11}));
        let v657=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (v647*v655));
        let v661=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (v30*self.scalar_static_f64[91]));
        let v677=(v138).sinh();
        let v678=(self.scalar_static_f64[43]*v677);
        let v679=(self.scalar_static_f64[92]*v677);
        let v680=(v139*v678);
        let v682=(v139*v679);
        let v686=(v141*v141);
        let v691=(v114*((-(self.scalar_static_f64[44]*(v680+v680)))/v686));
        let v692=(v114*((-(self.scalar_static_f64[44]*(v682+v682)))/v686));
        let v695=(v42-(v149*v149));
        let v699=(self.scalar_static_f64[45]*(self.scalar_static_f64[93]*v695));
        let v705=((v155*self.scalar_static_f64[94])+(-v154));
        let v706=((self.scalar_static_f64[45]*(self.scalar_static_f64[46]*v695))-(v154+(self.scalar_static_f64[47]*v155)));
        let v708=(-v706);
        let v709=(v396-v699);
        let v710=(v158*v708);
        let v711=(v710+v710);
        let v712=(v158*v705);
        let v713=(v712+v712);
        let v714=(v158*v709);
        let v715=(v714+v714);
        let v716=(v158+v158);
        let v747=((((v158*v691)+(v144*v708))+(self.scalar_static_f64[48]*v711))+((v165*v711)+(v159*(self.scalar_static_f64[49]*v708))));
        let v748=(((v144*v705)+(self.scalar_static_f64[48]*v713))+((v165*v713)+(v159*(self.scalar_static_f64[49]*v705))));
        let v749=((((v158*v692)+(v144*v709))+(self.scalar_static_f64[48]*v715))+((v165*v715)+(v159*(self.scalar_static_f64[49]*v709))));
        let v750=((v144+(self.scalar_static_f64[48]*v716))+((v165*v716)+(v159*self.scalar_static_f64[49])));
        let v752=(v42-(v168*v168));
        let v753=(v747*v752);
        let v754=(v748*v752);
        let v755=(v749*v752);
        let v756=(v750*v752);
        let v757=scalar_limexp_derivative(v167);
        let v766=scalar_limexp_derivative(v171);
        let v780=(v42-(v175*v175));
        let v797=(v42-(v181*v181));
        let v802=(v113*v753);
        let v803=(v113*v754);
        let v804=(v113*v755);
        let v805=(v113*v756);
        let v806=(v189*((v179+(v7*(self.scalar_static_f64[46]*v753)))*v797));
        let v809=(v189*((v7*(self.scalar_static_f64[46]*v754))*v797));
        let v812=(v189*(((-v179)+(v7*(self.scalar_static_f64[46]*v755)))*v797));
        let v815=(v189*((v7*(self.scalar_static_f64[46]*v756))*v797));
        let v819=scalar_limexp_derivative(v155);
        let v821=(v115*v819);
        let v822=(v115*(-v819));
        let v838=(v396-v706);
        let v839=(v42-(-v705));
        let v840=(-v699);
        let v841=(if self.scalar_static_bool[11]{v838}else{v678});
        let v842=(if self.scalar_static_bool[11]{v839}else{v11});
        let v843=(if self.scalar_static_bool[11]{v840}else{v679});
        let v844=(v202*v841);
        let v846=(v202*v842);
        let v848=(v202*v843);
        let v850=(if self.scalar_static_bool[11]{(v844+v844)}else{v708});
        let v851=(if self.scalar_static_bool[11]{(v846+v846)}else{v705});
        let v852=(if self.scalar_static_bool[11]{(v848+v848)}else{v709});
        let v864=(if self.scalar_static_bool[11]{((v204*v841)+(v202*v850))}else{v711});
        let v865=(if self.scalar_static_bool[11]{((v204*v842)+(v202*v851))}else{v713});
        let v866=(if self.scalar_static_bool[11]{((v204*v843)+(v202*v852))}else{v715});
        let v867=(if self.scalar_static_bool[11]{(v202*self.scalar_static_f64[96])}else{v716});
        let v890=(if self.scalar_static_bool[11]{((((v202*v691)+(v144*v841))+(self.scalar_static_f64[48]*v850))+(self.scalar_static_f64[49]*v864))}else{v11});
        let v891=(if self.scalar_static_bool[11]{(((v144*v842)+(self.scalar_static_f64[48]*v851))+(self.scalar_static_f64[49]*v865))}else{v11});
        let v892=(if self.scalar_static_bool[11]{((((v202*v692)+(v144*v843))+(self.scalar_static_f64[48]*v852))+(self.scalar_static_f64[49]*v866))}else{v11});
        let v893=(if self.scalar_static_bool[11]{(self.scalar_static_f64[97]+(self.scalar_static_f64[49]*v867))}else{v11});
        let v895=(v42-(v213*v213));
        let v900=(if self.scalar_static_bool[11]{(v890*v895)}else{v11});
        let v901=(if self.scalar_static_bool[11]{(v891*v895)}else{v11});
        let v902=(if self.scalar_static_bool[11]{(v892*v895)}else{v11});
        let v903=(if self.scalar_static_bool[11]{(v893*v895)}else{v11});
        let v916=(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v753)}else{v11});
        let v917=(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v754)}else{v11});
        let v918=(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v755)}else{v11});
        let v919=(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v756)}else{v11});
        let v935=scalar_limexp_derivative(v227);
        let v937=(v115*v935);
        let v938=(v115*(-v935));
        let v953=(if self.scalar_static_bool[11]{((v230*(v806+(v223*v802)))+(v224*((v222+(v7*v916))+v937)))}else{v11});
        let v954=(if self.scalar_static_bool[11]{((v230*(v809+(v223*v803)))+(v224*(v7*v917)))}else{v11});
        let v955=(if self.scalar_static_bool[11]{((v230*(v812+(v223*v804)))+(v224*(((-v222)+(v7*v918))+v938)))}else{v11});
        let v956=(if self.scalar_static_bool[11]{((v230*(v815+(v223*v805)))+(v224*(v7*v919)))}else{v11});
        let v973=(v42-(v237*v237));
        let v1025=(if self.scalar_static_bool[11]{((v243*((v240*(v113*v900))+(v239*(-(if self.scalar_static_bool[11]{((v218+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[46]*v900)}else{v11})))*v973)}else{v11})))))+(v241*(-(v235+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v900)}else{v11}))))))}else{v11});
        let v1026=(if self.scalar_static_bool[11]{((v243*((v240*(v113*v901))+(v239*(-(if self.scalar_static_bool[11]{((v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[46]*v901)}else{v11}))*v973)}else{v11})))))+(v241*(-(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v901)}else{v11})))))}else{v11});
        let v1027=(if self.scalar_static_bool[11]{((v243*((v240*(v113*v902))+(v239*(-(if self.scalar_static_bool[11]{(((-v218)+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[46]*v902)}else{v11})))*v973)}else{v11})))))+(v241*(-((-v235)+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v902)}else{v11}))))))}else{v11});
        let v1028=(if self.scalar_static_bool[11]{((v243*((v240*(v113*v903))+(v239*(-(if self.scalar_static_bool[11]{((v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[46]*v903)}else{v11}))*v973)}else{v11})))))+(v241*(-(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[53]*v903)}else{v11})))))}else{v11});
        let v1041=(if self.scalar_static_bool[14]{v708}else{v841});
        let v1042=(if self.scalar_static_bool[14]{v705}else{v842});
        let v1043=(if self.scalar_static_bool[14]{v709}else{v843});
        let v1045=(v252*v1041);
        let v1047=(v252*v1042);
        let v1049=(v252*v1043);
        let v1051=(v252*self.scalar_static_f64[98]);
        let v1053=(if self.scalar_static_bool[14]{(v1045+v1045)}else{v850});
        let v1054=(if self.scalar_static_bool[14]{(v1047+v1047)}else{v851});
        let v1055=(if self.scalar_static_bool[14]{(v1049+v1049)}else{v852});
        let v1056=(if self.scalar_static_bool[14]{(v1051+v1051)}else{self.scalar_static_f64[96]});
        let v1093=(if self.scalar_static_bool[14]{((v259*v691)+(v144*((v1041+(self.scalar_static_f64[48]*v1053))+((v257*v1041)+(v252*(self.scalar_static_f64[49]*v1053))))))}else{v747});
        let v1094=(if self.scalar_static_bool[14]{(v144*((v1042+(self.scalar_static_f64[48]*v1054))+((v257*v1042)+(v252*(self.scalar_static_f64[49]*v1054)))))}else{v748});
        let v1095=(if self.scalar_static_bool[14]{((v259*v692)+(v144*((v1043+(self.scalar_static_f64[48]*v1055))+((v257*v1043)+(v252*(self.scalar_static_f64[49]*v1055))))))}else{v749});
        let v1096=(if self.scalar_static_bool[14]{(v144*((self.scalar_static_f64[98]+(self.scalar_static_f64[48]*v1056))+((v257*self.scalar_static_f64[98])+(v252*(self.scalar_static_f64[49]*v1056)))))}else{v750});
        let v1097=scalar_limexp_derivative(v261);
        let v1106=scalar_limexp_derivative(v263);
        let v1120=(v42-(v267*v267));
        let v1125=(if self.scalar_static_bool[14]{((v129*((v1093*v1097)-((-v1093)*v1106)))*v1120)}else{((v129*((v747*v757)-((-v747)*v766)))*v780)});
        let v1126=(if self.scalar_static_bool[14]{((v129*((v1094*v1097)-((-v1094)*v1106)))*v1120)}else{((v129*((v748*v757)-((-v748)*v766)))*v780)});
        let v1127=(if self.scalar_static_bool[14]{((v129*((v1095*v1097)-((-v1095)*v1106)))*v1120)}else{((v129*((v749*v757)-((-v749)*v766)))*v780)});
        let v1128=(if self.scalar_static_bool[14]{((v129*((v1096*v1097)-((-v1096)*v1106)))*v1120)}else{((v129*((v750*v757)-((-v750)*v766)))*v780)});
        let v1133=(if self.scalar_static_bool[14]{(self.scalar_static_f64[46]*v1125)}else{v11});
        let v1134=(if self.scalar_static_bool[14]{(self.scalar_static_f64[46]*v1126)}else{v11});
        let v1135=(if self.scalar_static_bool[14]{(self.scalar_static_f64[46]*v1127)}else{v11});
        let v1136=(if self.scalar_static_bool[14]{(self.scalar_static_f64[46]*v1128)}else{v11});
        let v1145=(v42-(v274*v274));
        let v1150=(if self.scalar_static_bool[14]{((v272+(v7*v1133))*v1145)}else{v11});
        let v1151=(if self.scalar_static_bool[14]{((v7*v1134)*v1145)}else{v11});
        let v1152=(if self.scalar_static_bool[14]{(((-v272)+(v7*v1135))*v1145)}else{v11});
        let v1153=(if self.scalar_static_bool[14]{((v7*v1136)*v1145)}else{v11});
        let v1203=(if self.scalar_static_bool[17]{v708}else{v1041});
        let v1204=(if self.scalar_static_bool[17]{v705}else{v1042});
        let v1205=(if self.scalar_static_bool[17]{v709}else{v1043});
        let v1207=(v289*v1203);
        let v1209=(v289*v1204);
        let v1211=(v289*v1205);
        let v1213=(v289*self.scalar_static_f64[99]);
        let v1215=(if self.scalar_static_bool[17]{(v1207+v1207)}else{v1053});
        let v1216=(if self.scalar_static_bool[17]{(v1209+v1209)}else{v1054});
        let v1217=(if self.scalar_static_bool[17]{(v1211+v1211)}else{v1055});
        let v1218=(if self.scalar_static_bool[17]{(v1213+v1213)}else{v1056});
        let v1255=(if self.scalar_static_bool[17]{((v296*v691)+(v144*((v1203+(self.scalar_static_f64[48]*v1215))+((v294*v1203)+(v289*(self.scalar_static_f64[49]*v1215))))))}else{v1093});
        let v1256=(if self.scalar_static_bool[17]{(v144*((v1204+(self.scalar_static_f64[48]*v1216))+((v294*v1204)+(v289*(self.scalar_static_f64[49]*v1216)))))}else{v1094});
        let v1257=(if self.scalar_static_bool[17]{((v296*v692)+(v144*((v1205+(self.scalar_static_f64[48]*v1217))+((v294*v1205)+(v289*(self.scalar_static_f64[49]*v1217))))))}else{v1095});
        let v1258=(if self.scalar_static_bool[17]{(v144*((self.scalar_static_f64[99]+(self.scalar_static_f64[48]*v1218))+((v294*self.scalar_static_f64[99])+(v289*(self.scalar_static_f64[49]*v1218)))))}else{v1096});
        let v1259=(if self.scalar_static_bool[17]{v838}else{v864});
        let v1260=(if self.scalar_static_bool[17]{v839}else{v865});
        let v1261=(if self.scalar_static_bool[17]{v840}else{v866});
        let v1262=(if self.scalar_static_bool[17]{v11}else{v867});
        let v1263=(v299*v1259);
        let v1265=(v299*v1260);
        let v1267=(v299*v1261);
        let v1269=(v299*v1262);
        let v1271=(if self.scalar_static_bool[17]{(v1263+v1263)}else{v11});
        let v1272=(if self.scalar_static_bool[17]{(v1265+v1265)}else{v11});
        let v1273=(if self.scalar_static_bool[17]{(v1267+v1267)}else{v11});
        let v1274=(if self.scalar_static_bool[17]{(v1269+v1269)}else{v11});
        let v1311=(if self.scalar_static_bool[17]{((v306*v691)+(v144*((v1259+(self.scalar_static_f64[48]*v1271))+((v304*v1271)+(v301*(self.scalar_static_f64[49]*v1259))))))}else{v890});
        let v1312=(if self.scalar_static_bool[17]{(v144*((v1260+(self.scalar_static_f64[48]*v1272))+((v304*v1272)+(v301*(self.scalar_static_f64[49]*v1260)))))}else{v891});
        let v1313=(if self.scalar_static_bool[17]{((v306*v692)+(v144*((v1261+(self.scalar_static_f64[48]*v1273))+((v304*v1273)+(v301*(self.scalar_static_f64[49]*v1261))))))}else{v892});
        let v1314=(if self.scalar_static_bool[17]{(v144*((v1262+(self.scalar_static_f64[48]*v1274))+((v304*v1274)+(v301*(self.scalar_static_f64[49]*v1262)))))}else{v893});
        let v1315=scalar_limexp_derivative(v298);
        let v1324=scalar_limexp_derivative(v310);
        let v1338=(v42-(v314*v314));
        let v1343=(if self.scalar_static_bool[17]{((v129*((v1255*v1315)-((-v1255)*v1324)))*v1338)}else{v1125});
        let v1344=(if self.scalar_static_bool[17]{((v129*((v1256*v1315)-((-v1256)*v1324)))*v1338)}else{v1126});
        let v1345=(if self.scalar_static_bool[17]{((v129*((v1257*v1315)-((-v1257)*v1324)))*v1338)}else{v1127});
        let v1346=(if self.scalar_static_bool[17]{((v129*((v1258*v1315)-((-v1258)*v1324)))*v1338)}else{v1128});
        let v1347=scalar_limexp_derivative(v308);
        let v1356=scalar_limexp_derivative(v318);
        let v1370=(v42-(v322*v322));
        let v1375=(if self.scalar_static_bool[17]{((v129*((v1311*v1347)-((-v1311)*v1356)))*v1370)}else{v11});
        let v1376=(if self.scalar_static_bool[17]{((v129*((v1312*v1347)-((-v1312)*v1356)))*v1370)}else{v11});
        let v1377=(if self.scalar_static_bool[17]{((v129*((v1313*v1347)-((-v1313)*v1356)))*v1370)}else{v11});
        let v1378=(if self.scalar_static_bool[17]{((v129*((v1314*v1347)-((-v1314)*v1356)))*v1370)}else{v11});
        let v1403=(v42-(v332*v332));
        let v1420=(v42-(v335*v335));
        let v1541=(if self.scalar_static_bool[17]{(v129*((if self.scalar_static_bool[17]{((v348*((v344*(v113*v1343))+(v343*(if self.scalar_static_bool[17]{((v327+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1343)}else{v1133})))*v1403)}else{v1150}))))+(v345*(v937+(v342+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1343)}else{v11}))))))}else{v953})-(if self.scalar_static_bool[17]{((v355*((v352*(v113*v1375))+(v351*(-(if self.scalar_static_bool[17]{((v330+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1375)}else{v11})))*v1420)}else{v11})))))+(v353*(-(v339+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1375)}else{v11}))))))}else{v1025})))}else{(if self.scalar_static_bool[14]{((v283*((v279*v1150)+(v275*(v113*v1125))))+(v280*(v821+(v278+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[53]*v1125)}else{v916}))))))}else{(if self.scalar_static_bool[11]{(v129*(v953-v1025))}else{(if self.scalar_static_bool[6]{((v196*(v806+(v181*v802)))+(v190*(self.scalar_static_f64[52]+v821)))}else{v11})})})});
        let v1543=(if self.scalar_static_bool[17]{(v129*((if self.scalar_static_bool[17]{((v348*((v344*(v113*v1345))+(v343*(if self.scalar_static_bool[17]{(((-v327)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1345)}else{v1135})))*v1403)}else{v1152}))))+(v345*(v938+((-v342)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1345)}else{v11}))))))}else{v955})-(if self.scalar_static_bool[17]{((v355*((v352*(v113*v1377))+(v351*(-(if self.scalar_static_bool[17]{(((-v330)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1377)}else{v11})))*v1420)}else{v11})))))+(v353*(-((-v339)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1377)}else{v11}))))))}else{v1027})))}else{(if self.scalar_static_bool[14]{((v283*((v279*v1152)+(v275*(v113*v1127))))+(v280*((-v278)+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[53]*v1127)}else{v918})))))}else{(if self.scalar_static_bool[11]{(v129*(v955-v1027))}else{(if self.scalar_static_bool[6]{((v196*(v812+(v181*v804)))+(v190*self.scalar_static_f64[95]))}else{v11})})})});
        let v1547=(v362*v362);
        let v1572=(v374*v374);
        let v1591=(if self.scalar_static_bool[13]{(self.scalar_static_f64[56]*v1343)}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[56]*v753)}else{v11})});
        let v1592=(if self.scalar_static_bool[13]{(self.scalar_static_f64[56]*v1344)}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[56]*v754)}else{v11})});
        let v1593=(if self.scalar_static_bool[13]{(self.scalar_static_f64[56]*v1345)}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[56]*v755)}else{v11})});
        let v1594=(if self.scalar_static_bool[13]{(self.scalar_static_f64[56]*v1346)}else{(if self.scalar_static_bool[12]{(self.scalar_static_f64[56]*v756)}else{v11})});
        let v1603=(if v391{v1591}else{(if v383{(v386*v1591)}else{v11})});
        let v1604=(if v391{v1592}else{(if v383{(v386*v1592)}else{v11})});
        let v1605=(if v391{v1593}else{(if v383{(v386*v1593)}else{v11})});
        let v1606=(if v391{v1594}else{(if v383{(v386*v1594)}else{v11})});
        let v1613=(if self.scalar_static_bool[19]{v11}else{(if self.scalar_static_bool[18]{v11}else{v1203})});
        let v1615=(if self.scalar_static_bool[19]{v11}else{(if self.scalar_static_bool[18]{v11}else{v1205})});
        let v1618=(v42-(v413*v413));
        let v1623=(v42-(v415*v415));
        let v1633=scalar_limexp_derivative(v422);
        let v1641=(self.scalar_static_f64[60]*(-(if self.scalar_static_bool[19]{v11}else{(if self.scalar_static_bool[18]{v11}else{v1204})})));
        let v1646=scalar_limexp_derivative(v426);
        let v1660=(v42-(v435*v435));
        let v1666=(v42-(v441*v441));
        let v1667=(self.scalar_static_f64[63]*v1666);
        let v1668=(self.scalar_static_f64[109]*v1666);
        let v1671=(v42-(v447*v447));
        let v1672=(self.scalar_static_f64[110]*v1671);
        let v1673=(self.scalar_static_f64[65]*v1671);
        let v1677=(v42-(v453*v453));
        let v1707=(v480).sinh();
        let v1710=(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1707)}else{v11});
        let v1711=(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]*v1707)}else{v11});
        let v1720=(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1716)}else{v11});
        let v1721=(if self.scalar_static_bool[31]{(self.scalar_static_f64[108]*v1716)}else{v11});
        let v1760=(v501).sinh();
        let v1826=(-(if self.scalar_static_bool[17]{(v129*((if self.scalar_static_bool[17]{((v348*((v344*(v113*v1344))+(v343*(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1344)}else{v1134}))*v1403)}else{v1151}))))+(v345*(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1344)}else{v11}))))}else{v954})-(if self.scalar_static_bool[17]{((v355*((v352*(v113*v1376))+(v351*(-(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1376)}else{v11}))*v1420)}else{v11})))))+(v353*(-(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1376)}else{v11})))))}else{v1026})))}else{(if self.scalar_static_bool[14]{((v283*((v279*v1151)+(v275*(v113*v1126))))+(v280*(v822+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[53]*v1126)}else{v917})))))}else{(if self.scalar_static_bool[11]{(v129*(v954-v1026))}else{(if self.scalar_static_bool[6]{((v196*(v809+(v181*v803)))+(v190*v822))}else{v11})})})}));
        let v1829=ddt_scale;
        let v1873=(v377*v377);
        let v1905=(self.scalar_static_f64[76]*v1829);
        let v1926=(self.scalar_static_f64[77]*v1829);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * ((-(if self.scalar_static_bool[17]{(v129*((if self.scalar_static_bool[17]{(v345*v348)}else{v232})-(if self.scalar_static_bool[17]{(v353*v355)}else{v245})))}else{(if self.scalar_static_bool[14]{(v280*v283)}else{(if self.scalar_static_bool[11]{(v129*(v232-v245))}else{(if self.scalar_static_bool[6]{(v190*v196)}else{v11})})})}))),
            [3, 4, 5, 8],
            [(-v1541), v1826, (-v1543), (-(if self.scalar_static_bool[17]{(v129*((if self.scalar_static_bool[17]{((v348*((v344*(v113*v1346))+(v343*(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1346)}else{v1136}))*v1403)}else{v1153}))))+(v345*(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1346)}else{v11}))))}else{v956})-(if self.scalar_static_bool[17]{((v355*((v352*(v113*v1378))+(v351*(-(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[46]*v1378)}else{v11}))*v1420)}else{v11})))))+(v353*(-(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[53]*v1378)}else{v11})))))}else{v1028})))}else{(if self.scalar_static_bool[14]{((v283*((v279*v1153)+(v275*(v113*v1128))))+(v280*(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[53]*v1128)}else{v919}))))}else{(if self.scalar_static_bool[11]{(v129*(v956-v1028))}else{(if self.scalar_static_bool[6]{(v196*(v815+(v181*v805)))}else{v11})})})}))],
            [],
            [],
            multiplicity,
        );
        let v575_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_static_f64[85]*ctx.node_voltage(nodes[12])));
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v575_ddt),
            12,
            multiplicity * (((self.scalar_static_f64[85]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v10),
            13,
            multiplicity * (v42),
        );
        let v578_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_static_f64[86]*ctx.branch_current(branches[0])));
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v578_ddt,
            0,
            ((self.scalar_static_f64[86]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (v10),
            13,
            multiplicity * (v42),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v425),
            [3, 4, 5, 8],
            [(self.scalar_static_f64[60]*(-v1613)), v1641, (self.scalar_static_f64[60]*(((v136*(if self.scalar_static_bool[23]{v396}else{(if self.scalar_static_bool[21]{(-v1618)}else{self.scalar_static_f64[101]})}))*v1633)-v1615)), (self.scalar_static_f64[60]*(((v136*(if self.scalar_static_bool[23]{v42}else{(if self.scalar_static_bool[21]{v1618}else{self.scalar_static_f64[102]})}))*v1633)-self.scalar_static_f64[103]))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * ((self.scalar_static_f64[60]*(scalar_limexp(v426)-v410))),
            [3, 4, 5, 7, 8],
            [(self.scalar_static_f64[60]*(((v136*(if self.scalar_static_bool[23]{v396}else{(if self.scalar_static_bool[21]{(-v1623)}else{self.scalar_static_f64[101]})}))*v1646)-v1613)), v1641, (self.scalar_static_f64[60]*(-v1615)), (self.scalar_static_f64[60]*((v136*(if self.scalar_static_bool[23]{v42}else{(if self.scalar_static_bool[21]{v1623}else{self.scalar_static_f64[102]})}))*v1646)), self.scalar_static_f64[105]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[26]{v579}else{v11})),
            [3, 5, 7, 8],
            [(if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{((v117*(self.scalar_static_f64[113]+(((v513*v1672)+(v449*((self.scalar_static_f64[112]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[112]*v1769)}else{v1720})/v507)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[107]*v1760)}else{v1710})/v503)}else{v11}))}else{v11}))))/self.scalar_static_f64[32])))+self.scalar_static_f64[115])}else{v11})*v1829)}else{v11}), (if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v117*(((v513*v1673)+(v449*((self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1769)}else{v1721})/v507)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1760)}else{v1711})/v503)}else{v11}))}else{v11}))))/self.scalar_static_f64[32]))}else{v11})*v1829)}else{v11}), (if self.scalar_static_bool[26]{(v1817*v1829)}else{v11}), (if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v117*((v449*(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{v11}else{v1722})/v507)}else{v11}))/self.scalar_static_f64[32]))}else{v11})*v1829)}else{v11})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[26]{v581}else{v11})),
            3,
            multiplicity * ((if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v116*(((v492*v1667)+(v479*((self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{(v1720/v486)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{(v1710/v482)}else{v11}))}else{v11}))))/self.scalar_static_f64[29]))}else{v11})*v1829)}else{v11})),
            5,
            multiplicity * ((if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{((v116*((((v492*v1668)+(v479*((self.scalar_static_f64[108]+(if self.scalar_static_bool[31]{(v1721/v486)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]+(if self.scalar_static_bool[31]{(v1711/v482)}else{v11}))}else{v11}))))/self.scalar_static_f64[29])+self.scalar_static_f64[113]))+self.scalar_static_f64[114])}else{v11})*v1829)}else{v11})),
            8,
            multiplicity * ((if self.scalar_static_bool[26]{(v1759*v1829)}else{v11})),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[49]{v585}else{v11})),
            3,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*((-v525)+(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*((v454*v1672)+(v449*(self.scalar_static_f64[112]*v1677))))}else{v11})}))))}else{v11})),
            5,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*((v454*v1673)+(v449*(self.scalar_static_f64[61]*v1677))))}else{v11})})))}else{v11})),
            7,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v525+(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*(v449*(self.scalar_static_f64[32]*v1677)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[49]{v588}else{v11})),
            3,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{((v465*v1667)+(v442*(v116*(self.scalar_static_f64[61]*v1660))))}else{v11})})))}else{v11})),
            5,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*((-v523)+(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{((v465*v1668)+(v442*(v116*(self.scalar_static_f64[108]*v1660))))}else{v11})}))))}else{v11})),
            8,
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v523+(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v442*(v116*(self.scalar_static_f64[29]*v1660)))}else{v11})}))))}else{v11})),
        );
        let v593_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (self.scalar_static_f64[87]*(ctx.node_voltage(nodes[1])-v4)));
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v593_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[87]) * ddt_scale)),
            3,
            multiplicity * (((self.scalar_static_f64[116]) * ddt_scale)),
        );
        let v595_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (v7*self.scalar_static_f64[88]));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (v595_ddt),
            3,
            multiplicity * (((self.scalar_static_f64[88]) * ddt_scale)),
            5,
            multiplicity * (((self.scalar_static_f64[117]) * ddt_scale)),
        );
        let v598_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (v119*(v4-v596)));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (v598_ddt),
            3,
            multiplicity * (((v119) * ddt_scale)),
            10,
            multiplicity * ((((-v119)) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[32]{(v599/v377)}else{v11})),
            [3, 4, 5, 8, 10],
            [(if self.scalar_static_bool[32]{((-(v599*(if self.scalar_static_bool[13]{((-(v118*v1343))/v1572)}else{(if self.scalar_static_bool[12]{((-(v118*v753))/v1547)}else{v11})})))/v1873)}else{v11}), (if self.scalar_static_bool[32]{((-(v599*(if self.scalar_static_bool[13]{((-(v118*v1344))/v1572)}else{(if self.scalar_static_bool[12]{((-(v118*v754))/v1547)}else{v11})})))/v1873)}else{v11}), (if self.scalar_static_bool[32]{(((-v377)-(v599*(if self.scalar_static_bool[13]{((-(v118*v1345))/v1572)}else{(if self.scalar_static_bool[12]{((-(v118*v755))/v1547)}else{v11})})))/v1873)}else{v11}), (if self.scalar_static_bool[32]{((-(v599*(if self.scalar_static_bool[13]{((-(v118*v1346))/v1572)}else{(if self.scalar_static_bool[12]{((-(v118*v756))/v1547)}else{v11})})))/v1873)}else{v11}), (if self.scalar_static_bool[32]{(v42/v377)}else{v11})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v11,
        );
        let v605_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (self.scalar_static_f64[89]*(v603-v0)));
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v605_ddt),
            8,
            multiplicity * (((self.scalar_static_f64[118]) * ddt_scale)),
            9,
            multiplicity * (((self.scalar_static_f64[89]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[33]{((v603-v1)/self.scalar_static_f64[72])}else{v11})),
            5,
            multiplicity * (self.scalar_static_f64[121]),
            9,
            multiplicity * (self.scalar_static_f64[122]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v11,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[34]{((v3-v8)/self.scalar_static_f64[73])}else{v11})),
            4,
            multiplicity * (self.scalar_static_f64[125]),
            7,
            multiplicity * (self.scalar_static_f64[126]),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (v11),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v11,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[35]{((v3-v0)/self.scalar_static_f64[74])}else{v11})),
            4,
            multiplicity * (self.scalar_static_f64[129]),
            8,
            multiplicity * (self.scalar_static_f64[130]),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            5,
            (if self.scalar_static_bool[36]{(self.scalar_static_f64[75]*ctx.branch_current(branches[5]))}else{v11}),
            5,
            self.scalar_static_f64[131],
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            6,
            (if self.scalar_static_bool[36]{v620}else{v11}),
            6,
            (if self.scalar_static_bool[36]{v1905}else{v11}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            8,
            (if self.scalar_static_bool[51]{v626}else{v11}),
            8,
            (if self.scalar_static_bool[51]{v1905}else{v11}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            10,
            (if self.scalar_static_bool[38]{(v393*v628)}else{v11}),
            [3, 4, 5, 8],
            [(if self.scalar_static_bool[38]{(v628*v1603)}else{v11}), (if self.scalar_static_bool[38]{(v628*v1604)}else{v11}), (if self.scalar_static_bool[38]{(v628*v1605)}else{v11}), (if self.scalar_static_bool[38]{(v628*v1606)}else{v11})],
            [10],
            [(if self.scalar_static_bool[38]{v393}else{v11})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            v11,
        );
        let v633_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, (self.scalar_static_f64[90]*ctx.branch_current(branches[13])));
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            13,
            v633_ddt,
            13,
            ((self.scalar_static_f64[90]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            14,
            (if self.scalar_static_bool[41]{(v392*v634)}else{v11}),
            [3, 4, 5, 8],
            [(if self.scalar_static_bool[41]{(v634*v1603)}else{v11}), (if self.scalar_static_bool[41]{(v634*v1604)}else{v11}), (if self.scalar_static_bool[41]{(v634*v1605)}else{v11}), (if self.scalar_static_bool[41]{(v634*v1606)}else{v11})],
            [14],
            [(if self.scalar_static_bool[41]{v392}else{v11})],
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            15,
            (if self.scalar_static_bool[41]{v639}else{v11}),
            15,
            (if self.scalar_static_bool[41]{v1926}else{v11}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            17,
            (if self.scalar_static_bool[53]{v645}else{v11}),
            17,
            (if self.scalar_static_bool[53]{v1926}else{v11}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            v11,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (v11),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v648),
            14,
            multiplicity * (self.scalar_static_f64[132]),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (v11),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * ((if self.scalar_static_bool[47]{v649}else{v11})),
            15,
            multiplicity * (self.scalar_static_f64[132]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v648),
            14,
            multiplicity * (self.scalar_static_f64[132]),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[47]{((v568*v647)+(v564*v649))}else{v11})),
            14,
            multiplicity * ((if self.scalar_static_bool[47]{v568}else{v11})),
            15,
            multiplicity * ((if self.scalar_static_bool[47]{v564}else{v11})),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[47]{v657}else{v11})),
            14,
            multiplicity * ((if self.scalar_static_bool[47]{(v655*v1829)}else{v11})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v647),
            14,
            multiplicity * (v42),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v649),
            15,
            multiplicity * (v42),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (v11),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if self.scalar_static_bool[48]{v661}else{v11})),
            11,
            multiplicity * ((if self.scalar_static_bool[48]{(self.scalar_static_f64[91]*v1829)}else{v11})),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * ((if self.scalar_static_bool[48]{(-(((v7*(-v10))+(v2*v425))).abs())}else{v11})),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if self.scalar_static_bool[48]{(v30/self.scalar_static_f64[11])}else{v11})),
            11,
            multiplicity * (self.scalar_static_f64[134]),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if self.scalar_static_bool[54]{(v30*1e-12)}else{v11})),
            11,
            multiplicity * (self.scalar_static_f64[135]),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v0=ctx.node_voltage(nodes[8]);
        let v1=ctx.node_voltage(nodes[5]);
        let v2=(v0-v1);
        let v4=ctx.node_voltage(nodes[3]);
        let v7=(v4-v1);
        let v9=(ctx.node_voltage(nodes[7])-v4);
        let v11=0.0;
        let v30=ctx.node_voltage(nodes[11]);
        let v33=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[137]+(v30).abs())}else{self.scalar_static_f64[137]});
        let v37=((v33-self.scalar_static_f64[9])).abs();
        let v41=((v37>v11)||self.scalar_static_bool[2]);
        let v42=1.0;
        let v112=(!v41);
        let v116=(if v112{self.scalar_static_f64[18]}else{(if v41{(self.scalar_static_f64[18]*(v42+(v37*self.scalar_static_f64[19])))}else{v11})});
        let v117=(if v112{self.scalar_static_f64[20]}else{(if v41{(self.scalar_static_f64[20]*(v42+(v37*self.scalar_static_f64[21])))}else{v11})});
        let v119=(if v112{self.scalar_static_f64[24]}else{(if v41{(self.scalar_static_f64[24]*(v42+(v37*self.scalar_static_f64[25])))}else{v11})});
        let v121=(if v112{self.scalar_static_f64[28]}else{(if v41{(self.scalar_static_f64[28]+(v37*self.scalar_static_f64[30]))}else{v11})});
        let v122=(if v112{self.scalar_static_f64[31]}else{(if v41{(self.scalar_static_f64[31]+(v37*self.scalar_static_f64[33]))}else{v11})});
        let v433=(v7*self.scalar_static_f64[61]);
        let v434=((v121+(v2*self.scalar_static_f64[29]))+v433);
        let v435=(v434).tanh();
        let v441=((self.scalar_static_f64[62]+(v7*self.scalar_static_f64[63]))).tanh();
        let v442=(v42+v441);
        let v447=((self.scalar_static_f64[64]-(v7*self.scalar_static_f64[65]))).tanh();
        let v449=((v42+v447)-self.scalar_static_f64[61]);
        let v452=((v122+(v9*self.scalar_static_f64[32]))-v433);
        let v453=(v452).tanh();
        let v454=(v42+v453);
        let v465=(v116*(v42+v435));
        let v479=(if self.scalar_static_bool[31]{(v442-self.scalar_static_f64[61])}else{v442});
        let v480=(v121+v433);
        let v482=(if self.scalar_static_bool[31]{(v480).cosh()}else{v11});
        let v486=(if self.scalar_static_bool[31]{(v434).cosh()}else{v11});
        let v492=((v434+(if self.scalar_static_bool[31]{(v486).ln()}else{v11}))-(if self.scalar_static_bool[31]{(v480+(if self.scalar_static_bool[31]{(v482).ln()}else{v11}))}else{v11}));
        let v501=(v122-v433);
        let v503=(if self.scalar_static_bool[31]{(v501).cosh()}else{v482});
        let v507=(if self.scalar_static_bool[31]{(v452).cosh()}else{v486});
        let v513=((v452+(if self.scalar_static_bool[31]{(v507).ln()}else{v11}))-(if self.scalar_static_bool[31]{(v501+(if self.scalar_static_bool[31]{(v503).ln()}else{v11}))}else{v11}));
        let v1716=(v434).sinh();
        let v1722=(if self.scalar_static_bool[31]{(self.scalar_static_f64[29]*v1716)}else{v11});
        let v1759=(if self.scalar_static_bool[31]{(self.scalar_static_f64[67]+(v116*(self.scalar_static_f64[71]+((v479*(self.scalar_static_f64[29]+(if self.scalar_static_bool[31]{(v1722/v486)}else{v11})))/self.scalar_static_f64[29]))))}else{v11});
        let v522=v1759;
        let v523=(if self.scalar_static_bool[31]{v522}else{(if self.scalar_static_bool[28]{(self.scalar_static_f64[67]+(v442*v465))}else{self.scalar_static_f64[68]})});
        let v1769=(v452).sinh();
        let v1817=(if self.scalar_static_bool[31]{(self.scalar_static_f64[69]+(v117*(self.scalar_static_f64[71]+((v449*(self.scalar_static_f64[32]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[32]*v1769)}else{v11})/v507)}else{v11})))/self.scalar_static_f64[32]))))}else{v11});
        let v524=v1817;
        let v525=(if self.scalar_static_bool[31]{v524}else{(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]+(v117*((v449*v454)+self.scalar_static_f64[71])))}else{self.scalar_static_f64[70]})});
        let v579=0.0;
        let v581=0.0;
        let v585=0.0;
        let v588=0.0;
        let v620=0.0;
        let v626=0.0;
        let v639=0.0;
        let v645=0.0;
        let v655=(-(if self.scalar_static_bool[47]{((if self.scalar_static_bool[47]{((v116*((v33*5.5226012e-23)*self.scalar_static_f64[82]))*self.scalar_static_f64[84])}else{v11})*3.141592653589793)}else{v11}));
        let v657=0.0;
        let v661=0.0;
        let v1660=(v42-(v435*v435));
        let v1666=(v42-(v441*v441));
        let v1667=(self.scalar_static_f64[63]*v1666);
        let v1668=(self.scalar_static_f64[109]*v1666);
        let v1671=(v42-(v447*v447));
        let v1672=(self.scalar_static_f64[110]*v1671);
        let v1673=(self.scalar_static_f64[65]*v1671);
        let v1677=(v42-(v453*v453));
        let v1707=(v480).sinh();
        let v1710=(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1707)}else{v11});
        let v1711=(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]*v1707)}else{v11});
        let v1720=(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1716)}else{v11});
        let v1721=(if self.scalar_static_bool[31]{(self.scalar_static_f64[108]*v1716)}else{v11});
        let v1760=(v501).sinh();
        let v1829=1.0;
        let v1905=(self.scalar_static_f64[76]*v1829);
        let v1926=(self.scalar_static_f64[77]*v1829);

        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (self.scalar_static_f64[85]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[12]),
            Some(nodes[13]),
            branches[0],
            multiplicity * (self.scalar_static_f64[86]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &[nodes[3], nodes[5], nodes[7], nodes[8]],
            &[(if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{((v117*(self.scalar_static_f64[113]+(((v513*v1672)+(v449*((self.scalar_static_f64[112]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[112]*v1769)}else{v1720})/v507)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[107]*v1760)}else{v1710})/v503)}else{v11}))}else{v11}))))/self.scalar_static_f64[32])))+self.scalar_static_f64[115])}else{v11})*v1829)}else{v11}), (if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v117*(((v513*v1673)+(v449*((self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1769)}else{v1721})/v507)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{(self.scalar_static_f64[61]*v1760)}else{v1711})/v503)}else{v11}))}else{v11}))))/self.scalar_static_f64[32]))}else{v11})*v1829)}else{v11}), (if self.scalar_static_bool[26]{(v1817*v1829)}else{v11}), (if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v117*((v449*(if self.scalar_static_bool[31]{((if self.scalar_static_bool[31]{v11}else{v1722})/v507)}else{v11}))/self.scalar_static_f64[32]))}else{v11})*v1829)}else{v11})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{(v116*(((v492*v1667)+(v479*((self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{(v1720/v486)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[61]+(if self.scalar_static_bool[31]{(v1710/v482)}else{v11}))}else{v11}))))/self.scalar_static_f64[29]))}else{v11})*v1829)}else{v11})),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[26]{((if self.scalar_static_bool[31]{((v116*((((v492*v1668)+(v479*((self.scalar_static_f64[108]+(if self.scalar_static_bool[31]{(v1721/v486)}else{v11}))-(if self.scalar_static_bool[31]{(self.scalar_static_f64[107]+(if self.scalar_static_bool[31]{(v1711/v482)}else{v11}))}else{v11}))))/self.scalar_static_f64[29])+self.scalar_static_f64[113]))+self.scalar_static_f64[114])}else{v11})*v1829)}else{v11})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[26]{(v1759*v1829)}else{v11})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*((-v525)+(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*((v454*v1672)+(v449*(self.scalar_static_f64[112]*v1677))))}else{v11})}))))}else{v11})),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*((v454*v1673)+(v449*(self.scalar_static_f64[61]*v1677))))}else{v11})})))}else{v11})),
            nodes[7],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v525+(v9*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v117*(v449*(self.scalar_static_f64[32]*v1677)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{((v465*v1667)+(v442*(v116*(self.scalar_static_f64[61]*v1660))))}else{v11})})))}else{v11})),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*((-v523)+(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{((v465*v1668)+(v442*(v116*(self.scalar_static_f64[108]*v1660))))}else{v11})}))))}else{v11})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[49]{(v1829*(v523+(v2*(if self.scalar_static_bool[31]{v11}else{(if self.scalar_static_bool[28]{(v442*(v116*(self.scalar_static_f64[29]*v1660)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[87]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[116]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[88]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[117]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (v119),
            nodes[10],
            multiplicity * ((-v119)),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (self.scalar_static_f64[118]),
            nodes[9],
            multiplicity * (self.scalar_static_f64[89]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[6],
            multiplicity * ((if self.scalar_static_bool[36]{v1905}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[8],
            multiplicity * ((if self.scalar_static_bool[51]{v1905}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[2]),
            branches[13],
            multiplicity * (self.scalar_static_f64[90]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[15],
            multiplicity * ((if self.scalar_static_bool[41]{v1926}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[17],
            multiplicity * ((if self.scalar_static_bool[53]{v1926}else{v11})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[14],
            multiplicity * ((if self.scalar_static_bool[47]{(v655*v1829)}else{v11})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * ((if self.scalar_static_bool[48]{(self.scalar_static_f64[91]*v1829)}else{v11})),
        );
    }
}
