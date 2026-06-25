#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[409] = (ctx.temperature() + p.p0);

        s.v[429] = (p.p126 + 273.15);

        s.v[36] = p.p336;

        s.v[37] = p.p21;

        s.v[38] = p.p348;

        s.v[39] = p.p213;

        s.v[40] = p.p127;

        s.v[41] = p.p182;

        s.v[42] = p.p350;

        s.v[43] = p.p355;

        s.v[44] = p.p234;

        s.v[45] = p.p236;

        s.v[46] = p.p373;

        s.v[48] = p.p181;

        if (p.p41 != 0.0) {
            s.store_scalar(416, 3.9);
        }

        if (p.p41 != 0.0) {
            s.store_scalar(415, p.p45);
        }

        if (p.p41 != 0.0) {
            s.store_scalar(417, (8.85418e-12 * p.p47));
        }

        if (p.p41 != 0.0) {
            s.store_sqrt_ad(419, A::scale(s.ad_value(417), (2000000.0 * 1.602176462e-19)));
        }

        if (p.p41 != 0.0) {
            s.store_div_ad_lhs(396, A::scale(s.ad_value(416), 8.85418e-12), 415);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(416, p.p46);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(415, p.p66);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(417, 1.03594e-10);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(419, 5.753e-12);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(396, (3.453133e-11 / p.p66));
        }

        s.v[431] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        s.v[432] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        s.v[433] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        s.v[434] = if ((1.0 != 0.0) && (1.0 != 0.0)) { 1.0 } else { 0.0 };

        s.v[435] = 1.0;

        s.v[436] = if (((1.0 != 0.0) && (1.0 != 0.0)) && (1.0 != 0.0)) { 1.0 } else { 0.0 };

        s.v[437] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        s.v[438] = if (((1.0 != 0.0) && (1.0 != 0.0)) && (1.0 != 0.0)) { 1.0 } else { 0.0 };

        s.v[439] = if ((1.0 != 0.0) && (1.0 != 0.0)) { 1.0 } else { 0.0 };

        s.v[440] = 1.0;

        s.v[441] = if (((1.0 != 0.0) && (1.0 != 0.0)) && (1.0 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[431] != 0.0) {
            s.store_scalar(399, 0.0);
        }

        s.v[456] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[431] != 0.0)) && (s.v[456] != 0.0)) {
            s.store_scalar(399, 0.0);
        }

        s.v[458] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        s.v[459] = if ((s.v[38] == 0.0) && (p.p349 == 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[431] != 0.0)) && (!(s.v[456] != 0.0))) && (s.v[458] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_scalar(399, 2.0);
        }

        if ((((!(s.v[431] != 0.0)) && (!(s.v[456] != 0.0))) && (s.v[458] != 0.0)) && (!(s.v[459] != 0.0))) {
            s.store_scalar(399, 1.0);
        }

        s.v[460] = if ((s.v[38] == 0.0) && (p.p349 == 0.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[431] != 0.0)) && (!(s.v[456] != 0.0))) && (!(s.v[458] != 0.0))) && (s.v[460] != 0.0)) {
            s.store_scalar(38, 1.0);
        }

        if ((((!(s.v[431] != 0.0)) && (!(s.v[456] != 0.0))) && (!(s.v[458] != 0.0))) && (s.v[460] != 0.0)) {
            s.store_scalar(399, 1.0);
        }

        if ((((!(s.v[431] != 0.0)) && (!(s.v[456] != 0.0))) && (!(s.v[458] != 0.0))) && (!(s.v[460] != 0.0))) {
            s.store_scalar(399, 1.0);
        }

        s.v[461] = if self.param_given[213] { 1.0 } else { 0.0 };

        if (s.v[461] != 0.0) {
            s.store_scalar(39, p.p213);
        }

        if (!(s.v[461] != 0.0)) {
            s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p.p66))) as f64).ln()));
        }

        s.v[533] = if (s.v[48] < 0.1) { 1.0 } else { 0.0 };

        if (s.v[533] != 0.0) {
            s.store_scalar(48, 0.1);
        }

        s.v[534] = if (s.v[41] < 0.1) { 1.0 } else { 0.0 };

        if (s.v[534] != 0.0) {
            s.store_scalar(41, 0.1);
        }

        s.v[429] = (p.p126 + 273.15);

        s.v[476] = (s.v[409] / s.v[429]);

        if (p.p41 != 0.0) {
            s.store_sqrt_ad(397, A::mul(A::div(s.ad_value(417), A::scale(s.ad_value(416), 8.85418e-12)), s.ad_value(415)));
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p.p66)) as f64).sqrt());
        }

        s.v[535] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[535] != 0.0) {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
        }

        if (s.v[535] != 0.0) {
            s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));
        }

        if (s.v[535] != 0.0) {
            s.copy_ad(394, 466);
        }

        if (s.v[535] != 0.0) {
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
        }

        if (s.v[535] != 0.0) {
            s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));
        }

        if (s.v[535] != 0.0) {
            s.copy_ad(395, 465);
        }

        if (s.v[535] != 0.0) {
            s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div(s.ad_value(465), A::scale(s.ad_value(49), 2.0)));
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(466, (p.p49 - (((p.p50 * s.v[429]) * s.v[429]) / (s.v[429] + p.p51))));
        }

        if (!(s.v[535] != 0.0)) {
            s.copy_ad(394, 466);
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(465, (p.p49 - (((p.p50 * s.v[409]) * s.v[409]) / (s.v[409] + p.p51))));
        }

        if (!(s.v[535] != 0.0)) {
            s.copy_ad(395, 465);
        }

        if (!(s.v[535] != 0.0)) {
            s.store_offset_ad(530, A::sub(A::div(s.ad_value(466), A::scale(s.ad_value(480), 2.0)), A::div(s.ad_value(465), A::scale(s.ad_value(49), 2.0))), (if (((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));
        }

        s.v[50] = (p.p16 * p.p349);

        s.v[474] = p.p1;

        s.v[475] = (p.p2 / p.p3);

        s.v[467] = ((s.v[474]) as f64).powf(p.p190);

        s.v[468] = ((s.v[475]) as f64).powf(p.p193);

        s.v[463] = (((p.p188 / s.v[467]) + (p.p191 / s.v[468])) + (p.p194 / (s.v[467] * s.v[468])));

        s.v[326] = (p.p187 + s.v[463]);

        s.v[463] = (((p.p189 / s.v[467]) + (p.p192 / s.v[468])) + (p.p195 / (s.v[467] * s.v[468])));

        s.v[330] = (p.p217 + s.v[463]);

        s.v[215] = (p.p410 + s.v[463]);

        s.v[536] = if (s.v[215] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[536] != 0.0) {
            s.store_scalar(215, 0.0);
        }

        s.v[469] = ((s.v[474]) as f64).powf(p.p202);

        s.v[470] = ((s.v[475]) as f64).powf(p.p205);

        s.v[464] = (((p.p200 / s.v[469]) + (p.p203 / s.v[470])) + (p.p206 / (s.v[469] * s.v[470])));

        s.v[325] = (p.p197 + s.v[464]);

        s.v[464] = (((p.p201 / s.v[469]) + (p.p204 / s.v[470])) + (p.p207 / (s.v[469] * s.v[470])));

        s.v[329] = (p.p216 + s.v[464]);

        s.v[327] = (p.p1 - (2.0 * s.v[326]));

        s.v[328] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[325]));

        s.v[348] = ((s.v[328] / p.p23) + p.p24);

        s.v[347] = ((s.v[328] / p.p23) + p.p25);

        s.v[331] = (p.p1 - (2.0 * s.v[330]));

        s.v[332] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[329]));

        s.v[349] = ((s.v[332] / p.p23) + p.p24);

        s.v[350] = ((s.v[332] / p.p23) + p.p25);

        s.v[365] = ((p.p1 - (2.0 * s.v[330])) - p.p360);

        s.v[366] = (s.v[365] + (2.0 * p.p372));

        s.v[112] = p.p85;

        s.v[113] = p.p86;

        s.v[114] = p.p87;

        s.v[116] = p.p88;

        s.v[117] = p.p89;

        s.copy_ad(239, 39);

        s.v[240] = p.p214;

        s.v[241] = p.p215;

        s.v[543] = if (s.v[241] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_scalar(333, 2.0);
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));
        }

        s.v[544] = if (p.p65 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_scalar(477, (1e-6 / s.v[327]));
        }

        if (s.v[544] != 0.0) {
            s.store_scalar(478, (1e-6 / s.v[328]));
        }

        if (s.v[544] != 0.0) {
            s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(477, (1.0 / s.v[327]));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(478, (1.0 / s.v[328]));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));
        }

        s.store_add_ad(108, A::add(A::offset(A::scale(s.ad_value(477), p.p488), p.p82), A::scale(s.ad_value(478), p.p678)), A::scale(s.ad_value(479), p.p868));

        s.store_add_ad(109, A::add(A::offset(A::scale(s.ad_value(477), p.p489), p.p81), A::scale(s.ad_value(478), p.p679)), A::scale(s.ad_value(479), p.p869));

        s.store_add_ad(110, A::add(A::offset(A::scale(s.ad_value(477), p.p490), p.p83), A::scale(s.ad_value(478), p.p680)), A::scale(s.ad_value(479), p.p871));

        s.store_add_ad(111, A::add(A::offset(A::scale(s.ad_value(477), p.p491), p.p84), A::scale(s.ad_value(478), p.p681)), A::scale(s.ad_value(479), p.p870));

        s.store_add_ad(137, A::add(A::offset(A::scale(s.ad_value(477), p.p492), p.p108), A::scale(s.ad_value(478), p.p682)), A::scale(s.ad_value(479), p.p872));

        s.store_add_ad(152, A::add(A::offset(A::scale(s.ad_value(477), p.p493), p.p109), A::scale(s.ad_value(478), p.p683)), A::scale(s.ad_value(479), p.p873));

        s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(477), p.p494), p.p90), A::scale(s.ad_value(478), p.p684)), A::scale(s.ad_value(479), p.p874));

        s.store_add_ad(124, A::add(A::offset(A::scale(s.ad_value(477), p.p497), p.p94), A::scale(s.ad_value(478), p.p687)), A::scale(s.ad_value(479), p.p877));

        s.store_add_ad(264, A::add(A::offset(A::scale(s.ad_value(477), p.p495), p.p300), A::scale(s.ad_value(478), p.p685)), A::scale(s.ad_value(479), p.p875));

        s.store_add_ad(265, A::add(A::offset(A::scale(s.ad_value(477), p.p496), p.p301), A::scale(s.ad_value(478), p.p686)), A::scale(s.ad_value(479), p.p876));

        s.store_add_ad(125, A::add(A::offset(A::scale(s.ad_value(477), p.p498), p.p95), A::scale(s.ad_value(478), p.p688)), A::scale(s.ad_value(479), p.p878));

        s.store_add_ad(126, A::add(A::offset(A::scale(s.ad_value(477), p.p499), p.p96), A::scale(s.ad_value(478), p.p689)), A::scale(s.ad_value(479), p.p879));

        s.store_add_ad(263, A::add(A::offset(A::scale(s.ad_value(477), p.p500), p.p371), A::scale(s.ad_value(478), p.p690)), A::scale(s.ad_value(479), p.p880));

        s.store_add_ad(127, A::add(A::offset(A::scale(s.ad_value(477), p.p501), p.p97), A::scale(s.ad_value(478), p.p691)), A::scale(s.ad_value(479), p.p881));

        s.store_add_ad(128, A::add(A::offset(A::scale(s.ad_value(477), p.p1024), p.p1021), A::scale(s.ad_value(478), p.p1027)), A::scale(s.ad_value(479), p.p1030));

        s.store_add_ad(377, A::add(A::offset(A::scale(s.ad_value(477), p.p502), p.p98), A::scale(s.ad_value(478), p.p692)), A::scale(s.ad_value(479), p.p882));

        s.store_add_ad(129, A::add(A::offset(A::scale(s.ad_value(477), p.p503), p.p99), A::scale(s.ad_value(478), p.p693)), A::scale(s.ad_value(479), p.p883));

        s.store_add_ad(130, A::add(A::offset(A::scale(s.ad_value(477), p.p504), p.p100), A::scale(s.ad_value(478), p.p694)), A::scale(s.ad_value(479), p.p884));

        s.store_add_ad(131, A::add(A::offset(A::scale(s.ad_value(477), p.p505), p.p101), A::scale(s.ad_value(478), p.p695)), A::scale(s.ad_value(479), p.p885));

        s.store_add_ad(132, A::add(A::offset(A::scale(s.ad_value(477), p.p506), p.p102), A::scale(s.ad_value(478), p.p696)), A::scale(s.ad_value(479), p.p886));

        s.store_add_ad(133, A::add(A::offset(A::scale(s.ad_value(477), p.p507), p.p103), A::scale(s.ad_value(478), p.p697)), A::scale(s.ad_value(479), p.p887));

        s.store_add_ad(133, A::add(A::offset(A::scale(s.ad_value(477), p.p507), p.p103), A::scale(s.ad_value(478), p.p697)), A::scale(s.ad_value(479), p.p887));

        s.store_add_ad(134, A::add(A::offset(A::scale(s.ad_value(477), p.p508), p.p104), A::scale(s.ad_value(478), p.p698)), A::scale(s.ad_value(479), p.p888));

        s.store_add_ad(144, A::add(A::offset(A::scale(s.ad_value(477), p.p509), p.p116), A::scale(s.ad_value(478), p.p699)), A::scale(s.ad_value(479), p.p889));

        s.store_add_ad(138, A::add(A::offset(A::scale(s.ad_value(477), p.p511), p.p110), A::scale(s.ad_value(478), p.p701)), A::scale(s.ad_value(479), p.p891));

        s.store_add_ad(140, A::add(A::offset(A::scale(s.ad_value(477), p.p512), p.p112), A::scale(s.ad_value(478), p.p702)), A::scale(s.ad_value(479), p.p892));

        s.store_add_ad(142, A::add(A::offset(A::scale(s.ad_value(477), p.p513), p.p114), A::scale(s.ad_value(478), p.p703)), A::scale(s.ad_value(479), p.p893));

        s.store_add_ad(101, A::add(A::offset(A::scale(s.ad_value(477), p.p518), p.p74), A::scale(s.ad_value(478), p.p708)), A::scale(s.ad_value(479), p.p898));

        s.store_add_ad(103, A::add(A::offset(A::scale(s.ad_value(477), p.p519), p.p76), A::scale(s.ad_value(478), p.p709)), A::scale(s.ad_value(479), p.p899));

        s.store_add_ad(104, A::add(A::offset(A::scale(s.ad_value(477), p.p520), p.p77), A::scale(s.ad_value(478), p.p710)), A::scale(s.ad_value(479), p.p900));

        s.store_add_ad(199, A::add(A::offset(A::scale(s.ad_value(477), p.p521), p.p208), A::scale(s.ad_value(478), p.p711)), A::scale(s.ad_value(479), p.p901));

        s.store_add_ad(200, A::add(A::offset(A::scale(s.ad_value(477), p.p522), p.p209), A::scale(s.ad_value(478), p.p712)), A::scale(s.ad_value(479), p.p902));

        s.store_add_ad(107, A::add(A::offset(A::scale(s.ad_value(477), p.p523), p.p80), A::scale(s.ad_value(478), p.p713)), A::scale(s.ad_value(479), p.p903));

        s.store_add_ad(266, A::add(A::offset(A::scale(s.ad_value(477), p.p524), p.p302), A::scale(s.ad_value(478), p.p714)), A::scale(s.ad_value(479), p.p904));

        s.store_add_ad(105, A::add(A::offset(A::scale(s.ad_value(477), p.p525), p.p78), A::scale(s.ad_value(478), p.p715)), A::scale(s.ad_value(479), p.p905));

        s.store_add_ad(106, A::add(A::offset(A::scale(s.ad_value(477), p.p526), p.p79), A::scale(s.ad_value(478), p.p716)), A::scale(s.ad_value(479), p.p906));

        s.store_add_ad(181, A::add(A::offset(A::scale(s.ad_value(477), p.p527), p.p132), A::scale(s.ad_value(478), p.p717)), A::scale(s.ad_value(479), p.p907));

        s.store_add_ad(170, A::add(A::offset(A::scale(s.ad_value(477), p.p528), p.p133), A::scale(s.ad_value(478), p.p718)), A::scale(s.ad_value(479), p.p908));

        s.store_add_ad(169, A::add(A::offset(A::scale(s.ad_value(477), p.p529), p.p134), A::scale(s.ad_value(478), p.p719)), A::scale(s.ad_value(479), p.p909));

        s.store_add_ad(184, A::add(A::offset(A::scale(s.ad_value(477), p.p530), p.p142), A::scale(s.ad_value(478), p.p720)), A::scale(s.ad_value(479), p.p910));

        s.store_add_ad(185, A::add(A::offset(A::scale(s.ad_value(477), p.p531), p.p143), A::scale(s.ad_value(478), p.p721)), A::scale(s.ad_value(479), p.p911));

        s.store_add_ad(183, A::add(A::offset(A::scale(s.ad_value(477), p.p532), p.p141), A::scale(s.ad_value(478), p.p722)), A::scale(s.ad_value(479), p.p912));

        s.store_add_ad(196, A::add(A::offset(A::scale(s.ad_value(477), p.p533), p.p196), A::scale(s.ad_value(478), p.p723)), A::scale(s.ad_value(479), p.p913));

        s.store_add_ad(100, A::add(A::offset(A::scale(s.ad_value(477), p.p534), p.p73), A::scale(s.ad_value(478), p.p724)), A::scale(s.ad_value(479), p.p914));

        s.store_add_ad(197, A::add(A::offset(A::scale(s.ad_value(477), p.p535), p.p198), A::scale(s.ad_value(478), p.p725)), A::scale(s.ad_value(479), p.p915));

        s.store_add_ad(198, A::add(A::offset(A::scale(s.ad_value(477), p.p536), p.p199), A::scale(s.ad_value(478), p.p726)), A::scale(s.ad_value(479), p.p916));

        s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(477), p.p537), p.p125), A::scale(s.ad_value(478), p.p727)), A::scale(s.ad_value(479), p.p917));

        s.store_add_ad(187, A::add(A::offset(A::scale(s.ad_value(477), p.p538), p.p145), A::scale(s.ad_value(478), p.p728)), A::scale(s.ad_value(479), p.p918));

        s.store_add_ad(188, A::add(A::offset(A::scale(s.ad_value(477), p.p539), p.p146), A::scale(s.ad_value(478), p.p729)), A::scale(s.ad_value(479), p.p919));

        s.store_add_ad(189, A::add(A::offset(A::scale(s.ad_value(477), p.p540), p.p147), A::scale(s.ad_value(478), p.p730)), A::scale(s.ad_value(479), p.p920));

        s.store_add_ad(190, A::add(A::offset(A::scale(s.ad_value(477), p.p541), p.p148), A::scale(s.ad_value(478), p.p731)), A::scale(s.ad_value(479), p.p921));

        s.store_add_ad(136, A::add(A::offset(A::scale(s.ad_value(477), p.p542), p.p106), A::scale(s.ad_value(478), p.p732)), A::scale(s.ad_value(479), p.p922));

        s.store_add_ad(99, A::add(A::offset(A::scale(s.ad_value(477), p.p543), p.p72), A::scale(s.ad_value(478), p.p733)), A::scale(s.ad_value(479), p.p923));

        s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(477), p.p544), p.p69), A::scale(s.ad_value(478), p.p734)), A::scale(s.ad_value(479), p.p924));

        s.store_add_ad(97, A::add(A::offset(A::scale(s.ad_value(477), p.p545), p.p70), A::scale(s.ad_value(478), p.p735)), A::scale(s.ad_value(479), p.p925));

        s.store_add_ad(98, A::add(A::offset(A::scale(s.ad_value(477), p.p546), p.p71), A::scale(s.ad_value(478), p.p736)), A::scale(s.ad_value(479), p.p926));

        s.store_add_ad(191, A::add(A::offset(A::scale(s.ad_value(477), p.p547), p.p149), A::scale(s.ad_value(478), p.p737)), A::scale(s.ad_value(479), p.p927));

        s.store_add_ad(192, A::add(A::offset(A::scale(s.ad_value(477), p.p548), p.p150), A::scale(s.ad_value(478), p.p738)), A::scale(s.ad_value(479), p.p928));

        s.store_add_ad(193, A::add(A::offset(A::scale(s.ad_value(477), p.p549), p.p151), A::scale(s.ad_value(478), p.p739)), A::scale(s.ad_value(479), p.p929));

        s.store_add_ad(194, A::add(A::offset(A::scale(s.ad_value(477), p.p550), p.p152), A::scale(s.ad_value(478), p.p740)), A::scale(s.ad_value(479), p.p930));

        s.store_add_ad(135, A::add(A::offset(A::scale(s.ad_value(477), p.p551), p.p105), A::scale(s.ad_value(478), p.p741)), A::scale(s.ad_value(479), p.p931));

        s.store_add_ad(195, A::add(A::offset(A::scale(s.ad_value(477), p.p552), p.p153), A::scale(s.ad_value(478), p.p742)), A::scale(s.ad_value(479), p.p932));

        s.store_add_ad(180, A::add(A::offset(A::scale(s.ad_value(477), p.p553), p.p130), A::scale(s.ad_value(478), p.p743)), A::scale(s.ad_value(479), p.p933));

        s.store_add_ad(201, A::add(A::offset(A::scale(s.ad_value(477), p.p554), p.p218), A::scale(s.ad_value(478), p.p744)), A::scale(s.ad_value(479), p.p934));

        s.store_add_ad(267, A::add(A::offset(A::scale(s.ad_value(477), p.p555), p.p314), A::scale(s.ad_value(478), p.p745)), A::scale(s.ad_value(479), p.p935));

        s.store_add_ad(268, A::add(A::offset(A::scale(s.ad_value(477), p.p558), p.p315), A::scale(s.ad_value(478), p.p748)), A::scale(s.ad_value(479), p.p938));

        s.store_add_ad(269, A::add(A::offset(A::scale(s.ad_value(477), p.p557), p.p316), A::scale(s.ad_value(478), p.p747)), A::scale(s.ad_value(479), p.p937));

        s.store_add_ad(270, A::add(A::offset(A::scale(s.ad_value(477), p.p560), p.p317), A::scale(s.ad_value(478), p.p750)), A::scale(s.ad_value(479), p.p940));

        s.store_add_ad(271, A::add(A::offset(A::scale(s.ad_value(477), p.p556), p.p318), A::scale(s.ad_value(478), p.p746)), A::scale(s.ad_value(479), p.p936));

        s.store_add_ad(272, A::add(A::offset(A::scale(s.ad_value(477), p.p559), p.p319), A::scale(s.ad_value(478), p.p749)), A::scale(s.ad_value(479), p.p939));

        s.store_add_ad(202, A::add(A::offset(A::scale(s.ad_value(477), p.p561), p.p304), A::scale(s.ad_value(478), p.p751)), A::scale(s.ad_value(479), p.p941));

        s.store_add_ad(273, A::add(A::offset(A::scale(s.ad_value(477), p.p562), p.p305), A::scale(s.ad_value(478), p.p752)), A::scale(s.ad_value(479), p.p942));

        s.store_add_ad(274, A::add(A::offset(A::scale(s.ad_value(477), p.p563), p.p306), A::scale(s.ad_value(478), p.p753)), A::scale(s.ad_value(479), p.p943));

        s.store_add_ad(275, A::add(A::offset(A::scale(s.ad_value(477), p.p564), p.p307), A::scale(s.ad_value(478), p.p754)), A::scale(s.ad_value(479), p.p944));

        s.store_add_ad(276, A::add(A::offset(A::scale(s.ad_value(477), p.p565), p.p309), A::scale(s.ad_value(478), p.p755)), A::scale(s.ad_value(479), p.p945));

        s.store_add_ad(277, A::add(A::offset(A::scale(s.ad_value(477), p.p566), p.p321), A::scale(s.ad_value(478), p.p756)), A::scale(s.ad_value(479), p.p946));

        s.store_add_ad(278, A::add(A::offset(A::scale(s.ad_value(477), p.p567), p.p310), A::scale(s.ad_value(478), p.p757)), A::scale(s.ad_value(479), p.p947));

        s.store_add_ad(279, A::add(A::offset(A::scale(s.ad_value(477), p.p568), p.p311), A::scale(s.ad_value(478), p.p758)), A::scale(s.ad_value(479), p.p948));

        s.store_add_ad(280, A::add(A::offset(A::scale(s.ad_value(477), p.p569), p.p312), A::scale(s.ad_value(478), p.p759)), A::scale(s.ad_value(479), p.p949));

        s.store_add_ad(281, A::add(A::offset(A::scale(s.ad_value(477), p.p570), p.p313), A::scale(s.ad_value(478), p.p760)), A::scale(s.ad_value(479), p.p950));

        s.store_add_ad(282, A::add(A::offset(A::scale(s.ad_value(477), p.p571), p.p158), A::scale(s.ad_value(478), p.p761)), A::scale(s.ad_value(479), p.p951));

        s.store_add_ad(283, A::add(A::offset(A::scale(s.ad_value(477), p.p572), p.p159), A::scale(s.ad_value(478), p.p762)), A::scale(s.ad_value(479), p.p952));

        s.store_add_ad(284, A::add(A::offset(A::scale(s.ad_value(477), p.p573), p.p160), A::scale(s.ad_value(478), p.p763)), A::scale(s.ad_value(479), p.p953));

        s.store_add_ad(285, A::add(A::offset(A::scale(s.ad_value(477), p.p574), p.p161), A::scale(s.ad_value(478), p.p764)), A::scale(s.ad_value(479), p.p954));

        s.store_add_ad(286, A::add(A::offset(A::scale(s.ad_value(477), p.p1025), p.p1022), A::scale(s.ad_value(478), p.p1028)), A::scale(s.ad_value(479), p.p1031));

        s.store_add_ad(287, A::add(A::offset(A::scale(s.ad_value(477), p.p575), p.p162), A::scale(s.ad_value(478), p.p765)), A::scale(s.ad_value(479), p.p955));

        s.store_add_ad(288, A::add(A::offset(A::scale(s.ad_value(477), p.p576), p.p163), A::scale(s.ad_value(478), p.p766)), A::scale(s.ad_value(479), p.p956));

        s.store_add_ad(289, A::add(A::offset(A::scale(s.ad_value(477), p.p577), p.p164), A::scale(s.ad_value(478), p.p767)), A::scale(s.ad_value(479), p.p957));

        s.store_add_ad(290, A::add(A::offset(A::scale(s.ad_value(477), p.p578), p.p165), A::scale(s.ad_value(478), p.p768)), A::scale(s.ad_value(479), p.p958));

        s.store_add_ad(291, A::add(A::offset(A::scale(s.ad_value(477), p.p579), p.p166), A::scale(s.ad_value(478), p.p769)), A::scale(s.ad_value(479), p.p959));

        s.store_add_ad(292, A::add(A::offset(A::scale(s.ad_value(477), p.p580), p.p167), A::scale(s.ad_value(478), p.p770)), A::scale(s.ad_value(479), p.p960));

        s.store_add_ad(293, A::add(A::offset(A::scale(s.ad_value(477), p.p581), p.p168), A::scale(s.ad_value(478), p.p771)), A::scale(s.ad_value(479), p.p961));

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_add_ad(294, A::add(A::offset(A::scale(s.ad_value(477), p.p1026), p.p1023), A::scale(s.ad_value(478), p.p1029)), A::scale(s.ad_value(479), p.p1032));

        s.store_add_ad(295, A::add(A::offset(A::scale(s.ad_value(477), p.p582), p.p169), A::scale(s.ad_value(478), p.p772)), A::scale(s.ad_value(479), p.p962));

        s.store_add_ad(296, A::add(A::offset(A::scale(s.ad_value(477), p.p583), p.p170), A::scale(s.ad_value(478), p.p773)), A::scale(s.ad_value(479), p.p963));

        s.store_add_ad(297, A::add(A::offset(A::scale(s.ad_value(477), p.p584), p.p171), A::scale(s.ad_value(478), p.p774)), A::scale(s.ad_value(479), p.p964));

        s.store_add_ad(298, A::add(A::offset(A::scale(s.ad_value(477), p.p585), p.p322), A::scale(s.ad_value(478), p.p775)), A::scale(s.ad_value(479), p.p965));

        s.store_add_ad(299, A::add(A::offset(A::scale(s.ad_value(477), p.p586), p.p323), A::scale(s.ad_value(478), p.p776)), A::scale(s.ad_value(479), p.p966));

        s.store_add_ad(300, A::add(A::offset(A::scale(s.ad_value(477), p.p587), p.p172), A::scale(s.ad_value(478), p.p777)), A::scale(s.ad_value(479), p.p967));

        s.store_add_ad(301, A::add(A::offset(A::scale(s.ad_value(477), p.p588), p.p173), A::scale(s.ad_value(478), p.p778)), A::scale(s.ad_value(479), p.p968));

        s.store_add_ad(302, A::add(A::offset(A::scale(s.ad_value(477), p.p589), p.p324), A::scale(s.ad_value(478), p.p779)), A::scale(s.ad_value(479), p.p969));

        s.store_add_ad(303, A::add(A::offset(A::scale(s.ad_value(477), p.p590), p.p325), A::scale(s.ad_value(478), p.p780)), A::scale(s.ad_value(479), p.p970));

        s.store_add_ad(304, A::add(A::offset(A::scale(s.ad_value(477), p.p591), p.p326), A::scale(s.ad_value(478), p.p781)), A::scale(s.ad_value(479), p.p971));

        s.store_add_ad(305, A::add(A::offset(A::scale(s.ad_value(477), p.p592), p.p327), A::scale(s.ad_value(478), p.p782)), A::scale(s.ad_value(479), p.p972));

        s.store_add_ad(306, A::add(A::offset(A::scale(s.ad_value(477), p.p593), p.p328), A::scale(s.ad_value(478), p.p783)), A::scale(s.ad_value(479), p.p973));

        s.store_add_ad(307, A::add(A::offset(A::scale(s.ad_value(477), p.p594), p.p329), A::scale(s.ad_value(478), p.p784)), A::scale(s.ad_value(479), p.p974));

        s.store_add_ad(308, A::add(A::offset(A::scale(s.ad_value(477), p.p595), p.p330), A::scale(s.ad_value(478), p.p785)), A::scale(s.ad_value(479), p.p975));

        s.store_add_ad(309, A::add(A::offset(A::scale(s.ad_value(477), p.p596), p.p331), A::scale(s.ad_value(478), p.p786)), A::scale(s.ad_value(479), p.p976));

        s.store_add_ad(310, A::add(A::offset(A::scale(s.ad_value(477), p.p597), p.p332), A::scale(s.ad_value(478), p.p787)), A::scale(s.ad_value(479), p.p977));

        s.store_add_ad(312, A::add(A::offset(A::scale(s.ad_value(477), p.p599), p.p334), A::scale(s.ad_value(478), p.p789)), A::scale(s.ad_value(479), p.p979));

        s.store_add_ad(311, A::add(A::offset(A::scale(s.ad_value(477), p.p598), p.p333), A::scale(s.ad_value(478), p.p788)), A::scale(s.ad_value(479), p.p978));

        s.store_add_ad(313, A::add(A::offset(A::scale(s.ad_value(477), p.p600), p.p335), A::scale(s.ad_value(478), p.p790)), A::scale(s.ad_value(479), p.p980));

        s.store_add_ad(313, A::add(A::offset(A::scale(s.ad_value(477), p.p600), p.p335), A::scale(s.ad_value(478), p.p790)), A::scale(s.ad_value(479), p.p980));

        s.store_add_ad(314, A::add(A::offset(A::scale(s.ad_value(477), p.p601), p.p337), A::scale(s.ad_value(478), p.p791)), A::scale(s.ad_value(479), p.p981));

        s.store_add_ad(315, A::add(A::offset(A::scale(s.ad_value(477), p.p602), p.p338), A::scale(s.ad_value(478), p.p792)), A::scale(s.ad_value(479), p.p982));

        s.store_add_ad(316, A::add(A::offset(A::scale(s.ad_value(477), p.p603), p.p339), A::scale(s.ad_value(478), p.p793)), A::scale(s.ad_value(479), p.p983));

        s.store_add_ad(317, A::add(A::offset(A::scale(s.ad_value(477), p.p604), p.p340), A::scale(s.ad_value(478), p.p794)), A::scale(s.ad_value(479), p.p984));

        s.store_add_ad(318, A::add(A::offset(A::scale(s.ad_value(477), p.p605), p.p341), A::scale(s.ad_value(478), p.p795)), A::scale(s.ad_value(479), p.p985));

        s.store_add_ad(319, A::add(A::offset(A::scale(s.ad_value(477), p.p606), p.p342), A::scale(s.ad_value(478), p.p796)), A::scale(s.ad_value(479), p.p986));

        s.store_add_ad(320, A::add(A::offset(A::scale(s.ad_value(477), p.p607), p.p344), A::scale(s.ad_value(478), p.p797)), A::scale(s.ad_value(479), p.p987));

        s.store_add_ad(321, A::add(A::offset(A::scale(s.ad_value(477), p.p608), p.p345), A::scale(s.ad_value(478), p.p798)), A::scale(s.ad_value(479), p.p988));

        s.store_add_ad(355, A::add(A::offset(A::scale(s.ad_value(477), p.p609), p.p346), A::scale(s.ad_value(478), p.p799)), A::scale(s.ad_value(479), p.p989));

        s.store_add_ad(356, A::add(A::offset(A::scale(s.ad_value(477), p.p610), p.p347), A::scale(s.ad_value(478), p.p800)), A::scale(s.ad_value(479), p.p990));

        s.store_add_ad(242, A::add(A::offset(A::scale(s.ad_value(477), p.p443), p.p157), A::scale(s.ad_value(478), p.p633)), A::scale(s.ad_value(479), p.p823));

        s.store_add_ad(243, A::add(A::offset(A::scale(s.ad_value(477), p.p444), p.p383), A::scale(s.ad_value(478), p.p634)), A::scale(s.ad_value(479), p.p824));

        s.store_add_ad(244, A::add(A::offset(A::scale(s.ad_value(477), p.p445), p.p384), A::scale(s.ad_value(478), p.p635)), A::scale(s.ad_value(479), p.p825));

        s.store_add_ad(246, A::add(A::offset(A::scale(s.ad_value(477), p.p447), p.p388), A::scale(s.ad_value(478), p.p637)), A::scale(s.ad_value(479), p.p827));

        s.store_add_ad(247, A::add(A::offset(A::scale(s.ad_value(477), p.p448), p.p389), A::scale(s.ad_value(478), p.p638)), A::scale(s.ad_value(479), p.p828));

        s.store_add_ad(245, A::add(A::offset(A::scale(s.ad_value(477), p.p446), p.p385), A::scale(s.ad_value(478), p.p636)), A::scale(s.ad_value(479), p.p826));

        s.store_add_ad(249, A::add(A::offset(A::scale(s.ad_value(477), p.p449), p.p390), A::scale(s.ad_value(478), p.p639)), A::scale(s.ad_value(479), p.p829));

        s.store_add_ad(253, A::add(A::offset(A::scale(s.ad_value(477), p.p457), p.p352), A::scale(s.ad_value(478), p.p647)), A::scale(s.ad_value(479), p.p837));

        s.store_add_ad(254, A::add(A::offset(A::scale(s.ad_value(477), p.p467), p.p358), A::scale(s.ad_value(478), p.p657)), A::scale(s.ad_value(479), p.p847));

        s.store_add_ad(255, A::add(A::offset(A::scale(s.ad_value(477), p.p468), p.p359), A::scale(s.ad_value(478), p.p658)), A::scale(s.ad_value(479), p.p848));

        s.store_add_ad(256, A::add(A::offset(A::scale(s.ad_value(477), p.p469), p.p174), A::scale(s.ad_value(478), p.p659)), A::scale(s.ad_value(479), p.p849));

        s.store_add_ad(257, A::add(A::offset(A::scale(s.ad_value(477), p.p470), p.p175), A::scale(s.ad_value(478), p.p660)), A::scale(s.ad_value(479), p.p850));

        s.store_add_ad(258, A::add(A::offset(A::scale(s.ad_value(477), p.p471), p.p176), A::scale(s.ad_value(478), p.p661)), A::scale(s.ad_value(479), p.p851));

        s.store_add_ad(259, A::add(A::offset(A::scale(s.ad_value(477), p.p472), p.p177), A::scale(s.ad_value(478), p.p662)), A::scale(s.ad_value(479), p.p852));

        s.store_add_ad(260, A::add(A::offset(A::scale(s.ad_value(477), p.p473), p.p178), A::scale(s.ad_value(478), p.p663)), A::scale(s.ad_value(479), p.p853));

        s.store_add_ad(261, A::add(A::offset(A::scale(s.ad_value(477), p.p474), p.p179), A::scale(s.ad_value(478), p.p664)), A::scale(s.ad_value(479), p.p854));

        s.store_add_ad(262, A::add(A::offset(A::scale(s.ad_value(477), p.p475), p.p180), A::scale(s.ad_value(478), p.p665)), A::scale(s.ad_value(479), p.p855));

        s.store_add_ad(237, A::add(A::offset(A::scale(s.ad_value(477), p.p455), p.p211), A::scale(s.ad_value(478), p.p645)), A::scale(s.ad_value(479), p.p835));

        s.store_add_ad(236, A::add(A::offset(A::scale(s.ad_value(477), p.p454), p.p210), A::scale(s.ad_value(478), p.p644)), A::scale(s.ad_value(479), p.p834));

        s.store_add_ad(238, A::add(A::offset(A::scale(s.ad_value(477), p.p456), p.p212), A::scale(s.ad_value(478), p.p646)), A::scale(s.ad_value(479), p.p836));

        s.store_add_ad(145, A::add(A::offset(A::scale(s.ad_value(477), p.p458), p.p118), A::scale(s.ad_value(478), p.p648)), A::scale(s.ad_value(479), p.p838));

        s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(477), p.p514), p.p121), A::scale(s.ad_value(478), p.p704)), A::scale(s.ad_value(479), p.p894));

        s.store_add_ad(147, A::add(A::offset(A::scale(s.ad_value(477), p.p515), p.p122), A::scale(s.ad_value(478), p.p705)), A::scale(s.ad_value(479), p.p895));

        s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(477), p.p510), p.p117), A::scale(s.ad_value(478), p.p700)), A::scale(s.ad_value(479), p.p890));

        s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(477), p.p517), p.p119), A::scale(s.ad_value(478), p.p707)), A::scale(s.ad_value(479), p.p897));

        s.store_add_ad(150, A::add(A::offset(A::scale(s.ad_value(477), p.p516), p.p120), A::scale(s.ad_value(478), p.p706)), A::scale(s.ad_value(479), p.p896));

        s.store_add_ad(121, A::add(A::offset(A::scale(s.ad_value(477), p.p459), p.p91), A::scale(s.ad_value(478), p.p649)), A::scale(s.ad_value(479), p.p839));

        s.store_add_ad(123, A::add(A::offset(A::scale(s.ad_value(477), p.p461), p.p93), A::scale(s.ad_value(478), p.p651)), A::scale(s.ad_value(479), p.p841));

        s.store_add_ad(122, A::add(A::offset(A::scale(s.ad_value(477), p.p460), p.p92), A::scale(s.ad_value(478), p.p650)), A::scale(s.ad_value(479), p.p840));

        s.store_add_ad(139, A::add(A::offset(A::scale(s.ad_value(477), p.p462), p.p111), A::scale(s.ad_value(478), p.p652)), A::scale(s.ad_value(479), p.p842));

        s.store_add_ad(141, A::add(A::offset(A::scale(s.ad_value(477), p.p463), p.p113), A::scale(s.ad_value(478), p.p653)), A::scale(s.ad_value(479), p.p843));

        s.store_add_ad(143, A::add(A::offset(A::scale(s.ad_value(477), p.p464), p.p115), A::scale(s.ad_value(478), p.p654)), A::scale(s.ad_value(479), p.p844));

        s.store_add_ad(102, A::add(A::offset(A::scale(s.ad_value(477), p.p465), p.p75), A::scale(s.ad_value(478), p.p655)), A::scale(s.ad_value(479), p.p845));

        s.store_add_ad(186, A::add(A::offset(A::scale(s.ad_value(477), p.p466), p.p144), A::scale(s.ad_value(478), p.p656)), A::scale(s.ad_value(479), p.p846));

        s.store_add_ad(211, A::add(A::offset(A::scale(s.ad_value(477), p.p484), p.p406), A::scale(s.ad_value(478), p.p674)), A::scale(s.ad_value(479), p.p864));

        s.store_add_ad(203, A::add(A::offset(A::scale(s.ad_value(477), p.p476), p.p398), A::scale(s.ad_value(478), p.p666)), A::scale(s.ad_value(479), p.p856));

        s.store_add_ad(204, A::add(A::offset(A::scale(s.ad_value(477), p.p477), p.p399), A::scale(s.ad_value(478), p.p667)), A::scale(s.ad_value(479), p.p857));

        s.store_add_ad(205, A::add(A::offset(A::scale(s.ad_value(477), p.p478), p.p400), A::scale(s.ad_value(478), p.p668)), A::scale(s.ad_value(479), p.p858));

        s.store_add_ad(206, A::add(A::offset(A::scale(s.ad_value(477), p.p479), p.p401), A::scale(s.ad_value(478), p.p669)), A::scale(s.ad_value(479), p.p859));

        s.store_add_ad(207, A::add(A::offset(A::scale(s.ad_value(477), p.p480), p.p402), A::scale(s.ad_value(478), p.p670)), A::scale(s.ad_value(479), p.p860));

        s.store_add_ad(208, A::add(A::offset(A::scale(s.ad_value(477), p.p481), p.p403), A::scale(s.ad_value(478), p.p671)), A::scale(s.ad_value(479), p.p861));

        s.store_add_ad(209, A::add(A::offset(A::scale(s.ad_value(477), p.p482), p.p404), A::scale(s.ad_value(478), p.p672)), A::scale(s.ad_value(479), p.p862));

        s.store_add_ad(210, A::add(A::offset(A::scale(s.ad_value(477), p.p483), p.p405), A::scale(s.ad_value(478), p.p673)), A::scale(s.ad_value(479), p.p863));

        s.store_add_ad(212, A::add(A::offset(A::scale(s.ad_value(477), p.p485), p.p407), A::scale(s.ad_value(478), p.p675)), A::scale(s.ad_value(479), p.p865));

        s.store_add_ad(213, A::add(A::offset(A::scale(s.ad_value(477), p.p486), p.p408), A::scale(s.ad_value(478), p.p676)), A::scale(s.ad_value(479), p.p866));

        s.store_add_ad(214, A::add(A::offset(A::scale(s.ad_value(477), p.p487), p.p409), A::scale(s.ad_value(478), p.p677)), A::scale(s.ad_value(479), p.p867));

        s.store_add_ad(229, A::add(A::offset(A::scale(s.ad_value(477), p.p618), p.p422), A::scale(s.ad_value(478), p.p808)), A::scale(s.ad_value(479), p.p998));

        s.store_add_ad(230, A::add(A::offset(A::scale(s.ad_value(477), p.p619), p.p423), A::scale(s.ad_value(478), p.p809)), A::scale(s.ad_value(479), p.p999));

        s.store_add_ad(216, A::add(A::offset(A::scale(s.ad_value(477), p.p620), p.p413), A::scale(s.ad_value(478), p.p810)), A::scale(s.ad_value(479), p.p1000));

        s.store_add_ad(217, A::add(A::offset(A::scale(s.ad_value(477), p.p621), p.p433), A::scale(s.ad_value(478), p.p811)), A::scale(s.ad_value(479), p.p1001));

        s.store_add_ad(218, A::add(A::offset(A::scale(s.ad_value(477), p.p622), p.p434), A::scale(s.ad_value(478), p.p812)), A::scale(s.ad_value(479), p.p1002));

        s.store_add_ad(219, A::add(A::offset(A::scale(s.ad_value(477), p.p623), p.p414), A::scale(s.ad_value(478), p.p813)), A::scale(s.ad_value(479), p.p1003));

        s.store_add_ad(220, A::add(A::offset(A::scale(s.ad_value(477), p.p624), p.p415), A::scale(s.ad_value(478), p.p814)), A::scale(s.ad_value(479), p.p1004));

        s.store_add_ad(221, A::add(A::offset(A::scale(s.ad_value(477), p.p625), p.p416), A::scale(s.ad_value(478), p.p815)), A::scale(s.ad_value(479), p.p1005));

        s.store_add_ad(222, A::add(A::offset(A::scale(s.ad_value(477), p.p626), p.p417), A::scale(s.ad_value(478), p.p816)), A::scale(s.ad_value(479), p.p1006));

        s.store_add_ad(223, A::add(A::offset(A::scale(s.ad_value(477), p.p627), p.p418), A::scale(s.ad_value(478), p.p817)), A::scale(s.ad_value(479), p.p1007));

        s.store_add_ad(224, A::add(A::offset(A::scale(s.ad_value(477), p.p628), p.p419), A::scale(s.ad_value(478), p.p818)), A::scale(s.ad_value(479), p.p1008));

        s.store_add_ad(225, A::add(A::offset(A::scale(s.ad_value(477), p.p629), p.p420), A::scale(s.ad_value(478), p.p819)), A::scale(s.ad_value(479), p.p1009));

        s.store_add_ad(226, A::add(A::offset(A::scale(s.ad_value(477), p.p630), p.p421), A::scale(s.ad_value(478), p.p820)), A::scale(s.ad_value(479), p.p1010));

        s.store_add_ad(227, A::add(A::offset(A::scale(s.ad_value(477), p.p631), p.p411), A::scale(s.ad_value(478), p.p821)), A::scale(s.ad_value(479), p.p1011));

        s.store_add_ad(228, A::add(A::offset(A::scale(s.ad_value(477), p.p632), p.p412), A::scale(s.ad_value(478), p.p822)), A::scale(s.ad_value(479), p.p1012));

        s.store_add_ad(322, A::add(A::offset(A::scale(s.ad_value(477), p.p611), p.p353), A::scale(s.ad_value(478), p.p801)), A::scale(s.ad_value(479), p.p991));

        s.store_add_ad(323, A::add(A::offset(A::scale(s.ad_value(477), p.p612), p.p354), A::scale(s.ad_value(478), p.p802)), A::scale(s.ad_value(479), p.p992));

        s.store_add_ad(324, A::add(A::offset(A::scale(s.ad_value(477), p.p613), p.p370), A::scale(s.ad_value(478), p.p803)), A::scale(s.ad_value(479), p.p993));

        s.store_add_ad(361, A::add(A::offset(A::scale(s.ad_value(477), p.p614), p.p366), A::scale(s.ad_value(478), p.p804)), A::scale(s.ad_value(479), p.p994));

        s.store_mul_ad_rhs(361, 361, A::powf(A::scale(s.ad_value(108), 5e-17), (-0.25)));

        s.store_add_ad(362, A::add(A::offset(A::scale(s.ad_value(477), p.p615), p.p367), A::scale(s.ad_value(478), p.p805)), A::scale(s.ad_value(479), p.p995));

        s.store_add_ad(363, A::add(A::offset(A::scale(s.ad_value(477), p.p616), p.p368), A::scale(s.ad_value(478), p.p806)), A::scale(s.ad_value(479), p.p996));

        s.store_add_ad(364, A::add(A::offset(A::scale(s.ad_value(477), p.p617), p.p369), A::scale(s.ad_value(478), p.p807)), A::scale(s.ad_value(479), p.p997));

        s.store_add_ad(378, A::add(A::offset(A::scale(s.ad_value(477), p.p259), p.p258), A::scale(s.ad_value(478), p.p260)), A::scale(s.ad_value(479), p.p261));

        s.store_add_ad(379, A::add(A::offset(A::scale(s.ad_value(477), p.p263), p.p262), A::scale(s.ad_value(478), p.p264)), A::scale(s.ad_value(479), p.p265));

        s.store_add_ad(380, A::add(A::offset(A::scale(s.ad_value(477), p.p267), p.p266), A::scale(s.ad_value(478), p.p268)), A::scale(s.ad_value(479), p.p269));

        s.store_add_ad(381, A::add(A::offset(A::scale(s.ad_value(477), p.p271), p.p270), A::scale(s.ad_value(478), p.p272)), A::scale(s.ad_value(479), p.p273));

        s.store_add_ad(382, A::add(A::offset(A::scale(s.ad_value(477), p.p275), p.p274), A::scale(s.ad_value(478), p.p276)), A::scale(s.ad_value(479), p.p277));

        s.store_add_ad(383, A::add(A::offset(A::scale(s.ad_value(477), p.p279), p.p278), A::scale(s.ad_value(478), p.p280)), A::scale(s.ad_value(479), p.p281));

        s.store_add_ad(389, A::add(A::offset(A::scale(s.ad_value(477), p.p436), p.p435), A::scale(s.ad_value(478), p.p437)), A::scale(s.ad_value(479), p.p438));

        s.store_add_ad(390, A::add(A::offset(A::scale(s.ad_value(477), p.p440), p.p439), A::scale(s.ad_value(478), p.p441)), A::scale(s.ad_value(479), p.p442));

        s.store_add_ad(385, A::add(A::offset(A::scale(s.ad_value(477), p.p286), p.p285), A::scale(s.ad_value(478), p.p289)), A::scale(s.ad_value(479), p.p292));

        s.store_add_ad(386, A::add(A::offset(A::scale(s.ad_value(477), p.p287), p.p282), A::scale(s.ad_value(478), p.p290)), A::scale(s.ad_value(479), p.p293));

        s.store_add_ad(387, A::add(A::offset(A::scale(s.ad_value(477), p.p288), p.p284), A::scale(s.ad_value(478), p.p291)), A::scale(s.ad_value(479), p.p294));

        s.store_add_ad(250, A::add(A::offset(A::scale(s.ad_value(477), p.p450), p.p392), A::scale(s.ad_value(478), p.p640)), A::scale(s.ad_value(479), p.p830));

        s.store_add_ad(248, A::add(A::offset(A::scale(s.ad_value(477), p.p451), p.p393), A::scale(s.ad_value(478), p.p641)), A::scale(s.ad_value(479), p.p831));

        s.store_add_ad(251, A::add(A::offset(A::scale(s.ad_value(477), p.p452), p.p394), A::scale(s.ad_value(478), p.p642)), A::scale(s.ad_value(479), p.p832));

        s.store_add_ad(252, A::add(A::offset(A::scale(s.ad_value(477), p.p453), p.p395), A::scale(s.ad_value(478), p.p643)), A::scale(s.ad_value(479), p.p833));

        s.store_offset_ad(384, A::scale(A::atan(s.ad_value(383)), 0.3183098861837907), 0.5);

        s.store_offset_ad(388, A::scale(A::atan(s.ad_value(389)), 0.3183098861837907), 0.5);

        s.v[430] = (s.v[476] - 1.0);

        s.copy_ad(153, 138);

        s.copy_ad(154, 140);

        s.copy_ad(155, 142);

        s.store_ad(159, &A::pow_from_scalar((s.v[328] * 1000000.0), s.ad_value(196)));

        s.v[157] = ((p.p14 / (p.p3 * (s.v[328] + p.p377))) * p.p23);

        s.v[158] = ((p.p15 * (p.p3 * (s.v[328] + p.p377))) / p.p23);

        s.v[547] = if (s.v[38] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[547] != 0.0) {
            s.store_scalar(156, 0.0);
        }

        if (!(s.v[547] != 0.0)) {
            s.store_scale_ad(156, A::scale(A::div(A::scale(s.ad_value(38), (p.p17 * p.p378)), A::offset(A::scale(s.ad_value(38), 2.0), (p.p378 * s.v[327]))), (s.v[328] * 1.0 / (p.p23))), 1.0 / (p.p3));
        }

        s.v[345] = (((((p.p380 / p.p376)) as f64).powf(p.p379) / p.p376) / p.p376);

        s.store_add_ad_rhs(138, 138, A::scale(s.ad_value(139), s.v[430]));

        s.store_add_ad_rhs(140, 140, A::scale(s.ad_value(141), s.v[430]));

        s.store_add_ad_rhs(142, 142, A::scale(s.ad_value(143), s.v[430]));

        s.v[548] = if (s.v[144] > 1.0) { 1.0 } else { 0.0 };

        if (s.v[548] != 0.0) {
            s.store_scale(144, 144, 0.0001);
        }

        s.store_mul_ad_rhs(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));

        s.store_sub_ad_rhs(338, 101, A::scale(s.ad_value(102), s.v[430]));

        s.store_div_ad_lhs(182, A::add(s.ad_value(181), A::scale(s.ad_value(186), s.v[430])), 159);

        s.v[549] = if (p.p429 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[549] != 0.0) {
            s.store_scale(496, 159, p.p3);
        }

        if (s.v[549] != 0.0) {
            s.store_scale(497, 186, s.v[430]);
        }

        if (s.v[549] != 0.0) {
            s.store_add(468, 169, 497);
        }

        if (s.v[549] != 0.0) {
            s.store_offset(469, 497, p.p140);
        }

        s.v[550] = if (s.v[468] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[549] != 0.0) && (s.v[550] != 0.0)) {
            s.store_scalar(468, 0.0);
        }

        s.v[551] = if (s.v[469] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[549] != 0.0) && (s.v[551] != 0.0)) {
            s.store_scalar(469, 0.0);
        }

        if (s.v[549] != 0.0) {
            s.store_div(173, 468, 496);
        }

        if (s.v[549] != 0.0) {
            s.store_div(171, 469, 496);
        }

        if (s.v[549] != 0.0) {
            s.store_add(470, 170, 497);
        }

        if (s.v[549] != 0.0) {
            s.store_offset(471, 497, p.p139);
        }

        s.v[552] = if (s.v[470] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[549] != 0.0) && (s.v[552] != 0.0)) {
            s.store_scalar(470, 0.0);
        }

        s.v[553] = if (s.v[471] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[549] != 0.0) && (s.v[553] != 0.0)) {
            s.store_scalar(471, 0.0);
        }

        if (s.v[549] != 0.0) {
            s.store_div(174, 470, 496);
        }

        if (s.v[549] != 0.0) {
            s.store_div(172, 471, 496);
        }

        if (!(s.v[549] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (!(s.v[549] != 0.0)) {
            s.store_scalar(171, 0.0);
        }

        if (!(s.v[549] != 0.0)) {
            s.store_scalar(174, 0.0);
        }

        if (!(s.v[549] != 0.0)) {
            s.store_scalar(172, 0.0);
        }

        s.v[554] = if self.param_given[128] { 1.0 } else { 0.0 };

        if (s.v[554] != 0.0) {
            s.store_scalar(47, p.p128);
        }

        s.v[555] = if ((if self.param_given[217] { 1.0 } else { 0.0 } != 0.0) && (p.p217 > 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[554] != 0.0)) && (s.v[555] != 0.0)) {
            s.store_sub_ad_lhs(47, A::scale(s.ad_value(396), p.p217), 237);
        }

        if ((!(s.v[554] != 0.0)) && (!(s.v[555] != 0.0))) {
            s.store_scale(47, 396, (0.6 * p.p157));
        }

        s.v[556] = if self.param_given[127] { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_scalar(40, p.p127);
        }

        s.v[557] = if ((if self.param_given[217] { 1.0 } else { 0.0 } != 0.0) && (p.p217 > 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[556] != 0.0)) && (s.v[557] != 0.0)) {
            s.store_sub_ad_lhs(40, A::scale(s.ad_value(396), p.p217), 236);
        }

        if ((!(s.v[556] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_scale(40, 396, (0.6 * p.p157));
        }

        s.v[558] = if (s.v[47] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[558] != 0.0) {
            s.store_scalar(47, 0.0);
        }

        s.v[559] = if (s.v[40] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[559] != 0.0) {
            s.store_scalar(40, 0.0);
        }

        s.v[560] = if (s.v[42] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[560] != 0.0) {
            s.store_scalar(42, 0.0);
        }

        s.store_scaled_add(335, 47, 239, s.v[349]);

        s.store_scaled_add(334, 40, 239, s.v[350]);

        s.store_scale(336, 42, (s.v[331] * p.p3));

        s.v[561] = if ((!(if self.param_given[82] { 1.0 } else { 0.0 } != 0.0)) && (if self.param_given[85] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_scale(467, 396, s.v[112]);
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(467), 3.021e22), 467);
        }

        s.v[562] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[562] != 0.0) && (p.p41 != 0.0)) {
            s.store_scale(422, 417, ((((p.p49 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p156 * p.p156))));
        }

        s.v[563] = if (s.v[108] > s.v[422]) { 1.0 } else { 0.0 };

        if (((s.v[562] != 0.0) && (p.p41 != 0.0)) && (s.v[563] != 0.0)) {
            s.copy_ad(108, 422);
        }

        if ((s.v[562] != 0.0) && (!(p.p41 != 0.0))) {
            s.store_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p155 * p.p155))));
        }

        s.v[564] = if (s.v[108] > s.v[422]) { 1.0 } else { 0.0 };

        if (((s.v[562] != 0.0) && (!(p.p41 != 0.0))) && (s.v[564] != 0.0)) {
            s.copy_ad(108, 422);
        }

        s.v[392] = (3.453133e-11 / p.p154);

        if (p.p41 != 0.0) {
            s.store_scalar(393, (1.03594e-10 / p.p156));
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(393, (1.03594e-10 / p.p155));
        }

        if (p.p41 != 0.0) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p156))));
        }

        if (!(p.p41 != 0.0)) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p155))));
        }

        s.store_add_ad_lhs(421, A::sub_from_scalar(0.8, A::div(A::scale(s.ad_value(420), 0.5), s.ad_value(393))), 216);

        s.v[565] = if (s.v[37] == 3.0) { 1.0 } else { 0.0 };

        s.v[566] = if (s.v[421] > s.v[228]) { 1.0 } else { 0.0 };

        if ((s.v[565] != 0.0) && (s.v[566] != 0.0)) {
            s.store_scalar(37, 2.0);
        }

        s.v[567] = if (s.v[421] < s.v[227]) { 1.0 } else { 0.0 };

        if (((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[567] != 0.0)) {
            s.store_scalar(37, 0.0);
        }

        if (((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[567] != 0.0))) {
            s.store_scalar(37, 1.0);
        }

        s.store_scale_ad(471, A::div_from_scalar(1.115, s.ad_value(49)), s.v[430]);

        s.store_div_ad_lhs(532, A::mul(s.ad_value(256), s.ad_value(471)), 300);

        s.v[568] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[568] != 0.0) {
            s.store_scale_ad(467, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[569] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[568] != 0.0)) && (s.v[569] != 0.0)) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!(s.v[568] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_exp(467, 532);
        }

        s.store_div_ad_lhs(532, A::mul(s.ad_value(257), s.ad_value(471)), 300);

        s.v[570] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[570] != 0.0) {
            s.store_scale_ad(468, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[571] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!(s.v[570] != 0.0)) && (!(s.v[571] != 0.0))) {
            s.store_exp(468, 532);
        }

        s.store_div_ad_lhs(532, A::mul(s.ad_value(258), s.ad_value(471)), 302);

        s.v[572] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[572] != 0.0) {
            s.store_scale_ad(469, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[573] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[572] != 0.0)) && (s.v[573] != 0.0)) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!(s.v[572] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_exp(469, 532);
        }

        s.store_mul(357, 355, 467);

        s.store_mul(161, 306, 467);

        s.store_mul(163, 308, 468);

        s.store_mul(165, 310, 469);

        s.store_scale(532, 259, s.v[430]);

        s.v[574] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[574] != 0.0) {
            s.store_scale_ad(467, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[575] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[574] != 0.0)) && (s.v[575] != 0.0)) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!(s.v[574] != 0.0)) && (!(s.v[575] != 0.0))) {
            s.store_exp(467, 532);
        }

        s.store_mul(167, 312, 467);

        s.store_div_ad_lhs(532, A::mul(s.ad_value(256), s.ad_value(471)), 301);

        s.v[576] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[576] != 0.0) {
            s.store_scale_ad(467, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[577] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!(s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_exp(467, 532);
        }

        s.store_div_ad_lhs(532, A::mul(s.ad_value(260), s.ad_value(471)), 301);

        s.v[578] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[578] != 0.0) {
            s.store_scale_ad(468, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[579] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[578] != 0.0)) && (s.v[579] != 0.0)) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!(s.v[578] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_exp(468, 532);
        }

        s.store_div_ad_lhs(532, A::mul(s.ad_value(261), s.ad_value(471)), 303);

        s.v[580] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[580] != 0.0) {
            s.store_scale_ad(469, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[581] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!(s.v[580] != 0.0)) && (!(s.v[581] != 0.0))) {
            s.store_exp(469, 532);
        }

        s.store_mul(358, 356, 467);

        s.store_mul(162, 307, 467);

        s.store_mul(164, 309, 468);

        s.store_mul(166, 311, 469);

        s.store_scale(532, 262, s.v[430]);

        s.v[582] = if (s.v[532] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[582] != 0.0) {
            s.store_scale_ad(467, A::offset(A::offset(s.ad_value(532), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[583] = if (s.v[532] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[582] != 0.0)) && (s.v[583] != 0.0)) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!(s.v[582] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_exp(467, 532);
        }

        s.store_mul(168, 313, 467);

        s.v[584] = if (s.v[109] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[584] != 0.0) {
            s.store_ad(160, &A::mul(A::scale(s.ad_value(49), (-p.p37)), {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!(s.v[584] != 0.0)) {
            s.store_mul_ad(160, A::scale(s.ad_value(49), (-p.p37)), A::sub({
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul(A::neg(s.ad_value(108)), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, A::scale(s.ad_value(530), 2.0)));
        }

        s.v[585] = if !(if self.param_given[353] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        s.v[586] = if (s.v[109] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
            s.store_scale_ad(322, A::offset(A::sub(A::mul(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln(A::scale(s.ad_value(109), 1e20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), A::mul(A::scale(s.ad_value(49), 2.0), s.ad_value(530))), (-0.3)), (-p.p37));
        }

        s.v[587] = if (s.v[109] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[585] != 0.0) && (!(s.v[586] != 0.0))) && (s.v[587] != 0.0)) {
            s.store_scale_ad(322, A::offset(A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3), (-p.p37));
        }

        s.store_mul_ad(481, A::scale(s.ad_value(49), 2.0), A::sub({
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.ad_value(530)));

        s.store_scale_ad(482, A::mul(s.ad_value(419), A::sqrt(A::abs(s.ad_value(109)))), 1.0 / (s.v[392]));

        s.v[588] = if !(if self.param_given[354] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        s.v[589] = if (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[588] != 0.0) && (s.v[589] != 0.0)) {
            s.store_add_ad(323, A::add(s.ad_value(322), s.ad_value(481)), A::mul(s.ad_value(482), A::sqrt(s.ad_value(481))));
        }

        if ((s.v[588] != 0.0) && (!(s.v[589] != 0.0))) {
            s.store_sub_ad(323, A::sub(s.ad_value(322), s.ad_value(481)), A::mul(s.ad_value(482), A::sqrt(s.ad_value(481))));
        }

        s.v[590] = if !(if self.param_given[355] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[590] != 0.0) {
            s.store_sqrt_ad(462, A::div(A::mul(A::scale(s.ad_value(417), 2.0), s.ad_value(481)), A::scale(A::abs(s.ad_value(109)), (1.602176462e-19 * 1000000.0))));
        }

        if (s.v[590] != 0.0) {
            s.store_div(463, 417, 462);
        }

        if (s.v[590] != 0.0) {
            s.store_div_ad(43, A::scale(s.ad_value(463), s.v[392]), A::offset(s.ad_value(463), s.v[392]));
        }

        s.store_mul_ad(118, A::scale(s.ad_value(49), 2.0), A::sub({
            if (s.v[108] > 1e-38) {
                A::ln(s.ad_value(108))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.ad_value(530)));

        s.store_sqrt(339, 118);

        s.store_mul_ad_lhs(340, A::sqrt(A::div(A::scale(s.ad_value(417), 2.0), A::scale(s.ad_value(108), (1.602176462e-19 * 1000000.0)))), 339);

        s.store_sqrt(341, 340);

        s.v[591] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[591] != 0.0) {
            s.store_sqrt_ad(119, A::scale(A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p.p66));
        }

        if (!(s.v[591] != 0.0)) {
            s.store_sqrt_ad(119, A::div(A::mul(A::mul(s.ad_value(417), s.ad_value(242)), s.ad_value(415)), A::scale(s.ad_value(416), 8.85418e-12)));
        }

        s.store_mul_ad_rhs(115, 49, A::sub({
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln(A::scale(s.ad_value(108), 1e20))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, A::scale(s.ad_value(530), 2.0)));

        s.store_sqrt_ad(367, A::div(A::scale(A::mul(A::scale(s.ad_value(417), 1.602176462e-19), s.ad_value(108)), (1000000.0 * 0.5)), s.ad_value(118)));

        s.v[592] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        s.v[593] = if (s.v[110] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_mul_ad_rhs(375, 480, {
                if ((s.v[110] / 1e20) > 1e-38) {
                    A::ln(A::scale(s.ad_value(110), 1e-20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) {
            s.store_scalar(375, 0.0);
        }

        if (!(s.v[592] != 0.0)) {
            s.store_mul_ad_rhs(467, 480, A::sub({
                if (s.v[111] > 1e-38) {
                    A::ln(s.ad_value(111))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, s.ad_value(530)));
        }

        if (!(s.v[592] != 0.0)) {
            s.store_scale(468, 466, 0.5);
        }

        s.v[594] = if (s.v[467] > s.v[468]) { 1.0 } else { 0.0 };

        if ((!(s.v[592] != 0.0)) && (s.v[594] != 0.0)) {
            s.copy_ad(467, 468);
        }

        if (!(s.v[592] != 0.0)) {
            s.store_sub_ad(469, A::offset(s.ad_value(468), p.p53), A::scale(s.ad_value(467), p.p37));
        }

        if (!(s.v[592] != 0.0)) {
            s.store_sub_from_scalar(375, p.p52, 469);
        }

        s.v[368] = (((((p.p379 * (if ((p.p380 / p.p376) > 1e-38) { (((p.p380 / p.p376)) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p.p376) / p.p376);

        s.store_div_ad_lhs(371, A::div(A::scale(A::scale(A::exp(A::scale({
            if ((p.p380 / (p.p376 * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p.p380, A::scale(s.ad_value(213), p.p376)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p379)), 1.0 / (p.p376)), 1.0 / (p.p376)), s.ad_value(213)), 213);

        s.v[369] = (if (p.p37 == 1.0) { p.p1040 } else { p.p1039 });

        s.v[370] = (if (p.p37 == 1.0) { p.p1042 } else { p.p1041 });

        s.store_mul_ad_lhs(372, A::scale(s.ad_value(215), (s.v[369] * ((s.v[328] / p.p23) + p.p25))), 371);

        s.store_mul_ad_lhs(373, A::scale(s.ad_value(215), (s.v[369] * ((s.v[328] / p.p23) + p.p24))), 371);

        s.store_scale(374, 213, ((-s.v[370]) * p.p376));

        s.v[369] = ((s.v[369] * s.v[368]) * (((s.v[328] / p.p23) * s.v[327]) + (p.p28 / p.p3)));

        s.v[370] = (s.v[370] * (-p.p376));

        s.v[595] = if ((if self.param_given[90] { 1.0 } else { 0.0 } != 0.0) || (if self.param_given[94] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        s.v[596] = if !(if self.param_given[90] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_scalar(120, 0.53);
        }

        s.v[597] = if !(if self.param_given[94] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[597] != 0.0)) {
            s.store_scalar(124, (-0.0186));
        }

        s.v[603] = if !(if self.param_given[87] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (s.v[603] != 0.0)) && (p.p41 != 0.0)) {
            s.store_scale_ad(467, A::div_from_scalar(1.602176462e-19, A::scale(s.ad_value(417), 2.0)), 1000000.0);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[603] != 0.0)) && (!(p.p41 != 0.0))) {
            s.store_scalar(467, 0.00077348);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[603] != 0.0)) {
            s.store_sub_ad_rhs(114, 118, A::scale(A::mul(s.ad_value(467), s.ad_value(108)), (s.v[117] * s.v[117])));
        }

        s.v[604] = if (s.v[114] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[604] != 0.0)) {
            s.store_neg(114, 114);
        }

        s.v[605] = if (s.v[116] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scalar(116, (-s.v[116]));
        }

        s.v[606] = if !(if self.param_given[85] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[606] != 0.0)) {
            s.store_div_ad_lhs(112, A::mul(s.ad_value(419), A::sqrt(s.ad_value(108))), 396);
        }

        s.v[607] = if !(if self.param_given[86] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[607] != 0.0)) {
            s.store_div_ad_lhs(113, A::mul(s.ad_value(419), A::sqrt(s.ad_value(109))), 396);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_sub(467, 112, 113);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_sub_ad_lhs(468, A::sqrt(A::sub(s.ad_value(118), s.ad_value(114))), 339);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_mul_ad_rhs(469, 339, A::sub(A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), s.ad_value(339)));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_div_ad(124, A::mul(s.ad_value(467), s.ad_value(468)), A::add(A::scale(s.ad_value(469), 2.0), s.ad_value(116)));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_sub_ad_rhs(120, 113, A::mul(A::scale(s.ad_value(124), 2.0), A::sqrt(A::sub(s.ad_value(118), s.ad_value(116)))));
        }

        s.store_offset(467, 265, s.v[328]);

        s.v[608] = if (s.v[467] < 1e-8) { 1.0 } else { 0.0 };

        if (s.v[608] != 0.0) {
            s.store_scalar(467, 1e-8);
        }

        s.store_mul_ad_rhs(346, 120, A::offset(A::div(s.ad_value(264), s.ad_value(467)), 1.0));

        s.v[609] = if !(if self.param_given[109] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        s.v[610] = if ((if self.param_given[108] { 1.0 } else { 0.0 } != 0.0) || (if self.param_given[107] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[609] != 0.0) && (s.v[610] != 0.0)) {
            s.store_sub_ad(152, A::sub(A::scale(s.ad_value(137), p.p37), s.ad_value(118)), A::mul(s.ad_value(346), s.ad_value(339)));
        }

        if ((s.v[609] != 0.0) && (!(s.v[610] != 0.0))) {
            s.store_scalar(152, (-1.0));
        }

        s.v[611] = if !(if self.param_given[108] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[611] != 0.0) {
            s.store_scale_ad(137, A::add(A::add(s.ad_value(152), s.ad_value(118)), A::mul(s.ad_value(346), s.ad_value(339))), p.p37);
        }

        s.store_scale(376, 346, (p.p66 * 1.0 / (p.p67)));

        s.store_mul(468, 397, 341);

        s.store_exp_ad(467, A::div(A::scale(s.ad_value(136), ((-0.5) * s.v[327])), s.ad_value(468)));

        s.store_add_ad_rhs(342, 467, A::mul(A::scale(s.ad_value(467), 2.0), s.ad_value(467)));

        s.store_exp_ad(467, A::div(A::scale(s.ad_value(135), ((-0.5) * s.v[327])), s.ad_value(468)));

        s.store_add_ad_rhs(469, 467, A::mul(A::scale(s.ad_value(467), 2.0), s.ad_value(467)));

        s.store_add_ad_lhs(343, A::mul(s.ad_value(192), s.ad_value(469)), 193);

        s.store_div_ad_rhs(391, 380, A::exp(A::scale(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) }))));

        s.v[612] = if (s.v[44] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[612] != 0.0) {
            s.store_scalar(44, 0.0);
        }

        s.v[467] = ((s.v[474]) as f64).powf(p.p239);

        s.store_offset(489, 44, s.v[475]);

        s.store_powf(468, 489, p.p240);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p244, s.ad_value(468)), (p.p243 / s.v[467])), A::div_from_scalar(p.p245, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(231, 463, 1.0);

        s.v[467] = ((s.v[474]) as f64).powf(p.p241);

        s.store_powf(468, 489, p.p242);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p247, s.ad_value(468)), (p.p246 / s.v[467])), A::div_from_scalar(p.p248, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(232, 463, 1.0);

        s.store_sqrt_ad(232, A::offset(A::square(s.ad_value(232)), 1e-9));

        s.store_offset_scaled(233, 231, (1.0 + (p.p238 * s.v[430])), 1e-9);

        s.v[483] = (1.0 / (p.p232 + (0.5 * s.v[474])));

        s.v[484] = (1.0 / (p.p233 + (0.5 * s.v[474])));

        s.v[235] = (s.v[483] + s.v[484]);

        s.store_scale_ad(234, A::div_from_scalar(p.p235, s.ad_value(233)), s.v[235]);

        s.v[613] = if (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0)))) { 1.0 } else { 0.0 };

        if (s.v[613] != 0.0) {
            s.store_scalar(485, 0.0);
        }

        if (s.v[613] != 0.0) {
            s.store_scalar(486, 0.0);
        }

        s.v[614] = if (s.v[45] < (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[613] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(45, (-1.0));
        }

        s.v[615] = if (s.v[45] > 1.0) { 1.0 } else { 0.0 };

        if (((s.v[613] != 0.0) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) {
            s.store_scalar(45, 1.0);
        }

        if (((s.v[613] != 0.0) && (!(s.v[614] != 0.0))) && (!(s.v[615] != 0.0))) {
        }

        if (s.v[613] != 0.0) {
            s.store_scalar(495, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign6090_loop_guard: usize = 0;
        while {
            let assign6090_cond_e7340: f64 = if ((s.v[613] != 0.0) && (s.v[495] < p.p3)) { 1.0 } else { 0.0 };
            assign6090_cond_e7340 != 0.0
        } {
            assign6090_loop_guard += 1;
            assert!(assign6090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[613] != 0.0) {
                s.store_div_from_scalar_ad(616, (1.0 / p.p3), A::offset(A::scale(s.ad_value(495), (p.p6 + s.v[474])), (p.p4 + (0.5 * s.v[474]))));
            }
            if (s.v[613] != 0.0) {
                s.store_div_from_scalar_ad(617, (1.0 / p.p3), A::offset(A::scale(s.ad_value(495), (p.p6 + s.v[474])), (p.p5 + (0.5 * s.v[474]))));
            }
            if (s.v[613] != 0.0) {
                s.store_add(485, 485, 616);
            }
            if (s.v[613] != 0.0) {
                s.store_add(486, 486, 617);
            }
            if (s.v[613] != 0.0) {
                s.store_offset(495, 495, 1.0);
            }
        }

        if (s.v[613] != 0.0) {
            s.store_add(490, 485, 486);
        }

        if (s.v[613] != 0.0) {
            s.copy_ad(51, 490);
        }

        if (s.v[613] != 0.0) {
            s.store_mul_ad_lhs(487, A::div_from_scalar(p.p235, s.ad_value(233)), 490);
        }

        if (s.v[613] != 0.0) {
            s.store_div_ad(467, A::offset(s.ad_value(487), 1.0), A::offset(s.ad_value(234), 1.0));
        }

        if (s.v[613] != 0.0) {
            s.store_mul(404, 337, 467);
        }

        if (s.v[613] != 0.0) {
            s.store_div_ad(468, A::offset(A::mul(s.ad_value(45), s.ad_value(487)), 1.0), A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0));
        }

        if (s.v[613] != 0.0) {
            s.store_mul(407, 338, 468);
        }

        if (s.v[613] != 0.0) {
            s.store_offset(491, 490, (-s.v[235]));
        }

        if (s.v[613] != 0.0) {
            s.store_mul_ad_lhs(488, A::div_from_scalar(p.p237, s.ad_value(232)), 491);
        }

        if (s.v[613] != 0.0) {
            s.store_mul_ad_lhs(492, A::div_from_scalar(p.p249, A::powf(s.ad_value(232), p.p250)), 491);
        }

        if (s.v[613] != 0.0) {
            s.store_mul_ad_lhs(493, A::div_from_scalar(p.p251, A::powf(s.ad_value(232), p.p252)), 491);
        }

        if (s.v[613] != 0.0) {
            s.store_mul_ad_lhs(494, A::div_from_scalar(p.p253, A::powf(s.ad_value(232), p.p254)), 491);
        }

        if (s.v[613] != 0.0) {
            s.store_add(408, 137, 488);
        }

        if (s.v[613] != 0.0) {
            s.store_add(402, 124, 492);
        }

        if (s.v[613] != 0.0) {
            s.store_add(400, 187, 493);
        }

        if (s.v[613] != 0.0) {
            s.store_add(401, 189, 494);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(404, 337);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(408, 137);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(407, 338);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(402, 124);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(400, 187);
        }

        if (!(s.v[613] != 0.0)) {
            s.copy_ad(401, 189);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scalar(51, 0.0);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scalar(45, 0.0);
        }

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.store_offset(408, 408, p.p20);

        s.store_offset(406, 152, (p.p37 * p.p20));

        s.v[52] = (s.v[392] * p.p8);

        s.store_scale(53, 43, p.p8);

        s.v[54] = (s.v[392] * p.p7);

        s.store_scale(55, 43, p.p7);

        s.v[618] = if (s.v[43] > 0.0) { 1.0 } else { 0.0 };

        s.v[619] = if (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_sub(467, 323, 322);
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_add_ad_rhs(175, 322, A::scale(s.ad_value(467), p.p356));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_sub_from_scalar(468, s.v[52], 53);
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_scale(176, 469, 1.0 / (p.p356));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_sub_ad(56, A::scale(A::mul(s.ad_value(467), s.ad_value(468)), ((1.0 + p.p356) * 0.3333333333333333)), A::mul(s.ad_value(53), s.ad_value(322)));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_sub_from_scalar(468, s.v[54], 55);
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_scale(178, 469, 1.0 / (p.p356));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
        }

        if ((s.v[618] != 0.0) && (s.v[619] != 0.0)) {
            s.store_sub_ad(57, A::scale(A::mul(s.ad_value(467), s.ad_value(468)), ((1.0 + p.p356) * 0.3333333333333333)), A::mul(s.ad_value(55), s.ad_value(322)));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_sub(467, 322, 323);
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_add_ad_rhs(175, 323, A::scale(s.ad_value(467), p.p356));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_offset(468, 53, (-s.v[52]));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_scale(176, 469, 1.0 / (p.p356));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_sub_ad(56, A::scale(A::mul(s.ad_value(467), s.ad_value(468)), ((1.0 + p.p356) * 0.3333333333333333)), A::scale(s.ad_value(323), s.v[52]));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_offset(468, 55, (-s.v[54]));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_div_ad_lhs(469, A::div(s.ad_value(468), s.ad_value(467)), 467);
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_scale(178, 469, 1.0 / (p.p356));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
        }

        if ((s.v[618] != 0.0) && (!(s.v[619] != 0.0))) {
            s.store_sub_ad(57, A::scale(A::mul(s.ad_value(467), s.ad_value(468)), ((1.0 + p.p356) * 0.3333333333333333)), A::scale(s.ad_value(323), s.v[54]));
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(175, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(176, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(177, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(56, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(178, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(179, 0.0);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_scalar(57, 0.0);
        }

        s.v[620] = if ((s.v[46] < 1.0) || (s.v[46] > 2.0)) { 1.0 } else { 0.0 };

        if (s.v[620] != 0.0) {
            s.store_scalar(46, 1.0);
        }

        s.store_ad(467, &A::scale({
            if ((s.v[46] * (1.0 + (p.p155 / p.p154))) > 1e-38) {
                A::ln(A::scale(s.ad_value(46), (1.0 + (p.p155 / p.p154))))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p357));

        s.v[468] = (p.p10 - p.p2);

        s.v[621] = if (s.v[468] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[621] != 0.0) {
            s.store_scale(58, 467, s.v[468]);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_scalar(58, 0.0);
        }

        s.v[468] = (p.p9 - p.p2);

        s.v[622] = if (s.v[468] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[622] != 0.0) {
            s.store_scale(59, 467, s.v[468]);
        }

        if (!(s.v[622] != 0.0)) {
            s.store_scalar(59, 0.0);
        }

        s.v[61] = (p.p131 * p.p11);

        s.v[623] = if ((p.p429 == 1.0) && (s.v[61] < p.p431)) { 1.0 } else { 0.0 };

        if (s.v[623] != 0.0) {
            s.store_scalar(61, p.p431);
        }

        s.v[60] = (p.p131 * p.p12);

        s.v[624] = if ((p.p429 == 1.0) && (s.v[60] < p.p431)) { 1.0 } else { 0.0 };

        if (s.v[624] != 0.0) {
            s.store_scalar(60, p.p431);
        }

        s.v[625] = if (s.v[36] < 1e-15) { 1.0 } else { 0.0 };

        if (s.v[625] != 0.0) {
            s.store_scalar(36, 1e-15);
        }

        s.store_div_ad_lhs(467, A::div_from_scalar((((-0.5) * s.v[327]) * s.v[327]), s.ad_value(36)), 36);

        s.v[626] = if (s.v[467] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[626] != 0.0) {
            s.store_scale_ad(468, A::offset(A::offset(s.ad_value(467), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[627] = if (s.v[467] < (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[626] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!(s.v[626] != 0.0)) && (!(s.v[627] != 0.0))) {
            s.store_exp(468, 467);
        }

        s.copy_ad(351, 468);

        s.store_mul_ad_rhs(467, 319, A::offset(A::div_from_scalar(1.0, s.ad_value(36)), (1.0 / s.v[327])));

        s.store_ad(352, &A::pow(s.ad_value(467), s.ad_value(318)));

        s.store_offset_ad(353, A::scale(A::pow(s.ad_value(467), s.ad_value(253)), p.p343), 1.0);

        s.store_add_ad_rhs(354, 320, A::scale(s.ad_value(321), s.v[327]));

        s.v[628] = if (s.v[354] < 1.0) { 1.0 } else { 0.0 };

        if (s.v[628] != 0.0) {
            s.store_scalar(354, 1.0);
        }

        s.v[629] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[629] != 0.0) {
            s.store_scalar(62, (p.p66 - p.p68));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(498, (8.617087e-5 * p.p57));
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(499, 498);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul_ad_rhs(500, 498, A::sub({
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln(A::scale(s.ad_value(108), 1e20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, A::scale(s.ad_value(530), 2.0)));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul_ad(501, A::scale(s.ad_value(498), 2.0), A::sub({
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, s.ad_value(530)));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sqrt(502, 501);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_add(464, 406, 501);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(503, (p.p37 * p.p56));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(467, (p.p60 * 8.85418e-12));
        }

        s.v[630] = if ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_div_ad(468, A::mul(A::scale(s.ad_value(417), (1000000.0 * 1.602176462e-19)), s.ad_value(110)), A::square(s.ad_value(396)));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(471, A::offset(A::div(A::scale(A::sub(s.ad_value(503), s.ad_value(467)), 2.0), s.ad_value(468)), 1.0));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_mul_ad_rhs(469, 468, A::offset(s.ad_value(471), (-1.0)));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_div_ad_lhs(470, A::mul(A::scale(s.ad_value(469), 0.5), s.ad_value(469)), 468);
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_offset_ad(532, A::sub_from_scalar(p.p1034, s.ad_value(470)), (-0.05));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(473, A::offset(A::square(s.ad_value(532)), 0.224));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(472, p.p1034, A::scale(A::add(s.ad_value(532), s.ad_value(473)), 0.5));
        }

        if ((!(s.v[629] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_sub(504, 503, 472);
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[630] != 0.0))) {
            s.copy_ad(504, 503);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sub(506, 500, 501);
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(470, 341);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(509, 397, 470);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(510, 397, 470);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(467, A::scale(s.ad_value(130), ((-0.5) * p.p54)), 509);
        }

        s.v[631] = if (s.v[467] > (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[631] != 0.0)) {
            s.store_exp(468, 467);
        }

        if ((!(s.v[629] != 0.0)) && (s.v[631] != 0.0)) {
            s.store_mul_ad_rhs(522, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[631] != 0.0))) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[631] != 0.0))) {
            s.store_mul_ad_rhs(522, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(469, A::mul(s.ad_value(100), s.ad_value(417)), 340);
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(470, 96);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(471, A::add(A::add(s.ad_value(469), A::mul(s.ad_value(470), s.ad_value(522))), s.ad_value(99)), 396);
        }

        s.v[632] = if (s.v[471] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[632] != 0.0)) {
            s.store_offset(511, 471, 1.0);
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[632] != 0.0))) {
            s.store_div_from_scalar_ad(467, 1.0, A::offset(A::scale(s.ad_value(471), 8.0), 3.0));
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[632] != 0.0))) {
            s.store_mul_ad_lhs(511, A::offset(A::scale(s.ad_value(471), 3.0), 1.0), 467);
        }

    }

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[633] = if (s.v[378] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[633] != 0.0)) {
            s.store_offset_scaled(470, 378, 2.0, p.p54);
        }

        if ((!(s.v[629] != 0.0)) && (s.v[633] != 0.0)) {
            s.store_mul_ad_rhs(471, 499, {
                if ((p.p54 / s.v[470]) > 1e-38) {
                    A::ln(A::div_from_scalar(p.p54, s.ad_value(470)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!(s.v[629] != 0.0)) && (s.v[633] != 0.0)) {
            s.store_mul(519, 511, 471);
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[633] != 0.0))) {
            s.store_scalar(519, 0.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(63, 129, 522);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(523, 63, 506);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(467, A::scale(s.ad_value(133), ((-0.5) * (p.p55 * p.p54))), 510);
        }

        s.v[634] = if (s.v[467] > (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[634] != 0.0)) {
            s.store_exp(468, 467);
        }

        if ((!(s.v[629] != 0.0)) && (s.v[634] != 0.0)) {
            s.store_mul_ad_rhs(469, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[634] != 0.0))) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!(s.v[629] != 0.0)) && (!(s.v[634] != 0.0))) {
            s.store_mul_ad_rhs(469, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(467, 132, 469);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(524, 467, 506);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(430, ((p.p57 / s.v[429]) - 1.0));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sqrt_ad(467, A::offset(A::scale(s.ad_value(128), 1.0 / (p.p54)), 1.0));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_add_ad_rhs(468, 121, A::scale(s.ad_value(122), 1.0 / (p.p54)));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_add_ad(520, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(467), (-1.0))), s.ad_value(502)), A::mul(s.ad_value(468), s.ad_value(430)));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad(464, A::mul(s.ad_value(415), s.ad_value(501)), A::offset(s.ad_value(127), p.p55));
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(517, 0.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(521, 0.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sqrt_ad(518, A::offset(A::scale(s.ad_value(377), 1.0 / (p.p54)), 1.0));
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(514, 502);
        }

        if (!(s.v[629] != 0.0)) {
            let assign7680_ad_e8696: A = A::sub(A::sub(A::sub(A::add(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(408), p.p37), A::mul(A::sub(A::mul(s.ad_value(376), s.ad_value(514)), A::mul(s.ad_value(346), s.ad_value(502))), s.ad_value(518))), s.ad_value(523)), s.ad_value(524)), A::mul(s.ad_value(125), s.ad_value(464))), s.ad_value(520)), s.ad_value(517)), s.ad_value(519)), s.ad_value(521));
            s.store_ad(507, &assign7680_ad_e8696);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sub(508, 504, 507);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_mul(497, 511, 499);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(512, A::mul(s.ad_value(384), s.ad_value(508)), 497);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_div_ad_lhs(513, A::sub(s.ad_value(151), A::mul(A::sub_from_scalar(1.0, s.ad_value(384)), s.ad_value(508))), 497);
        }

        s.v[635] = if (s.v[512] > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[635] != 0.0)) {
            s.copy_ad(505, 508);
        }

        s.v[636] = if (s.v[513] > 100.0) { 1.0 } else { 0.0 };

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_div_ad(467, A::sub(s.ad_value(508), s.ad_value(151)), A::mul(s.ad_value(511), s.ad_value(499)));
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_exp(515, 467);
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_mul_ad_lhs(505, A::div(A::mul(s.ad_value(499), s.ad_value(367)), s.ad_value(396)), 515);
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_exp(515, 512);
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_mul_ad_rhs(468, 497, {
                if ((1.0 + s.v[515]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(515), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_mul_ad(471, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(498), s.ad_value(367))), A::exp(s.ad_value(513))), A::sub_from_scalar(1.0, s.ad_value(384)));
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_sub_ad_rhs(469, 384, A::div(A::mul(s.ad_value(497), s.ad_value(471)), A::sub_from_scalar(1.0, s.ad_value(384))));
        }

        if (((!(s.v[629] != 0.0)) && (!(s.v[635] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_div(505, 468, 469);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_sub_ad_lhs(470, A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(406)), 501);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scale(516, 470, 4.0);
        }

        s.v[637] = if (s.v[516] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[629] != 0.0)) && (s.v[637] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(525, 0.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(526, 415);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(527, 1000000.0);
        }

        let mut assign7910_loop_guard: usize = 0;
        while {
            let assign7910_cond_e8932: f64 = (s.v[526] - s.v[527]);
            let assign7910_cond_e8932_d_n0: f64 = (s.dn[526][0] - s.dn[527][0]);
            let assign7910_cond_e8932_d_n1: f64 = (s.dn[526][1] - s.dn[527][1]);
            let assign7910_cond_e8932_d_n2: f64 = (s.dn[526][2] - s.dn[527][2]);
            let assign7910_cond_e8932_d_n3: f64 = (s.dn[526][3] - s.dn[527][3]);
            let assign7910_cond_e8932_d_n4: f64 = (s.dn[526][4] - s.dn[527][4]);
            let assign7910_cond_e8932_d_n5: f64 = (s.dn[526][5] - s.dn[527][5]);
            let assign7910_cond_e8932_d_n6: f64 = (s.dn[526][6] - s.dn[527][6]);
            let assign7910_cond_e8932_d_n7: f64 = (s.dn[526][7] - s.dn[527][7]);
            let assign7910_cond_e8932_d_n8: f64 = (s.dn[526][8] - s.dn[527][8]);
            let assign7910_cond_e8932_d_n9: f64 = (s.dn[526][9] - s.dn[527][9]);
            let assign7910_cond_e8932_d_n10: f64 = (s.dn[526][10] - s.dn[527][10]);
            let assign7910_cond_e8932_d_n11: f64 = (s.dn[526][11] - s.dn[527][11]);
            let assign7910_cond_e8932_d_n12: f64 = (s.dn[526][12] - s.dn[527][12]);
            let assign7910_cond_e8932_d_n13: f64 = (s.dn[526][13] - s.dn[527][13]);
            let assign7910_cond_e8933: f64 = (assign7910_cond_e8932).abs();
            let assign7910_cond_e8933_d_n0: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n0 } else { (-assign7910_cond_e8932_d_n0) };
            let assign7910_cond_e8933_d_n1: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n1 } else { (-assign7910_cond_e8932_d_n1) };
            let assign7910_cond_e8933_d_n2: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n2 } else { (-assign7910_cond_e8932_d_n2) };
            let assign7910_cond_e8933_d_n3: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n3 } else { (-assign7910_cond_e8932_d_n3) };
            let assign7910_cond_e8933_d_n4: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n4 } else { (-assign7910_cond_e8932_d_n4) };
            let assign7910_cond_e8933_d_n5: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n5 } else { (-assign7910_cond_e8932_d_n5) };
            let assign7910_cond_e8933_d_n6: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n6 } else { (-assign7910_cond_e8932_d_n6) };
            let assign7910_cond_e8933_d_n7: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n7 } else { (-assign7910_cond_e8932_d_n7) };
            let assign7910_cond_e8933_d_n8: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n8 } else { (-assign7910_cond_e8932_d_n8) };
            let assign7910_cond_e8933_d_n9: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n9 } else { (-assign7910_cond_e8932_d_n9) };
            let assign7910_cond_e8933_d_n10: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n10 } else { (-assign7910_cond_e8932_d_n10) };
            let assign7910_cond_e8933_d_n11: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n11 } else { (-assign7910_cond_e8932_d_n11) };
            let assign7910_cond_e8933_d_n12: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n12 } else { (-assign7910_cond_e8932_d_n12) };
            let assign7910_cond_e8933_d_n13: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n13 } else { (-assign7910_cond_e8932_d_n13) };
            let assign7910_cond_e8937: f64 = if ((!(s.v[629] != 0.0)) && ((s.v[525] <= 4.0) && (assign7910_cond_e8933 > 1e-12))) { 1.0 } else { 0.0 };
            assign7910_cond_e8937 != 0.0
        } {
            assign7910_loop_guard += 1;
            assert!(assign7910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!(s.v[629] != 0.0)) {
                s.copy_ad(527, 526);
            }
            if (!(s.v[629] != 0.0)) {
                s.store_scale(464, 526, 200000000.0);
            }
            if (!(s.v[629] != 0.0)) {
                s.store_div_ad_lhs(638, A::add(s.ad_value(505), s.ad_value(516)), 464);
            }
            if (!(s.v[629] != 0.0)) {
                s.store_offset_ad(639, A::exp(A::scale({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p.p59 * 0.7))), 1.0);
            }
            if (!(s.v[629] != 0.0)) {
                s.store_div_from_scalar(528, (p.p58 * 1.9e-9), 639);
            }
            if (!(s.v[629] != 0.0)) {
                s.store_sub_ad_rhs(526, 415, A::mul(A::scale(s.ad_value(416), 1.0 / (p.p47)), s.ad_value(528)));
            }
            if (!(s.v[629] != 0.0)) {
                s.store_offset(525, 525, 1.0);
            }
        }

        if (!(s.v[629] != 0.0)) {
            s.copy_ad(62, 526);
        }

        s.copy_ad(462, 341);

        s.store_sub(463, 115, 118);

        s.store_mul(464, 397, 462);

        s.store_div_ad_lhs(467, A::scale(s.ad_value(133), ((-0.5) * (s.v[328] * s.v[327]))), 464);

        s.v[640] = if (s.v[467] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[640] != 0.0) {
            s.store_exp(468, 467);
        }

        if (s.v[640] != 0.0) {
            s.store_mul_ad_rhs(469, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_mul_ad_rhs(469, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        s.store_mul(467, 132, 469);

        s.store_mul(469, 467, 463);

        s.store_div_ad_lhs(467, A::scale(s.ad_value(130), ((-0.5) * s.v[327])), 464);

        s.v[641] = if (s.v[467] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[641] != 0.0) {
            s.store_exp(468, 467);
        }

        if (s.v[641] != 0.0) {
            s.store_mul_ad_rhs(470, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        if (!(s.v[641] != 0.0)) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if (!(s.v[641] != 0.0)) {
            s.store_mul_ad_rhs(470, 468, A::offset(A::scale(s.ad_value(468), 2.0), 1.0));
        }

        s.store_mul_ad_lhs(470, A::mul(s.ad_value(129), s.ad_value(470)), 463);

        s.store_div_ad(471, A::mul(s.ad_value(62), s.ad_value(118)), A::offset(s.ad_value(127), s.v[328]));

        s.store_sqrt_ad(467, A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[327])), 1.0));

        s.store_add_ad(472, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(467), (-1.0))), s.ad_value(339)), A::mul(A::add(s.ad_value(121), A::scale(s.ad_value(122), 1.0 / (s.v[327]))), s.ad_value(430)));

        s.store_add_ad_lhs(531, A::add(A::sub(A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(469)), s.ad_value(470)), A::mul(s.ad_value(125), s.ad_value(471))), 472);

        s.store_sub_ad(359, A::sub(s.ad_value(531), s.ad_value(118)), A::mul(s.ad_value(120), s.ad_value(339)));

        s.store_scale_ad(344, A::mul(A::scale(s.ad_value(108), 1.602176462e-19), A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[327])), 1.0)), (1000000.0 * p.p155));

        s.v[64] = (((p.p424 * (p.p427 + (((s.v[328] / p.p23) / 3.0) / p.p425))) / ((p.p425 * p.p3) * (p.p1 - p.p428))) + (p.p426 / ((p.p1 * s.v[328]) * p.p3)));

        s.v[642] = if (s.v[64] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[642] != 0.0) {
            s.store_scalar(64, (1.0 / s.v[64]));
        }

        if (!(s.v[642] != 0.0)) {
            s.store_scalar(64, 1000.0);
        }

        s.v[644] = if (p.p18 < 0.001) { 1.0 } else { 0.0 };

        if ((p.p40 != 0.0) && (s.v[644] != 0.0)) {
            s.store_scalar(65, 1000.0);
        }

        if ((p.p40 != 0.0) && (!(s.v[644] != 0.0))) {
            s.store_scalar(65, (p.p255 + (1.0 / p.p18)));
        }

        s.v[645] = if (p.p19 < 0.001) { 1.0 } else { 0.0 };

        if ((p.p40 != 0.0) && (s.v[645] != 0.0)) {
            s.store_scalar(66, 1000.0);
        }

        if ((p.p40 != 0.0) && (!(s.v[645] != 0.0))) {
            s.store_scalar(66, (p.p255 + (1.0 / p.p19)));
        }

        if (!(p.p40 != 0.0)) {
            s.store_scalar(65, 0.0);
        }

        if (!(p.p40 != 0.0)) {
            s.store_scalar(66, 0.0);
        }

        s.store_offset(67, 359, (p.p37 * p.p20));

        s.store_scale_ad(360, A::sqrt(A::div(A::mul(s.ad_value(417), s.ad_value(480)), A::scale(s.ad_value(108), (1.602176462e-19 * 1000000.0)))), 0.3333333333333333);

        s.store_sub_ad_lhs(468, A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(406)), 118);

        s.store_scale(469, 468, 2.0);

        s.store_scale(470, 468, 2.5);

        if (p.p37 == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }

        s.v[646] = if (s.v[68] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[646] != 0.0) {
            s.store_scalar(68, 0.0);
        }

        s.v[647] = if (p.p62 == 4.0) { 1.0 } else { 0.0 };

        if (s.v[647] != 0.0) {
            s.store_mul(509, 397, 341);
        }

        if (s.v[647] != 0.0) {
            s.store_div_ad_lhs(467, A::scale(s.ad_value(130), s.v[327]), 509);
        }

        s.v[648] = if (s.v[467] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_exp(468, 467);
        }

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_offset(469, 468, (-1.0));
        }

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_square(470, 469);
        }

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_add_ad_rhs(471, 470, A::scale(s.ad_value(468), (2.0 * 3.720075976e-44)));
        }

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_div(522, 468, 471);
        }

        if ((s.v[647] != 0.0) && (!(s.v[648] != 0.0))) {
            s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));
        }

        if (s.v[647] != 0.0) {
            s.store_div(463, 417, 340);
        }

        if (s.v[647] != 0.0) {
            s.store_mul(464, 100, 463);
        }

        if (s.v[647] != 0.0) {
            s.store_div_ad_lhs(531, A::add(A::add(s.ad_value(464), A::mul(s.ad_value(96), s.ad_value(522))), s.ad_value(99)), 396);
        }

        s.v[649] = if (s.v[531] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[649] != 0.0)) {
            s.store_offset(529, 531, 1.0);
        }

        if ((s.v[647] != 0.0) && (!(s.v[649] != 0.0))) {
            s.store_div_from_scalar_ad(467, 1.0, A::offset(A::scale(s.ad_value(531), 8.0), 3.0));
        }

        if ((s.v[647] != 0.0) && (!(s.v[649] != 0.0))) {
            s.store_mul_ad_lhs(529, A::offset(A::scale(s.ad_value(531), 3.0), 1.0), 467);
        }

        if (s.v[647] != 0.0) {
            s.store_mul(467, 529, 480);
        }

        if (s.v[647] != 0.0) {
            s.copy_ad(468, 151);
        }

        if (s.v[647] != 0.0) {
            s.store_div(469, 468, 467);
        }

        s.v[650] = if (s.v[469] < (-100.0)) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[650] != 0.0)) {
            s.store_div_ad_lhs(470, A::scale(s.ad_value(396), 3.720075976e-44), 367);
        }

        if ((s.v[647] != 0.0) && (s.v[650] != 0.0)) {
            s.store_add_ad_rhs(471, 384, A::mul(s.ad_value(470), s.ad_value(529)));
        }

        s.v[651] = if (s.v[469] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[647] != 0.0) && (!(s.v[650] != 0.0))) && (s.v[651] != 0.0)) {
            s.store_div_ad_lhs(470, A::scale(s.ad_value(396), 2.688117142e43), 367);
        }

        if (((s.v[647] != 0.0) && (!(s.v[650] != 0.0))) && (s.v[651] != 0.0)) {
            s.store_add_ad_rhs(471, 384, A::mul(s.ad_value(470), s.ad_value(529)));
        }

        if (((s.v[647] != 0.0) && (!(s.v[650] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_div_ad_lhs(470, A::mul(A::exp(s.ad_value(469)), s.ad_value(396)), 367);
        }

        if (((s.v[647] != 0.0) && (!(s.v[650] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_add_ad_rhs(471, 384, A::mul(s.ad_value(470), s.ad_value(529)));
        }

        if (s.v[647] != 0.0) {
            s.store_div_ad_lhs(69, A::scale(s.ad_value(467), 0.6931471805599453), 471);
        }

        if (!(s.v[647] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        s.v[704] = if ((p.p38 >= 4.4) || (p.p63 != 0.0)) { 1.0 } else { 0.0 };

        s.v[705] = if (s.v[106] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[704] != 0.0) && (s.v[705] != 0.0)) {
            s.store_scalar(106, 0.01);
        }

        s.v[706] = if (s.v[106] > 1.0) { 1.0 } else { 0.0 };

        if (((s.v[704] != 0.0) && (!(s.v[705] != 0.0))) && (s.v[706] != 0.0)) {
            s.store_scalar(106, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[704] != 0.0) && (!(s.v[705] != 0.0))) && (s.v[706] != 0.0)) {
            s.store_scalar(105, 0.0);
        }

        s.v[707] = if (s.v[181] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[707] != 0.0) {
            s.store_scalar(181, 0.0);
        }

        if (s.v[707] != 0.0) {
            s.store_scalar(182, 0.0);
        }

        s.v[708] = if ((s.v[182] < 0.001) && (s.v[182] != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[707] != 0.0)) && (s.v[708] != 0.0)) {
            s.store_scalar(182, 0.0);
        }

        s.v[738] = if (s.v[308] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[738] != 0.0)) {
            s.store_scalar(308, 0.0);
        }

        s.v[739] = if (s.v[309] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[739] != 0.0)) {
            s.store_scalar(309, 0.0);
        }

        s.v[740] = if (s.v[310] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[740] != 0.0)) {
            s.store_scalar(310, 0.0);
        }

        s.v[741] = if (s.v[311] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[741] != 0.0)) {
            s.store_scalar(311, 0.0);
        }

        s.v[742] = if (s.v[312] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[742] != 0.0)) {
            s.store_scalar(312, 0.0);
        }

        s.v[743] = if (s.v[313] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && (s.v[743] != 0.0)) {
            s.store_scalar(313, 0.0);
        }

        s.v[410] = 0.0;

        s.v[805] = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        s.v[806] = if ((p.p35 != 0.0) && (!(1.0 != 0.0))) { 1.0 } else { 0.0 };

        s.v[807] = 1.0;

        if (((s.v[805] != 0.0) && (s.v[806] != 0.0)) && (s.v[807] != 0.0)) {
            s.store_ad(410, &A::voltage(ctx, &nodes, Some(5), None));
        }

        s.v[808] = 1.0;

        if ((((s.v[805] != 0.0) && (s.v[806] != 0.0)) && (!(s.v[807] != 0.0))) && (s.v[808] != 0.0)) {
            s.store_ad(410, &A::voltage(ctx, &nodes, Some(4), None));
        }

        if ((((s.v[805] != 0.0) && (s.v[806] != 0.0)) && (!(s.v[807] != 0.0))) && (!(s.v[808] != 0.0))) {
            s.store_ad(410, &A::voltage(ctx, &nodes, Some(6), None));
        }

        if ((s.v[805] != 0.0) && (!(s.v[806] != 0.0))) {
            s.store_ad(410, &A::voltage(ctx, &nodes, Some(6), None));
        }

        s.store_offset(409, 410, s.v[409]);

        s.store_scale(411, 409, 1.0 / (s.v[429]));

        s.store_offset(430, 411, (-1.0));

        s.copy_ad(70, 409);

        s.v[1133] = 0.0;

        s.v[1134] = 0.0;

        s.v[1135] = 0.0;

        s.v[1136] = 0.0;

        s.v[1131] = 0.0;

        s.v[1121] = 0.0;

        s.v[855] = 0.0;

        s.v[1122] = 0.0;

        s.v[1130] = 0.0;

        s.v[1127] = 0.0;

        s.v[1128] = 0.0;

        s.v[1126] = 0.0;

        s.v[1118] = 0.0;

        s.copy_ad(955, 182);

        s.copy_ad(1095, 173);

        s.copy_ad(1096, 174);

        s.copy_ad(1097, 171);

        s.copy_ad(1098, 172);

        s.v[1159] = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        s.v[1160] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_scale(832, 409, 8.617087e-5);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_offset(843, 409, 1108.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_square(848, 409);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_sub_from_scalar_ad(912, 1.16, A::div(A::scale(s.ad_value(848), 0.000702), s.ad_value(843)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_scalar(845, 0.00019230584);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_sqrt(848, 409);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_mul_ad_lhs(846, A::mul(A::scale(s.ad_value(409), 14500000000.0), s.ad_value(848)), 845);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_sub_from_scalar_ad(849, 21.5565981, A::div(s.ad_value(912), A::scale(s.ad_value(832), 2.0)));
        }

        s.v[1161] = if (s.v[849] > (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) && (s.v[1161] != 0.0)) {
            s.store_exp(847, 849);
        }

        if (((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) && (!(s.v[1161] != 0.0))) {
            s.store_scalar(847, (((-100.0)) as f64).exp());
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_mul(911, 846, 847);
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_ad(843, &{
                if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                    A::ln(A::div(A::scale(s.ad_value(108), 1e20), A::square(s.ad_value(911))))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_mul(940, 832, 843);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_scalar(429, (p.p126 + 273.15));
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_scale(832, 409, 8.617087e-5);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_scale(1104, 429, 8.617087e-5);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.copy_ad(1103, 394);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_sub_from_scalar_ad(912, p.p49, A::div(A::mul(A::scale(s.ad_value(409), p.p50), s.ad_value(409)), A::offset(s.ad_value(409), p.p51)));
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_div_from_scalar_ad(845, 1.0, A::sqrt(A::mul(A::square(s.ad_value(429)), s.ad_value(429))));
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_sqrt(848, 409);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_mul_ad_lhs(846, A::mul(A::scale(s.ad_value(409), p.p48), s.ad_value(848)), 845);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_exp_ad(847, A::sub(A::div(s.ad_value(1103), A::scale(s.ad_value(1104), 2.0)), A::div(s.ad_value(912), A::scale(s.ad_value(832), 2.0))));
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_mul(911, 846, 847);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_ad(843, &{
                if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                    A::ln(A::div(A::scale(s.ad_value(108), 1e20), A::square(s.ad_value(911))))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_mul(940, 832, 843);
        }

        s.v[1162] = if (s.v[109] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_ad(843, &{
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((s.v[1159] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_mul_ad_lhs(941, A::scale(s.ad_value(832), (-p.p37)), 843);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1162] != 0.0))) {
            s.store_ad(843, &{
                if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                    A::ln(A::div(A::div(A::mul(A::neg(s.ad_value(108)), s.ad_value(109)), s.ad_value(911)), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1162] != 0.0))) {
            s.store_mul_ad_lhs(941, A::scale(s.ad_value(832), (-p.p37)), 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(942, &A::mul(A::scale(s.ad_value(832), 2.0), {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (s.v[1159] != 0.0) {
            s.store_sqrt(943, 942);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_lhs(944, A::sqrt(A::div(A::scale(s.ad_value(417), 2.0), A::scale(s.ad_value(108), (1.602176462e-19 * 1000000.0)))), 943);
        }

        if (s.v[1159] != 0.0) {
            s.store_div_ad_lhs(1140, A::sqrt(A::scale(A::mul(A::scale(s.ad_value(417), 1.602176462e-19), s.ad_value(108)), (1000000.0 * 0.5))), 943);
        }

        if (s.v[1159] != 0.0) {
            s.store_sqrt_ad(844, A::mul(A::mul(A::div(s.ad_value(417), A::scale(s.ad_value(416), 8.85418e-12)), s.ad_value(415)), s.ad_value(944)));
        }

        if (s.v[1159] != 0.0) {
            s.store_exp_ad(843, A::div(A::scale(s.ad_value(136), ((-0.5) * s.v[327])), s.ad_value(844)));
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_rhs(1141, 843, A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)));
        }

        if (s.v[1159] != 0.0) {
            s.store_exp_ad(843, A::div(A::scale(s.ad_value(135), ((-0.5) * s.v[327])), s.ad_value(844)));
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_rhs(845, 843, A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(843)));
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_lhs(1142, A::mul(s.ad_value(192), s.ad_value(845)), 193);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(49, 832);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_lhs(847, A::div_from_scalar(1.115, s.ad_value(832)), 430);
        }

        if (s.v[1159] != 0.0) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(256), s.ad_value(847)), 300);
        }

        s.v[1163] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scale_ad(843, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1164] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (s.v[1164] != 0.0)) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (!(s.v[1164] != 0.0))) {
            s.store_exp(843, 850);
        }

        s.v[1165] = if (s.v[256] == s.v[257]) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.copy_ad(844, 843);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1165] != 0.0))) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(257), s.ad_value(847)), 300);
        }

        s.v[1166] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1165] != 0.0))) && (s.v[1166] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1167] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (!(s.v[1165] != 0.0))) && (!(s.v[1166] != 0.0))) && (s.v[1167] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.v[1159] != 0.0) && (!(s.v[1165] != 0.0))) && (!(s.v[1166] != 0.0))) && (!(s.v[1167] != 0.0))) {
            s.store_exp(844, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(258), s.ad_value(847)), 302);
        }

        s.v[1168] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1168] != 0.0)) {
            s.store_scale_ad(845, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1169] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1168] != 0.0))) && (s.v[1169] != 0.0)) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1168] != 0.0))) && (!(s.v[1169] != 0.0))) {
            s.store_exp(845, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(972, 355, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(949, 306, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(947, 308, 844);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(951, 310, 845);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(850, 259, 430);
        }

        s.v[1170] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scale_ad(843, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1171] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1170] != 0.0))) && (s.v[1171] != 0.0)) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1170] != 0.0))) && (!(s.v[1171] != 0.0))) {
            s.store_exp(843, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(953, 312, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(256), s.ad_value(847)), 301);
        }

        s.v[1172] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(843, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1173] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1172] != 0.0))) && (s.v[1173] != 0.0)) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1172] != 0.0))) && (!(s.v[1173] != 0.0))) {
            s.store_exp(843, 850);
        }

        s.v[1174] = if (s.v[256] == s.v[260]) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.copy_ad(844, 843);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1174] != 0.0))) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(260), s.ad_value(847)), 301);
        }

        s.v[1175] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1174] != 0.0))) && (s.v[1175] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1176] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (!(s.v[1174] != 0.0))) && (!(s.v[1175] != 0.0))) && (s.v[1176] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.v[1159] != 0.0) && (!(s.v[1174] != 0.0))) && (!(s.v[1175] != 0.0))) && (!(s.v[1176] != 0.0))) {
            s.store_exp(844, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(261), s.ad_value(847)), 303);
        }

        s.v[1177] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scale_ad(845, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1178] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_exp(845, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(973, 356, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(950, 307, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(948, 309, 844);
        }

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1159] != 0.0) {
            s.store_mul(952, 311, 845);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(850, 262, 430);
        }

        s.v[1179] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scale_ad(843, A::offset(A::offset(s.ad_value(850), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1180] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1179] != 0.0))) && (!(s.v[1180] != 0.0))) {
            s.store_exp(843, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(954, 313, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_rhs(945, 144, A::pow(s.ad_value(411), s.ad_value(145)));
        }

        s.v[1181] = if (p.p38 < 4.2) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_offset_ad(961, A::mul(s.ad_value(231), A::offset(A::scale(s.ad_value(411), p.p238), 1.0)), 1e-9);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1181] != 0.0))) {
            s.store_offset_ad(961, A::mul(s.ad_value(231), A::offset(A::scale(s.ad_value(430), p.p238), 1.0)), 1e-9);
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(850, 235, p.p235);
        }

        if (s.v[1159] != 0.0) {
            s.store_div(960, 850, 961);
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(847, 51, p.p235);
        }

        if (s.v[1159] != 0.0) {
            s.store_div(959, 847, 961);
        }

        if (s.v[1159] != 0.0) {
            s.store_offset(845, 959, 1.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_offset(850, 960, 1.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_div(843, 845, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(945, 945, 843);
        }

        if (s.v[1159] != 0.0) {
            s.store_sub_ad_rhs(946, 101, A::mul(s.ad_value(102), s.ad_value(430)));
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(845, A::mul(s.ad_value(45), s.ad_value(959)), 1.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(850, A::mul(s.ad_value(45), s.ad_value(960)), 1.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_div(843, 845, 850);
        }

        if (s.v[1159] != 0.0) {
            s.store_mul(946, 946, 843);
        }

        s.v[1182] = if (p.p429 != 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_div_ad_lhs(955, A::add(s.ad_value(181), A::mul(s.ad_value(186), s.ad_value(430))), 159);
        }

        if ((s.v[1159] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_scalar(1095, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_scalar(1096, 0.0);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_scalar(955, 0.0);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_scale(1094, 159, p.p3);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_mul(853, 186, 430);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_add(844, 169, 853);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_offset(845, 853, p.p140);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_div(1095, 844, 1094);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_div(1097, 845, 1094);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_add(850, 170, 853);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_offset(847, 853, p.p139);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_div(1096, 850, 1094);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_div(1098, 847, 1094);
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_rhs(956, 153, A::mul(s.ad_value(139), s.ad_value(430)));
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_rhs(957, 154, A::mul(s.ad_value(141), s.ad_value(430)));
        }

        if (s.v[1159] != 0.0) {
            s.store_add_ad_rhs(958, 155, A::mul(s.ad_value(143), s.ad_value(430)));
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(940, 115);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(941, 160);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(942, 118);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(943, 339);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(944, 340);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(912, 395);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(1140, 367);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(1141, 342);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(1142, 343);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(949, 161);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(950, 162);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(947, 163);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(948, 164);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(951, 165);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(952, 166);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(953, 167);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(954, 168);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(972, 357);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(973, 358);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(945, 404);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(946, 407);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(956, 138);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(957, 140);
        }

        if (!(s.v[1159] != 0.0)) {
            s.copy_ad(958, 142);
        }

        s.v[1183] = if ((if self.param_given[90] { 1.0 } else { 0.0 } != 0.0) || (if self.param_given[94] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        s.v[1184] = if !(if self.param_given[90] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1183] != 0.0) && (s.v[1184] != 0.0)) {
            s.store_scalar(120, 0.53);
        }

        s.v[1185] = if !(if self.param_given[94] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1183] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_scalar(124, (-0.0186));
        }

        s.v[1186] = if !(if self.param_given[87] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1183] != 0.0)) && (s.v[1186] != 0.0)) && (p.p41 != 0.0)) {
            s.store_scale_ad(843, A::div_from_scalar(1.602176462e-19, A::scale(s.ad_value(417), 2.0)), 1000000.0);
        }

        if (((!(s.v[1183] != 0.0)) && (s.v[1186] != 0.0)) && (!(p.p41 != 0.0))) {
            s.store_scalar(843, 0.00077348);
        }

        if ((!(s.v[1183] != 0.0)) && (s.v[1186] != 0.0)) {
            s.store_sub_ad_rhs(114, 942, A::scale(A::mul(s.ad_value(843), s.ad_value(108)), (s.v[117] * s.v[117])));
        }

        s.v[1187] = if (s.v[114] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1183] != 0.0)) && (s.v[1187] != 0.0)) {
            s.store_neg(114, 114);
        }

        s.v[1188] = if (s.v[116] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1183] != 0.0)) && (s.v[1188] != 0.0)) {
            s.store_neg(116, 116);
        }

        s.v[1189] = if !(if self.param_given[85] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1183] != 0.0)) && (s.v[1189] != 0.0)) {
            s.store_div_ad_lhs(112, A::mul(s.ad_value(419), A::sqrt(s.ad_value(108))), 396);
        }

        s.v[1190] = if !(if self.param_given[86] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1183] != 0.0)) && (s.v[1190] != 0.0)) {
            s.store_div_ad_lhs(113, A::mul(s.ad_value(419), A::sqrt(s.ad_value(109))), 396);
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_sub(843, 112, 113);
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_sub_ad_lhs(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_mul_ad_rhs(845, 943, A::sub(A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), s.ad_value(943)));
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_div_ad(846, A::mul(s.ad_value(843), s.ad_value(844)), A::add(A::scale(s.ad_value(845), 2.0), s.ad_value(116)));
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_add_ad_lhs(402, A::sub(s.ad_value(402), s.ad_value(124)), 846);
        }

        if (!(s.v[1183] != 0.0)) {
            s.store_sub_ad_rhs(120, 113, A::mul(A::scale(s.ad_value(402), 2.0), A::sqrt(A::sub(s.ad_value(942), s.ad_value(116)))));
        }

        s.store_offset(843, 265, s.v[328]);

        s.v[1191] = if (s.v[843] < 1e-8) { 1.0 } else { 0.0 };

        if (s.v[1191] != 0.0) {
            s.store_scalar(843, 1e-8);
        }

        s.store_mul_ad_rhs(405, 120, A::offset(A::div(s.ad_value(264), s.ad_value(843)), 1.0));

        s.store_scale(376, 405, (p.p66 * 1.0 / (p.p67)));

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.v[1192] = if !(if self.param_given[109] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        s.v[1193] = if ((if self.param_given[108] { 1.0 } else { 0.0 } != 0.0) || (if self.param_given[107] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1192] != 0.0) && (s.v[1193] != 0.0)) {
            s.store_sub_ad(406, A::sub(A::add(A::sub(s.ad_value(406), s.ad_value(152)), A::scale(s.ad_value(408), p.p37)), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(943)));
        }

        if ((s.v[1192] != 0.0) && (!(s.v[1193] != 0.0))) {
        }

        s.v[1194] = if !(if self.param_given[108] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1194] != 0.0) {
            s.store_scale_ad(408, A::add(A::add(s.ad_value(406), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(943))), p.p37);
        }

        s.v[1195] = if (p.p38 < 4.2) { 1.0 } else { 0.0 };

        if (s.v[1195] != 0.0) {
            s.copy_ad(1095, 173);
        }

        if (s.v[1195] != 0.0) {
            s.copy_ad(1097, 171);
        }

        if (s.v[1195] != 0.0) {
            s.copy_ad(1140, 367);
        }

        if (s.v[1195] != 0.0) {
            s.copy_ad(1141, 342);
        }

        if (s.v[1195] != 0.0) {
            s.copy_ad(1142, 343);
        }

        s.v[1196] = if (p.p62 == 4.0) { 1.0 } else { 0.0 };

        if ((s.v[1195] != 0.0) && (s.v[1196] != 0.0)) {
            s.copy_ad(956, 138);
        }

        if ((s.v[1195] != 0.0) && (s.v[1196] != 0.0)) {
            s.copy_ad(958, 142);
        }

        s.store_ad(819, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p37));

        s.store_ad(818, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(8)), p.p37));

        s.store_ad(821, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(8)), p.p37));

        s.store_ad(897, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(8)), p.p37));

        s.store_ad(899, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p37));

        s.store_ad(1114, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(4)), p.p37));

        s.store_ad(1087, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(8)), p.p37));

        s.store_ad(1088, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(7)), p.p37));

        s.store_ad(1018, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(8)), p.p37));

        s.store_sub(817, 818, 819);

        s.store_sub(820, 821, 819);

        s.store_sub(898, 897, 819);

        s.store_sub(1019, 1018, 819);

        s.v[1197] = if (s.v[819] >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1197] != 0.0) {
            s.store_scalar(398, 1.0);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(822, 819);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(823, 821);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(824, 818);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(900, 817);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(901, 897);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1110, 820);
        }

        if (s.v[1197] != 0.0) {
            s.store_scalar(995, s.v[347]);
        }

        if (s.v[1197] != 0.0) {
            s.store_scalar(996, s.v[348]);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1143, 282);
        }

        if (s.v[1197] != 0.0) {
            s.store_add_ad_rhs(1144, 283, A::mul(s.ad_value(284), s.ad_value(430)));
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1145, 285);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1146, 286);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1147, 287);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1148, 288);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1149, 289);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1150, 290);
        }

        if (s.v[1197] != 0.0) {
            s.store_add_ad_rhs(1151, 291, A::mul(s.ad_value(292), s.ad_value(430)));
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1152, 293);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1153, 294);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1154, 295);
        }

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1197] != 0.0) {
            s.copy_ad(1155, 296);
        }

        if (s.v[1197] != 0.0) {
            s.copy_ad(1156, 297);
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_scalar(398, (-1.0));
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_neg(822, 819);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(823, 820);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(824, 817);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(900, 818);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(901, 898);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1110, 821);
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_scalar(995, s.v[348]);
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_scalar(996, s.v[347]);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1143, 290);
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_add_ad_rhs(1144, 291, A::mul(s.ad_value(292), s.ad_value(430)));
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1145, 293);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1146, 294);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1147, 295);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1148, 296);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1149, 297);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1150, 282);
        }

        if (!(s.v[1197] != 0.0)) {
            s.store_add_ad_rhs(1151, 283, A::mul(s.ad_value(284), s.ad_value(430)));
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1152, 285);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1153, 286);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1154, 287);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1155, 288);
        }

        if (!(s.v[1197] != 0.0)) {
            s.copy_ad(1156, 289);
        }

        s.store_sub(902, 901, 941);

        s.v[913] = s.v[392];

        s.store_add(843, 406, 942);

        s.v[1198] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1198] != 0.0) {
            s.copy_ad(418, 417);
        }

        if (!(s.v[1198] != 0.0)) {
            s.store_scalar(418, (p.p60 * 8.85418e-12));
        }

        s.v[1199] = if ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1199] != 0.0) {
            s.store_div_ad(844, A::mul(A::scale(s.ad_value(418), (1000000.0 * 1.602176462e-19)), s.ad_value(110)), A::square(s.ad_value(396)));
        }

        if (s.v[1199] != 0.0) {
            s.store_sqrt_ad(847, A::offset(A::div(A::scale(A::sub(s.ad_value(823), s.ad_value(843)), 2.0), s.ad_value(844)), 1.0));
        }

        if (s.v[1199] != 0.0) {
            s.store_mul_ad_rhs(845, 844, A::offset(s.ad_value(847), (-1.0)));
        }

        if (s.v[1199] != 0.0) {
            s.store_div_ad_lhs(846, A::mul(A::scale(s.ad_value(845), 0.5), s.ad_value(845)), 844);
        }

        if (s.v[1199] != 0.0) {
            s.store_offset_ad(850, A::sub_from_scalar(p.p1034, s.ad_value(846)), (-0.05));
        }

        if (s.v[1199] != 0.0) {
            s.store_sqrt_ad(849, A::offset(A::square(s.ad_value(850)), 0.224));
        }

        if (s.v[1199] != 0.0) {
            s.store_sub_from_scalar_ad(848, p.p1034, A::scale(A::add(s.ad_value(850), s.ad_value(849)), 0.5));
        }

        if (s.v[1199] != 0.0) {
            s.store_sub(825, 823, 848);
        }

        if (!(s.v[1199] != 0.0)) {
            s.copy_ad(825, 823);
        }

        s.v[1200] = if ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1200] != 0.0) {
            s.store_div_ad(844, A::mul(A::scale(s.ad_value(418), (1000000.0 * 1.602176462e-19)), s.ad_value(110)), A::square(s.ad_value(396)));
        }

        if (s.v[1200] != 0.0) {
            s.store_sqrt_ad(847, A::offset(A::div(A::scale(A::sub(s.ad_value(1110), s.ad_value(843)), 2.0), s.ad_value(844)), 1.0));
        }

        if (s.v[1200] != 0.0) {
            s.store_mul_ad_rhs(845, 844, A::offset(s.ad_value(847), (-1.0)));
        }

        if (s.v[1200] != 0.0) {
            s.store_div_ad_lhs(846, A::mul(A::scale(s.ad_value(845), 0.5), s.ad_value(845)), 844);
        }

        if (s.v[1200] != 0.0) {
            s.store_offset_ad(850, A::sub_from_scalar(p.p1034, s.ad_value(846)), (-0.05));
        }

        if (s.v[1200] != 0.0) {
            s.store_sqrt_ad(849, A::offset(A::square(s.ad_value(850)), 0.224));
        }

        if (s.v[1200] != 0.0) {
            s.store_sub_from_scalar_ad(848, p.p1034, A::scale(A::add(s.ad_value(850), s.ad_value(849)), 0.5));
        }

        if (s.v[1200] != 0.0) {
            s.store_sub(1111, 1110, 848);
        }

        if (!(s.v[1200] != 0.0)) {
            s.copy_ad(1111, 1110);
        }

        s.copy_ad(1125, 823);

        s.v[892] = s.v[327];

        s.v[1201] = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1201] != 0.0) {
            s.store_scale(832, 409, 8.617087e-5);
        }

        if (!(s.v[1201] != 0.0)) {
            s.copy_ad(832, 49);
        }

        s.store_sub(834, 940, 942);

        s.v[1202] = if (s.v[37] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1202] != 0.0) {
            s.copy_ad(1033, 824);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1048, 824);
        }

        s.v[1203] = if (p.p432 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_div_ad_lhs(843, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_mul_ad_rhs(844, 224, A::add(A::exp(A::scale(s.ad_value(843), 0.5)), A::scale(A::exp(s.ad_value(843)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::sub(s.ad_value(940), s.ad_value(942)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_add_ad_lhs(1036, A::add(A::sub(s.ad_value(942), s.ad_value(846)), s.ad_value(216)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(A::neg(s.ad_value(223)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_mul_ad_rhs(848, 222, A::add(A::exp(A::scale(s.ad_value(846), 0.5)), A::scale(A::exp(s.ad_value(846)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_mul(845, 844, 902);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1203] != 0.0)) {
            s.store_add_ad_lhs(1031, A::mul(s.ad_value(847), s.ad_value(1036)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::add(A::offset(s.ad_value(393), s.v[913]), s.ad_value(218)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_div_ad_lhs(844, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_mul_ad_rhs(845, 224, A::add(A::exp(A::scale(s.ad_value(844), 0.5)), A::scale(A::exp(s.ad_value(844)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_mul_ad_rhs(846, 845, A::add(s.ad_value(822), s.ad_value(217)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_div_ad_lhs(847, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_mul_ad(848, A::mul(s.ad_value(393), s.ad_value(843)), A::add(A::sub(s.ad_value(942), s.ad_value(847)), s.ad_value(216)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_mul_ad_lhs(849, A::mul(s.ad_value(218), s.ad_value(843)), 846);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_add(1036, 848, 849);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_mul_ad_lhs(850, A::scale(s.ad_value(843), s.v[913]), 902);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1203] != 0.0))) {
            s.store_add(1031, 1036, 850);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(844, A::sub(s.ad_value(1036), s.ad_value(1031)), (-0.005));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(844)), 2.5e-5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_scaled_add(846, 844, 845, 0.5);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(847, A::mul(s.ad_value(846), s.ad_value(393)), 344);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1032, 1031, A::mul(A::scale(s.ad_value(846), 0.5), s.ad_value(847)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset(844, 942, (-0.02));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(845, A::sub(s.ad_value(844), s.ad_value(1032)), (-0.005));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(846, A::offset(A::square(s.ad_value(845)), (4.0 * 0.005)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1032, 844, A::scale(A::add(s.ad_value(845), s.ad_value(846)), 0.5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub(827, 942, 1032);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt(828, 827);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(864, A::mul(s.ad_value(944), s.ad_value(828)), 943);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt(846, 864);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(843, 131, 1032);
        }

        s.v[1204] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1204] != 0.0)) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1204] != 0.0))) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1204] != 0.0))) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(865, A::mul(s.ad_value(397), s.ad_value(846)), 844);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(843, 134, 1032);
        }

        s.v[1205] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1205] != 0.0)) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1205] != 0.0))) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1205] != 0.0))) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(866, A::mul(s.ad_value(397), s.ad_value(846)), 844);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(843, A::scale(s.ad_value(130), ((-0.5) * s.v[892])), 865);
        }

        s.v[1206] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1206] != 0.0)) {
            s.store_exp(844, 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1206] != 0.0)) {
            s.store_mul_ad_rhs(868, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1206] != 0.0))) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1206] != 0.0))) {
            s.store_mul_ad_rhs(868, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(845, A::mul(s.ad_value(100), s.ad_value(417)), 864);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad(846, A::add(s.ad_value(96), A::mul(s.ad_value(97), s.ad_value(1032))), A::mul(s.ad_value(98), s.ad_value(822)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(847, A::add(A::add(s.ad_value(845), A::mul(s.ad_value(846), s.ad_value(868))), s.ad_value(99)), 396);
        }

        s.v[1207] = if (s.v[847] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1207] != 0.0)) {
            s.store_offset(831, 847, 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1207] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::scale(s.ad_value(847), 8.0), 3.0));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1207] != 0.0))) {
            s.store_mul_ad_lhs(831, A::offset(A::scale(s.ad_value(847), 3.0), 1.0), 843);
        }

        s.v[1208] = if (s.v[378] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_lhs(843, A::neg(s.ad_value(379)), 822);
        }

        s.v[1209] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) {
            s.store_exp(845, 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_offset_ad(846, A::mul(s.ad_value(378), A::offset(s.ad_value(845), 1.0)), s.v[892]);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_mul(1090, 831, 847);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1208] != 0.0))) {
            s.store_scalar(1090, 0.0);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(63, 129, 868);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(867, 63, 834);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(843, A::scale(s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892]))), 866);
        }

        s.v[1210] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1210] != 0.0)) {
            s.store_exp(844, 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1210] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1202] != 0.0)) && (!(s.v[1210] != 0.0))) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1210] != 0.0))) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(843, 132, 845);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(904, 843, 834);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(843, A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[892])), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad(844, A::add(s.ad_value(121), A::scale(s.ad_value(122), 1.0 / (s.v[892]))), A::mul(s.ad_value(123), s.ad_value(1032)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad(903, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(843), (-1.0))), s.ad_value(943)), A::mul(s.ad_value(844), s.ad_value(430)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad(870, A::mul(s.ad_value(415), s.ad_value(942)), A::offset(s.ad_value(127), s.v[328]));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(846, 400, A::mul(s.ad_value(188), s.ad_value(1032)));
        }

        s.v[1211] = if (s.v[846] < 0.0001) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(846), 20000.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_mul_ad_lhs(846, A::sub_from_scalar(0.0002, s.ad_value(846)), 852);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(873, A::mul(s.ad_value(846), s.ad_value(1141)), 822);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(846, 401, A::mul(s.ad_value(190), s.ad_value(1032)));
        }

        s.v[1212] = if (s.v[846] < 0.0001) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(846), 20000.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_mul_ad_lhs(846, A::sub_from_scalar(0.0002, s.ad_value(846)), 852);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(1070, A::mul(s.ad_value(846), s.ad_value(1141)), 822);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(1089, A::offset(A::scale(s.ad_value(377), 1.0 / (s.v[892])), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_exp_ad(843, A::mul(A::scale(s.ad_value(382), 2.0), s.ad_value(822)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad(1091, A::mul(s.ad_value(391), A::offset(s.ad_value(843), (-1.0))), A::offset(s.ad_value(843), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            let assign15050_ad_e13615: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(408), p.p37), A::mul(A::sub(A::mul(s.ad_value(376), s.ad_value(828)), A::mul(s.ad_value(405), s.ad_value(943))), s.ad_value(1089))), A::mul(s.ad_value(403), s.ad_value(1032))), s.ad_value(867)), s.ad_value(904)), A::mul(A::add(s.ad_value(125), A::mul(s.ad_value(126), s.ad_value(1032))), s.ad_value(870)));
            s.store_sub_ad_lhs(1037, A::sub(A::sub(A::add(assign15050_ad_e13615, s.ad_value(903)), s.ad_value(873)), s.ad_value(1090)), 1091);
        }

        if (!(s.v[1202] != 0.0)) {
            let assign15060_ad_e13656: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(408), p.p37), A::mul(A::sub(A::mul(s.ad_value(376), s.ad_value(828)), A::mul(s.ad_value(405), s.ad_value(943))), s.ad_value(1089))), A::mul(s.ad_value(403), s.ad_value(1032))), s.ad_value(867)), s.ad_value(904)), A::mul(A::add(s.ad_value(125), A::mul(s.ad_value(126), s.ad_value(1032))), s.ad_value(870)));
            s.store_sub_ad_lhs(1052, A::sub(A::sub(A::add(assign15060_ad_e13656, s.ad_value(903)), s.ad_value(1070)), s.ad_value(1090)), 1091);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub(1038, 1037, 825);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(853, 219, 832);
        }

        s.v[1213] = if (((s.v[1038] - s.v[220]) / s.v[853]) > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1213] != 0.0)) {
            s.store_scale_ad(1039, A::offset(A::offset(A::div(A::sub(s.ad_value(1038), s.ad_value(220)), s.ad_value(853)), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1214] = if (((s.v[1038] - s.v[220]) / s.v[853]) < (-100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1213] != 0.0))) && (s.v[1214] != 0.0)) {
            s.store_scalar(1039, 3.720075976e-44);
        }

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1213] != 0.0))) && (!(s.v[1214] != 0.0))) {
            s.store_exp_ad(1039, A::div(A::sub(s.ad_value(1038), s.ad_value(220)), s.ad_value(853)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_rhs(1042, 853, A::ln(A::offset(s.ad_value(1039), 1.0)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub(1040, 825, 1037);
        }

        s.v[1215] = if (((s.v[1040] - s.v[220]) / s.v[853]) > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1215] != 0.0)) {
            s.store_scale_ad(1041, A::offset(A::offset(A::div(A::sub(s.ad_value(1040), s.ad_value(220)), s.ad_value(853)), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1216] = if (((s.v[1040] - s.v[220]) / s.v[853]) < (-100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1215] != 0.0))) && (s.v[1216] != 0.0)) {
            s.store_scalar(1041, 3.720075976e-44);
        }

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1215] != 0.0))) && (!(s.v[1216] != 0.0))) {
            s.store_exp_ad(1041, A::div(A::sub(s.ad_value(1040), s.ad_value(220)), s.ad_value(853)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_rhs(1043, 853, A::ln(A::offset(s.ad_value(1041), 1.0)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(844, A::mul(A::mul(s.ad_value(226), s.ad_value(376)), s.ad_value(832)), 832);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(845, 1043, A::mul(A::scale(s.ad_value(405), 2.0), A::sqrt(s.ad_value(942))));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(843, A::div(A::mul(s.ad_value(1043), s.ad_value(845)), s.ad_value(844)), 1.0);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(1034, 942, A::mul(s.ad_value(832), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_rhs(843, 396, A::add(s.ad_value(396), A::div_from_scalar(1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913])))));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1035, 1034, A::mul(s.ad_value(843), s.ad_value(1042)));
        }

        s.v[1217] = if (p.p432 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_div_ad_lhs(843, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_mul_ad_rhs(844, 224, A::add(A::exp(A::scale(s.ad_value(843), 0.5)), A::scale(A::exp(s.ad_value(843)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::sub(s.ad_value(940), s.ad_value(942)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_add_ad_lhs(1036, A::add(A::sub(s.ad_value(1035), s.ad_value(846)), s.ad_value(216)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(A::neg(s.ad_value(223)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_mul_ad_rhs(848, 222, A::add(A::exp(A::scale(s.ad_value(846), 0.5)), A::scale(A::exp(s.ad_value(846)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_mul(845, 844, 902);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1217] != 0.0)) {
            s.store_add_ad_lhs(1031, A::mul(s.ad_value(843), s.ad_value(1036)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::add(A::offset(s.ad_value(393), s.v[913]), s.ad_value(218)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_div_ad_lhs(844, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_mul_ad_rhs(845, 224, A::add(A::exp(A::scale(s.ad_value(844), 0.5)), A::scale(A::exp(s.ad_value(844)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_mul_ad_rhs(846, 845, A::add(s.ad_value(822), s.ad_value(217)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_div_ad_lhs(847, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_mul_ad(848, A::mul(s.ad_value(393), s.ad_value(843)), A::add(A::sub(s.ad_value(1035), s.ad_value(847)), s.ad_value(216)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_mul_ad_lhs(849, A::mul(s.ad_value(218), s.ad_value(843)), 846);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_add(1036, 848, 849);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_mul_ad_lhs(850, A::scale(s.ad_value(843), s.v[913]), 902);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1217] != 0.0))) {
            s.store_add(1031, 1036, 850);
        }

        s.v[1218] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1218] != 0.0)) {
            s.store_offset(1030, 1031, 0.02);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1218] != 0.0)) {
            s.store_offset(824, 1031, 0.02);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1218] != 0.0))) {
            s.store_offset_ad(844, A::sub(s.ad_value(824), A::offset(s.ad_value(1031), 0.02)), (-0.01));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1218] != 0.0))) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(844)), 0.0001));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1218] != 0.0))) {
            s.store_add_ad(1030, A::offset(s.ad_value(1031), 0.02), A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(844, A::sub(s.ad_value(1036), s.ad_value(1030)), (-0.005));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(844)), 2.5e-5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_scaled_add(846, 844, 845, 0.5);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(847, A::mul(s.ad_value(846), s.ad_value(393)), 344);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1033, 1030, A::mul(A::scale(s.ad_value(846), 0.5), s.ad_value(847)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub(1060, 1052, 825);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul(853, 219, 832);
        }

        s.v[1219] = if (((s.v[1060] - s.v[220]) / s.v[853]) > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1219] != 0.0)) {
            s.store_scale_ad(1061, A::offset(A::offset(A::div(A::sub(s.ad_value(1060), s.ad_value(220)), s.ad_value(853)), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1220] = if (((s.v[1060] - s.v[220]) / s.v[853]) < (-100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1219] != 0.0))) && (s.v[1220] != 0.0)) {
            s.store_scalar(1061, 3.720075976e-44);
        }

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1219] != 0.0))) && (!(s.v[1220] != 0.0))) {
            s.store_exp_ad(1061, A::div(A::sub(s.ad_value(1060), s.ad_value(220)), s.ad_value(853)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_rhs(1064, 853, A::ln(A::offset(s.ad_value(1061), 1.0)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub(1062, 825, 1052);
        }

        s.v[1221] = if (((s.v[1062] - s.v[220]) / s.v[853]) > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scale_ad(1063, A::offset(A::offset(A::div(A::sub(s.ad_value(1062), s.ad_value(220)), s.ad_value(853)), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1222] = if (((s.v[1062] - s.v[220]) / s.v[853]) < (-100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1221] != 0.0))) && (s.v[1222] != 0.0)) {
            s.store_scalar(1063, 3.720075976e-44);
        }

        if (((!(s.v[1202] != 0.0)) && (!(s.v[1221] != 0.0))) && (!(s.v[1222] != 0.0))) {
            s.store_exp_ad(1063, A::div(A::sub(s.ad_value(1062), s.ad_value(220)), s.ad_value(853)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_rhs(1065, 853, A::ln(A::offset(s.ad_value(1063), 1.0)));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_mul_ad_lhs(844, A::mul(A::mul(s.ad_value(226), s.ad_value(376)), s.ad_value(832)), 832);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(845, 1065, A::mul(A::scale(s.ad_value(405), 2.0), A::sqrt(s.ad_value(942))));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(843, A::div(A::mul(s.ad_value(1065), s.ad_value(845)), s.ad_value(844)), 1.0);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_add_ad_rhs(1049, 942, A::mul(s.ad_value(832), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_rhs(843, 396, A::add(s.ad_value(396), A::div_from_scalar(1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913])))));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1050, 1049, A::mul(s.ad_value(843), s.ad_value(1064)));
        }

        s.v[1223] = if (p.p432 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_div_ad_lhs(843, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_mul_ad_rhs(844, 224, A::add(A::exp(A::scale(s.ad_value(843), 0.5)), A::scale(A::exp(s.ad_value(843)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::sub(s.ad_value(940), s.ad_value(942)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_add_ad_lhs(1051, A::add(A::sub(s.ad_value(1050), s.ad_value(846)), s.ad_value(216)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_div_ad_lhs(846, A::scale(A::neg(s.ad_value(223)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_mul_ad_rhs(848, 222, A::add(A::exp(A::scale(s.ad_value(846), 0.5)), A::scale(A::exp(s.ad_value(846)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(s.ad_value(221), s.ad_value(848)), 843);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_mul(845, 844, 902);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0));
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_add_ad_lhs(1047, A::mul(s.ad_value(843), s.ad_value(1051)), 845);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::add(A::offset(s.ad_value(393), s.v[913]), s.ad_value(218)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_div_ad_lhs(844, A::scale(A::neg(s.ad_value(225)), s.v[327]), 119);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_mul_ad_rhs(845, 224, A::add(A::exp(A::scale(s.ad_value(844), 0.5)), A::scale(A::exp(s.ad_value(844)), 2.0)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_mul_ad_rhs(846, 845, A::add(s.ad_value(822), s.ad_value(217)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_div_ad_lhs(847, A::scale(s.ad_value(344), 0.5), 393);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_mul_ad(848, A::mul(s.ad_value(393), s.ad_value(843)), A::add(A::sub(s.ad_value(1050), s.ad_value(847)), s.ad_value(216)));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_mul_ad_lhs(849, A::mul(s.ad_value(218), s.ad_value(843)), 846);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_add(1051, 848, 849);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_mul_ad_lhs(850, A::scale(s.ad_value(843), s.v[913]), 902);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_add(1047, 1051, 850);
        }

        s.v[1224] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1202] != 0.0)) && (s.v[1224] != 0.0)) {
            s.store_offset(1046, 1047, 0.02);
        }

        if ((!(s.v[1202] != 0.0)) && (s.v[1224] != 0.0)) {
            s.store_offset(824, 1047, 0.02);
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1224] != 0.0))) {
            s.store_offset_ad(844, A::sub(s.ad_value(824), A::offset(s.ad_value(1047), 0.02)), (-0.01));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1224] != 0.0))) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(844)), 0.0001));
        }

        if ((!(s.v[1202] != 0.0)) && (!(s.v[1224] != 0.0))) {
            s.store_add_ad(1046, A::offset(s.ad_value(1047), 0.02), A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_offset_ad(844, A::sub(s.ad_value(1051), s.ad_value(1046)), (-0.005));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sqrt_ad(845, A::offset(A::square(s.ad_value(844)), 2.5e-5));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_scaled_add(846, 844, 845, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1202] != 0.0)) {
            s.store_div_ad_lhs(847, A::mul(s.ad_value(846), s.ad_value(393)), 344);
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_ad_rhs(1048, 1046, A::mul(A::scale(s.ad_value(846), 0.5), s.ad_value(847)));
        }

        s.store_offset(843, 1033, ((5.0) + ((-0.001))));

        s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), (-(0.004 * (-5.0)))));

        s.store_offset_ad(845, A::scale(A::add(s.ad_value(843), s.ad_value(844)), 0.5), (-5.0));

        s.v[843] = 1.5;

        s.store_offset_ad(844, A::sub_from_scalar(s.v[843], s.ad_value(845)), (-0.002));

        s.store_sqrt_ad(846, A::offset(A::square(s.ad_value(844)), (0.008 * s.v[843])));

        s.store_sub_from_scalar_ad(962, s.v[843], A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));

        s.store_scale(843, 942, 0.95);

        s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(962)), (-0.002));

        s.store_sqrt_ad(845, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), 0.008)));

        s.store_sub_ad_rhs(841, 843, A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));

        s.store_offset(843, 1048, ((5.0) + ((-0.001))));

        s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), (-(0.004 * (-5.0)))));

        s.store_offset_ad(845, A::scale(A::add(s.ad_value(843), s.ad_value(844)), 0.5), (-5.0));

        s.v[843] = 1.5;

        s.store_offset_ad(844, A::sub_from_scalar(s.v[843], s.ad_value(845)), (-0.002));

        s.store_sqrt_ad(846, A::offset(A::square(s.ad_value(844)), (0.008 * s.v[843])));

        s.store_sub_from_scalar_ad(1045, s.v[843], A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));

        s.store_scale(843, 942, 0.95);

        s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(1045)), (-0.002));

        s.store_sqrt_ad(845, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), 0.008)));

        s.store_sub_ad_rhs(1044, 843, A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));

        s.store_sub(827, 942, 841);

        s.store_sqrt(828, 827);

        s.store_div_ad_lhs(864, A::mul(s.ad_value(944), s.ad_value(828)), 943);

        s.store_mul_ad(71, A::scale(s.ad_value(49), 6.241509744511525e18), A::add(A::add(s.ad_value(396), A::div(s.ad_value(417), s.ad_value(864))), s.ad_value(99)));

        s.store_sqrt(846, 864);

        s.store_mul(843, 131, 841);

        s.v[1225] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1225] != 0.0) {
            s.store_offset(844, 843, 1.0);
        }

        if (!(s.v[1225] != 0.0)) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if (!(s.v[1225] != 0.0)) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        s.store_mul_ad_lhs(865, A::mul(s.ad_value(397), s.ad_value(846)), 844);

        s.store_mul(843, 134, 841);

        s.v[1226] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1226] != 0.0) {
            s.store_offset(844, 843, 1.0);
        }

        if (!(s.v[1226] != 0.0)) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if (!(s.v[1226] != 0.0)) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        s.store_mul_ad_lhs(866, A::mul(s.ad_value(397), s.ad_value(846)), 844);

        s.store_div_ad_lhs(843, A::scale(s.ad_value(130), ((-0.5) * s.v[892])), 865);

        s.v[1227] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[1227] != 0.0) {
            s.store_exp(844, 843);
        }

        if (s.v[1227] != 0.0) {
            s.store_mul_ad_rhs(868, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1227] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (!(s.v[1227] != 0.0)) {
            s.store_mul_ad_rhs(868, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        s.store_div_ad_lhs(845, A::mul(s.ad_value(100), s.ad_value(417)), 864);

        s.store_add_ad(846, A::add(s.ad_value(96), A::mul(s.ad_value(97), s.ad_value(841))), A::mul(s.ad_value(98), s.ad_value(822)));

        s.store_div_ad_lhs(847, A::add(A::add(s.ad_value(845), A::mul(s.ad_value(846), s.ad_value(868))), s.ad_value(99)), 396);

        s.v[1228] = if (s.v[847] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1228] != 0.0) {
            s.store_offset(831, 847, 1.0);
        }

        if (!(s.v[1228] != 0.0)) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::scale(s.ad_value(847), 8.0), 3.0));
        }

        if (!(s.v[1228] != 0.0)) {
            s.store_mul_ad_lhs(831, A::offset(A::scale(s.ad_value(847), 3.0), 1.0), 843);
        }

        s.v[1229] = if (s.v[378] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1229] != 0.0) {
            s.store_mul_ad_lhs(843, A::neg(s.ad_value(379)), 822);
        }

        s.v[1230] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1229] != 0.0) && (s.v[1230] != 0.0)) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.v[1229] != 0.0) && (!(s.v[1230] != 0.0))) {
            s.store_exp(845, 843);
        }

        if (s.v[1229] != 0.0) {
            s.store_offset_ad(846, A::mul(s.ad_value(378), A::offset(s.ad_value(845), 1.0)), s.v[892]);
        }

        if (s.v[1229] != 0.0) {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.v[1229] != 0.0) {
            s.store_mul(1090, 831, 847);
        }

        if (!(s.v[1229] != 0.0)) {
            s.store_scalar(1090, 0.0);
        }

        s.store_mul(63, 129, 868);

        s.store_mul(867, 63, 834);

        s.store_div_ad_lhs(843, A::scale(s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892]))), 866);

        s.v[1231] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[1231] != 0.0) {
            s.store_exp(844, 843);
        }

        if (s.v[1231] != 0.0) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1231] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (!(s.v[1231] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        s.store_mul(843, 132, 845);

        s.store_mul(904, 843, 834);

        s.store_sqrt_ad(843, A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[892])), 1.0));

        s.store_add_ad(844, A::add(s.ad_value(121), A::scale(s.ad_value(122), 1.0 / (s.v[892]))), A::mul(s.ad_value(123), s.ad_value(841)));

        s.store_add_ad(903, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(843), (-1.0))), s.ad_value(943)), A::mul(s.ad_value(844), s.ad_value(430)));

        s.store_div_ad(870, A::mul(s.ad_value(415), s.ad_value(942)), A::offset(s.ad_value(127), s.v[328]));

        s.store_add_ad_rhs(846, 400, A::mul(s.ad_value(188), s.ad_value(841)));

        s.v[1232] = if (s.v[846] < 0.0001) { 1.0 } else { 0.0 };

        if (s.v[1232] != 0.0) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(846), 20000.0)));
        }

        if (s.v[1232] != 0.0) {
            s.store_mul_ad_lhs(846, A::sub_from_scalar(0.0002, s.ad_value(846)), 852);
        }

        s.store_mul_ad_lhs(873, A::mul(s.ad_value(846), s.ad_value(1141)), 822);

        s.store_sqrt_ad(1089, A::offset(A::scale(s.ad_value(377), 1.0 / (s.v[892])), 1.0));

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_sub_ad_rhs(963, 828, A::mul(s.ad_value(852), A::sub(s.ad_value(962), s.ad_value(841))));

        s.store_exp_ad(843, A::mul(A::scale(s.ad_value(382), 2.0), s.ad_value(822)));

        s.store_div_ad(1091, A::mul(s.ad_value(391), A::offset(s.ad_value(843), (-1.0))), A::offset(s.ad_value(843), 1.0));

        let assign17020_ad_e15496: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(408), p.p37), A::mul(A::sub(A::mul(s.ad_value(376), s.ad_value(963)), A::mul(s.ad_value(405), s.ad_value(943))), s.ad_value(1089))), A::mul(s.ad_value(403), s.ad_value(841))), s.ad_value(867)), s.ad_value(904)), A::mul(A::add(s.ad_value(125), A::mul(s.ad_value(126), s.ad_value(841))), s.ad_value(870)));
        s.store_sub_ad_lhs(829, A::sub(A::sub(A::add(assign17020_ad_e15496, s.ad_value(903)), s.ad_value(873)), s.ad_value(1090)), 1091);

        s.store_sub(1053, 942, 1044);

        s.store_sqrt(1054, 1053);

        s.store_div_ad_lhs(1055, A::mul(s.ad_value(944), s.ad_value(1054)), 943);

        s.store_mul_ad(71, A::scale(s.ad_value(49), 6.241509744511525e18), A::add(A::add(s.ad_value(396), A::div(s.ad_value(417), s.ad_value(1055))), s.ad_value(99)));

        s.store_sqrt(846, 1055);

        s.store_mul(843, 131, 1044);

        s.v[1233] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1233] != 0.0) {
            s.store_offset(844, 843, 1.0);
        }

        if (!(s.v[1233] != 0.0)) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if (!(s.v[1233] != 0.0)) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        s.store_mul_ad_lhs(1056, A::mul(s.ad_value(397), s.ad_value(846)), 844);

        s.store_mul(843, 134, 1044);

        s.v[1234] = if (s.v[843] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1234] != 0.0) {
            s.store_offset(844, 843, 1.0);
        }

        if (!(s.v[1234] != 0.0)) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(A::scale(s.ad_value(843), 8.0), 3.0));
        }

        if (!(s.v[1234] != 0.0)) {
            s.store_mul_ad_lhs(844, A::offset(A::scale(s.ad_value(843), 3.0), 1.0), 847);
        }

        s.store_mul_ad_lhs(1057, A::mul(s.ad_value(397), s.ad_value(846)), 844);

        s.store_div_ad_lhs(843, A::scale(s.ad_value(130), ((-0.5) * s.v[892])), 1056);

        s.v[1235] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[1235] != 0.0) {
            s.store_exp(844, 843);
        }

        if (s.v[1235] != 0.0) {
            s.store_mul_ad_rhs(1058, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1235] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (!(s.v[1235] != 0.0)) {
            s.store_mul_ad_rhs(1058, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        s.store_div_ad_lhs(845, A::mul(s.ad_value(100), s.ad_value(417)), 1055);

        s.store_add_ad(846, A::add(s.ad_value(96), A::mul(s.ad_value(97), s.ad_value(1044))), A::mul(s.ad_value(98), s.ad_value(822)));

        s.store_div_ad_lhs(847, A::add(A::add(s.ad_value(845), A::mul(s.ad_value(846), s.ad_value(1058))), s.ad_value(99)), 396);

        s.v[1236] = if (s.v[847] >= (-0.5)) { 1.0 } else { 0.0 };

        if (s.v[1236] != 0.0) {
            s.store_offset(1059, 847, 1.0);
        }

        if (!(s.v[1236] != 0.0)) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::scale(s.ad_value(847), 8.0), 3.0));
        }

        if (!(s.v[1236] != 0.0)) {
            s.store_mul_ad_lhs(1059, A::offset(A::scale(s.ad_value(847), 3.0), 1.0), 843);
        }

        s.v[1237] = if (s.v[378] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1237] != 0.0) {
            s.store_mul_ad_lhs(843, A::neg(s.ad_value(379)), 822);
        }

        s.v[1238] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1237] != 0.0) && (s.v[1238] != 0.0)) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.v[1237] != 0.0) && (!(s.v[1238] != 0.0))) {
            s.store_exp(845, 843);
        }

        if (s.v[1237] != 0.0) {
            s.store_offset_ad(846, A::mul(s.ad_value(378), A::offset(s.ad_value(845), 1.0)), s.v[892]);
        }

        if (s.v[1237] != 0.0) {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.v[1237] != 0.0) {
            s.store_mul(1071, 1059, 847);
        }

        if (!(s.v[1237] != 0.0)) {
            s.store_scalar(1071, 0.0);
        }

        s.store_mul(63, 129, 1058);

        s.store_mul(1067, 63, 834);

        s.store_div_ad_lhs(843, A::scale(s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892]))), 1057);

        s.v[1239] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if (s.v[1239] != 0.0) {
            s.store_exp(844, 843);
        }

        if (s.v[1239] != 0.0) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (!(s.v[1239] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (!(s.v[1239] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        s.store_mul(843, 132, 845);

        s.store_mul(1068, 843, 834);

        s.store_sqrt_ad(843, A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[892])), 1.0));

        s.store_add_ad(844, A::add(s.ad_value(121), A::scale(s.ad_value(122), 1.0 / (s.v[892]))), A::mul(s.ad_value(123), s.ad_value(1044)));

        s.store_add_ad(1069, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(843), (-1.0))), s.ad_value(943)), A::mul(s.ad_value(844), s.ad_value(430)));

        s.store_div_ad(1066, A::mul(s.ad_value(415), s.ad_value(942)), A::offset(s.ad_value(127), s.v[328]));

        s.store_add_ad_rhs(846, 401, A::mul(s.ad_value(190), s.ad_value(1044)));

        s.v[1240] = if (s.v[846] < 0.0001) { 1.0 } else { 0.0 };

        if (s.v[1240] != 0.0) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(846), 20000.0)));
        }

        if (s.v[1240] != 0.0) {
            s.store_mul_ad_lhs(846, A::sub_from_scalar(0.0002, s.ad_value(846)), 852);
        }

        s.store_mul_ad_lhs(1070, A::mul(s.ad_value(846), s.ad_value(1141)), 822);

        s.store_sqrt_ad(1089, A::offset(A::scale(s.ad_value(377), 1.0 / (s.v[892])), 1.0));

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_sub_ad_rhs(1072, 1054, A::mul(s.ad_value(852), A::sub(s.ad_value(1045), s.ad_value(1044))));

        s.store_exp_ad(843, A::mul(A::scale(s.ad_value(382), 2.0), s.ad_value(822)));

        s.store_div_ad(1091, A::mul(s.ad_value(391), A::offset(s.ad_value(843), (-1.0))), A::offset(s.ad_value(843), 1.0));

        let assign17670_ad_e15953: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(408), p.p37), A::mul(A::sub(A::mul(s.ad_value(376), s.ad_value(1072)), A::mul(s.ad_value(405), s.ad_value(943))), s.ad_value(1089))), A::mul(s.ad_value(403), s.ad_value(1044))), s.ad_value(1067)), s.ad_value(1068)), A::mul(A::add(s.ad_value(125), A::mul(s.ad_value(126), s.ad_value(1044))), s.ad_value(1066)));
        s.store_sub_ad_lhs(1073, A::sub(A::sub(A::add(assign17670_ad_e15953, s.ad_value(1069)), s.ad_value(1070)), s.ad_value(1071)), 1091);

        s.v[1241] = if (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1241] != 0.0) {
            s.store_sqrt(1007, 944);
        }

        if (s.v[1241] != 0.0) {
            s.store_mul(1008, 397, 1007);
        }

        if (s.v[1241] != 0.0) {
            s.store_mul(1009, 397, 1007);
        }

        if (s.v[1241] != 0.0) {
            s.store_div_ad_lhs(843, A::scale(s.ad_value(130), ((-0.5) * s.v[892])), 1008);
        }

        s.v[1242] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1241] != 0.0) && (s.v[1242] != 0.0)) {
            s.store_exp(844, 843);
        }

        if ((s.v[1241] != 0.0) && (s.v[1242] != 0.0)) {
            s.store_mul_ad_rhs(1010, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if ((s.v[1241] != 0.0) && (!(s.v[1242] != 0.0))) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.v[1241] != 0.0) && (!(s.v[1242] != 0.0))) {
            s.store_mul_ad_rhs(1010, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (s.v[1241] != 0.0) {
            s.store_mul_ad_lhs(1011, A::mul(s.ad_value(129), s.ad_value(1010)), 834);
        }

        if (s.v[1241] != 0.0) {
            s.store_div_ad_lhs(843, A::scale(s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892]))), 1009);
        }

        s.v[1243] = if (s.v[843] > (-100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1241] != 0.0) && (s.v[1243] != 0.0)) {
            s.store_exp(844, 843);
        }

        if ((s.v[1241] != 0.0) && (s.v[1243] != 0.0)) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if ((s.v[1241] != 0.0) && (!(s.v[1243] != 0.0))) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.v[1241] != 0.0) && (!(s.v[1243] != 0.0))) {
            s.store_mul_ad_rhs(845, 844, A::offset(A::scale(s.ad_value(844), 2.0), 1.0));
        }

        if (s.v[1241] != 0.0) {
            s.store_mul(843, 132, 845);
        }

        if (s.v[1241] != 0.0) {
            s.store_mul(1012, 843, 834);
        }

        if (s.v[1241] != 0.0) {
            s.store_sqrt_ad(843, A::offset(A::scale(s.ad_value(128), 1.0 / (s.v[892])), 1.0));
        }

        if (s.v[1241] != 0.0) {
            s.store_add_ad_rhs(844, 121, A::scale(s.ad_value(122), 1.0 / (s.v[892])));
        }

        if (s.v[1241] != 0.0) {
            s.store_add_ad(1013, A::mul(A::mul(s.ad_value(376), A::offset(s.ad_value(843), (-1.0))), s.ad_value(943)), A::mul(s.ad_value(844), s.ad_value(430)));
        }

        if (s.v[1241] != 0.0) {
            s.store_add_ad_lhs(1014, A::add(A::sub(A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(1011)), s.ad_value(1012)), A::mul(s.ad_value(125), s.ad_value(1066))), 1013);
        }

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1241] != 0.0)) {
            s.store_scalar(1014, 0.0);
        }

        s.store_sub(830, 825, 829);

        s.store_mul(853, 831, 832);

        s.store_div_ad_lhs(809, A::mul(s.ad_value(384), s.ad_value(830)), 853);

        s.store_div_ad_lhs(833, A::sub(s.ad_value(151), A::mul(A::sub_from_scalar(1.0, s.ad_value(384)), s.ad_value(830))), 853);

        s.v[1244] = if (s.v[809] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[1244] != 0.0) {
            s.copy_ad(875, 830);
        }

        if (s.v[1244] != 0.0) {
            s.store_scalar(810, 0.0);
        }

        s.v[1245] = if (s.v[833] > 100.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1244] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_div_ad(843, A::sub(s.ad_value(830), s.ad_value(151)), A::mul(s.ad_value(831), s.ad_value(832)));
        }

        if ((!(s.v[1244] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_exp(810, 843);
        }

        if ((!(s.v[1244] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_mul_ad_lhs(875, A::div(A::mul(s.ad_value(832), s.ad_value(1140)), s.ad_value(396)), 810);
        }

        if ((!(s.v[1244] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_exp(810, 809);
        }

        if ((!(s.v[1244] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_mul_ad_rhs(844, 853, A::ln(A::offset(s.ad_value(810), 1.0)));
        }

        if ((!(s.v[1244] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_mul_ad(857, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(832), s.ad_value(1140))), A::exp(s.ad_value(833))), A::sub_from_scalar(1.0, s.ad_value(384)));
        }

        if ((!(s.v[1244] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_sub_ad_rhs(845, 384, A::div(A::mul(s.ad_value(853), s.ad_value(857)), A::sub_from_scalar(1.0, s.ad_value(384))));
        }

        if ((!(s.v[1244] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_div(875, 844, 845);
        }

        s.store_add_ad_rhs(890, 875, A::scale(s.ad_value(832), 2.0));

        s.copy_ad(72, 875);

        s.v[1246] = if (s.v[385] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1246] != 0.0) {
            s.store_scalar(1092, 1.0);
        }

        if (!(s.v[1246] != 0.0)) {
            s.store_div_ad_lhs(852, A::scale(s.ad_value(385), ((s.v[892]) as f64).sqrt()), 890);
        }

        if (!(s.v[1246] != 0.0)) {
            s.store_div_from_scalar_ad(1092, 1.0, A::offset(s.ad_value(852), 1.0));
        }

        s.store_sub(852, 828, 943);

        s.store_sub_from_scalar_ad(893, s.v[328], A::scale(A::add(A::mul(s.ad_value(197), s.ad_value(875)), A::mul(s.ad_value(198), s.ad_value(852))), (2.0 - p.p22)));

        s.v[1247] = if (s.v[893] < 2e-8) { 1.0 } else { 0.0 };

        if (s.v[1247] != 0.0) {
            s.store_div_from_scalar_ad(843, 1.0, A::sub_from_scalar(6e-8, A::scale(s.ad_value(893), 2.0)));
        }

        if (s.v[1247] != 0.0) {
            s.store_mul_ad_lhs(893, A::scale(A::sub_from_scalar(4e-8, s.ad_value(893)), 2e-8), 843);
        }

        s.v[1248] = if (p.p429 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1248] != 0.0) {
            s.store_scalar(887, 0.0);
        }

        if (!(s.v[1248] != 0.0)) {
            s.store_add_ad(843, A::mul(s.ad_value(183), s.ad_value(875)), A::mul(s.ad_value(184), s.ad_value(852)));
        }

        s.v[1249] = if (s.v[843] >= (-0.9)) { 1.0 } else { 0.0 };

        if ((!(s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) {
            s.store_mul_ad_rhs(887, 955, A::offset(s.ad_value(843), 1.0));
        }

        if ((!(s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::offset(A::scale(s.ad_value(843), 20.0), 17.0));
        }

        if ((!(s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) {
            s.store_mul_ad_lhs(887, A::mul(s.ad_value(955), A::offset(s.ad_value(843), 0.8)), 844);
        }

        s.store_offset_scaled(1101, 430, p.p137, p.p135);

        s.store_offset_scaled(1102, 430, p.p138, p.p136);

        s.v[1250] = if (p.p429 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1250] != 0.0) {
            s.store_add_ad_lhs(887, A::add(A::add(A::add(s.ad_value(61), s.ad_value(887)), s.ad_value(60)), s.ad_value(1102)), 1101);
        }

        s.store_scale(73, 887, 1.0 / (p.p3));

        s.v[1251] = if (s.v[103] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1251] != 0.0) {
            s.store_scalar(860, 1.0);
        }

        if (s.v[1251] != 0.0) {
            s.store_scalar(861, 1.0);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul(853, 107, 962);
        }

        s.v[1252] = if (s.v[853] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[1251] != 0.0)) && (s.v[1252] != 0.0)) {
            s.store_div_from_scalar_ad(854, 1.0, A::offset(s.ad_value(853), 1.0));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1252] != 0.0))) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1252] != 0.0))) {
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1252] != 0.0))) {
            s.store_add_ad_lhs(854, A::mul(s.ad_value(855), s.ad_value(853)), 964);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_add(853, 942, 266);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_div_ad_lhs(964, A::mul(s.ad_value(962), s.ad_value(854)), 853);
        }

        s.v[1253] = if (s.v[964] < 0.5) { 1.0 } else { 0.0 };

        if ((!(s.v[1251] != 0.0)) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar_ad(965, 1.0, A::sqrt(A::sub_from_scalar(1.0, s.ad_value(964))));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1253] != 0.0))) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1253] != 0.0))) {
            s.store_sub_from_scalar_ad(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(854), 0.5));
        }

        if ((!(s.v[1251] != 0.0)) && (!(s.v[1253] != 0.0))) {
            s.store_add_ad_lhs(965, A::mul(s.ad_value(854), s.ad_value(964)), 855);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_div_ad(853, A::mul(A::scale(s.ad_value(376), 0.5), s.ad_value(1089)), A::sqrt(A::add(s.ad_value(942), s.ad_value(266))));
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul(844, 853, 965);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_sqrt_ad(852, A::mul(s.ad_value(242), s.ad_value(864)));
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_div_from_scalar(848, s.v[892], 869);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul(870, 103, 848);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_offset(871, 200, s.v[328]);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_div(872, 199, 871);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_add(845, 870, 872);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_square(849, 848);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul(850, 848, 849);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_offset_ad(861, A::mul(s.ad_value(844), s.ad_value(845)), 1.0);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul_ad_lhs(851, A::mul(s.ad_value(104), s.ad_value(103)), 850);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_mul_ad_lhs(879, A::neg(s.ad_value(844)), 851);
        }

        if (!(s.v[1251] != 0.0)) {
            s.store_add_ad_rhs(860, 861, A::mul(s.ad_value(879), s.ad_value(875)));
        }

        s.v[1254] = if (s.v[861] < 0.01) { 1.0 } else { 0.0 };

        if (s.v[1254] != 0.0) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(861), 200.0)));
        }

        if (s.v[1254] != 0.0) {
            s.store_mul_ad_lhs(861, A::sub_from_scalar(0.02, s.ad_value(861)), 852);
        }

        s.v[1255] = if (s.v[860] < 0.01) { 1.0 } else { 0.0 };

        if (s.v[1255] != 0.0) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(860), 200.0)));
        }

        if (s.v[1255] != 0.0) {
            s.store_mul_ad_lhs(860, A::sub_from_scalar(0.02, s.ad_value(860)), 852);
        }

        s.copy_ad(74, 860);

        s.v[1256] = if (s.v[103] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1256] != 0.0) {
            s.store_scalar(1074, 1.0);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_mul(853, 107, 1045);
        }

        s.v[1257] = if (s.v[853] >= (-0.5)) { 1.0 } else { 0.0 };

        if ((!(s.v[1256] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar_ad(854, 1.0, A::offset(s.ad_value(853), 1.0));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_add_ad_lhs(854, A::mul(s.ad_value(855), s.ad_value(853)), 964);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_add(853, 942, 266);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_div_ad_lhs(964, A::mul(s.ad_value(1045), s.ad_value(854)), 853);
        }

        s.v[1258] = if (s.v[964] < 0.5) { 1.0 } else { 0.0 };

        if ((!(s.v[1256] != 0.0)) && (s.v[1258] != 0.0)) {
            s.store_div_from_scalar_ad(965, 1.0, A::sqrt(A::sub_from_scalar(1.0, s.ad_value(964))));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1258] != 0.0))) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1258] != 0.0))) {
            s.store_sub_from_scalar_ad(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), A::scale(s.ad_value(854), 0.5));
        }

        if ((!(s.v[1256] != 0.0)) && (!(s.v[1258] != 0.0))) {
            s.store_add_ad_lhs(965, A::mul(s.ad_value(854), s.ad_value(964)), 855);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_div_ad(853, A::mul(A::scale(s.ad_value(376), 0.5), s.ad_value(1089)), A::sqrt(A::add(s.ad_value(942), s.ad_value(266))));
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_mul(844, 853, 965);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_sqrt_ad(852, A::mul(s.ad_value(242), s.ad_value(1055)));
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_div_from_scalar(848, s.v[892], 869);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_mul(870, 103, 848);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_offset(871, 200, s.v[328]);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_div(872, 199, 871);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_add(845, 870, 872);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_square(849, 848);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_mul(850, 848, 849);
        }

        if (!(s.v[1256] != 0.0)) {
            s.store_offset_ad(1074, A::mul(s.ad_value(844), s.ad_value(845)), 1.0);
        }

        s.v[1259] = if (s.v[1074] < 0.01) { 1.0 } else { 0.0 };

        if (s.v[1259] != 0.0) {
            s.store_div_from_scalar_ad(852, 1.0, A::sub_from_scalar(3.0, A::scale(s.ad_value(1074), 200.0)));
        }

        if (s.v[1259] != 0.0) {
            s.store_mul_ad_lhs(1074, A::sub_from_scalar(0.02, s.ad_value(1074)), 852);
        }

        if (p.p41 != 0.0) {
            s.store_scale_ad(965, A::offset(A::sub_from_scalar((p.p52 - p.p53), A::scale(s.ad_value(912), 0.5)), 0.45), (2.0 * p.p37));
        }

        if (p.p41 != 0.0) {
            s.store_scalar(1109, ((p.p45 * p.p47) / 3.9));
        }

        if (p.p41 != 0.0) {
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(965, 0.0);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scalar(1109, p.p66);
        }

        if (!(p.p41 != 0.0)) {
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        s.v[1260] = if (p.p62 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1260] != 0.0) {
            s.store_sub_ad_lhs(843, A::add(A::add(s.ad_value(875), s.ad_value(829)), s.ad_value(829)), 965);
        }

        if (s.v[1260] != 0.0) {
            s.store_add_ad_rhs(845, 956, A::mul(s.ad_value(958), s.ad_value(841)));
        }

        if (s.v[1260] != 0.0) {
            s.store_div(846, 843, 1109);
        }

        if (s.v[1260] != 0.0) {
            s.store_mul_ad_rhs(848, 846, A::add(A::add(s.ad_value(845), s.ad_value(856)), A::mul(s.ad_value(957), s.ad_value(846))));
        }

        s.v[1261] = if (p.p62 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_mul_ad(848, A::div(A::sub(s.ad_value(875), s.ad_value(965)), s.ad_value(415)), A::add(A::add(A::add(s.ad_value(956), A::mul(s.ad_value(958), s.ad_value(841))), s.ad_value(856)), A::div(A::mul(s.ad_value(957), A::sub(s.ad_value(875), s.ad_value(965))), s.ad_value(415))));
        }

        s.v[1262] = if (p.p62 == 3.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_sub_ad_lhs(843, A::add(A::add(s.ad_value(875), s.ad_value(829)), s.ad_value(829)), 965);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_offset_ad(845, A::mul(s.ad_value(958), s.ad_value(841)), 1.0);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_div(846, 843, 1109);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_mul_ad_rhs(847, 846, A::add(s.ad_value(956), A::mul(s.ad_value(957), s.ad_value(846))));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_mul(848, 847, 845);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(843, A::div(A::scale(A::add(s.ad_value(875), s.ad_value(68)), 1e-8), s.ad_value(415)), 0.16666666666666666);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_exp_ad(844, A::mul(s.ad_value(148), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_add_ad_rhs(845, 956, A::mul(s.ad_value(958), s.ad_value(841)));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_mul_ad_rhs(1157, 149, A::pow(s.ad_value(411), s.ad_value(150)));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_mul_ad_rhs(1158, 146, A::pow(s.ad_value(411), s.ad_value(147)));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.copy_ad(1108, 69);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_exp_ad(853, A::mul(s.ad_value(1157), {
                if ((1.0 + (s.v[875] / s.v[1108])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(875), s.ad_value(1108)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_div(854, 1158, 853);
        }

        if (((!(s.v[1260] != 0.0)) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_add_ad_lhs(848, A::mul(s.ad_value(844), s.ad_value(845)), 854);
        }

        s.v[1263] = if (s.v[848] >= (-0.8)) { 1.0 } else { 0.0 };

        if (s.v[1263] != 0.0) {
            s.store_offset(936, 848, 1.0);
        }

        if (!(s.v[1263] != 0.0)) {
            s.store_div_from_scalar_ad(852, 1.0, A::offset(A::scale(s.ad_value(848), 10.0), 7.0));
        }

        if (!(s.v[1263] != 0.0)) {
            s.store_mul_ad_lhs(936, A::offset(s.ad_value(848), 0.6), 852);
        }

        s.store_div_ad_lhs(835, A::add(s.ad_value(945), A::scale(A::sub(s.ad_value(897), s.ad_value(941)), p.p124)), 936);

        s.store_scale(835, 835, p.p31);

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.copy_ad(75, 835);

        s.store_mul_ad_lhs(888, A::mul(s.ad_value(893), s.ad_value(946)), 396);

        s.store_mul(889, 888, 887);

        s.store_div_ad_lhs(836, A::scale(s.ad_value(946), 2.0), 835);

        s.store_scale(838, 836, s.v[892]);

        s.v[1264] = if (s.v[105] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1264] != 0.0) {
            s.copy_ad(874, 106);
        }

        s.v[1265] = if (s.v[105] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1264] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_sub_from_scalar(843, 1.0, 106);
        }

        if ((!(s.v[1264] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_offset_ad(844, A::sub(s.ad_value(843), A::mul(s.ad_value(105), s.ad_value(875))), (-0.0001));
        }

        if ((!(s.v[1264] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_sqrt_ad(845, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), 0.0004)));
        }

        if ((!(s.v[1264] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_sub_ad(874, A::add(s.ad_value(106), s.ad_value(843)), A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));
        }

        if ((!(s.v[1264] != 0.0)) && (!(s.v[1265] != 0.0))) {
            s.store_offset_ad(844, A::add(s.ad_value(106), A::mul(s.ad_value(105), s.ad_value(875))), (-0.0001));
        }

        if ((!(s.v[1264] != 0.0)) && (!(s.v[1265] != 0.0))) {
            s.store_sqrt_ad(845, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(106), 0.0004)));
        }

        if ((!(s.v[1264] != 0.0)) && (!(s.v[1265] != 0.0))) {
            s.store_scaled_add(874, 844, 845, 0.5);
        }

        s.store_div(76, 860, 890);

        s.v[1266] = if ((s.v[887] == 0.0) && (s.v[874] == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1266] != 0.0) {
            s.store_div_from_scalar_ad(843, 1.0, A::add(A::mul(s.ad_value(860), s.ad_value(838)), s.ad_value(890)));
        }

        if (s.v[1266] != 0.0) {
            s.store_mul(846, 838, 890);
        }

        if (s.v[1266] != 0.0) {
            s.store_mul(837, 846, 843);
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_mul(852, 860, 889);
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_mul(850, 890, 852);
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_mul(849, 890, 889);
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_mul_ad(843, A::scale(s.ad_value(860), 2.0), A::add(A::offset(s.ad_value(852), (-1.0)), A::div_from_scalar(1.0, s.ad_value(874))));
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_add_ad(844, A::add(A::mul(s.ad_value(890), A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0))), A::mul(s.ad_value(860), s.ad_value(838))), A::scale(s.ad_value(850), 3.0));
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_mul_ad_rhs(845, 890, A::add(s.ad_value(838), A::scale(s.ad_value(849), 2.0)));
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_sqrt_ad(846, A::sub(A::square(s.ad_value(844)), A::mul(A::scale(s.ad_value(843), 2.0), s.ad_value(845))));
        }

        if (!(s.v[1266] != 0.0)) {
            s.store_div_ad_lhs(837, A::sub(s.ad_value(844), s.ad_value(846)), 843);
        }

        s.store_sub_ad_lhs(844, A::sub(s.ad_value(837), s.ad_value(822)), 180);

        s.store_sqrt_ad(845, A::add(A::square(s.ad_value(844)), A::mul(A::scale(s.ad_value(180), 4.0), s.ad_value(837))));

        s.store_sub_ad_rhs(876, 837, A::scale(A::add(s.ad_value(844), s.ad_value(845)), 0.5));

        s.v[1267] = if (s.v[876] > s.v[822]) { 1.0 } else { 0.0 };

        if (s.v[1267] != 0.0) {
            s.copy_ad(876, 822);
        }

        s.store_sub(878, 822, 876);

        s.copy_ad(77, 876);

        s.store_sub_from_scalar_ad(872, 1.0, A::div(A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(837)), s.ad_value(890)));

        s.store_mul(852, 889, 875);

        s.store_add_ad(843, A::add(s.ad_value(838), s.ad_value(837)), A::mul(A::scale(s.ad_value(852), 2.0), s.ad_value(872)));

        s.store_mul(852, 889, 860);

        s.store_add_ad_lhs(844, A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0)), 852);

        s.store_div(840, 843, 844);

        s.v[1268] = if ((s.v[191] > 0.0) && (s.v[878] > 1e-10)) { 1.0 } else { 0.0 };

        if (s.v[1268] != 0.0) {
            s.store_div_from_scalar_ad(843, 1.0, A::mul(A::mul(s.ad_value(191), s.ad_value(860)), s.ad_value(119)));
        }

        if (s.v[1268] != 0.0) {
            s.store_div(845, 875, 838);
        }

        if (s.v[1268] != 0.0) {
            s.store_scaled_add(844, 860, 845, s.v[892]);
        }

        if (s.v[1268] != 0.0) {
            s.store_mul(852, 843, 844);
        }

        if (s.v[1268] != 0.0) {
            s.store_mul(862, 852, 878);
        }

        if (!(s.v[1268] != 0.0)) {
            s.store_scalar(862, 2.688117142e43);
        }

        s.v[1269] = if (s.v[1142] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1269] != 0.0) {
            s.store_mul(851, 860, 837);
        }

        if (s.v[1269] != 0.0) {
            s.store_mul(843, 890, 851);
        }

        if (s.v[1269] != 0.0) {
            s.store_add(844, 890, 851);
        }

        if (s.v[1269] != 0.0) {
            s.copy_ad(845, 1142);
        }

        if (s.v[1269] != 0.0) {
            s.store_div_ad_lhs(863, A::sub(s.ad_value(890), A::div(s.ad_value(843), s.ad_value(844))), 845);
        }

        if (s.v[1269] != 0.0) {
            s.store_mul(850, 194, 841);
        }

        s.v[1270] = if (s.v[850] >= (-0.9)) { 1.0 } else { 0.0 };

        if ((s.v[1269] != 0.0) && (s.v[1270] != 0.0)) {
            s.store_div_from_scalar_ad(846, 1.0, A::offset(s.ad_value(850), 1.0));
        }

        if ((s.v[1269] != 0.0) && (s.v[1270] != 0.0)) {
            s.store_mul(863, 863, 846);
        }

        if ((s.v[1269] != 0.0) && (!(s.v[1270] != 0.0))) {
            s.store_div_from_scalar_ad(847, 1.0, A::offset(s.ad_value(850), 0.8));
        }

        if ((s.v[1269] != 0.0) && (!(s.v[1270] != 0.0))) {
            s.store_mul_ad_lhs(846, A::offset(A::scale(s.ad_value(850), 20.0), 17.0), 847);
        }

        if ((s.v[1269] != 0.0) && (!(s.v[1270] != 0.0))) {
            s.store_mul(863, 863, 846);
        }

        if (!(s.v[1269] != 0.0)) {
            s.store_scalar(863, 2.688117142e43);
        }

        s.store_mul(843, 387, 822);

        s.v[1271] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if (s.v[1271] != 0.0) {
            s.store_scalar(844, 2.688117142e43);
        }

        if (!(s.v[1271] != 0.0)) {
            s.store_exp(844, 843);
        }

        s.v[1272] = if (s.v[386] > 3.720075976e-44) { 1.0 } else { 0.0 };

        if (s.v[1272] != 0.0) {
            s.store_scalar(845, (1.0 + (p.p283 * s.v[892])));
        }

        if (s.v[1272] != 0.0) {
            s.store_div_ad_lhs(1093, A::offset(A::mul(s.ad_value(845), s.ad_value(844)), 1.0), 386);
        }

        if (s.v[1272] != 0.0) {
            s.store_mul(1093, 1093, 1092);
        }

        if (!(s.v[1272] != 0.0)) {
            s.store_scalar(1093, 2.688117142e43);
        }

        s.store_div(851, 195, 838);

        s.store_mul(852, 851, 875);

        s.v[1273] = if (s.v[852] > (-0.9)) { 1.0 } else { 0.0 };

        if (s.v[1273] != 0.0) {
            s.store_offset(843, 852, 1.0);
        }

        if (!(s.v[1273] != 0.0)) {
            s.store_div_from_scalar_ad(844, 1.0, A::offset(A::scale(s.ad_value(852), 20.0), 17.0));
        }

        if (!(s.v[1273] != 0.0)) {
            s.store_mul_ad_lhs(843, A::offset(s.ad_value(852), 0.8), 844);
        }

        s.store_add(871, 862, 863);

        s.store_div_ad_lhs(844, A::mul(s.ad_value(862), s.ad_value(863)), 871);

        s.store_add(871, 844, 1093);

        s.store_div_ad_lhs(845, A::mul(s.ad_value(844), s.ad_value(1093)), 871);

        s.store_add_ad_rhs(839, 840, A::mul(s.ad_value(843), s.ad_value(845)));

        s.store_scaled_mul(886, 396, 893, 1.0 / (s.v[892]));

        s.store_mul(880, 835, 886);

        s.store_sub_from_scalar_ad(843, 1.0, A::div(A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(876)), s.ad_value(890)));

        s.store_mul(882, 875, 843);

        s.store_div(852, 876, 838);

        s.store_offset(883, 852, 1.0);

        s.store_div_ad_lhs(881, A::mul(s.ad_value(880), s.ad_value(882)), 883);

        s.store_offset_ad(843, A::mul(s.ad_value(881), s.ad_value(887)), 1.0);

        s.store_div(852, 876, 843);

        s.store_mul(884, 881, 852);

        s.store_div(1085, 881, 843);

        s.store_div(852, 878, 839);

        s.store_offset(843, 852, 1.0);

        s.store_scaled_mul(885, 884, 843, 1.0 / (p.p23));

        s.store_scale(885, 885, p.p30);

        s.store_scaled_mul(78, 1085, 843, 1.0 / (p.p23));

        s.v[1274] = if (s.v[78] < 1e-9) { 1.0 } else { 0.0 };

        if (s.v[1274] != 0.0) {
            s.store_scalar(78, 1e-9);
        }

        s.store_scaled_mul(1086, 1085, 843, 1.0 / (p.p23));

        s.v[1275] = if (s.v[37] != 2.0) { 1.0 } else { 0.0 };

        s.v[1276] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1276] != 0.0)) {
            s.store_mul_ad_lhs(843, A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), 415);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1276] != 0.0))) {
            s.store_div_ad_lhs(843, A::scale(s.ad_value(415), p.p47), 416);
        }

        s.v[1277] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[1278] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(A::sub(A::neg(s.ad_value(822)), s.ad_value(1111)), s.ad_value(1153)), 843);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1278] != 0.0))) {
            s.store_div_ad_lhs(844, A::add(A::sub(A::sub(A::neg(s.ad_value(822)), s.ad_value(1111)), s.ad_value(1153)), s.ad_value(375)), 843);
        }

        s.v[1279] = if (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (s.v[1279] != 0.0)) {
            s.store_scalar(906, 0.0);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_scale_ad(844, A::add(s.ad_value(844), A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_div_ad_rhs(845, 1151, A::offset(s.ad_value(844), 0.001));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_mul_ad(906, A::mul(A::mul(s.ad_value(995), s.ad_value(1150)), s.ad_value(844)), A::exp(A::neg(s.ad_value(845))));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_square(847, 824);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_mul_ad_lhs(848, A::neg(s.ad_value(824)), 847);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_offset_ad(849, A::add(s.ad_value(1152), A::abs(s.ad_value(848))), 1e-9);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_offset_ad(850, A::scale(A::add(A::div(s.ad_value(848), s.ad_value(849)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(848), s.ad_value(849)), A::div(s.ad_value(848), s.ad_value(849))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1279] != 0.0))) {
            s.store_mul(906, 906, 850);
        }

        s.v[1280] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (s.v[1280] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(A::sub(s.ad_value(822), s.ad_value(825)), s.ad_value(1146)), 843);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1280] != 0.0))) {
            s.store_div_ad_lhs(844, A::add(A::sub(A::sub(s.ad_value(822), s.ad_value(825)), s.ad_value(1146)), s.ad_value(375)), 843);
        }

        s.v[1281] = if (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scalar(905, 0.0);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_scale_ad(844, A::add(s.ad_value(844), A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_div_ad_rhs(845, 1144, A::offset(s.ad_value(844), 0.001));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_mul_ad(905, A::mul(A::mul(s.ad_value(996), s.ad_value(1143)), s.ad_value(844)), A::exp(A::neg(s.ad_value(845))));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_square(847, 900);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_mul_ad_lhs(848, A::neg(s.ad_value(900)), 847);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_offset_ad(849, A::add(s.ad_value(1145), A::abs(s.ad_value(848))), 1e-9);
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_offset_ad(850, A::scale(A::add(A::div(s.ad_value(848), s.ad_value(849)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(848), s.ad_value(849)), A::div(s.ad_value(848), s.ad_value(849))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if (((s.v[1275] != 0.0) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_mul(905, 905, 850);
        }

        s.v[1282] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (s.v[1282] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(A::sub(A::neg(s.ad_value(822)), A::mul(s.ad_value(1154), s.ad_value(1111))), s.ad_value(1153)), 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1282] != 0.0))) {
            s.store_div_ad_lhs(844, A::add(A::sub(A::sub(A::neg(s.ad_value(822)), A::mul(s.ad_value(1154), s.ad_value(1111))), s.ad_value(1153)), s.ad_value(375)), 843);
        }

        s.v[1283] = if (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (s.v[1283] != 0.0)) {
            s.store_scalar(906, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_scale_ad(844, A::add(s.ad_value(844), A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_div_ad_rhs(845, 1151, A::offset(s.ad_value(844), 0.001));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_mul_ad(906, A::mul(A::mul(s.ad_value(995), s.ad_value(1150)), s.ad_value(844)), A::exp(A::neg(s.ad_value(845))));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_sub(847, 824, 1156);
        }

        s.v[1284] = if (s.v[847] >= ((-1.0) / 100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) && (s.v[1284] != 0.0)) {
            s.store_scale_ad(848, A::neg(s.ad_value(1155)), 100.0);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) && (!(s.v[1284] != 0.0))) {
            s.store_div(848, 1155, 847);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_mul(906, 906, 849);
        }

        s.v[1285] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (s.v[1285] != 0.0)) {
            s.store_div_ad_lhs(844, A::sub(A::sub(s.ad_value(822), A::mul(s.ad_value(1147), s.ad_value(825))), s.ad_value(1146)), 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1285] != 0.0))) {
            s.store_div_ad_lhs(844, A::add(A::sub(A::sub(s.ad_value(822), A::mul(s.ad_value(1147), s.ad_value(825))), s.ad_value(1146)), s.ad_value(375)), 843);
        }

        s.v[1286] = if (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (s.v[1286] != 0.0)) {
            s.store_scalar(905, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(844, A::add(s.ad_value(844), A::sqrt(A::offset(A::square(s.ad_value(844)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_div_ad_rhs(845, 1144, A::offset(s.ad_value(844), 0.001));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul_ad(905, A::mul(A::mul(s.ad_value(996), s.ad_value(1143)), s.ad_value(844)), A::exp(A::neg(s.ad_value(845))));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_sub(847, 900, 1149);
        }

        s.v[1287] = if (s.v[847] >= ((-1.0) / 100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1287] != 0.0)) {
            s.store_scale_ad(848, A::neg(s.ad_value(1148)), 100.0);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_div(848, 1148, 847);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1277] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul(905, 905, 849);
        }

        if (s.v[1275] != 0.0) {
            s.store_scalar(974, (s.v[347] * p.p155));
        }

    }

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1275] != 0.0) {
            s.store_scalar(975, (s.v[348] * p.p155));
        }

        if (s.v[1275] != 0.0) {
            s.store_mul(931, 832, 300);
        }

        if (s.v[1275] != 0.0) {
            s.store_div(843, 1087, 931);
        }

        s.v[1288] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1288] != 0.0)) {
            s.store_scale_ad(983, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1289] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1288] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_scalar(983, 3.720075976e-44);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) {
            s.store_exp(983, 843);
        }

        if (s.v[1275] != 0.0) {
            s.store_mul(931, 832, 301);
        }

        if (s.v[1275] != 0.0) {
            s.store_div(843, 1088, 931);
        }

        s.v[1290] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1290] != 0.0)) {
            s.store_scale_ad(984, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1291] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_scalar(984, 3.720075976e-44);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) {
            s.store_exp(984, 843);
        }

        s.v[1292] = if (s.v[947] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1292] != 0.0)) {
            s.store_scalar(926, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_mul(843, 974, 947);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_mul_ad_rhs(926, 843, A::offset(s.ad_value(983), (-1.0)));
        }

        s.v[1293] = if (s.v[948] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1293] != 0.0)) {
            s.store_scalar(922, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1293] != 0.0))) {
            s.store_mul(843, 975, 948);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1293] != 0.0))) {
            s.store_mul_ad_rhs(922, 843, A::offset(s.ad_value(984), (-1.0)));
        }

        s.v[1294] = if (s.v[951] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1294] != 0.0)) {
            s.store_scalar(927, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_mul_ad(970, A::scale(s.ad_value(302), p.p1043), A::offset(A::mul(s.ad_value(254), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_mul_ad(971, A::scale(s.ad_value(304), p.p1043), A::offset(A::mul(s.ad_value(255), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_div(843, 1087, 970);
        }

        s.v[1295] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_scale_ad(853, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1296] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_exp(853, 843);
        }

        s.v[1297] = if ((s.v[314] - s.v[1087]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(971)), s.ad_value(314)), 844);
        }

        s.v[1298] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) && (s.v[1298] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1299] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) && (!(s.v[1298] != 0.0))) && (s.v[1299] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_neg(854, 854);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(314), s.ad_value(1087)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(971)), s.ad_value(314)), 844);
        }

        s.v[1300] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) && (s.v[1300] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1301] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1301] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) && (!(s.v[1300] != 0.0))) && (!(s.v[1301] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) && (!(s.v[1297] != 0.0))) {
            s.store_neg(854, 854);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_mul(846, 974, 951);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1294] != 0.0))) {
            s.store_mul_ad_rhs(927, 846, A::add(s.ad_value(853), s.ad_value(854)));
        }

        s.v[1302] = if (s.v[952] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1302] != 0.0)) {
            s.store_scalar(923, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad(970, A::scale(s.ad_value(303), p.p1043), A::offset(A::mul(s.ad_value(254), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad(971, A::scale(s.ad_value(305), p.p1043), A::offset(A::mul(s.ad_value(255), s.ad_value(430)), 1.0));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_div(843, 1088, 970);
        }

        s.v[1303] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1303] != 0.0)) {
            s.store_scale_ad(853, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1304] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_exp(853, 843);
        }

        s.v[1305] = if ((s.v[315] - s.v[1088]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(971)), s.ad_value(315)), 844);
        }

        s.v[1306] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (s.v[1306] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1307] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (!(s.v[1306] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_neg(854, 854);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(315), s.ad_value(1088)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(971)), s.ad_value(315)), 844);
        }

        s.v[1308] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_scale_ad(854, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1309] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_exp(854, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_neg(854, 854);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul(846, 975, 952);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad_rhs(923, 846, A::add(s.ad_value(853), s.ad_value(854)));
        }

        if (s.v[1275] != 0.0) {
            s.store_scalar(930, ((s.v[328] / p.p23) * p.p155));
        }

        s.v[1310] = if ((s.v[949] <= 0.0) && (s.v[950] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(928, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(924, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(987, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(988, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1310] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_rhs(989, 972, A::offset(s.ad_value(983), (-1.0)));
        }

        s.v[1311] = if (s.v[989] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(989, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_scalar(991, 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1311] != 0.0))) {
            s.store_div_from_scalar_ad(991, 1.0, A::sqrt(A::offset(s.ad_value(989), 1.0)));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_rhs(990, 973, A::offset(s.ad_value(984), (-1.0)));
        }

        s.v[1312] = if (s.v[990] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(990, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(992, 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1312] != 0.0))) {
            s.store_div_from_scalar_ad(992, 1.0, A::sqrt(A::offset(s.ad_value(990), 1.0)));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_sub_from_scalar(843, 1.0, 351);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(985, A::mul(s.ad_value(930), s.ad_value(949)), 352);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul(844, 843, 985);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(928, A::mul(s.ad_value(844), A::offset(s.ad_value(983), (-1.0))), 991);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(985, A::mul(s.ad_value(930), s.ad_value(950)), 352);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul(844, 843, 985);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(924, A::mul(s.ad_value(844), A::offset(s.ad_value(984), (-1.0))), 992);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(986, A::mul(s.ad_value(930), s.ad_value(949)), 353);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(987, A::mul(s.ad_value(986), A::offset(s.ad_value(983), (-1.0))), 991);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(986, A::mul(s.ad_value(930), s.ad_value(950)), 353);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) {
            s.store_mul_ad_lhs(988, A::mul(s.ad_value(986), A::offset(s.ad_value(984), (-1.0))), 992);
        }

        s.v[1313] = if (p.p13 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_offset_ad(843, A::div(A::add(s.ad_value(1087), s.ad_value(1088)), s.ad_value(354)), 1.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_add(844, 989, 990);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(843)), A::scale(s.ad_value(844), 4.0)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_scaled_add(845, 843, 846, 0.5);
        }

        s.v[1314] = if (s.v[845] < 0.1) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_scalar(993, 10.0);
        }

        if ((((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) {
            s.store_div_from_scalar(993, 1.0, 845);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_mul(843, 351, 985);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1310] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_lhs(933, A::mul(s.ad_value(843), A::sub(s.ad_value(983), s.ad_value(984))), 993);
        }

        s.v[1315] = if ((s.v[953] <= 0.0) && (s.v[954] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1275] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_scalar(925, 0.0);
        }

        if ((s.v[1275] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_scalar(929, 0.0);
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) {
            s.store_scale(932, 298, p.p1043);
        }

        s.v[1316] = if ((s.v[316] - s.v[1087]) < 0.001) { 1.0 } else { 0.0 };

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(932)), s.ad_value(316)), 844);
        }

        s.v[1317] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (s.v[1317] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1318] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) && (!(s.v[1317] != 0.0))) && (!(s.v[1318] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul(846, 974, 953);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_mul_ad_rhs(929, 846, A::sub_from_scalar(1.0, s.ad_value(844)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(316), s.ad_value(1087)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1087)), s.ad_value(932)), s.ad_value(316)), 844);
        }

        s.v[1319] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1320] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul(846, 974, 953);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_mul_ad_rhs(929, 846, A::sub_from_scalar(1.0, s.ad_value(844)));
        }

        if ((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) {
            s.store_scale(932, 299, p.p1043);
        }

        s.v[1321] = if ((s.v[317] - s.v[1088]) < 0.001) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_scalar(844, 1000.0);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(932)), s.ad_value(317)), 844);
        }

        s.v[1322] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (s.v[1322] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1323] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (!(s.v[1322] != 0.0))) && (s.v[1323] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) && (!(s.v[1322] != 0.0))) && (!(s.v[1323] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_mul(846, 975, 954);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_mul_ad_rhs(925, 846, A::sub_from_scalar(1.0, s.ad_value(844)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::sub(s.ad_value(317), s.ad_value(1088)));
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_mul_ad_lhs(843, A::mul(A::div(A::neg(s.ad_value(1088)), s.ad_value(932)), s.ad_value(317)), 844);
        }

        s.v[1324] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (s.v[1324] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1325] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_mul(846, 975, 954);
        }

        if (((s.v[1275] != 0.0) && (!(s.v[1315] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_mul_ad_rhs(925, 846, A::sub_from_scalar(1.0, s.ad_value(844)));
        }

        if (s.v[1275] != 0.0) {
            s.store_add_ad_lhs(934, A::add(A::add(s.ad_value(926), s.ad_value(927)), s.ad_value(928)), 929);
        }

        if (s.v[1275] != 0.0) {
            s.store_add_ad_lhs(935, A::add(A::add(s.ad_value(922), s.ad_value(923)), s.ad_value(924)), 925);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(905, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(906, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(934, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(935, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(987, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(988, 0.0);
        }

        if (!(s.v[1275] != 0.0)) {
            s.store_scalar(933, 0.0);
        }

        s.store_exp_ad(1025, A::mul(s.ad_value(214), {
            if (s.v[411] > 1e-38) {
                A::ln(s.ad_value(411))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }));

        s.store_add_ad_rhs(203, 203, A::mul(s.ad_value(204), s.ad_value(430)));

        s.store_add_ad_rhs(207, 207, A::mul(s.ad_value(208), s.ad_value(430)));

        s.store_add_ad_rhs(243, 243, A::mul(s.ad_value(244), s.ad_value(430)));

        s.store_add_ad_rhs(246, 246, A::mul(s.ad_value(247), s.ad_value(430)));

        s.store_add_ad_rhs(250, 250, A::mul(s.ad_value(248), s.ad_value(430)));

        s.v[1326] = if ((p.p374 != 0.0) || (p.p375 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1326] != 0.0) {
            s.store_sub(1075, 825, 824);
        }

        if (s.v[1326] != 0.0) {
            s.store_sub_ad(826, A::sub(A::scale(s.ad_value(408), p.p37), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(943)));
        }

        if (s.v[1326] != 0.0) {
            s.store_offset_ad(846, A::add(A::sub(s.ad_value(826), s.ad_value(825)), s.ad_value(824)), (-0.02));
        }

        s.v[1327] = if (s.v[826] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1327] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(846)), A::scale(s.ad_value(826), (4.0 * 0.02))));
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1327] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(846)), A::scale(s.ad_value(826), (4.0 * 0.02))));
        }

        if (s.v[1326] != 0.0) {
            s.store_sub_ad_rhs(812, 826, A::scale(A::add(s.ad_value(846), s.ad_value(843)), 0.5));
        }

        if (s.v[1326] != 0.0) {
            s.store_sub(1081, 826, 812);
        }

        s.v[1328] = if (s.v[1081] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1328] != 0.0)) {
            s.store_scalar(1081, 0.0);
        }

        s.v[1329] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1326] != 0.0) && (s.v[1329] != 0.0)) {
            s.store_scalar(1082, 0.0);
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) {
            s.store_sub_ad_lhs(843, A::sub(A::sub(s.ad_value(825), s.ad_value(875)), s.ad_value(812)), 841);
        }

        s.v[1330] = if (s.v[843] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) && (s.v[1330] != 0.0)) {
            s.store_div(844, 843, 376);
        }

        if (((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) && (!(s.v[1330] != 0.0))) {
            s.store_mul_ad(844, A::scale(s.ad_value(376), 0.5), A::offset(A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(843), 4.0), s.ad_value(376)), s.ad_value(376)), 1.0)), (-1.0)));
        }

        if ((s.v[1326] != 0.0) && (!(s.v[1329] != 0.0))) {
            s.store_sub_ad_lhs(1082, A::sub(s.ad_value(825), A::add(A::square(s.ad_value(844)), s.ad_value(824))), 826);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(826, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1075, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1081, 0.0);
        }

        if (!(s.v[1326] != 0.0)) {
            s.store_scalar(1082, 0.0);
        }

        if (p.p375 != 0.0) {
            s.store_mul(843, 832, 211);
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(1028, A::sub(s.ad_value(825), A::scale(s.ad_value(408), p.p37)), 843);
        }

        s.v[1331] = if (s.v[1028] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1331] != 0.0)) {
            s.store_sub_ad_rhs(1078, 825, A::scale(s.ad_value(408), p.p37));
        }

        s.v[1332] = if (s.v[1028] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (s.v[1332] != 0.0)) {
            s.store_scale(1078, 843, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (!(s.v[1332] != 0.0))) {
            s.store_exp(1029, 1028);
        }

        if (((p.p375 != 0.0) && (!(s.v[1331] != 0.0))) && (!(s.v[1332] != 0.0))) {
            s.store_mul_ad_rhs(1078, 843, A::ln(A::offset(s.ad_value(1029), 1.0)));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 825, 1078);
        }

        if (p.p375 != 0.0) {
            s.store_scalar(854, s.v[369]);
        }

        if (p.p375 != 0.0) {
            s.store_scalar(855, s.v[370]);
        }

        if (p.p375 != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(203), s.ad_value(206)), 205);
        }

        if (p.p375 != 0.0) {
            s.store_mul(847, 205, 206);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(203), A::mul(s.ad_value(846), s.ad_value(1082))), A::mul(A::mul(s.ad_value(847), s.ad_value(1082)), s.ad_value(1082))));
        }

        s.v[1333] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1333] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1334] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1333] != 0.0))) && (s.v[1334] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1333] != 0.0))) && (!(s.v[1334] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1020, A::mul(A::mul(s.ad_value(854), s.ad_value(845)), s.ad_value(849)), 1025);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(850, A::neg(s.ad_value(212)), 822);
        }

        if (p.p375 != 0.0) {
            s.store_offset_ad(851, A::square(s.ad_value(850)), 0.0002);
        }

        s.v[1335] = if (s.v[850] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1335] != 0.0)) {
            s.store_scalar(852, 2.688117142e43);
        }

        s.v[1336] = if (s.v[850] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1335] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_scalar(852, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1335] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_exp(852, 850);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + (0.0001)));
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(853, A::sub(s.ad_value(844), s.ad_value(850)), 851);
        }

        if (p.p375 != 0.0) {
            s.store_mul(1023, 1020, 853);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + ((-0.0001))));
        }

        if (p.p375 != 0.0) {
            s.store_div_ad_lhs(853, A::sub(A::mul(s.ad_value(850), s.ad_value(852)), s.ad_value(844)), 851);
        }

        if (p.p375 != 0.0) {
            s.store_mul(1024, 1020, 853);
        }

        if (p.p375 != 0.0) {
            s.store_sub(843, 821, 375);
        }

        if (p.p375 != 0.0) {
            s.store_sqrt_ad(1026, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 821, 1026);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(964, 372);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(965, 373);
        }

        if (p.p375 != 0.0) {
            s.copy_ad(855, 374);
        }

        if (p.p375 != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(207), s.ad_value(210)), 209);
        }

        if (p.p375 != 0.0) {
            s.store_mul(847, 209, 210);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(207), A::mul(s.ad_value(846), s.ad_value(1026))), A::mul(A::mul(s.ad_value(847), s.ad_value(1026)), s.ad_value(1026))));
        }

        s.v[1337] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1337] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1338] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1337] != 0.0))) && (s.v[1338] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1337] != 0.0))) && (!(s.v[1338] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1021, A::mul(A::mul(s.ad_value(964), s.ad_value(845)), s.ad_value(849)), 1025);
        }

        if (p.p375 != 0.0) {
            s.store_sub(843, 820, 375);
        }

        if (p.p375 != 0.0) {
            s.store_sqrt_ad(1027, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 820, 1027);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_rhs(848, 855, A::sub(A::add(s.ad_value(207), A::mul(s.ad_value(846), s.ad_value(1027))), A::mul(A::mul(s.ad_value(847), s.ad_value(1027)), s.ad_value(1027))));
        }

        s.v[1339] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && (s.v[1339] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1340] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!(s.v[1339] != 0.0))) && (s.v[1340] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!(s.v[1339] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_ad_lhs(1022, A::mul(A::mul(s.ad_value(965), s.ad_value(845)), s.ad_value(849)), 1025);
        }

        if (!(p.p375 != 0.0)) {
            s.store_scalar(1022, 0.0);
        }

        if (!(p.p375 != 0.0)) {
            s.store_scalar(1021, 0.0);
        }

        if (!(p.p375 != 0.0)) {
            s.store_scalar(1024, 0.0);
        }

        if (!(p.p375 != 0.0)) {
            s.store_scalar(1023, 0.0);
        }

        s.v[1341] = if ((p.p374 != 0.0) && (s.v[37] != 2.0)) { 1.0 } else { 0.0 };

        if (s.v[1341] != 0.0) {
            s.store_scalar(1077, s.v[345]);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1082);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(843, p.p396);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(1076)), (-p.p397));
        }

        if (s.v[1341] != 0.0) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), (4.0 * p.p397))));
        }

        if (s.v[1341] != 0.0) {
            s.store_sub_ad_rhs(1080, 843, A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1080);
        }

        if (s.v[1341] != 0.0) {
            s.store_scaled_offset(843, 1076, (-p.p381), 1.0 / (p.p382));
        }

        s.v[1342] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1342] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1343] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (s.v[1341] != 0.0) {
            s.store_scale_ad(1078, A::ln(A::offset(s.ad_value(844), 1.0)), p.p382);
        }

        s.v[1344] = if (p.p386 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1344] != 0.0)) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p386)));
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1344] != 0.0))) {
            s.store_scalar(843, 1.0);
        }

        s.v[1345] = if (s.v[843] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1345] != 0.0)) {
            s.store_scalar(843, 0.01);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(844, A::scale(A::offset(A::scale(s.ad_value(893), (s.v[892] * 1.0 / (p.p23))), (p.p28 / p.p3)), p.p1035), 1077);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(845, (p.p1036 * p.p376));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(846, 243);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(847, 245);
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1341] != 0.0) {
            s.store_div_ad_lhs(849, A::mul(s.ad_value(845), A::sub(s.ad_value(846), A::mul(s.ad_value(847), s.ad_value(1076)))), 843);
        }

        s.v[1346] = if (s.v[849] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1346] != 0.0)) {
            s.store_scale_ad(848, A::offset(A::offset(s.ad_value(849), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1347] = if (s.v[849] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_exp(848, 849);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(1083, A::mul(A::mul(A::mul(s.ad_value(844), s.ad_value(1075)), s.ad_value(1078)), s.ad_value(848)), 1025);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1081);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(843, p.p396);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset_ad(844, A::sub(s.ad_value(843), s.ad_value(1076)), (-p.p397));
        }

        if (s.v[1341] != 0.0) {
            s.store_sqrt_ad(846, A::add(A::square(s.ad_value(844)), A::scale(s.ad_value(843), (4.0 * p.p397))));
        }

        if (s.v[1341] != 0.0) {
            s.store_sub_ad_rhs(1080, 843, A::scale(A::add(s.ad_value(844), s.ad_value(846)), 0.5));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(1076, 1080);
        }

        if (s.v[1341] != 0.0) {
            s.store_scaled_sub(843, 826, 1075, 1.0 / (p.p387));
        }

        s.v[1348] = if (s.v[843] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1348] != 0.0)) {
            s.store_scale_ad(844, A::offset(A::offset(s.ad_value(843), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1349] = if (s.v[843] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1348] != 0.0))) && (s.v[1349] != 0.0)) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1348] != 0.0))) && (!(s.v[1349] != 0.0))) {
            s.store_exp(844, 843);
        }

        if (s.v[1341] != 0.0) {
            s.store_scale_ad(1078, A::ln(A::offset(s.ad_value(844), 1.0)), p.p387);
        }

        s.v[1350] = if (p.p391 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1350] != 0.0)) {
            s.store_sub_from_scalar_ad(843, 1.0, A::scale(s.ad_value(1076), 1.0 / (p.p391)));
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1350] != 0.0))) {
            s.store_scalar(843, 1.0);
        }

        s.v[1351] = if (s.v[843] < 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1351] != 0.0)) {
            s.store_scalar(843, 0.01);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(844, A::scale(A::offset(A::scale(s.ad_value(893), (s.v[892] * 1.0 / (p.p23))), (p.p28 / p.p3)), p.p1037), 1077);
        }

        if (s.v[1341] != 0.0) {
            s.store_scalar(845, (p.p1038 * p.p376));
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(846, 246);
        }

        if (s.v[1341] != 0.0) {
            s.copy_ad(847, 249);
        }

        if (s.v[1341] != 0.0) {
            s.store_div_ad_lhs(849, A::mul(s.ad_value(845), A::sub(s.ad_value(846), A::mul(s.ad_value(847), s.ad_value(1076)))), 843);
        }

        s.v[1352] = if (s.v[849] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1352] != 0.0)) {
            s.store_scale_ad(848, A::offset(A::offset(s.ad_value(849), 1.0), (-100.0)), 2.688117142e43);
        }

        s.v[1353] = if (s.v[849] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1341] != 0.0) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if (((s.v[1341] != 0.0) && (!(s.v[1352] != 0.0))) && (!(s.v[1353] != 0.0))) {
            s.store_exp(848, 849);
        }

        if (s.v[1341] != 0.0) {
            s.store_mul_ad_lhs(1084, A::mul(A::mul(A::mul(s.ad_value(844), s.ad_value(1075)), s.ad_value(1078)), s.ad_value(848)), 1025);
        }

        s.v[1354] = if (s.v[1075] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1354] != 0.0)) {
            s.copy_ad(1079, 1083);
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1354] != 0.0))) {
            s.copy_ad(1079, 1084);
        }

        if (s.v[1341] != 0.0) {
            s.store_offset(1127, 826, p.p1033);
        }

        if (!(s.v[1341] != 0.0)) {
            s.store_scalar(1079, 0.0);
        }

        s.store_scale(79, 1079, p.p37);

        s.v[1355] = if (((((p.p374 != 0.0) && (s.v[37] != 2.0)) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) && (s.v[1114] < s.v[1127])) { 1.0 } else { 0.0 };

        if (s.v[1355] != 0.0) {
            s.store_sub(843, 1114, 1127);
        }

        if (s.v[1355] != 0.0) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if (s.v[1355] != 0.0) {
            s.store_scale_ad(1113, A::offset(A::sub(s.ad_value(844), s.ad_value(843)), (-0.01)), 0.5);
        }

        if (s.v[1355] != 0.0) {
            s.store_scalar(854, (if (p.p37 == 1.0) { p.p1039 } else { p.p1040 }));
        }

        if (s.v[1355] != 0.0) {
            s.store_scalar(855, (if (p.p37 == 1.0) { p.p1041 } else { p.p1042 }));
        }

        if (s.v[1355] != 0.0) {
            s.store_mul(845, 1114, 1113);
        }

        if (s.v[1355] != 0.0) {
            s.store_sub_ad_lhs(846, A::mul(s.ad_value(250), s.ad_value(252)), 251);
        }

        if (s.v[1355] != 0.0) {
            s.store_mul(847, 251, 252);
        }

        if (s.v[1355] != 0.0) {
            s.store_mul_ad(848, A::scale(A::neg(s.ad_value(855)), p.p376), A::sub(A::add(s.ad_value(250), A::mul(s.ad_value(846), s.ad_value(1113))), A::mul(A::mul(s.ad_value(847), s.ad_value(1113)), s.ad_value(1113))));
        }

        s.v[1356] = if (s.v[848] > 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1355] != 0.0) && (s.v[1356] != 0.0)) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.v[1357] = if (s.v[848] < (-100.0)) { 1.0 } else { 0.0 };

        if (((s.v[1355] != 0.0) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((s.v[1355] != 0.0) && (!(s.v[1356] != 0.0))) && (!(s.v[1357] != 0.0))) {
            s.store_exp(849, 848);
        }

        if (s.v[1355] != 0.0) {
            s.store_scale(854, 854, (p.p27 * s.v[345]));
        }

        if (s.v[1355] != 0.0) {
            s.store_mul_ad_lhs(1112, A::mul(A::mul(s.ad_value(854), s.ad_value(845)), s.ad_value(849)), 1025);
        }

        if (!(s.v[1355] != 0.0)) {
            s.store_scalar(1112, 0.0);
        }

        s.store_scale(80, 1112, p.p37);

        s.v[1358] = if (s.v[37] != 2.0) { 1.0 } else { 0.0 };

        s.v[1359] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        s.v[1360] = if (s.v[201] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (s.v[1360] != 0.0)) {
            s.store_scalar(908, 0.0);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_sub_ad(966, A::mul(s.ad_value(275), A::offset(A::scale(s.ad_value(430), p.p308), 1.0)), A::scale(s.ad_value(276), 1.0 / (s.v[892])));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_scale(843, 277, s.v[892]);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_ad(844, A::mul(s.ad_value(278), s.ad_value(843)), A::offset(s.ad_value(843), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::mul(s.ad_value(279), s.ad_value(875)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add(846, 843, 280);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_mul(845, 830, 846);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_div_from_scalar_ad(846, 1.0, A::offset(A::mul(s.ad_value(281), s.ad_value(822)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_mul_ad_lhs(967, A::mul(s.ad_value(844), s.ad_value(845)), 846);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add(921, 966, 967);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_sub(969, 822, 921);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add_ad(843, A::add(s.ad_value(274), A::mul(s.ad_value(273), s.ad_value(969))), A::mul(A::mul(s.ad_value(202), s.ad_value(969)), s.ad_value(969)));
        }

        s.v[1361] = if (s.v[843] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_scalar(843, 1e-5);
        }

        s.v[1362] = if ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (s.v[1362] != 0.0)) {
            s.store_scale(968, 201, 2.688117142e43);
        }

        s.v[1363] = if ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (!(s.v[1362] != 0.0))) && (s.v[1363] != 0.0)) {
            s.store_scale(968, 201, 3.720075976e-44);
        }

        if (((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (!(s.v[1362] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_mul_ad_rhs(968, 201, A::exp(A::div(s.ad_value(969), s.ad_value(843))));
        }

        s.v[1364] = if (s.v[968] > 10.0) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) && (s.v[1364] != 0.0)) {
            s.store_scalar(968, 10.0);
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_add_ad_rhs(843, 885, A::mul(A::mul(s.ad_value(267), s.ad_value(398)), s.ad_value(933)));
        }

        if (((s.v[1358] != 0.0) && (s.v[1359] != 0.0)) && (!(s.v[1360] != 0.0))) {
            s.store_mul(908, 968, 843);
        }

        s.v[1365] = if (s.v[201] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1365] != 0.0)) {
            s.store_scalar(1106, 0.0);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_sub_ad(966, A::mul(s.ad_value(275), A::offset(A::scale(s.ad_value(430), p.p308), 1.0)), A::scale(s.ad_value(276), 1.0 / (s.v[892])));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_scale(843, 277, s.v[892]);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_ad(844, A::mul(s.ad_value(278), s.ad_value(843)), A::offset(s.ad_value(843), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_from_scalar_ad(843, 1.0, A::offset(A::mul(s.ad_value(279), s.ad_value(875)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add(846, 843, 280);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_mul(845, 830, 846);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_from_scalar_ad(846, 1.0, A::offset(A::mul(s.ad_value(281), s.ad_value(822)), 1.0));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_mul_ad_lhs(967, A::mul(s.ad_value(844), s.ad_value(845)), 846);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add(921, 966, 967);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_sub(969, 822, 921);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_add_ad(843, A::add(s.ad_value(274), A::mul(s.ad_value(273), s.ad_value(969))), A::mul(A::mul(s.ad_value(202), s.ad_value(969)), s.ad_value(969)));
        }

        s.v[1366] = if (s.v[843] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1366] != 0.0)) {
            s.store_scalar(843, 1e-5);
        }

        s.v[1367] = if ((s.v[843] < (s.v[969] / 100.0)) && (s.v[969] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1367] != 0.0)) {
            s.store_scale(968, 201, 2.688117142e43);
        }

        s.v[1368] = if ((s.v[843] < ((-s.v[969]) / 100.0)) && (s.v[969] < 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (!(s.v[1367] != 0.0))) && (s.v[1368] != 0.0)) {
            s.store_scale(968, 201, 3.720075976e-44);
        }

        if (((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (!(s.v[1367] != 0.0))) && (!(s.v[1368] != 0.0))) {
            s.store_mul_ad_rhs(968, 201, A::exp(A::div(s.ad_value(969), s.ad_value(843))));
        }

        s.v[1369] = if (s.v[968] > 10.0) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1369] != 0.0)) {
            s.store_scalar(968, 10.0);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.copy_ad(843, 885);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_mul(1106, 968, 843);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(843, A::add(s.ad_value(269), A::scale(s.ad_value(268), s.v[892])), 1.0 / (s.v[892]));
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_rhs(1105, 270, A::offset(A::scale(s.ad_value(430), p.p320), 1.0));
        }

        s.v[1370] = if (s.v[398] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1370] != 0.0)) {
            s.store_sub(844, 1105, 1088);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1370] != 0.0))) {
            s.store_sub(844, 1105, 1087);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_offset(845, 272, (-1.0));
        }

        s.v[1371] = if (s.v[844] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1371] != 0.0)) {
            s.store_scalar(846, 0.0);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1371] != 0.0))) {
            s.store_mul_ad(846, A::neg(s.ad_value(271)), A::pow(s.ad_value(844), s.ad_value(845)));
        }

        s.v[1372] = if (s.v[846] > 100.0) { 1.0 } else { 0.0 };

        if (((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (s.v[1372] != 0.0)) {
            s.store_scalar(847, 2.688117142e43);
        }

        s.v[1373] = if (s.v[846] < (-100.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1372] != 0.0))) && (s.v[1373] != 0.0)) {
            s.store_scalar(847, 3.720075976e-44);
        }

        if ((((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_exp(847, 846);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_lhs(1107, A::mul(A::mul(A::mul(s.ad_value(843), s.ad_value(398)), s.ad_value(933)), s.ad_value(844)), 847);
        }

        if ((s.v[1358] != 0.0) && (!(s.v[1359] != 0.0))) {
            s.store_add(908, 1106, 1107);
        }

        s.v[1374] = if ((s.v[399] == 0.0) || (s.v[399] == 2.0)) { 1.0 } else { 0.0 };

        if ((s.v[1358] != 0.0) && (s.v[1374] != 0.0)) {
            s.store_scalar(907, 0.0);
        }

        s.v[1375] = if (s.v[156] < 0.001) { 1.0 } else { 0.0 };

        s.v[1376] = if (s.v[50] <= 0.001) { 1.0 } else { 0.0 };

        if ((((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (s.v[1375] != 0.0)) && (s.v[1376] != 0.0)) {
            s.store_scalar(843, (1.0 / 0.001));
        }

        if ((((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (s.v[1375] != 0.0)) && (!(s.v[1376] != 0.0))) {
            s.store_scalar(843, (1.0 / s.v[50]));
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (s.v[1375] != 0.0)) {
            s.store_mul(907, 899, 843);
        }

        if (((s.v[1358] != 0.0) && (!(s.v[1374] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_div_ad_rhs(907, 899, A::offset(s.ad_value(156), s.v[50]));
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_scalar(908, 0.0);
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_scalar(907, 0.0);
        }

        s.v[1377] = if (p.p39 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[1377] != 0.0) {
            s.store_mul(852, 230, 49);
        }

        if (s.v[1377] != 0.0) {
            s.store_mul(843, 852, 880);
        }

        if (s.v[1377] != 0.0) {
            s.store_mul_ad_rhs(81, 229, A::add(s.ad_value(843), s.ad_value(1086)));
        }

        s.v[1378] = if (p.p3 != 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1377] != 0.0) && (s.v[1378] != 0.0)) {
            s.store_scale(81, 81, p.p3);
        }

        s.v[1379] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1377] != 0.0) && (s.v[1379] != 0.0)) {
            s.store_add(854, 64, 81);
        }

        if ((s.v[1377] != 0.0) && (s.v[1379] != 0.0)) {
            s.store_div_ad_lhs(81, A::mul(s.ad_value(64), s.ad_value(81)), 854);
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1377] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.v[1380] = if (p.p429 == 0.0) { 1.0 } else { 0.0 };

        s.v[1381] = if ((s.v[60] + p.p135) > p.p431) { 1.0 } else { 0.0 };

        if ((s.v[1380] != 0.0) && (s.v[1381] != 0.0)) {
            s.store_add(1100, 60, 1101);
        }

        s.v[1382] = if (s.v[1100] < p.p431) { 1.0 } else { 0.0 };

        if (((s.v[1380] != 0.0) && (s.v[1381] != 0.0)) && (s.v[1382] != 0.0)) {
            s.store_scalar(1100, p.p431);
        }

        if ((s.v[1380] != 0.0) && (!(s.v[1381] != 0.0))) {
            s.store_scalar(1100, 0.0);
        }

        s.v[1383] = if ((s.v[61] + p.p136) > p.p431) { 1.0 } else { 0.0 };

        if ((s.v[1380] != 0.0) && (s.v[1383] != 0.0)) {
            s.store_add(1099, 61, 1102);
        }

        s.v[1384] = if (s.v[1099] < p.p431) { 1.0 } else { 0.0 };

        if (((s.v[1380] != 0.0) && (s.v[1383] != 0.0)) && (s.v[1384] != 0.0)) {
            s.store_scalar(1099, p.p431);
        }

        if ((s.v[1380] != 0.0) && (!(s.v[1383] != 0.0))) {
            s.store_scalar(1099, 0.0);
        }

        s.v[1385] = if (p.p429 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scalar(887, 0.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sub(843, 821, 375);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scaled_add(1026, 843, 844, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_offset_ad(843, A::mul(s.ad_value(183), s.ad_value(1026)), 1.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul_ad_lhs(844, A::neg(s.ad_value(184)), 818);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad(845, A::add(A::div_from_scalar(1.0, s.ad_value(843)), s.ad_value(844)), A::mul(s.ad_value(185), A::sub(s.ad_value(897), s.ad_value(941))));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scale(847, 1096, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_lhs(1100, A::add(A::add(s.ad_value(1098), A::mul(s.ad_value(846), s.ad_value(847))), s.ad_value(60)), 1101);
        }

        s.v[1386] = if (s.v[1100] < p.p431) { 1.0 } else { 0.0 };

        if (((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) && (s.v[1386] != 0.0)) {
            s.store_scalar(1100, p.p431);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sub(843, 820, 375);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_sqrt_ad(844, A::offset(A::square(s.ad_value(843)), 0.0001));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scaled_add(1027, 843, 844, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_offset_ad(843, A::mul(s.ad_value(183), s.ad_value(1027)), 1.0);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_mul_ad_lhs(844, A::neg(s.ad_value(184)), 817);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad(845, A::add(A::div_from_scalar(1.0, s.ad_value(843)), s.ad_value(844)), A::mul(s.ad_value(185), A::sub(s.ad_value(897), s.ad_value(941))));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_scale(847, 1095, 0.5);
        }

        if ((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) {
            s.store_add_ad_lhs(1099, A::add(A::add(s.ad_value(1097), A::mul(s.ad_value(846), s.ad_value(847))), s.ad_value(61)), 1102);
        }

        s.v[1387] = if (s.v[1099] < p.p431) { 1.0 } else { 0.0 };

        if (((!(s.v[1380] != 0.0)) && (s.v[1385] != 0.0)) && (s.v[1387] != 0.0)) {
            s.store_scalar(1099, p.p431);
        }

        if ((!(s.v[1380] != 0.0)) && (!(s.v[1385] != 0.0))) {
            s.store_scalar(1100, 0.0);
        }

        if ((!(s.v[1380] != 0.0)) && (!(s.v[1385] != 0.0))) {
            s.store_scalar(1099, 0.0);
        }

        s.v[1388] = if (p.p430 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1388] != 0.0) {
            s.store_scale(1100, 1100, 1.0 / (p.p30));
        }

        if (s.v[1388] != 0.0) {
            s.store_scale(1099, 1099, 1.0 / (p.p30));
        }

        s.store_mul_ad_rhs(844, 875, A::sub_from_scalar(1.0, A::div(A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(876)), s.ad_value(890))));

        s.store_mul_ad_lhs(82, A::scale(A::neg(s.ad_value(396)), (s.v[328] * (p.p3 * s.v[892]))), 844);

        s.v[1389] = if (p.p3 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1389] != 0.0) {
            s.store_scale(885, 885, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(933, 933, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(78, 78, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(934, 934, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(935, 935, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(1023, 1023, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(1024, 1024, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(1021, 1021, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(1022, 1022, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(908, 908, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(79, 79, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(905, 905, p.p3);
        }

        if (s.v[1389] != 0.0) {
            s.store_scale(906, 906, p.p3);
        }

        s.store_ad(83, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(9), None)), p.p37));

        s.v[1390] = if (s.v[398] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1390] != 0.0) {
            s.store_ad(84, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(7), None)), p.p37));
        }

        if (!(s.v[1390] != 0.0)) {
            s.store_ad(84, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(8), None)), p.p37));
        }

        s.store_ad(85, &A::scale(A::constant(A::ddx_projection(&s.ad_value(885), Some(5), None)), p.p37));

        s.store_scale(842, 396, ((((s.v[332] / p.p23) * p.p3) * s.v[331]) + p.p26));

        s.store_scale(981, 396, (p.p361 * ((((s.v[332] / p.p23) * p.p3) * s.v[365]) + p.p26)));

        s.store_scale(1115, 396, p.p27);

        s.store_scale(1116, 396, (p.p361 * p.p27));

        s.store_sub(830, 825, 1073);

        s.store_mul(853, 1059, 832);

        s.store_div_ad_lhs(809, A::mul(s.ad_value(384), s.ad_value(830)), 853);

        s.store_mul_ad_lhs(1016, A::mul(s.ad_value(1059), s.ad_value(363)), 832);

        s.store_mul_ad_lhs(1017, A::mul(s.ad_value(1059), s.ad_value(364)), 832);

        s.v[1391] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        s.v[1392] = if ((s.v[809] > (-100.0)) && (s.v[809] < 100.0)) { 1.0 } else { 0.0 };

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad(810, A::exp(s.ad_value(809)), A::exp(s.ad_value(809)));
        }

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad_rhs(810, 810, A::exp(A::neg(A::div(s.ad_value(324), s.ad_value(1016)))));
        }

        if ((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1393] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_mul_ad_rhs(1117, 810, A::exp(A::div(A::div_from_scalar((-p.p1033), s.ad_value(1017)), A::square(s.ad_value(832)))));
        }

        if (((s.v[1391] != 0.0) && (s.v[1392] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1394] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        s.v[1395] = if ((s.v[809] > (-100.0)) && (s.v[809] < 100.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_exp_ad(810, A::div(s.ad_value(809), A::mul(s.ad_value(384), s.ad_value(363))));
        }

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_mul_ad_rhs(810, 810, A::exp(A::neg(A::div(s.ad_value(324), s.ad_value(1016)))));
        }

        if (((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.v[1396] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul_ad_rhs(1117, 810, A::exp(A::div(A::div_from_scalar((-p.p1033), s.ad_value(1017)), A::square(s.ad_value(832)))));
        }

        if ((((!(s.v[1391] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) && (s.v[1396] != 0.0)) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_div_ad_lhs(809, A::mul(s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324))), 1016);
        }

        if ((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_div_ad_lhs(833, A::sub(s.ad_value(390), A::mul(A::sub_from_scalar(1.0, s.ad_value(388)), A::sub(s.ad_value(830), s.ad_value(324)))), 1016);
        }

        s.v[1397] = if (s.v[809] > 100.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1397] != 0.0)) {
            s.store_sub(875, 830, 324);
        }

        s.v[1398] = if (s.v[833] > 100.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_div_ad_lhs(843, A::sub(A::sub(s.ad_value(830), s.ad_value(324)), s.ad_value(390)), 1016);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_exp(810, 843);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_mul_ad_lhs(875, A::div(A::mul(s.ad_value(832), s.ad_value(1140)), s.ad_value(396)), 810);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_exp(810, 809);
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad_rhs(844, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad(857, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(832), s.ad_value(1140))), A::exp(s.ad_value(833))), A::sub_from_scalar(1.0, s.ad_value(388)));
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sub_ad_rhs(845, 388, A::div(A::mul(s.ad_value(1016), s.ad_value(857)), A::sub_from_scalar(1.0, s.ad_value(388))));
        }

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1397] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_div(875, 844, 845);
        }

        s.v[1399] = if (p.p27 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_div_ad_lhs(1119, A::mul(s.ad_value(388), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033))), 1017);
        }

        if (((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_div_ad_lhs(1120, A::sub(s.ad_value(390), A::mul(A::sub_from_scalar(1.0, s.ad_value(388)), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033)))), 1017);
        }

        s.v[1400] = if (s.v[1119] > 100.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (s.v[1400] != 0.0)) {
            s.store_offset_ad(1118, A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033));
        }

        s.v[1401] = if (s.v[1120] > 100.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_div_ad_lhs(843, A::offset(A::sub(A::sub(s.ad_value(830), s.ad_value(324)), s.ad_value(390)), (-p.p1033)), 1017);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_exp(1117, 843);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_mul_ad_lhs(1118, A::div(A::mul(s.ad_value(832), s.ad_value(1140)), s.ad_value(396)), 1117);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_exp(1117, 1119);
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_mul_ad_rhs(844, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_mul_ad(857, A::mul(A::div(A::neg(s.ad_value(396)), A::mul(s.ad_value(832), s.ad_value(1140))), A::exp(s.ad_value(1120))), A::sub_from_scalar(1.0, s.ad_value(388)));
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_sub_ad_rhs(845, 388, A::div(A::mul(s.ad_value(1017), s.ad_value(857)), A::sub_from_scalar(1.0, s.ad_value(388))));
        }

        if (((((!(s.v[1391] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1399] != 0.0)) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_div(1118, 844, 845);
        }

        s.copy_ad(829, 1073);

        s.copy_ad(828, 1054);

        s.copy_ad(841, 1044);

        s.v[1402] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        s.v[1403] = if (s.v[37] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1402] != 0.0) && (s.v[1403] != 0.0)) {
            s.store_scalar(938, 0.0);
        }

        if ((s.v[1402] != 0.0) && (s.v[1403] != 0.0)) {
            s.store_scalar(937, 0.0);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_add_ad_lhs(826, A::sub(A::sub(s.ad_value(829), s.ad_value(942)), A::mul(s.ad_value(405), s.ad_value(828))), 324);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_offset_ad(813, A::add(A::sub(s.ad_value(826), s.ad_value(825)), s.ad_value(841)), (-0.08));
        }

        s.v[1404] = if (s.v[826] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1404] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::scale(s.ad_value(826), (4.0 * 0.08))));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (!(s.v[1404] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::scale(s.ad_value(826), (4.0 * 0.08))));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_sub_ad_rhs(812, 826, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_mul_ad_rhs(938, 981, A::sub(s.ad_value(812), s.ad_value(826)));
        }

        s.v[1405] = if (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_offset(1127, 826, p.p1033);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_scalar(1139, 0.08);
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_sub_ad_lhs(813, A::add(A::sub(s.ad_value(1127), s.ad_value(1125)), s.ad_value(841)), 1139);
        }

        s.v[1406] = if (s.v[1127] <= 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_sqrt_ad(843, A::sub(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1139), 100.0), s.ad_value(1127))));
        }

        if ((((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) && (!(s.v[1406] != 0.0))) {
            s.store_sqrt_ad(843, A::add(A::square(s.ad_value(813)), A::mul(A::scale(s.ad_value(1139), 100.0), s.ad_value(1127))));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_sub_ad_rhs(1128, 1127, A::scale(A::add(s.ad_value(813), s.ad_value(843)), 0.5));
        }

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_add_ad_rhs(938, 938, A::mul(s.ad_value(1116), A::sub(s.ad_value(1128), s.ad_value(1127))));
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_scale(843, 376, 0.5);
        }

        if ((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(825), s.ad_value(812)), s.ad_value(841)), 875);
        }

        s.v[1407] = if (s.v[376] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1402] != 0.0) && (!(s.v[1403] != 0.0))) && (s.v[1407] != 0.0)) {
            s.store_scalar(844, 0.0);
        }

    }
}
