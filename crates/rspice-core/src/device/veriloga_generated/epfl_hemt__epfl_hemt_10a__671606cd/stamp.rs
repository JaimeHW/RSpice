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
        let v71=(if (v68>80.0){v4}else{v9});
        let v73=(!(v71!=0.0));
        let v74=(v68).exp();
        let v75=(v4+v74);
        let v83=((((if v73{(v75).ln()}else{(if (v71!=0.0){v68}else{v9})})*self.scalar_static_f64[17])-v66)-self.scalar_static_f64[18]);
        let v87=(-(v63+(v34*(v66-v83))));
        let v100=(v4+(((self.scalar_static_f64[21]+(v83*self.scalar_static_f64[22]))-(v87*self.scalar_static_f64[23]))/self.scalar_static_f64[15]));
        let v102=(v100-v4);
        let v106=(((v102*v102)+0.0006250000000000001)).sqrt();
        let v108=(v34*((v4+v100)+v106));
        let v109=(v41*v108);
        let v110=(v4/v109);
        let v111=(v65*v110);
        let v113=(v63*v110);
        let v120=(-(self.scalar_static_f64[25]+(v87*self.scalar_static_f64[26])));
        let v121=(v83*v120);
        let v124=(((v62*v110)-(v110*self.scalar_static_f64[24]))-(v110*v121));
        let v128=(((v4/v41)*self.scalar_static_f64[27])).sqrt();
        let v129=(v128/self.scalar_static_f64[15]);
        let v131=3.0;
        let v132=1.4142135623730951;
        let v136=((v34*v124)-(v131*(v4+(v129/v132))));
        let v138=6.0;
        let v141=(((v136*v136)+(v124*v138))).sqrt();
        let v142=(v136+v141);
        let v144=(if (v124<v9){v4}else{v9});
        let v145=(v124-v142);
        let v147=(if (v144!=0.0){(v145/v129)}else{v9});
        let v150=((v4-v142)+(v147*v147));
        let v151=(v150>v54);
        let v152=(if v151{v150}else{v54});
        let v156=(!(v144!=0.0));
        let v157=(-v142);
        let v159=(if v156{scalar_limited_exp(v157)}else{v147});
        let v161=(if v156{(v34*v129)}else{v136});
        let v166=(((v159+(v124-v4))+(v161*v161))).sqrt();
        let v168=(if v156{(v166-v161)}else{v142});
        let v172=(if v156{((v4+(v168*v168))-v159)}else{(if (v144!=0.0){(-(v152).ln())}else{v9})});
        let v174=(v172-v4);
        let v177=((v4+(v174*v174))).sqrt();
        let v180=((v34*((v4+v172)+v177))).sqrt();
        let v181=(v5*v180);
        let v183=(v4+(v129/v181));
        let v185=(self.scalar_static_f64[15]*(v5*v183));
        let v186=(v41*v185);
        let v188=(self.scalar_static_f64[28]/v41);
        let v189=(8.353992494899963e17*v41);
        let v190=0.6666;
        let v191=f64::powf(v189,v190);
        let v192=(v188*v191);
        let v195=(v0*v41);
        let v199=(v57+(v172-(self.scalar_static_f64[30]+(v24/v195))));
        let v200=(v199-v113);
        let v201=(v200<=v9);
        let v202=(!v201);
        let v203=50.0;
        let v204=(v200<v203);
        let v206=(v202&&(!v204));
        let v207=(v0*v189);
        let v208=(v4+v200);
        let v209=(v33*v192);
        let v210=(self.scalar_static_f64[15]*v183);
        let v211=(v0/v210);
        let v212=(v189*v211);
        let v213=(v212/v41);
        let v214=(v4+v213);
        let v215=0.6666666666666666;
        let v216=(v192*v215);
        let v217=(v192+v213);
        let v219=(v34+(v200/v217));
        let v221=(v202&&v204);
        let v223=(v200).exp();
        let v225=(if v201{(v138/v223)}else{v9});
        let v226=(if v221{(v4/v219)}else{v225});
        let v227=(if v206{(v131/v219)}else{v226});
        let v228=f64::powf(v227,v33);
        let v230=(v214+(v216*v228));
        let v231=(v34+v200);
        let v232=-0.6666666666666666;
        let v233=f64::powf(v227,v232);
        let v235=(v231-(v209*v233));
        let v238=f64::powf(v226,v33);
        let v240=((v213+v226)+(v216*v238));
        let v243=f64::powf(v226,v232);
        let v245=((v208+(v226).ln())-(v209*v243));
        let v248=f64::powf(v225,v33);
        let v250=((v213+v225)+(v216*v248));
        let v253=f64::powf(v225,v232);
        let v255=((v208+(v225).ln())-(v209*v253));
        let v257=(if v201{(v250/v255)}else{v9});
        let v258=(if v221{(v240/v245)}else{v257});
        let v259=(if v206{(v230/v235)}else{v258});
        let v260=f64::powf(v259,v33);
        let v262=(v214+(v216*v260));
        let v263=f64::powf(v259,v232);
        let v265=(v231-(v209*v263));
        let v268=f64::powf(v258,v33);
        let v270=((v213+v258)+(v216*v268));
        let v273=f64::powf(v258,v232);
        let v275=((v208+(v258).ln())-(v209*v273));
        let v278=f64::powf(v257,v33);
        let v280=((v213+v257)+(v216*v278));
        let v283=f64::powf(v257,v232);
        let v285=((v208+(v257).ln())-(v209*v283));
        let v287=(if v201{(v280/v285)}else{v9});
        let v288=(if v221{(v270/v275)}else{v287});
        let v289=(if v206{(v262/v265)}else{v288});
        let v290=f64::powf(v289,v33);
        let v292=(v214+(v216*v290));
        let v293=f64::powf(v289,v232);
        let v295=(v231-(v209*v293));
        let v297=(if v206{(v292/v295)}else{v9});
        let v298=f64::powf(v297,v232);
        let v300=(v208-(v209*v298));
        let v301=f64::powf(v297,v33);
        let v303=(v214+(v216*v301));
        let v307=f64::powf(v288,v232);
        let v309=((v208+(v288).ln())-(v209*v307));
        let v311=f64::powf(v288,v33);
        let v313=((v213+v288)+(v216*v311));
        let v317=f64::powf(v287,v232);
        let v319=((v208+(v287).ln())-(v209*v317));
        let v321=f64::powf(v287,v33);
        let v323=((v213+v287)+(v216*v321));
        let v325=(if v201{(v319/v323)}else{v9});
        let v326=(if v221{(v309/v313)}else{v325});
        let v327=(if v206{(v300/v303)}else{v326});
        let v328=(v207*v327);
        let v330=(v207*v326);
        let v332=(v207*v325);
        let v336=(if v206{(v328/v186)}else{(if v221{(v330/v186)}else{(if v201{(v332/v186)}else{v9})})});
        let v337=(v5*v336);
        let v338=(v172-v337);
        let v340=(v338-v4);
        let v343=((v4+(v340*v340))).sqrt();
        let v346=((v34*((v4+v338)+v343))).sqrt();
        let v347=(v180+v346);
        let v349=(v4+(v129/v347));
        let v359=(self.scalar_static_f64[35]+(v87*self.scalar_static_f64[36]));
        let v369=(v5*v109);
        let v370=(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*f64::powf(v15,self.scalar_static_f64[41])));
        let v371=(v369/v370);
        let v372=(v371*v371);
        let v373=(v5*v371);
        let v374=(v5+v371);
        let v375=(v336*v373);
        let v377=(v336+f64::powf(v336,v5));
        let v378=(v373*v377);
        let v381=4.0;
        let v384=(((v374*v374)+(v375*v381))).sqrt();
        let v385=((v374+v375)+v384);
        let v386=(v378/v385);
        let v387=(v336-v386);
        let v388=(v387*v387);
        let v393=((v5*v386)+(v386).ln());
        let v395=(v4+(v371*v387));
        let v396=(v393*v395);
        let v397=(v371*self.scalar_static_f64[43]);
        let v399=(0.1+(v387*v397));
        let v402=(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v372)));
        let v403=(v388*v402);
        let v408=(((v4+(v403/v399))+(v372*v388))).sqrt();
        let v411=(((v172-v57)-(v396/v408))-v113);
        let v413=(v411-v131);
        let v416=((v381+(v413*v413))).sqrt();
        let v418=(v34*((v131+v411)+v416));
        let v423=(self.scalar_static_f64[45]*(self.scalar_static_f64[42]+(v381*v386)));
        let v424=(v4+v336);
        let v426=(v111-v113);
        let v427=(v381*(v423/v424));
        let v430=((v4+(v427/v418))).sqrt();
        let v431=(v426*v430);
        let v432=(v418+v431);
        let v434=(v418*v427);
        let v436=(((v432*v432)+v434)).sqrt();
        let v437=(v431-v418);
        let v440=((v434+(v437*v437))).sqrt();
        let v443=(v113+(v34*(v436-v440)));
        let v447=((self.scalar_static_f64[12]*(v34*v371))/self.scalar_static_f64[46]);
        let v448=(v111-v443);
        let v449=(v447*v448);
        let v461=((v4+((v449*v449)+(v449*self.scalar_static_f64[52])))).sqrt();
        let v464=(((v449+self.scalar_static_f64[49])+v461)/self.scalar_static_f64[53]);
        let v467=(v199-v443);
        let v468=(v467<=v9);
        let v469=(!v468);
        let v470=(v467<v203);
        let v472=(v469&&(!v470));
        let v473=(v4+v467);
        let v474=(self.scalar_static_f64[15]*v349);
        let v475=(v0/v474);
        let v476=(v189*v475);
        let v477=(v476/v41);
        let v478=(v4+v477);
        let v479=(v192+v477);
        let v481=(v34+(v467/v479));
        let v483=(v469&&v470);
        let v485=(v467).exp();
        let v487=(if v468{(v138/v485)}else{v9});
        let v488=(if v483{(v4/v481)}else{v487});
        let v489=(if v472{(v131/v481)}else{v488});
        let v490=f64::powf(v489,v33);
        let v492=(v478+(v216*v490));
        let v493=(v34+v467);
        let v494=f64::powf(v489,v232);
        let v496=(v493-(v209*v494));
        let v499=f64::powf(v488,v33);
        let v501=((v477+v488)+(v216*v499));
        let v504=f64::powf(v488,v232);
        let v506=((v473+(v488).ln())-(v209*v504));
        let v509=f64::powf(v487,v33);
        let v511=((v477+v487)+(v216*v509));
        let v514=f64::powf(v487,v232);
        let v516=((v473+(v487).ln())-(v209*v514));
        let v518=(if v468{(v511/v516)}else{v9});
        let v519=(if v483{(v501/v506)}else{v518});
        let v520=(if v472{(v492/v496)}else{v519});
        let v521=f64::powf(v520,v33);
        let v523=(v478+(v216*v521));
        let v524=f64::powf(v520,v232);
        let v526=(v493-(v209*v524));
        let v529=f64::powf(v519,v33);
        let v531=((v477+v519)+(v216*v529));
        let v534=f64::powf(v519,v232);
        let v536=((v473+(v519).ln())-(v209*v534));
        let v539=f64::powf(v518,v33);
        let v541=((v477+v518)+(v216*v539));
        let v544=f64::powf(v518,v232);
        let v546=((v473+(v518).ln())-(v209*v544));
        let v548=(if v468{(v541/v546)}else{v9});
        let v549=(if v483{(v531/v536)}else{v548});
        let v550=(if v472{(v523/v526)}else{v549});
        let v551=f64::powf(v550,v33);
        let v553=(v478+(v216*v551));
        let v554=f64::powf(v550,v232);
        let v556=(v493-(v209*v554));
        let v558=(if v472{(v553/v556)}else{v9});
        let v559=f64::powf(v558,v232);
        let v561=(v473-(v209*v559));
        let v562=f64::powf(v558,v33);
        let v564=(v478+(v216*v562));
        let v568=f64::powf(v549,v232);
        let v570=((v473+(v549).ln())-(v209*v568));
        let v572=f64::powf(v549,v33);
        let v574=((v477+v549)+(v216*v572));
        let v578=f64::powf(v548,v232);
        let v580=((v473+(v548).ln())-(v209*v578));
        let v582=f64::powf(v548,v33);
        let v584=((v477+v548)+(v216*v582));
        let v586=(if v468{(v580/v584)}else{v9});
        let v587=(if v483{(v570/v574)}else{v586});
        let v588=(if v472{(v561/v564)}else{v587});
        let v589=(v207*v588);
        let v590=(self.scalar_static_f64[15]*(v5*v349));
        let v591=(v41*v590);
        let v593=(v207*v587);
        let v595=(v207*v586);
        let v599=(if v472{(v589/v591)}else{(if v483{(v593/v591)}else{(if v468{(v595/v591)}else{v9})})});
        let v601=((v172-v336)-v599);
        let v603=(v601-v4);
        let v606=((v4+(v603*v603))).sqrt();
        let v609=((v34*((v4+v601)+v606))).sqrt();
        let v610=(v180+v609);
        let v612=(v4+(v129/v610));
        let v613=(v336-v599);
        let v614=(v613*v613);
        let v615=(v424+v599);
        let v616=(v4/v615);
        let v617=(v614*v616);
        let v618=(v612-v4);
        let v621=((v336+v599)+(v33*v617));
        let v623=((v124-v172)-(v618*v621));
        let v624=(v33*v612);
        let v625=(v616*v617);
        let v627=0.8;
        let v630=1.2;
        let v633=(v34*((v4+(v336*v627))+(v599*v630)));
        let v635=((v337+v599)+(v625*v633));
        let v643=(v34*((v4+(v336*v630))+(v599*v627)));
        let v645=((v336+(v5*v599))+(v625*v643));
        let v647=(v109*v623);
        let v650=((0.0025000000000000005+(v647*v647))).sqrt();
        let v652=(v34*(v647+v650));
        let v653=((v624*v635)+(v624*v645));
        let v654=(v109*v653);
        let v657=(self.scalar_static_f64[33]*(v652+(self.scalar_static_f64[8]*v654)));
        let v660=(v34*(v4+(v654/v652)));
        let v661=f64::powf(v660,self.scalar_static_f64[34]);
        let v662=f64::powf(v657,self.scalar_static_f64[37]);
        let v666=(v4+((v359*v662)+(self.scalar_static_f64[38]/v661)));
        let v668=(v666-v4);
        let v671=((5.625e-7+(v668*v668))).sqrt();
        let v673=(v34*((v4+v666)+v671));
        let v675=(v5*(v371/v673));
        let v676=(v613*v675);
        let v677=(v676*v676);
        let v679=((v4+v677)).sqrt();
        let v681=(if (v9!=v676){v4}else{v9});
        let v682=(v4/v676);
        let v683=(v676).asinh();
        let v688=(!(v681!=0.0));
        let v692=(if v688{(v34*(v679+(v4/v679)))}else{(if (v681!=0.0){(v34*(v679+(v682*v683)))}else{v9})});
        let v693=(v673*v692);
        let v694=(v39/v693);
        let v697=(v612*self.scalar_static_f64[55]);
        let v699=(self.scalar_static_f64[11]*(v694*v697));
        let v700=(self.scalar_static_f64[12]-(self.scalar_static_f64[51]*(v464).ln()));
        let v702=(self.scalar_static_f64[15]*(v699/v700));
        let v703=(v109*v702);
        let v704=(v109*v703);
        let v705=(v613*v615);
        let v706=(v704*v705);
        let v712=(v4-(self.scalar_static_f64[58]*(v15-v4)));
        let v723=((self.scalar_static_f64[60]*f64::powf(v15,self.scalar_static_f64[61]))*self.scalar_static_f64[63]);
        let v724=(self.scalar_static_f64[64]/v723);
        let v725=(v712*v724);
        let v727=(self.scalar_static_f64[65]/v723);
        let v728=(v712*v727);
        let v729=((self.scalar_static_f64[57]*f64::powf(v712,self.scalar_static_f64[59]))*self.scalar_static_f64[63]);
        let v730=(v706/v729);
        let v731=0.96;
        let v733=(if (v730>=v731){v4}else{v9});
        let v734=(if (v733!=0.0){v731}else{v730});
        let v737=(v4-f64::powf(v734,self.scalar_static_f64[66]));
        let v739=f64::powf(v737,self.scalar_static_f64[67]);
        let v747=f64::powf(v15,v4);
        let v751=((v725/v739)+(self.scalar_static_f64[71]*(v4+(self.scalar_static_f64[72]*v747))));
        let v756=((v728/v739)+(self.scalar_static_f64[69]*(v4+(v747*self.scalar_static_f64[73]))));
        let v759=(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*(self.scalar_static_f64[9]/v693)));
        let v760=(v759/v700);
        let v761=(v654*v760);
        let v762=(v751+v756);
        let v764=(v4+(v761*v762));
        let v765=(v706/v764);
        let v772=(v765*v765);
        let v781=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (v12*self.scalar_static_f64[76]));
        let v798=(v0*(((v21*(v17*(v5*f64::powf(v13,v4))))-v19)/(v21*v21)));
        let v807=(v41*v41);
        let v822=((if v55{((-(self.scalar_static_f64[13]*(v45*(v50*(((v48*v798)-(v46*self.scalar_static_f64[81]))/(v48*v48))))))/(v51*v51))}else{v9})/v56);
        let v834=((self.scalar_static_f64[17]*(if v73{((self.scalar_static_f64[16]*v74)/v75)}else{(if (v71!=0.0){self.scalar_static_f64[16]}else{v9})}))-v4);
        let v835=((self.scalar_static_f64[17]*(if v73{((v74*self.scalar_static_f64[82])/v75)}else{(if (v71!=0.0){self.scalar_static_f64[82]}else{v9})}))-v10);
        let v840=(-(v34*(v4-v834)));
        let v841=(-(v34*(v10-v835)));
        let v848=(((self.scalar_static_f64[22]*v834)-(self.scalar_static_f64[23]*v840))/self.scalar_static_f64[15]);
        let v849=(((self.scalar_static_f64[22]*v835)-(self.scalar_static_f64[23]*v841))/self.scalar_static_f64[15]);
        let v850=(v102*v848);
        let v852=(v102*v849);
        let v854=(v5*v106);
        let v861=(v108*self.scalar_static_f64[79]);
        let v862=(v41*(v34*(v848+((v850+v850)/v854))));
        let v863=(v41*(v34*(v849+((v852+v852)/v854))));
        let v865=(v109*v109);
        let v866=((-v861)/v865);
        let v868=((-v862)/v865);
        let v870=((-v863)/v865);
        let v871=(v65*v866);
        let v873=(v110+(v65*v868));
        let v874=(-v110);
        let v876=(v874+(v65*v870));
        let v881=(v63*v866);
        let v882=(v63*v868);
        let v883=(v63*v870);
        let v907=(((v62*v866)-(self.scalar_static_f64[24]*v866))-(v121*v866));
        let v908=(((v62*v868)-(self.scalar_static_f64[24]*v868))-((v121*v868)+(v110*((v120*v834)+(v83*(-(self.scalar_static_f64[26]*v840)))))));
        let v909=(((v874+(v62*v870))-(self.scalar_static_f64[24]*v870))-((v121*v870)+(v110*((v120*v835)+(v83*(-(self.scalar_static_f64[26]*v841)))))));
        let v913=(((self.scalar_static_f64[27]*(self.scalar_static_f64[80]/v807))/(v5*v128))/self.scalar_static_f64[15]);
        let v915=(v34*v110);
        let v917=(v34*v908);
        let v918=(v34*v909);
        let v921=((v34*v907)-(v131*(v913/v132)));
        let v922=(v136*v915);
        let v924=(v136*v921);
        let v926=(v136*v917);
        let v928=(v136*v918);
        let v938=(v5*v141);
        let v943=(v915+(((v922+v922)+(v110*v138))/v938));
        let v944=(v921+(((v924+v924)+(v138*v907))/v938));
        let v945=(v917+(((v926+v926)+(v138*v908))/v938));
        let v946=(v918+(((v928+v928)+(v138*v909))/v938));
        let v958=(if (v144!=0.0){((v110-v943)/v129)}else{v9});
        let v959=(if (v144!=0.0){(((v129*(v907-v944))-(v145*v913))/(v129*v129))}else{v9});
        let v960=(if (v144!=0.0){((v908-v945)/v129)}else{v9});
        let v961=(if (v144!=0.0){((v909-v946)/v129)}else{v9});
        let v962=(-v943);
        let v963=(-v944);
        let v964=(-v945);
        let v965=(-v946);
        let v966=(v147*v958);
        let v968=(v147*v959);
        let v970=(v147*v960);
        let v972=(v147*v961);
        let v994=scalar_limited_exp_derivative(v157);
        let v999=(if v156{(v962*v994)}else{v958});
        let v1000=(if v156{(v963*v994)}else{v959});
        let v1001=(if v156{(v964*v994)}else{v960});
        let v1002=(if v156{(v965*v994)}else{v961});
        let v1004=(if v156{v9}else{v915});
        let v1005=(if v156{(v34*v913)}else{v921});
        let v1006=(if v156{v9}else{v917});
        let v1007=(if v156{v9}else{v918});
        let v1012=(v161*v1004);
        let v1014=(v161*v1005);
        let v1016=(v161*v1006);
        let v1018=(v161*v1007);
        let v1024=(v5*v166);
        let v1037=(v168*(if v156{((((v110+v999)+(v1012+v1012))/v1024)-v1004)}else{v943}));
        let v1039=(v168*(if v156{((((v907+v1000)+(v1014+v1014))/v1024)-v1005)}else{v944}));
        let v1041=(v168*(if v156{((((v908+v1001)+(v1016+v1016))/v1024)-v1006)}else{v945}));
        let v1043=(v168*(if v156{((((v909+v1002)+(v1018+v1018))/v1024)-v1007)}else{v946}));
        let v1049=(if v156{((v1037+v1037)-v999)}else{(if (v144!=0.0){(-((if v151{(v962+(v966+v966))}else{v9})/v152))}else{v9})});
        let v1050=(if v156{((v1039+v1039)-v1000)}else{(if (v144!=0.0){(-((if v151{(v963+(v968+v968))}else{v9})/v152))}else{v9})});
        let v1051=(if v156{((v1041+v1041)-v1001)}else{(if (v144!=0.0){(-((if v151{(v964+(v970+v970))}else{v9})/v152))}else{v9})});
        let v1052=(if v156{((v1043+v1043)-v1002)}else{(if (v144!=0.0){(-((if v151{(v965+(v972+v972))}else{v9})/v152))}else{v9})});
        let v1053=(v174*v1049);
        let v1055=(v174*v1050);
        let v1057=(v174*v1051);
        let v1059=(v174*v1052);
        let v1061=(v5*v177);
        let v1074=((v34*(v1049+((v1053+v1053)/v1061)))/v181);
        let v1075=((v34*(v1050+((v1055+v1055)/v1061)))/v181);
        let v1076=((v34*(v1051+((v1057+v1057)/v1061)))/v181);
        let v1077=((v34*(v1052+((v1059+v1059)/v1061)))/v181);
        let v1084=(v181*v181);
        let v1085=((-(v129*(v5*v1074)))/v1084);
        let v1089=(((v181*v913)-(v129*(v5*v1075)))/v1084);
        let v1092=((-(v129*(v5*v1076)))/v1084);
        let v1095=((-(v129*(v5*v1077)))/v1084);
        let v1104=(v41*(self.scalar_static_f64[15]*(v5*v1085)));
        let v1107=((v185*self.scalar_static_f64[79])+(v41*(self.scalar_static_f64[15]*(v5*v1089))));
        let v1108=(v41*(self.scalar_static_f64[15]*(v5*v1092)));
        let v1109=(v41*(self.scalar_static_f64[15]*(v5*v1095)));
        let v1120=((v191*(self.scalar_static_f64[84]/v807))+(v188*(self.scalar_static_f64[85]*(v190*f64::powf(v189,-0.33340000000000003)))));
        let v1128=(v822+(v1050-(((v195*(-v798))-(v24*self.scalar_static_f64[86]))/(v195*v195))));
        let v1129=(v1128-v881);
        let v1130=(v1051-v882);
        let v1131=(v1052-v883);
        let v1133=(v33*v1120);
        let v1140=(v210*v210);
        let v1157=((v189*((-(v0*(self.scalar_static_f64[15]*v1085)))/v1140))/v41);
        let v1161=(((v41*((v211*self.scalar_static_f64[85])+(v189*((-(v0*(self.scalar_static_f64[15]*v1089)))/v1140))))-(v212*self.scalar_static_f64[79]))/v807);
        let v1162=((v189*((-(v0*(self.scalar_static_f64[15]*v1092)))/v1140))/v41);
        let v1163=((v189*((-(v0*(self.scalar_static_f64[15]*v1095)))/v1140))/v41);
        let v1164=(v215*v1120);
        let v1169=(v217*v217);
        let v1170=(((v217*v1049)-(v200*v1157))/v1169);
        let v1174=(((v217*v1129)-(v200*(v1120+v1161)))/v1169);
        let v1178=(((v217*v1130)-(v200*v1162))/v1169);
        let v1182=(((v217*v1131)-(v200*v1163))/v1169);
        let v1185=(v219*v219);
        let v1210=(v223*v223);
        let v1221=(if v201{((-(v138*(v223*v1049)))/v1210)}else{v9});
        let v1222=(if v201{((-(v138*(v223*v1129)))/v1210)}else{v9});
        let v1223=(if v201{((-(v138*(v223*v1130)))/v1210)}else{v9});
        let v1224=(if v201{((-(v138*(v223*v1131)))/v1210)}else{v9});
        let v1225=(if v221{((-v1170)/v1185)}else{v1221});
        let v1226=(if v221{((-v1174)/v1185)}else{v1222});
        let v1227=(if v221{((-v1178)/v1185)}else{v1223});
        let v1228=(if v221{((-v1182)/v1185)}else{v1224});
        let v1229=(if v206{((-(v131*v1170))/v1185)}else{v1225});
        let v1230=(if v206{((-(v131*v1174))/v1185)}else{v1226});
        let v1231=(if v206{((-(v131*v1178))/v1185)}else{v1227});
        let v1232=(if v206{((-(v131*v1182))/v1185)}else{v1228});
        let v1233=-0.6666666666666667;
        let v1235=(v33*f64::powf(v227,v1233));
        let v1250=-1.6666666666666665;
        let v1252=(v232*f64::powf(v227,v1250));
        let v1270=(v235*v235);
        let v1289=(v33*f64::powf(v226,v1233));
        let v1313=(v232*f64::powf(v226,v1250));
        let v1331=(v245*v245);
        let v1350=(v33*f64::powf(v225,v1233));
        let v1374=(v232*f64::powf(v225,v1250));
        let v1392=(v255*v255);
        let v1406=(if v201{(((v255*((v1157+v1221)+(v216*(v1221*v1350))))-(v250*((v1049+(v1221/v225))-(v209*(v1221*v1374)))))/v1392)}else{v9});
        let v1407=(if v201{(((v255*((v1161+v1222)+((v248*v1164)+(v216*(v1222*v1350)))))-(v250*((v1129+(v1222/v225))-((v253*v1133)+(v209*(v1222*v1374))))))/v1392)}else{v9});
        let v1408=(if v201{(((v255*((v1162+v1223)+(v216*(v1223*v1350))))-(v250*((v1130+(v1223/v225))-(v209*(v1223*v1374)))))/v1392)}else{v9});
        let v1409=(if v201{(((v255*((v1163+v1224)+(v216*(v1224*v1350))))-(v250*((v1131+(v1224/v225))-(v209*(v1224*v1374)))))/v1392)}else{v9});
        let v1410=(if v221{(((v245*((v1157+v1225)+(v216*(v1225*v1289))))-(v240*((v1049+(v1225/v226))-(v209*(v1225*v1313)))))/v1331)}else{v1406});
        let v1411=(if v221{(((v245*((v1161+v1226)+((v238*v1164)+(v216*(v1226*v1289)))))-(v240*((v1129+(v1226/v226))-((v243*v1133)+(v209*(v1226*v1313))))))/v1331)}else{v1407});
        let v1412=(if v221{(((v245*((v1162+v1227)+(v216*(v1227*v1289))))-(v240*((v1130+(v1227/v226))-(v209*(v1227*v1313)))))/v1331)}else{v1408});
        let v1413=(if v221{(((v245*((v1163+v1228)+(v216*(v1228*v1289))))-(v240*((v1131+(v1228/v226))-(v209*(v1228*v1313)))))/v1331)}else{v1409});
        let v1414=(if v206{(((v235*(v1157+(v216*(v1229*v1235))))-(v230*(v1049-(v209*(v1229*v1252)))))/v1270)}else{v1410});
        let v1415=(if v206{(((v235*(v1161+((v228*v1164)+(v216*(v1230*v1235)))))-(v230*(v1129-((v233*v1133)+(v209*(v1230*v1252))))))/v1270)}else{v1411});
        let v1416=(if v206{(((v235*(v1162+(v216*(v1231*v1235))))-(v230*(v1130-(v209*(v1231*v1252)))))/v1270)}else{v1412});
        let v1417=(if v206{(((v235*(v1163+(v216*(v1232*v1235))))-(v230*(v1131-(v209*(v1232*v1252)))))/v1270)}else{v1413});
        let v1419=(v33*f64::powf(v259,v1233));
        let v1435=(v232*f64::powf(v259,v1250));
        let v1453=(v265*v265);
        let v1472=(v33*f64::powf(v258,v1233));
        let v1496=(v232*f64::powf(v258,v1250));
        let v1514=(v275*v275);
        let v1533=(v33*f64::powf(v257,v1233));
        let v1557=(v232*f64::powf(v257,v1250));
        let v1575=(v285*v285);
        let v1589=(if v201{(((v285*((v1157+v1406)+(v216*(v1406*v1533))))-(v280*((v1049+(v1406/v257))-(v209*(v1406*v1557)))))/v1575)}else{v9});
        let v1590=(if v201{(((v285*((v1161+v1407)+((v278*v1164)+(v216*(v1407*v1533)))))-(v280*((v1129+(v1407/v257))-((v283*v1133)+(v209*(v1407*v1557))))))/v1575)}else{v9});
        let v1591=(if v201{(((v285*((v1162+v1408)+(v216*(v1408*v1533))))-(v280*((v1130+(v1408/v257))-(v209*(v1408*v1557)))))/v1575)}else{v9});
        let v1592=(if v201{(((v285*((v1163+v1409)+(v216*(v1409*v1533))))-(v280*((v1131+(v1409/v257))-(v209*(v1409*v1557)))))/v1575)}else{v9});
        let v1593=(if v221{(((v275*((v1157+v1410)+(v216*(v1410*v1472))))-(v270*((v1049+(v1410/v258))-(v209*(v1410*v1496)))))/v1514)}else{v1589});
        let v1594=(if v221{(((v275*((v1161+v1411)+((v268*v1164)+(v216*(v1411*v1472)))))-(v270*((v1129+(v1411/v258))-((v273*v1133)+(v209*(v1411*v1496))))))/v1514)}else{v1590});
        let v1595=(if v221{(((v275*((v1162+v1412)+(v216*(v1412*v1472))))-(v270*((v1130+(v1412/v258))-(v209*(v1412*v1496)))))/v1514)}else{v1591});
        let v1596=(if v221{(((v275*((v1163+v1413)+(v216*(v1413*v1472))))-(v270*((v1131+(v1413/v258))-(v209*(v1413*v1496)))))/v1514)}else{v1592});
        let v1597=(if v206{(((v265*(v1157+(v216*(v1414*v1419))))-(v262*(v1049-(v209*(v1414*v1435)))))/v1453)}else{v1593});
        let v1598=(if v206{(((v265*(v1161+((v260*v1164)+(v216*(v1415*v1419)))))-(v262*(v1129-((v263*v1133)+(v209*(v1415*v1435))))))/v1453)}else{v1594});
        let v1599=(if v206{(((v265*(v1162+(v216*(v1416*v1419))))-(v262*(v1130-(v209*(v1416*v1435)))))/v1453)}else{v1595});
        let v1600=(if v206{(((v265*(v1163+(v216*(v1417*v1419))))-(v262*(v1131-(v209*(v1417*v1435)))))/v1453)}else{v1596});
        let v1602=(v33*f64::powf(v289,v1233));
        let v1618=(v232*f64::powf(v289,v1250));
        let v1636=(v295*v295);
        let v1650=(if v206{(((v295*(v1157+(v216*(v1597*v1602))))-(v292*(v1049-(v209*(v1597*v1618)))))/v1636)}else{v9});
        let v1651=(if v206{(((v295*(v1161+((v290*v1164)+(v216*(v1598*v1602)))))-(v292*(v1129-((v293*v1133)+(v209*(v1598*v1618))))))/v1636)}else{v9});
        let v1652=(if v206{(((v295*(v1162+(v216*(v1599*v1602))))-(v292*(v1130-(v209*(v1599*v1618)))))/v1636)}else{v9});
        let v1653=(if v206{(((v295*(v1163+(v216*(v1600*v1602))))-(v292*(v1131-(v209*(v1600*v1618)))))/v1636)}else{v9});
        let v1655=(v232*f64::powf(v297,v1250));
        let v1671=(v33*f64::powf(v297,v1233));
        let v1689=(v303*v303);
        let v1712=(v232*f64::powf(v288,v1250));
        let v1732=(v33*f64::powf(v288,v1233));
        let v1750=(v313*v313);
        let v1773=(v232*f64::powf(v287,v1250));
        let v1793=(v33*f64::powf(v287,v1233));
        let v1811=(v323*v323);
        let v1825=(if v201{(((v323*((v1049+(v1589/v287))-(v209*(v1589*v1773))))-(v319*((v1157+v1589)+(v216*(v1589*v1793)))))/v1811)}else{v9});
        let v1826=(if v201{(((v323*((v1129+(v1590/v287))-((v317*v1133)+(v209*(v1590*v1773)))))-(v319*((v1161+v1590)+((v321*v1164)+(v216*(v1590*v1793))))))/v1811)}else{v9});
        let v1827=(if v201{(((v323*((v1130+(v1591/v287))-(v209*(v1591*v1773))))-(v319*((v1162+v1591)+(v216*(v1591*v1793)))))/v1811)}else{v9});
        let v1828=(if v201{(((v323*((v1131+(v1592/v287))-(v209*(v1592*v1773))))-(v319*((v1163+v1592)+(v216*(v1592*v1793)))))/v1811)}else{v9});
        let v1829=(if v221{(((v313*((v1049+(v1593/v288))-(v209*(v1593*v1712))))-(v309*((v1157+v1593)+(v216*(v1593*v1732)))))/v1750)}else{v1825});
        let v1830=(if v221{(((v313*((v1129+(v1594/v288))-((v307*v1133)+(v209*(v1594*v1712)))))-(v309*((v1161+v1594)+((v311*v1164)+(v216*(v1594*v1732))))))/v1750)}else{v1826});
        let v1831=(if v221{(((v313*((v1130+(v1595/v288))-(v209*(v1595*v1712))))-(v309*((v1162+v1595)+(v216*(v1595*v1732)))))/v1750)}else{v1827});
        let v1832=(if v221{(((v313*((v1131+(v1596/v288))-(v209*(v1596*v1712))))-(v309*((v1163+v1596)+(v216*(v1596*v1732)))))/v1750)}else{v1828});
        let v1846=(v186*v186);
        let v1912=(if v206{(((v186*(v207*(if v206{(((v303*(v1049-(v209*(v1650*v1655))))-(v300*(v1157+(v216*(v1650*v1671)))))/v1689)}else{v1829})))-(v328*v1104))/v1846)}else{(if v221{(((v186*(v207*v1829))-(v330*v1104))/v1846)}else{(if v201{(((v186*(v207*v1825))-(v332*v1104))/v1846)}else{v9})})});
        let v1913=(if v206{(((v186*((v327*self.scalar_static_f64[87])+(v207*(if v206{(((v303*(v1129-((v298*v1133)+(v209*(v1651*v1655)))))-(v300*(v1161+((v301*v1164)+(v216*(v1651*v1671))))))/v1689)}else{v1830}))))-(v328*v1107))/v1846)}else{(if v221{(((v186*((v326*self.scalar_static_f64[87])+(v207*v1830)))-(v330*v1107))/v1846)}else{(if v201{(((v186*((v325*self.scalar_static_f64[87])+(v207*v1826)))-(v332*v1107))/v1846)}else{v9})})});
        let v1914=(if v206{(((v186*(v207*(if v206{(((v303*(v1130-(v209*(v1652*v1655))))-(v300*(v1162+(v216*(v1652*v1671)))))/v1689)}else{v1831})))-(v328*v1108))/v1846)}else{(if v221{(((v186*(v207*v1831))-(v330*v1108))/v1846)}else{(if v201{(((v186*(v207*v1827))-(v332*v1108))/v1846)}else{v9})})});
        let v1915=(if v206{(((v186*(v207*(if v206{(((v303*(v1131-(v209*(v1653*v1655))))-(v300*(v1163+(v216*(v1653*v1671)))))/v1689)}else{v1832})))-(v328*v1109))/v1846)}else{(if v221{(((v186*(v207*v1832))-(v330*v1109))/v1846)}else{(if v201{(((v186*(v207*v1828))-(v332*v1109))/v1846)}else{v9})})});
        let v1916=(v5*v1912);
        let v1917=(v5*v1913);
        let v1918=(v5*v1914);
        let v1919=(v5*v1915);
        let v1920=(v1049-v1916);
        let v1921=(v1050-v1917);
        let v1922=(v1051-v1918);
        let v1923=(v1052-v1919);
        let v1924=(v340*v1920);
        let v1926=(v340*v1921);
        let v1928=(v340*v1922);
        let v1930=(v340*v1923);
        let v1932=(v5*v343);
        let v1945=(v5*v346);
        let v1956=(v347*v347);
        let v1957=((-(v129*(v1074+((v34*(v1920+((v1924+v1924)/v1932)))/v1945))))/v1956);
        let v1961=(((v347*v913)-(v129*(v1075+((v34*(v1921+((v1926+v1926)/v1932)))/v1945))))/v1956);
        let v1964=((-(v129*(v1076+((v34*(v1922+((v1928+v1928)/v1932)))/v1945))))/v1956);
        let v1967=((-(v129*(v1077+((v34*(v1923+((v1930+v1930)/v1932)))/v1945))))/v1956);
        let v1994=(((v370*(v5*v861))-(v369*(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*(self.scalar_static_f64[77]*(self.scalar_static_f64[41]*f64::powf(v15,self.scalar_static_f64[90])))))))/(v370*v370));
        let v1995=((v5*v862)/v370);
        let v1996=((v5*v863)/v370);
        let v1997=(v371*v1994);
        let v1998=(v1997+v1997);
        let v1999=(v371*v1995);
        let v2000=(v1999+v1999);
        let v2001=(v371*v1996);
        let v2002=(v2001+v2001);
        let v2003=(v5*v1994);
        let v2004=(v5*v1995);
        let v2005=(v5*v1996);
        let v2006=(v373*v1912);
        let v2009=((v373*v1913)+(v336*v2003));
        let v2012=((v373*v1914)+(v336*v2004));
        let v2015=((v373*v1915)+(v336*v2005));
        let v2017=(v5*f64::powf(v336,v4));
        let v2039=(v374*v1994);
        let v2041=(v374*v1995);
        let v2043=(v374*v1996);
        let v2052=(v5*v384);
        let v2064=(v385*v385);
        let v2065=(((v385*(v373*(v1912+(v1912*v2017))))-(v378*(v2006+((v381*v2006)/v2052))))/v2064);
        let v2069=(((v385*((v377*v2003)+(v373*(v1913+(v1913*v2017)))))-(v378*((v1994+v2009)+(((v2039+v2039)+(v381*v2009))/v2052))))/v2064);
        let v2073=(((v385*((v377*v2004)+(v373*(v1914+(v1914*v2017)))))-(v378*((v1995+v2012)+(((v2041+v2041)+(v381*v2012))/v2052))))/v2064);
        let v2077=(((v385*((v377*v2005)+(v373*(v1915+(v1915*v2017)))))-(v378*((v1996+v2015)+(((v2043+v2043)+(v381*v2015))/v2052))))/v2064);
        let v2078=(v1912-v2065);
        let v2079=(v1913-v2069);
        let v2080=(v1914-v2073);
        let v2081=(v1915-v2077);
        let v2082=(v387*v2078);
        let v2083=(v2082+v2082);
        let v2084=(v387*v2079);
        let v2085=(v2084+v2084);
        let v2086=(v387*v2080);
        let v2087=(v2086+v2086);
        let v2088=(v387*v2081);
        let v2089=(v2088+v2088);
        let v2159=(v399*v399);
        let v2187=(v5*v408);
        let v2195=(v408*v408);
        let v2209=(v1049-(((v408*((v395*((v5*v2065)+(v2065/v386)))+(v393*(v371*v2078))))-(v396*(((((v399*(v402*v2083))-(v403*(v397*v2078)))/v2159)+(v372*v2083))/v2187)))/v2195));
        let v2213=(((v1050-v822)-(((v408*((v395*((v5*v2069)+(v2069/v386)))+(v393*((v387*v1994)+(v371*v2079)))))-(v396*(((((v399*((v402*v2085)+(v388*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1998))))))-(v403*((v397*v2079)+(v387*(self.scalar_static_f64[43]*v1994)))))/v2159)+((v388*v1998)+(v372*v2085)))/v2187)))/v2195))-v881);
        let v2214=((v1051-(((v408*((v395*((v5*v2073)+(v2073/v386)))+(v393*((v387*v1995)+(v371*v2080)))))-(v396*(((((v399*((v402*v2087)+(v388*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v2000))))))-(v403*((v397*v2080)+(v387*(self.scalar_static_f64[43]*v1995)))))/v2159)+((v388*v2000)+(v372*v2087)))/v2187)))/v2195))-v882);
        let v2215=((v1052-(((v408*((v395*((v5*v2077)+(v2077/v386)))+(v393*((v387*v1996)+(v371*v2081)))))-(v396*(((((v399*((v402*v2089)+(v388*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v2002))))))-(v403*((v397*v2081)+(v387*(self.scalar_static_f64[43]*v1996)))))/v2159)+((v388*v2002)+(v372*v2089)))/v2187)))/v2195))-v883);
        let v2216=(v413*v2209);
        let v2218=(v413*v2213);
        let v2220=(v413*v2214);
        let v2222=(v413*v2215);
        let v2224=(v5*v416);
        let v2233=(v34*(v2209+((v2216+v2216)/v2224)));
        let v2234=(v34*(v2213+((v2218+v2218)/v2224)));
        let v2235=(v34*(v2214+((v2220+v2220)/v2224)));
        let v2236=(v34*(v2215+((v2222+v2222)/v2224)));
        let v2248=(v424*v424);
        let v2269=(v418*(v381*(((v424*(self.scalar_static_f64[45]*(v381*v2065)))-(v423*v1912))/v2248)));
        let v2270=(v427*v2233);
        let v2272=(v418*v418);
        let v2274=(v418*(v381*(((v424*(self.scalar_static_f64[45]*(v381*v2069)))-(v423*v1913))/v2248)));
        let v2275=(v427*v2234);
        let v2278=(v418*(v381*(((v424*(self.scalar_static_f64[45]*(v381*v2073)))-(v423*v1914))/v2248)));
        let v2279=(v427*v2235);
        let v2282=(v418*(v381*(((v424*(self.scalar_static_f64[45]*(v381*v2077)))-(v423*v1915))/v2248)));
        let v2283=(v427*v2236);
        let v2286=(v5*v430);
        let v2291=(v426*(((v2269-v2270)/v2272)/v2286));
        let v2294=((v430*(v871-v881))+(v426*(((v2274-v2275)/v2272)/v2286)));
        let v2297=((v430*(v873-v882))+(v426*(((v2278-v2279)/v2272)/v2286)));
        let v2300=((v430*(v876-v883))+(v426*(((v2282-v2283)/v2272)/v2286)));
        let v2305=(v432*(v2233+v2291));
        let v2307=(v432*(v2234+v2294));
        let v2309=(v432*(v2235+v2297));
        let v2311=(v432*(v2236+v2300));
        let v2313=(v2269+v2270);
        let v2314=(v2274+v2275);
        let v2315=(v2278+v2279);
        let v2316=(v2282+v2283);
        let v2321=(v5*v436);
        let v2330=(v437*(v2291-v2233));
        let v2332=(v437*(v2294-v2234));
        let v2334=(v437*(v2297-v2235));
        let v2336=(v437*(v2300-v2236));
        let v2342=(v5*v440);
        let v2351=(v34*((((v2305+v2305)+v2313)/v2321)-((v2313+(v2330+v2330))/v2342)));
        let v2355=(v881+(v34*((((v2307+v2307)+v2314)/v2321)-((v2314+(v2332+v2332))/v2342))));
        let v2356=(v882+(v34*((((v2309+v2309)+v2315)/v2321)-((v2315+(v2334+v2334))/v2342))));
        let v2357=(v883+(v34*((((v2311+v2311)+v2316)/v2321)-((v2316+(v2336+v2336))/v2342))));
        let v2371=(v447*(-v2351));
        let v2374=((v448*((self.scalar_static_f64[12]*(v34*v1994))/self.scalar_static_f64[46]))+(v447*(v871-v2355)));
        let v2377=((v448*((self.scalar_static_f64[12]*(v34*v1995))/self.scalar_static_f64[46]))+(v447*(v873-v2356)));
        let v2380=((v448*((self.scalar_static_f64[12]*(v34*v1996))/self.scalar_static_f64[46]))+(v447*(v876-v2357)));
        let v2381=(v449*v2371);
        let v2383=(v449*v2374);
        let v2385=(v449*v2377);
        let v2387=(v449*v2380);
        let v2397=(v5*v461);
        let v2418=(v1049-v2351);
        let v2419=(v1128-v2355);
        let v2420=(v1051-v2356);
        let v2421=(v1052-v2357);
        let v2428=(v474*v474);
        let v2445=((v189*((-(v0*(self.scalar_static_f64[15]*v1957)))/v2428))/v41);
        let v2449=(((v41*((v475*self.scalar_static_f64[85])+(v189*((-(v0*(self.scalar_static_f64[15]*v1961)))/v2428))))-(v476*self.scalar_static_f64[79]))/v807);
        let v2450=((v189*((-(v0*(self.scalar_static_f64[15]*v1964)))/v2428))/v41);
        let v2451=((v189*((-(v0*(self.scalar_static_f64[15]*v1967)))/v2428))/v41);
        let v2456=(v479*v479);
        let v2457=(((v479*v2418)-(v467*v2445))/v2456);
        let v2461=(((v479*v2419)-(v467*(v1120+v2449)))/v2456);
        let v2465=(((v479*v2420)-(v467*v2450))/v2456);
        let v2469=(((v479*v2421)-(v467*v2451))/v2456);
        let v2472=(v481*v481);
        let v2497=(v485*v485);
        let v2508=(if v468{((-(v138*(v485*v2418)))/v2497)}else{v9});
        let v2509=(if v468{((-(v138*(v485*v2419)))/v2497)}else{v9});
        let v2510=(if v468{((-(v138*(v485*v2420)))/v2497)}else{v9});
        let v2511=(if v468{((-(v138*(v485*v2421)))/v2497)}else{v9});
        let v2512=(if v483{((-v2457)/v2472)}else{v2508});
        let v2513=(if v483{((-v2461)/v2472)}else{v2509});
        let v2514=(if v483{((-v2465)/v2472)}else{v2510});
        let v2515=(if v483{((-v2469)/v2472)}else{v2511});
        let v2516=(if v472{((-(v131*v2457))/v2472)}else{v2512});
        let v2517=(if v472{((-(v131*v2461))/v2472)}else{v2513});
        let v2518=(if v472{((-(v131*v2465))/v2472)}else{v2514});
        let v2519=(if v472{((-(v131*v2469))/v2472)}else{v2515});
        let v2521=(v33*f64::powf(v489,v1233));
        let v2537=(v232*f64::powf(v489,v1250));
        let v2555=(v496*v496);
        let v2574=(v33*f64::powf(v488,v1233));
        let v2598=(v232*f64::powf(v488,v1250));
        let v2616=(v506*v506);
        let v2635=(v33*f64::powf(v487,v1233));
        let v2659=(v232*f64::powf(v487,v1250));
        let v2677=(v516*v516);
        let v2691=(if v468{(((v516*((v2445+v2508)+(v216*(v2508*v2635))))-(v511*((v2418+(v2508/v487))-(v209*(v2508*v2659)))))/v2677)}else{v9});
        let v2692=(if v468{(((v516*((v2449+v2509)+((v509*v1164)+(v216*(v2509*v2635)))))-(v511*((v2419+(v2509/v487))-((v514*v1133)+(v209*(v2509*v2659))))))/v2677)}else{v9});
        let v2693=(if v468{(((v516*((v2450+v2510)+(v216*(v2510*v2635))))-(v511*((v2420+(v2510/v487))-(v209*(v2510*v2659)))))/v2677)}else{v9});
        let v2694=(if v468{(((v516*((v2451+v2511)+(v216*(v2511*v2635))))-(v511*((v2421+(v2511/v487))-(v209*(v2511*v2659)))))/v2677)}else{v9});
        let v2695=(if v483{(((v506*((v2445+v2512)+(v216*(v2512*v2574))))-(v501*((v2418+(v2512/v488))-(v209*(v2512*v2598)))))/v2616)}else{v2691});
        let v2696=(if v483{(((v506*((v2449+v2513)+((v499*v1164)+(v216*(v2513*v2574)))))-(v501*((v2419+(v2513/v488))-((v504*v1133)+(v209*(v2513*v2598))))))/v2616)}else{v2692});
        let v2697=(if v483{(((v506*((v2450+v2514)+(v216*(v2514*v2574))))-(v501*((v2420+(v2514/v488))-(v209*(v2514*v2598)))))/v2616)}else{v2693});
        let v2698=(if v483{(((v506*((v2451+v2515)+(v216*(v2515*v2574))))-(v501*((v2421+(v2515/v488))-(v209*(v2515*v2598)))))/v2616)}else{v2694});
        let v2699=(if v472{(((v496*(v2445+(v216*(v2516*v2521))))-(v492*(v2418-(v209*(v2516*v2537)))))/v2555)}else{v2695});
        let v2700=(if v472{(((v496*(v2449+((v490*v1164)+(v216*(v2517*v2521)))))-(v492*(v2419-((v494*v1133)+(v209*(v2517*v2537))))))/v2555)}else{v2696});
        let v2701=(if v472{(((v496*(v2450+(v216*(v2518*v2521))))-(v492*(v2420-(v209*(v2518*v2537)))))/v2555)}else{v2697});
        let v2702=(if v472{(((v496*(v2451+(v216*(v2519*v2521))))-(v492*(v2421-(v209*(v2519*v2537)))))/v2555)}else{v2698});
        let v2704=(v33*f64::powf(v520,v1233));
        let v2720=(v232*f64::powf(v520,v1250));
        let v2738=(v526*v526);
        let v2757=(v33*f64::powf(v519,v1233));
        let v2781=(v232*f64::powf(v519,v1250));
        let v2799=(v536*v536);
        let v2818=(v33*f64::powf(v518,v1233));
        let v2842=(v232*f64::powf(v518,v1250));
        let v2860=(v546*v546);
        let v2874=(if v468{(((v546*((v2445+v2691)+(v216*(v2691*v2818))))-(v541*((v2418+(v2691/v518))-(v209*(v2691*v2842)))))/v2860)}else{v9});
        let v2875=(if v468{(((v546*((v2449+v2692)+((v539*v1164)+(v216*(v2692*v2818)))))-(v541*((v2419+(v2692/v518))-((v544*v1133)+(v209*(v2692*v2842))))))/v2860)}else{v9});
        let v2876=(if v468{(((v546*((v2450+v2693)+(v216*(v2693*v2818))))-(v541*((v2420+(v2693/v518))-(v209*(v2693*v2842)))))/v2860)}else{v9});
        let v2877=(if v468{(((v546*((v2451+v2694)+(v216*(v2694*v2818))))-(v541*((v2421+(v2694/v518))-(v209*(v2694*v2842)))))/v2860)}else{v9});
        let v2878=(if v483{(((v536*((v2445+v2695)+(v216*(v2695*v2757))))-(v531*((v2418+(v2695/v519))-(v209*(v2695*v2781)))))/v2799)}else{v2874});
        let v2879=(if v483{(((v536*((v2449+v2696)+((v529*v1164)+(v216*(v2696*v2757)))))-(v531*((v2419+(v2696/v519))-((v534*v1133)+(v209*(v2696*v2781))))))/v2799)}else{v2875});
        let v2880=(if v483{(((v536*((v2450+v2697)+(v216*(v2697*v2757))))-(v531*((v2420+(v2697/v519))-(v209*(v2697*v2781)))))/v2799)}else{v2876});
        let v2881=(if v483{(((v536*((v2451+v2698)+(v216*(v2698*v2757))))-(v531*((v2421+(v2698/v519))-(v209*(v2698*v2781)))))/v2799)}else{v2877});
        let v2882=(if v472{(((v526*(v2445+(v216*(v2699*v2704))))-(v523*(v2418-(v209*(v2699*v2720)))))/v2738)}else{v2878});
        let v2883=(if v472{(((v526*(v2449+((v521*v1164)+(v216*(v2700*v2704)))))-(v523*(v2419-((v524*v1133)+(v209*(v2700*v2720))))))/v2738)}else{v2879});
        let v2884=(if v472{(((v526*(v2450+(v216*(v2701*v2704))))-(v523*(v2420-(v209*(v2701*v2720)))))/v2738)}else{v2880});
        let v2885=(if v472{(((v526*(v2451+(v216*(v2702*v2704))))-(v523*(v2421-(v209*(v2702*v2720)))))/v2738)}else{v2881});
        let v2887=(v33*f64::powf(v550,v1233));
        let v2903=(v232*f64::powf(v550,v1250));
        let v2921=(v556*v556);
        let v2935=(if v472{(((v556*(v2445+(v216*(v2882*v2887))))-(v553*(v2418-(v209*(v2882*v2903)))))/v2921)}else{v9});
        let v2936=(if v472{(((v556*(v2449+((v551*v1164)+(v216*(v2883*v2887)))))-(v553*(v2419-((v554*v1133)+(v209*(v2883*v2903))))))/v2921)}else{v9});
        let v2937=(if v472{(((v556*(v2450+(v216*(v2884*v2887))))-(v553*(v2420-(v209*(v2884*v2903)))))/v2921)}else{v9});
        let v2938=(if v472{(((v556*(v2451+(v216*(v2885*v2887))))-(v553*(v2421-(v209*(v2885*v2903)))))/v2921)}else{v9});
        let v2940=(v232*f64::powf(v558,v1250));
        let v2956=(v33*f64::powf(v558,v1233));
        let v2974=(v564*v564);
        let v2997=(v232*f64::powf(v549,v1250));
        let v3017=(v33*f64::powf(v549,v1233));
        let v3035=(v574*v574);
        let v3058=(v232*f64::powf(v548,v1250));
        let v3078=(v33*f64::powf(v548,v1233));
        let v3096=(v584*v584);
        let v3110=(if v468{(((v584*((v2418+(v2874/v548))-(v209*(v2874*v3058))))-(v580*((v2445+v2874)+(v216*(v2874*v3078)))))/v3096)}else{v9});
        let v3111=(if v468{(((v584*((v2419+(v2875/v548))-((v578*v1133)+(v209*(v2875*v3058)))))-(v580*((v2449+v2875)+((v582*v1164)+(v216*(v2875*v3078))))))/v3096)}else{v9});
        let v3112=(if v468{(((v584*((v2420+(v2876/v548))-(v209*(v2876*v3058))))-(v580*((v2450+v2876)+(v216*(v2876*v3078)))))/v3096)}else{v9});
        let v3113=(if v468{(((v584*((v2421+(v2877/v548))-(v209*(v2877*v3058))))-(v580*((v2451+v2877)+(v216*(v2877*v3078)))))/v3096)}else{v9});
        let v3114=(if v483{(((v574*((v2418+(v2878/v549))-(v209*(v2878*v2997))))-(v570*((v2445+v2878)+(v216*(v2878*v3017)))))/v3035)}else{v3110});
        let v3115=(if v483{(((v574*((v2419+(v2879/v549))-((v568*v1133)+(v209*(v2879*v2997)))))-(v570*((v2449+v2879)+((v572*v1164)+(v216*(v2879*v3017))))))/v3035)}else{v3111});
        let v3116=(if v483{(((v574*((v2420+(v2880/v549))-(v209*(v2880*v2997))))-(v570*((v2450+v2880)+(v216*(v2880*v3017)))))/v3035)}else{v3112});
        let v3117=(if v483{(((v574*((v2421+(v2881/v549))-(v209*(v2881*v2997))))-(v570*((v2451+v2881)+(v216*(v2881*v3017)))))/v3035)}else{v3113});
        let v3132=(v41*(self.scalar_static_f64[15]*(v5*v1957)));
        let v3135=((v590*self.scalar_static_f64[79])+(v41*(self.scalar_static_f64[15]*(v5*v1961))));
        let v3136=(v41*(self.scalar_static_f64[15]*(v5*v1964)));
        let v3137=(v41*(self.scalar_static_f64[15]*(v5*v1967)));
        let v3141=(v591*v591);
        let v3207=(if v472{(((v591*(v207*(if v472{(((v564*(v2418-(v209*(v2935*v2940))))-(v561*(v2445+(v216*(v2935*v2956)))))/v2974)}else{v3114})))-(v589*v3132))/v3141)}else{(if v483{(((v591*(v207*v3114))-(v593*v3132))/v3141)}else{(if v468{(((v591*(v207*v3110))-(v595*v3132))/v3141)}else{v9})})});
        let v3208=(if v472{(((v591*((v588*self.scalar_static_f64[87])+(v207*(if v472{(((v564*(v2419-((v559*v1133)+(v209*(v2936*v2940)))))-(v561*(v2449+((v562*v1164)+(v216*(v2936*v2956))))))/v2974)}else{v3115}))))-(v589*v3135))/v3141)}else{(if v483{(((v591*((v587*self.scalar_static_f64[87])+(v207*v3115)))-(v593*v3135))/v3141)}else{(if v468{(((v591*((v586*self.scalar_static_f64[87])+(v207*v3111)))-(v595*v3135))/v3141)}else{v9})})});
        let v3209=(if v472{(((v591*(v207*(if v472{(((v564*(v2420-(v209*(v2937*v2940))))-(v561*(v2450+(v216*(v2937*v2956)))))/v2974)}else{v3116})))-(v589*v3136))/v3141)}else{(if v483{(((v591*(v207*v3116))-(v593*v3136))/v3141)}else{(if v468{(((v591*(v207*v3112))-(v595*v3136))/v3141)}else{v9})})});
        let v3210=(if v472{(((v591*(v207*(if v472{(((v564*(v2421-(v209*(v2938*v2940))))-(v561*(v2451+(v216*(v2938*v2956)))))/v2974)}else{v3117})))-(v589*v3137))/v3141)}else{(if v483{(((v591*(v207*v3117))-(v593*v3137))/v3141)}else{(if v468{(((v591*(v207*v3113))-(v595*v3137))/v3141)}else{v9})})});
        let v3215=((v1049-v1912)-v3207);
        let v3216=((v1050-v1913)-v3208);
        let v3217=((v1051-v1914)-v3209);
        let v3218=((v1052-v1915)-v3210);
        let v3219=(v603*v3215);
        let v3221=(v603*v3216);
        let v3223=(v603*v3217);
        let v3225=(v603*v3218);
        let v3227=(v5*v606);
        let v3240=(v5*v609);
        let v3251=(v610*v610);
        let v3252=((-(v129*(v1074+((v34*(v3215+((v3219+v3219)/v3227)))/v3240))))/v3251);
        let v3256=(((v610*v913)-(v129*(v1075+((v34*(v3216+((v3221+v3221)/v3227)))/v3240))))/v3251);
        let v3259=((-(v129*(v1076+((v34*(v3217+((v3223+v3223)/v3227)))/v3240))))/v3251);
        let v3262=((-(v129*(v1077+((v34*(v3218+((v3225+v3225)/v3227)))/v3240))))/v3251);
        let v3263=(v1912-v3207);
        let v3264=(v1913-v3208);
        let v3265=(v1914-v3209);
        let v3266=(v1915-v3210);
        let v3267=(v613*v3263);
        let v3269=(v613*v3264);
        let v3271=(v613*v3265);
        let v3273=(v613*v3266);
        let v3275=(v1912+v3207);
        let v3276=(v1913+v3208);
        let v3277=(v1914+v3209);
        let v3278=(v1915+v3210);
        let v3280=(v615*v615);
        let v3281=((-v3275)/v3280);
        let v3283=((-v3276)/v3280);
        let v3285=((-v3277)/v3280);
        let v3287=((-v3278)/v3280);
        let v3290=((v616*(v3267+v3267))+(v614*v3281));
        let v3293=((v616*(v3269+v3269))+(v614*v3283));
        let v3296=((v616*(v3271+v3271))+(v614*v3285));
        let v3299=((v616*(v3273+v3273))+(v614*v3287));
        let v3324=(v33*v3252);
        let v3325=(v33*v3256);
        let v3326=(v33*v3259);
        let v3327=(v33*v3262);
        let v3330=((v617*v3281)+(v616*v3290));
        let v3333=((v617*v3283)+(v616*v3293));
        let v3336=((v617*v3285)+(v616*v3296));
        let v3339=((v617*v3287)+(v616*v3299));
        let v3440=(v109*((v110-v1049)-((v621*v3252)+(v618*(v3275+(v33*v3290))))));
        let v3443=((v623*v861)+(v109*((v907-v1050)-((v621*v3256)+(v618*(v3276+(v33*v3293)))))));
        let v3446=((v623*v862)+(v109*((v908-v1051)-((v621*v3259)+(v618*(v3277+(v33*v3296)))))));
        let v3449=((v623*v863)+(v109*((v909-v1052)-((v621*v3262)+(v618*(v3278+(v33*v3299)))))));
        let v3450=(v647*v3440);
        let v3452=(v647*v3443);
        let v3454=(v647*v3446);
        let v3456=(v647*v3449);
        let v3458=(v5*v650);
        let v3467=(v34*(v3440+((v3450+v3450)/v3458)));
        let v3468=(v34*(v3443+((v3452+v3452)/v3458)));
        let v3469=(v34*(v3446+((v3454+v3454)/v3458)));
        let v3470=(v34*(v3449+((v3456+v3456)/v3458)));
        let v3475=(v109*(((v635*v3324)+(v624*((v1916+v3207)+((v633*v3330)+(v625*(v34*((v627*v1912)+(v630*v3207))))))))+((v645*v3324)+(v624*((v1912+(v5*v3207))+((v643*v3330)+(v625*(v34*((v630*v1912)+(v627*v3207))))))))));
        let v3478=((v653*v861)+(v109*(((v635*v3325)+(v624*((v1917+v3208)+((v633*v3333)+(v625*(v34*((v627*v1913)+(v630*v3208))))))))+((v645*v3325)+(v624*((v1913+(v5*v3208))+((v643*v3333)+(v625*(v34*((v630*v1913)+(v627*v3208)))))))))));
        let v3481=((v653*v862)+(v109*(((v635*v3326)+(v624*((v1918+v3209)+((v633*v3336)+(v625*(v34*((v627*v1914)+(v630*v3209))))))))+((v645*v3326)+(v624*((v1914+(v5*v3209))+((v643*v3336)+(v625*(v34*((v630*v1914)+(v627*v3209)))))))))));
        let v3484=((v653*v863)+(v109*(((v635*v3327)+(v624*((v1919+v3210)+((v633*v3339)+(v625*(v34*((v627*v1915)+(v630*v3210))))))))+((v645*v3327)+(v624*((v1915+(v5*v3210))+((v643*v3339)+(v625*(v34*((v630*v1915)+(v627*v3210)))))))))));
        let v3500=(v652*v652);
        let v3519=(self.scalar_static_f64[34]*f64::powf(v660,self.scalar_static_f64[88]));
        let v3525=(self.scalar_static_f64[37]*f64::powf(v657,self.scalar_static_f64[89]));
        let v3540=(v661*v661);
        let v3551=((v359*((self.scalar_static_f64[33]*(v3467+(self.scalar_static_f64[8]*v3475)))*v3525))+((-(self.scalar_static_f64[38]*((v34*(((v652*v3475)-(v654*v3467))/v3500))*v3519)))/v3540));
        let v3552=((v359*((self.scalar_static_f64[33]*(v3468+(self.scalar_static_f64[8]*v3478)))*v3525))+((-(self.scalar_static_f64[38]*((v34*(((v652*v3478)-(v654*v3468))/v3500))*v3519)))/v3540));
        let v3553=(((v662*(self.scalar_static_f64[36]*v840))+(v359*((self.scalar_static_f64[33]*(v3469+(self.scalar_static_f64[8]*v3481)))*v3525)))+((-(self.scalar_static_f64[38]*((v34*(((v652*v3481)-(v654*v3469))/v3500))*v3519)))/v3540));
        let v3554=(((v662*(self.scalar_static_f64[36]*v841))+(v359*((self.scalar_static_f64[33]*(v3470+(self.scalar_static_f64[8]*v3484)))*v3525)))+((-(self.scalar_static_f64[38]*((v34*(((v652*v3484)-(v654*v3470))/v3500))*v3519)))/v3540));
        let v3555=(v668*v3551);
        let v3557=(v668*v3552);
        let v3559=(v668*v3553);
        let v3561=(v668*v3554);
        let v3563=(v5*v671);
        let v3572=(v34*(v3551+((v3555+v3555)/v3563)));
        let v3573=(v34*(v3552+((v3557+v3557)/v3563)));
        let v3574=(v34*(v3553+((v3559+v3559)/v3563)));
        let v3575=(v34*(v3554+((v3561+v3561)/v3563)));
        let v3578=(v673*v673);
        let v3598=((v675*v3263)+(v613*(v5*((-(v371*v3572))/v3578))));
        let v3601=((v675*v3264)+(v613*(v5*(((v673*v1994)-(v371*v3573))/v3578))));
        let v3604=((v675*v3265)+(v613*(v5*(((v673*v1995)-(v371*v3574))/v3578))));
        let v3607=((v675*v3266)+(v613*(v5*(((v673*v1996)-(v371*v3575))/v3578))));
        let v3608=(v676*v3598);
        let v3610=(v676*v3601);
        let v3612=(v676*v3604);
        let v3614=(v676*v3607);
        let v3616=(v5*v679);
        let v3617=((v3608+v3608)/v3616);
        let v3618=((v3610+v3610)/v3616);
        let v3619=((v3612+v3612)/v3616);
        let v3620=((v3614+v3614)/v3616);
        let v3658=(v679*v679);
        let v3680=((v692*v3572)+(v673*(if v688{(v34*(v3617+((-v3617)/v3658)))}else{(if (v681!=0.0){(v34*(v3617+((v683*((-v3598)/v677))+(v682*(v3598/v679)))))}else{v9})})));
        let v3683=((v692*v3573)+(v673*(if v688{(v34*(v3618+((-v3618)/v3658)))}else{(if (v681!=0.0){(v34*(v3618+((v683*((-v3601)/v677))+(v682*(v3601/v679)))))}else{v9})})));
        let v3686=((v692*v3574)+(v673*(if v688{(v34*(v3619+((-v3619)/v3658)))}else{(if (v681!=0.0){(v34*(v3619+((v683*((-v3604)/v677))+(v682*(v3604/v679)))))}else{v9})})));
        let v3689=((v692*v3575)+(v673*(if v688{(v34*(v3620+((-v3620)/v3658)))}else{(if (v681!=0.0){(v34*(v3620+((v683*((-v3607)/v677))+(v682*(v3607/v679)))))}else{v9})})));
        let v3692=(v693*v693);
        let v3724=(-(self.scalar_static_f64[51]*(((v2371+(((v2381+v2381)+(self.scalar_static_f64[52]*v2371))/v2397))/self.scalar_static_f64[53])/v464)));
        let v3725=(-(self.scalar_static_f64[51]*(((v2374+(((v2383+v2383)+(self.scalar_static_f64[52]*v2374))/v2397))/self.scalar_static_f64[53])/v464)));
        let v3726=(-(self.scalar_static_f64[51]*(((v2377+(((v2385+v2385)+(self.scalar_static_f64[52]*v2377))/v2397))/self.scalar_static_f64[53])/v464)));
        let v3727=(-(self.scalar_static_f64[51]*(((v2380+(((v2387+v2387)+(self.scalar_static_f64[52]*v2380))/v2397))/self.scalar_static_f64[53])/v464)));
        let v3731=(v700*v700);
        let v3783=((v705*(v109*(v109*(self.scalar_static_f64[15]*(((v700*(self.scalar_static_f64[11]*((v697*((-(v39*v3680))/v3692))+(v694*(self.scalar_static_f64[55]*v3252)))))-(v699*v3724))/v3731)))))+(v704*((v615*v3263)+(v613*v3275))));
        let v3786=((v705*((v703*v861)+(v109*((v702*v861)+(v109*(self.scalar_static_f64[15]*(((v700*(self.scalar_static_f64[11]*((v697*(((v693*(self.scalar_static_f64[9]*(self.scalar_static_f64[77]*(self.scalar_static_f64[10]*f64::powf(v15,self.scalar_static_f64[78])))))-(v39*v3683))/v3692))+(v694*(self.scalar_static_f64[55]*v3256)))))-(v699*v3725))/v3731)))))))+(v704*((v615*v3264)+(v613*v3276))));
        let v3789=((v705*((v703*v862)+(v109*((v702*v862)+(v109*(self.scalar_static_f64[15]*(((v700*(self.scalar_static_f64[11]*((v697*((-(v39*v3686))/v3692))+(v694*(self.scalar_static_f64[55]*v3259)))))-(v699*v3726))/v3731)))))))+(v704*((v615*v3265)+(v613*v3277))));
        let v3792=((v705*((v703*v863)+(v109*((v702*v863)+(v109*(self.scalar_static_f64[15]*(((v700*(self.scalar_static_f64[11]*((v697*((-(v39*v3689))/v3692))+(v694*(self.scalar_static_f64[55]*v3262)))))-(v699*v3727))/v3731)))))))+(v704*((v615*v3266)+(v613*v3278))));
        let v3805=(self.scalar_static_f64[63]*(self.scalar_static_f64[60]*(self.scalar_static_f64[77]*(self.scalar_static_f64[61]*f64::powf(v15,self.scalar_static_f64[94])))));
        let v3808=(v723*v723);
        let v3834=(self.scalar_static_f64[66]*f64::powf(v734,self.scalar_static_f64[95]));
        let v3845=(self.scalar_static_f64[67]*f64::powf(v737,self.scalar_static_f64[96]));
        let v3846=((-((if (v733!=0.0){v9}else{(v3783/v729)})*v3834))*v3845);
        let v3847=((-((if (v733!=0.0){v9}else{(((v729*v3786)-(v706*(self.scalar_static_f64[63]*(self.scalar_static_f64[57]*(self.scalar_static_f64[92]*(self.scalar_static_f64[59]*f64::powf(v712,self.scalar_static_f64[93])))))))/(v729*v729))})*v3834))*v3845);
        let v3848=((-((if (v733!=0.0){v9}else{(v3789/v729)})*v3834))*v3845);
        let v3849=((-((if (v733!=0.0){v9}else{(v3792/v729)})*v3834))*v3845);
        let v3852=(v739*v739);
        let v3853=((-(v725*v3846))/v3852);
        let v3860=((-(v725*v3848))/v3852);
        let v3863=((-(v725*v3849))/v3852);
        let v3866=((-(v728*v3846))/v3852);
        let v3873=((-(v728*v3848))/v3852);
        let v3876=((-(v728*v3849))/v3852);
        let v3878=(self.scalar_static_f64[77]*f64::powf(v15,v9));
        let v3881=((((v739*((v724*self.scalar_static_f64[92])+(v712*((-(self.scalar_static_f64[64]*v3805))/v3808))))-(v725*v3847))/v3852)+(self.scalar_static_f64[71]*(self.scalar_static_f64[72]*v3878)));
        let v3884=((((v739*((v727*self.scalar_static_f64[92])+(v712*((-(self.scalar_static_f64[65]*v3805))/v3808))))-(v728*v3847))/v3852)+(self.scalar_static_f64[69]*(self.scalar_static_f64[73]*v3878)));
        let v3952=(v764*v764);
        let v3953=(((v764*v3783)-(v706*((v762*((v760*v3475)+(v654*(((v700*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3680))/v3692))))-(v759*v3724))/v3731))))+(v761*(v3853+v3866)))))/v3952);
        let v3957=(((v764*v3786)-(v706*((v762*((v760*v3478)+(v654*(((v700*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3683))/v3692))))-(v759*v3725))/v3731))))+(v761*(v3881+v3884)))))/v3952);
        let v3961=(((v764*v3789)-(v706*((v762*((v760*v3481)+(v654*(((v700*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3686))/v3692))))-(v759*v3726))/v3731))))+(v761*(v3860+v3873)))))/v3952);
        let v3965=(((v764*v3792)-(v706*((v762*((v760*v3484)+(v654*(((v700*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3689))/v3692))))-(v759*v3727))/v3731))))+(v761*(v3863+v3876)))))/v3952);
        let v3997=(v765*v3953);
        let v3998=(v3997+v3997);
        let v3999=(v765*v3957);
        let v4000=(v3999+v3999);
        let v4001=(v765*v3961);
        let v4002=(v4001+v4001);
        let v4003=(v765*v3965);
        let v4004=(v4003+v4003);

        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            (v756*v765),
            [1, 4, 5, 6],
            [((v765*v3866)+(v756*v3953)), ((v765*v3884)+(v756*v3957)), ((v765*v3873)+(v756*v3961)), ((v765*v3876)+(v756*v3965))],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v765),
            [1, 4, 5, 6],
            [v3953, v3957, v3961, v3965],
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
            (v751*v765),
            [1, 4, 5, 6],
            [((v765*v3853)+(v751*v3953)), ((v765*v3881)+(v751*v3957)), ((v765*v3860)+(v751*v3961)), ((v765*v3863)+(v751*v3965))],
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
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){v781}else{v9})),
            4,
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(self.scalar_static_f64[76]*ddt_scale)}else{v9})),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v765)+(v751*v772))+(v756*v772)))}else{v9})}else{v9})),
            [1, 4, 5, 6],
            [(if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v3953)+((v772*v3853)+(v751*v3998)))+((v772*v3866)+(v756*v3998))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v3957)+((v772*v3881)+(v751*v4000)))+((v772*v3884)+(v756*v4000))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v765+(v65*v3961))+((v772*v3860)+(v751*v4002)))+((v772*v3873)+(v756*v4002))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-((((-v765)+(v65*v3965))+((v772*v3863)+(v751*v4004)))+((v772*v3876)+(v756*v4004))))}else{v9})}else{v9})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(v12/self.scalar_static_f64[74])}else{v9})),
            4,
            multiplicity * (self.scalar_static_f64[98]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if self.scalar_static_bool[2]{(v12*1000000000.0)}else{v9})),
            4,
            multiplicity * (self.scalar_static_f64[99]),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v9=0.0;
        let v781=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(self.scalar_static_f64[76]*1.0)}else{v9})),
        );
    }
}
