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
    v1: f64,
    v2: f64,
    v10: f64,
    v40: f64,
    v45: f64,
    v96: f64,
    v125: f64,
    v143: f64,
    v162: f64,
    v166: f64,
    v173: f64,
    v375: f64,
    v376: f64,
    v377: f64,
    v381: f64,
    v410: f64,
    v418: f64,
    v455: f64,
    v456: f64,
    v457: f64,
    v463: f64,
    v479: f64,
    v483: f64,
    v489: f64,
    v511: f64,
    v515: f64,
    v559: f64,
    v567: f64,
    v634: f64,
    v642: f64,
    v1204: f64,
    v1538: f64,
    v1545: bool,
    v1548: bool,
    v1552: f64,
    v1564: bool,
    v1567: f64,
    v1575: bool,
    v1601: bool,
    v1629: f64,
    v1631: f64,
    v1642: f64,
    v2390: f64,
    v2530: f64,
    v2577: f64,
    v3345: f64,
    v3347: f64,
    v3348: f64,
    v3350: f64,
    v3506: f64,
    v3507: f64,
    v3508: f64,
    v3509: f64,
    v3511: f64,
    v3513: f64,
    v3526: f64,
    v3527: f64,
    v3557: f64,
    v3558: f64,
    v5169: f64,
    v5170: f64,
    v6027: f64,
    v6028: f64,
    v6038: f64,
    v6039: f64,
    v6067: f64,
    v6068: f64,
    v6219: f64,
    v6220: f64,
    v6224: f64,
    v6225: f64,
    v8732: f64,
    v8733: f64,
    v8734: f64,
    v8762: f64,
    v9083: f64,
    v9084: f64,
    v9085: f64,
    v10654: f64,
    v10655: f64,
    v10656: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=1.0;
        let v2=0.0;
        let v10=2.0;
        let v12=3.348580862e-29;
        let v27=0.6666666666666666;
        let v40=0.3333333333333333;
        let v45=0.5;
        let v96=4.0;
        let v125=0.001;
        let v143=6.0;
        let v150=0.7071067811865475;
        let v154=1e-5;
        let v162=1.25;
        let v163=0.7324648775608221;
        let v166=460.51701859880916;
        let v173=1e-200;
        let v300=1.3333333333333333;
        let v375=ctx.node_voltage(nodes[4]);
        let v376=ctx.node_voltage(nodes[5]);
        let v377=(v375-v376);
        let v380=(self.scalar_static_f64[20]*(v377-self.scalar_static_f64[142]));
        let v381=1e-16;
        let v382=(v380>v381);
        let v386=(((v380*v380)+self.scalar_static_f64[143])).sqrt();
        let v389=(v2-v380);
        let v390=(v389>v381);
        let v394=((self.scalar_static_f64[143]+(v389*v389))).sqrt();
        let v395=(v389+v394);
        let v405=(v1+(self.scalar_static_f64[141]*(if v382{(v45*(v380+v386))}else{(if v390{(self.scalar_static_f64[144]/v395)}else{(v45*(v380+self.scalar_static_f64[146]))})})));
        let v407=(self.scalar_static_f64[147]-v405);
        let v408=(v407>v381);
        let v410=1e-6;
        let v412=(((v407*v407)+v410)).sqrt();
        let v416=(v405-self.scalar_static_f64[147]);
        let v417=(v416>v381);
        let v418=5e-7;
        let v421=((v410+(v416*v416))).sqrt();
        let v422=(v416+v421);
        let v430=(self.scalar_static_f64[5]*(if v408{(self.scalar_static_f64[147]-(v45*(v407+v412)))}else{(if v417{(self.scalar_static_f64[147]-(v418/v422))}else{(self.scalar_static_f64[147]-(v45*(v125+v407)))})}));
        let v431=1e23;
        let v433=(self.scalar_static_f64[206]*v430);
        let v436=(self.scalar_static_f64[191]+(self.scalar_static_f64[207]*(v433).ln()));
        let v438=((v12*v430)).sqrt();
        let v439=(v438/self.scalar_static_f64[4]);
        let v440=(v439*v439);
        let v442=((v436*v440)).sqrt();
        let v443=(if (self.scalar_static_f64[15]!=0.0){v442}else{v2});
        let v448=(if (self.scalar_static_f64[15]!=0.0){(self.scalar_static_f64[148]*f64::powf(v443,v27))}else{v2});
        let v451=(v300*v448);
        let v453=(v1+(v451/v443));
        let v455=(if (self.scalar_static_f64[15]!=0.0){(v439*v453)}else{v439});
        let v456=(self.scalar_static_f64[218]*v455);
        let v457=(v456*v456);
        let v458=(v1/v457);
        let v460=(v1+(v150*v456));
        let v461=(v1/v460);
        let v462=(v154*v460);
        let v463=(self.scalar_static_f64[175]*(if (self.scalar_static_f64[15]!=0.0){(v436+v448)}else{v436}));
        let v465=(if (v463<v166){v1}else{v2});
        let v467=((-v463)).exp();
        let v469=(!(v465!=0.0));
        let v470=(v463-v166);
        let v471=(v45*v470);
        let v473=(v1+(v40*v470));
        let v475=(v1+(v471*v473));
        let v477=(v1+(v470*v475));
        let v479=(if v469{(v173/v477)}else{(if (v465!=0.0){v467}else{v2})});
        let v481=(v162+(v163*v456));
        let v483=(self.scalar_static_f64[20]*(v377-self.scalar_static_f64[177]));
        let v484=(self.scalar_static_f64[175]*v483);
        let v487=(if ((v484).abs()<=v462){v1}else{v2});
        let v489=0.1666666666666667;
        let v491=(v150*((v461*v461)*v489));
        let v492=(if (v487!=0.0){v491}else{v2});
        let v493=(v461*v484);
        let v494=(v1-v479);
        let v495=(v484*v494);
        let v496=(v456*v495);
        let v498=(v1+(v492*v496));
        let v501=(-v462);
        let v503=(if (v484<v501){v1}else{v2});
        let v504=(!(v487!=0.0));
        let v505=((v503!=0.0)&&v504);
        let v507=(if v505{(-v484)}else{v2});
        let v508=(v162*v507);
        let v510=(if v505{(v461*v508)}else{v2});
        let v511=10.0;
        let v513=(v510-v143);
        let v515=64.0;
        let v517=(((v513*v513)+v515)).sqrt();
        let v520=(if v505{(v45*((v510+v511)-v517))}else{v2});
        let v522=(if v505{(v507-v520)}else{v2});
        let v524=(v1+v520);
        let v527=(if v505{((v522*v522)+(v457*v524))}else{v2});
        let v530=(if v505{((v10*v522)-v457)}else{v2});
        let v532=(v458*v527);
        let v535=(if v505{((-v520)+(v532).ln())}else{v2});
        let v537=(if v505{(v527+v530)}else{v2});
        let v539=(v45*v530);
        let v541=((v530*v539)-v527);
        let v544=(if v505{((v537*v537)+(v535*v541))}else{v2});
        let v545=(v527*v537);
        let v546=(v535*v545);
        let v547=(v535*v537);
        let v548=(v535*v547);
        let v549=(v548/v544);
        let v550=(v530*v549);
        let v553=((v40*(v530*v530))-v527);
        let v555=(v544+(v550*v553));
        let v558=(if v505{(v520+(v546/v555))}else{v2});
        let v559=230.25850929940458;
        let v561=(if (v558<v559){v1}else{v2});
        let v562=(v505&&(v561!=0.0));
        let v563=(v558).exp();
        let v566=(v505&&(!(v561!=0.0)));
        let v567=1e100;
        let v568=(v558-v559);
        let v569=(v45*v568);
        let v571=(v1+(v40*v568));
        let v573=(v1+(v569*v571));
        let v577=(if v566{(v567*(v1+(v568*v573)))}else{(if v562{v563}else{v2})});
        let v579=(if v505{(v1/v577)}else{v2});
        let v581=(v10+(v558*v558));
        let v585=(if v505{(v507-v558)}else{(if v505{(v1/v581)}else{v522})});
        let v587=(if v505{(v479*v579)}else{v492});
        let v591=(v479+((v577-v1)-v587));
        let v594=(if v505{((v10*v585)+(v457*v591))}else{v2});
        let v599=(v558-v1);
        let v601=((v587+((v577-v558)-v1))+(v479*v599));
        let v604=(if v505{((v585*v585)-(v457*v601))}else{v2});
        let v605=(v577+v587);
        let v608=(if v505{(v10-(v457*v605))}else{v585});
        let v610=(v10*v604);
        let v613=(if v505{((v594*v594)-(v608*v610))}else{v608});
        let v615=(v613).sqrt();
        let v616=(v594+v615);
        let v621=(v504&&(!(v503!=0.0)));
        let v622=(v1/v481);
        let v623=(if v621{v622}else{v2});
        let v624=(v162*v460);
        let v626=((v623*v624)-v1);
        let v628=(if v621{(v623*v626)}else{v2});
        let v630=(v1+(v484*v628));
        let v633=(-(if v621{(v493*v630)}else{v2}));
        let v634=-230.25850929940458;
        let v636=(if (v633>v634){v1}else{v2});
        let v637=(v621&&(v636!=0.0));
        let v638=(v633).exp();
        let v641=(v621&&(!(v636!=0.0)));
        let v642=1e-100;
        let v643=(v634-v633);
        let v644=(v45*v643);
        let v646=(v1+(v40*v643));
        let v648=(v1+(v644*v646));
        let v650=(v1+(v643*v648));
        let v652=(if v641{(v642/v650)}else{(if v637{v638}else{v613})});
        let v655=(v45*v457);
        let v657=0.25;
        let v658=(v457*v657);
        let v661=(((v484+v658)-(if v621{(v1-v652)}else{v2}))).sqrt();
        let v664=(if v621{((v484+v655)-(v456*v661))}else{v2});
        let v665=(3.0+v463);
        let v666=(if v621{v665}else{v2});
        let v667=(v666-v664);
        let v668=(v667>v381);
        let v670=5.0;
        let v672=(((v667*v667)+v670)).sqrt();
        let v676=(v664-v666);
        let v677=(v676>v381);
        let v678=2.5;
        let v681=((v670+(v676*v676))).sqrt();
        let v682=(v676+v681);
        let v685=2.23606797749979;
        let v693=((v670+(v666*v666))).sqrt();
        let v697=(if v621{((if v668{(v666-(v45*(v667+v672)))}else{(if v677{(v666-(v678/v682))}else{(v666-(v45*(v667+v685)))})})-(v45*(v666-v693)))}else{v520});
        let v699=(if v621{(v484-v697)}else{v652});
        let v701=((-v697)).exp();
        let v702=(if v621{v701}else{v587});
        let v703=1e-40;
        let v707=(v1+v697);
        let v709=(((v697+v702)-v1)-(v479*v707));
        let v711=((v699*v699)-(v457*v709));
        let v712=(v703>v711);
        let v714=(if v621{(if v712{v703}else{v711})}else{v527});
        let v717=(if v621{(v1-(v655*v702))}else{v2});
        let v720=((v1-v702)-v479);
        let v723=(if v621{((v10*v699)+(v457*v720))}else{v530});
        let v725=(v714/v457);
        let v728=(if v621{((v463-v697)+(v725).ln())}else{v535});
        let v730=(if v621{(v714+v723)}else{v2});
        let v732=1e-120;
        let v734=(if ((v728).abs()<v732){v1}else{v2});
        let v735=(v621&&(v734!=0.0));
        let v738=(v621&&(!(v734!=0.0)));
        let v740=(v45*v723);
        let v742=(v714*v717);
        let v743=((v723*v740)-v742);
        let v746=(if v738{((v730*v730)+(v728*v743))}else{v2});
        let v747=(v714*v730);
        let v748=(v728*v747);
        let v749=(v728*v730);
        let v750=(v728*v749);
        let v751=(v750/v746);
        let v752=(v723*v751);
        let v755=((v40*(v723*v723))-v742);
        let v757=(v746+(v752*v755));
        let v760=(if v738{(v697+(v748/v757))}else{(if v735{v697}else{v2})});
        let v762=(if (v760<v559){v1}else{v2});
        let v763=(v621&&(v762!=0.0));
        let v764=(v760).exp();
        let v765=(if v763{v764}else{v577});
        let v770=(v463-v559);
        let v772=(if (v760>v770){v1}else{v2});
        let v774=(v621&&(!(v762!=0.0)));
        let v775=((v772!=0.0)&&v774);
        let v777=((v760-v463)).exp();
        let v778=(if v775{v777}else{(if v763{(v479*v765)}else{v765})});
        let v782=(v774&&(!(v772!=0.0)));
        let v784=((v463-v760)-v559);
        let v785=(v45*v784);
        let v787=(v1+(v40*v784));
        let v789=(v1+(v785*v787));
        let v791=(v1+(v784*v789));
        let v793=(if v782{(v642/v791)}else{v778});
        let v794=(v760-v559);
        let v795=(v45*v794);
        let v797=(v1+(v40*v794));
        let v799=(v1+(v795*v797));
        let v801=(v1+(v794*v799));
        let v803=(if v782{(v642/v801)}else{(if v775{(v479/v778)}else{(if v763{(v1/v765)}else{v579})})});
        let v805=(v10+(v760*v760));
        let v809=(if v621{(v484-v760)}else{(if v621{(v1/v805)}else{v699})});
        let v813=((v793+(v1-v803))-v479);
        let v816=(if v621{((v10*v809)+(v457*v813))}else{v594});
        let v821=(v1+v760);
        let v823=((v793+((v760+v803)-v1))-(v479*v821));
        let v827=(v793+v803);
        let v830=(if v621{(v10-(v457*v827))}else{v809});
        let v832=(v10*(if v621{((v809*v809)-(v457*v823))}else{v604}));
        let v836=((if v621{((v816*v816)-(v830*v832))}else{v830})).sqrt();
        let v837=(v816+v836);
        let v840=(if v621{(v760+(v832/v837))}else{(if v505{((-v558)-(v610/v616))}else{(if (v487!=0.0){(v493*v498)}else{v2})})});
        let v851=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(v483-(self.scalar_static_f64[172]*v840))))}else{v2});
        let v854=(if ((v851).abs()<=self.scalar_static_f64[225]){v1}else{v2});
        let v855=((self.scalar_static_f64[149]!=0.0)&&(v854!=0.0));
        let v859=(if v855{self.scalar_static_f64[322]}else{v2});
        let v860=(self.scalar_static_f64[224]*v851);
        let v865=(v1+(v859*(self.scalar_static_f64[219]*(v851*self.scalar_static_f64[323]))));
        let v870=(if (v851<self.scalar_static_f64[324]){v1}else{v2});
        let v872=((self.scalar_static_f64[149]!=0.0)&&(!(v854!=0.0)));
        let v873=((v870!=0.0)&&v872);
        let v875=(if v873{(-v851)}else{v2});
        let v878=(if v873{(self.scalar_static_f64[224]*(v162*v875))}else{v2});
        let v880=(v878-v143);
        let v883=((v515+(v880*v880))).sqrt();
        let v886=(if v873{(v45*((v511+v878)-v883))}else{v2});
        let v888=(if v873{(v875-v886)}else{v2});
        let v893=(if v873{((v888*v888)+(self.scalar_static_f64[220]*(v1+v886)))}else{v2});
        let v896=(if v873{((v10*v888)-self.scalar_static_f64[220])}else{v2});
        let v898=(self.scalar_static_f64[221]*v893);
        let v901=(if v873{((-v886)+(v898).ln())}else{v2});
        let v903=(if v873{(v893+v896)}else{v2});
        let v905=(v45*v896);
        let v907=((v896*v905)-v893);
        let v910=(if v873{((v903*v903)+(v901*v907))}else{v2});
        let v911=(v893*v903);
        let v912=(v901*v911);
        let v913=(v901*v903);
        let v914=(v901*v913);
        let v915=(v914/v910);
        let v916=(v896*v915);
        let v919=((v40*(v896*v896))-v893);
        let v921=(v910+(v916*v919));
        let v924=(if v873{(v886+(v912/v921))}else{v2});
        let v926=(if (v924<v559){v1}else{v2});
        let v927=(v873&&(v926!=0.0));
        let v928=(v924).exp();
        let v931=(v873&&(!(v926!=0.0)));
        let v932=(v924-v559);
        let v933=(v45*v932);
        let v935=(v1+(v40*v932));
        let v937=(v1+(v933*v935));
        let v941=(if v931{(v567*(v1+(v932*v937)))}else{(if v927{v928}else{v2})});
        let v943=(if v873{(v1/v941)}else{v2});
        let v945=(v10+(v924*v924));
        let v949=(if v873{(v875-v924)}else{(if v873{(v1/v945)}else{v888})});
        let v951=(if v873{(self.scalar_static_f64[247]*v943)}else{v859});
        let v958=(if v873{((v10*v949)+(self.scalar_static_f64[220]*(self.scalar_static_f64[247]+((v941-v1)-v951))))}else{v2});
        let v968=(if v873{((v949*v949)-(self.scalar_static_f64[220]*((v951+((v941-v924)-v1))+(self.scalar_static_f64[247]*(v924-v1)))))}else{v2});
        let v972=(if v873{(v10-(self.scalar_static_f64[220]*(v941+v951)))}else{v949});
        let v974=(v10*v968);
        let v977=(if v873{((v958*v958)-(v972*v974))}else{v972});
        let v979=(v977).sqrt();
        let v980=(v958+v979);
        let v985=(v872&&(!(v870!=0.0)));
        let v989=(if v985{self.scalar_static_f64[327]}else{v2});
        let v994=(if v985{(v989*((v989*self.scalar_static_f64[328])-v1))}else{v2});
        let v996=(v1+(v851*v994));
        let v999=(-(if v985{(v860*v996)}else{v2}));
        let v1001=(if (v999>v634){v1}else{v2});
        let v1002=(v985&&(v1001!=0.0));
        let v1003=(v999).exp();
        let v1006=(v985&&(!(v1001!=0.0)));
        let v1007=(v634-v999);
        let v1008=(v45*v1007);
        let v1010=(v1+(v40*v1007));
        let v1012=(v1+(v1008*v1010));
        let v1014=(v1+(v1007*v1012));
        let v1016=(if v1006{(v642/v1014)}else{(if v1002{v1003}else{v977})});
        let v1024=(((v851+self.scalar_static_f64[330])-(if v985{(v1-v1016)}else{v2}))).sqrt();
        let v1027=(if v985{((v851+self.scalar_static_f64[329])-(self.scalar_static_f64[219]*v1024))}else{v2});
        let v1029=(if v985{self.scalar_static_f64[331]}else{v2});
        let v1030=(v1029-v1027);
        let v1031=(v1030>v381);
        let v1034=((v670+(v1030*v1030))).sqrt();
        let v1038=(v1027-v1029);
        let v1039=(v1038>v381);
        let v1042=((v670+(v1038*v1038))).sqrt();
        let v1043=(v1038+v1042);
        let v1057=(if v985{((if v1031{(v1029-(v45*(v1030+v1034)))}else{(if v1039{(v1029-(v678/v1043))}else{(v1029-(v45*(v685+v1030)))})})-(v45*(v1029-((v670+(v1029*v1029))).sqrt())))}else{v886});
        let v1059=(if v985{(v851-v1057)}else{v1016});
        let v1061=((-v1057)).exp();
        let v1062=(if v985{v1061}else{v951});
        let v1070=((v1059*v1059)-(self.scalar_static_f64[220]*(((v1057+v1062)-v1)-(self.scalar_static_f64[247]*(v1+v1057)))));
        let v1071=(v703>v1070);
        let v1073=(if v985{(if v1071{v703}else{v1070})}else{v893});
        let v1076=(if v985{(v1-(self.scalar_static_f64[329]*v1062))}else{v2});
        let v1082=(if v985{((v10*v1059)+(self.scalar_static_f64[220]*((v1-v1062)-self.scalar_static_f64[247])))}else{v896});
        let v1084=(v1073/self.scalar_static_f64[220]);
        let v1087=(if v985{((self.scalar_static_f64[226]-v1057)+(v1084).ln())}else{v901});
        let v1089=(if v985{(v1073+v1082)}else{v2});
        let v1092=(if ((v1087).abs()<v732){v1}else{v2});
        let v1093=(v985&&(v1092!=0.0));
        let v1096=(v985&&(!(v1092!=0.0)));
        let v1098=(v45*v1082);
        let v1100=(v1073*v1076);
        let v1101=((v1082*v1098)-v1100);
        let v1104=(if v1096{((v1089*v1089)+(v1087*v1101))}else{v2});
        let v1105=(v1073*v1089);
        let v1106=(v1087*v1105);
        let v1107=(v1087*v1089);
        let v1108=(v1087*v1107);
        let v1109=(v1108/v1104);
        let v1110=(v1082*v1109);
        let v1113=((v40*(v1082*v1082))-v1100);
        let v1115=(v1104+(v1110*v1113));
        let v1118=(if v1096{(v1057+(v1106/v1115))}else{(if v1093{v1057}else{v2})});
        let v1120=(if (v1118<v559){v1}else{v2});
        let v1121=(v985&&(v1120!=0.0));
        let v1122=(v1118).exp();
        let v1123=(if v1121{v1122}else{v941});
        let v1130=(if (v1118>self.scalar_static_f64[332]){v1}else{v2});
        let v1132=(v985&&(!(v1120!=0.0)));
        let v1133=((v1130!=0.0)&&v1132);
        let v1135=((v1118-self.scalar_static_f64[226])).exp();
        let v1136=(if v1133{v1135}else{(if v1121{(self.scalar_static_f64[247]*v1123)}else{v1123})});
        let v1140=(v1132&&(!(v1130!=0.0)));
        let v1142=((self.scalar_static_f64[226]-v1118)-v559);
        let v1143=(v45*v1142);
        let v1145=(v1+(v40*v1142));
        let v1147=(v1+(v1143*v1145));
        let v1149=(v1+(v1142*v1147));
        let v1151=(if v1140{(v642/v1149)}else{v1136});
        let v1152=(v1118-v559);
        let v1153=(v45*v1152);
        let v1155=(v1+(v40*v1152));
        let v1157=(v1+(v1153*v1155));
        let v1159=(v1+(v1152*v1157));
        let v1161=(if v1140{(v642/v1159)}else{(if v1133{(self.scalar_static_f64[247]/v1136)}else{(if v1121{(v1/v1123)}else{v943})})});
        let v1163=(v10+(v1118*v1118));
        let v1167=(if v985{(v851-v1118)}else{(if v985{(v1/v1163)}else{v1059})});
        let v1174=(if v985{((v10*v1167)+(self.scalar_static_f64[220]*((v1151+(v1-v1161))-self.scalar_static_f64[247])))}else{v958});
        let v1188=(if v985{(v10-(self.scalar_static_f64[220]*(v1151+v1161)))}else{v1167});
        let v1190=(v10*(if v985{((v1167*v1167)-(self.scalar_static_f64[220]*((v1151+((v1118+v1161)-v1))-(self.scalar_static_f64[247]*(v1+v1118)))))}else{v968}));
        let v1194=((if v985{((v1174*v1174)-(v1188*v1190))}else{v1188})).sqrt();
        let v1195=(v1174+v1194);
        let v1204=(if (self.scalar_static_f64[149]!=0.0){((v483-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v985{(v1118+(v1190/v1195))}else{(if v873{((-v924)-(v974/v980))}else{(if v855{(v860*v865)}else{v2})})})))}else{v2}))/self.scalar_static_f64[172])}else{v484});
        let v1207=(if ((v1204).abs()<=v462){v1}else{v2});
        let v1208=((self.scalar_static_f64[149]!=0.0)&&(v1207!=0.0));
        let v1209=(if v1208{v491}else{v2});
        let v1210=(v461*v1204);
        let v1211=(v494*v1204);
        let v1212=(v456*v1211);
        let v1214=(v1+(v1209*v1212));
        let v1218=(if (v1204<v501){v1}else{v2});
        let v1220=((self.scalar_static_f64[149]!=0.0)&&(!(v1207!=0.0)));
        let v1221=((v1218!=0.0)&&v1220);
        let v1223=(if v1221{(-v1204)}else{v2});
        let v1224=(v162*v1223);
        let v1226=(if v1221{(v461*v1224)}else{v2});
        let v1228=(v1226-v143);
        let v1231=((v515+(v1228*v1228))).sqrt();
        let v1234=(if v1221{(v45*((v511+v1226)-v1231))}else{v2});
        let v1236=(if v1221{(v1223-v1234)}else{v2});
        let v1238=(v1+v1234);
        let v1241=(if v1221{((v1236*v1236)+(v457*v1238))}else{v2});
        let v1244=(if v1221{((v10*v1236)-v457)}else{v2});
        let v1246=(v458*v1241);
        let v1249=(if v1221{((-v1234)+(v1246).ln())}else{v2});
        let v1251=(if v1221{(v1241+v1244)}else{v2});
        let v1253=(v45*v1244);
        let v1255=((v1244*v1253)-v1241);
        let v1258=(if v1221{((v1251*v1251)+(v1249*v1255))}else{v2});
        let v1259=(v1241*v1251);
        let v1260=(v1249*v1259);
        let v1261=(v1249*v1251);
        let v1262=(v1249*v1261);
        let v1263=(v1262/v1258);
        let v1264=(v1244*v1263);
        let v1267=((v40*(v1244*v1244))-v1241);
        let v1269=(v1258+(v1264*v1267));
        let v1272=(if v1221{(v1234+(v1260/v1269))}else{v2});
        let v1274=(if (v1272<v559){v1}else{v2});
        let v1275=(v1221&&(v1274!=0.0));
        let v1276=(v1272).exp();
        let v1279=(v1221&&(!(v1274!=0.0)));
        let v1280=(v1272-v559);
        let v1281=(v45*v1280);
        let v1283=(v1+(v40*v1280));
        let v1285=(v1+(v1281*v1283));
        let v1289=(if v1279{(v567*(v1+(v1280*v1285)))}else{(if v1275{v1276}else{v2})});
        let v1291=(if v1221{(v1/v1289)}else{v2});
        let v1293=(v10+(v1272*v1272));
        let v1297=(if v1221{(v1223-v1272)}else{(if v1221{(v1/v1293)}else{v1236})});
        let v1299=(if v1221{(v479*v1291)}else{v1209});
        let v1303=(v479+((v1289-v1)-v1299));
        let v1306=(if v1221{((v10*v1297)+(v457*v1303))}else{v2});
        let v1311=(v1272-v1);
        let v1313=((v1299+((v1289-v1272)-v1))+(v479*v1311));
        let v1316=(if v1221{((v1297*v1297)-(v457*v1313))}else{v2});
        let v1317=(v1289+v1299);
        let v1320=(if v1221{(v10-(v457*v1317))}else{v1297});
        let v1322=(v10*v1316);
        let v1325=(if v1221{((v1306*v1306)-(v1320*v1322))}else{v1320});
        let v1327=(v1325).sqrt();
        let v1328=(v1306+v1327);
        let v1333=(v1220&&(!(v1218!=0.0)));
        let v1334=(if v1333{v622}else{v2});
        let v1336=((v624*v1334)-v1);
        let v1338=(if v1333{(v1334*v1336)}else{v2});
        let v1340=(v1+(v1204*v1338));
        let v1343=(-(if v1333{(v1210*v1340)}else{v2}));
        let v1345=(if (v1343>v634){v1}else{v2});
        let v1346=(v1333&&(v1345!=0.0));
        let v1347=(v1343).exp();
        let v1350=(v1333&&(!(v1345!=0.0)));
        let v1351=(v634-v1343);
        let v1352=(v45*v1351);
        let v1354=(v1+(v40*v1351));
        let v1356=(v1+(v1352*v1354));
        let v1358=(v1+(v1351*v1356));
        let v1360=(if v1350{(v642/v1358)}else{(if v1346{v1347}else{v1325})});
        let v1366=(((v658+v1204)-(if v1333{(v1-v1360)}else{v2}))).sqrt();
        let v1369=(if v1333{((v655+v1204)-(v456*v1366))}else{v2});
        let v1370=(if v1333{v665}else{v2});
        let v1371=(v1370-v1369);
        let v1372=(v1371>v381);
        let v1375=((v670+(v1371*v1371))).sqrt();
        let v1379=(v1369-v1370);
        let v1380=(v1379>v381);
        let v1383=((v670+(v1379*v1379))).sqrt();
        let v1384=(v1379+v1383);
        let v1394=((v670+(v1370*v1370))).sqrt();
        let v1398=(if v1333{((if v1372{(v1370-(v45*(v1371+v1375)))}else{(if v1380{(v1370-(v678/v1384))}else{(v1370-(v45*(v685+v1371)))})})-(v45*(v1370-v1394)))}else{v1234});
        let v1400=(if v1333{(v1204-v1398)}else{v1360});
        let v1402=((-v1398)).exp();
        let v1403=(if v1333{v1402}else{v1299});
        let v1407=(v1+v1398);
        let v1409=(((v1398+v1403)-v1)-(v479*v1407));
        let v1411=((v1400*v1400)-(v457*v1409));
        let v1412=(v703>v1411);
        let v1414=(if v1333{(if v1412{v703}else{v1411})}else{v1241});
        let v1417=(if v1333{(v1-(v655*v1403))}else{v2});
        let v1420=((v1-v1403)-v479);
        let v1423=(if v1333{((v10*v1400)+(v457*v1420))}else{v1244});
        let v1425=(v1414/v457);
        let v1428=(if v1333{((v463-v1398)+(v1425).ln())}else{v1249});
        let v1430=(if v1333{(v1414+v1423)}else{v2});
        let v1433=(if ((v1428).abs()<v732){v1}else{v2});
        let v1434=(v1333&&(v1433!=0.0));
        let v1437=(v1333&&(!(v1433!=0.0)));
        let v1439=(v45*v1423);
        let v1441=(v1414*v1417);
        let v1442=((v1423*v1439)-v1441);
        let v1445=(if v1437{((v1430*v1430)+(v1428*v1442))}else{v2});
        let v1446=(v1414*v1430);
        let v1447=(v1428*v1446);
        let v1448=(v1428*v1430);
        let v1449=(v1428*v1448);
        let v1450=(v1449/v1445);
        let v1451=(v1423*v1450);
        let v1454=((v40*(v1423*v1423))-v1441);
        let v1456=(v1445+(v1451*v1454));
        let v1459=(if v1437{(v1398+(v1447/v1456))}else{(if v1434{v1398}else{v2})});
        let v1461=(if (v1459<v559){v1}else{v2});
        let v1462=(v1333&&(v1461!=0.0));
        let v1463=(v1459).exp();
        let v1464=(if v1462{v1463}else{v1289});
        let v1470=(if (v1459>v770){v1}else{v2});
        let v1472=(v1333&&(!(v1461!=0.0)));
        let v1473=((v1470!=0.0)&&v1472);
        let v1475=((v1459-v463)).exp();
        let v1476=(if v1473{v1475}else{(if v1462{(v479*v1464)}else{v1464})});
        let v1480=(v1472&&(!(v1470!=0.0)));
        let v1482=((v463-v1459)-v559);
        let v1483=(v45*v1482);
        let v1485=(v1+(v40*v1482));
        let v1487=(v1+(v1483*v1485));
        let v1489=(v1+(v1482*v1487));
        let v1491=(if v1480{(v642/v1489)}else{v1476});
        let v1492=(v1459-v559);
        let v1493=(v45*v1492);
        let v1495=(v1+(v40*v1492));
        let v1497=(v1+(v1493*v1495));
        let v1499=(v1+(v1492*v1497));
        let v1501=(if v1480{(v642/v1499)}else{(if v1473{(v479/v1476)}else{(if v1462{(v1/v1464)}else{v1291})})});
        let v1503=(v10+(v1459*v1459));
        let v1507=(if v1333{(v1204-v1459)}else{(if v1333{(v1/v1503)}else{v1400})});
        let v1511=((v1491+(v1-v1501))-v479);
        let v1514=(if v1333{((v10*v1507)+(v457*v1511))}else{v1306});
        let v1519=(v1+v1459);
        let v1521=((v1491+((v1459+v1501)-v1))-(v479*v1519));
        let v1525=(v1491+v1501);
        let v1528=(if v1333{(v10-(v457*v1525))}else{v1507});
        let v1530=(v10*(if v1333{((v1507*v1507)-(v457*v1521))}else{v1316}));
        let v1534=((if v1333{((v1514*v1514)-(v1528*v1530))}else{v1528})).sqrt();
        let v1535=(v1514+v1534);
        let v1538=(if v1333{(v1459+(v1530/v1535))}else{(if v1221{((-v1272)-(v1322/v1328))}else{(if v1208{(v1210*v1214)}else{v840})})});
        let v1545=(!((if ((v1204<=v2)||self.scalar_static_bool[18]){v1}else{v2})!=0.0));
        let v1547=(if (v1538<v559){v1}else{v2});
        let v1548=(v1545&&(v1547!=0.0));
        let v1549=(v1538).exp();
        let v1550=(if v1548{v1549}else{v2});
        let v1552=(if v1548{(v1/v1550)}else{v2});
        let v1561=(if (v1538>v770){v1}else{v2});
        let v1563=(v1545&&(!(v1547!=0.0)));
        let v1564=((v1561!=0.0)&&v1563);
        let v1566=((v1538-v463)).exp();
        let v1567=(if v1564{v1566}else{(if v1548{(v479*v1550)}else{v1550})});
        let v1575=(v1563&&(!(v1561!=0.0)));
        let v1587=(v1538-v559);
        let v1588=(v45*v1587);
        let v1590=(v1+(v40*v1587));
        let v1592=(v1+(v1588*v1590));
        let v1594=(v1+(v1587*v1592));
        let v1596=(if v1575{(v642/v1594)}else{(if v1564{(v479/v1567)}else{v1552})});
        let v1600=(if (v1538<v154){v1}else{v2});
        let v1601=(v1545&&(v1600!=0.0));
        let v1602=(v45*v1538);
        let v1603=(v1538*v1602);
        let v1604=(v40*v1538);
        let v1606=(v1-(v657*v1538));
        let v1608=(v1-(v1604*v1606));
        let v1620=(v1608).sqrt();
        let v1621=(if v1601{v1620}else{self.scalar_static_f64[218]});
        let v1622=(v150*v1538);
        let v1626=(v1545&&(!(v1600!=0.0)));
        let v1629=(if v1626{(v1596+(v1538-v1))}else{(if v1601{(v1603*v1608)}else{v2})});
        let v1630=(v1629).sqrt();
        let v1631=(if v1626{v1630}else{(if v1601{(v1621*v1622)}else{v2})});
        let v1642=ctx.node_voltage(nodes[6]);
        let v1643=(v483+v1642);
        let v1644=(self.scalar_static_f64[175]*v1643);
        let v1647=(if ((v1644).abs()<=v462){v1}else{v2});
        let v1648=(v1644/v460);
        let v1651=(if (v1644>v462){v1}else{v2});
        let v1652=(!(v1647!=0.0));
        let v1653=((v1651!=0.0)&&v1652);
        let v1655=((v624/v481)-v1);
        let v1656=(v1655/v481);
        let v1657=(if v1653{v1656}else{v2});
        let v1659=(v1+(v1644*v1657));
        let v1661=(if v1653{(v1648*v1659)}else{v2});
        let v1663=(if (v1661<v166){v1}else{v2});
        let v1664=(v1653&&(v1663!=0.0));
        let v1666=((-v1661)).exp();
        let v1669=(v1653&&(!(v1663!=0.0)));
        let v1670=(v1661-v166);
        let v1671=(v45*v1670);
        let v1673=(v1+(v40*v1670));
        let v1675=(v1+(v1671*v1673));
        let v1677=(v1+(v1670*v1675));
        let v1679=(if v1669{(v173/v1677)}else{(if v1664{v1666}else{v2})});
        let v1681=(if v1653{(v1-v1679)}else{v2});
        let v1685=(((v658+v1644)-v1681)).sqrt();
        let v1688=(if v1653{((v655+v1644)-(v456*v1685))}else{v2});
        let v1690=(if (v1688<v166){v1}else{v2});
        let v1691=(v1653&&(v1690!=0.0));
        let v1693=((-v1688)).exp();
        let v1696=(v1653&&(!(v1690!=0.0)));
        let v1697=(v1688-v166);
        let v1698=(v45*v1697);
        let v1700=(v1+(v40*v1697));
        let v1702=(v1+(v1698*v1700));
        let v1704=(v1+(v1697*v1702));
        let v1706=(if v1696{(v173/v1704)}else{(if v1691{v1693}else{v2})});
        let v1709=(if v1653{(v1-(v655*v1706))}else{v2});
        let v1710=(v1644-v1688);
        let v1712=(v1-v1706);
        let v1715=(if v1653{((v10*v1710)+(v457*v1712))}else{v2});
        let v1718=(v1706+(v1688-v1));
        let v1721=(if v1653{((v1710*v1710)-(v457*v1718))}else{v2});
        let v1723=(v96*v1709);
        let v1726=(if v1653{((v1715*v1715)-(v1721*v1723))}else{v1679});
        let v1727=(v10*v1721);
        let v1728=(v1726).sqrt();
        let v1729=(v1715+v1728);
        let v1735=(v1652&&(!(v1651!=0.0)));
        let v1737=(if v1735{(-v1644)}else{v2});
        let v1738=(v162*v1737);
        let v1740=(if v1735{(v1738/v460)}else{v2});
        let v1742=(v1740-v143);
        let v1745=((v515+(v1742*v1742))).sqrt();
        let v1748=(if v1735{(v45*((v511+v1740)-v1745))}else{v2});
        let v1749=(v1737-v1748);
        let v1751=(v1+v1748);
        let v1754=(if v1735{((v1749*v1749)+(v457*v1751))}else{v2});
        let v1757=(if v1735{((v10*v1749)-v457)}else{v2});
        let v1758=(v1754/v457);
        let v1761=(if v1735{((v1758).ln()-v1748)}else{v2});
        let v1763=(if v1735{(v1754+v1757)}else{v2});
        let v1765=(v45*v1757);
        let v1767=((v1757*v1765)-v1754);
        let v1770=(if v1735{((v1763*v1763)+(v1761*v1767))}else{v2});
        let v1771=(v1754*v1763);
        let v1772=(v1761*v1771);
        let v1773=(v1761*v1763);
        let v1774=(v1761*v1773);
        let v1775=(v1774/v1770);
        let v1776=(v1757*v1775);
        let v1779=((v40*(v1757*v1757))-v1754);
        let v1781=(v1770+(v1776*v1779));
        let v1784=(if v1735{(v1748+(v1772/v1781))}else{v2});
        let v1787=(if ((v1784).abs()<v559){v1}else{v2});
        let v1788=(v1735&&(v1787!=0.0));
        let v1789=(v1784).exp();
        let v1792=(if (v1784<v634){v1}else{v2});
        let v1794=(v1735&&(!(v1787!=0.0)));
        let v1795=((v1792!=0.0)&&v1794);
        let v1796=(v634-v1784);
        let v1797=(v45*v1796);
        let v1799=(v1+(v40*v1796));
        let v1801=(v1+(v1797*v1799));
        let v1803=(v1+(v1796*v1801));
        let v1807=(v1794&&(!(v1792!=0.0)));
        let v1808=(v1784-v559);
        let v1809=(v45*v1808);
        let v1811=(v1+(v40*v1808));
        let v1813=(v1+(v1809*v1811));
        let v1817=(if v1807{(v567*(v1+(v1808*v1813)))}else{(if v1795{(v642/v1803)}else{(if v1788{v1789}else{v1706})})});
        let v1821=(v1737-v1784);
        let v1823=(v1817-v1);
        let v1826=(if v1735{((v10*v1821)+(v457*v1823))}else{v1715});
        let v1829=((v1+v1784)-v1817);
        let v1832=(if v1735{((v1821*v1821)+(v457*v1829))}else{v1721});
        let v1834=(v96*(if v1735{(v1-(v655*v1817))}else{v1709}));
        let v1838=(v10*v1832);
        let v1839=((if v1735{((v1826*v1826)-(v1832*v1834))}else{v1726})).sqrt();
        let v1840=(v1826+v1839);
        let v1845=(if v1735{(-(v1784+(if v1735{(v1838/v1840)}else{v1681})))}else{(if v1653{(v1688+(if v1653{(v1727/v1729)}else{v2}))}else{(if (v1647!=0.0){v1648}else{v2})})});
        let v1846=(self.scalar_static_f64[172]*v1845);
        let v1850=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(v483-v1846)))}else{v2});
        let v1853=(if ((v1850).abs()<=self.scalar_static_f64[225]){v1}else{v2});
        let v1854=((self.scalar_static_f64[149]!=0.0)&&(v1853!=0.0));
        let v1855=(if v1854{self.scalar_static_f64[322]}else{v2});
        let v1856=(self.scalar_static_f64[224]*v1850);
        let v1860=(v1+(v1855*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v1850))));
        let v1864=(if (v1850<self.scalar_static_f64[324]){v1}else{v2});
        let v1866=((self.scalar_static_f64[149]!=0.0)&&(!(v1853!=0.0)));
        let v1867=((v1864!=0.0)&&v1866);
        let v1869=(if v1867{(-v1850)}else{v2});
        let v1872=(if v1867{(self.scalar_static_f64[224]*(v162*v1869))}else{v2});
        let v1874=(v1872-v143);
        let v1877=((v515+(v1874*v1874))).sqrt();
        let v1880=(if v1867{(v45*((v511+v1872)-v1877))}else{v2});
        let v1882=(if v1867{(v1869-v1880)}else{v2});
        let v1887=(if v1867{((v1882*v1882)+(self.scalar_static_f64[220]*(v1+v1880)))}else{v2});
        let v1890=(if v1867{((v10*v1882)-self.scalar_static_f64[220])}else{v2});
        let v1892=(self.scalar_static_f64[221]*v1887);
        let v1895=(if v1867{((-v1880)+(v1892).ln())}else{v2});
        let v1897=(if v1867{(v1887+v1890)}else{v2});
        let v1899=(v45*v1890);
        let v1901=((v1890*v1899)-v1887);
        let v1904=(if v1867{((v1897*v1897)+(v1895*v1901))}else{v2});
        let v1905=(v1887*v1897);
        let v1906=(v1895*v1905);
        let v1907=(v1895*v1897);
        let v1908=(v1895*v1907);
        let v1909=(v1908/v1904);
        let v1910=(v1890*v1909);
        let v1913=((v40*(v1890*v1890))-v1887);
        let v1915=(v1904+(v1910*v1913));
        let v1918=(if v1867{(v1880+(v1906/v1915))}else{v2});
        let v1920=(if (v1918<v559){v1}else{v2});
        let v1921=(v1867&&(v1920!=0.0));
        let v1922=(v1918).exp();
        let v1925=(v1867&&(!(v1920!=0.0)));
        let v1926=(v1918-v559);
        let v1927=(v45*v1926);
        let v1929=(v1+(v40*v1926));
        let v1931=(v1+(v1927*v1929));
        let v1935=(if v1925{(v567*(v1+(v1926*v1931)))}else{(if v1921{v1922}else{v2})});
        let v1937=(if v1867{(v1/v1935)}else{v2});
        let v1939=(v10+(v1918*v1918));
        let v1943=(if v1867{(v1869-v1918)}else{(if v1867{(v1/v1939)}else{v1882})});
        let v1945=(if v1867{(self.scalar_static_f64[247]*v1937)}else{v1855});
        let v1952=(if v1867{((v10*v1943)+(self.scalar_static_f64[220]*(self.scalar_static_f64[247]+((v1935-v1)-v1945))))}else{v2});
        let v1962=(if v1867{((v1943*v1943)-(self.scalar_static_f64[220]*((v1945+((v1935-v1918)-v1))+(self.scalar_static_f64[247]*(v1918-v1)))))}else{v2});
        let v1966=(if v1867{(v10-(self.scalar_static_f64[220]*(v1935+v1945)))}else{v1943});
        let v1968=(v10*v1962);
        let v1971=(if v1867{((v1952*v1952)-(v1966*v1968))}else{v1966});
        let v1973=(v1971).sqrt();
        let v1974=(v1952+v1973);
        let v1979=(v1866&&(!(v1864!=0.0)));
        let v1980=(if v1979{self.scalar_static_f64[327]}else{v2});
        let v1984=(if v1979{(v1980*((self.scalar_static_f64[328]*v1980)-v1))}else{v2});
        let v1986=(v1+(v1850*v1984));
        let v1989=(-(if v1979{(v1856*v1986)}else{v2}));
        let v1991=(if (v1989>v634){v1}else{v2});
        let v1992=(v1979&&(v1991!=0.0));
        let v1993=(v1989).exp();
        let v1996=(v1979&&(!(v1991!=0.0)));
        let v1997=(v634-v1989);
        let v1998=(v45*v1997);
        let v2000=(v1+(v40*v1997));
        let v2002=(v1+(v1998*v2000));
        let v2004=(v1+(v1997*v2002));
        let v2006=(if v1996{(v642/v2004)}else{(if v1992{v1993}else{v1971})});
        let v2012=(((self.scalar_static_f64[330]+v1850)-(if v1979{(v1-v2006)}else{v2}))).sqrt();
        let v2015=(if v1979{((self.scalar_static_f64[329]+v1850)-(self.scalar_static_f64[219]*v2012))}else{v2});
        let v2016=(if v1979{self.scalar_static_f64[331]}else{v2});
        let v2017=(v2016-v2015);
        let v2018=(v2017>v381);
        let v2021=((v670+(v2017*v2017))).sqrt();
        let v2025=(v2015-v2016);
        let v2026=(v2025>v381);
        let v2029=((v670+(v2025*v2025))).sqrt();
        let v2030=(v2025+v2029);
        let v2044=(if v1979{((if v2018{(v2016-(v45*(v2017+v2021)))}else{(if v2026{(v2016-(v678/v2030))}else{(v2016-(v45*(v685+v2017)))})})-(v45*(v2016-((v670+(v2016*v2016))).sqrt())))}else{v1880});
        let v2046=(if v1979{(v1850-v2044)}else{v2006});
        let v2048=((-v2044)).exp();
        let v2049=(if v1979{v2048}else{v1945});
        let v2057=((v2046*v2046)-(self.scalar_static_f64[220]*(((v2044+v2049)-v1)-(self.scalar_static_f64[247]*(v1+v2044)))));
        let v2058=(v703>v2057);
        let v2060=(if v1979{(if v2058{v703}else{v2057})}else{v1887});
        let v2063=(if v1979{(v1-(self.scalar_static_f64[329]*v2049))}else{v2});
        let v2069=(if v1979{((v10*v2046)+(self.scalar_static_f64[220]*((v1-v2049)-self.scalar_static_f64[247])))}else{v1890});
        let v2071=(v2060/self.scalar_static_f64[220]);
        let v2074=(if v1979{((self.scalar_static_f64[226]-v2044)+(v2071).ln())}else{v1895});
        let v2076=(if v1979{(v2060+v2069)}else{v2});
        let v2079=(if ((v2074).abs()<v732){v1}else{v2});
        let v2080=(v1979&&(v2079!=0.0));
        let v2083=(v1979&&(!(v2079!=0.0)));
        let v2085=(v45*v2069);
        let v2087=(v2060*v2063);
        let v2088=((v2069*v2085)-v2087);
        let v2091=(if v2083{((v2076*v2076)+(v2074*v2088))}else{v2});
        let v2092=(v2060*v2076);
        let v2093=(v2074*v2092);
        let v2094=(v2074*v2076);
        let v2095=(v2074*v2094);
        let v2096=(v2095/v2091);
        let v2097=(v2069*v2096);
        let v2100=((v40*(v2069*v2069))-v2087);
        let v2102=(v2091+(v2097*v2100));
        let v2105=(if v2083{(v2044+(v2093/v2102))}else{(if v2080{v2044}else{v2})});
        let v2107=(if (v2105<v559){v1}else{v2});
        let v2108=(v1979&&(v2107!=0.0));
        let v2109=(v2105).exp();
        let v2110=(if v2108{v2109}else{v1935});
        let v2116=(if (v2105>self.scalar_static_f64[332]){v1}else{v2});
        let v2118=(v1979&&(!(v2107!=0.0)));
        let v2119=((v2116!=0.0)&&v2118);
        let v2121=((v2105-self.scalar_static_f64[226])).exp();
        let v2122=(if v2119{v2121}else{(if v2108{(self.scalar_static_f64[247]*v2110)}else{v2110})});
        let v2126=(v2118&&(!(v2116!=0.0)));
        let v2128=((self.scalar_static_f64[226]-v2105)-v559);
        let v2129=(v45*v2128);
        let v2131=(v1+(v40*v2128));
        let v2133=(v1+(v2129*v2131));
        let v2135=(v1+(v2128*v2133));
        let v2137=(if v2126{(v642/v2135)}else{v2122});
        let v2138=(v2105-v559);
        let v2139=(v45*v2138);
        let v2141=(v1+(v40*v2138));
        let v2143=(v1+(v2139*v2141));
        let v2145=(v1+(v2138*v2143));
        let v2147=(if v2126{(v642/v2145)}else{(if v2119{(self.scalar_static_f64[247]/v2122)}else{(if v2108{(v1/v2110)}else{v1937})})});
        let v2149=(v10+(v2105*v2105));
        let v2153=(if v1979{(v1850-v2105)}else{(if v1979{(v1/v2149)}else{v2046})});
        let v2160=(if v1979{((v10*v2153)+(self.scalar_static_f64[220]*((v2137+(v1-v2147))-self.scalar_static_f64[247])))}else{v1952});
        let v2174=(if v1979{(v10-(self.scalar_static_f64[220]*(v2137+v2147)))}else{v2153});
        let v2176=(v10*(if v1979{((v2153*v2153)-(self.scalar_static_f64[220]*((v2137+((v2105+v2147)-v1))-(self.scalar_static_f64[247]*(v1+v2105)))))}else{v1962}));
        let v2180=((if v1979{((v2160*v2160)-(v2174*v2176))}else{v2174})).sqrt();
        let v2181=(v2160+v2180);
        let v2187=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v1979{(v2105+(v2176/v2181))}else{(if v1867{((-v1918)-(v1968/v1974))}else{(if v1854{(v1856*v1860)}else{v2})})})))}else{v2});
        let v2190=(if (self.scalar_static_f64[149]!=0.0){((v1643-v2187)/self.scalar_static_f64[172])}else{v1644});
        let v2193=(if ((v2190).abs()<=v462){v1}else{v2});
        let v2194=((self.scalar_static_f64[149]!=0.0)&&(v2193!=0.0));
        let v2195=(v2190/v460);
        let v2198=(if (v2190>v462){v1}else{v2});
        let v2200=((self.scalar_static_f64[149]!=0.0)&&(!(v2193!=0.0)));
        let v2201=((v2198!=0.0)&&v2200);
        let v2202=(if v2201{v1656}else{v2});
        let v2204=(v1+(v2190*v2202));
        let v2206=(if v2201{(v2195*v2204)}else{v2});
        let v2208=(if (v2206<v166){v1}else{v2});
        let v2209=(v2201&&(v2208!=0.0));
        let v2211=((-v2206)).exp();
        let v2214=(v2201&&(!(v2208!=0.0)));
        let v2215=(v2206-v166);
        let v2216=(v45*v2215);
        let v2218=(v1+(v40*v2215));
        let v2220=(v1+(v2216*v2218));
        let v2222=(v1+(v2215*v2220));
        let v2224=(if v2214{(v173/v2222)}else{(if v2209{v2211}else{v2})});
        let v2226=(if v2201{(v1-v2224)}else{v2});
        let v2230=(((v658+v2190)-v2226)).sqrt();
        let v2233=(if v2201{((v655+v2190)-(v456*v2230))}else{v2});
        let v2235=(if (v2233<v166){v1}else{v2});
        let v2236=(v2201&&(v2235!=0.0));
        let v2238=((-v2233)).exp();
        let v2241=(v2201&&(!(v2235!=0.0)));
        let v2242=(v2233-v166);
        let v2243=(v45*v2242);
        let v2245=(v1+(v40*v2242));
        let v2247=(v1+(v2243*v2245));
        let v2249=(v1+(v2242*v2247));
        let v2251=(if v2241{(v173/v2249)}else{(if v2236{v2238}else{v2})});
        let v2254=(if v2201{(v1-(v655*v2251))}else{v2});
        let v2255=(v2190-v2233);
        let v2257=(v1-v2251);
        let v2260=(if v2201{((v10*v2255)+(v457*v2257))}else{v2});
        let v2263=(v2251+(v2233-v1));
        let v2266=(if v2201{((v2255*v2255)-(v457*v2263))}else{v2});
        let v2268=(v96*v2254);
        let v2271=(if v2201{((v2260*v2260)-(v2266*v2268))}else{v2224});
        let v2272=(v10*v2266);
        let v2273=(v2271).sqrt();
        let v2274=(v2260+v2273);
        let v2280=(v2200&&(!(v2198!=0.0)));
        let v2282=(if v2280{(-v2190)}else{v2});
        let v2283=(v162*v2282);
        let v2285=(if v2280{(v2283/v460)}else{v2});
        let v2287=(v2285-v143);
        let v2290=((v515+(v2287*v2287))).sqrt();
        let v2293=(if v2280{(v45*((v511+v2285)-v2290))}else{v2});
        let v2294=(v2282-v2293);
        let v2296=(v1+v2293);
        let v2299=(if v2280{((v2294*v2294)+(v457*v2296))}else{v2});
        let v2302=(if v2280{((v10*v2294)-v457)}else{v2});
        let v2303=(v2299/v457);
        let v2306=(if v2280{((v2303).ln()-v2293)}else{v2});
        let v2308=(if v2280{(v2299+v2302)}else{v2});
        let v2310=(v45*v2302);
        let v2312=((v2302*v2310)-v2299);
        let v2315=(if v2280{((v2308*v2308)+(v2306*v2312))}else{v2});
        let v2316=(v2299*v2308);
        let v2317=(v2306*v2316);
        let v2318=(v2306*v2308);
        let v2319=(v2306*v2318);
        let v2320=(v2319/v2315);
        let v2321=(v2302*v2320);
        let v2324=((v40*(v2302*v2302))-v2299);
        let v2326=(v2315+(v2321*v2324));
        let v2329=(if v2280{(v2293+(v2317/v2326))}else{v2});
        let v2332=(if ((v2329).abs()<v559){v1}else{v2});
        let v2333=(v2280&&(v2332!=0.0));
        let v2334=(v2329).exp();
        let v2337=(if (v2329<v634){v1}else{v2});
        let v2339=(v2280&&(!(v2332!=0.0)));
        let v2340=((v2337!=0.0)&&v2339);
        let v2341=(v634-v2329);
        let v2342=(v45*v2341);
        let v2344=(v1+(v40*v2341));
        let v2346=(v1+(v2342*v2344));
        let v2348=(v1+(v2341*v2346));
        let v2352=(v2339&&(!(v2337!=0.0)));
        let v2353=(v2329-v559);
        let v2354=(v45*v2353);
        let v2356=(v1+(v40*v2353));
        let v2358=(v1+(v2354*v2356));
        let v2362=(if v2352{(v567*(v1+(v2353*v2358)))}else{(if v2340{(v642/v2348)}else{(if v2333{v2334}else{v2251})})});
        let v2366=(v2282-v2329);
        let v2368=(v2362-v1);
        let v2371=(if v2280{((v10*v2366)+(v457*v2368))}else{v2260});
        let v2374=((v1+v2329)-v2362);
        let v2377=(if v2280{((v2366*v2366)+(v457*v2374))}else{v2266});
        let v2379=(v96*(if v2280{(v1-(v655*v2362))}else{v2254}));
        let v2383=(v10*v2377);
        let v2384=((if v2280{((v2371*v2371)-(v2377*v2379))}else{v2271})).sqrt();
        let v2385=(v2371+v2384);
        let v2390=(if v2280{(-(v2329+(if v2280{(v2383/v2385)}else{v2226})))}else{(if v2201{(v2233+(if v2201{(v2272/v2274)}else{v2}))}else{(if v2194{v2195}else{v1845})})});
        let v2395=(if (v2390<v559){v1}else{v2});
        let v2396=(v2390).exp();
        let v2397=(if (v2395!=0.0){v2396}else{v2});
        let v2401=(if (v2390>v770){v1}else{v2});
        let v2402=(!(v2395!=0.0));
        let v2403=((v2401!=0.0)&&v2402);
        let v2405=((v463-v2390)).exp();
        let v2406=(if v2403{v2405}else{v2397});
        let v2410=(v2402&&(!(v2401!=0.0)));
        let v2411=(v2390-v559);
        let v2412=(v45*v2411);
        let v2414=(v1+(v40*v2411));
        let v2416=(v1+(v2412*v2414));
        let v2418=(v1+(v2411*v2416));
        let v2420=(if v2410{(v642/v2418)}else{(if v2403{(v479*v2406)}else{(if (v2395!=0.0){(v1/v2397)}else{v1596})})});
        let v2422=(if (v2390<v501){v1}else{v2});
        let v2425=(if (v2422!=0.0){((v2390+v2420)-v1)}else{v1629});
        let v2426=(v2425).sqrt();
        let v2431=(if ((v2390).abs()<=v462){v1}else{v2});
        let v2432=(!(v2422!=0.0));
        let v2433=((v2431!=0.0)&&v2432);
        let v2434=(v40*v2390);
        let v2436=(v1-(v657*v2390));
        let v2439=(if v2433{(v1-(v2434*v2436))}else{v1621});
        let v2440=(v45*v2390);
        let v2441=(v2390*v2440);
        let v2444=(v150*v2390);
        let v2445=(v2439).sqrt();
        let v2449=(v2432&&(!(v2431!=0.0)));
        let v2453=((if v2449{(v2420+(v2390-v1))}else{(if v2433{(v2439*v2441)}else{v2425})})).sqrt();
        let v2455=(self.scalar_static_f64[172]*(if v2449{v2453}else{(if v2433{(v2444*v2445)}else{(if (v2422!=0.0){(-v2426)}else{v1631})})}));
        let v2456=(v456*v2455);
        let v2457=1.62;
        let v2458=(v1+(v430/v431));
        let v2459=(v2457*v2458);
        let v2470=(self.scalar_static_f64[172]*(self.scalar_static_f64[172]*((self.scalar_static_f64[170]*(self.scalar_static_f64[155]*((v2458*v2459)*self.scalar_static_f64[155])))*self.scalar_static_f64[333])));
        let v2471=(-v2456);
        let v2472=(v2456-v2471);
        let v2473=(v2472>v381);
        let v2476=((v2470+(v2472*v2472))).sqrt();
        let v2480=(v2471-v2456);
        let v2481=(v2480>v381);
        let v2482=(v45*v2470);
        let v2485=((v2470+(v2480*v2480))).sqrt();
        let v2486=(v2480+v2485);
        let v2490=((1e-32+v2470)).sqrt();
        let v2496=(-v1642);
        let v2497=(v2496-v1642);
        let v2498=(v2497>v381);
        let v2501=((v2470+(v2497*v2497))).sqrt();
        let v2505=(v1642-v2496);
        let v2506=(v2505>v381);
        let v2509=((v2470+(v2505*v2505))).sqrt();
        let v2510=(v2505+v2509);
        let v2519=((if v2473{(v2471+(v45*(v2472+v2476)))}else{(if v2481{(v2471+(v2482/v2486))}else{(v2471+(v45*(v2472+v2490)))})})+(self.scalar_static_f64[29]*(if v2498{(v1642+(v45*(v2497+v2501)))}else{(if v2506{(v1642+(v2482/v2510))}else{(v1642+(v45*(v2490+v2497)))})})));
        let v2523=(self.scalar_static_f64[174]+(v2519*v2519));
        let v2525=-0.1666666666666667;
        let v2528=(v1+(self.scalar_static_f64[24]*f64::powf(v2523,v2525)));
        let v2530=(if (self.scalar_static_f64[156]!=0.0){(self.scalar_static_f64[4]/v2528)}else{self.scalar_static_f64[4]});
        let v2577=ctx.node_voltage(nodes[1]);
        let v3343=(self.scalar_static_f64[52]*(self.scalar_static_f64[50]*((v483-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*v2390)}else{v1846}))-(if self.scalar_static_bool[17]{v2}else{v2187}))));
        let v3345=(self.scalar_static_f64[20]*(v2530*v3343));
        let v3347=(v1642*self.scalar_static_f64[160]);
        let v3348=ctx.node_voltage(nodes[3]);
        let v3350=(self.scalar_static_f64[58]*(v3348-v2577));
        let v3368=(self.scalar_static_f64[20]*v380);
        let v3370=(v380*self.scalar_static_f64[150]);
        let v3372=(v10*v386);
        let v3379=(v389*self.scalar_static_f64[150]);
        let v3381=(self.scalar_static_f64[20]*v389);
        let v3383=(v10*v394);
        let v3390=(v395*v395);
        let v3401=(self.scalar_static_f64[141]*(if v382{(v45*(self.scalar_static_f64[20]+((v3368+v3368)/v3372)))}else{(if v390{((-(self.scalar_static_f64[144]*(self.scalar_static_f64[150]+((v3379+v3379)/v3383))))/v3390)}else{self.scalar_static_f64[161]})}));
        let v3402=(self.scalar_static_f64[141]*(if v382{(v45*(self.scalar_static_f64[150]+((v3370+v3370)/v3372)))}else{(if v390{((-(self.scalar_static_f64[144]*(self.scalar_static_f64[20]+((v3381+v3381)/v3383))))/v3390)}else{self.scalar_static_f64[162]})}));
        let v3403=(-v3401);
        let v3404=(-v3402);
        let v3405=(v407*v3403);
        let v3407=(v407*v3404);
        let v3409=(v10*v412);
        let v3418=(v416*v3401);
        let v3420=(v416*v3402);
        let v3422=(v10*v421);
        let v3429=(v422*v422);
        let v3444=(self.scalar_static_f64[5]*(if v408{(-(v45*(v3403+((v3405+v3405)/v3409))))}else{(if v417{(-((-(v418*(v3401+((v3418+v3418)/v3422))))/v3429))}else{(-(v45*v3403))})}));
        let v3445=(self.scalar_static_f64[5]*(if v408{(-(v45*(v3404+((v3407+v3407)/v3409))))}else{(if v417{(-((-(v418*(v3402+((v3420+v3420)/v3422))))/v3429))}else{(-(v45*v3404))})}));
        let v3446=(v3444/v431);
        let v3447=(v3445/v431);
        let v3452=(self.scalar_static_f64[207]*((self.scalar_static_f64[206]*v3444)/v433));
        let v3453=(self.scalar_static_f64[207]*((self.scalar_static_f64[206]*v3445)/v433));
        let v3456=(v10*v438);
        let v3459=(((v12*v3444)/v3456)/self.scalar_static_f64[4]);
        let v3460=(((v12*v3445)/v3456)/self.scalar_static_f64[4]);
        let v3461=(v439*v3459);
        let v3463=(v439*v3460);
        let v3471=(v10*v442);
        let v3474=(if (self.scalar_static_f64[15]!=0.0){(((v440*v3452)+(v436*(v3461+v3461)))/v3471)}else{v2});
        let v3475=(if (self.scalar_static_f64[15]!=0.0){(((v440*v3453)+(v436*(v3463+v3463)))/v3471)}else{v2});
        let v3478=(v27*f64::powf(v443,-0.33333333333333337));
        let v3483=(if (self.scalar_static_f64[15]!=0.0){(self.scalar_static_f64[148]*(v3474*v3478))}else{v2});
        let v3484=(if (self.scalar_static_f64[15]!=0.0){(self.scalar_static_f64[148]*(v3475*v3478))}else{v2});
        let v3494=(v443*v443);
        let v3506=(if (self.scalar_static_f64[15]!=0.0){((v453*v3459)+(v439*(((v443*(v300*v3483))-(v451*v3474))/v3494)))}else{v3459});
        let v3507=(if (self.scalar_static_f64[15]!=0.0){((v453*v3460)+(v439*(((v443*(v300*v3484))-(v451*v3475))/v3494)))}else{v3460});
        let v3508=(self.scalar_static_f64[218]*v3506);
        let v3509=(self.scalar_static_f64[218]*v3507);
        let v3510=(v456*v3508);
        let v3511=(v3510+v3510);
        let v3512=(v456*v3509);
        let v3513=(v3512+v3512);
        let v3515=(v457*v457);
        let v3516=((-v3511)/v3515);
        let v3518=((-v3513)/v3515);
        let v3519=(v150*v3508);
        let v3520=(v150*v3509);
        let v3522=(v460*v460);
        let v3523=((-v3519)/v3522);
        let v3525=((-v3520)/v3522);
        let v3526=(self.scalar_static_f64[175]*(if (self.scalar_static_f64[15]!=0.0){(v3452+v3483)}else{v3452}));
        let v3527=(self.scalar_static_f64[175]*(if (self.scalar_static_f64[15]!=0.0){(v3453+v3484)}else{v3453}));
        let v3552=(v477*v477);
        let v3557=(if v469{((-(v173*((v475*v3526)+(v470*((v473*(v45*v3526))+(v471*(v40*v3526)))))))/v3552)}else{(if (v465!=0.0){(v467*(-v3526))}else{v2})});
        let v3558=(if v469{((-(v173*((v475*v3527)+(v470*((v473*(v45*v3527))+(v471*(v40*v3527)))))))/v3552)}else{(if (v465!=0.0){(v467*(-v3527))}else{v2})});
        let v3559=(v163*v3508);
        let v3560=(v163*v3509);
        let v3563=(v461*v3523);
        let v3565=(v461*v3525);
        let v3569=(v150*(v489*(v3563+v3563)));
        let v3570=(v150*(v489*(v3565+v3565)));
        let v3571=(if (v487!=0.0){v3569}else{v2});
        let v3572=(if (v487!=0.0){v3570}else{v2});
        let v3575=((v484*v3523)+(v461*self.scalar_static_f64[351]));
        let v3578=((v484*v3525)+(v461*self.scalar_static_f64[352]));
        let v3579=(-v3557);
        let v3580=(-v3558);
        let v3609=(if v505{self.scalar_static_f64[353]}else{v2});
        let v3610=(if v505{self.scalar_static_f64[354]}else{v2});
        let v3619=(if v505{((v508*v3523)+(v461*(v162*v3609)))}else{v2});
        let v3620=(if v505{((v508*v3525)+(v461*(v162*v3610)))}else{v2});
        let v3621=(v513*v3619);
        let v3623=(v513*v3620);
        let v3625=(v10*v517);
        let v3632=(if v505{(v45*(v3619-((v3621+v3621)/v3625)))}else{v2});
        let v3633=(if v505{(v45*(v3620-((v3623+v3623)/v3625)))}else{v2});
        let v3636=(if v505{(v3609-v3632)}else{v2});
        let v3637=(if v505{(v3610-v3633)}else{v2});
        let v3638=(v522*v3636);
        let v3640=(v522*v3637);
        let v3650=(if v505{((v3638+v3638)+((v524*v3511)+(v457*v3632)))}else{v2});
        let v3651=(if v505{((v3640+v3640)+((v524*v3513)+(v457*v3633)))}else{v2});
        let v3656=(if v505{((v10*v3636)-v3511)}else{v2});
        let v3657=(if v505{((v10*v3637)-v3513)}else{v2});
        let v3670=(if v505{((-v3632)+(((v527*v3516)+(v458*v3650))/v532))}else{v2});
        let v3671=(if v505{((-v3633)+(((v527*v3518)+(v458*v3651))/v532))}else{v2});
        let v3674=(if v505{(v3650+v3656)}else{v2});
        let v3675=(if v505{(v3651+v3657)}else{v2});
        let v3676=(v537*v3674);
        let v3678=(v537*v3675);
        let v3698=(if v505{((v3676+v3676)+((v541*v3670)+(v535*(((v539*v3656)+(v530*(v45*v3656)))-v3650))))}else{v2});
        let v3699=(if v505{((v3678+v3678)+((v541*v3671)+(v535*(((v539*v3657)+(v530*(v45*v3657)))-v3651))))}else{v2});
        let v3727=(v544*v544);
        let v3739=(v530*v3656);
        let v3741=(v530*v3657);
        let v3758=(v555*v555);
        let v3766=(if v505{(v3632+(((v555*((v545*v3670)+(v535*((v537*v3650)+(v527*v3674)))))-(v546*(v3698+((v553*((v549*v3656)+(v530*(((v544*((v547*v3670)+(v535*((v537*v3670)+(v535*v3674)))))-(v548*v3698))/v3727))))+(v550*((v40*(v3739+v3739))-v3650))))))/v3758))}else{v2});
        let v3767=(if v505{(v3633+(((v555*((v545*v3671)+(v535*((v537*v3651)+(v527*v3675)))))-(v546*(v3699+((v553*((v549*v3657)+(v530*(((v544*((v547*v3671)+(v535*((v537*v3671)+(v535*v3675)))))-(v548*v3699))/v3727))))+(v550*((v40*(v3741+v3741))-v3651))))))/v3758))}else{v2});
        let v3790=(if v566{(v567*((v573*v3766)+(v568*((v571*(v45*v3766))+(v569*(v40*v3766))))))}else{(if v562{(v563*v3766)}else{v2})});
        let v3791=(if v566{(v567*((v573*v3767)+(v568*((v571*(v45*v3767))+(v569*(v40*v3767))))))}else{(if v562{(v563*v3767)}else{v2})});
        let v3793=(v577*v577);
        let v3797=(if v505{((-v3790)/v3793)}else{v2});
        let v3798=(if v505{((-v3791)/v3793)}else{v2});
        let v3799=(v558*v3766);
        let v3801=(v558*v3767);
        let v3804=(v581*v581);
        let v3812=(if v505{(v3609-v3766)}else{(if v505{((-(v3799+v3799))/v3804)}else{v3636})});
        let v3813=(if v505{(v3610-v3767)}else{(if v505{((-(v3801+v3801))/v3804)}else{v3637})});
        let v3820=(if v505{((v579*v3557)+(v479*v3797))}else{v3571});
        let v3821=(if v505{((v579*v3558)+(v479*v3798))}else{v3572});
        let v3836=(if v505{((v10*v3812)+((v591*v3511)+(v457*(v3557+(v3790-v3820)))))}else{v2});
        let v3837=(if v505{((v10*v3813)+((v591*v3513)+(v457*(v3558+(v3791-v3821)))))}else{v2});
        let v3838=(v585*v3812);
        let v3840=(v585*v3813);
        let v3862=(if v505{((v3838+v3838)-((v601*v3511)+(v457*((v3820+(v3790-v3766))+((v599*v3557)+(v479*v3766))))))}else{v2});
        let v3863=(if v505{((v3840+v3840)-((v601*v3513)+(v457*((v3821+(v3791-v3767))+((v599*v3558)+(v479*v3767))))))}else{v2});
        let v3874=(if v505{(-((v605*v3511)+(v457*(v3790+v3820))))}else{v3812});
        let v3875=(if v505{(-((v605*v3513)+(v457*(v3791+v3821))))}else{v3813});
        let v3876=(v594*v3836);
        let v3878=(v594*v3837);
        let v3880=(v10*v3862);
        let v3881=(v10*v3863);
        let v3890=(if v505{((v3876+v3876)-((v610*v3874)+(v608*v3880)))}else{v3874});
        let v3891=(if v505{((v3878+v3878)-((v610*v3875)+(v608*v3881)))}else{v3875});
        let v3894=(v10*v615);
        let v3902=(v616*v616);
        let v3913=(v481*v481);
        let v3914=((-v3559)/v3913);
        let v3916=((-v3560)/v3913);
        let v3917=(if v621{v3914}else{v2});
        let v3918=(if v621{v3916}else{v2});
        let v3919=(v162*v3519);
        let v3920=(v162*v3520);
        let v3947=(if v621{((v630*v3575)+(v493*((v628*self.scalar_static_f64[351])+(v484*(if v621{((v626*v3917)+(v623*((v624*v3917)+(v623*v3919))))}else{v2})))))}else{v2});
        let v3948=(if v621{((v630*v3578)+(v493*((v628*self.scalar_static_f64[352])+(v484*(if v621{((v626*v3918)+(v623*((v624*v3918)+(v623*v3920))))}else{v2})))))}else{v2});
        let v3973=(v650*v650);
        let v3978=(if v641{((-(v642*((v648*v3947)+(v643*((v646*(v45*v3947))+(v644*(v40*v3947)))))))/v3973)}else{(if v637{(v638*(-v3947))}else{v3890})});
        let v3979=(if v641{((-(v642*((v648*v3948)+(v643*((v646*(v45*v3948))+(v644*(v40*v3948)))))))/v3973)}else{(if v637{(v638*(-v3948))}else{v3891})});
        let v3984=(v45*v3511);
        let v3985=(v45*v3513);
        let v3986=(self.scalar_static_f64[351]+v3984);
        let v3987=(self.scalar_static_f64[352]+v3985);
        let v3988=(v657*v3511);
        let v3989=(v657*v3513);
        let v3990=(self.scalar_static_f64[351]+v3988);
        let v3991=(self.scalar_static_f64[352]+v3989);
        let v3994=(v10*v661);
        let v4005=(if v621{(v3986-((v661*v3508)+(v456*((v3990-(if v621{(-v3978)}else{v2}))/v3994))))}else{v2});
        let v4006=(if v621{(v3987-((v661*v3509)+(v456*((v3991-(if v621{(-v3979)}else{v2}))/v3994))))}else{v2});
        let v4007=(if v621{v3526}else{v2});
        let v4008=(if v621{v3527}else{v2});
        let v4009=(v4007-v4005);
        let v4010=(v4008-v4006);
        let v4011=(v667*v4009);
        let v4013=(v667*v4010);
        let v4015=(v10*v672);
        let v4024=(v4005-v4007);
        let v4025=(v4006-v4008);
        let v4026=(v676*v4024);
        let v4028=(v676*v4025);
        let v4030=(v10*v681);
        let v4037=(v682*v682);
        let v4052=(v666*v4007);
        let v4054=(v666*v4008);
        let v4056=(v10*v693);
        let v4065=(if v621{((if v668{(v4007-(v45*(v4009+((v4011+v4011)/v4015))))}else{(if v677{(v4007-((-(v678*(v4024+((v4026+v4026)/v4030))))/v4037))}else{(v4007-(v45*v4009))})})-(v45*(v4007-((v4052+v4052)/v4056))))}else{v3632});
        let v4066=(if v621{((if v668{(v4008-(v45*(v4010+((v4013+v4013)/v4015))))}else{(if v677{(v4008-((-(v678*(v4025+((v4028+v4028)/v4030))))/v4037))}else{(v4008-(v45*v4010))})})-(v45*(v4008-((v4054+v4054)/v4056))))}else{v3633});
        let v4069=(if v621{(self.scalar_static_f64[351]-v4065)}else{v3978});
        let v4070=(if v621{(self.scalar_static_f64[352]-v4066)}else{v3979});
        let v4075=(if v621{(v701*(-v4065))}else{v3820});
        let v4076=(if v621{(v701*(-v4066))}else{v3821});
        let v4077=(v699*v4069);
        let v4079=(v699*v4070);
        let v4101=(if v621{(if v712{v2}else{((v4077+v4077)-((v709*v3511)+(v457*((v4065+v4075)-((v707*v3557)+(v479*v4065))))))})}else{v3650});
        let v4102=(if v621{(if v712{v2}else{((v4079+v4079)-((v709*v3513)+(v457*((v4066+v4076)-((v707*v3558)+(v479*v4066))))))})}else{v3651});
        let v4127=(if v621{((v10*v4069)+((v720*v3511)+(v457*((-v4075)-v3557))))}else{v3656});
        let v4128=(if v621{((v10*v4070)+((v720*v3513)+(v457*((-v4076)-v3558))))}else{v3657});
        let v4143=(if v621{((v3526-v4065)+((((v457*v4101)-(v714*v3511))/v3515)/v725))}else{v3670});
        let v4144=(if v621{((v3527-v4066)+((((v457*v4102)-(v714*v3513))/v3515)/v725))}else{v3671});
        let v4147=(if v621{(v4101+v4127)}else{v2});
        let v4148=(if v621{(v4102+v4128)}else{v2});
        let v4151=(v730*v4147);
        let v4153=(v730*v4148);
        let v4165=((v717*v4101)+(v714*(if v621{(-((v702*v3984)+(v655*v4075)))}else{v2})));
        let v4168=((v717*v4102)+(v714*(if v621{(-((v702*v3985)+(v655*v4076)))}else{v2})));
        let v4179=(if v738{((v4151+v4151)+((v743*v4143)+(v728*(((v740*v4127)+(v723*(v45*v4127)))-v4165))))}else{v2});
        let v4180=(if v738{((v4153+v4153)+((v743*v4144)+(v728*(((v740*v4128)+(v723*(v45*v4128)))-v4168))))}else{v2});
        let v4208=(v746*v746);
        let v4220=(v723*v4127);
        let v4222=(v723*v4128);
        let v4239=(v757*v757);
        let v4247=(if v738{(v4065+(((v757*((v747*v4143)+(v728*((v730*v4101)+(v714*v4147)))))-(v748*(v4179+((v755*((v751*v4127)+(v723*(((v746*((v749*v4143)+(v728*((v730*v4143)+(v728*v4147)))))-(v750*v4179))/v4208))))+(v752*((v40*(v4220+v4220))-v4165))))))/v4239))}else{(if v735{v4065}else{v2})});
        let v4248=(if v738{(v4066+(((v757*((v747*v4144)+(v728*((v730*v4102)+(v714*v4148)))))-(v748*(v4180+((v755*((v751*v4128)+(v723*(((v746*((v749*v4144)+(v728*((v730*v4144)+(v728*v4148)))))-(v750*v4180))/v4208))))+(v752*((v40*(v4222+v4222))-v4168))))))/v4239))}else{(if v735{v4066}else{v2})});
        let v4251=(if v763{(v764*v4247)}else{v3790});
        let v4252=(if v763{(v764*v4248)}else{v3791});
        let v4254=(v765*v765);
        let v4272=(if v775{(v777*(v4247-v3526))}else{(if v763{((v765*v3557)+(v479*v4251))}else{v4251})});
        let v4273=(if v775{(v777*(v4248-v3527))}else{(if v763{((v765*v3558)+(v479*v4252))}else{v4252})});
        let v4277=(v778*v778);
        let v4285=(v3526-v4247);
        let v4286=(v3527-v4248);
        let v4305=(v791*v791);
        let v4310=(if v782{((-(v642*((v789*v4285)+(v784*((v787*(v45*v4285))+(v785*(v40*v4285)))))))/v4305)}else{v4272});
        let v4311=(if v782{((-(v642*((v789*v4286)+(v784*((v787*(v45*v4286))+(v785*(v40*v4286)))))))/v4305)}else{v4273});
        let v4330=(v801*v801);
        let v4335=(if v782{((-(v642*((v799*v4247)+(v794*((v797*(v45*v4247))+(v795*(v40*v4247)))))))/v4330)}else{(if v775{(((v778*v3557)-(v479*v4272))/v4277)}else{(if v763{((-v4251)/v4254)}else{v3797})})});
        let v4336=(if v782{((-(v642*((v799*v4248)+(v794*((v797*(v45*v4248))+(v795*(v40*v4248)))))))/v4330)}else{(if v775{(((v778*v3558)-(v479*v4273))/v4277)}else{(if v763{((-v4252)/v4254)}else{v3798})})});
        let v4337=(v760*v4247);
        let v4339=(v760*v4248);
        let v4342=(v805*v805);
        let v4350=(if v621{(self.scalar_static_f64[351]-v4247)}else{(if v621{((-(v4337+v4337))/v4342)}else{v4069})});
        let v4351=(if v621{(self.scalar_static_f64[352]-v4248)}else{(if v621{((-(v4339+v4339))/v4342)}else{v4070})});
        let v4368=(if v621{((v10*v4350)+((v813*v3511)+(v457*((v4310+(-v4335))-v3557))))}else{v3836});
        let v4369=(if v621{((v10*v4351)+((v813*v3513)+(v457*((v4311+(-v4336))-v3558))))}else{v3837});
        let v4370=(v809*v4350);
        let v4372=(v809*v4351);
        let v4406=(if v621{(-((v827*v3511)+(v457*(v4310+v4335))))}else{v4350});
        let v4407=(if v621{(-((v827*v3513)+(v457*(v4311+v4336))))}else{v4351});
        let v4408=(v816*v4368);
        let v4410=(v816*v4369);
        let v4412=(v10*(if v621{((v4370+v4370)-((v823*v3511)+(v457*((v4310+(v4247+v4335))-((v821*v3557)+(v479*v4247))))))}else{v3862}));
        let v4413=(v10*(if v621{((v4372+v4372)-((v823*v3513)+(v457*((v4311+(v4248+v4336))-((v821*v3558)+(v479*v4248))))))}else{v3863}));
        let v4424=(v10*v836);
        let v4432=(v837*v837);
        let v4440=(if v621{(v4247+(((v837*v4412)-(v832*(v4368+((if v621{((v4408+v4408)-((v832*v4406)+(v830*v4412)))}else{v4406})/v4424))))/v4432))}else{(if v505{((-v3766)-(((v616*v3880)-(v610*(v3836+(v3890/v3894))))/v3902))}else{(if (v487!=0.0){((v498*v3575)+(v493*((v496*v3571)+(v492*((v495*v3508)+(v456*((v494*self.scalar_static_f64[351])+(v484*v3579))))))))}else{v2})})});
        let v4441=(if v621{(v4248+(((v837*v4413)-(v832*(v4369+((if v621{((v4410+v4410)-((v832*v4407)+(v830*v4413)))}else{v4407})/v4424))))/v4432))}else{(if v505{((-v3767)-(((v616*v3881)-(v610*(v3837+(v3891/v3894))))/v3902))}else{(if (v487!=0.0){((v498*v3578)+(v493*((v496*v3572)+(v492*((v495*v3509)+(v456*((v494*self.scalar_static_f64[352])+(v484*v3580))))))))}else{v2})})});
        let v4450=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(self.scalar_static_f64[20]-(self.scalar_static_f64[172]*v4440))))}else{v2});
        let v4451=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(self.scalar_static_f64[150]-(self.scalar_static_f64[172]*v4441))))}else{v2});
        let v4452=(self.scalar_static_f64[224]*v4450);
        let v4453=(self.scalar_static_f64[224]*v4451);
        let v4470=(if v873{(-v4450)}else{v2});
        let v4471=(if v873{(-v4451)}else{v2});
        let v4476=(if v873{(self.scalar_static_f64[224]*(v162*v4470))}else{v2});
        let v4477=(if v873{(self.scalar_static_f64[224]*(v162*v4471))}else{v2});
        let v4478=(v880*v4476);
        let v4480=(v880*v4477);
        let v4482=(v10*v883);
        let v4489=(if v873{(v45*(v4476-((v4478+v4478)/v4482)))}else{v2});
        let v4490=(if v873{(v45*(v4477-((v4480+v4480)/v4482)))}else{v2});
        let v4493=(if v873{(v4470-v4489)}else{v2});
        let v4494=(if v873{(v4471-v4490)}else{v2});
        let v4495=(v888*v4493);
        let v4497=(v888*v4494);
        let v4503=(if v873{((v4495+v4495)+(self.scalar_static_f64[220]*v4489))}else{v2});
        let v4504=(if v873{((v4497+v4497)+(self.scalar_static_f64[220]*v4490))}else{v2});
        let v4507=(if v873{(v10*v4493)}else{v2});
        let v4508=(if v873{(v10*v4494)}else{v2});
        let v4517=(if v873{((-v4489)+((self.scalar_static_f64[221]*v4503)/v898))}else{v2});
        let v4518=(if v873{((-v4490)+((self.scalar_static_f64[221]*v4504)/v898))}else{v2});
        let v4521=(if v873{(v4503+v4507)}else{v2});
        let v4522=(if v873{(v4504+v4508)}else{v2});
        let v4523=(v903*v4521);
        let v4525=(v903*v4522);
        let v4545=(if v873{((v4523+v4523)+((v907*v4517)+(v901*(((v905*v4507)+(v896*(v45*v4507)))-v4503))))}else{v2});
        let v4546=(if v873{((v4525+v4525)+((v907*v4518)+(v901*(((v905*v4508)+(v896*(v45*v4508)))-v4504))))}else{v2});
        let v4574=(v910*v910);
        let v4586=(v896*v4507);
        let v4588=(v896*v4508);
        let v4605=(v921*v921);
        let v4613=(if v873{(v4489+(((v921*((v911*v4517)+(v901*((v903*v4503)+(v893*v4521)))))-(v912*(v4545+((v919*((v915*v4507)+(v896*(((v910*((v913*v4517)+(v901*((v903*v4517)+(v901*v4521)))))-(v914*v4545))/v4574))))+(v916*((v40*(v4586+v4586))-v4503))))))/v4605))}else{v2});
        let v4614=(if v873{(v4490+(((v921*((v911*v4518)+(v901*((v903*v4504)+(v893*v4522)))))-(v912*(v4546+((v919*((v915*v4508)+(v896*(((v910*((v913*v4518)+(v901*((v903*v4518)+(v901*v4522)))))-(v914*v4546))/v4574))))+(v916*((v40*(v4588+v4588))-v4504))))))/v4605))}else{v2});
        let v4637=(if v931{(v567*((v937*v4613)+(v932*((v935*(v45*v4613))+(v933*(v40*v4613))))))}else{(if v927{(v928*v4613)}else{v2})});
        let v4638=(if v931{(v567*((v937*v4614)+(v932*((v935*(v45*v4614))+(v933*(v40*v4614))))))}else{(if v927{(v928*v4614)}else{v2})});
        let v4640=(v941*v941);
        let v4644=(if v873{((-v4637)/v4640)}else{v2});
        let v4645=(if v873{((-v4638)/v4640)}else{v2});
        let v4646=(v924*v4613);
        let v4648=(v924*v4614);
        let v4651=(v945*v945);
        let v4659=(if v873{(v4470-v4613)}else{(if v873{((-(v4646+v4646))/v4651)}else{v4493})});
        let v4660=(if v873{(v4471-v4614)}else{(if v873{((-(v4648+v4648))/v4651)}else{v4494})});
        let v4663=(if v873{(self.scalar_static_f64[247]*v4644)}else{v2});
        let v4664=(if v873{(self.scalar_static_f64[247]*v4645)}else{v2});
        let v4673=(if v873{((v10*v4659)+(self.scalar_static_f64[220]*(v4637-v4663)))}else{v2});
        let v4674=(if v873{((v10*v4660)+(self.scalar_static_f64[220]*(v4638-v4664)))}else{v2});
        let v4675=(v949*v4659);
        let v4677=(v949*v4660);
        let v4691=(if v873{((v4675+v4675)-(self.scalar_static_f64[220]*((v4663+(v4637-v4613))+(self.scalar_static_f64[247]*v4613))))}else{v2});
        let v4692=(if v873{((v4677+v4677)-(self.scalar_static_f64[220]*((v4664+(v4638-v4614))+(self.scalar_static_f64[247]*v4614))))}else{v2});
        let v4699=(if v873{(-(self.scalar_static_f64[220]*(v4637+v4663)))}else{v4659});
        let v4700=(if v873{(-(self.scalar_static_f64[220]*(v4638+v4664)))}else{v4660});
        let v4701=(v958*v4673);
        let v4703=(v958*v4674);
        let v4705=(v10*v4691);
        let v4706=(v10*v4692);
        let v4715=(if v873{((v4701+v4701)-((v974*v4699)+(v972*v4705)))}else{v4699});
        let v4716=(if v873{((v4703+v4703)-((v974*v4700)+(v972*v4706)))}else{v4700});
        let v4719=(v10*v979);
        let v4727=(v980*v980);
        let v4745=(if v985{((v996*v4452)+(v860*(v994*v4450)))}else{v2});
        let v4746=(if v985{((v996*v4453)+(v860*(v994*v4451)))}else{v2});
        let v4771=(v1014*v1014);
        let v4776=(if v1006{((-(v642*((v1012*v4745)+(v1007*((v1010*(v45*v4745))+(v1008*(v40*v4745)))))))/v4771)}else{(if v1002{(v1003*(-v4745))}else{v4715})});
        let v4777=(if v1006{((-(v642*((v1012*v4746)+(v1007*((v1010*(v45*v4746))+(v1008*(v40*v4746)))))))/v4771)}else{(if v1002{(v1003*(-v4746))}else{v4716})});
        let v4784=(v10*v1024);
        let v4791=(if v985{(v4450-(self.scalar_static_f64[219]*((v4450-(if v985{(-v4776)}else{v2}))/v4784)))}else{v2});
        let v4792=(if v985{(v4451-(self.scalar_static_f64[219]*((v4451-(if v985{(-v4777)}else{v2}))/v4784)))}else{v2});
        let v4793=(-v4791);
        let v4794=(-v4792);
        let v4795=(v1030*v4793);
        let v4797=(v1030*v4794);
        let v4799=(v10*v1034);
        let v4808=(v1038*v4791);
        let v4810=(v1038*v4792);
        let v4812=(v10*v1042);
        let v4819=(v1043*v1043);
        let v4834=(if v985{(if v1031{(-(v45*(v4793+((v4795+v4795)/v4799))))}else{(if v1039{(-((-(v678*(v4791+((v4808+v4808)/v4812))))/v4819))}else{(-(v45*v4793))})})}else{v4489});
        let v4835=(if v985{(if v1031{(-(v45*(v4794+((v4797+v4797)/v4799))))}else{(if v1039{(-((-(v678*(v4792+((v4810+v4810)/v4812))))/v4819))}else{(-(v45*v4794))})})}else{v4490});
        let v4838=(if v985{(v4450-v4834)}else{v4776});
        let v4839=(if v985{(v4451-v4835)}else{v4777});
        let v4840=(-v4834);
        let v4841=(-v4835);
        let v4844=(if v985{(v1061*v4840)}else{v4663});
        let v4845=(if v985{(v1061*v4841)}else{v4664});
        let v4846=(v1059*v4838);
        let v4848=(v1059*v4839);
        let v4862=(if v985{(if v1071{v2}else{((v4846+v4846)-(self.scalar_static_f64[220]*((v4834+v4844)-(self.scalar_static_f64[247]*v4834))))})}else{v4503});
        let v4863=(if v985{(if v1071{v2}else{((v4848+v4848)-(self.scalar_static_f64[220]*((v4835+v4845)-(self.scalar_static_f64[247]*v4835))))})}else{v4504});
        let v4878=(if v985{((v10*v4838)+(self.scalar_static_f64[220]*(-v4844)))}else{v4507});
        let v4879=(if v985{((v10*v4839)+(self.scalar_static_f64[220]*(-v4845)))}else{v4508});
        let v4886=(if v985{(v4840+((v4862/self.scalar_static_f64[220])/v1084))}else{v4517});
        let v4887=(if v985{(v4841+((v4863/self.scalar_static_f64[220])/v1084))}else{v4518});
        let v4890=(if v985{(v4862+v4878)}else{v2});
        let v4891=(if v985{(v4863+v4879)}else{v2});
        let v4894=(v1089*v4890);
        let v4896=(v1089*v4891);
        let v4908=((v1076*v4862)+(v1073*(if v985{(-(self.scalar_static_f64[329]*v4844))}else{v2})));
        let v4911=((v1076*v4863)+(v1073*(if v985{(-(self.scalar_static_f64[329]*v4845))}else{v2})));
        let v4922=(if v1096{((v4894+v4894)+((v1101*v4886)+(v1087*(((v1098*v4878)+(v1082*(v45*v4878)))-v4908))))}else{v2});
        let v4923=(if v1096{((v4896+v4896)+((v1101*v4887)+(v1087*(((v1098*v4879)+(v1082*(v45*v4879)))-v4911))))}else{v2});
        let v4951=(v1104*v1104);
        let v4963=(v1082*v4878);
        let v4965=(v1082*v4879);
        let v4982=(v1115*v1115);
        let v4990=(if v1096{(v4834+(((v1115*((v1105*v4886)+(v1087*((v1089*v4862)+(v1073*v4890)))))-(v1106*(v4922+((v1113*((v1109*v4878)+(v1082*(((v1104*((v1107*v4886)+(v1087*((v1089*v4886)+(v1087*v4890)))))-(v1108*v4922))/v4951))))+(v1110*((v40*(v4963+v4963))-v4908))))))/v4982))}else{(if v1093{v4834}else{v2})});
        let v4991=(if v1096{(v4835+(((v1115*((v1105*v4887)+(v1087*((v1089*v4863)+(v1073*v4891)))))-(v1106*(v4923+((v1113*((v1109*v4879)+(v1082*(((v1104*((v1107*v4887)+(v1087*((v1089*v4887)+(v1087*v4891)))))-(v1108*v4923))/v4951))))+(v1110*((v40*(v4965+v4965))-v4911))))))/v4982))}else{(if v1093{v4835}else{v2})});
        let v4994=(if v1121{(v1122*v4990)}else{v4637});
        let v4995=(if v1121{(v1122*v4991)}else{v4638});
        let v4997=(v1123*v1123);
        let v5009=(if v1133{(v1135*v4990)}else{(if v1121{(self.scalar_static_f64[247]*v4994)}else{v4994})});
        let v5010=(if v1133{(v1135*v4991)}else{(if v1121{(self.scalar_static_f64[247]*v4995)}else{v4995})});
        let v5013=(v1136*v1136);
        let v5020=(-v4990);
        let v5021=(-v4991);
        let v5040=(v1149*v1149);
        let v5045=(if v1140{((-(v642*((v1147*v5020)+(v1142*((v1145*(v45*v5020))+(v1143*(v40*v5020)))))))/v5040)}else{v5009});
        let v5046=(if v1140{((-(v642*((v1147*v5021)+(v1142*((v1145*(v45*v5021))+(v1143*(v40*v5021)))))))/v5040)}else{v5010});
        let v5065=(v1159*v1159);
        let v5070=(if v1140{((-(v642*((v1157*v4990)+(v1152*((v1155*(v45*v4990))+(v1153*(v40*v4990)))))))/v5065)}else{(if v1133{((-(self.scalar_static_f64[247]*v5009))/v5013)}else{(if v1121{((-v4994)/v4997)}else{v4644})})});
        let v5071=(if v1140{((-(v642*((v1157*v4991)+(v1152*((v1155*(v45*v4991))+(v1153*(v40*v4991)))))))/v5065)}else{(if v1133{((-(self.scalar_static_f64[247]*v5010))/v5013)}else{(if v1121{((-v4995)/v4997)}else{v4645})})});
        let v5072=(v1118*v4990);
        let v5074=(v1118*v4991);
        let v5077=(v1163*v1163);
        let v5085=(if v985{(v4450-v4990)}else{(if v985{((-(v5072+v5072))/v5077)}else{v4838})});
        let v5086=(if v985{(v4451-v4991)}else{(if v985{((-(v5074+v5074))/v5077)}else{v4839})});
        let v5097=(if v985{((v10*v5085)+(self.scalar_static_f64[220]*(v5045+(-v5070))))}else{v4673});
        let v5098=(if v985{((v10*v5086)+(self.scalar_static_f64[220]*(v5046+(-v5071))))}else{v4674});
        let v5099=(v1167*v5085);
        let v5101=(v1167*v5086);
        let v5123=(if v985{(-(self.scalar_static_f64[220]*(v5045+v5070)))}else{v5085});
        let v5124=(if v985{(-(self.scalar_static_f64[220]*(v5046+v5071)))}else{v5086});
        let v5125=(v1174*v5097);
        let v5127=(v1174*v5098);
        let v5129=(v10*(if v985{((v5099+v5099)-(self.scalar_static_f64[220]*((v5045+(v4990+v5070))-(self.scalar_static_f64[247]*v4990))))}else{v4691}));
        let v5130=(v10*(if v985{((v5101+v5101)-(self.scalar_static_f64[220]*((v5046+(v4991+v5071))-(self.scalar_static_f64[247]*v4991))))}else{v4692}));
        let v5141=(v10*v1194);
        let v5149=(v1195*v1195);
        let v5169=(if (self.scalar_static_f64[149]!=0.0){((self.scalar_static_f64[20]-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v985{(v4990+(((v1195*v5129)-(v1190*(v5097+((if v985{((v5125+v5125)-((v1190*v5123)+(v1188*v5129)))}else{v5123})/v5141))))/v5149))}else{(if v873{((-v4613)-(((v980*v4705)-(v974*(v4673+(v4715/v4719))))/v4727))}else{(if v855{((v865*v4452)+(v860*(v859*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v4450)))))}else{v2})})})))}else{v2}))/self.scalar_static_f64[172])}else{self.scalar_static_f64[351]});
        let v5170=(if (self.scalar_static_f64[149]!=0.0){((self.scalar_static_f64[150]-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v985{(v4991+(((v1195*v5130)-(v1190*(v5098+((if v985{((v5127+v5127)-((v1190*v5124)+(v1188*v5130)))}else{v5124})/v5141))))/v5149))}else{(if v873{((-v4614)-(((v980*v4706)-(v974*(v4674+(v4716/v4719))))/v4727))}else{(if v855{((v865*v4453)+(v860*(v859*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v4451)))))}else{v2})})})))}else{v2}))/self.scalar_static_f64[172])}else{self.scalar_static_f64[352]});
        let v5171=(if v1208{v3569}else{v2});
        let v5172=(if v1208{v3570}else{v2});
        let v5175=((v1204*v3523)+(v461*v5169));
        let v5178=((v1204*v3525)+(v461*v5170));
        let v5207=(if v1221{(-v5169)}else{v2});
        let v5208=(if v1221{(-v5170)}else{v2});
        let v5217=(if v1221{((v1224*v3523)+(v461*(v162*v5207)))}else{v2});
        let v5218=(if v1221{((v1224*v3525)+(v461*(v162*v5208)))}else{v2});
        let v5219=(v1228*v5217);
        let v5221=(v1228*v5218);
        let v5223=(v10*v1231);
        let v5230=(if v1221{(v45*(v5217-((v5219+v5219)/v5223)))}else{v2});
        let v5231=(if v1221{(v45*(v5218-((v5221+v5221)/v5223)))}else{v2});
        let v5234=(if v1221{(v5207-v5230)}else{v2});
        let v5235=(if v1221{(v5208-v5231)}else{v2});
        let v5236=(v1236*v5234);
        let v5238=(v1236*v5235);
        let v5248=(if v1221{((v5236+v5236)+((v1238*v3511)+(v457*v5230)))}else{v2});
        let v5249=(if v1221{((v5238+v5238)+((v1238*v3513)+(v457*v5231)))}else{v2});
        let v5254=(if v1221{((v10*v5234)-v3511)}else{v2});
        let v5255=(if v1221{((v10*v5235)-v3513)}else{v2});
        let v5268=(if v1221{((-v5230)+(((v1241*v3516)+(v458*v5248))/v1246))}else{v2});
        let v5269=(if v1221{((-v5231)+(((v1241*v3518)+(v458*v5249))/v1246))}else{v2});
        let v5272=(if v1221{(v5248+v5254)}else{v2});
        let v5273=(if v1221{(v5249+v5255)}else{v2});
        let v5274=(v1251*v5272);
        let v5276=(v1251*v5273);
        let v5296=(if v1221{((v5274+v5274)+((v1255*v5268)+(v1249*(((v1253*v5254)+(v1244*(v45*v5254)))-v5248))))}else{v2});
        let v5297=(if v1221{((v5276+v5276)+((v1255*v5269)+(v1249*(((v1253*v5255)+(v1244*(v45*v5255)))-v5249))))}else{v2});
        let v5325=(v1258*v1258);
        let v5337=(v1244*v5254);
        let v5339=(v1244*v5255);
        let v5356=(v1269*v1269);
        let v5364=(if v1221{(v5230+(((v1269*((v1259*v5268)+(v1249*((v1251*v5248)+(v1241*v5272)))))-(v1260*(v5296+((v1267*((v1263*v5254)+(v1244*(((v1258*((v1261*v5268)+(v1249*((v1251*v5268)+(v1249*v5272)))))-(v1262*v5296))/v5325))))+(v1264*((v40*(v5337+v5337))-v5248))))))/v5356))}else{v2});
        let v5365=(if v1221{(v5231+(((v1269*((v1259*v5269)+(v1249*((v1251*v5249)+(v1241*v5273)))))-(v1260*(v5297+((v1267*((v1263*v5255)+(v1244*(((v1258*((v1261*v5269)+(v1249*((v1251*v5269)+(v1249*v5273)))))-(v1262*v5297))/v5325))))+(v1264*((v40*(v5339+v5339))-v5249))))))/v5356))}else{v2});
        let v5388=(if v1279{(v567*((v1285*v5364)+(v1280*((v1283*(v45*v5364))+(v1281*(v40*v5364))))))}else{(if v1275{(v1276*v5364)}else{v2})});
        let v5389=(if v1279{(v567*((v1285*v5365)+(v1280*((v1283*(v45*v5365))+(v1281*(v40*v5365))))))}else{(if v1275{(v1276*v5365)}else{v2})});
        let v5391=(v1289*v1289);
        let v5395=(if v1221{((-v5388)/v5391)}else{v2});
        let v5396=(if v1221{((-v5389)/v5391)}else{v2});
        let v5397=(v1272*v5364);
        let v5399=(v1272*v5365);
        let v5402=(v1293*v1293);
        let v5410=(if v1221{(v5207-v5364)}else{(if v1221{((-(v5397+v5397))/v5402)}else{v5234})});
        let v5411=(if v1221{(v5208-v5365)}else{(if v1221{((-(v5399+v5399))/v5402)}else{v5235})});
        let v5418=(if v1221{((v1291*v3557)+(v479*v5395))}else{v5171});
        let v5419=(if v1221{((v1291*v3558)+(v479*v5396))}else{v5172});
        let v5434=(if v1221{((v10*v5410)+((v1303*v3511)+(v457*(v3557+(v5388-v5418)))))}else{v2});
        let v5435=(if v1221{((v10*v5411)+((v1303*v3513)+(v457*(v3558+(v5389-v5419)))))}else{v2});
        let v5436=(v1297*v5410);
        let v5438=(v1297*v5411);
        let v5460=(if v1221{((v5436+v5436)-((v1313*v3511)+(v457*((v5418+(v5388-v5364))+((v1311*v3557)+(v479*v5364))))))}else{v2});
        let v5461=(if v1221{((v5438+v5438)-((v1313*v3513)+(v457*((v5419+(v5389-v5365))+((v1311*v3558)+(v479*v5365))))))}else{v2});
        let v5472=(if v1221{(-((v1317*v3511)+(v457*(v5388+v5418))))}else{v5410});
        let v5473=(if v1221{(-((v1317*v3513)+(v457*(v5389+v5419))))}else{v5411});
        let v5474=(v1306*v5434);
        let v5476=(v1306*v5435);
        let v5478=(v10*v5460);
        let v5479=(v10*v5461);
        let v5488=(if v1221{((v5474+v5474)-((v1322*v5472)+(v1320*v5478)))}else{v5472});
        let v5489=(if v1221{((v5476+v5476)-((v1322*v5473)+(v1320*v5479)))}else{v5473});
        let v5492=(v10*v1327);
        let v5500=(v1328*v1328);
        let v5510=(if v1333{v3914}else{v2});
        let v5511=(if v1333{v3916}else{v2});
        let v5538=(if v1333{((v1340*v5175)+(v1210*((v1338*v5169)+(v1204*(if v1333{((v1336*v5510)+(v1334*((v1334*v3919)+(v624*v5510))))}else{v2})))))}else{v2});
        let v5539=(if v1333{((v1340*v5178)+(v1210*((v1338*v5170)+(v1204*(if v1333{((v1336*v5511)+(v1334*((v1334*v3920)+(v624*v5511))))}else{v2})))))}else{v2});
        let v5564=(v1358*v1358);
        let v5569=(if v1350{((-(v642*((v1356*v5538)+(v1351*((v1354*(v45*v5538))+(v1352*(v40*v5538)))))))/v5564)}else{(if v1346{(v1347*(-v5538))}else{v5488})});
        let v5570=(if v1350{((-(v642*((v1356*v5539)+(v1351*((v1354*(v45*v5539))+(v1352*(v40*v5539)))))))/v5564)}else{(if v1346{(v1347*(-v5539))}else{v5489})});
        let v5581=(v10*v1366);
        let v5592=(if v1333{((v3984+v5169)-((v1366*v3508)+(v456*(((v3988+v5169)-(if v1333{(-v5569)}else{v2}))/v5581))))}else{v2});
        let v5593=(if v1333{((v3985+v5170)-((v1366*v3509)+(v456*(((v3989+v5170)-(if v1333{(-v5570)}else{v2}))/v5581))))}else{v2});
        let v5594=(if v1333{v3526}else{v2});
        let v5595=(if v1333{v3527}else{v2});
        let v5596=(v5594-v5592);
        let v5597=(v5595-v5593);
        let v5598=(v1371*v5596);
        let v5600=(v1371*v5597);
        let v5602=(v10*v1375);
        let v5611=(v5592-v5594);
        let v5612=(v5593-v5595);
        let v5613=(v1379*v5611);
        let v5615=(v1379*v5612);
        let v5617=(v10*v1383);
        let v5624=(v1384*v1384);
        let v5639=(v1370*v5594);
        let v5641=(v1370*v5595);
        let v5643=(v10*v1394);
        let v5652=(if v1333{((if v1372{(v5594-(v45*(v5596+((v5598+v5598)/v5602))))}else{(if v1380{(v5594-((-(v678*(v5611+((v5613+v5613)/v5617))))/v5624))}else{(v5594-(v45*v5596))})})-(v45*(v5594-((v5639+v5639)/v5643))))}else{v5230});
        let v5653=(if v1333{((if v1372{(v5595-(v45*(v5597+((v5600+v5600)/v5602))))}else{(if v1380{(v5595-((-(v678*(v5612+((v5615+v5615)/v5617))))/v5624))}else{(v5595-(v45*v5597))})})-(v45*(v5595-((v5641+v5641)/v5643))))}else{v5231});
        let v5656=(if v1333{(v5169-v5652)}else{v5569});
        let v5657=(if v1333{(v5170-v5653)}else{v5570});
        let v5662=(if v1333{(v1402*(-v5652))}else{v5418});
        let v5663=(if v1333{(v1402*(-v5653))}else{v5419});
        let v5664=(v1400*v5656);
        let v5666=(v1400*v5657);
        let v5688=(if v1333{(if v1412{v2}else{((v5664+v5664)-((v1409*v3511)+(v457*((v5652+v5662)-((v1407*v3557)+(v479*v5652))))))})}else{v5248});
        let v5689=(if v1333{(if v1412{v2}else{((v5666+v5666)-((v1409*v3513)+(v457*((v5653+v5663)-((v1407*v3558)+(v479*v5653))))))})}else{v5249});
        let v5714=(if v1333{((v10*v5656)+((v1420*v3511)+(v457*((-v5662)-v3557))))}else{v5254});
        let v5715=(if v1333{((v10*v5657)+((v1420*v3513)+(v457*((-v5663)-v3558))))}else{v5255});
        let v5730=(if v1333{((v3526-v5652)+((((v457*v5688)-(v1414*v3511))/v3515)/v1425))}else{v5268});
        let v5731=(if v1333{((v3527-v5653)+((((v457*v5689)-(v1414*v3513))/v3515)/v1425))}else{v5269});
        let v5734=(if v1333{(v5688+v5714)}else{v2});
        let v5735=(if v1333{(v5689+v5715)}else{v2});
        let v5738=(v1430*v5734);
        let v5740=(v1430*v5735);
        let v5752=((v1417*v5688)+(v1414*(if v1333{(-((v1403*v3984)+(v655*v5662)))}else{v2})));
        let v5755=((v1417*v5689)+(v1414*(if v1333{(-((v1403*v3985)+(v655*v5663)))}else{v2})));
        let v5766=(if v1437{((v5738+v5738)+((v1442*v5730)+(v1428*(((v1439*v5714)+(v1423*(v45*v5714)))-v5752))))}else{v2});
        let v5767=(if v1437{((v5740+v5740)+((v1442*v5731)+(v1428*(((v1439*v5715)+(v1423*(v45*v5715)))-v5755))))}else{v2});
        let v5795=(v1445*v1445);
        let v5807=(v1423*v5714);
        let v5809=(v1423*v5715);
        let v5826=(v1456*v1456);
        let v5834=(if v1437{(v5652+(((v1456*((v1446*v5730)+(v1428*((v1430*v5688)+(v1414*v5734)))))-(v1447*(v5766+((v1454*((v1450*v5714)+(v1423*(((v1445*((v1448*v5730)+(v1428*((v1430*v5730)+(v1428*v5734)))))-(v1449*v5766))/v5795))))+(v1451*((v40*(v5807+v5807))-v5752))))))/v5826))}else{(if v1434{v5652}else{v2})});
        let v5835=(if v1437{(v5653+(((v1456*((v1446*v5731)+(v1428*((v1430*v5689)+(v1414*v5735)))))-(v1447*(v5767+((v1454*((v1450*v5715)+(v1423*(((v1445*((v1448*v5731)+(v1428*((v1430*v5731)+(v1428*v5735)))))-(v1449*v5767))/v5795))))+(v1451*((v40*(v5809+v5809))-v5755))))))/v5826))}else{(if v1434{v5653}else{v2})});
        let v5838=(if v1462{(v1463*v5834)}else{v5388});
        let v5839=(if v1462{(v1463*v5835)}else{v5389});
        let v5841=(v1464*v1464);
        let v5859=(if v1473{(v1475*(v5834-v3526))}else{(if v1462{((v1464*v3557)+(v479*v5838))}else{v5838})});
        let v5860=(if v1473{(v1475*(v5835-v3527))}else{(if v1462{((v1464*v3558)+(v479*v5839))}else{v5839})});
        let v5864=(v1476*v1476);
        let v5872=(v3526-v5834);
        let v5873=(v3527-v5835);
        let v5892=(v1489*v1489);
        let v5897=(if v1480{((-(v642*((v1487*v5872)+(v1482*((v1485*(v45*v5872))+(v1483*(v40*v5872)))))))/v5892)}else{v5859});
        let v5898=(if v1480{((-(v642*((v1487*v5873)+(v1482*((v1485*(v45*v5873))+(v1483*(v40*v5873)))))))/v5892)}else{v5860});
        let v5917=(v1499*v1499);
        let v5922=(if v1480{((-(v642*((v1497*v5834)+(v1492*((v1495*(v45*v5834))+(v1493*(v40*v5834)))))))/v5917)}else{(if v1473{(((v1476*v3557)-(v479*v5859))/v5864)}else{(if v1462{((-v5838)/v5841)}else{v5395})})});
        let v5923=(if v1480{((-(v642*((v1497*v5835)+(v1492*((v1495*(v45*v5835))+(v1493*(v40*v5835)))))))/v5917)}else{(if v1473{(((v1476*v3558)-(v479*v5860))/v5864)}else{(if v1462{((-v5839)/v5841)}else{v5396})})});
        let v5924=(v1459*v5834);
        let v5926=(v1459*v5835);
        let v5929=(v1503*v1503);
        let v5937=(if v1333{(v5169-v5834)}else{(if v1333{((-(v5924+v5924))/v5929)}else{v5656})});
        let v5938=(if v1333{(v5170-v5835)}else{(if v1333{((-(v5926+v5926))/v5929)}else{v5657})});
        let v5955=(if v1333{((v10*v5937)+((v1511*v3511)+(v457*((v5897+(-v5922))-v3557))))}else{v5434});
        let v5956=(if v1333{((v10*v5938)+((v1511*v3513)+(v457*((v5898+(-v5923))-v3558))))}else{v5435});
        let v5957=(v1507*v5937);
        let v5959=(v1507*v5938);
        let v5993=(if v1333{(-((v1525*v3511)+(v457*(v5897+v5922))))}else{v5937});
        let v5994=(if v1333{(-((v1525*v3513)+(v457*(v5898+v5923))))}else{v5938});
        let v5995=(v1514*v5955);
        let v5997=(v1514*v5956);
        let v5999=(v10*(if v1333{((v5957+v5957)-((v1521*v3511)+(v457*((v5897+(v5834+v5922))-((v1519*v3557)+(v479*v5834))))))}else{v5460}));
        let v6000=(v10*(if v1333{((v5959+v5959)-((v1521*v3513)+(v457*((v5898+(v5835+v5923))-((v1519*v3558)+(v479*v5835))))))}else{v5461}));
        let v6011=(v10*v1534);
        let v6019=(v1535*v1535);
        let v6027=(if v1333{(v5834+(((v1535*v5999)-(v1530*(v5955+((if v1333{((v5995+v5995)-((v1530*v5993)+(v1528*v5999)))}else{v5993})/v6011))))/v6019))}else{(if v1221{((-v5364)-(((v1328*v5478)-(v1322*(v5434+(v5488/v5492))))/v5500))}else{(if v1208{((v1214*v5175)+(v1210*((v1212*v5171)+(v1209*((v1211*v3508)+(v456*((v1204*v3579)+(v494*v5169))))))))}else{v4440})})});
        let v6028=(if v1333{(v5835+(((v1535*v6000)-(v1530*(v5956+((if v1333{((v5997+v5997)-((v1530*v5994)+(v1528*v6000)))}else{v5994})/v6011))))/v6019))}else{(if v1221{((-v5365)-(((v1328*v5479)-(v1322*(v5435+(v5489/v5492))))/v5500))}else{(if v1208{((v1214*v5178)+(v1210*((v1212*v5172)+(v1209*((v1211*v3509)+(v456*((v1204*v3580)+(v494*v5170))))))))}else{v4441})})});
        let v6031=(if v1548{(v1549*v6027)}else{v2});
        let v6032=(if v1548{(v1549*v6028)}else{v2});
        let v6034=(v1550*v1550);
        let v6038=(if v1548{((-v6031)/v6034)}else{v2});
        let v6039=(if v1548{((-v6032)/v6034)}else{v2});
        let v6067=(if v1564{(v1566*(v6027-v3526))}else{(if v1548{((v1550*v3557)+(v479*v6031))}else{v6031})});
        let v6068=(if v1564{(v1566*(v6028-v3527))}else{(if v1548{((v1550*v3558)+(v479*v6032))}else{v6032})});
        let v6072=(v1567*v1567);
        let v6117=(v45*v6027);
        let v6118=(v45*v6028);
        let v6119=(v40*v6027);
        let v6120=(v40*v6028);
        let v6135=(v1594*v1594);
        let v6140=(if v1575{((-(v642*((v1592*v6027)+(v1587*((v1590*v6117)+(v1588*v6119))))))/v6135)}else{(if v1564{(((v1567*v3557)-(v479*v6067))/v6072)}else{v6038})});
        let v6141=(if v1575{((-(v642*((v1592*v6028)+(v1587*((v1590*v6118)+(v1588*v6120))))))/v6135)}else{(if v1564{(((v1567*v3558)-(v479*v6068))/v6072)}else{v6039})});
        let v6162=(-((v1606*v6119)+(v1604*(-(v657*v6027)))));
        let v6163=(-((v1606*v6120)+(v1604*(-(v657*v6028)))));
        let v6202=(v10*v1620);
        let v6205=(if v1601{(v6162/v6202)}else{v2});
        let v6206=(if v1601{(v6163/v6202)}else{v2});
        let v6219=(if v1626{(v6027+v6140)}else{(if v1601{((v1608*((v1602*v6027)+(v1538*v6117)))+(v1603*v6162))}else{v2})});
        let v6220=(if v1626{(v6028+v6141)}else{(if v1601{((v1608*((v1602*v6028)+(v1538*v6118)))+(v1603*v6163))}else{v2})});
        let v6221=(v10*v1630);
        let v6224=(if v1626{(v6219/v6221)}else{(if v1601{((v1622*v6205)+(v1621*(v150*v6027)))}else{v2})});
        let v6225=(if v1626{(v6220/v6221)}else{(if v1601{((v1622*v6206)+(v1621*(v150*v6028)))}else{v2})});
        let v6269=(((v460*self.scalar_static_f64[351])-(v1644*v3519))/v3522);
        let v6273=(((v460*self.scalar_static_f64[352])-(v1644*v3520))/v3522);
        let v6274=(self.scalar_static_f64[175]/v460);
        let v6289=(((v481*(((v481*v3919)-(v624*v3559))/v3913))-(v1655*v3559))/v3913);
        let v6293=(((v481*(((v481*v3920)-(v624*v3560))/v3913))-(v1655*v3560))/v3913);
        let v6312=(if v1653{((v1659*v6269)+(v1648*((v1657*self.scalar_static_f64[351])+(v1644*(if v1653{v6289}else{v2})))))}else{v2});
        let v6313=(if v1653{((v1659*v6273)+(v1648*((v1657*self.scalar_static_f64[352])+(v1644*(if v1653{v6293}else{v2})))))}else{v2});
        let v6314=(if v1653{((v1659*v6274)+(v1648*(self.scalar_static_f64[175]*v1657)))}else{v2});
        let v6350=(v1677*v1677);
        let v6358=(if v1669{((-(v173*((v1675*v6312)+(v1670*((v1673*(v45*v6312))+(v1671*(v40*v6312)))))))/v6350)}else{(if v1664{(v1666*(-v6312))}else{v2})});
        let v6359=(if v1669{((-(v173*((v1675*v6313)+(v1670*((v1673*(v45*v6313))+(v1671*(v40*v6313)))))))/v6350)}else{(if v1664{(v1666*(-v6313))}else{v2})});
        let v6360=(if v1669{((-(v173*((v1675*v6314)+(v1670*((v1673*(v45*v6314))+(v1671*(v40*v6314)))))))/v6350)}else{(if v1664{(v1666*(-v6314))}else{v2})});
        let v6364=(if v1653{(-v6358)}else{v2});
        let v6365=(if v1653{(-v6359)}else{v2});
        let v6366=(if v1653{(-v6360)}else{v2});
        let v6370=(v10*v1685);
        let v6384=(if v1653{(v3986-((v1685*v3508)+(v456*((v3990-v6364)/v6370))))}else{v2});
        let v6385=(if v1653{(v3987-((v1685*v3509)+(v456*((v3991-v6365)/v6370))))}else{v2});
        let v6386=(if v1653{(self.scalar_static_f64[175]-(v456*((self.scalar_static_f64[175]-v6366)/v6370)))}else{v2});
        let v6422=(v1704*v1704);
        let v6430=(if v1696{((-(v173*((v1702*v6384)+(v1697*((v1700*(v45*v6384))+(v1698*(v40*v6384)))))))/v6422)}else{(if v1691{(v1693*(-v6384))}else{v2})});
        let v6431=(if v1696{((-(v173*((v1702*v6385)+(v1697*((v1700*(v45*v6385))+(v1698*(v40*v6385)))))))/v6422)}else{(if v1691{(v1693*(-v6385))}else{v2})});
        let v6432=(if v1696{((-(v173*((v1702*v6386)+(v1697*((v1700*(v45*v6386))+(v1698*(v40*v6386)))))))/v6422)}else{(if v1691{(v1693*(-v6386))}else{v2})});
        let v6443=(if v1653{(-((v1706*v3984)+(v655*v6430)))}else{v2});
        let v6444=(if v1653{(-((v1706*v3985)+(v655*v6431)))}else{v2});
        let v6445=(if v1653{(-(v655*v6432))}else{v2});
        let v6446=(self.scalar_static_f64[351]-v6384);
        let v6447=(self.scalar_static_f64[352]-v6385);
        let v6448=(self.scalar_static_f64[175]-v6386);
        let v6465=(if v1653{((v10*v6446)+((v1712*v3511)+(v457*(-v6430))))}else{v2});
        let v6466=(if v1653{((v10*v6447)+((v1712*v3513)+(v457*(-v6431))))}else{v2});
        let v6467=(if v1653{((v10*v6448)+(v457*(-v6432)))}else{v2});
        let v6468=(v1710*v6446);
        let v6470=(v1710*v6447);
        let v6472=(v1710*v6448);
        let v6487=(if v1653{((v6468+v6468)-((v1718*v3511)+(v457*(v6384+v6430))))}else{v2});
        let v6488=(if v1653{((v6470+v6470)-((v1718*v3513)+(v457*(v6385+v6431))))}else{v2});
        let v6489=(if v1653{((v6472+v6472)-(v457*(v6386+v6432)))}else{v2});
        let v6490=(v1715*v6465);
        let v6492=(v1715*v6466);
        let v6494=(v1715*v6467);
        let v6511=(if v1653{((v6490+v6490)-((v1723*v6487)+(v1721*(v96*v6443))))}else{v6358});
        let v6512=(if v1653{((v6492+v6492)-((v1723*v6488)+(v1721*(v96*v6444))))}else{v6359});
        let v6513=(if v1653{((v6494+v6494)-((v1723*v6489)+(v1721*(v96*v6445))))}else{v6360});
        let v6517=(v10*v1728);
        let v6527=(v1729*v1729);
        let v6547=(if v1735{self.scalar_static_f64[353]}else{v2});
        let v6548=(if v1735{self.scalar_static_f64[354]}else{v2});
        let v6549=(if v1735{self.scalar_static_f64[355]}else{v2});
        let v6562=(if v1735{(((v460*(v162*v6547))-(v1738*v3519))/v3522)}else{v2});
        let v6563=(if v1735{(((v460*(v162*v6548))-(v1738*v3520))/v3522)}else{v2});
        let v6564=(if v1735{((v162*v6549)/v460)}else{v2});
        let v6565=(v1742*v6562);
        let v6567=(v1742*v6563);
        let v6569=(v1742*v6564);
        let v6571=(v10*v1745);
        let v6581=(if v1735{(v45*(v6562-((v6565+v6565)/v6571)))}else{v2});
        let v6582=(if v1735{(v45*(v6563-((v6567+v6567)/v6571)))}else{v2});
        let v6583=(if v1735{(v45*(v6564-((v6569+v6569)/v6571)))}else{v2});
        let v6584=(v6547-v6581);
        let v6585=(v6548-v6582);
        let v6586=(v6549-v6583);
        let v6587=(v1749*v6584);
        let v6589=(v1749*v6585);
        let v6591=(v1749*v6586);
        let v6603=(if v1735{((v6587+v6587)+((v1751*v3511)+(v457*v6581)))}else{v2});
        let v6604=(if v1735{((v6589+v6589)+((v1751*v3513)+(v457*v6582)))}else{v2});
        let v6605=(if v1735{((v6591+v6591)+(v457*v6583))}else{v2});
        let v6611=(if v1735{((v10*v6584)-v3511)}else{v2});
        let v6612=(if v1735{((v10*v6585)-v3513)}else{v2});
        let v6613=(if v1735{(v10*v6586)}else{v2});
        let v6629=(if v1735{(((((v457*v6603)-(v1754*v3511))/v3515)/v1758)-v6581)}else{v2});
        let v6630=(if v1735{(((((v457*v6604)-(v1754*v3513))/v3515)/v1758)-v6582)}else{v2});
        let v6631=(if v1735{(((v6605/v457)/v1758)-v6583)}else{v2});
        let v6635=(if v1735{(v6603+v6611)}else{v2});
        let v6636=(if v1735{(v6604+v6612)}else{v2});
        let v6637=(if v1735{(v6605+v6613)}else{v2});
        let v6638=(v1763*v6635);
        let v6640=(v1763*v6636);
        let v6642=(v1763*v6637);
        let v6671=(if v1735{((v6638+v6638)+((v1767*v6629)+(v1761*(((v1765*v6611)+(v1757*(v45*v6611)))-v6603))))}else{v2});
        let v6672=(if v1735{((v6640+v6640)+((v1767*v6630)+(v1761*(((v1765*v6612)+(v1757*(v45*v6612)))-v6604))))}else{v2});
        let v6673=(if v1735{((v6642+v6642)+((v1767*v6631)+(v1761*(((v1765*v6613)+(v1757*(v45*v6613)))-v6605))))}else{v2});
        let v6713=(v1770*v1770);
        let v6732=(v1757*v6611);
        let v6734=(v1757*v6612);
        let v6736=(v1757*v6613);
        let v6759=(v1781*v1781);
        let v6772=(if v1735{(v6581+(((v1781*((v1771*v6629)+(v1761*((v1763*v6603)+(v1754*v6635)))))-(v1772*(v6671+((v1779*((v1775*v6611)+(v1757*(((v1770*((v1773*v6629)+(v1761*((v1763*v6629)+(v1761*v6635)))))-(v1774*v6671))/v6713))))+(v1776*((v40*(v6732+v6732))-v6603))))))/v6759))}else{v2});
        let v6773=(if v1735{(v6582+(((v1781*((v1771*v6630)+(v1761*((v1763*v6604)+(v1754*v6636)))))-(v1772*(v6672+((v1779*((v1775*v6612)+(v1757*(((v1770*((v1773*v6630)+(v1761*((v1763*v6630)+(v1761*v6636)))))-(v1774*v6672))/v6713))))+(v1776*((v40*(v6734+v6734))-v6604))))))/v6759))}else{v2});
        let v6774=(if v1735{(v6583+(((v1781*((v1771*v6631)+(v1761*((v1763*v6605)+(v1754*v6637)))))-(v1772*(v6673+((v1779*((v1775*v6613)+(v1757*(((v1770*((v1773*v6631)+(v1761*((v1763*v6631)+(v1761*v6637)))))-(v1774*v6673))/v6713))))+(v1776*((v40*(v6736+v6736))-v6605))))))/v6759))}else{v2});
        let v6781=(-v6772);
        let v6782=(-v6773);
        let v6783=(-v6774);
        let v6810=(v1803*v1803);
        let v6848=(if v1807{(v567*((v1813*v6772)+(v1808*((v1811*(v45*v6772))+(v1809*(v40*v6772))))))}else{(if v1795{((-(v642*((v1801*v6781)+(v1796*((v1799*(v45*v6781))+(v1797*(v40*v6781)))))))/v6810)}else{(if v1788{(v1789*v6772)}else{v6430})})});
        let v6849=(if v1807{(v567*((v1813*v6773)+(v1808*((v1811*(v45*v6773))+(v1809*(v40*v6773))))))}else{(if v1795{((-(v642*((v1801*v6782)+(v1796*((v1799*(v45*v6782))+(v1797*(v40*v6782)))))))/v6810)}else{(if v1788{(v1789*v6773)}else{v6431})})});
        let v6850=(if v1807{(v567*((v1813*v6774)+(v1808*((v1811*(v45*v6774))+(v1809*(v40*v6774))))))}else{(if v1795{((-(v642*((v1801*v6783)+(v1796*((v1799*(v45*v6783))+(v1797*(v40*v6783)))))))/v6810)}else{(if v1788{(v1789*v6774)}else{v6432})})});
        let v6864=(v6547-v6772);
        let v6865=(v6548-v6773);
        let v6866=(v6549-v6774);
        let v6880=(if v1735{((v10*v6864)+((v1823*v3511)+(v457*v6848)))}else{v6465});
        let v6881=(if v1735{((v10*v6865)+((v1823*v3513)+(v457*v6849)))}else{v6466});
        let v6882=(if v1735{((v10*v6866)+(v457*v6850))}else{v6467});
        let v6883=(v1821*v6864);
        let v6885=(v1821*v6865);
        let v6887=(v1821*v6866);
        let v6902=(if v1735{((v6883+v6883)+((v1829*v3511)+(v457*(v6772-v6848))))}else{v6487});
        let v6903=(if v1735{((v6885+v6885)+((v1829*v3513)+(v457*(v6773-v6849))))}else{v6488});
        let v6904=(if v1735{((v6887+v6887)+(v457*(v6774-v6850)))}else{v6489});
        let v6905=(v1826*v6880);
        let v6907=(v1826*v6881);
        let v6909=(v1826*v6882);
        let v6932=(v10*v1839);
        let v6942=(v1840*v1840);
        let v6961=(if v1735{(-(v6772+(if v1735{(((v1840*(v10*v6902))-(v1838*(v6880+((if v1735{((v6905+v6905)-((v1834*v6902)+(v1832*(v96*(if v1735{(-((v1817*v3984)+(v655*v6848)))}else{v6443})))))}else{v6511})/v6932))))/v6942)}else{v6364})))}else{(if v1653{(v6384+(if v1653{(((v1729*(v10*v6487))-(v1727*(v6465+(v6511/v6517))))/v6527)}else{v2}))}else{(if (v1647!=0.0){v6269}else{v2})})});
        let v6962=(if v1735{(-(v6773+(if v1735{(((v1840*(v10*v6903))-(v1838*(v6881+((if v1735{((v6907+v6907)-((v1834*v6903)+(v1832*(v96*(if v1735{(-((v1817*v3985)+(v655*v6849)))}else{v6444})))))}else{v6512})/v6932))))/v6942)}else{v6365})))}else{(if v1653{(v6385+(if v1653{(((v1729*(v10*v6488))-(v1727*(v6466+(v6512/v6517))))/v6527)}else{v2}))}else{(if (v1647!=0.0){v6273}else{v2})})});
        let v6963=(if v1735{(-(v6774+(if v1735{(((v1840*(v10*v6904))-(v1838*(v6882+((if v1735{((v6909+v6909)-((v1834*v6904)+(v1832*(v96*(if v1735{(-(v655*v6850))}else{v6445})))))}else{v6513})/v6932))))/v6942)}else{v6366})))}else{(if v1653{(v6386+(if v1653{(((v1729*(v10*v6489))-(v1727*(v6467+(v6513/v6517))))/v6527)}else{v2}))}else{(if (v1647!=0.0){v6274}else{v2})})});
        let v6964=(self.scalar_static_f64[172]*v6961);
        let v6965=(self.scalar_static_f64[172]*v6962);
        let v6966=(self.scalar_static_f64[172]*v6963);
        let v6976=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(self.scalar_static_f64[20]-v6964)))}else{v2});
        let v6977=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(self.scalar_static_f64[150]-v6965)))}else{v2});
        let v6978=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[175]*(self.scalar_static_f64[152]*(-v6966)))}else{v2});
        let v6979=(self.scalar_static_f64[224]*v6976);
        let v6980=(self.scalar_static_f64[224]*v6977);
        let v6981=(self.scalar_static_f64[224]*v6978);
        let v7006=(if v1867{(-v6976)}else{v2});
        let v7007=(if v1867{(-v6977)}else{v2});
        let v7008=(if v1867{(-v6978)}else{v2});
        let v7015=(if v1867{(self.scalar_static_f64[224]*(v162*v7006))}else{v2});
        let v7016=(if v1867{(self.scalar_static_f64[224]*(v162*v7007))}else{v2});
        let v7017=(if v1867{(self.scalar_static_f64[224]*(v162*v7008))}else{v2});
        let v7018=(v1874*v7015);
        let v7020=(v1874*v7016);
        let v7022=(v1874*v7017);
        let v7024=(v10*v1877);
        let v7034=(if v1867{(v45*(v7015-((v7018+v7018)/v7024)))}else{v2});
        let v7035=(if v1867{(v45*(v7016-((v7020+v7020)/v7024)))}else{v2});
        let v7036=(if v1867{(v45*(v7017-((v7022+v7022)/v7024)))}else{v2});
        let v7040=(if v1867{(v7006-v7034)}else{v2});
        let v7041=(if v1867{(v7007-v7035)}else{v2});
        let v7042=(if v1867{(v7008-v7036)}else{v2});
        let v7043=(v1882*v7040);
        let v7045=(v1882*v7041);
        let v7047=(v1882*v7042);
        let v7055=(if v1867{((v7043+v7043)+(self.scalar_static_f64[220]*v7034))}else{v2});
        let v7056=(if v1867{((v7045+v7045)+(self.scalar_static_f64[220]*v7035))}else{v2});
        let v7057=(if v1867{((v7047+v7047)+(self.scalar_static_f64[220]*v7036))}else{v2});
        let v7061=(if v1867{(v10*v7040)}else{v2});
        let v7062=(if v1867{(v10*v7041)}else{v2});
        let v7063=(if v1867{(v10*v7042)}else{v2});
        let v7076=(if v1867{((-v7034)+((self.scalar_static_f64[221]*v7055)/v1892))}else{v2});
        let v7077=(if v1867{((-v7035)+((self.scalar_static_f64[221]*v7056)/v1892))}else{v2});
        let v7078=(if v1867{((-v7036)+((self.scalar_static_f64[221]*v7057)/v1892))}else{v2});
        let v7082=(if v1867{(v7055+v7061)}else{v2});
        let v7083=(if v1867{(v7056+v7062)}else{v2});
        let v7084=(if v1867{(v7057+v7063)}else{v2});
        let v7085=(v1897*v7082);
        let v7087=(v1897*v7083);
        let v7089=(v1897*v7084);
        let v7118=(if v1867{((v7085+v7085)+((v1901*v7076)+(v1895*(((v1899*v7061)+(v1890*(v45*v7061)))-v7055))))}else{v2});
        let v7119=(if v1867{((v7087+v7087)+((v1901*v7077)+(v1895*(((v1899*v7062)+(v1890*(v45*v7062)))-v7056))))}else{v2});
        let v7120=(if v1867{((v7089+v7089)+((v1901*v7078)+(v1895*(((v1899*v7063)+(v1890*(v45*v7063)))-v7057))))}else{v2});
        let v7160=(v1904*v1904);
        let v7179=(v1890*v7061);
        let v7181=(v1890*v7062);
        let v7183=(v1890*v7063);
        let v7206=(v1915*v1915);
        let v7219=(if v1867{(v7034+(((v1915*((v1905*v7076)+(v1895*((v1897*v7055)+(v1887*v7082)))))-(v1906*(v7118+((v1913*((v1909*v7061)+(v1890*(((v1904*((v1907*v7076)+(v1895*((v1897*v7076)+(v1895*v7082)))))-(v1908*v7118))/v7160))))+(v1910*((v40*(v7179+v7179))-v7055))))))/v7206))}else{v2});
        let v7220=(if v1867{(v7035+(((v1915*((v1905*v7077)+(v1895*((v1897*v7056)+(v1887*v7083)))))-(v1906*(v7119+((v1913*((v1909*v7062)+(v1890*(((v1904*((v1907*v7077)+(v1895*((v1897*v7077)+(v1895*v7083)))))-(v1908*v7119))/v7160))))+(v1910*((v40*(v7181+v7181))-v7056))))))/v7206))}else{v2});
        let v7221=(if v1867{(v7036+(((v1915*((v1905*v7078)+(v1895*((v1897*v7057)+(v1887*v7084)))))-(v1906*(v7120+((v1913*((v1909*v7063)+(v1890*(((v1904*((v1907*v7078)+(v1895*((v1897*v7078)+(v1895*v7084)))))-(v1908*v7120))/v7160))))+(v1910*((v40*(v7183+v7183))-v7057))))))/v7206))}else{v2});
        let v7255=(if v1925{(v567*((v1931*v7219)+(v1926*((v1929*(v45*v7219))+(v1927*(v40*v7219))))))}else{(if v1921{(v1922*v7219)}else{v2})});
        let v7256=(if v1925{(v567*((v1931*v7220)+(v1926*((v1929*(v45*v7220))+(v1927*(v40*v7220))))))}else{(if v1921{(v1922*v7220)}else{v2})});
        let v7257=(if v1925{(v567*((v1931*v7221)+(v1926*((v1929*(v45*v7221))+(v1927*(v40*v7221))))))}else{(if v1921{(v1922*v7221)}else{v2})});
        let v7259=(v1935*v1935);
        let v7265=(if v1867{((-v7255)/v7259)}else{v2});
        let v7266=(if v1867{((-v7256)/v7259)}else{v2});
        let v7267=(if v1867{((-v7257)/v7259)}else{v2});
        let v7268=(v1918*v7219);
        let v7270=(v1918*v7220);
        let v7272=(v1918*v7221);
        let v7275=(v1939*v1939);
        let v7287=(if v1867{(v7006-v7219)}else{(if v1867{((-(v7268+v7268))/v7275)}else{v7040})});
        let v7288=(if v1867{(v7007-v7220)}else{(if v1867{((-(v7270+v7270))/v7275)}else{v7041})});
        let v7289=(if v1867{(v7008-v7221)}else{(if v1867{((-(v7272+v7272))/v7275)}else{v7042})});
        let v7293=(if v1867{(self.scalar_static_f64[247]*v7265)}else{v2});
        let v7294=(if v1867{(self.scalar_static_f64[247]*v7266)}else{v2});
        let v7295=(if v1867{(self.scalar_static_f64[247]*v7267)}else{v2});
        let v7308=(if v1867{((v10*v7287)+(self.scalar_static_f64[220]*(v7255-v7293)))}else{v2});
        let v7309=(if v1867{((v10*v7288)+(self.scalar_static_f64[220]*(v7256-v7294)))}else{v2});
        let v7310=(if v1867{((v10*v7289)+(self.scalar_static_f64[220]*(v7257-v7295)))}else{v2});
        let v7311=(v1943*v7287);
        let v7313=(v1943*v7288);
        let v7315=(v1943*v7289);
        let v7335=(if v1867{((v7311+v7311)-(self.scalar_static_f64[220]*((v7293+(v7255-v7219))+(self.scalar_static_f64[247]*v7219))))}else{v2});
        let v7336=(if v1867{((v7313+v7313)-(self.scalar_static_f64[220]*((v7294+(v7256-v7220))+(self.scalar_static_f64[247]*v7220))))}else{v2});
        let v7337=(if v1867{((v7315+v7315)-(self.scalar_static_f64[220]*((v7295+(v7257-v7221))+(self.scalar_static_f64[247]*v7221))))}else{v2});
        let v7347=(if v1867{(-(self.scalar_static_f64[220]*(v7255+v7293)))}else{v7287});
        let v7348=(if v1867{(-(self.scalar_static_f64[220]*(v7256+v7294)))}else{v7288});
        let v7349=(if v1867{(-(self.scalar_static_f64[220]*(v7257+v7295)))}else{v7289});
        let v7350=(v1952*v7308);
        let v7352=(v1952*v7309);
        let v7354=(v1952*v7310);
        let v7356=(v10*v7335);
        let v7357=(v10*v7336);
        let v7358=(v10*v7337);
        let v7371=(if v1867{((v7350+v7350)-((v1968*v7347)+(v1966*v7356)))}else{v7347});
        let v7372=(if v1867{((v7352+v7352)-((v1968*v7348)+(v1966*v7357)))}else{v7348});
        let v7373=(if v1867{((v7354+v7354)-((v1968*v7349)+(v1966*v7358)))}else{v7349});
        let v7377=(v10*v1973);
        let v7387=(v1974*v1974);
        let v7415=(if v1979{((v1986*v6979)+(v1856*(v1984*v6976)))}else{v2});
        let v7416=(if v1979{((v1986*v6980)+(v1856*(v1984*v6977)))}else{v2});
        let v7417=(if v1979{((v1986*v6981)+(v1856*(v1984*v6978)))}else{v2});
        let v7453=(v2004*v2004);
        let v7461=(if v1996{((-(v642*((v2002*v7415)+(v1997*((v2000*(v45*v7415))+(v1998*(v40*v7415)))))))/v7453)}else{(if v1992{(v1993*(-v7415))}else{v7371})});
        let v7462=(if v1996{((-(v642*((v2002*v7416)+(v1997*((v2000*(v45*v7416))+(v1998*(v40*v7416)))))))/v7453)}else{(if v1992{(v1993*(-v7416))}else{v7372})});
        let v7463=(if v1996{((-(v642*((v2002*v7417)+(v1997*((v2000*(v45*v7417))+(v1998*(v40*v7417)))))))/v7453)}else{(if v1992{(v1993*(-v7417))}else{v7373})});
        let v7473=(v10*v2012);
        let v7483=(if v1979{(v6976-(self.scalar_static_f64[219]*((v6976-(if v1979{(-v7461)}else{v2}))/v7473)))}else{v2});
        let v7484=(if v1979{(v6977-(self.scalar_static_f64[219]*((v6977-(if v1979{(-v7462)}else{v2}))/v7473)))}else{v2});
        let v7485=(if v1979{(v6978-(self.scalar_static_f64[219]*((v6978-(if v1979{(-v7463)}else{v2}))/v7473)))}else{v2});
        let v7486=(-v7483);
        let v7487=(-v7484);
        let v7488=(-v7485);
        let v7489=(v2017*v7486);
        let v7491=(v2017*v7487);
        let v7493=(v2017*v7488);
        let v7495=(v10*v2021);
        let v7508=(v2025*v7483);
        let v7510=(v2025*v7484);
        let v7512=(v2025*v7485);
        let v7514=(v10*v2029);
        let v7523=(v2030*v2030);
        let v7546=(if v1979{(if v2018{(-(v45*(v7486+((v7489+v7489)/v7495))))}else{(if v2026{(-((-(v678*(v7483+((v7508+v7508)/v7514))))/v7523))}else{(-(v45*v7486))})})}else{v7034});
        let v7547=(if v1979{(if v2018{(-(v45*(v7487+((v7491+v7491)/v7495))))}else{(if v2026{(-((-(v678*(v7484+((v7510+v7510)/v7514))))/v7523))}else{(-(v45*v7487))})})}else{v7035});
        let v7548=(if v1979{(if v2018{(-(v45*(v7488+((v7493+v7493)/v7495))))}else{(if v2026{(-((-(v678*(v7485+((v7512+v7512)/v7514))))/v7523))}else{(-(v45*v7488))})})}else{v7036});
        let v7552=(if v1979{(v6976-v7546)}else{v7461});
        let v7553=(if v1979{(v6977-v7547)}else{v7462});
        let v7554=(if v1979{(v6978-v7548)}else{v7463});
        let v7555=(-v7546);
        let v7556=(-v7547);
        let v7557=(-v7548);
        let v7561=(if v1979{(v2048*v7555)}else{v7293});
        let v7562=(if v1979{(v2048*v7556)}else{v7294});
        let v7563=(if v1979{(v2048*v7557)}else{v7295});
        let v7564=(v2046*v7552);
        let v7566=(v2046*v7553);
        let v7568=(v2046*v7554);
        let v7588=(if v1979{(if v2058{v2}else{((v7564+v7564)-(self.scalar_static_f64[220]*((v7546+v7561)-(self.scalar_static_f64[247]*v7546))))})}else{v7055});
        let v7589=(if v1979{(if v2058{v2}else{((v7566+v7566)-(self.scalar_static_f64[220]*((v7547+v7562)-(self.scalar_static_f64[247]*v7547))))})}else{v7056});
        let v7590=(if v1979{(if v2058{v2}else{((v7568+v7568)-(self.scalar_static_f64[220]*((v7548+v7563)-(self.scalar_static_f64[247]*v7548))))})}else{v7057});
        let v7612=(if v1979{((v10*v7552)+(self.scalar_static_f64[220]*(-v7561)))}else{v7061});
        let v7613=(if v1979{((v10*v7553)+(self.scalar_static_f64[220]*(-v7562)))}else{v7062});
        let v7614=(if v1979{((v10*v7554)+(self.scalar_static_f64[220]*(-v7563)))}else{v7063});
        let v7624=(if v1979{(v7555+((v7588/self.scalar_static_f64[220])/v2071))}else{v7076});
        let v7625=(if v1979{(v7556+((v7589/self.scalar_static_f64[220])/v2071))}else{v7077});
        let v7626=(if v1979{(v7557+((v7590/self.scalar_static_f64[220])/v2071))}else{v7078});
        let v7630=(if v1979{(v7588+v7612)}else{v2});
        let v7631=(if v1979{(v7589+v7613)}else{v2});
        let v7632=(if v1979{(v7590+v7614)}else{v2});
        let v7636=(v2076*v7630);
        let v7638=(v2076*v7631);
        let v7640=(v2076*v7632);
        let v7656=((v2063*v7588)+(v2060*(if v1979{(-(self.scalar_static_f64[329]*v7561))}else{v2})));
        let v7659=((v2063*v7589)+(v2060*(if v1979{(-(self.scalar_static_f64[329]*v7562))}else{v2})));
        let v7662=((v2063*v7590)+(v2060*(if v1979{(-(self.scalar_static_f64[329]*v7563))}else{v2})));
        let v7678=(if v2083{((v7636+v7636)+((v2088*v7624)+(v2074*(((v2085*v7612)+(v2069*(v45*v7612)))-v7656))))}else{v2});
        let v7679=(if v2083{((v7638+v7638)+((v2088*v7625)+(v2074*(((v2085*v7613)+(v2069*(v45*v7613)))-v7659))))}else{v2});
        let v7680=(if v2083{((v7640+v7640)+((v2088*v7626)+(v2074*(((v2085*v7614)+(v2069*(v45*v7614)))-v7662))))}else{v2});
        let v7720=(v2091*v2091);
        let v7739=(v2069*v7612);
        let v7741=(v2069*v7613);
        let v7743=(v2069*v7614);
        let v7766=(v2102*v2102);
        let v7779=(if v2083{(v7546+(((v2102*((v2092*v7624)+(v2074*((v2076*v7588)+(v2060*v7630)))))-(v2093*(v7678+((v2100*((v2096*v7612)+(v2069*(((v2091*((v2094*v7624)+(v2074*((v2076*v7624)+(v2074*v7630)))))-(v2095*v7678))/v7720))))+(v2097*((v40*(v7739+v7739))-v7656))))))/v7766))}else{(if v2080{v7546}else{v2})});
        let v7780=(if v2083{(v7547+(((v2102*((v2092*v7625)+(v2074*((v2076*v7589)+(v2060*v7631)))))-(v2093*(v7679+((v2100*((v2096*v7613)+(v2069*(((v2091*((v2094*v7625)+(v2074*((v2076*v7625)+(v2074*v7631)))))-(v2095*v7679))/v7720))))+(v2097*((v40*(v7741+v7741))-v7659))))))/v7766))}else{(if v2080{v7547}else{v2})});
        let v7781=(if v2083{(v7548+(((v2102*((v2092*v7626)+(v2074*((v2076*v7590)+(v2060*v7632)))))-(v2093*(v7680+((v2100*((v2096*v7614)+(v2069*(((v2091*((v2094*v7626)+(v2074*((v2076*v7626)+(v2074*v7632)))))-(v2095*v7680))/v7720))))+(v2097*((v40*(v7743+v7743))-v7662))))))/v7766))}else{(if v2080{v7548}else{v2})});
        let v7785=(if v2108{(v2109*v7779)}else{v7255});
        let v7786=(if v2108{(v2109*v7780)}else{v7256});
        let v7787=(if v2108{(v2109*v7781)}else{v7257});
        let v7789=(v2110*v2110);
        let v7807=(if v2119{(v2121*v7779)}else{(if v2108{(self.scalar_static_f64[247]*v7785)}else{v7785})});
        let v7808=(if v2119{(v2121*v7780)}else{(if v2108{(self.scalar_static_f64[247]*v7786)}else{v7786})});
        let v7809=(if v2119{(v2121*v7781)}else{(if v2108{(self.scalar_static_f64[247]*v7787)}else{v7787})});
        let v7812=(v2122*v2122);
        let v7823=(-v7779);
        let v7824=(-v7780);
        let v7825=(-v7781);
        let v7852=(v2135*v2135);
        let v7860=(if v2126{((-(v642*((v2133*v7823)+(v2128*((v2131*(v45*v7823))+(v2129*(v40*v7823)))))))/v7852)}else{v7807});
        let v7861=(if v2126{((-(v642*((v2133*v7824)+(v2128*((v2131*(v45*v7824))+(v2129*(v40*v7824)))))))/v7852)}else{v7808});
        let v7862=(if v2126{((-(v642*((v2133*v7825)+(v2128*((v2131*(v45*v7825))+(v2129*(v40*v7825)))))))/v7852)}else{v7809});
        let v7889=(v2145*v2145);
        let v7897=(if v2126{((-(v642*((v2143*v7779)+(v2138*((v2141*(v45*v7779))+(v2139*(v40*v7779)))))))/v7889)}else{(if v2119{((-(self.scalar_static_f64[247]*v7807))/v7812)}else{(if v2108{((-v7785)/v7789)}else{v7265})})});
        let v7898=(if v2126{((-(v642*((v2143*v7780)+(v2138*((v2141*(v45*v7780))+(v2139*(v40*v7780)))))))/v7889)}else{(if v2119{((-(self.scalar_static_f64[247]*v7808))/v7812)}else{(if v2108{((-v7786)/v7789)}else{v7266})})});
        let v7899=(if v2126{((-(v642*((v2143*v7781)+(v2138*((v2141*(v45*v7781))+(v2139*(v40*v7781)))))))/v7889)}else{(if v2119{((-(self.scalar_static_f64[247]*v7809))/v7812)}else{(if v2108{((-v7787)/v7789)}else{v7267})})});
        let v7900=(v2105*v7779);
        let v7902=(v2105*v7780);
        let v7904=(v2105*v7781);
        let v7907=(v2149*v2149);
        let v7919=(if v1979{(v6976-v7779)}else{(if v1979{((-(v7900+v7900))/v7907)}else{v7552})});
        let v7920=(if v1979{(v6977-v7780)}else{(if v1979{((-(v7902+v7902))/v7907)}else{v7553})});
        let v7921=(if v1979{(v6978-v7781)}else{(if v1979{((-(v7904+v7904))/v7907)}else{v7554})});
        let v7937=(if v1979{((v10*v7919)+(self.scalar_static_f64[220]*(v7860+(-v7897))))}else{v7308});
        let v7938=(if v1979{((v10*v7920)+(self.scalar_static_f64[220]*(v7861+(-v7898))))}else{v7309});
        let v7939=(if v1979{((v10*v7921)+(self.scalar_static_f64[220]*(v7862+(-v7899))))}else{v7310});
        let v7940=(v2153*v7919);
        let v7942=(v2153*v7920);
        let v7944=(v2153*v7921);
        let v7976=(if v1979{(-(self.scalar_static_f64[220]*(v7860+v7897)))}else{v7919});
        let v7977=(if v1979{(-(self.scalar_static_f64[220]*(v7861+v7898)))}else{v7920});
        let v7978=(if v1979{(-(self.scalar_static_f64[220]*(v7862+v7899)))}else{v7921});
        let v7979=(v2160*v7937);
        let v7981=(v2160*v7938);
        let v7983=(v2160*v7939);
        let v7985=(v10*(if v1979{((v7940+v7940)-(self.scalar_static_f64[220]*((v7860+(v7779+v7897))-(self.scalar_static_f64[247]*v7779))))}else{v7335}));
        let v7986=(v10*(if v1979{((v7942+v7942)-(self.scalar_static_f64[220]*((v7861+(v7780+v7898))-(self.scalar_static_f64[247]*v7780))))}else{v7336}));
        let v7987=(v10*(if v1979{((v7944+v7944)-(self.scalar_static_f64[220]*((v7862+(v7781+v7899))-(self.scalar_static_f64[247]*v7781))))}else{v7337}));
        let v8003=(v10*v2180);
        let v8013=(v2181*v2181);
        let v8035=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v1979{(v7779+(((v2181*v7985)-(v2176*(v7937+((if v1979{((v7979+v7979)-((v2176*v7976)+(v2174*v7985)))}else{v7976})/v8003))))/v8013))}else{(if v1867{((-v7219)-(((v1974*v7356)-(v1968*(v7308+(v7371/v7377))))/v7387))}else{(if v1854{((v1860*v6979)+(v1856*(v1855*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v6976)))))}else{v2})})})))}else{v2});
        let v8036=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v1979{(v7780+(((v2181*v7986)-(v2176*(v7938+((if v1979{((v7981+v7981)-((v2176*v7977)+(v2174*v7986)))}else{v7977})/v8003))))/v8013))}else{(if v1867{((-v7220)-(((v1974*v7357)-(v1968*(v7309+(v7372/v7377))))/v7387))}else{(if v1854{((v1860*v6980)+(v1856*(v1855*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v6977)))))}else{v2})})})))}else{v2});
        let v8037=(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[152]*(if v1979{(v7781+(((v2181*v7987)-(v2176*(v7939+((if v1979{((v7983+v7983)-((v2176*v7978)+(v2174*v7987)))}else{v7978})/v8003))))/v8013))}else{(if v1867{((-v7221)-(((v1974*v7358)-(v1968*(v7310+(v7373/v7377))))/v7387))}else{(if v1854{((v1860*v6981)+(v1856*(v1855*(self.scalar_static_f64[219]*(self.scalar_static_f64[323]*v6978)))))}else{v2})})})))}else{v2});
        let v8044=(if (self.scalar_static_f64[149]!=0.0){((self.scalar_static_f64[20]-v8035)/self.scalar_static_f64[172])}else{self.scalar_static_f64[351]});
        let v8045=(if (self.scalar_static_f64[149]!=0.0){((self.scalar_static_f64[150]-v8036)/self.scalar_static_f64[172])}else{self.scalar_static_f64[352]});
        let v8046=(if (self.scalar_static_f64[149]!=0.0){((v1-v8037)/self.scalar_static_f64[172])}else{self.scalar_static_f64[175]});
        let v8050=(((v460*v8044)-(v2190*v3519))/v3522);
        let v8054=(((v460*v8045)-(v2190*v3520))/v3522);
        let v8055=(v8046/v460);
        let v8077=(if v2201{((v2204*v8050)+(v2195*((v2202*v8044)+(v2190*(if v2201{v6289}else{v2})))))}else{v2});
        let v8078=(if v2201{((v2204*v8054)+(v2195*((v2202*v8045)+(v2190*(if v2201{v6293}else{v2})))))}else{v2});
        let v8079=(if v2201{((v2204*v8055)+(v2195*(v2202*v8046)))}else{v2});
        let v8115=(v2222*v2222);
        let v8123=(if v2214{((-(v173*((v2220*v8077)+(v2215*((v2218*(v45*v8077))+(v2216*(v40*v8077)))))))/v8115)}else{(if v2209{(v2211*(-v8077))}else{v2})});
        let v8124=(if v2214{((-(v173*((v2220*v8078)+(v2215*((v2218*(v45*v8078))+(v2216*(v40*v8078)))))))/v8115)}else{(if v2209{(v2211*(-v8078))}else{v2})});
        let v8125=(if v2214{((-(v173*((v2220*v8079)+(v2215*((v2218*(v45*v8079))+(v2216*(v40*v8079)))))))/v8115)}else{(if v2209{(v2211*(-v8079))}else{v2})});
        let v8129=(if v2201{(-v8123)}else{v2});
        let v8130=(if v2201{(-v8124)}else{v2});
        let v8131=(if v2201{(-v8125)}else{v2});
        let v8139=(v10*v2230);
        let v8153=(if v2201{((v3984+v8044)-((v2230*v3508)+(v456*(((v3988+v8044)-v8129)/v8139))))}else{v2});
        let v8154=(if v2201{((v3985+v8045)-((v2230*v3509)+(v456*(((v3989+v8045)-v8130)/v8139))))}else{v2});
        let v8155=(if v2201{(v8046-(v456*((v8046-v8131)/v8139)))}else{v2});
        let v8191=(v2249*v2249);
        let v8199=(if v2241{((-(v173*((v2247*v8153)+(v2242*((v2245*(v45*v8153))+(v2243*(v40*v8153)))))))/v8191)}else{(if v2236{(v2238*(-v8153))}else{v2})});
        let v8200=(if v2241{((-(v173*((v2247*v8154)+(v2242*((v2245*(v45*v8154))+(v2243*(v40*v8154)))))))/v8191)}else{(if v2236{(v2238*(-v8154))}else{v2})});
        let v8201=(if v2241{((-(v173*((v2247*v8155)+(v2242*((v2245*(v45*v8155))+(v2243*(v40*v8155)))))))/v8191)}else{(if v2236{(v2238*(-v8155))}else{v2})});
        let v8212=(if v2201{(-((v2251*v3984)+(v655*v8199)))}else{v2});
        let v8213=(if v2201{(-((v2251*v3985)+(v655*v8200)))}else{v2});
        let v8214=(if v2201{(-(v655*v8201))}else{v2});
        let v8215=(v8044-v8153);
        let v8216=(v8045-v8154);
        let v8217=(v8046-v8155);
        let v8234=(if v2201{((v10*v8215)+((v2257*v3511)+(v457*(-v8199))))}else{v2});
        let v8235=(if v2201{((v10*v8216)+((v2257*v3513)+(v457*(-v8200))))}else{v2});
        let v8236=(if v2201{((v10*v8217)+(v457*(-v8201)))}else{v2});
        let v8237=(v2255*v8215);
        let v8239=(v2255*v8216);
        let v8241=(v2255*v8217);
        let v8256=(if v2201{((v8237+v8237)-((v2263*v3511)+(v457*(v8153+v8199))))}else{v2});
        let v8257=(if v2201{((v8239+v8239)-((v2263*v3513)+(v457*(v8154+v8200))))}else{v2});
        let v8258=(if v2201{((v8241+v8241)-(v457*(v8155+v8201)))}else{v2});
        let v8259=(v2260*v8234);
        let v8261=(v2260*v8235);
        let v8263=(v2260*v8236);
        let v8280=(if v2201{((v8259+v8259)-((v2268*v8256)+(v2266*(v96*v8212))))}else{v8123});
        let v8281=(if v2201{((v8261+v8261)-((v2268*v8257)+(v2266*(v96*v8213))))}else{v8124});
        let v8282=(if v2201{((v8263+v8263)-((v2268*v8258)+(v2266*(v96*v8214))))}else{v8125});
        let v8286=(v10*v2273);
        let v8296=(v2274*v2274);
        let v8318=(if v2280{(-v8044)}else{v2});
        let v8319=(if v2280{(-v8045)}else{v2});
        let v8320=(if v2280{(-v8046)}else{v2});
        let v8333=(if v2280{(((v460*(v162*v8318))-(v2283*v3519))/v3522)}else{v2});
        let v8334=(if v2280{(((v460*(v162*v8319))-(v2283*v3520))/v3522)}else{v2});
        let v8335=(if v2280{((v162*v8320)/v460)}else{v2});
        let v8336=(v2287*v8333);
        let v8338=(v2287*v8334);
        let v8340=(v2287*v8335);
        let v8342=(v10*v2290);
        let v8352=(if v2280{(v45*(v8333-((v8336+v8336)/v8342)))}else{v2});
        let v8353=(if v2280{(v45*(v8334-((v8338+v8338)/v8342)))}else{v2});
        let v8354=(if v2280{(v45*(v8335-((v8340+v8340)/v8342)))}else{v2});
        let v8355=(v8318-v8352);
        let v8356=(v8319-v8353);
        let v8357=(v8320-v8354);
        let v8358=(v2294*v8355);
        let v8360=(v2294*v8356);
        let v8362=(v2294*v8357);
        let v8374=(if v2280{((v8358+v8358)+((v2296*v3511)+(v457*v8352)))}else{v2});
        let v8375=(if v2280{((v8360+v8360)+((v2296*v3513)+(v457*v8353)))}else{v2});
        let v8376=(if v2280{((v8362+v8362)+(v457*v8354))}else{v2});
        let v8382=(if v2280{((v10*v8355)-v3511)}else{v2});
        let v8383=(if v2280{((v10*v8356)-v3513)}else{v2});
        let v8384=(if v2280{(v10*v8357)}else{v2});
        let v8400=(if v2280{(((((v457*v8374)-(v2299*v3511))/v3515)/v2303)-v8352)}else{v2});
        let v8401=(if v2280{(((((v457*v8375)-(v2299*v3513))/v3515)/v2303)-v8353)}else{v2});
        let v8402=(if v2280{(((v8376/v457)/v2303)-v8354)}else{v2});
        let v8406=(if v2280{(v8374+v8382)}else{v2});
        let v8407=(if v2280{(v8375+v8383)}else{v2});
        let v8408=(if v2280{(v8376+v8384)}else{v2});
        let v8409=(v2308*v8406);
        let v8411=(v2308*v8407);
        let v8413=(v2308*v8408);
        let v8442=(if v2280{((v8409+v8409)+((v2312*v8400)+(v2306*(((v2310*v8382)+(v2302*(v45*v8382)))-v8374))))}else{v2});
        let v8443=(if v2280{((v8411+v8411)+((v2312*v8401)+(v2306*(((v2310*v8383)+(v2302*(v45*v8383)))-v8375))))}else{v2});
        let v8444=(if v2280{((v8413+v8413)+((v2312*v8402)+(v2306*(((v2310*v8384)+(v2302*(v45*v8384)))-v8376))))}else{v2});
        let v8484=(v2315*v2315);
        let v8503=(v2302*v8382);
        let v8505=(v2302*v8383);
        let v8507=(v2302*v8384);
        let v8530=(v2326*v2326);
        let v8543=(if v2280{(v8352+(((v2326*((v2316*v8400)+(v2306*((v2308*v8374)+(v2299*v8406)))))-(v2317*(v8442+((v2324*((v2320*v8382)+(v2302*(((v2315*((v2318*v8400)+(v2306*((v2308*v8400)+(v2306*v8406)))))-(v2319*v8442))/v8484))))+(v2321*((v40*(v8503+v8503))-v8374))))))/v8530))}else{v2});
        let v8544=(if v2280{(v8353+(((v2326*((v2316*v8401)+(v2306*((v2308*v8375)+(v2299*v8407)))))-(v2317*(v8443+((v2324*((v2320*v8383)+(v2302*(((v2315*((v2318*v8401)+(v2306*((v2308*v8401)+(v2306*v8407)))))-(v2319*v8443))/v8484))))+(v2321*((v40*(v8505+v8505))-v8375))))))/v8530))}else{v2});
        let v8545=(if v2280{(v8354+(((v2326*((v2316*v8402)+(v2306*((v2308*v8376)+(v2299*v8408)))))-(v2317*(v8444+((v2324*((v2320*v8384)+(v2302*(((v2315*((v2318*v8402)+(v2306*((v2308*v8402)+(v2306*v8408)))))-(v2319*v8444))/v8484))))+(v2321*((v40*(v8507+v8507))-v8376))))))/v8530))}else{v2});
        let v8552=(-v8543);
        let v8553=(-v8544);
        let v8554=(-v8545);
        let v8581=(v2348*v2348);
        let v8619=(if v2352{(v567*((v2358*v8543)+(v2353*((v2356*(v45*v8543))+(v2354*(v40*v8543))))))}else{(if v2340{((-(v642*((v2346*v8552)+(v2341*((v2344*(v45*v8552))+(v2342*(v40*v8552)))))))/v8581)}else{(if v2333{(v2334*v8543)}else{v8199})})});
        let v8620=(if v2352{(v567*((v2358*v8544)+(v2353*((v2356*(v45*v8544))+(v2354*(v40*v8544))))))}else{(if v2340{((-(v642*((v2346*v8553)+(v2341*((v2344*(v45*v8553))+(v2342*(v40*v8553)))))))/v8581)}else{(if v2333{(v2334*v8544)}else{v8200})})});
        let v8621=(if v2352{(v567*((v2358*v8545)+(v2353*((v2356*(v45*v8545))+(v2354*(v40*v8545))))))}else{(if v2340{((-(v642*((v2346*v8554)+(v2341*((v2344*(v45*v8554))+(v2342*(v40*v8554)))))))/v8581)}else{(if v2333{(v2334*v8545)}else{v8201})})});
        let v8635=(v8318-v8543);
        let v8636=(v8319-v8544);
        let v8637=(v8320-v8545);
        let v8651=(if v2280{((v10*v8635)+((v2368*v3511)+(v457*v8619)))}else{v8234});
        let v8652=(if v2280{((v10*v8636)+((v2368*v3513)+(v457*v8620)))}else{v8235});
        let v8653=(if v2280{((v10*v8637)+(v457*v8621))}else{v8236});
        let v8654=(v2366*v8635);
        let v8656=(v2366*v8636);
        let v8658=(v2366*v8637);
        let v8673=(if v2280{((v8654+v8654)+((v2374*v3511)+(v457*(v8543-v8619))))}else{v8256});
        let v8674=(if v2280{((v8656+v8656)+((v2374*v3513)+(v457*(v8544-v8620))))}else{v8257});
        let v8675=(if v2280{((v8658+v8658)+(v457*(v8545-v8621)))}else{v8258});
        let v8676=(v2371*v8651);
        let v8678=(v2371*v8652);
        let v8680=(v2371*v8653);
        let v8703=(v10*v2384);
        let v8713=(v2385*v2385);
        let v8732=(if v2280{(-(v8543+(if v2280{(((v2385*(v10*v8673))-(v2383*(v8651+((if v2280{((v8676+v8676)-((v2379*v8673)+(v2377*(v96*(if v2280{(-((v2362*v3984)+(v655*v8619)))}else{v8212})))))}else{v8280})/v8703))))/v8713)}else{v8129})))}else{(if v2201{(v8153+(if v2201{(((v2274*(v10*v8256))-(v2272*(v8234+(v8280/v8286))))/v8296)}else{v2}))}else{(if v2194{v8050}else{v6961})})});
        let v8733=(if v2280{(-(v8544+(if v2280{(((v2385*(v10*v8674))-(v2383*(v8652+((if v2280{((v8678+v8678)-((v2379*v8674)+(v2377*(v96*(if v2280{(-((v2362*v3985)+(v655*v8620)))}else{v8213})))))}else{v8281})/v8703))))/v8713)}else{v8130})))}else{(if v2201{(v8154+(if v2201{(((v2274*(v10*v8257))-(v2272*(v8235+(v8281/v8286))))/v8296)}else{v2}))}else{(if v2194{v8054}else{v6962})})});
        let v8734=(if v2280{(-(v8545+(if v2280{(((v2385*(v10*v8675))-(v2383*(v8653+((if v2280{((v8680+v8680)-((v2379*v8675)+(v2377*(v96*(if v2280{(-(v655*v8621))}else{v8214})))))}else{v8282})/v8703))))/v8713)}else{v8131})))}else{(if v2201{(v8155+(if v2201{(((v2274*(v10*v8258))-(v2272*(v8236+(v8282/v8286))))/v8296)}else{v2}))}else{(if v2194{v8055}else{v6963})})});
        let v8747=(if (v2395!=0.0){(v2396*v8732)}else{v2});
        let v8748=(if (v2395!=0.0){(v2396*v8733)}else{v2});
        let v8749=(if (v2395!=0.0){(v2396*v8734)}else{v2});
        let v8751=(v2397*v2397);
        let v8762=(-v8734);
        let v8779=(v45*v8732);
        let v8780=(v45*v8733);
        let v8781=(v45*v8734);
        let v8782=(v40*v8732);
        let v8783=(v40*v8733);
        let v8784=(v40*v8734);
        let v8805=(v2418*v2418);
        let v8816=(v8732+(if v2410{((-(v642*((v2416*v8732)+(v2411*((v2414*v8779)+(v2412*v8782))))))/v8805)}else{(if v2403{((v2406*v3557)+(v479*(if v2403{(v2405*(v3526-v8732))}else{v8747})))}else{(if (v2395!=0.0){((-v8747)/v8751)}else{v6140})})}));
        let v8817=(v8733+(if v2410{((-(v642*((v2416*v8733)+(v2411*((v2414*v8780)+(v2412*v8783))))))/v8805)}else{(if v2403{((v2406*v3558)+(v479*(if v2403{(v2405*(v3527-v8733))}else{v8748})))}else{(if (v2395!=0.0){((-v8748)/v8751)}else{v6141})})}));
        let v8818=(v8734+(if v2410{((-(v642*((v2416*v8734)+(v2411*((v2414*v8781)+(v2412*v8784))))))/v8805)}else{(if v2403{(v479*(if v2403{(v2405*v8762)}else{v8749}))}else{(if (v2395!=0.0){((-v8749)/v8751)}else{v2})})}));
        let v8819=(if (v2422!=0.0){v8816}else{v6219});
        let v8820=(if (v2422!=0.0){v8817}else{v6220});
        let v8821=(if (v2422!=0.0){v8818}else{v2});
        let v8822=(v10*v2426);
        let v8850=(if v2433{(-((v2436*v8782)+(v2434*(-(v657*v8732)))))}else{v6205});
        let v8851=(if v2433{(-((v2436*v8783)+(v2434*(-(v657*v8733)))))}else{v6206});
        let v8852=(if v2433{(-((v2436*v8784)+(v2434*(-(v657*v8734)))))}else{v2});
        let v8877=(v10*v2445);
        let v8896=(v10*v2453);
        let v8908=((v2455*v3508)+(v456*(self.scalar_static_f64[172]*(if v2449{((if v2449{v8816}else{(if v2433{((v2441*v8850)+(v2439*((v2440*v8732)+(v2390*v8779))))}else{v8819})})/v8896)}else{(if v2433{((v2445*(v150*v8732))+(v2444*(v8850/v8877)))}else{(if (v2422!=0.0){(-(v8819/v8822))}else{v6224})})}))));
        let v8911=((v2455*v3509)+(v456*(self.scalar_static_f64[172]*(if v2449{((if v2449{v8817}else{(if v2433{((v2441*v8851)+(v2439*((v2440*v8733)+(v2390*v8780))))}else{v8820})})/v8896)}else{(if v2433{((v2445*(v150*v8733))+(v2444*(v8851/v8877)))}else{(if (v2422!=0.0){(-(v8820/v8822))}else{v6225})})}))));
        let v8912=(v456*(self.scalar_static_f64[172]*(if v2449{((if v2449{v8818}else{(if v2433{((v2441*v8852)+(v2439*((v2440*v8734)+(v2390*v8781))))}else{v8821})})/v8896)}else{(if v2433{((v2445*(v150*v8734))+(v2444*(v8852/v8877)))}else{(if (v2422!=0.0){(-(v8821/v8822))}else{v2})})})));
        let v8931=(self.scalar_static_f64[172]*(self.scalar_static_f64[172]*(self.scalar_static_f64[333]*(self.scalar_static_f64[170]*(self.scalar_static_f64[155]*(self.scalar_static_f64[155]*((v2459*v3446)+(v2458*(v2457*v3446)))))))));
        let v8932=(self.scalar_static_f64[172]*(self.scalar_static_f64[172]*(self.scalar_static_f64[333]*(self.scalar_static_f64[170]*(self.scalar_static_f64[155]*(self.scalar_static_f64[155]*((v2459*v3447)+(v2458*(v2457*v3447)))))))));
        let v8933=(-v8908);
        let v8934=(-v8911);
        let v8935=(-v8912);
        let v8936=(v8908-v8933);
        let v8937=(v8911-v8934);
        let v8938=(v8912-v8935);
        let v8939=(v2472*v8936);
        let v8941=(v2472*v8937);
        let v8943=(v2472*v8938);
        let v8947=(v10*v2476);
        let v8960=(v8933-v8908);
        let v8961=(v8934-v8911);
        let v8962=(v8935-v8912);
        let v8963=(v45*v8931);
        let v8964=(v45*v8932);
        let v8965=(v2480*v8960);
        let v8967=(v2480*v8961);
        let v8969=(v2480*v8962);
        let v8973=(v10*v2485);
        let v8983=(v2486*v2486);
        let v8995=(v10*v2490);
        let v8996=(v8931/v8995);
        let v8997=(v8932/v8995);
        let v9012=-2.0;
        let v9013=(v2497*v9012);
        let v9015=(v10*v2501);
        let v9024=(v10*v2505);
        let v9026=(v10*v2509);
        let v9034=(v2510*v2510);
        let v9058=(v2519*((if v2473{(v8933+(v45*(v8936+((v8931+(v8939+v8939))/v8947))))}else{(if v2481{(v8933+(((v2486*v8963)-(v2482*(v8960+((v8931+(v8965+v8965))/v8973))))/v8983))}else{(v8933+(v45*(v8936+v8996)))})})+(self.scalar_static_f64[29]*(if v2498{(v45*(v8931/v9015))}else{(if v2506{(((v2510*v8963)-(v2482*(v8931/v9026)))/v9034)}else{(v45*v8996)})}))));
        let v9060=(v2519*((if v2473{(v8934+(v45*(v8937+((v8932+(v8941+v8941))/v8947))))}else{(if v2481{(v8934+(((v2486*v8964)-(v2482*(v8961+((v8932+(v8967+v8967))/v8973))))/v8983))}else{(v8934+(v45*(v8937+v8997)))})})+(self.scalar_static_f64[29]*(if v2498{(v45*(v8932/v9015))}else{(if v2506{(((v2510*v8964)-(v2482*(v8932/v9026)))/v9034)}else{(v45*v8997)})}))));
        let v9062=(v2519*((if v2473{(v8935+(v45*(v8938+((v8943+v8943)/v8947))))}else{(if v2481{(v8935+((-(v2482*(v8962+((v8969+v8969)/v8973))))/v8983))}else{(v8935+(v45*v8938))})})+(self.scalar_static_f64[29]*(if v2498{(v1+(v45*(v9012+((v9013+v9013)/v9015))))}else{(if v2506{(v1+((-(v2482*(v10+((v9024+v9024)/v9026))))/v9034))}else{v2})}))));
        let v9066=(v2525*f64::powf(v2523,-1.1666666666666667));
        let v9075=(v2528*v2528);
        let v9083=(if (self.scalar_static_f64[156]!=0.0){((-(self.scalar_static_f64[4]*(self.scalar_static_f64[24]*((v9058+v9058)*v9066))))/v9075)}else{v2});
        let v9084=(if (self.scalar_static_f64[156]!=0.0){((-(self.scalar_static_f64[4]*(self.scalar_static_f64[24]*((v9060+v9060)*v9066))))/v9075)}else{v2});
        let v9085=(if (self.scalar_static_f64[156]!=0.0){((-(self.scalar_static_f64[4]*(self.scalar_static_f64[24]*((v9062+v9062)*v9066))))/v9075)}else{v2});
        let v10654=(self.scalar_static_f64[20]*((v3343*v9083)+(v2530*(self.scalar_static_f64[52]*(self.scalar_static_f64[50]*((self.scalar_static_f64[20]-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*v8732)}else{v6964}))-(if self.scalar_static_bool[17]{v2}else{v8035})))))));
        let v10655=(self.scalar_static_f64[20]*((v3343*v9084)+(v2530*(self.scalar_static_f64[52]*(self.scalar_static_f64[50]*((self.scalar_static_f64[150]-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*v8733)}else{v6965}))-(if self.scalar_static_bool[17]{v2}else{v8036})))))));
        let v10656=(self.scalar_static_f64[20]*((v3343*v9085)+(v2530*(self.scalar_static_f64[52]*(self.scalar_static_f64[50]*((-(if (self.scalar_static_f64[149]!=0.0){(self.scalar_static_f64[172]*v8734)}else{v6966}))-(if self.scalar_static_bool[17]{v2}else{v8037})))))));

        CommonStampValues {
            v1,
            v2,
            v10,
            v40,
            v45,
            v96,
            v125,
            v143,
            v162,
            v166,
            v173,
            v375,
            v376,
            v377,
            v381,
            v410,
            v418,
            v455,
            v456,
            v457,
            v463,
            v479,
            v483,
            v489,
            v511,
            v515,
            v559,
            v567,
            v634,
            v642,
            v1204,
            v1538,
            v1545,
            v1548,
            v1552,
            v1564,
            v1567,
            v1575,
            v1601,
            v1629,
            v1631,
            v1642,
            v2390,
            v2530,
            v2577,
            v3345,
            v3347,
            v3348,
            v3350,
            v3506,
            v3507,
            v3508,
            v3509,
            v3511,
            v3513,
            v3526,
            v3527,
            v3557,
            v3558,
            v5169,
            v5170,
            v6027,
            v6028,
            v6038,
            v6039,
            v6067,
            v6068,
            v6219,
            v6220,
            v6224,
            v6225,
            v8732,
            v8733,
            v8734,
            v8762,
            v9083,
            v9084,
            v9085,
            v10654,
            v10655,
            v10656,
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
        let v365=0.1;
        let v1557=(((common.v1/common.v1552)-common.v1538)-common.v1);
        let v1570=(common.v1+common.v1538);
        let v1571=(common.v479*v1570);
        let v1577=((common.v463-common.v1538)-common.v559);
        let v1578=(common.v45*v1577);
        let v1580=(common.v1+(common.v40*v1577));
        let v1582=(common.v1+(v1578*v1580));
        let v1584=(common.v1+(v1577*v1582));
        let v1611=(common.v479*common.v489);
        let v1612=(common.v1538*v1611);
        let v1613=(common.v1538*v1612);
        let v1614=(common.v1538*v1613);
        let v1615=1.75;
        let v1617=(common.v1+(common.v1538*v1615));
        let v1619=(if common.v1601{(v1614*v1617)}else{(if common.v1575{((if common.v1575{(common.v642/v1584)}else{common.v1567})-v1571)}else{(if common.v1564{(common.v1567-v1571)}else{(if common.v1548{(common.v479*v1557)}else{common.v2})})})});
        let v1633=((v1619+common.v1629)).sqrt();
        let v1636=(self.scalar_static_f64[172]*common.v457);
        let v1637=(v1619*v1636);
        let v1639=((if common.v1545{(common.v456*v1633)}else{common.v2})+(common.v456*common.v1631));
        let v2531=(common.v511-common.v1538);
        let v2532=(v2531>common.v381);
        let v2534=0.01;
        let v2536=(((v2531*v2531)+v2534)).sqrt();
        let v2540=(common.v1538-common.v511);
        let v2541=(v2540>common.v381);
        let v2542=0.005;
        let v2545=((v2534+(v2540*v2540))).sqrt();
        let v2546=(v2540+v2545);
        let v2555=((-(if v2532{(common.v511-(common.v45*(v2531+v2536)))}else{(if v2541{(common.v511-(v2542/v2546))}else{(common.v511-(common.v45*(v365+v2531)))})}))).exp();
        let v2557=((self.scalar_static_f64[172]*v2555)).sqrt();
        let v2558=(common.v455*common.v2530);
        let v2564=(((common.v483*common.v483)+0.04)).sqrt();
        let v2567=(self.scalar_static_f64[289]*(v2557*v2558));
        let v2570=(common.v1+((common.v45*((-common.v483)+v2564))*self.scalar_static_f64[157]));
        let v2578=(common.v375-common.v2577);
        let v2581=(self.scalar_static_f64[175]*(self.scalar_static_f64[20]*(v2578-self.scalar_static_f64[335])));
        let v2590=(if ((v2581).abs()<=self.scalar_static_f64[231]){common.v1}else{common.v2});
        let v2591=((self.scalar_static_f64[336]!=0.0)&&(v2590!=0.0));
        let v2592=(v2581/self.scalar_static_f64[230]);
        let v2595=(if (v2581>self.scalar_static_f64[231]){common.v1}else{common.v2});
        let v2597=((self.scalar_static_f64[336]!=0.0)&&(!(v2590!=0.0)));
        let v2598=((v2595!=0.0)&&v2597);
        let v2603=(if v2598{self.scalar_static_f64[340]}else{common.v2});
        let v2605=(common.v1+(v2581*v2603));
        let v2607=(if v2598{(v2592*v2605)}else{common.v2});
        let v2609=(if (v2607<common.v166){common.v1}else{common.v2});
        let v2610=(v2598&&(v2609!=0.0));
        let v2612=((-v2607)).exp();
        let v2615=(v2598&&(!(v2609!=0.0)));
        let v2616=(v2607-common.v166);
        let v2617=(common.v45*v2616);
        let v2619=(common.v1+(common.v40*v2616));
        let v2621=(common.v1+(v2617*v2619));
        let v2623=(common.v1+(v2616*v2621));
        let v2625=(if v2615{(common.v173/v2623)}else{(if v2610{v2612}else{common.v2})});
        let v2627=(if v2598{(common.v1-v2625)}else{common.v2});
        let v2633=(((v2581+self.scalar_static_f64[342])-v2627)).sqrt();
        let v2636=(if v2598{((v2581+self.scalar_static_f64[341])-(self.scalar_static_f64[227]*v2633))}else{common.v2});
        let v2638=(if (v2636<common.v166){common.v1}else{common.v2});
        let v2639=(v2598&&(v2638!=0.0));
        let v2641=((-v2636)).exp();
        let v2644=(v2598&&(!(v2638!=0.0)));
        let v2645=(v2636-common.v166);
        let v2646=(common.v45*v2645);
        let v2648=(common.v1+(common.v40*v2645));
        let v2650=(common.v1+(v2646*v2648));
        let v2652=(common.v1+(v2645*v2650));
        let v2654=(if v2644{(common.v173/v2652)}else{(if v2639{v2641}else{common.v2})});
        let v2657=(if v2598{(common.v1-(self.scalar_static_f64[341]*v2654))}else{common.v2});
        let v2658=(v2581-v2636);
        let v2663=(if v2598{((common.v10*v2658)+(self.scalar_static_f64[228]*(common.v1-v2654)))}else{common.v2});
        let v2669=(if v2598{((v2658*v2658)-(self.scalar_static_f64[228]*(v2654+(v2636-common.v1))))}else{common.v2});
        let v2671=(common.v96*v2657);
        let v2674=(if v2598{((v2663*v2663)-(v2669*v2671))}else{v2625});
        let v2675=(common.v10*v2669);
        let v2676=(v2674).sqrt();
        let v2677=(v2663+v2676);
        let v2683=(v2597&&(!(v2595!=0.0)));
        let v2685=(if v2683{(-v2581)}else{common.v2});
        let v2688=(if v2683{((common.v162*v2685)/self.scalar_static_f64[230])}else{common.v2});
        let v2690=(v2688-common.v143);
        let v2693=((common.v515+(v2690*v2690))).sqrt();
        let v2696=(if v2683{(common.v45*((common.v511+v2688)-v2693))}else{common.v2});
        let v2697=(v2685-v2696);
        let v2702=(if v2683{((v2697*v2697)+(self.scalar_static_f64[228]*(common.v1+v2696)))}else{common.v2});
        let v2705=(if v2683{((common.v10*v2697)-self.scalar_static_f64[228])}else{common.v2});
        let v2706=(v2702/self.scalar_static_f64[228]);
        let v2709=(if v2683{((v2706).ln()-v2696)}else{common.v2});
        let v2711=(if v2683{(v2702+v2705)}else{common.v2});
        let v2713=(common.v45*v2705);
        let v2715=((v2705*v2713)-v2702);
        let v2718=(if v2683{((v2711*v2711)+(v2709*v2715))}else{common.v2});
        let v2719=(v2702*v2711);
        let v2720=(v2709*v2719);
        let v2721=(v2709*v2711);
        let v2722=(v2709*v2721);
        let v2723=(v2722/v2718);
        let v2724=(v2705*v2723);
        let v2727=((common.v40*(v2705*v2705))-v2702);
        let v2729=(v2718+(v2724*v2727));
        let v2732=(if v2683{(v2696+(v2720/v2729))}else{common.v2});
        let v2735=(if ((v2732).abs()<common.v559){common.v1}else{common.v2});
        let v2736=(v2683&&(v2735!=0.0));
        let v2737=(v2732).exp();
        let v2740=(if (v2732<common.v634){common.v1}else{common.v2});
        let v2742=(v2683&&(!(v2735!=0.0)));
        let v2743=((v2740!=0.0)&&v2742);
        let v2744=(common.v634-v2732);
        let v2745=(common.v45*v2744);
        let v2747=(common.v1+(common.v40*v2744));
        let v2749=(common.v1+(v2745*v2747));
        let v2751=(common.v1+(v2744*v2749));
        let v2755=(v2742&&(!(v2740!=0.0)));
        let v2756=(v2732-common.v559);
        let v2757=(common.v45*v2756);
        let v2759=(common.v1+(common.v40*v2756));
        let v2761=(common.v1+(v2757*v2759));
        let v2765=(if v2755{(common.v567*(common.v1+(v2756*v2761)))}else{(if v2743{(common.v642/v2751)}else{(if v2736{v2737}else{v2654})})});
        let v2769=(v2685-v2732);
        let v2774=(if v2683{((common.v10*v2769)+(self.scalar_static_f64[228]*(v2765-common.v1)))}else{v2663});
        let v2780=(if v2683{((v2769*v2769)+(self.scalar_static_f64[228]*((common.v1+v2732)-v2765)))}else{v2669});
        let v2782=(common.v96*(if v2683{(common.v1-(self.scalar_static_f64[341]*v2765))}else{v2657}));
        let v2786=(common.v10*v2780);
        let v2787=((if v2683{((v2774*v2774)-(v2780*v2782))}else{v2674})).sqrt();
        let v2788=(v2774+v2787);
        let v2793=(if v2683{(-(v2732+(if v2683{(v2786/v2788)}else{v2627})))}else{(if v2598{(v2636+(if v2598{(v2675/v2677)}else{common.v2}))}else{(if v2591{v2592}else{common.v2})})});
        let v2798=(if self.scalar_static_bool[40]{common.v2}else{(if (self.scalar_static_f64[336]!=0.0){(self.scalar_static_f64[172]*(v2581-v2793))}else{common.v2})});
        let v2808=(self.scalar_static_f64[20]*v2798);
        let v2810=(if self.scalar_static_bool[43]{(self.scalar_static_f64[317]+v2808)}else{common.v2});
        let v2811=(common.v2-v2810);
        let v2812=(v2811>common.v381);
        let v2815=((v2534+(v2811*v2811))).sqrt();
        let v2819=(v2810>common.v381);
        let v2822=((v2534+(v2810*v2810))).sqrt();
        let v2823=(v2810+v2822);
        let v2831=(if self.scalar_static_bool[43]{(if v2812{(v2810+(common.v45*(v2811+v2815)))}else{(if v2819{(v2810+(v2542/v2823))}else{(v2810+(common.v45*(v365+v2811)))})})}else{common.v2});
        let v2834=((common.v410+(v2798*v2798))).sqrt();
        let v2836=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*v2834)}else{common.v2});
        let v2838=(self.scalar_static_f64[134]-v2836);
        let v2839=(v2838>common.v381);
        let v2842=((common.v410+(v2838*v2838))).sqrt();
        let v2846=(v2836-self.scalar_static_f64[134]);
        let v2847=(v2846>common.v381);
        let v2850=((common.v410+(v2846*v2846))).sqrt();
        let v2851=(v2846+v2850);
        let v2859=(if self.scalar_static_bool[44]{(if v2839{(self.scalar_static_f64[134]-(common.v45*(v2838+v2842)))}else{(if v2847{(self.scalar_static_f64[134]-(common.v418/v2851))}else{(self.scalar_static_f64[134]-(common.v45*(common.v125+v2838)))})})}else{v2836});
        let v2862=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v2}else{v2793}));
        let v2876=(if self.scalar_static_bool[46]{(-(v2862+(self.scalar_static_f64[175]*(v2831+self.scalar_static_f64[346]))))}else{(if self.scalar_static_bool[45]{(-(v2862+(self.scalar_static_f64[175]*(v2831+self.scalar_static_f64[345]))))}else{common.v2})});
        let v2878=(if (v2876<common.v559){common.v1}else{common.v2});
        let v2879=(self.scalar_static_bool[43]&&(v2878!=0.0));
        let v2880=(v2876).exp();
        let v2881=(common.v1+v2880);
        let v2885=(self.scalar_static_bool[43]&&(!(v2878!=0.0)));
        let v2886=(if v2885{v2876}else{(if v2879{(v2881).ln()}else{common.v2})});
        let v2888=(self.scalar_static_f64[175]*(self.scalar_static_f64[20]*(if self.scalar_static_bool[41]{(self.scalar_static_f64[20]*v2578)}else{common.v2})));
        let v2890=(if self.scalar_static_bool[43]{(v2876+v2888)}else{common.v2});
        let v2892=(if (v2890<common.v559){common.v1}else{common.v2});
        let v2893=(self.scalar_static_bool[43]&&(v2892!=0.0));
        let v2894=(v2890).exp();
        let v2895=(common.v1+v2894);
        let v2899=(self.scalar_static_bool[43]&&(!(v2892!=0.0)));
        let v2900=(if v2899{v2890}else{(if v2893{(v2895).ln()}else{common.v2})});
        let v2901=-1.5;
        let v2903=(self.scalar_static_f64[126]+(self.scalar_static_f64[124]*v2859));
        let v2907=(if self.scalar_static_bool[43]{(self.scalar_static_f64[140]*(v2901+(v2859*v2903)))}else{common.v2});
        let v2909=(if (v2907>common.v2){common.v1}else{common.v2});
        let v2910=(self.scalar_static_bool[43]&&(v2909!=0.0));
        let v2911=(common.v45*v2907);
        let v2913=(common.v1+(common.v40*v2907));
        let v2915=(common.v1+(v2911*v2913));
        let v2920=(if (v2907>common.v634){common.v1}else{common.v2});
        let v2922=(self.scalar_static_bool[43]&&(!(v2909!=0.0)));
        let v2923=((v2920!=0.0)&&v2922);
        let v2924=(v2907).exp();
        let v2927=(v2922&&(!(v2920!=0.0)));
        let v2928=(common.v634-v2907);
        let v2929=(common.v45*v2928);
        let v2931=(common.v1+(common.v40*v2928));
        let v2933=(common.v1+(v2929*v2931));
        let v2935=(common.v1+(v2928*v2933));
        let v2937=(if v2927{(common.v642/v2935)}else{(if v2923{v2924}else{(if v2910{(common.v1+(v2907*v2915))}else{common.v2})})});
        let v2939=(self.scalar_static_f64[20]*(self.scalar_static_f64[315]*v2937));
        let v2940=(v2900-v2886);
        let v2942=(if self.scalar_static_bool[43]{(v2939*v2940)}else{common.v2});
        let v2946=(if self.scalar_static_bool[47]{(self.scalar_static_f64[316]+v2808)}else{common.v2});
        let v2947=(v2946>common.v381);
        let v2950=((v2534+(v2946*v2946))).sqrt();
        let v2954=(common.v2-v2946);
        let v2955=(v2954>common.v381);
        let v2958=((v2534+(v2954*v2954))).sqrt();
        let v2959=(v2954+v2958);
        let v2967=(if self.scalar_static_bool[47]{(if v2947{(v2946-(common.v45*(v2946+v2950)))}else{(if v2955{(v2946-(v2542/v2959))}else{(v2946-(common.v45*(v365+v2946)))})})}else{v2831});
        let v2969=(if self.scalar_static_bool[47]{(self.scalar_static_f64[135]*v2834)}else{v2859});
        let v2971=(self.scalar_static_f64[133]-v2969);
        let v2972=(v2971>common.v381);
        let v2975=((common.v410+(v2971*v2971))).sqrt();
        let v2979=(v2969-self.scalar_static_f64[133]);
        let v2980=(v2979>common.v381);
        let v2983=((common.v410+(v2979*v2979))).sqrt();
        let v2984=(v2979+v2983);
        let v2992=(if self.scalar_static_bool[48]{(if v2972{(self.scalar_static_f64[133]-(common.v45*(v2971+v2975)))}else{(if v2980{(self.scalar_static_f64[133]-(common.v418/v2984))}else{(self.scalar_static_f64[133]-(common.v45*(common.v125+v2971)))})})}else{v2969});
        let v3002=(if self.scalar_static_bool[50]{(v2862+(self.scalar_static_f64[175]*(v2967-self.scalar_static_f64[318])))}else{(if self.scalar_static_bool[49]{(v2862+(self.scalar_static_f64[175]*(v2967-self.scalar_static_f64[319])))}else{v2876})});
        let v3004=(if (v3002<common.v559){common.v1}else{common.v2});
        let v3005=(self.scalar_static_bool[47]&&(v3004!=0.0));
        let v3006=(v3002).exp();
        let v3007=(common.v1+v3006);
        let v3011=(self.scalar_static_bool[47]&&(!(v3004!=0.0)));
        let v3014=(if self.scalar_static_bool[47]{(v3002-v2888)}else{v2890});
        let v3016=(if (v3014<common.v559){common.v1}else{common.v2});
        let v3017=(self.scalar_static_bool[47]&&(v3016!=0.0));
        let v3018=(v3014).exp();
        let v3019=(common.v1+v3018);
        let v3023=(self.scalar_static_bool[47]&&(!(v3016!=0.0)));
        let v3026=(self.scalar_static_f64[119]+(self.scalar_static_f64[117]*v2992));
        let v3030=(if self.scalar_static_bool[47]{(self.scalar_static_f64[138]*(v2901+(v2992*v3026)))}else{v2907});
        let v3033=(if ((v3030).abs()<common.v559){common.v1}else{common.v2});
        let v3034=(self.scalar_static_bool[47]&&(v3033!=0.0));
        let v3035=(v3030).exp();
        let v3038=(if (v3030<common.v634){common.v1}else{common.v2});
        let v3040=(self.scalar_static_bool[47]&&(!(v3033!=0.0)));
        let v3041=((v3038!=0.0)&&v3040);
        let v3042=(common.v634-v3030);
        let v3043=(common.v45*v3042);
        let v3045=(common.v1+(common.v40*v3042));
        let v3047=(common.v1+(v3043*v3045));
        let v3049=(common.v1+(v3042*v3047));
        let v3053=(v3040&&(!(v3038!=0.0)));
        let v3054=(v3030-common.v559);
        let v3055=(common.v45*v3054);
        let v3057=(common.v1+(common.v40*v3054));
        let v3059=(common.v1+(v3055*v3057));
        let v3065=(self.scalar_static_f64[20]*(self.scalar_static_f64[313]*(if v3053{(common.v567*(common.v1+(v3054*v3059)))}else{(if v3041{(common.v642/v3049)}else{(if v3034{v3035}else{v2937})})})));
        let v3066=((if v3011{v3002}else{(if v3005{(v3007).ln()}else{v2886})})-(if v3023{v3014}else{(if v3017{(v3019).ln()}else{v2900})}));
        let v3079=(if self.scalar_static_bool[54]{(self.scalar_static_f64[172]*(common.v1204-common.v2390))}else{common.v2});
        let v3083=(self.scalar_static_f64[20]*v3079);
        let v3085=(if self.scalar_static_bool[56]{(self.scalar_static_f64[317]+v3083)}else{common.v2});
        let v3086=(common.v2-v3085);
        let v3087=(v3086>common.v381);
        let v3090=((v2534+(v3086*v3086))).sqrt();
        let v3094=(v3085>common.v381);
        let v3097=((v2534+(v3085*v3085))).sqrt();
        let v3098=(v3085+v3097);
        let v3106=(if self.scalar_static_bool[56]{(if v3087{(v3085+(common.v45*(v3086+v3090)))}else{(if v3094{(v3085+(v2542/v3098))}else{(v3085+(common.v45*(v365+v3086)))})})}else{common.v2});
        let v3109=((common.v410+(v3079*v3079))).sqrt();
        let v3111=(if self.scalar_static_bool[56]{(self.scalar_static_f64[136]*v3109)}else{common.v2});
        let v3113=(self.scalar_static_f64[134]-v3111);
        let v3114=(v3113>common.v381);
        let v3117=((common.v410+(v3113*v3113))).sqrt();
        let v3121=(v3111-self.scalar_static_f64[134]);
        let v3122=(v3121>common.v381);
        let v3125=((common.v410+(v3121*v3121))).sqrt();
        let v3126=(v3121+v3125);
        let v3134=(if self.scalar_static_bool[57]{(if v3114{(self.scalar_static_f64[134]-(common.v45*(v3113+v3117)))}else{(if v3122{(self.scalar_static_f64[134]-(common.v418/v3126))}else{(self.scalar_static_f64[134]-(common.v45*(common.v125+v3113)))})})}else{v3111});
        let v3136=(self.scalar_static_f64[20]*common.v2390);
        let v3147=(if self.scalar_static_bool[59]{(-(v3136+(self.scalar_static_f64[175]*(self.scalar_static_f64[346]+v3106))))}else{(if self.scalar_static_bool[58]{(-(v3136+(self.scalar_static_f64[175]*(self.scalar_static_f64[345]+v3106))))}else{common.v2})});
        let v3149=(if (v3147<common.v559){common.v1}else{common.v2});
        let v3150=(self.scalar_static_bool[56]&&(v3149!=0.0));
        let v3151=(v3147).exp();
        let v3152=(common.v1+v3151);
        let v3156=(self.scalar_static_bool[56]&&(!(v3149!=0.0)));
        let v3157=(if v3156{v3147}else{(if v3150{(v3152).ln()}else{common.v2})});
        let v3159=(self.scalar_static_f64[175]*(self.scalar_static_f64[20]*(if self.scalar_static_bool[54]{(self.scalar_static_f64[20]*common.v377)}else{common.v2})));
        let v3161=(if self.scalar_static_bool[56]{(v3147+v3159)}else{common.v2});
        let v3163=(if (v3161<common.v559){common.v1}else{common.v2});
        let v3164=(self.scalar_static_bool[56]&&(v3163!=0.0));
        let v3165=(v3161).exp();
        let v3166=(common.v1+v3165);
        let v3170=(self.scalar_static_bool[56]&&(!(v3163!=0.0)));
        let v3171=(if v3170{v3161}else{(if v3164{(v3166).ln()}else{common.v2})});
        let v3173=(self.scalar_static_f64[126]+(self.scalar_static_f64[124]*v3134));
        let v3177=(if self.scalar_static_bool[56]{(self.scalar_static_f64[139]*(v2901+(v3134*v3173)))}else{common.v2});
        let v3179=(if (v3177>common.v2){common.v1}else{common.v2});
        let v3180=(self.scalar_static_bool[56]&&(v3179!=0.0));
        let v3181=(common.v45*v3177);
        let v3183=(common.v1+(common.v40*v3177));
        let v3185=(common.v1+(v3181*v3183));
        let v3190=(if (v3177>common.v634){common.v1}else{common.v2});
        let v3192=(self.scalar_static_bool[56]&&(!(v3179!=0.0)));
        let v3193=((v3190!=0.0)&&v3192);
        let v3194=(v3177).exp();
        let v3197=(v3192&&(!(v3190!=0.0)));
        let v3198=(common.v634-v3177);
        let v3199=(common.v45*v3198);
        let v3201=(common.v1+(common.v40*v3198));
        let v3203=(common.v1+(v3199*v3201));
        let v3205=(common.v1+(v3198*v3203));
        let v3207=(if v3197{(common.v642/v3205)}else{(if v3193{v3194}else{(if v3180{(common.v1+(v3177*v3185))}else{common.v2})})});
        let v3209=(self.scalar_static_f64[20]*(self.scalar_static_f64[314]*v3207));
        let v3210=(v3171-v3157);
        let v3212=(if self.scalar_static_bool[56]{(v3209*v3210)}else{common.v2});
        let v3216=(if self.scalar_static_bool[60]{(self.scalar_static_f64[316]+v3083)}else{common.v2});
        let v3217=(v3216>common.v381);
        let v3220=((v2534+(v3216*v3216))).sqrt();
        let v3224=(common.v2-v3216);
        let v3225=(v3224>common.v381);
        let v3228=((v2534+(v3224*v3224))).sqrt();
        let v3229=(v3224+v3228);
        let v3237=(if self.scalar_static_bool[60]{(if v3217{(v3216-(common.v45*(v3216+v3220)))}else{(if v3225{(v3216-(v2542/v3229))}else{(v3216-(common.v45*(v365+v3216)))})})}else{v3106});
        let v3239=(if self.scalar_static_bool[60]{(self.scalar_static_f64[135]*v3109)}else{v3134});
        let v3241=(self.scalar_static_f64[133]-v3239);
        let v3242=(v3241>common.v381);
        let v3245=((common.v410+(v3241*v3241))).sqrt();
        let v3249=(v3239-self.scalar_static_f64[133]);
        let v3250=(v3249>common.v381);
        let v3253=((common.v410+(v3249*v3249))).sqrt();
        let v3254=(v3249+v3253);
        let v3262=(if self.scalar_static_bool[61]{(if v3242{(self.scalar_static_f64[133]-(common.v45*(v3241+v3245)))}else{(if v3250{(self.scalar_static_f64[133]-(common.v418/v3254))}else{(self.scalar_static_f64[133]-(common.v45*(common.v125+v3241)))})})}else{v3239});
        let v3272=(if self.scalar_static_bool[63]{(v3136+(self.scalar_static_f64[175]*(v3237-self.scalar_static_f64[318])))}else{(if self.scalar_static_bool[62]{(v3136+(self.scalar_static_f64[175]*(v3237-self.scalar_static_f64[319])))}else{v3147})});
        let v3274=(if (v3272<common.v559){common.v1}else{common.v2});
        let v3275=(self.scalar_static_bool[60]&&(v3274!=0.0));
        let v3276=(v3272).exp();
        let v3277=(common.v1+v3276);
        let v3281=(self.scalar_static_bool[60]&&(!(v3274!=0.0)));
        let v3284=(if self.scalar_static_bool[60]{(v3272-v3159)}else{v3161});
        let v3286=(if (v3284<common.v559){common.v1}else{common.v2});
        let v3287=(self.scalar_static_bool[60]&&(v3286!=0.0));
        let v3288=(v3284).exp();
        let v3289=(common.v1+v3288);
        let v3293=(self.scalar_static_bool[60]&&(!(v3286!=0.0)));
        let v3296=(self.scalar_static_f64[119]+(self.scalar_static_f64[117]*v3262));
        let v3300=(if self.scalar_static_bool[60]{(self.scalar_static_f64[137]*(v2901+(v3262*v3296)))}else{v3177});
        let v3303=(if ((v3300).abs()<common.v559){common.v1}else{common.v2});
        let v3304=(self.scalar_static_bool[60]&&(v3303!=0.0));
        let v3305=(v3300).exp();
        let v3308=(if (v3300<common.v634){common.v1}else{common.v2});
        let v3310=(self.scalar_static_bool[60]&&(!(v3303!=0.0)));
        let v3311=((v3308!=0.0)&&v3310);
        let v3312=(common.v634-v3300);
        let v3313=(common.v45*v3312);
        let v3315=(common.v1+(common.v40*v3312));
        let v3317=(common.v1+(v3313*v3315));
        let v3319=(common.v1+(v3312*v3317));
        let v3323=(v3310&&(!(v3308!=0.0)));
        let v3324=(v3300-common.v559);
        let v3325=(common.v45*v3324);
        let v3327=(common.v1+(common.v40*v3324));
        let v3329=(common.v1+(v3325*v3327));
        let v3335=(self.scalar_static_f64[20]*(self.scalar_static_f64[312]*(if v3323{(common.v567*(common.v1+(v3324*v3329)))}else{(if v3311{(common.v642/v3319)}else{(if v3304{v3305}else{v3207})})})));
        let v3336=((if v3281{v3272}else{(if v3275{(v3277).ln()}else{v3157})})-(if v3293{v3284}else{(if v3287{(v3289).ln()}else{v3171})}));
        let v3359=(common.v376-common.v2577);
        let v3360=(self.scalar_static_f64[288]+(v2567/v2570));
        let v6049=(common.v1552*common.v1552);
        let v6082=((v1570*common.v3557)+(common.v479*common.v6027));
        let v6085=((v1570*common.v3558)+(common.v479*common.v6028));
        let v6090=(common.v3526-common.v6027);
        let v6091=(common.v3527-common.v6028);
        let v6110=(v1584*v1584);
        let v6200=(if common.v1601{((v1617*((v1613*common.v6027)+(common.v1538*((v1612*common.v6027)+(common.v1538*((v1611*common.v6027)+(common.v1538*(common.v489*common.v3557))))))))+(v1614*(v1615*common.v6027)))}else{(if common.v1575{((if common.v1575{((-(common.v642*((v1582*v6090)+(v1577*((v1580*(common.v45*v6090))+(v1578*(common.v40*v6090)))))))/v6110)}else{common.v6067})-v6082)}else{(if common.v1564{(common.v6067-v6082)}else{(if common.v1548{((v1557*common.v3557)+(common.v479*(((-common.v6038)/v6049)-common.v6027)))}else{common.v2})})})});
        let v6201=(if common.v1601{((v1617*((v1613*common.v6028)+(common.v1538*((v1612*common.v6028)+(common.v1538*((v1611*common.v6028)+(common.v1538*(common.v489*common.v3558))))))))+(v1614*(v1615*common.v6028)))}else{(if common.v1575{((if common.v1575{((-(common.v642*((v1582*v6091)+(v1577*((v1580*(common.v45*v6091))+(v1578*(common.v40*v6091)))))))/v6110)}else{common.v6068})-v6085)}else{(if common.v1564{(common.v6068-v6085)}else{(if common.v1548{((v1557*common.v3558)+(common.v479*(((-common.v6039)/v6049)-common.v6028)))}else{common.v2})})})});
        let v6228=(common.v10*v1633);
        let v6258=(v1639*v1639);
        let v9086=(-common.v6027);
        let v9087=(-common.v6028);
        let v9088=(v2531*v9086);
        let v9090=(v2531*v9087);
        let v9092=(common.v10*v2536);
        let v9101=(v2540*common.v6027);
        let v9103=(v2540*common.v6028);
        let v9105=(common.v10*v2545);
        let v9112=(v2546*v2546);
        let v9133=(common.v10*v2557);
        let v9150=(self.scalar_static_f64[20]*common.v483);
        let v9152=(common.v483*self.scalar_static_f64[150]);
        let v9154=(common.v10*v2564);
        let v9169=(v2570*v2570);
        let v9188=(if v2598{((v2605*self.scalar_static_f64[356])+(v2592*(v2603*self.scalar_static_f64[352])))}else{common.v2});
        let v9189=(if v2598{((v2605*self.scalar_static_f64[357])+(v2592*(v2603*self.scalar_static_f64[351])))}else{common.v2});
        let v9214=(v2623*v2623);
        let v9219=(if v2615{((-(common.v173*((v2621*v9188)+(v2616*((v2619*(common.v45*v9188))+(v2617*(common.v40*v9188)))))))/v9214)}else{(if v2610{(v2612*(-v9188))}else{common.v2})});
        let v9220=(if v2615{((-(common.v173*((v2621*v9189)+(v2616*((v2619*(common.v45*v9189))+(v2617*(common.v40*v9189)))))))/v9214)}else{(if v2610{(v2612*(-v9189))}else{common.v2})});
        let v9223=(if v2598{(-v9219)}else{common.v2});
        let v9224=(if v2598{(-v9220)}else{common.v2});
        let v9227=(common.v10*v2633);
        let v9234=(if v2598{(self.scalar_static_f64[352]-(self.scalar_static_f64[227]*((self.scalar_static_f64[352]-v9223)/v9227)))}else{common.v2});
        let v9235=(if v2598{(self.scalar_static_f64[351]-(self.scalar_static_f64[227]*((self.scalar_static_f64[351]-v9224)/v9227)))}else{common.v2});
        let v9260=(v2652*v2652);
        let v9265=(if v2644{((-(common.v173*((v2650*v9234)+(v2645*((v2648*(common.v45*v9234))+(v2646*(common.v40*v9234)))))))/v9260)}else{(if v2639{(v2641*(-v9234))}else{common.v2})});
        let v9266=(if v2644{((-(common.v173*((v2650*v9235)+(v2645*((v2648*(common.v45*v9235))+(v2646*(common.v40*v9235)))))))/v9260)}else{(if v2639{(v2641*(-v9235))}else{common.v2})});
        let v9271=(if v2598{(-(self.scalar_static_f64[341]*v9265))}else{common.v2});
        let v9272=(if v2598{(-(self.scalar_static_f64[341]*v9266))}else{common.v2});
        let v9273=(self.scalar_static_f64[352]-v9234);
        let v9274=(self.scalar_static_f64[351]-v9235);
        let v9283=(if v2598{((common.v10*v9273)+(self.scalar_static_f64[228]*(-v9265)))}else{common.v2});
        let v9284=(if v2598{((common.v10*v9274)+(self.scalar_static_f64[228]*(-v9266)))}else{common.v2});
        let v9285=(v2658*v9273);
        let v9287=(v2658*v9274);
        let v9295=(if v2598{((v9285+v9285)-(self.scalar_static_f64[228]*(v9234+v9265)))}else{common.v2});
        let v9296=(if v2598{((v9287+v9287)-(self.scalar_static_f64[228]*(v9235+v9266)))}else{common.v2});
        let v9297=(v2663*v9283);
        let v9299=(v2663*v9284);
        let v9311=(if v2598{((v9297+v9297)-((v2671*v9295)+(v2669*(common.v96*v9271))))}else{v9219});
        let v9312=(if v2598{((v9299+v9299)-((v2671*v9296)+(v2669*(common.v96*v9272))))}else{v9220});
        let v9315=(common.v10*v2676);
        let v9323=(v2677*v2677);
        let v9335=(if v2683{self.scalar_static_f64[354]}else{common.v2});
        let v9336=(if v2683{self.scalar_static_f64[353]}else{common.v2});
        let v9341=(if v2683{((common.v162*v9335)/self.scalar_static_f64[230])}else{common.v2});
        let v9342=(if v2683{((common.v162*v9336)/self.scalar_static_f64[230])}else{common.v2});
        let v9343=(v2690*v9341);
        let v9345=(v2690*v9342);
        let v9347=(common.v10*v2693);
        let v9354=(if v2683{(common.v45*(v9341-((v9343+v9343)/v9347)))}else{common.v2});
        let v9355=(if v2683{(common.v45*(v9342-((v9345+v9345)/v9347)))}else{common.v2});
        let v9356=(v9335-v9354);
        let v9357=(v9336-v9355);
        let v9358=(v2697*v9356);
        let v9360=(v2697*v9357);
        let v9366=(if v2683{((v9358+v9358)+(self.scalar_static_f64[228]*v9354))}else{common.v2});
        let v9367=(if v2683{((v9360+v9360)+(self.scalar_static_f64[228]*v9355))}else{common.v2});
        let v9370=(if v2683{(common.v10*v9356)}else{common.v2});
        let v9371=(if v2683{(common.v10*v9357)}else{common.v2});
        let v9378=(if v2683{(((v9366/self.scalar_static_f64[228])/v2706)-v9354)}else{common.v2});
        let v9379=(if v2683{(((v9367/self.scalar_static_f64[228])/v2706)-v9355)}else{common.v2});
        let v9382=(if v2683{(v9366+v9370)}else{common.v2});
        let v9383=(if v2683{(v9367+v9371)}else{common.v2});
        let v9384=(v2711*v9382);
        let v9386=(v2711*v9383);
        let v9406=(if v2683{((v9384+v9384)+((v2715*v9378)+(v2709*(((v2713*v9370)+(v2705*(common.v45*v9370)))-v9366))))}else{common.v2});
        let v9407=(if v2683{((v9386+v9386)+((v2715*v9379)+(v2709*(((v2713*v9371)+(v2705*(common.v45*v9371)))-v9367))))}else{common.v2});
        let v9435=(v2718*v2718);
        let v9447=(v2705*v9370);
        let v9449=(v2705*v9371);
        let v9466=(v2729*v2729);
        let v9474=(if v2683{(v9354+(((v2729*((v2719*v9378)+(v2709*((v2711*v9366)+(v2702*v9382)))))-(v2720*(v9406+((v2727*((v2723*v9370)+(v2705*(((v2718*((v2721*v9378)+(v2709*((v2711*v9378)+(v2709*v9382)))))-(v2722*v9406))/v9435))))+(v2724*((common.v40*(v9447+v9447))-v9366))))))/v9466))}else{common.v2});
        let v9475=(if v2683{(v9355+(((v2729*((v2719*v9379)+(v2709*((v2711*v9367)+(v2702*v9383)))))-(v2720*(v9407+((v2727*((v2723*v9371)+(v2705*(((v2718*((v2721*v9379)+(v2709*((v2711*v9379)+(v2709*v9383)))))-(v2722*v9407))/v9435))))+(v2724*((common.v40*(v9449+v9449))-v9367))))))/v9466))}else{common.v2});
        let v9480=(-v9474);
        let v9481=(-v9475);
        let v9500=(v2751*v2751);
        let v9525=(if v2755{(common.v567*((v2761*v9474)+(v2756*((v2759*(common.v45*v9474))+(v2757*(common.v40*v9474))))))}else{(if v2743{((-(common.v642*((v2749*v9480)+(v2744*((v2747*(common.v45*v9480))+(v2745*(common.v40*v9480)))))))/v9500)}else{(if v2736{(v2737*v9474)}else{v9265})})});
        let v9526=(if v2755{(common.v567*((v2761*v9475)+(v2756*((v2759*(common.v45*v9475))+(v2757*(common.v40*v9475))))))}else{(if v2743{((-(common.v642*((v2749*v9481)+(v2744*((v2747*(common.v45*v9481))+(v2745*(common.v40*v9481)))))))/v9500)}else{(if v2736{(v2737*v9475)}else{v9266})})});
        let v9533=(v9335-v9474);
        let v9534=(v9336-v9475);
        let v9541=(if v2683{((common.v10*v9533)+(self.scalar_static_f64[228]*v9525))}else{v9283});
        let v9542=(if v2683{((common.v10*v9534)+(self.scalar_static_f64[228]*v9526))}else{v9284});
        let v9543=(v2769*v9533);
        let v9545=(v2769*v9534);
        let v9553=(if v2683{((v9543+v9543)+(self.scalar_static_f64[228]*(v9474-v9525)))}else{v9295});
        let v9554=(if v2683{((v9545+v9545)+(self.scalar_static_f64[228]*(v9475-v9526)))}else{v9296});
        let v9555=(v2774*v9541);
        let v9557=(v2774*v9542);
        let v9573=(common.v10*v2787);
        let v9581=(v2788*v2788);
        let v9593=(if v2683{(-(v9474+(if v2683{(((v2788*(common.v10*v9553))-(v2786*(v9541+((if v2683{((v9555+v9555)-((v2782*v9553)+(v2780*(common.v96*(if v2683{(-(self.scalar_static_f64[341]*v9525))}else{v9271})))))}else{v9311})/v9573))))/v9581)}else{v9223})))}else{(if v2598{(v9234+(if v2598{(((v2677*(common.v10*v9295))-(v2675*(v9283+(v9311/v9315))))/v9323)}else{common.v2}))}else{(if v2591{self.scalar_static_f64[356]}else{common.v2})})});
        let v9594=(if v2683{(-(v9475+(if v2683{(((v2788*(common.v10*v9554))-(v2786*(v9542+((if v2683{((v9557+v9557)-((v2782*v9554)+(v2780*(common.v96*(if v2683{(-(self.scalar_static_f64[341]*v9526))}else{v9272})))))}else{v9312})/v9573))))/v9581)}else{v9224})))}else{(if v2598{(v9235+(if v2598{(((v2677*(common.v10*v9296))-(v2675*(v9284+(v9312/v9315))))/v9323)}else{common.v2}))}else{(if v2591{self.scalar_static_f64[357]}else{common.v2})})});
        let v9601=(if self.scalar_static_bool[40]{common.v2}else{(if (self.scalar_static_f64[336]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[352]-v9593))}else{common.v2})});
        let v9602=(if self.scalar_static_bool[40]{common.v2}else{(if (self.scalar_static_f64[336]!=0.0){(self.scalar_static_f64[172]*(self.scalar_static_f64[351]-v9594))}else{common.v2})});
        let v9607=(self.scalar_static_f64[20]*v9601);
        let v9608=(self.scalar_static_f64[20]*v9602);
        let v9609=(if self.scalar_static_bool[43]{v9607}else{common.v2});
        let v9610=(if self.scalar_static_bool[43]{v9608}else{common.v2});
        let v9611=(-v9609);
        let v9612=(-v9610);
        let v9613=(v2811*v9611);
        let v9615=(v2811*v9612);
        let v9617=(common.v10*v2815);
        let v9626=(v2810*v9609);
        let v9628=(v2810*v9610);
        let v9630=(common.v10*v2822);
        let v9637=(v2823*v2823);
        let v9652=(if self.scalar_static_bool[43]{(if v2812{(v9609+(common.v45*(v9611+((v9613+v9613)/v9617))))}else{(if v2819{(v9609+((-(v2542*(v9609+((v9626+v9626)/v9630))))/v9637))}else{(v9609+(common.v45*v9611))})})}else{common.v2});
        let v9653=(if self.scalar_static_bool[43]{(if v2812{(v9610+(common.v45*(v9612+((v9615+v9615)/v9617))))}else{(if v2819{(v9610+((-(v2542*(v9610+((v9628+v9628)/v9630))))/v9637))}else{(v9610+(common.v45*v9612))})})}else{common.v2});
        let v9654=(v2798*v9601);
        let v9656=(v2798*v9602);
        let v9658=(common.v10*v2834);
        let v9659=((v9654+v9654)/v9658);
        let v9660=((v9656+v9656)/v9658);
        let v9663=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*v9659)}else{common.v2});
        let v9664=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*v9660)}else{common.v2});
        let v9665=(-v9663);
        let v9666=(-v9664);
        let v9667=(v2838*v9665);
        let v9669=(v2838*v9666);
        let v9671=(common.v10*v2842);
        let v9680=(v2846*v9663);
        let v9682=(v2846*v9664);
        let v9684=(common.v10*v2850);
        let v9691=(v2851*v2851);
        let v9706=(if self.scalar_static_bool[44]{(if v2839{(-(common.v45*(v9665+((v9667+v9667)/v9671))))}else{(if v2847{(-((-(common.v418*(v9663+((v9680+v9680)/v9684))))/v9691))}else{(-(common.v45*v9665))})})}else{v9663});
        let v9707=(if self.scalar_static_bool[44]{(if v2839{(-(common.v45*(v9666+((v9669+v9669)/v9671))))}else{(if v2847{(-((-(common.v418*(v9664+((v9682+v9682)/v9684))))/v9691))}else{(-(common.v45*v9666))})})}else{v9664});
        let v9708=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v2}else{v9593}));
        let v9709=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v2}else{v9594}));
        let v9714=(-(v9708+(self.scalar_static_f64[175]*v9652)));
        let v9715=(-(v9709+(self.scalar_static_f64[175]*v9653)));
        let v9718=(if self.scalar_static_bool[46]{v9714}else{(if self.scalar_static_bool[45]{v9714}else{common.v2})});
        let v9719=(if self.scalar_static_bool[46]{v9715}else{(if self.scalar_static_bool[45]{v9715}else{common.v2})});
        let v9726=(if v2885{v9718}else{(if v2879{((v2880*v9718)/v2881)}else{common.v2})});
        let v9727=(if v2885{v9719}else{(if v2879{((v2880*v9719)/v2881)}else{common.v2})});
        let v9734=(if self.scalar_static_bool[43]{(v9718+self.scalar_static_f64[362])}else{common.v2});
        let v9735=(if self.scalar_static_bool[43]{(v9719+self.scalar_static_f64[363])}else{common.v2});
        let v9742=(if v2899{v9734}else{(if v2893{((v2894*v9734)/v2895)}else{common.v2})});
        let v9743=(if v2899{v9735}else{(if v2893{((v2894*v9735)/v2895)}else{common.v2})});
        let v9754=(if self.scalar_static_bool[43]{(self.scalar_static_f64[140]*((v2903*v9706)+(v2859*(self.scalar_static_f64[124]*v9706))))}else{common.v2});
        let v9755=(if self.scalar_static_bool[43]{(self.scalar_static_f64[140]*((v2903*v9707)+(v2859*(self.scalar_static_f64[124]*v9707))))}else{common.v2});
        let v9778=(-v9754);
        let v9779=(-v9755);
        let v9798=(v2935*v2935);
        let v9803=(if v2927{((-(common.v642*((v2933*v9778)+(v2928*((v2931*(common.v45*v9778))+(v2929*(common.v40*v9778)))))))/v9798)}else{(if v2923{(v2924*v9754)}else{(if v2910{((v2915*v9754)+(v2907*((v2913*(common.v45*v9754))+(v2911*(common.v40*v9754)))))}else{common.v2})})});
        let v9804=(if v2927{((-(common.v642*((v2933*v9779)+(v2928*((v2931*(common.v45*v9779))+(v2929*(common.v40*v9779)))))))/v9798)}else{(if v2923{(v2924*v9755)}else{(if v2910{((v2915*v9755)+(v2907*((v2913*(common.v45*v9755))+(v2911*(common.v40*v9755)))))}else{common.v2})})});
        let v9817=(if self.scalar_static_bool[43]{((v2940*(self.scalar_static_f64[20]*(self.scalar_static_f64[315]*v9803)))+(v2939*(v9742-v9726)))}else{common.v2});
        let v9818=(if self.scalar_static_bool[43]{((v2940*(self.scalar_static_f64[20]*(self.scalar_static_f64[315]*v9804)))+(v2939*(v9743-v9727)))}else{common.v2});
        let v9819=(if self.scalar_static_bool[47]{v9607}else{common.v2});
        let v9820=(if self.scalar_static_bool[47]{v9608}else{common.v2});
        let v9821=(v2946*v9819);
        let v9823=(v2946*v9820);
        let v9825=(common.v10*v2950);
        let v9834=(-v9819);
        let v9835=(-v9820);
        let v9836=(v2954*v9834);
        let v9838=(v2954*v9835);
        let v9840=(common.v10*v2958);
        let v9847=(v2959*v2959);
        let v9866=(if self.scalar_static_bool[47]{(self.scalar_static_f64[135]*v9659)}else{v9706});
        let v9867=(if self.scalar_static_bool[47]{(self.scalar_static_f64[135]*v9660)}else{v9707});
        let v9868=(-v9866);
        let v9869=(-v9867);
        let v9870=(v2971*v9868);
        let v9872=(v2971*v9869);
        let v9874=(common.v10*v2975);
        let v9883=(v2979*v9866);
        let v9885=(v2979*v9867);
        let v9887=(common.v10*v2983);
        let v9894=(v2984*v2984);
        let v9909=(if self.scalar_static_bool[48]{(if v2972{(-(common.v45*(v9868+((v9870+v9870)/v9874))))}else{(if v2980{(-((-(common.v418*(v9866+((v9883+v9883)/v9887))))/v9894))}else{(-(common.v45*v9868))})})}else{v9866});
        let v9910=(if self.scalar_static_bool[48]{(if v2972{(-(common.v45*(v9869+((v9872+v9872)/v9874))))}else{(if v2980{(-((-(common.v418*(v9867+((v9885+v9885)/v9887))))/v9894))}else{(-(common.v45*v9869))})})}else{v9867});
        let v9913=(v9708+(self.scalar_static_f64[175]*(if self.scalar_static_bool[47]{(if v2947{(v9819-(common.v45*(v9819+((v9821+v9821)/v9825))))}else{(if v2955{(v9819-((-(v2542*(v9834+((v9836+v9836)/v9840))))/v9847))}else{(v9819-(common.v45*v9819))})})}else{v9652})));
        let v9914=(v9709+(self.scalar_static_f64[175]*(if self.scalar_static_bool[47]{(if v2947{(v9820-(common.v45*(v9820+((v9823+v9823)/v9825))))}else{(if v2955{(v9820-((-(v2542*(v9835+((v9838+v9838)/v9840))))/v9847))}else{(v9820-(common.v45*v9820))})})}else{v9653})));
        let v9917=(if self.scalar_static_bool[50]{v9913}else{(if self.scalar_static_bool[49]{v9913}else{v9718})});
        let v9918=(if self.scalar_static_bool[50]{v9914}else{(if self.scalar_static_bool[49]{v9914}else{v9719})});
        let v9929=(if self.scalar_static_bool[47]{(v9917-self.scalar_static_f64[362])}else{v9734});
        let v9930=(if self.scalar_static_bool[47]{(v9918-self.scalar_static_f64[363])}else{v9735});
        let v9949=(if self.scalar_static_bool[47]{(self.scalar_static_f64[138]*((v3026*v9909)+(v2992*(self.scalar_static_f64[117]*v9909))))}else{v9754});
        let v9950=(if self.scalar_static_bool[47]{(self.scalar_static_f64[138]*((v3026*v9910)+(v2992*(self.scalar_static_f64[117]*v9910))))}else{v9755});
        let v9955=(-v9949);
        let v9956=(-v9950);
        let v9975=(v3049*v3049);
        let v10025=(if self.scalar_static_bool[54]{(self.scalar_static_f64[172]*(common.v5169-common.v8732))}else{common.v2});
        let v10026=(if self.scalar_static_bool[54]{(self.scalar_static_f64[172]*(common.v5170-common.v8733))}else{common.v2});
        let v10027=(if self.scalar_static_bool[54]{(self.scalar_static_f64[172]*common.v8762)}else{common.v2});
        let v10028=(self.scalar_static_f64[20]*v10025);
        let v10029=(self.scalar_static_f64[20]*v10026);
        let v10030=(self.scalar_static_f64[20]*v10027);
        let v10031=(if self.scalar_static_bool[56]{v10028}else{common.v2});
        let v10032=(if self.scalar_static_bool[56]{v10029}else{common.v2});
        let v10033=(if self.scalar_static_bool[56]{v10030}else{common.v2});
        let v10034=(-v10031);
        let v10035=(-v10032);
        let v10036=(-v10033);
        let v10037=(v3086*v10034);
        let v10039=(v3086*v10035);
        let v10041=(v3086*v10036);
        let v10043=(common.v10*v3090);
        let v10056=(v3085*v10031);
        let v10058=(v3085*v10032);
        let v10060=(v3085*v10033);
        let v10062=(common.v10*v3097);
        let v10071=(v3098*v3098);
        let v10094=(if self.scalar_static_bool[56]{(if v3087{(v10031+(common.v45*(v10034+((v10037+v10037)/v10043))))}else{(if v3094{(v10031+((-(v2542*(v10031+((v10056+v10056)/v10062))))/v10071))}else{(v10031+(common.v45*v10034))})})}else{common.v2});
        let v10095=(if self.scalar_static_bool[56]{(if v3087{(v10032+(common.v45*(v10035+((v10039+v10039)/v10043))))}else{(if v3094{(v10032+((-(v2542*(v10032+((v10058+v10058)/v10062))))/v10071))}else{(v10032+(common.v45*v10035))})})}else{common.v2});
        let v10096=(if self.scalar_static_bool[56]{(if v3087{(v10033+(common.v45*(v10036+((v10041+v10041)/v10043))))}else{(if v3094{(v10033+((-(v2542*(v10033+((v10060+v10060)/v10062))))/v10071))}else{(v10033+(common.v45*v10036))})})}else{common.v2});
        let v10097=(v3079*v10025);
        let v10099=(v3079*v10026);
        let v10101=(v3079*v10027);
        let v10103=(common.v10*v3109);
        let v10104=((v10097+v10097)/v10103);
        let v10105=((v10099+v10099)/v10103);
        let v10106=((v10101+v10101)/v10103);
        let v10110=(if self.scalar_static_bool[56]{(self.scalar_static_f64[136]*v10104)}else{common.v2});
        let v10111=(if self.scalar_static_bool[56]{(self.scalar_static_f64[136]*v10105)}else{common.v2});
        let v10112=(if self.scalar_static_bool[56]{(self.scalar_static_f64[136]*v10106)}else{common.v2});
        let v10113=(-v10110);
        let v10114=(-v10111);
        let v10115=(-v10112);
        let v10116=(v3113*v10113);
        let v10118=(v3113*v10114);
        let v10120=(v3113*v10115);
        let v10122=(common.v10*v3117);
        let v10135=(v3121*v10110);
        let v10137=(v3121*v10111);
        let v10139=(v3121*v10112);
        let v10141=(common.v10*v3125);
        let v10150=(v3126*v3126);
        let v10173=(if self.scalar_static_bool[57]{(if v3114{(-(common.v45*(v10113+((v10116+v10116)/v10122))))}else{(if v3122{(-((-(common.v418*(v10110+((v10135+v10135)/v10141))))/v10150))}else{(-(common.v45*v10113))})})}else{v10110});
        let v10174=(if self.scalar_static_bool[57]{(if v3114{(-(common.v45*(v10114+((v10118+v10118)/v10122))))}else{(if v3122{(-((-(common.v418*(v10111+((v10137+v10137)/v10141))))/v10150))}else{(-(common.v45*v10114))})})}else{v10111});
        let v10175=(if self.scalar_static_bool[57]{(if v3114{(-(common.v45*(v10115+((v10120+v10120)/v10122))))}else{(if v3122{(-((-(common.v418*(v10112+((v10139+v10139)/v10141))))/v10150))}else{(-(common.v45*v10115))})})}else{v10112});
        let v10176=(self.scalar_static_f64[20]*common.v8732);
        let v10177=(self.scalar_static_f64[20]*common.v8733);
        let v10178=(self.scalar_static_f64[20]*common.v8734);
        let v10185=(-(v10176+(self.scalar_static_f64[175]*v10094)));
        let v10186=(-(v10177+(self.scalar_static_f64[175]*v10095)));
        let v10187=(-(v10178+(self.scalar_static_f64[175]*v10096)));
        let v10191=(if self.scalar_static_bool[59]{v10185}else{(if self.scalar_static_bool[58]{v10185}else{common.v2})});
        let v10192=(if self.scalar_static_bool[59]{v10186}else{(if self.scalar_static_bool[58]{v10186}else{common.v2})});
        let v10193=(if self.scalar_static_bool[59]{v10187}else{(if self.scalar_static_bool[58]{v10187}else{common.v2})});
        let v10203=(if v3156{v10191}else{(if v3150{((v3151*v10191)/v3152)}else{common.v2})});
        let v10204=(if v3156{v10192}else{(if v3150{((v3151*v10192)/v3152)}else{common.v2})});
        let v10205=(if v3156{v10193}else{(if v3150{((v3151*v10193)/v3152)}else{common.v2})});
        let v10212=(if self.scalar_static_bool[56]{(v10191+self.scalar_static_f64[368])}else{common.v2});
        let v10213=(if self.scalar_static_bool[56]{(v10192+self.scalar_static_f64[369])}else{common.v2});
        let v10214=(if self.scalar_static_bool[56]{v10193}else{common.v2});
        let v10224=(if v3170{v10212}else{(if v3164{((v3165*v10212)/v3166)}else{common.v2})});
        let v10225=(if v3170{v10213}else{(if v3164{((v3165*v10213)/v3166)}else{common.v2})});
        let v10226=(if v3170{v10214}else{(if v3164{((v3165*v10214)/v3166)}else{common.v2})});
        let v10242=(if self.scalar_static_bool[56]{(self.scalar_static_f64[139]*((v3173*v10173)+(v3134*(self.scalar_static_f64[124]*v10173))))}else{common.v2});
        let v10243=(if self.scalar_static_bool[56]{(self.scalar_static_f64[139]*((v3173*v10174)+(v3134*(self.scalar_static_f64[124]*v10174))))}else{common.v2});
        let v10244=(if self.scalar_static_bool[56]{(self.scalar_static_f64[139]*((v3173*v10175)+(v3134*(self.scalar_static_f64[124]*v10175))))}else{common.v2});
        let v10278=(-v10242);
        let v10279=(-v10243);
        let v10280=(-v10244);
        let v10307=(v3205*v3205);
        let v10315=(if v3197{((-(common.v642*((v3203*v10278)+(v3198*((v3201*(common.v45*v10278))+(v3199*(common.v40*v10278)))))))/v10307)}else{(if v3193{(v3194*v10242)}else{(if v3180{((v3185*v10242)+(v3177*((v3183*(common.v45*v10242))+(v3181*(common.v40*v10242)))))}else{common.v2})})});
        let v10316=(if v3197{((-(common.v642*((v3203*v10279)+(v3198*((v3201*(common.v45*v10279))+(v3199*(common.v40*v10279)))))))/v10307)}else{(if v3193{(v3194*v10243)}else{(if v3180{((v3185*v10243)+(v3177*((v3183*(common.v45*v10243))+(v3181*(common.v40*v10243)))))}else{common.v2})})});
        let v10317=(if v3197{((-(common.v642*((v3203*v10280)+(v3198*((v3201*(common.v45*v10280))+(v3199*(common.v40*v10280)))))))/v10307)}else{(if v3193{(v3194*v10244)}else{(if v3180{((v3185*v10244)+(v3177*((v3183*(common.v45*v10244))+(v3181*(common.v40*v10244)))))}else{common.v2})})});
        let v10336=(if self.scalar_static_bool[56]{((v3210*(self.scalar_static_f64[20]*(self.scalar_static_f64[314]*v10315)))+(v3209*(v10224-v10203)))}else{common.v2});
        let v10337=(if self.scalar_static_bool[56]{((v3210*(self.scalar_static_f64[20]*(self.scalar_static_f64[314]*v10316)))+(v3209*(v10225-v10204)))}else{common.v2});
        let v10338=(if self.scalar_static_bool[56]{((v3210*(self.scalar_static_f64[20]*(self.scalar_static_f64[314]*v10317)))+(v3209*(v10226-v10205)))}else{common.v2});
        let v10339=(if self.scalar_static_bool[60]{v10028}else{common.v2});
        let v10340=(if self.scalar_static_bool[60]{v10029}else{common.v2});
        let v10341=(if self.scalar_static_bool[60]{v10030}else{common.v2});
        let v10342=(v3216*v10339);
        let v10344=(v3216*v10340);
        let v10346=(v3216*v10341);
        let v10348=(common.v10*v3220);
        let v10361=(-v10339);
        let v10362=(-v10340);
        let v10363=(-v10341);
        let v10364=(v3224*v10361);
        let v10366=(v3224*v10362);
        let v10368=(v3224*v10363);
        let v10370=(common.v10*v3228);
        let v10379=(v3229*v3229);
        let v10408=(if self.scalar_static_bool[60]{(self.scalar_static_f64[135]*v10104)}else{v10173});
        let v10409=(if self.scalar_static_bool[60]{(self.scalar_static_f64[135]*v10105)}else{v10174});
        let v10410=(if self.scalar_static_bool[60]{(self.scalar_static_f64[135]*v10106)}else{v10175});
        let v10411=(-v10408);
        let v10412=(-v10409);
        let v10413=(-v10410);
        let v10414=(v3241*v10411);
        let v10416=(v3241*v10412);
        let v10418=(v3241*v10413);
        let v10420=(common.v10*v3245);
        let v10433=(v3249*v10408);
        let v10435=(v3249*v10409);
        let v10437=(v3249*v10410);
        let v10439=(common.v10*v3253);
        let v10448=(v3254*v3254);
        let v10471=(if self.scalar_static_bool[61]{(if v3242{(-(common.v45*(v10411+((v10414+v10414)/v10420))))}else{(if v3250{(-((-(common.v418*(v10408+((v10433+v10433)/v10439))))/v10448))}else{(-(common.v45*v10411))})})}else{v10408});
        let v10472=(if self.scalar_static_bool[61]{(if v3242{(-(common.v45*(v10412+((v10416+v10416)/v10420))))}else{(if v3250{(-((-(common.v418*(v10409+((v10435+v10435)/v10439))))/v10448))}else{(-(common.v45*v10412))})})}else{v10409});
        let v10473=(if self.scalar_static_bool[61]{(if v3242{(-(common.v45*(v10413+((v10418+v10418)/v10420))))}else{(if v3250{(-((-(common.v418*(v10410+((v10437+v10437)/v10439))))/v10448))}else{(-(common.v45*v10413))})})}else{v10410});
        let v10477=(v10176+(self.scalar_static_f64[175]*(if self.scalar_static_bool[60]{(if v3217{(v10339-(common.v45*(v10339+((v10342+v10342)/v10348))))}else{(if v3225{(v10339-((-(v2542*(v10361+((v10364+v10364)/v10370))))/v10379))}else{(v10339-(common.v45*v10339))})})}else{v10094})));
        let v10478=(v10177+(self.scalar_static_f64[175]*(if self.scalar_static_bool[60]{(if v3217{(v10340-(common.v45*(v10340+((v10344+v10344)/v10348))))}else{(if v3225{(v10340-((-(v2542*(v10362+((v10366+v10366)/v10370))))/v10379))}else{(v10340-(common.v45*v10340))})})}else{v10095})));
        let v10479=(v10178+(self.scalar_static_f64[175]*(if self.scalar_static_bool[60]{(if v3217{(v10341-(common.v45*(v10341+((v10346+v10346)/v10348))))}else{(if v3225{(v10341-((-(v2542*(v10363+((v10368+v10368)/v10370))))/v10379))}else{(v10341-(common.v45*v10341))})})}else{v10096})));
        let v10483=(if self.scalar_static_bool[63]{v10477}else{(if self.scalar_static_bool[62]{v10477}else{v10191})});
        let v10484=(if self.scalar_static_bool[63]{v10478}else{(if self.scalar_static_bool[62]{v10478}else{v10192})});
        let v10485=(if self.scalar_static_bool[63]{v10479}else{(if self.scalar_static_bool[62]{v10479}else{v10193})});
        let v10500=(if self.scalar_static_bool[60]{(v10483-self.scalar_static_f64[368])}else{v10212});
        let v10501=(if self.scalar_static_bool[60]{(v10484-self.scalar_static_f64[369])}else{v10213});
        let v10502=(if self.scalar_static_bool[60]{v10485}else{v10214});
        let v10530=(if self.scalar_static_bool[60]{(self.scalar_static_f64[137]*((v3296*v10471)+(v3262*(self.scalar_static_f64[117]*v10471))))}else{v10242});
        let v10531=(if self.scalar_static_bool[60]{(self.scalar_static_f64[137]*((v3296*v10472)+(v3262*(self.scalar_static_f64[117]*v10472))))}else{v10243});
        let v10532=(if self.scalar_static_bool[60]{(self.scalar_static_f64[137]*((v3296*v10473)+(v3262*(self.scalar_static_f64[117]*v10473))))}else{v10244});
        let v10539=(-v10530);
        let v10540=(-v10531);
        let v10541=(-v10532);
        let v10568=(v3319*v3319);

        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (common.v1642),
            6,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[285]*(ctx.node_voltage(nodes[0])-common.v3348))}else{common.v2})),
            0,
            multiplicity * (self.scalar_static_f64[371]),
            3,
            multiplicity * (self.scalar_static_f64[372]),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * ((if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[286]*(common.v3348-common.v375))}else{common.v2})),
            3,
            multiplicity * (self.scalar_static_f64[374]),
            4,
            multiplicity * (self.scalar_static_f64[375]),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(1),
            multiplicity * ((if (self.scalar_static_f64[59]!=0.0){(v3359*v3360)}else{common.v2})),
            [1, 4, 5, 6],
            [(if (self.scalar_static_f64[59]!=0.0){(-v3360)}else{common.v2}), (if (self.scalar_static_f64[59]!=0.0){(v3359*(((v2570*(self.scalar_static_f64[289]*((v2558*((self.scalar_static_f64[172]*(v2555*(-(if v2532{(-(common.v45*(v9086+((v9088+v9088)/v9092))))}else{(if v2541{(-((-(v2542*(common.v6027+((v9101+v9101)/v9105))))/v9112))}else{(-(common.v45*v9086))})}))))/v9133))+(v2557*((common.v2530*common.v3506)+(common.v455*common.v9083))))))-(v2567*(self.scalar_static_f64[157]*(common.v45*(self.scalar_static_f64[150]+((v9150+v9150)/v9154))))))/v9169))}else{common.v2}), (if (self.scalar_static_f64[59]!=0.0){(v3360+(v3359*(((v2570*(self.scalar_static_f64[289]*((v2558*((self.scalar_static_f64[172]*(v2555*(-(if v2532{(-(common.v45*(v9087+((v9090+v9090)/v9092))))}else{(if v2541{(-((-(v2542*(common.v6028+((v9103+v9103)/v9105))))/v9112))}else{(-(common.v45*v9087))})}))))/v9133))+(v2557*((common.v2530*common.v3507)+(common.v455*common.v9084))))))-(v2567*(self.scalar_static_f64[157]*(common.v45*(self.scalar_static_f64[20]+((v9152+v9152)/v9154))))))/v9169)))}else{common.v2}), (if (self.scalar_static_f64[59]!=0.0){(v3359*((self.scalar_static_f64[289]*(v2557*(common.v455*common.v9085)))/v2570))}else{common.v2})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((if (self.scalar_static_f64[59]!=0.0){(self.scalar_static_f64[287]*(common.v2577-ctx.node_voltage(nodes[2])))}else{common.v2})),
            1,
            multiplicity * (self.scalar_static_f64[377]),
            2,
            multiplicity * (self.scalar_static_f64[378]),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v2,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(4),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v2,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(1),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            common.v2,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(2),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            common.v2,
        );
        stamper.stamp_current_node3_local(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[60]{(v3212+(v3335*v3336))}else{v3212}))),
            4,
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[60]{(v10336+((v3336*(self.scalar_static_f64[20]*(self.scalar_static_f64[312]*(if v3323{(common.v567*((v3329*v10530)+(v3324*((v3327*(common.v45*v10530))+(v3325*(common.v40*v10530))))))}else{(if v3311{((-(common.v642*((v3317*v10539)+(v3312*((v3315*(common.v45*v10539))+(v3313*(common.v40*v10539)))))))/v10568)}else{(if v3304{(v3305*v10530)}else{v10315})})}))))+(v3335*((if v3281{v10483}else{(if v3275{((v3276*v10483)/v3277)}else{v10203})})-(if v3293{v10500}else{(if v3287{((v3288*v10500)/v3289)}else{v10224})})))))}else{v10336}))),
            5,
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[60]{(v10337+((v3336*(self.scalar_static_f64[20]*(self.scalar_static_f64[312]*(if v3323{(common.v567*((v3329*v10531)+(v3324*((v3327*(common.v45*v10531))+(v3325*(common.v40*v10531))))))}else{(if v3311{((-(common.v642*((v3317*v10540)+(v3312*((v3315*(common.v45*v10540))+(v3313*(common.v40*v10540)))))))/v10568)}else{(if v3304{(v3305*v10531)}else{v10316})})}))))+(v3335*((if v3281{v10484}else{(if v3275{((v3276*v10484)/v3277)}else{v10204})})-(if v3293{v10501}else{(if v3287{((v3288*v10501)/v3289)}else{v10225})})))))}else{v10337}))),
            6,
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[60]{(v10338+((v3336*(self.scalar_static_f64[20]*(self.scalar_static_f64[312]*(if v3323{(common.v567*((v3329*v10532)+(v3324*((v3327*(common.v45*v10532))+(v3325*(common.v40*v10532))))))}else{(if v3311{((-(common.v642*((v3317*v10541)+(v3312*((v3315*(common.v45*v10541))+(v3313*(common.v40*v10541)))))))/v10568)}else{(if v3304{(v3305*v10532)}else{v10317})})}))))+(v3335*((if v3281{v10485}else{(if v3275{((v3276*v10485)/v3277)}else{v10205})})-(if v3293{v10502}else{(if v3287{((v3288*v10502)/v3289)}else{v10226})})))))}else{v10338}))),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(1),
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[47]{(v2942+(v3065*v3066))}else{v2942}))),
            1,
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[47]{(v9817+((v3066*(self.scalar_static_f64[20]*(self.scalar_static_f64[313]*(if v3053{(common.v567*((v3059*v9949)+(v3054*((v3057*(common.v45*v9949))+(v3055*(common.v40*v9949))))))}else{(if v3041{((-(common.v642*((v3047*v9955)+(v3042*((v3045*(common.v45*v9955))+(v3043*(common.v40*v9955)))))))/v9975)}else{(if v3034{(v3035*v9949)}else{v9803})})}))))+(v3065*((if v3011{v9917}else{(if v3005{((v3006*v9917)/v3007)}else{v9726})})-(if v3023{v9929}else{(if v3017{((v3018*v9929)/v3019)}else{v9742})})))))}else{v9817}))),
            4,
            multiplicity * ((self.scalar_static_f64[20]*(if self.scalar_static_bool[47]{(v9818+((v3066*(self.scalar_static_f64[20]*(self.scalar_static_f64[313]*(if v3053{(common.v567*((v3059*v9950)+(v3054*((v3057*(common.v45*v9950))+(v3055*(common.v40*v9950))))))}else{(if v3041{((-(common.v642*((v3047*v9956)+(v3042*((v3045*(common.v45*v9956))+(v3043*(common.v40*v9956)))))))/v9975)}else{(if v3034{(v3035*v9950)}else{v9804})})}))))+(v3065*((if v3011{v9918}else{(if v3005{((v3006*v9918)/v3007)}else{v9727})})-(if v3023{v9930}else{(if v3017{((v3018*v9930)/v3019)}else{v9743})})))))}else{v9818}))),
        );
        let v3345_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v3345);
        stamper.stamp_current_node3_local(
            Some(4),
            Some(5),
            multiplicity * (v3345_ddt),
            4,
            multiplicity * (((common.v10654) * ddt_scale)),
            5,
            multiplicity * (((common.v10655) * ddt_scale)),
            6,
            multiplicity * (((common.v10656) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * ((if common.v1545{(v1637/v1639)}else{common.v2})),
            4,
            multiplicity * ((if common.v1545{(((v1639*((v1636*v6200)+(v1619*(self.scalar_static_f64[172]*common.v3511))))-(v1637*((if common.v1545{((v1633*common.v3508)+(common.v456*((v6200+common.v6219)/v6228)))}else{common.v2})+((common.v1631*common.v3508)+(common.v456*common.v6224)))))/v6258)}else{common.v2})),
            5,
            multiplicity * ((if common.v1545{(((v1639*((v1636*v6201)+(v1619*(self.scalar_static_f64[172]*common.v3513))))-(v1637*((if common.v1545{((v1633*common.v3509)+(common.v456*((v6201+common.v6220)/v6228)))}else{common.v2})+((common.v1631*common.v3509)+(common.v456*common.v6225)))))/v6258)}else{common.v2})),
        );
        let v3347_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v3347);
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v3347_ddt),
            6,
            multiplicity * (((self.scalar_static_f64[160]) * ddt_scale)),
        );
        let v3350_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v3350);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(1),
            multiplicity * (v3350_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[163]) * ddt_scale)),
            3,
            multiplicity * (((self.scalar_static_f64[58]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(1),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(2),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (common.v2),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_node3(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (common.v10654),
            nodes[5],
            multiplicity * (common.v10655),
            nodes[6],
            multiplicity * (common.v10656),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (self.scalar_static_f64[160]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[163]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[58]),
        );
    }
}
