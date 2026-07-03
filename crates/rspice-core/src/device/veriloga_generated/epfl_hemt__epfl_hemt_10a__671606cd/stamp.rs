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
        let v220=(v131/v219);
        let v221=f64::powf(v220,v33);
        let v223=(v214+(v216*v221));
        let v224=(v34+v200);
        let v225=-0.6666666666666666;
        let v226=f64::powf(v220,v225);
        let v228=(v224-(v209*v226));
        let v229=(v223/v228);
        let v230=f64::powf(v229,v33);
        let v232=(v214+(v216*v230));
        let v233=f64::powf(v229,v225);
        let v235=(v224-(v209*v233));
        let v236=(v232/v235);
        let v237=f64::powf(v236,v33);
        let v239=(v214+(v216*v237));
        let v240=f64::powf(v236,v225);
        let v242=(v224-(v209*v240));
        let v243=(v239/v242);
        let v244=f64::powf(v243,v225);
        let v246=(v208-(v209*v244));
        let v247=f64::powf(v243,v33);
        let v249=(v214+(v216*v247));
        let v250=(v246/v249);
        let v251=(v207*v250);
        let v253=(v202&&v204);
        let v254=(v4/v219);
        let v256=f64::powf(v254,v33);
        let v258=((v213+v254)+(v216*v256));
        let v261=f64::powf(v254,v225);
        let v263=((v208+(v254).ln())-(v209*v261));
        let v264=(v258/v263);
        let v266=f64::powf(v264,v33);
        let v268=((v213+v264)+(v216*v266));
        let v271=f64::powf(v264,v225);
        let v273=((v208+(v264).ln())-(v209*v271));
        let v274=(v268/v273);
        let v277=f64::powf(v274,v225);
        let v279=((v208+(v274).ln())-(v209*v277));
        let v281=f64::powf(v274,v33);
        let v283=((v213+v274)+(v216*v281));
        let v284=(v279/v283);
        let v285=(v207*v284);
        let v287=(v200).exp();
        let v288=(v138/v287);
        let v290=f64::powf(v288,v33);
        let v292=((v213+v288)+(v216*v290));
        let v295=f64::powf(v288,v225);
        let v297=((v208+(v288).ln())-(v209*v295));
        let v298=(v292/v297);
        let v300=f64::powf(v298,v33);
        let v302=((v213+v298)+(v216*v300));
        let v305=f64::powf(v298,v225);
        let v307=((v208+(v298).ln())-(v209*v305));
        let v308=(v302/v307);
        let v311=f64::powf(v308,v225);
        let v313=((v208+(v308).ln())-(v209*v311));
        let v315=f64::powf(v308,v33);
        let v317=((v213+v308)+(v216*v315));
        let v318=(v313/v317);
        let v319=(v207*v318);
        let v323=(if v206{(v251/v186)}else{(if v253{(v285/v186)}else{(if v201{(v319/v186)}else{v9})})});
        let v324=(v5*v323);
        let v325=(v172-v324);
        let v327=(v325-v4);
        let v330=((v4+(v327*v327))).sqrt();
        let v333=((v34*((v4+v325)+v330))).sqrt();
        let v334=(v180+v333);
        let v336=(v4+(v129/v334));
        let v346=(self.scalar_static_f64[35]+(v87*self.scalar_static_f64[36]));
        let v356=(v5*v109);
        let v357=(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*f64::powf(v15,self.scalar_static_f64[41])));
        let v358=(v356/v357);
        let v359=(v358*v358);
        let v360=(v5*v358);
        let v361=(v5+v358);
        let v362=(v323*v360);
        let v364=(v323+f64::powf(v323,v5));
        let v365=(v360*v364);
        let v368=4.0;
        let v371=(((v361*v361)+(v362*v368))).sqrt();
        let v372=((v361+v362)+v371);
        let v373=(v365/v372);
        let v374=(v323-v373);
        let v375=(v374*v374);
        let v380=((v5*v373)+(v373).ln());
        let v382=(v4+(v358*v374));
        let v383=(v380*v382);
        let v384=(v358*self.scalar_static_f64[43]);
        let v386=(0.1+(v374*v384));
        let v389=(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v359)));
        let v390=(v375*v389);
        let v395=(((v4+(v390/v386))+(v359*v375))).sqrt();
        let v398=(((v172-v57)-(v383/v395))-v113);
        let v400=(v398-v131);
        let v403=((v368+(v400*v400))).sqrt();
        let v405=(v34*((v131+v398)+v403));
        let v410=(self.scalar_static_f64[45]*(self.scalar_static_f64[42]+(v368*v373)));
        let v411=(v4+v323);
        let v413=(v111-v113);
        let v414=(v368*(v410/v411));
        let v417=((v4+(v414/v405))).sqrt();
        let v418=(v413*v417);
        let v419=(v405+v418);
        let v421=(v405*v414);
        let v423=(((v419*v419)+v421)).sqrt();
        let v424=(v418-v405);
        let v427=((v421+(v424*v424))).sqrt();
        let v430=(v113+(v34*(v423-v427)));
        let v434=((self.scalar_static_f64[12]*(v34*v358))/self.scalar_static_f64[46]);
        let v435=(v111-v430);
        let v436=(v434*v435);
        let v448=((v4+((v436*v436)+(v436*self.scalar_static_f64[52])))).sqrt();
        let v451=(((v436+self.scalar_static_f64[49])+v448)/self.scalar_static_f64[53]);
        let v454=(v199-v430);
        let v455=(v454<=v9);
        let v456=(!v455);
        let v457=(v454<v203);
        let v459=(v456&&(!v457));
        let v460=(v4+v454);
        let v461=(self.scalar_static_f64[15]*v336);
        let v462=(v0/v461);
        let v463=(v189*v462);
        let v464=(v463/v41);
        let v465=(v4+v464);
        let v466=(v192+v464);
        let v468=(v34+(v454/v466));
        let v469=(v131/v468);
        let v470=f64::powf(v469,v33);
        let v472=(v465+(v216*v470));
        let v473=(v34+v454);
        let v474=f64::powf(v469,v225);
        let v476=(v473-(v209*v474));
        let v477=(v472/v476);
        let v478=f64::powf(v477,v33);
        let v480=(v465+(v216*v478));
        let v481=f64::powf(v477,v225);
        let v483=(v473-(v209*v481));
        let v484=(v480/v483);
        let v485=f64::powf(v484,v33);
        let v487=(v465+(v216*v485));
        let v488=f64::powf(v484,v225);
        let v490=(v473-(v209*v488));
        let v491=(v487/v490);
        let v492=f64::powf(v491,v225);
        let v494=(v460-(v209*v492));
        let v495=f64::powf(v491,v33);
        let v497=(v465+(v216*v495));
        let v498=(v494/v497);
        let v499=(v207*v498);
        let v500=(self.scalar_static_f64[15]*(v5*v336));
        let v501=(v41*v500);
        let v503=(v456&&v457);
        let v504=(v4/v468);
        let v506=f64::powf(v504,v33);
        let v508=((v464+v504)+(v216*v506));
        let v511=f64::powf(v504,v225);
        let v513=((v460+(v504).ln())-(v209*v511));
        let v514=(v508/v513);
        let v516=f64::powf(v514,v33);
        let v518=((v464+v514)+(v216*v516));
        let v521=f64::powf(v514,v225);
        let v523=((v460+(v514).ln())-(v209*v521));
        let v524=(v518/v523);
        let v527=f64::powf(v524,v225);
        let v529=((v460+(v524).ln())-(v209*v527));
        let v531=f64::powf(v524,v33);
        let v533=((v464+v524)+(v216*v531));
        let v534=(v529/v533);
        let v535=(v207*v534);
        let v537=(v454).exp();
        let v538=(v138/v537);
        let v540=f64::powf(v538,v33);
        let v542=((v464+v538)+(v216*v540));
        let v545=f64::powf(v538,v225);
        let v547=((v460+(v538).ln())-(v209*v545));
        let v548=(v542/v547);
        let v550=f64::powf(v548,v33);
        let v552=((v464+v548)+(v216*v550));
        let v555=f64::powf(v548,v225);
        let v557=((v460+(v548).ln())-(v209*v555));
        let v558=(v552/v557);
        let v561=f64::powf(v558,v225);
        let v563=((v460+(v558).ln())-(v209*v561));
        let v565=f64::powf(v558,v33);
        let v567=((v464+v558)+(v216*v565));
        let v568=(v563/v567);
        let v569=(v207*v568);
        let v573=(if v459{(v499/v501)}else{(if v503{(v535/v501)}else{(if v455{(v569/v501)}else{v9})})});
        let v575=((v172-v323)-v573);
        let v577=(v575-v4);
        let v580=((v4+(v577*v577))).sqrt();
        let v583=((v34*((v4+v575)+v580))).sqrt();
        let v584=(v180+v583);
        let v586=(v4+(v129/v584));
        let v587=(v323-v573);
        let v588=(v587*v587);
        let v589=(v411+v573);
        let v590=(v4/v589);
        let v591=(v588*v590);
        let v592=(v586-v4);
        let v595=((v323+v573)+(v33*v591));
        let v597=((v124-v172)-(v592*v595));
        let v598=(v33*v586);
        let v599=(v590*v591);
        let v601=0.8;
        let v604=1.2;
        let v607=(v34*((v4+(v323*v601))+(v573*v604)));
        let v609=((v324+v573)+(v599*v607));
        let v617=(v34*((v4+(v323*v604))+(v573*v601)));
        let v619=((v323+(v5*v573))+(v599*v617));
        let v621=(v109*v597);
        let v624=((0.0025000000000000005+(v621*v621))).sqrt();
        let v626=(v34*(v621+v624));
        let v627=((v598*v609)+(v598*v619));
        let v628=(v109*v627);
        let v631=(self.scalar_static_f64[33]*(v626+(self.scalar_static_f64[8]*v628)));
        let v634=(v34*(v4+(v628/v626)));
        let v635=f64::powf(v634,self.scalar_static_f64[34]);
        let v636=f64::powf(v631,self.scalar_static_f64[37]);
        let v640=(v4+((v346*v636)+(self.scalar_static_f64[38]/v635)));
        let v642=(v640-v4);
        let v645=((5.625e-7+(v642*v642))).sqrt();
        let v647=(v34*((v4+v640)+v645));
        let v649=(v5*(v358/v647));
        let v650=(v587*v649);
        let v651=(v650*v650);
        let v653=((v4+v651)).sqrt();
        let v655=(if (v9!=v650){v4}else{v9});
        let v656=(v4/v650);
        let v657=(v650).asinh();
        let v662=(!(v655!=0.0));
        let v666=(if v662{(v34*(v653+(v4/v653)))}else{(if (v655!=0.0){(v34*(v653+(v656*v657)))}else{v9})});
        let v667=(v647*v666);
        let v668=(v39/v667);
        let v671=(v586*self.scalar_static_f64[55]);
        let v673=(self.scalar_static_f64[11]*(v668*v671));
        let v674=(self.scalar_static_f64[12]-(self.scalar_static_f64[51]*(v451).ln()));
        let v676=(self.scalar_static_f64[15]*(v673/v674));
        let v677=(v109*v676);
        let v678=(v109*v677);
        let v679=(v587*v589);
        let v680=(v678*v679);
        let v686=(v4-(self.scalar_static_f64[58]*(v15-v4)));
        let v697=((self.scalar_static_f64[60]*f64::powf(v15,self.scalar_static_f64[61]))*self.scalar_static_f64[63]);
        let v698=(self.scalar_static_f64[64]/v697);
        let v699=(v686*v698);
        let v701=(self.scalar_static_f64[65]/v697);
        let v702=(v686*v701);
        let v703=((self.scalar_static_f64[57]*f64::powf(v686,self.scalar_static_f64[59]))*self.scalar_static_f64[63]);
        let v704=(v680/v703);
        let v705=0.96;
        let v707=(if (v704>=v705){v4}else{v9});
        let v708=(if (v707!=0.0){v705}else{v704});
        let v711=(v4-f64::powf(v708,self.scalar_static_f64[66]));
        let v713=f64::powf(v711,self.scalar_static_f64[67]);
        let v721=f64::powf(v15,v4);
        let v725=((v699/v713)+(self.scalar_static_f64[71]*(v4+(self.scalar_static_f64[72]*v721))));
        let v730=((v702/v713)+(self.scalar_static_f64[69]*(v4+(v721*self.scalar_static_f64[73]))));
        let v733=(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*(self.scalar_static_f64[9]/v667)));
        let v734=(v733/v674);
        let v735=(v628*v734);
        let v736=(v725+v730);
        let v738=(v4+(v735*v736));
        let v739=(v680/v738);
        let v746=(v739*v739);
        let v755=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (v12*self.scalar_static_f64[76]));
        let v772=(v0*(((v21*(v17*(v5*f64::powf(v13,v4))))-v19)/(v21*v21)));
        let v781=(v41*v41);
        let v796=((if v55{((-(self.scalar_static_f64[13]*(v45*(v50*(((v48*v772)-(v46*self.scalar_static_f64[81]))/(v48*v48))))))/(v51*v51))}else{v9})/v56);
        let v808=((self.scalar_static_f64[17]*(if v73{((self.scalar_static_f64[16]*v74)/v75)}else{(if (v71!=0.0){self.scalar_static_f64[16]}else{v9})}))-v4);
        let v809=((self.scalar_static_f64[17]*(if v73{((v74*self.scalar_static_f64[82])/v75)}else{(if (v71!=0.0){self.scalar_static_f64[82]}else{v9})}))-v10);
        let v814=(-(v34*(v4-v808)));
        let v815=(-(v34*(v10-v809)));
        let v822=(((self.scalar_static_f64[22]*v808)-(self.scalar_static_f64[23]*v814))/self.scalar_static_f64[15]);
        let v823=(((self.scalar_static_f64[22]*v809)-(self.scalar_static_f64[23]*v815))/self.scalar_static_f64[15]);
        let v824=(v102*v822);
        let v826=(v102*v823);
        let v828=(v5*v106);
        let v835=(v108*self.scalar_static_f64[79]);
        let v836=(v41*(v34*(v822+((v824+v824)/v828))));
        let v837=(v41*(v34*(v823+((v826+v826)/v828))));
        let v839=(v109*v109);
        let v840=((-v835)/v839);
        let v842=((-v836)/v839);
        let v844=((-v837)/v839);
        let v845=(v65*v840);
        let v847=(v110+(v65*v842));
        let v848=(-v110);
        let v850=(v848+(v65*v844));
        let v855=(v63*v840);
        let v856=(v63*v842);
        let v857=(v63*v844);
        let v881=(((v62*v840)-(self.scalar_static_f64[24]*v840))-(v121*v840));
        let v882=(((v62*v842)-(self.scalar_static_f64[24]*v842))-((v121*v842)+(v110*((v120*v808)+(v83*(-(self.scalar_static_f64[26]*v814)))))));
        let v883=(((v848+(v62*v844))-(self.scalar_static_f64[24]*v844))-((v121*v844)+(v110*((v120*v809)+(v83*(-(self.scalar_static_f64[26]*v815)))))));
        let v887=(((self.scalar_static_f64[27]*(self.scalar_static_f64[80]/v781))/(v5*v128))/self.scalar_static_f64[15]);
        let v889=(v34*v110);
        let v891=(v34*v882);
        let v892=(v34*v883);
        let v895=((v34*v881)-(v131*(v887/v132)));
        let v896=(v136*v889);
        let v898=(v136*v895);
        let v900=(v136*v891);
        let v902=(v136*v892);
        let v912=(v5*v141);
        let v917=(v889+(((v896+v896)+(v110*v138))/v912));
        let v918=(v895+(((v898+v898)+(v138*v881))/v912));
        let v919=(v891+(((v900+v900)+(v138*v882))/v912));
        let v920=(v892+(((v902+v902)+(v138*v883))/v912));
        let v932=(if (v144!=0.0){((v110-v917)/v129)}else{v9});
        let v933=(if (v144!=0.0){(((v129*(v881-v918))-(v145*v887))/(v129*v129))}else{v9});
        let v934=(if (v144!=0.0){((v882-v919)/v129)}else{v9});
        let v935=(if (v144!=0.0){((v883-v920)/v129)}else{v9});
        let v936=(-v917);
        let v937=(-v918);
        let v938=(-v919);
        let v939=(-v920);
        let v940=(v147*v932);
        let v942=(v147*v933);
        let v944=(v147*v934);
        let v946=(v147*v935);
        let v968=scalar_limited_exp_derivative(v157);
        let v973=(if v156{(v936*v968)}else{v932});
        let v974=(if v156{(v937*v968)}else{v933});
        let v975=(if v156{(v938*v968)}else{v934});
        let v976=(if v156{(v939*v968)}else{v935});
        let v978=(if v156{v9}else{v889});
        let v979=(if v156{(v34*v887)}else{v895});
        let v980=(if v156{v9}else{v891});
        let v981=(if v156{v9}else{v892});
        let v986=(v161*v978);
        let v988=(v161*v979);
        let v990=(v161*v980);
        let v992=(v161*v981);
        let v998=(v5*v166);
        let v1011=(v168*(if v156{((((v110+v973)+(v986+v986))/v998)-v978)}else{v917}));
        let v1013=(v168*(if v156{((((v881+v974)+(v988+v988))/v998)-v979)}else{v918}));
        let v1015=(v168*(if v156{((((v882+v975)+(v990+v990))/v998)-v980)}else{v919}));
        let v1017=(v168*(if v156{((((v883+v976)+(v992+v992))/v998)-v981)}else{v920}));
        let v1023=(if v156{((v1011+v1011)-v973)}else{(if (v144!=0.0){(-((if v151{(v936+(v940+v940))}else{v9})/v152))}else{v9})});
        let v1024=(if v156{((v1013+v1013)-v974)}else{(if (v144!=0.0){(-((if v151{(v937+(v942+v942))}else{v9})/v152))}else{v9})});
        let v1025=(if v156{((v1015+v1015)-v975)}else{(if (v144!=0.0){(-((if v151{(v938+(v944+v944))}else{v9})/v152))}else{v9})});
        let v1026=(if v156{((v1017+v1017)-v976)}else{(if (v144!=0.0){(-((if v151{(v939+(v946+v946))}else{v9})/v152))}else{v9})});
        let v1027=(v174*v1023);
        let v1029=(v174*v1024);
        let v1031=(v174*v1025);
        let v1033=(v174*v1026);
        let v1035=(v5*v177);
        let v1048=((v34*(v1023+((v1027+v1027)/v1035)))/v181);
        let v1049=((v34*(v1024+((v1029+v1029)/v1035)))/v181);
        let v1050=((v34*(v1025+((v1031+v1031)/v1035)))/v181);
        let v1051=((v34*(v1026+((v1033+v1033)/v1035)))/v181);
        let v1058=(v181*v181);
        let v1059=((-(v129*(v5*v1048)))/v1058);
        let v1063=(((v181*v887)-(v129*(v5*v1049)))/v1058);
        let v1066=((-(v129*(v5*v1050)))/v1058);
        let v1069=((-(v129*(v5*v1051)))/v1058);
        let v1078=(v41*(self.scalar_static_f64[15]*(v5*v1059)));
        let v1081=((v185*self.scalar_static_f64[79])+(v41*(self.scalar_static_f64[15]*(v5*v1063))));
        let v1082=(v41*(self.scalar_static_f64[15]*(v5*v1066)));
        let v1083=(v41*(self.scalar_static_f64[15]*(v5*v1069)));
        let v1094=((v191*(self.scalar_static_f64[84]/v781))+(v188*(self.scalar_static_f64[85]*(v190*f64::powf(v189,-0.33340000000000003)))));
        let v1102=(v796+(v1024-(((v195*(-v772))-(v24*self.scalar_static_f64[86]))/(v195*v195))));
        let v1103=(v1102-v855);
        let v1104=(v1025-v856);
        let v1105=(v1026-v857);
        let v1107=(v33*v1094);
        let v1114=(v210*v210);
        let v1131=((v189*((-(v0*(self.scalar_static_f64[15]*v1059)))/v1114))/v41);
        let v1135=(((v41*((v211*self.scalar_static_f64[85])+(v189*((-(v0*(self.scalar_static_f64[15]*v1063)))/v1114))))-(v212*self.scalar_static_f64[79]))/v781);
        let v1136=((v189*((-(v0*(self.scalar_static_f64[15]*v1066)))/v1114))/v41);
        let v1137=((v189*((-(v0*(self.scalar_static_f64[15]*v1069)))/v1114))/v41);
        let v1138=(v215*v1094);
        let v1143=(v217*v217);
        let v1144=(((v217*v1023)-(v200*v1131))/v1143);
        let v1148=(((v217*v1103)-(v200*(v1094+v1135)))/v1143);
        let v1152=(((v217*v1104)-(v200*v1136))/v1143);
        let v1156=(((v217*v1105)-(v200*v1137))/v1143);
        let v1159=(v219*v219);
        let v1160=((-(v131*v1144))/v1159);
        let v1163=((-(v131*v1148))/v1159);
        let v1166=((-(v131*v1152))/v1159);
        let v1169=((-(v131*v1156))/v1159);
        let v1170=-0.6666666666666667;
        let v1172=(v33*f64::powf(v220,v1170));
        let v1187=-1.6666666666666665;
        let v1189=(v225*f64::powf(v220,v1187));
        let v1207=(v228*v228);
        let v1208=(((v228*(v1131+(v216*(v1160*v1172))))-(v223*(v1023-(v209*(v1160*v1189)))))/v1207);
        let v1212=(((v228*(v1135+((v221*v1138)+(v216*(v1163*v1172)))))-(v223*(v1103-((v226*v1107)+(v209*(v1163*v1189))))))/v1207);
        let v1216=(((v228*(v1136+(v216*(v1166*v1172))))-(v223*(v1104-(v209*(v1166*v1189)))))/v1207);
        let v1220=(((v228*(v1137+(v216*(v1169*v1172))))-(v223*(v1105-(v209*(v1169*v1189)))))/v1207);
        let v1222=(v33*f64::powf(v229,v1170));
        let v1238=(v225*f64::powf(v229,v1187));
        let v1256=(v235*v235);
        let v1257=(((v235*(v1131+(v216*(v1208*v1222))))-(v232*(v1023-(v209*(v1208*v1238)))))/v1256);
        let v1261=(((v235*(v1135+((v230*v1138)+(v216*(v1212*v1222)))))-(v232*(v1103-((v233*v1107)+(v209*(v1212*v1238))))))/v1256);
        let v1265=(((v235*(v1136+(v216*(v1216*v1222))))-(v232*(v1104-(v209*(v1216*v1238)))))/v1256);
        let v1269=(((v235*(v1137+(v216*(v1220*v1222))))-(v232*(v1105-(v209*(v1220*v1238)))))/v1256);
        let v1271=(v33*f64::powf(v236,v1170));
        let v1287=(v225*f64::powf(v236,v1187));
        let v1305=(v242*v242);
        let v1306=(((v242*(v1131+(v216*(v1257*v1271))))-(v239*(v1023-(v209*(v1257*v1287)))))/v1305);
        let v1310=(((v242*(v1135+((v237*v1138)+(v216*(v1261*v1271)))))-(v239*(v1103-((v240*v1107)+(v209*(v1261*v1287))))))/v1305);
        let v1314=(((v242*(v1136+(v216*(v1265*v1271))))-(v239*(v1104-(v209*(v1265*v1287)))))/v1305);
        let v1318=(((v242*(v1137+(v216*(v1269*v1271))))-(v239*(v1105-(v209*(v1269*v1287)))))/v1305);
        let v1320=(v225*f64::powf(v243,v1187));
        let v1336=(v33*f64::powf(v243,v1170));
        let v1354=(v249*v249);
        let v1377=(v186*v186);
        let v1392=((-v1144)/v1159);
        let v1394=((-v1148)/v1159);
        let v1396=((-v1152)/v1159);
        let v1398=((-v1156)/v1159);
        let v1404=(v33*f64::powf(v254,v1170));
        let v1428=(v225*f64::powf(v254,v1187));
        let v1446=(v263*v263);
        let v1447=(((v263*((v1131+v1392)+(v216*(v1392*v1404))))-(v258*((v1023+(v1392/v254))-(v209*(v1392*v1428)))))/v1446);
        let v1451=(((v263*((v1135+v1394)+((v256*v1138)+(v216*(v1394*v1404)))))-(v258*((v1103+(v1394/v254))-((v261*v1107)+(v209*(v1394*v1428))))))/v1446);
        let v1455=(((v263*((v1136+v1396)+(v216*(v1396*v1404))))-(v258*((v1104+(v1396/v254))-(v209*(v1396*v1428)))))/v1446);
        let v1459=(((v263*((v1137+v1398)+(v216*(v1398*v1404))))-(v258*((v1105+(v1398/v254))-(v209*(v1398*v1428)))))/v1446);
        let v1465=(v33*f64::powf(v264,v1170));
        let v1489=(v225*f64::powf(v264,v1187));
        let v1507=(v273*v273);
        let v1508=(((v273*((v1131+v1447)+(v216*(v1447*v1465))))-(v268*((v1023+(v1447/v264))-(v209*(v1447*v1489)))))/v1507);
        let v1512=(((v273*((v1135+v1451)+((v266*v1138)+(v216*(v1451*v1465)))))-(v268*((v1103+(v1451/v264))-((v271*v1107)+(v209*(v1451*v1489))))))/v1507);
        let v1516=(((v273*((v1136+v1455)+(v216*(v1455*v1465))))-(v268*((v1104+(v1455/v264))-(v209*(v1455*v1489)))))/v1507);
        let v1520=(((v273*((v1137+v1459)+(v216*(v1459*v1465))))-(v268*((v1105+(v1459/v264))-(v209*(v1459*v1489)))))/v1507);
        let v1530=(v225*f64::powf(v274,v1187));
        let v1550=(v33*f64::powf(v274,v1170));
        let v1568=(v283*v283);
        let v1610=(v287*v287);
        let v1611=((-(v138*(v287*v1023)))/v1610);
        let v1614=((-(v138*(v287*v1103)))/v1610);
        let v1617=((-(v138*(v287*v1104)))/v1610);
        let v1620=((-(v138*(v287*v1105)))/v1610);
        let v1626=(v33*f64::powf(v288,v1170));
        let v1650=(v225*f64::powf(v288,v1187));
        let v1668=(v297*v297);
        let v1669=(((v297*((v1131+v1611)+(v216*(v1611*v1626))))-(v292*((v1023+(v1611/v288))-(v209*(v1611*v1650)))))/v1668);
        let v1673=(((v297*((v1135+v1614)+((v290*v1138)+(v216*(v1614*v1626)))))-(v292*((v1103+(v1614/v288))-((v295*v1107)+(v209*(v1614*v1650))))))/v1668);
        let v1677=(((v297*((v1136+v1617)+(v216*(v1617*v1626))))-(v292*((v1104+(v1617/v288))-(v209*(v1617*v1650)))))/v1668);
        let v1681=(((v297*((v1137+v1620)+(v216*(v1620*v1626))))-(v292*((v1105+(v1620/v288))-(v209*(v1620*v1650)))))/v1668);
        let v1687=(v33*f64::powf(v298,v1170));
        let v1711=(v225*f64::powf(v298,v1187));
        let v1729=(v307*v307);
        let v1730=(((v307*((v1131+v1669)+(v216*(v1669*v1687))))-(v302*((v1023+(v1669/v298))-(v209*(v1669*v1711)))))/v1729);
        let v1734=(((v307*((v1135+v1673)+((v300*v1138)+(v216*(v1673*v1687)))))-(v302*((v1103+(v1673/v298))-((v305*v1107)+(v209*(v1673*v1711))))))/v1729);
        let v1738=(((v307*((v1136+v1677)+(v216*(v1677*v1687))))-(v302*((v1104+(v1677/v298))-(v209*(v1677*v1711)))))/v1729);
        let v1742=(((v307*((v1137+v1681)+(v216*(v1681*v1687))))-(v302*((v1105+(v1681/v298))-(v209*(v1681*v1711)))))/v1729);
        let v1752=(v225*f64::powf(v308,v1187));
        let v1772=(v33*f64::powf(v308,v1170));
        let v1790=(v317*v317);
        let v1834=(if v206{(((v186*(v207*(((v249*(v1023-(v209*(v1306*v1320))))-(v246*(v1131+(v216*(v1306*v1336)))))/v1354)))-(v251*v1078))/v1377)}else{(if v253{(((v186*(v207*(((v283*((v1023+(v1508/v274))-(v209*(v1508*v1530))))-(v279*((v1131+v1508)+(v216*(v1508*v1550)))))/v1568)))-(v285*v1078))/v1377)}else{(if v201{(((v186*(v207*(((v317*((v1023+(v1730/v308))-(v209*(v1730*v1752))))-(v313*((v1131+v1730)+(v216*(v1730*v1772)))))/v1790)))-(v319*v1078))/v1377)}else{v9})})});
        let v1835=(if v206{(((v186*((v250*self.scalar_static_f64[87])+(v207*(((v249*(v1103-((v244*v1107)+(v209*(v1310*v1320)))))-(v246*(v1135+((v247*v1138)+(v216*(v1310*v1336))))))/v1354))))-(v251*v1081))/v1377)}else{(if v253{(((v186*((v284*self.scalar_static_f64[87])+(v207*(((v283*((v1103+(v1512/v274))-((v277*v1107)+(v209*(v1512*v1530)))))-(v279*((v1135+v1512)+((v281*v1138)+(v216*(v1512*v1550))))))/v1568))))-(v285*v1081))/v1377)}else{(if v201{(((v186*((v318*self.scalar_static_f64[87])+(v207*(((v317*((v1103+(v1734/v308))-((v311*v1107)+(v209*(v1734*v1752)))))-(v313*((v1135+v1734)+((v315*v1138)+(v216*(v1734*v1772))))))/v1790))))-(v319*v1081))/v1377)}else{v9})})});
        let v1836=(if v206{(((v186*(v207*(((v249*(v1104-(v209*(v1314*v1320))))-(v246*(v1136+(v216*(v1314*v1336)))))/v1354)))-(v251*v1082))/v1377)}else{(if v253{(((v186*(v207*(((v283*((v1104+(v1516/v274))-(v209*(v1516*v1530))))-(v279*((v1136+v1516)+(v216*(v1516*v1550)))))/v1568)))-(v285*v1082))/v1377)}else{(if v201{(((v186*(v207*(((v317*((v1104+(v1738/v308))-(v209*(v1738*v1752))))-(v313*((v1136+v1738)+(v216*(v1738*v1772)))))/v1790)))-(v319*v1082))/v1377)}else{v9})})});
        let v1837=(if v206{(((v186*(v207*(((v249*(v1105-(v209*(v1318*v1320))))-(v246*(v1137+(v216*(v1318*v1336)))))/v1354)))-(v251*v1083))/v1377)}else{(if v253{(((v186*(v207*(((v283*((v1105+(v1520/v274))-(v209*(v1520*v1530))))-(v279*((v1137+v1520)+(v216*(v1520*v1550)))))/v1568)))-(v285*v1083))/v1377)}else{(if v201{(((v186*(v207*(((v317*((v1105+(v1742/v308))-(v209*(v1742*v1752))))-(v313*((v1137+v1742)+(v216*(v1742*v1772)))))/v1790)))-(v319*v1083))/v1377)}else{v9})})});
        let v1838=(v5*v1834);
        let v1839=(v5*v1835);
        let v1840=(v5*v1836);
        let v1841=(v5*v1837);
        let v1842=(v1023-v1838);
        let v1843=(v1024-v1839);
        let v1844=(v1025-v1840);
        let v1845=(v1026-v1841);
        let v1846=(v327*v1842);
        let v1848=(v327*v1843);
        let v1850=(v327*v1844);
        let v1852=(v327*v1845);
        let v1854=(v5*v330);
        let v1867=(v5*v333);
        let v1878=(v334*v334);
        let v1879=((-(v129*(v1048+((v34*(v1842+((v1846+v1846)/v1854)))/v1867))))/v1878);
        let v1883=(((v334*v887)-(v129*(v1049+((v34*(v1843+((v1848+v1848)/v1854)))/v1867))))/v1878);
        let v1886=((-(v129*(v1050+((v34*(v1844+((v1850+v1850)/v1854)))/v1867))))/v1878);
        let v1889=((-(v129*(v1051+((v34*(v1845+((v1852+v1852)/v1854)))/v1867))))/v1878);
        let v1916=(((v357*(v5*v835))-(v356*(self.scalar_static_f64[12]*(self.scalar_static_f64[39]*(self.scalar_static_f64[77]*(self.scalar_static_f64[41]*f64::powf(v15,self.scalar_static_f64[90])))))))/(v357*v357));
        let v1917=((v5*v836)/v357);
        let v1918=((v5*v837)/v357);
        let v1919=(v358*v1916);
        let v1920=(v1919+v1919);
        let v1921=(v358*v1917);
        let v1922=(v1921+v1921);
        let v1923=(v358*v1918);
        let v1924=(v1923+v1923);
        let v1925=(v5*v1916);
        let v1926=(v5*v1917);
        let v1927=(v5*v1918);
        let v1928=(v360*v1834);
        let v1931=((v360*v1835)+(v323*v1925));
        let v1934=((v360*v1836)+(v323*v1926));
        let v1937=((v360*v1837)+(v323*v1927));
        let v1939=(v5*f64::powf(v323,v4));
        let v1961=(v361*v1916);
        let v1963=(v361*v1917);
        let v1965=(v361*v1918);
        let v1974=(v5*v371);
        let v1986=(v372*v372);
        let v1987=(((v372*(v360*(v1834+(v1834*v1939))))-(v365*(v1928+((v368*v1928)/v1974))))/v1986);
        let v1991=(((v372*((v364*v1925)+(v360*(v1835+(v1835*v1939)))))-(v365*((v1916+v1931)+(((v1961+v1961)+(v368*v1931))/v1974))))/v1986);
        let v1995=(((v372*((v364*v1926)+(v360*(v1836+(v1836*v1939)))))-(v365*((v1917+v1934)+(((v1963+v1963)+(v368*v1934))/v1974))))/v1986);
        let v1999=(((v372*((v364*v1927)+(v360*(v1837+(v1837*v1939)))))-(v365*((v1918+v1937)+(((v1965+v1965)+(v368*v1937))/v1974))))/v1986);
        let v2000=(v1834-v1987);
        let v2001=(v1835-v1991);
        let v2002=(v1836-v1995);
        let v2003=(v1837-v1999);
        let v2004=(v374*v2000);
        let v2005=(v2004+v2004);
        let v2006=(v374*v2001);
        let v2007=(v2006+v2006);
        let v2008=(v374*v2002);
        let v2009=(v2008+v2008);
        let v2010=(v374*v2003);
        let v2011=(v2010+v2010);
        let v2081=(v386*v386);
        let v2109=(v5*v395);
        let v2117=(v395*v395);
        let v2131=(v1023-(((v395*((v382*((v5*v1987)+(v1987/v373)))+(v380*(v358*v2000))))-(v383*(((((v386*(v389*v2005))-(v390*(v384*v2000)))/v2081)+(v359*v2005))/v2109)))/v2117));
        let v2135=(((v1024-v796)-(((v395*((v382*((v5*v1991)+(v1991/v373)))+(v380*((v374*v1916)+(v358*v2001)))))-(v383*(((((v386*((v389*v2007)+(v375*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1920))))))-(v390*((v384*v2001)+(v374*(self.scalar_static_f64[43]*v1916)))))/v2081)+((v375*v1920)+(v359*v2007)))/v2109)))/v2117))-v855);
        let v2136=((v1025-(((v395*((v382*((v5*v1995)+(v1995/v373)))+(v380*((v374*v1917)+(v358*v2002)))))-(v383*(((((v386*((v389*v2009)+(v375*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1922))))))-(v390*((v384*v2002)+(v374*(self.scalar_static_f64[43]*v1917)))))/v2081)+((v375*v1922)+(v359*v2009)))/v2109)))/v2117))-v856);
        let v2137=((v1026-(((v395*((v382*((v5*v1999)+(v1999/v373)))+(v380*((v374*v1918)+(v358*v2003)))))-(v383*(((((v386*((v389*v2011)+(v375*(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v5*v1924))))))-(v390*((v384*v2003)+(v374*(self.scalar_static_f64[43]*v1918)))))/v2081)+((v375*v1924)+(v359*v2011)))/v2109)))/v2117))-v857);
        let v2138=(v400*v2131);
        let v2140=(v400*v2135);
        let v2142=(v400*v2136);
        let v2144=(v400*v2137);
        let v2146=(v5*v403);
        let v2155=(v34*(v2131+((v2138+v2138)/v2146)));
        let v2156=(v34*(v2135+((v2140+v2140)/v2146)));
        let v2157=(v34*(v2136+((v2142+v2142)/v2146)));
        let v2158=(v34*(v2137+((v2144+v2144)/v2146)));
        let v2170=(v411*v411);
        let v2191=(v405*(v368*(((v411*(self.scalar_static_f64[45]*(v368*v1987)))-(v410*v1834))/v2170)));
        let v2192=(v414*v2155);
        let v2194=(v405*v405);
        let v2196=(v405*(v368*(((v411*(self.scalar_static_f64[45]*(v368*v1991)))-(v410*v1835))/v2170)));
        let v2197=(v414*v2156);
        let v2200=(v405*(v368*(((v411*(self.scalar_static_f64[45]*(v368*v1995)))-(v410*v1836))/v2170)));
        let v2201=(v414*v2157);
        let v2204=(v405*(v368*(((v411*(self.scalar_static_f64[45]*(v368*v1999)))-(v410*v1837))/v2170)));
        let v2205=(v414*v2158);
        let v2208=(v5*v417);
        let v2213=(v413*(((v2191-v2192)/v2194)/v2208));
        let v2216=((v417*(v845-v855))+(v413*(((v2196-v2197)/v2194)/v2208)));
        let v2219=((v417*(v847-v856))+(v413*(((v2200-v2201)/v2194)/v2208)));
        let v2222=((v417*(v850-v857))+(v413*(((v2204-v2205)/v2194)/v2208)));
        let v2227=(v419*(v2155+v2213));
        let v2229=(v419*(v2156+v2216));
        let v2231=(v419*(v2157+v2219));
        let v2233=(v419*(v2158+v2222));
        let v2235=(v2191+v2192);
        let v2236=(v2196+v2197);
        let v2237=(v2200+v2201);
        let v2238=(v2204+v2205);
        let v2243=(v5*v423);
        let v2252=(v424*(v2213-v2155));
        let v2254=(v424*(v2216-v2156));
        let v2256=(v424*(v2219-v2157));
        let v2258=(v424*(v2222-v2158));
        let v2264=(v5*v427);
        let v2273=(v34*((((v2227+v2227)+v2235)/v2243)-((v2235+(v2252+v2252))/v2264)));
        let v2277=(v855+(v34*((((v2229+v2229)+v2236)/v2243)-((v2236+(v2254+v2254))/v2264))));
        let v2278=(v856+(v34*((((v2231+v2231)+v2237)/v2243)-((v2237+(v2256+v2256))/v2264))));
        let v2279=(v857+(v34*((((v2233+v2233)+v2238)/v2243)-((v2238+(v2258+v2258))/v2264))));
        let v2293=(v434*(-v2273));
        let v2296=((v435*((self.scalar_static_f64[12]*(v34*v1916))/self.scalar_static_f64[46]))+(v434*(v845-v2277)));
        let v2299=((v435*((self.scalar_static_f64[12]*(v34*v1917))/self.scalar_static_f64[46]))+(v434*(v847-v2278)));
        let v2302=((v435*((self.scalar_static_f64[12]*(v34*v1918))/self.scalar_static_f64[46]))+(v434*(v850-v2279)));
        let v2303=(v436*v2293);
        let v2305=(v436*v2296);
        let v2307=(v436*v2299);
        let v2309=(v436*v2302);
        let v2319=(v5*v448);
        let v2340=(v1023-v2273);
        let v2341=(v1102-v2277);
        let v2342=(v1025-v2278);
        let v2343=(v1026-v2279);
        let v2350=(v461*v461);
        let v2367=((v189*((-(v0*(self.scalar_static_f64[15]*v1879)))/v2350))/v41);
        let v2371=(((v41*((v462*self.scalar_static_f64[85])+(v189*((-(v0*(self.scalar_static_f64[15]*v1883)))/v2350))))-(v463*self.scalar_static_f64[79]))/v781);
        let v2372=((v189*((-(v0*(self.scalar_static_f64[15]*v1886)))/v2350))/v41);
        let v2373=((v189*((-(v0*(self.scalar_static_f64[15]*v1889)))/v2350))/v41);
        let v2378=(v466*v466);
        let v2379=(((v466*v2340)-(v454*v2367))/v2378);
        let v2383=(((v466*v2341)-(v454*(v1094+v2371)))/v2378);
        let v2387=(((v466*v2342)-(v454*v2372))/v2378);
        let v2391=(((v466*v2343)-(v454*v2373))/v2378);
        let v2394=(v468*v468);
        let v2395=((-(v131*v2379))/v2394);
        let v2398=((-(v131*v2383))/v2394);
        let v2401=((-(v131*v2387))/v2394);
        let v2404=((-(v131*v2391))/v2394);
        let v2406=(v33*f64::powf(v469,v1170));
        let v2422=(v225*f64::powf(v469,v1187));
        let v2440=(v476*v476);
        let v2441=(((v476*(v2367+(v216*(v2395*v2406))))-(v472*(v2340-(v209*(v2395*v2422)))))/v2440);
        let v2445=(((v476*(v2371+((v470*v1138)+(v216*(v2398*v2406)))))-(v472*(v2341-((v474*v1107)+(v209*(v2398*v2422))))))/v2440);
        let v2449=(((v476*(v2372+(v216*(v2401*v2406))))-(v472*(v2342-(v209*(v2401*v2422)))))/v2440);
        let v2453=(((v476*(v2373+(v216*(v2404*v2406))))-(v472*(v2343-(v209*(v2404*v2422)))))/v2440);
        let v2455=(v33*f64::powf(v477,v1170));
        let v2471=(v225*f64::powf(v477,v1187));
        let v2489=(v483*v483);
        let v2490=(((v483*(v2367+(v216*(v2441*v2455))))-(v480*(v2340-(v209*(v2441*v2471)))))/v2489);
        let v2494=(((v483*(v2371+((v478*v1138)+(v216*(v2445*v2455)))))-(v480*(v2341-((v481*v1107)+(v209*(v2445*v2471))))))/v2489);
        let v2498=(((v483*(v2372+(v216*(v2449*v2455))))-(v480*(v2342-(v209*(v2449*v2471)))))/v2489);
        let v2502=(((v483*(v2373+(v216*(v2453*v2455))))-(v480*(v2343-(v209*(v2453*v2471)))))/v2489);
        let v2504=(v33*f64::powf(v484,v1170));
        let v2520=(v225*f64::powf(v484,v1187));
        let v2538=(v490*v490);
        let v2539=(((v490*(v2367+(v216*(v2490*v2504))))-(v487*(v2340-(v209*(v2490*v2520)))))/v2538);
        let v2543=(((v490*(v2371+((v485*v1138)+(v216*(v2494*v2504)))))-(v487*(v2341-((v488*v1107)+(v209*(v2494*v2520))))))/v2538);
        let v2547=(((v490*(v2372+(v216*(v2498*v2504))))-(v487*(v2342-(v209*(v2498*v2520)))))/v2538);
        let v2551=(((v490*(v2373+(v216*(v2502*v2504))))-(v487*(v2343-(v209*(v2502*v2520)))))/v2538);
        let v2553=(v225*f64::powf(v491,v1187));
        let v2569=(v33*f64::powf(v491,v1170));
        let v2587=(v497*v497);
        let v2611=(v41*(self.scalar_static_f64[15]*(v5*v1879)));
        let v2614=((v500*self.scalar_static_f64[79])+(v41*(self.scalar_static_f64[15]*(v5*v1883))));
        let v2615=(v41*(self.scalar_static_f64[15]*(v5*v1886)));
        let v2616=(v41*(self.scalar_static_f64[15]*(v5*v1889)));
        let v2620=(v501*v501);
        let v2635=((-v2379)/v2394);
        let v2637=((-v2383)/v2394);
        let v2639=((-v2387)/v2394);
        let v2641=((-v2391)/v2394);
        let v2647=(v33*f64::powf(v504,v1170));
        let v2671=(v225*f64::powf(v504,v1187));
        let v2689=(v513*v513);
        let v2690=(((v513*((v2367+v2635)+(v216*(v2635*v2647))))-(v508*((v2340+(v2635/v504))-(v209*(v2635*v2671)))))/v2689);
        let v2694=(((v513*((v2371+v2637)+((v506*v1138)+(v216*(v2637*v2647)))))-(v508*((v2341+(v2637/v504))-((v511*v1107)+(v209*(v2637*v2671))))))/v2689);
        let v2698=(((v513*((v2372+v2639)+(v216*(v2639*v2647))))-(v508*((v2342+(v2639/v504))-(v209*(v2639*v2671)))))/v2689);
        let v2702=(((v513*((v2373+v2641)+(v216*(v2641*v2647))))-(v508*((v2343+(v2641/v504))-(v209*(v2641*v2671)))))/v2689);
        let v2708=(v33*f64::powf(v514,v1170));
        let v2732=(v225*f64::powf(v514,v1187));
        let v2750=(v523*v523);
        let v2751=(((v523*((v2367+v2690)+(v216*(v2690*v2708))))-(v518*((v2340+(v2690/v514))-(v209*(v2690*v2732)))))/v2750);
        let v2755=(((v523*((v2371+v2694)+((v516*v1138)+(v216*(v2694*v2708)))))-(v518*((v2341+(v2694/v514))-((v521*v1107)+(v209*(v2694*v2732))))))/v2750);
        let v2759=(((v523*((v2372+v2698)+(v216*(v2698*v2708))))-(v518*((v2342+(v2698/v514))-(v209*(v2698*v2732)))))/v2750);
        let v2763=(((v523*((v2373+v2702)+(v216*(v2702*v2708))))-(v518*((v2343+(v2702/v514))-(v209*(v2702*v2732)))))/v2750);
        let v2773=(v225*f64::powf(v524,v1187));
        let v2793=(v33*f64::powf(v524,v1170));
        let v2811=(v533*v533);
        let v2853=(v537*v537);
        let v2854=((-(v138*(v537*v2340)))/v2853);
        let v2857=((-(v138*(v537*v2341)))/v2853);
        let v2860=((-(v138*(v537*v2342)))/v2853);
        let v2863=((-(v138*(v537*v2343)))/v2853);
        let v2869=(v33*f64::powf(v538,v1170));
        let v2893=(v225*f64::powf(v538,v1187));
        let v2911=(v547*v547);
        let v2912=(((v547*((v2367+v2854)+(v216*(v2854*v2869))))-(v542*((v2340+(v2854/v538))-(v209*(v2854*v2893)))))/v2911);
        let v2916=(((v547*((v2371+v2857)+((v540*v1138)+(v216*(v2857*v2869)))))-(v542*((v2341+(v2857/v538))-((v545*v1107)+(v209*(v2857*v2893))))))/v2911);
        let v2920=(((v547*((v2372+v2860)+(v216*(v2860*v2869))))-(v542*((v2342+(v2860/v538))-(v209*(v2860*v2893)))))/v2911);
        let v2924=(((v547*((v2373+v2863)+(v216*(v2863*v2869))))-(v542*((v2343+(v2863/v538))-(v209*(v2863*v2893)))))/v2911);
        let v2930=(v33*f64::powf(v548,v1170));
        let v2954=(v225*f64::powf(v548,v1187));
        let v2972=(v557*v557);
        let v2973=(((v557*((v2367+v2912)+(v216*(v2912*v2930))))-(v552*((v2340+(v2912/v548))-(v209*(v2912*v2954)))))/v2972);
        let v2977=(((v557*((v2371+v2916)+((v550*v1138)+(v216*(v2916*v2930)))))-(v552*((v2341+(v2916/v548))-((v555*v1107)+(v209*(v2916*v2954))))))/v2972);
        let v2981=(((v557*((v2372+v2920)+(v216*(v2920*v2930))))-(v552*((v2342+(v2920/v548))-(v209*(v2920*v2954)))))/v2972);
        let v2985=(((v557*((v2373+v2924)+(v216*(v2924*v2930))))-(v552*((v2343+(v2924/v548))-(v209*(v2924*v2954)))))/v2972);
        let v2995=(v225*f64::powf(v558,v1187));
        let v3015=(v33*f64::powf(v558,v1170));
        let v3033=(v567*v567);
        let v3077=(if v459{(((v501*(v207*(((v497*(v2340-(v209*(v2539*v2553))))-(v494*(v2367+(v216*(v2539*v2569)))))/v2587)))-(v499*v2611))/v2620)}else{(if v503{(((v501*(v207*(((v533*((v2340+(v2751/v524))-(v209*(v2751*v2773))))-(v529*((v2367+v2751)+(v216*(v2751*v2793)))))/v2811)))-(v535*v2611))/v2620)}else{(if v455{(((v501*(v207*(((v567*((v2340+(v2973/v558))-(v209*(v2973*v2995))))-(v563*((v2367+v2973)+(v216*(v2973*v3015)))))/v3033)))-(v569*v2611))/v2620)}else{v9})})});
        let v3078=(if v459{(((v501*((v498*self.scalar_static_f64[87])+(v207*(((v497*(v2341-((v492*v1107)+(v209*(v2543*v2553)))))-(v494*(v2371+((v495*v1138)+(v216*(v2543*v2569))))))/v2587))))-(v499*v2614))/v2620)}else{(if v503{(((v501*((v534*self.scalar_static_f64[87])+(v207*(((v533*((v2341+(v2755/v524))-((v527*v1107)+(v209*(v2755*v2773)))))-(v529*((v2371+v2755)+((v531*v1138)+(v216*(v2755*v2793))))))/v2811))))-(v535*v2614))/v2620)}else{(if v455{(((v501*((v568*self.scalar_static_f64[87])+(v207*(((v567*((v2341+(v2977/v558))-((v561*v1107)+(v209*(v2977*v2995)))))-(v563*((v2371+v2977)+((v565*v1138)+(v216*(v2977*v3015))))))/v3033))))-(v569*v2614))/v2620)}else{v9})})});
        let v3079=(if v459{(((v501*(v207*(((v497*(v2342-(v209*(v2547*v2553))))-(v494*(v2372+(v216*(v2547*v2569)))))/v2587)))-(v499*v2615))/v2620)}else{(if v503{(((v501*(v207*(((v533*((v2342+(v2759/v524))-(v209*(v2759*v2773))))-(v529*((v2372+v2759)+(v216*(v2759*v2793)))))/v2811)))-(v535*v2615))/v2620)}else{(if v455{(((v501*(v207*(((v567*((v2342+(v2981/v558))-(v209*(v2981*v2995))))-(v563*((v2372+v2981)+(v216*(v2981*v3015)))))/v3033)))-(v569*v2615))/v2620)}else{v9})})});
        let v3080=(if v459{(((v501*(v207*(((v497*(v2343-(v209*(v2551*v2553))))-(v494*(v2373+(v216*(v2551*v2569)))))/v2587)))-(v499*v2616))/v2620)}else{(if v503{(((v501*(v207*(((v533*((v2343+(v2763/v524))-(v209*(v2763*v2773))))-(v529*((v2373+v2763)+(v216*(v2763*v2793)))))/v2811)))-(v535*v2616))/v2620)}else{(if v455{(((v501*(v207*(((v567*((v2343+(v2985/v558))-(v209*(v2985*v2995))))-(v563*((v2373+v2985)+(v216*(v2985*v3015)))))/v3033)))-(v569*v2616))/v2620)}else{v9})})});
        let v3085=((v1023-v1834)-v3077);
        let v3086=((v1024-v1835)-v3078);
        let v3087=((v1025-v1836)-v3079);
        let v3088=((v1026-v1837)-v3080);
        let v3089=(v577*v3085);
        let v3091=(v577*v3086);
        let v3093=(v577*v3087);
        let v3095=(v577*v3088);
        let v3097=(v5*v580);
        let v3110=(v5*v583);
        let v3121=(v584*v584);
        let v3122=((-(v129*(v1048+((v34*(v3085+((v3089+v3089)/v3097)))/v3110))))/v3121);
        let v3126=(((v584*v887)-(v129*(v1049+((v34*(v3086+((v3091+v3091)/v3097)))/v3110))))/v3121);
        let v3129=((-(v129*(v1050+((v34*(v3087+((v3093+v3093)/v3097)))/v3110))))/v3121);
        let v3132=((-(v129*(v1051+((v34*(v3088+((v3095+v3095)/v3097)))/v3110))))/v3121);
        let v3133=(v1834-v3077);
        let v3134=(v1835-v3078);
        let v3135=(v1836-v3079);
        let v3136=(v1837-v3080);
        let v3137=(v587*v3133);
        let v3139=(v587*v3134);
        let v3141=(v587*v3135);
        let v3143=(v587*v3136);
        let v3145=(v1834+v3077);
        let v3146=(v1835+v3078);
        let v3147=(v1836+v3079);
        let v3148=(v1837+v3080);
        let v3150=(v589*v589);
        let v3151=((-v3145)/v3150);
        let v3153=((-v3146)/v3150);
        let v3155=((-v3147)/v3150);
        let v3157=((-v3148)/v3150);
        let v3160=((v590*(v3137+v3137))+(v588*v3151));
        let v3163=((v590*(v3139+v3139))+(v588*v3153));
        let v3166=((v590*(v3141+v3141))+(v588*v3155));
        let v3169=((v590*(v3143+v3143))+(v588*v3157));
        let v3194=(v33*v3122);
        let v3195=(v33*v3126);
        let v3196=(v33*v3129);
        let v3197=(v33*v3132);
        let v3200=((v591*v3151)+(v590*v3160));
        let v3203=((v591*v3153)+(v590*v3163));
        let v3206=((v591*v3155)+(v590*v3166));
        let v3209=((v591*v3157)+(v590*v3169));
        let v3310=(v109*((v110-v1023)-((v595*v3122)+(v592*(v3145+(v33*v3160))))));
        let v3313=((v597*v835)+(v109*((v881-v1024)-((v595*v3126)+(v592*(v3146+(v33*v3163)))))));
        let v3316=((v597*v836)+(v109*((v882-v1025)-((v595*v3129)+(v592*(v3147+(v33*v3166)))))));
        let v3319=((v597*v837)+(v109*((v883-v1026)-((v595*v3132)+(v592*(v3148+(v33*v3169)))))));
        let v3320=(v621*v3310);
        let v3322=(v621*v3313);
        let v3324=(v621*v3316);
        let v3326=(v621*v3319);
        let v3328=(v5*v624);
        let v3337=(v34*(v3310+((v3320+v3320)/v3328)));
        let v3338=(v34*(v3313+((v3322+v3322)/v3328)));
        let v3339=(v34*(v3316+((v3324+v3324)/v3328)));
        let v3340=(v34*(v3319+((v3326+v3326)/v3328)));
        let v3345=(v109*(((v609*v3194)+(v598*((v1838+v3077)+((v607*v3200)+(v599*(v34*((v601*v1834)+(v604*v3077))))))))+((v619*v3194)+(v598*((v1834+(v5*v3077))+((v617*v3200)+(v599*(v34*((v604*v1834)+(v601*v3077))))))))));
        let v3348=((v627*v835)+(v109*(((v609*v3195)+(v598*((v1839+v3078)+((v607*v3203)+(v599*(v34*((v601*v1835)+(v604*v3078))))))))+((v619*v3195)+(v598*((v1835+(v5*v3078))+((v617*v3203)+(v599*(v34*((v604*v1835)+(v601*v3078)))))))))));
        let v3351=((v627*v836)+(v109*(((v609*v3196)+(v598*((v1840+v3079)+((v607*v3206)+(v599*(v34*((v601*v1836)+(v604*v3079))))))))+((v619*v3196)+(v598*((v1836+(v5*v3079))+((v617*v3206)+(v599*(v34*((v604*v1836)+(v601*v3079)))))))))));
        let v3354=((v627*v837)+(v109*(((v609*v3197)+(v598*((v1841+v3080)+((v607*v3209)+(v599*(v34*((v601*v1837)+(v604*v3080))))))))+((v619*v3197)+(v598*((v1837+(v5*v3080))+((v617*v3209)+(v599*(v34*((v604*v1837)+(v601*v3080)))))))))));
        let v3370=(v626*v626);
        let v3389=(self.scalar_static_f64[34]*f64::powf(v634,self.scalar_static_f64[88]));
        let v3395=(self.scalar_static_f64[37]*f64::powf(v631,self.scalar_static_f64[89]));
        let v3410=(v635*v635);
        let v3421=((v346*((self.scalar_static_f64[33]*(v3337+(self.scalar_static_f64[8]*v3345)))*v3395))+((-(self.scalar_static_f64[38]*((v34*(((v626*v3345)-(v628*v3337))/v3370))*v3389)))/v3410));
        let v3422=((v346*((self.scalar_static_f64[33]*(v3338+(self.scalar_static_f64[8]*v3348)))*v3395))+((-(self.scalar_static_f64[38]*((v34*(((v626*v3348)-(v628*v3338))/v3370))*v3389)))/v3410));
        let v3423=(((v636*(self.scalar_static_f64[36]*v814))+(v346*((self.scalar_static_f64[33]*(v3339+(self.scalar_static_f64[8]*v3351)))*v3395)))+((-(self.scalar_static_f64[38]*((v34*(((v626*v3351)-(v628*v3339))/v3370))*v3389)))/v3410));
        let v3424=(((v636*(self.scalar_static_f64[36]*v815))+(v346*((self.scalar_static_f64[33]*(v3340+(self.scalar_static_f64[8]*v3354)))*v3395)))+((-(self.scalar_static_f64[38]*((v34*(((v626*v3354)-(v628*v3340))/v3370))*v3389)))/v3410));
        let v3425=(v642*v3421);
        let v3427=(v642*v3422);
        let v3429=(v642*v3423);
        let v3431=(v642*v3424);
        let v3433=(v5*v645);
        let v3442=(v34*(v3421+((v3425+v3425)/v3433)));
        let v3443=(v34*(v3422+((v3427+v3427)/v3433)));
        let v3444=(v34*(v3423+((v3429+v3429)/v3433)));
        let v3445=(v34*(v3424+((v3431+v3431)/v3433)));
        let v3448=(v647*v647);
        let v3468=((v649*v3133)+(v587*(v5*((-(v358*v3442))/v3448))));
        let v3471=((v649*v3134)+(v587*(v5*(((v647*v1916)-(v358*v3443))/v3448))));
        let v3474=((v649*v3135)+(v587*(v5*(((v647*v1917)-(v358*v3444))/v3448))));
        let v3477=((v649*v3136)+(v587*(v5*(((v647*v1918)-(v358*v3445))/v3448))));
        let v3478=(v650*v3468);
        let v3480=(v650*v3471);
        let v3482=(v650*v3474);
        let v3484=(v650*v3477);
        let v3486=(v5*v653);
        let v3487=((v3478+v3478)/v3486);
        let v3488=((v3480+v3480)/v3486);
        let v3489=((v3482+v3482)/v3486);
        let v3490=((v3484+v3484)/v3486);
        let v3528=(v653*v653);
        let v3550=((v666*v3442)+(v647*(if v662{(v34*(v3487+((-v3487)/v3528)))}else{(if (v655!=0.0){(v34*(v3487+((v657*((-v3468)/v651))+(v656*(v3468/v653)))))}else{v9})})));
        let v3553=((v666*v3443)+(v647*(if v662{(v34*(v3488+((-v3488)/v3528)))}else{(if (v655!=0.0){(v34*(v3488+((v657*((-v3471)/v651))+(v656*(v3471/v653)))))}else{v9})})));
        let v3556=((v666*v3444)+(v647*(if v662{(v34*(v3489+((-v3489)/v3528)))}else{(if (v655!=0.0){(v34*(v3489+((v657*((-v3474)/v651))+(v656*(v3474/v653)))))}else{v9})})));
        let v3559=((v666*v3445)+(v647*(if v662{(v34*(v3490+((-v3490)/v3528)))}else{(if (v655!=0.0){(v34*(v3490+((v657*((-v3477)/v651))+(v656*(v3477/v653)))))}else{v9})})));
        let v3562=(v667*v667);
        let v3594=(-(self.scalar_static_f64[51]*(((v2293+(((v2303+v2303)+(self.scalar_static_f64[52]*v2293))/v2319))/self.scalar_static_f64[53])/v451)));
        let v3595=(-(self.scalar_static_f64[51]*(((v2296+(((v2305+v2305)+(self.scalar_static_f64[52]*v2296))/v2319))/self.scalar_static_f64[53])/v451)));
        let v3596=(-(self.scalar_static_f64[51]*(((v2299+(((v2307+v2307)+(self.scalar_static_f64[52]*v2299))/v2319))/self.scalar_static_f64[53])/v451)));
        let v3597=(-(self.scalar_static_f64[51]*(((v2302+(((v2309+v2309)+(self.scalar_static_f64[52]*v2302))/v2319))/self.scalar_static_f64[53])/v451)));
        let v3601=(v674*v674);
        let v3653=((v679*(v109*(v109*(self.scalar_static_f64[15]*(((v674*(self.scalar_static_f64[11]*((v671*((-(v39*v3550))/v3562))+(v668*(self.scalar_static_f64[55]*v3122)))))-(v673*v3594))/v3601)))))+(v678*((v589*v3133)+(v587*v3145))));
        let v3656=((v679*((v677*v835)+(v109*((v676*v835)+(v109*(self.scalar_static_f64[15]*(((v674*(self.scalar_static_f64[11]*((v671*(((v667*(self.scalar_static_f64[9]*(self.scalar_static_f64[77]*(self.scalar_static_f64[10]*f64::powf(v15,self.scalar_static_f64[78])))))-(v39*v3553))/v3562))+(v668*(self.scalar_static_f64[55]*v3126)))))-(v673*v3595))/v3601)))))))+(v678*((v589*v3134)+(v587*v3146))));
        let v3659=((v679*((v677*v836)+(v109*((v676*v836)+(v109*(self.scalar_static_f64[15]*(((v674*(self.scalar_static_f64[11]*((v671*((-(v39*v3556))/v3562))+(v668*(self.scalar_static_f64[55]*v3129)))))-(v673*v3596))/v3601)))))))+(v678*((v589*v3135)+(v587*v3147))));
        let v3662=((v679*((v677*v837)+(v109*((v676*v837)+(v109*(self.scalar_static_f64[15]*(((v674*(self.scalar_static_f64[11]*((v671*((-(v39*v3559))/v3562))+(v668*(self.scalar_static_f64[55]*v3132)))))-(v673*v3597))/v3601)))))))+(v678*((v589*v3136)+(v587*v3148))));
        let v3675=(self.scalar_static_f64[63]*(self.scalar_static_f64[60]*(self.scalar_static_f64[77]*(self.scalar_static_f64[61]*f64::powf(v15,self.scalar_static_f64[94])))));
        let v3678=(v697*v697);
        let v3704=(self.scalar_static_f64[66]*f64::powf(v708,self.scalar_static_f64[95]));
        let v3715=(self.scalar_static_f64[67]*f64::powf(v711,self.scalar_static_f64[96]));
        let v3716=((-((if (v707!=0.0){v9}else{(v3653/v703)})*v3704))*v3715);
        let v3717=((-((if (v707!=0.0){v9}else{(((v703*v3656)-(v680*(self.scalar_static_f64[63]*(self.scalar_static_f64[57]*(self.scalar_static_f64[92]*(self.scalar_static_f64[59]*f64::powf(v686,self.scalar_static_f64[93])))))))/(v703*v703))})*v3704))*v3715);
        let v3718=((-((if (v707!=0.0){v9}else{(v3659/v703)})*v3704))*v3715);
        let v3719=((-((if (v707!=0.0){v9}else{(v3662/v703)})*v3704))*v3715);
        let v3722=(v713*v713);
        let v3723=((-(v699*v3716))/v3722);
        let v3730=((-(v699*v3718))/v3722);
        let v3733=((-(v699*v3719))/v3722);
        let v3736=((-(v702*v3716))/v3722);
        let v3743=((-(v702*v3718))/v3722);
        let v3746=((-(v702*v3719))/v3722);
        let v3748=(self.scalar_static_f64[77]*f64::powf(v15,v9));
        let v3751=((((v713*((v698*self.scalar_static_f64[92])+(v686*((-(self.scalar_static_f64[64]*v3675))/v3678))))-(v699*v3717))/v3722)+(self.scalar_static_f64[71]*(self.scalar_static_f64[72]*v3748)));
        let v3754=((((v713*((v701*self.scalar_static_f64[92])+(v686*((-(self.scalar_static_f64[65]*v3675))/v3678))))-(v702*v3717))/v3722)+(self.scalar_static_f64[69]*(self.scalar_static_f64[73]*v3748)));
        let v3822=(v738*v738);
        let v3823=(((v738*v3653)-(v680*((v736*((v734*v3345)+(v628*(((v674*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3550))/v3562))))-(v733*v3594))/v3601))))+(v735*(v3723+v3736)))))/v3822);
        let v3827=(((v738*v3656)-(v680*((v736*((v734*v3348)+(v628*(((v674*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3553))/v3562))))-(v733*v3595))/v3601))))+(v735*(v3751+v3754)))))/v3822);
        let v3831=(((v738*v3659)-(v680*((v736*((v734*v3351)+(v628*(((v674*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3556))/v3562))))-(v733*v3596))/v3601))))+(v735*(v3730+v3743)))))/v3822);
        let v3835=(((v738*v3662)-(v680*((v736*((v734*v3354)+(v628*(((v674*(self.scalar_static_f64[11]*(self.scalar_static_f64[15]*((-(self.scalar_static_f64[9]*v3559))/v3562))))-(v733*v3597))/v3601))))+(v735*(v3733+v3746)))))/v3822);
        let v3867=(v739*v3823);
        let v3868=(v3867+v3867);
        let v3869=(v739*v3827);
        let v3870=(v3869+v3869);
        let v3871=(v739*v3831);
        let v3872=(v3871+v3871);
        let v3873=(v739*v3835);
        let v3874=(v3873+v3873);

        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            (v730*v739),
            [1, 4, 5, 6],
            [((v739*v3736)+(v730*v3823)), ((v739*v3754)+(v730*v3827)), ((v739*v3743)+(v730*v3831)), ((v739*v3746)+(v730*v3835))],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v739),
            [1, 4, 5, 6],
            [v3823, v3827, v3831, v3835],
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
            (v725*v739),
            [1, 4, 5, 6],
            [((v739*v3723)+(v725*v3823)), ((v739*v3751)+(v725*v3827)), ((v739*v3730)+(v725*v3831)), ((v739*v3733)+(v725*v3835))],
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
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){v755}else{v9})),
            4,
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(self.scalar_static_f64[76]*ddt_scale)}else{v9})),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v739)+(v725*v746))+(v730*v746)))}else{v9})}else{v9})),
            [1, 4, 5, 6],
            [(if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v3823)+((v746*v3723)+(v725*v3868)))+((v746*v3736)+(v730*v3868))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v65*v3827)+((v746*v3751)+(v725*v3870)))+((v746*v3754)+(v730*v3870))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-(((v739+(v65*v3831))+((v746*v3730)+(v725*v3872)))+((v746*v3743)+(v730*v3872))))}else{v9})}else{v9}), (if (self.scalar_static_f64[75]!=0.0){(if (self.scalar_static_f64[75]!=0.0){(-((((-v739)+(v65*v3835))+((v746*v3733)+(v725*v3874)))+((v746*v3746)+(v730*v3874))))}else{v9})}else{v9})],
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
        let v755=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if (self.scalar_static_f64[75]!=0.0){(self.scalar_static_f64[76]*1.0)}else{v9})),
        );
    }
}
