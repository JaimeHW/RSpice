#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;

#[inline]
fn scalar_limited_exp(arg: f64) -> f64 {
    if arg > 80.0 { LIMEXP_MAX * (1.0 + arg - 80.0) } else if arg < -80.0 { 1.804851387e-35 } else { arg.exp() }
}

#[inline]
fn scalar_limited_exp_derivative(arg: f64) -> f64 {
    if arg > 80.0 { LIMEXP_MAX } else if arg < -80.0 { 0.0 } else { arg.exp() }
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
        let v0=1.602e-19;
        let v4=1.0;
        let v5=2.0;
        let v9=0.0;
        let v10=-1.0;
        let v12=ctx.node_voltage(nodes[4]);
        let v13=(ctx.temperature()+v12);
        let v15=(v13/self.scalar_static_f64[1]);
        let v17=0.000909;
        let v19=(v17*f64::powf(v13,v5));
        let v21=(v13+830.0);
        let v24=(5.618214e-19-(v0*(v19/v21)));
        let v33=0.3333333333333333;
        let v34=0.5;
        let v39=(self.scalar_static_f64[9]*f64::powf(v15,self.scalar_static_f64[10]));
        let v41=(v15*0.0259);
        let v45=6.434283176858164e24;
        let v46=(-v24);
        let v48=(v41*3.204e-19);
        let v50=((v46/v48)).exp();
        let v51=(v45*v50);
        let v53=(self.scalar_static_f64[13]/v51);
        let v54=1e-38;
        let v55=(v53>v54);
        let v56=(if v55{v53}else{v54});
        let v57=(v56).ln();
        let v61=ctx.node_voltage(nodes[6]);
        let v62=(ctx.node_voltage(nodes[1])-v61);
        let v63=(v61-v61);
        let v65=(ctx.node_voltage(nodes[5])-v61);
        let v66=(v65-v63);
        let v68=(v66*self.scalar_static_f64[16]);
        let v70=(v68>80.0);
        let v72=(!v70);
        let v73=(v68).exp();
        let v74=(v4+v73);
        let v82=((((if v72{(v74).ln()}else{(if v70{v68}else{v9})})*self.scalar_static_f64[17])-v66)-self.scalar_static_f64[18]);
        let v86=(-(v63+(v34*(v66-v82))));
        let v99=(v4+(((self.scalar_static_f64[21]+(v82*self.scalar_static_f64[22]))-(v86*self.scalar_static_f64[23]))/self.scalar_static_f64[15]));
        let v101=(v99-v4);
        let v105=(((v101*v101)+0.0006250000000000001)).sqrt();
        let v107=(v34*((v4+v99)+v105));
        let v108=(v41*v107);
        let v109=(v4/v108);
        let v110=(v65*v109);
        let v112=(v63*v109);
        let v119=(-(self.scalar_static_f64[25]+(v86*self.scalar_static_f64[26])));
        let v120=(v82*v119);
        let v123=(((v62*v109)-(v109*self.scalar_static_f64[24]))-(v109*v120));
        let v127=(((v4/v41)*self.scalar_static_f64[27])).sqrt();
        let v128=(v127/self.scalar_static_f64[15]);
        let v130=3.0;
        let v131=1.4142135623730951;
        let v135=((v34*v123)-(v130*(v4+(v128/v131))));
        let v137=6.0;
        let v140=(((v135*v135)+(v123*v137))).sqrt();
        let v141=(v135+v140);
        let v142=(v123<v9);
        let v143=(v123-v141);
        let v145=(if v142{(v143/v128)}else{v9});
        let v148=((v4-v141)+(v145*v145));
        let v149=(v148>v54);
        let v150=(if v149{v148}else{v54});
        let v154=(!v142);
        let v155=(-v141);
        let v157=(if v154{scalar_limited_exp(v155)}else{v145});
        let v159=(if v154{(v34*v128)}else{v135});
        let v164=(((v157+(v123-v4))+(v159*v159))).sqrt();
        let v166=(if v154{(v164-v159)}else{v141});
        let v170=(if v154{((v4+(v166*v166))-v157)}else{(if v142{(-(v150).ln())}else{v9})});
        let v172=(v170-v4);
        let v175=((v4+(v172*v172))).sqrt();
        let v178=((v34*((v4+v170)+v175))).sqrt();
        let v179=(v5*v178);
        let v181=(v4+(v128/v179));
        let v183=(self.scalar_static_f64[15]*(v5*v181));
        let v184=(v41*v183);
        let v186=(self.scalar_static_f64[28]/v41);
        let v187=(8.353992494899963e17*v41);
        let v188=0.6666;
        let v189=f64::powf(v187,v188);
        let v190=(v186*v189);
        let v193=(v0*v41);
        let v197=(v57+(v170-(self.scalar_static_f64[30]+(v24/v193))));
        let v198=(v197-v112);
        let v199=(v198<=v9);
        let v200=(!v199);
        let v201=50.0;
        let v202=(v198<v201);
        let v204=(v200&&(!v202));
        let v205=(v0*v187);
        let v206=(v4+v198);
        let v207=(v33*v190);
        let v208=(self.scalar_static_f64[15]*v181);
        let v209=(v0/v208);
        let v210=(v187*v209);
        let v211=(v210/v41);
        let v212=(v4+v211);
        let v213=0.6666666666666666;
        let v214=(v190*v213);
        let v215=(v190+v211);
        let v217=(v34+(v198/v215));
        let v219=(v200&&v202);
        let v221=(v198).exp();
        let v223=(if v199{(v137/v221)}else{v9});
        let v224=(if v219{(v4/v217)}else{v223});
        let v225=(if v204{(v130/v217)}else{v224});
        let v226=f64::powf(v225,v33);
        let v228=(v212+(v214*v226));
        let v229=(v34+v198);
        let v230=-0.6666666666666666;
        let v231=f64::powf(v225,v230);
        let v233=(v229-(v207*v231));
        let v236=f64::powf(v224,v33);
        let v238=((v211+v224)+(v214*v236));
        let v241=f64::powf(v224,v230);
        let v243=((v206+(v224).ln())-(v207*v241));
        let v246=f64::powf(v223,v33);
        let v248=((v211+v223)+(v214*v246));
        let v251=f64::powf(v223,v230);
        let v253=((v206+(v223).ln())-(v207*v251));
        let v255=(if v199{(v248/v253)}else{v9});
        let v256=(if v219{(v238/v243)}else{v255});
        let v257=(if v204{(v228/v233)}else{v256});
        let v258=f64::powf(v257,v33);
        let v260=(v212+(v214*v258));
        let v261=f64::powf(v257,v230);
        let v263=(v229-(v207*v261));
        let v266=f64::powf(v256,v33);
        let v268=((v211+v256)+(v214*v266));
        let v271=f64::powf(v256,v230);
        let v273=((v206+(v256).ln())-(v207*v271));
        let v276=f64::powf(v255,v33);
        let v278=((v211+v255)+(v214*v276));
        let v281=f64::powf(v255,v230);
        let v283=((v206+(v255).ln())-(v207*v281));
        let v285=(if v199{(v278/v283)}else{v9});
        let v286=(if v219{(v268/v273)}else{v285});
        let v287=(if v204{(v260/v263)}else{v286});
        let v288=f64::powf(v287,v33);
        let v290=(v212+(v214*v288));
        let v291=f64::powf(v287,v230);
        let v293=(v229-(v207*v291));
        let v295=(if v204{(v290/v293)}else{v9});
        let v296=f64::powf(v295,v230);
        let v298=(v206-(v207*v296));
        let v299=f64::powf(v295,v33);
        let v301=(v212+(v214*v299));
        let v305=f64::powf(v286,v230);
        let v307=((v206+(v286).ln())-(v207*v305));
        let v309=f64::powf(v286,v33);
        let v311=((v211+v286)+(v214*v309));
        let v315=f64::powf(v285,v230);
        let v317=((v206+(v285).ln())-(v207*v315));
        let v319=f64::powf(v285,v33);
        let v321=((v211+v285)+(v214*v319));
        let v323=(if v199{(v317/v321)}else{v9});
        let v324=(if v219{(v307/v311)}else{v323});
        let v325=(if v204{(v298/v301)}else{v324});
        let v326=(v205*v325);
        let v328=(v205*v324);
        let v330=(v205*v323);
        let v334=(if v204{(v326/v184)}else{(if v219{(v328/v184)}else{(if v199{(v330/v184)}else{v9})})});
        let v335=(v5*v334);
        let v336=(v170-v335);
        let v338=(v336-v4);
        let v341=((v4+(v338*v338))).sqrt();
        let v344=((v34*((v4+v336)+v341))).sqrt();
        let v345=(v178+v344);
        let v347=(v4+(v128/v345));
        let v357=(self.scalar_static_f64[35]+(v86*self.scalar_static_f64[36]));
        let v367=(v5*v108);
        let v368=(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*f64::powf(v15,self.scalar_static_f64[41])));
        let v369=(v367/v368);
        let v370=(v369*v369);
        let v371=(v5*v369);
        let v372=(v5+v369);
        let v373=(v334*v371);
        let v375=(v334+f64::powf(v334,v5));
        let v376=(v371*v375);
        let v379=4.0;
        let v382=(((v372*v372)+(v373*v379))).sqrt();
        let v383=((v372+v373)+v382);
        let v384=(v376/v383);
        let v385=(v334-v384);
        let v386=(v385*v385);
        let v391=((v5*v384)+(v384).ln());
        let v393=(v4+(v369*v385));
        let v394=(v391*v393);
        let v395=(v369*self.scalar_static_f64[43]);
        let v397=(0.1+(v385*v395));
        let v400=(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v370)));
        let v401=(v386*v400);
        let v406=(((v4+(v401/v397))+(v370*v386))).sqrt();
        let v409=(((v170-v57)-(v394/v406))-v112);
        let v411=(v409-v130);
        let v414=((v379+(v411*v411))).sqrt();
        let v416=(v34*((v130+v409)+v414));
        let v421=(self.scalar_static_f64[45]*(self.scalar_static_f64[42]+(v379*v384)));
        let v422=(v4+v334);
        let v424=(v110-v112);
        let v425=(v379*(v421/v422));
        let v428=((v4+(v425/v416))).sqrt();
        let v429=(v424*v428);
        let v430=(v416+v429);
        let v432=(v416*v425);
        let v434=(((v430*v430)+v432)).sqrt();
        let v435=(v429-v416);
        let v438=((v432+(v435*v435))).sqrt();
        let v441=(v112+(v34*(v434-v438)));
        let v445=((self.scalar_static_f64[12]*(v34*v369))/self.scalar_static_f64[46]);
        let v446=(v110-v441);
        let v447=(v445*v446);
        let v459=((v4+((v447*v447)+(v447*self.scalar_static_f64[52])))).sqrt();
        let v462=(((v447+self.scalar_static_f64[49])+v459)/self.scalar_static_f64[53]);
        let v465=(v197-v441);
        let v466=(v465<=v9);
        let v467=(!v466);
        let v468=(v465<v201);
        let v470=(v467&&(!v468));
        let v471=(v4+v465);
        let v472=(self.scalar_static_f64[15]*v347);
        let v473=(v0/v472);
        let v474=(v187*v473);
        let v475=(v474/v41);
        let v476=(v4+v475);
        let v477=(v190+v475);
        let v479=(v34+(v465/v477));
        let v481=(v467&&v468);
        let v483=(v465).exp();
        let v485=(if v466{(v137/v483)}else{v9});
        let v486=(if v481{(v4/v479)}else{v485});
        let v487=(if v470{(v130/v479)}else{v486});
        let v488=f64::powf(v487,v33);
        let v490=(v476+(v214*v488));
        let v491=(v34+v465);
        let v492=f64::powf(v487,v230);
        let v494=(v491-(v207*v492));
        let v497=f64::powf(v486,v33);
        let v499=((v475+v486)+(v214*v497));
        let v502=f64::powf(v486,v230);
        let v504=((v471+(v486).ln())-(v207*v502));
        let v507=f64::powf(v485,v33);
        let v509=((v475+v485)+(v214*v507));
        let v512=f64::powf(v485,v230);
        let v514=((v471+(v485).ln())-(v207*v512));
        let v516=(if v466{(v509/v514)}else{v9});
        let v517=(if v481{(v499/v504)}else{v516});
        let v518=(if v470{(v490/v494)}else{v517});
        let v519=f64::powf(v518,v33);
        let v521=(v476+(v214*v519));
        let v522=f64::powf(v518,v230);
        let v524=(v491-(v207*v522));
        let v527=f64::powf(v517,v33);
        let v529=((v475+v517)+(v214*v527));
        let v532=f64::powf(v517,v230);
        let v534=((v471+(v517).ln())-(v207*v532));
        let v537=f64::powf(v516,v33);
        let v539=((v475+v516)+(v214*v537));
        let v542=f64::powf(v516,v230);
        let v544=((v471+(v516).ln())-(v207*v542));
        let v546=(if v466{(v539/v544)}else{v9});
        let v547=(if v481{(v529/v534)}else{v546});
        let v548=(if v470{(v521/v524)}else{v547});
        let v549=f64::powf(v548,v33);
        let v551=(v476+(v214*v549));
        let v552=f64::powf(v548,v230);
        let v554=(v491-(v207*v552));
        let v556=(if v470{(v551/v554)}else{v9});
        let v557=f64::powf(v556,v230);
        let v559=(v471-(v207*v557));
        let v560=f64::powf(v556,v33);
        let v562=(v476+(v214*v560));
        let v566=f64::powf(v547,v230);
        let v568=((v471+(v547).ln())-(v207*v566));
        let v570=f64::powf(v547,v33);
        let v572=((v475+v547)+(v214*v570));
        let v576=f64::powf(v546,v230);
        let v578=((v471+(v546).ln())-(v207*v576));
        let v580=f64::powf(v546,v33);
        let v582=((v475+v546)+(v214*v580));
        let v584=(if v466{(v578/v582)}else{v9});
        let v585=(if v481{(v568/v572)}else{v584});
        let v586=(if v470{(v559/v562)}else{v585});
        let v587=(v205*v586);
        let v588=(self.scalar_static_f64[15]*(v5*v347));
        let v589=(v41*v588);
        let v591=(v205*v585);
        let v593=(v205*v584);
        let v597=(if v470{(v587/v589)}else{(if v481{(v591/v589)}else{(if v466{(v593/v589)}else{v9})})});
        let v599=((v170-v334)-v597);
        let v601=(v599-v4);
        let v604=((v4+(v601*v601))).sqrt();
        let v607=((v34*((v4+v599)+v604))).sqrt();
        let v608=(v178+v607);
        let v610=(v4+(v128/v608));
        let v611=(v334-v597);
        let v612=(v611*v611);
        let v613=(v422+v597);
        let v614=(v4/v613);
        let v615=(v612*v614);
        let v616=(v610-v4);
        let v619=((v334+v597)+(v33*v615));
        let v621=((v123-v170)-(v616*v619));
        let v622=(v33*v610);
        let v623=(v614*v615);
        let v625=0.8;
        let v628=1.2;
        let v631=(v34*((v4+(v334*v625))+(v597*v628)));
        let v633=((v335+v597)+(v623*v631));
        let v641=(v34*((v4+(v334*v628))+(v597*v625)));
        let v643=((v334+(v5*v597))+(v623*v641));
        let v645=(v108*v621);
        let v648=((0.0025000000000000005+(v645*v645))).sqrt();
        let v650=(v34*(v645+v648));
        let v651=((v622*v633)+(v622*v643));
        let v652=(v108*v651);
        let v655=(self.scalar_static_f64[33]*(v650+(self.scalar_static_f64[8]*v652)));
        let v658=(v34*(v4+(v652/v650)));
        let v659=f64::powf(v658,self.scalar_static_f64[34]);
        let v660=f64::powf(v655,self.scalar_static_f64[37]);
        let v664=(v4+((v357*v660)+(self.scalar_static_f64[38]/v659)));
        let v666=(v664-v4);
        let v669=((5.625e-7+(v666*v666))).sqrt();
        let v671=(v34*((v4+v664)+v669));
        let v673=(v5*(v369/v671));
        let v674=(v611*v673);
        let v675=(v674*v674);
        let v677=((v4+v675)).sqrt();
        let v678=(v9!=v674);
        let v679=(v4/v674);
        let v680=(v674).asinh();
        let v685=(!v678);
        let v689=(if v685{(v34*(v677+(v4/v677)))}else{(if v678{(v34*(v677+(v679*v680)))}else{v9})});
        let v690=(v671*v689);
        let v691=(v39/v690);
        let v694=(v610*self.scalar_static_f64[55]);
        let v696=(self.scalar_static_f64[11]*(v691*v694));
        let v697=(self.scalar_static_f64[12]-(self.scalar_static_f64[51]*(v462).ln()));
        let v699=(self.scalar_static_f64[15]*(v696/v697));
        let v700=(v108*v699);
        let v701=(v108*v700);
        let v702=(v611*v613);
        let v703=(v701*v702);
        let v709=(v4-(self.scalar_static_f64[58]*(v15-v4)));
        let v720=((self.scalar_static_f64[60]*f64::powf(v15,self.scalar_static_f64[61]))*self.scalar_static_f64[63]);
        let v721=(self.scalar_static_f64[64]/v720);
        let v722=(v709*v721);
        let v724=(self.scalar_static_f64[65]/v720);
        let v725=(v709*v724);
        let v726=((self.scalar_static_f64[57]*f64::powf(v709,self.scalar_static_f64[59]))*self.scalar_static_f64[63]);
        let v727=(v703/v726);
        let v728=0.96;
        let v729=(v727>=v728);
        let v730=(if v729{v728}else{v727});
        let v733=(v4-f64::powf(v730,self.scalar_static_f64[66]));
        let v735=f64::powf(v733,self.scalar_static_f64[67]);
        let v743=f64::powf(v15,v4);
        let v747=((v722/v735)+(self.scalar_static_f64[71]*(v4+(self.scalar_static_f64[72]*v743))));
        let v752=((v725/v735)+(self.scalar_static_f64[69]*(v4+(v743*self.scalar_static_f64[73]))));
        let v755=(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*(self.scalar_static_f64[9]/v690)));
        let v756=(v755/v697);
        let v757=(v652*v756);
        let v758=(v747+v752);
        let v760=(v4+(v757*v758));
        let v761=(v703/v760);
        let v767=(v761*v761);
        let v776=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (v12*self.scalar_static_f64[75]));
        let v793=(v0*(((v21*(v17*(v5*f64::powf(v13,v4))))-v19)/(v21*v21)));
        let v802=(v41*v41);
        let v817=((if v55{((-(self.scalar_static_f64[13]*(v45*(v50*(((v48*v793)-(v46*self.scalar_static_f64[80]))/(v48*v48))))))/(v51*v51))}else{v9})/v56);
        let v829=((self.scalar_static_f64[17]*(if v72{((self.scalar_static_f64[16]*v73)/v74)}else{(if v70{self.scalar_static_f64[16]}else{v9})}))-v4);
        let v830=((self.scalar_static_f64[17]*(if v72{((v73*self.scalar_static_f64[81])/v74)}else{(if v70{self.scalar_static_f64[81]}else{v9})}))-v10);
        let v835=(-(v34*(v4-v829)));
        let v836=(-(v34*(v10-v830)));
        let v843=(((self.scalar_static_f64[22]*v829)-(self.scalar_static_f64[23]*v835))/self.scalar_static_f64[15]);
        let v844=(((self.scalar_static_f64[22]*v830)-(self.scalar_static_f64[23]*v836))/self.scalar_static_f64[15]);
        let v845=(v101*v843);
        let v847=(v101*v844);
        let v849=(v5*v105);
        let v856=(v107*self.scalar_static_f64[78]);
        let v857=(v41*(v34*(v843+((v845+v845)/v849))));
        let v858=(v41*(v34*(v844+((v847+v847)/v849))));
        let v860=(v108*v108);
        let v861=((-v856)/v860);
        let v863=((-v857)/v860);
        let v865=((-v858)/v860);
        let v866=(v65*v861);
        let v868=(v109+(v65*v863));
        let v869=(-v109);
        let v871=(v869+(v65*v865));
        let v876=(v63*v861);
        let v877=(v63*v863);
        let v878=(v63*v865);
        let v902=(((v62*v861)-(self.scalar_static_f64[24]*v861))-(v120*v861));
        let v903=(((v62*v863)-(self.scalar_static_f64[24]*v863))-((v120*v863)+(v109*((v119*v829)+(v82*(-(self.scalar_static_f64[26]*v835)))))));
        let v904=(((v869+(v62*v865))-(self.scalar_static_f64[24]*v865))-((v120*v865)+(v109*((v119*v830)+(v82*(-(self.scalar_static_f64[26]*v836)))))));
        let v908=(((self.scalar_static_f64[27]*(self.scalar_static_f64[79]/v802))/(v5*v127))/self.scalar_static_f64[15]);
        let v910=(v34*v109);
        let v912=(v34*v903);
        let v913=(v34*v904);
        let v916=((v34*v902)-(v130*(v908/v131)));
        let v917=(v135*v910);
        let v919=(v135*v916);
        let v921=(v135*v912);
        let v923=(v135*v913);
        let v933=(v5*v140);
        let v938=(v910+(((v917+v917)+(v109*v137))/v933));
        let v939=(v916+(((v919+v919)+(v137*v902))/v933));
        let v940=(v912+(((v921+v921)+(v137*v903))/v933));
        let v941=(v913+(((v923+v923)+(v137*v904))/v933));
        let v953=(if v142{((v109-v938)/v128)}else{v9});
        let v954=(if v142{(((v128*(v902-v939))-(v143*v908))/(v128*v128))}else{v9});
        let v955=(if v142{((v903-v940)/v128)}else{v9});
        let v956=(if v142{((v904-v941)/v128)}else{v9});
        let v957=(-v938);
        let v958=(-v939);
        let v959=(-v940);
        let v960=(-v941);
        let v961=(v145*v953);
        let v963=(v145*v954);
        let v965=(v145*v955);
        let v967=(v145*v956);
        let v989=scalar_limited_exp_derivative(v155);
        let v994=(if v154{(v957*v989)}else{v953});
        let v995=(if v154{(v958*v989)}else{v954});
        let v996=(if v154{(v959*v989)}else{v955});
        let v997=(if v154{(v960*v989)}else{v956});
        let v999=(if v154{v9}else{v910});
        let v1000=(if v154{(v34*v908)}else{v916});
        let v1001=(if v154{v9}else{v912});
        let v1002=(if v154{v9}else{v913});
        let v1007=(v159*v999);
        let v1009=(v159*v1000);
        let v1011=(v159*v1001);
        let v1013=(v159*v1002);
        let v1019=(v5*v164);
        let v1032=(v166*(if v154{((((v109+v994)+(v1007+v1007))/v1019)-v999)}else{v938}));
        let v1034=(v166*(if v154{((((v902+v995)+(v1009+v1009))/v1019)-v1000)}else{v939}));
        let v1036=(v166*(if v154{((((v903+v996)+(v1011+v1011))/v1019)-v1001)}else{v940}));
        let v1038=(v166*(if v154{((((v904+v997)+(v1013+v1013))/v1019)-v1002)}else{v941}));
        let v1044=(if v154{((v1032+v1032)-v994)}else{(if v142{(-((if v149{(v957+(v961+v961))}else{v9})/v150))}else{v9})});
        let v1045=(if v154{((v1034+v1034)-v995)}else{(if v142{(-((if v149{(v958+(v963+v963))}else{v9})/v150))}else{v9})});
        let v1046=(if v154{((v1036+v1036)-v996)}else{(if v142{(-((if v149{(v959+(v965+v965))}else{v9})/v150))}else{v9})});
        let v1047=(if v154{((v1038+v1038)-v997)}else{(if v142{(-((if v149{(v960+(v967+v967))}else{v9})/v150))}else{v9})});
        let v1048=(v172*v1044);
        let v1050=(v172*v1045);
        let v1052=(v172*v1046);
        let v1054=(v172*v1047);
        let v1056=(v5*v175);
        let v1069=((v34*(v1044+((v1048+v1048)/v1056)))/v179);
        let v1070=((v34*(v1045+((v1050+v1050)/v1056)))/v179);
        let v1071=((v34*(v1046+((v1052+v1052)/v1056)))/v179);
        let v1072=((v34*(v1047+((v1054+v1054)/v1056)))/v179);
        let v1079=(v179*v179);
        let v1080=((-(v128*(v5*v1069)))/v1079);
        let v1084=(((v179*v908)-(v128*(v5*v1070)))/v1079);
        let v1087=((-(v128*(v5*v1071)))/v1079);
        let v1090=((-(v128*(v5*v1072)))/v1079);
        let v1099=(v41*(self.scalar_static_f64[15]*(v5*v1080)));
        let v1102=((v183*self.scalar_static_f64[78])+(v41*(self.scalar_static_f64[15]*(v5*v1084))));
        let v1103=(v41*(self.scalar_static_f64[15]*(v5*v1087)));
        let v1104=(v41*(self.scalar_static_f64[15]*(v5*v1090)));
        let v1115=((v189*(self.scalar_static_f64[83]/v802))+(v186*(self.scalar_static_f64[84]*(v188*f64::powf(v187,-0.33340000000000003)))));
        let v1123=(v817+(v1045-(((v193*(-v793))-(v24*self.scalar_static_f64[85]))/(v193*v193))));
        let v1124=(v1123-v876);
        let v1125=(v1046-v877);
        let v1126=(v1047-v878);
        let v1128=(v33*v1115);
        let v1135=(v208*v208);
        let v1152=((v187*((-(v0*(self.scalar_static_f64[15]*v1080)))/v1135))/v41);
        let v1156=(((v41*((v209*self.scalar_static_f64[84])+(v187*((-(v0*(self.scalar_static_f64[15]*v1084)))/v1135))))-(v210*self.scalar_static_f64[78]))/v802);
        let v1157=((v187*((-(v0*(self.scalar_static_f64[15]*v1087)))/v1135))/v41);
        let v1158=((v187*((-(v0*(self.scalar_static_f64[15]*v1090)))/v1135))/v41);
        let v1159=(v213*v1115);
        let v1164=(v215*v215);
        let v1165=(((v215*v1044)-(v198*v1152))/v1164);
        let v1169=(((v215*v1124)-(v198*(v1115+v1156)))/v1164);
        let v1173=(((v215*v1125)-(v198*v1157))/v1164);
        let v1177=(((v215*v1126)-(v198*v1158))/v1164);
        let v1180=(v217*v217);
        let v1205=(v221*v221);
        let v1216=(if v199{((-(v137*(v221*v1044)))/v1205)}else{v9});
        let v1217=(if v199{((-(v137*(v221*v1124)))/v1205)}else{v9});
        let v1218=(if v199{((-(v137*(v221*v1125)))/v1205)}else{v9});
        let v1219=(if v199{((-(v137*(v221*v1126)))/v1205)}else{v9});
        let v1220=(if v219{((-v1165)/v1180)}else{v1216});
        let v1221=(if v219{((-v1169)/v1180)}else{v1217});
        let v1222=(if v219{((-v1173)/v1180)}else{v1218});
        let v1223=(if v219{((-v1177)/v1180)}else{v1219});
        let v1224=(if v204{((-(v130*v1165))/v1180)}else{v1220});
        let v1225=(if v204{((-(v130*v1169))/v1180)}else{v1221});
        let v1226=(if v204{((-(v130*v1173))/v1180)}else{v1222});
        let v1227=(if v204{((-(v130*v1177))/v1180)}else{v1223});
        let v1228=-0.6666666666666667;
        let v1230=(v33*f64::powf(v225,v1228));
        let v1245=-1.6666666666666665;
        let v1247=(v230*f64::powf(v225,v1245));
        let v1265=(v233*v233);
        let v1284=(v33*f64::powf(v224,v1228));
        let v1308=(v230*f64::powf(v224,v1245));
        let v1326=(v243*v243);
        let v1345=(v33*f64::powf(v223,v1228));
        let v1369=(v230*f64::powf(v223,v1245));
        let v1387=(v253*v253);
        let v1401=(if v199{(((v253*((v1152+v1216)+(v214*(v1216*v1345))))-(v248*((v1044+(v1216/v223))-(v207*(v1216*v1369)))))/v1387)}else{v9});
        let v1402=(if v199{(((v253*((v1156+v1217)+((v246*v1159)+(v214*(v1217*v1345)))))-(v248*((v1124+(v1217/v223))-((v251*v1128)+(v207*(v1217*v1369))))))/v1387)}else{v9});
        let v1403=(if v199{(((v253*((v1157+v1218)+(v214*(v1218*v1345))))-(v248*((v1125+(v1218/v223))-(v207*(v1218*v1369)))))/v1387)}else{v9});
        let v1404=(if v199{(((v253*((v1158+v1219)+(v214*(v1219*v1345))))-(v248*((v1126+(v1219/v223))-(v207*(v1219*v1369)))))/v1387)}else{v9});
        let v1405=(if v219{(((v243*((v1152+v1220)+(v214*(v1220*v1284))))-(v238*((v1044+(v1220/v224))-(v207*(v1220*v1308)))))/v1326)}else{v1401});
        let v1406=(if v219{(((v243*((v1156+v1221)+((v236*v1159)+(v214*(v1221*v1284)))))-(v238*((v1124+(v1221/v224))-((v241*v1128)+(v207*(v1221*v1308))))))/v1326)}else{v1402});
        let v1407=(if v219{(((v243*((v1157+v1222)+(v214*(v1222*v1284))))-(v238*((v1125+(v1222/v224))-(v207*(v1222*v1308)))))/v1326)}else{v1403});
        let v1408=(if v219{(((v243*((v1158+v1223)+(v214*(v1223*v1284))))-(v238*((v1126+(v1223/v224))-(v207*(v1223*v1308)))))/v1326)}else{v1404});
        let v1409=(if v204{(((v233*(v1152+(v214*(v1224*v1230))))-(v228*(v1044-(v207*(v1224*v1247)))))/v1265)}else{v1405});
        let v1410=(if v204{(((v233*(v1156+((v226*v1159)+(v214*(v1225*v1230)))))-(v228*(v1124-((v231*v1128)+(v207*(v1225*v1247))))))/v1265)}else{v1406});
        let v1411=(if v204{(((v233*(v1157+(v214*(v1226*v1230))))-(v228*(v1125-(v207*(v1226*v1247)))))/v1265)}else{v1407});
        let v1412=(if v204{(((v233*(v1158+(v214*(v1227*v1230))))-(v228*(v1126-(v207*(v1227*v1247)))))/v1265)}else{v1408});
        let v1414=(v33*f64::powf(v257,v1228));
        let v1430=(v230*f64::powf(v257,v1245));
        let v1448=(v263*v263);
        let v1467=(v33*f64::powf(v256,v1228));
        let v1491=(v230*f64::powf(v256,v1245));
        let v1509=(v273*v273);
        let v1528=(v33*f64::powf(v255,v1228));
        let v1552=(v230*f64::powf(v255,v1245));
        let v1570=(v283*v283);
        let v1584=(if v199{(((v283*((v1152+v1401)+(v214*(v1401*v1528))))-(v278*((v1044+(v1401/v255))-(v207*(v1401*v1552)))))/v1570)}else{v9});
        let v1585=(if v199{(((v283*((v1156+v1402)+((v276*v1159)+(v214*(v1402*v1528)))))-(v278*((v1124+(v1402/v255))-((v281*v1128)+(v207*(v1402*v1552))))))/v1570)}else{v9});
        let v1586=(if v199{(((v283*((v1157+v1403)+(v214*(v1403*v1528))))-(v278*((v1125+(v1403/v255))-(v207*(v1403*v1552)))))/v1570)}else{v9});
        let v1587=(if v199{(((v283*((v1158+v1404)+(v214*(v1404*v1528))))-(v278*((v1126+(v1404/v255))-(v207*(v1404*v1552)))))/v1570)}else{v9});
        let v1588=(if v219{(((v273*((v1152+v1405)+(v214*(v1405*v1467))))-(v268*((v1044+(v1405/v256))-(v207*(v1405*v1491)))))/v1509)}else{v1584});
        let v1589=(if v219{(((v273*((v1156+v1406)+((v266*v1159)+(v214*(v1406*v1467)))))-(v268*((v1124+(v1406/v256))-((v271*v1128)+(v207*(v1406*v1491))))))/v1509)}else{v1585});
        let v1590=(if v219{(((v273*((v1157+v1407)+(v214*(v1407*v1467))))-(v268*((v1125+(v1407/v256))-(v207*(v1407*v1491)))))/v1509)}else{v1586});
        let v1591=(if v219{(((v273*((v1158+v1408)+(v214*(v1408*v1467))))-(v268*((v1126+(v1408/v256))-(v207*(v1408*v1491)))))/v1509)}else{v1587});
        let v1592=(if v204{(((v263*(v1152+(v214*(v1409*v1414))))-(v260*(v1044-(v207*(v1409*v1430)))))/v1448)}else{v1588});
        let v1593=(if v204{(((v263*(v1156+((v258*v1159)+(v214*(v1410*v1414)))))-(v260*(v1124-((v261*v1128)+(v207*(v1410*v1430))))))/v1448)}else{v1589});
        let v1594=(if v204{(((v263*(v1157+(v214*(v1411*v1414))))-(v260*(v1125-(v207*(v1411*v1430)))))/v1448)}else{v1590});
        let v1595=(if v204{(((v263*(v1158+(v214*(v1412*v1414))))-(v260*(v1126-(v207*(v1412*v1430)))))/v1448)}else{v1591});
        let v1597=(v33*f64::powf(v287,v1228));
        let v1613=(v230*f64::powf(v287,v1245));
        let v1631=(v293*v293);
        let v1645=(if v204{(((v293*(v1152+(v214*(v1592*v1597))))-(v290*(v1044-(v207*(v1592*v1613)))))/v1631)}else{v9});
        let v1646=(if v204{(((v293*(v1156+((v288*v1159)+(v214*(v1593*v1597)))))-(v290*(v1124-((v291*v1128)+(v207*(v1593*v1613))))))/v1631)}else{v9});
        let v1647=(if v204{(((v293*(v1157+(v214*(v1594*v1597))))-(v290*(v1125-(v207*(v1594*v1613)))))/v1631)}else{v9});
        let v1648=(if v204{(((v293*(v1158+(v214*(v1595*v1597))))-(v290*(v1126-(v207*(v1595*v1613)))))/v1631)}else{v9});
        let v1650=(v230*f64::powf(v295,v1245));
        let v1666=(v33*f64::powf(v295,v1228));
        let v1684=(v301*v301);
        let v1707=(v230*f64::powf(v286,v1245));
        let v1727=(v33*f64::powf(v286,v1228));
        let v1745=(v311*v311);
        let v1768=(v230*f64::powf(v285,v1245));
        let v1788=(v33*f64::powf(v285,v1228));
        let v1806=(v321*v321);
        let v1820=(if v199{(((v321*((v1044+(v1584/v285))-(v207*(v1584*v1768))))-(v317*((v1152+v1584)+(v214*(v1584*v1788)))))/v1806)}else{v9});
        let v1821=(if v199{(((v321*((v1124+(v1585/v285))-((v315*v1128)+(v207*(v1585*v1768)))))-(v317*((v1156+v1585)+((v319*v1159)+(v214*(v1585*v1788))))))/v1806)}else{v9});
        let v1822=(if v199{(((v321*((v1125+(v1586/v285))-(v207*(v1586*v1768))))-(v317*((v1157+v1586)+(v214*(v1586*v1788)))))/v1806)}else{v9});
        let v1823=(if v199{(((v321*((v1126+(v1587/v285))-(v207*(v1587*v1768))))-(v317*((v1158+v1587)+(v214*(v1587*v1788)))))/v1806)}else{v9});
        let v1824=(if v219{(((v311*((v1044+(v1588/v286))-(v207*(v1588*v1707))))-(v307*((v1152+v1588)+(v214*(v1588*v1727)))))/v1745)}else{v1820});
        let v1825=(if v219{(((v311*((v1124+(v1589/v286))-((v305*v1128)+(v207*(v1589*v1707)))))-(v307*((v1156+v1589)+((v309*v1159)+(v214*(v1589*v1727))))))/v1745)}else{v1821});
        let v1826=(if v219{(((v311*((v1125+(v1590/v286))-(v207*(v1590*v1707))))-(v307*((v1157+v1590)+(v214*(v1590*v1727)))))/v1745)}else{v1822});
        let v1827=(if v219{(((v311*((v1126+(v1591/v286))-(v207*(v1591*v1707))))-(v307*((v1158+v1591)+(v214*(v1591*v1727)))))/v1745)}else{v1823});
        let v1841=(v184*v184);
        let v1907=(if v204{(((v184*(v205*(if v204{(((v301*(v1044-(v207*(v1645*v1650))))-(v298*(v1152+(v214*(v1645*v1666)))))/v1684)}else{v1824})))-(v326*v1099))/v1841)}else{(if v219{(((v184*(v205*v1824))-(v328*v1099))/v1841)}else{(if v199{(((v184*(v205*v1820))-(v330*v1099))/v1841)}else{v9})})});
        let v1908=(if v204{(((v184*((v325*self.scalar_static_f64[86])+(v205*(if v204{(((v301*(v1124-((v296*v1128)+(v207*(v1646*v1650)))))-(v298*(v1156+((v299*v1159)+(v214*(v1646*v1666))))))/v1684)}else{v1825}))))-(v326*v1102))/v1841)}else{(if v219{(((v184*((v324*self.scalar_static_f64[86])+(v205*v1825)))-(v328*v1102))/v1841)}else{(if v199{(((v184*((v323*self.scalar_static_f64[86])+(v205*v1821)))-(v330*v1102))/v1841)}else{v9})})});
        let v1909=(if v204{(((v184*(v205*(if v204{(((v301*(v1125-(v207*(v1647*v1650))))-(v298*(v1157+(v214*(v1647*v1666)))))/v1684)}else{v1826})))-(v326*v1103))/v1841)}else{(if v219{(((v184*(v205*v1826))-(v328*v1103))/v1841)}else{(if v199{(((v184*(v205*v1822))-(v330*v1103))/v1841)}else{v9})})});
        let v1910=(if v204{(((v184*(v205*(if v204{(((v301*(v1126-(v207*(v1648*v1650))))-(v298*(v1158+(v214*(v1648*v1666)))))/v1684)}else{v1827})))-(v326*v1104))/v1841)}else{(if v219{(((v184*(v205*v1827))-(v328*v1104))/v1841)}else{(if v199{(((v184*(v205*v1823))-(v330*v1104))/v1841)}else{v9})})});
        let v1911=(v5*v1907);
        let v1912=(v5*v1908);
        let v1913=(v5*v1909);
        let v1914=(v5*v1910);
        let v1915=(v1044-v1911);
        let v1916=(v1045-v1912);
        let v1917=(v1046-v1913);
        let v1918=(v1047-v1914);
        let v1919=(v338*v1915);
        let v1921=(v338*v1916);
        let v1923=(v338*v1917);
        let v1925=(v338*v1918);
        let v1927=(v5*v341);
        let v1940=(v5*v344);
        let v1951=(v345*v345);
        let v1952=((-(v128*(v1069+((v34*(v1915+((v1919+v1919)/v1927)))/v1940))))/v1951);
        let v1956=(((v345*v908)-(v128*(v1070+((v34*(v1916+((v1921+v1921)/v1927)))/v1940))))/v1951);
        let v1959=((-(v128*(v1071+((v34*(v1917+((v1923+v1923)/v1927)))/v1940))))/v1951);
        let v1962=((-(v128*(v1072+((v34*(v1918+((v1925+v1925)/v1927)))/v1940))))/v1951);
        let v1989=(((v368*(v5*v856))-(v367*(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*(self.scalar_static_f64[76]*(self.scalar_static_f64[41]*f64::powf(v15,self.scalar_static_f64[89])))))))/(v368*v368));
        let v1990=((v5*v857)/v368);
        let v1991=((v5*v858)/v368);
        let v1992=(v369*v1989);
        let v1993=(v1992+v1992);
        let v1994=(v369*v1990);
        let v1995=(v1994+v1994);
        let v1996=(v369*v1991);
        let v1997=(v1996+v1996);
        let v1998=(v5*v1989);
        let v1999=(v5*v1990);
        let v2000=(v5*v1991);
        let v2001=(v371*v1907);
        let v2004=((v371*v1908)+(v334*v1998));
        let v2007=((v371*v1909)+(v334*v1999));
        let v2010=((v371*v1910)+(v334*v2000));
        let v2012=(v5*f64::powf(v334,v4));
        let v2034=(v372*v1989);
        let v2036=(v372*v1990);
        let v2038=(v372*v1991);
        let v2047=(v5*v382);
        let v2059=(v383*v383);
        let v2060=(((v383*(v371*(v1907+(v1907*v2012))))-(v376*(v2001+((v379*v2001)/v2047))))/v2059);
        let v2064=(((v383*((v375*v1998)+(v371*(v1908+(v1908*v2012)))))-(v376*((v1989+v2004)+(((v2034+v2034)+(v379*v2004))/v2047))))/v2059);
        let v2068=(((v383*((v375*v1999)+(v371*(v1909+(v1909*v2012)))))-(v376*((v1990+v2007)+(((v2036+v2036)+(v379*v2007))/v2047))))/v2059);
        let v2072=(((v383*((v375*v2000)+(v371*(v1910+(v1910*v2012)))))-(v376*((v1991+v2010)+(((v2038+v2038)+(v379*v2010))/v2047))))/v2059);
        let v2073=(v1907-v2060);
        let v2074=(v1908-v2064);
        let v2075=(v1909-v2068);
        let v2076=(v1910-v2072);
        let v2077=(v385*v2073);
        let v2078=(v2077+v2077);
        let v2079=(v385*v2074);
        let v2080=(v2079+v2079);
        let v2081=(v385*v2075);
        let v2082=(v2081+v2081);
        let v2083=(v385*v2076);
        let v2084=(v2083+v2083);
        let v2154=(v397*v397);
        let v2182=(v5*v406);
        let v2190=(v406*v406);
        let v2204=(v1044-(((v406*((v393*((v5*v2060)+(v2060/v384)))+(v391*(v369*v2073))))-(v394*(((((v397*(v400*v2078))-(v401*(v395*v2073)))/v2154)+(v370*v2078))/v2182)))/v2190));
        let v2208=(((v1045-v817)-(((v406*((v393*((v5*v2064)+(v2064/v384)))+(v391*((v385*v1989)+(v369*v2074)))))-(v394*(((((v397*((v400*v2080)+(v386*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1993))))))-(v401*((v395*v2074)+(v385*(self.scalar_static_f64[43]*v1989)))))/v2154)+((v386*v1993)+(v370*v2080)))/v2182)))/v2190))-v876);
        let v2209=((v1046-(((v406*((v393*((v5*v2068)+(v2068/v384)))+(v391*((v385*v1990)+(v369*v2075)))))-(v394*(((((v397*((v400*v2082)+(v386*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1995))))))-(v401*((v395*v2075)+(v385*(self.scalar_static_f64[43]*v1990)))))/v2154)+((v386*v1995)+(v370*v2082)))/v2182)))/v2190))-v877);
        let v2210=((v1047-(((v406*((v393*((v5*v2072)+(v2072/v384)))+(v391*((v385*v1991)+(v369*v2076)))))-(v394*(((((v397*((v400*v2084)+(v386*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1997))))))-(v401*((v395*v2076)+(v385*(self.scalar_static_f64[43]*v1991)))))/v2154)+((v386*v1997)+(v370*v2084)))/v2182)))/v2190))-v878);
        let v2211=(v411*v2204);
        let v2213=(v411*v2208);
        let v2215=(v411*v2209);
        let v2217=(v411*v2210);
        let v2219=(v5*v414);
        let v2228=(v34*(v2204+((v2211+v2211)/v2219)));
        let v2229=(v34*(v2208+((v2213+v2213)/v2219)));
        let v2230=(v34*(v2209+((v2215+v2215)/v2219)));
        let v2231=(v34*(v2210+((v2217+v2217)/v2219)));
        let v2243=(v422*v422);
        let v2264=(v416*(v379*(((v422*(self.scalar_static_f64[45]*(v379*v2060)))-(v421*v1907))/v2243)));
        let v2265=(v425*v2228);
        let v2267=(v416*v416);
        let v2269=(v416*(v379*(((v422*(self.scalar_static_f64[45]*(v379*v2064)))-(v421*v1908))/v2243)));
        let v2270=(v425*v2229);
        let v2273=(v416*(v379*(((v422*(self.scalar_static_f64[45]*(v379*v2068)))-(v421*v1909))/v2243)));
        let v2274=(v425*v2230);
        let v2277=(v416*(v379*(((v422*(self.scalar_static_f64[45]*(v379*v2072)))-(v421*v1910))/v2243)));
        let v2278=(v425*v2231);
        let v2281=(v5*v428);
        let v2286=(v424*(((v2264-v2265)/v2267)/v2281));
        let v2289=((v428*(v866-v876))+(v424*(((v2269-v2270)/v2267)/v2281)));
        let v2292=((v428*(v868-v877))+(v424*(((v2273-v2274)/v2267)/v2281)));
        let v2295=((v428*(v871-v878))+(v424*(((v2277-v2278)/v2267)/v2281)));
        let v2300=(v430*(v2228+v2286));
        let v2302=(v430*(v2229+v2289));
        let v2304=(v430*(v2230+v2292));
        let v2306=(v430*(v2231+v2295));
        let v2308=(v2264+v2265);
        let v2309=(v2269+v2270);
        let v2310=(v2273+v2274);
        let v2311=(v2277+v2278);
        let v2316=(v5*v434);
        let v2325=(v435*(v2286-v2228));
        let v2327=(v435*(v2289-v2229));
        let v2329=(v435*(v2292-v2230));
        let v2331=(v435*(v2295-v2231));
        let v2337=(v5*v438);
        let v2346=(v34*((((v2300+v2300)+v2308)/v2316)-((v2308+(v2325+v2325))/v2337)));
        let v2350=(v876+(v34*((((v2302+v2302)+v2309)/v2316)-((v2309+(v2327+v2327))/v2337))));
        let v2351=(v877+(v34*((((v2304+v2304)+v2310)/v2316)-((v2310+(v2329+v2329))/v2337))));
        let v2352=(v878+(v34*((((v2306+v2306)+v2311)/v2316)-((v2311+(v2331+v2331))/v2337))));
        let v2366=(v445*(-v2346));
        let v2369=((v446*((self.scalar_static_f64[12]*(v34*v1989))/self.scalar_static_f64[46]))+(v445*(v866-v2350)));
        let v2372=((v446*((self.scalar_static_f64[12]*(v34*v1990))/self.scalar_static_f64[46]))+(v445*(v868-v2351)));
        let v2375=((v446*((self.scalar_static_f64[12]*(v34*v1991))/self.scalar_static_f64[46]))+(v445*(v871-v2352)));
        let v2376=(v447*v2366);
        let v2378=(v447*v2369);
        let v2380=(v447*v2372);
        let v2382=(v447*v2375);
        let v2392=(v5*v459);
        let v2413=(v1044-v2346);
        let v2414=(v1123-v2350);
        let v2415=(v1046-v2351);
        let v2416=(v1047-v2352);
        let v2423=(v472*v472);
        let v2440=((v187*((-(v0*(self.scalar_static_f64[15]*v1952)))/v2423))/v41);
        let v2444=(((v41*((v473*self.scalar_static_f64[84])+(v187*((-(v0*(self.scalar_static_f64[15]*v1956)))/v2423))))-(v474*self.scalar_static_f64[78]))/v802);
        let v2445=((v187*((-(v0*(self.scalar_static_f64[15]*v1959)))/v2423))/v41);
        let v2446=((v187*((-(v0*(self.scalar_static_f64[15]*v1962)))/v2423))/v41);
        let v2451=(v477*v477);
        let v2452=(((v477*v2413)-(v465*v2440))/v2451);
        let v2456=(((v477*v2414)-(v465*(v1115+v2444)))/v2451);
        let v2460=(((v477*v2415)-(v465*v2445))/v2451);
        let v2464=(((v477*v2416)-(v465*v2446))/v2451);
        let v2467=(v479*v479);
        let v2492=(v483*v483);
        let v2503=(if v466{((-(v137*(v483*v2413)))/v2492)}else{v9});
        let v2504=(if v466{((-(v137*(v483*v2414)))/v2492)}else{v9});
        let v2505=(if v466{((-(v137*(v483*v2415)))/v2492)}else{v9});
        let v2506=(if v466{((-(v137*(v483*v2416)))/v2492)}else{v9});
        let v2507=(if v481{((-v2452)/v2467)}else{v2503});
        let v2508=(if v481{((-v2456)/v2467)}else{v2504});
        let v2509=(if v481{((-v2460)/v2467)}else{v2505});
        let v2510=(if v481{((-v2464)/v2467)}else{v2506});
        let v2511=(if v470{((-(v130*v2452))/v2467)}else{v2507});
        let v2512=(if v470{((-(v130*v2456))/v2467)}else{v2508});
        let v2513=(if v470{((-(v130*v2460))/v2467)}else{v2509});
        let v2514=(if v470{((-(v130*v2464))/v2467)}else{v2510});
        let v2516=(v33*f64::powf(v487,v1228));
        let v2532=(v230*f64::powf(v487,v1245));
        let v2550=(v494*v494);
        let v2569=(v33*f64::powf(v486,v1228));
        let v2593=(v230*f64::powf(v486,v1245));
        let v2611=(v504*v504);
        let v2630=(v33*f64::powf(v485,v1228));
        let v2654=(v230*f64::powf(v485,v1245));
        let v2672=(v514*v514);
        let v2686=(if v466{(((v514*((v2440+v2503)+(v214*(v2503*v2630))))-(v509*((v2413+(v2503/v485))-(v207*(v2503*v2654)))))/v2672)}else{v9});
        let v2687=(if v466{(((v514*((v2444+v2504)+((v507*v1159)+(v214*(v2504*v2630)))))-(v509*((v2414+(v2504/v485))-((v512*v1128)+(v207*(v2504*v2654))))))/v2672)}else{v9});
        let v2688=(if v466{(((v514*((v2445+v2505)+(v214*(v2505*v2630))))-(v509*((v2415+(v2505/v485))-(v207*(v2505*v2654)))))/v2672)}else{v9});
        let v2689=(if v466{(((v514*((v2446+v2506)+(v214*(v2506*v2630))))-(v509*((v2416+(v2506/v485))-(v207*(v2506*v2654)))))/v2672)}else{v9});
        let v2690=(if v481{(((v504*((v2440+v2507)+(v214*(v2507*v2569))))-(v499*((v2413+(v2507/v486))-(v207*(v2507*v2593)))))/v2611)}else{v2686});
        let v2691=(if v481{(((v504*((v2444+v2508)+((v497*v1159)+(v214*(v2508*v2569)))))-(v499*((v2414+(v2508/v486))-((v502*v1128)+(v207*(v2508*v2593))))))/v2611)}else{v2687});
        let v2692=(if v481{(((v504*((v2445+v2509)+(v214*(v2509*v2569))))-(v499*((v2415+(v2509/v486))-(v207*(v2509*v2593)))))/v2611)}else{v2688});
        let v2693=(if v481{(((v504*((v2446+v2510)+(v214*(v2510*v2569))))-(v499*((v2416+(v2510/v486))-(v207*(v2510*v2593)))))/v2611)}else{v2689});
        let v2694=(if v470{(((v494*(v2440+(v214*(v2511*v2516))))-(v490*(v2413-(v207*(v2511*v2532)))))/v2550)}else{v2690});
        let v2695=(if v470{(((v494*(v2444+((v488*v1159)+(v214*(v2512*v2516)))))-(v490*(v2414-((v492*v1128)+(v207*(v2512*v2532))))))/v2550)}else{v2691});
        let v2696=(if v470{(((v494*(v2445+(v214*(v2513*v2516))))-(v490*(v2415-(v207*(v2513*v2532)))))/v2550)}else{v2692});
        let v2697=(if v470{(((v494*(v2446+(v214*(v2514*v2516))))-(v490*(v2416-(v207*(v2514*v2532)))))/v2550)}else{v2693});
        let v2699=(v33*f64::powf(v518,v1228));
        let v2715=(v230*f64::powf(v518,v1245));
        let v2733=(v524*v524);
        let v2752=(v33*f64::powf(v517,v1228));
        let v2776=(v230*f64::powf(v517,v1245));
        let v2794=(v534*v534);
        let v2813=(v33*f64::powf(v516,v1228));
        let v2837=(v230*f64::powf(v516,v1245));
        let v2855=(v544*v544);
        let v2869=(if v466{(((v544*((v2440+v2686)+(v214*(v2686*v2813))))-(v539*((v2413+(v2686/v516))-(v207*(v2686*v2837)))))/v2855)}else{v9});
        let v2870=(if v466{(((v544*((v2444+v2687)+((v537*v1159)+(v214*(v2687*v2813)))))-(v539*((v2414+(v2687/v516))-((v542*v1128)+(v207*(v2687*v2837))))))/v2855)}else{v9});
        let v2871=(if v466{(((v544*((v2445+v2688)+(v214*(v2688*v2813))))-(v539*((v2415+(v2688/v516))-(v207*(v2688*v2837)))))/v2855)}else{v9});
        let v2872=(if v466{(((v544*((v2446+v2689)+(v214*(v2689*v2813))))-(v539*((v2416+(v2689/v516))-(v207*(v2689*v2837)))))/v2855)}else{v9});
        let v2873=(if v481{(((v534*((v2440+v2690)+(v214*(v2690*v2752))))-(v529*((v2413+(v2690/v517))-(v207*(v2690*v2776)))))/v2794)}else{v2869});
        let v2874=(if v481{(((v534*((v2444+v2691)+((v527*v1159)+(v214*(v2691*v2752)))))-(v529*((v2414+(v2691/v517))-((v532*v1128)+(v207*(v2691*v2776))))))/v2794)}else{v2870});
        let v2875=(if v481{(((v534*((v2445+v2692)+(v214*(v2692*v2752))))-(v529*((v2415+(v2692/v517))-(v207*(v2692*v2776)))))/v2794)}else{v2871});
        let v2876=(if v481{(((v534*((v2446+v2693)+(v214*(v2693*v2752))))-(v529*((v2416+(v2693/v517))-(v207*(v2693*v2776)))))/v2794)}else{v2872});
        let v2877=(if v470{(((v524*(v2440+(v214*(v2694*v2699))))-(v521*(v2413-(v207*(v2694*v2715)))))/v2733)}else{v2873});
        let v2878=(if v470{(((v524*(v2444+((v519*v1159)+(v214*(v2695*v2699)))))-(v521*(v2414-((v522*v1128)+(v207*(v2695*v2715))))))/v2733)}else{v2874});
        let v2879=(if v470{(((v524*(v2445+(v214*(v2696*v2699))))-(v521*(v2415-(v207*(v2696*v2715)))))/v2733)}else{v2875});
        let v2880=(if v470{(((v524*(v2446+(v214*(v2697*v2699))))-(v521*(v2416-(v207*(v2697*v2715)))))/v2733)}else{v2876});
        let v2882=(v33*f64::powf(v548,v1228));
        let v2898=(v230*f64::powf(v548,v1245));
        let v2916=(v554*v554);
        let v2930=(if v470{(((v554*(v2440+(v214*(v2877*v2882))))-(v551*(v2413-(v207*(v2877*v2898)))))/v2916)}else{v9});
        let v2931=(if v470{(((v554*(v2444+((v549*v1159)+(v214*(v2878*v2882)))))-(v551*(v2414-((v552*v1128)+(v207*(v2878*v2898))))))/v2916)}else{v9});
        let v2932=(if v470{(((v554*(v2445+(v214*(v2879*v2882))))-(v551*(v2415-(v207*(v2879*v2898)))))/v2916)}else{v9});
        let v2933=(if v470{(((v554*(v2446+(v214*(v2880*v2882))))-(v551*(v2416-(v207*(v2880*v2898)))))/v2916)}else{v9});
        let v2935=(v230*f64::powf(v556,v1245));
        let v2951=(v33*f64::powf(v556,v1228));
        let v2969=(v562*v562);
        let v2992=(v230*f64::powf(v547,v1245));
        let v3012=(v33*f64::powf(v547,v1228));
        let v3030=(v572*v572);
        let v3053=(v230*f64::powf(v546,v1245));
        let v3073=(v33*f64::powf(v546,v1228));
        let v3091=(v582*v582);
        let v3105=(if v466{(((v582*((v2413+(v2869/v546))-(v207*(v2869*v3053))))-(v578*((v2440+v2869)+(v214*(v2869*v3073)))))/v3091)}else{v9});
        let v3106=(if v466{(((v582*((v2414+(v2870/v546))-((v576*v1128)+(v207*(v2870*v3053)))))-(v578*((v2444+v2870)+((v580*v1159)+(v214*(v2870*v3073))))))/v3091)}else{v9});
        let v3107=(if v466{(((v582*((v2415+(v2871/v546))-(v207*(v2871*v3053))))-(v578*((v2445+v2871)+(v214*(v2871*v3073)))))/v3091)}else{v9});
        let v3108=(if v466{(((v582*((v2416+(v2872/v546))-(v207*(v2872*v3053))))-(v578*((v2446+v2872)+(v214*(v2872*v3073)))))/v3091)}else{v9});
        let v3109=(if v481{(((v572*((v2413+(v2873/v547))-(v207*(v2873*v2992))))-(v568*((v2440+v2873)+(v214*(v2873*v3012)))))/v3030)}else{v3105});
        let v3110=(if v481{(((v572*((v2414+(v2874/v547))-((v566*v1128)+(v207*(v2874*v2992)))))-(v568*((v2444+v2874)+((v570*v1159)+(v214*(v2874*v3012))))))/v3030)}else{v3106});
        let v3111=(if v481{(((v572*((v2415+(v2875/v547))-(v207*(v2875*v2992))))-(v568*((v2445+v2875)+(v214*(v2875*v3012)))))/v3030)}else{v3107});
        let v3112=(if v481{(((v572*((v2416+(v2876/v547))-(v207*(v2876*v2992))))-(v568*((v2446+v2876)+(v214*(v2876*v3012)))))/v3030)}else{v3108});
        let v3127=(v41*(self.scalar_static_f64[15]*(v5*v1952)));
        let v3130=((v588*self.scalar_static_f64[78])+(v41*(self.scalar_static_f64[15]*(v5*v1956))));
        let v3131=(v41*(self.scalar_static_f64[15]*(v5*v1959)));
        let v3132=(v41*(self.scalar_static_f64[15]*(v5*v1962)));
        let v3136=(v589*v589);
        let v3202=(if v470{(((v589*(v205*(if v470{(((v562*(v2413-(v207*(v2930*v2935))))-(v559*(v2440+(v214*(v2930*v2951)))))/v2969)}else{v3109})))-(v587*v3127))/v3136)}else{(if v481{(((v589*(v205*v3109))-(v591*v3127))/v3136)}else{(if v466{(((v589*(v205*v3105))-(v593*v3127))/v3136)}else{v9})})});
        let v3203=(if v470{(((v589*((v586*self.scalar_static_f64[86])+(v205*(if v470{(((v562*(v2414-((v557*v1128)+(v207*(v2931*v2935)))))-(v559*(v2444+((v560*v1159)+(v214*(v2931*v2951))))))/v2969)}else{v3110}))))-(v587*v3130))/v3136)}else{(if v481{(((v589*((v585*self.scalar_static_f64[86])+(v205*v3110)))-(v591*v3130))/v3136)}else{(if v466{(((v589*((v584*self.scalar_static_f64[86])+(v205*v3106)))-(v593*v3130))/v3136)}else{v9})})});
        let v3204=(if v470{(((v589*(v205*(if v470{(((v562*(v2415-(v207*(v2932*v2935))))-(v559*(v2445+(v214*(v2932*v2951)))))/v2969)}else{v3111})))-(v587*v3131))/v3136)}else{(if v481{(((v589*(v205*v3111))-(v591*v3131))/v3136)}else{(if v466{(((v589*(v205*v3107))-(v593*v3131))/v3136)}else{v9})})});
        let v3205=(if v470{(((v589*(v205*(if v470{(((v562*(v2416-(v207*(v2933*v2935))))-(v559*(v2446+(v214*(v2933*v2951)))))/v2969)}else{v3112})))-(v587*v3132))/v3136)}else{(if v481{(((v589*(v205*v3112))-(v591*v3132))/v3136)}else{(if v466{(((v589*(v205*v3108))-(v593*v3132))/v3136)}else{v9})})});
        let v3210=((v1044-v1907)-v3202);
        let v3211=((v1045-v1908)-v3203);
        let v3212=((v1046-v1909)-v3204);
        let v3213=((v1047-v1910)-v3205);
        let v3214=(v601*v3210);
        let v3216=(v601*v3211);
        let v3218=(v601*v3212);
        let v3220=(v601*v3213);
        let v3222=(v5*v604);
        let v3235=(v5*v607);
        let v3246=(v608*v608);
        let v3247=((-(v128*(v1069+((v34*(v3210+((v3214+v3214)/v3222)))/v3235))))/v3246);
        let v3251=(((v608*v908)-(v128*(v1070+((v34*(v3211+((v3216+v3216)/v3222)))/v3235))))/v3246);
        let v3254=((-(v128*(v1071+((v34*(v3212+((v3218+v3218)/v3222)))/v3235))))/v3246);
        let v3257=((-(v128*(v1072+((v34*(v3213+((v3220+v3220)/v3222)))/v3235))))/v3246);
        let v3258=(v1907-v3202);
        let v3259=(v1908-v3203);
        let v3260=(v1909-v3204);
        let v3261=(v1910-v3205);
        let v3262=(v611*v3258);
        let v3264=(v611*v3259);
        let v3266=(v611*v3260);
        let v3268=(v611*v3261);
        let v3270=(v1907+v3202);
        let v3271=(v1908+v3203);
        let v3272=(v1909+v3204);
        let v3273=(v1910+v3205);
        let v3275=(v613*v613);
        let v3276=((-v3270)/v3275);
        let v3278=((-v3271)/v3275);
        let v3280=((-v3272)/v3275);
        let v3282=((-v3273)/v3275);
        let v3285=((v614*(v3262+v3262))+(v612*v3276));
        let v3288=((v614*(v3264+v3264))+(v612*v3278));
        let v3291=((v614*(v3266+v3266))+(v612*v3280));
        let v3294=((v614*(v3268+v3268))+(v612*v3282));
        let v3319=(v33*v3247);
        let v3320=(v33*v3251);
        let v3321=(v33*v3254);
        let v3322=(v33*v3257);
        let v3325=((v615*v3276)+(v614*v3285));
        let v3328=((v615*v3278)+(v614*v3288));
        let v3331=((v615*v3280)+(v614*v3291));
        let v3334=((v615*v3282)+(v614*v3294));
        let v3435=(v108*((v109-v1044)-((v619*v3247)+(v616*(v3270+(v33*v3285))))));
        let v3438=((v621*v856)+(v108*((v902-v1045)-((v619*v3251)+(v616*(v3271+(v33*v3288)))))));
        let v3441=((v621*v857)+(v108*((v903-v1046)-((v619*v3254)+(v616*(v3272+(v33*v3291)))))));
        let v3444=((v621*v858)+(v108*((v904-v1047)-((v619*v3257)+(v616*(v3273+(v33*v3294)))))));
        let v3445=(v645*v3435);
        let v3447=(v645*v3438);
        let v3449=(v645*v3441);
        let v3451=(v645*v3444);
        let v3453=(v5*v648);
        let v3462=(v34*(v3435+((v3445+v3445)/v3453)));
        let v3463=(v34*(v3438+((v3447+v3447)/v3453)));
        let v3464=(v34*(v3441+((v3449+v3449)/v3453)));
        let v3465=(v34*(v3444+((v3451+v3451)/v3453)));
        let v3470=(v108*(((v633*v3319)+(v622*((v1911+v3202)+((v631*v3325)+(v623*(v34*((v625*v1907)+(v628*v3202))))))))+((v643*v3319)+(v622*((v1907+(v5*v3202))+((v641*v3325)+(v623*(v34*((v628*v1907)+(v625*v3202))))))))));
        let v3473=((v651*v856)+(v108*(((v633*v3320)+(v622*((v1912+v3203)+((v631*v3328)+(v623*(v34*((v625*v1908)+(v628*v3203))))))))+((v643*v3320)+(v622*((v1908+(v5*v3203))+((v641*v3328)+(v623*(v34*((v628*v1908)+(v625*v3203)))))))))));
        let v3476=((v651*v857)+(v108*(((v633*v3321)+(v622*((v1913+v3204)+((v631*v3331)+(v623*(v34*((v625*v1909)+(v628*v3204))))))))+((v643*v3321)+(v622*((v1909+(v5*v3204))+((v641*v3331)+(v623*(v34*((v628*v1909)+(v625*v3204)))))))))));
        let v3479=((v651*v858)+(v108*(((v633*v3322)+(v622*((v1914+v3205)+((v631*v3334)+(v623*(v34*((v625*v1910)+(v628*v3205))))))))+((v643*v3322)+(v622*((v1910+(v5*v3205))+((v641*v3334)+(v623*(v34*((v628*v1910)+(v625*v3205)))))))))));
        let v3495=(v650*v650);
        let v3514=(self.scalar_static_f64[34]*f64::powf(v658,self.scalar_static_f64[87]));
        let v3520=(self.scalar_static_f64[37]*f64::powf(v655,self.scalar_static_f64[88]));
        let v3535=(v659*v659);
        let v3546=((v357*((self.scalar_static_f64[33]*(v3462+(self.scalar_static_f64[8]*v3470)))*v3520))+((-(self.scalar_static_f64[38]*((v34*(((v650*v3470)-(v652*v3462))/v3495))*v3514)))/v3535));
        let v3547=((v357*((self.scalar_static_f64[33]*(v3463+(self.scalar_static_f64[8]*v3473)))*v3520))+((-(self.scalar_static_f64[38]*((v34*(((v650*v3473)-(v652*v3463))/v3495))*v3514)))/v3535));
        let v3548=(((v660*(self.scalar_static_f64[36]*v835))+(v357*((self.scalar_static_f64[33]*(v3464+(self.scalar_static_f64[8]*v3476)))*v3520)))+((-(self.scalar_static_f64[38]*((v34*(((v650*v3476)-(v652*v3464))/v3495))*v3514)))/v3535));
        let v3549=(((v660*(self.scalar_static_f64[36]*v836))+(v357*((self.scalar_static_f64[33]*(v3465+(self.scalar_static_f64[8]*v3479)))*v3520)))+((-(self.scalar_static_f64[38]*((v34*(((v650*v3479)-(v652*v3465))/v3495))*v3514)))/v3535));
        let v3550=(v666*v3546);
        let v3552=(v666*v3547);
        let v3554=(v666*v3548);
        let v3556=(v666*v3549);
        let v3558=(v5*v669);
        let v3567=(v34*(v3546+((v3550+v3550)/v3558)));
        let v3568=(v34*(v3547+((v3552+v3552)/v3558)));
        let v3569=(v34*(v3548+((v3554+v3554)/v3558)));
        let v3570=(v34*(v3549+((v3556+v3556)/v3558)));
        let v3573=(v671*v671);
        let v3593=((v673*v3258)+(v611*(v5*((-(v369*v3567))/v3573))));
        let v3596=((v673*v3259)+(v611*(v5*(((v671*v1989)-(v369*v3568))/v3573))));
        let v3599=((v673*v3260)+(v611*(v5*(((v671*v1990)-(v369*v3569))/v3573))));
        let v3602=((v673*v3261)+(v611*(v5*(((v671*v1991)-(v369*v3570))/v3573))));
        let v3603=(v674*v3593);
        let v3605=(v674*v3596);
        let v3607=(v674*v3599);
        let v3609=(v674*v3602);
        let v3611=(v5*v677);
        let v3612=((v3603+v3603)/v3611);
        let v3613=((v3605+v3605)/v3611);
        let v3614=((v3607+v3607)/v3611);
        let v3615=((v3609+v3609)/v3611);
        let v3653=(v677*v677);
        let v3675=((v689*v3567)+(v671*(if v685{(v34*(v3612+((-v3612)/v3653)))}else{(if v678{(v34*(v3612+((v680*((-v3593)/v675))+(v679*(v3593/v677)))))}else{v9})})));
        let v3678=((v689*v3568)+(v671*(if v685{(v34*(v3613+((-v3613)/v3653)))}else{(if v678{(v34*(v3613+((v680*((-v3596)/v675))+(v679*(v3596/v677)))))}else{v9})})));
        let v3681=((v689*v3569)+(v671*(if v685{(v34*(v3614+((-v3614)/v3653)))}else{(if v678{(v34*(v3614+((v680*((-v3599)/v675))+(v679*(v3599/v677)))))}else{v9})})));
        let v3684=((v689*v3570)+(v671*(if v685{(v34*(v3615+((-v3615)/v3653)))}else{(if v678{(v34*(v3615+((v680*((-v3602)/v675))+(v679*(v3602/v677)))))}else{v9})})));
        let v3687=(v690*v690);
        let v3719=(-(self.scalar_static_f64[51]*(((v2366+(((v2376+v2376)+(self.scalar_static_f64[52]*v2366))/v2392))/self.scalar_static_f64[53])/v462)));
        let v3720=(-(self.scalar_static_f64[51]*(((v2369+(((v2378+v2378)+(self.scalar_static_f64[52]*v2369))/v2392))/self.scalar_static_f64[53])/v462)));
        let v3721=(-(self.scalar_static_f64[51]*(((v2372+(((v2380+v2380)+(self.scalar_static_f64[52]*v2372))/v2392))/self.scalar_static_f64[53])/v462)));
        let v3722=(-(self.scalar_static_f64[51]*(((v2375+(((v2382+v2382)+(self.scalar_static_f64[52]*v2375))/v2392))/self.scalar_static_f64[53])/v462)));
        let v3726=(v697*v697);
        let v3778=((v702*(v108*(v108*(self.scalar_static_f64[15]*(((v697*(self.scalar_static_f64[11]*((v694*((-(v39*v3675))/v3687))+(v691*(self.scalar_static_f64[55]*v3247)))))-(v696*v3719))/v3726)))))+(v701*((v613*v3258)+(v611*v3270))));
        let v3781=((v702*((v700*v856)+(v108*((v699*v856)+(v108*(self.scalar_static_f64[15]*(((v697*(self.scalar_static_f64[11]*((v694*(((v690*(self.scalar_static_f64[9]*(self.scalar_static_f64[76]*(self.scalar_static_f64[10]*f64::powf(v15,self.scalar_static_f64[77])))))-(v39*v3678))/v3687))+(v691*(self.scalar_static_f64[55]*v3251)))))-(v696*v3720))/v3726)))))))+(v701*((v613*v3259)+(v611*v3271))));
        let v3784=((v702*((v700*v857)+(v108*((v699*v857)+(v108*(self.scalar_static_f64[15]*(((v697*(self.scalar_static_f64[11]*((v694*((-(v39*v3681))/v3687))+(v691*(self.scalar_static_f64[55]*v3254)))))-(v696*v3721))/v3726)))))))+(v701*((v613*v3260)+(v611*v3272))));
        let v3787=((v702*((v700*v858)+(v108*((v699*v858)+(v108*(self.scalar_static_f64[15]*(((v697*(self.scalar_static_f64[11]*((v694*((-(v39*v3684))/v3687))+(v691*(self.scalar_static_f64[55]*v3257)))))-(v696*v3722))/v3726)))))))+(v701*((v613*v3261)+(v611*v3273))));
        let v3800=(self.scalar_static_f64[63]*(self.scalar_static_f64[60]*(self.scalar_static_f64[76]*(self.scalar_static_f64[61]*f64::powf(v15,self.scalar_static_f64[93])))));
        let v3803=(v720*v720);
        let v3829=(self.scalar_static_f64[66]*f64::powf(v730,self.scalar_static_f64[94]));
        let v3840=(self.scalar_static_f64[67]*f64::powf(v733,self.scalar_static_f64[95]));
        let v3841=((-((if v729{v9}else{(v3778/v726)})*v3829))*v3840);
        let v3842=((-((if v729{v9}else{(((v726*v3781)-(v703*(self.scalar_static_f64[63]*(self.scalar_static_f64[57]*(self.scalar_static_f64[91]*(self.scalar_static_f64[59]*f64::powf(v709,self.scalar_static_f64[92])))))))/(v726*v726))})*v3829))*v3840);
        let v3843=((-((if v729{v9}else{(v3784/v726)})*v3829))*v3840);
        let v3844=((-((if v729{v9}else{(v3787/v726)})*v3829))*v3840);
        let v3847=(v735*v735);
        let v3848=((-(v722*v3841))/v3847);
        let v3855=((-(v722*v3843))/v3847);
        let v3858=((-(v722*v3844))/v3847);
        let v3861=((-(v725*v3841))/v3847);
        let v3868=((-(v725*v3843))/v3847);
        let v3871=((-(v725*v3844))/v3847);
        let v3873=(self.scalar_static_f64[76]*f64::powf(v15,v9));
        let v3876=((((v735*((v721*self.scalar_static_f64[91])+(v709*((-(self.scalar_static_f64[64]*v3800))/v3803))))-(v722*v3842))/v3847)+(self.scalar_static_f64[71]*(self.scalar_static_f64[72]*v3873)));
        let v3879=((((v735*((v724*self.scalar_static_f64[91])+(v709*((-(self.scalar_static_f64[65]*v3800))/v3803))))-(v725*v3842))/v3847)+(self.scalar_static_f64[69]*(self.scalar_static_f64[73]*v3873)));
        let v3947=(v760*v760);
        let v3948=(((v760*v3778)-(v703*((v758*((v756*v3470)+(v652*(((v697*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3675))/v3687))))-(v755*v3719))/v3726))))+(v757*(v3848+v3861)))))/v3947);
        let v3952=(((v760*v3781)-(v703*((v758*((v756*v3473)+(v652*(((v697*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3678))/v3687))))-(v755*v3720))/v3726))))+(v757*(v3876+v3879)))))/v3947);
        let v3956=(((v760*v3784)-(v703*((v758*((v756*v3476)+(v652*(((v697*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3681))/v3687))))-(v755*v3721))/v3726))))+(v757*(v3855+v3868)))))/v3947);
        let v3960=(((v760*v3787)-(v703*((v758*((v756*v3479)+(v652*(((v697*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3684))/v3687))))-(v755*v3722))/v3726))))+(v757*(v3858+v3871)))))/v3947);
        let v3992=(v761*v3948);
        let v3993=(v3992+v3992);
        let v3994=(v761*v3952);
        let v3995=(v3994+v3994);
        let v3996=(v761*v3956);
        let v3997=(v3996+v3996);
        let v3998=(v761*v3960);
        let v3999=(v3998+v3998);

        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            (v752*v761),
            [1, 4, 5, 6],
            [((v761*v3861)+(v752*v3948)), ((v761*v3879)+(v752*v3952)), ((v761*v3868)+(v752*v3956)), ((v761*v3871)+(v752*v3960))],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v761),
            [1, 4, 5, 6],
            [v3948, v3952, v3956, v3960],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 0>(
            1,
            (v747*v761),
            [1, 4, 5, 6],
            [((v761*v3848)+(v747*v3948)), ((v761*v3876)+(v747*v3952)), ((v761*v3855)+(v747*v3956)), ((v761*v3858)+(v747*v3960))],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v9,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{v776}else{v9})),
            4,
            multiplicity * ((if self.scalar_static_bool[1]{(self.scalar_static_f64[75]*ddt_scale)}else{v9})),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{(if self.scalar_static_bool[1]{(-(((v65*v761)+(v747*v767))+(v752*v767)))}else{v9})}else{v9})),
            [1, 4, 5, 6],
            [(if self.scalar_static_bool[1]{(if self.scalar_static_bool[1]{(-(((v65*v3948)+((v767*v3848)+(v747*v3993)))+((v767*v3861)+(v752*v3993))))}else{v9})}else{v9}), (if self.scalar_static_bool[1]{(if self.scalar_static_bool[1]{(-(((v65*v3952)+((v767*v3876)+(v747*v3995)))+((v767*v3879)+(v752*v3995))))}else{v9})}else{v9}), (if self.scalar_static_bool[1]{(if self.scalar_static_bool[1]{(-(((v761+(v65*v3956))+((v767*v3855)+(v747*v3997)))+((v767*v3868)+(v752*v3997))))}else{v9})}else{v9}), (if self.scalar_static_bool[1]{(if self.scalar_static_bool[1]{(-((((-v761)+(v65*v3960))+((v767*v3858)+(v747*v3999)))+((v767*v3871)+(v752*v3999))))}else{v9})}else{v9})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{(v12/self.scalar_static_f64[74])}else{v9})),
            4,
            multiplicity * (self.scalar_static_f64[97]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[2]{(v12*1000000000.0)}else{v9})),
            4,
            multiplicity * (self.scalar_static_f64[98]),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v9=0.0;
        let v776=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if self.scalar_static_bool[1]{(self.scalar_static_f64[75]*1.0)}else{v9})),
        );
    }
}
