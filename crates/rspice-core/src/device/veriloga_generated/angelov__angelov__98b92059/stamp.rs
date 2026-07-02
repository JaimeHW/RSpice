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
        let v33=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[160]+(v30).abs())}else{self.scalar_static_f64[160]});
        let v37=((v33-self.scalar_static_f64[9])).abs();
        let v42=1.0;
        let v43=(if ((v37>v11)||self.scalar_static_bool[2]){v42}else{v11});
        let v113=(!(v43!=0.0));
        let v114=(if v113{self.scalar_static_f64[12]}else{(if (v43!=0.0){(self.scalar_static_f64[12]*(v42+(v37*self.scalar_static_f64[13])))}else{v11})});
        let v115=(if v113{self.scalar_static_f64[14]}else{(if (v43!=0.0){(self.scalar_static_f64[14]*(v42+(v37*self.scalar_static_f64[15])))}else{v11})});
        let v116=(if v113{self.scalar_static_f64[16]}else{(if (v43!=0.0){(self.scalar_static_f64[16]*(v42+(v37*self.scalar_static_f64[17])))}else{v11})});
        let v117=(if v113{self.scalar_static_f64[18]}else{(if (v43!=0.0){(self.scalar_static_f64[18]*(v42+(v37*self.scalar_static_f64[19])))}else{v11})});
        let v118=(if v113{self.scalar_static_f64[20]}else{(if (v43!=0.0){(self.scalar_static_f64[20]*(v42+(v37*self.scalar_static_f64[21])))}else{v11})});
        let v119=(if v113{self.scalar_static_f64[22]}else{(if (v43!=0.0){(self.scalar_static_f64[22]*(v42+(v37*self.scalar_static_f64[23])))}else{v11})});
        let v120=(if v113{self.scalar_static_f64[24]}else{(if (v43!=0.0){(self.scalar_static_f64[24]*(v42+(v37*self.scalar_static_f64[25])))}else{v11})});
        let v122=(if v113{self.scalar_static_f64[28]}else{(if (v43!=0.0){(self.scalar_static_f64[28]+(v37*self.scalar_static_f64[30]))}else{v11})});
        let v123=(if v113{self.scalar_static_f64[31]}else{(if (v43!=0.0){(self.scalar_static_f64[31]+(v37*self.scalar_static_f64[33]))}else{v11})});
        let v124=(if v113{self.scalar_static_f64[34]}else{(if (v43!=0.0){(self.scalar_static_f64[34]+(v37*self.scalar_static_f64[35]))}else{v11})});
        let v125=(if v113{self.scalar_static_f64[36]}else{(if (v43!=0.0){(self.scalar_static_f64[36]+(v37*self.scalar_static_f64[37]))}else{v11})});
        let v131=0.5;
        let v138=(if self.scalar_static_bool[5]{self.scalar_static_f64[43]}else{(if (self.scalar_static_f64[40]!=0.0){(self.scalar_static_f64[42]/(v33*8.617333262145179e-5))}else{v11})});
        let v140=(v7*self.scalar_static_f64[44]);
        let v141=(v140).cosh();
        let v143=(v141*v141);
        let v146=(v115*(v42+(self.scalar_static_f64[45]/v143)));
        let v151=((v7*self.scalar_static_f64[47])).tanh();
        let v156=(self.scalar_static_f64[48]*(v6-self.scalar_static_f64[36]));
        let v157=(v6-v125);
        let v159=((((if v113{self.scalar_static_f64[26]}else{(if (v43!=0.0){(self.scalar_static_f64[26]+(v37*self.scalar_static_f64[27]))}else{v11})})-self.scalar_static_f64[46])+(self.scalar_static_f64[46]*v151))-(v156*v157));
        let v160=(v2-v159);
        let v161=(v160*v160);
        let v167=(v160*self.scalar_static_f64[50]);
        let v169=(((v146*v160)+(v161*self.scalar_static_f64[49]))+(v161*v167));
        let v170=(v169).tanh();
        let v171=(v42+v170);
        let v173=(-v169);
        let v177=((v131*(scalar_limexp(v169)-scalar_limexp(v173)))).tanh();
        let v181=(self.scalar_static_f64[51]+(self.scalar_static_f64[47]*v171));
        let v183=((v7*v181)).tanh();
        let v195=(v114*v171);
        let v196=(v183*v195);
        let v201=(v116*scalar_limexp(v157));
        let v202=((v42+(v7*self.scalar_static_f64[57]))+v201);
        let v207=(v5-v159);
        let v208=(if self.scalar_static_bool[11]{v207}else{v141});
        let v210=(if self.scalar_static_bool[11]{(v208*v208)}else{v160});
        let v212=(if self.scalar_static_bool[11]{(v208*v210)}else{v161});
        let v218=(if self.scalar_static_bool[11]{(((v146*v208)+(self.scalar_static_f64[49]*v210))+(self.scalar_static_f64[50]*v212))}else{v11});
        let v219=(v218).tanh();
        let v221=(if self.scalar_static_bool[11]{(v42+v219)}else{v11});
        let v224=(if self.scalar_static_bool[11]{(self.scalar_static_f64[51]+(self.scalar_static_f64[47]*v221))}else{v11});
        let v228=(if self.scalar_static_bool[11]{(self.scalar_static_f64[57]+(v171*self.scalar_static_f64[58]))}else{v11});
        let v229=(v42+v183);
        let v230=(v195*v229);
        let v233=(v7-v125);
        let v235=(v116*scalar_limexp(v233));
        let v236=((v42+(v7*v228))+v235);
        let v238=(if self.scalar_static_bool[11]{(v230*v236)}else{v11});
        let v241=(if self.scalar_static_bool[11]{(self.scalar_static_f64[57]+(v221*self.scalar_static_f64[58]))}else{v11});
        let v243=((v7*v224)).tanh();
        let v245=(v114*v221);
        let v246=(v42-(if self.scalar_static_bool[11]{v243}else{v11}));
        let v247=(v245*v246);
        let v249=(v42-(v7*v241));
        let v251=(if self.scalar_static_bool[11]{(v247*v249)}else{v11});
        let v258=(if self.scalar_static_bool[14]{v160}else{v208});
        let v260=(if self.scalar_static_bool[14]{(v258*v258)}else{v210});
        let v263=(self.scalar_static_f64[50]*v260);
        let v265=((v258+(self.scalar_static_f64[49]*v260))+(v258*v263));
        let v267=(if self.scalar_static_bool[14]{(v146*v265)}else{v169});
        let v269=(-v267);
        let v273=((v131*(scalar_limexp(v267)-scalar_limexp(v269)))).tanh();
        let v275=(if self.scalar_static_bool[14]{(v42+v273)}else{(v42+v177)});
        let v278=(if self.scalar_static_bool[14]{(self.scalar_static_f64[51]+(self.scalar_static_f64[47]*v275))}else{v11});
        let v280=((v7*v278)).tanh();
        let v281=(if self.scalar_static_bool[14]{v280}else{v11});
        let v284=(if self.scalar_static_bool[14]{(self.scalar_static_f64[57]+(self.scalar_static_f64[58]*v275))}else{v228});
        let v285=(v114*v275);
        let v286=(v281*v285);
        let v289=(v201+(v42+(v7*v284)));
        let v295=(if self.scalar_static_bool[17]{v160}else{v258});
        let v297=(if self.scalar_static_bool[17]{(v295*v295)}else{v260});
        let v300=(self.scalar_static_f64[50]*v297);
        let v302=((v295+(self.scalar_static_f64[49]*v297))+(v295*v300));
        let v304=(if self.scalar_static_bool[17]{(v146*v302)}else{v267});
        let v305=(if self.scalar_static_bool[17]{v207}else{v212});
        let v307=(if self.scalar_static_bool[17]{(v305*v305)}else{v11});
        let v310=(self.scalar_static_f64[50]*v305);
        let v312=((v305+(self.scalar_static_f64[49]*v307))+(v307*v310));
        let v314=(if self.scalar_static_bool[17]{(v146*v312)}else{v218});
        let v316=(-v304);
        let v320=((v131*(scalar_limexp(v304)-scalar_limexp(v316)))).tanh();
        let v322=(if self.scalar_static_bool[17]{(v42+v320)}else{v275});
        let v324=(-v314);
        let v328=((v131*(scalar_limexp(v314)-scalar_limexp(v324)))).tanh();
        let v330=(if self.scalar_static_bool[17]{(v42+v328)}else{v11});
        let v333=(if self.scalar_static_bool[17]{(self.scalar_static_f64[51]+(self.scalar_static_f64[47]*v322))}else{v278});
        let v336=(if self.scalar_static_bool[17]{(self.scalar_static_f64[51]+(self.scalar_static_f64[47]*v330))}else{v11});
        let v338=((v7*v333)).tanh();
        let v341=((v7*v336)).tanh();
        let v345=(if self.scalar_static_bool[17]{(self.scalar_static_f64[57]+(self.scalar_static_f64[58]*v330))}else{v11});
        let v348=(if self.scalar_static_bool[17]{(self.scalar_static_f64[57]+(self.scalar_static_f64[58]*v322))}else{v11});
        let v349=(v114*v322);
        let v350=(v42+(if self.scalar_static_bool[17]{v338}else{v281}));
        let v351=(v349*v350);
        let v354=(v235+(v42+(v7*v348)));
        let v357=(v114*v330);
        let v358=(v42-(if self.scalar_static_bool[17]{v341}else{v11}));
        let v359=(v357*v358);
        let v361=(v42-(v7*v345));
        let v370=(v42+v171);
        let v376=(v171*self.scalar_static_f64[62]);
        let v383=(v42+v322);
        let v386=(if self.scalar_static_bool[19]{(self.scalar_static_f64[60]+(v119/v383))}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[60]+(v119/v370))}else{v11})});
        let v387=(v322*self.scalar_static_f64[62]);
        let v389=(if self.scalar_static_bool[19]{(self.scalar_static_f64[61]+v387)}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[61]+v376)}else{v11})});
        let v391=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]+v387)}else{(if (self.scalar_static_f64[59]!=0.0){(v376+self.scalar_static_f64[63])}else{v11})});
        let v393=(if ((v37!=0.0)||self.scalar_static_bool[2]){v42}else{v11});
        let v396=(v42+(v37*self.scalar_static_f64[64]));
        let v401=(!(v393!=0.0));
        let v402=(if v401{v389}else{(if (v393!=0.0){(v389*v396)}else{v11})});
        let v403=(if v401{v391}else{(if (v393!=0.0){(v391*v396)}else{v11})});
        let v407=-1.0;
        let v413=(v2-v124);
        let v415=(v9-v124);
        let v421=(if self.scalar_static_bool[21]{scalar_limexp((v124*(-v138)))}else{(if (self.scalar_static_f64[66]!=0.0){scalar_limexp((v138*((-v124)).tanh()))}else{v295})});
        let v425=(v413).tanh();
        let v427=(v415).tanh();
        let v434=(v138*(if self.scalar_static_bool[25]{v413}else{(if self.scalar_static_bool[23]{v425}else{(if (self.scalar_static_f64[66]!=0.0){v413}else{v11})})}));
        let v437=(self.scalar_static_f64[68]*(scalar_limexp(v434)-v421));
        let v438=(v138*(if self.scalar_static_bool[25]{v415}else{(if self.scalar_static_bool[23]{v427}else{(if (self.scalar_static_f64[66]!=0.0){v415}else{v11})})}));
        let v445=(v7*self.scalar_static_f64[69]);
        let v446=((v122+(v2*self.scalar_static_f64[29]))+v445);
        let v447=(v446).tanh();
        let v453=((self.scalar_static_f64[70]+(v7*self.scalar_static_f64[71]))).tanh();
        let v454=(v42+v453);
        let v459=((self.scalar_static_f64[72]-(v7*self.scalar_static_f64[73]))).tanh();
        let v461=((v42+v459)-self.scalar_static_f64[69]);
        let v464=((v123+(v9*self.scalar_static_f64[32]))-v445);
        let v465=(v464).tanh();
        let v466=(v42+v465);
        let v480=(v117*(v42+v447));
        let v494=(if self.scalar_static_bool[33]{(v454-self.scalar_static_f64[69])}else{v454});
        let v495=(v122+v445);
        let v497=(if self.scalar_static_bool[33]{(v495).cosh()}else{v11});
        let v501=(if self.scalar_static_bool[33]{(v446).cosh()}else{v11});
        let v507=((v446+(if self.scalar_static_bool[33]{(v501).ln()}else{v11}))-(if self.scalar_static_bool[33]{(v495+(if self.scalar_static_bool[33]{(v497).ln()}else{v11}))}else{v11}));
        let v516=(v123-v445);
        let v518=(if self.scalar_static_bool[33]{(v516).cosh()}else{v497});
        let v522=(if self.scalar_static_bool[33]{(v464).cosh()}else{v501});
        let v528=((v464+(if self.scalar_static_bool[33]{(v522).ln()}else{v11}))-(if self.scalar_static_bool[33]{(v516+(if self.scalar_static_bool[33]{(v518).ln()}else{v11}))}else{v11}));
        let v1743=(v446).sinh();
        let v1749=(if self.scalar_static_bool[33]{(self.scalar_static_f64[29]*v1743)}else{v11});
        let v1786=(if self.scalar_static_bool[33]{(self.scalar_static_f64[78]+(v117*(self.scalar_static_f64[82]+((v494*(self.scalar_static_f64[29]+(if self.scalar_static_bool[33]{(v1749/v501)}else{v11})))/self.scalar_static_f64[29]))))}else{v11});
        let v537=v1786;
        let v538=(if self.scalar_static_bool[33]{v537}else{(if self.scalar_static_bool[30]{(self.scalar_static_f64[78]+(v454*v480))}else{self.scalar_static_f64[79]})});
        let v1796=(v464).sinh();
        let v1844=(if self.scalar_static_bool[33]{(self.scalar_static_f64[80]+(v118*(self.scalar_static_f64[82]+((v461*(self.scalar_static_f64[32]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[32]*v1796)}else{v11})/v522)}else{v11})))/self.scalar_static_f64[32]))))}else{v11});
        let v539=v1844;
        let v540=(if self.scalar_static_bool[33]{v539}else{(if self.scalar_static_bool[30]{(self.scalar_static_f64[80]+(v118*((v461*v466)+self.scalar_static_f64[82])))}else{self.scalar_static_f64[81]})});
        let v586=(if self.scalar_static_bool[49]{((v117*((v33*5.5226012e-23)*self.scalar_static_f64[104]))*self.scalar_static_f64[106])}else{v11});
        let v590=(if self.scalar_static_bool[49]{((v42-(v586*v586))).sqrt()}else{v11});
        let v592=3.141592653589793;
        let v594=(if self.scalar_static_bool[49]{((-v586)*v592)}else{v11});
        let v606=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (if self.scalar_static_bool[33]{((v118*(((v461*v528)/self.scalar_static_f64[32])+(v9*self.scalar_static_f64[82])))+(v9*self.scalar_static_f64[80]))}else{v11}));
        let v608=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (if self.scalar_static_bool[33]{((v117*(((v494*v507)/self.scalar_static_f64[29])+(v2*self.scalar_static_f64[82])))+(v2*self.scalar_static_f64[78]))}else{v11}));
        let v612=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (v9*v540));
        let v615=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v2*v538));
        let v623=ctx.node_voltage(nodes[10]);
        let v626=(v623-v1);
        let v630=ctx.node_voltage(nodes[9]);
        let v647=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (self.scalar_static_f64[92]*ctx.branch_current(branches[6])));
        let v653=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, (self.scalar_static_f64[92]*ctx.branch_current(branches[8])));
        let v655=ctx.branch_current(branches[10]);
        let v661=ctx.branch_current(branches[14]);
        let v666=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, (self.scalar_static_f64[96]*ctx.branch_current(branches[15])));
        let v672=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, (self.scalar_static_f64[96]*ctx.branch_current(branches[17])));
        let v674=ctx.node_voltage(nodes[14]);
        let v675=(if self.scalar_static_bool[49]{v674}else{v11});
        let v676=ctx.node_voltage(nodes[15]);
        let v682=(-(if self.scalar_static_bool[49]{(v586*v592)}else{v11}));
        let v684=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (v674*v682));
        let v688=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (v30*self.scalar_static_f64[114]));
        let v704=(v140).sinh();
        let v705=(self.scalar_static_f64[44]*v704);
        let v706=(self.scalar_static_f64[115]*v704);
        let v707=(v141*v705);
        let v709=(v141*v706);
        let v713=(v143*v143);
        let v718=(v115*((-(self.scalar_static_f64[45]*(v707+v707)))/v713));
        let v719=(v115*((-(self.scalar_static_f64[45]*(v709+v709)))/v713));
        let v722=(v42-(v151*v151));
        let v726=(self.scalar_static_f64[46]*(self.scalar_static_f64[116]*v722));
        let v732=((v157*self.scalar_static_f64[117])+(-v156));
        let v733=((self.scalar_static_f64[46]*(self.scalar_static_f64[47]*v722))-(v156+(self.scalar_static_f64[48]*v157)));
        let v735=(-v733);
        let v736=(v407-v726);
        let v737=(v160*v735);
        let v738=(v737+v737);
        let v739=(v160*v732);
        let v740=(v739+v739);
        let v741=(v160*v736);
        let v742=(v741+v741);
        let v743=(v160+v160);
        let v774=((((v160*v718)+(v146*v735))+(self.scalar_static_f64[49]*v738))+((v167*v738)+(v161*(self.scalar_static_f64[50]*v735))));
        let v775=(((v146*v732)+(self.scalar_static_f64[49]*v740))+((v167*v740)+(v161*(self.scalar_static_f64[50]*v732))));
        let v776=((((v160*v719)+(v146*v736))+(self.scalar_static_f64[49]*v742))+((v167*v742)+(v161*(self.scalar_static_f64[50]*v736))));
        let v777=((v146+(self.scalar_static_f64[49]*v743))+((v167*v743)+(v161*self.scalar_static_f64[50])));
        let v779=(v42-(v170*v170));
        let v780=(v774*v779);
        let v781=(v775*v779);
        let v782=(v776*v779);
        let v783=(v777*v779);
        let v784=scalar_limexp_derivative(v169);
        let v793=scalar_limexp_derivative(v173);
        let v807=(v42-(v177*v177));
        let v824=(v42-(v183*v183));
        let v829=(v114*v780);
        let v830=(v114*v781);
        let v831=(v114*v782);
        let v832=(v114*v783);
        let v833=(v195*((v181+(v7*(self.scalar_static_f64[47]*v780)))*v824));
        let v836=(v195*((v7*(self.scalar_static_f64[47]*v781))*v824));
        let v839=(v195*(((-v181)+(v7*(self.scalar_static_f64[47]*v782)))*v824));
        let v842=(v195*((v7*(self.scalar_static_f64[47]*v783))*v824));
        let v846=scalar_limexp_derivative(v157);
        let v848=(v116*v846);
        let v849=(v116*(-v846));
        let v865=(v407-v733);
        let v866=(v42-(-v732));
        let v867=(-v726);
        let v868=(if self.scalar_static_bool[11]{v865}else{v705});
        let v869=(if self.scalar_static_bool[11]{v866}else{v11});
        let v870=(if self.scalar_static_bool[11]{v867}else{v706});
        let v871=(v208*v868);
        let v873=(v208*v869);
        let v875=(v208*v870);
        let v877=(if self.scalar_static_bool[11]{(v871+v871)}else{v735});
        let v878=(if self.scalar_static_bool[11]{(v873+v873)}else{v732});
        let v879=(if self.scalar_static_bool[11]{(v875+v875)}else{v736});
        let v891=(if self.scalar_static_bool[11]{((v210*v868)+(v208*v877))}else{v738});
        let v892=(if self.scalar_static_bool[11]{((v210*v869)+(v208*v878))}else{v740});
        let v893=(if self.scalar_static_bool[11]{((v210*v870)+(v208*v879))}else{v742});
        let v894=(if self.scalar_static_bool[11]{(v208*self.scalar_static_f64[119])}else{v743});
        let v917=(if self.scalar_static_bool[11]{((((v208*v718)+(v146*v868))+(self.scalar_static_f64[49]*v877))+(self.scalar_static_f64[50]*v891))}else{v11});
        let v918=(if self.scalar_static_bool[11]{(((v146*v869)+(self.scalar_static_f64[49]*v878))+(self.scalar_static_f64[50]*v892))}else{v11});
        let v919=(if self.scalar_static_bool[11]{((((v208*v719)+(v146*v870))+(self.scalar_static_f64[49]*v879))+(self.scalar_static_f64[50]*v893))}else{v11});
        let v920=(if self.scalar_static_bool[11]{(self.scalar_static_f64[120]+(self.scalar_static_f64[50]*v894))}else{v11});
        let v922=(v42-(v219*v219));
        let v927=(if self.scalar_static_bool[11]{(v917*v922)}else{v11});
        let v928=(if self.scalar_static_bool[11]{(v918*v922)}else{v11});
        let v929=(if self.scalar_static_bool[11]{(v919*v922)}else{v11});
        let v930=(if self.scalar_static_bool[11]{(v920*v922)}else{v11});
        let v943=(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v780)}else{v11});
        let v944=(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v781)}else{v11});
        let v945=(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v782)}else{v11});
        let v946=(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v783)}else{v11});
        let v962=scalar_limexp_derivative(v233);
        let v964=(v116*v962);
        let v965=(v116*(-v962));
        let v980=(if self.scalar_static_bool[11]{((v236*(v833+(v229*v829)))+(v230*((v228+(v7*v943))+v964)))}else{v11});
        let v981=(if self.scalar_static_bool[11]{((v236*(v836+(v229*v830)))+(v230*(v7*v944)))}else{v11});
        let v982=(if self.scalar_static_bool[11]{((v236*(v839+(v229*v831)))+(v230*(((-v228)+(v7*v945))+v965)))}else{v11});
        let v983=(if self.scalar_static_bool[11]{((v236*(v842+(v229*v832)))+(v230*(v7*v946)))}else{v11});
        let v1000=(v42-(v243*v243));
        let v1052=(if self.scalar_static_bool[11]{((v249*((v246*(v114*v927))+(v245*(-(if self.scalar_static_bool[11]{((v224+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[47]*v927)}else{v11})))*v1000)}else{v11})))))+(v247*(-(v241+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v927)}else{v11}))))))}else{v11});
        let v1053=(if self.scalar_static_bool[11]{((v249*((v246*(v114*v928))+(v245*(-(if self.scalar_static_bool[11]{((v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[47]*v928)}else{v11}))*v1000)}else{v11})))))+(v247*(-(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v928)}else{v11})))))}else{v11});
        let v1054=(if self.scalar_static_bool[11]{((v249*((v246*(v114*v929))+(v245*(-(if self.scalar_static_bool[11]{(((-v224)+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[47]*v929)}else{v11})))*v1000)}else{v11})))))+(v247*(-((-v241)+(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v929)}else{v11}))))))}else{v11});
        let v1055=(if self.scalar_static_bool[11]{((v249*((v246*(v114*v930))+(v245*(-(if self.scalar_static_bool[11]{((v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[47]*v930)}else{v11}))*v1000)}else{v11})))))+(v247*(-(v7*(if self.scalar_static_bool[11]{(self.scalar_static_f64[58]*v930)}else{v11})))))}else{v11});
        let v1068=(if self.scalar_static_bool[14]{v735}else{v868});
        let v1069=(if self.scalar_static_bool[14]{v732}else{v869});
        let v1070=(if self.scalar_static_bool[14]{v736}else{v870});
        let v1072=(v258*v1068);
        let v1074=(v258*v1069);
        let v1076=(v258*v1070);
        let v1078=(v258*self.scalar_static_f64[121]);
        let v1080=(if self.scalar_static_bool[14]{(v1072+v1072)}else{v877});
        let v1081=(if self.scalar_static_bool[14]{(v1074+v1074)}else{v878});
        let v1082=(if self.scalar_static_bool[14]{(v1076+v1076)}else{v879});
        let v1083=(if self.scalar_static_bool[14]{(v1078+v1078)}else{self.scalar_static_f64[119]});
        let v1120=(if self.scalar_static_bool[14]{((v265*v718)+(v146*((v1068+(self.scalar_static_f64[49]*v1080))+((v263*v1068)+(v258*(self.scalar_static_f64[50]*v1080))))))}else{v774});
        let v1121=(if self.scalar_static_bool[14]{(v146*((v1069+(self.scalar_static_f64[49]*v1081))+((v263*v1069)+(v258*(self.scalar_static_f64[50]*v1081)))))}else{v775});
        let v1122=(if self.scalar_static_bool[14]{((v265*v719)+(v146*((v1070+(self.scalar_static_f64[49]*v1082))+((v263*v1070)+(v258*(self.scalar_static_f64[50]*v1082))))))}else{v776});
        let v1123=(if self.scalar_static_bool[14]{(v146*((self.scalar_static_f64[121]+(self.scalar_static_f64[49]*v1083))+((v263*self.scalar_static_f64[121])+(v258*(self.scalar_static_f64[50]*v1083)))))}else{v777});
        let v1124=scalar_limexp_derivative(v267);
        let v1133=scalar_limexp_derivative(v269);
        let v1147=(v42-(v273*v273));
        let v1152=(if self.scalar_static_bool[14]{((v131*((v1120*v1124)-((-v1120)*v1133)))*v1147)}else{((v131*((v774*v784)-((-v774)*v793)))*v807)});
        let v1153=(if self.scalar_static_bool[14]{((v131*((v1121*v1124)-((-v1121)*v1133)))*v1147)}else{((v131*((v775*v784)-((-v775)*v793)))*v807)});
        let v1154=(if self.scalar_static_bool[14]{((v131*((v1122*v1124)-((-v1122)*v1133)))*v1147)}else{((v131*((v776*v784)-((-v776)*v793)))*v807)});
        let v1155=(if self.scalar_static_bool[14]{((v131*((v1123*v1124)-((-v1123)*v1133)))*v1147)}else{((v131*((v777*v784)-((-v777)*v793)))*v807)});
        let v1160=(if self.scalar_static_bool[14]{(self.scalar_static_f64[47]*v1152)}else{v11});
        let v1161=(if self.scalar_static_bool[14]{(self.scalar_static_f64[47]*v1153)}else{v11});
        let v1162=(if self.scalar_static_bool[14]{(self.scalar_static_f64[47]*v1154)}else{v11});
        let v1163=(if self.scalar_static_bool[14]{(self.scalar_static_f64[47]*v1155)}else{v11});
        let v1172=(v42-(v280*v280));
        let v1177=(if self.scalar_static_bool[14]{((v278+(v7*v1160))*v1172)}else{v11});
        let v1178=(if self.scalar_static_bool[14]{((v7*v1161)*v1172)}else{v11});
        let v1179=(if self.scalar_static_bool[14]{(((-v278)+(v7*v1162))*v1172)}else{v11});
        let v1180=(if self.scalar_static_bool[14]{((v7*v1163)*v1172)}else{v11});
        let v1230=(if self.scalar_static_bool[17]{v735}else{v1068});
        let v1231=(if self.scalar_static_bool[17]{v732}else{v1069});
        let v1232=(if self.scalar_static_bool[17]{v736}else{v1070});
        let v1234=(v295*v1230);
        let v1236=(v295*v1231);
        let v1238=(v295*v1232);
        let v1240=(v295*self.scalar_static_f64[122]);
        let v1242=(if self.scalar_static_bool[17]{(v1234+v1234)}else{v1080});
        let v1243=(if self.scalar_static_bool[17]{(v1236+v1236)}else{v1081});
        let v1244=(if self.scalar_static_bool[17]{(v1238+v1238)}else{v1082});
        let v1245=(if self.scalar_static_bool[17]{(v1240+v1240)}else{v1083});
        let v1282=(if self.scalar_static_bool[17]{((v302*v718)+(v146*((v1230+(self.scalar_static_f64[49]*v1242))+((v300*v1230)+(v295*(self.scalar_static_f64[50]*v1242))))))}else{v1120});
        let v1283=(if self.scalar_static_bool[17]{(v146*((v1231+(self.scalar_static_f64[49]*v1243))+((v300*v1231)+(v295*(self.scalar_static_f64[50]*v1243)))))}else{v1121});
        let v1284=(if self.scalar_static_bool[17]{((v302*v719)+(v146*((v1232+(self.scalar_static_f64[49]*v1244))+((v300*v1232)+(v295*(self.scalar_static_f64[50]*v1244))))))}else{v1122});
        let v1285=(if self.scalar_static_bool[17]{(v146*((self.scalar_static_f64[122]+(self.scalar_static_f64[49]*v1245))+((v300*self.scalar_static_f64[122])+(v295*(self.scalar_static_f64[50]*v1245)))))}else{v1123});
        let v1286=(if self.scalar_static_bool[17]{v865}else{v891});
        let v1287=(if self.scalar_static_bool[17]{v866}else{v892});
        let v1288=(if self.scalar_static_bool[17]{v867}else{v893});
        let v1289=(if self.scalar_static_bool[17]{v11}else{v894});
        let v1290=(v305*v1286);
        let v1292=(v305*v1287);
        let v1294=(v305*v1288);
        let v1296=(v305*v1289);
        let v1298=(if self.scalar_static_bool[17]{(v1290+v1290)}else{v11});
        let v1299=(if self.scalar_static_bool[17]{(v1292+v1292)}else{v11});
        let v1300=(if self.scalar_static_bool[17]{(v1294+v1294)}else{v11});
        let v1301=(if self.scalar_static_bool[17]{(v1296+v1296)}else{v11});
        let v1338=(if self.scalar_static_bool[17]{((v312*v718)+(v146*((v1286+(self.scalar_static_f64[49]*v1298))+((v310*v1298)+(v307*(self.scalar_static_f64[50]*v1286))))))}else{v917});
        let v1339=(if self.scalar_static_bool[17]{(v146*((v1287+(self.scalar_static_f64[49]*v1299))+((v310*v1299)+(v307*(self.scalar_static_f64[50]*v1287)))))}else{v918});
        let v1340=(if self.scalar_static_bool[17]{((v312*v719)+(v146*((v1288+(self.scalar_static_f64[49]*v1300))+((v310*v1300)+(v307*(self.scalar_static_f64[50]*v1288))))))}else{v919});
        let v1341=(if self.scalar_static_bool[17]{(v146*((v1289+(self.scalar_static_f64[49]*v1301))+((v310*v1301)+(v307*(self.scalar_static_f64[50]*v1289)))))}else{v920});
        let v1342=scalar_limexp_derivative(v304);
        let v1351=scalar_limexp_derivative(v316);
        let v1365=(v42-(v320*v320));
        let v1370=(if self.scalar_static_bool[17]{((v131*((v1282*v1342)-((-v1282)*v1351)))*v1365)}else{v1152});
        let v1371=(if self.scalar_static_bool[17]{((v131*((v1283*v1342)-((-v1283)*v1351)))*v1365)}else{v1153});
        let v1372=(if self.scalar_static_bool[17]{((v131*((v1284*v1342)-((-v1284)*v1351)))*v1365)}else{v1154});
        let v1373=(if self.scalar_static_bool[17]{((v131*((v1285*v1342)-((-v1285)*v1351)))*v1365)}else{v1155});
        let v1374=scalar_limexp_derivative(v314);
        let v1383=scalar_limexp_derivative(v324);
        let v1397=(v42-(v328*v328));
        let v1402=(if self.scalar_static_bool[17]{((v131*((v1338*v1374)-((-v1338)*v1383)))*v1397)}else{v11});
        let v1403=(if self.scalar_static_bool[17]{((v131*((v1339*v1374)-((-v1339)*v1383)))*v1397)}else{v11});
        let v1404=(if self.scalar_static_bool[17]{((v131*((v1340*v1374)-((-v1340)*v1383)))*v1397)}else{v11});
        let v1405=(if self.scalar_static_bool[17]{((v131*((v1341*v1374)-((-v1341)*v1383)))*v1397)}else{v11});
        let v1430=(v42-(v338*v338));
        let v1447=(v42-(v341*v341));
        let v1568=(if self.scalar_static_bool[17]{(v131*((if self.scalar_static_bool[17]{((v354*((v350*(v114*v1370))+(v349*(if self.scalar_static_bool[17]{((v333+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1370)}else{v1160})))*v1430)}else{v1177}))))+(v351*(v964+(v348+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1370)}else{v11}))))))}else{v980})-(if self.scalar_static_bool[17]{((v361*((v358*(v114*v1402))+(v357*(-(if self.scalar_static_bool[17]{((v336+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1402)}else{v11})))*v1447)}else{v11})))))+(v359*(-(v345+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1402)}else{v11}))))))}else{v1052})))}else{(if self.scalar_static_bool[14]{((v289*((v285*v1177)+(v281*(v114*v1152))))+(v286*(v848+(v284+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[58]*v1152)}else{v943}))))))}else{(if self.scalar_static_bool[11]{(v131*(v980-v1052))}else{(if (self.scalar_static_f64[53]!=0.0){((v202*(v833+(v183*v829)))+(v196*(self.scalar_static_f64[57]+v848)))}else{v11})})})});
        let v1570=(if self.scalar_static_bool[17]{(v131*((if self.scalar_static_bool[17]{((v354*((v350*(v114*v1372))+(v349*(if self.scalar_static_bool[17]{(((-v333)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1372)}else{v1162})))*v1430)}else{v1179}))))+(v351*(v965+((-v348)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1372)}else{v11}))))))}else{v982})-(if self.scalar_static_bool[17]{((v361*((v358*(v114*v1404))+(v357*(-(if self.scalar_static_bool[17]{(((-v336)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1404)}else{v11})))*v1447)}else{v11})))))+(v359*(-((-v345)+(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1404)}else{v11}))))))}else{v1054})))}else{(if self.scalar_static_bool[14]{((v289*((v285*v1179)+(v281*(v114*v1154))))+(v286*((-v284)+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[58]*v1154)}else{v945})))))}else{(if self.scalar_static_bool[11]{(v131*(v982-v1054))}else{(if (self.scalar_static_f64[53]!=0.0){((v202*(v839+(v183*v831)))+(v196*self.scalar_static_f64[118]))}else{v11})})})});
        let v1574=(v370*v370);
        let v1599=(v383*v383);
        let v1618=(if self.scalar_static_bool[19]{(self.scalar_static_f64[62]*v1370)}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[62]*v780)}else{v11})});
        let v1619=(if self.scalar_static_bool[19]{(self.scalar_static_f64[62]*v1371)}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[62]*v781)}else{v11})});
        let v1620=(if self.scalar_static_bool[19]{(self.scalar_static_f64[62]*v1372)}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[62]*v782)}else{v11})});
        let v1621=(if self.scalar_static_bool[19]{(self.scalar_static_f64[62]*v1373)}else{(if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[62]*v783)}else{v11})});
        let v1630=(if v401{v1618}else{(if (v393!=0.0){(v396*v1618)}else{v11})});
        let v1631=(if v401{v1619}else{(if (v393!=0.0){(v396*v1619)}else{v11})});
        let v1632=(if v401{v1620}else{(if (v393!=0.0){(v396*v1620)}else{v11})});
        let v1633=(if v401{v1621}else{(if (v393!=0.0){(v396*v1621)}else{v11})});
        let v1640=(if self.scalar_static_bool[21]{v11}else{(if (self.scalar_static_f64[66]!=0.0){v11}else{v1230})});
        let v1642=(if self.scalar_static_bool[21]{v11}else{(if (self.scalar_static_f64[66]!=0.0){v11}else{v1232})});
        let v1645=(v42-(v425*v425));
        let v1650=(v42-(v427*v427));
        let v1660=scalar_limexp_derivative(v434);
        let v1668=(self.scalar_static_f64[68]*(-(if self.scalar_static_bool[21]{v11}else{(if (self.scalar_static_f64[66]!=0.0){v11}else{v1231})})));
        let v1673=scalar_limexp_derivative(v438);
        let v1687=(v42-(v447*v447));
        let v1693=(v42-(v453*v453));
        let v1694=(self.scalar_static_f64[71]*v1693);
        let v1695=(self.scalar_static_f64[132]*v1693);
        let v1698=(v42-(v459*v459));
        let v1699=(self.scalar_static_f64[133]*v1698);
        let v1700=(self.scalar_static_f64[73]*v1698);
        let v1704=(v42-(v465*v465));
        let v1734=(v495).sinh();
        let v1737=(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1734)}else{v11});
        let v1738=(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]*v1734)}else{v11});
        let v1747=(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1743)}else{v11});
        let v1748=(if self.scalar_static_bool[33]{(self.scalar_static_f64[131]*v1743)}else{v11});
        let v1787=(v516).sinh();
        let v1853=(-(if self.scalar_static_bool[17]{(v131*((if self.scalar_static_bool[17]{((v354*((v350*(v114*v1371))+(v349*(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1371)}else{v1161}))*v1430)}else{v1178}))))+(v351*(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1371)}else{v11}))))}else{v981})-(if self.scalar_static_bool[17]{((v361*((v358*(v114*v1403))+(v357*(-(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1403)}else{v11}))*v1447)}else{v11})))))+(v359*(-(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1403)}else{v11})))))}else{v1053})))}else{(if self.scalar_static_bool[14]{((v289*((v285*v1178)+(v281*(v114*v1153))))+(v286*(v849+(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[58]*v1153)}else{v944})))))}else{(if self.scalar_static_bool[11]{(v131*(v981-v1053))}else{(if (self.scalar_static_f64[53]!=0.0){((v202*(v836+(v183*v830)))+(v196*v849))}else{v11})})})}));
        let v1856=ddt_scale;
        let v1900=(v386*v386);
        let v1932=(self.scalar_static_f64[92]*v1856);
        let v1953=(self.scalar_static_f64[96]*v1856);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * ((-(if self.scalar_static_bool[17]{(v131*((if self.scalar_static_bool[17]{(v351*v354)}else{v238})-(if self.scalar_static_bool[17]{(v359*v361)}else{v251})))}else{(if self.scalar_static_bool[14]{(v286*v289)}else{(if self.scalar_static_bool[11]{(v131*(v238-v251))}else{(if (self.scalar_static_f64[53]!=0.0){(v196*v202)}else{v11})})})}))),
            [3, 4, 5, 8],
            [(-v1568), v1853, (-v1570), (-(if self.scalar_static_bool[17]{(v131*((if self.scalar_static_bool[17]{((v354*((v350*(v114*v1373))+(v349*(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1373)}else{v1163}))*v1430)}else{v1180}))))+(v351*(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1373)}else{v11}))))}else{v983})-(if self.scalar_static_bool[17]{((v361*((v358*(v114*v1405))+(v357*(-(if self.scalar_static_bool[17]{((v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[47]*v1405)}else{v11}))*v1447)}else{v11})))))+(v359*(-(v7*(if self.scalar_static_bool[17]{(self.scalar_static_f64[58]*v1405)}else{v11})))))}else{v1055})))}else{(if self.scalar_static_bool[14]{((v289*((v285*v1180)+(v281*(v114*v1155))))+(v286*(v7*(if self.scalar_static_bool[14]{(self.scalar_static_f64[58]*v1155)}else{v946}))))}else{(if self.scalar_static_bool[11]{(v131*(v983-v1055))}else{(if (self.scalar_static_f64[53]!=0.0){(v202*(v842+(v183*v832)))}else{v11})})})}))],
            [],
            [],
            multiplicity,
        );
        let v602_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_static_f64[108]*ctx.node_voltage(nodes[12])));
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v602_ddt),
            12,
            multiplicity * (((self.scalar_static_f64[108]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v10),
            13,
            multiplicity * (v42),
        );
        let v605_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_static_f64[109]*ctx.branch_current(branches[0])));
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v605_ddt,
            0,
            ((self.scalar_static_f64[109]) * ddt_scale),
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
            multiplicity * (v437),
            [3, 4, 5, 8],
            [(self.scalar_static_f64[68]*(-v1640)), v1668, (self.scalar_static_f64[68]*(((v138*(if self.scalar_static_bool[25]{v407}else{(if self.scalar_static_bool[23]{(-v1645)}else{self.scalar_static_f64[124]})}))*v1660)-v1642)), (self.scalar_static_f64[68]*(((v138*(if self.scalar_static_bool[25]{v42}else{(if self.scalar_static_bool[23]{v1645}else{self.scalar_static_f64[125]})}))*v1660)-self.scalar_static_f64[126]))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * ((self.scalar_static_f64[68]*(scalar_limexp(v438)-v421))),
            [3, 4, 5, 7, 8],
            [(self.scalar_static_f64[68]*(((v138*(if self.scalar_static_bool[25]{v407}else{(if self.scalar_static_bool[23]{(-v1650)}else{self.scalar_static_f64[124]})}))*v1673)-v1640)), v1668, (self.scalar_static_f64[68]*(-v1642)), (self.scalar_static_f64[68]*((v138*(if self.scalar_static_bool[25]{v42}else{(if self.scalar_static_bool[23]{v1650}else{self.scalar_static_f64[125]})}))*v1673)), self.scalar_static_f64[128]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){v606}else{v11})),
            [3, 5, 7, 8],
            [(if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{((v118*(self.scalar_static_f64[136]+(((v528*v1699)+(v461*((self.scalar_static_f64[135]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[135]*v1796)}else{v1747})/v522)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[130]*v1787)}else{v1737})/v518)}else{v11}))}else{v11}))))/self.scalar_static_f64[32])))+self.scalar_static_f64[138])}else{v11})*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v118*(((v528*v1700)+(v461*((self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1796)}else{v1748})/v522)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1787)}else{v1738})/v518)}else{v11}))}else{v11}))))/self.scalar_static_f64[32]))}else{v11})*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){(v1844*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v118*((v461*(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{v11}else{v1749})/v522)}else{v11}))/self.scalar_static_f64[32]))}else{v11})*v1856)}else{v11})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){v608}else{v11})),
            3,
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v117*(((v507*v1694)+(v494*((self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{(v1747/v501)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{(v1737/v497)}else{v11}))}else{v11}))))/self.scalar_static_f64[29]))}else{v11})*v1856)}else{v11})),
            5,
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{((v117*((((v507*v1695)+(v494*((self.scalar_static_f64[131]+(if self.scalar_static_bool[33]{(v1748/v501)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]+(if self.scalar_static_bool[33]{(v1738/v497)}else{v11}))}else{v11}))))/self.scalar_static_f64[29])+self.scalar_static_f64[136]))+self.scalar_static_f64[137])}else{v11})*v1856)}else{v11})),
            8,
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){(v1786*v1856)}else{v11})),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[51]{v612}else{v11})),
            3,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*((-v540)+(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*((v466*v1699)+(v461*(self.scalar_static_f64[135]*v1704))))}else{v11})}))))}else{v11})),
            5,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*((v466*v1700)+(v461*(self.scalar_static_f64[69]*v1704))))}else{v11})})))}else{v11})),
            7,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v540+(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*(v461*(self.scalar_static_f64[32]*v1704)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[51]{v615}else{v11})),
            3,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{((v480*v1694)+(v454*(v117*(self.scalar_static_f64[69]*v1687))))}else{v11})})))}else{v11})),
            5,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*((-v538)+(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{((v480*v1695)+(v454*(v117*(self.scalar_static_f64[131]*v1687))))}else{v11})}))))}else{v11})),
            8,
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v538+(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v454*(v117*(self.scalar_static_f64[29]*v1687)))}else{v11})}))))}else{v11})),
        );
        let v620_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (self.scalar_static_f64[110]*(ctx.node_voltage(nodes[1])-v4)));
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v620_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[110]) * ddt_scale)),
            3,
            multiplicity * (((self.scalar_static_f64[139]) * ddt_scale)),
        );
        let v622_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (v7*self.scalar_static_f64[111]));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (v622_ddt),
            3,
            multiplicity * (((self.scalar_static_f64[111]) * ddt_scale)),
            5,
            multiplicity * (((self.scalar_static_f64[140]) * ddt_scale)),
        );
        let v625_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (v120*(v4-v623)));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (v625_ddt),
            3,
            multiplicity * (((v120) * ddt_scale)),
            10,
            multiplicity * ((((-v120)) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){(v626/v386)}else{v11})),
            [3, 4, 5, 8, 10],
            [(if (self.scalar_static_f64[83]!=0.0){((-(v626*(if self.scalar_static_bool[19]{((-(v119*v1370))/v1599)}else{(if (self.scalar_static_f64[59]!=0.0){((-(v119*v780))/v1574)}else{v11})})))/v1900)}else{v11}), (if (self.scalar_static_f64[83]!=0.0){((-(v626*(if self.scalar_static_bool[19]{((-(v119*v1371))/v1599)}else{(if (self.scalar_static_f64[59]!=0.0){((-(v119*v781))/v1574)}else{v11})})))/v1900)}else{v11}), (if (self.scalar_static_f64[83]!=0.0){(((-v386)-(v626*(if self.scalar_static_bool[19]{((-(v119*v1372))/v1599)}else{(if (self.scalar_static_f64[59]!=0.0){((-(v119*v782))/v1574)}else{v11})})))/v1900)}else{v11}), (if (self.scalar_static_f64[83]!=0.0){((-(v626*(if self.scalar_static_bool[19]{((-(v119*v1373))/v1599)}else{(if (self.scalar_static_f64[59]!=0.0){((-(v119*v783))/v1574)}else{v11})})))/v1900)}else{v11}), (if (self.scalar_static_f64[83]!=0.0){(v42/v386)}else{v11})],
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
        let v632_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (self.scalar_static_f64[112]*(v630-v0)));
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v632_ddt),
            8,
            multiplicity * (((self.scalar_static_f64[141]) * ddt_scale)),
            9,
            multiplicity * (((self.scalar_static_f64[112]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * ((if (self.scalar_static_f64[85]!=0.0){((v630-v1)/self.scalar_static_f64[84])}else{v11})),
            5,
            multiplicity * (self.scalar_static_f64[144]),
            9,
            multiplicity * (self.scalar_static_f64[145]),
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
            multiplicity * ((if (self.scalar_static_f64[87]!=0.0){((v3-v8)/self.scalar_static_f64[86])}else{v11})),
            4,
            multiplicity * (self.scalar_static_f64[148]),
            7,
            multiplicity * (self.scalar_static_f64[149]),
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
            multiplicity * ((if (self.scalar_static_f64[89]!=0.0){((v3-v0)/self.scalar_static_f64[88])}else{v11})),
            4,
            multiplicity * (self.scalar_static_f64[152]),
            8,
            multiplicity * (self.scalar_static_f64[153]),
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
            (if (self.scalar_static_f64[91]!=0.0){(self.scalar_static_f64[90]*ctx.branch_current(branches[5]))}else{v11}),
            5,
            self.scalar_static_f64[154],
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            6,
            (if (self.scalar_static_f64[91]!=0.0){v647}else{v11}),
            6,
            (if (self.scalar_static_f64[91]!=0.0){v1932}else{v11}),
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
            (if self.scalar_static_bool[53]{v653}else{v11}),
            8,
            (if self.scalar_static_bool[53]{v1932}else{v11}),
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
            (if (self.scalar_static_f64[94]!=0.0){(v403*v655)}else{v11}),
            [3, 4, 5, 8],
            [(if (self.scalar_static_f64[94]!=0.0){(v655*v1630)}else{v11}), (if (self.scalar_static_f64[94]!=0.0){(v655*v1631)}else{v11}), (if (self.scalar_static_f64[94]!=0.0){(v655*v1632)}else{v11}), (if (self.scalar_static_f64[94]!=0.0){(v655*v1633)}else{v11})],
            [10],
            [(if (self.scalar_static_f64[94]!=0.0){v403}else{v11})],
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
        let v660_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, (self.scalar_static_f64[113]*ctx.branch_current(branches[13])));
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            13,
            v660_ddt,
            13,
            ((self.scalar_static_f64[113]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            14,
            (if (self.scalar_static_f64[95]!=0.0){(v402*v661)}else{v11}),
            [3, 4, 5, 8],
            [(if (self.scalar_static_f64[95]!=0.0){(v661*v1630)}else{v11}), (if (self.scalar_static_f64[95]!=0.0){(v661*v1631)}else{v11}), (if (self.scalar_static_f64[95]!=0.0){(v661*v1632)}else{v11}), (if (self.scalar_static_f64[95]!=0.0){(v661*v1633)}else{v11})],
            [14],
            [(if (self.scalar_static_f64[95]!=0.0){v402}else{v11})],
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            15,
            (if (self.scalar_static_f64[95]!=0.0){v666}else{v11}),
            15,
            (if (self.scalar_static_f64[95]!=0.0){v1953}else{v11}),
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
            (if self.scalar_static_bool[55]{v672}else{v11}),
            17,
            (if self.scalar_static_bool[55]{v1953}else{v11}),
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
            multiplicity * (v675),
            14,
            multiplicity * (self.scalar_static_f64[155]),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (v11),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * ((if self.scalar_static_bool[49]{v676}else{v11})),
            15,
            multiplicity * (self.scalar_static_f64[155]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v675),
            14,
            multiplicity * (self.scalar_static_f64[155]),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[49]{((v594*v674)+(v590*v676))}else{v11})),
            14,
            multiplicity * ((if self.scalar_static_bool[49]{v594}else{v11})),
            15,
            multiplicity * ((if self.scalar_static_bool[49]{v590}else{v11})),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[49]{v684}else{v11})),
            14,
            multiplicity * ((if self.scalar_static_bool[49]{(v682*v1856)}else{v11})),
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
            multiplicity * (v674),
            14,
            multiplicity * (v42),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v676),
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
            multiplicity * ((if (self.scalar_static_f64[107]!=0.0){v688}else{v11})),
            11,
            multiplicity * ((if (self.scalar_static_f64[107]!=0.0){(self.scalar_static_f64[114]*v1856)}else{v11})),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * ((if (self.scalar_static_f64[107]!=0.0){(-(((v7*(-v10))+(v2*v437))).abs())}else{v11})),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if (self.scalar_static_f64[107]!=0.0){(v30/self.scalar_static_f64[11])}else{v11})),
            11,
            multiplicity * (self.scalar_static_f64[157]),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if self.scalar_static_bool[56]{(v30*1e-12)}else{v11})),
            11,
            multiplicity * (self.scalar_static_f64[158]),
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
        let v33=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[160]+(v30).abs())}else{self.scalar_static_f64[160]});
        let v37=((v33-self.scalar_static_f64[9])).abs();
        let v42=1.0;
        let v43=(if ((v37>v11)||self.scalar_static_bool[2]){v42}else{v11});
        let v113=(!(v43!=0.0));
        let v117=(if v113{self.scalar_static_f64[18]}else{(if (v43!=0.0){(self.scalar_static_f64[18]*(v42+(v37*self.scalar_static_f64[19])))}else{v11})});
        let v118=(if v113{self.scalar_static_f64[20]}else{(if (v43!=0.0){(self.scalar_static_f64[20]*(v42+(v37*self.scalar_static_f64[21])))}else{v11})});
        let v120=(if v113{self.scalar_static_f64[24]}else{(if (v43!=0.0){(self.scalar_static_f64[24]*(v42+(v37*self.scalar_static_f64[25])))}else{v11})});
        let v122=(if v113{self.scalar_static_f64[28]}else{(if (v43!=0.0){(self.scalar_static_f64[28]+(v37*self.scalar_static_f64[30]))}else{v11})});
        let v123=(if v113{self.scalar_static_f64[31]}else{(if (v43!=0.0){(self.scalar_static_f64[31]+(v37*self.scalar_static_f64[33]))}else{v11})});
        let v445=(v7*self.scalar_static_f64[69]);
        let v446=((v122+(v2*self.scalar_static_f64[29]))+v445);
        let v447=(v446).tanh();
        let v453=((self.scalar_static_f64[70]+(v7*self.scalar_static_f64[71]))).tanh();
        let v454=(v42+v453);
        let v459=((self.scalar_static_f64[72]-(v7*self.scalar_static_f64[73]))).tanh();
        let v461=((v42+v459)-self.scalar_static_f64[69]);
        let v464=((v123+(v9*self.scalar_static_f64[32]))-v445);
        let v465=(v464).tanh();
        let v466=(v42+v465);
        let v480=(v117*(v42+v447));
        let v494=(if self.scalar_static_bool[33]{(v454-self.scalar_static_f64[69])}else{v454});
        let v495=(v122+v445);
        let v497=(if self.scalar_static_bool[33]{(v495).cosh()}else{v11});
        let v501=(if self.scalar_static_bool[33]{(v446).cosh()}else{v11});
        let v507=((v446+(if self.scalar_static_bool[33]{(v501).ln()}else{v11}))-(if self.scalar_static_bool[33]{(v495+(if self.scalar_static_bool[33]{(v497).ln()}else{v11}))}else{v11}));
        let v516=(v123-v445);
        let v518=(if self.scalar_static_bool[33]{(v516).cosh()}else{v497});
        let v522=(if self.scalar_static_bool[33]{(v464).cosh()}else{v501});
        let v528=((v464+(if self.scalar_static_bool[33]{(v522).ln()}else{v11}))-(if self.scalar_static_bool[33]{(v516+(if self.scalar_static_bool[33]{(v518).ln()}else{v11}))}else{v11}));
        let v1743=(v446).sinh();
        let v1749=(if self.scalar_static_bool[33]{(self.scalar_static_f64[29]*v1743)}else{v11});
        let v1786=(if self.scalar_static_bool[33]{(self.scalar_static_f64[78]+(v117*(self.scalar_static_f64[82]+((v494*(self.scalar_static_f64[29]+(if self.scalar_static_bool[33]{(v1749/v501)}else{v11})))/self.scalar_static_f64[29]))))}else{v11});
        let v537=v1786;
        let v538=(if self.scalar_static_bool[33]{v537}else{(if self.scalar_static_bool[30]{(self.scalar_static_f64[78]+(v454*v480))}else{self.scalar_static_f64[79]})});
        let v1796=(v464).sinh();
        let v1844=(if self.scalar_static_bool[33]{(self.scalar_static_f64[80]+(v118*(self.scalar_static_f64[82]+((v461*(self.scalar_static_f64[32]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[32]*v1796)}else{v11})/v522)}else{v11})))/self.scalar_static_f64[32]))))}else{v11});
        let v539=v1844;
        let v540=(if self.scalar_static_bool[33]{v539}else{(if self.scalar_static_bool[30]{(self.scalar_static_f64[80]+(v118*((v461*v466)+self.scalar_static_f64[82])))}else{self.scalar_static_f64[81]})});
        let v606=0.0;
        let v608=0.0;
        let v612=0.0;
        let v615=0.0;
        let v647=0.0;
        let v653=0.0;
        let v666=0.0;
        let v672=0.0;
        let v682=(-(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{((v117*((v33*5.5226012e-23)*self.scalar_static_f64[104]))*self.scalar_static_f64[106])}else{v11})*3.141592653589793)}else{v11}));
        let v684=0.0;
        let v688=0.0;
        let v1687=(v42-(v447*v447));
        let v1693=(v42-(v453*v453));
        let v1694=(self.scalar_static_f64[71]*v1693);
        let v1695=(self.scalar_static_f64[132]*v1693);
        let v1698=(v42-(v459*v459));
        let v1699=(self.scalar_static_f64[133]*v1698);
        let v1700=(self.scalar_static_f64[73]*v1698);
        let v1704=(v42-(v465*v465));
        let v1734=(v495).sinh();
        let v1737=(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1734)}else{v11});
        let v1738=(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]*v1734)}else{v11});
        let v1747=(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1743)}else{v11});
        let v1748=(if self.scalar_static_bool[33]{(self.scalar_static_f64[131]*v1743)}else{v11});
        let v1787=(v516).sinh();
        let v1856=1.0;
        let v1932=(self.scalar_static_f64[92]*v1856);
        let v1953=(self.scalar_static_f64[96]*v1856);

        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (self.scalar_static_f64[108]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[12]),
            Some(nodes[13]),
            branches[0],
            multiplicity * (self.scalar_static_f64[109]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &[nodes[3], nodes[5], nodes[7], nodes[8]],
            &[(if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{((v118*(self.scalar_static_f64[136]+(((v528*v1699)+(v461*((self.scalar_static_f64[135]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[135]*v1796)}else{v1747})/v522)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[130]*v1787)}else{v1737})/v518)}else{v11}))}else{v11}))))/self.scalar_static_f64[32])))+self.scalar_static_f64[138])}else{v11})*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v118*(((v528*v1700)+(v461*((self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1796)}else{v1748})/v522)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{(self.scalar_static_f64[69]*v1787)}else{v1738})/v518)}else{v11}))}else{v11}))))/self.scalar_static_f64[32]))}else{v11})*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){(v1844*v1856)}else{v11}), (if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v118*((v461*(if self.scalar_static_bool[33]{((if self.scalar_static_bool[33]{v11}else{v1749})/v522)}else{v11}))/self.scalar_static_f64[32]))}else{v11})*v1856)}else{v11})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{(v117*(((v507*v1694)+(v494*((self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{(v1747/v501)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[69]+(if self.scalar_static_bool[33]{(v1737/v497)}else{v11}))}else{v11}))))/self.scalar_static_f64[29]))}else{v11})*v1856)}else{v11})),
            nodes[5],
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){((if self.scalar_static_bool[33]{((v117*((((v507*v1695)+(v494*((self.scalar_static_f64[131]+(if self.scalar_static_bool[33]{(v1748/v501)}else{v11}))-(if self.scalar_static_bool[33]{(self.scalar_static_f64[130]+(if self.scalar_static_bool[33]{(v1738/v497)}else{v11}))}else{v11}))))/self.scalar_static_f64[29])+self.scalar_static_f64[136]))+self.scalar_static_f64[137])}else{v11})*v1856)}else{v11})),
            nodes[8],
            multiplicity * ((if (self.scalar_static_f64[77]!=0.0){(v1786*v1856)}else{v11})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*((-v540)+(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*((v466*v1699)+(v461*(self.scalar_static_f64[135]*v1704))))}else{v11})}))))}else{v11})),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*((v466*v1700)+(v461*(self.scalar_static_f64[69]*v1704))))}else{v11})})))}else{v11})),
            nodes[7],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v540+(v9*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v118*(v461*(self.scalar_static_f64[32]*v1704)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{((v480*v1694)+(v454*(v117*(self.scalar_static_f64[69]*v1687))))}else{v11})})))}else{v11})),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*((-v538)+(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{((v480*v1695)+(v454*(v117*(self.scalar_static_f64[131]*v1687))))}else{v11})}))))}else{v11})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[51]{(v1856*(v538+(v2*(if self.scalar_static_bool[33]{v11}else{(if self.scalar_static_bool[30]{(v454*(v117*(self.scalar_static_f64[29]*v1687)))}else{v11})}))))}else{v11})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[110]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[139]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[111]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[140]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (v120),
            nodes[10],
            multiplicity * ((-v120)),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (self.scalar_static_f64[141]),
            nodes[9],
            multiplicity * (self.scalar_static_f64[112]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[6],
            multiplicity * ((if (self.scalar_static_f64[91]!=0.0){v1932}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[8],
            multiplicity * ((if self.scalar_static_bool[53]{v1932}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[2]),
            branches[13],
            multiplicity * (self.scalar_static_f64[113]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[15],
            multiplicity * ((if (self.scalar_static_f64[95]!=0.0){v1953}else{v11})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[17],
            multiplicity * ((if self.scalar_static_bool[55]{v1953}else{v11})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[14],
            multiplicity * ((if self.scalar_static_bool[49]{(v682*v1856)}else{v11})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * ((if (self.scalar_static_f64[107]!=0.0){(self.scalar_static_f64[114]*v1856)}else{v11})),
        );
    }
}
