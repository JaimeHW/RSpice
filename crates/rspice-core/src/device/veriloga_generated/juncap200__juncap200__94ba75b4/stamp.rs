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
    v5: f64,
    v59: f64,
    v399: f64,
    v400: f64,
    v4773: f64,
    v4774: f64,
    v4816: f64,
    v4972: f64,
    v4974: f64,
    v5005: f64,
    v5029: f64,
    v5037: f64,
    v5061: f64,
    v5088: f64,
    v5102: f64,
    v5116: f64,
    v5119: bool,
    v5126: bool,
    v5147: f64,
    v5173: f64,
    v5197: f64,
    v5229: f64,
    v5237: bool,
    v5239: bool,
    v5249: f64,
    v5290: f64,
    v5315: f64,
    v5343: f64,
    v5357: f64,
    v5371: f64,
    v5374: bool,
    v5381: bool,
    v5402: f64,
    v5428: f64,
    v5454: f64,
    v5486: f64,
    v5494: bool,
    v5496: bool,
    v5506: f64,
    v5545: f64,
    v5570: f64,
    v5598: f64,
    v5612: f64,
    v5626: f64,
    v5629: bool,
    v5636: bool,
    v5657: f64,
    v5683: f64,
    v5709: f64,
    v5741: f64,
    v5749: bool,
    v5751: bool,
    v5761: f64,
    v5909: f64,
    v6222: f64,
    v6223: f64,
    v6227: f64,
    v6228: f64,
    v6278: f64,
    v6279: f64,
    v6325: f64,
    v6326: f64,
    v6335: f64,
    v6336: f64,
    v6340: f64,
    v6404: f64,
    v6405: f64,
    v6488: f64,
    v6491: f64,
    v6539: f64,
    v6540: f64,
    v6577: f64,
    v6578: f64,
    v6632: f64,
    v6633: f64,
    v6693: f64,
    v6694: f64,
    v6760: f64,
    v6761: f64,
    v6818: f64,
    v6819: f64,
    v6862: f64,
    v6863: f64,
    v6926: f64,
    v6927: f64,
    v6931: f64,
    v6997: f64,
    v6998: f64,
    v7083: f64,
    v7086: f64,
    v7134: f64,
    v7135: f64,
    v7172: f64,
    v7173: f64,
    v7227: f64,
    v7228: f64,
    v7288: f64,
    v7289: f64,
    v7355: f64,
    v7356: f64,
    v7413: f64,
    v7414: f64,
    v7459: f64,
    v7460: f64,
    v7521: f64,
    v7522: f64,
    v7526: f64,
    v7592: f64,
    v7593: f64,
    v7678: f64,
    v7681: f64,
    v7729: f64,
    v7730: f64,
    v7767: f64,
    v7768: f64,
    v7822: f64,
    v7823: f64,
    v7883: f64,
    v7884: f64,
    v7950: f64,
    v7951: f64,
    v8008: f64,
    v8009: f64,
    v8054: f64,
    v8055: f64,
    v8291: f64,
    v8292: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=0.0;
        let v3=0.5;
        let v5=1.0;
        let v59=2.0;
        let v60=3.0;
        let v392=230.25850929940458;
        let v399=1e-100;
        let v400=-230.25850929940458;
        let v402=0.3333333333333333;
        let v414=1e100;
        let v700=0.375;
        let v4773=(self.scalar_static_f64[515]*(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1])));
        let v4774=(self.scalar_static_f64[559]*v4773);
        let v4816=(-v4773);
        let v4843=(if self.scalar_static_bool[29]{(v4773+self.scalar_static_f64[4096])}else{v1});
        let v4845=(if self.scalar_static_bool[29]{(self.scalar_static_f64[770]+v4843)}else{v1});
        let v4847=(if self.scalar_static_bool[29]{(self.scalar_static_f64[770]-v4843)}else{v1});
        let v4850=((self.scalar_static_f64[4094]+(v4847*v4847))).sqrt();
        let v4851=(if self.scalar_static_bool[29]{v4850}else{v1});
        let v4852=(self.scalar_static_f64[770]*v4773);
        let v4853=(v4845+v4851);
        let v4856=(if self.scalar_static_bool[29]{(v59*(v4852/v4853))}else{v1});
        let v4862=(v5-(self.scalar_static_f64[624]*v4856));
        let v4863=(v4862).sqrt();
        let v4868=(if self.scalar_static_bool[743]{f64::powf(v4862,self.scalar_static_f64[19])}else{(if self.scalar_static_bool[742]{v4863}else{v1})});
        let v4871=(v4773-v4856);
        let v4880=(v5-(self.scalar_static_f64[625]*v4856));
        let v4881=(v4880).sqrt();
        let v4886=(if self.scalar_static_bool[747]{f64::powf(v4880,self.scalar_static_f64[21])}else{(if self.scalar_static_bool[746]{v4881}else{v4868})});
        let v4897=(v5-(self.scalar_static_f64[626]*v4856));
        let v4898=(v4897).sqrt();
        let v4915=(if self.scalar_static_bool[247]{(v4773+self.scalar_static_f64[4099])}else{v4843});
        let v4917=(if self.scalar_static_bool[247]{(self.scalar_static_f64[770]+v4915)}else{v4845});
        let v4919=(if self.scalar_static_bool[247]{(self.scalar_static_f64[770]-v4915)}else{v4847});
        let v4922=((self.scalar_static_f64[4097]+(v4919*v4919))).sqrt();
        let v4923=(if self.scalar_static_bool[247]{v4922}else{v4851});
        let v4924=(v4917+v4923);
        let v4927=(if self.scalar_static_bool[247]{(v59*(v4852/v4924))}else{v1});
        let v4928=(v4773<self.scalar_static_f64[736]);
        let v4929=(-0.5*v4774);
        let v4931=((v4929).abs()<v392);
        let v4932=(self.scalar_static_bool[247]&&v4928);
        let v4933=(v4931&&v4932);
        let v4934=(v4929).exp();
        let v4936=(v4929<v1);
        let v4938=(v4932&&(!v4931));
        let v4939=(v4936&&v4938);
        let v4940=(v400-v4929);
        let v4942=(v5+(v402*v4940));
        let v4945=(v5+(v3*(v4940*v4942)));
        let v4947=(v5+(v4940*v4945));
        let v4951=(v4938&&(!v4936));
        let v4952=(v4929-v392);
        let v4954=(v5+(v402*v4952));
        let v4957=(v5+(v3*(v4952*v4954)));
        let v4961=(if v4951{(v414*(v5+(v4952*v4957)))}else{(if v4939{(v399/v4947)}else{(if v4933{v4934}else{v1})})});
        let v4963=(if v4932{(v5/v4961)}else{v1});
        let v4967=(self.scalar_static_bool[247]&&(!v4928));
        let v4972=(if v4967{(self.scalar_static_f64[760]*(v5+(self.scalar_static_f64[559]*(v4773-self.scalar_static_f64[736]))))}else{(if v4932{(v4963*v4963)}else{v1})});
        let v4973=(v4972).sqrt();
        let v4974=(if v4967{v4973}else{v4963});
        let v4976=(if v4967{(v5/v4974)}else{v4961});
        let v4979=(v4773>v1);
        let v4980=(self.scalar_static_bool[247]&&v4979);
        let v4982=(v5+v4976);
        let v4983=(v60+v4976);
        let v4985=((v4982*v4983)).sqrt();
        let v4986=((v59+v4976)+v4985);
        let v4992=(self.scalar_static_bool[247]&&(!v4979));
        let v4995=(v5+v4974);
        let v4997=(v5+(v60*v4974));
        let v4999=((v4995*v4997)).sqrt();
        let v5000=((v5+(v59*v4974))+v4999);
        let v5005=(if v4992{(v4816+(v59*(self.scalar_static_f64[558]*(v5000).ln())))}else{(if v4980{(v59*(self.scalar_static_f64[558]*(v4986).ln()))}else{v1})});
        let v5007=(if self.scalar_static_bool[247]{(self.scalar_static_f64[768]-v5005)}else{v1});
        let v5009=(v4773-v5007);
        let v5012=((self.scalar_static_f64[843]+(v5009*v5009))).sqrt();
        let v5015=(if self.scalar_static_bool[247]{(v3*((v4773+v5007)-v5012))}else{v1});
        let v5017=(v4773-self.scalar_static_f64[170]);
        let v5020=((self.scalar_static_f64[191]+(v5017*v5017))).sqrt();
        let v5023=(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[170]+v4773)-v5020))}else{v1});
        let v5026=((4e-12+(v4773*v4773))).sqrt();
        let v5029=(if self.scalar_static_bool[247]{(v3*(v4773-v5026))}else{v1});
        let v5037=(if self.scalar_static_bool[250]{(self.scalar_static_f64[609]-v5015)}else{v1});
        let v5055=(self.scalar_static_f64[41]*v5037);
        let v5056=(v5055).sqrt();
        let v5059=(if self.scalar_static_bool[252]{f64::powf(v5055,self.scalar_static_f64[18])}else{(if self.scalar_static_bool[251]{v5056}else{v1})});
        let v5061=(if self.scalar_static_bool[250]{(self.scalar_static_f64[28]*v5059)}else{v1});
        let v5070=(self.scalar_static_f64[19]*v5061);
        let v5073=(if self.scalar_static_bool[253]{(self.scalar_static_f64[658]*(v5070/v5037))}else{v1});
        let v5075=(if self.scalar_static_bool[253]{(self.scalar_static_f64[886]/v5073)}else{v1});
        let v5077=(if self.scalar_static_bool[253]{(v5075*v5075)}else{v1});
        let v5078=(v5077*v5077);
        let v5079=(v5+v5078);
        let v5081=((v5078/v5079)).sqrt();
        let v5082=(if self.scalar_static_bool[253]{v5081}else{v1});
        let v5083=(v5082).sqrt();
        let v5084=(if self.scalar_static_bool[253]{v5083}else{v1});
        let v5086=(if self.scalar_static_bool[253]{(v5082*v5084)}else{v1});
        let v5088=(v5073*v5086);
        let v5101=((v700*(v5073/v5084))).sqrt();
        let v5102=(if self.scalar_static_bool[253]{v5101}else{v1});
        let v5106=(if self.scalar_static_bool[253]{((v59*(v5075*v5084))-v5082)}else{v1});
        let v5107=(self.scalar_static_f64[651]*v5075);
        let v5113=(if self.scalar_static_bool[253]{(((v5084*v5107)-(self.scalar_static_f64[651]*v5082))+(v3*v5088))}else{v1});
        let v5114=(v5106-v5);
        let v5116=(if self.scalar_static_bool[253]{(v5102*v5114)}else{v1});
        let v5118=(if self.scalar_static_bool[253]{(v5116*v5116)}else{v1});
        let v5119=(v5116>v1);
        let v5126=(self.scalar_static_bool[253]&&(!v5119));
        let v5131=(v5113+(-v5118));
        let v5132=(v5131>v400);
        let v5133=(self.scalar_static_bool[253]&&v5132);
        let v5134=(v5131).exp();
        let v5137=(self.scalar_static_bool[253]&&(!v5132));
        let v5138=(v400-v5131);
        let v5140=(v5+(v402*v5138));
        let v5143=(v5+(v3*(v5138*v5140)));
        let v5145=(v5+(v5138*v5143));
        let v5147=(if v5137{(v399/v5145)}else{(if v5133{v5134}else{v5059})});
        let v5158=(v5113>v400);
        let v5159=(v5126&&v5158);
        let v5160=(v5113).exp();
        let v5163=(v5126&&(!v5158));
        let v5164=(v400-v5113);
        let v5166=(v5+(v402*v5164));
        let v5169=(v5+(v3*(v5164*v5166)));
        let v5171=(v5+(v5164*v5169));
        let v5173=(if v5163{(v399/v5171)}else{(if v5159{v5160}else{v5147})});
        let v5187=(self.scalar_static_f64[40]-v5023);
        let v5188=(self.scalar_static_f64[41]*v5187);
        let v5189=(v5188).sqrt();
        let v5193=(if self.scalar_static_bool[258]{f64::powf(v5188,self.scalar_static_f64[18])}else{(if self.scalar_static_bool[257]{v5189}else{v5173})});
        let v5194=(self.scalar_static_f64[37]*v5187);
        let v5197=(if self.scalar_static_bool[256]{(self.scalar_static_f64[24]*(v5194/v5193))}else{v1});
        let v5198=(self.scalar_static_f64[989]/v5197);
        let v5200=((v5198).abs()<v392);
        let v5201=(self.scalar_static_bool[256]&&v5200);
        let v5202=(v5198).exp();
        let v5204=(v5198<v1);
        let v5206=(self.scalar_static_bool[256]&&(!v5200));
        let v5207=(v5204&&v5206);
        let v5208=(v400-v5198);
        let v5210=(v5+(v402*v5208));
        let v5213=(v5+(v3*(v5208*v5210)));
        let v5215=(v5+(v5208*v5213));
        let v5219=(v5206&&(!v5204));
        let v5220=(v5198-v392);
        let v5222=(v5+(v402*v5220));
        let v5225=(v5+(v3*(v5220*v5222)));
        let v5229=(if v5219{(v414*(v5+(v5220*v5225)))}else{(if v5207{(v399/v5215)}else{(if v5201{v5202}else{v5193})})});
        let v5237=(v5029>self.scalar_static_f64[217]);
        let v5239=(v5237&&self.scalar_static_bool[260]);
        let v5240=(self.scalar_static_bool[67]&&v5239);
        let v5241=(self.scalar_static_f64[62]*v5029);
        let v5242=(v5241*v5241);
        let v5243=(v5241*v5242);
        let v5246=(self.scalar_static_bool[72]&&v5239);
        let v5249=(if v5246{f64::powf((v5241).abs(),self.scalar_static_f64[49])}else{(if v5240{(v5241*v5243)}else{v5229})});
        let v5267=(v5-(self.scalar_static_f64[624]*v4927));
        let v5268=(v5267).sqrt();
        let v5272=(if self.scalar_static_bool[262]{f64::powf(v5267,self.scalar_static_f64[19])}else{(if self.scalar_static_bool[261]{v5268}else{v5249})});
        let v5276=(v4773-v4927);
        let v5290=(if self.scalar_static_bool[266]{(self.scalar_static_f64[616]-v5015)}else{v5037});
        let v5309=(self.scalar_static_f64[43]*v5290);
        let v5310=(v5309).sqrt();
        let v5313=(if self.scalar_static_bool[268]{f64::powf(v5309,self.scalar_static_f64[20])}else{(if self.scalar_static_bool[267]{v5310}else{v5272})});
        let v5315=(if self.scalar_static_bool[266]{(self.scalar_static_f64[32]*v5313)}else{v5061});
        let v5325=(self.scalar_static_f64[21]*v5315);
        let v5328=(if self.scalar_static_bool[270]{(self.scalar_static_f64[663]*(v5325/v5290))}else{v5073});
        let v5330=(if self.scalar_static_bool[270]{(self.scalar_static_f64[1070]/v5328)}else{v5075});
        let v5332=(if self.scalar_static_bool[270]{(v5330*v5330)}else{v5077});
        let v5333=(v5332*v5332);
        let v5334=(v5+v5333);
        let v5336=((v5333/v5334)).sqrt();
        let v5337=(if self.scalar_static_bool[270]{v5336}else{v5082});
        let v5338=(v5337).sqrt();
        let v5339=(if self.scalar_static_bool[270]{v5338}else{v5084});
        let v5341=(if self.scalar_static_bool[270]{(v5337*v5339)}else{v5086});
        let v5343=(v5328*v5341);
        let v5356=((v700*(v5328/v5339))).sqrt();
        let v5357=(if self.scalar_static_bool[270]{v5356}else{v5102});
        let v5361=(if self.scalar_static_bool[270]{((v59*(v5330*v5339))-v5337)}else{v5106});
        let v5362=(self.scalar_static_f64[652]*v5330);
        let v5368=(if self.scalar_static_bool[270]{(((v5339*v5362)-(self.scalar_static_f64[652]*v5337))+(v3*v5343))}else{v5113});
        let v5369=(v5361-v5);
        let v5371=(if self.scalar_static_bool[270]{(v5357*v5369)}else{v5116});
        let v5373=(if self.scalar_static_bool[270]{(v5371*v5371)}else{v5118});
        let v5374=(v5371>v1);
        let v5381=(self.scalar_static_bool[270]&&(!v5374));
        let v5386=(v5368+(-v5373));
        let v5387=(v5386>v400);
        let v5388=(self.scalar_static_bool[270]&&v5387);
        let v5389=(v5386).exp();
        let v5392=(self.scalar_static_bool[270]&&(!v5387));
        let v5393=(v400-v5386);
        let v5395=(v5+(v402*v5393));
        let v5398=(v5+(v3*(v5393*v5395)));
        let v5400=(v5+(v5393*v5398));
        let v5402=(if v5392{(v399/v5400)}else{(if v5388{v5389}else{v5313})});
        let v5413=(v5368>v400);
        let v5414=(v5381&&v5413);
        let v5415=(v5368).exp();
        let v5418=(v5381&&(!v5413));
        let v5419=(v400-v5368);
        let v5421=(v5+(v402*v5419));
        let v5424=(v5+(v3*(v5419*v5421)));
        let v5426=(v5+(v5419*v5424));
        let v5428=(if v5418{(v399/v5426)}else{(if v5414{v5415}else{v5402})});
        let v5444=(self.scalar_static_f64[42]-v5023);
        let v5445=(self.scalar_static_f64[43]*v5444);
        let v5446=(v5445).sqrt();
        let v5450=(if self.scalar_static_bool[276]{f64::powf(v5445,self.scalar_static_f64[20])}else{(if self.scalar_static_bool[275]{v5446}else{v5428})});
        let v5451=(self.scalar_static_f64[38]*v5444);
        let v5454=(if self.scalar_static_bool[274]{(self.scalar_static_f64[25]*(v5451/v5450))}else{v5197});
        let v5455=(self.scalar_static_f64[1174]/v5454);
        let v5457=((v5455).abs()<v392);
        let v5458=(self.scalar_static_bool[274]&&v5457);
        let v5459=(v5455).exp();
        let v5461=(v5455<v1);
        let v5463=(self.scalar_static_bool[274]&&(!v5457));
        let v5464=(v5461&&v5463);
        let v5465=(v400-v5455);
        let v5467=(v5+(v402*v5465));
        let v5470=(v5+(v3*(v5465*v5467)));
        let v5472=(v5+(v5465*v5470));
        let v5476=(v5463&&(!v5461));
        let v5477=(v5455-v392);
        let v5479=(v5+(v402*v5477));
        let v5482=(v5+(v3*(v5477*v5479)));
        let v5486=(if v5476{(v414*(v5+(v5477*v5482)))}else{(if v5464{(v399/v5472)}else{(if v5458{v5459}else{v5450})})});
        let v5494=(v5029>self.scalar_static_f64[241]);
        let v5496=(v5494&&self.scalar_static_bool[278]);
        let v5497=(self.scalar_static_bool[105]&&v5496);
        let v5498=(self.scalar_static_f64[64]*v5029);
        let v5499=(v5498*v5498);
        let v5500=(v5498*v5499);
        let v5503=(self.scalar_static_bool[110]&&v5496);
        let v5506=(if v5503{f64::powf((v5498).abs(),self.scalar_static_f64[53])}else{(if v5497{(v5498*v5500)}else{v5486})});
        let v5524=(v5-(self.scalar_static_f64[625]*v4927));
        let v5525=(v5524).sqrt();
        let v5529=(if self.scalar_static_bool[280]{f64::powf(v5524,self.scalar_static_f64[21])}else{(if self.scalar_static_bool[279]{v5525}else{v5506})});
        let v5545=(if self.scalar_static_bool[284]{(self.scalar_static_f64[623]-v5015)}else{v5290});
        let v5564=(self.scalar_static_f64[45]*v5545);
        let v5565=(v5564).sqrt();
        let v5568=(if self.scalar_static_bool[286]{f64::powf(v5564,self.scalar_static_f64[22])}else{(if self.scalar_static_bool[285]{v5565}else{v5529})});
        let v5570=(if self.scalar_static_bool[284]{(self.scalar_static_f64[36]*v5568)}else{v5315});
        let v5580=(self.scalar_static_f64[23]*v5570);
        let v5583=(if self.scalar_static_bool[288]{(self.scalar_static_f64[668]*(v5580/v5545))}else{v5328});
        let v5585=(if self.scalar_static_bool[288]{(self.scalar_static_f64[1256]/v5583)}else{v5330});
        let v5587=(if self.scalar_static_bool[288]{(v5585*v5585)}else{v5332});
        let v5588=(v5587*v5587);
        let v5589=(v5+v5588);
        let v5591=((v5588/v5589)).sqrt();
        let v5592=(if self.scalar_static_bool[288]{v5591}else{v5337});
        let v5593=(v5592).sqrt();
        let v5594=(if self.scalar_static_bool[288]{v5593}else{v5339});
        let v5596=(if self.scalar_static_bool[288]{(v5592*v5594)}else{v5341});
        let v5598=(v5583*v5596);
        let v5611=((v700*(v5583/v5594))).sqrt();
        let v5612=(if self.scalar_static_bool[288]{v5611}else{v5357});
        let v5617=(self.scalar_static_f64[653]*v5585);
        let v5623=(if self.scalar_static_bool[288]{(((v5594*v5617)-(self.scalar_static_f64[653]*v5592))+(v3*v5598))}else{v5368});
        let v5624=((if self.scalar_static_bool[288]{((v59*(v5585*v5594))-v5592)}else{v5361})-v5);
        let v5626=(if self.scalar_static_bool[288]{(v5612*v5624)}else{v5371});
        let v5629=(v5626>v1);
        let v5636=(self.scalar_static_bool[288]&&(!v5629));
        let v5641=(v5623+(-(if self.scalar_static_bool[288]{(v5626*v5626)}else{v5373})));
        let v5642=(v5641>v400);
        let v5643=(self.scalar_static_bool[288]&&v5642);
        let v5644=(v5641).exp();
        let v5647=(self.scalar_static_bool[288]&&(!v5642));
        let v5648=(v400-v5641);
        let v5650=(v5+(v402*v5648));
        let v5653=(v5+(v3*(v5648*v5650)));
        let v5655=(v5+(v5648*v5653));
        let v5657=(if v5647{(v399/v5655)}else{(if v5643{v5644}else{v5568})});
        let v5668=(v5623>v400);
        let v5669=(v5636&&v5668);
        let v5670=(v5623).exp();
        let v5673=(v5636&&(!v5668));
        let v5674=(v400-v5623);
        let v5676=(v5+(v402*v5674));
        let v5679=(v5+(v3*(v5674*v5676)));
        let v5681=(v5+(v5674*v5679));
        let v5683=(if v5673{(v399/v5681)}else{(if v5669{v5670}else{v5657})});
        let v5699=(self.scalar_static_f64[44]-v5023);
        let v5700=(self.scalar_static_f64[45]*v5699);
        let v5701=(v5700).sqrt();
        let v5705=(if self.scalar_static_bool[294]{f64::powf(v5700,self.scalar_static_f64[22])}else{(if self.scalar_static_bool[293]{v5701}else{v5683})});
        let v5706=(self.scalar_static_f64[39]*v5699);
        let v5709=(if self.scalar_static_bool[292]{(self.scalar_static_f64[26]*(v5706/v5705))}else{v5454});
        let v5710=(self.scalar_static_f64[1360]/v5709);
        let v5712=((v5710).abs()<v392);
        let v5713=(self.scalar_static_bool[292]&&v5712);
        let v5714=(v5710).exp();
        let v5716=(v5710<v1);
        let v5718=(self.scalar_static_bool[292]&&(!v5712));
        let v5719=(v5716&&v5718);
        let v5720=(v400-v5710);
        let v5722=(v5+(v402*v5720));
        let v5725=(v5+(v3*(v5720*v5722)));
        let v5727=(v5+(v5720*v5725));
        let v5731=(v5718&&(!v5716));
        let v5732=(v5710-v392);
        let v5734=(v5+(v402*v5732));
        let v5737=(v5+(v3*(v5732*v5734)));
        let v5741=(if v5731{(v414*(v5+(v5732*v5737)))}else{(if v5719{(v399/v5727)}else{(if v5713{v5714}else{v5705})})});
        let v5749=(v5029>self.scalar_static_f64[264]);
        let v5751=(v5749&&self.scalar_static_bool[296]);
        let v5752=(self.scalar_static_bool[143]&&v5751);
        let v5753=(self.scalar_static_f64[66]*v5029);
        let v5754=(v5753*v5753);
        let v5755=(v5753*v5754);
        let v5758=(self.scalar_static_bool[148]&&v5751);
        let v5761=(if v5758{f64::powf((v5753).abs(),self.scalar_static_f64[57])}else{(if v5752{(v5753*v5755)}else{v5741})});
        let v5779=(v4773<self.scalar_static_f64[518]);
        let v5782=((v4773-self.scalar_static_f64[518])/self.scalar_static_f64[519]);
        let v5785=(v5782< -37.0);
        let v5786=(v5782).exp();
        let v5787=(v5+v5786);
        let v5792=(v5782>37.0);
        let v5795=(((self.scalar_static_f64[518]-v4773)/self.scalar_static_f64[519])).exp();
        let v5796=(v5+v5795);
        let v5802=(if self.scalar_static_bool[297]{(if v5779{(if v5785{self.scalar_static_f64[518]}else{(self.scalar_static_f64[518]+(self.scalar_static_f64[519]*(v5787).ln()))})}else{(if v5792{v4773}else{(v4773+(self.scalar_static_f64[519]*(v5796).ln()))})})}else{v1});
        let v5807=(if self.scalar_static_bool[297]{(v5802+self.scalar_static_f64[4102])}else{v4915});
        let v5809=(if self.scalar_static_bool[297]{(self.scalar_static_f64[770]+v5807)}else{v4917});
        let v5811=(if self.scalar_static_bool[297]{(self.scalar_static_f64[770]-v5807)}else{v4919});
        let v5814=((self.scalar_static_f64[4100]+(v5811*v5811))).sqrt();
        let v5815=(if self.scalar_static_bool[297]{v5814}else{v4923});
        let v5816=(self.scalar_static_f64[770]*v5802);
        let v5817=(v5809+v5815);
        let v5820=(if self.scalar_static_bool[297]{(v59*(v5816/v5817))}else{v1});
        let v5823=(v5-(self.scalar_static_f64[626]*v5820));
        let v5824=(v5823).sqrt();
        let v5828=(if self.scalar_static_bool[299]{f64::powf(v5823,self.scalar_static_f64[23])}else{(if self.scalar_static_bool[298]{v5824}else{v5761})});
        let v5835=(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(v5-v5828))+(self.scalar_static_f64[644]*(v5802-v5820))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[641]*(v5-(if self.scalar_static_bool[751]{f64::powf(v4897,self.scalar_static_f64[23])}else{(if self.scalar_static_bool[750]{v4898}else{v4886})})))+(self.scalar_static_f64[644]*v4871))}else{v1})})});
        let v5838=(if self.scalar_static_bool[297]{((v4773+self.scalar_static_f64[518])-v5802)}else{v5802});
        let v5843=(if self.scalar_static_bool[297]{(v5838+self.scalar_static_f64[4105])}else{v5807});
        let v5847=(if self.scalar_static_bool[297]{(self.scalar_static_f64[770]-v5843)}else{v5811});
        let v5850=((self.scalar_static_f64[4103]+(v5847*v5847))).sqrt();
        let v5852=(self.scalar_static_f64[770]*v5838);
        let v5853=((if self.scalar_static_bool[297]{(self.scalar_static_f64[770]+v5843)}else{v5809})+(if self.scalar_static_bool[297]{v5850}else{v5815}));
        let v5856=(if self.scalar_static_bool[297]{(v59*(v5852/v5853))}else{v5820});
        let v5860=(v5-(self.scalar_static_f64[704]*v5856));
        let v5861=(v5860).sqrt();
        let v5866=(if self.scalar_static_bool[303]{f64::powf(v5860,self.scalar_static_f64[109])}else{(if self.scalar_static_bool[301]{v5861}else{v5828})});
        let v5880=(v5-(self.scalar_static_f64[626]*v4927));
        let v5881=(v5880).sqrt();
        let v5909=((((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[517]*((self.scalar_static_f64[637]*(v5-v5272))+(self.scalar_static_f64[642]*v5276)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[637]*(v5-v4868))+(self.scalar_static_f64[642]*v4871))}else{v1})})}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[517]*((self.scalar_static_f64[639]*(v5-v5529))+(self.scalar_static_f64[643]*v5276)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[639]*(v5-v4886))+(self.scalar_static_f64[643]*v4871))}else{v1})})})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(v5-(if self.scalar_static_bool[307]{f64::powf(v5880,self.scalar_static_f64[23])}else{(if self.scalar_static_bool[306]{v5881}else{v5866})})))+(self.scalar_static_f64[644]*v5276)))}else{(if self.scalar_static_bool[297]{(v5835+(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[711]*(v5-v5866))+(self.scalar_static_f64[713]*(v5838-v5856))))}else{v1}))}else{v5835})})))*self.scalar_static_f64[522]);
        let v6016=(v4847*self.scalar_static_f64[532]);
        let v6018=(v4847*self.scalar_static_f64[533]);
        let v6020=(v59*v4850);
        let v6023=(if self.scalar_static_bool[29]{((v6016+v6016)/v6020)}else{v1});
        let v6024=(if self.scalar_static_bool[29]{((v6018+v6018)/v6020)}else{v1});
        let v6032=(v4853*v4853);
        let v6040=(if self.scalar_static_bool[29]{(v59*(((v4853*self.scalar_static_f64[4144])-(v4852*(self.scalar_static_f64[528]+v6023)))/v6032))}else{v1});
        let v6041=(if self.scalar_static_bool[29]{(v59*(((v4853*self.scalar_static_f64[4145])-(v4852*(self.scalar_static_f64[529]+v6024)))/v6032))}else{v1});
        let v6044=(-(self.scalar_static_f64[624]*v6040));
        let v6045=(-(self.scalar_static_f64[624]*v6041));
        let v6046=(v59*v4863);
        let v6053=(self.scalar_static_f64[19]*f64::powf(v4862,self.scalar_static_f64[534]));
        let v6056=(if self.scalar_static_bool[743]{(v6044*v6053)}else{(if self.scalar_static_bool[742]{(v6044/v6046)}else{v1})});
        let v6057=(if self.scalar_static_bool[743]{(v6045*v6053)}else{(if self.scalar_static_bool[742]{(v6045/v6046)}else{v1})});
        let v6062=(self.scalar_static_f64[515]-v6040);
        let v6063=(self.scalar_static_f64[525]-v6041);
        let v6072=(-(self.scalar_static_f64[625]*v6040));
        let v6073=(-(self.scalar_static_f64[625]*v6041));
        let v6074=(v59*v4881);
        let v6081=(self.scalar_static_f64[21]*f64::powf(v4880,self.scalar_static_f64[535]));
        let v6084=(if self.scalar_static_bool[747]{(v6072*v6081)}else{(if self.scalar_static_bool[746]{(v6072/v6074)}else{v6056})});
        let v6085=(if self.scalar_static_bool[747]{(v6073*v6081)}else{(if self.scalar_static_bool[746]{(v6073/v6074)}else{v6057})});
        let v6098=(-(self.scalar_static_f64[626]*v6040));
        let v6099=(-(self.scalar_static_f64[626]*v6041));
        let v6100=(v59*v4898);
        let v6107=(self.scalar_static_f64[23]*f64::powf(v4897,self.scalar_static_f64[536]));
        let v6130=(v4919*self.scalar_static_f64[543]);
        let v6132=(v4919*self.scalar_static_f64[544]);
        let v6134=(v59*v4922);
        let v6137=(if self.scalar_static_bool[247]{((v6130+v6130)/v6134)}else{v6023});
        let v6138=(if self.scalar_static_bool[247]{((v6132+v6132)/v6134)}else{v6024});
        let v6144=(v4924*v4924);
        let v6152=(if self.scalar_static_bool[247]{(v59*(((v4924*self.scalar_static_f64[4144])-(v4852*(self.scalar_static_f64[539]+v6137)))/v6144))}else{v1});
        let v6153=(if self.scalar_static_bool[247]{(v59*(((v4924*self.scalar_static_f64[4145])-(v4852*(self.scalar_static_f64[540]+v6138)))/v6144))}else{v1});
        let v6180=(v4947*v4947);
        let v6205=(if v4951{(v414*((v4957*self.scalar_static_f64[4146])+(v4952*(v3*((v4954*self.scalar_static_f64[4146])+(v4952*self.scalar_static_f64[4152]))))))}else{(if v4939{((-(v399*((v4945*self.scalar_static_f64[4148])+(v4940*(v3*((v4942*self.scalar_static_f64[4148])+(v4940*self.scalar_static_f64[4150])))))))/v6180)}else{(if v4933{(v4934*self.scalar_static_f64[4146])}else{v1})})});
        let v6206=(if v4951{(v414*((v4957*self.scalar_static_f64[4147])+(v4952*(v3*((v4954*self.scalar_static_f64[4147])+(v4952*self.scalar_static_f64[4153]))))))}else{(if v4939{((-(v399*((v4945*self.scalar_static_f64[4149])+(v4940*(v3*((v4942*self.scalar_static_f64[4149])+(v4940*self.scalar_static_f64[4151])))))))/v6180)}else{(if v4933{(v4934*self.scalar_static_f64[4147])}else{v1})})});
        let v6208=(v4961*v4961);
        let v6212=(if v4932{((-v6205)/v6208)}else{v1});
        let v6213=(if v4932{((-v6206)/v6208)}else{v1});
        let v6214=(v4963*v6212);
        let v6216=(v4963*v6213);
        let v6222=(if v4967{self.scalar_static_f64[4154]}else{(if v4932{(v6214+v6214)}else{v1})});
        let v6223=(if v4967{self.scalar_static_f64[4155]}else{(if v4932{(v6216+v6216)}else{v1})});
        let v6224=(v59*v4973);
        let v6227=(if v4967{(v6222/v6224)}else{v6212});
        let v6228=(if v4967{(v6223/v6224)}else{v6213});
        let v6230=(v4974*v4974);
        let v6234=(if v4967{((-v6227)/v6230)}else{v6205});
        let v6235=(if v4967{((-v6228)/v6230)}else{v6206});
        let v6242=(v59*v4985);
        let v6265=(v59*v4999);
        let v6278=(if v4992{(self.scalar_static_f64[525]+(v59*(self.scalar_static_f64[558]*(((v59*v6227)+(((v4997*v6227)+(v4995*(v60*v6227)))/v6265))/v5000))))}else{(if v4980{(v59*(self.scalar_static_f64[558]*((v6234+(((v4983*v6234)+(v4982*v6234))/v6242))/v4986)))}else{v1})});
        let v6279=(if v4992{(self.scalar_static_f64[515]+(v59*(self.scalar_static_f64[558]*(((v59*v6228)+(((v4997*v6228)+(v4995*(v60*v6228)))/v6265))/v5000))))}else{(if v4980{(v59*(self.scalar_static_f64[558]*((v6235+(((v4983*v6235)+(v4982*v6235))/v6242))/v4986)))}else{v1})});
        let v6282=(if self.scalar_static_bool[247]{(-v6278)}else{v1});
        let v6283=(if self.scalar_static_bool[247]{(-v6279)}else{v1});
        let v6288=(v5009*(self.scalar_static_f64[515]-v6282));
        let v6290=(v5009*(self.scalar_static_f64[525]-v6283));
        let v6292=(v59*v5012);
        let v6301=(self.scalar_static_f64[515]*v5017);
        let v6303=(v5017*self.scalar_static_f64[525]);
        let v6305=(v59*v5020);
        let v6314=(self.scalar_static_f64[515]*v4773);
        let v6316=(v4773*self.scalar_static_f64[525]);
        let v6318=(v59*v5026);
        let v6325=(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[515]-((v6314+v6314)/v6318)))}else{v1});
        let v6326=(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[525]-((v6316+v6316)/v6318)))}else{v1});
        let v6333=(-(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[515]+v6282)-((v6288+v6288)/v6292)))}else{v1}));
        let v6334=(-(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[525]+v6283)-((v6290+v6290)/v6292)))}else{v1}));
        let v6335=(if self.scalar_static_bool[250]{v6333}else{v1});
        let v6336=(if self.scalar_static_bool[250]{v6334}else{v1});
        let v6340=(v5037*v5037);
        let v6388=(self.scalar_static_f64[41]*v6335);
        let v6389=(self.scalar_static_f64[41]*v6336);
        let v6390=(v59*v5056);
        let v6397=(self.scalar_static_f64[18]*f64::powf(v5055,self.scalar_static_f64[545]));
        let v6400=(if self.scalar_static_bool[252]{(v6388*v6397)}else{(if self.scalar_static_bool[251]{(v6388/v6390)}else{v1})});
        let v6401=(if self.scalar_static_bool[252]{(v6389*v6397)}else{(if self.scalar_static_bool[251]{(v6389/v6390)}else{v1})});
        let v6404=(if self.scalar_static_bool[250]{(self.scalar_static_f64[28]*v6400)}else{v1});
        let v6405=(if self.scalar_static_bool[250]{(self.scalar_static_f64[28]*v6401)}else{v1});
        let v6438=(if self.scalar_static_bool[253]{(self.scalar_static_f64[658]*(((v5037*(self.scalar_static_f64[19]*v6404))-(v5070*v6335))/v6340))}else{v1});
        let v6439=(if self.scalar_static_bool[253]{(self.scalar_static_f64[658]*(((v5037*(self.scalar_static_f64[19]*v6405))-(v5070*v6336))/v6340))}else{v1});
        let v6442=(v5073*v5073);
        let v6447=(if self.scalar_static_bool[253]{((-(self.scalar_static_f64[886]*v6438))/v6442)}else{v1});
        let v6448=(if self.scalar_static_bool[253]{((-(self.scalar_static_f64[886]*v6439))/v6442)}else{v1});
        let v6449=(v5075*v6447);
        let v6451=(v5075*v6448);
        let v6453=(if self.scalar_static_bool[253]{(v6449+v6449)}else{v1});
        let v6454=(if self.scalar_static_bool[253]{(v6451+v6451)}else{v1});
        let v6455=(v5077*v6453);
        let v6456=(v6455+v6455);
        let v6457=(v5077*v6454);
        let v6458=(v6457+v6457);
        let v6462=(v5079*v5079);
        let v6468=(v59*v5081);
        let v6471=(if self.scalar_static_bool[253]{((((v5079*v6456)-(v5078*v6456))/v6462)/v6468)}else{v1});
        let v6472=(if self.scalar_static_bool[253]{((((v5079*v6458)-(v5078*v6458))/v6462)/v6468)}else{v1});
        let v6473=(v59*v5083);
        let v6476=(if self.scalar_static_bool[253]{(v6471/v6473)}else{v1});
        let v6477=(if self.scalar_static_bool[253]{(v6472/v6473)}else{v1});
        let v6484=(if self.scalar_static_bool[253]{((v5084*v6471)+(v5082*v6476))}else{v1});
        let v6485=(if self.scalar_static_bool[253]{((v5084*v6472)+(v5082*v6477))}else{v1});
        let v6488=((v5086*v6438)+(v5073*v6484));
        let v6491=((v5086*v6439)+(v5073*v6485));
        let v6528=(v5084*v5084);
        let v6536=(v59*v5101);
        let v6539=(if self.scalar_static_bool[253]{((v700*(((v5084*v6438)-(v5073*v6476))/v6528))/v6536)}else{v1});
        let v6540=(if self.scalar_static_bool[253]{((v700*(((v5084*v6439)-(v5073*v6477))/v6528))/v6536)}else{v1});
        let v6551=(if self.scalar_static_bool[253]{((v59*((v5084*v6447)+(v5075*v6476)))-v6471)}else{v1});
        let v6552=(if self.scalar_static_bool[253]{((v59*((v5084*v6448)+(v5075*v6477)))-v6472)}else{v1});
        let v6569=(if self.scalar_static_bool[253]{((((v5107*v6476)+(v5084*(self.scalar_static_f64[651]*v6447)))-(self.scalar_static_f64[651]*v6471))+(v3*v6488))}else{v1});
        let v6570=(if self.scalar_static_bool[253]{((((v5107*v6477)+(v5084*(self.scalar_static_f64[651]*v6448)))-(self.scalar_static_f64[651]*v6472))+(v3*v6491))}else{v1});
        let v6577=(if self.scalar_static_bool[253]{((v5114*v6539)+(v5102*v6551))}else{v1});
        let v6578=(if self.scalar_static_bool[253]{((v5114*v6540)+(v5102*v6552))}else{v1});
        let v6579=(v5116*v6577);
        let v6581=(v5116*v6578);
        let v6583=(if self.scalar_static_bool[253]{(v6579+v6579)}else{v1});
        let v6584=(if self.scalar_static_bool[253]{(v6581+v6581)}else{v1});
        let v6601=(v6569+(-v6583));
        let v6602=(v6570+(-v6584));
        let v6607=(-v6601);
        let v6608=(-v6602);
        let v6627=(v5145*v5145);
        let v6632=(if v5137{((-(v399*((v5143*v6607)+(v5138*(v3*((v5140*v6607)+(v5138*(v402*v6607))))))))/v6627)}else{(if v5133{(v5134*v6601)}else{v6400})});
        let v6633=(if v5137{((-(v399*((v5143*v6608)+(v5138*(v3*((v5140*v6608)+(v5138*(v402*v6608))))))))/v6627)}else{(if v5133{(v5134*v6602)}else{v6401})});
        let v6668=(-v6569);
        let v6669=(-v6570);
        let v6688=(v5171*v5171);
        let v6693=(if v5163{((-(v399*((v5169*v6668)+(v5164*(v3*((v5166*v6668)+(v5164*(v402*v6668))))))))/v6688)}else{(if v5159{(v5160*v6569)}else{v6632})});
        let v6694=(if v5163{((-(v399*((v5169*v6669)+(v5164*(v3*((v5166*v6669)+(v5164*(v402*v6669))))))))/v6688)}else{(if v5159{(v5160*v6570)}else{v6633})});
        let v6732=(-(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[515]-((v6301+v6301)/v6305)))}else{v1}));
        let v6733=(-(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[525]-((v6303+v6303)/v6305)))}else{v1}));
        let v6734=(self.scalar_static_f64[41]*v6732);
        let v6735=(self.scalar_static_f64[41]*v6733);
        let v6736=(v59*v5189);
        let v6742=(self.scalar_static_f64[18]*f64::powf(v5188,self.scalar_static_f64[545]));
        let v6745=(if self.scalar_static_bool[258]{(v6734*v6742)}else{(if self.scalar_static_bool[257]{(v6734/v6736)}else{v6693})});
        let v6746=(if self.scalar_static_bool[258]{(v6735*v6742)}else{(if self.scalar_static_bool[257]{(v6735/v6736)}else{v6694})});
        let v6752=(v5193*v5193);
        let v6760=(if self.scalar_static_bool[256]{(self.scalar_static_f64[24]*(((v5193*(self.scalar_static_f64[37]*v6732))-(v5194*v6745))/v6752))}else{v1});
        let v6761=(if self.scalar_static_bool[256]{(self.scalar_static_f64[24]*(((v5193*(self.scalar_static_f64[37]*v6733))-(v5194*v6746))/v6752))}else{v1});
        let v6764=(v5197*v5197);
        let v6765=((-(self.scalar_static_f64[989]*v6760))/v6764);
        let v6768=((-(self.scalar_static_f64[989]*v6761))/v6764);
        let v6773=(-v6765);
        let v6774=(-v6768);
        let v6793=(v5215*v5215);
        let v6818=(if v5219{(v414*((v5225*v6765)+(v5220*(v3*((v5222*v6765)+(v5220*(v402*v6765)))))))}else{(if v5207{((-(v399*((v5213*v6773)+(v5208*(v3*((v5210*v6773)+(v5208*(v402*v6773))))))))/v6793)}else{(if v5201{(v5202*v6765)}else{v6745})})});
        let v6819=(if v5219{(v414*((v5225*v6768)+(v5220*(v3*((v5222*v6768)+(v5220*(v402*v6768)))))))}else{(if v5207{((-(v399*((v5213*v6774)+(v5208*(v3*((v5210*v6774)+(v5208*(v402*v6774))))))))/v6793)}else{(if v5201{(v5202*v6768)}else{v6746})})});
        let v6842=(self.scalar_static_f64[62]*v6325);
        let v6843=(self.scalar_static_f64[62]*v6326);
        let v6844=(v5241*v6842);
        let v6846=(v5241*v6843);
        let v6862=(if v5246{v1}else{(if v5240{((v5243*v6842)+(v5241*((v5242*v6842)+(v5241*(v6844+v6844)))))}else{v6818})});
        let v6863=(if v5246{v1}else{(if v5240{((v5243*v6843)+(v5241*((v5242*v6843)+(v5241*(v6846+v6846)))))}else{v6819})});
        let v6891=(-(self.scalar_static_f64[624]*v6152));
        let v6892=(-(self.scalar_static_f64[624]*v6153));
        let v6893=(v59*v5268);
        let v6899=(self.scalar_static_f64[19]*f64::powf(v5267,self.scalar_static_f64[534]));
        let v6902=(if self.scalar_static_bool[262]{(v6891*v6899)}else{(if self.scalar_static_bool[261]{(v6891/v6893)}else{v6862})});
        let v6903=(if self.scalar_static_bool[262]{(v6892*v6899)}else{(if self.scalar_static_bool[261]{(v6892/v6893)}else{v6863})});
        let v6908=(self.scalar_static_f64[515]-v6152);
        let v6909=(self.scalar_static_f64[525]-v6153);
        let v6926=(if self.scalar_static_bool[266]{v6333}else{v6335});
        let v6927=(if self.scalar_static_bool[266]{v6334}else{v6336});
        let v6931=(v5290*v5290);
        let v6981=(self.scalar_static_f64[43]*v6926);
        let v6982=(self.scalar_static_f64[43]*v6927);
        let v6983=(v59*v5310);
        let v6990=(self.scalar_static_f64[20]*f64::powf(v5309,self.scalar_static_f64[547]));
        let v6993=(if self.scalar_static_bool[268]{(v6981*v6990)}else{(if self.scalar_static_bool[267]{(v6981/v6983)}else{v6902})});
        let v6994=(if self.scalar_static_bool[268]{(v6982*v6990)}else{(if self.scalar_static_bool[267]{(v6982/v6983)}else{v6903})});
        let v6997=(if self.scalar_static_bool[266]{(self.scalar_static_f64[32]*v6993)}else{v6404});
        let v6998=(if self.scalar_static_bool[266]{(self.scalar_static_f64[32]*v6994)}else{v6405});
        let v7033=(if self.scalar_static_bool[270]{(self.scalar_static_f64[663]*(((v5290*(self.scalar_static_f64[21]*v6997))-(v5325*v6926))/v6931))}else{v6438});
        let v7034=(if self.scalar_static_bool[270]{(self.scalar_static_f64[663]*(((v5290*(self.scalar_static_f64[21]*v6998))-(v5325*v6927))/v6931))}else{v6439});
        let v7037=(v5328*v5328);
        let v7042=(if self.scalar_static_bool[270]{((-(self.scalar_static_f64[1070]*v7033))/v7037)}else{v6447});
        let v7043=(if self.scalar_static_bool[270]{((-(self.scalar_static_f64[1070]*v7034))/v7037)}else{v6448});
        let v7044=(v5330*v7042);
        let v7046=(v5330*v7043);
        let v7048=(if self.scalar_static_bool[270]{(v7044+v7044)}else{v6453});
        let v7049=(if self.scalar_static_bool[270]{(v7046+v7046)}else{v6454});
        let v7050=(v5332*v7048);
        let v7051=(v7050+v7050);
        let v7052=(v5332*v7049);
        let v7053=(v7052+v7052);
        let v7057=(v5334*v5334);
        let v7063=(v59*v5336);
        let v7066=(if self.scalar_static_bool[270]{((((v5334*v7051)-(v5333*v7051))/v7057)/v7063)}else{v6471});
        let v7067=(if self.scalar_static_bool[270]{((((v5334*v7053)-(v5333*v7053))/v7057)/v7063)}else{v6472});
        let v7068=(v59*v5338);
        let v7071=(if self.scalar_static_bool[270]{(v7066/v7068)}else{v6476});
        let v7072=(if self.scalar_static_bool[270]{(v7067/v7068)}else{v6477});
        let v7079=(if self.scalar_static_bool[270]{((v5339*v7066)+(v5337*v7071))}else{v6484});
        let v7080=(if self.scalar_static_bool[270]{((v5339*v7067)+(v5337*v7072))}else{v6485});
        let v7083=((v5341*v7033)+(v5328*v7079));
        let v7086=((v5341*v7034)+(v5328*v7080));
        let v7123=(v5339*v5339);
        let v7131=(v59*v5356);
        let v7134=(if self.scalar_static_bool[270]{((v700*(((v5339*v7033)-(v5328*v7071))/v7123))/v7131)}else{v6539});
        let v7135=(if self.scalar_static_bool[270]{((v700*(((v5339*v7034)-(v5328*v7072))/v7123))/v7131)}else{v6540});
        let v7146=(if self.scalar_static_bool[270]{((v59*((v5339*v7042)+(v5330*v7071)))-v7066)}else{v6551});
        let v7147=(if self.scalar_static_bool[270]{((v59*((v5339*v7043)+(v5330*v7072)))-v7067)}else{v6552});
        let v7164=(if self.scalar_static_bool[270]{((((v5362*v7071)+(v5339*(self.scalar_static_f64[652]*v7042)))-(self.scalar_static_f64[652]*v7066))+(v3*v7083))}else{v6569});
        let v7165=(if self.scalar_static_bool[270]{((((v5362*v7072)+(v5339*(self.scalar_static_f64[652]*v7043)))-(self.scalar_static_f64[652]*v7067))+(v3*v7086))}else{v6570});
        let v7172=(if self.scalar_static_bool[270]{((v5369*v7134)+(v5357*v7146))}else{v6577});
        let v7173=(if self.scalar_static_bool[270]{((v5369*v7135)+(v5357*v7147))}else{v6578});
        let v7174=(v5371*v7172);
        let v7176=(v5371*v7173);
        let v7178=(if self.scalar_static_bool[270]{(v7174+v7174)}else{v6583});
        let v7179=(if self.scalar_static_bool[270]{(v7176+v7176)}else{v6584});
        let v7196=(v7164+(-v7178));
        let v7197=(v7165+(-v7179));
        let v7202=(-v7196);
        let v7203=(-v7197);
        let v7222=(v5400*v5400);
        let v7227=(if v5392{((-(v399*((v5398*v7202)+(v5393*(v3*((v5395*v7202)+(v5393*(v402*v7202))))))))/v7222)}else{(if v5388{(v5389*v7196)}else{v6993})});
        let v7228=(if v5392{((-(v399*((v5398*v7203)+(v5393*(v3*((v5395*v7203)+(v5393*(v402*v7203))))))))/v7222)}else{(if v5388{(v5389*v7197)}else{v6994})});
        let v7263=(-v7164);
        let v7264=(-v7165);
        let v7283=(v5426*v5426);
        let v7288=(if v5418{((-(v399*((v5424*v7263)+(v5419*(v3*((v5421*v7263)+(v5419*(v402*v7263))))))))/v7283)}else{(if v5414{(v5415*v7164)}else{v7227})});
        let v7289=(if v5418{((-(v399*((v5424*v7264)+(v5419*(v3*((v5421*v7264)+(v5419*(v402*v7264))))))))/v7283)}else{(if v5414{(v5415*v7165)}else{v7228})});
        let v7329=(self.scalar_static_f64[43]*v6732);
        let v7330=(self.scalar_static_f64[43]*v6733);
        let v7331=(v59*v5446);
        let v7337=(self.scalar_static_f64[20]*f64::powf(v5445,self.scalar_static_f64[547]));
        let v7340=(if self.scalar_static_bool[276]{(v7329*v7337)}else{(if self.scalar_static_bool[275]{(v7329/v7331)}else{v7288})});
        let v7341=(if self.scalar_static_bool[276]{(v7330*v7337)}else{(if self.scalar_static_bool[275]{(v7330/v7331)}else{v7289})});
        let v7347=(v5450*v5450);
        let v7355=(if self.scalar_static_bool[274]{(self.scalar_static_f64[25]*(((v5450*(self.scalar_static_f64[38]*v6732))-(v5451*v7340))/v7347))}else{v6760});
        let v7356=(if self.scalar_static_bool[274]{(self.scalar_static_f64[25]*(((v5450*(self.scalar_static_f64[38]*v6733))-(v5451*v7341))/v7347))}else{v6761});
        let v7359=(v5454*v5454);
        let v7360=((-(self.scalar_static_f64[1174]*v7355))/v7359);
        let v7363=((-(self.scalar_static_f64[1174]*v7356))/v7359);
        let v7368=(-v7360);
        let v7369=(-v7363);
        let v7388=(v5472*v5472);
        let v7413=(if v5476{(v414*((v5482*v7360)+(v5477*(v3*((v5479*v7360)+(v5477*(v402*v7360)))))))}else{(if v5464{((-(v399*((v5470*v7368)+(v5465*(v3*((v5467*v7368)+(v5465*(v402*v7368))))))))/v7388)}else{(if v5458{(v5459*v7360)}else{v7340})})});
        let v7414=(if v5476{(v414*((v5482*v7363)+(v5477*(v3*((v5479*v7363)+(v5477*(v402*v7363)))))))}else{(if v5464{((-(v399*((v5470*v7369)+(v5465*(v3*((v5467*v7369)+(v5465*(v402*v7369))))))))/v7388)}else{(if v5458{(v5459*v7363)}else{v7341})})});
        let v7439=(self.scalar_static_f64[64]*v6325);
        let v7440=(self.scalar_static_f64[64]*v6326);
        let v7441=(v5498*v7439);
        let v7443=(v5498*v7440);
        let v7459=(if v5503{v1}else{(if v5497{((v5500*v7439)+(v5498*((v5499*v7439)+(v5498*(v7441+v7441)))))}else{v7413})});
        let v7460=(if v5503{v1}else{(if v5497{((v5500*v7440)+(v5498*((v5499*v7440)+(v5498*(v7443+v7443)))))}else{v7414})});
        let v7488=(-(self.scalar_static_f64[625]*v6152));
        let v7489=(-(self.scalar_static_f64[625]*v6153));
        let v7490=(v59*v5525);
        let v7496=(self.scalar_static_f64[21]*f64::powf(v5524,self.scalar_static_f64[535]));
        let v7499=(if self.scalar_static_bool[280]{(v7488*v7496)}else{(if self.scalar_static_bool[279]{(v7488/v7490)}else{v7459})});
        let v7500=(if self.scalar_static_bool[280]{(v7489*v7496)}else{(if self.scalar_static_bool[279]{(v7489/v7490)}else{v7460})});
        let v7521=(if self.scalar_static_bool[284]{v6333}else{v6926});
        let v7522=(if self.scalar_static_bool[284]{v6334}else{v6927});
        let v7526=(v5545*v5545);
        let v7576=(self.scalar_static_f64[45]*v7521);
        let v7577=(self.scalar_static_f64[45]*v7522);
        let v7578=(v59*v5565);
        let v7585=(self.scalar_static_f64[22]*f64::powf(v5564,self.scalar_static_f64[549]));
        let v7588=(if self.scalar_static_bool[286]{(v7576*v7585)}else{(if self.scalar_static_bool[285]{(v7576/v7578)}else{v7499})});
        let v7589=(if self.scalar_static_bool[286]{(v7577*v7585)}else{(if self.scalar_static_bool[285]{(v7577/v7578)}else{v7500})});
        let v7592=(if self.scalar_static_bool[284]{(self.scalar_static_f64[36]*v7588)}else{v6997});
        let v7593=(if self.scalar_static_bool[284]{(self.scalar_static_f64[36]*v7589)}else{v6998});
        let v7628=(if self.scalar_static_bool[288]{(self.scalar_static_f64[668]*(((v5545*(self.scalar_static_f64[23]*v7592))-(v5580*v7521))/v7526))}else{v7033});
        let v7629=(if self.scalar_static_bool[288]{(self.scalar_static_f64[668]*(((v5545*(self.scalar_static_f64[23]*v7593))-(v5580*v7522))/v7526))}else{v7034});
        let v7632=(v5583*v5583);
        let v7637=(if self.scalar_static_bool[288]{((-(self.scalar_static_f64[1256]*v7628))/v7632)}else{v7042});
        let v7638=(if self.scalar_static_bool[288]{((-(self.scalar_static_f64[1256]*v7629))/v7632)}else{v7043});
        let v7639=(v5585*v7637);
        let v7641=(v5585*v7638);
        let v7645=(v5587*(if self.scalar_static_bool[288]{(v7639+v7639)}else{v7048}));
        let v7646=(v7645+v7645);
        let v7647=(v5587*(if self.scalar_static_bool[288]{(v7641+v7641)}else{v7049}));
        let v7648=(v7647+v7647);
        let v7652=(v5589*v5589);
        let v7658=(v59*v5591);
        let v7661=(if self.scalar_static_bool[288]{((((v5589*v7646)-(v5588*v7646))/v7652)/v7658)}else{v7066});
        let v7662=(if self.scalar_static_bool[288]{((((v5589*v7648)-(v5588*v7648))/v7652)/v7658)}else{v7067});
        let v7663=(v59*v5593);
        let v7666=(if self.scalar_static_bool[288]{(v7661/v7663)}else{v7071});
        let v7667=(if self.scalar_static_bool[288]{(v7662/v7663)}else{v7072});
        let v7678=((v5596*v7628)+(v5583*(if self.scalar_static_bool[288]{((v5594*v7661)+(v5592*v7666))}else{v7079})));
        let v7681=((v5596*v7629)+(v5583*(if self.scalar_static_bool[288]{((v5594*v7662)+(v5592*v7667))}else{v7080})));
        let v7718=(v5594*v5594);
        let v7726=(v59*v5611);
        let v7729=(if self.scalar_static_bool[288]{((v700*(((v5594*v7628)-(v5583*v7666))/v7718))/v7726)}else{v7134});
        let v7730=(if self.scalar_static_bool[288]{((v700*(((v5594*v7629)-(v5583*v7667))/v7718))/v7726)}else{v7135});
        let v7759=(if self.scalar_static_bool[288]{((((v5617*v7666)+(v5594*(self.scalar_static_f64[653]*v7637)))-(self.scalar_static_f64[653]*v7661))+(v3*v7678))}else{v7164});
        let v7760=(if self.scalar_static_bool[288]{((((v5617*v7667)+(v5594*(self.scalar_static_f64[653]*v7638)))-(self.scalar_static_f64[653]*v7662))+(v3*v7681))}else{v7165});
        let v7767=(if self.scalar_static_bool[288]{((v5624*v7729)+(v5612*(if self.scalar_static_bool[288]{((v59*((v5594*v7637)+(v5585*v7666)))-v7661)}else{v7146})))}else{v7172});
        let v7768=(if self.scalar_static_bool[288]{((v5624*v7730)+(v5612*(if self.scalar_static_bool[288]{((v59*((v5594*v7638)+(v5585*v7667)))-v7662)}else{v7147})))}else{v7173});
        let v7769=(v5626*v7767);
        let v7771=(v5626*v7768);
        let v7791=(v7759+(-(if self.scalar_static_bool[288]{(v7769+v7769)}else{v7178})));
        let v7792=(v7760+(-(if self.scalar_static_bool[288]{(v7771+v7771)}else{v7179})));
        let v7797=(-v7791);
        let v7798=(-v7792);
        let v7817=(v5655*v5655);
        let v7822=(if v5647{((-(v399*((v5653*v7797)+(v5648*(v3*((v5650*v7797)+(v5648*(v402*v7797))))))))/v7817)}else{(if v5643{(v5644*v7791)}else{v7588})});
        let v7823=(if v5647{((-(v399*((v5653*v7798)+(v5648*(v3*((v5650*v7798)+(v5648*(v402*v7798))))))))/v7817)}else{(if v5643{(v5644*v7792)}else{v7589})});
        let v7858=(-v7759);
        let v7859=(-v7760);
        let v7878=(v5681*v5681);
        let v7883=(if v5673{((-(v399*((v5679*v7858)+(v5674*(v3*((v5676*v7858)+(v5674*(v402*v7858))))))))/v7878)}else{(if v5669{(v5670*v7759)}else{v7822})});
        let v7884=(if v5673{((-(v399*((v5679*v7859)+(v5674*(v3*((v5676*v7859)+(v5674*(v402*v7859))))))))/v7878)}else{(if v5669{(v5670*v7760)}else{v7823})});
        let v7924=(self.scalar_static_f64[45]*v6732);
        let v7925=(self.scalar_static_f64[45]*v6733);
        let v7926=(v59*v5701);
        let v7932=(self.scalar_static_f64[22]*f64::powf(v5700,self.scalar_static_f64[549]));
        let v7935=(if self.scalar_static_bool[294]{(v7924*v7932)}else{(if self.scalar_static_bool[293]{(v7924/v7926)}else{v7883})});
        let v7936=(if self.scalar_static_bool[294]{(v7925*v7932)}else{(if self.scalar_static_bool[293]{(v7925/v7926)}else{v7884})});
        let v7942=(v5705*v5705);
        let v7950=(if self.scalar_static_bool[292]{(self.scalar_static_f64[26]*(((v5705*(self.scalar_static_f64[39]*v6732))-(v5706*v7935))/v7942))}else{v7355});
        let v7951=(if self.scalar_static_bool[292]{(self.scalar_static_f64[26]*(((v5705*(self.scalar_static_f64[39]*v6733))-(v5706*v7936))/v7942))}else{v7356});
        let v7954=(v5709*v5709);
        let v7955=((-(self.scalar_static_f64[1360]*v7950))/v7954);
        let v7958=((-(self.scalar_static_f64[1360]*v7951))/v7954);
        let v7963=(-v7955);
        let v7964=(-v7958);
        let v7983=(v5727*v5727);
        let v8008=(if v5731{(v414*((v5737*v7955)+(v5732*(v3*((v5734*v7955)+(v5732*(v402*v7955)))))))}else{(if v5719{((-(v399*((v5725*v7963)+(v5720*(v3*((v5722*v7963)+(v5720*(v402*v7963))))))))/v7983)}else{(if v5713{(v5714*v7955)}else{v7935})})});
        let v8009=(if v5731{(v414*((v5737*v7958)+(v5732*(v3*((v5734*v7958)+(v5732*(v402*v7958)))))))}else{(if v5719{((-(v399*((v5725*v7964)+(v5720*(v3*((v5722*v7964)+(v5720*(v402*v7964))))))))/v7983)}else{(if v5713{(v5714*v7958)}else{v7936})})});
        let v8034=(self.scalar_static_f64[66]*v6325);
        let v8035=(self.scalar_static_f64[66]*v6326);
        let v8036=(v5753*v8034);
        let v8038=(v5753*v8035);
        let v8054=(if v5758{v1}else{(if v5752{((v5755*v8034)+(v5753*((v5754*v8034)+(v5753*(v8036+v8036)))))}else{v8008})});
        let v8055=(if v5758{v1}else{(if v5752{((v5755*v8035)+(v5753*((v5754*v8035)+(v5753*(v8038+v8038)))))}else{v8009})});
        let v8103=(if self.scalar_static_bool[297]{(if v5779{(if v5785{v1}else{(self.scalar_static_f64[519]*((v5786*self.scalar_static_f64[551])/v5787))})}else{(if v5792{self.scalar_static_f64[515]}else{(self.scalar_static_f64[515]+(self.scalar_static_f64[519]*((v5795*self.scalar_static_f64[552])/v5796)))})})}else{v1});
        let v8104=(if self.scalar_static_bool[297]{(if v5779{(if v5785{v1}else{(self.scalar_static_f64[519]*((v5786*self.scalar_static_f64[552])/v5787))})}else{(if v5792{self.scalar_static_f64[525]}else{(self.scalar_static_f64[525]+(self.scalar_static_f64[519]*((v5795*self.scalar_static_f64[551])/v5796)))})})}else{v1});
        let v8105=(if self.scalar_static_bool[297]{v8103}else{self.scalar_static_f64[537]});
        let v8106=(if self.scalar_static_bool[297]{v8104}else{self.scalar_static_f64[538]});
        let v8107=(if self.scalar_static_bool[297]{v8105}else{self.scalar_static_f64[539]});
        let v8108=(if self.scalar_static_bool[297]{v8106}else{self.scalar_static_f64[540]});
        let v8111=(if self.scalar_static_bool[297]{(-v8105)}else{self.scalar_static_f64[543]});
        let v8112=(if self.scalar_static_bool[297]{(-v8106)}else{self.scalar_static_f64[544]});
        let v8113=(v5811*v8111);
        let v8115=(v5811*v8112);
        let v8117=(v59*v5814);
        let v8120=(if self.scalar_static_bool[297]{((v8113+v8113)/v8117)}else{v6137});
        let v8121=(if self.scalar_static_bool[297]{((v8115+v8115)/v8117)}else{v6138});
        let v8129=(v5817*v5817);
        let v8137=(if self.scalar_static_bool[297]{(v59*(((v5817*(self.scalar_static_f64[770]*v8103))-(v5816*(v8107+v8120)))/v8129))}else{v1});
        let v8138=(if self.scalar_static_bool[297]{(v59*(((v5817*(self.scalar_static_f64[770]*v8104))-(v5816*(v8108+v8121)))/v8129))}else{v1});
        let v8141=(-(self.scalar_static_f64[626]*v8137));
        let v8142=(-(self.scalar_static_f64[626]*v8138));
        let v8143=(v59*v5824);
        let v8149=(self.scalar_static_f64[23]*f64::powf(v5823,self.scalar_static_f64[536]));
        let v8152=(if self.scalar_static_bool[299]{(v8141*v8149)}else{(if self.scalar_static_bool[298]{(v8141/v8143)}else{v8054})});
        let v8153=(if self.scalar_static_bool[299]{(v8142*v8149)}else{(if self.scalar_static_bool[298]{(v8142/v8143)}else{v8055})});
        let v8166=(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(-v8152))+(self.scalar_static_f64[644]*(v8103-v8137))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[641]*(-(if self.scalar_static_bool[751]{(v6098*v6107)}else{(if self.scalar_static_bool[750]{(v6098/v6100)}else{v6084})})))+(self.scalar_static_f64[644]*v6062))}else{v1})})});
        let v8167=(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(-v8153))+(self.scalar_static_f64[644]*(v8104-v8138))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[641]*(-(if self.scalar_static_bool[751]{(v6099*v6107)}else{(if self.scalar_static_bool[750]{(v6099/v6100)}else{v6085})})))+(self.scalar_static_f64[644]*v6063))}else{v1})})});
        let v8170=(if self.scalar_static_bool[297]{(self.scalar_static_f64[515]-v8103)}else{v8103});
        let v8171=(if self.scalar_static_bool[297]{(self.scalar_static_f64[525]-v8104)}else{v8104});
        let v8172=(if self.scalar_static_bool[297]{v8170}else{v8105});
        let v8173=(if self.scalar_static_bool[297]{v8171}else{v8106});
        let v8180=(v5847*(if self.scalar_static_bool[297]{(-v8172)}else{v8111}));
        let v8182=(v5847*(if self.scalar_static_bool[297]{(-v8173)}else{v8112}));
        let v8184=(v59*v5850);
        let v8196=(v5853*v5853);
        let v8204=(if self.scalar_static_bool[297]{(v59*(((v5853*(self.scalar_static_f64[770]*v8170))-(v5852*((if self.scalar_static_bool[297]{v8172}else{v8107})+(if self.scalar_static_bool[297]{((v8180+v8180)/v8184)}else{v8120}))))/v8196))}else{v8137});
        let v8205=(if self.scalar_static_bool[297]{(v59*(((v5853*(self.scalar_static_f64[770]*v8171))-(v5852*((if self.scalar_static_bool[297]{v8173}else{v8108})+(if self.scalar_static_bool[297]{((v8182+v8182)/v8184)}else{v8121}))))/v8196))}else{v8138});
        let v8208=(-(self.scalar_static_f64[704]*v8204));
        let v8209=(-(self.scalar_static_f64[704]*v8205));
        let v8210=(v59*v5861);
        let v8217=(self.scalar_static_f64[109]*f64::powf(v5860,self.scalar_static_f64[553]));
        let v8220=(if self.scalar_static_bool[303]{(v8208*v8217)}else{(if self.scalar_static_bool[301]{(v8208/v8210)}else{v8152})});
        let v8221=(if self.scalar_static_bool[303]{(v8209*v8217)}else{(if self.scalar_static_bool[301]{(v8209/v8210)}else{v8153})});
        let v8242=(-(self.scalar_static_f64[626]*v6152));
        let v8243=(-(self.scalar_static_f64[626]*v6153));
        let v8244=(v59*v5881);
        let v8250=(self.scalar_static_f64[23]*f64::powf(v5880,self.scalar_static_f64[536]));
        let v8291=(self.scalar_static_f64[522]*(((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[517]*((self.scalar_static_f64[637]*(-v6902))+(self.scalar_static_f64[642]*v6908)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[637]*(-v6056))+(self.scalar_static_f64[642]*v6062))}else{v1})})}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[517]*((self.scalar_static_f64[639]*(-v7499))+(self.scalar_static_f64[643]*v6908)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[639]*(-v6084))+(self.scalar_static_f64[643]*v6062))}else{v1})})})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(-(if self.scalar_static_bool[307]{(v8242*v8250)}else{(if self.scalar_static_bool[306]{(v8242/v8244)}else{v8220})})))+(self.scalar_static_f64[644]*v6908)))}else{(if self.scalar_static_bool[297]{(v8166+(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[711]*(-v8220))+(self.scalar_static_f64[713]*(v8170-v8204))))}else{v1}))}else{v8166})}))));
        let v8292=(self.scalar_static_f64[522]*(((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[517]*((self.scalar_static_f64[637]*(-v6903))+(self.scalar_static_f64[642]*v6909)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[637]*(-v6057))+(self.scalar_static_f64[642]*v6063))}else{v1})})}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[517]*((self.scalar_static_f64[639]*(-v7500))+(self.scalar_static_f64[643]*v6909)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[639]*(-v6085))+(self.scalar_static_f64[643]*v6063))}else{v1})})})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[517]*((self.scalar_static_f64[641]*(-(if self.scalar_static_bool[307]{(v8243*v8250)}else{(if self.scalar_static_bool[306]{(v8243/v8244)}else{v8221})})))+(self.scalar_static_f64[644]*v6909)))}else{(if self.scalar_static_bool[297]{(v8167+(if self.scalar_static_bool[297]{(self.scalar_static_f64[517]*((self.scalar_static_f64[711]*(-v8221))+(self.scalar_static_f64[713]*(v8171-v8205))))}else{v1}))}else{v8167})}))));

        CommonStampValues {
            v1,
            v5,
            v59,
            v399,
            v400,
            v4773,
            v4774,
            v4816,
            v4972,
            v4974,
            v5005,
            v5029,
            v5037,
            v5061,
            v5088,
            v5102,
            v5116,
            v5119,
            v5126,
            v5147,
            v5173,
            v5197,
            v5229,
            v5237,
            v5239,
            v5249,
            v5290,
            v5315,
            v5343,
            v5357,
            v5371,
            v5374,
            v5381,
            v5402,
            v5428,
            v5454,
            v5486,
            v5494,
            v5496,
            v5506,
            v5545,
            v5570,
            v5598,
            v5612,
            v5626,
            v5629,
            v5636,
            v5657,
            v5683,
            v5709,
            v5741,
            v5749,
            v5751,
            v5761,
            v5909,
            v6222,
            v6223,
            v6227,
            v6228,
            v6278,
            v6279,
            v6325,
            v6326,
            v6335,
            v6336,
            v6340,
            v6404,
            v6405,
            v6488,
            v6491,
            v6539,
            v6540,
            v6577,
            v6578,
            v6632,
            v6633,
            v6693,
            v6694,
            v6760,
            v6761,
            v6818,
            v6819,
            v6862,
            v6863,
            v6926,
            v6927,
            v6931,
            v6997,
            v6998,
            v7083,
            v7086,
            v7134,
            v7135,
            v7172,
            v7173,
            v7227,
            v7228,
            v7288,
            v7289,
            v7355,
            v7356,
            v7413,
            v7414,
            v7459,
            v7460,
            v7521,
            v7522,
            v7526,
            v7592,
            v7593,
            v7678,
            v7681,
            v7729,
            v7730,
            v7767,
            v7768,
            v7822,
            v7823,
            v7883,
            v7884,
            v7950,
            v7951,
            v8008,
            v8009,
            v8054,
            v8055,
            v8291,
            v8292,
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
        let v57=0.29214664;
        let v58=0.5178164370971076;
        let v61=0.26992878119627894;
        let v62=0.43792457880372104;
        let v779=0.886226925452758;
        let v4775=(if self.scalar_static_bool[29]{common.v4774}else{common.v1});
        let v4776=(v4775<common.v400);
        let v4778=(common.v5+(common.v400-v4775));
        let v4780=(v4775>self.scalar_static_f64[4082]);
        let v4784=(v4775).exp();
        let v4787=(if self.scalar_static_bool[29]{(if v4776{(common.v399/v4778)}else{(if v4780{(self.scalar_static_f64[4084]*(common.v5+(v4775-self.scalar_static_f64[4082])))}else{v4784})})}else{common.v1});
        let v4792=(if self.scalar_static_bool[29]{(self.scalar_static_f64[3973]*common.v4774)}else{v4775});
        let v4793=(v4792<common.v400);
        let v4795=(common.v5+(common.v400-v4792));
        let v4797=(v4792>self.scalar_static_f64[4086]);
        let v4801=(v4792).exp();
        let v4804=(if self.scalar_static_bool[29]{(if v4793{(common.v399/v4795)}else{(if v4797{(self.scalar_static_f64[4088]*(common.v5+(v4792-self.scalar_static_f64[4086])))}else{v4801})})}else{v4787});
        let v4811=(self.scalar_static_f64[4057]+(self.scalar_static_f64[4049]*common.v4773));
        let v4819=(if self.scalar_static_bool[739]{(self.scalar_static_f64[4049]*(self.scalar_static_f64[559]*common.v4816))}else{v4792});
        let v4820=(v4819<common.v400);
        let v4822=(common.v5+(common.v400-v4819));
        let v4824=(v4819>self.scalar_static_f64[4090]);
        let v4828=(v4819).exp();
        let v4978=(if self.scalar_static_bool[247]{(common.v4972-common.v5)}else{common.v4972});
        let v5034=(if self.scalar_static_bool[249]{(self.scalar_static_f64[585]*v4978)}else{common.v1});
        let v5040=((common.v5-(common.v5005/common.v5037))).sqrt();
        let v5042=(if self.scalar_static_bool[250]{(common.v5-v5040)}else{common.v1});
        let v5045=(v5042*v5042);
        let v5046=(v5042).ln();
        let v5047=(v5045*v5046);
        let v5048=(common.v5-v5042);
        let v5052=(if self.scalar_static_bool[252]{(self.scalar_static_f64[206]*(v5042+(v5047/v5048)))}else{common.v1});
        let v5054=(if self.scalar_static_bool[250]{(v5042+v5052)}else{common.v1});
        let v5062=(common.v4974-common.v5);
        let v5065=(if self.scalar_static_bool[250]{(self.scalar_static_f64[573]*(common.v5061*v5062))}else{common.v1});
        let v5068=(if self.scalar_static_bool[250]{(self.scalar_static_f64[203]*(v5054*v5065))}else{common.v1});
        let v5089=(common.v5+common.v5088);
        let v5094=(if self.scalar_static_bool[255]{f64::powf(v5089,self.scalar_static_f64[208])}else{(if self.scalar_static_bool[254]{(common.v5/v5089)}else{common.v1})});
        let v5095=(v5054*v5094);
        let v5096=(v5054+v5094);
        let v5098=(if self.scalar_static_bool[253]{(v5095/v5096)}else{common.v1});
        let v5120=(self.scalar_static_bool[253]&&common.v5119);
        let v5121=(v58*common.v5116);
        let v5122=(common.v5+v5121);
        let v5127=(common.v5-v5121);
        let v5129=(if common.v5126{(common.v5/v5127)}else{(if v5120{(common.v5/v5122)}else{common.v1})});
        let v5149=(v5129*v5129);
        let v5154=(((v57*v5129)+(v61*v5149))+(v62*(v5129*v5149)));
        let v5156=(if self.scalar_static_bool[253]{(common.v5147*v5154)}else{common.v1});
        let v5176=(if common.v5126{((common.v59*common.v5173)-v5156)}else{(if v5120{v5156}else{common.v1})});
        let v5177=(self.scalar_static_f64[651]*v5176);
        let v5180=(if self.scalar_static_bool[253]{(v779*(v5177/common.v5102))}else{common.v1});
        let v5181=(v5065*v5180);
        let v5184=(if self.scalar_static_bool[253]{(self.scalar_static_f64[204]*(v5098*v5181))}else{common.v1});
        let v5230=(common.v4773*common.v5197);
        let v5231=(common.v5197*v5230);
        let v5234=(if self.scalar_static_bool[256]{(self.scalar_static_f64[209]*(common.v5229*v5231))}else{common.v1});
        let v5250=(common.v5-common.v5249);
        let v5254=(self.scalar_static_bool[260]&&(!common.v5237));
        let v5258=(if v5254{(self.scalar_static_f64[52]+(self.scalar_static_f64[73]*(self.scalar_static_f64[224]+common.v5029)))}else{(if common.v5239{(common.v5/v5250)}else{self.scalar_static_f64[516]})});
        let v5262=(self.scalar_static_f64[228]*(v5234+(v5184+(v5034+v5068))));
        let v5285=(if self.scalar_static_bool[264]{(self.scalar_static_f64[587]*v4978)}else{v5034});
        let v5293=((common.v5-(common.v5005/common.v5290))).sqrt();
        let v5295=(if self.scalar_static_bool[266]{(common.v5-v5293)}else{v5042});
        let v5299=(v5295*v5295);
        let v5300=(v5295).ln();
        let v5301=(v5299*v5300);
        let v5302=(common.v5-v5295);
        let v5306=(if self.scalar_static_bool[268]{(self.scalar_static_f64[232]*(v5295+(v5301/v5302)))}else{(if self.scalar_static_bool[267]{common.v1}else{v5052})});
        let v5308=(if self.scalar_static_bool[266]{(v5295+v5306)}else{v5054});
        let v5318=(if self.scalar_static_bool[266]{(self.scalar_static_f64[578]*(v5062*common.v5315))}else{v5065});
        let v5321=(if self.scalar_static_bool[266]{(self.scalar_static_f64[229]*(v5308*v5318))}else{(if self.scalar_static_bool[265]{common.v1}else{v5068})});
        let v5344=(common.v5+common.v5343);
        let v5349=(if self.scalar_static_bool[272]{f64::powf(v5344,self.scalar_static_f64[234])}else{(if self.scalar_static_bool[271]{(common.v5/v5344)}else{v5094})});
        let v5350=(v5308*v5349);
        let v5351=(v5308+v5349);
        let v5353=(if self.scalar_static_bool[270]{(v5350/v5351)}else{v5098});
        let v5375=(self.scalar_static_bool[270]&&common.v5374);
        let v5376=(v58*common.v5371);
        let v5377=(common.v5+v5376);
        let v5382=(common.v5-v5376);
        let v5384=(if common.v5381{(common.v5/v5382)}else{(if v5375{(common.v5/v5377)}else{v5129})});
        let v5404=(v5384*v5384);
        let v5409=(((v57*v5384)+(v61*v5404))+(v62*(v5384*v5404)));
        let v5411=(if self.scalar_static_bool[270]{(common.v5402*v5409)}else{v5156});
        let v5431=(if common.v5381{((common.v59*common.v5428)-v5411)}else{(if v5375{v5411}else{v5176})});
        let v5432=(self.scalar_static_f64[652]*v5431);
        let v5435=(if self.scalar_static_bool[270]{(v779*(v5432/common.v5357))}else{v5180});
        let v5436=(v5318*v5435);
        let v5439=(if self.scalar_static_bool[270]{(self.scalar_static_f64[230]*(v5353*v5436))}else{(if self.scalar_static_bool[269]{common.v1}else{v5184})});
        let v5487=(common.v4773*common.v5454);
        let v5488=(common.v5454*v5487);
        let v5491=(if self.scalar_static_bool[274]{(self.scalar_static_f64[235]*(common.v5486*v5488))}else{(if self.scalar_static_bool[273]{common.v1}else{v5234})});
        let v5507=(common.v5-common.v5506);
        let v5511=(self.scalar_static_bool[278]&&(!common.v5494));
        let v5515=(if v5511{(self.scalar_static_f64[56]+(self.scalar_static_f64[80]*(self.scalar_static_f64[248]+common.v5029)))}else{(if common.v5496{(common.v5/v5507)}else{(if self.scalar_static_bool[277]{common.v5}else{v5258})})});
        let v5519=(self.scalar_static_f64[228]*(v5491+(v5439+(v5285+v5321))));
        let v5548=((common.v5-(common.v5005/common.v5545))).sqrt();
        let v5550=(if self.scalar_static_bool[284]{(common.v5-v5548)}else{v5295});
        let v5554=(v5550*v5550);
        let v5555=(v5550).ln();
        let v5556=(v5554*v5555);
        let v5557=(common.v5-v5550);
        let v5563=(if self.scalar_static_bool[284]{(v5550+(if self.scalar_static_bool[286]{(self.scalar_static_f64[255]*(v5550+(v5556/v5557)))}else{(if self.scalar_static_bool[285]{common.v1}else{v5306})}))}else{v5308});
        let v5573=(if self.scalar_static_bool[284]{(self.scalar_static_f64[583]*(v5062*common.v5570))}else{v5318});
        let v5599=(common.v5+common.v5598);
        let v5604=(if self.scalar_static_bool[290]{f64::powf(v5599,self.scalar_static_f64[257])}else{(if self.scalar_static_bool[289]{(common.v5/v5599)}else{v5349})});
        let v5605=(v5563*v5604);
        let v5606=(v5563+v5604);
        let v5608=(if self.scalar_static_bool[288]{(v5605/v5606)}else{v5353});
        let v5630=(self.scalar_static_bool[288]&&common.v5629);
        let v5631=(v58*common.v5626);
        let v5632=(common.v5+v5631);
        let v5637=(common.v5-v5631);
        let v5639=(if common.v5636{(common.v5/v5637)}else{(if v5630{(common.v5/v5632)}else{v5384})});
        let v5659=(v5639*v5639);
        let v5664=(((v57*v5639)+(v61*v5659))+(v62*(v5639*v5659)));
        let v5666=(if self.scalar_static_bool[288]{(common.v5657*v5664)}else{v5411});
        let v5687=(self.scalar_static_f64[653]*(if common.v5636{((common.v59*common.v5683)-v5666)}else{(if v5630{v5666}else{v5431})}));
        let v5690=(if self.scalar_static_bool[288]{(v779*(v5687/common.v5612))}else{v5435});
        let v5691=(v5573*v5690);
        let v5742=(common.v4773*common.v5709);
        let v5743=(common.v5709*v5742);
        let v5762=(common.v5-common.v5761);
        let v5766=(self.scalar_static_bool[296]&&(!common.v5749));
        let v5770=(if v5766{(self.scalar_static_f64[60]+(self.scalar_static_f64[87]*(self.scalar_static_f64[271]+common.v5029)))}else{(if common.v5751{(common.v5/v5762)}else{(if self.scalar_static_bool[295]{common.v5}else{v5515})})});
        let v5774=(self.scalar_static_f64[228]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[258]*(common.v5741*v5743))}else{(if self.scalar_static_bool[291]{common.v1}else{v5491})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[253]*(v5608*v5691))}else{(if self.scalar_static_bool[287]{common.v1}else{v5439})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[589]*v4978)}else{v5285})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[252]*(v5563*v5573))}else{(if self.scalar_static_bool[283]{common.v1}else{v5321})})))));
        let v5919=(v4778*v4778);
        let v5932=(if self.scalar_static_bool[29]{(if v4776{(self.scalar_static_f64[4113]/v5919)}else{(if v4780{self.scalar_static_f64[4116]}else{(v4784*self.scalar_static_f64[4108])})})}else{common.v1});
        let v5933=(if self.scalar_static_bool[29]{(if v4776{(self.scalar_static_f64[4115]/v5919)}else{(if v4780{self.scalar_static_f64[4117]}else{(v4784*self.scalar_static_f64[4109])})})}else{common.v1});
        let v5946=(v4795*v4795);
        let v5959=(if self.scalar_static_bool[29]{(if v4793{(self.scalar_static_f64[4125]/v5946)}else{(if v4797{self.scalar_static_f64[4128]}else{(v4801*self.scalar_static_f64[4120])})})}else{v5932});
        let v5960=(if self.scalar_static_bool[29]{(if v4793{(self.scalar_static_f64[4127]/v5946)}else{(if v4797{self.scalar_static_f64[4129]}else{(v4801*self.scalar_static_f64[4121])})})}else{v5933});
        let v5983=(v4822*v4822);
        let v6331=(if self.scalar_static_bool[249]{(self.scalar_static_f64[585]*common.v6222)}else{common.v1});
        let v6332=(if self.scalar_static_bool[249]{(self.scalar_static_f64[585]*common.v6223)}else{common.v1});
        let v6348=(common.v59*v5040);
        let v6353=(if self.scalar_static_bool[250]{(-((-(((common.v5037*common.v6278)-(common.v5005*common.v6335))/common.v6340))/v6348))}else{common.v1});
        let v6354=(if self.scalar_static_bool[250]{(-((-(((common.v5037*common.v6279)-(common.v5005*common.v6336))/common.v6340))/v6348))}else{common.v1});
        let v6355=(v5042*v6353);
        let v6357=(v5042*v6354);
        let v6372=(v5048*v5048);
        let v6382=(if self.scalar_static_bool[252]{(self.scalar_static_f64[206]*(v6353+(((v5048*((v5046*(v6355+v6355))+(v5045*(v6353/v5042))))-(v5047*(-v6353)))/v6372)))}else{common.v1});
        let v6383=(if self.scalar_static_bool[252]{(self.scalar_static_f64[206]*(v6354+(((v5048*((v5046*(v6357+v6357))+(v5045*(v6354/v5042))))-(v5047*(-v6354)))/v6372)))}else{common.v1});
        let v6386=(if self.scalar_static_bool[250]{(v6353+v6382)}else{common.v1});
        let v6387=(if self.scalar_static_bool[250]{(v6354+v6383)}else{common.v1});
        let v6414=(if self.scalar_static_bool[250]{(self.scalar_static_f64[573]*((v5062*common.v6404)+(common.v5061*common.v6227)))}else{common.v1});
        let v6415=(if self.scalar_static_bool[250]{(self.scalar_static_f64[573]*((v5062*common.v6405)+(common.v5061*common.v6228)))}else{common.v1});
        let v6424=(if self.scalar_static_bool[250]{(self.scalar_static_f64[203]*((v5065*v6386)+(v5054*v6414)))}else{common.v1});
        let v6425=(if self.scalar_static_bool[250]{(self.scalar_static_f64[203]*((v5065*v6387)+(v5054*v6415)))}else{common.v1});
        let v6493=(v5089*v5089);
        let v6501=(self.scalar_static_f64[208]*f64::powf(v5089,self.scalar_static_f64[546]));
        let v6504=(if self.scalar_static_bool[255]{(common.v6488*v6501)}else{(if self.scalar_static_bool[254]{((-common.v6488)/v6493)}else{common.v1})});
        let v6505=(if self.scalar_static_bool[255]{(common.v6491*v6501)}else{(if self.scalar_static_bool[254]{((-common.v6491)/v6493)}else{common.v1})});
        let v6517=(v5096*v5096);
        let v6523=(if self.scalar_static_bool[253]{(((v5096*((v5094*v6386)+(v5054*v6504)))-(v5095*(v6386+v6504)))/v6517)}else{common.v1});
        let v6524=(if self.scalar_static_bool[253]{(((v5096*((v5094*v6387)+(v5054*v6505)))-(v5095*(v6387+v6505)))/v6517)}else{common.v1});
        let v6585=(v58*common.v6577);
        let v6586=(v58*common.v6578);
        let v6588=(v5122*v5122);
        let v6594=(v5127*v5127);
        let v6597=(if common.v5126{(v6585/v6594)}else{(if v5120{((-v6585)/v6588)}else{common.v1})});
        let v6598=(if common.v5126{(v6586/v6594)}else{(if v5120{((-v6586)/v6588)}else{common.v1})});
        let v6636=(v5129*v6597);
        let v6637=(v6636+v6636);
        let v6638=(v5129*v6598);
        let v6639=(v6638+v6638);
        let v6660=(if self.scalar_static_bool[253]{((v5154*common.v6632)+(common.v5147*(((v57*v6597)+(v61*v6637))+(v62*((v5149*v6597)+(v5129*v6637))))))}else{common.v1});
        let v6661=(if self.scalar_static_bool[253]{((v5154*common.v6633)+(common.v5147*(((v57*v6598)+(v61*v6639))+(v62*((v5149*v6598)+(v5129*v6639))))))}else{common.v1});
        let v6699=(if common.v5126{((common.v59*common.v6693)-v6660)}else{(if v5120{v6660}else{common.v1})});
        let v6700=(if common.v5126{((common.v59*common.v6694)-v6661)}else{(if v5120{v6661}else{common.v1})});
        let v6706=(common.v5102*common.v5102);
        let v6714=(if self.scalar_static_bool[253]{(v779*(((common.v5102*(self.scalar_static_f64[651]*v6699))-(v5177*common.v6539))/v6706))}else{common.v1});
        let v6715=(if self.scalar_static_bool[253]{(v779*(((common.v5102*(self.scalar_static_f64[651]*v6700))-(v5177*common.v6540))/v6706))}else{common.v1});
        let v6730=(if self.scalar_static_bool[253]{(self.scalar_static_f64[204]*((v5181*v6523)+(v5098*((v5180*v6414)+(v5065*v6714)))))}else{common.v1});
        let v6731=(if self.scalar_static_bool[253]{(self.scalar_static_f64[204]*((v5181*v6524)+(v5098*((v5180*v6415)+(v5065*v6715)))))}else{common.v1});
        let v6840=(if self.scalar_static_bool[256]{(self.scalar_static_f64[209]*((v5231*common.v6818)+(common.v5229*((v5230*common.v6760)+(common.v5197*((self.scalar_static_f64[515]*common.v5197)+(common.v4773*common.v6760)))))))}else{common.v1});
        let v6841=(if self.scalar_static_bool[256]{(self.scalar_static_f64[209]*((v5231*common.v6819)+(common.v5229*((v5230*common.v6761)+(common.v5197*((common.v5197*self.scalar_static_f64[525])+(common.v4773*common.v6761)))))))}else{common.v1});
        let v6864=(v5250*v5250);
        let v6871=(if v5254{(self.scalar_static_f64[73]*common.v6325)}else{(if common.v5239{(common.v6862/v6864)}else{common.v1})});
        let v6872=(if v5254{(self.scalar_static_f64[73]*common.v6326)}else{(if common.v5239{(common.v6863/v6864)}else{common.v1})});
        let v6922=(if self.scalar_static_bool[264]{(self.scalar_static_f64[587]*common.v6222)}else{v6331});
        let v6923=(if self.scalar_static_bool[264]{(self.scalar_static_f64[587]*common.v6223)}else{v6332});
        let v6939=(common.v59*v5293);
        let v6944=(if self.scalar_static_bool[266]{(-((-(((common.v5290*common.v6278)-(common.v5005*common.v6926))/common.v6931))/v6939))}else{v6353});
        let v6945=(if self.scalar_static_bool[266]{(-((-(((common.v5290*common.v6279)-(common.v5005*common.v6927))/common.v6931))/v6939))}else{v6354});
        let v6948=(v5295*v6944);
        let v6950=(v5295*v6945);
        let v6965=(v5302*v5302);
        let v6975=(if self.scalar_static_bool[268]{(self.scalar_static_f64[232]*(v6944+(((v5302*((v5300*(v6948+v6948))+(v5299*(v6944/v5295))))-(v5301*(-v6944)))/v6965)))}else{(if self.scalar_static_bool[267]{common.v1}else{v6382})});
        let v6976=(if self.scalar_static_bool[268]{(self.scalar_static_f64[232]*(v6945+(((v5302*((v5300*(v6950+v6950))+(v5299*(v6945/v5295))))-(v5301*(-v6945)))/v6965)))}else{(if self.scalar_static_bool[267]{common.v1}else{v6383})});
        let v6979=(if self.scalar_static_bool[266]{(v6944+v6975)}else{v6386});
        let v6980=(if self.scalar_static_bool[266]{(v6945+v6976)}else{v6387});
        let v7007=(if self.scalar_static_bool[266]{(self.scalar_static_f64[578]*((common.v5315*common.v6227)+(v5062*common.v6997)))}else{v6414});
        let v7008=(if self.scalar_static_bool[266]{(self.scalar_static_f64[578]*((common.v5315*common.v6228)+(v5062*common.v6998)))}else{v6415});
        let v7017=(if self.scalar_static_bool[266]{(self.scalar_static_f64[229]*((v5318*v6979)+(v5308*v7007)))}else{(if self.scalar_static_bool[265]{common.v1}else{v6424})});
        let v7018=(if self.scalar_static_bool[266]{(self.scalar_static_f64[229]*((v5318*v6980)+(v5308*v7008)))}else{(if self.scalar_static_bool[265]{common.v1}else{v6425})});
        let v7088=(v5344*v5344);
        let v7096=(self.scalar_static_f64[234]*f64::powf(v5344,self.scalar_static_f64[548]));
        let v7099=(if self.scalar_static_bool[272]{(common.v7083*v7096)}else{(if self.scalar_static_bool[271]{((-common.v7083)/v7088)}else{v6504})});
        let v7100=(if self.scalar_static_bool[272]{(common.v7086*v7096)}else{(if self.scalar_static_bool[271]{((-common.v7086)/v7088)}else{v6505})});
        let v7112=(v5351*v5351);
        let v7118=(if self.scalar_static_bool[270]{(((v5351*((v5349*v6979)+(v5308*v7099)))-(v5350*(v6979+v7099)))/v7112)}else{v6523});
        let v7119=(if self.scalar_static_bool[270]{(((v5351*((v5349*v6980)+(v5308*v7100)))-(v5350*(v6980+v7100)))/v7112)}else{v6524});
        let v7180=(v58*common.v7172);
        let v7181=(v58*common.v7173);
        let v7183=(v5377*v5377);
        let v7189=(v5382*v5382);
        let v7192=(if common.v5381{(v7180/v7189)}else{(if v5375{((-v7180)/v7183)}else{v6597})});
        let v7193=(if common.v5381{(v7181/v7189)}else{(if v5375{((-v7181)/v7183)}else{v6598})});
        let v7231=(v5384*v7192);
        let v7232=(v7231+v7231);
        let v7233=(v5384*v7193);
        let v7234=(v7233+v7233);
        let v7255=(if self.scalar_static_bool[270]{((v5409*common.v7227)+(common.v5402*(((v57*v7192)+(v61*v7232))+(v62*((v5404*v7192)+(v5384*v7232))))))}else{v6660});
        let v7256=(if self.scalar_static_bool[270]{((v5409*common.v7228)+(common.v5402*(((v57*v7193)+(v61*v7234))+(v62*((v5404*v7193)+(v5384*v7234))))))}else{v6661});
        let v7294=(if common.v5381{((common.v59*common.v7288)-v7255)}else{(if v5375{v7255}else{v6699})});
        let v7295=(if common.v5381{((common.v59*common.v7289)-v7256)}else{(if v5375{v7256}else{v6700})});
        let v7301=(common.v5357*common.v5357);
        let v7309=(if self.scalar_static_bool[270]{(v779*(((common.v5357*(self.scalar_static_f64[652]*v7294))-(v5432*common.v7134))/v7301))}else{v6714});
        let v7310=(if self.scalar_static_bool[270]{(v779*(((common.v5357*(self.scalar_static_f64[652]*v7295))-(v5432*common.v7135))/v7301))}else{v6715});
        let v7325=(if self.scalar_static_bool[270]{(self.scalar_static_f64[230]*((v5436*v7118)+(v5353*((v5435*v7007)+(v5318*v7309)))))}else{(if self.scalar_static_bool[269]{common.v1}else{v6730})});
        let v7326=(if self.scalar_static_bool[270]{(self.scalar_static_f64[230]*((v5436*v7119)+(v5353*((v5435*v7008)+(v5318*v7310)))))}else{(if self.scalar_static_bool[269]{common.v1}else{v6731})});
        let v7435=(if self.scalar_static_bool[274]{(self.scalar_static_f64[235]*((v5488*common.v7413)+(common.v5486*((v5487*common.v7355)+(common.v5454*((self.scalar_static_f64[515]*common.v5454)+(common.v4773*common.v7355)))))))}else{(if self.scalar_static_bool[273]{common.v1}else{v6840})});
        let v7436=(if self.scalar_static_bool[274]{(self.scalar_static_f64[235]*((v5488*common.v7414)+(common.v5486*((v5487*common.v7356)+(common.v5454*((common.v5454*self.scalar_static_f64[525])+(common.v4773*common.v7356)))))))}else{(if self.scalar_static_bool[273]{common.v1}else{v6841})});
        let v7461=(v5507*v5507);
        let v7468=(if v5511{(self.scalar_static_f64[80]*common.v6325)}else{(if common.v5496{(common.v7459/v7461)}else{(if self.scalar_static_bool[277]{common.v1}else{v6871})})});
        let v7469=(if v5511{(self.scalar_static_f64[80]*common.v6326)}else{(if common.v5496{(common.v7460/v7461)}else{(if self.scalar_static_bool[277]{common.v1}else{v6872})})});
        let v7534=(common.v59*v5548);
        let v7539=(if self.scalar_static_bool[284]{(-((-(((common.v5545*common.v6278)-(common.v5005*common.v7521))/common.v7526))/v7534))}else{v6944});
        let v7540=(if self.scalar_static_bool[284]{(-((-(((common.v5545*common.v6279)-(common.v5005*common.v7522))/common.v7526))/v7534))}else{v6945});
        let v7543=(v5550*v7539);
        let v7545=(v5550*v7540);
        let v7560=(v5557*v5557);
        let v7574=(if self.scalar_static_bool[284]{(v7539+(if self.scalar_static_bool[286]{(self.scalar_static_f64[255]*(v7539+(((v5557*((v5555*(v7543+v7543))+(v5554*(v7539/v5550))))-(v5556*(-v7539)))/v7560)))}else{(if self.scalar_static_bool[285]{common.v1}else{v6975})}))}else{v6979});
        let v7575=(if self.scalar_static_bool[284]{(v7540+(if self.scalar_static_bool[286]{(self.scalar_static_f64[255]*(v7540+(((v5557*((v5555*(v7545+v7545))+(v5554*(v7540/v5550))))-(v5556*(-v7540)))/v7560)))}else{(if self.scalar_static_bool[285]{common.v1}else{v6976})}))}else{v6980});
        let v7602=(if self.scalar_static_bool[284]{(self.scalar_static_f64[583]*((common.v5570*common.v6227)+(v5062*common.v7592)))}else{v7007});
        let v7603=(if self.scalar_static_bool[284]{(self.scalar_static_f64[583]*((common.v5570*common.v6228)+(v5062*common.v7593)))}else{v7008});
        let v7683=(v5599*v5599);
        let v7691=(self.scalar_static_f64[257]*f64::powf(v5599,self.scalar_static_f64[550]));
        let v7694=(if self.scalar_static_bool[290]{(common.v7678*v7691)}else{(if self.scalar_static_bool[289]{((-common.v7678)/v7683)}else{v7099})});
        let v7695=(if self.scalar_static_bool[290]{(common.v7681*v7691)}else{(if self.scalar_static_bool[289]{((-common.v7681)/v7683)}else{v7100})});
        let v7707=(v5606*v5606);
        let v7775=(v58*common.v7767);
        let v7776=(v58*common.v7768);
        let v7778=(v5632*v5632);
        let v7784=(v5637*v5637);
        let v7787=(if common.v5636{(v7775/v7784)}else{(if v5630{((-v7775)/v7778)}else{v7192})});
        let v7788=(if common.v5636{(v7776/v7784)}else{(if v5630{((-v7776)/v7778)}else{v7193})});
        let v7826=(v5639*v7787);
        let v7827=(v7826+v7826);
        let v7828=(v5639*v7788);
        let v7829=(v7828+v7828);
        let v7850=(if self.scalar_static_bool[288]{((v5664*common.v7822)+(common.v5657*(((v57*v7787)+(v61*v7827))+(v62*((v5659*v7787)+(v5639*v7827))))))}else{v7255});
        let v7851=(if self.scalar_static_bool[288]{((v5664*common.v7823)+(common.v5657*(((v57*v7788)+(v61*v7829))+(v62*((v5659*v7788)+(v5639*v7829))))))}else{v7256});
        let v7896=(common.v5612*common.v5612);
        let v8056=(v5762*v5762);
        let v8075=((v5774*(if v5766{(self.scalar_static_f64[87]*common.v6325)}else{(if common.v5751{(common.v8054/v8056)}else{(if self.scalar_static_bool[295]{common.v1}else{v7468})})}))+(v5770*(self.scalar_static_f64[228]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[258]*((v5743*common.v8008)+(common.v5741*((v5742*common.v7950)+(common.v5709*((self.scalar_static_f64[515]*common.v5709)+(common.v4773*common.v7950)))))))}else{(if self.scalar_static_bool[291]{common.v1}else{v7435})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[253]*((v5691*(if self.scalar_static_bool[288]{(((v5606*((v5604*v7574)+(v5563*v7694)))-(v5605*(v7574+v7694)))/v7707)}else{v7118}))+(v5608*((v5690*v7602)+(v5573*(if self.scalar_static_bool[288]{(v779*(((common.v5612*(self.scalar_static_f64[653]*(if common.v5636{((common.v59*common.v7883)-v7850)}else{(if v5630{v7850}else{v7294})})))-(v5687*common.v7729))/v7896))}else{v7309}))))))}else{(if self.scalar_static_bool[287]{common.v1}else{v7325})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[589]*common.v6222)}else{v6922})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[252]*((v5573*v7574)+(v5563*v7602)))}else{(if self.scalar_static_bool[283]{common.v1}else{v7017})})))))));
        let v8078=((v5774*(if v5766{(self.scalar_static_f64[87]*common.v6326)}else{(if common.v5751{(common.v8055/v8056)}else{(if self.scalar_static_bool[295]{common.v1}else{v7469})})}))+(v5770*(self.scalar_static_f64[228]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[258]*((v5743*common.v8009)+(common.v5741*((v5742*common.v7951)+(common.v5709*((common.v5709*self.scalar_static_f64[525])+(common.v4773*common.v7951)))))))}else{(if self.scalar_static_bool[291]{common.v1}else{v7436})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[253]*((v5691*(if self.scalar_static_bool[288]{(((v5606*((v5604*v7575)+(v5563*v7695)))-(v5605*(v7575+v7695)))/v7707)}else{v7119}))+(v5608*((v5690*v7603)+(v5573*(if self.scalar_static_bool[288]{(v779*(((common.v5612*(self.scalar_static_f64[653]*(if common.v5636{((common.v59*common.v7884)-v7851)}else{(if v5630{v7851}else{v7295})})))-(v5687*common.v7730))/v7896))}else{v7310}))))))}else{(if self.scalar_static_bool[287]{common.v1}else{v7326})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[589]*common.v6223)}else{v6923})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[252]*((v5573*v7575)+(v5563*v7603)))}else{(if self.scalar_static_bool[283]{common.v1}else{v7018})})))))));

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (((if self.scalar_static_bool[246]{(((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{(v5258*v5262)}else{common.v1}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{(v5515*v5519)}else{common.v1})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[282]{(v5770*v5774)}else{common.v1})))}else{(if self.scalar_static_bool[29]{((if self.scalar_static_bool[739]{(self.scalar_static_f64[4093]*((if self.scalar_static_bool[739]{(if v4820{(common.v399/v4822)}else{(if v4824{(self.scalar_static_f64[4092]*(common.v5+(v4819-self.scalar_static_f64[4090])))}else{v4828})})}else{v4804})-common.v5))}else{(if self.scalar_static_bool[737]{(common.v4773*v4811)}else{common.v1})})+((if self.scalar_static_bool[29]{(self.scalar_static_f64[3955]*(v4787-common.v5))}else{common.v1})+(if self.scalar_static_bool[29]{(self.scalar_static_f64[3978]*(v4804-common.v5))}else{common.v1})))}else{common.v1})})*self.scalar_static_f64[524])),
            0,
            multiplicity * ((self.scalar_static_f64[524]*(if self.scalar_static_bool[246]{(((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{((v5262*v6871)+(v5258*(self.scalar_static_f64[228]*(v6840+(v6730+(v6331+v6424))))))}else{common.v1}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{((v5519*v7468)+(v5515*(self.scalar_static_f64[228]*(v7435+(v7325+(v6922+v7017))))))}else{common.v1})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[282]{v8075}else{common.v1})))}else{(if self.scalar_static_bool[29]{((if self.scalar_static_bool[739]{(self.scalar_static_f64[4093]*(if self.scalar_static_bool[739]{(if v4820{(self.scalar_static_f64[4139]/v5983)}else{(if v4824{self.scalar_static_f64[4142]}else{(v4828*self.scalar_static_f64[4134])})})}else{v5959}))}else{(if self.scalar_static_bool[737]{((self.scalar_static_f64[515]*v4811)+(common.v4773*self.scalar_static_f64[4130]))}else{common.v1})})+((if self.scalar_static_bool[29]{(self.scalar_static_f64[3955]*v5932)}else{common.v1})+(if self.scalar_static_bool[29]{(self.scalar_static_f64[3978]*v5959)}else{common.v1})))}else{common.v1})}))),
            1,
            multiplicity * ((self.scalar_static_f64[524]*(if self.scalar_static_bool[246]{(((self.scalar_static_f64[140]*(if self.scalar_static_bool[249]{((v5262*v6872)+(v5258*(self.scalar_static_f64[228]*(v6841+(v6731+(v6332+v6425))))))}else{common.v1}))+(self.scalar_static_f64[142]*(if self.scalar_static_bool[264]{((v5519*v7469)+(v5515*(self.scalar_static_f64[228]*(v7436+(v7326+(v6923+v7018))))))}else{common.v1})))+(self.scalar_static_f64[144]*(if self.scalar_static_bool[282]{v8078}else{common.v1})))}else{(if self.scalar_static_bool[29]{((if self.scalar_static_bool[739]{(self.scalar_static_f64[4093]*(if self.scalar_static_bool[739]{(if v4820{(self.scalar_static_f64[4141]/v5983)}else{(if v4824{self.scalar_static_f64[4143]}else{(v4828*self.scalar_static_f64[4135])})})}else{v5960}))}else{(if self.scalar_static_bool[737]{((v4811*self.scalar_static_f64[525])+(common.v4773*self.scalar_static_f64[4131]))}else{common.v1})})+((if self.scalar_static_bool[29]{(self.scalar_static_f64[3955]*v5933)}else{common.v1})+(if self.scalar_static_bool[29]{(self.scalar_static_f64[3978]*v5960)}else{common.v1})))}else{common.v1})}))),
        );
        let v5909_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v5909);
        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (v5909_ddt),
            0,
            multiplicity * (((common.v8291) * ddt_scale)),
            1,
            multiplicity * (((common.v8292) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (common.v1),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[1]),
            nodes[0],
            multiplicity * (common.v8291),
            nodes[1],
            multiplicity * (common.v8292),
        );
    }
}
