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
    v8: f64,
    v21: f64,
    v37: f64,
    v42: f64,
    v68: f64,
    v94: f64,
    v123: f64,
    v141: f64,
    v160: f64,
    v164: f64,
    v170: f64,
    v370: f64,
    v371: f64,
    v372: f64,
    v376: f64,
    v405: f64,
    v413: f64,
    v450: f64,
    v451: f64,
    v452: f64,
    v458: f64,
    v473: f64,
    v477: f64,
    v482: f64,
    v503: f64,
    v507: f64,
    v551: f64,
    v558: f64,
    v625: f64,
    v632: f64,
    v1183: f64,
    v1510: f64,
    v1516: bool,
    v1518: bool,
    v1522: f64,
    v1533: bool,
    v1536: f64,
    v1544: bool,
    v1569: bool,
    v1597: f64,
    v1599: f64,
    v1610: f64,
    v2339: f64,
    v2474: f64,
    v2520: f64,
    v3259: f64,
    v3261: f64,
    v3262: f64,
    v3264: f64,
    v3420: f64,
    v3421: f64,
    v3422: f64,
    v3423: f64,
    v3425: f64,
    v3427: f64,
    v3440: f64,
    v3441: f64,
    v3471: f64,
    v3472: f64,
    v5083: f64,
    v5084: f64,
    v5941: f64,
    v5942: f64,
    v5952: f64,
    v5953: f64,
    v5981: f64,
    v5982: f64,
    v6133: f64,
    v6134: f64,
    v6138: f64,
    v6139: f64,
    v8646: f64,
    v8647: f64,
    v8648: f64,
    v8676: f64,
    v8997: f64,
    v8998: f64,
    v8999: f64,
    v10568: f64,
    v10569: f64,
    v10570: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v8=2.0;
        let v10=3.348580862e-29;
        let v21=0.0;
        let v25=0.6666666666666666;
        let v37=0.3333333333333333;
        let v42=0.5;
        let v68=1.0;
        let v94=4.0;
        let v123=0.001;
        let v141=6.0;
        let v148=0.7071067811865475;
        let v152=1e-5;
        let v160=1.25;
        let v161=0.7324648775608221;
        let v164=460.51701859880916;
        let v170=1e-200;
        let v297=1.3333333333333333;
        let v370=ctx.node_voltage(nodes[4]);
        let v371=ctx.node_voltage(nodes[5]);
        let v372=(v370-v371);
        let v375=(self.scalar_static_f64[19]*(v372-self.scalar_static_f64[138]));
        let v376=1e-16;
        let v377=(v375>v376);
        let v381=(((v375*v375)+self.scalar_static_f64[139])).sqrt();
        let v384=(v21-v375);
        let v385=(v384>v376);
        let v389=((self.scalar_static_f64[139]+(v384*v384))).sqrt();
        let v390=(v384+v389);
        let v400=(v68+(self.scalar_static_f64[137]*(if v377{(v42*(v375+v381))}else{(if v385{(self.scalar_static_f64[140]/v390)}else{(v42*(v375+self.scalar_static_f64[142]))})})));
        let v402=(self.scalar_static_f64[143]-v400);
        let v403=(v402>v376);
        let v405=1e-6;
        let v407=(((v402*v402)+v405)).sqrt();
        let v411=(v400-self.scalar_static_f64[143]);
        let v412=(v411>v376);
        let v413=5e-7;
        let v416=((v405+(v411*v411))).sqrt();
        let v417=(v411+v416);
        let v425=(self.scalar_static_f64[5]*(if v403{(self.scalar_static_f64[143]-(v42*(v402+v407)))}else{(if v412{(self.scalar_static_f64[143]-(v413/v417))}else{(self.scalar_static_f64[143]-(v42*(v123+v402)))})}));
        let v426=1e23;
        let v428=(self.scalar_static_f64[199]*v425);
        let v431=(self.scalar_static_f64[184]+(self.scalar_static_f64[200]*(v428).ln()));
        let v433=((v10*v425)).sqrt();
        let v434=(v433/self.scalar_static_f64[4]);
        let v435=(v434*v434);
        let v437=((v431*v435)).sqrt();
        let v438=(if self.scalar_static_bool[0]{v437}else{v21});
        let v443=(if self.scalar_static_bool[0]{(self.scalar_static_f64[144]*f64::powf(v438,v25))}else{v21});
        let v446=(v297*v443);
        let v448=(v68+(v446/v438));
        let v450=(if self.scalar_static_bool[0]{(v434*v448)}else{v434});
        let v451=(self.scalar_static_f64[211]*v450);
        let v452=(v451*v451);
        let v453=(v68/v452);
        let v455=(v68+(v148*v451));
        let v456=(v68/v455);
        let v457=(v152*v455);
        let v458=(self.scalar_static_f64[168]*(if self.scalar_static_bool[0]{(v431+v443)}else{v431}));
        let v459=(v458<v164);
        let v461=((-v458)).exp();
        let v463=(!v459);
        let v464=(v458-v164);
        let v465=(v42*v464);
        let v467=(v68+(v37*v464));
        let v469=(v68+(v465*v467));
        let v471=(v68+(v464*v469));
        let v473=(if v463{(v170/v471)}else{(if v459{v461}else{v21})});
        let v475=(v160+(v161*v451));
        let v477=(self.scalar_static_f64[19]*(v372-self.scalar_static_f64[170]));
        let v478=(self.scalar_static_f64[168]*v477);
        let v480=((v478).abs()<=v457);
        let v482=0.1666666666666667;
        let v484=(v148*((v456*v456)*v482));
        let v485=(if v480{v484}else{v21});
        let v486=(v456*v478);
        let v487=(v68-v473);
        let v488=(v478*v487);
        let v489=(v451*v488);
        let v491=(v68+(v485*v489));
        let v494=(-v457);
        let v495=(v478<v494);
        let v496=(!v480);
        let v497=(v495&&v496);
        let v499=(if v497{(-v478)}else{v21});
        let v500=(v160*v499);
        let v502=(if v497{(v456*v500)}else{v21});
        let v503=10.0;
        let v505=(v502-v141);
        let v507=64.0;
        let v509=(((v505*v505)+v507)).sqrt();
        let v512=(if v497{(v42*((v502+v503)-v509))}else{v21});
        let v514=(if v497{(v499-v512)}else{v21});
        let v516=(v68+v512);
        let v519=(if v497{((v514*v514)+(v452*v516))}else{v21});
        let v522=(if v497{((v8*v514)-v452)}else{v21});
        let v524=(v453*v519);
        let v527=(if v497{((-v512)+(v524).ln())}else{v21});
        let v529=(if v497{(v519+v522)}else{v21});
        let v531=(v42*v522);
        let v533=((v522*v531)-v519);
        let v536=(if v497{((v529*v529)+(v527*v533))}else{v21});
        let v537=(v519*v529);
        let v538=(v527*v537);
        let v539=(v527*v529);
        let v540=(v527*v539);
        let v541=(v540/v536);
        let v542=(v522*v541);
        let v545=((v37*(v522*v522))-v519);
        let v547=(v536+(v542*v545));
        let v550=(if v497{(v512+(v538/v547))}else{v21});
        let v551=230.25850929940458;
        let v552=(v550<v551);
        let v553=(v497&&v552);
        let v554=(v550).exp();
        let v557=(v497&&(!v552));
        let v558=1e100;
        let v559=(v550-v551);
        let v560=(v42*v559);
        let v562=(v68+(v37*v559));
        let v564=(v68+(v560*v562));
        let v568=(if v557{(v558*(v68+(v559*v564)))}else{(if v553{v554}else{v21})});
        let v570=(if v497{(v68/v568)}else{v21});
        let v572=(v8+(v550*v550));
        let v576=(if v497{(v499-v550)}else{(if v497{(v68/v572)}else{v514})});
        let v578=(if v497{(v473*v570)}else{v485});
        let v582=(v473+((v568-v68)-v578));
        let v585=(if v497{((v8*v576)+(v452*v582))}else{v21});
        let v590=(v550-v68);
        let v592=((v578+((v568-v550)-v68))+(v473*v590));
        let v595=(if v497{((v576*v576)-(v452*v592))}else{v21});
        let v596=(v568+v578);
        let v599=(if v497{(v8-(v452*v596))}else{v576});
        let v601=(v8*v595);
        let v604=(if v497{((v585*v585)-(v599*v601))}else{v599});
        let v606=(v604).sqrt();
        let v607=(v585+v606);
        let v612=(v496&&(!v495));
        let v613=(v68/v475);
        let v614=(if v612{v613}else{v21});
        let v615=(v160*v455);
        let v617=((v614*v615)-v68);
        let v619=(if v612{(v614*v617)}else{v21});
        let v621=(v68+(v478*v619));
        let v624=(-(if v612{(v486*v621)}else{v21}));
        let v625=-230.25850929940458;
        let v626=(v624>v625);
        let v627=(v612&&v626);
        let v628=(v624).exp();
        let v631=(v612&&(!v626));
        let v632=1e-100;
        let v633=(v625-v624);
        let v634=(v42*v633);
        let v636=(v68+(v37*v633));
        let v638=(v68+(v634*v636));
        let v640=(v68+(v633*v638));
        let v642=(if v631{(v632/v640)}else{(if v627{v628}else{v604})});
        let v645=(v42*v452);
        let v647=0.25;
        let v648=(v452*v647);
        let v651=(((v478+v648)-(if v612{(v68-v642)}else{v21}))).sqrt();
        let v654=(if v612{((v478+v645)-(v451*v651))}else{v21});
        let v655=(3.0+v458);
        let v656=(if v612{v655}else{v21});
        let v657=(v656-v654);
        let v658=(v657>v376);
        let v660=5.0;
        let v662=(((v657*v657)+v660)).sqrt();
        let v666=(v654-v656);
        let v667=(v666>v376);
        let v668=2.5;
        let v671=((v660+(v666*v666))).sqrt();
        let v672=(v666+v671);
        let v675=2.23606797749979;
        let v683=((v660+(v656*v656))).sqrt();
        let v687=(if v612{((if v658{(v656-(v42*(v657+v662)))}else{(if v667{(v656-(v668/v672))}else{(v656-(v42*(v657+v675)))})})-(v42*(v656-v683)))}else{v512});
        let v689=(if v612{(v478-v687)}else{v642});
        let v691=((-v687)).exp();
        let v692=(if v612{v691}else{v578});
        let v693=1e-40;
        let v697=(v68+v687);
        let v699=(((v687+v692)-v68)-(v473*v697));
        let v701=((v689*v689)-(v452*v699));
        let v702=(v693>v701);
        let v704=(if v612{(if v702{v693}else{v701})}else{v519});
        let v707=(if v612{(v68-(v645*v692))}else{v21});
        let v710=((v68-v692)-v473);
        let v713=(if v612{((v8*v689)+(v452*v710))}else{v522});
        let v715=(v704/v452);
        let v718=(if v612{((v458-v687)+(v715).ln())}else{v527});
        let v720=(if v612{(v704+v713)}else{v21});
        let v722=1e-120;
        let v723=((v718).abs()<v722);
        let v724=(v612&&v723);
        let v727=(v612&&(!v723));
        let v729=(v42*v713);
        let v731=(v704*v707);
        let v732=((v713*v729)-v731);
        let v735=(if v727{((v720*v720)+(v718*v732))}else{v21});
        let v736=(v704*v720);
        let v737=(v718*v736);
        let v738=(v718*v720);
        let v739=(v718*v738);
        let v740=(v739/v735);
        let v741=(v713*v740);
        let v744=((v37*(v713*v713))-v731);
        let v746=(v735+(v741*v744));
        let v749=(if v727{(v687+(v737/v746))}else{(if v724{v687}else{v21})});
        let v750=(v749<v551);
        let v751=(v612&&v750);
        let v752=(v749).exp();
        let v753=(if v751{v752}else{v568});
        let v758=(v458-v551);
        let v759=(v749>v758);
        let v761=(v612&&(!v750));
        let v762=(v759&&v761);
        let v764=((v749-v458)).exp();
        let v765=(if v762{v764}else{(if v751{(v473*v753)}else{v753})});
        let v769=(v761&&(!v759));
        let v771=((v458-v749)-v551);
        let v772=(v42*v771);
        let v774=(v68+(v37*v771));
        let v776=(v68+(v772*v774));
        let v778=(v68+(v771*v776));
        let v780=(if v769{(v632/v778)}else{v765});
        let v781=(v749-v551);
        let v782=(v42*v781);
        let v784=(v68+(v37*v781));
        let v786=(v68+(v782*v784));
        let v788=(v68+(v781*v786));
        let v790=(if v769{(v632/v788)}else{(if v762{(v473/v765)}else{(if v751{(v68/v753)}else{v570})})});
        let v792=(v8+(v749*v749));
        let v796=(if v612{(v478-v749)}else{(if v612{(v68/v792)}else{v689})});
        let v800=((v780+(v68-v790))-v473);
        let v803=(if v612{((v8*v796)+(v452*v800))}else{v585});
        let v808=(v68+v749);
        let v810=((v780+((v749+v790)-v68))-(v473*v808));
        let v814=(v780+v790);
        let v817=(if v612{(v8-(v452*v814))}else{v796});
        let v819=(v8*(if v612{((v796*v796)-(v452*v810))}else{v595}));
        let v823=((if v612{((v803*v803)-(v817*v819))}else{v817})).sqrt();
        let v824=(v803+v823);
        let v827=(if v612{(v749+(v819/v824))}else{(if v497{((-v550)-(v601/v607))}else{(if v480{(v486*v491)}else{v21})})});
        let v837=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(v477-(self.scalar_static_f64[165]*v827))))}else{v21});
        let v839=((v837).abs()<=self.scalar_static_f64[218]);
        let v840=(self.scalar_static_bool[16]&&v839);
        let v844=(if v840{self.scalar_static_f64[314]}else{v21});
        let v845=(self.scalar_static_f64[217]*v837);
        let v850=(v68+(v844*(self.scalar_static_f64[212]*(v837*self.scalar_static_f64[315]))));
        let v854=(v837<self.scalar_static_f64[316]);
        let v856=(self.scalar_static_bool[16]&&(!v839));
        let v857=(v854&&v856);
        let v859=(if v857{(-v837)}else{v21});
        let v862=(if v857{(self.scalar_static_f64[217]*(v160*v859))}else{v21});
        let v864=(v862-v141);
        let v867=((v507+(v864*v864))).sqrt();
        let v870=(if v857{(v42*((v503+v862)-v867))}else{v21});
        let v872=(if v857{(v859-v870)}else{v21});
        let v877=(if v857{((v872*v872)+(self.scalar_static_f64[213]*(v68+v870)))}else{v21});
        let v880=(if v857{((v8*v872)-self.scalar_static_f64[213])}else{v21});
        let v882=(self.scalar_static_f64[214]*v877);
        let v885=(if v857{((-v870)+(v882).ln())}else{v21});
        let v887=(if v857{(v877+v880)}else{v21});
        let v889=(v42*v880);
        let v891=((v880*v889)-v877);
        let v894=(if v857{((v887*v887)+(v885*v891))}else{v21});
        let v895=(v877*v887);
        let v896=(v885*v895);
        let v897=(v885*v887);
        let v898=(v885*v897);
        let v899=(v898/v894);
        let v900=(v880*v899);
        let v903=((v37*(v880*v880))-v877);
        let v905=(v894+(v900*v903));
        let v908=(if v857{(v870+(v896/v905))}else{v21});
        let v909=(v908<v551);
        let v910=(v857&&v909);
        let v911=(v908).exp();
        let v914=(v857&&(!v909));
        let v915=(v908-v551);
        let v916=(v42*v915);
        let v918=(v68+(v37*v915));
        let v920=(v68+(v916*v918));
        let v924=(if v914{(v558*(v68+(v915*v920)))}else{(if v910{v911}else{v21})});
        let v926=(if v857{(v68/v924)}else{v21});
        let v928=(v8+(v908*v908));
        let v932=(if v857{(v859-v908)}else{(if v857{(v68/v928)}else{v872})});
        let v934=(if v857{(self.scalar_static_f64[239]*v926)}else{v844});
        let v941=(if v857{((v8*v932)+(self.scalar_static_f64[213]*(self.scalar_static_f64[239]+((v924-v68)-v934))))}else{v21});
        let v951=(if v857{((v932*v932)-(self.scalar_static_f64[213]*((v934+((v924-v908)-v68))+(self.scalar_static_f64[239]*(v908-v68)))))}else{v21});
        let v955=(if v857{(v8-(self.scalar_static_f64[213]*(v924+v934)))}else{v932});
        let v957=(v8*v951);
        let v960=(if v857{((v941*v941)-(v955*v957))}else{v955});
        let v962=(v960).sqrt();
        let v963=(v941+v962);
        let v968=(v856&&(!v854));
        let v972=(if v968{self.scalar_static_f64[319]}else{v21});
        let v977=(if v968{(v972*((v972*self.scalar_static_f64[320])-v68))}else{v21});
        let v979=(v68+(v837*v977));
        let v982=(-(if v968{(v845*v979)}else{v21}));
        let v983=(v982>v625);
        let v984=(v968&&v983);
        let v985=(v982).exp();
        let v988=(v968&&(!v983));
        let v989=(v625-v982);
        let v990=(v42*v989);
        let v992=(v68+(v37*v989));
        let v994=(v68+(v990*v992));
        let v996=(v68+(v989*v994));
        let v998=(if v988{(v632/v996)}else{(if v984{v985}else{v960})});
        let v1006=(((v837+self.scalar_static_f64[322])-(if v968{(v68-v998)}else{v21}))).sqrt();
        let v1009=(if v968{((v837+self.scalar_static_f64[321])-(self.scalar_static_f64[212]*v1006))}else{v21});
        let v1011=(if v968{self.scalar_static_f64[323]}else{v21});
        let v1012=(v1011-v1009);
        let v1013=(v1012>v376);
        let v1016=((v660+(v1012*v1012))).sqrt();
        let v1020=(v1009-v1011);
        let v1021=(v1020>v376);
        let v1024=((v660+(v1020*v1020))).sqrt();
        let v1025=(v1020+v1024);
        let v1039=(if v968{((if v1013{(v1011-(v42*(v1012+v1016)))}else{(if v1021{(v1011-(v668/v1025))}else{(v1011-(v42*(v675+v1012)))})})-(v42*(v1011-((v660+(v1011*v1011))).sqrt())))}else{v870});
        let v1041=(if v968{(v837-v1039)}else{v998});
        let v1043=((-v1039)).exp();
        let v1044=(if v968{v1043}else{v934});
        let v1052=((v1041*v1041)-(self.scalar_static_f64[213]*(((v1039+v1044)-v68)-(self.scalar_static_f64[239]*(v68+v1039)))));
        let v1053=(v693>v1052);
        let v1055=(if v968{(if v1053{v693}else{v1052})}else{v877});
        let v1058=(if v968{(v68-(self.scalar_static_f64[321]*v1044))}else{v21});
        let v1064=(if v968{((v8*v1041)+(self.scalar_static_f64[213]*((v68-v1044)-self.scalar_static_f64[239])))}else{v880});
        let v1066=(v1055/self.scalar_static_f64[213]);
        let v1069=(if v968{((self.scalar_static_f64[219]-v1039)+(v1066).ln())}else{v885});
        let v1071=(if v968{(v1055+v1064)}else{v21});
        let v1073=((v1069).abs()<v722);
        let v1074=(v968&&v1073);
        let v1077=(v968&&(!v1073));
        let v1079=(v42*v1064);
        let v1081=(v1055*v1058);
        let v1082=((v1064*v1079)-v1081);
        let v1085=(if v1077{((v1071*v1071)+(v1069*v1082))}else{v21});
        let v1086=(v1055*v1071);
        let v1087=(v1069*v1086);
        let v1088=(v1069*v1071);
        let v1089=(v1069*v1088);
        let v1090=(v1089/v1085);
        let v1091=(v1064*v1090);
        let v1094=((v37*(v1064*v1064))-v1081);
        let v1096=(v1085+(v1091*v1094));
        let v1099=(if v1077{(v1039+(v1087/v1096))}else{(if v1074{v1039}else{v21})});
        let v1100=(v1099<v551);
        let v1101=(v968&&v1100);
        let v1102=(v1099).exp();
        let v1103=(if v1101{v1102}else{v924});
        let v1109=(v1099>self.scalar_static_f64[324]);
        let v1111=(v968&&(!v1100));
        let v1112=(v1109&&v1111);
        let v1114=((v1099-self.scalar_static_f64[219])).exp();
        let v1115=(if v1112{v1114}else{(if v1101{(self.scalar_static_f64[239]*v1103)}else{v1103})});
        let v1119=(v1111&&(!v1109));
        let v1121=((self.scalar_static_f64[219]-v1099)-v551);
        let v1122=(v42*v1121);
        let v1124=(v68+(v37*v1121));
        let v1126=(v68+(v1122*v1124));
        let v1128=(v68+(v1121*v1126));
        let v1130=(if v1119{(v632/v1128)}else{v1115});
        let v1131=(v1099-v551);
        let v1132=(v42*v1131);
        let v1134=(v68+(v37*v1131));
        let v1136=(v68+(v1132*v1134));
        let v1138=(v68+(v1131*v1136));
        let v1140=(if v1119{(v632/v1138)}else{(if v1112{(self.scalar_static_f64[239]/v1115)}else{(if v1101{(v68/v1103)}else{v926})})});
        let v1142=(v8+(v1099*v1099));
        let v1146=(if v968{(v837-v1099)}else{(if v968{(v68/v1142)}else{v1041})});
        let v1153=(if v968{((v8*v1146)+(self.scalar_static_f64[213]*((v1130+(v68-v1140))-self.scalar_static_f64[239])))}else{v941});
        let v1167=(if v968{(v8-(self.scalar_static_f64[213]*(v1130+v1140)))}else{v1146});
        let v1169=(v8*(if v968{((v1146*v1146)-(self.scalar_static_f64[213]*((v1130+((v1099+v1140)-v68))-(self.scalar_static_f64[239]*(v68+v1099)))))}else{v951}));
        let v1173=((if v968{((v1153*v1153)-(v1167*v1169))}else{v1167})).sqrt();
        let v1174=(v1153+v1173);
        let v1183=(if self.scalar_static_bool[16]{((v477-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v968{(v1099+(v1169/v1174))}else{(if v857{((-v908)-(v957/v963))}else{(if v840{(v845*v850)}else{v21})})})))}else{v21}))/self.scalar_static_f64[165])}else{v478});
        let v1185=((v1183).abs()<=v457);
        let v1186=(self.scalar_static_bool[16]&&v1185);
        let v1187=(if v1186{v484}else{v21});
        let v1188=(v456*v1183);
        let v1189=(v487*v1183);
        let v1190=(v451*v1189);
        let v1192=(v68+(v1187*v1190));
        let v1195=(v1183<v494);
        let v1197=(self.scalar_static_bool[16]&&(!v1185));
        let v1198=(v1195&&v1197);
        let v1200=(if v1198{(-v1183)}else{v21});
        let v1201=(v160*v1200);
        let v1203=(if v1198{(v456*v1201)}else{v21});
        let v1205=(v1203-v141);
        let v1208=((v507+(v1205*v1205))).sqrt();
        let v1211=(if v1198{(v42*((v503+v1203)-v1208))}else{v21});
        let v1213=(if v1198{(v1200-v1211)}else{v21});
        let v1215=(v68+v1211);
        let v1218=(if v1198{((v1213*v1213)+(v452*v1215))}else{v21});
        let v1221=(if v1198{((v8*v1213)-v452)}else{v21});
        let v1223=(v453*v1218);
        let v1226=(if v1198{((-v1211)+(v1223).ln())}else{v21});
        let v1228=(if v1198{(v1218+v1221)}else{v21});
        let v1230=(v42*v1221);
        let v1232=((v1221*v1230)-v1218);
        let v1235=(if v1198{((v1228*v1228)+(v1226*v1232))}else{v21});
        let v1236=(v1218*v1228);
        let v1237=(v1226*v1236);
        let v1238=(v1226*v1228);
        let v1239=(v1226*v1238);
        let v1240=(v1239/v1235);
        let v1241=(v1221*v1240);
        let v1244=((v37*(v1221*v1221))-v1218);
        let v1246=(v1235+(v1241*v1244));
        let v1249=(if v1198{(v1211+(v1237/v1246))}else{v21});
        let v1250=(v1249<v551);
        let v1251=(v1198&&v1250);
        let v1252=(v1249).exp();
        let v1255=(v1198&&(!v1250));
        let v1256=(v1249-v551);
        let v1257=(v42*v1256);
        let v1259=(v68+(v37*v1256));
        let v1261=(v68+(v1257*v1259));
        let v1265=(if v1255{(v558*(v68+(v1256*v1261)))}else{(if v1251{v1252}else{v21})});
        let v1267=(if v1198{(v68/v1265)}else{v21});
        let v1269=(v8+(v1249*v1249));
        let v1273=(if v1198{(v1200-v1249)}else{(if v1198{(v68/v1269)}else{v1213})});
        let v1275=(if v1198{(v473*v1267)}else{v1187});
        let v1279=(v473+((v1265-v68)-v1275));
        let v1282=(if v1198{((v8*v1273)+(v452*v1279))}else{v21});
        let v1287=(v1249-v68);
        let v1289=((v1275+((v1265-v1249)-v68))+(v473*v1287));
        let v1292=(if v1198{((v1273*v1273)-(v452*v1289))}else{v21});
        let v1293=(v1265+v1275);
        let v1296=(if v1198{(v8-(v452*v1293))}else{v1273});
        let v1298=(v8*v1292);
        let v1301=(if v1198{((v1282*v1282)-(v1296*v1298))}else{v1296});
        let v1303=(v1301).sqrt();
        let v1304=(v1282+v1303);
        let v1309=(v1197&&(!v1195));
        let v1310=(if v1309{v613}else{v21});
        let v1312=((v615*v1310)-v68);
        let v1314=(if v1309{(v1310*v1312)}else{v21});
        let v1316=(v68+(v1183*v1314));
        let v1319=(-(if v1309{(v1188*v1316)}else{v21}));
        let v1320=(v1319>v625);
        let v1321=(v1309&&v1320);
        let v1322=(v1319).exp();
        let v1325=(v1309&&(!v1320));
        let v1326=(v625-v1319);
        let v1327=(v42*v1326);
        let v1329=(v68+(v37*v1326));
        let v1331=(v68+(v1327*v1329));
        let v1333=(v68+(v1326*v1331));
        let v1335=(if v1325{(v632/v1333)}else{(if v1321{v1322}else{v1301})});
        let v1341=(((v648+v1183)-(if v1309{(v68-v1335)}else{v21}))).sqrt();
        let v1344=(if v1309{((v645+v1183)-(v451*v1341))}else{v21});
        let v1345=(if v1309{v655}else{v21});
        let v1346=(v1345-v1344);
        let v1347=(v1346>v376);
        let v1350=((v660+(v1346*v1346))).sqrt();
        let v1354=(v1344-v1345);
        let v1355=(v1354>v376);
        let v1358=((v660+(v1354*v1354))).sqrt();
        let v1359=(v1354+v1358);
        let v1369=((v660+(v1345*v1345))).sqrt();
        let v1373=(if v1309{((if v1347{(v1345-(v42*(v1346+v1350)))}else{(if v1355{(v1345-(v668/v1359))}else{(v1345-(v42*(v675+v1346)))})})-(v42*(v1345-v1369)))}else{v1211});
        let v1375=(if v1309{(v1183-v1373)}else{v1335});
        let v1377=((-v1373)).exp();
        let v1378=(if v1309{v1377}else{v1275});
        let v1382=(v68+v1373);
        let v1384=(((v1373+v1378)-v68)-(v473*v1382));
        let v1386=((v1375*v1375)-(v452*v1384));
        let v1387=(v693>v1386);
        let v1389=(if v1309{(if v1387{v693}else{v1386})}else{v1218});
        let v1392=(if v1309{(v68-(v645*v1378))}else{v21});
        let v1395=((v68-v1378)-v473);
        let v1398=(if v1309{((v8*v1375)+(v452*v1395))}else{v1221});
        let v1400=(v1389/v452);
        let v1403=(if v1309{((v458-v1373)+(v1400).ln())}else{v1226});
        let v1405=(if v1309{(v1389+v1398)}else{v21});
        let v1407=((v1403).abs()<v722);
        let v1408=(v1309&&v1407);
        let v1411=(v1309&&(!v1407));
        let v1413=(v42*v1398);
        let v1415=(v1389*v1392);
        let v1416=((v1398*v1413)-v1415);
        let v1419=(if v1411{((v1405*v1405)+(v1403*v1416))}else{v21});
        let v1420=(v1389*v1405);
        let v1421=(v1403*v1420);
        let v1422=(v1403*v1405);
        let v1423=(v1403*v1422);
        let v1424=(v1423/v1419);
        let v1425=(v1398*v1424);
        let v1428=((v37*(v1398*v1398))-v1415);
        let v1430=(v1419+(v1425*v1428));
        let v1433=(if v1411{(v1373+(v1421/v1430))}else{(if v1408{v1373}else{v21})});
        let v1434=(v1433<v551);
        let v1435=(v1309&&v1434);
        let v1436=(v1433).exp();
        let v1437=(if v1435{v1436}else{v1265});
        let v1442=(v1433>v758);
        let v1444=(v1309&&(!v1434));
        let v1445=(v1442&&v1444);
        let v1447=((v1433-v458)).exp();
        let v1448=(if v1445{v1447}else{(if v1435{(v473*v1437)}else{v1437})});
        let v1452=(v1444&&(!v1442));
        let v1454=((v458-v1433)-v551);
        let v1455=(v42*v1454);
        let v1457=(v68+(v37*v1454));
        let v1459=(v68+(v1455*v1457));
        let v1461=(v68+(v1454*v1459));
        let v1463=(if v1452{(v632/v1461)}else{v1448});
        let v1464=(v1433-v551);
        let v1465=(v42*v1464);
        let v1467=(v68+(v37*v1464));
        let v1469=(v68+(v1465*v1467));
        let v1471=(v68+(v1464*v1469));
        let v1473=(if v1452{(v632/v1471)}else{(if v1445{(v473/v1448)}else{(if v1435{(v68/v1437)}else{v1267})})});
        let v1475=(v8+(v1433*v1433));
        let v1479=(if v1309{(v1183-v1433)}else{(if v1309{(v68/v1475)}else{v1375})});
        let v1483=((v1463+(v68-v1473))-v473);
        let v1486=(if v1309{((v8*v1479)+(v452*v1483))}else{v1282});
        let v1491=(v68+v1433);
        let v1493=((v1463+((v1433+v1473)-v68))-(v473*v1491));
        let v1497=(v1463+v1473);
        let v1500=(if v1309{(v8-(v452*v1497))}else{v1479});
        let v1502=(v8*(if v1309{((v1479*v1479)-(v452*v1493))}else{v1292}));
        let v1506=((if v1309{((v1486*v1486)-(v1500*v1502))}else{v1500})).sqrt();
        let v1507=(v1486+v1506);
        let v1510=(if v1309{(v1433+(v1502/v1507))}else{(if v1198{((-v1249)-(v1298/v1304))}else{(if v1186{(v1188*v1192)}else{v827})})});
        let v1516=(!((v1183<=v21)||self.scalar_static_bool[18]));
        let v1517=(v1510<v551);
        let v1518=(v1516&&v1517);
        let v1519=(v1510).exp();
        let v1520=(if v1518{v1519}else{v21});
        let v1522=(if v1518{(v68/v1520)}else{v21});
        let v1530=(v1510>v758);
        let v1532=(v1516&&(!v1517));
        let v1533=(v1530&&v1532);
        let v1535=((v1510-v458)).exp();
        let v1536=(if v1533{v1535}else{(if v1518{(v473*v1520)}else{v1520})});
        let v1544=(v1532&&(!v1530));
        let v1556=(v1510-v551);
        let v1557=(v42*v1556);
        let v1559=(v68+(v37*v1556));
        let v1561=(v68+(v1557*v1559));
        let v1563=(v68+(v1556*v1561));
        let v1565=(if v1544{(v632/v1563)}else{(if v1533{(v473/v1536)}else{v1522})});
        let v1568=(v1510<v152);
        let v1569=(v1516&&v1568);
        let v1570=(v42*v1510);
        let v1571=(v1510*v1570);
        let v1572=(v37*v1510);
        let v1574=(v68-(v647*v1510));
        let v1576=(v68-(v1572*v1574));
        let v1588=(v1576).sqrt();
        let v1589=(if v1569{v1588}else{self.scalar_static_f64[211]});
        let v1590=(v148*v1510);
        let v1594=(v1516&&(!v1568));
        let v1597=(if v1594{(v1565+(v1510-v68))}else{(if v1569{(v1571*v1576)}else{v21})});
        let v1598=(v1597).sqrt();
        let v1599=(if v1594{v1598}else{(if v1569{(v1589*v1590)}else{v21})});
        let v1610=ctx.node_voltage(nodes[6]);
        let v1611=(v477+v1610);
        let v1612=(self.scalar_static_f64[168]*v1611);
        let v1614=((v1612).abs()<=v457);
        let v1615=(v1612/v455);
        let v1617=(v1612>v457);
        let v1618=(!v1614);
        let v1619=(v1617&&v1618);
        let v1621=((v615/v475)-v68);
        let v1622=(v1621/v475);
        let v1623=(if v1619{v1622}else{v21});
        let v1625=(v68+(v1612*v1623));
        let v1627=(if v1619{(v1615*v1625)}else{v21});
        let v1628=(v1627<v164);
        let v1629=(v1619&&v1628);
        let v1631=((-v1627)).exp();
        let v1634=(v1619&&(!v1628));
        let v1635=(v1627-v164);
        let v1636=(v42*v1635);
        let v1638=(v68+(v37*v1635));
        let v1640=(v68+(v1636*v1638));
        let v1642=(v68+(v1635*v1640));
        let v1644=(if v1634{(v170/v1642)}else{(if v1629{v1631}else{v21})});
        let v1646=(if v1619{(v68-v1644)}else{v21});
        let v1650=(((v648+v1612)-v1646)).sqrt();
        let v1653=(if v1619{((v645+v1612)-(v451*v1650))}else{v21});
        let v1654=(v1653<v164);
        let v1655=(v1619&&v1654);
        let v1657=((-v1653)).exp();
        let v1660=(v1619&&(!v1654));
        let v1661=(v1653-v164);
        let v1662=(v42*v1661);
        let v1664=(v68+(v37*v1661));
        let v1666=(v68+(v1662*v1664));
        let v1668=(v68+(v1661*v1666));
        let v1670=(if v1660{(v170/v1668)}else{(if v1655{v1657}else{v21})});
        let v1673=(if v1619{(v68-(v645*v1670))}else{v21});
        let v1674=(v1612-v1653);
        let v1676=(v68-v1670);
        let v1679=(if v1619{((v8*v1674)+(v452*v1676))}else{v21});
        let v1682=(v1670+(v1653-v68));
        let v1685=(if v1619{((v1674*v1674)-(v452*v1682))}else{v21});
        let v1687=(v94*v1673);
        let v1690=(if v1619{((v1679*v1679)-(v1685*v1687))}else{v1644});
        let v1691=(v8*v1685);
        let v1692=(v1690).sqrt();
        let v1693=(v1679+v1692);
        let v1699=(v1618&&(!v1617));
        let v1701=(if v1699{(-v1612)}else{v21});
        let v1702=(v160*v1701);
        let v1704=(if v1699{(v1702/v455)}else{v21});
        let v1706=(v1704-v141);
        let v1709=((v507+(v1706*v1706))).sqrt();
        let v1712=(if v1699{(v42*((v503+v1704)-v1709))}else{v21});
        let v1713=(v1701-v1712);
        let v1715=(v68+v1712);
        let v1718=(if v1699{((v1713*v1713)+(v452*v1715))}else{v21});
        let v1721=(if v1699{((v8*v1713)-v452)}else{v21});
        let v1722=(v1718/v452);
        let v1725=(if v1699{((v1722).ln()-v1712)}else{v21});
        let v1727=(if v1699{(v1718+v1721)}else{v21});
        let v1729=(v42*v1721);
        let v1731=((v1721*v1729)-v1718);
        let v1734=(if v1699{((v1727*v1727)+(v1725*v1731))}else{v21});
        let v1735=(v1718*v1727);
        let v1736=(v1725*v1735);
        let v1737=(v1725*v1727);
        let v1738=(v1725*v1737);
        let v1739=(v1738/v1734);
        let v1740=(v1721*v1739);
        let v1743=((v37*(v1721*v1721))-v1718);
        let v1745=(v1734+(v1740*v1743));
        let v1748=(if v1699{(v1712+(v1736/v1745))}else{v21});
        let v1750=((v1748).abs()<v551);
        let v1751=(v1699&&v1750);
        let v1752=(v1748).exp();
        let v1754=(v1748<v625);
        let v1756=(v1699&&(!v1750));
        let v1757=(v1754&&v1756);
        let v1758=(v625-v1748);
        let v1759=(v42*v1758);
        let v1761=(v68+(v37*v1758));
        let v1763=(v68+(v1759*v1761));
        let v1765=(v68+(v1758*v1763));
        let v1769=(v1756&&(!v1754));
        let v1770=(v1748-v551);
        let v1771=(v42*v1770);
        let v1773=(v68+(v37*v1770));
        let v1775=(v68+(v1771*v1773));
        let v1779=(if v1769{(v558*(v68+(v1770*v1775)))}else{(if v1757{(v632/v1765)}else{(if v1751{v1752}else{v1670})})});
        let v1783=(v1701-v1748);
        let v1785=(v1779-v68);
        let v1788=(if v1699{((v8*v1783)+(v452*v1785))}else{v1679});
        let v1791=((v68+v1748)-v1779);
        let v1794=(if v1699{((v1783*v1783)+(v452*v1791))}else{v1685});
        let v1796=(v94*(if v1699{(v68-(v645*v1779))}else{v1673}));
        let v1800=(v8*v1794);
        let v1801=((if v1699{((v1788*v1788)-(v1794*v1796))}else{v1690})).sqrt();
        let v1802=(v1788+v1801);
        let v1807=(if v1699{(-(v1748+(if v1699{(v1800/v1802)}else{v1646})))}else{(if v1619{(v1653+(if v1619{(v1691/v1693)}else{v21}))}else{(if v1614{v1615}else{v21})})});
        let v1808=(self.scalar_static_f64[165]*v1807);
        let v1812=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(v477-v1808)))}else{v21});
        let v1814=((v1812).abs()<=self.scalar_static_f64[218]);
        let v1815=(self.scalar_static_bool[16]&&v1814);
        let v1816=(if v1815{self.scalar_static_f64[314]}else{v21});
        let v1817=(self.scalar_static_f64[217]*v1812);
        let v1821=(v68+(v1816*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v1812))));
        let v1824=(v1812<self.scalar_static_f64[316]);
        let v1826=(self.scalar_static_bool[16]&&(!v1814));
        let v1827=(v1824&&v1826);
        let v1829=(if v1827{(-v1812)}else{v21});
        let v1832=(if v1827{(self.scalar_static_f64[217]*(v160*v1829))}else{v21});
        let v1834=(v1832-v141);
        let v1837=((v507+(v1834*v1834))).sqrt();
        let v1840=(if v1827{(v42*((v503+v1832)-v1837))}else{v21});
        let v1842=(if v1827{(v1829-v1840)}else{v21});
        let v1847=(if v1827{((v1842*v1842)+(self.scalar_static_f64[213]*(v68+v1840)))}else{v21});
        let v1850=(if v1827{((v8*v1842)-self.scalar_static_f64[213])}else{v21});
        let v1852=(self.scalar_static_f64[214]*v1847);
        let v1855=(if v1827{((-v1840)+(v1852).ln())}else{v21});
        let v1857=(if v1827{(v1847+v1850)}else{v21});
        let v1859=(v42*v1850);
        let v1861=((v1850*v1859)-v1847);
        let v1864=(if v1827{((v1857*v1857)+(v1855*v1861))}else{v21});
        let v1865=(v1847*v1857);
        let v1866=(v1855*v1865);
        let v1867=(v1855*v1857);
        let v1868=(v1855*v1867);
        let v1869=(v1868/v1864);
        let v1870=(v1850*v1869);
        let v1873=((v37*(v1850*v1850))-v1847);
        let v1875=(v1864+(v1870*v1873));
        let v1878=(if v1827{(v1840+(v1866/v1875))}else{v21});
        let v1879=(v1878<v551);
        let v1880=(v1827&&v1879);
        let v1881=(v1878).exp();
        let v1884=(v1827&&(!v1879));
        let v1885=(v1878-v551);
        let v1886=(v42*v1885);
        let v1888=(v68+(v37*v1885));
        let v1890=(v68+(v1886*v1888));
        let v1894=(if v1884{(v558*(v68+(v1885*v1890)))}else{(if v1880{v1881}else{v21})});
        let v1896=(if v1827{(v68/v1894)}else{v21});
        let v1898=(v8+(v1878*v1878));
        let v1902=(if v1827{(v1829-v1878)}else{(if v1827{(v68/v1898)}else{v1842})});
        let v1904=(if v1827{(self.scalar_static_f64[239]*v1896)}else{v1816});
        let v1911=(if v1827{((v8*v1902)+(self.scalar_static_f64[213]*(self.scalar_static_f64[239]+((v1894-v68)-v1904))))}else{v21});
        let v1921=(if v1827{((v1902*v1902)-(self.scalar_static_f64[213]*((v1904+((v1894-v1878)-v68))+(self.scalar_static_f64[239]*(v1878-v68)))))}else{v21});
        let v1925=(if v1827{(v8-(self.scalar_static_f64[213]*(v1894+v1904)))}else{v1902});
        let v1927=(v8*v1921);
        let v1930=(if v1827{((v1911*v1911)-(v1925*v1927))}else{v1925});
        let v1932=(v1930).sqrt();
        let v1933=(v1911+v1932);
        let v1938=(v1826&&(!v1824));
        let v1939=(if v1938{self.scalar_static_f64[319]}else{v21});
        let v1943=(if v1938{(v1939*((self.scalar_static_f64[320]*v1939)-v68))}else{v21});
        let v1945=(v68+(v1812*v1943));
        let v1948=(-(if v1938{(v1817*v1945)}else{v21}));
        let v1949=(v1948>v625);
        let v1950=(v1938&&v1949);
        let v1951=(v1948).exp();
        let v1954=(v1938&&(!v1949));
        let v1955=(v625-v1948);
        let v1956=(v42*v1955);
        let v1958=(v68+(v37*v1955));
        let v1960=(v68+(v1956*v1958));
        let v1962=(v68+(v1955*v1960));
        let v1964=(if v1954{(v632/v1962)}else{(if v1950{v1951}else{v1930})});
        let v1970=(((self.scalar_static_f64[322]+v1812)-(if v1938{(v68-v1964)}else{v21}))).sqrt();
        let v1973=(if v1938{((self.scalar_static_f64[321]+v1812)-(self.scalar_static_f64[212]*v1970))}else{v21});
        let v1974=(if v1938{self.scalar_static_f64[323]}else{v21});
        let v1975=(v1974-v1973);
        let v1976=(v1975>v376);
        let v1979=((v660+(v1975*v1975))).sqrt();
        let v1983=(v1973-v1974);
        let v1984=(v1983>v376);
        let v1987=((v660+(v1983*v1983))).sqrt();
        let v1988=(v1983+v1987);
        let v2002=(if v1938{((if v1976{(v1974-(v42*(v1975+v1979)))}else{(if v1984{(v1974-(v668/v1988))}else{(v1974-(v42*(v675+v1975)))})})-(v42*(v1974-((v660+(v1974*v1974))).sqrt())))}else{v1840});
        let v2004=(if v1938{(v1812-v2002)}else{v1964});
        let v2006=((-v2002)).exp();
        let v2007=(if v1938{v2006}else{v1904});
        let v2015=((v2004*v2004)-(self.scalar_static_f64[213]*(((v2002+v2007)-v68)-(self.scalar_static_f64[239]*(v68+v2002)))));
        let v2016=(v693>v2015);
        let v2018=(if v1938{(if v2016{v693}else{v2015})}else{v1847});
        let v2021=(if v1938{(v68-(self.scalar_static_f64[321]*v2007))}else{v21});
        let v2027=(if v1938{((v8*v2004)+(self.scalar_static_f64[213]*((v68-v2007)-self.scalar_static_f64[239])))}else{v1850});
        let v2029=(v2018/self.scalar_static_f64[213]);
        let v2032=(if v1938{((self.scalar_static_f64[219]-v2002)+(v2029).ln())}else{v1855});
        let v2034=(if v1938{(v2018+v2027)}else{v21});
        let v2036=((v2032).abs()<v722);
        let v2037=(v1938&&v2036);
        let v2040=(v1938&&(!v2036));
        let v2042=(v42*v2027);
        let v2044=(v2018*v2021);
        let v2045=((v2027*v2042)-v2044);
        let v2048=(if v2040{((v2034*v2034)+(v2032*v2045))}else{v21});
        let v2049=(v2018*v2034);
        let v2050=(v2032*v2049);
        let v2051=(v2032*v2034);
        let v2052=(v2032*v2051);
        let v2053=(v2052/v2048);
        let v2054=(v2027*v2053);
        let v2057=((v37*(v2027*v2027))-v2044);
        let v2059=(v2048+(v2054*v2057));
        let v2062=(if v2040{(v2002+(v2050/v2059))}else{(if v2037{v2002}else{v21})});
        let v2063=(v2062<v551);
        let v2064=(v1938&&v2063);
        let v2065=(v2062).exp();
        let v2066=(if v2064{v2065}else{v1894});
        let v2071=(v2062>self.scalar_static_f64[324]);
        let v2073=(v1938&&(!v2063));
        let v2074=(v2071&&v2073);
        let v2076=((v2062-self.scalar_static_f64[219])).exp();
        let v2077=(if v2074{v2076}else{(if v2064{(self.scalar_static_f64[239]*v2066)}else{v2066})});
        let v2081=(v2073&&(!v2071));
        let v2083=((self.scalar_static_f64[219]-v2062)-v551);
        let v2084=(v42*v2083);
        let v2086=(v68+(v37*v2083));
        let v2088=(v68+(v2084*v2086));
        let v2090=(v68+(v2083*v2088));
        let v2092=(if v2081{(v632/v2090)}else{v2077});
        let v2093=(v2062-v551);
        let v2094=(v42*v2093);
        let v2096=(v68+(v37*v2093));
        let v2098=(v68+(v2094*v2096));
        let v2100=(v68+(v2093*v2098));
        let v2102=(if v2081{(v632/v2100)}else{(if v2074{(self.scalar_static_f64[239]/v2077)}else{(if v2064{(v68/v2066)}else{v1896})})});
        let v2104=(v8+(v2062*v2062));
        let v2108=(if v1938{(v1812-v2062)}else{(if v1938{(v68/v2104)}else{v2004})});
        let v2115=(if v1938{((v8*v2108)+(self.scalar_static_f64[213]*((v2092+(v68-v2102))-self.scalar_static_f64[239])))}else{v1911});
        let v2129=(if v1938{(v8-(self.scalar_static_f64[213]*(v2092+v2102)))}else{v2108});
        let v2131=(v8*(if v1938{((v2108*v2108)-(self.scalar_static_f64[213]*((v2092+((v2062+v2102)-v68))-(self.scalar_static_f64[239]*(v68+v2062)))))}else{v1921}));
        let v2135=((if v1938{((v2115*v2115)-(v2129*v2131))}else{v2129})).sqrt();
        let v2136=(v2115+v2135);
        let v2142=(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v1938{(v2062+(v2131/v2136))}else{(if v1827{((-v1878)-(v1927/v1933))}else{(if v1815{(v1817*v1821)}else{v21})})})))}else{v21});
        let v2145=(if self.scalar_static_bool[16]{((v1611-v2142)/self.scalar_static_f64[165])}else{v1612});
        let v2147=((v2145).abs()<=v457);
        let v2148=(self.scalar_static_bool[16]&&v2147);
        let v2149=(v2145/v455);
        let v2151=(v2145>v457);
        let v2153=(self.scalar_static_bool[16]&&(!v2147));
        let v2154=(v2151&&v2153);
        let v2155=(if v2154{v1622}else{v21});
        let v2157=(v68+(v2145*v2155));
        let v2159=(if v2154{(v2149*v2157)}else{v21});
        let v2160=(v2159<v164);
        let v2161=(v2154&&v2160);
        let v2163=((-v2159)).exp();
        let v2166=(v2154&&(!v2160));
        let v2167=(v2159-v164);
        let v2168=(v42*v2167);
        let v2170=(v68+(v37*v2167));
        let v2172=(v68+(v2168*v2170));
        let v2174=(v68+(v2167*v2172));
        let v2176=(if v2166{(v170/v2174)}else{(if v2161{v2163}else{v21})});
        let v2178=(if v2154{(v68-v2176)}else{v21});
        let v2182=(((v648+v2145)-v2178)).sqrt();
        let v2185=(if v2154{((v645+v2145)-(v451*v2182))}else{v21});
        let v2186=(v2185<v164);
        let v2187=(v2154&&v2186);
        let v2189=((-v2185)).exp();
        let v2192=(v2154&&(!v2186));
        let v2193=(v2185-v164);
        let v2194=(v42*v2193);
        let v2196=(v68+(v37*v2193));
        let v2198=(v68+(v2194*v2196));
        let v2200=(v68+(v2193*v2198));
        let v2202=(if v2192{(v170/v2200)}else{(if v2187{v2189}else{v21})});
        let v2205=(if v2154{(v68-(v645*v2202))}else{v21});
        let v2206=(v2145-v2185);
        let v2208=(v68-v2202);
        let v2211=(if v2154{((v8*v2206)+(v452*v2208))}else{v21});
        let v2214=(v2202+(v2185-v68));
        let v2217=(if v2154{((v2206*v2206)-(v452*v2214))}else{v21});
        let v2219=(v94*v2205);
        let v2222=(if v2154{((v2211*v2211)-(v2217*v2219))}else{v2176});
        let v2223=(v8*v2217);
        let v2224=(v2222).sqrt();
        let v2225=(v2211+v2224);
        let v2231=(v2153&&(!v2151));
        let v2233=(if v2231{(-v2145)}else{v21});
        let v2234=(v160*v2233);
        let v2236=(if v2231{(v2234/v455)}else{v21});
        let v2238=(v2236-v141);
        let v2241=((v507+(v2238*v2238))).sqrt();
        let v2244=(if v2231{(v42*((v503+v2236)-v2241))}else{v21});
        let v2245=(v2233-v2244);
        let v2247=(v68+v2244);
        let v2250=(if v2231{((v2245*v2245)+(v452*v2247))}else{v21});
        let v2253=(if v2231{((v8*v2245)-v452)}else{v21});
        let v2254=(v2250/v452);
        let v2257=(if v2231{((v2254).ln()-v2244)}else{v21});
        let v2259=(if v2231{(v2250+v2253)}else{v21});
        let v2261=(v42*v2253);
        let v2263=((v2253*v2261)-v2250);
        let v2266=(if v2231{((v2259*v2259)+(v2257*v2263))}else{v21});
        let v2267=(v2250*v2259);
        let v2268=(v2257*v2267);
        let v2269=(v2257*v2259);
        let v2270=(v2257*v2269);
        let v2271=(v2270/v2266);
        let v2272=(v2253*v2271);
        let v2275=((v37*(v2253*v2253))-v2250);
        let v2277=(v2266+(v2272*v2275));
        let v2280=(if v2231{(v2244+(v2268/v2277))}else{v21});
        let v2282=((v2280).abs()<v551);
        let v2283=(v2231&&v2282);
        let v2284=(v2280).exp();
        let v2286=(v2280<v625);
        let v2288=(v2231&&(!v2282));
        let v2289=(v2286&&v2288);
        let v2290=(v625-v2280);
        let v2291=(v42*v2290);
        let v2293=(v68+(v37*v2290));
        let v2295=(v68+(v2291*v2293));
        let v2297=(v68+(v2290*v2295));
        let v2301=(v2288&&(!v2286));
        let v2302=(v2280-v551);
        let v2303=(v42*v2302);
        let v2305=(v68+(v37*v2302));
        let v2307=(v68+(v2303*v2305));
        let v2311=(if v2301{(v558*(v68+(v2302*v2307)))}else{(if v2289{(v632/v2297)}else{(if v2283{v2284}else{v2202})})});
        let v2315=(v2233-v2280);
        let v2317=(v2311-v68);
        let v2320=(if v2231{((v8*v2315)+(v452*v2317))}else{v2211});
        let v2323=((v68+v2280)-v2311);
        let v2326=(if v2231{((v2315*v2315)+(v452*v2323))}else{v2217});
        let v2328=(v94*(if v2231{(v68-(v645*v2311))}else{v2205}));
        let v2332=(v8*v2326);
        let v2333=((if v2231{((v2320*v2320)-(v2326*v2328))}else{v2222})).sqrt();
        let v2334=(v2320+v2333);
        let v2339=(if v2231{(-(v2280+(if v2231{(v2332/v2334)}else{v2178})))}else{(if v2154{(v2185+(if v2154{(v2223/v2225)}else{v21}))}else{(if v2148{v2149}else{v1807})})});
        let v2343=(v2339<v551);
        let v2344=(v2339).exp();
        let v2345=(if v2343{v2344}else{v21});
        let v2348=(v2339>v758);
        let v2349=(!v2343);
        let v2350=(v2348&&v2349);
        let v2352=((v458-v2339)).exp();
        let v2353=(if v2350{v2352}else{v2345});
        let v2357=(v2349&&(!v2348));
        let v2358=(v2339-v551);
        let v2359=(v42*v2358);
        let v2361=(v68+(v37*v2358));
        let v2363=(v68+(v2359*v2361));
        let v2365=(v68+(v2358*v2363));
        let v2367=(if v2357{(v632/v2365)}else{(if v2350{(v473*v2353)}else{(if v2343{(v68/v2345)}else{v1565})})});
        let v2368=(v2339<v494);
        let v2371=(if v2368{((v2339+v2367)-v68)}else{v1597});
        let v2372=(v2371).sqrt();
        let v2376=((v2339).abs()<=v457);
        let v2377=(!v2368);
        let v2378=(v2376&&v2377);
        let v2379=(v37*v2339);
        let v2381=(v68-(v647*v2339));
        let v2384=(if v2378{(v68-(v2379*v2381))}else{v1589});
        let v2385=(v42*v2339);
        let v2386=(v2339*v2385);
        let v2389=(v148*v2339);
        let v2390=(v2384).sqrt();
        let v2394=(v2377&&(!v2376));
        let v2398=((if v2394{(v2367+(v2339-v68))}else{(if v2378{(v2384*v2386)}else{v2371})})).sqrt();
        let v2400=(self.scalar_static_f64[165]*(if v2394{v2398}else{(if v2378{(v2389*v2390)}else{(if v2368{(-v2372)}else{v1599})})}));
        let v2401=(v451*v2400);
        let v2402=1.62;
        let v2403=(v68+(v425/v426));
        let v2404=(v2402*v2403);
        let v2415=(self.scalar_static_f64[165]*(self.scalar_static_f64[165]*((self.scalar_static_f64[163]*(self.scalar_static_f64[150]*((v2403*v2404)*self.scalar_static_f64[150])))*self.scalar_static_f64[325])));
        let v2416=(-v2401);
        let v2417=(v2401-v2416);
        let v2418=(v2417>v376);
        let v2421=((v2415+(v2417*v2417))).sqrt();
        let v2425=(v2416-v2401);
        let v2426=(v2425>v376);
        let v2427=(v42*v2415);
        let v2430=((v2415+(v2425*v2425))).sqrt();
        let v2431=(v2425+v2430);
        let v2435=((1e-32+v2415)).sqrt();
        let v2441=(-v1610);
        let v2442=(v2441-v1610);
        let v2443=(v2442>v376);
        let v2446=((v2415+(v2442*v2442))).sqrt();
        let v2450=(v1610-v2441);
        let v2451=(v2450>v376);
        let v2454=((v2415+(v2450*v2450))).sqrt();
        let v2455=(v2450+v2454);
        let v2464=((if v2418{(v2416+(v42*(v2417+v2421)))}else{(if v2426{(v2416+(v2427/v2431))}else{(v2416+(v42*(v2417+v2435)))})})+(self.scalar_static_f64[27]*(if v2443{(v1610+(v42*(v2442+v2446)))}else{(if v2451{(v1610+(v2427/v2455))}else{(v1610+(v42*(v2435+v2442)))})})));
        let v2467=(self.scalar_static_f64[167]+(v2464*v2464));
        let v2469=-0.1666666666666667;
        let v2472=(v68+(self.scalar_static_f64[22]*f64::powf(v2467,v2469)));
        let v2474=(if self.scalar_static_bool[19]{(self.scalar_static_f64[4]/v2472)}else{self.scalar_static_f64[4]});
        let v2520=ctx.node_voltage(nodes[1]);
        let v3257=(self.scalar_static_f64[50]*(self.scalar_static_f64[48]*((v477-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*v2339)}else{v1808}))-(if self.scalar_static_bool[17]{v21}else{v2142}))));
        let v3259=(self.scalar_static_f64[19]*(v2474*v3257));
        let v3261=(v1610*self.scalar_static_f64[153]);
        let v3262=ctx.node_voltage(nodes[3]);
        let v3264=(self.scalar_static_f64[56]*(v3262-v2520));
        let v3282=(self.scalar_static_f64[19]*v375);
        let v3284=(v375*self.scalar_static_f64[145]);
        let v3286=(v8*v381);
        let v3293=(v384*self.scalar_static_f64[145]);
        let v3295=(self.scalar_static_f64[19]*v384);
        let v3297=(v8*v389);
        let v3304=(v390*v390);
        let v3315=(self.scalar_static_f64[137]*(if v377{(v42*(self.scalar_static_f64[19]+((v3282+v3282)/v3286)))}else{(if v385{((-(self.scalar_static_f64[140]*(self.scalar_static_f64[145]+((v3293+v3293)/v3297))))/v3304)}else{self.scalar_static_f64[154]})}));
        let v3316=(self.scalar_static_f64[137]*(if v377{(v42*(self.scalar_static_f64[145]+((v3284+v3284)/v3286)))}else{(if v385{((-(self.scalar_static_f64[140]*(self.scalar_static_f64[19]+((v3295+v3295)/v3297))))/v3304)}else{self.scalar_static_f64[155]})}));
        let v3317=(-v3315);
        let v3318=(-v3316);
        let v3319=(v402*v3317);
        let v3321=(v402*v3318);
        let v3323=(v8*v407);
        let v3332=(v411*v3315);
        let v3334=(v411*v3316);
        let v3336=(v8*v416);
        let v3343=(v417*v417);
        let v3358=(self.scalar_static_f64[5]*(if v403{(-(v42*(v3317+((v3319+v3319)/v3323))))}else{(if v412{(-((-(v413*(v3315+((v3332+v3332)/v3336))))/v3343))}else{(-(v42*v3317))})}));
        let v3359=(self.scalar_static_f64[5]*(if v403{(-(v42*(v3318+((v3321+v3321)/v3323))))}else{(if v412{(-((-(v413*(v3316+((v3334+v3334)/v3336))))/v3343))}else{(-(v42*v3318))})}));
        let v3360=(v3358/v426);
        let v3361=(v3359/v426);
        let v3366=(self.scalar_static_f64[200]*((self.scalar_static_f64[199]*v3358)/v428));
        let v3367=(self.scalar_static_f64[200]*((self.scalar_static_f64[199]*v3359)/v428));
        let v3370=(v8*v433);
        let v3373=(((v10*v3358)/v3370)/self.scalar_static_f64[4]);
        let v3374=(((v10*v3359)/v3370)/self.scalar_static_f64[4]);
        let v3375=(v434*v3373);
        let v3377=(v434*v3374);
        let v3385=(v8*v437);
        let v3388=(if self.scalar_static_bool[0]{(((v435*v3366)+(v431*(v3375+v3375)))/v3385)}else{v21});
        let v3389=(if self.scalar_static_bool[0]{(((v435*v3367)+(v431*(v3377+v3377)))/v3385)}else{v21});
        let v3392=(v25*f64::powf(v438,-0.33333333333333337));
        let v3397=(if self.scalar_static_bool[0]{(self.scalar_static_f64[144]*(v3388*v3392))}else{v21});
        let v3398=(if self.scalar_static_bool[0]{(self.scalar_static_f64[144]*(v3389*v3392))}else{v21});
        let v3408=(v438*v438);
        let v3420=(if self.scalar_static_bool[0]{((v448*v3373)+(v434*(((v438*(v297*v3397))-(v446*v3388))/v3408)))}else{v3373});
        let v3421=(if self.scalar_static_bool[0]{((v448*v3374)+(v434*(((v438*(v297*v3398))-(v446*v3389))/v3408)))}else{v3374});
        let v3422=(self.scalar_static_f64[211]*v3420);
        let v3423=(self.scalar_static_f64[211]*v3421);
        let v3424=(v451*v3422);
        let v3425=(v3424+v3424);
        let v3426=(v451*v3423);
        let v3427=(v3426+v3426);
        let v3429=(v452*v452);
        let v3430=((-v3425)/v3429);
        let v3432=((-v3427)/v3429);
        let v3433=(v148*v3422);
        let v3434=(v148*v3423);
        let v3436=(v455*v455);
        let v3437=((-v3433)/v3436);
        let v3439=((-v3434)/v3436);
        let v3440=(self.scalar_static_f64[168]*(if self.scalar_static_bool[0]{(v3366+v3397)}else{v3366}));
        let v3441=(self.scalar_static_f64[168]*(if self.scalar_static_bool[0]{(v3367+v3398)}else{v3367}));
        let v3466=(v471*v471);
        let v3471=(if v463{((-(v170*((v469*v3440)+(v464*((v467*(v42*v3440))+(v465*(v37*v3440)))))))/v3466)}else{(if v459{(v461*(-v3440))}else{v21})});
        let v3472=(if v463{((-(v170*((v469*v3441)+(v464*((v467*(v42*v3441))+(v465*(v37*v3441)))))))/v3466)}else{(if v459{(v461*(-v3441))}else{v21})});
        let v3473=(v161*v3422);
        let v3474=(v161*v3423);
        let v3477=(v456*v3437);
        let v3479=(v456*v3439);
        let v3483=(v148*(v482*(v3477+v3477)));
        let v3484=(v148*(v482*(v3479+v3479)));
        let v3485=(if v480{v3483}else{v21});
        let v3486=(if v480{v3484}else{v21});
        let v3489=((v478*v3437)+(v456*self.scalar_static_f64[336]));
        let v3492=((v478*v3439)+(v456*self.scalar_static_f64[337]));
        let v3493=(-v3471);
        let v3494=(-v3472);
        let v3523=(if v497{self.scalar_static_f64[338]}else{v21});
        let v3524=(if v497{self.scalar_static_f64[339]}else{v21});
        let v3533=(if v497{((v500*v3437)+(v456*(v160*v3523)))}else{v21});
        let v3534=(if v497{((v500*v3439)+(v456*(v160*v3524)))}else{v21});
        let v3535=(v505*v3533);
        let v3537=(v505*v3534);
        let v3539=(v8*v509);
        let v3546=(if v497{(v42*(v3533-((v3535+v3535)/v3539)))}else{v21});
        let v3547=(if v497{(v42*(v3534-((v3537+v3537)/v3539)))}else{v21});
        let v3550=(if v497{(v3523-v3546)}else{v21});
        let v3551=(if v497{(v3524-v3547)}else{v21});
        let v3552=(v514*v3550);
        let v3554=(v514*v3551);
        let v3564=(if v497{((v3552+v3552)+((v516*v3425)+(v452*v3546)))}else{v21});
        let v3565=(if v497{((v3554+v3554)+((v516*v3427)+(v452*v3547)))}else{v21});
        let v3570=(if v497{((v8*v3550)-v3425)}else{v21});
        let v3571=(if v497{((v8*v3551)-v3427)}else{v21});
        let v3584=(if v497{((-v3546)+(((v519*v3430)+(v453*v3564))/v524))}else{v21});
        let v3585=(if v497{((-v3547)+(((v519*v3432)+(v453*v3565))/v524))}else{v21});
        let v3588=(if v497{(v3564+v3570)}else{v21});
        let v3589=(if v497{(v3565+v3571)}else{v21});
        let v3590=(v529*v3588);
        let v3592=(v529*v3589);
        let v3612=(if v497{((v3590+v3590)+((v533*v3584)+(v527*(((v531*v3570)+(v522*(v42*v3570)))-v3564))))}else{v21});
        let v3613=(if v497{((v3592+v3592)+((v533*v3585)+(v527*(((v531*v3571)+(v522*(v42*v3571)))-v3565))))}else{v21});
        let v3641=(v536*v536);
        let v3653=(v522*v3570);
        let v3655=(v522*v3571);
        let v3672=(v547*v547);
        let v3680=(if v497{(v3546+(((v547*((v537*v3584)+(v527*((v529*v3564)+(v519*v3588)))))-(v538*(v3612+((v545*((v541*v3570)+(v522*(((v536*((v539*v3584)+(v527*((v529*v3584)+(v527*v3588)))))-(v540*v3612))/v3641))))+(v542*((v37*(v3653+v3653))-v3564))))))/v3672))}else{v21});
        let v3681=(if v497{(v3547+(((v547*((v537*v3585)+(v527*((v529*v3565)+(v519*v3589)))))-(v538*(v3613+((v545*((v541*v3571)+(v522*(((v536*((v539*v3585)+(v527*((v529*v3585)+(v527*v3589)))))-(v540*v3613))/v3641))))+(v542*((v37*(v3655+v3655))-v3565))))))/v3672))}else{v21});
        let v3704=(if v557{(v558*((v564*v3680)+(v559*((v562*(v42*v3680))+(v560*(v37*v3680))))))}else{(if v553{(v554*v3680)}else{v21})});
        let v3705=(if v557{(v558*((v564*v3681)+(v559*((v562*(v42*v3681))+(v560*(v37*v3681))))))}else{(if v553{(v554*v3681)}else{v21})});
        let v3707=(v568*v568);
        let v3711=(if v497{((-v3704)/v3707)}else{v21});
        let v3712=(if v497{((-v3705)/v3707)}else{v21});
        let v3713=(v550*v3680);
        let v3715=(v550*v3681);
        let v3718=(v572*v572);
        let v3726=(if v497{(v3523-v3680)}else{(if v497{((-(v3713+v3713))/v3718)}else{v3550})});
        let v3727=(if v497{(v3524-v3681)}else{(if v497{((-(v3715+v3715))/v3718)}else{v3551})});
        let v3734=(if v497{((v570*v3471)+(v473*v3711))}else{v3485});
        let v3735=(if v497{((v570*v3472)+(v473*v3712))}else{v3486});
        let v3750=(if v497{((v8*v3726)+((v582*v3425)+(v452*(v3471+(v3704-v3734)))))}else{v21});
        let v3751=(if v497{((v8*v3727)+((v582*v3427)+(v452*(v3472+(v3705-v3735)))))}else{v21});
        let v3752=(v576*v3726);
        let v3754=(v576*v3727);
        let v3776=(if v497{((v3752+v3752)-((v592*v3425)+(v452*((v3734+(v3704-v3680))+((v590*v3471)+(v473*v3680))))))}else{v21});
        let v3777=(if v497{((v3754+v3754)-((v592*v3427)+(v452*((v3735+(v3705-v3681))+((v590*v3472)+(v473*v3681))))))}else{v21});
        let v3788=(if v497{(-((v596*v3425)+(v452*(v3704+v3734))))}else{v3726});
        let v3789=(if v497{(-((v596*v3427)+(v452*(v3705+v3735))))}else{v3727});
        let v3790=(v585*v3750);
        let v3792=(v585*v3751);
        let v3794=(v8*v3776);
        let v3795=(v8*v3777);
        let v3804=(if v497{((v3790+v3790)-((v601*v3788)+(v599*v3794)))}else{v3788});
        let v3805=(if v497{((v3792+v3792)-((v601*v3789)+(v599*v3795)))}else{v3789});
        let v3808=(v8*v606);
        let v3816=(v607*v607);
        let v3827=(v475*v475);
        let v3828=((-v3473)/v3827);
        let v3830=((-v3474)/v3827);
        let v3831=(if v612{v3828}else{v21});
        let v3832=(if v612{v3830}else{v21});
        let v3833=(v160*v3433);
        let v3834=(v160*v3434);
        let v3861=(if v612{((v621*v3489)+(v486*((v619*self.scalar_static_f64[336])+(v478*(if v612{((v617*v3831)+(v614*((v615*v3831)+(v614*v3833))))}else{v21})))))}else{v21});
        let v3862=(if v612{((v621*v3492)+(v486*((v619*self.scalar_static_f64[337])+(v478*(if v612{((v617*v3832)+(v614*((v615*v3832)+(v614*v3834))))}else{v21})))))}else{v21});
        let v3887=(v640*v640);
        let v3892=(if v631{((-(v632*((v638*v3861)+(v633*((v636*(v42*v3861))+(v634*(v37*v3861)))))))/v3887)}else{(if v627{(v628*(-v3861))}else{v3804})});
        let v3893=(if v631{((-(v632*((v638*v3862)+(v633*((v636*(v42*v3862))+(v634*(v37*v3862)))))))/v3887)}else{(if v627{(v628*(-v3862))}else{v3805})});
        let v3898=(v42*v3425);
        let v3899=(v42*v3427);
        let v3900=(self.scalar_static_f64[336]+v3898);
        let v3901=(self.scalar_static_f64[337]+v3899);
        let v3902=(v647*v3425);
        let v3903=(v647*v3427);
        let v3904=(self.scalar_static_f64[336]+v3902);
        let v3905=(self.scalar_static_f64[337]+v3903);
        let v3908=(v8*v651);
        let v3919=(if v612{(v3900-((v651*v3422)+(v451*((v3904-(if v612{(-v3892)}else{v21}))/v3908))))}else{v21});
        let v3920=(if v612{(v3901-((v651*v3423)+(v451*((v3905-(if v612{(-v3893)}else{v21}))/v3908))))}else{v21});
        let v3921=(if v612{v3440}else{v21});
        let v3922=(if v612{v3441}else{v21});
        let v3923=(v3921-v3919);
        let v3924=(v3922-v3920);
        let v3925=(v657*v3923);
        let v3927=(v657*v3924);
        let v3929=(v8*v662);
        let v3938=(v3919-v3921);
        let v3939=(v3920-v3922);
        let v3940=(v666*v3938);
        let v3942=(v666*v3939);
        let v3944=(v8*v671);
        let v3951=(v672*v672);
        let v3966=(v656*v3921);
        let v3968=(v656*v3922);
        let v3970=(v8*v683);
        let v3979=(if v612{((if v658{(v3921-(v42*(v3923+((v3925+v3925)/v3929))))}else{(if v667{(v3921-((-(v668*(v3938+((v3940+v3940)/v3944))))/v3951))}else{(v3921-(v42*v3923))})})-(v42*(v3921-((v3966+v3966)/v3970))))}else{v3546});
        let v3980=(if v612{((if v658{(v3922-(v42*(v3924+((v3927+v3927)/v3929))))}else{(if v667{(v3922-((-(v668*(v3939+((v3942+v3942)/v3944))))/v3951))}else{(v3922-(v42*v3924))})})-(v42*(v3922-((v3968+v3968)/v3970))))}else{v3547});
        let v3983=(if v612{(self.scalar_static_f64[336]-v3979)}else{v3892});
        let v3984=(if v612{(self.scalar_static_f64[337]-v3980)}else{v3893});
        let v3989=(if v612{(v691*(-v3979))}else{v3734});
        let v3990=(if v612{(v691*(-v3980))}else{v3735});
        let v3991=(v689*v3983);
        let v3993=(v689*v3984);
        let v4015=(if v612{(if v702{v21}else{((v3991+v3991)-((v699*v3425)+(v452*((v3979+v3989)-((v697*v3471)+(v473*v3979))))))})}else{v3564});
        let v4016=(if v612{(if v702{v21}else{((v3993+v3993)-((v699*v3427)+(v452*((v3980+v3990)-((v697*v3472)+(v473*v3980))))))})}else{v3565});
        let v4041=(if v612{((v8*v3983)+((v710*v3425)+(v452*((-v3989)-v3471))))}else{v3570});
        let v4042=(if v612{((v8*v3984)+((v710*v3427)+(v452*((-v3990)-v3472))))}else{v3571});
        let v4057=(if v612{((v3440-v3979)+((((v452*v4015)-(v704*v3425))/v3429)/v715))}else{v3584});
        let v4058=(if v612{((v3441-v3980)+((((v452*v4016)-(v704*v3427))/v3429)/v715))}else{v3585});
        let v4061=(if v612{(v4015+v4041)}else{v21});
        let v4062=(if v612{(v4016+v4042)}else{v21});
        let v4065=(v720*v4061);
        let v4067=(v720*v4062);
        let v4079=((v707*v4015)+(v704*(if v612{(-((v692*v3898)+(v645*v3989)))}else{v21})));
        let v4082=((v707*v4016)+(v704*(if v612{(-((v692*v3899)+(v645*v3990)))}else{v21})));
        let v4093=(if v727{((v4065+v4065)+((v732*v4057)+(v718*(((v729*v4041)+(v713*(v42*v4041)))-v4079))))}else{v21});
        let v4094=(if v727{((v4067+v4067)+((v732*v4058)+(v718*(((v729*v4042)+(v713*(v42*v4042)))-v4082))))}else{v21});
        let v4122=(v735*v735);
        let v4134=(v713*v4041);
        let v4136=(v713*v4042);
        let v4153=(v746*v746);
        let v4161=(if v727{(v3979+(((v746*((v736*v4057)+(v718*((v720*v4015)+(v704*v4061)))))-(v737*(v4093+((v744*((v740*v4041)+(v713*(((v735*((v738*v4057)+(v718*((v720*v4057)+(v718*v4061)))))-(v739*v4093))/v4122))))+(v741*((v37*(v4134+v4134))-v4079))))))/v4153))}else{(if v724{v3979}else{v21})});
        let v4162=(if v727{(v3980+(((v746*((v736*v4058)+(v718*((v720*v4016)+(v704*v4062)))))-(v737*(v4094+((v744*((v740*v4042)+(v713*(((v735*((v738*v4058)+(v718*((v720*v4058)+(v718*v4062)))))-(v739*v4094))/v4122))))+(v741*((v37*(v4136+v4136))-v4082))))))/v4153))}else{(if v724{v3980}else{v21})});
        let v4165=(if v751{(v752*v4161)}else{v3704});
        let v4166=(if v751{(v752*v4162)}else{v3705});
        let v4168=(v753*v753);
        let v4186=(if v762{(v764*(v4161-v3440))}else{(if v751{((v753*v3471)+(v473*v4165))}else{v4165})});
        let v4187=(if v762{(v764*(v4162-v3441))}else{(if v751{((v753*v3472)+(v473*v4166))}else{v4166})});
        let v4191=(v765*v765);
        let v4199=(v3440-v4161);
        let v4200=(v3441-v4162);
        let v4219=(v778*v778);
        let v4224=(if v769{((-(v632*((v776*v4199)+(v771*((v774*(v42*v4199))+(v772*(v37*v4199)))))))/v4219)}else{v4186});
        let v4225=(if v769{((-(v632*((v776*v4200)+(v771*((v774*(v42*v4200))+(v772*(v37*v4200)))))))/v4219)}else{v4187});
        let v4244=(v788*v788);
        let v4249=(if v769{((-(v632*((v786*v4161)+(v781*((v784*(v42*v4161))+(v782*(v37*v4161)))))))/v4244)}else{(if v762{(((v765*v3471)-(v473*v4186))/v4191)}else{(if v751{((-v4165)/v4168)}else{v3711})})});
        let v4250=(if v769{((-(v632*((v786*v4162)+(v781*((v784*(v42*v4162))+(v782*(v37*v4162)))))))/v4244)}else{(if v762{(((v765*v3472)-(v473*v4187))/v4191)}else{(if v751{((-v4166)/v4168)}else{v3712})})});
        let v4251=(v749*v4161);
        let v4253=(v749*v4162);
        let v4256=(v792*v792);
        let v4264=(if v612{(self.scalar_static_f64[336]-v4161)}else{(if v612{((-(v4251+v4251))/v4256)}else{v3983})});
        let v4265=(if v612{(self.scalar_static_f64[337]-v4162)}else{(if v612{((-(v4253+v4253))/v4256)}else{v3984})});
        let v4282=(if v612{((v8*v4264)+((v800*v3425)+(v452*((v4224+(-v4249))-v3471))))}else{v3750});
        let v4283=(if v612{((v8*v4265)+((v800*v3427)+(v452*((v4225+(-v4250))-v3472))))}else{v3751});
        let v4284=(v796*v4264);
        let v4286=(v796*v4265);
        let v4320=(if v612{(-((v814*v3425)+(v452*(v4224+v4249))))}else{v4264});
        let v4321=(if v612{(-((v814*v3427)+(v452*(v4225+v4250))))}else{v4265});
        let v4322=(v803*v4282);
        let v4324=(v803*v4283);
        let v4326=(v8*(if v612{((v4284+v4284)-((v810*v3425)+(v452*((v4224+(v4161+v4249))-((v808*v3471)+(v473*v4161))))))}else{v3776}));
        let v4327=(v8*(if v612{((v4286+v4286)-((v810*v3427)+(v452*((v4225+(v4162+v4250))-((v808*v3472)+(v473*v4162))))))}else{v3777}));
        let v4338=(v8*v823);
        let v4346=(v824*v824);
        let v4354=(if v612{(v4161+(((v824*v4326)-(v819*(v4282+((if v612{((v4322+v4322)-((v819*v4320)+(v817*v4326)))}else{v4320})/v4338))))/v4346))}else{(if v497{((-v3680)-(((v607*v3794)-(v601*(v3750+(v3804/v3808))))/v3816))}else{(if v480{((v491*v3489)+(v486*((v489*v3485)+(v485*((v488*v3422)+(v451*((v487*self.scalar_static_f64[336])+(v478*v3493))))))))}else{v21})})});
        let v4355=(if v612{(v4162+(((v824*v4327)-(v819*(v4283+((if v612{((v4324+v4324)-((v819*v4321)+(v817*v4327)))}else{v4321})/v4338))))/v4346))}else{(if v497{((-v3681)-(((v607*v3795)-(v601*(v3751+(v3805/v3808))))/v3816))}else{(if v480{((v491*v3492)+(v486*((v489*v3486)+(v485*((v488*v3423)+(v451*((v487*self.scalar_static_f64[337])+(v478*v3494))))))))}else{v21})})});
        let v4364=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(self.scalar_static_f64[19]-(self.scalar_static_f64[165]*v4354))))}else{v21});
        let v4365=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(self.scalar_static_f64[145]-(self.scalar_static_f64[165]*v4355))))}else{v21});
        let v4366=(self.scalar_static_f64[217]*v4364);
        let v4367=(self.scalar_static_f64[217]*v4365);
        let v4384=(if v857{(-v4364)}else{v21});
        let v4385=(if v857{(-v4365)}else{v21});
        let v4390=(if v857{(self.scalar_static_f64[217]*(v160*v4384))}else{v21});
        let v4391=(if v857{(self.scalar_static_f64[217]*(v160*v4385))}else{v21});
        let v4392=(v864*v4390);
        let v4394=(v864*v4391);
        let v4396=(v8*v867);
        let v4403=(if v857{(v42*(v4390-((v4392+v4392)/v4396)))}else{v21});
        let v4404=(if v857{(v42*(v4391-((v4394+v4394)/v4396)))}else{v21});
        let v4407=(if v857{(v4384-v4403)}else{v21});
        let v4408=(if v857{(v4385-v4404)}else{v21});
        let v4409=(v872*v4407);
        let v4411=(v872*v4408);
        let v4417=(if v857{((v4409+v4409)+(self.scalar_static_f64[213]*v4403))}else{v21});
        let v4418=(if v857{((v4411+v4411)+(self.scalar_static_f64[213]*v4404))}else{v21});
        let v4421=(if v857{(v8*v4407)}else{v21});
        let v4422=(if v857{(v8*v4408)}else{v21});
        let v4431=(if v857{((-v4403)+((self.scalar_static_f64[214]*v4417)/v882))}else{v21});
        let v4432=(if v857{((-v4404)+((self.scalar_static_f64[214]*v4418)/v882))}else{v21});
        let v4435=(if v857{(v4417+v4421)}else{v21});
        let v4436=(if v857{(v4418+v4422)}else{v21});
        let v4437=(v887*v4435);
        let v4439=(v887*v4436);
        let v4459=(if v857{((v4437+v4437)+((v891*v4431)+(v885*(((v889*v4421)+(v880*(v42*v4421)))-v4417))))}else{v21});
        let v4460=(if v857{((v4439+v4439)+((v891*v4432)+(v885*(((v889*v4422)+(v880*(v42*v4422)))-v4418))))}else{v21});
        let v4488=(v894*v894);
        let v4500=(v880*v4421);
        let v4502=(v880*v4422);
        let v4519=(v905*v905);
        let v4527=(if v857{(v4403+(((v905*((v895*v4431)+(v885*((v887*v4417)+(v877*v4435)))))-(v896*(v4459+((v903*((v899*v4421)+(v880*(((v894*((v897*v4431)+(v885*((v887*v4431)+(v885*v4435)))))-(v898*v4459))/v4488))))+(v900*((v37*(v4500+v4500))-v4417))))))/v4519))}else{v21});
        let v4528=(if v857{(v4404+(((v905*((v895*v4432)+(v885*((v887*v4418)+(v877*v4436)))))-(v896*(v4460+((v903*((v899*v4422)+(v880*(((v894*((v897*v4432)+(v885*((v887*v4432)+(v885*v4436)))))-(v898*v4460))/v4488))))+(v900*((v37*(v4502+v4502))-v4418))))))/v4519))}else{v21});
        let v4551=(if v914{(v558*((v920*v4527)+(v915*((v918*(v42*v4527))+(v916*(v37*v4527))))))}else{(if v910{(v911*v4527)}else{v21})});
        let v4552=(if v914{(v558*((v920*v4528)+(v915*((v918*(v42*v4528))+(v916*(v37*v4528))))))}else{(if v910{(v911*v4528)}else{v21})});
        let v4554=(v924*v924);
        let v4558=(if v857{((-v4551)/v4554)}else{v21});
        let v4559=(if v857{((-v4552)/v4554)}else{v21});
        let v4560=(v908*v4527);
        let v4562=(v908*v4528);
        let v4565=(v928*v928);
        let v4573=(if v857{(v4384-v4527)}else{(if v857{((-(v4560+v4560))/v4565)}else{v4407})});
        let v4574=(if v857{(v4385-v4528)}else{(if v857{((-(v4562+v4562))/v4565)}else{v4408})});
        let v4577=(if v857{(self.scalar_static_f64[239]*v4558)}else{v21});
        let v4578=(if v857{(self.scalar_static_f64[239]*v4559)}else{v21});
        let v4587=(if v857{((v8*v4573)+(self.scalar_static_f64[213]*(v4551-v4577)))}else{v21});
        let v4588=(if v857{((v8*v4574)+(self.scalar_static_f64[213]*(v4552-v4578)))}else{v21});
        let v4589=(v932*v4573);
        let v4591=(v932*v4574);
        let v4605=(if v857{((v4589+v4589)-(self.scalar_static_f64[213]*((v4577+(v4551-v4527))+(self.scalar_static_f64[239]*v4527))))}else{v21});
        let v4606=(if v857{((v4591+v4591)-(self.scalar_static_f64[213]*((v4578+(v4552-v4528))+(self.scalar_static_f64[239]*v4528))))}else{v21});
        let v4613=(if v857{(-(self.scalar_static_f64[213]*(v4551+v4577)))}else{v4573});
        let v4614=(if v857{(-(self.scalar_static_f64[213]*(v4552+v4578)))}else{v4574});
        let v4615=(v941*v4587);
        let v4617=(v941*v4588);
        let v4619=(v8*v4605);
        let v4620=(v8*v4606);
        let v4629=(if v857{((v4615+v4615)-((v957*v4613)+(v955*v4619)))}else{v4613});
        let v4630=(if v857{((v4617+v4617)-((v957*v4614)+(v955*v4620)))}else{v4614});
        let v4633=(v8*v962);
        let v4641=(v963*v963);
        let v4659=(if v968{((v979*v4366)+(v845*(v977*v4364)))}else{v21});
        let v4660=(if v968{((v979*v4367)+(v845*(v977*v4365)))}else{v21});
        let v4685=(v996*v996);
        let v4690=(if v988{((-(v632*((v994*v4659)+(v989*((v992*(v42*v4659))+(v990*(v37*v4659)))))))/v4685)}else{(if v984{(v985*(-v4659))}else{v4629})});
        let v4691=(if v988{((-(v632*((v994*v4660)+(v989*((v992*(v42*v4660))+(v990*(v37*v4660)))))))/v4685)}else{(if v984{(v985*(-v4660))}else{v4630})});
        let v4698=(v8*v1006);
        let v4705=(if v968{(v4364-(self.scalar_static_f64[212]*((v4364-(if v968{(-v4690)}else{v21}))/v4698)))}else{v21});
        let v4706=(if v968{(v4365-(self.scalar_static_f64[212]*((v4365-(if v968{(-v4691)}else{v21}))/v4698)))}else{v21});
        let v4707=(-v4705);
        let v4708=(-v4706);
        let v4709=(v1012*v4707);
        let v4711=(v1012*v4708);
        let v4713=(v8*v1016);
        let v4722=(v1020*v4705);
        let v4724=(v1020*v4706);
        let v4726=(v8*v1024);
        let v4733=(v1025*v1025);
        let v4748=(if v968{(if v1013{(-(v42*(v4707+((v4709+v4709)/v4713))))}else{(if v1021{(-((-(v668*(v4705+((v4722+v4722)/v4726))))/v4733))}else{(-(v42*v4707))})})}else{v4403});
        let v4749=(if v968{(if v1013{(-(v42*(v4708+((v4711+v4711)/v4713))))}else{(if v1021{(-((-(v668*(v4706+((v4724+v4724)/v4726))))/v4733))}else{(-(v42*v4708))})})}else{v4404});
        let v4752=(if v968{(v4364-v4748)}else{v4690});
        let v4753=(if v968{(v4365-v4749)}else{v4691});
        let v4754=(-v4748);
        let v4755=(-v4749);
        let v4758=(if v968{(v1043*v4754)}else{v4577});
        let v4759=(if v968{(v1043*v4755)}else{v4578});
        let v4760=(v1041*v4752);
        let v4762=(v1041*v4753);
        let v4776=(if v968{(if v1053{v21}else{((v4760+v4760)-(self.scalar_static_f64[213]*((v4748+v4758)-(self.scalar_static_f64[239]*v4748))))})}else{v4417});
        let v4777=(if v968{(if v1053{v21}else{((v4762+v4762)-(self.scalar_static_f64[213]*((v4749+v4759)-(self.scalar_static_f64[239]*v4749))))})}else{v4418});
        let v4792=(if v968{((v8*v4752)+(self.scalar_static_f64[213]*(-v4758)))}else{v4421});
        let v4793=(if v968{((v8*v4753)+(self.scalar_static_f64[213]*(-v4759)))}else{v4422});
        let v4800=(if v968{(v4754+((v4776/self.scalar_static_f64[213])/v1066))}else{v4431});
        let v4801=(if v968{(v4755+((v4777/self.scalar_static_f64[213])/v1066))}else{v4432});
        let v4804=(if v968{(v4776+v4792)}else{v21});
        let v4805=(if v968{(v4777+v4793)}else{v21});
        let v4808=(v1071*v4804);
        let v4810=(v1071*v4805);
        let v4822=((v1058*v4776)+(v1055*(if v968{(-(self.scalar_static_f64[321]*v4758))}else{v21})));
        let v4825=((v1058*v4777)+(v1055*(if v968{(-(self.scalar_static_f64[321]*v4759))}else{v21})));
        let v4836=(if v1077{((v4808+v4808)+((v1082*v4800)+(v1069*(((v1079*v4792)+(v1064*(v42*v4792)))-v4822))))}else{v21});
        let v4837=(if v1077{((v4810+v4810)+((v1082*v4801)+(v1069*(((v1079*v4793)+(v1064*(v42*v4793)))-v4825))))}else{v21});
        let v4865=(v1085*v1085);
        let v4877=(v1064*v4792);
        let v4879=(v1064*v4793);
        let v4896=(v1096*v1096);
        let v4904=(if v1077{(v4748+(((v1096*((v1086*v4800)+(v1069*((v1071*v4776)+(v1055*v4804)))))-(v1087*(v4836+((v1094*((v1090*v4792)+(v1064*(((v1085*((v1088*v4800)+(v1069*((v1071*v4800)+(v1069*v4804)))))-(v1089*v4836))/v4865))))+(v1091*((v37*(v4877+v4877))-v4822))))))/v4896))}else{(if v1074{v4748}else{v21})});
        let v4905=(if v1077{(v4749+(((v1096*((v1086*v4801)+(v1069*((v1071*v4777)+(v1055*v4805)))))-(v1087*(v4837+((v1094*((v1090*v4793)+(v1064*(((v1085*((v1088*v4801)+(v1069*((v1071*v4801)+(v1069*v4805)))))-(v1089*v4837))/v4865))))+(v1091*((v37*(v4879+v4879))-v4825))))))/v4896))}else{(if v1074{v4749}else{v21})});
        let v4908=(if v1101{(v1102*v4904)}else{v4551});
        let v4909=(if v1101{(v1102*v4905)}else{v4552});
        let v4911=(v1103*v1103);
        let v4923=(if v1112{(v1114*v4904)}else{(if v1101{(self.scalar_static_f64[239]*v4908)}else{v4908})});
        let v4924=(if v1112{(v1114*v4905)}else{(if v1101{(self.scalar_static_f64[239]*v4909)}else{v4909})});
        let v4927=(v1115*v1115);
        let v4934=(-v4904);
        let v4935=(-v4905);
        let v4954=(v1128*v1128);
        let v4959=(if v1119{((-(v632*((v1126*v4934)+(v1121*((v1124*(v42*v4934))+(v1122*(v37*v4934)))))))/v4954)}else{v4923});
        let v4960=(if v1119{((-(v632*((v1126*v4935)+(v1121*((v1124*(v42*v4935))+(v1122*(v37*v4935)))))))/v4954)}else{v4924});
        let v4979=(v1138*v1138);
        let v4984=(if v1119{((-(v632*((v1136*v4904)+(v1131*((v1134*(v42*v4904))+(v1132*(v37*v4904)))))))/v4979)}else{(if v1112{((-(self.scalar_static_f64[239]*v4923))/v4927)}else{(if v1101{((-v4908)/v4911)}else{v4558})})});
        let v4985=(if v1119{((-(v632*((v1136*v4905)+(v1131*((v1134*(v42*v4905))+(v1132*(v37*v4905)))))))/v4979)}else{(if v1112{((-(self.scalar_static_f64[239]*v4924))/v4927)}else{(if v1101{((-v4909)/v4911)}else{v4559})})});
        let v4986=(v1099*v4904);
        let v4988=(v1099*v4905);
        let v4991=(v1142*v1142);
        let v4999=(if v968{(v4364-v4904)}else{(if v968{((-(v4986+v4986))/v4991)}else{v4752})});
        let v5000=(if v968{(v4365-v4905)}else{(if v968{((-(v4988+v4988))/v4991)}else{v4753})});
        let v5011=(if v968{((v8*v4999)+(self.scalar_static_f64[213]*(v4959+(-v4984))))}else{v4587});
        let v5012=(if v968{((v8*v5000)+(self.scalar_static_f64[213]*(v4960+(-v4985))))}else{v4588});
        let v5013=(v1146*v4999);
        let v5015=(v1146*v5000);
        let v5037=(if v968{(-(self.scalar_static_f64[213]*(v4959+v4984)))}else{v4999});
        let v5038=(if v968{(-(self.scalar_static_f64[213]*(v4960+v4985)))}else{v5000});
        let v5039=(v1153*v5011);
        let v5041=(v1153*v5012);
        let v5043=(v8*(if v968{((v5013+v5013)-(self.scalar_static_f64[213]*((v4959+(v4904+v4984))-(self.scalar_static_f64[239]*v4904))))}else{v4605}));
        let v5044=(v8*(if v968{((v5015+v5015)-(self.scalar_static_f64[213]*((v4960+(v4905+v4985))-(self.scalar_static_f64[239]*v4905))))}else{v4606}));
        let v5055=(v8*v1173);
        let v5063=(v1174*v1174);
        let v5083=(if self.scalar_static_bool[16]{((self.scalar_static_f64[19]-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v968{(v4904+(((v1174*v5043)-(v1169*(v5011+((if v968{((v5039+v5039)-((v1169*v5037)+(v1167*v5043)))}else{v5037})/v5055))))/v5063))}else{(if v857{((-v4527)-(((v963*v4619)-(v957*(v4587+(v4629/v4633))))/v4641))}else{(if v840{((v850*v4366)+(v845*(v844*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v4364)))))}else{v21})})})))}else{v21}))/self.scalar_static_f64[165])}else{self.scalar_static_f64[336]});
        let v5084=(if self.scalar_static_bool[16]{((self.scalar_static_f64[145]-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v968{(v4905+(((v1174*v5044)-(v1169*(v5012+((if v968{((v5041+v5041)-((v1169*v5038)+(v1167*v5044)))}else{v5038})/v5055))))/v5063))}else{(if v857{((-v4528)-(((v963*v4620)-(v957*(v4588+(v4630/v4633))))/v4641))}else{(if v840{((v850*v4367)+(v845*(v844*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v4365)))))}else{v21})})})))}else{v21}))/self.scalar_static_f64[165])}else{self.scalar_static_f64[337]});
        let v5085=(if v1186{v3483}else{v21});
        let v5086=(if v1186{v3484}else{v21});
        let v5089=((v1183*v3437)+(v456*v5083));
        let v5092=((v1183*v3439)+(v456*v5084));
        let v5121=(if v1198{(-v5083)}else{v21});
        let v5122=(if v1198{(-v5084)}else{v21});
        let v5131=(if v1198{((v1201*v3437)+(v456*(v160*v5121)))}else{v21});
        let v5132=(if v1198{((v1201*v3439)+(v456*(v160*v5122)))}else{v21});
        let v5133=(v1205*v5131);
        let v5135=(v1205*v5132);
        let v5137=(v8*v1208);
        let v5144=(if v1198{(v42*(v5131-((v5133+v5133)/v5137)))}else{v21});
        let v5145=(if v1198{(v42*(v5132-((v5135+v5135)/v5137)))}else{v21});
        let v5148=(if v1198{(v5121-v5144)}else{v21});
        let v5149=(if v1198{(v5122-v5145)}else{v21});
        let v5150=(v1213*v5148);
        let v5152=(v1213*v5149);
        let v5162=(if v1198{((v5150+v5150)+((v1215*v3425)+(v452*v5144)))}else{v21});
        let v5163=(if v1198{((v5152+v5152)+((v1215*v3427)+(v452*v5145)))}else{v21});
        let v5168=(if v1198{((v8*v5148)-v3425)}else{v21});
        let v5169=(if v1198{((v8*v5149)-v3427)}else{v21});
        let v5182=(if v1198{((-v5144)+(((v1218*v3430)+(v453*v5162))/v1223))}else{v21});
        let v5183=(if v1198{((-v5145)+(((v1218*v3432)+(v453*v5163))/v1223))}else{v21});
        let v5186=(if v1198{(v5162+v5168)}else{v21});
        let v5187=(if v1198{(v5163+v5169)}else{v21});
        let v5188=(v1228*v5186);
        let v5190=(v1228*v5187);
        let v5210=(if v1198{((v5188+v5188)+((v1232*v5182)+(v1226*(((v1230*v5168)+(v1221*(v42*v5168)))-v5162))))}else{v21});
        let v5211=(if v1198{((v5190+v5190)+((v1232*v5183)+(v1226*(((v1230*v5169)+(v1221*(v42*v5169)))-v5163))))}else{v21});
        let v5239=(v1235*v1235);
        let v5251=(v1221*v5168);
        let v5253=(v1221*v5169);
        let v5270=(v1246*v1246);
        let v5278=(if v1198{(v5144+(((v1246*((v1236*v5182)+(v1226*((v1228*v5162)+(v1218*v5186)))))-(v1237*(v5210+((v1244*((v1240*v5168)+(v1221*(((v1235*((v1238*v5182)+(v1226*((v1228*v5182)+(v1226*v5186)))))-(v1239*v5210))/v5239))))+(v1241*((v37*(v5251+v5251))-v5162))))))/v5270))}else{v21});
        let v5279=(if v1198{(v5145+(((v1246*((v1236*v5183)+(v1226*((v1228*v5163)+(v1218*v5187)))))-(v1237*(v5211+((v1244*((v1240*v5169)+(v1221*(((v1235*((v1238*v5183)+(v1226*((v1228*v5183)+(v1226*v5187)))))-(v1239*v5211))/v5239))))+(v1241*((v37*(v5253+v5253))-v5163))))))/v5270))}else{v21});
        let v5302=(if v1255{(v558*((v1261*v5278)+(v1256*((v1259*(v42*v5278))+(v1257*(v37*v5278))))))}else{(if v1251{(v1252*v5278)}else{v21})});
        let v5303=(if v1255{(v558*((v1261*v5279)+(v1256*((v1259*(v42*v5279))+(v1257*(v37*v5279))))))}else{(if v1251{(v1252*v5279)}else{v21})});
        let v5305=(v1265*v1265);
        let v5309=(if v1198{((-v5302)/v5305)}else{v21});
        let v5310=(if v1198{((-v5303)/v5305)}else{v21});
        let v5311=(v1249*v5278);
        let v5313=(v1249*v5279);
        let v5316=(v1269*v1269);
        let v5324=(if v1198{(v5121-v5278)}else{(if v1198{((-(v5311+v5311))/v5316)}else{v5148})});
        let v5325=(if v1198{(v5122-v5279)}else{(if v1198{((-(v5313+v5313))/v5316)}else{v5149})});
        let v5332=(if v1198{((v1267*v3471)+(v473*v5309))}else{v5085});
        let v5333=(if v1198{((v1267*v3472)+(v473*v5310))}else{v5086});
        let v5348=(if v1198{((v8*v5324)+((v1279*v3425)+(v452*(v3471+(v5302-v5332)))))}else{v21});
        let v5349=(if v1198{((v8*v5325)+((v1279*v3427)+(v452*(v3472+(v5303-v5333)))))}else{v21});
        let v5350=(v1273*v5324);
        let v5352=(v1273*v5325);
        let v5374=(if v1198{((v5350+v5350)-((v1289*v3425)+(v452*((v5332+(v5302-v5278))+((v1287*v3471)+(v473*v5278))))))}else{v21});
        let v5375=(if v1198{((v5352+v5352)-((v1289*v3427)+(v452*((v5333+(v5303-v5279))+((v1287*v3472)+(v473*v5279))))))}else{v21});
        let v5386=(if v1198{(-((v1293*v3425)+(v452*(v5302+v5332))))}else{v5324});
        let v5387=(if v1198{(-((v1293*v3427)+(v452*(v5303+v5333))))}else{v5325});
        let v5388=(v1282*v5348);
        let v5390=(v1282*v5349);
        let v5392=(v8*v5374);
        let v5393=(v8*v5375);
        let v5402=(if v1198{((v5388+v5388)-((v1298*v5386)+(v1296*v5392)))}else{v5386});
        let v5403=(if v1198{((v5390+v5390)-((v1298*v5387)+(v1296*v5393)))}else{v5387});
        let v5406=(v8*v1303);
        let v5414=(v1304*v1304);
        let v5424=(if v1309{v3828}else{v21});
        let v5425=(if v1309{v3830}else{v21});
        let v5452=(if v1309{((v1316*v5089)+(v1188*((v1314*v5083)+(v1183*(if v1309{((v1312*v5424)+(v1310*((v1310*v3833)+(v615*v5424))))}else{v21})))))}else{v21});
        let v5453=(if v1309{((v1316*v5092)+(v1188*((v1314*v5084)+(v1183*(if v1309{((v1312*v5425)+(v1310*((v1310*v3834)+(v615*v5425))))}else{v21})))))}else{v21});
        let v5478=(v1333*v1333);
        let v5483=(if v1325{((-(v632*((v1331*v5452)+(v1326*((v1329*(v42*v5452))+(v1327*(v37*v5452)))))))/v5478)}else{(if v1321{(v1322*(-v5452))}else{v5402})});
        let v5484=(if v1325{((-(v632*((v1331*v5453)+(v1326*((v1329*(v42*v5453))+(v1327*(v37*v5453)))))))/v5478)}else{(if v1321{(v1322*(-v5453))}else{v5403})});
        let v5495=(v8*v1341);
        let v5506=(if v1309{((v3898+v5083)-((v1341*v3422)+(v451*(((v3902+v5083)-(if v1309{(-v5483)}else{v21}))/v5495))))}else{v21});
        let v5507=(if v1309{((v3899+v5084)-((v1341*v3423)+(v451*(((v3903+v5084)-(if v1309{(-v5484)}else{v21}))/v5495))))}else{v21});
        let v5508=(if v1309{v3440}else{v21});
        let v5509=(if v1309{v3441}else{v21});
        let v5510=(v5508-v5506);
        let v5511=(v5509-v5507);
        let v5512=(v1346*v5510);
        let v5514=(v1346*v5511);
        let v5516=(v8*v1350);
        let v5525=(v5506-v5508);
        let v5526=(v5507-v5509);
        let v5527=(v1354*v5525);
        let v5529=(v1354*v5526);
        let v5531=(v8*v1358);
        let v5538=(v1359*v1359);
        let v5553=(v1345*v5508);
        let v5555=(v1345*v5509);
        let v5557=(v8*v1369);
        let v5566=(if v1309{((if v1347{(v5508-(v42*(v5510+((v5512+v5512)/v5516))))}else{(if v1355{(v5508-((-(v668*(v5525+((v5527+v5527)/v5531))))/v5538))}else{(v5508-(v42*v5510))})})-(v42*(v5508-((v5553+v5553)/v5557))))}else{v5144});
        let v5567=(if v1309{((if v1347{(v5509-(v42*(v5511+((v5514+v5514)/v5516))))}else{(if v1355{(v5509-((-(v668*(v5526+((v5529+v5529)/v5531))))/v5538))}else{(v5509-(v42*v5511))})})-(v42*(v5509-((v5555+v5555)/v5557))))}else{v5145});
        let v5570=(if v1309{(v5083-v5566)}else{v5483});
        let v5571=(if v1309{(v5084-v5567)}else{v5484});
        let v5576=(if v1309{(v1377*(-v5566))}else{v5332});
        let v5577=(if v1309{(v1377*(-v5567))}else{v5333});
        let v5578=(v1375*v5570);
        let v5580=(v1375*v5571);
        let v5602=(if v1309{(if v1387{v21}else{((v5578+v5578)-((v1384*v3425)+(v452*((v5566+v5576)-((v1382*v3471)+(v473*v5566))))))})}else{v5162});
        let v5603=(if v1309{(if v1387{v21}else{((v5580+v5580)-((v1384*v3427)+(v452*((v5567+v5577)-((v1382*v3472)+(v473*v5567))))))})}else{v5163});
        let v5628=(if v1309{((v8*v5570)+((v1395*v3425)+(v452*((-v5576)-v3471))))}else{v5168});
        let v5629=(if v1309{((v8*v5571)+((v1395*v3427)+(v452*((-v5577)-v3472))))}else{v5169});
        let v5644=(if v1309{((v3440-v5566)+((((v452*v5602)-(v1389*v3425))/v3429)/v1400))}else{v5182});
        let v5645=(if v1309{((v3441-v5567)+((((v452*v5603)-(v1389*v3427))/v3429)/v1400))}else{v5183});
        let v5648=(if v1309{(v5602+v5628)}else{v21});
        let v5649=(if v1309{(v5603+v5629)}else{v21});
        let v5652=(v1405*v5648);
        let v5654=(v1405*v5649);
        let v5666=((v1392*v5602)+(v1389*(if v1309{(-((v1378*v3898)+(v645*v5576)))}else{v21})));
        let v5669=((v1392*v5603)+(v1389*(if v1309{(-((v1378*v3899)+(v645*v5577)))}else{v21})));
        let v5680=(if v1411{((v5652+v5652)+((v1416*v5644)+(v1403*(((v1413*v5628)+(v1398*(v42*v5628)))-v5666))))}else{v21});
        let v5681=(if v1411{((v5654+v5654)+((v1416*v5645)+(v1403*(((v1413*v5629)+(v1398*(v42*v5629)))-v5669))))}else{v21});
        let v5709=(v1419*v1419);
        let v5721=(v1398*v5628);
        let v5723=(v1398*v5629);
        let v5740=(v1430*v1430);
        let v5748=(if v1411{(v5566+(((v1430*((v1420*v5644)+(v1403*((v1405*v5602)+(v1389*v5648)))))-(v1421*(v5680+((v1428*((v1424*v5628)+(v1398*(((v1419*((v1422*v5644)+(v1403*((v1405*v5644)+(v1403*v5648)))))-(v1423*v5680))/v5709))))+(v1425*((v37*(v5721+v5721))-v5666))))))/v5740))}else{(if v1408{v5566}else{v21})});
        let v5749=(if v1411{(v5567+(((v1430*((v1420*v5645)+(v1403*((v1405*v5603)+(v1389*v5649)))))-(v1421*(v5681+((v1428*((v1424*v5629)+(v1398*(((v1419*((v1422*v5645)+(v1403*((v1405*v5645)+(v1403*v5649)))))-(v1423*v5681))/v5709))))+(v1425*((v37*(v5723+v5723))-v5669))))))/v5740))}else{(if v1408{v5567}else{v21})});
        let v5752=(if v1435{(v1436*v5748)}else{v5302});
        let v5753=(if v1435{(v1436*v5749)}else{v5303});
        let v5755=(v1437*v1437);
        let v5773=(if v1445{(v1447*(v5748-v3440))}else{(if v1435{((v1437*v3471)+(v473*v5752))}else{v5752})});
        let v5774=(if v1445{(v1447*(v5749-v3441))}else{(if v1435{((v1437*v3472)+(v473*v5753))}else{v5753})});
        let v5778=(v1448*v1448);
        let v5786=(v3440-v5748);
        let v5787=(v3441-v5749);
        let v5806=(v1461*v1461);
        let v5811=(if v1452{((-(v632*((v1459*v5786)+(v1454*((v1457*(v42*v5786))+(v1455*(v37*v5786)))))))/v5806)}else{v5773});
        let v5812=(if v1452{((-(v632*((v1459*v5787)+(v1454*((v1457*(v42*v5787))+(v1455*(v37*v5787)))))))/v5806)}else{v5774});
        let v5831=(v1471*v1471);
        let v5836=(if v1452{((-(v632*((v1469*v5748)+(v1464*((v1467*(v42*v5748))+(v1465*(v37*v5748)))))))/v5831)}else{(if v1445{(((v1448*v3471)-(v473*v5773))/v5778)}else{(if v1435{((-v5752)/v5755)}else{v5309})})});
        let v5837=(if v1452{((-(v632*((v1469*v5749)+(v1464*((v1467*(v42*v5749))+(v1465*(v37*v5749)))))))/v5831)}else{(if v1445{(((v1448*v3472)-(v473*v5774))/v5778)}else{(if v1435{((-v5753)/v5755)}else{v5310})})});
        let v5838=(v1433*v5748);
        let v5840=(v1433*v5749);
        let v5843=(v1475*v1475);
        let v5851=(if v1309{(v5083-v5748)}else{(if v1309{((-(v5838+v5838))/v5843)}else{v5570})});
        let v5852=(if v1309{(v5084-v5749)}else{(if v1309{((-(v5840+v5840))/v5843)}else{v5571})});
        let v5869=(if v1309{((v8*v5851)+((v1483*v3425)+(v452*((v5811+(-v5836))-v3471))))}else{v5348});
        let v5870=(if v1309{((v8*v5852)+((v1483*v3427)+(v452*((v5812+(-v5837))-v3472))))}else{v5349});
        let v5871=(v1479*v5851);
        let v5873=(v1479*v5852);
        let v5907=(if v1309{(-((v1497*v3425)+(v452*(v5811+v5836))))}else{v5851});
        let v5908=(if v1309{(-((v1497*v3427)+(v452*(v5812+v5837))))}else{v5852});
        let v5909=(v1486*v5869);
        let v5911=(v1486*v5870);
        let v5913=(v8*(if v1309{((v5871+v5871)-((v1493*v3425)+(v452*((v5811+(v5748+v5836))-((v1491*v3471)+(v473*v5748))))))}else{v5374}));
        let v5914=(v8*(if v1309{((v5873+v5873)-((v1493*v3427)+(v452*((v5812+(v5749+v5837))-((v1491*v3472)+(v473*v5749))))))}else{v5375}));
        let v5925=(v8*v1506);
        let v5933=(v1507*v1507);
        let v5941=(if v1309{(v5748+(((v1507*v5913)-(v1502*(v5869+((if v1309{((v5909+v5909)-((v1502*v5907)+(v1500*v5913)))}else{v5907})/v5925))))/v5933))}else{(if v1198{((-v5278)-(((v1304*v5392)-(v1298*(v5348+(v5402/v5406))))/v5414))}else{(if v1186{((v1192*v5089)+(v1188*((v1190*v5085)+(v1187*((v1189*v3422)+(v451*((v1183*v3493)+(v487*v5083))))))))}else{v4354})})});
        let v5942=(if v1309{(v5749+(((v1507*v5914)-(v1502*(v5870+((if v1309{((v5911+v5911)-((v1502*v5908)+(v1500*v5914)))}else{v5908})/v5925))))/v5933))}else{(if v1198{((-v5279)-(((v1304*v5393)-(v1298*(v5349+(v5403/v5406))))/v5414))}else{(if v1186{((v1192*v5092)+(v1188*((v1190*v5086)+(v1187*((v1189*v3423)+(v451*((v1183*v3494)+(v487*v5084))))))))}else{v4355})})});
        let v5945=(if v1518{(v1519*v5941)}else{v21});
        let v5946=(if v1518{(v1519*v5942)}else{v21});
        let v5948=(v1520*v1520);
        let v5952=(if v1518{((-v5945)/v5948)}else{v21});
        let v5953=(if v1518{((-v5946)/v5948)}else{v21});
        let v5981=(if v1533{(v1535*(v5941-v3440))}else{(if v1518{((v1520*v3471)+(v473*v5945))}else{v5945})});
        let v5982=(if v1533{(v1535*(v5942-v3441))}else{(if v1518{((v1520*v3472)+(v473*v5946))}else{v5946})});
        let v5986=(v1536*v1536);
        let v6031=(v42*v5941);
        let v6032=(v42*v5942);
        let v6033=(v37*v5941);
        let v6034=(v37*v5942);
        let v6049=(v1563*v1563);
        let v6054=(if v1544{((-(v632*((v1561*v5941)+(v1556*((v1559*v6031)+(v1557*v6033))))))/v6049)}else{(if v1533{(((v1536*v3471)-(v473*v5981))/v5986)}else{v5952})});
        let v6055=(if v1544{((-(v632*((v1561*v5942)+(v1556*((v1559*v6032)+(v1557*v6034))))))/v6049)}else{(if v1533{(((v1536*v3472)-(v473*v5982))/v5986)}else{v5953})});
        let v6076=(-((v1574*v6033)+(v1572*(-(v647*v5941)))));
        let v6077=(-((v1574*v6034)+(v1572*(-(v647*v5942)))));
        let v6116=(v8*v1588);
        let v6119=(if v1569{(v6076/v6116)}else{v21});
        let v6120=(if v1569{(v6077/v6116)}else{v21});
        let v6133=(if v1594{(v5941+v6054)}else{(if v1569{((v1576*((v1570*v5941)+(v1510*v6031)))+(v1571*v6076))}else{v21})});
        let v6134=(if v1594{(v5942+v6055)}else{(if v1569{((v1576*((v1570*v5942)+(v1510*v6032)))+(v1571*v6077))}else{v21})});
        let v6135=(v8*v1598);
        let v6138=(if v1594{(v6133/v6135)}else{(if v1569{((v1590*v6119)+(v1589*(v148*v5941)))}else{v21})});
        let v6139=(if v1594{(v6134/v6135)}else{(if v1569{((v1590*v6120)+(v1589*(v148*v5942)))}else{v21})});
        let v6183=(((v455*self.scalar_static_f64[336])-(v1612*v3433))/v3436);
        let v6187=(((v455*self.scalar_static_f64[337])-(v1612*v3434))/v3436);
        let v6188=(self.scalar_static_f64[168]/v455);
        let v6203=(((v475*(((v475*v3833)-(v615*v3473))/v3827))-(v1621*v3473))/v3827);
        let v6207=(((v475*(((v475*v3834)-(v615*v3474))/v3827))-(v1621*v3474))/v3827);
        let v6226=(if v1619{((v1625*v6183)+(v1615*((v1623*self.scalar_static_f64[336])+(v1612*(if v1619{v6203}else{v21})))))}else{v21});
        let v6227=(if v1619{((v1625*v6187)+(v1615*((v1623*self.scalar_static_f64[337])+(v1612*(if v1619{v6207}else{v21})))))}else{v21});
        let v6228=(if v1619{((v1625*v6188)+(v1615*(self.scalar_static_f64[168]*v1623)))}else{v21});
        let v6264=(v1642*v1642);
        let v6272=(if v1634{((-(v170*((v1640*v6226)+(v1635*((v1638*(v42*v6226))+(v1636*(v37*v6226)))))))/v6264)}else{(if v1629{(v1631*(-v6226))}else{v21})});
        let v6273=(if v1634{((-(v170*((v1640*v6227)+(v1635*((v1638*(v42*v6227))+(v1636*(v37*v6227)))))))/v6264)}else{(if v1629{(v1631*(-v6227))}else{v21})});
        let v6274=(if v1634{((-(v170*((v1640*v6228)+(v1635*((v1638*(v42*v6228))+(v1636*(v37*v6228)))))))/v6264)}else{(if v1629{(v1631*(-v6228))}else{v21})});
        let v6278=(if v1619{(-v6272)}else{v21});
        let v6279=(if v1619{(-v6273)}else{v21});
        let v6280=(if v1619{(-v6274)}else{v21});
        let v6284=(v8*v1650);
        let v6298=(if v1619{(v3900-((v1650*v3422)+(v451*((v3904-v6278)/v6284))))}else{v21});
        let v6299=(if v1619{(v3901-((v1650*v3423)+(v451*((v3905-v6279)/v6284))))}else{v21});
        let v6300=(if v1619{(self.scalar_static_f64[168]-(v451*((self.scalar_static_f64[168]-v6280)/v6284)))}else{v21});
        let v6336=(v1668*v1668);
        let v6344=(if v1660{((-(v170*((v1666*v6298)+(v1661*((v1664*(v42*v6298))+(v1662*(v37*v6298)))))))/v6336)}else{(if v1655{(v1657*(-v6298))}else{v21})});
        let v6345=(if v1660{((-(v170*((v1666*v6299)+(v1661*((v1664*(v42*v6299))+(v1662*(v37*v6299)))))))/v6336)}else{(if v1655{(v1657*(-v6299))}else{v21})});
        let v6346=(if v1660{((-(v170*((v1666*v6300)+(v1661*((v1664*(v42*v6300))+(v1662*(v37*v6300)))))))/v6336)}else{(if v1655{(v1657*(-v6300))}else{v21})});
        let v6357=(if v1619{(-((v1670*v3898)+(v645*v6344)))}else{v21});
        let v6358=(if v1619{(-((v1670*v3899)+(v645*v6345)))}else{v21});
        let v6359=(if v1619{(-(v645*v6346))}else{v21});
        let v6360=(self.scalar_static_f64[336]-v6298);
        let v6361=(self.scalar_static_f64[337]-v6299);
        let v6362=(self.scalar_static_f64[168]-v6300);
        let v6379=(if v1619{((v8*v6360)+((v1676*v3425)+(v452*(-v6344))))}else{v21});
        let v6380=(if v1619{((v8*v6361)+((v1676*v3427)+(v452*(-v6345))))}else{v21});
        let v6381=(if v1619{((v8*v6362)+(v452*(-v6346)))}else{v21});
        let v6382=(v1674*v6360);
        let v6384=(v1674*v6361);
        let v6386=(v1674*v6362);
        let v6401=(if v1619{((v6382+v6382)-((v1682*v3425)+(v452*(v6298+v6344))))}else{v21});
        let v6402=(if v1619{((v6384+v6384)-((v1682*v3427)+(v452*(v6299+v6345))))}else{v21});
        let v6403=(if v1619{((v6386+v6386)-(v452*(v6300+v6346)))}else{v21});
        let v6404=(v1679*v6379);
        let v6406=(v1679*v6380);
        let v6408=(v1679*v6381);
        let v6425=(if v1619{((v6404+v6404)-((v1687*v6401)+(v1685*(v94*v6357))))}else{v6272});
        let v6426=(if v1619{((v6406+v6406)-((v1687*v6402)+(v1685*(v94*v6358))))}else{v6273});
        let v6427=(if v1619{((v6408+v6408)-((v1687*v6403)+(v1685*(v94*v6359))))}else{v6274});
        let v6431=(v8*v1692);
        let v6441=(v1693*v1693);
        let v6461=(if v1699{self.scalar_static_f64[338]}else{v21});
        let v6462=(if v1699{self.scalar_static_f64[339]}else{v21});
        let v6463=(if v1699{self.scalar_static_f64[340]}else{v21});
        let v6476=(if v1699{(((v455*(v160*v6461))-(v1702*v3433))/v3436)}else{v21});
        let v6477=(if v1699{(((v455*(v160*v6462))-(v1702*v3434))/v3436)}else{v21});
        let v6478=(if v1699{((v160*v6463)/v455)}else{v21});
        let v6479=(v1706*v6476);
        let v6481=(v1706*v6477);
        let v6483=(v1706*v6478);
        let v6485=(v8*v1709);
        let v6495=(if v1699{(v42*(v6476-((v6479+v6479)/v6485)))}else{v21});
        let v6496=(if v1699{(v42*(v6477-((v6481+v6481)/v6485)))}else{v21});
        let v6497=(if v1699{(v42*(v6478-((v6483+v6483)/v6485)))}else{v21});
        let v6498=(v6461-v6495);
        let v6499=(v6462-v6496);
        let v6500=(v6463-v6497);
        let v6501=(v1713*v6498);
        let v6503=(v1713*v6499);
        let v6505=(v1713*v6500);
        let v6517=(if v1699{((v6501+v6501)+((v1715*v3425)+(v452*v6495)))}else{v21});
        let v6518=(if v1699{((v6503+v6503)+((v1715*v3427)+(v452*v6496)))}else{v21});
        let v6519=(if v1699{((v6505+v6505)+(v452*v6497))}else{v21});
        let v6525=(if v1699{((v8*v6498)-v3425)}else{v21});
        let v6526=(if v1699{((v8*v6499)-v3427)}else{v21});
        let v6527=(if v1699{(v8*v6500)}else{v21});
        let v6543=(if v1699{(((((v452*v6517)-(v1718*v3425))/v3429)/v1722)-v6495)}else{v21});
        let v6544=(if v1699{(((((v452*v6518)-(v1718*v3427))/v3429)/v1722)-v6496)}else{v21});
        let v6545=(if v1699{(((v6519/v452)/v1722)-v6497)}else{v21});
        let v6549=(if v1699{(v6517+v6525)}else{v21});
        let v6550=(if v1699{(v6518+v6526)}else{v21});
        let v6551=(if v1699{(v6519+v6527)}else{v21});
        let v6552=(v1727*v6549);
        let v6554=(v1727*v6550);
        let v6556=(v1727*v6551);
        let v6585=(if v1699{((v6552+v6552)+((v1731*v6543)+(v1725*(((v1729*v6525)+(v1721*(v42*v6525)))-v6517))))}else{v21});
        let v6586=(if v1699{((v6554+v6554)+((v1731*v6544)+(v1725*(((v1729*v6526)+(v1721*(v42*v6526)))-v6518))))}else{v21});
        let v6587=(if v1699{((v6556+v6556)+((v1731*v6545)+(v1725*(((v1729*v6527)+(v1721*(v42*v6527)))-v6519))))}else{v21});
        let v6627=(v1734*v1734);
        let v6646=(v1721*v6525);
        let v6648=(v1721*v6526);
        let v6650=(v1721*v6527);
        let v6673=(v1745*v1745);
        let v6686=(if v1699{(v6495+(((v1745*((v1735*v6543)+(v1725*((v1727*v6517)+(v1718*v6549)))))-(v1736*(v6585+((v1743*((v1739*v6525)+(v1721*(((v1734*((v1737*v6543)+(v1725*((v1727*v6543)+(v1725*v6549)))))-(v1738*v6585))/v6627))))+(v1740*((v37*(v6646+v6646))-v6517))))))/v6673))}else{v21});
        let v6687=(if v1699{(v6496+(((v1745*((v1735*v6544)+(v1725*((v1727*v6518)+(v1718*v6550)))))-(v1736*(v6586+((v1743*((v1739*v6526)+(v1721*(((v1734*((v1737*v6544)+(v1725*((v1727*v6544)+(v1725*v6550)))))-(v1738*v6586))/v6627))))+(v1740*((v37*(v6648+v6648))-v6518))))))/v6673))}else{v21});
        let v6688=(if v1699{(v6497+(((v1745*((v1735*v6545)+(v1725*((v1727*v6519)+(v1718*v6551)))))-(v1736*(v6587+((v1743*((v1739*v6527)+(v1721*(((v1734*((v1737*v6545)+(v1725*((v1727*v6545)+(v1725*v6551)))))-(v1738*v6587))/v6627))))+(v1740*((v37*(v6650+v6650))-v6519))))))/v6673))}else{v21});
        let v6695=(-v6686);
        let v6696=(-v6687);
        let v6697=(-v6688);
        let v6724=(v1765*v1765);
        let v6762=(if v1769{(v558*((v1775*v6686)+(v1770*((v1773*(v42*v6686))+(v1771*(v37*v6686))))))}else{(if v1757{((-(v632*((v1763*v6695)+(v1758*((v1761*(v42*v6695))+(v1759*(v37*v6695)))))))/v6724)}else{(if v1751{(v1752*v6686)}else{v6344})})});
        let v6763=(if v1769{(v558*((v1775*v6687)+(v1770*((v1773*(v42*v6687))+(v1771*(v37*v6687))))))}else{(if v1757{((-(v632*((v1763*v6696)+(v1758*((v1761*(v42*v6696))+(v1759*(v37*v6696)))))))/v6724)}else{(if v1751{(v1752*v6687)}else{v6345})})});
        let v6764=(if v1769{(v558*((v1775*v6688)+(v1770*((v1773*(v42*v6688))+(v1771*(v37*v6688))))))}else{(if v1757{((-(v632*((v1763*v6697)+(v1758*((v1761*(v42*v6697))+(v1759*(v37*v6697)))))))/v6724)}else{(if v1751{(v1752*v6688)}else{v6346})})});
        let v6778=(v6461-v6686);
        let v6779=(v6462-v6687);
        let v6780=(v6463-v6688);
        let v6794=(if v1699{((v8*v6778)+((v1785*v3425)+(v452*v6762)))}else{v6379});
        let v6795=(if v1699{((v8*v6779)+((v1785*v3427)+(v452*v6763)))}else{v6380});
        let v6796=(if v1699{((v8*v6780)+(v452*v6764))}else{v6381});
        let v6797=(v1783*v6778);
        let v6799=(v1783*v6779);
        let v6801=(v1783*v6780);
        let v6816=(if v1699{((v6797+v6797)+((v1791*v3425)+(v452*(v6686-v6762))))}else{v6401});
        let v6817=(if v1699{((v6799+v6799)+((v1791*v3427)+(v452*(v6687-v6763))))}else{v6402});
        let v6818=(if v1699{((v6801+v6801)+(v452*(v6688-v6764)))}else{v6403});
        let v6819=(v1788*v6794);
        let v6821=(v1788*v6795);
        let v6823=(v1788*v6796);
        let v6846=(v8*v1801);
        let v6856=(v1802*v1802);
        let v6875=(if v1699{(-(v6686+(if v1699{(((v1802*(v8*v6816))-(v1800*(v6794+((if v1699{((v6819+v6819)-((v1796*v6816)+(v1794*(v94*(if v1699{(-((v1779*v3898)+(v645*v6762)))}else{v6357})))))}else{v6425})/v6846))))/v6856)}else{v6278})))}else{(if v1619{(v6298+(if v1619{(((v1693*(v8*v6401))-(v1691*(v6379+(v6425/v6431))))/v6441)}else{v21}))}else{(if v1614{v6183}else{v21})})});
        let v6876=(if v1699{(-(v6687+(if v1699{(((v1802*(v8*v6817))-(v1800*(v6795+((if v1699{((v6821+v6821)-((v1796*v6817)+(v1794*(v94*(if v1699{(-((v1779*v3899)+(v645*v6763)))}else{v6358})))))}else{v6426})/v6846))))/v6856)}else{v6279})))}else{(if v1619{(v6299+(if v1619{(((v1693*(v8*v6402))-(v1691*(v6380+(v6426/v6431))))/v6441)}else{v21}))}else{(if v1614{v6187}else{v21})})});
        let v6877=(if v1699{(-(v6688+(if v1699{(((v1802*(v8*v6818))-(v1800*(v6796+((if v1699{((v6823+v6823)-((v1796*v6818)+(v1794*(v94*(if v1699{(-(v645*v6764))}else{v6359})))))}else{v6427})/v6846))))/v6856)}else{v6280})))}else{(if v1619{(v6300+(if v1619{(((v1693*(v8*v6403))-(v1691*(v6381+(v6427/v6431))))/v6441)}else{v21}))}else{(if v1614{v6188}else{v21})})});
        let v6878=(self.scalar_static_f64[165]*v6875);
        let v6879=(self.scalar_static_f64[165]*v6876);
        let v6880=(self.scalar_static_f64[165]*v6877);
        let v6890=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(self.scalar_static_f64[19]-v6878)))}else{v21});
        let v6891=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(self.scalar_static_f64[145]-v6879)))}else{v21});
        let v6892=(if self.scalar_static_bool[16]{(self.scalar_static_f64[168]*(self.scalar_static_f64[147]*(-v6880)))}else{v21});
        let v6893=(self.scalar_static_f64[217]*v6890);
        let v6894=(self.scalar_static_f64[217]*v6891);
        let v6895=(self.scalar_static_f64[217]*v6892);
        let v6920=(if v1827{(-v6890)}else{v21});
        let v6921=(if v1827{(-v6891)}else{v21});
        let v6922=(if v1827{(-v6892)}else{v21});
        let v6929=(if v1827{(self.scalar_static_f64[217]*(v160*v6920))}else{v21});
        let v6930=(if v1827{(self.scalar_static_f64[217]*(v160*v6921))}else{v21});
        let v6931=(if v1827{(self.scalar_static_f64[217]*(v160*v6922))}else{v21});
        let v6932=(v1834*v6929);
        let v6934=(v1834*v6930);
        let v6936=(v1834*v6931);
        let v6938=(v8*v1837);
        let v6948=(if v1827{(v42*(v6929-((v6932+v6932)/v6938)))}else{v21});
        let v6949=(if v1827{(v42*(v6930-((v6934+v6934)/v6938)))}else{v21});
        let v6950=(if v1827{(v42*(v6931-((v6936+v6936)/v6938)))}else{v21});
        let v6954=(if v1827{(v6920-v6948)}else{v21});
        let v6955=(if v1827{(v6921-v6949)}else{v21});
        let v6956=(if v1827{(v6922-v6950)}else{v21});
        let v6957=(v1842*v6954);
        let v6959=(v1842*v6955);
        let v6961=(v1842*v6956);
        let v6969=(if v1827{((v6957+v6957)+(self.scalar_static_f64[213]*v6948))}else{v21});
        let v6970=(if v1827{((v6959+v6959)+(self.scalar_static_f64[213]*v6949))}else{v21});
        let v6971=(if v1827{((v6961+v6961)+(self.scalar_static_f64[213]*v6950))}else{v21});
        let v6975=(if v1827{(v8*v6954)}else{v21});
        let v6976=(if v1827{(v8*v6955)}else{v21});
        let v6977=(if v1827{(v8*v6956)}else{v21});
        let v6990=(if v1827{((-v6948)+((self.scalar_static_f64[214]*v6969)/v1852))}else{v21});
        let v6991=(if v1827{((-v6949)+((self.scalar_static_f64[214]*v6970)/v1852))}else{v21});
        let v6992=(if v1827{((-v6950)+((self.scalar_static_f64[214]*v6971)/v1852))}else{v21});
        let v6996=(if v1827{(v6969+v6975)}else{v21});
        let v6997=(if v1827{(v6970+v6976)}else{v21});
        let v6998=(if v1827{(v6971+v6977)}else{v21});
        let v6999=(v1857*v6996);
        let v7001=(v1857*v6997);
        let v7003=(v1857*v6998);
        let v7032=(if v1827{((v6999+v6999)+((v1861*v6990)+(v1855*(((v1859*v6975)+(v1850*(v42*v6975)))-v6969))))}else{v21});
        let v7033=(if v1827{((v7001+v7001)+((v1861*v6991)+(v1855*(((v1859*v6976)+(v1850*(v42*v6976)))-v6970))))}else{v21});
        let v7034=(if v1827{((v7003+v7003)+((v1861*v6992)+(v1855*(((v1859*v6977)+(v1850*(v42*v6977)))-v6971))))}else{v21});
        let v7074=(v1864*v1864);
        let v7093=(v1850*v6975);
        let v7095=(v1850*v6976);
        let v7097=(v1850*v6977);
        let v7120=(v1875*v1875);
        let v7133=(if v1827{(v6948+(((v1875*((v1865*v6990)+(v1855*((v1857*v6969)+(v1847*v6996)))))-(v1866*(v7032+((v1873*((v1869*v6975)+(v1850*(((v1864*((v1867*v6990)+(v1855*((v1857*v6990)+(v1855*v6996)))))-(v1868*v7032))/v7074))))+(v1870*((v37*(v7093+v7093))-v6969))))))/v7120))}else{v21});
        let v7134=(if v1827{(v6949+(((v1875*((v1865*v6991)+(v1855*((v1857*v6970)+(v1847*v6997)))))-(v1866*(v7033+((v1873*((v1869*v6976)+(v1850*(((v1864*((v1867*v6991)+(v1855*((v1857*v6991)+(v1855*v6997)))))-(v1868*v7033))/v7074))))+(v1870*((v37*(v7095+v7095))-v6970))))))/v7120))}else{v21});
        let v7135=(if v1827{(v6950+(((v1875*((v1865*v6992)+(v1855*((v1857*v6971)+(v1847*v6998)))))-(v1866*(v7034+((v1873*((v1869*v6977)+(v1850*(((v1864*((v1867*v6992)+(v1855*((v1857*v6992)+(v1855*v6998)))))-(v1868*v7034))/v7074))))+(v1870*((v37*(v7097+v7097))-v6971))))))/v7120))}else{v21});
        let v7169=(if v1884{(v558*((v1890*v7133)+(v1885*((v1888*(v42*v7133))+(v1886*(v37*v7133))))))}else{(if v1880{(v1881*v7133)}else{v21})});
        let v7170=(if v1884{(v558*((v1890*v7134)+(v1885*((v1888*(v42*v7134))+(v1886*(v37*v7134))))))}else{(if v1880{(v1881*v7134)}else{v21})});
        let v7171=(if v1884{(v558*((v1890*v7135)+(v1885*((v1888*(v42*v7135))+(v1886*(v37*v7135))))))}else{(if v1880{(v1881*v7135)}else{v21})});
        let v7173=(v1894*v1894);
        let v7179=(if v1827{((-v7169)/v7173)}else{v21});
        let v7180=(if v1827{((-v7170)/v7173)}else{v21});
        let v7181=(if v1827{((-v7171)/v7173)}else{v21});
        let v7182=(v1878*v7133);
        let v7184=(v1878*v7134);
        let v7186=(v1878*v7135);
        let v7189=(v1898*v1898);
        let v7201=(if v1827{(v6920-v7133)}else{(if v1827{((-(v7182+v7182))/v7189)}else{v6954})});
        let v7202=(if v1827{(v6921-v7134)}else{(if v1827{((-(v7184+v7184))/v7189)}else{v6955})});
        let v7203=(if v1827{(v6922-v7135)}else{(if v1827{((-(v7186+v7186))/v7189)}else{v6956})});
        let v7207=(if v1827{(self.scalar_static_f64[239]*v7179)}else{v21});
        let v7208=(if v1827{(self.scalar_static_f64[239]*v7180)}else{v21});
        let v7209=(if v1827{(self.scalar_static_f64[239]*v7181)}else{v21});
        let v7222=(if v1827{((v8*v7201)+(self.scalar_static_f64[213]*(v7169-v7207)))}else{v21});
        let v7223=(if v1827{((v8*v7202)+(self.scalar_static_f64[213]*(v7170-v7208)))}else{v21});
        let v7224=(if v1827{((v8*v7203)+(self.scalar_static_f64[213]*(v7171-v7209)))}else{v21});
        let v7225=(v1902*v7201);
        let v7227=(v1902*v7202);
        let v7229=(v1902*v7203);
        let v7249=(if v1827{((v7225+v7225)-(self.scalar_static_f64[213]*((v7207+(v7169-v7133))+(self.scalar_static_f64[239]*v7133))))}else{v21});
        let v7250=(if v1827{((v7227+v7227)-(self.scalar_static_f64[213]*((v7208+(v7170-v7134))+(self.scalar_static_f64[239]*v7134))))}else{v21});
        let v7251=(if v1827{((v7229+v7229)-(self.scalar_static_f64[213]*((v7209+(v7171-v7135))+(self.scalar_static_f64[239]*v7135))))}else{v21});
        let v7261=(if v1827{(-(self.scalar_static_f64[213]*(v7169+v7207)))}else{v7201});
        let v7262=(if v1827{(-(self.scalar_static_f64[213]*(v7170+v7208)))}else{v7202});
        let v7263=(if v1827{(-(self.scalar_static_f64[213]*(v7171+v7209)))}else{v7203});
        let v7264=(v1911*v7222);
        let v7266=(v1911*v7223);
        let v7268=(v1911*v7224);
        let v7270=(v8*v7249);
        let v7271=(v8*v7250);
        let v7272=(v8*v7251);
        let v7285=(if v1827{((v7264+v7264)-((v1927*v7261)+(v1925*v7270)))}else{v7261});
        let v7286=(if v1827{((v7266+v7266)-((v1927*v7262)+(v1925*v7271)))}else{v7262});
        let v7287=(if v1827{((v7268+v7268)-((v1927*v7263)+(v1925*v7272)))}else{v7263});
        let v7291=(v8*v1932);
        let v7301=(v1933*v1933);
        let v7329=(if v1938{((v1945*v6893)+(v1817*(v1943*v6890)))}else{v21});
        let v7330=(if v1938{((v1945*v6894)+(v1817*(v1943*v6891)))}else{v21});
        let v7331=(if v1938{((v1945*v6895)+(v1817*(v1943*v6892)))}else{v21});
        let v7367=(v1962*v1962);
        let v7375=(if v1954{((-(v632*((v1960*v7329)+(v1955*((v1958*(v42*v7329))+(v1956*(v37*v7329)))))))/v7367)}else{(if v1950{(v1951*(-v7329))}else{v7285})});
        let v7376=(if v1954{((-(v632*((v1960*v7330)+(v1955*((v1958*(v42*v7330))+(v1956*(v37*v7330)))))))/v7367)}else{(if v1950{(v1951*(-v7330))}else{v7286})});
        let v7377=(if v1954{((-(v632*((v1960*v7331)+(v1955*((v1958*(v42*v7331))+(v1956*(v37*v7331)))))))/v7367)}else{(if v1950{(v1951*(-v7331))}else{v7287})});
        let v7387=(v8*v1970);
        let v7397=(if v1938{(v6890-(self.scalar_static_f64[212]*((v6890-(if v1938{(-v7375)}else{v21}))/v7387)))}else{v21});
        let v7398=(if v1938{(v6891-(self.scalar_static_f64[212]*((v6891-(if v1938{(-v7376)}else{v21}))/v7387)))}else{v21});
        let v7399=(if v1938{(v6892-(self.scalar_static_f64[212]*((v6892-(if v1938{(-v7377)}else{v21}))/v7387)))}else{v21});
        let v7400=(-v7397);
        let v7401=(-v7398);
        let v7402=(-v7399);
        let v7403=(v1975*v7400);
        let v7405=(v1975*v7401);
        let v7407=(v1975*v7402);
        let v7409=(v8*v1979);
        let v7422=(v1983*v7397);
        let v7424=(v1983*v7398);
        let v7426=(v1983*v7399);
        let v7428=(v8*v1987);
        let v7437=(v1988*v1988);
        let v7460=(if v1938{(if v1976{(-(v42*(v7400+((v7403+v7403)/v7409))))}else{(if v1984{(-((-(v668*(v7397+((v7422+v7422)/v7428))))/v7437))}else{(-(v42*v7400))})})}else{v6948});
        let v7461=(if v1938{(if v1976{(-(v42*(v7401+((v7405+v7405)/v7409))))}else{(if v1984{(-((-(v668*(v7398+((v7424+v7424)/v7428))))/v7437))}else{(-(v42*v7401))})})}else{v6949});
        let v7462=(if v1938{(if v1976{(-(v42*(v7402+((v7407+v7407)/v7409))))}else{(if v1984{(-((-(v668*(v7399+((v7426+v7426)/v7428))))/v7437))}else{(-(v42*v7402))})})}else{v6950});
        let v7466=(if v1938{(v6890-v7460)}else{v7375});
        let v7467=(if v1938{(v6891-v7461)}else{v7376});
        let v7468=(if v1938{(v6892-v7462)}else{v7377});
        let v7469=(-v7460);
        let v7470=(-v7461);
        let v7471=(-v7462);
        let v7475=(if v1938{(v2006*v7469)}else{v7207});
        let v7476=(if v1938{(v2006*v7470)}else{v7208});
        let v7477=(if v1938{(v2006*v7471)}else{v7209});
        let v7478=(v2004*v7466);
        let v7480=(v2004*v7467);
        let v7482=(v2004*v7468);
        let v7502=(if v1938{(if v2016{v21}else{((v7478+v7478)-(self.scalar_static_f64[213]*((v7460+v7475)-(self.scalar_static_f64[239]*v7460))))})}else{v6969});
        let v7503=(if v1938{(if v2016{v21}else{((v7480+v7480)-(self.scalar_static_f64[213]*((v7461+v7476)-(self.scalar_static_f64[239]*v7461))))})}else{v6970});
        let v7504=(if v1938{(if v2016{v21}else{((v7482+v7482)-(self.scalar_static_f64[213]*((v7462+v7477)-(self.scalar_static_f64[239]*v7462))))})}else{v6971});
        let v7526=(if v1938{((v8*v7466)+(self.scalar_static_f64[213]*(-v7475)))}else{v6975});
        let v7527=(if v1938{((v8*v7467)+(self.scalar_static_f64[213]*(-v7476)))}else{v6976});
        let v7528=(if v1938{((v8*v7468)+(self.scalar_static_f64[213]*(-v7477)))}else{v6977});
        let v7538=(if v1938{(v7469+((v7502/self.scalar_static_f64[213])/v2029))}else{v6990});
        let v7539=(if v1938{(v7470+((v7503/self.scalar_static_f64[213])/v2029))}else{v6991});
        let v7540=(if v1938{(v7471+((v7504/self.scalar_static_f64[213])/v2029))}else{v6992});
        let v7544=(if v1938{(v7502+v7526)}else{v21});
        let v7545=(if v1938{(v7503+v7527)}else{v21});
        let v7546=(if v1938{(v7504+v7528)}else{v21});
        let v7550=(v2034*v7544);
        let v7552=(v2034*v7545);
        let v7554=(v2034*v7546);
        let v7570=((v2021*v7502)+(v2018*(if v1938{(-(self.scalar_static_f64[321]*v7475))}else{v21})));
        let v7573=((v2021*v7503)+(v2018*(if v1938{(-(self.scalar_static_f64[321]*v7476))}else{v21})));
        let v7576=((v2021*v7504)+(v2018*(if v1938{(-(self.scalar_static_f64[321]*v7477))}else{v21})));
        let v7592=(if v2040{((v7550+v7550)+((v2045*v7538)+(v2032*(((v2042*v7526)+(v2027*(v42*v7526)))-v7570))))}else{v21});
        let v7593=(if v2040{((v7552+v7552)+((v2045*v7539)+(v2032*(((v2042*v7527)+(v2027*(v42*v7527)))-v7573))))}else{v21});
        let v7594=(if v2040{((v7554+v7554)+((v2045*v7540)+(v2032*(((v2042*v7528)+(v2027*(v42*v7528)))-v7576))))}else{v21});
        let v7634=(v2048*v2048);
        let v7653=(v2027*v7526);
        let v7655=(v2027*v7527);
        let v7657=(v2027*v7528);
        let v7680=(v2059*v2059);
        let v7693=(if v2040{(v7460+(((v2059*((v2049*v7538)+(v2032*((v2034*v7502)+(v2018*v7544)))))-(v2050*(v7592+((v2057*((v2053*v7526)+(v2027*(((v2048*((v2051*v7538)+(v2032*((v2034*v7538)+(v2032*v7544)))))-(v2052*v7592))/v7634))))+(v2054*((v37*(v7653+v7653))-v7570))))))/v7680))}else{(if v2037{v7460}else{v21})});
        let v7694=(if v2040{(v7461+(((v2059*((v2049*v7539)+(v2032*((v2034*v7503)+(v2018*v7545)))))-(v2050*(v7593+((v2057*((v2053*v7527)+(v2027*(((v2048*((v2051*v7539)+(v2032*((v2034*v7539)+(v2032*v7545)))))-(v2052*v7593))/v7634))))+(v2054*((v37*(v7655+v7655))-v7573))))))/v7680))}else{(if v2037{v7461}else{v21})});
        let v7695=(if v2040{(v7462+(((v2059*((v2049*v7540)+(v2032*((v2034*v7504)+(v2018*v7546)))))-(v2050*(v7594+((v2057*((v2053*v7528)+(v2027*(((v2048*((v2051*v7540)+(v2032*((v2034*v7540)+(v2032*v7546)))))-(v2052*v7594))/v7634))))+(v2054*((v37*(v7657+v7657))-v7576))))))/v7680))}else{(if v2037{v7462}else{v21})});
        let v7699=(if v2064{(v2065*v7693)}else{v7169});
        let v7700=(if v2064{(v2065*v7694)}else{v7170});
        let v7701=(if v2064{(v2065*v7695)}else{v7171});
        let v7703=(v2066*v2066);
        let v7721=(if v2074{(v2076*v7693)}else{(if v2064{(self.scalar_static_f64[239]*v7699)}else{v7699})});
        let v7722=(if v2074{(v2076*v7694)}else{(if v2064{(self.scalar_static_f64[239]*v7700)}else{v7700})});
        let v7723=(if v2074{(v2076*v7695)}else{(if v2064{(self.scalar_static_f64[239]*v7701)}else{v7701})});
        let v7726=(v2077*v2077);
        let v7737=(-v7693);
        let v7738=(-v7694);
        let v7739=(-v7695);
        let v7766=(v2090*v2090);
        let v7774=(if v2081{((-(v632*((v2088*v7737)+(v2083*((v2086*(v42*v7737))+(v2084*(v37*v7737)))))))/v7766)}else{v7721});
        let v7775=(if v2081{((-(v632*((v2088*v7738)+(v2083*((v2086*(v42*v7738))+(v2084*(v37*v7738)))))))/v7766)}else{v7722});
        let v7776=(if v2081{((-(v632*((v2088*v7739)+(v2083*((v2086*(v42*v7739))+(v2084*(v37*v7739)))))))/v7766)}else{v7723});
        let v7803=(v2100*v2100);
        let v7811=(if v2081{((-(v632*((v2098*v7693)+(v2093*((v2096*(v42*v7693))+(v2094*(v37*v7693)))))))/v7803)}else{(if v2074{((-(self.scalar_static_f64[239]*v7721))/v7726)}else{(if v2064{((-v7699)/v7703)}else{v7179})})});
        let v7812=(if v2081{((-(v632*((v2098*v7694)+(v2093*((v2096*(v42*v7694))+(v2094*(v37*v7694)))))))/v7803)}else{(if v2074{((-(self.scalar_static_f64[239]*v7722))/v7726)}else{(if v2064{((-v7700)/v7703)}else{v7180})})});
        let v7813=(if v2081{((-(v632*((v2098*v7695)+(v2093*((v2096*(v42*v7695))+(v2094*(v37*v7695)))))))/v7803)}else{(if v2074{((-(self.scalar_static_f64[239]*v7723))/v7726)}else{(if v2064{((-v7701)/v7703)}else{v7181})})});
        let v7814=(v2062*v7693);
        let v7816=(v2062*v7694);
        let v7818=(v2062*v7695);
        let v7821=(v2104*v2104);
        let v7833=(if v1938{(v6890-v7693)}else{(if v1938{((-(v7814+v7814))/v7821)}else{v7466})});
        let v7834=(if v1938{(v6891-v7694)}else{(if v1938{((-(v7816+v7816))/v7821)}else{v7467})});
        let v7835=(if v1938{(v6892-v7695)}else{(if v1938{((-(v7818+v7818))/v7821)}else{v7468})});
        let v7851=(if v1938{((v8*v7833)+(self.scalar_static_f64[213]*(v7774+(-v7811))))}else{v7222});
        let v7852=(if v1938{((v8*v7834)+(self.scalar_static_f64[213]*(v7775+(-v7812))))}else{v7223});
        let v7853=(if v1938{((v8*v7835)+(self.scalar_static_f64[213]*(v7776+(-v7813))))}else{v7224});
        let v7854=(v2108*v7833);
        let v7856=(v2108*v7834);
        let v7858=(v2108*v7835);
        let v7890=(if v1938{(-(self.scalar_static_f64[213]*(v7774+v7811)))}else{v7833});
        let v7891=(if v1938{(-(self.scalar_static_f64[213]*(v7775+v7812)))}else{v7834});
        let v7892=(if v1938{(-(self.scalar_static_f64[213]*(v7776+v7813)))}else{v7835});
        let v7893=(v2115*v7851);
        let v7895=(v2115*v7852);
        let v7897=(v2115*v7853);
        let v7899=(v8*(if v1938{((v7854+v7854)-(self.scalar_static_f64[213]*((v7774+(v7693+v7811))-(self.scalar_static_f64[239]*v7693))))}else{v7249}));
        let v7900=(v8*(if v1938{((v7856+v7856)-(self.scalar_static_f64[213]*((v7775+(v7694+v7812))-(self.scalar_static_f64[239]*v7694))))}else{v7250}));
        let v7901=(v8*(if v1938{((v7858+v7858)-(self.scalar_static_f64[213]*((v7776+(v7695+v7813))-(self.scalar_static_f64[239]*v7695))))}else{v7251}));
        let v7917=(v8*v2135);
        let v7927=(v2136*v2136);
        let v7949=(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v1938{(v7693+(((v2136*v7899)-(v2131*(v7851+((if v1938{((v7893+v7893)-((v2131*v7890)+(v2129*v7899)))}else{v7890})/v7917))))/v7927))}else{(if v1827{((-v7133)-(((v1933*v7270)-(v1927*(v7222+(v7285/v7291))))/v7301))}else{(if v1815{((v1821*v6893)+(v1817*(v1816*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v6890)))))}else{v21})})})))}else{v21});
        let v7950=(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v1938{(v7694+(((v2136*v7900)-(v2131*(v7852+((if v1938{((v7895+v7895)-((v2131*v7891)+(v2129*v7900)))}else{v7891})/v7917))))/v7927))}else{(if v1827{((-v7134)-(((v1933*v7271)-(v1927*(v7223+(v7286/v7291))))/v7301))}else{(if v1815{((v1821*v6894)+(v1817*(v1816*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v6891)))))}else{v21})})})))}else{v21});
        let v7951=(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*(self.scalar_static_f64[147]*(if v1938{(v7695+(((v2136*v7901)-(v2131*(v7853+((if v1938{((v7897+v7897)-((v2131*v7892)+(v2129*v7901)))}else{v7892})/v7917))))/v7927))}else{(if v1827{((-v7135)-(((v1933*v7272)-(v1927*(v7224+(v7287/v7291))))/v7301))}else{(if v1815{((v1821*v6895)+(v1817*(v1816*(self.scalar_static_f64[212]*(self.scalar_static_f64[315]*v6892)))))}else{v21})})})))}else{v21});
        let v7958=(if self.scalar_static_bool[16]{((self.scalar_static_f64[19]-v7949)/self.scalar_static_f64[165])}else{self.scalar_static_f64[336]});
        let v7959=(if self.scalar_static_bool[16]{((self.scalar_static_f64[145]-v7950)/self.scalar_static_f64[165])}else{self.scalar_static_f64[337]});
        let v7960=(if self.scalar_static_bool[16]{((v68-v7951)/self.scalar_static_f64[165])}else{self.scalar_static_f64[168]});
        let v7964=(((v455*v7958)-(v2145*v3433))/v3436);
        let v7968=(((v455*v7959)-(v2145*v3434))/v3436);
        let v7969=(v7960/v455);
        let v7991=(if v2154{((v2157*v7964)+(v2149*((v2155*v7958)+(v2145*(if v2154{v6203}else{v21})))))}else{v21});
        let v7992=(if v2154{((v2157*v7968)+(v2149*((v2155*v7959)+(v2145*(if v2154{v6207}else{v21})))))}else{v21});
        let v7993=(if v2154{((v2157*v7969)+(v2149*(v2155*v7960)))}else{v21});
        let v8029=(v2174*v2174);
        let v8037=(if v2166{((-(v170*((v2172*v7991)+(v2167*((v2170*(v42*v7991))+(v2168*(v37*v7991)))))))/v8029)}else{(if v2161{(v2163*(-v7991))}else{v21})});
        let v8038=(if v2166{((-(v170*((v2172*v7992)+(v2167*((v2170*(v42*v7992))+(v2168*(v37*v7992)))))))/v8029)}else{(if v2161{(v2163*(-v7992))}else{v21})});
        let v8039=(if v2166{((-(v170*((v2172*v7993)+(v2167*((v2170*(v42*v7993))+(v2168*(v37*v7993)))))))/v8029)}else{(if v2161{(v2163*(-v7993))}else{v21})});
        let v8043=(if v2154{(-v8037)}else{v21});
        let v8044=(if v2154{(-v8038)}else{v21});
        let v8045=(if v2154{(-v8039)}else{v21});
        let v8053=(v8*v2182);
        let v8067=(if v2154{((v3898+v7958)-((v2182*v3422)+(v451*(((v3902+v7958)-v8043)/v8053))))}else{v21});
        let v8068=(if v2154{((v3899+v7959)-((v2182*v3423)+(v451*(((v3903+v7959)-v8044)/v8053))))}else{v21});
        let v8069=(if v2154{(v7960-(v451*((v7960-v8045)/v8053)))}else{v21});
        let v8105=(v2200*v2200);
        let v8113=(if v2192{((-(v170*((v2198*v8067)+(v2193*((v2196*(v42*v8067))+(v2194*(v37*v8067)))))))/v8105)}else{(if v2187{(v2189*(-v8067))}else{v21})});
        let v8114=(if v2192{((-(v170*((v2198*v8068)+(v2193*((v2196*(v42*v8068))+(v2194*(v37*v8068)))))))/v8105)}else{(if v2187{(v2189*(-v8068))}else{v21})});
        let v8115=(if v2192{((-(v170*((v2198*v8069)+(v2193*((v2196*(v42*v8069))+(v2194*(v37*v8069)))))))/v8105)}else{(if v2187{(v2189*(-v8069))}else{v21})});
        let v8126=(if v2154{(-((v2202*v3898)+(v645*v8113)))}else{v21});
        let v8127=(if v2154{(-((v2202*v3899)+(v645*v8114)))}else{v21});
        let v8128=(if v2154{(-(v645*v8115))}else{v21});
        let v8129=(v7958-v8067);
        let v8130=(v7959-v8068);
        let v8131=(v7960-v8069);
        let v8148=(if v2154{((v8*v8129)+((v2208*v3425)+(v452*(-v8113))))}else{v21});
        let v8149=(if v2154{((v8*v8130)+((v2208*v3427)+(v452*(-v8114))))}else{v21});
        let v8150=(if v2154{((v8*v8131)+(v452*(-v8115)))}else{v21});
        let v8151=(v2206*v8129);
        let v8153=(v2206*v8130);
        let v8155=(v2206*v8131);
        let v8170=(if v2154{((v8151+v8151)-((v2214*v3425)+(v452*(v8067+v8113))))}else{v21});
        let v8171=(if v2154{((v8153+v8153)-((v2214*v3427)+(v452*(v8068+v8114))))}else{v21});
        let v8172=(if v2154{((v8155+v8155)-(v452*(v8069+v8115)))}else{v21});
        let v8173=(v2211*v8148);
        let v8175=(v2211*v8149);
        let v8177=(v2211*v8150);
        let v8194=(if v2154{((v8173+v8173)-((v2219*v8170)+(v2217*(v94*v8126))))}else{v8037});
        let v8195=(if v2154{((v8175+v8175)-((v2219*v8171)+(v2217*(v94*v8127))))}else{v8038});
        let v8196=(if v2154{((v8177+v8177)-((v2219*v8172)+(v2217*(v94*v8128))))}else{v8039});
        let v8200=(v8*v2224);
        let v8210=(v2225*v2225);
        let v8232=(if v2231{(-v7958)}else{v21});
        let v8233=(if v2231{(-v7959)}else{v21});
        let v8234=(if v2231{(-v7960)}else{v21});
        let v8247=(if v2231{(((v455*(v160*v8232))-(v2234*v3433))/v3436)}else{v21});
        let v8248=(if v2231{(((v455*(v160*v8233))-(v2234*v3434))/v3436)}else{v21});
        let v8249=(if v2231{((v160*v8234)/v455)}else{v21});
        let v8250=(v2238*v8247);
        let v8252=(v2238*v8248);
        let v8254=(v2238*v8249);
        let v8256=(v8*v2241);
        let v8266=(if v2231{(v42*(v8247-((v8250+v8250)/v8256)))}else{v21});
        let v8267=(if v2231{(v42*(v8248-((v8252+v8252)/v8256)))}else{v21});
        let v8268=(if v2231{(v42*(v8249-((v8254+v8254)/v8256)))}else{v21});
        let v8269=(v8232-v8266);
        let v8270=(v8233-v8267);
        let v8271=(v8234-v8268);
        let v8272=(v2245*v8269);
        let v8274=(v2245*v8270);
        let v8276=(v2245*v8271);
        let v8288=(if v2231{((v8272+v8272)+((v2247*v3425)+(v452*v8266)))}else{v21});
        let v8289=(if v2231{((v8274+v8274)+((v2247*v3427)+(v452*v8267)))}else{v21});
        let v8290=(if v2231{((v8276+v8276)+(v452*v8268))}else{v21});
        let v8296=(if v2231{((v8*v8269)-v3425)}else{v21});
        let v8297=(if v2231{((v8*v8270)-v3427)}else{v21});
        let v8298=(if v2231{(v8*v8271)}else{v21});
        let v8314=(if v2231{(((((v452*v8288)-(v2250*v3425))/v3429)/v2254)-v8266)}else{v21});
        let v8315=(if v2231{(((((v452*v8289)-(v2250*v3427))/v3429)/v2254)-v8267)}else{v21});
        let v8316=(if v2231{(((v8290/v452)/v2254)-v8268)}else{v21});
        let v8320=(if v2231{(v8288+v8296)}else{v21});
        let v8321=(if v2231{(v8289+v8297)}else{v21});
        let v8322=(if v2231{(v8290+v8298)}else{v21});
        let v8323=(v2259*v8320);
        let v8325=(v2259*v8321);
        let v8327=(v2259*v8322);
        let v8356=(if v2231{((v8323+v8323)+((v2263*v8314)+(v2257*(((v2261*v8296)+(v2253*(v42*v8296)))-v8288))))}else{v21});
        let v8357=(if v2231{((v8325+v8325)+((v2263*v8315)+(v2257*(((v2261*v8297)+(v2253*(v42*v8297)))-v8289))))}else{v21});
        let v8358=(if v2231{((v8327+v8327)+((v2263*v8316)+(v2257*(((v2261*v8298)+(v2253*(v42*v8298)))-v8290))))}else{v21});
        let v8398=(v2266*v2266);
        let v8417=(v2253*v8296);
        let v8419=(v2253*v8297);
        let v8421=(v2253*v8298);
        let v8444=(v2277*v2277);
        let v8457=(if v2231{(v8266+(((v2277*((v2267*v8314)+(v2257*((v2259*v8288)+(v2250*v8320)))))-(v2268*(v8356+((v2275*((v2271*v8296)+(v2253*(((v2266*((v2269*v8314)+(v2257*((v2259*v8314)+(v2257*v8320)))))-(v2270*v8356))/v8398))))+(v2272*((v37*(v8417+v8417))-v8288))))))/v8444))}else{v21});
        let v8458=(if v2231{(v8267+(((v2277*((v2267*v8315)+(v2257*((v2259*v8289)+(v2250*v8321)))))-(v2268*(v8357+((v2275*((v2271*v8297)+(v2253*(((v2266*((v2269*v8315)+(v2257*((v2259*v8315)+(v2257*v8321)))))-(v2270*v8357))/v8398))))+(v2272*((v37*(v8419+v8419))-v8289))))))/v8444))}else{v21});
        let v8459=(if v2231{(v8268+(((v2277*((v2267*v8316)+(v2257*((v2259*v8290)+(v2250*v8322)))))-(v2268*(v8358+((v2275*((v2271*v8298)+(v2253*(((v2266*((v2269*v8316)+(v2257*((v2259*v8316)+(v2257*v8322)))))-(v2270*v8358))/v8398))))+(v2272*((v37*(v8421+v8421))-v8290))))))/v8444))}else{v21});
        let v8466=(-v8457);
        let v8467=(-v8458);
        let v8468=(-v8459);
        let v8495=(v2297*v2297);
        let v8533=(if v2301{(v558*((v2307*v8457)+(v2302*((v2305*(v42*v8457))+(v2303*(v37*v8457))))))}else{(if v2289{((-(v632*((v2295*v8466)+(v2290*((v2293*(v42*v8466))+(v2291*(v37*v8466)))))))/v8495)}else{(if v2283{(v2284*v8457)}else{v8113})})});
        let v8534=(if v2301{(v558*((v2307*v8458)+(v2302*((v2305*(v42*v8458))+(v2303*(v37*v8458))))))}else{(if v2289{((-(v632*((v2295*v8467)+(v2290*((v2293*(v42*v8467))+(v2291*(v37*v8467)))))))/v8495)}else{(if v2283{(v2284*v8458)}else{v8114})})});
        let v8535=(if v2301{(v558*((v2307*v8459)+(v2302*((v2305*(v42*v8459))+(v2303*(v37*v8459))))))}else{(if v2289{((-(v632*((v2295*v8468)+(v2290*((v2293*(v42*v8468))+(v2291*(v37*v8468)))))))/v8495)}else{(if v2283{(v2284*v8459)}else{v8115})})});
        let v8549=(v8232-v8457);
        let v8550=(v8233-v8458);
        let v8551=(v8234-v8459);
        let v8565=(if v2231{((v8*v8549)+((v2317*v3425)+(v452*v8533)))}else{v8148});
        let v8566=(if v2231{((v8*v8550)+((v2317*v3427)+(v452*v8534)))}else{v8149});
        let v8567=(if v2231{((v8*v8551)+(v452*v8535))}else{v8150});
        let v8568=(v2315*v8549);
        let v8570=(v2315*v8550);
        let v8572=(v2315*v8551);
        let v8587=(if v2231{((v8568+v8568)+((v2323*v3425)+(v452*(v8457-v8533))))}else{v8170});
        let v8588=(if v2231{((v8570+v8570)+((v2323*v3427)+(v452*(v8458-v8534))))}else{v8171});
        let v8589=(if v2231{((v8572+v8572)+(v452*(v8459-v8535)))}else{v8172});
        let v8590=(v2320*v8565);
        let v8592=(v2320*v8566);
        let v8594=(v2320*v8567);
        let v8617=(v8*v2333);
        let v8627=(v2334*v2334);
        let v8646=(if v2231{(-(v8457+(if v2231{(((v2334*(v8*v8587))-(v2332*(v8565+((if v2231{((v8590+v8590)-((v2328*v8587)+(v2326*(v94*(if v2231{(-((v2311*v3898)+(v645*v8533)))}else{v8126})))))}else{v8194})/v8617))))/v8627)}else{v8043})))}else{(if v2154{(v8067+(if v2154{(((v2225*(v8*v8170))-(v2223*(v8148+(v8194/v8200))))/v8210)}else{v21}))}else{(if v2148{v7964}else{v6875})})});
        let v8647=(if v2231{(-(v8458+(if v2231{(((v2334*(v8*v8588))-(v2332*(v8566+((if v2231{((v8592+v8592)-((v2328*v8588)+(v2326*(v94*(if v2231{(-((v2311*v3899)+(v645*v8534)))}else{v8127})))))}else{v8195})/v8617))))/v8627)}else{v8044})))}else{(if v2154{(v8068+(if v2154{(((v2225*(v8*v8171))-(v2223*(v8149+(v8195/v8200))))/v8210)}else{v21}))}else{(if v2148{v7968}else{v6876})})});
        let v8648=(if v2231{(-(v8459+(if v2231{(((v2334*(v8*v8589))-(v2332*(v8567+((if v2231{((v8594+v8594)-((v2328*v8589)+(v2326*(v94*(if v2231{(-(v645*v8535))}else{v8128})))))}else{v8196})/v8617))))/v8627)}else{v8045})))}else{(if v2154{(v8069+(if v2154{(((v2225*(v8*v8172))-(v2223*(v8150+(v8196/v8200))))/v8210)}else{v21}))}else{(if v2148{v7969}else{v6877})})});
        let v8661=(if v2343{(v2344*v8646)}else{v21});
        let v8662=(if v2343{(v2344*v8647)}else{v21});
        let v8663=(if v2343{(v2344*v8648)}else{v21});
        let v8665=(v2345*v2345);
        let v8676=(-v8648);
        let v8693=(v42*v8646);
        let v8694=(v42*v8647);
        let v8695=(v42*v8648);
        let v8696=(v37*v8646);
        let v8697=(v37*v8647);
        let v8698=(v37*v8648);
        let v8719=(v2365*v2365);
        let v8730=(v8646+(if v2357{((-(v632*((v2363*v8646)+(v2358*((v2361*v8693)+(v2359*v8696))))))/v8719)}else{(if v2350{((v2353*v3471)+(v473*(if v2350{(v2352*(v3440-v8646))}else{v8661})))}else{(if v2343{((-v8661)/v8665)}else{v6054})})}));
        let v8731=(v8647+(if v2357{((-(v632*((v2363*v8647)+(v2358*((v2361*v8694)+(v2359*v8697))))))/v8719)}else{(if v2350{((v2353*v3472)+(v473*(if v2350{(v2352*(v3441-v8647))}else{v8662})))}else{(if v2343{((-v8662)/v8665)}else{v6055})})}));
        let v8732=(v8648+(if v2357{((-(v632*((v2363*v8648)+(v2358*((v2361*v8695)+(v2359*v8698))))))/v8719)}else{(if v2350{(v473*(if v2350{(v2352*v8676)}else{v8663}))}else{(if v2343{((-v8663)/v8665)}else{v21})})}));
        let v8733=(if v2368{v8730}else{v6133});
        let v8734=(if v2368{v8731}else{v6134});
        let v8735=(if v2368{v8732}else{v21});
        let v8736=(v8*v2372);
        let v8764=(if v2378{(-((v2381*v8696)+(v2379*(-(v647*v8646)))))}else{v6119});
        let v8765=(if v2378{(-((v2381*v8697)+(v2379*(-(v647*v8647)))))}else{v6120});
        let v8766=(if v2378{(-((v2381*v8698)+(v2379*(-(v647*v8648)))))}else{v21});
        let v8791=(v8*v2390);
        let v8810=(v8*v2398);
        let v8822=((v2400*v3422)+(v451*(self.scalar_static_f64[165]*(if v2394{((if v2394{v8730}else{(if v2378{((v2386*v8764)+(v2384*((v2385*v8646)+(v2339*v8693))))}else{v8733})})/v8810)}else{(if v2378{((v2390*(v148*v8646))+(v2389*(v8764/v8791)))}else{(if v2368{(-(v8733/v8736))}else{v6138})})}))));
        let v8825=((v2400*v3423)+(v451*(self.scalar_static_f64[165]*(if v2394{((if v2394{v8731}else{(if v2378{((v2386*v8765)+(v2384*((v2385*v8647)+(v2339*v8694))))}else{v8734})})/v8810)}else{(if v2378{((v2390*(v148*v8647))+(v2389*(v8765/v8791)))}else{(if v2368{(-(v8734/v8736))}else{v6139})})}))));
        let v8826=(v451*(self.scalar_static_f64[165]*(if v2394{((if v2394{v8732}else{(if v2378{((v2386*v8766)+(v2384*((v2385*v8648)+(v2339*v8695))))}else{v8735})})/v8810)}else{(if v2378{((v2390*(v148*v8648))+(v2389*(v8766/v8791)))}else{(if v2368{(-(v8735/v8736))}else{v21})})})));
        let v8845=(self.scalar_static_f64[165]*(self.scalar_static_f64[165]*(self.scalar_static_f64[325]*(self.scalar_static_f64[163]*(self.scalar_static_f64[150]*(self.scalar_static_f64[150]*((v2404*v3360)+(v2403*(v2402*v3360)))))))));
        let v8846=(self.scalar_static_f64[165]*(self.scalar_static_f64[165]*(self.scalar_static_f64[325]*(self.scalar_static_f64[163]*(self.scalar_static_f64[150]*(self.scalar_static_f64[150]*((v2404*v3361)+(v2403*(v2402*v3361)))))))));
        let v8847=(-v8822);
        let v8848=(-v8825);
        let v8849=(-v8826);
        let v8850=(v8822-v8847);
        let v8851=(v8825-v8848);
        let v8852=(v8826-v8849);
        let v8853=(v2417*v8850);
        let v8855=(v2417*v8851);
        let v8857=(v2417*v8852);
        let v8861=(v8*v2421);
        let v8874=(v8847-v8822);
        let v8875=(v8848-v8825);
        let v8876=(v8849-v8826);
        let v8877=(v42*v8845);
        let v8878=(v42*v8846);
        let v8879=(v2425*v8874);
        let v8881=(v2425*v8875);
        let v8883=(v2425*v8876);
        let v8887=(v8*v2430);
        let v8897=(v2431*v2431);
        let v8909=(v8*v2435);
        let v8910=(v8845/v8909);
        let v8911=(v8846/v8909);
        let v8926=-2.0;
        let v8927=(v2442*v8926);
        let v8929=(v8*v2446);
        let v8938=(v8*v2450);
        let v8940=(v8*v2454);
        let v8948=(v2455*v2455);
        let v8972=(v2464*((if v2418{(v8847+(v42*(v8850+((v8845+(v8853+v8853))/v8861))))}else{(if v2426{(v8847+(((v2431*v8877)-(v2427*(v8874+((v8845+(v8879+v8879))/v8887))))/v8897))}else{(v8847+(v42*(v8850+v8910)))})})+(self.scalar_static_f64[27]*(if v2443{(v42*(v8845/v8929))}else{(if v2451{(((v2455*v8877)-(v2427*(v8845/v8940)))/v8948)}else{(v42*v8910)})}))));
        let v8974=(v2464*((if v2418{(v8848+(v42*(v8851+((v8846+(v8855+v8855))/v8861))))}else{(if v2426{(v8848+(((v2431*v8878)-(v2427*(v8875+((v8846+(v8881+v8881))/v8887))))/v8897))}else{(v8848+(v42*(v8851+v8911)))})})+(self.scalar_static_f64[27]*(if v2443{(v42*(v8846/v8929))}else{(if v2451{(((v2455*v8878)-(v2427*(v8846/v8940)))/v8948)}else{(v42*v8911)})}))));
        let v8976=(v2464*((if v2418{(v8849+(v42*(v8852+((v8857+v8857)/v8861))))}else{(if v2426{(v8849+((-(v2427*(v8876+((v8883+v8883)/v8887))))/v8897))}else{(v8849+(v42*v8852))})})+(self.scalar_static_f64[27]*(if v2443{(v68+(v42*(v8926+((v8927+v8927)/v8929))))}else{(if v2451{(v68+((-(v2427*(v8+((v8938+v8938)/v8940))))/v8948))}else{v21})}))));
        let v8980=(v2469*f64::powf(v2467,-1.1666666666666667));
        let v8989=(v2472*v2472);
        let v8997=(if self.scalar_static_bool[19]{((-(self.scalar_static_f64[4]*(self.scalar_static_f64[22]*((v8972+v8972)*v8980))))/v8989)}else{v21});
        let v8998=(if self.scalar_static_bool[19]{((-(self.scalar_static_f64[4]*(self.scalar_static_f64[22]*((v8974+v8974)*v8980))))/v8989)}else{v21});
        let v8999=(if self.scalar_static_bool[19]{((-(self.scalar_static_f64[4]*(self.scalar_static_f64[22]*((v8976+v8976)*v8980))))/v8989)}else{v21});
        let v10568=(self.scalar_static_f64[19]*((v3257*v8997)+(v2474*(self.scalar_static_f64[50]*(self.scalar_static_f64[48]*((self.scalar_static_f64[19]-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*v8646)}else{v6878}))-(if self.scalar_static_bool[17]{v21}else{v7949})))))));
        let v10569=(self.scalar_static_f64[19]*((v3257*v8998)+(v2474*(self.scalar_static_f64[50]*(self.scalar_static_f64[48]*((self.scalar_static_f64[145]-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*v8647)}else{v6879}))-(if self.scalar_static_bool[17]{v21}else{v7950})))))));
        let v10570=(self.scalar_static_f64[19]*((v3257*v8999)+(v2474*(self.scalar_static_f64[50]*(self.scalar_static_f64[48]*((-(if self.scalar_static_bool[16]{(self.scalar_static_f64[165]*v8648)}else{v6880}))-(if self.scalar_static_bool[17]{v21}else{v7951})))))));

        CommonStampValues {
            v8,
            v21,
            v37,
            v42,
            v68,
            v94,
            v123,
            v141,
            v160,
            v164,
            v170,
            v370,
            v371,
            v372,
            v376,
            v405,
            v413,
            v450,
            v451,
            v452,
            v458,
            v473,
            v477,
            v482,
            v503,
            v507,
            v551,
            v558,
            v625,
            v632,
            v1183,
            v1510,
            v1516,
            v1518,
            v1522,
            v1533,
            v1536,
            v1544,
            v1569,
            v1597,
            v1599,
            v1610,
            v2339,
            v2474,
            v2520,
            v3259,
            v3261,
            v3262,
            v3264,
            v3420,
            v3421,
            v3422,
            v3423,
            v3425,
            v3427,
            v3440,
            v3441,
            v3471,
            v3472,
            v5083,
            v5084,
            v5941,
            v5942,
            v5952,
            v5953,
            v5981,
            v5982,
            v6133,
            v6134,
            v6138,
            v6139,
            v8646,
            v8647,
            v8648,
            v8676,
            v8997,
            v8998,
            v8999,
            v10568,
            v10569,
            v10570,
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
        let v360=0.1;
        let v1527=(((common.v68/common.v1522)-common.v1510)-common.v68);
        let v1539=(common.v68+common.v1510);
        let v1540=(common.v473*v1539);
        let v1546=((common.v458-common.v1510)-common.v551);
        let v1547=(common.v42*v1546);
        let v1549=(common.v68+(common.v37*v1546));
        let v1551=(common.v68+(v1547*v1549));
        let v1553=(common.v68+(v1546*v1551));
        let v1579=(common.v473*common.v482);
        let v1580=(common.v1510*v1579);
        let v1581=(common.v1510*v1580);
        let v1582=(common.v1510*v1581);
        let v1583=1.75;
        let v1585=(common.v68+(common.v1510*v1583));
        let v1587=(if common.v1569{(v1582*v1585)}else{(if common.v1544{((if common.v1544{(common.v632/v1553)}else{common.v1536})-v1540)}else{(if common.v1533{(common.v1536-v1540)}else{(if common.v1518{(common.v473*v1527)}else{common.v21})})})});
        let v1601=((v1587+common.v1597)).sqrt();
        let v1604=(self.scalar_static_f64[165]*common.v452);
        let v1605=(v1587*v1604);
        let v1607=((if common.v1516{(common.v451*v1601)}else{common.v21})+(common.v451*common.v1599));
        let v2475=(common.v503-common.v1510);
        let v2476=(v2475>common.v376);
        let v2478=0.01;
        let v2480=(((v2475*v2475)+v2478)).sqrt();
        let v2484=(common.v1510-common.v503);
        let v2485=(v2484>common.v376);
        let v2486=0.005;
        let v2489=((v2478+(v2484*v2484))).sqrt();
        let v2490=(v2484+v2489);
        let v2499=((-(if v2476{(common.v503-(common.v42*(v2475+v2480)))}else{(if v2485{(common.v503-(v2486/v2490))}else{(common.v503-(common.v42*(v360+v2475)))})}))).exp();
        let v2501=((self.scalar_static_f64[165]*v2499)).sqrt();
        let v2502=(common.v450*common.v2474);
        let v2508=(((common.v477*common.v477)+0.04)).sqrt();
        let v2511=(self.scalar_static_f64[281]*(v2501*v2502));
        let v2514=(common.v68+((common.v42*((-common.v477)+v2508))*self.scalar_static_f64[151]));
        let v2521=(common.v370-common.v2520);
        let v2524=(self.scalar_static_f64[168]*(self.scalar_static_f64[19]*(v2521-self.scalar_static_f64[327])));
        let v2531=((v2524).abs()<=self.scalar_static_f64[224]);
        let v2532=(self.scalar_static_bool[39]&&v2531);
        let v2533=(v2524/self.scalar_static_f64[223]);
        let v2535=(v2524>self.scalar_static_f64[224]);
        let v2537=(self.scalar_static_bool[39]&&(!v2531));
        let v2538=(v2535&&v2537);
        let v2543=(if v2538{self.scalar_static_f64[331]}else{common.v21});
        let v2545=(common.v68+(v2524*v2543));
        let v2547=(if v2538{(v2533*v2545)}else{common.v21});
        let v2548=(v2547<common.v164);
        let v2549=(v2538&&v2548);
        let v2551=((-v2547)).exp();
        let v2554=(v2538&&(!v2548));
        let v2555=(v2547-common.v164);
        let v2556=(common.v42*v2555);
        let v2558=(common.v68+(common.v37*v2555));
        let v2560=(common.v68+(v2556*v2558));
        let v2562=(common.v68+(v2555*v2560));
        let v2564=(if v2554{(common.v170/v2562)}else{(if v2549{v2551}else{common.v21})});
        let v2566=(if v2538{(common.v68-v2564)}else{common.v21});
        let v2572=(((v2524+self.scalar_static_f64[333])-v2566)).sqrt();
        let v2575=(if v2538{((v2524+self.scalar_static_f64[332])-(self.scalar_static_f64[220]*v2572))}else{common.v21});
        let v2576=(v2575<common.v164);
        let v2577=(v2538&&v2576);
        let v2579=((-v2575)).exp();
        let v2582=(v2538&&(!v2576));
        let v2583=(v2575-common.v164);
        let v2584=(common.v42*v2583);
        let v2586=(common.v68+(common.v37*v2583));
        let v2588=(common.v68+(v2584*v2586));
        let v2590=(common.v68+(v2583*v2588));
        let v2592=(if v2582{(common.v170/v2590)}else{(if v2577{v2579}else{common.v21})});
        let v2595=(if v2538{(common.v68-(self.scalar_static_f64[332]*v2592))}else{common.v21});
        let v2596=(v2524-v2575);
        let v2601=(if v2538{((common.v8*v2596)+(self.scalar_static_f64[221]*(common.v68-v2592)))}else{common.v21});
        let v2607=(if v2538{((v2596*v2596)-(self.scalar_static_f64[221]*(v2592+(v2575-common.v68))))}else{common.v21});
        let v2609=(common.v94*v2595);
        let v2612=(if v2538{((v2601*v2601)-(v2607*v2609))}else{v2564});
        let v2613=(common.v8*v2607);
        let v2614=(v2612).sqrt();
        let v2615=(v2601+v2614);
        let v2621=(v2537&&(!v2535));
        let v2623=(if v2621{(-v2524)}else{common.v21});
        let v2626=(if v2621{((common.v160*v2623)/self.scalar_static_f64[223])}else{common.v21});
        let v2628=(v2626-common.v141);
        let v2631=((common.v507+(v2628*v2628))).sqrt();
        let v2634=(if v2621{(common.v42*((common.v503+v2626)-v2631))}else{common.v21});
        let v2635=(v2623-v2634);
        let v2640=(if v2621{((v2635*v2635)+(self.scalar_static_f64[221]*(common.v68+v2634)))}else{common.v21});
        let v2643=(if v2621{((common.v8*v2635)-self.scalar_static_f64[221])}else{common.v21});
        let v2644=(v2640/self.scalar_static_f64[221]);
        let v2647=(if v2621{((v2644).ln()-v2634)}else{common.v21});
        let v2649=(if v2621{(v2640+v2643)}else{common.v21});
        let v2651=(common.v42*v2643);
        let v2653=((v2643*v2651)-v2640);
        let v2656=(if v2621{((v2649*v2649)+(v2647*v2653))}else{common.v21});
        let v2657=(v2640*v2649);
        let v2658=(v2647*v2657);
        let v2659=(v2647*v2649);
        let v2660=(v2647*v2659);
        let v2661=(v2660/v2656);
        let v2662=(v2643*v2661);
        let v2665=((common.v37*(v2643*v2643))-v2640);
        let v2667=(v2656+(v2662*v2665));
        let v2670=(if v2621{(v2634+(v2658/v2667))}else{common.v21});
        let v2672=((v2670).abs()<common.v551);
        let v2673=(v2621&&v2672);
        let v2674=(v2670).exp();
        let v2676=(v2670<common.v625);
        let v2678=(v2621&&(!v2672));
        let v2679=(v2676&&v2678);
        let v2680=(common.v625-v2670);
        let v2681=(common.v42*v2680);
        let v2683=(common.v68+(common.v37*v2680));
        let v2685=(common.v68+(v2681*v2683));
        let v2687=(common.v68+(v2680*v2685));
        let v2691=(v2678&&(!v2676));
        let v2692=(v2670-common.v551);
        let v2693=(common.v42*v2692);
        let v2695=(common.v68+(common.v37*v2692));
        let v2697=(common.v68+(v2693*v2695));
        let v2701=(if v2691{(common.v558*(common.v68+(v2692*v2697)))}else{(if v2679{(common.v632/v2687)}else{(if v2673{v2674}else{v2592})})});
        let v2705=(v2623-v2670);
        let v2710=(if v2621{((common.v8*v2705)+(self.scalar_static_f64[221]*(v2701-common.v68)))}else{v2601});
        let v2716=(if v2621{((v2705*v2705)+(self.scalar_static_f64[221]*((common.v68+v2670)-v2701)))}else{v2607});
        let v2718=(common.v94*(if v2621{(common.v68-(self.scalar_static_f64[332]*v2701))}else{v2595}));
        let v2722=(common.v8*v2716);
        let v2723=((if v2621{((v2710*v2710)-(v2716*v2718))}else{v2612})).sqrt();
        let v2724=(v2710+v2723);
        let v2729=(if v2621{(-(v2670+(if v2621{(v2722/v2724)}else{v2566})))}else{(if v2538{(v2575+(if v2538{(v2613/v2615)}else{common.v21}))}else{(if v2532{v2533}else{common.v21})})});
        let v2734=(if self.scalar_static_bool[40]{common.v21}else{(if self.scalar_static_bool[39]{(self.scalar_static_f64[165]*(v2524-v2729))}else{common.v21})});
        let v2742=(self.scalar_static_f64[19]*v2734);
        let v2744=(if self.scalar_static_bool[43]{(self.scalar_static_f64[309]+v2742)}else{common.v21});
        let v2745=(common.v21-v2744);
        let v2746=(v2745>common.v376);
        let v2749=((v2478+(v2745*v2745))).sqrt();
        let v2753=(v2744>common.v376);
        let v2756=((v2478+(v2744*v2744))).sqrt();
        let v2757=(v2744+v2756);
        let v2765=(if self.scalar_static_bool[43]{(if v2746{(v2744+(common.v42*(v2745+v2749)))}else{(if v2753{(v2744+(v2486/v2757))}else{(v2744+(common.v42*(v360+v2745)))})})}else{common.v21});
        let v2768=((common.v405+(v2734*v2734))).sqrt();
        let v2770=(if self.scalar_static_bool[43]{(self.scalar_static_f64[132]*v2768)}else{common.v21});
        let v2772=(self.scalar_static_f64[130]-v2770);
        let v2773=(v2772>common.v376);
        let v2776=((common.v405+(v2772*v2772))).sqrt();
        let v2780=(v2770-self.scalar_static_f64[130]);
        let v2781=(v2780>common.v376);
        let v2784=((common.v405+(v2780*v2780))).sqrt();
        let v2785=(v2780+v2784);
        let v2793=(if self.scalar_static_bool[44]{(if v2773{(self.scalar_static_f64[130]-(common.v42*(v2772+v2776)))}else{(if v2781{(self.scalar_static_f64[130]-(common.v413/v2785))}else{(self.scalar_static_f64[130]-(common.v42*(common.v123+v2772)))})})}else{v2770});
        let v2796=(self.scalar_static_f64[19]*(if self.scalar_static_bool[40]{common.v21}else{v2729}));
        let v2810=(if self.scalar_static_bool[46]{(-(v2796+(self.scalar_static_f64[168]*(v2765+self.scalar_static_f64[335]))))}else{(if self.scalar_static_bool[45]{(-(v2796+(self.scalar_static_f64[168]*(v2765+self.scalar_static_f64[334]))))}else{common.v21})});
        let v2811=(v2810<common.v551);
        let v2812=(self.scalar_static_bool[43]&&v2811);
        let v2813=(v2810).exp();
        let v2814=(common.v68+v2813);
        let v2818=(self.scalar_static_bool[43]&&(!v2811));
        let v2819=(if v2818{v2810}else{(if v2812{(v2814).ln()}else{common.v21})});
        let v2821=(self.scalar_static_f64[168]*(self.scalar_static_f64[19]*(if self.scalar_static_bool[41]{(self.scalar_static_f64[19]*v2521)}else{common.v21})));
        let v2823=(if self.scalar_static_bool[43]{(v2810+v2821)}else{common.v21});
        let v2824=(v2823<common.v551);
        let v2825=(self.scalar_static_bool[43]&&v2824);
        let v2826=(v2823).exp();
        let v2827=(common.v68+v2826);
        let v2831=(self.scalar_static_bool[43]&&(!v2824));
        let v2832=(if v2831{v2823}else{(if v2825{(v2827).ln()}else{common.v21})});
        let v2833=-1.5;
        let v2835=(self.scalar_static_f64[122]+(self.scalar_static_f64[121]*v2793));
        let v2839=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*(v2833+(v2793*v2835)))}else{common.v21});
        let v2840=(v2839>common.v21);
        let v2841=(self.scalar_static_bool[43]&&v2840);
        let v2842=(common.v42*v2839);
        let v2844=(common.v68+(common.v37*v2839));
        let v2846=(common.v68+(v2842*v2844));
        let v2850=(v2839>common.v625);
        let v2852=(self.scalar_static_bool[43]&&(!v2840));
        let v2853=(v2850&&v2852);
        let v2854=(v2839).exp();
        let v2857=(v2852&&(!v2850));
        let v2858=(common.v625-v2839);
        let v2859=(common.v42*v2858);
        let v2861=(common.v68+(common.v37*v2858));
        let v2863=(common.v68+(v2859*v2861));
        let v2865=(common.v68+(v2858*v2863));
        let v2867=(if v2857{(common.v632/v2865)}else{(if v2853{v2854}else{(if v2841{(common.v68+(v2839*v2846))}else{common.v21})})});
        let v2869=(self.scalar_static_f64[19]*(self.scalar_static_f64[307]*v2867));
        let v2870=(v2832-v2819);
        let v2872=(if self.scalar_static_bool[43]{(v2869*v2870)}else{common.v21});
        let v2875=(if self.scalar_static_bool[47]{(self.scalar_static_f64[308]+v2742)}else{common.v21});
        let v2876=(v2875>common.v376);
        let v2879=((v2478+(v2875*v2875))).sqrt();
        let v2883=(common.v21-v2875);
        let v2884=(v2883>common.v376);
        let v2887=((v2478+(v2883*v2883))).sqrt();
        let v2888=(v2883+v2887);
        let v2896=(if self.scalar_static_bool[47]{(if v2876{(v2875-(common.v42*(v2875+v2879)))}else{(if v2884{(v2875-(v2486/v2888))}else{(v2875-(common.v42*(v360+v2875)))})})}else{v2765});
        let v2898=(if self.scalar_static_bool[47]{(self.scalar_static_f64[131]*v2768)}else{v2793});
        let v2900=(self.scalar_static_f64[129]-v2898);
        let v2901=(v2900>common.v376);
        let v2904=((common.v405+(v2900*v2900))).sqrt();
        let v2908=(v2898-self.scalar_static_f64[129]);
        let v2909=(v2908>common.v376);
        let v2912=((common.v405+(v2908*v2908))).sqrt();
        let v2913=(v2908+v2912);
        let v2921=(if self.scalar_static_bool[48]{(if v2901{(self.scalar_static_f64[129]-(common.v42*(v2900+v2904)))}else{(if v2909{(self.scalar_static_f64[129]-(common.v413/v2913))}else{(self.scalar_static_f64[129]-(common.v42*(common.v123+v2900)))})})}else{v2898});
        let v2931=(if self.scalar_static_bool[50]{(v2796+(self.scalar_static_f64[168]*(v2896-self.scalar_static_f64[310])))}else{(if self.scalar_static_bool[49]{(v2796+(self.scalar_static_f64[168]*(v2896-self.scalar_static_f64[311])))}else{v2810})});
        let v2932=(v2931<common.v551);
        let v2933=(self.scalar_static_bool[47]&&v2932);
        let v2934=(v2931).exp();
        let v2935=(common.v68+v2934);
        let v2939=(self.scalar_static_bool[47]&&(!v2932));
        let v2942=(if self.scalar_static_bool[47]{(v2931-v2821)}else{v2823});
        let v2943=(v2942<common.v551);
        let v2944=(self.scalar_static_bool[47]&&v2943);
        let v2945=(v2942).exp();
        let v2946=(common.v68+v2945);
        let v2950=(self.scalar_static_bool[47]&&(!v2943));
        let v2953=(self.scalar_static_f64[116]+(self.scalar_static_f64[115]*v2921));
        let v2957=(if self.scalar_static_bool[47]{(self.scalar_static_f64[134]*(v2833+(v2921*v2953)))}else{v2839});
        let v2959=((v2957).abs()<common.v551);
        let v2960=(self.scalar_static_bool[47]&&v2959);
        let v2961=(v2957).exp();
        let v2963=(v2957<common.v625);
        let v2965=(self.scalar_static_bool[47]&&(!v2959));
        let v2966=(v2963&&v2965);
        let v2967=(common.v625-v2957);
        let v2968=(common.v42*v2967);
        let v2970=(common.v68+(common.v37*v2967));
        let v2972=(common.v68+(v2968*v2970));
        let v2974=(common.v68+(v2967*v2972));
        let v2978=(v2965&&(!v2963));
        let v2979=(v2957-common.v551);
        let v2980=(common.v42*v2979);
        let v2982=(common.v68+(common.v37*v2979));
        let v2984=(common.v68+(v2980*v2982));
        let v2990=(self.scalar_static_f64[19]*(self.scalar_static_f64[305]*(if v2978{(common.v558*(common.v68+(v2979*v2984)))}else{(if v2966{(common.v632/v2974)}else{(if v2960{v2961}else{v2867})})})));
        let v2991=((if v2939{v2931}else{(if v2933{(v2935).ln()}else{v2819})})-(if v2950{v2942}else{(if v2944{(v2946).ln()}else{v2832})}));
        let v3003=(if self.scalar_static_bool[54]{(self.scalar_static_f64[165]*(common.v1183-common.v2339))}else{common.v21});
        let v3006=(self.scalar_static_f64[19]*v3003);
        let v3008=(if self.scalar_static_bool[56]{(self.scalar_static_f64[309]+v3006)}else{common.v21});
        let v3009=(common.v21-v3008);
        let v3010=(v3009>common.v376);
        let v3013=((v2478+(v3009*v3009))).sqrt();
        let v3017=(v3008>common.v376);
        let v3020=((v2478+(v3008*v3008))).sqrt();
        let v3021=(v3008+v3020);
        let v3029=(if self.scalar_static_bool[56]{(if v3010{(v3008+(common.v42*(v3009+v3013)))}else{(if v3017{(v3008+(v2486/v3021))}else{(v3008+(common.v42*(v360+v3009)))})})}else{common.v21});
        let v3032=((common.v405+(v3003*v3003))).sqrt();
        let v3034=(if self.scalar_static_bool[56]{(self.scalar_static_f64[132]*v3032)}else{common.v21});
        let v3036=(self.scalar_static_f64[130]-v3034);
        let v3037=(v3036>common.v376);
        let v3040=((common.v405+(v3036*v3036))).sqrt();
        let v3044=(v3034-self.scalar_static_f64[130]);
        let v3045=(v3044>common.v376);
        let v3048=((common.v405+(v3044*v3044))).sqrt();
        let v3049=(v3044+v3048);
        let v3057=(if self.scalar_static_bool[57]{(if v3037{(self.scalar_static_f64[130]-(common.v42*(v3036+v3040)))}else{(if v3045{(self.scalar_static_f64[130]-(common.v413/v3049))}else{(self.scalar_static_f64[130]-(common.v42*(common.v123+v3036)))})})}else{v3034});
        let v3059=(self.scalar_static_f64[19]*common.v2339);
        let v3070=(if self.scalar_static_bool[59]{(-(v3059+(self.scalar_static_f64[168]*(self.scalar_static_f64[335]+v3029))))}else{(if self.scalar_static_bool[58]{(-(v3059+(self.scalar_static_f64[168]*(self.scalar_static_f64[334]+v3029))))}else{common.v21})});
        let v3071=(v3070<common.v551);
        let v3072=(self.scalar_static_bool[56]&&v3071);
        let v3073=(v3070).exp();
        let v3074=(common.v68+v3073);
        let v3078=(self.scalar_static_bool[56]&&(!v3071));
        let v3079=(if v3078{v3070}else{(if v3072{(v3074).ln()}else{common.v21})});
        let v3081=(self.scalar_static_f64[168]*(self.scalar_static_f64[19]*(if self.scalar_static_bool[54]{(self.scalar_static_f64[19]*common.v372)}else{common.v21})));
        let v3083=(if self.scalar_static_bool[56]{(v3070+v3081)}else{common.v21});
        let v3084=(v3083<common.v551);
        let v3085=(self.scalar_static_bool[56]&&v3084);
        let v3086=(v3083).exp();
        let v3087=(common.v68+v3086);
        let v3091=(self.scalar_static_bool[56]&&(!v3084));
        let v3092=(if v3091{v3083}else{(if v3085{(v3087).ln()}else{common.v21})});
        let v3094=(self.scalar_static_f64[122]+(self.scalar_static_f64[121]*v3057));
        let v3098=(if self.scalar_static_bool[56]{(self.scalar_static_f64[135]*(v2833+(v3057*v3094)))}else{common.v21});
        let v3099=(v3098>common.v21);
        let v3100=(self.scalar_static_bool[56]&&v3099);
        let v3101=(common.v42*v3098);
        let v3103=(common.v68+(common.v37*v3098));
        let v3105=(common.v68+(v3101*v3103));
        let v3109=(v3098>common.v625);
        let v3111=(self.scalar_static_bool[56]&&(!v3099));
        let v3112=(v3109&&v3111);
        let v3113=(v3098).exp();
        let v3116=(v3111&&(!v3109));
        let v3117=(common.v625-v3098);
        let v3118=(common.v42*v3117);
        let v3120=(common.v68+(common.v37*v3117));
        let v3122=(common.v68+(v3118*v3120));
        let v3124=(common.v68+(v3117*v3122));
        let v3126=(if v3116{(common.v632/v3124)}else{(if v3112{v3113}else{(if v3100{(common.v68+(v3098*v3105))}else{common.v21})})});
        let v3128=(self.scalar_static_f64[19]*(self.scalar_static_f64[306]*v3126));
        let v3129=(v3092-v3079);
        let v3131=(if self.scalar_static_bool[56]{(v3128*v3129)}else{common.v21});
        let v3134=(if self.scalar_static_bool[60]{(self.scalar_static_f64[308]+v3006)}else{common.v21});
        let v3135=(v3134>common.v376);
        let v3138=((v2478+(v3134*v3134))).sqrt();
        let v3142=(common.v21-v3134);
        let v3143=(v3142>common.v376);
        let v3146=((v2478+(v3142*v3142))).sqrt();
        let v3147=(v3142+v3146);
        let v3155=(if self.scalar_static_bool[60]{(if v3135{(v3134-(common.v42*(v3134+v3138)))}else{(if v3143{(v3134-(v2486/v3147))}else{(v3134-(common.v42*(v360+v3134)))})})}else{v3029});
        let v3157=(if self.scalar_static_bool[60]{(self.scalar_static_f64[131]*v3032)}else{v3057});
        let v3159=(self.scalar_static_f64[129]-v3157);
        let v3160=(v3159>common.v376);
        let v3163=((common.v405+(v3159*v3159))).sqrt();
        let v3167=(v3157-self.scalar_static_f64[129]);
        let v3168=(v3167>common.v376);
        let v3171=((common.v405+(v3167*v3167))).sqrt();
        let v3172=(v3167+v3171);
        let v3180=(if self.scalar_static_bool[61]{(if v3160{(self.scalar_static_f64[129]-(common.v42*(v3159+v3163)))}else{(if v3168{(self.scalar_static_f64[129]-(common.v413/v3172))}else{(self.scalar_static_f64[129]-(common.v42*(common.v123+v3159)))})})}else{v3157});
        let v3190=(if self.scalar_static_bool[63]{(v3059+(self.scalar_static_f64[168]*(v3155-self.scalar_static_f64[310])))}else{(if self.scalar_static_bool[62]{(v3059+(self.scalar_static_f64[168]*(v3155-self.scalar_static_f64[311])))}else{v3070})});
        let v3191=(v3190<common.v551);
        let v3192=(self.scalar_static_bool[60]&&v3191);
        let v3193=(v3190).exp();
        let v3194=(common.v68+v3193);
        let v3198=(self.scalar_static_bool[60]&&(!v3191));
        let v3201=(if self.scalar_static_bool[60]{(v3190-v3081)}else{v3083});
        let v3202=(v3201<common.v551);
        let v3203=(self.scalar_static_bool[60]&&v3202);
        let v3204=(v3201).exp();
        let v3205=(common.v68+v3204);
        let v3209=(self.scalar_static_bool[60]&&(!v3202));
        let v3212=(self.scalar_static_f64[116]+(self.scalar_static_f64[115]*v3180));
        let v3216=(if self.scalar_static_bool[60]{(self.scalar_static_f64[133]*(v2833+(v3180*v3212)))}else{v3098});
        let v3218=((v3216).abs()<common.v551);
        let v3219=(self.scalar_static_bool[60]&&v3218);
        let v3220=(v3216).exp();
        let v3222=(v3216<common.v625);
        let v3224=(self.scalar_static_bool[60]&&(!v3218));
        let v3225=(v3222&&v3224);
        let v3226=(common.v625-v3216);
        let v3227=(common.v42*v3226);
        let v3229=(common.v68+(common.v37*v3226));
        let v3231=(common.v68+(v3227*v3229));
        let v3233=(common.v68+(v3226*v3231));
        let v3237=(v3224&&(!v3222));
        let v3238=(v3216-common.v551);
        let v3239=(common.v42*v3238);
        let v3241=(common.v68+(common.v37*v3238));
        let v3243=(common.v68+(v3239*v3241));
        let v3249=(self.scalar_static_f64[19]*(self.scalar_static_f64[304]*(if v3237{(common.v558*(common.v68+(v3238*v3243)))}else{(if v3225{(common.v632/v3233)}else{(if v3219{v3220}else{v3126})})})));
        let v3250=((if v3198{v3190}else{(if v3192{(v3194).ln()}else{v3079})})-(if v3209{v3201}else{(if v3203{(v3205).ln()}else{v3092})}));
        let v3273=(common.v371-common.v2520);
        let v3274=(self.scalar_static_f64[280]+(v2511/v2514));
        let v5963=(common.v1522*common.v1522);
        let v5996=((v1539*common.v3471)+(common.v473*common.v5941));
        let v5999=((v1539*common.v3472)+(common.v473*common.v5942));
        let v6004=(common.v3440-common.v5941);
        let v6005=(common.v3441-common.v5942);
        let v6024=(v1553*v1553);
        let v6114=(if common.v1569{((v1585*((v1581*common.v5941)+(common.v1510*((v1580*common.v5941)+(common.v1510*((v1579*common.v5941)+(common.v1510*(common.v482*common.v3471))))))))+(v1582*(v1583*common.v5941)))}else{(if common.v1544{((if common.v1544{((-(common.v632*((v1551*v6004)+(v1546*((v1549*(common.v42*v6004))+(v1547*(common.v37*v6004)))))))/v6024)}else{common.v5981})-v5996)}else{(if common.v1533{(common.v5981-v5996)}else{(if common.v1518{((v1527*common.v3471)+(common.v473*(((-common.v5952)/v5963)-common.v5941)))}else{common.v21})})})});
        let v6115=(if common.v1569{((v1585*((v1581*common.v5942)+(common.v1510*((v1580*common.v5942)+(common.v1510*((v1579*common.v5942)+(common.v1510*(common.v482*common.v3472))))))))+(v1582*(v1583*common.v5942)))}else{(if common.v1544{((if common.v1544{((-(common.v632*((v1551*v6005)+(v1546*((v1549*(common.v42*v6005))+(v1547*(common.v37*v6005)))))))/v6024)}else{common.v5982})-v5999)}else{(if common.v1533{(common.v5982-v5999)}else{(if common.v1518{((v1527*common.v3472)+(common.v473*(((-common.v5953)/v5963)-common.v5942)))}else{common.v21})})})});
        let v6142=(common.v8*v1601);
        let v6172=(v1607*v1607);
        let v9000=(-common.v5941);
        let v9001=(-common.v5942);
        let v9002=(v2475*v9000);
        let v9004=(v2475*v9001);
        let v9006=(common.v8*v2480);
        let v9015=(v2484*common.v5941);
        let v9017=(v2484*common.v5942);
        let v9019=(common.v8*v2489);
        let v9026=(v2490*v2490);
        let v9047=(common.v8*v2501);
        let v9064=(self.scalar_static_f64[19]*common.v477);
        let v9066=(common.v477*self.scalar_static_f64[145]);
        let v9068=(common.v8*v2508);
        let v9083=(v2514*v2514);
        let v9102=(if v2538{((v2545*self.scalar_static_f64[341])+(v2533*(v2543*self.scalar_static_f64[337])))}else{common.v21});
        let v9103=(if v2538{((v2545*self.scalar_static_f64[342])+(v2533*(v2543*self.scalar_static_f64[336])))}else{common.v21});
        let v9128=(v2562*v2562);
        let v9133=(if v2554{((-(common.v170*((v2560*v9102)+(v2555*((v2558*(common.v42*v9102))+(v2556*(common.v37*v9102)))))))/v9128)}else{(if v2549{(v2551*(-v9102))}else{common.v21})});
        let v9134=(if v2554{((-(common.v170*((v2560*v9103)+(v2555*((v2558*(common.v42*v9103))+(v2556*(common.v37*v9103)))))))/v9128)}else{(if v2549{(v2551*(-v9103))}else{common.v21})});
        let v9137=(if v2538{(-v9133)}else{common.v21});
        let v9138=(if v2538{(-v9134)}else{common.v21});
        let v9141=(common.v8*v2572);
        let v9148=(if v2538{(self.scalar_static_f64[337]-(self.scalar_static_f64[220]*((self.scalar_static_f64[337]-v9137)/v9141)))}else{common.v21});
        let v9149=(if v2538{(self.scalar_static_f64[336]-(self.scalar_static_f64[220]*((self.scalar_static_f64[336]-v9138)/v9141)))}else{common.v21});
        let v9174=(v2590*v2590);
        let v9179=(if v2582{((-(common.v170*((v2588*v9148)+(v2583*((v2586*(common.v42*v9148))+(v2584*(common.v37*v9148)))))))/v9174)}else{(if v2577{(v2579*(-v9148))}else{common.v21})});
        let v9180=(if v2582{((-(common.v170*((v2588*v9149)+(v2583*((v2586*(common.v42*v9149))+(v2584*(common.v37*v9149)))))))/v9174)}else{(if v2577{(v2579*(-v9149))}else{common.v21})});
        let v9185=(if v2538{(-(self.scalar_static_f64[332]*v9179))}else{common.v21});
        let v9186=(if v2538{(-(self.scalar_static_f64[332]*v9180))}else{common.v21});
        let v9187=(self.scalar_static_f64[337]-v9148);
        let v9188=(self.scalar_static_f64[336]-v9149);
        let v9197=(if v2538{((common.v8*v9187)+(self.scalar_static_f64[221]*(-v9179)))}else{common.v21});
        let v9198=(if v2538{((common.v8*v9188)+(self.scalar_static_f64[221]*(-v9180)))}else{common.v21});
        let v9199=(v2596*v9187);
        let v9201=(v2596*v9188);
        let v9209=(if v2538{((v9199+v9199)-(self.scalar_static_f64[221]*(v9148+v9179)))}else{common.v21});
        let v9210=(if v2538{((v9201+v9201)-(self.scalar_static_f64[221]*(v9149+v9180)))}else{common.v21});
        let v9211=(v2601*v9197);
        let v9213=(v2601*v9198);
        let v9225=(if v2538{((v9211+v9211)-((v2609*v9209)+(v2607*(common.v94*v9185))))}else{v9133});
        let v9226=(if v2538{((v9213+v9213)-((v2609*v9210)+(v2607*(common.v94*v9186))))}else{v9134});
        let v9229=(common.v8*v2614);
        let v9237=(v2615*v2615);
        let v9249=(if v2621{self.scalar_static_f64[339]}else{common.v21});
        let v9250=(if v2621{self.scalar_static_f64[338]}else{common.v21});
        let v9255=(if v2621{((common.v160*v9249)/self.scalar_static_f64[223])}else{common.v21});
        let v9256=(if v2621{((common.v160*v9250)/self.scalar_static_f64[223])}else{common.v21});
        let v9257=(v2628*v9255);
        let v9259=(v2628*v9256);
        let v9261=(common.v8*v2631);
        let v9268=(if v2621{(common.v42*(v9255-((v9257+v9257)/v9261)))}else{common.v21});
        let v9269=(if v2621{(common.v42*(v9256-((v9259+v9259)/v9261)))}else{common.v21});
        let v9270=(v9249-v9268);
        let v9271=(v9250-v9269);
        let v9272=(v2635*v9270);
        let v9274=(v2635*v9271);
        let v9280=(if v2621{((v9272+v9272)+(self.scalar_static_f64[221]*v9268))}else{common.v21});
        let v9281=(if v2621{((v9274+v9274)+(self.scalar_static_f64[221]*v9269))}else{common.v21});
        let v9284=(if v2621{(common.v8*v9270)}else{common.v21});
        let v9285=(if v2621{(common.v8*v9271)}else{common.v21});
        let v9292=(if v2621{(((v9280/self.scalar_static_f64[221])/v2644)-v9268)}else{common.v21});
        let v9293=(if v2621{(((v9281/self.scalar_static_f64[221])/v2644)-v9269)}else{common.v21});
        let v9296=(if v2621{(v9280+v9284)}else{common.v21});
        let v9297=(if v2621{(v9281+v9285)}else{common.v21});
        let v9298=(v2649*v9296);
        let v9300=(v2649*v9297);
        let v9320=(if v2621{((v9298+v9298)+((v2653*v9292)+(v2647*(((v2651*v9284)+(v2643*(common.v42*v9284)))-v9280))))}else{common.v21});
        let v9321=(if v2621{((v9300+v9300)+((v2653*v9293)+(v2647*(((v2651*v9285)+(v2643*(common.v42*v9285)))-v9281))))}else{common.v21});
        let v9349=(v2656*v2656);
        let v9361=(v2643*v9284);
        let v9363=(v2643*v9285);
        let v9380=(v2667*v2667);
        let v9388=(if v2621{(v9268+(((v2667*((v2657*v9292)+(v2647*((v2649*v9280)+(v2640*v9296)))))-(v2658*(v9320+((v2665*((v2661*v9284)+(v2643*(((v2656*((v2659*v9292)+(v2647*((v2649*v9292)+(v2647*v9296)))))-(v2660*v9320))/v9349))))+(v2662*((common.v37*(v9361+v9361))-v9280))))))/v9380))}else{common.v21});
        let v9389=(if v2621{(v9269+(((v2667*((v2657*v9293)+(v2647*((v2649*v9281)+(v2640*v9297)))))-(v2658*(v9321+((v2665*((v2661*v9285)+(v2643*(((v2656*((v2659*v9293)+(v2647*((v2649*v9293)+(v2647*v9297)))))-(v2660*v9321))/v9349))))+(v2662*((common.v37*(v9363+v9363))-v9281))))))/v9380))}else{common.v21});
        let v9394=(-v9388);
        let v9395=(-v9389);
        let v9414=(v2687*v2687);
        let v9439=(if v2691{(common.v558*((v2697*v9388)+(v2692*((v2695*(common.v42*v9388))+(v2693*(common.v37*v9388))))))}else{(if v2679{((-(common.v632*((v2685*v9394)+(v2680*((v2683*(common.v42*v9394))+(v2681*(common.v37*v9394)))))))/v9414)}else{(if v2673{(v2674*v9388)}else{v9179})})});
        let v9440=(if v2691{(common.v558*((v2697*v9389)+(v2692*((v2695*(common.v42*v9389))+(v2693*(common.v37*v9389))))))}else{(if v2679{((-(common.v632*((v2685*v9395)+(v2680*((v2683*(common.v42*v9395))+(v2681*(common.v37*v9395)))))))/v9414)}else{(if v2673{(v2674*v9389)}else{v9180})})});
        let v9447=(v9249-v9388);
        let v9448=(v9250-v9389);
        let v9455=(if v2621{((common.v8*v9447)+(self.scalar_static_f64[221]*v9439))}else{v9197});
        let v9456=(if v2621{((common.v8*v9448)+(self.scalar_static_f64[221]*v9440))}else{v9198});
        let v9457=(v2705*v9447);
        let v9459=(v2705*v9448);
        let v9467=(if v2621{((v9457+v9457)+(self.scalar_static_f64[221]*(v9388-v9439)))}else{v9209});
        let v9468=(if v2621{((v9459+v9459)+(self.scalar_static_f64[221]*(v9389-v9440)))}else{v9210});
        let v9469=(v2710*v9455);
        let v9471=(v2710*v9456);
        let v9487=(common.v8*v2723);
        let v9495=(v2724*v2724);
        let v9507=(if v2621{(-(v9388+(if v2621{(((v2724*(common.v8*v9467))-(v2722*(v9455+((if v2621{((v9469+v9469)-((v2718*v9467)+(v2716*(common.v94*(if v2621{(-(self.scalar_static_f64[332]*v9439))}else{v9185})))))}else{v9225})/v9487))))/v9495)}else{v9137})))}else{(if v2538{(v9148+(if v2538{(((v2615*(common.v8*v9209))-(v2613*(v9197+(v9225/v9229))))/v9237)}else{common.v21}))}else{(if v2532{self.scalar_static_f64[341]}else{common.v21})})});
        let v9508=(if v2621{(-(v9389+(if v2621{(((v2724*(common.v8*v9468))-(v2722*(v9456+((if v2621{((v9471+v9471)-((v2718*v9468)+(v2716*(common.v94*(if v2621{(-(self.scalar_static_f64[332]*v9440))}else{v9186})))))}else{v9226})/v9487))))/v9495)}else{v9138})))}else{(if v2538{(v9149+(if v2538{(((v2615*(common.v8*v9210))-(v2613*(v9198+(v9226/v9229))))/v9237)}else{common.v21}))}else{(if v2532{self.scalar_static_f64[342]}else{common.v21})})});
        let v9515=(if self.scalar_static_bool[40]{common.v21}else{(if self.scalar_static_bool[39]{(self.scalar_static_f64[165]*(self.scalar_static_f64[337]-v9507))}else{common.v21})});
        let v9516=(if self.scalar_static_bool[40]{common.v21}else{(if self.scalar_static_bool[39]{(self.scalar_static_f64[165]*(self.scalar_static_f64[336]-v9508))}else{common.v21})});
        let v9521=(self.scalar_static_f64[19]*v9515);
        let v9522=(self.scalar_static_f64[19]*v9516);
        let v9523=(if self.scalar_static_bool[43]{v9521}else{common.v21});
        let v9524=(if self.scalar_static_bool[43]{v9522}else{common.v21});
        let v9525=(-v9523);
        let v9526=(-v9524);
        let v9527=(v2745*v9525);
        let v9529=(v2745*v9526);
        let v9531=(common.v8*v2749);
        let v9540=(v2744*v9523);
        let v9542=(v2744*v9524);
        let v9544=(common.v8*v2756);
        let v9551=(v2757*v2757);
        let v9566=(if self.scalar_static_bool[43]{(if v2746{(v9523+(common.v42*(v9525+((v9527+v9527)/v9531))))}else{(if v2753{(v9523+((-(v2486*(v9523+((v9540+v9540)/v9544))))/v9551))}else{(v9523+(common.v42*v9525))})})}else{common.v21});
        let v9567=(if self.scalar_static_bool[43]{(if v2746{(v9524+(common.v42*(v9526+((v9529+v9529)/v9531))))}else{(if v2753{(v9524+((-(v2486*(v9524+((v9542+v9542)/v9544))))/v9551))}else{(v9524+(common.v42*v9526))})})}else{common.v21});
        let v9568=(v2734*v9515);
        let v9570=(v2734*v9516);
        let v9572=(common.v8*v2768);
        let v9573=((v9568+v9568)/v9572);
        let v9574=((v9570+v9570)/v9572);
        let v9577=(if self.scalar_static_bool[43]{(self.scalar_static_f64[132]*v9573)}else{common.v21});
        let v9578=(if self.scalar_static_bool[43]{(self.scalar_static_f64[132]*v9574)}else{common.v21});
        let v9579=(-v9577);
        let v9580=(-v9578);
        let v9581=(v2772*v9579);
        let v9583=(v2772*v9580);
        let v9585=(common.v8*v2776);
        let v9594=(v2780*v9577);
        let v9596=(v2780*v9578);
        let v9598=(common.v8*v2784);
        let v9605=(v2785*v2785);
        let v9620=(if self.scalar_static_bool[44]{(if v2773{(-(common.v42*(v9579+((v9581+v9581)/v9585))))}else{(if v2781{(-((-(common.v413*(v9577+((v9594+v9594)/v9598))))/v9605))}else{(-(common.v42*v9579))})})}else{v9577});
        let v9621=(if self.scalar_static_bool[44]{(if v2773{(-(common.v42*(v9580+((v9583+v9583)/v9585))))}else{(if v2781{(-((-(common.v413*(v9578+((v9596+v9596)/v9598))))/v9605))}else{(-(common.v42*v9580))})})}else{v9578});
        let v9622=(self.scalar_static_f64[19]*(if self.scalar_static_bool[40]{common.v21}else{v9507}));
        let v9623=(self.scalar_static_f64[19]*(if self.scalar_static_bool[40]{common.v21}else{v9508}));
        let v9628=(-(v9622+(self.scalar_static_f64[168]*v9566)));
        let v9629=(-(v9623+(self.scalar_static_f64[168]*v9567)));
        let v9632=(if self.scalar_static_bool[46]{v9628}else{(if self.scalar_static_bool[45]{v9628}else{common.v21})});
        let v9633=(if self.scalar_static_bool[46]{v9629}else{(if self.scalar_static_bool[45]{v9629}else{common.v21})});
        let v9640=(if v2818{v9632}else{(if v2812{((v2813*v9632)/v2814)}else{common.v21})});
        let v9641=(if v2818{v9633}else{(if v2812{((v2813*v9633)/v2814)}else{common.v21})});
        let v9648=(if self.scalar_static_bool[43]{(v9632+self.scalar_static_f64[347])}else{common.v21});
        let v9649=(if self.scalar_static_bool[43]{(v9633+self.scalar_static_f64[348])}else{common.v21});
        let v9656=(if v2831{v9648}else{(if v2825{((v2826*v9648)/v2827)}else{common.v21})});
        let v9657=(if v2831{v9649}else{(if v2825{((v2826*v9649)/v2827)}else{common.v21})});
        let v9668=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*((v2835*v9620)+(v2793*(self.scalar_static_f64[121]*v9620))))}else{common.v21});
        let v9669=(if self.scalar_static_bool[43]{(self.scalar_static_f64[136]*((v2835*v9621)+(v2793*(self.scalar_static_f64[121]*v9621))))}else{common.v21});
        let v9692=(-v9668);
        let v9693=(-v9669);
        let v9712=(v2865*v2865);
        let v9717=(if v2857{((-(common.v632*((v2863*v9692)+(v2858*((v2861*(common.v42*v9692))+(v2859*(common.v37*v9692)))))))/v9712)}else{(if v2853{(v2854*v9668)}else{(if v2841{((v2846*v9668)+(v2839*((v2844*(common.v42*v9668))+(v2842*(common.v37*v9668)))))}else{common.v21})})});
        let v9718=(if v2857{((-(common.v632*((v2863*v9693)+(v2858*((v2861*(common.v42*v9693))+(v2859*(common.v37*v9693)))))))/v9712)}else{(if v2853{(v2854*v9669)}else{(if v2841{((v2846*v9669)+(v2839*((v2844*(common.v42*v9669))+(v2842*(common.v37*v9669)))))}else{common.v21})})});
        let v9731=(if self.scalar_static_bool[43]{((v2870*(self.scalar_static_f64[19]*(self.scalar_static_f64[307]*v9717)))+(v2869*(v9656-v9640)))}else{common.v21});
        let v9732=(if self.scalar_static_bool[43]{((v2870*(self.scalar_static_f64[19]*(self.scalar_static_f64[307]*v9718)))+(v2869*(v9657-v9641)))}else{common.v21});
        let v9733=(if self.scalar_static_bool[47]{v9521}else{common.v21});
        let v9734=(if self.scalar_static_bool[47]{v9522}else{common.v21});
        let v9735=(v2875*v9733);
        let v9737=(v2875*v9734);
        let v9739=(common.v8*v2879);
        let v9748=(-v9733);
        let v9749=(-v9734);
        let v9750=(v2883*v9748);
        let v9752=(v2883*v9749);
        let v9754=(common.v8*v2887);
        let v9761=(v2888*v2888);
        let v9780=(if self.scalar_static_bool[47]{(self.scalar_static_f64[131]*v9573)}else{v9620});
        let v9781=(if self.scalar_static_bool[47]{(self.scalar_static_f64[131]*v9574)}else{v9621});
        let v9782=(-v9780);
        let v9783=(-v9781);
        let v9784=(v2900*v9782);
        let v9786=(v2900*v9783);
        let v9788=(common.v8*v2904);
        let v9797=(v2908*v9780);
        let v9799=(v2908*v9781);
        let v9801=(common.v8*v2912);
        let v9808=(v2913*v2913);
        let v9823=(if self.scalar_static_bool[48]{(if v2901{(-(common.v42*(v9782+((v9784+v9784)/v9788))))}else{(if v2909{(-((-(common.v413*(v9780+((v9797+v9797)/v9801))))/v9808))}else{(-(common.v42*v9782))})})}else{v9780});
        let v9824=(if self.scalar_static_bool[48]{(if v2901{(-(common.v42*(v9783+((v9786+v9786)/v9788))))}else{(if v2909{(-((-(common.v413*(v9781+((v9799+v9799)/v9801))))/v9808))}else{(-(common.v42*v9783))})})}else{v9781});
        let v9827=(v9622+(self.scalar_static_f64[168]*(if self.scalar_static_bool[47]{(if v2876{(v9733-(common.v42*(v9733+((v9735+v9735)/v9739))))}else{(if v2884{(v9733-((-(v2486*(v9748+((v9750+v9750)/v9754))))/v9761))}else{(v9733-(common.v42*v9733))})})}else{v9566})));
        let v9828=(v9623+(self.scalar_static_f64[168]*(if self.scalar_static_bool[47]{(if v2876{(v9734-(common.v42*(v9734+((v9737+v9737)/v9739))))}else{(if v2884{(v9734-((-(v2486*(v9749+((v9752+v9752)/v9754))))/v9761))}else{(v9734-(common.v42*v9734))})})}else{v9567})));
        let v9831=(if self.scalar_static_bool[50]{v9827}else{(if self.scalar_static_bool[49]{v9827}else{v9632})});
        let v9832=(if self.scalar_static_bool[50]{v9828}else{(if self.scalar_static_bool[49]{v9828}else{v9633})});
        let v9843=(if self.scalar_static_bool[47]{(v9831-self.scalar_static_f64[347])}else{v9648});
        let v9844=(if self.scalar_static_bool[47]{(v9832-self.scalar_static_f64[348])}else{v9649});
        let v9863=(if self.scalar_static_bool[47]{(self.scalar_static_f64[134]*((v2953*v9823)+(v2921*(self.scalar_static_f64[115]*v9823))))}else{v9668});
        let v9864=(if self.scalar_static_bool[47]{(self.scalar_static_f64[134]*((v2953*v9824)+(v2921*(self.scalar_static_f64[115]*v9824))))}else{v9669});
        let v9869=(-v9863);
        let v9870=(-v9864);
        let v9889=(v2974*v2974);
        let v9939=(if self.scalar_static_bool[54]{(self.scalar_static_f64[165]*(common.v5083-common.v8646))}else{common.v21});
        let v9940=(if self.scalar_static_bool[54]{(self.scalar_static_f64[165]*(common.v5084-common.v8647))}else{common.v21});
        let v9941=(if self.scalar_static_bool[54]{(self.scalar_static_f64[165]*common.v8676)}else{common.v21});
        let v9942=(self.scalar_static_f64[19]*v9939);
        let v9943=(self.scalar_static_f64[19]*v9940);
        let v9944=(self.scalar_static_f64[19]*v9941);
        let v9945=(if self.scalar_static_bool[56]{v9942}else{common.v21});
        let v9946=(if self.scalar_static_bool[56]{v9943}else{common.v21});
        let v9947=(if self.scalar_static_bool[56]{v9944}else{common.v21});
        let v9948=(-v9945);
        let v9949=(-v9946);
        let v9950=(-v9947);
        let v9951=(v3009*v9948);
        let v9953=(v3009*v9949);
        let v9955=(v3009*v9950);
        let v9957=(common.v8*v3013);
        let v9970=(v3008*v9945);
        let v9972=(v3008*v9946);
        let v9974=(v3008*v9947);
        let v9976=(common.v8*v3020);
        let v9985=(v3021*v3021);
        let v10008=(if self.scalar_static_bool[56]{(if v3010{(v9945+(common.v42*(v9948+((v9951+v9951)/v9957))))}else{(if v3017{(v9945+((-(v2486*(v9945+((v9970+v9970)/v9976))))/v9985))}else{(v9945+(common.v42*v9948))})})}else{common.v21});
        let v10009=(if self.scalar_static_bool[56]{(if v3010{(v9946+(common.v42*(v9949+((v9953+v9953)/v9957))))}else{(if v3017{(v9946+((-(v2486*(v9946+((v9972+v9972)/v9976))))/v9985))}else{(v9946+(common.v42*v9949))})})}else{common.v21});
        let v10010=(if self.scalar_static_bool[56]{(if v3010{(v9947+(common.v42*(v9950+((v9955+v9955)/v9957))))}else{(if v3017{(v9947+((-(v2486*(v9947+((v9974+v9974)/v9976))))/v9985))}else{(v9947+(common.v42*v9950))})})}else{common.v21});
        let v10011=(v3003*v9939);
        let v10013=(v3003*v9940);
        let v10015=(v3003*v9941);
        let v10017=(common.v8*v3032);
        let v10018=((v10011+v10011)/v10017);
        let v10019=((v10013+v10013)/v10017);
        let v10020=((v10015+v10015)/v10017);
        let v10024=(if self.scalar_static_bool[56]{(self.scalar_static_f64[132]*v10018)}else{common.v21});
        let v10025=(if self.scalar_static_bool[56]{(self.scalar_static_f64[132]*v10019)}else{common.v21});
        let v10026=(if self.scalar_static_bool[56]{(self.scalar_static_f64[132]*v10020)}else{common.v21});
        let v10027=(-v10024);
        let v10028=(-v10025);
        let v10029=(-v10026);
        let v10030=(v3036*v10027);
        let v10032=(v3036*v10028);
        let v10034=(v3036*v10029);
        let v10036=(common.v8*v3040);
        let v10049=(v3044*v10024);
        let v10051=(v3044*v10025);
        let v10053=(v3044*v10026);
        let v10055=(common.v8*v3048);
        let v10064=(v3049*v3049);
        let v10087=(if self.scalar_static_bool[57]{(if v3037{(-(common.v42*(v10027+((v10030+v10030)/v10036))))}else{(if v3045{(-((-(common.v413*(v10024+((v10049+v10049)/v10055))))/v10064))}else{(-(common.v42*v10027))})})}else{v10024});
        let v10088=(if self.scalar_static_bool[57]{(if v3037{(-(common.v42*(v10028+((v10032+v10032)/v10036))))}else{(if v3045{(-((-(common.v413*(v10025+((v10051+v10051)/v10055))))/v10064))}else{(-(common.v42*v10028))})})}else{v10025});
        let v10089=(if self.scalar_static_bool[57]{(if v3037{(-(common.v42*(v10029+((v10034+v10034)/v10036))))}else{(if v3045{(-((-(common.v413*(v10026+((v10053+v10053)/v10055))))/v10064))}else{(-(common.v42*v10029))})})}else{v10026});
        let v10090=(self.scalar_static_f64[19]*common.v8646);
        let v10091=(self.scalar_static_f64[19]*common.v8647);
        let v10092=(self.scalar_static_f64[19]*common.v8648);
        let v10099=(-(v10090+(self.scalar_static_f64[168]*v10008)));
        let v10100=(-(v10091+(self.scalar_static_f64[168]*v10009)));
        let v10101=(-(v10092+(self.scalar_static_f64[168]*v10010)));
        let v10105=(if self.scalar_static_bool[59]{v10099}else{(if self.scalar_static_bool[58]{v10099}else{common.v21})});
        let v10106=(if self.scalar_static_bool[59]{v10100}else{(if self.scalar_static_bool[58]{v10100}else{common.v21})});
        let v10107=(if self.scalar_static_bool[59]{v10101}else{(if self.scalar_static_bool[58]{v10101}else{common.v21})});
        let v10117=(if v3078{v10105}else{(if v3072{((v3073*v10105)/v3074)}else{common.v21})});
        let v10118=(if v3078{v10106}else{(if v3072{((v3073*v10106)/v3074)}else{common.v21})});
        let v10119=(if v3078{v10107}else{(if v3072{((v3073*v10107)/v3074)}else{common.v21})});
        let v10126=(if self.scalar_static_bool[56]{(v10105+self.scalar_static_f64[353])}else{common.v21});
        let v10127=(if self.scalar_static_bool[56]{(v10106+self.scalar_static_f64[354])}else{common.v21});
        let v10128=(if self.scalar_static_bool[56]{v10107}else{common.v21});
        let v10138=(if v3091{v10126}else{(if v3085{((v3086*v10126)/v3087)}else{common.v21})});
        let v10139=(if v3091{v10127}else{(if v3085{((v3086*v10127)/v3087)}else{common.v21})});
        let v10140=(if v3091{v10128}else{(if v3085{((v3086*v10128)/v3087)}else{common.v21})});
        let v10156=(if self.scalar_static_bool[56]{(self.scalar_static_f64[135]*((v3094*v10087)+(v3057*(self.scalar_static_f64[121]*v10087))))}else{common.v21});
        let v10157=(if self.scalar_static_bool[56]{(self.scalar_static_f64[135]*((v3094*v10088)+(v3057*(self.scalar_static_f64[121]*v10088))))}else{common.v21});
        let v10158=(if self.scalar_static_bool[56]{(self.scalar_static_f64[135]*((v3094*v10089)+(v3057*(self.scalar_static_f64[121]*v10089))))}else{common.v21});
        let v10192=(-v10156);
        let v10193=(-v10157);
        let v10194=(-v10158);
        let v10221=(v3124*v3124);
        let v10229=(if v3116{((-(common.v632*((v3122*v10192)+(v3117*((v3120*(common.v42*v10192))+(v3118*(common.v37*v10192)))))))/v10221)}else{(if v3112{(v3113*v10156)}else{(if v3100{((v3105*v10156)+(v3098*((v3103*(common.v42*v10156))+(v3101*(common.v37*v10156)))))}else{common.v21})})});
        let v10230=(if v3116{((-(common.v632*((v3122*v10193)+(v3117*((v3120*(common.v42*v10193))+(v3118*(common.v37*v10193)))))))/v10221)}else{(if v3112{(v3113*v10157)}else{(if v3100{((v3105*v10157)+(v3098*((v3103*(common.v42*v10157))+(v3101*(common.v37*v10157)))))}else{common.v21})})});
        let v10231=(if v3116{((-(common.v632*((v3122*v10194)+(v3117*((v3120*(common.v42*v10194))+(v3118*(common.v37*v10194)))))))/v10221)}else{(if v3112{(v3113*v10158)}else{(if v3100{((v3105*v10158)+(v3098*((v3103*(common.v42*v10158))+(v3101*(common.v37*v10158)))))}else{common.v21})})});
        let v10250=(if self.scalar_static_bool[56]{((v3129*(self.scalar_static_f64[19]*(self.scalar_static_f64[306]*v10229)))+(v3128*(v10138-v10117)))}else{common.v21});
        let v10251=(if self.scalar_static_bool[56]{((v3129*(self.scalar_static_f64[19]*(self.scalar_static_f64[306]*v10230)))+(v3128*(v10139-v10118)))}else{common.v21});
        let v10252=(if self.scalar_static_bool[56]{((v3129*(self.scalar_static_f64[19]*(self.scalar_static_f64[306]*v10231)))+(v3128*(v10140-v10119)))}else{common.v21});
        let v10253=(if self.scalar_static_bool[60]{v9942}else{common.v21});
        let v10254=(if self.scalar_static_bool[60]{v9943}else{common.v21});
        let v10255=(if self.scalar_static_bool[60]{v9944}else{common.v21});
        let v10256=(v3134*v10253);
        let v10258=(v3134*v10254);
        let v10260=(v3134*v10255);
        let v10262=(common.v8*v3138);
        let v10275=(-v10253);
        let v10276=(-v10254);
        let v10277=(-v10255);
        let v10278=(v3142*v10275);
        let v10280=(v3142*v10276);
        let v10282=(v3142*v10277);
        let v10284=(common.v8*v3146);
        let v10293=(v3147*v3147);
        let v10322=(if self.scalar_static_bool[60]{(self.scalar_static_f64[131]*v10018)}else{v10087});
        let v10323=(if self.scalar_static_bool[60]{(self.scalar_static_f64[131]*v10019)}else{v10088});
        let v10324=(if self.scalar_static_bool[60]{(self.scalar_static_f64[131]*v10020)}else{v10089});
        let v10325=(-v10322);
        let v10326=(-v10323);
        let v10327=(-v10324);
        let v10328=(v3159*v10325);
        let v10330=(v3159*v10326);
        let v10332=(v3159*v10327);
        let v10334=(common.v8*v3163);
        let v10347=(v3167*v10322);
        let v10349=(v3167*v10323);
        let v10351=(v3167*v10324);
        let v10353=(common.v8*v3171);
        let v10362=(v3172*v3172);
        let v10385=(if self.scalar_static_bool[61]{(if v3160{(-(common.v42*(v10325+((v10328+v10328)/v10334))))}else{(if v3168{(-((-(common.v413*(v10322+((v10347+v10347)/v10353))))/v10362))}else{(-(common.v42*v10325))})})}else{v10322});
        let v10386=(if self.scalar_static_bool[61]{(if v3160{(-(common.v42*(v10326+((v10330+v10330)/v10334))))}else{(if v3168{(-((-(common.v413*(v10323+((v10349+v10349)/v10353))))/v10362))}else{(-(common.v42*v10326))})})}else{v10323});
        let v10387=(if self.scalar_static_bool[61]{(if v3160{(-(common.v42*(v10327+((v10332+v10332)/v10334))))}else{(if v3168{(-((-(common.v413*(v10324+((v10351+v10351)/v10353))))/v10362))}else{(-(common.v42*v10327))})})}else{v10324});
        let v10391=(v10090+(self.scalar_static_f64[168]*(if self.scalar_static_bool[60]{(if v3135{(v10253-(common.v42*(v10253+((v10256+v10256)/v10262))))}else{(if v3143{(v10253-((-(v2486*(v10275+((v10278+v10278)/v10284))))/v10293))}else{(v10253-(common.v42*v10253))})})}else{v10008})));
        let v10392=(v10091+(self.scalar_static_f64[168]*(if self.scalar_static_bool[60]{(if v3135{(v10254-(common.v42*(v10254+((v10258+v10258)/v10262))))}else{(if v3143{(v10254-((-(v2486*(v10276+((v10280+v10280)/v10284))))/v10293))}else{(v10254-(common.v42*v10254))})})}else{v10009})));
        let v10393=(v10092+(self.scalar_static_f64[168]*(if self.scalar_static_bool[60]{(if v3135{(v10255-(common.v42*(v10255+((v10260+v10260)/v10262))))}else{(if v3143{(v10255-((-(v2486*(v10277+((v10282+v10282)/v10284))))/v10293))}else{(v10255-(common.v42*v10255))})})}else{v10010})));
        let v10397=(if self.scalar_static_bool[63]{v10391}else{(if self.scalar_static_bool[62]{v10391}else{v10105})});
        let v10398=(if self.scalar_static_bool[63]{v10392}else{(if self.scalar_static_bool[62]{v10392}else{v10106})});
        let v10399=(if self.scalar_static_bool[63]{v10393}else{(if self.scalar_static_bool[62]{v10393}else{v10107})});
        let v10414=(if self.scalar_static_bool[60]{(v10397-self.scalar_static_f64[353])}else{v10126});
        let v10415=(if self.scalar_static_bool[60]{(v10398-self.scalar_static_f64[354])}else{v10127});
        let v10416=(if self.scalar_static_bool[60]{v10399}else{v10128});
        let v10444=(if self.scalar_static_bool[60]{(self.scalar_static_f64[133]*((v3212*v10385)+(v3180*(self.scalar_static_f64[115]*v10385))))}else{v10156});
        let v10445=(if self.scalar_static_bool[60]{(self.scalar_static_f64[133]*((v3212*v10386)+(v3180*(self.scalar_static_f64[115]*v10386))))}else{v10157});
        let v10446=(if self.scalar_static_bool[60]{(self.scalar_static_f64[133]*((v3212*v10387)+(v3180*(self.scalar_static_f64[115]*v10387))))}else{v10158});
        let v10453=(-v10444);
        let v10454=(-v10445);
        let v10455=(-v10446);
        let v10482=(v3233*v3233);

        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (common.v1610),
            6,
            multiplicity * (common.v68),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((if (self.scalar_static_f64[57]!=0.0){(self.scalar_static_f64[277]*(ctx.node_voltage(nodes[0])-common.v3262))}else{common.v21})),
            0,
            multiplicity * (self.scalar_static_f64[356]),
            3,
            multiplicity * (self.scalar_static_f64[357]),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * ((if (self.scalar_static_f64[57]!=0.0){(self.scalar_static_f64[278]*(common.v3262-common.v370))}else{common.v21})),
            3,
            multiplicity * (self.scalar_static_f64[359]),
            4,
            multiplicity * (self.scalar_static_f64[360]),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(1),
            multiplicity * ((if (self.scalar_static_f64[57]!=0.0){(v3273*v3274)}else{common.v21})),
            [1, 4, 5, 6],
            [(if (self.scalar_static_f64[57]!=0.0){(-v3274)}else{common.v21}), (if (self.scalar_static_f64[57]!=0.0){(v3273*(((v2514*(self.scalar_static_f64[281]*((v2502*((self.scalar_static_f64[165]*(v2499*(-(if v2476{(-(common.v42*(v9000+((v9002+v9002)/v9006))))}else{(if v2485{(-((-(v2486*(common.v5941+((v9015+v9015)/v9019))))/v9026))}else{(-(common.v42*v9000))})}))))/v9047))+(v2501*((common.v2474*common.v3420)+(common.v450*common.v8997))))))-(v2511*(self.scalar_static_f64[151]*(common.v42*(self.scalar_static_f64[145]+((v9064+v9064)/v9068))))))/v9083))}else{common.v21}), (if (self.scalar_static_f64[57]!=0.0){(v3274+(v3273*(((v2514*(self.scalar_static_f64[281]*((v2502*((self.scalar_static_f64[165]*(v2499*(-(if v2476{(-(common.v42*(v9001+((v9004+v9004)/v9006))))}else{(if v2485{(-((-(v2486*(common.v5942+((v9017+v9017)/v9019))))/v9026))}else{(-(common.v42*v9001))})}))))/v9047))+(v2501*((common.v2474*common.v3421)+(common.v450*common.v8998))))))-(v2511*(self.scalar_static_f64[151]*(common.v42*(self.scalar_static_f64[19]+((v9066+v9066)/v9068))))))/v9083)))}else{common.v21}), (if (self.scalar_static_f64[57]!=0.0){(v3273*((self.scalar_static_f64[281]*(v2501*(common.v450*common.v8999)))/v2514))}else{common.v21})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((if (self.scalar_static_f64[57]!=0.0){(self.scalar_static_f64[279]*(common.v2520-ctx.node_voltage(nodes[2])))}else{common.v21})),
            1,
            multiplicity * (self.scalar_static_f64[362]),
            2,
            multiplicity * (self.scalar_static_f64[363]),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v21,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(4),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v21,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(1),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            common.v21,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(2),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            common.v21,
        );
        stamper.stamp_current_node3_local(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[60]{(v3131+(v3249*v3250))}else{v3131}))),
            4,
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[60]{(v10250+((v3250*(self.scalar_static_f64[19]*(self.scalar_static_f64[304]*(if v3237{(common.v558*((v3243*v10444)+(v3238*((v3241*(common.v42*v10444))+(v3239*(common.v37*v10444))))))}else{(if v3225{((-(common.v632*((v3231*v10453)+(v3226*((v3229*(common.v42*v10453))+(v3227*(common.v37*v10453)))))))/v10482)}else{(if v3219{(v3220*v10444)}else{v10229})})}))))+(v3249*((if v3198{v10397}else{(if v3192{((v3193*v10397)/v3194)}else{v10117})})-(if v3209{v10414}else{(if v3203{((v3204*v10414)/v3205)}else{v10138})})))))}else{v10250}))),
            5,
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[60]{(v10251+((v3250*(self.scalar_static_f64[19]*(self.scalar_static_f64[304]*(if v3237{(common.v558*((v3243*v10445)+(v3238*((v3241*(common.v42*v10445))+(v3239*(common.v37*v10445))))))}else{(if v3225{((-(common.v632*((v3231*v10454)+(v3226*((v3229*(common.v42*v10454))+(v3227*(common.v37*v10454)))))))/v10482)}else{(if v3219{(v3220*v10445)}else{v10230})})}))))+(v3249*((if v3198{v10398}else{(if v3192{((v3193*v10398)/v3194)}else{v10118})})-(if v3209{v10415}else{(if v3203{((v3204*v10415)/v3205)}else{v10139})})))))}else{v10251}))),
            6,
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[60]{(v10252+((v3250*(self.scalar_static_f64[19]*(self.scalar_static_f64[304]*(if v3237{(common.v558*((v3243*v10446)+(v3238*((v3241*(common.v42*v10446))+(v3239*(common.v37*v10446))))))}else{(if v3225{((-(common.v632*((v3231*v10455)+(v3226*((v3229*(common.v42*v10455))+(v3227*(common.v37*v10455)))))))/v10482)}else{(if v3219{(v3220*v10446)}else{v10231})})}))))+(v3249*((if v3198{v10399}else{(if v3192{((v3193*v10399)/v3194)}else{v10119})})-(if v3209{v10416}else{(if v3203{((v3204*v10416)/v3205)}else{v10140})})))))}else{v10252}))),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(1),
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[47]{(v2872+(v2990*v2991))}else{v2872}))),
            1,
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[47]{(v9731+((v2991*(self.scalar_static_f64[19]*(self.scalar_static_f64[305]*(if v2978{(common.v558*((v2984*v9863)+(v2979*((v2982*(common.v42*v9863))+(v2980*(common.v37*v9863))))))}else{(if v2966{((-(common.v632*((v2972*v9869)+(v2967*((v2970*(common.v42*v9869))+(v2968*(common.v37*v9869)))))))/v9889)}else{(if v2960{(v2961*v9863)}else{v9717})})}))))+(v2990*((if v2939{v9831}else{(if v2933{((v2934*v9831)/v2935)}else{v9640})})-(if v2950{v9843}else{(if v2944{((v2945*v9843)/v2946)}else{v9656})})))))}else{v9731}))),
            4,
            multiplicity * ((self.scalar_static_f64[19]*(if self.scalar_static_bool[47]{(v9732+((v2991*(self.scalar_static_f64[19]*(self.scalar_static_f64[305]*(if v2978{(common.v558*((v2984*v9864)+(v2979*((v2982*(common.v42*v9864))+(v2980*(common.v37*v9864))))))}else{(if v2966{((-(common.v632*((v2972*v9870)+(v2967*((v2970*(common.v42*v9870))+(v2968*(common.v37*v9870)))))))/v9889)}else{(if v2960{(v2961*v9864)}else{v9718})})}))))+(v2990*((if v2939{v9832}else{(if v2933{((v2934*v9832)/v2935)}else{v9641})})-(if v2950{v9844}else{(if v2944{((v2945*v9844)/v2946)}else{v9657})})))))}else{v9732}))),
        );
        let v3259_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v3259);
        stamper.stamp_current_node3_local(
            Some(4),
            Some(5),
            multiplicity * (v3259_ddt),
            4,
            multiplicity * (((common.v10568) * ddt_scale)),
            5,
            multiplicity * (((common.v10569) * ddt_scale)),
            6,
            multiplicity * (((common.v10570) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * ((if common.v1516{(v1605/v1607)}else{common.v21})),
            4,
            multiplicity * ((if common.v1516{(((v1607*((v1604*v6114)+(v1587*(self.scalar_static_f64[165]*common.v3425))))-(v1605*((if common.v1516{((v1601*common.v3422)+(common.v451*((v6114+common.v6133)/v6142)))}else{common.v21})+((common.v1599*common.v3422)+(common.v451*common.v6138)))))/v6172)}else{common.v21})),
            5,
            multiplicity * ((if common.v1516{(((v1607*((v1604*v6115)+(v1587*(self.scalar_static_f64[165]*common.v3427))))-(v1605*((if common.v1516{((v1601*common.v3423)+(common.v451*((v6115+common.v6134)/v6142)))}else{common.v21})+((common.v1599*common.v3423)+(common.v451*common.v6139)))))/v6172)}else{common.v21})),
        );
        let v3261_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v3261);
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v3261_ddt),
            6,
            multiplicity * (((self.scalar_static_f64[153]) * ddt_scale)),
        );
        let v3264_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v3264);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(1),
            multiplicity * (v3264_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[156]) * ddt_scale)),
            3,
            multiplicity * (((self.scalar_static_f64[56]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(1),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(2),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (common.v21),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (common.v21),
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
            multiplicity * (common.v10568),
            nodes[5],
            multiplicity * (common.v10569),
            nodes[6],
            multiplicity * (common.v10570),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (self.scalar_static_f64[153]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[156]),
            nodes[3],
            multiplicity * (self.scalar_static_f64[56]),
        );
    }
}
