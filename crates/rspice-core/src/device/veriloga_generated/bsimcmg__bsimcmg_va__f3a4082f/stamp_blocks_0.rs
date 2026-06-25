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
        s.v[188] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[197] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[268] = 0.0;

        s.v[269] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[280] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[288] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[292] = 0.0;

        s.v[300] = 0.0;

        s.v[302] = 0.0;

        s.v[305] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[316] = 0.0;

        s.v[320] = 0.0;

        s.v[333] = 0.0;

        s.v[335] = 0.0;

        s.v[338] = 0.0;

        s.v[258] = 0.0;

        s.v[857] = 0.0;

        s.v[373] = 0.0;

        s.v[401] = 0.0;

        s.v[417] = 0.0;

        s.v[453] = 0.0;

        s.v[456] = 0.0;

        s.v[458] = 0.0;

        s.v[459] = 0.0;

        s.v[461] = 0.0;

        s.v[469] = 0.0;

        s.v[464] = 0.0;

        s.v[465] = 0.0;

        s.v[470] = 0.0;

        s.v[471] = 0.0;

        s.v[480] = 0.0;

        s.v[481] = 0.0;

        s.v[475] = 0.0;

        s.v[476] = 0.0;

        s.v[477] = 0.0;

        s.v[478] = 0.0;

        s.v[756] = 0.0;

        s.v[757] = 0.0;

        s.v[255] = 0.0;

        s.v[758] = 0.0;

        s.v[759] = 0.0;

        s.v[760] = 0.0;

        s.v[770] = 0.0;

        s.v[771] = 0.0;

        s.v[251] = 0.0;

        s.v[772] = 0.0;

        s.v[773] = 0.0;

        s.v[774] = 0.0;

        s.v[488] = 0.0;

        s.v[494] = 0.0;

        s.v[495] = 0.0;

        s.v[496] = 0.0;

        s.v[498] = 0.0;

        s.v[499] = 0.0;

        s.v[519] = 0.0;

        s.v[520] = 0.0;

        s.v[523] = 0.0;

        s.v[524] = 0.0;

        s.v[525] = 0.0;

        s.v[526] = 0.0;

        s.v[527] = 0.0;

        s.v[528] = 0.0;

        s.v[529] = 0.0;

        s.v[533] = 0.0;

        s.v[537] = 0.0;

        s.v[538] = 0.0;

        s.v[539] = 0.0;

        s.v[540] = 0.0;

        s.v[544] = 0.0;

        s.v[545] = 0.0;

        s.v[546] = 0.0;

        s.v[547] = 0.0;

        s.v[541] = 0.0;

        s.v[542] = 0.0;

        s.v[543] = 0.0;

        s.v[551] = 0.0;

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[548] = 0.0;

        s.v[549] = 0.0;

        s.v[550] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[567] = 0.0;

        s.v[568] = 0.0;

        s.v[569] = 0.0;

        s.v[570] = 0.0;

        s.v[594] = 0.0;

        s.v[589] = 0.0;

        s.v[577] = 0.0;

        s.v[590] = 0.0;

        s.v[591] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[588] = 0.0;

        s.v[620] = 0.0;

        s.v[631] = 0.0;

        s.v[632] = 0.0;

        s.v[633] = 0.0;

        s.v[634] = 0.0;

        s.v[668] = 0.0;

        s.v[665] = 0.0;

        s.v[677] = 0.0;

        s.v[806] = 0.0;

        s.v[370] = 0.0;

        s.v[689] = 0.0;

        s.v[690] = 0.0;

        s.v[691] = 0.0;

        s.v[692] = 0.0;

        s.v[693] = 0.0;

        s.v[871] = 0.0;

        s.v[872] = 0.0;

        s.v[680] = 0.0;

        s.v[699] = 0.0;

        s.v[658] = 0.0;

        s.v[791] = 0.0;

        s.v[701] = 0.0;

        s.v[851] = 0.0;

        s.v[706] = 0.0;

        s.v[710] = 0.0;

        s.v[815] = 0.0;

        s.v[809] = 0.0;

        s.v[817] = 0.0;

        s.v[816] = 0.0;

        s.v[818] = 0.0;

        s.v[845] = 0.0;

        s.v[846] = 0.0;

        s.v[825] = 0.0;

        s.v[828] = 0.0;

        s.v[843] = 0.0;

        s.v[844] = 0.0;

        s.v[715] = 0.0;

        s.v[717] = 0.0;

        s.v[796] = 0.0;

        s.v[646] = 0.0;

        s.v[647] = 0.0;

        s.v[645] = 0.0;

        s.v[644] = 0.0;

        s.v[873] = 0.0;

        s.v[874] = 0.0;

        s.v[893] = 0.0;

        s.v[894] = 0.0;

        s.v[895] = 0.0;

        s.v[896] = 0.0;

        s.v[898] = 0.0;

        s.v[903] = 0.0;

        s.v[904] = 0.0;

        s.v[923] = 0.0;

        s.v[392] = 0.0;

        s.v[393] = 0.0;

        s.v[503] = 0.0;

        s.v[504] = 0.0;

        s.v[949] = 0.0;

        s.v[950] = 0.0;

        s.v[951] = 0.0;

        s.v[952] = 0.0;

        s.v[953] = 0.0;

        s.v[955] = 0.0;

        s.v[956] = 0.0;

        s.v[957] = 0.0;

        s.v[958] = 0.0;

        s.v[959] = 0.0;

        s.v[1004] = 0.0;

        s.v[1005] = 0.0;

        s.v[1006] = 0.0;

        s.v[1007] = 0.0;

        s.v[1008] = 0.0;

        s.v[1009] = 0.0;

        s.v[983] = 1.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.v[962] = 0.0;

        s.v[963] = 0.0;

        s.v[964] = 0.0;

        s.v[965] = 0.0;

        s.v[984] = 0.0;

        s.v[985] = 0.0;

        s.v[986] = 0.0;

        s.v[1010] = 0.0;

        s.v[1011] = 0.0;

        s.v[1012] = 0.0;

        s.v[882] = 0.0;

        s.v[883] = 0.0;

        s.v[884] = 0.0;

        s.v[885] = 0.0;

        s.v[886] = 0.0;

        s.v[887] = 0.0;

        s.v[888] = 0.0;

        s.v[889] = 0.0;

        s.v[890] = 0.0;

        s.v[891] = 0.0;

        s.v[892] = 0.0;

        s.v[119] = 0.0;

        s.v[120] = 0.0;

        s.v[118] = 0.0;

        s.v[117] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[182] = 0.0;

        s.v[142] = 0.0;

        s.v[324] = 0.0;

        s.v[327] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[310] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[312] = 0.0;

        s.v[331] = 0.0;

        s.v[330] = 0.0;

        s.v[1041] = 0.0;

        s.v[1040] = 0.0;

        s.v[1039] = 0.0;

        s.v[1043] = 0.0;

        s.v[1042] = 0.0;

        s.v[446] = 0.0;

        s.v[595] = 0.0;

        s.v[576] = 0.0;

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
        s.v[596] = 0.0;

        s.v[1052] = 0.0;

        s.v[1057] = if (p.p60 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1057] != 0.0) {
            s.store_scalar(114, 1.0);
        }

        if (!(s.v[1057] != 0.0)) {
            s.store_scalar(114, (-1.0));
        }

        s.v[143] = (p.p103 * 8.8542e-12);

        s.v[144] = (p.p1088 * 8.8542e-12);

        s.v[165] = ((p.p102 * 8.8542e-12) / p.p91);

        s.v[145] = (p.p103 / p.p102);

        s.v[381] = (0.916 * 9.11e-31);

        s.v[382] = (0.19 * 9.11e-31);

        s.v[383] = (0.19 * 9.11e-31);

        s.v[384] = (0.417 * 9.11e-31);

        s.v[385] = 4.0;

        s.v[386] = 2.0;

        s.v[876] = (((p.p109 + ((1e-6 * p.p110) / p.p0)) + (p.p111 / p.p5)) + ((p.p112 * 1e-6) / (p.p0 * p.p5)));

        s.v[878] = (((p.p117 + ((1e-6 * p.p118) / p.p0)) + (p.p119 / p.p5)) + ((p.p120 * 1e-6) / (p.p0 * p.p5)));

        s.v[877] = (((p.p113 + ((1e-6 * p.p114) / p.p0)) + (p.p115 / p.p5)) + ((p.p116 * 1e-6) / (p.p0 * p.p5)));

        s.v[149] = (p.p0 + s.v[876]);

        s.v[1058] = if (s.v[149] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1058] != 0.0) {
            s.store_scalar(149, p.p0);
        }

        s.store_powf(168, 149, (-p.p84));

        s.store_offset_scaled(150, 168, p.p83, s.v[877]);

        s.store_offset_ad(151, A::scale(A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83), s.v[877]);

        s.store_offset_scaled(152, 168, p.p88, p.p85);

        s.store_sub_ad_rhs(153, 149, A::scale(s.ad_value(150), 2.0));

        s.store_sub_ad(155, A::offset(s.ad_value(149), s.v[878]), A::scale(s.ad_value(151), 2.0));

        s.store_sub_ad_rhs(156, 149, A::scale(s.ad_value(152), 2.0));

        s.store_offset(157, 156, (-p.p86));

        s.v[1059] = if (s.v[153] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1059] != 0.0) {
            s.copy_ad(153, 149);
        }

        s.v[1061] = if (s.v[155] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1061] != 0.0) {
            s.copy_ad(155, 149);
        }

        s.v[1063] = if (s.v[156] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1063] != 0.0) {
            s.copy_ad(156, 149);
        }

        s.v[1065] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1066] = if (s.v[157] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1065] != 0.0) && (s.v[1066] != 0.0)) {
            s.copy_ad(157, 149);
        }

        s.v[1068] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1068] != 0.0) {
            s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));
        }

        if (s.v[1068] != 0.0) {
            s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));
        }

        if (!(s.v[1068] != 0.0)) {
            s.store_scalar(879, 0.0);
        }

        if (!(s.v[1068] != 0.0)) {
            s.store_scalar(880, 0.0);
        }

        s.store_offset(161, 879, p.p43);

        s.store_add(162, 161, 880);

        s.v[1069] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1070] = if (s.v[162] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1069] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_scalar(162, p.p43);
        }

        s.v[115] = (p.p5 * p.p59);

        s.store_div_from_scalar(635, 1e-6, 155);

        s.v[636] = (1.0 / p.p5);

        s.store_div_from_scalar_ad(637, 1e-6, A::scale(s.ad_value(155), p.p5));

        s.v[1072] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1072] != 0.0) {
            s.store_div_from_scalar(638, 1e-6, 162);
        }

        if (s.v[1072] != 0.0) {
            s.store_div_from_scalar_ad(639, 1e-12, A::mul(s.ad_value(162), s.ad_value(155)));
        }

        if (!(s.v[1072] != 0.0)) {
            s.store_scalar(638, 0.0);
        }

        if (!(s.v[1072] != 0.0)) {
            s.store_scalar(639, 0.0);
        }

        s.store_add_ad(640, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p134), p.p133), (s.v[636] * p.p135)), A::scale(s.ad_value(637), p.p136)), A::scale(s.ad_value(638), 0.0)), A::scale(s.ad_value(639), 0.0));

        s.v[1073] = if (p.p95 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1073] != 0.0) {
            s.store_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1074] = if (s.v[640] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1074] != 0.0) {
            s.store_scalar(640, 1e22);
        }

        s.v[1076] = if (p.p62 == 0.0) { 1.0 } else { 0.0 };

        s.v[1077] = if (p.p62 == 1.0) { 1.0 } else { 0.0 };

        s.v[1078] = if (p.p62 == 2.0) { 1.0 } else { 0.0 };

        s.v[1079] = if (p.p62 == 3.0) { 1.0 } else { 0.0 };

        s.v[1080] = if (p.p62 == 4.0) { 1.0 } else { 0.0 };

        s.v[1081] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1082] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scalar(895, (2.0 * p.p92));
        }

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.v[1083] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scalar(895, ((2.0 * p.p92) + p.p3));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.v[1084] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        if ((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) {
            s.store_scalar(896, p.p1803);
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(895, (3.141592653589793 * p.p2));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(896, p.p2);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(895, p.p1801);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(893, p.p1800);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(894, p.p1799);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_offset_ad(954, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p44);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_offset_scaled(948, 161, p.p40, p.p45);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.copy_ad(895, 954);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.copy_ad(894, 948);
        }

        s.v[1085] = if (p.p56 > 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_offset_ad(955, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p46);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_offset_scaled(949, 161, p.p40, p.p47);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_add(895, 954, 955);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_add(894, 948, 949);
        }

        s.v[1086] = if (p.p56 > 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_offset_ad(956, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p48);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_offset_scaled(950, 161, p.p40, p.p49);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(s.ad_value(954), s.ad_value(955)), 956);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(s.ad_value(948), s.ad_value(949)), 950);
        }

        s.v[1087] = if (p.p56 > 3.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(957, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p50);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_offset_scaled(951, 161, p.p40, p.p51);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), 957);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), 951);
        }

        s.v[1088] = if (p.p56 > 4.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_offset_ad(958, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p52);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_offset_scaled(952, 161, p.p40, p.p53);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), s.ad_value(957)), 958);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), s.ad_value(951)), 952);
        }

        s.v[1089] = if (p.p56 > 5.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_offset_ad(959, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p54);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_offset_scaled(953, 161, p.p40, p.p55);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), s.ad_value(957)), s.ad_value(958)), 959);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), s.ad_value(951)), s.ad_value(952)), 953);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_scalar(896, p.p43);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        s.store_div_ad(898, A::scale(s.ad_value(893), 2.0), A::div(A::scale(A::square(s.ad_value(895)), s.v[143]), s.ad_value(894)));

        s.store_div_ad_lhs(903, A::mul(A::scale(s.ad_value(640), (-1.60219e-19)), s.ad_value(894)), 893);

        s.store_div(163, 893, 895);

        s.v[1090] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1090] != 0.0) {
            s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));
        }

        s.store_offset(158, 895, (-p.p93));

        s.store_offset(159, 895, (-p.p94));

        s.v[1091] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1091] != 0.0) {
            s.store_offset(160, 158, (-((2.0 * p.p56) * p.p87)));
        }

        if (!(s.v[1091] != 0.0)) {
            s.copy_ad(160, 158);
        }

        s.v[1092] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1093] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1094] = if (s.v[160] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1092] != 0.0) && (s.v[1093] != 0.0)) && (s.v[1094] != 0.0)) {
            s.copy_ad(160, 895);
        }

        s.v[446] = p.p1085;

        s.v[577] = p.p1680;

        s.store_add_ad(641, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p138), p.p137), (s.v[636] * p.p139)), A::scale(s.ad_value(637), p.p140)), A::scale(s.ad_value(638), p.p141)), A::scale(s.ad_value(639), p.p142));

        s.store_add_ad(666, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p189), p.p188), (s.v[636] * p.p190)), A::scale(s.ad_value(637), p.p191)), A::scale(s.ad_value(638), p.p192)), A::scale(s.ad_value(639), p.p193));

        s.store_add_ad(662, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p201), p.p200), (s.v[636] * p.p202)), A::scale(s.ad_value(637), p.p203)), A::scale(s.ad_value(638), p.p204)), A::scale(s.ad_value(639), p.p205));

        s.store_add_ad(663, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p207), p.p206), (s.v[636] * p.p208)), A::scale(s.ad_value(637), p.p209)), A::scale(s.ad_value(638), p.p210)), A::scale(s.ad_value(639), p.p211));

        s.store_add_ad(667, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p219), p.p218), (s.v[636] * p.p220)), A::scale(s.ad_value(637), p.p221)), A::scale(s.ad_value(638), p.p222)), A::scale(s.ad_value(639), p.p223));

        s.store_add_ad(670, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p225), p.p224), (s.v[636] * p.p226)), A::scale(s.ad_value(637), p.p227)), A::scale(s.ad_value(638), p.p228)), A::scale(s.ad_value(639), p.p229));

        s.store_add_ad(671, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p231), p.p230), (s.v[636] * p.p232)), A::scale(s.ad_value(637), p.p233)), A::scale(s.ad_value(638), p.p234)), A::scale(s.ad_value(639), p.p235));

        s.store_add_ad(672, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p237), p.p236), (s.v[636] * p.p238)), A::scale(s.ad_value(637), p.p239)), A::scale(s.ad_value(638), p.p240)), A::scale(s.ad_value(639), p.p241));

        s.store_add_ad(673, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p243), p.p242), (s.v[636] * p.p244)), A::scale(s.ad_value(637), p.p245)), A::scale(s.ad_value(638), p.p246)), A::scale(s.ad_value(639), p.p247));

        s.store_add_ad(674, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p249), p.p248), (s.v[636] * p.p250)), A::scale(s.ad_value(637), p.p251)), A::scale(s.ad_value(638), p.p252)), A::scale(s.ad_value(639), p.p253));

        s.store_add_ad(678, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p267), p.p266), (s.v[636] * p.p268)), A::scale(s.ad_value(637), p.p269)), A::scale(s.ad_value(638), p.p270)), A::scale(s.ad_value(639), p.p271));

        s.store_add_ad(802, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p273), p.p272), (s.v[636] * p.p274)), A::scale(s.ad_value(637), p.p275)), A::scale(s.ad_value(638), p.p276)), A::scale(s.ad_value(639), p.p277));

        s.store_add_ad(803, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p279), p.p278), (s.v[636] * p.p280)), A::scale(s.ad_value(637), p.p281)), A::scale(s.ad_value(638), p.p282)), A::scale(s.ad_value(639), p.p283));

        s.store_add_ad(804, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p285), p.p284), (s.v[636] * p.p286)), A::scale(s.ad_value(637), p.p287)), A::scale(s.ad_value(638), p.p288)), A::scale(s.ad_value(639), p.p289));

        s.store_add_ad(683, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p297), p.p296), (s.v[636] * p.p298)), A::scale(s.ad_value(637), p.p299)), A::scale(s.ad_value(638), p.p300)), A::scale(s.ad_value(639), p.p301));

        s.store_add_ad(684, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p303), p.p302), (s.v[636] * p.p304)), A::scale(s.ad_value(637), p.p305)), A::scale(s.ad_value(638), p.p306)), A::scale(s.ad_value(639), p.p307));

        s.store_add_ad(685, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p309), p.p308), (s.v[636] * p.p310)), A::scale(s.ad_value(637), p.p311)), A::scale(s.ad_value(638), p.p312)), A::scale(s.ad_value(639), p.p313));

        s.store_add_ad(686, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p315), p.p314), (s.v[636] * p.p316)), A::scale(s.ad_value(637), p.p317)), A::scale(s.ad_value(638), p.p318)), A::scale(s.ad_value(639), p.p319));

        s.store_add_ad(687, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p321), p.p320), (s.v[636] * p.p322)), A::scale(s.ad_value(637), p.p323)), A::scale(s.ad_value(638), p.p324)), A::scale(s.ad_value(639), p.p325));

        s.store_add_ad(688, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p327), p.p326), (s.v[636] * p.p328)), A::scale(s.ad_value(637), p.p329)), A::scale(s.ad_value(638), p.p330)), A::scale(s.ad_value(639), p.p331));

        s.store_add_ad(867, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p333), p.p332), (s.v[636] * p.p334)), A::scale(s.ad_value(637), p.p335)), A::scale(s.ad_value(638), p.p336)), A::scale(s.ad_value(639), p.p337));

        s.store_add_ad(868, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p339), p.p338), (s.v[636] * p.p340)), A::scale(s.ad_value(637), p.p341)), A::scale(s.ad_value(638), p.p342)), A::scale(s.ad_value(639), p.p343));

        s.store_add_ad(869, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p345), p.p344), (s.v[636] * p.p346)), A::scale(s.ad_value(637), p.p347)), A::scale(s.ad_value(638), p.p348)), A::scale(s.ad_value(639), p.p349));

        s.store_add_ad(870, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p351), p.p350), (s.v[636] * p.p352)), A::scale(s.ad_value(637), p.p353)), A::scale(s.ad_value(638), p.p354)), A::scale(s.ad_value(639), p.p355));

        s.store_add_ad(654, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p404), p.p403), (s.v[636] * p.p405)), A::scale(s.ad_value(637), p.p406)), A::scale(s.ad_value(638), p.p407)), A::scale(s.ad_value(639), p.p408));

        s.store_add_ad(655, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p410), p.p409), (s.v[636] * p.p411)), A::scale(s.ad_value(637), p.p412)), A::scale(s.ad_value(638), p.p413)), A::scale(s.ad_value(639), p.p414));

        s.store_add_ad(656, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p416), p.p415), (s.v[636] * p.p417)), A::scale(s.ad_value(637), p.p418)), A::scale(s.ad_value(638), p.p419)), A::scale(s.ad_value(639), p.p420));

        s.store_add_ad(661, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p422), p.p421), (s.v[636] * p.p423)), A::scale(s.ad_value(637), p.p424)), A::scale(s.ad_value(638), p.p425)), A::scale(s.ad_value(639), p.p426));

        s.store_add_ad(679, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p456), p.p455), (s.v[636] * p.p457)), A::scale(s.ad_value(637), p.p458)), A::scale(s.ad_value(638), p.p459)), A::scale(s.ad_value(639), p.p460));

        s.store_add_ad(698, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p468), p.p467), (s.v[636] * p.p469)), A::scale(s.ad_value(637), p.p470)), A::scale(s.ad_value(638), p.p471)), A::scale(s.ad_value(639), p.p472));

        s.store_add_ad(702, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p507), p.p506), (s.v[636] * p.p508)), A::scale(s.ad_value(637), p.p509)), A::scale(s.ad_value(638), p.p510)), A::scale(s.ad_value(639), p.p511));

        s.store_add_ad(881, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p513), p.p512), (s.v[636] * p.p514)), A::scale(s.ad_value(637), p.p515)), A::scale(s.ad_value(638), p.p516)), A::scale(s.ad_value(639), p.p517));

        s.store_add_ad(694, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p480), p.p479), (s.v[636] * p.p481)), A::scale(s.ad_value(637), p.p482)), A::scale(s.ad_value(638), p.p483)), A::scale(s.ad_value(639), p.p484));

        s.store_add_ad(695, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p486), p.p485), (s.v[636] * p.p487)), A::scale(s.ad_value(637), p.p488)), A::scale(s.ad_value(638), p.p489)), A::scale(s.ad_value(639), p.p490));

        s.store_add_ad(696, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p519), p.p518), (s.v[636] * p.p520)), A::scale(s.ad_value(637), p.p521)), A::scale(s.ad_value(638), p.p522)), A::scale(s.ad_value(639), p.p523));

        s.store_add_ad(697, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p525), p.p524), (s.v[636] * p.p526)), A::scale(s.ad_value(637), p.p527)), A::scale(s.ad_value(638), p.p528)), A::scale(s.ad_value(639), p.p529));

        s.store_add_ad(657, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p493), p.p492), (s.v[636] * p.p494)), A::scale(s.ad_value(637), p.p495)), A::scale(s.ad_value(638), p.p496)), A::scale(s.ad_value(639), p.p497));

        s.store_add_ad(790, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p532), p.p531), (s.v[636] * p.p533)), A::scale(s.ad_value(637), p.p534)), A::scale(s.ad_value(638), p.p535)), A::scale(s.ad_value(639), p.p536));

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
        s.store_add_ad(700, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p544), p.p543), (s.v[636] * p.p545)), A::scale(s.ad_value(637), p.p546)), A::scale(s.ad_value(638), p.p547)), A::scale(s.ad_value(639), p.p548));

        s.store_add_ad(704, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p606), p.p605), (s.v[636] * p.p607)), A::scale(s.ad_value(637), p.p608)), A::scale(s.ad_value(638), p.p609)), A::scale(s.ad_value(639), p.p610));

        s.store_add_ad(707, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p624), p.p623), (s.v[636] * p.p625)), A::scale(s.ad_value(637), p.p626)), A::scale(s.ad_value(638), p.p627)), A::scale(s.ad_value(639), p.p628));

        s.store_add_ad(703, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p630), p.p629), (s.v[636] * p.p631)), A::scale(s.ad_value(637), p.p632)), A::scale(s.ad_value(638), p.p633)), A::scale(s.ad_value(639), p.p634));

        s.store_add_ad(807, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p642), p.p641), (s.v[636] * p.p643)), A::scale(s.ad_value(637), p.p644)), A::scale(s.ad_value(638), p.p645)), A::scale(s.ad_value(639), p.p646));

        s.store_add_ad(811, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p678), p.p677), (s.v[636] * p.p679)), A::scale(s.ad_value(637), p.p680)), A::scale(s.ad_value(638), p.p681)), A::scale(s.ad_value(639), p.p682));

        s.store_add_ad(812, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p690), p.p689), (s.v[636] * p.p691)), A::scale(s.ad_value(637), p.p692)), A::scale(s.ad_value(638), p.p693)), A::scale(s.ad_value(639), p.p694));

        s.store_add_ad(814, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p708), p.p707), (s.v[636] * p.p709)), A::scale(s.ad_value(637), p.p710)), A::scale(s.ad_value(638), p.p711)), A::scale(s.ad_value(639), p.p712));

        s.store_add_ad(325, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p714), p.p713), (s.v[636] * p.p715)), A::scale(s.ad_value(637), p.p716)), A::scale(s.ad_value(638), p.p717)), A::scale(s.ad_value(639), p.p718));

        s.store_add_ad(326, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p720), p.p719), (s.v[636] * p.p721)), A::scale(s.ad_value(637), p.p722)), A::scale(s.ad_value(638), p.p723)), A::scale(s.ad_value(639), p.p724));

        s.store_add_ad(328, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p726), p.p725), (s.v[636] * p.p727)), A::scale(s.ad_value(637), p.p728)), A::scale(s.ad_value(638), p.p729)), A::scale(s.ad_value(639), p.p730));

        s.store_add_ad(329, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p732), p.p731), (s.v[636] * p.p733)), A::scale(s.ad_value(637), p.p734)), A::scale(s.ad_value(638), p.p735)), A::scale(s.ad_value(639), p.p736));

        s.store_add_ad(792, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1027), p.p1025), (s.v[636] * p.p1028)), A::scale(s.ad_value(637), p.p1029)), A::scale(s.ad_value(638), p.p1030)), A::scale(s.ad_value(639), p.p1031));

        s.store_add_ad(793, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1039), p.p1038), (s.v[636] * p.p1040)), A::scale(s.ad_value(637), p.p1041)), A::scale(s.ad_value(638), p.p1042)), A::scale(s.ad_value(639), p.p1043));

        s.store_add_ad(794, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1045), p.p1044), (s.v[636] * p.p1046)), A::scale(s.ad_value(637), p.p1047)), A::scale(s.ad_value(638), p.p1048)), A::scale(s.ad_value(639), p.p1049));

        s.store_add_ad(798, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1051), p.p1050), (s.v[636] * p.p1052)), A::scale(s.ad_value(637), p.p1053)), A::scale(s.ad_value(638), p.p1054)), A::scale(s.ad_value(639), p.p1055));

        s.store_add_ad(800, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1057), p.p1056), (s.v[636] * p.p1058)), A::scale(s.ad_value(637), p.p1059)), A::scale(s.ad_value(638), p.p1060)), A::scale(s.ad_value(639), p.p1061));

        s.store_add_ad(799, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1063), p.p1062), (s.v[636] * p.p1064)), A::scale(s.ad_value(637), p.p1065)), A::scale(s.ad_value(638), p.p1066)), A::scale(s.ad_value(639), p.p1067));

        s.store_add_ad(801, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1069), p.p1068), (s.v[636] * p.p1070)), A::scale(s.ad_value(637), p.p1071)), A::scale(s.ad_value(638), p.p1072)), A::scale(s.ad_value(639), p.p1073));

        s.store_add_ad(709, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p926), p.p925), (s.v[636] * p.p927)), A::scale(s.ad_value(637), p.p928)), A::scale(s.ad_value(638), p.p929)), A::scale(s.ad_value(639), p.p930));

        s.store_add_ad(853, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p932), p.p931), (s.v[636] * p.p933)), A::scale(s.ad_value(637), p.p934)), A::scale(s.ad_value(638), p.p935)), A::scale(s.ad_value(639), p.p936));

        s.store_add_ad(852, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p938), p.p937), (s.v[636] * p.p939)), A::scale(s.ad_value(637), p.p940)), A::scale(s.ad_value(638), p.p941)), A::scale(s.ad_value(639), p.p942));

        s.store_add_ad(712, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p950), p.p949), (s.v[636] * p.p951)), A::scale(s.ad_value(637), p.p952)), A::scale(s.ad_value(638), p.p953)), A::scale(s.ad_value(639), p.p954));

        s.store_add_ad(711, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p944), p.p943), (s.v[636] * p.p945)), A::scale(s.ad_value(637), p.p946)), A::scale(s.ad_value(638), p.p947)), A::scale(s.ad_value(639), p.p948));

        s.store_add_ad(713, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p956), p.p955), (s.v[636] * p.p957)), A::scale(s.ad_value(637), p.p958)), A::scale(s.ad_value(638), p.p959)), A::scale(s.ad_value(639), p.p960));

        s.store_add_ad(714, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p986), p.p985), (s.v[636] * p.p987)), A::scale(s.ad_value(637), p.p988)), A::scale(s.ad_value(638), p.p989)), A::scale(s.ad_value(639), p.p990));

        s.store_add_ad(716, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p992), p.p991), (s.v[636] * p.p993)), A::scale(s.ad_value(637), p.p994)), A::scale(s.ad_value(638), p.p995)), A::scale(s.ad_value(639), p.p996));

        s.store_add_ad(719, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1010), p.p1009), (s.v[636] * p.p1011)), A::scale(s.ad_value(637), p.p1012)), A::scale(s.ad_value(638), p.p1013)), A::scale(s.ad_value(639), p.p1014));

        s.store_add_ad(720, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1016), p.p1015), (s.v[636] * p.p1017)), A::scale(s.ad_value(637), p.p1018)), A::scale(s.ad_value(638), p.p1019)), A::scale(s.ad_value(639), p.p1020));

        s.store_add_ad(721, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1120), p.p1119), (s.v[636] * p.p1121)), A::scale(s.ad_value(637), p.p1122)), A::scale(s.ad_value(638), p.p1123)), A::scale(s.ad_value(639), p.p1124));

        s.store_add_ad(722, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1126), p.p1125), (s.v[636] * p.p1127)), A::scale(s.ad_value(637), p.p1128)), A::scale(s.ad_value(638), p.p1129)), A::scale(s.ad_value(639), p.p1130));

        s.store_add_ad(723, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1132), p.p1131), (s.v[636] * p.p1133)), A::scale(s.ad_value(637), p.p1134)), A::scale(s.ad_value(638), p.p1135)), A::scale(s.ad_value(639), p.p1136));

        s.store_add_ad(724, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1138), p.p1137), (s.v[636] * p.p1139)), A::scale(s.ad_value(637), p.p1140)), A::scale(s.ad_value(638), p.p1141)), A::scale(s.ad_value(639), p.p1142));

        s.store_add_ad(725, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1144), p.p1143), (s.v[636] * p.p1145)), A::scale(s.ad_value(637), p.p1146)), A::scale(s.ad_value(638), p.p1147)), A::scale(s.ad_value(639), p.p1148));

        s.store_add_ad(726, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1150), p.p1149), (s.v[636] * p.p1151)), A::scale(s.ad_value(637), p.p1152)), A::scale(s.ad_value(638), p.p1153)), A::scale(s.ad_value(639), p.p1154));

        s.store_add_ad(727, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1156), p.p1155), (s.v[636] * p.p1157)), A::scale(s.ad_value(637), p.p1158)), A::scale(s.ad_value(638), p.p1159)), A::scale(s.ad_value(639), p.p1160));

        s.store_add_ad(728, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1162), p.p1161), (s.v[636] * p.p1163)), A::scale(s.ad_value(637), p.p1164)), A::scale(s.ad_value(638), p.p1165)), A::scale(s.ad_value(639), p.p1166));

        s.store_add_ad(729, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1168), p.p1167), (s.v[636] * p.p1169)), A::scale(s.ad_value(637), p.p1170)), A::scale(s.ad_value(638), p.p1171)), A::scale(s.ad_value(639), p.p1172));

        s.store_add_ad(730, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1174), p.p1173), (s.v[636] * p.p1175)), A::scale(s.ad_value(637), p.p1176)), A::scale(s.ad_value(638), p.p1177)), A::scale(s.ad_value(639), p.p1178));

        s.store_add_ad(731, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1180), p.p1179), (s.v[636] * p.p1181)), A::scale(s.ad_value(637), p.p1182)), A::scale(s.ad_value(638), p.p1183)), A::scale(s.ad_value(639), p.p1184));

        s.store_add_ad(732, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1186), p.p1185), (s.v[636] * p.p1187)), A::scale(s.ad_value(637), p.p1188)), A::scale(s.ad_value(638), p.p1189)), A::scale(s.ad_value(639), p.p1190));

        s.store_add_ad(733, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1192), p.p1191), (s.v[636] * p.p1193)), A::scale(s.ad_value(637), p.p1194)), A::scale(s.ad_value(638), p.p1195)), A::scale(s.ad_value(639), p.p1196));

        s.store_add_ad(734, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1198), p.p1197), (s.v[636] * p.p1199)), A::scale(s.ad_value(637), p.p1200)), A::scale(s.ad_value(638), p.p1201)), A::scale(s.ad_value(639), p.p1202));

        s.store_add_ad(735, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1204), p.p1203), (s.v[636] * p.p1205)), A::scale(s.ad_value(637), p.p1206)), A::scale(s.ad_value(638), p.p1207)), A::scale(s.ad_value(639), p.p1208));

        s.store_add_ad(736, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1210), p.p1209), (s.v[636] * p.p1211)), A::scale(s.ad_value(637), p.p1212)), A::scale(s.ad_value(638), p.p1213)), A::scale(s.ad_value(639), p.p1214));

        s.store_add_ad(737, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1216), p.p1215), (s.v[636] * p.p1217)), A::scale(s.ad_value(637), p.p1218)), A::scale(s.ad_value(638), p.p1219)), A::scale(s.ad_value(639), p.p1220));

        s.store_add_ad(738, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1222), p.p1221), (s.v[636] * p.p1223)), A::scale(s.ad_value(637), p.p1224)), A::scale(s.ad_value(638), p.p1225)), A::scale(s.ad_value(639), p.p1226));

        s.store_add_ad(739, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1228), p.p1227), (s.v[636] * p.p1229)), A::scale(s.ad_value(637), p.p1230)), A::scale(s.ad_value(638), p.p1231)), A::scale(s.ad_value(639), p.p1232));

        s.store_add_ad(740, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1234), p.p1233), (s.v[636] * p.p1235)), A::scale(s.ad_value(637), p.p1236)), A::scale(s.ad_value(638), p.p1237)), A::scale(s.ad_value(639), p.p1238));

        s.store_add_ad(743, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1240), p.p1239), (s.v[636] * p.p1241)), A::scale(s.ad_value(637), p.p1242)), A::scale(s.ad_value(638), p.p1243)), A::scale(s.ad_value(639), p.p1244));

        s.store_add_ad(744, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1246), p.p1245), (s.v[636] * p.p1247)), A::scale(s.ad_value(637), p.p1248)), A::scale(s.ad_value(638), p.p1249)), A::scale(s.ad_value(639), p.p1250));

        s.store_add_ad(745, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1252), p.p1251), (s.v[636] * p.p1253)), A::scale(s.ad_value(637), p.p1254)), A::scale(s.ad_value(638), p.p1255)), A::scale(s.ad_value(639), p.p1256));

        s.store_add_ad(746, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1258), p.p1257), (s.v[636] * p.p1259)), A::scale(s.ad_value(637), p.p1260)), A::scale(s.ad_value(638), p.p1261)), A::scale(s.ad_value(639), p.p1262));

        s.store_add_ad(741, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1114), p.p1113), (s.v[636] * p.p1115)), A::scale(s.ad_value(637), p.p1116)), A::scale(s.ad_value(638), p.p1117)), A::scale(s.ad_value(639), p.p1118));

        s.store_add_ad(742, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1264), p.p1263), (s.v[636] * p.p1265)), A::scale(s.ad_value(637), p.p1266)), A::scale(s.ad_value(638), p.p1267)), A::scale(s.ad_value(639), p.p1268));

        s.store_add_ad(747, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1270), p.p1269), (s.v[636] * p.p1271)), A::scale(s.ad_value(637), p.p1272)), A::scale(s.ad_value(638), p.p1273)), A::scale(s.ad_value(639), p.p1274));

        s.store_add_ad(748, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1276), p.p1275), (s.v[636] * p.p1277)), A::scale(s.ad_value(637), p.p1278)), A::scale(s.ad_value(638), p.p1279)), A::scale(s.ad_value(639), p.p1280));

        s.store_add_ad(749, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1282), p.p1281), (s.v[636] * p.p1283)), A::scale(s.ad_value(637), p.p1284)), A::scale(s.ad_value(638), p.p1285)), A::scale(s.ad_value(639), p.p1286));

        s.store_add_ad(750, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1288), p.p1287), (s.v[636] * p.p1289)), A::scale(s.ad_value(637), p.p1290)), A::scale(s.ad_value(638), p.p1291)), A::scale(s.ad_value(639), p.p1292));

        s.store_add_ad(751, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1294), p.p1293), (s.v[636] * p.p1295)), A::scale(s.ad_value(637), p.p1296)), A::scale(s.ad_value(638), p.p1297)), A::scale(s.ad_value(639), p.p1298));

        s.store_add_ad(752, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1330), p.p1329), (s.v[636] * p.p1331)), A::scale(s.ad_value(637), p.p1332)), A::scale(s.ad_value(638), p.p1333)), A::scale(s.ad_value(639), p.p1334));

        s.store_add_ad(753, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1336), p.p1335), (s.v[636] * p.p1337)), A::scale(s.ad_value(637), p.p1338)), A::scale(s.ad_value(638), p.p1339)), A::scale(s.ad_value(639), p.p1340));

        s.store_add_ad(754, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1342), p.p1341), (s.v[636] * p.p1343)), A::scale(s.ad_value(637), p.p1344)), A::scale(s.ad_value(638), p.p1345)), A::scale(s.ad_value(639), p.p1346));

        s.store_add_ad(755, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1348), p.p1347), (s.v[636] * p.p1349)), A::scale(s.ad_value(637), p.p1350)), A::scale(s.ad_value(638), p.p1351)), A::scale(s.ad_value(639), p.p1352));

        s.store_add_ad(761, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1300), p.p1299), (s.v[636] * p.p1301)), A::scale(s.ad_value(637), p.p1302)), A::scale(s.ad_value(638), p.p1303)), A::scale(s.ad_value(639), p.p1304));

        s.store_add_ad(762, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1306), p.p1305), (s.v[636] * p.p1307)), A::scale(s.ad_value(637), p.p1308)), A::scale(s.ad_value(638), p.p1309)), A::scale(s.ad_value(639), p.p1310));

        s.store_add_ad(763, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1312), p.p1311), (s.v[636] * p.p1313)), A::scale(s.ad_value(637), p.p1314)), A::scale(s.ad_value(638), p.p1315)), A::scale(s.ad_value(639), p.p1316));

        s.store_add_ad(764, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1318), p.p1317), (s.v[636] * p.p1319)), A::scale(s.ad_value(637), p.p1320)), A::scale(s.ad_value(638), p.p1321)), A::scale(s.ad_value(639), p.p1322));

        s.store_add_ad(765, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1324), p.p1323), (s.v[636] * p.p1325)), A::scale(s.ad_value(637), p.p1326)), A::scale(s.ad_value(638), p.p1327)), A::scale(s.ad_value(639), p.p1328));

        s.store_add_ad(766, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1354), p.p1353), (s.v[636] * p.p1355)), A::scale(s.ad_value(637), p.p1356)), A::scale(s.ad_value(638), p.p1357)), A::scale(s.ad_value(639), p.p1358));

        s.store_add_ad(767, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1360), p.p1359), (s.v[636] * p.p1361)), A::scale(s.ad_value(637), p.p1362)), A::scale(s.ad_value(638), p.p1363)), A::scale(s.ad_value(639), p.p1364));

        s.store_add_ad(768, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1366), p.p1365), (s.v[636] * p.p1367)), A::scale(s.ad_value(637), p.p1368)), A::scale(s.ad_value(638), p.p1369)), A::scale(s.ad_value(639), p.p1370));

        s.store_add_ad(769, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1372), p.p1371), (s.v[636] * p.p1373)), A::scale(s.ad_value(637), p.p1374)), A::scale(s.ad_value(638), p.p1375)), A::scale(s.ad_value(639), p.p1376));

        s.store_add_ad(775, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1445), p.p1444), (s.v[636] * p.p1446)), A::scale(s.ad_value(637), p.p1447)), A::scale(s.ad_value(638), p.p1448)), A::scale(s.ad_value(639), p.p1449));

        s.store_add_ad(776, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1451), p.p1450), (s.v[636] * p.p1452)), A::scale(s.ad_value(637), p.p1453)), A::scale(s.ad_value(638), p.p1454)), A::scale(s.ad_value(639), p.p1455));

        s.store_add_ad(777, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1463), p.p1462), (s.v[636] * p.p1464)), A::scale(s.ad_value(637), p.p1465)), A::scale(s.ad_value(638), p.p1466)), A::scale(s.ad_value(639), p.p1467));

        s.store_add_ad(778, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1469), p.p1468), (s.v[636] * p.p1470)), A::scale(s.ad_value(637), p.p1471)), A::scale(s.ad_value(638), p.p1472)), A::scale(s.ad_value(639), p.p1473));

        s.store_add_ad(779, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1457), p.p1456), (s.v[636] * p.p1458)), A::scale(s.ad_value(637), p.p1459)), A::scale(s.ad_value(638), p.p1460)), A::scale(s.ad_value(639), p.p1461));

        s.store_add_ad(780, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1475), p.p1474), (s.v[636] * p.p1476)), A::scale(s.ad_value(637), p.p1477)), A::scale(s.ad_value(638), p.p1478)), A::scale(s.ad_value(639), p.p1479));

        s.store_add_ad(781, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1481), p.p1480), (s.v[636] * p.p1482)), A::scale(s.ad_value(637), p.p1483)), A::scale(s.ad_value(638), p.p1484)), A::scale(s.ad_value(639), p.p1485));

        s.store_add_ad(782, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1487), p.p1486), (s.v[636] * p.p1488)), A::scale(s.ad_value(637), p.p1489)), A::scale(s.ad_value(638), p.p1490)), A::scale(s.ad_value(639), p.p1491));

        s.store_add_ad(783, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1493), p.p1492), (s.v[636] * p.p1494)), A::scale(s.ad_value(637), p.p1495)), A::scale(s.ad_value(638), p.p1496)), A::scale(s.ad_value(639), p.p1497));

        s.store_add_ad(784, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1499), p.p1498), (s.v[636] * p.p1500)), A::scale(s.ad_value(637), p.p1501)), A::scale(s.ad_value(638), p.p1502)), A::scale(s.ad_value(639), p.p1503));

        s.store_add_ad(785, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1505), p.p1504), (s.v[636] * p.p1506)), A::scale(s.ad_value(637), p.p1507)), A::scale(s.ad_value(638), p.p1508)), A::scale(s.ad_value(639), p.p1509));

        s.store_add_ad(786, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1511), p.p1510), (s.v[636] * p.p1512)), A::scale(s.ad_value(637), p.p1513)), A::scale(s.ad_value(638), p.p1514)), A::scale(s.ad_value(639), p.p1515));

        s.store_add_ad(787, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1517), p.p1516), (s.v[636] * p.p1518)), A::scale(s.ad_value(637), p.p1519)), A::scale(s.ad_value(638), p.p1520)), A::scale(s.ad_value(639), p.p1521));

        s.store_add_ad(788, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1523), p.p1522), (s.v[636] * p.p1524)), A::scale(s.ad_value(637), p.p1525)), A::scale(s.ad_value(638), p.p1526)), A::scale(s.ad_value(639), p.p1527));

        s.store_add_ad(789, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1763), p.p1762), (s.v[636] * p.p1764)), A::scale(s.ad_value(637), p.p1765)), A::scale(s.ad_value(638), p.p1766)), A::scale(s.ad_value(639), p.p1767));

        s.store_add_ad(643, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1531), p.p1530), (s.v[636] * p.p1532)), A::scale(s.ad_value(637), p.p1533)), A::scale(s.ad_value(638), p.p1534)), A::scale(s.ad_value(639), p.p1535));

        s.store_add_ad(642, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1537), p.p1536), (s.v[636] * p.p1538)), A::scale(s.ad_value(637), p.p1539)), A::scale(s.ad_value(638), p.p1540)), A::scale(s.ad_value(639), p.p1541));

        s.store_add_ad(644, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p29), p.p28), (s.v[636] * p.p30)), A::scale(s.ad_value(637), p.p31)), A::scale(s.ad_value(638), p.p32)), A::scale(s.ad_value(639), p.p33));

        s.store_add_ad(645, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p35), p.p34), (s.v[636] * p.p36)), A::scale(s.ad_value(637), p.p37)), A::scale(s.ad_value(638), p.p38)), A::scale(s.ad_value(639), p.p39));

        s.store_add_ad(648, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1548), p.p1547), (s.v[636] * p.p1549)), A::scale(s.ad_value(637), p.p1550)), A::scale(s.ad_value(638), p.p1551)), A::scale(s.ad_value(639), p.p1552));

        s.store_add_ad(649, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1554), p.p1553), (s.v[636] * p.p1555)), A::scale(s.ad_value(637), p.p1556)), A::scale(s.ad_value(638), p.p1557)), A::scale(s.ad_value(639), p.p1558));

        s.store_add_ad(650, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1560), p.p1559), (s.v[636] * p.p1561)), A::scale(s.ad_value(637), p.p1562)), A::scale(s.ad_value(638), p.p1563)), A::scale(s.ad_value(639), p.p1564));

        s.store_add_ad(651, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1566), p.p1565), (s.v[636] * p.p1567)), A::scale(s.ad_value(637), p.p1568)), A::scale(s.ad_value(638), p.p1569)), A::scale(s.ad_value(639), p.p1570));

        s.store_add_ad(652, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1572), p.p1571), (s.v[636] * p.p1573)), A::scale(s.ad_value(637), p.p1574)), A::scale(s.ad_value(638), p.p1575)), A::scale(s.ad_value(639), p.p1576));

        s.store_add_ad(653, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1578), p.p1577), (s.v[636] * p.p1579)), A::scale(s.ad_value(637), p.p1580)), A::scale(s.ad_value(638), p.p1581)), A::scale(s.ad_value(639), p.p1582));

        s.store_add_ad(864, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1651), p.p1650), (s.v[636] * p.p1652)), A::scale(s.ad_value(637), p.p1653)), A::scale(s.ad_value(638), p.p1654)), A::scale(s.ad_value(639), p.p1655));

        s.store_add_ad(865, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1657), p.p1656), (s.v[636] * p.p1658)), A::scale(s.ad_value(637), p.p1659)), A::scale(s.ad_value(638), p.p1660)), A::scale(s.ad_value(639), p.p1661));

        s.store_add_ad(866, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1663), p.p1662), (s.v[636] * p.p1664)), A::scale(s.ad_value(637), p.p1665)), A::scale(s.ad_value(638), p.p1666)), A::scale(s.ad_value(639), p.p1667));

        s.store_add_ad(836, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p738), p.p737), (s.v[636] * p.p739)), A::scale(s.ad_value(637), p.p740)), A::scale(s.ad_value(638), p.p741)), A::scale(s.ad_value(639), p.p742));

        s.store_add_ad(837, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p756), p.p755), (s.v[636] * p.p757)), A::scale(s.ad_value(637), p.p758)), A::scale(s.ad_value(638), p.p759)), A::scale(s.ad_value(639), p.p760));

        s.store_add_ad(838, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p768), p.p767), (s.v[636] * p.p769)), A::scale(s.ad_value(637), p.p770)), A::scale(s.ad_value(638), p.p771)), A::scale(s.ad_value(639), p.p772));

        s.store_add_ad(842, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p786), p.p785), (s.v[636] * p.p787)), A::scale(s.ad_value(637), p.p788)), A::scale(s.ad_value(638), p.p789)), A::scale(s.ad_value(639), p.p790));

        s.store_add_ad(823, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p792), p.p791), (s.v[636] * p.p793)), A::scale(s.ad_value(637), p.p794)), A::scale(s.ad_value(638), p.p795)), A::scale(s.ad_value(639), p.p796));

        s.store_add_ad(824, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p810), p.p809), (s.v[636] * p.p811)), A::scale(s.ad_value(637), p.p812)), A::scale(s.ad_value(638), p.p813)), A::scale(s.ad_value(639), p.p814));

        s.store_add_ad(847, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p822), p.p821), (s.v[636] * p.p823)), A::scale(s.ad_value(637), p.p824)), A::scale(s.ad_value(638), p.p825)), A::scale(s.ad_value(639), p.p826));

        s.store_add_ad(830, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p846), p.p845), (s.v[636] * p.p847)), A::scale(s.ad_value(637), p.p848)), A::scale(s.ad_value(638), p.p849)), A::scale(s.ad_value(639), p.p850));

        s.store_add_ad(831, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p864), p.p863), (s.v[636] * p.p865)), A::scale(s.ad_value(637), p.p866)), A::scale(s.ad_value(638), p.p867)), A::scale(s.ad_value(639), p.p868));

        s.store_add_ad(834, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p876), p.p875), (s.v[636] * p.p877)), A::scale(s.ad_value(637), p.p878)), A::scale(s.ad_value(638), p.p879)), A::scale(s.ad_value(639), p.p880));

        s.store_add_ad(835, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p882), p.p881), (s.v[636] * p.p883)), A::scale(s.ad_value(637), p.p884)), A::scale(s.ad_value(638), p.p885)), A::scale(s.ad_value(639), p.p886));

        s.store_add_ad(848, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p576), p.p575), (s.v[636] * p.p577)), A::scale(s.ad_value(637), p.p578)), A::scale(s.ad_value(638), p.p579)), A::scale(s.ad_value(639), p.p580));

        s.store_add_ad(849, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p556), p.p555), (s.v[636] * p.p557)), A::scale(s.ad_value(637), p.p558)), A::scale(s.ad_value(638), p.p559)), A::scale(s.ad_value(639), p.p560));

        s.store_add_ad(850, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p569), p.p568), (s.v[636] * p.p570)), A::scale(s.ad_value(637), p.p571)), A::scale(s.ad_value(638), p.p572)), A::scale(s.ad_value(639), p.p573));

        s.store_add_ad(854, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p962), p.p961), (s.v[636] * p.p963)), A::scale(s.ad_value(637), p.p964)), A::scale(s.ad_value(638), p.p965)), A::scale(s.ad_value(639), p.p966));

        s.store_add_ad(855, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p968), p.p967), (s.v[636] * p.p969)), A::scale(s.ad_value(637), p.p970)), A::scale(s.ad_value(638), p.p971)), A::scale(s.ad_value(639), p.p972));

        s.store_add_ad(856, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p974), p.p973), (s.v[636] * p.p975)), A::scale(s.ad_value(637), p.p976)), A::scale(s.ad_value(638), p.p977)), A::scale(s.ad_value(639), p.p978));

        s.store_add_ad(857, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p980), p.p979), (s.v[636] * p.p981)), A::scale(s.ad_value(637), p.p982)), A::scale(s.ad_value(638), p.p983)), A::scale(s.ad_value(639), p.p984));

        s.store_add_ad(858, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1742), p.p1741), (s.v[636] * p.p1743)), A::scale(s.ad_value(637), p.p1744)), A::scale(s.ad_value(638), p.p1745)), A::scale(s.ad_value(639), p.p1746));

        s.store_add_ad(859, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1751), p.p1750), (s.v[636] * p.p1752)), A::scale(s.ad_value(637), p.p1753)), A::scale(s.ad_value(638), p.p1754)), A::scale(s.ad_value(639), p.p1755));

        s.store_add_ad(860, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1757), p.p1756), (s.v[636] * p.p1758)), A::scale(s.ad_value(637), p.p1759)), A::scale(s.ad_value(638), p.p1760)), A::scale(s.ad_value(639), p.p1761));

        s.store_add_ad(862, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1769), p.p1768), (s.v[636] * p.p1770)), A::scale(s.ad_value(637), p.p1771)), A::scale(s.ad_value(638), p.p1772)), A::scale(s.ad_value(639), p.p1773));

        s.store_add_ad(863, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1775), p.p1774), (s.v[636] * p.p1776)), A::scale(s.ad_value(637), p.p1777)), A::scale(s.ad_value(638), p.p1778)), A::scale(s.ad_value(639), p.p1779));

        s.store_add_ad(861, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1781), p.p1780), (s.v[636] * p.p1782)), A::scale(s.ad_value(637), p.p1783)), A::scale(s.ad_value(638), p.p1784)), A::scale(s.ad_value(639), p.p1785));

        s.store_add_ad(681, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p177), p.p176), (s.v[636] * p.p178)), A::scale(s.ad_value(637), p.p179)), A::scale(s.ad_value(638), p.p180)), A::scale(s.ad_value(639), p.p181));

        s.store_add_ad(682, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p183), p.p182), (s.v[636] * p.p184)), A::scale(s.ad_value(637), p.p185)), A::scale(s.ad_value(638), p.p186)), A::scale(s.ad_value(639), p.p187));

        s.store_add_ad(574, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1690), p.p1689), (s.v[636] * p.p1691)), A::scale(s.ad_value(637), p.p1692)), A::scale(s.ad_value(638), p.p1693)), A::scale(s.ad_value(639), p.p1694));

        s.store_add_ad(576, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1702), p.p1701), (s.v[636] * p.p1703)), A::scale(s.ad_value(637), p.p1704)), A::scale(s.ad_value(638), p.p1705)), A::scale(s.ad_value(639), p.p1706));

        s.store_add_ad(575, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1696), p.p1695), (s.v[636] * p.p1697)), A::scale(s.ad_value(637), p.p1698)), A::scale(s.ad_value(638), p.p1699)), A::scale(s.ad_value(639), p.p1700));

        s.v[1096] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1096] != 0.0) {
            s.store_add_ad(689, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p357), p.p356), (s.v[636] * p.p358)), A::scale(s.ad_value(637), p.p359)), A::scale(s.ad_value(638), p.p360)), A::scale(s.ad_value(639), p.p361));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(690, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p363), p.p362), (s.v[636] * p.p364)), A::scale(s.ad_value(637), p.p365)), A::scale(s.ad_value(638), p.p366)), A::scale(s.ad_value(639), p.p367));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(691, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p369), p.p368), (s.v[636] * p.p370)), A::scale(s.ad_value(637), p.p371)), A::scale(s.ad_value(638), p.p372)), A::scale(s.ad_value(639), p.p373));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(809, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p660), p.p659), (s.v[636] * p.p661)), A::scale(s.ad_value(637), p.p662)), A::scale(s.ad_value(638), p.p663)), A::scale(s.ad_value(639), p.p664));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(828, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p828), p.p827), (s.v[636] * p.p829)), A::scale(s.ad_value(637), p.p830)), A::scale(s.ad_value(638), p.p831)), A::scale(s.ad_value(639), p.p832));
        }

        s.v[1097] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(871, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p387), p.p386), (s.v[636] * p.p388)), A::scale(s.ad_value(637), p.p389)), A::scale(s.ad_value(638), p.p390)), A::scale(s.ad_value(639), p.p391));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(872, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p393), p.p392), (s.v[636] * p.p394)), A::scale(s.ad_value(637), p.p395)), A::scale(s.ad_value(638), p.p396)), A::scale(s.ad_value(639), p.p397));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(692, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p375), p.p374), (s.v[636] * p.p376)), A::scale(s.ad_value(637), p.p377)), A::scale(s.ad_value(638), p.p378)), A::scale(s.ad_value(639), p.p379));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(693, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p381), p.p380), (s.v[636] * p.p382)), A::scale(s.ad_value(637), p.p383)), A::scale(s.ad_value(638), p.p384)), A::scale(s.ad_value(639), p.p385));
        }

        s.v[1098] = if (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(756, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1378), p.p1377), (s.v[636] * p.p1379)), A::scale(s.ad_value(637), p.p1380)), A::scale(s.ad_value(638), p.p1381)), A::scale(s.ad_value(639), p.p1382));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(757, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1384), p.p1383), (s.v[636] * p.p1385)), A::scale(s.ad_value(637), p.p1386)), A::scale(s.ad_value(638), p.p1387)), A::scale(s.ad_value(639), p.p1388));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(758, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1390), p.p1389), (s.v[636] * p.p1391)), A::scale(s.ad_value(637), p.p1392)), A::scale(s.ad_value(638), p.p1393)), A::scale(s.ad_value(639), p.p1394));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(759, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1396), p.p1395), (s.v[636] * p.p1397)), A::scale(s.ad_value(637), p.p1398)), A::scale(s.ad_value(638), p.p1399)), A::scale(s.ad_value(639), p.p1400));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(760, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1402), p.p1401), (s.v[636] * p.p1403)), A::scale(s.ad_value(637), p.p1404)), A::scale(s.ad_value(638), p.p1405)), A::scale(s.ad_value(639), p.p1406));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(770, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1408), p.p1407), (s.v[636] * p.p1409)), A::scale(s.ad_value(637), p.p1410)), A::scale(s.ad_value(638), p.p1411)), A::scale(s.ad_value(639), p.p1412));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(771, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1414), p.p1413), (s.v[636] * p.p1415)), A::scale(s.ad_value(637), p.p1416)), A::scale(s.ad_value(638), p.p1417)), A::scale(s.ad_value(639), p.p1418));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(772, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1420), p.p1419), (s.v[636] * p.p1421)), A::scale(s.ad_value(637), p.p1422)), A::scale(s.ad_value(638), p.p1423)), A::scale(s.ad_value(639), p.p1424));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(773, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1426), p.p1425), (s.v[636] * p.p1427)), A::scale(s.ad_value(637), p.p1428)), A::scale(s.ad_value(638), p.p1429)), A::scale(s.ad_value(639), p.p1430));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(774, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1432), p.p1431), (s.v[636] * p.p1433)), A::scale(s.ad_value(637), p.p1434)), A::scale(s.ad_value(638), p.p1435)), A::scale(s.ad_value(639), p.p1436));
        }

        s.v[1099] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1099] != 0.0) {
            s.store_add_ad(665, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p213), p.p212), (s.v[636] * p.p214)), A::scale(s.ad_value(637), p.p215)), A::scale(s.ad_value(638), p.p216)), A::scale(s.ad_value(639), p.p217));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(668, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p195), p.p194), (s.v[636] * p.p196)), A::scale(s.ad_value(637), p.p197)), A::scale(s.ad_value(638), p.p198)), A::scale(s.ad_value(639), p.p199));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(677, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p255), p.p254), (s.v[636] * p.p256)), A::scale(s.ad_value(637), p.p257)), A::scale(s.ad_value(638), p.p258)), A::scale(s.ad_value(639), p.p259));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(699, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p474), p.p473), (s.v[636] * p.p475)), A::scale(s.ad_value(637), p.p476)), A::scale(s.ad_value(638), p.p477)), A::scale(s.ad_value(639), p.p478));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(791, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p538), p.p537), (s.v[636] * p.p539)), A::scale(s.ad_value(637), p.p540)), A::scale(s.ad_value(638), p.p541)), A::scale(s.ad_value(639), p.p542));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(701, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p550), p.p549), (s.v[636] * p.p551)), A::scale(s.ad_value(637), p.p552)), A::scale(s.ad_value(638), p.p553)), A::scale(s.ad_value(639), p.p554));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(715, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p998), p.p997), (s.v[636] * p.p999)), A::scale(s.ad_value(637), p.p1000)), A::scale(s.ad_value(638), p.p1001)), A::scale(s.ad_value(639), p.p1002));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(717, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1004), p.p1003), (s.v[636] * p.p1005)), A::scale(s.ad_value(637), p.p1006)), A::scale(s.ad_value(638), p.p1007)), A::scale(s.ad_value(639), p.p1008));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(796, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1033), p.p1032), (s.v[636] * p.p1034)), A::scale(s.ad_value(637), p.p1035)), A::scale(s.ad_value(638), p.p1036)), A::scale(s.ad_value(639), p.p1037));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(806, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p291), p.p290), (s.v[636] * p.p292)), A::scale(s.ad_value(637), p.p293)), A::scale(s.ad_value(638), p.p294)), A::scale(s.ad_value(639), p.p295));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(680, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p462), p.p461), (s.v[636] * p.p463)), A::scale(s.ad_value(637), p.p464)), A::scale(s.ad_value(638), p.p465)), A::scale(s.ad_value(639), p.p466));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(658, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p501), p.p500), (s.v[636] * p.p502)), A::scale(s.ad_value(637), p.p503)), A::scale(s.ad_value(638), p.p504)), A::scale(s.ad_value(639), p.p505));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(706, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p612), p.p611), (s.v[636] * p.p613)), A::scale(s.ad_value(637), p.p614)), A::scale(s.ad_value(638), p.p615)), A::scale(s.ad_value(639), p.p616));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(815, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p648), p.p647), (s.v[636] * p.p649)), A::scale(s.ad_value(637), p.p650)), A::scale(s.ad_value(638), p.p651)), A::scale(s.ad_value(639), p.p652));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(710, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p636), p.p635), (s.v[636] * p.p637)), A::scale(s.ad_value(637), p.p638)), A::scale(s.ad_value(638), p.p639)), A::scale(s.ad_value(639), p.p640));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(816, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p684), p.p683), (s.v[636] * p.p685)), A::scale(s.ad_value(637), p.p686)), A::scale(s.ad_value(638), p.p687)), A::scale(s.ad_value(639), p.p688));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(818, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p696), p.p695), (s.v[636] * p.p697)), A::scale(s.ad_value(637), p.p698)), A::scale(s.ad_value(638), p.p699)), A::scale(s.ad_value(639), p.p700));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(845, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p744), p.p743), (s.v[636] * p.p745)), A::scale(s.ad_value(637), p.p746)), A::scale(s.ad_value(638), p.p747)), A::scale(s.ad_value(639), p.p748));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(846, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p774), p.p773), (s.v[636] * p.p775)), A::scale(s.ad_value(637), p.p776)), A::scale(s.ad_value(638), p.p777)), A::scale(s.ad_value(639), p.p778));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(825, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p798), p.p797), (s.v[636] * p.p799)), A::scale(s.ad_value(637), p.p800)), A::scale(s.ad_value(638), p.p801)), A::scale(s.ad_value(639), p.p802));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(844, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p852), p.p851), (s.v[636] * p.p853)), A::scale(s.ad_value(637), p.p854)), A::scale(s.ad_value(638), p.p855)), A::scale(s.ad_value(639), p.p856));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(851, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p563), p.p562), (s.v[636] * p.p564)), A::scale(s.ad_value(637), p.p565)), A::scale(s.ad_value(638), p.p566)), A::scale(s.ad_value(639), p.p567));
        }

        s.v[1100] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1099] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_add_ad(817, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p666), p.p665), (s.v[636] * p.p667)), A::scale(s.ad_value(637), p.p668)), A::scale(s.ad_value(638), p.p669)), A::scale(s.ad_value(639), p.p670));
        }

        if ((s.v[1099] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_add_ad(843, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p834), p.p833), (s.v[636] * p.p835)), A::scale(s.ad_value(637), p.p836)), A::scale(s.ad_value(638), p.p837)), A::scale(s.ad_value(639), p.p838));
        }

        s.v[1101] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1101] != 0.0) {
            s.store_add_ad(705, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p618), p.p617), (s.v[636] * p.p619)), A::scale(s.ad_value(637), p.p620)), A::scale(s.ad_value(638), p.p621)), A::scale(s.ad_value(639), p.p622));
        }

        s.v[1102] = if (p.p582 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(808, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p654), p.p653), (s.v[636] * p.p655)), A::scale(s.ad_value(637), p.p656)), A::scale(s.ad_value(638), p.p657)), A::scale(s.ad_value(639), p.p658));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(813, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p702), p.p701), (s.v[636] * p.p703)), A::scale(s.ad_value(637), p.p704)), A::scale(s.ad_value(638), p.p705)), A::scale(s.ad_value(639), p.p706));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(839, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p750), p.p749), (s.v[636] * p.p751)), A::scale(s.ad_value(637), p.p752)), A::scale(s.ad_value(638), p.p753)), A::scale(s.ad_value(639), p.p754));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(840, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p762), p.p761), (s.v[636] * p.p763)), A::scale(s.ad_value(637), p.p764)), A::scale(s.ad_value(638), p.p765)), A::scale(s.ad_value(639), p.p766));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(841, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p780), p.p779), (s.v[636] * p.p781)), A::scale(s.ad_value(637), p.p782)), A::scale(s.ad_value(638), p.p783)), A::scale(s.ad_value(639), p.p784));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(826, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p804), p.p803), (s.v[636] * p.p805)), A::scale(s.ad_value(637), p.p806)), A::scale(s.ad_value(638), p.p807)), A::scale(s.ad_value(639), p.p808));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(827, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p816), p.p815), (s.v[636] * p.p817)), A::scale(s.ad_value(637), p.p818)), A::scale(s.ad_value(638), p.p819)), A::scale(s.ad_value(639), p.p820));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(832, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p858), p.p857), (s.v[636] * p.p859)), A::scale(s.ad_value(637), p.p860)), A::scale(s.ad_value(638), p.p861)), A::scale(s.ad_value(639), p.p862));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(833, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p870), p.p869), (s.v[636] * p.p871)), A::scale(s.ad_value(637), p.p872)), A::scale(s.ad_value(638), p.p873)), A::scale(s.ad_value(639), p.p874));
        }

        s.v[1103] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_add_ad(810, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p672), p.p671), (s.v[636] * p.p673)), A::scale(s.ad_value(637), p.p674)), A::scale(s.ad_value(638), p.p675)), A::scale(s.ad_value(639), p.p676));
        }

        if ((s.v[1101] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_add_ad(829, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p840), p.p839), (s.v[636] * p.p841)), A::scale(s.ad_value(637), p.p842)), A::scale(s.ad_value(638), p.p843)), A::scale(s.ad_value(639), p.p844));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(675, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p261), p.p260), (s.v[636] * p.p262)), A::scale(s.ad_value(637), p.p263)), A::scale(s.ad_value(638), p.p264)), A::scale(s.ad_value(639), p.p265));
        }

        s.v[1104] = if (p.p161 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1104] != 0.0)) {
            s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1105] = if (p.p21 != 0.0) { 1.0 } else { 0.0 };

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
        if ((s.v[1101] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(705, 705, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p588)), 1.0));
        }

        if ((s.v[1101] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(675, 675, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p163)), 1.0));
        }

        s.v[1106] = if ((p.p73 != 0.0) && (p.p1668 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1106] != 0.0) {
            s.store_add_ad(873, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1669), p.p1668), (s.v[636] * p.p1670)), A::scale(s.ad_value(637), p.p1671)), A::scale(s.ad_value(638), p.p1672)), A::scale(s.ad_value(639), p.p1673));
        }

        if (s.v[1106] != 0.0) {
            s.store_add_ad(874, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1675), p.p1674), (s.v[636] * p.p1676)), A::scale(s.ad_value(637), p.p1677)), A::scale(s.ad_value(638), p.p1678)), A::scale(s.ad_value(639), p.p1679));
        }

        s.v[1107] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1107] != 0.0) {
            s.store_add_ad(882, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1808), p.p1807), (s.v[636] * p.p1809)), A::scale(s.ad_value(637), p.p1810)), A::scale(s.ad_value(638), p.p1811)), A::scale(s.ad_value(639), p.p1812));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(883, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1815), p.p1814), (s.v[636] * p.p1816)), A::scale(s.ad_value(637), p.p1817)), A::scale(s.ad_value(638), p.p1818)), A::scale(s.ad_value(639), p.p1819));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(884, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1822), p.p1821), (s.v[636] * p.p1823)), A::scale(s.ad_value(637), p.p1824)), A::scale(s.ad_value(638), p.p1825)), A::scale(s.ad_value(639), p.p1826));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(885, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1830), p.p1829), (s.v[636] * p.p1831)), A::scale(s.ad_value(637), p.p1832)), A::scale(s.ad_value(638), p.p1833)), A::scale(s.ad_value(639), p.p1834));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(886, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1836), p.p1835), (s.v[636] * p.p1837)), A::scale(s.ad_value(637), p.p1838)), A::scale(s.ad_value(638), p.p1839)), A::scale(s.ad_value(639), p.p1840));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(887, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1842), p.p1841), (s.v[636] * p.p1843)), A::scale(s.ad_value(637), p.p1844)), A::scale(s.ad_value(638), p.p1845)), A::scale(s.ad_value(639), p.p1846));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(888, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1854), p.p1853), (s.v[636] * p.p1855)), A::scale(s.ad_value(637), p.p1856)), A::scale(s.ad_value(638), p.p1857)), A::scale(s.ad_value(639), p.p1858));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(889, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1860), p.p1859), (s.v[636] * p.p1861)), A::scale(s.ad_value(637), p.p1862)), A::scale(s.ad_value(638), p.p1863)), A::scale(s.ad_value(639), p.p1864));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(890, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1870), p.p1869), (s.v[636] * p.p1871)), A::scale(s.ad_value(637), p.p1872)), A::scale(s.ad_value(638), p.p1873)), A::scale(s.ad_value(639), p.p1874));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(891, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1876), p.p1875), (s.v[636] * p.p1877)), A::scale(s.ad_value(637), p.p1878)), A::scale(s.ad_value(638), p.p1879)), A::scale(s.ad_value(639), p.p1880));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(892, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1882), p.p1881), (s.v[636] * p.p1883)), A::scale(s.ad_value(637), p.p1884)), A::scale(s.ad_value(638), p.p1885)), A::scale(s.ad_value(639), p.p1886));
        }

        s.v[1108] = if (p.p100 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1108] != 0.0) {
            s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1109] = if (p.p158 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1109] != 0.0) {
            s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1110] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1110] != 0.0) {
            s.store_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1111] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1111] != 0.0) {
            s.store_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1112] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1112] != 0.0) {
            s.store_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1113] = if (p.p428 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1113] != 0.0) {
            s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1114] = if (p.p432 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1114] != 0.0) {
            s.store_scale(698, 698, (1.0 + ((p.p432 / p.p5) * (if (!((1.0 + (p.p5 / p.p433)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p433)) > 1e-38) { (((1.0 + (p.p5 / p.p433))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1115] = if (p.p434 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1115] != 0.0) {
            s.store_scale(699, 699, (1.0 + ((p.p434 / p.p5) * (if (!((1.0 + (p.p5 / p.p435)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p435)) > 1e-38) { (((1.0 + (p.p5 / p.p435))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1116] = if (p.p581 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1116] != 0.0) {
            s.store_scale(704, 704, (1.0 + ((p.p581 / p.p5) * (if (!((1.0 + (p.p5 / p.p584)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p584)) > 1e-38) { (((1.0 + (p.p5 / p.p584))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1117] = if (p.p583 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1117] != 0.0) {
            s.store_scale(706, 706, (1.0 + ((p.p583 / p.p5) * (if (!((1.0 + (p.p5 / p.p586)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p586)) > 1e-38) { (((1.0 + (p.p5 / p.p586))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1118] = if (p.p21 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1118] != 0.0) {
            s.store_mul_ad_rhs(641, 641, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p99)), 1.0));
        }

        if (s.v[1118] != 0.0) {
            s.store_mul_ad_rhs(673, 673, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p160)), 1.0));
        }

        if (s.v[1118] != 0.0) {
            s.store_mul_ad_rhs(704, 704, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p587)), 1.0));
        }

        s.store_ln(154, 153);

        s.store_add_ad_rhs(641, 641, A::scale(s.ad_value(153), p.p98));

        s.store_add_ad_rhs(661, 661, A::scale(s.ad_value(153), p.p427));

        s.v[1119] = if (p.p589 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1119] != 0.0) {
            s.store_mul_ad_rhs(704, 704, A::sub_from_scalar(1.0, A::mul(s.ad_value(703), A::exp(A::scale(s.ad_value(154), (-p.p589))))));
        }

        if (!(s.v[1119] != 0.0)) {
            s.store_mul_ad_rhs(704, 704, A::sub_from_scalar(1.0, s.ad_value(703)));
        }

        s.store_add_ad_rhs(807, 807, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p593))), p.p591));

        s.store_add_ad_rhs(812, 812, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p601))), p.p599));

        s.store_add_ad_rhs(811, 811, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p597))), p.p595));

        s.v[1120] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1120] != 0.0) {
            s.store_add_ad_rhs(815, 815, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p594))), p.p592));
        }

        if (s.v[1120] != 0.0) {
            s.store_add_ad_rhs(818, 818, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p602))), p.p600));
        }

        if (s.v[1120] != 0.0) {
            s.store_add_ad_rhs(816, 816, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p598))), p.p596));
        }

        s.v[1121] = if (p.p590 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1120] != 0.0) && (s.v[1121] != 0.0)) {
            s.store_mul_ad_rhs(706, 706, A::sub_from_scalar(1.0, A::mul(s.ad_value(710), A::exp(A::scale(s.ad_value(154), (-p.p590))))));
        }

        if ((s.v[1120] != 0.0) && (!(s.v[1121] != 0.0))) {
            s.store_mul_ad_rhs(706, 706, A::sub_from_scalar(1.0, s.ad_value(710)));
        }

        s.v[1122] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1122] != 0.0) {
            s.store_add_ad_rhs(853, 853, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p913))), p.p912));
        }

        if (s.v[1122] != 0.0) {
            s.store_add_ad_rhs(852, 852, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p916))), p.p915));
        }

        if (!(s.v[1122] != 0.0)) {
            s.store_add_ad_rhs(709, 709, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p910))), p.p909));
        }

        s.store_add_ad_rhs(792, 792, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p1023))), p.p1021));

        s.v[1123] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1123] != 0.0) {
            s.store_add_ad_rhs(796, 796, A::scale(A::exp(A::scale(s.ad_value(154), (-p.p1024))), p.p1022));
        }

        s.store_add_ad_rhs(790, 790, A::scale(A::exp(A::scale(s.ad_value(154), (-p.p445))), p.p444));

        s.v[1124] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1124] != 0.0) {
            s.store_add_ad_rhs(791, 791, A::scale(A::exp(A::scale(s.ad_value(154), (-p.p447))), p.p446));
        }

        s.store_add_ad_rhs(700, 700, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p449))), p.p448));

        s.v[1125] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1125] != 0.0) {
            s.store_add_ad_rhs(701, 701, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p449))), p.p448));
        }

        s.store_add_ad_rhs(679, 679, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p431))), p.p430));

        s.store_add_ad_rhs(698, 698, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p437))), p.p436));

        s.v[1126] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1126] != 0.0) {
            s.store_add_ad_rhs(699, 699, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p437))), p.p436));
        }

        s.store_add_ad_rhs(695, 695, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p439))), p.p438));

        s.store_add_ad_rhs(697, 697, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p443))), p.p442));

        s.store_add_ad_rhs(702, 702, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p441))), p.p440));

        s.store_add_ad_rhs(681, 681, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p168))), p.p167));

        s.store_add_ad_rhs(682, 682, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(153)), 1.0 / (p.p170))), p.p169));

        s.v[1127] = if ((s.v[655] > 0.0) || (s.v[656] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1127] != 0.0) {
            s.store_offset_ad(376, A::scale(A::limited_exp(A::scale(A::neg(A::div(A::scale(s.ad_value(894), 2.0), s.ad_value(895))), 1.0 / (p.p399))), p.p398), 1.0);
        }

        if (s.v[1127] != 0.0) {
            s.store_mul_ad_lhs(373, A::div(A::scale(s.ad_value(894), 2.0), s.ad_value(895)), 376);
        }

        s.v[1130] = if (s.v[576] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1130] != 0.0) {
            s.store_scalar(576, 0.05);
        }

        s.v[1135] = if (s.v[641] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1135] != 0.0) {
            s.store_scalar(641, 4.61);
        }

        s.v[1136] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1137] = if (s.v[690] < 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[1136] != 0.0) && (s.v[1137] != 0.0)) {
            s.store_scalar(690, 1e-6);
        }

        s.v[1138] = if (s.v[857] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1138] != 0.0) {
            s.store_scalar(857, 0.01);
        }

        s.v[1139] = if (s.v[576] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1139] != 0.0) {
            s.store_scalar(576, 0.05);
        }

        s.v[1140] = if (s.v[574] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_scalar(574, p.p1682);
        }

        s.v[1141] = if (s.v[575] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_scalar(575, 1.2);
        }

        s.v[1142] = if (s.v[644] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_scalar(644, 0.0);
        }

        s.v[1143] = if (s.v[645] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_scalar(645, 0.0);
        }

        s.v[1144] = if (s.v[679] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1144] != 0.0) {
            s.store_scalar(679, 85000.0);
        }

        s.v[1145] = if (s.v[698] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_scalar(698, 85000.0);
        }

        s.v[1146] = if ((p.p66 != 0.0) && (s.v[699] <= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_scalar(699, 85000.0);
        }

        s.v[1147] = if (s.v[670] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1147] != 0.0) {
            s.store_scalar(670, 0.6);
        }

        s.v[1148] = if (s.v[671] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1148] != 0.0) {
            s.store_scalar(671, 0.6);
        }

        s.v[1152] = if (s.v[678] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_scalar(678, 1.06);
        }

        s.v[1153] = if (s.v[673] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_scalar(673, 0.0);
        }

        s.v[1154] = if (s.v[677] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_scalar(677, 0.0);
        }

        s.v[1155] = if (s.v[803] < (-s.v[153])) { 1.0 } else { 0.0 };

        if (s.v[1155] != 0.0) {
            s.store_scalar(803, 0.0);
        }

        s.v[1156] = if (s.v[685] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_scalar(685, 0.0);
        }

        s.v[1157] = if (s.v[687] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_scalar(687, 0.0);
        }

        s.v[1158] = if ((p.p61 != 0.0) && (s.v[689] < 0.2)) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_scalar(689, 0.2);
        }

        s.v[1159] = if ((p.p61 != 0.0) && (s.v[689] > 1.2)) { 1.0 } else { 0.0 };

        if (s.v[1159] != 0.0) {
            s.store_scalar(689, 1.2);
        }

        s.v[1160] = if (s.v[695] < 2.0) { 1.0 } else { 0.0 };

        if (s.v[1160] != 0.0) {
            s.store_scalar(695, 2.0);
        }

        s.v[1161] = if (s.v[697] < 2.0) { 1.0 } else { 0.0 };

        if (s.v[1161] != 0.0) {
            s.store_scalar(697, 2.0);
        }

        s.v[1162] = if (s.v[704] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1162] != 0.0) {
            s.store_scalar(704, 0.03);
        }

        s.v[1163] = if (s.v[807] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1163] != 0.0) {
            s.store_scalar(807, 0.0);
        }

        s.v[1164] = if (s.v[811] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1164] != 0.0) {
            s.store_scalar(811, 0.0);
        }

        s.v[1165] = if (s.v[812] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1165] != 0.0) {
            s.store_scalar(812, 0.0);
        }

        s.v[1166] = if (s.v[814] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1166] != 0.0) {
            s.store_scalar(814, 0.0);
        }

        s.v[1167] = if (s.v[707] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1167] != 0.0) {
            s.store_scalar(707, 0.0);
        }

        s.v[1168] = if (s.v[709] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1168] != 0.0) {
            s.store_scalar(709, 0.0);
        }

        s.v[1169] = if (s.v[853] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1169] != 0.0) {
            s.store_scalar(853, 0.0);
        }

        s.v[1170] = if (s.v[852] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1170] != 0.0) {
            s.store_scalar(852, 0.0);
        }

        s.v[1171] = if (s.v[712] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1171] != 0.0) {
            s.store_scalar(712, 0.0);
        }

        s.v[1172] = if (s.v[711] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1172] != 0.0) {
            s.store_scalar(711, 0.0);
        }

        s.v[1175] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1178] = if (s.v[706] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1175] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(706, 0.0);
        }

        s.v[1179] = if (s.v[815] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1175] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(815, 0.0);
        }

        s.v[1180] = if (s.v[816] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1175] != 0.0) && (s.v[1180] != 0.0)) {
            s.store_scalar(816, 0.0);
        }

        s.v[1181] = if (s.v[818] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1175] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_scalar(818, 0.0);
        }

        s.v[1183] = if (s.v[719] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1183] != 0.0) {
            s.store_scalar(719, 1.06);
        }

        s.v[1184] = if (s.v[790] < 2.0) { 1.0 } else { 0.0 };

        if (s.v[1184] != 0.0) {
            s.store_scalar(790, 2.0);
        }

        s.v[1185] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1186] = if (s.v[791] < 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1185] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_scalar(791, 2.0);
        }

        s.v[1187] = if (s.v[700] < 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[1187] != 0.0) {
            s.store_scalar(700, 0.0);
        }

        s.v[1188] = if (s.v[749] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1188] != 0.0) {
            s.store_scalar(749, 0.0);
        }

        s.v[1189] = if (s.v[763] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1189] != 0.0) {
            s.store_scalar(763, 0.0);
        }

        s.v[1190] = if (p.p69 != 0.0) { 1.0 } else { 0.0 };

        s.v[1191] = if (s.v[726] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1190] != 0.0) && (s.v[1191] != 0.0)) {
            s.store_scalar(726, 3.0);
        }

        s.v[1192] = if (s.v[731] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1190] != 0.0) && (s.v[1192] != 0.0)) {
            s.store_scalar(731, 1.0);
        }

        s.v[1193] = if (p.p68 != 0.0) { 1.0 } else { 0.0 };

        s.v[1194] = if (s.v[742] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1193] != 0.0) && (s.v[1194] != 0.0)) {
            s.store_scalar(742, 1.0);
        }

        s.v[1195] = if (s.v[736] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1193] != 0.0) && (s.v[1195] != 0.0)) {
            s.store_scalar(736, 1.0);
        }

        s.v[1198] = if (p.p1649 >= (s.v[153] / 2.0)) { 1.0 } else { 0.0 };

        if (s.v[1198] != 0.0) {
            s.store_scalar(875, 0.0);
        }

        if (!(s.v[1198] != 0.0)) {
            s.store_scalar(875, p.p1649);
        }

        s.v[1199] = if (s.v[864] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1199] != 0.0) {
            s.store_scalar(864, 1.0);
        }

        s.v[1200] = if ((p.p73 == 1.0) && (s.v[873] != 0.0)) { 1.0 } else { 0.0 };

        s.v[1201] = if (s.v[873] < 0.001) { 1.0 } else { 0.0 };

        if ((s.v[1200] != 0.0) && (s.v[1201] != 0.0)) {
            s.store_scalar(873, 0.0);
        }

        s.v[1211] = if (p.p1680 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1211] != 0.0) {
            s.store_scalar(577, 1.0);
        }

        s.v[1212] = if (p.p1680 > 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1211] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_scalar(577, 1.0);
        }

        s.v[1213] = if (s.v[648] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1213] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        s.v[1214] = if (s.v[649] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1214] != 0.0) {
            s.store_scalar(649, 0.0);
        }

        s.v[1215] = if (s.v[643] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1215] != 0.0) {
            s.store_scalar(643, 0.0);
        }

        s.v[1216] = if (s.v[642] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1216] != 0.0) {
            s.store_scalar(642, 0.0);
        }

        s.v[1217] = if (s.v[650] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1217] != 0.0) {
            s.store_scalar(650, 0.0);
        }

        s.v[1218] = if (s.v[651] <= 0.02) { 1.0 } else { 0.0 };

        if (s.v[1218] != 0.0) {
            s.store_scalar(651, 0.02);
        }

        s.v[1219] = if (s.v[652] <= 0.02) { 1.0 } else { 0.0 };

        if (s.v[1219] != 0.0) {
            s.store_scalar(652, 0.02);
        }

        s.v[1220] = if (s.v[653] <= 0.02) { 1.0 } else { 0.0 };

        if (s.v[1220] != 0.0) {
            s.store_scalar(653, 0.02);
        }

        s.v[1221] = if (s.v[446] < (-p.p4)) { 1.0 } else { 0.0 };

        if (s.v[1221] != 0.0) {
            s.store_scalar(446, 0.0);
        }

        s.v[1222] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        s.v[1223] = if ((s.v[882] < 1.0) || (s.v[882] > 3.0)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1223] != 0.0)) {
            s.store_scalar(882, 2.0);
        }

        s.v[1224] = if ((s.v[883] < 1.0) || (s.v[883] > 3.0)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1224] != 0.0)) {
            s.store_scalar(883, 2.6);
        }

        s.v[1225] = if ((s.v[884] < 1.0) || (s.v[884] > 3.0)) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1225] != 0.0)) {
            s.store_scalar(884, 2.6);
        }

        s.v[1226] = if (s.v[885] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1226] != 0.0)) {
            s.store_scalar(885, 14.0);
        }

        s.v[1227] = if (s.v[886] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1227] != 0.0)) {
            s.store_scalar(886, 24.0);
        }

        s.v[1228] = if (s.v[887] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1228] != 0.0)) {
            s.store_scalar(887, 24.0);
        }

        s.v[1229] = if (s.v[888] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1229] != 0.0)) {
            s.store_scalar(888, 0.139);
        }

        s.v[1230] = if (s.v[889] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1230] != 0.0)) {
            s.store_scalar(889, 2.0);
        }

        s.v[1231] = if (s.v[890] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1231] != 0.0)) {
            s.store_scalar(890, 11.2);
        }

        s.v[1232] = if (s.v[891] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1232] != 0.0)) {
            s.store_scalar(891, 8.02);
        }

        s.v[1233] = if (s.v[892] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1233] != 0.0)) {
            s.store_scalar(892, 6.18);
        }

        s.v[1234] = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1235] = if (p.p1795 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1234] != 0.0) && (s.v[1235] != 0.0)) {
            s.store_scalar(169, (p.p1793 * ((p.p59) as f64).powf(p.p1795)));
        }

        if ((s.v[1234] != 0.0) && (!(s.v[1235] != 0.0))) {
            s.store_scalar(169, p.p1793);
        }

        s.v[1236] = if (p.p1794 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1234] != 0.0) && (s.v[1236] != 0.0)) {
            s.store_scalar(170, ((p.p1797 * p.p4) * ((s.v[115]) as f64).powf(p.p1794)));
        }

        if ((s.v[1234] != 0.0) && (!(s.v[1236] != 0.0))) {
            s.store_scalar(170, (p.p1797 * p.p4));
        }

        s.v[1237] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1238] = if (p.p1796 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1234] != 0.0) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) {
            s.store_scalar(171, (((p.p1798 * p.p59) * p.p43) * ((p.p56) as f64).powf(p.p1796)));
        }

        if (((s.v[1234] != 0.0) && (s.v[1237] != 0.0)) && (!(s.v[1238] != 0.0))) {
            s.store_scalar(171, ((p.p1798 * p.p59) * p.p43));
        }

        if ((s.v[1234] != 0.0) && (!(s.v[1237] != 0.0))) {
            s.store_scalar(171, 0.0);
        }

        if (s.v[1234] != 0.0) {
            s.store_scale_ad(633, A::add(A::add(s.ad_value(169), s.ad_value(170)), s.ad_value(171)), 1.0 / (p.p1791));
        }

        if (s.v[1234] != 0.0) {
            s.store_scale_ad(634, A::add(A::add(s.ad_value(169), s.ad_value(170)), s.ad_value(171)), p.p1792);
        }

        s.v[1239] = if (p.p76 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1239] != 0.0) {
            s.store_scalar(457, (((p.p1074 / p.p6) + ((p.p1075 * p.p5) / (if (p.p6 == 2.0) { 12.0 } else { 3.0 }))) / p.p59));
        }

        if (s.v[1239] != 0.0) {
            s.store_div_from_scalar_ad(456, 1.0, A::max_from_scalar(0.001, s.ad_value(457)));
        }

        s.v[1240] = if (p.p76 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1239] != 0.0) && (s.v[1240] != 0.0)) {
            s.store_scalar(458, (1.0 / (0.001_f64).max(p.p1076)));
        }

        if ((s.v[1239] != 0.0) && (s.v[1240] != 0.0)) {
            s.store_scalar(459, (1.0 / (0.001_f64).max(p.p1077)));
        }

        s.v[1241] = if (p.p77 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1241] != 0.0) {
            s.store_scalar(190, (p.p1078 * p.p18));
        }

        if (s.v[1241] != 0.0) {
            s.store_scalar(191, (p.p1079 * p.p19));
        }

        s.v[1242] = if (p.p1080 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1241] != 0.0)) && (s.v[1242] != 0.0)) {
            s.store_scalar(444, ((p.p4 * p.p92) + ((p.p3 + ((p.p4 - p.p3) * p.p1084)) * p.p1080)));
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1242] != 0.0))) {
            s.store_scalar(444, (p.p4 * (1e-9_f64).max((p.p92 + p.p1080))));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_offset(445, 446, p.p4);
        }

        s.v[1243] = if self.param_given[1083] { 1.0 } else { 0.0 };

        if ((!(s.v[1241] != 0.0)) && (s.v[1243] != 0.0)) {
            s.store_scalar(431, p.p1083);
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) {
            s.store_scalar(429, (if (p.p60 == 1.0) { 1417.0 } else { 470.5 }));
        }

        s.v[1244] = if (p.p60 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scalar(168, (((p.p97 / 9.68e22)) as f64).powf(0.68));
        }

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scalar(169, (3.43e26 / p.p97));
        }

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scale_ad(430, A::sub(A::offset(A::div(A::offset(s.ad_value(429), (-52.2)), A::offset(s.ad_value(168), 1.0)), 52.2), A::div_from_scalar(43.4, A::offset(A::square(s.ad_value(169)), 1.0))), 0.0001);
        }

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scalar(168, (((p.p97 / 2.23e22)) as f64).powf(0.719));
        }

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scalar(169, (6.1e26 / p.p97));
        }

        if (((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale_ad(430, A::sub(A::offset(A::div(A::offset(s.ad_value(429), (-44.9)), A::offset(s.ad_value(168), 1.0)), 44.9), A::div_from_scalar(29.0, A::offset(A::square(s.ad_value(169)), 1.0))), 0.0001);
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) {
            s.store_div_from_scalar_ad(431, 1.0, A::scale(s.ad_value(430), (1.60219e-19 * p.p97)));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_ad(432, &A::min_with_scalar(s.ad_value(444), (1e-18_f64).max((p.p3 * (p.p92 + (0.0_f64).min(p.p1080))))));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_mul_ad(434, A::scale(A::div(s.ad_value(431), A::tan(s.ad_value(433))), 1.0 / ((((3.141592653589793) as f64).sqrt() * p.p5))), A::add(A::sub(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), A::div_from_scalar(2.0, A::sqrt(s.ad_value(444)))), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444))))));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_offset_scaled(436, 444, p.p5, p.p1092);
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_offset_scaled(437, 445, p.p5, p.p1093);
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_sqrt_ad(435, A::div(A::scale(s.ad_value(436), p.p1082), A::mul(s.ad_value(431), s.ad_value(437))));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_div_from_scalar(438, p.p20, 435);
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_limited_exp_ad(168, A::scale(s.ad_value(438), 2.0));
        }

        s.v[1245] = if (p.p1086 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1241] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_scaled_mul(439, 431, 435, 1.0 / (p.p1082));
        }

        if ((!(s.v[1241] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_mul_ad_rhs(169, 168, A::offset(s.ad_value(439), 1.0));
        }

        if ((!(s.v[1241] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_sub_ad_lhs(170, A::offset(s.ad_value(169), 1.0), 439);
        }

        if ((!(s.v[1241] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_add_ad_lhs(171, A::offset(s.ad_value(169), (-1.0)), 439);
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_offset(170, 168, 1.0);
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_offset(171, 168, (-1.0));
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_div_ad(440, A::mul(A::mul(s.ad_value(431), s.ad_value(435)), s.ad_value(170)), A::mul(s.ad_value(436), s.ad_value(171)));
        }

        s.v[1246] = if (p.p1080 < (-1e-10)) { 1.0 } else { 0.0 };

        if ((!(s.v[1241] != 0.0)) && (s.v[1246] != 0.0)) {
            s.store_scalar(441, (p.p1082 / (((-p.p1080) * p.p3) * p.p5)));
        }

        if ((!(s.v[1241] != 0.0)) && (s.v[1246] != 0.0)) {
            s.store_div_ad(442, A::mul(A::add(s.ad_value(440), s.ad_value(434)), s.ad_value(441)), A::add(A::add(s.ad_value(440), s.ad_value(434)), s.ad_value(441)));
        }

        if ((!(s.v[1241] != 0.0)) && (!(s.v[1246] != 0.0))) {
            s.store_add(442, 440, 434);
        }

        if (!(s.v[1241] != 0.0)) {
            s.store_scale_ad(443, A::scale(s.ad_value(442), 1.0 / (p.p59)), (0.0_f64).max(((((p.p1094 + (p.p1095 * p.p3)) + (p.p1096 * p.p4)) + (p.p1097 * p.p20)) + (p.p1098 * p.p1080))));
        }

        if (!(s.v[1241] != 0.0)) {
            s.copy_ad(190, 443);
        }

        if (!(s.v[1241] != 0.0)) {
            s.copy_ad(191, 443);
        }

        s.v[1247] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        s.v[1248] = if (s.v[190] < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[1247] != 0.0) && (s.v[1248] != 0.0)) {
            s.store_scalar(190, 0.0);
        }

        s.v[1249] = if (s.v[191] < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[1247] != 0.0) && (s.v[1249] != 0.0)) {
            s.store_scalar(191, 0.0);
        }

        s.v[1250] = if (s.v[190] <= p.p151) { 1.0 } else { 0.0 };

        if ((!(s.v[1247] != 0.0)) && (s.v[1250] != 0.0)) {
            s.store_scalar(190, p.p151);
        }

        s.v[1251] = if (s.v[191] <= p.p151) { 1.0 } else { 0.0 };

        if ((!(s.v[1247] != 0.0)) && (s.v[1251] != 0.0)) {
            s.store_scalar(191, p.p151);
        }

        s.v[1252] = if (p.p78 != 1.0) { 1.0 } else { 0.0 };

        s.v[1253] = if self.param_given[1542] { 1.0 } else { 0.0 };

        if ((s.v[1252] != 0.0) && (s.v[1253] != 0.0)) {
            s.store_scalar(646, p.p1542);
        }

        s.v[1254] = if ((if self.param_given[85] { 1.0 } else { 0.0 } != 0.0) && (p.p85 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1252] != 0.0) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) {
            s.store_max_from_scalar_ad(646, 0.0, A::sub(A::scale(s.ad_value(163), p.p85), s.ad_value(648)));
        }

        s.v[1255] = if (p.p78 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[1252] != 0.0) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) && (s.v[1255] != 0.0)) {
            s.store_scale(646, 163, (0.3 * p.p43));
        }

        if ((((s.v[1252] != 0.0) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) && (!(s.v[1255] != 0.0))) {
            s.store_scale(646, 163, (0.3 * p.p3));
        }

        s.v[1256] = if self.param_given[1543] { 1.0 } else { 0.0 };

        if ((s.v[1252] != 0.0) && (s.v[1256] != 0.0)) {
            s.store_scalar(647, p.p1543);
        }

        s.v[1257] = if ((if self.param_given[85] { 1.0 } else { 0.0 } != 0.0) && (p.p85 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1252] != 0.0) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_max_from_scalar_ad(647, 0.0, A::sub(A::scale(s.ad_value(163), p.p85), s.ad_value(649)));
        }

        s.v[1258] = if (p.p78 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[1252] != 0.0) && (!(s.v[1256] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_scale(647, 163, (0.3 * p.p43));
        }

        if ((((s.v[1252] != 0.0) && (!(s.v[1256] != 0.0))) && (!(s.v[1257] != 0.0))) && (!(s.v[1258] != 0.0))) {
            s.store_scale(647, 163, (0.3 * p.p3));
        }

        s.v[1259] = if (p.p78 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1259] != 0.0) {
            s.store_scalar(447, (p.p1089 + p.p1090));
        }

        if (s.v[1259] != 0.0) {
            s.store_scalar(449, (0.5 * (p.p4 - p.p3)));
        }

        if (s.v[1259] != 0.0) {
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
        }

        if (s.v[1259] != 0.0) {
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
        }

        s.v[1260] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

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
        if ((s.v[1259] != 0.0) && (s.v[1260] != 0.0)) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if ((s.v[1259] != 0.0) && (s.v[1260] != 0.0)) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
        }

        if ((s.v[1259] != 0.0) && (s.v[1260] != 0.0)) {
            s.store_scaled_add(451, 168, 169, (p.p3 + ((p.p4 - p.p3) * p.p1084)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(447), p.p90), 0.2), s.ad_value(450)), 2.3);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_ad(929, &A::min(s.ad_value(450), A::offset(s.ad_value(447), p.p90)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_mul(933, 931, 932);
        }

        s.v[1261] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) && (s.v[1261] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if (((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            let assign10160_ad_e12710: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign10160_ad_e12710);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(450), A::offset(s.ad_value(447), p.p90)), A::div(A::offset(s.ad_value(447), p.p90), s.ad_value(450))), 0.5);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            let assign10190_ad_e12788: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10190_ad_e12788, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scaled_add(938, 934, 937, p.p3);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_div(930, 928, 447);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(447), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(447)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(447), s.ad_value(930))), 447);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            let assign10280_ad_e12979: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign10280_ad_e12979, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            let assign10300_ad_e13041: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign10300_ad_e13041);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p3);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1260] != 0.0))) {
            s.store_add(451, 938, 947);
        }

        s.v[1262] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_mul(933, 931, 932);
        }

        s.v[1263] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) && (s.v[1263] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if (((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) && (!(s.v[1263] != 0.0))) {
            let assign10470_ad_e13236: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign10470_ad_e13236);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            let assign10500_ad_e13311: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10500_ad_e13311, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scaled_add(938, 934, 937, p.p92);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_div(930, 928, 448);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            let assign10590_ad_e13493: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign10590_ad_e13493, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            let assign10610_ad_e13553: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign10610_ad_e13553);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p92);
        }

        if ((s.v[1259] != 0.0) && (s.v[1262] != 0.0)) {
            s.store_add(452, 938, 947);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_mul(933, 931, 932);
        }

        s.v[1264] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) && (s.v[1264] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if (((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) && (!(s.v[1264] != 0.0))) {
            let assign10770_ad_e13752: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign10770_ad_e13752);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            let assign10800_ad_e13830: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10800_ad_e13830, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scaled_add(938, 934, 937, p.p92);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_div(930, 928, 448);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            let assign10890_ad_e14021: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign10890_ad_e14021, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
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
        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            let assign10910_ad_e14083: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign10910_ad_e14083);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p92);
        }

        if ((s.v[1259] != 0.0) && (!(s.v[1262] != 0.0))) {
            s.store_add(452, 938, 947);
        }

        s.v[1265] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1259] != 0.0) && (s.v[1265] != 0.0)) {
            s.store_scalar(454, 0.0);
        }

        s.v[1266] = if (p.p1080 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1259] != 0.0) && (!(s.v[1265] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_scalar(454, ((p.p4 - p.p3) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if (((s.v[1259] != 0.0) && (!(s.v[1265] != 0.0))) && (!(s.v[1266] != 0.0))) {
            s.store_scale(454, 450, (p.p4 - p.p3));
        }

        if (s.v[1259] != 0.0) {
            s.store_scale_ad(455, A::offset(A::offset(A::scale(s.ad_value(454), p.p5), p.p1092), p.p1091), (s.v[144] * 1.0 / (p.p1087)));
        }

        if (s.v[1259] != 0.0) {
            s.store_scale_ad(453, A::add(A::add(s.ad_value(455), A::scale(s.ad_value(451), p.p5)), A::scale(s.ad_value(452), (p.p1103 * (p.p5 * 2.0)))), p.p59);
        }

        if (s.v[1259] != 0.0) {
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p3)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.v[1267] = if (p.p78 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1267] != 0.0) {
            s.store_scalar(447, (p.p1089 + p.p1090));
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(449, (0.5 * (p.p4 - p.p43)));
        }

        if (s.v[1267] != 0.0) {
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(1031, (0.5 * p.p41));
        }

        s.v[1268] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1268] != 0.0)) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if ((s.v[1267] != 0.0) && (s.v[1268] != 0.0)) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
        }

        if ((s.v[1267] != 0.0) && (s.v[1268] != 0.0)) {
            s.store_scaled_add(1034, 168, 169, (p.p43 + ((p.p4 - p.p43) * p.p1084)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(447), p.p90), 0.2), s.ad_value(450)), 2.3);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_ad(929, &A::min(s.ad_value(450), A::offset(s.ad_value(447), p.p90)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_mul(933, 931, 932);
        }

        s.v[1269] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if (((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) && (!(s.v[1269] != 0.0))) {
            let assign11250_ad_e14490: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign11250_ad_e14490);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(450), A::offset(s.ad_value(447), p.p90)), A::div(A::offset(s.ad_value(447), p.p90), s.ad_value(450))), 0.5);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            let assign11280_ad_e14568: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11280_ad_e14568, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scaled_add(938, 934, 937, p.p43);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_div(930, 928, 447);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(447), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(447)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(447), s.ad_value(930))), 447);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            let assign11370_ad_e14759: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign11370_ad_e14759, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            let assign11390_ad_e14821: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign11390_ad_e14821);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p43);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1268] != 0.0))) {
            s.store_add(1034, 938, 947);
        }

        if (s.v[1267] != 0.0) {
            s.store_offset_ad(925, A::div_from_scalar((0.2 * (p.p1089 + p.p90)), s.ad_value(1031)), 2.3);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(926, 1.05);
        }

        if (s.v[1267] != 0.0) {
            s.store_abs_ad(927, A::sub_from_scalar((p.p1089 + p.p90), s.ad_value(1031)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(928, 926, p.p1087);
        }

        if (s.v[1267] != 0.0) {
            s.store_ad(929, &A::min_with_scalar(s.ad_value(1031), (p.p1089 + p.p90)));
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(931, 1700000000000.0);
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(933, 931, 932);
        }

        s.v[1270] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1270] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1270] != 0.0))) {
            let assign11550_ad_e14991: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign11550_ad_e14991);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p.p1089 + p.p90))), A::div_from_scalar((p.p1089 + p.p90), s.ad_value(1031))), 0.5);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(936, 927, 935);
        }

        if (s.v[1267] != 0.0) {
            let assign11580_ad_e15060: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11580_ad_e15060, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_add(938, 934, 937, p.p43);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(930, 928, 1.0 / (p.p1089));
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_offset_ad(940, A::add(A::offset(A::mul(A::sqrt(A::offset(A::scale(A::offset(s.ad_value(930), 1.0), (p.p1089 * p.p1089)), ((p.p90 * p.p90) + ((2.0 * p.p1089) * p.p90)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::scale(s.ad_value(930), p.p1089)), p.p1089);
        }

        if (s.v[1267] != 0.0) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(943, 925, 926);
        }

        if (s.v[1267] != 0.0) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if (s.v[1267] != 0.0) {
            let assign11670_ad_e15224: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign11670_ad_e15224, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if (s.v[1267] != 0.0) {
            let assign11690_ad_e15280: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign11690_ad_e15280);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(627, 1.2e-12);
        }

        if (s.v[1267] != 0.0) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p43);
        }

        if (s.v[1267] != 0.0) {
            s.store_add(1035, 938, 947);
        }

        s.v[1271] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_mul(933, 931, 932);
        }

        s.v[1272] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) {
            s.copy_ad(934, 932);
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
        if (((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) {
            let assign11860_ad_e15463: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign11860_ad_e15463);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            let assign11890_ad_e15538: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11890_ad_e15538, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scaled_add(938, 934, 937, p.p40);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_div(930, 928, 448);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            let assign11980_ad_e15720: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign11980_ad_e15720, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            let assign12000_ad_e15780: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign12000_ad_e15780);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p40);
        }

        if ((s.v[1267] != 0.0) && (s.v[1271] != 0.0)) {
            s.store_add(1036, 938, 947);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scalar(926, 1.05);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scale(928, 926, p.p1087);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scalar(931, 1700000000000.0);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_mul(933, 931, 932);
        }

        s.v[1273] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) && (s.v[1273] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if (((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) && (!(s.v[1273] != 0.0))) {
            let assign12160_ad_e15979: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign12160_ad_e15979);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_mul(936, 927, 935);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            let assign12190_ad_e16057: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12190_ad_e16057, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scaled_add(938, 934, 937, p.p40);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_div(930, 928, 448);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_mul(943, 925, 926);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            let assign12280_ad_e16248: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign12280_ad_e16248, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            let assign12300_ad_e16310: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign12300_ad_e16310);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scalar(627, 1.2e-12);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p40);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1271] != 0.0))) {
            s.store_add(1036, 938, 947);
        }

        if (s.v[1267] != 0.0) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(926, 1.05);
        }

        if (s.v[1267] != 0.0) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(928, 926, p.p1087);
        }

        if (s.v[1267] != 0.0) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(931, 1700000000000.0);
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(933, 931, 932);
        }

        s.v[1274] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1274] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1274] != 0.0))) {
            let assign12460_ad_e16480: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign12460_ad_e16480);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(936, 927, 935);
        }

        if (s.v[1267] != 0.0) {
            let assign12490_ad_e16549: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12490_ad_e16549, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_add(938, 934, 937, p.p40);
        }

        if (s.v[1267] != 0.0) {
            s.store_div(930, 928, 448);
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if (s.v[1267] != 0.0) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(943, 925, 926);
        }

        if (s.v[1267] != 0.0) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if (s.v[1267] != 0.0) {
            let assign12580_ad_e16713: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign12580_ad_e16713, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if (s.v[1267] != 0.0) {
            let assign12600_ad_e16769: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign12600_ad_e16769);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(627, 1.2e-12);
        }

        if (s.v[1267] != 0.0) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p40);
        }

        if (s.v[1267] != 0.0) {
            s.store_add(1037, 938, 947);
        }

        if (s.v[1267] != 0.0) {
            s.store_offset_ad(925, A::div(A::scale(A::offset(s.ad_value(448), p.p90), 0.2), s.ad_value(449)), 2.3);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(926, 1.05);
        }

        if (s.v[1267] != 0.0) {
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(928, 926, p.p1087);
        }

        if (s.v[1267] != 0.0) {
            s.store_ad(929, &A::min(s.ad_value(449), A::offset(s.ad_value(448), p.p90)));
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(930, p.p1087, A::offset(s.ad_value(925), 1.0));
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
        if (s.v[1267] != 0.0) {
            s.store_scalar(931, 1700000000000.0);
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(933, 931, 932);
        }

        s.v[1275] = if (s.v[933] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1275] != 0.0)) {
            s.copy_ad(934, 932);
        }

        if ((s.v[1267] != 0.0) && (!(s.v[1275] != 0.0))) {
            let assign12760_ad_e16927: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(933)), 1.0))
                } else {
                    {
                        if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                            A::exp(s.ad_value(933))
                        } else {
                            {
                                if (s.v[933] > 37.0) {
                                    s.ad_value(933)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(934, A::div_from_scalar(1.0, s.ad_value(931)), assign12760_ad_e16927);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(936, 927, 935);
        }

        if (s.v[1267] != 0.0) {
            let assign12790_ad_e16996: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln(A::scale(A::offset(A::scale(s.ad_value(936), (0.5 * 3.141592653589793)), p.p1087), 1.0 / (p.p1087)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12790_ad_e16996, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_scaled_add(938, 934, 937, p.p42);
        }

        if (s.v[1267] != 0.0) {
            s.store_div(930, 928, 448);
        }

        if (s.v[1267] != 0.0) {
            s.store_div_from_scalar_ad(939, 4.0, A::scale(A::sqrt(A::scale(A::offset(s.ad_value(930), 1.0), 2.0)), 3.141592653589793));
        }

        if (s.v[1267] != 0.0) {
            s.store_add_ad_lhs(940, A::add(A::offset(A::mul(A::sqrt(A::add(A::offset(A::scale(s.ad_value(448), (2.0 * p.p90)), (p.p90 * p.p90)), A::mul(A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0)))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), A::mul(s.ad_value(448), s.ad_value(930))), 448);
        }

        if (s.v[1267] != 0.0) {
            s.store_add_ad(941, A::scale(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90), A::scale(A::offset(s.ad_value(930), 2.0), p.p90));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(942, A::offset(A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27), s.v[144]);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul(943, 925, 926);
        }

        if (s.v[1267] != 0.0) {
            s.store_sqrt_ad(944, A::offset(A::square(s.ad_value(943)), 1.0));
        }

        if (s.v[1267] != 0.0) {
            let assign12880_ad_e17160: A = A::add(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add(A::mul(A::scale(s.ad_value(943), p.p90), A::scale(s.ad_value(943), p.p90)), A::scale(A::mul(A::scale(s.ad_value(943), 2.0), s.ad_value(928)), p.p90)), A::mul(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928)), s.ad_value(928))))), A::scale(s.ad_value(943), p.p90));
            s.store_add_ad_lhs(933, A::add(assign12880_ad_e17160, A::mul(A::square(s.ad_value(943)), s.ad_value(928))), 928);
        }

        if (s.v[1267] != 0.0) {
            s.store_mul_ad(945, A::offset(s.ad_value(944), 1.0), A::scale(s.ad_value(943), p.p90));
        }

        if (s.v[1267] != 0.0) {
            let assign12900_ad_e17216: A = A::mul(A::div(A::scale(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85)), s.ad_value(944)), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(946, &assign12900_ad_e17216);
        }

        if (s.v[1267] != 0.0) {
            s.store_scalar(627, 1.2e-12);
        }

        if (s.v[1267] != 0.0) {
            s.store_sub_ad_lhs(933, A::sub(s.ad_value(946), s.ad_value(942)), 627);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(947, A::sub(s.ad_value(946), A::scale(A::add(s.ad_value(933), A::sqrt(A::add(A::square(s.ad_value(933)), A::mul(A::scale(s.ad_value(627), 4.0), s.ad_value(946))))), 0.5)), p.p42);
        }

        if (s.v[1267] != 0.0) {
            s.store_add(1038, 938, 947);
        }

        s.v[1276] = if (p.p1090 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1267] != 0.0) && (s.v[1276] != 0.0)) {
            s.store_scalar(1032, 0.0);
        }

        s.v[1277] = if (p.p1080 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1267] != 0.0) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
            s.store_scalar(1032, ((p.p4 - p.p43) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if (((s.v[1267] != 0.0) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_scale(1032, 450, (p.p4 - p.p43));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(1033, 1031, (p.p4 - p.p43));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(455, A::offset(A::offset(A::scale(A::add(s.ad_value(1032), A::scale(s.ad_value(1033), (2.0 * p.p56))), p.p5), p.p1092), p.p1091), (s.v[144] * 1.0 / (p.p1087)));
        }

        if (s.v[1267] != 0.0) {
            s.store_scale_ad(453, A::add(A::add(s.ad_value(455), A::scale(A::add(s.ad_value(1034), A::scale(s.ad_value(1035), (2.0 * p.p56))), p.p5)), A::scale(A::add(A::add(s.ad_value(1036), A::scale(s.ad_value(1037), (p.p56 - 1.0))), s.ad_value(1038)), (p.p1103 * (p.p5 * 2.0)))), p.p59);
        }

        if (s.v[1267] != 0.0) {
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p43)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.v[168] = (p.p1583 * (if (!((1.0 + (p.p92 / p.p91)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p92 / p.p91)) > 1e-38) { (((1.0 + (p.p92 / p.p91))) as f64).ln() } else { 0.0 }) }));

        s.v[515] = ((s.v[165] * p.p7) + (s.v[168] * (0.0_f64).max((p.p9 - (p.p4 * s.v[115])))));

        s.v[516] = ((s.v[165] * p.p8) + (s.v[168] * (0.0_f64).max((p.p10 - (p.p4 * s.v[115])))));

        s.v[1278] = if (p.p62 != 5.0) { 1.0 } else { 0.0 };

        if (s.v[1278] != 0.0) {
            s.store_scale(517, 149, (((p.p1544 * p.p59) * p.p6) + (p.p1545 * s.v[115])));
        }

        if (!(s.v[1278] != 0.0)) {
            s.store_mul_ad_lhs(517, A::offset(A::scale(A::offset(A::scale(s.ad_value(161), p.p1546), p.p1545), s.v[115]), ((p.p1544 * p.p59) * p.p6)), 149);
        }

        s.v[420] = (1e-8 / (s.v[145] * p.p89));

        s.store_div_from_scalar_ad(189, 1.0, A::scale(A::pow(A::scale(s.ad_value(158), 1000000.0), s.ad_value(713)), s.v[115]));

        s.v[578] = (((((s.v[145] * p.p89) * 0.5) * p.p3)) as f64).sqrt();

        s.store_sqrt_ad(351, A::mul(A::div(A::scale(s.ad_value(894), s.v[143]), s.ad_value(893)), A::offset(A::div(A::mul(s.ad_value(894), s.ad_value(893)), A::mul(A::scale(s.ad_value(895), (2.0 * s.v[143])), s.ad_value(895))), 1.0)));

        s.v[1279] = if !(if self.param_given[172] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1279] != 0.0) {
            s.store_offset_ad(360, A::div(A::mul(s.ad_value(670), s.ad_value(153)), s.ad_value(351)), 1e-6);
        }

        s.v[1280] = if (s.v[360] < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[1279] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_div_from_scalar_ad(361, 0.5, A::offset(A::cosh(s.ad_value(360)), (-1.0)));
        }

        if ((s.v[1279] != 0.0) && (!(s.v[1280] != 0.0))) {
            s.store_limited_exp_ad(361, A::neg(s.ad_value(360)));
        }

        if (!(s.v[1279] != 0.0)) {
            s.store_scalar(361, p.p172);
        }

        s.v[1281] = if !(if self.param_given[174] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1281] != 0.0) {
            s.store_offset_ad(360, A::div(A::mul(s.ad_value(671), s.ad_value(153)), s.ad_value(351)), 1e-6);
        }

        s.v[1282] = if (s.v[360] < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[1281] != 0.0) && (s.v[1282] != 0.0)) {
            s.store_div_from_scalar_ad(362, 0.5, A::offset(A::cosh(s.ad_value(360)), (-1.0)));
        }

        if ((s.v[1281] != 0.0) && (!(s.v[1282] != 0.0))) {
            s.store_limited_exp_ad(362, A::neg(s.ad_value(360)));
        }

        if (!(s.v[1281] != 0.0)) {
            s.store_scalar(362, p.p174);
        }

        s.v[1283] = if !(if self.param_given[173] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1283] != 0.0) {
            s.store_offset_ad(360, A::div(A::mul(s.ad_value(678), s.ad_value(153)), s.ad_value(351)), 1e-6);
        }

        s.v[1284] = if (s.v[360] < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[1283] != 0.0) && (s.v[1284] != 0.0)) {
            s.store_div_from_scalar_ad(363, 0.5, A::offset(A::cosh(s.ad_value(360)), (-1.0)));
        }

        if ((s.v[1283] != 0.0) && (!(s.v[1284] != 0.0))) {
            s.store_limited_exp_ad(363, A::neg(s.ad_value(360)));
        }

        if (!(s.v[1283] != 0.0)) {
            s.store_scalar(363, p.p173);
        }

        s.store_offset_ad(364, A::sqrt(A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0)), (-1.0));

        s.store_offset_ad(360, A::div(A::mul(s.ad_value(678), s.ad_value(153)), s.ad_value(351)), 1e-6);

        s.v[1285] = if (s.v[360] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[1285] != 0.0) {
            s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::offset(A::scale(A::offset(A::cosh(s.ad_value(360)), (-2.0)), p.p171), 1.0), 1e-6));
        }

        if (!(s.v[1285] != 0.0)) {
            s.store_div_ad(365, A::limited_exp(A::neg(s.ad_value(360))), A::max_with_scalar(A::offset(A::limited_exp(A::neg(s.ad_value(360))), p.p171), 1e-6));
        }

        s.store_div_ad_lhs(396, A::mul(A::scale(s.ad_value(640), 1.60219e-19), s.ad_value(894)), 893);

        s.v[1286] = if (p.p60 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1286] != 0.0) {
            s.store_scalar(484, 4.97232e-7);
        }

        if (s.v[1286] != 0.0) {
            s.store_scalar(485, 745669000000.0);
        }

        if (!(s.v[1286] != 0.0)) {
            s.store_scalar(484, 3.42537e-7);
        }

        if (!(s.v[1286] != 0.0)) {
            s.store_scalar(485, 1166450000000.0);
        }

        s.v[168] = (p.p1109 * p.p1109);

        s.store_scale(169, 742, p.p1109);

        s.store_square(170, 169);

        s.store_scale_ad(486, A::pow_from_scalar((p.p1108 / p.p1109), s.ad_value(741)), 1.0 / (s.v[168]));

        s.store_div_ad_lhs(487, A::pow(A::div_from_scalar(p.p1108, s.ad_value(169)), s.ad_value(741)), 170);

        s.store_mul_ad_lhs(463, A::mul(s.ad_value(158), s.ad_value(484)), 487);

        s.v[1287] = if (p.p1717 < (-273.15)) { 1.0 } else { 0.0 };

        if (s.v[1287] != 0.0) {
            s.store_scalar(228, 300.15);
        }

        if (!(s.v[1287] != 0.0)) {
            s.store_scalar(228, (p.p1717 + 273.15));
        }

        s.v[1288] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1288] != 0.0) {
            let assign13530_ad_e17774: A = A::scale(A::sub_from_scalar(p.p1806, s.ad_value(882)), 1.0 / ((1.0 + if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) - 80.0) } else if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) < -80.0 { 1.804851387e-35 } else { (((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) as f64).exp() })));
            s.store_add_ad_lhs(960, assign13530_ad_e17774, 882);
        }

        if (s.v[1288] != 0.0) {
            let assign13540_ad_e17795: A = A::scale(A::sub_from_scalar(p.p1813, s.ad_value(883)), 1.0 / ((1.0 + if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) - 80.0) } else if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) < -80.0 { 1.804851387e-35 } else { (((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) as f64).exp() })));
            s.store_add_ad_lhs(961, assign13540_ad_e17795, 883);
        }

        if (s.v[1288] != 0.0) {
            let assign13550_ad_e17816: A = A::scale(A::sub_from_scalar(p.p1820, s.ad_value(884)), 1.0 / ((1.0 + if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) - 80.0) } else if (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828) < -80.0 { 1.804851387e-35 } else { (((((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828)) as f64).exp() })));
            s.store_add_ad_lhs(962, assign13550_ad_e17816, 884);
        }

        if (s.v[1288] != 0.0) {
            let assign13560_ad_e17880: A = A::mul(A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))));
            let assign13560_ad_e17888: A = A::add(A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::sqrt(A::offset(assign13560_ad_e17880, ((0.25 * 0.001) * 0.001))));
            s.store_scale_ad(963, assign13560_ad_e17888, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13570_ad_e17951: A = A::mul(A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))));
            let assign13570_ad_e17959: A = A::add(A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::sqrt(A::offset(assign13570_ad_e17951, ((0.25 * 0.001) * 0.001))));
            s.store_scale_ad(964, assign13570_ad_e17959, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13580_ad_e18022: A = A::mul(A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))));
            let assign13580_ad_e18030: A = A::add(A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) - 80.0) } else if (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851) < -80.0 { 1.804851387e-35 } else { (((((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851)) as f64).exp() }))), A::sqrt(A::offset(assign13580_ad_e18022, ((0.25 * 0.001) * 0.001))));
            s.store_scale_ad(965, assign13580_ad_e18030, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13590_ad_e18138: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(966, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13590_ad_e18138, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            let assign13600_ad_e18258: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(969, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(960), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13600_ad_e18258, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            let assign13610_ad_e18378: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(967, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13610_ad_e18378, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            let assign13620_ad_e18498: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(970, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(961), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13620_ad_e18498, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            let assign13630_ad_e18618: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(968, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-1.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13630_ad_e18618, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            let assign13640_ad_e18738: A = A::mul(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)), A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), (-1.0)));
            s.store_offset_ad(971, A::scale(A::sub(A::offset(A::scale(A::add(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001), A::scale(A::offset(s.ad_value(962), (-2.001)), 1.001)), ((0.25 * 0.001) * 0.001)))), 0.5), 1.0), A::sqrt(A::offset(assign13640_ad_e18738, ((0.25 * 0.001) * 0.001)))), 0.5), (0.25 * 0.001));
        }

        if (s.v[1288] != 0.0) {
            s.store_mul_ad(976, A::pow(s.ad_value(158), s.ad_value(966)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)));
        }

        if (s.v[1288] != 0.0) {
            s.store_div(979, 976, 893);
        }

        if (s.v[1288] != 0.0) {
            s.store_mul_ad(977, A::pow(s.ad_value(158), s.ad_value(967)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)));
        }

        if (s.v[1288] != 0.0) {
            s.store_div(980, 977, 893);
        }

        if (s.v[1288] != 0.0) {
            s.store_mul_ad(978, A::pow(s.ad_value(158), s.ad_value(968)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)));
        }

        if (s.v[1288] != 0.0) {
            s.store_div(981, 978, 893);
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(982, (0.5 * (((1.0 / (1.0 + if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) > 80.0 { 5.540622384e34 * (1.0 + (((2.75 - (p.p40 * 1000000000.0)) / 0.78)) - 80.0) } else if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) < -80.0 { 1.804851387e-35 } else { ((((2.75 - (p.p40 * 1000000000.0)) / 0.78)) as f64).exp() })) + 0.5) + ((((((1.0 / (1.0 + if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) > 80.0 { 5.540622384e34 * (1.0 + (((2.75 - (p.p40 * 1000000000.0)) / 0.78)) - 80.0) } else if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) < -80.0 { 1.804851387e-35 } else { ((((2.75 - (p.p40 * 1000000000.0)) / 0.78)) as f64).exp() })) - 0.5) * ((1.0 / (1.0 + if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) > 80.0 { 5.540622384e34 * (1.0 + (((2.75 - (p.p40 * 1000000000.0)) / 0.78)) - 80.0) } else if ((2.75 - (p.p40 * 1000000000.0)) / 0.78) < -80.0 { 1.804851387e-35 } else { ((((2.75 - (p.p40 * 1000000000.0)) / 0.78)) as f64).exp() })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));
        }

        if (s.v[1288] != 0.0) {
            s.store_add_ad_lhs(983, A::div(A::mul(A::sub_from_scalar(1.0, s.ad_value(982)), A::sub(s.ad_value(960), s.ad_value(882))), A::sub_from_scalar(p.p1806, s.ad_value(882))), 982);
        }

        if (s.v[1288] != 0.0) {
            s.store_div_from_scalar_ad(984, 1.0, A::offset(A::limited_exp(A::scale(A::offset(s.ad_value(983), (-0.999)), 10000.0)), 1.0));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(1013, (((((0.5 * p.p40) * p.p40) * 1e18) - ((1.5 * p.p40) * 1000000000.0)) + 2.0));
        }

        if (s.v[1288] != 0.0) {
            s.store_offset_ad(1014, A::scale(A::sub(A::offset(s.ad_value(1013), 4.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1013), (-4.0)), A::offset(s.ad_value(1013), (-4.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));
        }

        if (s.v[1288] != 0.0) {
            let assign13760_ad_e18998: A = A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)));
            let assign13760_ad_e19068: A = A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)));
            let assign13760_ad_e19138: A = A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), (-18100.0)));
            let assign13760_ad_e19150: A = A::mul(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), 18100.0), A::sqrt(A::offset(assign13760_ad_e19068, ((0.25 * 0.01) * 0.01)))), 0.5), (-924000.0)), A::offset(A::scale(A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), 18100.0), A::sqrt(A::offset(assign13760_ad_e19138, ((0.25 * 0.01) * 0.01)))), 0.5), (-924000.0)));
            let assign13760_ad_e19159: A = A::scale(A::sub(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))), s.v[168]), 18100.0), A::sqrt(A::offset(assign13760_ad_e18998, ((0.25 * 0.01) * 0.01)))), 0.5), 924000.0), A::sqrt(A::offset(assign13760_ad_e19150, ((0.25 * 9240.0) * 9240.0)))), 0.5);
            s.store_offset_ad(974, assign13760_ad_e19159, (0.25 * 9240.0));
        }

        if (s.v[1288] != 0.0) {
            let assign13770_ad_e19234: A = A::add(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5)), ((0.25 * 0.01) * 0.01))));
            let assign13770_ad_e19304: A = A::add(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5)), ((0.25 * 0.01) * 0.01))));
            let assign13770_ad_e19374: A = A::add(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5), A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))), 5.5)), ((0.25 * 0.01) * 0.01))));
            s.store_offset_ad(975, A::scale(A::sub(A::offset(A::scale(assign13770_ad_e19234, 0.5), 8.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(assign13770_ad_e19304, 0.5), (-8.0)), A::offset(A::scale(assign13770_ad_e19374, 0.5), (-8.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p.p1895)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1895)));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p.p1896)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1896)));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p.p1897)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1897)));
        }

        if (s.v[1288] != 0.0) {
            let assign13810_ad_e19486: A = A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5))), ((0.25 * 0.01) * 0.01))));
            let assign13810_ad_e19538: A = A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5))), ((0.25 * 0.01) * 0.01))));
            let assign13810_ad_e19590: A = A::add(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5)), A::offset(A::offset(A::scale(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1), 0.7), (-0.5))), ((0.25 * 0.01) * 0.01))));
            s.store_offset_ad(990, A::scale(A::sub(A::offset(A::scale(assign13810_ad_e19486, 0.5), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(assign13810_ad_e19538, 0.5), (-1.0)), A::offset(A::scale(assign13810_ad_e19590, 0.5), (-1.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p.p1899)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1899)));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p.p1900)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1900)));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p.p1901)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1901)));
        }

        if (s.v[1288] != 0.0) {
            s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p.p1902)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1902)));
        }

        if (s.v[1288] != 0.0) {
            s.store_div_ad_rhs(987, 974, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(975), p.p1867)));
        }

        if (s.v[1288] != 0.0) {
            s.store_div_ad_rhs(988, 972, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(973), p.p1868)));
        }

        if (s.v[1288] != 0.0) {
            let assign13880_ad_e19749: A = A::offset(A::mul(A::add(s.ad_value(888), A::scale(A::sub(A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), s.ad_value(987)), p.p1865)), A::add(s.ad_value(888), A::scale(A::sub(A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), s.ad_value(987)), p.p1865))), ((0.25 * 0.01) * 0.01));
            s.store_scale_ad(985, A::add(A::add(s.ad_value(888), A::scale(A::sub(A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), s.ad_value(987)), p.p1865)), A::sqrt(assign13880_ad_e19749)), 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13890_ad_e19814: A = A::offset(A::mul(A::add(s.ad_value(889), A::scale(A::sub(A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), s.ad_value(988)), p.p1866)), A::add(s.ad_value(889), A::scale(A::sub(A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), s.ad_value(988)), p.p1866))), ((0.25 * 0.01) * 0.01));
            s.store_scale_ad(986, A::add(A::add(s.ad_value(889), A::scale(A::sub(A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), s.ad_value(988)), p.p1866)), A::sqrt(assign13890_ad_e19814)), 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13900_ad_e19881: A = A::add(A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5)), A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(995, assign13900_ad_e19881, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13910_ad_e19946: A = A::add(A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5)), A::div(s.ad_value(989), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(996, assign13910_ad_e19946, 0.5);
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
        if (s.v[1288] != 0.0) {
            s.store_add_ad_rhs(997, 890, A::scale(A::sub(s.ad_value(995), s.ad_value(996)), p.p1887));
        }

        if (s.v[1288] != 0.0) {
            let assign13930_ad_e20021: A = A::add(A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5)), A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(998, assign13930_ad_e20021, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13940_ad_e20086: A = A::add(A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5)), A::div(s.ad_value(991), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(999, assign13940_ad_e20086, 0.5);
        }

        if (s.v[1288] != 0.0) {
            s.store_add_ad_rhs(1000, 891, A::scale(A::sub(s.ad_value(998), s.ad_value(999)), p.p1888));
        }

        if (s.v[1288] != 0.0) {
            let assign13960_ad_e20161: A = A::add(A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5)), A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(1001, assign13960_ad_e20161, 0.5);
        }

        if (s.v[1288] != 0.0) {
            let assign13970_ad_e20226: A = A::add(A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5)), A::div(s.ad_value(993), A::powf(A::offset(A::scale(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0), 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(1002, assign13970_ad_e20226, 0.5);
        }

        if (s.v[1288] != 0.0) {
            s.store_add_ad_rhs(1003, 892, A::scale(A::sub(s.ad_value(1001), s.ad_value(1002)), p.p1889));
        }

        if (s.v[1288] != 0.0) {
            let assign13990_ad_e20305: A = A::sub(A::add(A::add(A::scale(A::exp(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), (-4.6))), 0.0385), A::scale(A::powf(A::offset(A::scale(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), 2.0), (-3.0)), 8.0), 7.5893e-7)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), 2.0), 6.0), 6.9583e-5)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), 2.0), 5.0), 0.0006583));
            let assign13990_ad_e20347: A = A::add(A::sub(A::add(assign13990_ad_e20305, A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), 2.0), 4.0), 0.0065)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), 2.0), 3.0), 0.026)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), 2.0), 2.0), 0.1371));
            s.store_mul_ad_lhs(1010, A::mul(A::mul(A::scale(A::scale(s.ad_value(960), 0.5), 1.60219e-19), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(960), 0.5)), A::offset(A::sub(assign13990_ad_e20347, A::scale(A::offset(A::offset(A::scale(s.ad_value(960), 0.5), 1.0), (-1.0)), (0.194 * 2.0))), 0.959))), A::pow(A::scale(s.ad_value(997), 1000000.0), s.ad_value(960))), 979);
        }

        if (s.v[1288] != 0.0) {
            let assign14000_ad_e20439: A = A::sub(A::add(A::add(A::scale(A::exp(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), (-4.6))), 0.0385), A::scale(A::powf(A::offset(A::scale(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), 2.0), (-3.0)), 8.0), 7.5893e-7)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), 2.0), 6.0), 6.9583e-5)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), 2.0), 5.0), 0.0006583));
            let assign14000_ad_e20481: A = A::add(A::sub(A::add(assign14000_ad_e20439, A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), 2.0), 4.0), 0.0065)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), 2.0), 3.0), 0.026)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), 2.0), 2.0), 0.1371));
            s.store_mul_ad_lhs(1011, A::mul(A::mul(A::scale(A::scale(s.ad_value(961), 0.5), 1.60219e-19), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(961), 0.5)), A::offset(A::sub(assign14000_ad_e20481, A::scale(A::offset(A::offset(A::scale(s.ad_value(961), 0.5), 1.0), (-1.0)), (0.194 * 2.0))), 0.959))), A::pow(A::scale(s.ad_value(1000), 1000000.0), s.ad_value(961))), 980);
        }

        if (s.v[1288] != 0.0) {
            let assign14010_ad_e20573: A = A::sub(A::add(A::add(A::scale(A::exp(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), (-4.6))), 0.0385), A::scale(A::powf(A::offset(A::scale(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), 2.0), (-3.0)), 8.0), 7.5893e-7)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), 2.0), 6.0), 6.9583e-5)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), 2.0), 5.0), 0.0006583));
            let assign14010_ad_e20615: A = A::add(A::sub(A::add(assign14010_ad_e20573, A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), 2.0), 4.0), 0.0065)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), 2.0), 3.0), 0.026)), A::scale(A::powf(A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), 2.0), 2.0), 0.1371));
            s.store_mul_ad_lhs(1012, A::mul(A::mul(A::scale(A::scale(s.ad_value(962), 0.5), 1.60219e-19), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(A::sub(assign14010_ad_e20615, A::scale(A::offset(A::offset(A::scale(s.ad_value(962), 0.5), 1.0), (-1.0)), (0.194 * 2.0))), 0.959))), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962))), 981);
        }

        s.v[1289] = if (p.p58 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1289] != 0.0) {
            let assign14030_ad_e20663: A = A::offset(A::scale(A::offset(s.ad_value(707), (-p.p889)), 1.0 / ((if (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891)) - 80.0) } else if (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891) < -80.0 { 1.804851387e-35 } else { (((((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891)) as f64).exp() } + 1.0))), p.p889);
            s.store_ad(707, &assign14030_ad_e20663);
        }

        if (s.v[1289] != 0.0) {
            s.store_offset(1024, 807, (((-p.p892)) + ((-((p.p893 * 1000000000.0) * p.p894)))));
        }

        if (s.v[1289] != 0.0) {
            let assign14050_ad_e20698: A = A::scale(A::offset(s.ad_value(1024), ((p.p40 * 1000000000.0) * p.p894)), 1.0 / ((1.0 + if (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896)) - 80.0) } else if (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896) < -80.0 { 1.804851387e-35 } else { (((((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896)) as f64).exp() })));
            s.store_ad(1025, &assign14050_ad_e20698);
        }

        if (s.v[1289] != 0.0) {
            s.store_scale_ad(807, A::sub(A::add(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), A::sqrt(A::offset(A::mul(A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2))), ((0.25 * 0.6) * 0.6)))), 0.5);
        }

        if (s.v[1289] != 0.0) {
            let assign14070_ad_e20764: A = A::add(A::scale(A::sub_from_scalar(p.p897, s.ad_value(811)), (370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898)))), A::scale(A::sub_from_scalar(p.p897, s.ad_value(811)), 1.0 / ((1.0 + if (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900) > 80.0 { 5.540622384e34 * (1.0 + ((((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900)) - 80.0) } else if (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900) < -80.0 { 1.804851387e-35 } else { (((((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900)) as f64).exp() }))));
            s.store_add_ad_lhs(1026, assign14070_ad_e20764, 811);
        }

        if (s.v[1289] != 0.0) {
            s.store_scale_ad(811, A::sub(A::offset(s.ad_value(1026), p.p897), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1026), (-p.p897)), A::offset(s.ad_value(1026), (-p.p897))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        if (s.v[1289] != 0.0) {
            s.store_scalar(1027, (p.p43 / (p.p43 + p.p40)));
        }

        if (s.v[1289] != 0.0) {
            s.store_scalar(1028, ((((p.p905 * p.p40) * p.p40) * 1e18) - (p.p906 * 0.001)));
        }

        if (s.v[1289] != 0.0) {
            s.store_scale_ad(1029, A::add(s.ad_value(1028), A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p.p906) * 0.001) * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)), 0.5)), 1.0 / (((((2.0 * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)));
        }

        if (s.v[1289] != 0.0) {
            let assign14120_ad_e20895: A = A::sub(A::offset(A::div_from_scalar(0.0001, A::offset(A::offset(s.ad_value(1029), (-0.8208)), (-(p.p907 * 1e-5)))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(0.0001, A::offset(A::offset(s.ad_value(1029), (-0.8208)), (-(p.p907 * 1e-5)))), (-1.0)), A::offset(A::div_from_scalar(0.0001, A::offset(A::offset(s.ad_value(1029), (-0.8208)), (-(p.p907 * 1e-5)))), (-1.0))), ((0.25 * 0.06) * 0.06))));
            s.store_scale_ad(1030, assign14120_ad_e20895, 0.5);
        }

        if (s.v[1289] != 0.0) {
            s.store_mul_ad_lhs(704, A::mul(s.ad_value(704), A::add(s.ad_value(1027), A::scale(A::sub_from_scalar(1.0, s.ad_value(1027)), p.p904))), 1030);
        }

        if (s.v[1289] != 0.0) {
            s.store_add_ad_lhs(812, A::scale(A::sub_from_scalar(p.p901, s.ad_value(812)), (((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)), 812);
        }

        s.v[1290] = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1290] != 0.0) {
            s.store_offset_ad(116, A::offset(A::voltage(ctx, &nodes, Some(4), None), ctx.temperature()), p.p22);
        }

        if (!(s.v[1290] != 0.0)) {
            s.store_scalar(116, (ctx.temperature() + p.p22));
        }

        s.store_div(229, 116, 228);

        s.store_offset(230, 229, (-1.0));

        s.store_sub(232, 116, 228);

        s.store_scale(179, 116, 8.617087e-5);

        s.store_scale(180, 228, 8.617087e-5);

        s.v[121] = p.p1786;

        s.v[1291] = if (p.p80 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1291] != 0.0) {
            s.store_scale_ad(119, A::add(A::offset(s.ad_value(116), s.v[121]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(116), (-s.v[121])), A::offset(s.ad_value(116), (-s.v[121]))), ((0.25 * p.p1788) * p.p1788)))), 0.5);
        }

        if (s.v[1291] != 0.0) {
            s.store_scale_ad(120, A::add(A::scale(A::offset(s.ad_value(116), (-p.p1787)), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(116), (-p.p1787)), (-p.p1790)), A::scale(A::offset(s.ad_value(116), (-p.p1787)), (-p.p1790))), ((0.25 * p.p1789) * p.p1789)))), 0.5);
        }

        s.v[1292] = if (p.p80 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1291] != 0.0) && (s.v[1292] != 0.0)) {
            s.store_scale_ad(169, A::add(A::offset(s.ad_value(228), s.v[121]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-s.v[121])), A::offset(s.ad_value(228), (-s.v[121]))), ((0.25 * p.p1788) * p.p1788)))), 0.5);
        }

        if ((s.v[1291] != 0.0) && (s.v[1292] != 0.0)) {
            s.store_scale_ad(170, A::add(A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790)), A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790))), ((0.25 * p.p1789) * p.p1789)))), 0.5);
        }

        s.v[1293] = if (s.v[228] > s.v[121]) { 1.0 } else { 0.0 };

        if (((s.v[1291] != 0.0) && (s.v[1292] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_add_ad_lhs(171, A::sub(A::sub(A::add(s.ad_value(119), s.ad_value(120)), s.ad_value(169)), s.ad_value(170)), 228);
        }

        if (((s.v[1291] != 0.0) && (s.v[1292] != 0.0)) && (!(s.v[1293] != 0.0))) {
            s.store_offset_ad(171, A::sub(A::sub(A::add(s.ad_value(119), s.ad_value(120)), s.ad_value(169)), s.ad_value(170)), s.v[121]);
        }

        if ((s.v[1291] != 0.0) && (s.v[1292] != 0.0)) {
            s.store_scale_ad(118, A::add(A::add(s.ad_value(116), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        s.v[1294] = if (s.v[121] > 210.0) { 1.0 } else { 0.0 };

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (s.v[1294] != 0.0)) {
            s.store_scalar(121, 210.0);
        }

        if ((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_offset_ad(312, A::scale(A::tanh(A::scale(A::offset(s.ad_value(116), (-210.0)), 0.5)), 0.5), 0.5);
        }

        if ((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_sub_from_scalar(313, 1.0, 312);
        }

        s.v[1295] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_scale_ad(169, A::add(A::offset(s.ad_value(121), 210.0), A::sqrt(A::offset(A::mul(A::sub_from_scalar(210.0, s.ad_value(121)), A::sub_from_scalar(210.0, s.ad_value(121))), ((0.25 * p.p1788) * p.p1788)))), 0.5);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_scalar(170, (0.5 * (((-p.p1790) * (210.0 - p.p1787)) + ((((((-p.p1790) * (210.0 - p.p1787)) * ((-p.p1790) * (210.0 - p.p1787))) + ((0.25 * p.p1789) * p.p1789))) as f64).sqrt())));
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_offset_ad(171, A::sub(A::sub(A::add(s.ad_value(119), s.ad_value(120)), s.ad_value(169)), s.ad_value(170)), 210.0);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_scale_ad(118, A::add(A::add(s.ad_value(116), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) {
            s.store_scale_ad(169, A::add(A::add(s.ad_value(228), s.ad_value(121)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(228), s.ad_value(121)), A::sub(s.ad_value(228), s.ad_value(121))), ((0.25 * p.p1788) * p.p1788)))), 0.5);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) {
            s.store_scale_ad(170, A::add(A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790)), A::scale(A::offset(s.ad_value(228), (-p.p1787)), (-p.p1790))), ((0.25 * p.p1789) * p.p1789)))), 0.5);
        }

        s.v[1296] = if (s.v[228] > s.v[121]) { 1.0 } else { 0.0 };

        if ((((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_add_ad_lhs(171, A::sub(A::sub(A::add(s.ad_value(119), s.ad_value(120)), s.ad_value(169)), s.ad_value(170)), 228);
        }

        if ((((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_add_ad_lhs(171, A::sub(A::sub(A::add(s.ad_value(119), s.ad_value(120)), s.ad_value(169)), s.ad_value(170)), 121);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) {
            s.store_scale_ad(172, A::add(A::add(s.ad_value(116), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        if (((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) && (!(s.v[1295] != 0.0))) {
            s.store_add_ad(118, A::mul(s.ad_value(313), s.ad_value(172)), A::mul(s.ad_value(312), s.ad_value(116)));
        }

        if ((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_scale_ad(117, A::sub(A::offset(s.ad_value(116), 210.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(116), (-210.0)), A::offset(s.ad_value(116), (-210.0))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        if ((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_sub_ad_rhs(233, 117, A::scale(A::sub(A::offset(s.ad_value(228), 210.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-210.0)), A::offset(s.ad_value(228), (-210.0))), ((0.25 * 0.2) * 0.2)))), 0.5));
        }

        if ((s.v[1291] != 0.0) && (!(s.v[1292] != 0.0))) {
            s.store_div_ad_lhs(234, A::offset(s.ad_value(117), (-210.0)), 228);
        }

        if (s.v[1291] != 0.0) {
            s.store_scale(182, 118, 8.617087e-5);
        }

        s.store_sub_from_scalar_ad(146, p.p106, A::div(A::mul(A::scale(s.ad_value(116), p.p1718), s.ad_value(116)), A::offset(s.ad_value(116), p.p1719)));

        s.store_sub_from_scalar_ad(147, p.p106, A::div(A::mul(A::scale(s.ad_value(228), p.p1718), s.ad_value(228)), A::offset(s.ad_value(228), p.p1719)));

        s.store_mul_ad(169, A::scale(s.ad_value(116), 0.003331667499583542), A::sqrt(A::scale(s.ad_value(116), 0.003331667499583542)));

        s.store_mul_ad(141, A::scale(s.ad_value(169), p.p105), A::limited_exp(A::sub_from_scalar((p.p106 / ((2.0 * 8.617087e-5) * 300.15)), A::div(s.ad_value(146), A::scale(s.ad_value(179), 2.0)))));

        s.v[1297] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1297] != 0.0) {
            s.store_scale(148, 169, p.p107);
        }

        if (!(s.v[1297] != 0.0)) {
            s.store_mul_ad(148, A::scale(A::scale(s.ad_value(118), 0.003331667499583542), p.p107), A::sqrt(A::scale(s.ad_value(118), 0.003331667499583542)));
        }

        if (!(s.v[1297] != 0.0)) {
            let assign14610_ad_e21688: A = A::sub(A::offset({
                if (!((p.p105 * s.v[169]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((p.p105 * s.v[169]) > 1e-38) {
                            A::ln(A::scale(s.ad_value(169), p.p105))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (p.p106 / ((2.0 * 8.617087e-5) * 300.15))), A::div(s.ad_value(146), A::scale(s.ad_value(179), 2.0)));
            s.store_ad(142, &assign14610_ad_e21688);
        }

        if (!(((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
            s.store_scale_ad(235, A::add(A::offset(A::offset(A::mul(s.ad_value(859), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(859), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(859), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5);
        } else {
            if (((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(235, ((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(859), s.ad_value(232)), 1.0), (-1e-6)));
            } else {
                s.store_scalar(235, 0.0);
            }
        }

        s.store_scale(389, 179, 1.60219e-19);

        s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div(A::scale(s.ad_value(894), 2.0), s.ad_value(895)));

        s.store_scale_ad(377, A::square(s.ad_value(168)), 1.0 / ((2.0 * s.v[381])));

        s.store_scale_ad(378, A::square(s.ad_value(168)), 1.0 / ((2.0 * s.v[382])));

        s.store_scale(379, 377, 4.0);

        s.store_scale(380, 378, 4.0);

        s.v[169] = ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383]));

        s.store_offset_ad(387, A::scale(A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(378)), s.ad_value(389))), s.v[169]), 1.0);

        s.store_add_ad(388, A::add(s.ad_value(387), A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(379)), s.ad_value(389)))), A::scale(A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(380)), s.ad_value(389))), s.v[169]));

        let assign14720_ad_e21904: A = {
    if (!((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38)) {
        A::neg(A::constant(87.498233534))
    } else {
        let assign14720_ad_e21903: A = {
            if ((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38) {
                A::ln(A::mul(A::div(A::mul(A::div_from_scalar((s.v[386] * s.v[383]), A::scale(s.ad_value(148), ((3.141592653589793 * 1.05457e-34) * 1.05457e-34))), s.ad_value(389)), A::div(A::scale(s.ad_value(894), 2.0), s.ad_value(895))), s.ad_value(388)))
            } else {
                A::constant(0.0)
            }
        };
        assign14720_ad_e21903
    }
};
        s.store_mul_ad(170, A::neg(s.ad_value(179)), assign14720_ad_e21904);

        s.store_mul_ad_rhs(375, 654, A::add(A::scale(s.ad_value(377), 6.241457005723417e18), s.ad_value(170)));

        s.store_ln(418, 229);

        s.v[1298] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1298] != 0.0) {
            s.store_mul_ad_rhs(169, 704, A::exp(A::mul(s.ad_value(836), s.ad_value(418))));
        }

        if (s.v[1298] != 0.0) {
            let assign14770_ad_e21975: A = A::add(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(413, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign14770_ad_e21975, 0.5)));
        }

        s.v[1299] = if (p.p66 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1299] != 0.0)) {
            s.store_mul_ad_rhs(169, 706, A::exp(A::mul(s.ad_value(845), s.ad_value(418))));
        }

        if ((s.v[1298] != 0.0) && (s.v[1299] != 0.0)) {
            let assign14800_ad_e22045: A = A::add(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(321, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign14800_ad_e22045, 0.5)));
        }

        if ((s.v[1298] != 0.0) && (s.v[1299] != 0.0)) {
            s.copy_ad(417, 321);
        }

        if (s.v[1298] != 0.0) {
            let assign14820_ad_e22095: A = A::add(A::offset(A::sub(A::mul(s.ad_value(823), s.ad_value(232)), A::neg(s.ad_value(807))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(823), s.ad_value(232)), A::neg(s.ad_value(807))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(823), s.ad_value(232)), A::neg(s.ad_value(807))), (-1e-6))), A::scale(A::neg(s.ad_value(807)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(303, 807, A::sub(A::scale(assign14820_ad_e22095, 0.5), s.ad_value(807)));
        }

        if (s.v[1298] != 0.0) {
            s.copy_ad(323, 811);
        }

        s.v[1300] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1300] != 0.0)) {
            let assign14850_ad_e22148: A = A::add(A::offset(A::sub(A::mul(s.ad_value(825), s.ad_value(232)), A::neg(s.ad_value(815))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(825), s.ad_value(232)), A::neg(s.ad_value(815))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(825), s.ad_value(232)), A::neg(s.ad_value(815))), (-1e-6))), A::scale(A::neg(s.ad_value(815)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(305, 815, A::sub(A::scale(assign14850_ad_e22148, 0.5), s.ad_value(815)));
        }

        if (s.v[1298] != 0.0) {
            s.store_mul_ad_rhs(318, 812, A::exp(A::mul(s.ad_value(830), s.ad_value(418))));
        }

        s.v[1301] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1301] != 0.0)) {
            s.store_mul_ad_rhs(320, 818, A::exp(A::mul(s.ad_value(844), s.ad_value(418))));
        }

        if (s.v[1298] != 0.0) {
            s.store_mul_ad_rhs(317, 814, A::exp(A::mul(s.ad_value(834), s.ad_value(418))));
        }

        if (s.v[1298] != 0.0) {
            let assign14900_ad_e22258: A = {
                if (!(((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(194, &assign14900_ad_e22258);
        }

        s.v[1302] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1302] != 0.0)) {
            let assign14920_ad_e22307: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(679))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(679))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(679))), (-1e-6))), A::scale(A::neg(s.ad_value(679)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(332, 679, A::sub(A::scale(assign14920_ad_e22307, 0.5), s.ad_value(679)));
        }

        if ((s.v[1298] != 0.0) && (!(s.v[1302] != 0.0))) {
            let assign14930_ad_e22395: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign14930_ad_e22395);
        }

        s.v[1303] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1304] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1298] != 0.0) && (s.v[1303] != 0.0)) && (s.v[1304] != 0.0)) {
            let assign14960_ad_e22450: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::neg(s.ad_value(680))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::neg(s.ad_value(680))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::neg(s.ad_value(680))), (-1e-6))), A::scale(A::neg(s.ad_value(680)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(333, 680, A::sub(A::scale(assign14960_ad_e22450, 0.5), s.ad_value(680)));
        }

        if (((s.v[1298] != 0.0) && (s.v[1303] != 0.0)) && (!(s.v[1304] != 0.0))) {
            let assign14970_ad_e22540: A = {
                if (!(((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign14970_ad_e22540);
        }

        s.v[1305] = if (s.v[333] < 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[1298] != 0.0) && (s.v[1303] != 0.0)) && (s.v[1305] != 0.0)) {
            s.store_scalar(333, 1000.0);
        }

        s.v[1306] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1306] != 0.0)) {
            s.store_mul_ad_rhs(169, 705, A::exp(A::mul(s.ad_value(839), s.ad_value(418))));
        }

        if ((s.v[1298] != 0.0) && (s.v[1306] != 0.0)) {
            let assign15020_ad_e22619: A = A::add(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(414, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign15020_ad_e22619, 0.5)));
        }

        if ((s.v[1298] != 0.0) && (s.v[1306] != 0.0)) {
            let assign15030_ad_e22665: A = A::add(A::offset(A::sub(A::mul(s.ad_value(826), s.ad_value(232)), A::neg(s.ad_value(808))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(826), s.ad_value(232)), A::neg(s.ad_value(808))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(826), s.ad_value(232)), A::neg(s.ad_value(808))), (-1e-6))), A::scale(A::neg(s.ad_value(808)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(304, 808, A::sub(A::scale(assign15030_ad_e22665, 0.5), s.ad_value(808)));
        }

        if ((s.v[1298] != 0.0) && (s.v[1306] != 0.0)) {
            s.store_mul_ad_rhs(319, 813, A::exp(A::mul(s.ad_value(832), s.ad_value(418))));
        }

        s.v[1307] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1307] != 0.0)) {
            let assign15060_ad_e22728: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(698))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(698))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(698))), (-1e-6))), A::scale(A::neg(s.ad_value(698)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(334, 698, A::sub(A::scale(assign15060_ad_e22728, 0.5), s.ad_value(698)));
        }

        if ((s.v[1298] != 0.0) && (!(s.v[1307] != 0.0))) {
            let assign15070_ad_e22816: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign15070_ad_e22816);
        }

        s.v[1308] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1309] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1298] != 0.0) && (s.v[1308] != 0.0)) && (s.v[1309] != 0.0)) {
            let assign15100_ad_e22871: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(699))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(699))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::neg(s.ad_value(699))), (-1e-6))), A::scale(A::neg(s.ad_value(699)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(335, 699, A::sub(A::scale(assign15100_ad_e22871, 0.5), s.ad_value(699)));
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
        if (((s.v[1298] != 0.0) && (s.v[1308] != 0.0)) && (!(s.v[1309] != 0.0))) {
            let assign15110_ad_e22961: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign15110_ad_e22961);
        }

        s.v[1310] = if (s.v[335] < 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[1298] != 0.0) && (s.v[1308] != 0.0)) && (s.v[1310] != 0.0)) {
            s.store_scalar(335, 1000.0);
        }

        s.v[1311] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1311] != 0.0)) {
            let assign15150_ad_e23022: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::neg(s.ad_value(702))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::neg(s.ad_value(702))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::neg(s.ad_value(702))), (-1e-6))), A::scale(A::neg(s.ad_value(702)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(336, 702, A::sub(A::scale(assign15150_ad_e23022, 0.5), s.ad_value(702)));
        }

        if ((s.v[1298] != 0.0) && (!(s.v[1311] != 0.0))) {
            let assign15160_ad_e23110: A = {
                if (!(((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign15160_ad_e23110);
        }

        if (s.v[1298] != 0.0) {
            let assign15170_ad_e23198: A = {
                if (!(((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::mul(s.ad_value(790), A::offset(A::scale(s.ad_value(232), p.p450), 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::offset(A::scale(s.ad_value(232), p.p450), 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(790), A::offset(A::scale(s.ad_value(232), p.p450), 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::offset(A::scale(s.ad_value(232), p.p450), 1.0)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(337, assign15170_ad_e23198, 2.0);
        }

        s.v[1312] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1298] != 0.0) && (s.v[1312] != 0.0)) {
            let assign15190_ad_e23292: A = {
                if (!(((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::mul(s.ad_value(791), A::offset(A::scale(s.ad_value(232), p.p452), 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::offset(A::scale(s.ad_value(232), p.p452), 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(791), A::offset(A::scale(s.ad_value(232), p.p452), 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::offset(A::scale(s.ad_value(232), p.p452), 1.0)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(338, assign15190_ad_e23292, 2.0);
        }

        if (s.v[1298] != 0.0) {
            s.copy_ad(660, 657);
        }

        if (s.v[1298] != 0.0) {
            s.copy_ad(797, 792);
        }

        if (s.v[1298] != 0.0) {
            s.store_mul_ad_lhs(231, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), 230);
        }

        s.v[1313] = if (p.p80 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(169, 704, A::exp(A::mul(A::add(s.ad_value(836), A::mul(s.ad_value(837), s.ad_value(229))), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            let assign15250_ad_e23385: A = A::add(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(413, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign15250_ad_e23385, 0.5)));
        }

        s.v[1314] = if (p.p66 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            s.store_mul_ad_rhs(169, 706, A::exp(A::mul(A::add(s.ad_value(845), A::mul(s.ad_value(837), s.ad_value(229))), s.ad_value(418))));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            let assign15280_ad_e23465: A = A::add(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(321, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign15280_ad_e23465, 0.5)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            s.copy_ad(417, 321);
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(303, 807, A::exp(A::mul(A::add(s.ad_value(823), A::mul(s.ad_value(824), s.ad_value(229))), s.ad_value(418))));
        }

        s.v[1315] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1315] != 0.0)) {
            s.store_mul_ad_rhs(305, 815, A::exp(A::mul(A::add(s.ad_value(825), A::mul(s.ad_value(824), s.ad_value(229))), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(318, 812, A::exp(A::mul(A::add(s.ad_value(830), A::mul(s.ad_value(831), s.ad_value(229))), s.ad_value(418))));
        }

        s.v[1316] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1316] != 0.0)) {
            s.store_mul_ad_rhs(320, 818, A::exp(A::mul(A::add(s.ad_value(844), A::mul(s.ad_value(831), s.ad_value(229))), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(317, 814, A::exp(A::mul(A::add(s.ad_value(834), A::scale(s.ad_value(229), p.p881)), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(324, 325, A::offset(A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), (-1.0)));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_mul_ad_rhs(327, 328, A::offset(A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), (-1.0)));
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_offset(330, 324, 0.5);
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_offset(331, 327, 0.5);
        }

        s.v[1317] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1317] != 0.0)) {
            let assign15420_ad_e23662: A = A::add(A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(232)), A::neg(s.ad_value(811))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(232)), A::neg(s.ad_value(811))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(232)), A::neg(s.ad_value(811))), (-1e-6))), A::scale(A::neg(s.ad_value(811)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(323, 811, A::sub(A::scale(assign15420_ad_e23662, 0.5), s.ad_value(811)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1317] != 0.0))) {
            let assign15430_ad_e23747: A = {
                if (!(((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(323, 811, assign15430_ad_e23747);
        }

        s.v[1318] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1318] != 0.0)) {
            s.store_mul_ad_rhs(169, 705, A::exp(A::mul(A::add(s.ad_value(839), A::mul(s.ad_value(840), s.ad_value(229))), s.ad_value(418))));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1318] != 0.0)) {
            let assign15460_ad_e23825: A = A::add(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(414, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign15460_ad_e23825, 0.5)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1318] != 0.0)) {
            s.store_mul_ad_rhs(304, 808, A::exp(A::mul(A::add(s.ad_value(826), A::mul(s.ad_value(827), s.ad_value(229))), s.ad_value(418))));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1318] != 0.0)) {
            s.store_mul_ad_rhs(319, 813, A::exp(A::mul(A::add(s.ad_value(832), A::mul(s.ad_value(833), s.ad_value(229))), s.ad_value(418))));
        }

        s.v[1319] = if (s.v[854] == s.v[855]) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1319] != 0.0)) {
            s.store_offset_ad(170, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        s.v[1320] = if (s.v[856] < s.v[228]) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_offset_ad(195, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_add_ad(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), A::mul(s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.v[1321] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) && (s.v[1321] != 0.0)) {
            let assign15560_ad_e23998: A = A::sub(A::scale(A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(170, &assign15560_ad_e23998);
        }

        if (((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) && (!(s.v[1321] != 0.0))) {
            let assign15570_ad_e24053: A = A::sub(A::scale(A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(170, &assign15570_ad_e24053);
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_offset_ad(196, A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228))), 1.0);
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_add_ad(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), A::mul(s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.v[1322] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) && (s.v[1322] != 0.0)) {
            let assign15620_ad_e24174: A = A::sub(A::scale(A::add(A::add(s.ad_value(196), s.ad_value(195)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(196), s.ad_value(195)), A::sub(s.ad_value(196), s.ad_value(195))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(170, &assign15620_ad_e24174);
        }

        if (((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1322] != 0.0))) {
            let assign15630_ad_e24230: A = A::sub(A::scale(A::sub(A::add(s.ad_value(196), s.ad_value(195)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(196), s.ad_value(195)), A::sub(s.ad_value(196), s.ad_value(195))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(170, &assign15630_ad_e24230);
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            let assign15640_ad_e24284: A = {
                if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(s.ad_value(170), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(170), (-1e-6)), A::offset(s.ad_value(170), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(s.ad_value(170), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(194, &assign15640_ad_e24284);
        }

        s.v[1323] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1323] != 0.0)) {
            let assign15660_ad_e24345: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(679))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(679))), (-1e-6)));
            s.store_add_ad_rhs(332, 679, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(679))), (-1e-6)), A::sqrt(A::sub(assign15660_ad_e24345, A::scale(A::neg(s.ad_value(679)), (4.0 * 1e-6))))), 0.5), s.ad_value(679)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1323] != 0.0))) {
            let assign15670_ad_e24481: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15670_ad_e24438: A = A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001)));
                    A::scale(A::add(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), assign15670_ad_e24438), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign15670_ad_e24481);
        }

        s.v[1324] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1325] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1324] != 0.0)) && (s.v[1325] != 0.0)) {
            let assign15700_ad_e24548: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(680))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(680))), (-1e-6)));
            s.store_add_ad_rhs(333, 680, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(680))), (-1e-6)), A::sqrt(A::sub(assign15700_ad_e24548, A::scale(A::neg(s.ad_value(680)), (4.0 * 1e-6))))), 0.5), s.ad_value(680)));
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1324] != 0.0)) && (!(s.v[1325] != 0.0))) {
            let assign15710_ad_e24686: A = {
                if (!((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15710_ad_e24643: A = A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001)));
                    A::scale(A::add(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), assign15710_ad_e24643), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign15710_ad_e24686);
        }

        s.v[1326] = if (s.v[333] < 1000.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1324] != 0.0)) && (s.v[1326] != 0.0)) {
            s.store_scalar(333, 1000.0);
        }

        s.v[1327] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1327] != 0.0)) {
            let assign15750_ad_e24762: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(698))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(698))), (-1e-6)));
            s.store_add_ad_rhs(334, 698, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(698))), (-1e-6)), A::sqrt(A::sub(assign15750_ad_e24762, A::scale(A::neg(s.ad_value(698)), (4.0 * 1e-6))))), 0.5), s.ad_value(698)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1327] != 0.0))) {
            let assign15760_ad_e24898: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15760_ad_e24855: A = A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001)));
                    A::scale(A::add(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), assign15760_ad_e24855), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign15760_ad_e24898);
        }

        s.v[1328] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1329] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1328] != 0.0)) && (s.v[1329] != 0.0)) {
            let assign15790_ad_e24965: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(699))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(699))), (-1e-6)));
            s.store_add_ad_rhs(335, 699, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), A::neg(s.ad_value(699))), (-1e-6)), A::sqrt(A::sub(assign15790_ad_e24965, A::scale(A::neg(s.ad_value(699)), (4.0 * 1e-6))))), 0.5), s.ad_value(699)));
        }

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1328] != 0.0)) && (!(s.v[1329] != 0.0))) {
            let assign15800_ad_e25103: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15800_ad_e25060: A = A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001)));
                    A::scale(A::add(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)), assign15800_ad_e25060), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p561), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign15800_ad_e25103);
        }

        s.v[1330] = if (s.v[335] < 1000.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1328] != 0.0)) && (s.v[1330] != 0.0)) {
            s.store_scalar(335, 1000.0);
        }

        s.v[1331] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1331] != 0.0)) {
            let assign15840_ad_e25179: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), A::neg(s.ad_value(702))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), A::neg(s.ad_value(702))), (-1e-6)));
            s.store_add_ad_rhs(336, 702, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), A::neg(s.ad_value(702))), (-1e-6)), A::sqrt(A::sub(assign15840_ad_e25179, A::scale(A::neg(s.ad_value(702)), (4.0 * 1e-6))))), 0.5), s.ad_value(702)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1331] != 0.0))) {
            let assign15850_ad_e25315: A = {
                if (!((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15850_ad_e25272: A = A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001)));
                    A::scale(A::add(A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), (-1e-6)), assign15850_ad_e25272), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), 1.0), A::mul(A::scale(s.ad_value(232), p.p574), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign15850_ad_e25315);
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            let assign15860_ad_e25442: A = {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign15860_ad_e25396: A = A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)), A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0))), ((4.0 * 0.001) * 0.001));
                    A::scale(A::add(A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)), A::sqrt(assign15860_ad_e25396)), 0.5)
                } else {
                    let assign15860_ad_e25441: A = {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign15860_ad_e25441
                }
            };
            s.store_offset_ad(337, assign15860_ad_e25442, 2.0);
        }

        s.v[1332] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1332] != 0.0)) {
            let assign15880_ad_e25575: A = {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign15880_ad_e25529: A = A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)), A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0))), ((4.0 * 0.001) * 0.001));
                    A::scale(A::add(A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)), A::sqrt(assign15880_ad_e25529)), 0.5)
                } else {
                    let assign15880_ad_e25574: A = {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(232), p.p451), s.ad_value(232)))), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign15880_ad_e25574
                }
            };
            s.store_offset_ad(338, assign15880_ad_e25575, 2.0);
        }

        s.v[1333] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1333] != 0.0)) {
            let assign15900_ad_e25642: A = A::sub(A::mul(A::offset(A::sub(A::add(A::scale(s.ad_value(232), p.p498), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), A::neg(s.ad_value(657))), (-1e-6)), A::offset(A::sub(A::add(A::scale(s.ad_value(232), p.p498), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), A::neg(s.ad_value(657))), (-1e-6))), A::scale(A::neg(s.ad_value(657)), (4.0 * 1e-6)));
            s.store_add_ad_rhs(660, 657, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::scale(s.ad_value(232), p.p498), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), A::neg(s.ad_value(657))), (-1e-6)), A::sqrt(assign15900_ad_e25642)), 0.5), s.ad_value(657)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1333] != 0.0))) {
            let assign15910_ad_e25765: A = {
                if (!((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15910_ad_e25725: A = A::add(A::offset(A::add(A::offset(A::scale(s.ad_value(232), p.p498), 1.0), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::scale(s.ad_value(232), p.p498), 1.0), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), (-1e-6)), A::offset(A::add(A::offset(A::scale(s.ad_value(232), p.p498), 1.0), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15910_ad_e25725, 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::scale(s.ad_value(232), p.p498), 1.0), A::mul(A::scale(s.ad_value(232), p.p499), s.ad_value(232))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(660, 657, assign15910_ad_e25765);
        }

        s.v[1334] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (s.v[1334] != 0.0)) {
            let assign15930_ad_e25816: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p1026), A::neg(s.ad_value(792))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p1026), A::neg(s.ad_value(792))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p1026), A::neg(s.ad_value(792))), (-1e-6))), A::scale(A::neg(s.ad_value(792)), (4.0 * 1e-6))))), 0.5);
            s.store_add_ad_rhs(797, 792, A::sub(assign15930_ad_e25816, s.ad_value(792)));
        }

        if (((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) && (!(s.v[1334] != 0.0))) {
            let assign15940_ad_e25900: A = {
                if (!(((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1026), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1026), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1026), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1026), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(797, 792, assign15940_ad_e25900);
        }

        if ((!(s.v[1298] != 0.0)) && (s.v[1313] != 0.0)) {
            s.store_sub_ad(231, A::add(A::mul(A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230)), A::div_from_scalar(p.p1747, A::offset(A::limited_exp(A::scale(A::offset(s.ad_value(116), (-p.p1749)), p.p1748)), 1.0))), A::div_from_scalar(p.p1747, A::offset(A::limited_exp(A::scale(A::offset(s.ad_value(228), (-p.p1749)), p.p1748)), 1.0)));
        }

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
        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_rhs(169, 704, A::exp(A::mul(A::add(s.ad_value(836), A::mul(s.ad_value(837), s.ad_value(234))), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            let assign15970_ad_e26008: A = A::add(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(838), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(413, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign15970_ad_e26008, 0.5)));
        }

        s.v[1335] = if (p.p66 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1335] != 0.0)) {
            s.store_mul_ad_rhs(169, 706, A::exp(A::mul(A::add(s.ad_value(845), A::mul(s.ad_value(837), s.ad_value(234))), s.ad_value(418))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1335] != 0.0)) {
            let assign16000_ad_e26090: A = A::add(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(846), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(321, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign16000_ad_e26090, 0.5)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1335] != 0.0)) {
            s.copy_ad(417, 321);
        }

        s.v[1336] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1336] != 0.0)) {
            let assign16030_ad_e26152: A = A::sub(A::div(s.ad_value(823), A::add(s.ad_value(807), A::mul(s.ad_value(823), A::sub_from_scalar(210.0, s.ad_value(228))))), A::div(A::mul(s.ad_value(824), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_scale_ad(170, assign16030_ad_e26152, 210.0);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_div_ad(169, A::add(s.ad_value(807), A::mul(s.ad_value(823), A::sub_from_scalar(210.0, s.ad_value(228)))), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(170), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228))))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_mul_ad_rhs(306, 169, A::pow(s.ad_value(229), A::add(s.ad_value(170), A::mul(s.ad_value(824), s.ad_value(229)))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_add_ad_rhs(307, 807, A::mul(s.ad_value(823), s.ad_value(232)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1336] != 0.0))) {
            let assign16070_ad_e26266: A = A::add(A::scale(s.ad_value(823), 0.004761904761904762), A::div(A::mul(s.ad_value(824), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_mul_ad(170, A::mul(s.ad_value(807), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(823), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)))))), assign16070_ad_e26266);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_sub_ad(169, A::mul(s.ad_value(807), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(823), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)))))), A::mul(s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_mul_ad_rhs(306, 807, A::pow(s.ad_value(229), A::add(s.ad_value(823), A::mul(s.ad_value(824), s.ad_value(229)))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_add_ad_rhs(307, 169, A::mul(s.ad_value(170), s.ad_value(232)));
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_add_ad(168, A::mul(s.ad_value(313), s.ad_value(306)), A::mul(s.ad_value(312), s.ad_value(307)));
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_ad(303, &{
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.v[1337] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1338] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            let assign16150_ad_e26441: A = A::sub(A::div(s.ad_value(825), A::add(s.ad_value(815), A::mul(s.ad_value(825), A::sub_from_scalar(210.0, s.ad_value(228))))), A::div(A::mul(s.ad_value(824), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_scale_ad(170, assign16150_ad_e26441, 210.0);
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_div_ad(169, A::add(s.ad_value(815), A::mul(s.ad_value(825), A::sub_from_scalar(210.0, s.ad_value(228)))), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(170), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228))))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_mul_ad_rhs(310, 169, A::pow(s.ad_value(229), A::add(s.ad_value(170), A::mul(s.ad_value(824), s.ad_value(229)))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_add_ad_rhs(311, 815, A::mul(s.ad_value(825), s.ad_value(232)));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            let assign16190_ad_e26563: A = A::add(A::scale(s.ad_value(825), 0.004761904761904762), A::div(A::mul(s.ad_value(824), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_mul_ad(170, A::mul(s.ad_value(815), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(825), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)))))), assign16190_ad_e26563);
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_sub_ad(169, A::mul(s.ad_value(815), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(825), A::mul(s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)))))), A::mul(s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_mul_ad_rhs(310, 815, A::pow(s.ad_value(229), A::add(s.ad_value(825), A::mul(s.ad_value(824), s.ad_value(229)))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_add_ad_rhs(311, 169, A::mul(s.ad_value(170), s.ad_value(232)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) {
            s.store_add_ad(168, A::mul(s.ad_value(313), s.ad_value(310)), A::mul(s.ad_value(312), s.ad_value(311)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1337] != 0.0)) {
            s.store_ad(305, &{
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_rhs(318, 812, A::exp(A::mul(A::add(s.ad_value(830), A::mul(s.ad_value(831), s.ad_value(234))), s.ad_value(418))));
        }

        s.v[1339] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1339] != 0.0)) {
            s.store_mul_ad_rhs(320, 818, A::exp(A::mul(A::add(s.ad_value(844), A::mul(s.ad_value(831), s.ad_value(234))), s.ad_value(418))));
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_mul_ad_rhs(317, 814, A::exp(A::mul(A::add(s.ad_value(834), A::mul(s.ad_value(835), s.ad_value(234))), s.ad_value(418))));
        }

        s.v[1340] = if (((((s.v[326] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1340] != 0.0)) {
            s.store_mul_ad_rhs(324, 325, A::offset(A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_div_ad(324, A::mul(s.ad_value(325), A::offset(A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0))), A::abs(A::offset(A::limited_exp(A::div(A::mul(s.ad_value(326), A::offset(s.ad_value(228), (-210.0))), s.ad_value(228))), (-1.0))));
        }

        s.v[1341] = if (((((s.v[329] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1341] != 0.0)) {
            s.store_mul_ad_rhs(327, 328, A::offset(A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1341] != 0.0))) {
            s.store_div_ad(327, A::mul(s.ad_value(328), A::offset(A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0))), A::abs(A::offset(A::limited_exp(A::div(A::mul(s.ad_value(329), A::offset(s.ad_value(228), (-210.0))), s.ad_value(228))), (-1.0))));
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_offset(330, 324, 0.5);
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_offset(331, 327, 0.5);
        }

        s.v[1342] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16380_ad_e26934: A = A::add(A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(233)), A::neg(s.ad_value(811))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(233)), A::neg(s.ad_value(811))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(847), s.ad_value(233)), A::neg(s.ad_value(811))), (-1e-6))), A::scale(A::neg(s.ad_value(811)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(323, 811, A::sub(A::scale(assign16380_ad_e26934, 0.5), s.ad_value(811)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16390_ad_e26996: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(679))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(679))), (-1e-6)));
            s.store_add_ad_rhs(332, 679, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(679))), (-1e-6)), A::sqrt(A::sub(assign16390_ad_e26996, A::scale(A::neg(s.ad_value(679)), (4.0 * 1e-6))))), 0.5), s.ad_value(679)));
        }

        s.v[1343] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) && (s.v[1343] != 0.0)) {
            let assign16410_ad_e27072: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(680))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(680))), (-1e-6)));
            s.store_add_ad_rhs(333, 680, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(851)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(680))), (-1e-6)), A::sqrt(A::sub(assign16410_ad_e27072, A::scale(A::neg(s.ad_value(680)), (4.0 * 1e-6))))), 0.5), s.ad_value(680)));
        }

        s.v[1344] = if (s.v[333] < 1000.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) && (s.v[1343] != 0.0)) && (s.v[1344] != 0.0)) {
            s.store_scalar(333, 1000.0);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16440_ad_e27160: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(698))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(698))), (-1e-6)));
            s.store_add_ad_rhs(334, 698, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(698))), (-1e-6)), A::sqrt(A::sub(assign16440_ad_e27160, A::scale(A::neg(s.ad_value(698)), (4.0 * 1e-6))))), 0.5), s.ad_value(698)));
        }

        s.v[1345] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) && (s.v[1345] != 0.0)) {
            let assign16460_ad_e27236: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(699))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(699))), (-1e-6)));
            s.store_add_ad_rhs(335, 699, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(849)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), A::neg(s.ad_value(699))), (-1e-6)), A::sqrt(A::sub(assign16460_ad_e27236, A::scale(A::neg(s.ad_value(699)), (4.0 * 1e-6))))), 0.5), s.ad_value(699)));
        }

        s.v[1346] = if (s.v[335] < 1000.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) && (s.v[1345] != 0.0)) && (s.v[1346] != 0.0)) {
            s.store_scalar(335, 1000.0);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16490_ad_e27324: A = A::mul(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), A::neg(s.ad_value(702))), (-1e-6)), A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), A::neg(s.ad_value(702))), (-1e-6)));
            s.store_add_ad_rhs(336, 702, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::mul(A::neg(s.ad_value(850)), s.ad_value(232)), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), A::neg(s.ad_value(702))), (-1e-6)), A::sqrt(A::sub(assign16490_ad_e27324, A::scale(A::neg(s.ad_value(702)), (4.0 * 1e-6))))), 0.5), s.ad_value(702)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16500_ad_e27399: A = A::sub(A::mul(A::offset(A::sub(A::add(A::scale(s.ad_value(233), p.p498), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), A::neg(s.ad_value(657))), (-1e-6)), A::offset(A::sub(A::add(A::scale(s.ad_value(233), p.p498), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), A::neg(s.ad_value(657))), (-1e-6))), A::scale(A::neg(s.ad_value(657)), (4.0 * 1e-6)));
            s.store_add_ad_rhs(660, 657, A::sub(A::scale(A::add(A::offset(A::sub(A::add(A::scale(s.ad_value(233), p.p498), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), A::neg(s.ad_value(657))), (-1e-6)), A::sqrt(assign16500_ad_e27399)), 0.5), s.ad_value(657)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1342] != 0.0)) {
            let assign16510_ad_e27452: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(233), p.p1026), A::neg(s.ad_value(792))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(233), p.p1026), A::neg(s.ad_value(792))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(233), p.p1026), A::neg(s.ad_value(792))), (-1e-6))), A::scale(A::neg(s.ad_value(792)), (4.0 * 1e-6))))), 0.5);
            s.store_add_ad_rhs(797, 792, A::sub(assign16510_ad_e27452, s.ad_value(792)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16520_ad_e27537: A = {
                if (!(((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(323, 811, assign16520_ad_e27537);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16530_ad_e27657: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16530_ad_e27617: A = A::add(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16530_ad_e27617, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign16530_ad_e27657);
        }

        s.v[1347] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1347] != 0.0)) {
            let assign16550_ad_e27782: A = {
                if (!((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16550_ad_e27742: A = A::add(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16550_ad_e27742, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign16550_ad_e27782);
        }

        s.v[1348] = if (s.v[333] < 1000.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1347] != 0.0)) && (s.v[1348] != 0.0)) {
            s.store_scalar(333, 1000.0);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16580_ad_e27920: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16580_ad_e27880: A = A::add(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16580_ad_e27880, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign16580_ad_e27920);
        }

        s.v[1349] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1349] != 0.0)) {
            let assign16600_ad_e28045: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16600_ad_e28005: A = A::add(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16600_ad_e28005, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p561), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign16600_ad_e28045);
        }

        s.v[1350] = if (s.v[335] < 1000.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_scalar(335, 1000.0);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16630_ad_e28183: A = {
                if (!((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16630_ad_e28143: A = A::add(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16630_ad_e28143, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), A::mul(A::scale(s.ad_value(233), p.p574), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign16630_ad_e28183);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16640_ad_e28303: A = {
                if (!((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16640_ad_e28263: A = A::add(A::offset(A::add(A::offset(A::scale(s.ad_value(233), p.p498), 1.0), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add(A::offset(A::scale(s.ad_value(233), p.p498), 1.0), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), (-1e-6)), A::offset(A::add(A::offset(A::scale(s.ad_value(233), p.p498), 1.0), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16640_ad_e28263, 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add(A::offset(A::scale(s.ad_value(233), p.p498), 1.0), A::mul(A::scale(s.ad_value(233), p.p499), s.ad_value(233))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(660, 657, assign16640_ad_e28303);
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1342] != 0.0))) {
            let assign16650_ad_e28387: A = {
                if (!(((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(233), p.p1026), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(233), p.p1026), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(233), p.p1026), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(233), p.p1026), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(797, 792, assign16650_ad_e28387);
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            let assign16660_ad_e28515: A = {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign16660_ad_e28469: A = A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)), A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0))), ((4.0 * 0.001) * 0.001));
                    A::scale(A::add(A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)), A::sqrt(assign16660_ad_e28469)), 0.5)
                } else {
                    let assign16660_ad_e28514: A = {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::add(A::offset(A::scale(s.ad_value(232), p.p450), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign16660_ad_e28514
                }
            };
            s.store_offset_ad(337, assign16660_ad_e28515, 2.0);
        }

        s.v[1351] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1351] != 0.0)) {
            let assign16680_ad_e28649: A = {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign16680_ad_e28603: A = A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)), A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0))), ((4.0 * 0.001) * 0.001));
                    A::scale(A::add(A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)), A::sqrt(assign16680_ad_e28603)), 0.5)
                } else {
                    let assign16680_ad_e28648: A = {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::add(A::offset(A::scale(s.ad_value(232), p.p452), 1.0), A::mul(A::scale(s.ad_value(233), p.p451), s.ad_value(233)))), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign16680_ad_e28648
                }
            };
            s.store_offset_ad(338, assign16680_ad_e28649, 2.0);
        }

        s.v[1352] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_mul_ad_rhs(169, 705, A::exp(A::mul(A::add(s.ad_value(839), A::mul(s.ad_value(840), s.ad_value(234))), s.ad_value(418))));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) {
            let assign16710_ad_e28730: A = A::add(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001)), A::offset(A::sub(A::mul(s.ad_value(841), s.ad_value(232)), A::scale(s.ad_value(169), (-0.9))), (-0.0001))), A::scale(s.ad_value(169), ((-0.9) * (4.0 * 0.0001))))));
            s.store_add_ad_rhs(414, 169, A::add(A::scale(s.ad_value(169), (-0.9)), A::scale(assign16710_ad_e28730, 0.5)));
        }

        s.v[1353] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

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
        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (s.v[1353] != 0.0)) {
            let assign16730_ad_e28784: A = A::sub(A::div(s.ad_value(826), A::add(s.ad_value(808), A::mul(s.ad_value(826), A::sub_from_scalar(210.0, s.ad_value(228))))), A::div(A::mul(s.ad_value(827), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_scale_ad(170, assign16730_ad_e28784, 210.0);
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (s.v[1353] != 0.0)) {
            s.store_div_ad(169, A::add(s.ad_value(808), A::mul(s.ad_value(826), A::sub_from_scalar(210.0, s.ad_value(228)))), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(170), A::mul(s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228))))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (s.v[1353] != 0.0)) {
            s.store_mul_ad_rhs(308, 169, A::pow(s.ad_value(229), A::add(s.ad_value(170), A::mul(s.ad_value(827), s.ad_value(229)))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (s.v[1353] != 0.0)) {
            s.store_add_ad_rhs(309, 808, A::mul(s.ad_value(826), s.ad_value(232)));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (!(s.v[1353] != 0.0))) {
            let assign16770_ad_e28906: A = A::add(A::scale(s.ad_value(826), 0.004761904761904762), A::div(A::mul(s.ad_value(827), A::offset({
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0)), s.ad_value(228)));
            s.store_mul_ad(170, A::mul(s.ad_value(808), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(826), A::mul(s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)))))), assign16770_ad_e28906);
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (!(s.v[1353] != 0.0))) {
            s.store_sub_ad(169, A::mul(s.ad_value(808), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add(s.ad_value(826), A::mul(s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)))))), A::mul(s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (!(s.v[1353] != 0.0))) {
            s.store_mul_ad_rhs(308, 808, A::pow(s.ad_value(229), A::add(s.ad_value(826), A::mul(s.ad_value(827), s.ad_value(229)))));
        }

        if ((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) && (!(s.v[1353] != 0.0))) {
            s.store_add_ad_rhs(309, 169, A::mul(s.ad_value(170), s.ad_value(232)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_add_ad(168, A::mul(s.ad_value(313), s.ad_value(308)), A::mul(s.ad_value(312), s.ad_value(309)));
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_ad(304, &{
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_mul_ad_rhs(319, 813, A::exp(A::mul(A::add(s.ad_value(832), A::mul(s.ad_value(833), s.ad_value(234))), s.ad_value(418))));
        }

        s.v[1354] = if (s.v[854] == s.v[855]) { 1.0 } else { 0.0 };

        if (((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (s.v[1354] != 0.0)) {
            s.store_offset_ad(170, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        s.v[1355] = if (s.v[856] < 210.0) { 1.0 } else { 0.0 };

        s.v[1356] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) {
            s.store_offset_ad(195, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) {
            s.store_add_ad(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), A::mul(s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) {
            s.store_offset_ad(171, A::mul(s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228))), 1.0);
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) {
            s.store_add_ad(172, A::offset(A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), A::mul(s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        s.v[1357] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) && (s.v[1357] != 0.0)) {
            let assign16930_ad_e29233: A = A::sub(A::scale(A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(A::add(s.ad_value(171), s.ad_value(172)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(171), s.ad_value(172)), A::sub(s.ad_value(171), s.ad_value(172))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_add_ad_lhs(174, assign16930_ad_e29233, 171);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) && (s.v[1357] != 0.0)) {
            s.store_scale_ad(170, A::add(A::add(s.ad_value(174), s.ad_value(195)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(174), s.ad_value(195)), A::sub(s.ad_value(174), s.ad_value(195))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) && (!(s.v[1357] != 0.0))) {
            let assign16950_ad_e29329: A = A::sub(A::scale(A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(A::add(s.ad_value(171), s.ad_value(172)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(171), s.ad_value(172)), A::sub(s.ad_value(171), s.ad_value(172))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_add_ad_lhs(174, assign16950_ad_e29329, 171);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (s.v[1356] != 0.0)) && (!(s.v[1357] != 0.0))) {
            s.store_scale_ad(170, A::sub(A::add(s.ad_value(174), s.ad_value(195)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(174), s.ad_value(195)), A::sub(s.ad_value(174), s.ad_value(195))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.v[1358] = if (s.v[228] > s.v[856]) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_offset_ad(195, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_add_ad(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), A::mul(s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228))), 1.0);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_add_ad(174, A::offset(A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), A::mul(s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        s.v[1359] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (s.v[1359] != 0.0)) {
            let assign17040_ad_e29564: A = A::sub(A::scale(A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(175, &assign17040_ad_e29564);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (s.v[1359] != 0.0)) {
            let assign17050_ad_e29624: A = A::sub(A::scale(A::add(A::add(s.ad_value(172), s.ad_value(174)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(176, &assign17050_ad_e29624);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (s.v[1359] != 0.0)) {
            s.store_add_ad_rhs(177, 176, A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))));
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (s.v[1359] != 0.0)) {
            s.store_scale_ad(170, A::add(A::add(s.ad_value(175), s.ad_value(177)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (!(s.v[1359] != 0.0))) {
            let assign17080_ad_e29750: A = A::sub(A::scale(A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(175, &assign17080_ad_e29750);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (!(s.v[1359] != 0.0))) {
            let assign17090_ad_e29811: A = A::sub(A::scale(A::sub(A::add(s.ad_value(172), s.ad_value(174)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(176, &assign17090_ad_e29811);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (!(s.v[1359] != 0.0))) {
            s.store_add_ad_rhs(177, 176, A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))));
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(170, A::sub(A::add(s.ad_value(175), s.ad_value(177)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_offset_ad(196, A::mul(s.ad_value(855), s.ad_value(232)), 1.0);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_add_ad(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), A::mul(s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_offset_ad(172, A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(228))), 1.0);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_add_ad(174, A::offset(A::mul(s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), A::mul(s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228))));
        }

        s.v[1360] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1360] != 0.0)) {
            let assign17180_ad_e30077: A = A::sub(A::scale(A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(175, &assign17180_ad_e30077);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1360] != 0.0)) {
            let assign17190_ad_e30138: A = A::sub(A::scale(A::add(A::add(s.ad_value(172), s.ad_value(174)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::add(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(176, &assign17190_ad_e30138);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1360] != 0.0)) {
            s.store_add_ad_rhs(177, 176, A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))));
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1360] != 0.0)) {
            s.store_scale_ad(170, A::add(A::add(s.ad_value(175), s.ad_value(177)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1360] != 0.0))) {
            let assign17220_ad_e30267: A = A::sub(A::scale(A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(175, &assign17220_ad_e30267);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1360] != 0.0))) {
            let assign17230_ad_e30329: A = A::sub(A::scale(A::sub(A::add(s.ad_value(172), s.ad_value(174)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174))), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5), A::scale(A::sub(s.ad_value(171), A::sqrt(A::add(A::mul(s.ad_value(171), s.ad_value(171)), A::mul(A::scale(s.ad_value(857), 0.25), s.ad_value(857))))), 0.5));
            s.store_ad(176, &assign17230_ad_e30329);
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_add_ad_rhs(177, 176, A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))));
        }

        if (((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_scale_ad(170, A::sub(A::add(s.ad_value(175), s.ad_value(177)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.v[1361] = if (s.v[228] > 210.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_offset_ad(195, A::mul(s.ad_value(854), s.ad_value(232)), 1.0);
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_add_ad(196, A::offset(A::mul(s.ad_value(855), A::offset(s.ad_value(116), (-210.0))), 1.0), A::mul(s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228))));
        }

        s.v[1362] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1361] != 0.0)) && (s.v[1362] != 0.0)) {
            s.store_scale_ad(170, A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01)))), 0.5);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1361] != 0.0)) && (!(s.v[1362] != 0.0))) {
            s.store_scale_ad(170, A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01)))), 0.5);
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_offset_ad(196, A::mul(s.ad_value(855), s.ad_value(232)), 1.0);
        }

        if (((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_add_ad(195, A::offset(A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))), 1.0), A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(228))));
        }

        s.v[1363] = if (s.v[855] < s.v[854]) { 1.0 } else { 0.0 };

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1361] != 0.0))) && (s.v[1363] != 0.0)) {
            s.store_scale_ad(170, A::add(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01)))), 0.5);
        }

        if ((((((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1361] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_scale_ad(170, A::sub(A::add(s.ad_value(195), s.ad_value(196)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01)))), 0.5);
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            let assign17370_ad_e30712: A = {
                if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(s.ad_value(170), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(170), (-1e-6)), A::offset(s.ad_value(170), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(s.ad_value(170), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(194, &assign17370_ad_e30712);
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_scale_ad(172, A::sub(A::offset(s.ad_value(228), 210.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-210.0)), A::offset(s.ad_value(228), (-210.0))), ((0.25 * 0.2) * 0.2)))), 0.5);
        }

        if ((!(s.v[1298] != 0.0)) && (!(s.v[1313] != 0.0))) {
            s.store_sub_ad(231, A::add(A::mul(A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230)), A::div_from_scalar(p.p1747, A::offset(A::limited_exp(A::scale(A::offset(s.ad_value(117), (-p.p1749)), p.p1748)), 1.0))), A::div_from_scalar(p.p1747, A::offset(A::limited_exp(A::scale(A::offset(s.ad_value(172), (-p.p1749)), p.p1748)), 1.0)));
        }

        s.v[1364] = if (s.v[332] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[1364] != 0.0) {
            s.store_scalar(332, 1000.0);
        }

        s.v[1365] = if (s.v[334] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[1365] != 0.0) {
            s.store_scalar(334, 1000.0);
        }

        s.v[1366] = if (s.v[336] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[1366] != 0.0) {
            s.store_scalar(336, 1000.0);
        }

        s.v[1367] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1368] = if (p.p75 == 0.0) { 1.0 } else { 0.0 };

        s.v[1369] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (s.v[1369] != 0.0)) {
            let assign17490_ad_e30850: A = A::add(A::offset(A::sub(A::mul(s.ad_value(828), s.ad_value(232)), A::neg(s.ad_value(809))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(828), s.ad_value(232)), A::neg(s.ad_value(809))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(828), s.ad_value(232)), A::neg(s.ad_value(809))), (-1e-6))), A::scale(A::neg(s.ad_value(809)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(314, 809, A::sub(A::scale(assign17490_ad_e30850, 0.5), s.ad_value(809)));
        }

        if (((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (!(s.v[1369] != 0.0))) {
            let assign17500_ad_e30934: A = {
                if (!(((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(314, 809, assign17500_ad_e30934);
        }

        s.v[1370] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        s.v[1371] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (s.v[1370] != 0.0)) && (s.v[1371] != 0.0)) {
            let assign17530_ad_e30988: A = A::add(A::offset(A::sub(A::mul(s.ad_value(829), s.ad_value(232)), A::neg(s.ad_value(810))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(829), s.ad_value(232)), A::neg(s.ad_value(810))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(829), s.ad_value(232)), A::neg(s.ad_value(810))), (-1e-6))), A::scale(A::neg(s.ad_value(810)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(315, 810, A::sub(A::scale(assign17530_ad_e30988, 0.5), s.ad_value(810)));
        }

        if ((((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (s.v[1370] != 0.0)) && (!(s.v[1371] != 0.0))) {
            let assign17540_ad_e31074: A = {
                if (!(((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(315, 810, assign17540_ad_e31074);
        }

        s.v[1372] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1373] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (s.v[1372] != 0.0)) && (s.v[1373] != 0.0)) {
            let assign17570_ad_e31128: A = A::add(A::offset(A::sub(A::mul(s.ad_value(843), s.ad_value(232)), A::neg(s.ad_value(817))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(843), s.ad_value(232)), A::neg(s.ad_value(817))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(843), s.ad_value(232)), A::neg(s.ad_value(817))), (-1e-6))), A::scale(A::neg(s.ad_value(817)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(316, 817, A::sub(A::scale(assign17570_ad_e31128, 0.5), s.ad_value(817)));
        }

        if ((((s.v[1367] != 0.0) && (s.v[1368] != 0.0)) && (s.v[1372] != 0.0)) && (!(s.v[1373] != 0.0))) {
            let assign17580_ad_e31214: A = {
                if (!(((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(316, 817, assign17580_ad_e31214);
        }

        if ((s.v[1367] != 0.0) && (!(s.v[1368] != 0.0))) {
            s.store_add_ad_rhs(314, 809, A::mul(s.ad_value(828), s.ad_value(232)));
        }

        s.v[1374] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1367] != 0.0) && (!(s.v[1368] != 0.0))) && (s.v[1374] != 0.0)) {
            s.store_add_ad_rhs(315, 810, A::mul(s.ad_value(829), s.ad_value(232)));
        }

        s.v[1375] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1367] != 0.0) && (!(s.v[1368] != 0.0))) && (s.v[1375] != 0.0)) {
            s.store_add_ad_rhs(316, 817, A::mul(s.ad_value(843), s.ad_value(232)));
        }

        s.v[1376] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1376] != 0.0) {
            let assign17650_ad_e31303: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p164), A::neg(s.ad_value(673))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p164), A::neg(s.ad_value(673))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p164), A::neg(s.ad_value(673))), (-1e-6))), A::scale(A::neg(s.ad_value(673)), (4.0 * 1e-6))))), 0.5);
            s.store_add_ad_rhs(296, 673, A::sub(assign17650_ad_e31303, s.ad_value(673)));
        }

        if (!(s.v[1376] != 0.0)) {
            let assign17660_ad_e31382: A = {
                if (!(((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p164), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p164), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p164), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p164), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(296, 673, assign17660_ad_e31382);
        }

        s.v[1377] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        s.v[1378] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1377] != 0.0) && (s.v[1378] != 0.0)) {
            let assign17690_ad_e31433: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p165), A::neg(s.ad_value(675))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p165), A::neg(s.ad_value(675))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p165), A::neg(s.ad_value(675))), (-1e-6))), A::scale(A::neg(s.ad_value(675)), (4.0 * 1e-6))))), 0.5);
            s.store_add_ad_rhs(297, 675, A::sub(assign17690_ad_e31433, s.ad_value(675)));
        }

        if ((s.v[1377] != 0.0) && (!(s.v[1378] != 0.0))) {
            let assign17700_ad_e31514: A = {
                if (!(((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p165), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p165), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p165), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p165), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(297, 675, assign17700_ad_e31514);
        }

        s.v[1379] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1379] != 0.0) {
            let assign17720_ad_e31560: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p166), A::neg(s.ad_value(677))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p166), A::neg(s.ad_value(677))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p166), A::neg(s.ad_value(677))), (-1e-6))), A::scale(A::neg(s.ad_value(677)), (4.0 * 1e-6))))), 0.5);
            s.store_add_ad_rhs(298, 677, A::sub(assign17720_ad_e31560, s.ad_value(677)));
        }

        if (!(s.v[1379] != 0.0)) {
            let assign17730_ad_e31639: A = {
                if (!(((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p166), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p166), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p166), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p166), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(298, 677, assign17730_ad_e31639);
        }

        s.v[1380] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1380] != 0.0) {
            let assign17750_ad_e31684: A = A::add(A::offset(A::sub(A::mul(s.ad_value(842), s.ad_value(232)), A::neg(s.ad_value(707))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(842), s.ad_value(232)), A::neg(s.ad_value(707))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(842), s.ad_value(232)), A::neg(s.ad_value(707))), (-1e-6))), A::scale(A::neg(s.ad_value(707)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(322, 707, A::sub(A::scale(assign17750_ad_e31684, 0.5), s.ad_value(707)));
        }

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
        if (!(s.v[1380] != 0.0)) {
            let assign17760_ad_e31764: A = {
                if (!(((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(322, 707, assign17760_ad_e31764);
        }

        s.v[1381] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1381] != 0.0) {
            let assign17780_ad_e31812: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p917))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p917))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p917))), (-1e-6))), (-((4.0 * (-p.p917)) * 1e-6))))), 0.5), (-p.p917)), p.p917);
            s.store_ad(299, &assign17780_ad_e31812);
        }

        if (!(s.v[1381] != 0.0)) {
            let assign17790_ad_e31889: A = {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(299, assign17790_ad_e31889, p.p917);
        }

        s.v[1382] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1383] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1382] != 0.0) && (s.v[1383] != 0.0)) {
            let assign17820_ad_e31942: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p918))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p918))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), (-(-p.p918))), (-1e-6))), (-((4.0 * (-p.p918)) * 1e-6))))), 0.5), (-p.p918)), p.p918);
            s.store_ad(300, &assign17820_ad_e31942);
        }

        if ((s.v[1382] != 0.0) && (!(s.v[1383] != 0.0))) {
            let assign17830_ad_e32021: A = {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p923), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(300, assign17830_ad_e32021, p.p918);
        }

        s.v[1384] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1384] != 0.0) {
            let assign17850_ad_e32069: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p919))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p919))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p919))), (-1e-6))), (-((4.0 * (-p.p919)) * 1e-6))))), 0.5), (-p.p919)), p.p919);
            s.store_ad(301, &assign17850_ad_e32069);
        }

        if (!(s.v[1384] != 0.0)) {
            let assign17860_ad_e32146: A = {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(301, assign17860_ad_e32146, p.p919);
        }

        s.v[1385] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1386] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1385] != 0.0) && (s.v[1386] != 0.0)) {
            let assign17890_ad_e32199: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p920))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p920))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), (-(-p.p920))), (-1e-6))), (-((4.0 * (-p.p920)) * 1e-6))))), 0.5), (-p.p920)), p.p920);
            s.store_ad(302, &assign17890_ad_e32199);
        }

        if ((s.v[1385] != 0.0) && (!(s.v[1386] != 0.0))) {
            let assign17900_ad_e32278: A = {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p924), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(302, assign17900_ad_e32278, p.p920);
        }

        s.v[1387] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1387] != 0.0) {
            let assign17920_ad_e32326: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(700))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(700))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(700))), (-1e-6))), A::scale(A::neg(s.ad_value(700)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(257, 700, A::sub(A::scale(assign17920_ad_e32326, 0.5), s.ad_value(700)));
        }

        if (!(s.v[1387] != 0.0)) {
            let assign17930_ad_e32412: A = {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(257, 700, assign17930_ad_e32412);
        }

        s.v[1388] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        s.v[1389] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1388] != 0.0) && (s.v[1389] != 0.0)) {
            let assign17960_ad_e32465: A = A::add(A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(701))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(701))), (-1e-6)), A::offset(A::sub(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), A::neg(s.ad_value(701))), (-1e-6))), A::scale(A::neg(s.ad_value(701)), (4.0 * 1e-6)))));
            s.store_add_ad_rhs(258, 701, A::sub(A::scale(assign17960_ad_e32465, 0.5), s.ad_value(701)));
        }

        if ((s.v[1388] != 0.0) && (!(s.v[1389] != 0.0))) {
            let assign17970_ad_e32553: A = {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(A::neg(s.ad_value(848)), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(258, 701, assign17970_ad_e32553);
        }

        s.store_mul_ad_rhs(248, 779, A::exp(A::mul(s.ad_value(860), s.ad_value(418))));

        let assign17990_ad_e32635: A = {
    if (!(((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), 1.0), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), 1.0), (-0.01)), A::offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), 1.0), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), 1.0), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(249, 785, A::offset(assign17990_ad_e32635, 0.01));

        s.store_add_ad_rhs(236, 683, A::mul(s.ad_value(684), s.ad_value(232)));

        let assign18010_ad_e32681: A = A::add(A::offset(A::sub(A::mul(s.ad_value(686), s.ad_value(232)), A::neg(s.ad_value(685))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(686), s.ad_value(232)), A::neg(s.ad_value(685))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(686), s.ad_value(232)), A::neg(s.ad_value(685))), (-1e-6))), A::scale(A::neg(s.ad_value(685)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(237, 685, A::sub(A::scale(assign18010_ad_e32681, 0.5), s.ad_value(685)));

        let assign18020_ad_e32722: A = A::add(A::offset(A::sub(A::mul(s.ad_value(688), s.ad_value(232)), A::neg(s.ad_value(687))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(688), s.ad_value(232)), A::neg(s.ad_value(687))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(688), s.ad_value(232)), A::neg(s.ad_value(687))), (-1e-6))), A::scale(A::neg(s.ad_value(687)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(238, 687, A::sub(A::scale(assign18020_ad_e32722, 0.5), s.ad_value(687)));

        let assign18030_ad_e32763: A = A::add(A::offset(A::sub(A::mul(s.ad_value(691), s.ad_value(232)), A::neg(s.ad_value(690))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(691), s.ad_value(232)), A::neg(s.ad_value(690))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(691), s.ad_value(232)), A::neg(s.ad_value(690))), (-1e-6))), A::scale(A::neg(s.ad_value(690)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(239, 690, A::sub(A::scale(assign18030_ad_e32763, 0.5), s.ad_value(690)));

        s.store_add_ad_rhs(240, 692, A::mul(s.ad_value(693), s.ad_value(232)));

        s.store_add_ad_rhs(241, 798, A::mul(s.ad_value(800), s.ad_value(232)));

        s.store_add_ad_rhs(242, 799, A::mul(s.ad_value(801), s.ad_value(232)));

        let assign18070_ad_e32819: A = A::add(A::offset(A::sub(A::mul(s.ad_value(872), s.ad_value(232)), A::neg(s.ad_value(871))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(872), s.ad_value(232)), A::neg(s.ad_value(871))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(872), s.ad_value(232)), A::neg(s.ad_value(871))), (-1e-6))), A::scale(A::neg(s.ad_value(871)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(293, 871, A::sub(A::scale(assign18070_ad_e32819, 0.5), s.ad_value(871)));

        s.store_add_ad_rhs(294, 867, A::mul(s.ad_value(868), s.ad_value(232)));

        s.store_add_ad_rhs(295, 869, A::mul(s.ad_value(870), s.ad_value(232)));

        let assign18100_ad_e32870: A = A::add(A::offset(A::sub(A::mul(s.ad_value(722), s.ad_value(232)), A::neg(s.ad_value(721))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(722), s.ad_value(232)), A::neg(s.ad_value(721))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(722), s.ad_value(232)), A::neg(s.ad_value(721))), (-1e-6))), A::scale(A::neg(s.ad_value(721)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(243, 721, A::sub(A::scale(assign18100_ad_e32870, 0.5), s.ad_value(721)));

        let assign18110_ad_e32911: A = A::add(A::offset(A::sub(A::mul(s.ad_value(728), s.ad_value(232)), A::neg(s.ad_value(727))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(728), s.ad_value(232)), A::neg(s.ad_value(727))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(728), s.ad_value(232)), A::neg(s.ad_value(727))), (-1e-6))), A::scale(A::neg(s.ad_value(727)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(244, 727, A::sub(A::scale(assign18110_ad_e32911, 0.5), s.ad_value(727)));

        let assign18120_ad_e32952: A = A::add(A::offset(A::sub(A::mul(s.ad_value(733), s.ad_value(232)), A::neg(s.ad_value(732))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(733), s.ad_value(232)), A::neg(s.ad_value(732))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(733), s.ad_value(232)), A::neg(s.ad_value(732))), (-1e-6))), A::scale(A::neg(s.ad_value(732)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(245, 732, A::sub(A::scale(assign18120_ad_e32952, 0.5), s.ad_value(732)));

        let assign18130_ad_e32993: A = A::add(A::offset(A::sub(A::mul(s.ad_value(738), s.ad_value(232)), A::neg(s.ad_value(737))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(738), s.ad_value(232)), A::neg(s.ad_value(737))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(738), s.ad_value(232)), A::neg(s.ad_value(737))), (-1e-6))), A::scale(A::neg(s.ad_value(737)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(246, 737, A::sub(A::scale(assign18130_ad_e32993, 0.5), s.ad_value(737)));

        let assign18140_ad_e33034: A = A::add(A::offset(A::sub(A::mul(s.ad_value(744), s.ad_value(232)), A::neg(s.ad_value(743))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(744), s.ad_value(232)), A::neg(s.ad_value(743))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(744), s.ad_value(232)), A::neg(s.ad_value(743))), (-1e-6))), A::scale(A::neg(s.ad_value(743)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(247, 743, A::sub(A::scale(assign18140_ad_e33034, 0.5), s.ad_value(743)));

        let assign18150_ad_e33110: A = {
    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(252, 748, assign18150_ad_e33110);

        let assign18160_ad_e33184: A = {
    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(250, 762, assign18160_ad_e33184);

        let assign18170_ad_e33224: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p1437), A::neg(s.ad_value(775))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p1437), A::neg(s.ad_value(775))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p1437), A::neg(s.ad_value(775))), (-1e-6))), A::scale(A::neg(s.ad_value(775)), (4.0 * 1e-6))))), 0.5);
        s.store_add_ad_rhs(259, 775, A::sub(assign18170_ad_e33224, s.ad_value(775)));

        let assign18180_ad_e33265: A = A::scale(A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p1438), A::neg(s.ad_value(776))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p1438), A::neg(s.ad_value(776))), (-1e-6)), A::offset(A::sub(A::scale(s.ad_value(232), p.p1438), A::neg(s.ad_value(776))), (-1e-6))), A::scale(A::neg(s.ad_value(776)), (4.0 * 1e-6))))), 0.5);
        s.store_add_ad_rhs(260, 776, A::sub(assign18180_ad_e33265, s.ad_value(776)));

        let assign18190_ad_e33305: A = A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p1439), A::neg(s.ad_value(777))), (-1e-25)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p1439), A::neg(s.ad_value(777))), (-1e-25)), A::offset(A::sub(A::scale(s.ad_value(232), p.p1439), A::neg(s.ad_value(777))), (-1e-25))), A::scale(A::neg(s.ad_value(777)), (4.0 * 1e-25)))));
        s.store_add_ad_rhs(261, 777, A::sub(A::scale(assign18190_ad_e33305, 0.5), s.ad_value(777)));

        let assign18200_ad_e33346: A = A::add(A::offset(A::sub(A::scale(s.ad_value(232), p.p1440), A::neg(s.ad_value(778))), (-1e-20)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::scale(s.ad_value(232), p.p1440), A::neg(s.ad_value(778))), (-1e-20)), A::offset(A::sub(A::scale(s.ad_value(232), p.p1440), A::neg(s.ad_value(778))), (-1e-20))), A::scale(A::neg(s.ad_value(778)), (4.0 * 1e-20)))));
        s.store_add_ad_rhs(262, 778, A::sub(A::scale(assign18200_ad_e33346, 0.5), s.ad_value(778)));

        s.store_exp_ad(256, A::mul(s.ad_value(861), s.ad_value(418)));

        s.store_mul(462, 463, 256);

        s.v[1390] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1391] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1391] != 0.0)) {
            let assign18250_ad_e33406: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1584))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1584))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1584))), (-1e-6))), (-((4.0 * (-p.p1584)) * 1e-6))))), 0.5), (-p.p1584)), p.p1584);
            s.store_ad(263, &assign18250_ad_e33406);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1391] != 0.0))) {
            let assign18260_ad_e33485: A = {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(263, assign18260_ad_e33485, p.p1584);
        }

        s.v[1392] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1392] != 0.0)) {
            let assign18280_ad_e33535: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1585))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1585))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), (-(-p.p1585))), (-1e-6))), (-((4.0 * (-p.p1585)) * 1e-6))))), 0.5), (-p.p1585)), p.p1585);
            s.store_ad(266, &assign18280_ad_e33535);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1392] != 0.0))) {
            let assign18290_ad_e33614: A = {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1721), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(266, assign18290_ad_e33614, p.p1585);
        }

        s.v[1393] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1393] != 0.0)) {
            let assign18310_ad_e33664: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1586))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1586))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1586))), (-1e-6))), (-((4.0 * (-p.p1586)) * 1e-6))))), 0.5), (-p.p1586)), p.p1586);
            s.store_ad(264, &assign18310_ad_e33664);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1393] != 0.0))) {
            let assign18320_ad_e33743: A = {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(264, assign18320_ad_e33743, p.p1586);
        }

        s.v[1394] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1394] != 0.0)) {
            let assign18340_ad_e33793: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1587))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1587))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), (-(-p.p1587))), (-1e-6))), (-((4.0 * (-p.p1587)) * 1e-6))))), 0.5), (-p.p1587)), p.p1587);
            s.store_ad(267, &assign18340_ad_e33793);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1394] != 0.0))) {
            let assign18350_ad_e33872: A = {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1722), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(267, assign18350_ad_e33872, p.p1587);
        }

        s.v[1395] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1395] != 0.0)) {
            let assign18370_ad_e33922: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1588))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1588))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1588))), (-1e-6))), (-((4.0 * (-p.p1588)) * 1e-6))))), 0.5), (-p.p1588)), p.p1588);
            s.store_ad(268, &assign18370_ad_e33922);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1395] != 0.0))) {
            let assign18380_ad_e34001: A = {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(268, assign18380_ad_e34001, p.p1588);
        }

        s.v[1396] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1396] != 0.0)) {
            let assign18400_ad_e34051: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1589))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1589))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), (-(-p.p1589))), (-1e-6))), (-((4.0 * (-p.p1589)) * 1e-6))))), 0.5), (-p.p1589)), p.p1589);
            s.store_ad(265, &assign18400_ad_e34051);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1396] != 0.0))) {
            let assign18410_ad_e34130: A = {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1723), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(265, assign18410_ad_e34130, p.p1589);
        }

        if (s.v[1390] != 0.0) {
            let assign18420_ad_e34206: A = {
                if (!(((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(269, assign18420_ad_e34206, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18430_ad_e34283: A = {
                if (!(((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(272, assign18430_ad_e34283, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18440_ad_e34360: A = {
                if (!(((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(270, assign18440_ad_e34360, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18450_ad_e34437: A = {
                if (!(((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(273, assign18450_ad_e34437, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18460_ad_e34514: A = {
                if (!(((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(271, assign18460_ad_e34514, 0.01);
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
        if (s.v[1390] != 0.0) {
            let assign18470_ad_e34591: A = {
                if (!(((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(274, assign18470_ad_e34591, 0.01);
        }

        if (s.v[1390] != 0.0) {
            s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));
        }

        if (s.v[1390] != 0.0) {
            s.store_limited_exp_ad(171, A::scale(A::add(s.ad_value(168), A::scale(s.ad_value(418), p.p1727)), 1.0 / (p.p1620)));
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(275, 171, p.p1614);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(276, 171, p.p1616);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(277, 171, p.p1618);
        }

        if (s.v[1390] != 0.0) {
            s.store_limited_exp_ad(171, A::scale(A::add(s.ad_value(168), A::scale(s.ad_value(418), p.p1728)), 1.0 / (p.p1621)));
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(278, 171, p.p1615);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(279, 171, p.p1617);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale(280, 171, p.p1619);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale_ad(281, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1729), s.ad_value(230)), s.ad_value(179))), p.p1630);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale_ad(282, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1730), s.ad_value(230)), s.ad_value(179))), p.p1631);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale_ad(283, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1731), s.ad_value(230)), s.ad_value(179))), p.p1632);
        }

        if (s.v[1390] != 0.0) {
            s.store_scale_ad(284, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1732), s.ad_value(230)), s.ad_value(179))), p.p1633);
        }

        if (s.v[1390] != 0.0) {
            s.store_mul_ad(285, A::scale(A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), p.p1634), A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1733), s.ad_value(230)), s.ad_value(179))));
        }

        if (s.v[1390] != 0.0) {
            s.store_mul_ad(286, A::scale(A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), p.p1635), A::limited_exp(A::div(A::mul(A::scale(s.ad_value(147), p.p1734), s.ad_value(230)), s.ad_value(179))));
        }

        if (s.v[1390] != 0.0) {
            let assign18630_ad_e34840: A = {
                if (!(((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1735), 1.0), p.p1637), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1735), 1.0), p.p1637), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1735), 1.0), p.p1637), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1735), 1.0), p.p1637), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(287, assign18630_ad_e34840, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18640_ad_e34929: A = {
                if (!(((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1736), 1.0), p.p1638), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1736), 1.0), p.p1638), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1736), 1.0), p.p1638), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1736), 1.0), p.p1638), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(288, assign18640_ad_e34929, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18650_ad_e35018: A = {
                if (!(((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1737), 1.0), p.p1639), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1737), 1.0), p.p1639), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1737), 1.0), p.p1639), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1737), 1.0), p.p1639), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(289, assign18650_ad_e35018, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18660_ad_e35107: A = {
                if (!(((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1738), 1.0), p.p1640), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1738), 1.0), p.p1640), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1738), 1.0), p.p1640), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1738), 1.0), p.p1640), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(290, assign18660_ad_e35107, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18670_ad_e35196: A = {
                if (!(((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1739), 1.0), p.p1641), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1739), 1.0), p.p1641), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1739), 1.0), p.p1641), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1739), 1.0), p.p1641), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(291, assign18670_ad_e35196, 0.01);
        }

        if (s.v[1390] != 0.0) {
            let assign18680_ad_e35285: A = {
                if (!(((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1740), 1.0), p.p1642), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1740), 1.0), p.p1642), (-0.01)), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1740), 1.0), p.p1642), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(s.ad_value(230), p.p1740), 1.0), p.p1642), (-0.01)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(292, assign18680_ad_e35285, 0.01);
        }

        s.v[1397] = if !(if self.param_given[1106] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        s.v[1398] = if (p.p145 > 0.0) { 1.0 } else { 0.0 };

        s.v[1399] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1397] != 0.0) && (s.v[1398] != 0.0)) && (s.v[1399] != 0.0)) {
            let assign18720_ad_e35490: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    let assign18720_ad_e35416: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })));
                    let assign18720_ad_e35424: A = A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sqrt(A::offset(assign18720_ad_e35416, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18720_ad_e35424, 0.5)
                } else {
                    let assign18720_ad_e35489: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18720_ad_e35487: A = A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                                if (!((p.p145 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p145 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            })));
                            assign18720_ad_e35487
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18720_ad_e35489
                }
            };
            let assign18720_ad_e35683: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    let assign18720_ad_e35609: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })));
                    let assign18720_ad_e35617: A = A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sqrt(A::offset(assign18720_ad_e35609, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18720_ad_e35617, 0.5)
                } else {
                    let assign18720_ad_e35682: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18720_ad_e35680: A = A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            })));
                            assign18720_ad_e35680
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18720_ad_e35682
                }
            };
            s.store_mul_ad_rhs(479, 114, A::sub(assign18720_ad_e35490, A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(114), A::sub(A::scale(s.ad_value(146), 0.5), assign18720_ad_e35683)))));
        }

        if (((s.v[1397] != 0.0) && (s.v[1398] != 0.0)) && (!(s.v[1399] != 0.0))) {
            let assign18730_ad_e35859: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18730_ad_e35793: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))));
                    A::scale(A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sqrt(A::offset(assign18730_ad_e35793, ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    let assign18730_ad_e35858: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18730_ad_e35858
                }
            };
            let assign18730_ad_e36028: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18730_ad_e35962: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))));
                    A::scale(A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sqrt(A::offset(assign18730_ad_e35962, ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    let assign18730_ad_e36027: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18730_ad_e36027
                }
            };
            s.store_mul_ad_rhs(479, 114, A::sub(assign18730_ad_e35859, A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(114), A::sub(A::scale(s.ad_value(146), 0.5), assign18730_ad_e36028)))));
        }

        s.v[1400] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1397] != 0.0) && (!(s.v[1398] != 0.0))) && (s.v[1400] != 0.0)) {
            let assign18750_ad_e36241: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    let assign18750_ad_e36167: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })));
                    let assign18750_ad_e36175: A = A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    })), A::sqrt(A::offset(assign18750_ad_e36167, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18750_ad_e36175, 0.5)
                } else {
                    let assign18750_ad_e36240: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18750_ad_e36238: A = A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            })));
                            assign18750_ad_e36238
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18750_ad_e36240
                }
            };
            s.store_mul_ad_rhs(479, 114, A::sub(s.ad_value(641), A::sub(A::offset(A::scale(s.ad_value(146), 0.5), p.p104), A::mul(s.ad_value(114), A::sub(A::scale(s.ad_value(146), 0.5), assign18750_ad_e36241)))));
        }

        if (((s.v[1397] != 0.0) && (!(s.v[1398] != 0.0))) && (!(s.v[1400] != 0.0))) {
            let assign18760_ad_e36428: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18760_ad_e36362: A = A::mul(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))));
                    A::scale(A::add(A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))), A::sqrt(A::offset(assign18760_ad_e36362, ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    let assign18760_ad_e36427: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub(A::scale(s.ad_value(146), 0.5), A::mul(s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)))))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18760_ad_e36427
                }
            };
            s.store_mul_ad_rhs(479, 114, A::sub(s.ad_value(641), A::sub(A::offset(A::scale(s.ad_value(146), 0.5), p.p104), A::mul(s.ad_value(114), A::sub(A::scale(s.ad_value(146), 0.5), assign18760_ad_e36428)))));
        }

        if (!(s.v[1397] != 0.0)) {
            s.store_scalar(479, p.p1106);
        }

        s.v[1401] = if !(if self.param_given[1107] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1401] != 0.0) {
            s.copy_ad(518, 479);
        }

        if (!(s.v[1401] != 0.0)) {
            s.store_scalar(518, p.p1107);
        }

        s.v[1402] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1402] != 0.0) {
            s.store_mul_ad_rhs(166, 179, {
                if (!((s.v[640] / s.v[141]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[141]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(141)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1402] != 0.0) {
            s.store_scale_ad(166, A::add(s.ad_value(166), A::sqrt(A::offset(A::mul(s.ad_value(166), s.ad_value(166)), ((0.25 * 1e-10) * 1e-10)))), 0.5);
        }

        if (s.v[1402] != 0.0) {
            let assign18840_ad_e36537: A = {
                if (!(((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38) {
                            A::ln(A::div(A::scale(s.ad_value(640), p.p97), A::square(s.ad_value(141))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(352, 179, assign18840_ad_e36537);
        }

        if (!(s.v[1402] != 0.0)) {
            s.store_mul_ad_rhs(166, 179, A::sub({
                if (!(s.v[640] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[640] > 1e-38) {
                            A::ln(s.ad_value(640))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(142)));
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
        if (!(s.v[1402] != 0.0)) {
            s.store_scale_ad(166, A::add(s.ad_value(166), A::sqrt(A::offset(A::mul(s.ad_value(166), s.ad_value(166)), ((0.25 * 1e-10) * 1e-10)))), 0.5);
        }

        if (!(s.v[1402] != 0.0)) {
            s.store_mul_ad_rhs(352, 179, A::sub({
                if (!((s.v[640] * p.p97) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p.p97) > 1e-38) {
                            A::ln(A::scale(s.ad_value(640), p.p97))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::scale(s.ad_value(142), 2.0)));
        }

        s.store_mul_ad_rhs(167, 114, A::sub(s.ad_value(641), A::offset({
            if (p.p60 == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p.p104)));

        s.store_scale(407, 322, 0.5);

        s.v[408] = 0.5;

        s.v[1403] = if (p.p60 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[1403] != 0.0) {
            s.store_scale(407, 322, 0.333333333);
        }

        if (s.v[1403] != 0.0) {
            s.store_scalar(408, 0.333333333);
        }

        s.v[1404] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1404] != 0.0) {
            s.store_add_ad(537, A::add(A::scale(s.ad_value(275), p.p11), A::scale(s.ad_value(276), p.p13)), A::scale(s.ad_value(277), (p.p3 * s.v[115])));
        }

        s.v[1405] = if (s.v[537] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_scale(539, 179, p.p1620);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_scale_ad(547, A::limited_exp(A::div_from_scalar((-p.p1626), s.ad_value(539))), p.p1628);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1622, s.ad_value(537)), 10.0);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_sub_ad_lhs(226, A::offset(s.ad_value(170), 1.0), 547);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            let assign19010_ad_e36758: A = {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38) {
                            A::ln(A::scale(A::add(s.ad_value(226), A::sqrt(A::add(A::square(s.ad_value(226)), A::scale(s.ad_value(547), 4.0)))), 0.5))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(546, 539, assign19010_ad_e36758);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_limited_exp_ad(168, A::div(s.ad_value(546), s.ad_value(539)));
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_rhs(545, 537, A::offset(A::add(A::sub(s.ad_value(168), A::div(s.ad_value(547), s.ad_value(168))), s.ad_value(547)), (-1.0)));
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_div_ad_lhs(544, A::mul(s.ad_value(537), A::add(s.ad_value(168), A::div(s.ad_value(547), s.ad_value(168)))), 539);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            let assign19050_ad_e36863: A = {
                if (!(((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(170, assign19050_ad_e36863, 10.0);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_sub_from_scalar_ad(543, (-p.p1626), A::mul(s.ad_value(539), {
                if (!(((s.v[170] - 1.0) / p.p1628) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1628) > 1e-38) {
                            A::ln(A::scale(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1628)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_scale_ad(169, A::limited_exp(A::div(A::neg(A::offset(s.ad_value(543), p.p1626)), s.ad_value(539))), p.p1628);
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_mul_ad_rhs(542, 537, A::offset(s.ad_value(169), 1.0));
        }

        if ((s.v[1404] != 0.0) && (s.v[1405] != 0.0)) {
            s.store_div_ad_lhs(541, A::mul(A::neg(s.ad_value(537)), s.ad_value(169)), 539);
        }

        if (s.v[1404] != 0.0) {
            s.store_add_ad(538, A::add(A::scale(s.ad_value(278), p.p12), A::scale(s.ad_value(279), p.p14)), A::scale(s.ad_value(280), (p.p3 * s.v[115])));
        }

        s.v[1406] = if (s.v[538] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_scale(540, 179, p.p1621);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_scale_ad(554, A::limited_exp(A::div_from_scalar((-p.p1627), s.ad_value(540))), p.p1629);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1623, s.ad_value(538)), 10.0);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_sub_ad_lhs(226, A::offset(s.ad_value(170), 1.0), 554);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            let assign19160_ad_e37048: A = {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38) {
                            A::ln(A::scale(A::add(s.ad_value(226), A::sqrt(A::add(A::square(s.ad_value(226)), A::scale(s.ad_value(554), 4.0)))), 0.5))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(553, 540, assign19160_ad_e37048);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_limited_exp_ad(168, A::div(s.ad_value(553), s.ad_value(540)));
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_rhs(552, 538, A::offset(A::add(A::sub(s.ad_value(168), A::div(s.ad_value(554), s.ad_value(168))), s.ad_value(554)), (-1.0)));
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_div_ad_lhs(551, A::mul(s.ad_value(538), A::add(s.ad_value(168), A::div(s.ad_value(554), s.ad_value(168)))), 540);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            let assign19200_ad_e37153: A = {
                if (!(((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(170, assign19200_ad_e37153, 10.0);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_sub_from_scalar_ad(550, (-p.p1627), A::mul(s.ad_value(540), {
                if (!(((s.v[170] - 1.0) / p.p1629) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1629) > 1e-38) {
                            A::ln(A::scale(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1629)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_scale_ad(169, A::limited_exp(A::div(A::neg(A::offset(s.ad_value(550), p.p1627)), s.ad_value(540))), p.p1629);
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_mul_ad_rhs(549, 538, A::offset(s.ad_value(169), 1.0));
        }

        if ((s.v[1404] != 0.0) && (s.v[1406] != 0.0)) {
            s.store_div_ad_lhs(548, A::mul(A::neg(s.ad_value(538)), s.ad_value(169)), 540);
        }

        if (s.v[1404] != 0.0) {
            s.store_scale(523, 263, p.p11);
        }

        if (s.v[1404] != 0.0) {
            s.store_scale(524, 264, p.p13);
        }

        if (s.v[1404] != 0.0) {
            s.store_scaled_mul(525, 268, 158, s.v[115]);
        }

        if (s.v[1404] != 0.0) {
            s.store_scale(526, 266, p.p12);
        }

        if (s.v[1404] != 0.0) {
            s.store_scale(527, 267, p.p14);
        }

        if (s.v[1404] != 0.0) {
            s.store_scaled_mul(528, 265, 158, s.v[115]);
        }

        s.v[1407] = if (p.p1602 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1407] != 0.0)) {
            s.store_scale(557, 269, (1.0 - (((1.0 / p.p1602)) as f64).powf((1.0 / p.p1596))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1407] != 0.0)) {
            s.store_div_ad(558, A::scale(s.ad_value(269), (p.p1602 * (p.p1608 * 1.0 / (p.p1596)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(557), s.ad_value(269))), (-(1.0 + p.p1596))));
        }

        s.v[1408] = if (p.p1604 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1408] != 0.0)) {
            s.store_scale(559, 270, (1.0 - (((1.0 / p.p1604)) as f64).powf((1.0 / p.p1598))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1408] != 0.0)) {
            s.store_div_ad(560, A::scale(s.ad_value(270), (p.p1604 * (p.p1610 * 1.0 / (p.p1598)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(559), s.ad_value(270))), (-(1.0 + p.p1598))));
        }

        s.v[1409] = if (p.p1606 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1409] != 0.0)) {
            s.store_scale(561, 271, (1.0 - (((1.0 / p.p1606)) as f64).powf((1.0 / p.p1600))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1409] != 0.0)) {
            s.store_div_ad(562, A::scale(s.ad_value(271), (p.p1606 * (p.p1612 * 1.0 / (p.p1600)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(271))), (-(1.0 + p.p1600))));
        }

        s.v[1410] = if (p.p1603 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1410] != 0.0)) {
            s.store_scale(563, 272, (1.0 - (((1.0 / p.p1603)) as f64).powf((1.0 / p.p1597))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1410] != 0.0)) {
            s.store_div_ad(564, A::scale(s.ad_value(272), (p.p1603 * (p.p1609 * 1.0 / (p.p1597)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(563), s.ad_value(272))), (-(1.0 + p.p1597))));
        }

        s.v[1411] = if (p.p1605 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_scale(565, 273, (1.0 - (((1.0 / p.p1605)) as f64).powf((1.0 / p.p1599))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1411] != 0.0)) {
            s.store_div_ad(566, A::scale(s.ad_value(273), (p.p1605 * (p.p1611 * 1.0 / (p.p1599)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(565), s.ad_value(273))), (-(1.0 + p.p1599))));
        }

        s.v[1412] = if (p.p1607 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1404] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_scale(567, 274, (1.0 - (((1.0 / p.p1607)) as f64).powf((1.0 / p.p1601))));
        }

        if ((s.v[1404] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_div_ad(568, A::scale(s.ad_value(274), (p.p1607 * (p.p1613 * 1.0 / (p.p1601)))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(567), s.ad_value(274))), (-(1.0 + p.p1601))));
        }

        s.store_limited_exp_ad(555, A::div(A::div(A::mul(s.ad_value(146), s.ad_value(230)), s.ad_value(179)), s.ad_value(864)));

        s.store_mul_ad_rhs(134, 114, A::voltage(ctx, &nodes, Some(11), Some(6)));

        s.store_mul_ad_rhs(135, 114, A::voltage(ctx, &nodes, Some(5), Some(6)));

        s.store_mul_ad_rhs(136, 114, A::voltage(ctx, &nodes, Some(11), Some(5)));

        s.store_mul_ad_rhs(521, 114, A::voltage(ctx, &nodes, Some(3), Some(6)));

        s.store_mul_ad_rhs(522, 114, A::voltage(ctx, &nodes, Some(3), Some(5)));

        s.store_mul_ad_rhs(497, 114, A::voltage(ctx, &nodes, Some(11), Some(3)));

        s.v[1413] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        if (s.v[1413] != 0.0) {
            s.store_mul_ad_rhs(132, 114, A::voltage(ctx, &nodes, Some(10), Some(5)));
        }

        if (s.v[1413] != 0.0) {
            s.store_mul_ad_rhs(133, 114, A::voltage(ctx, &nodes, Some(10), Some(6)));
        }

        if (!(s.v[1413] != 0.0)) {
            s.store_mul_ad_rhs(132, 114, A::voltage(ctx, &nodes, Some(14), Some(5)));
        }

        if (!(s.v[1413] != 0.0)) {
            s.store_mul_ad_rhs(133, 114, A::voltage(ctx, &nodes, Some(13), Some(6)));
        }

        s.v[128] = 1.0;

        s.v[1414] = if (s.v[135] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1414] != 0.0) {
            s.store_scalar(128, (-1.0));
        }

        if (s.v[1414] != 0.0) {
            s.store_sub(125, 134, 135);
        }

        if (s.v[1414] != 0.0) {
            s.store_scale(126, 135, (-1.0));
        }

        if (s.v[1414] != 0.0) {
            s.copy_ad(367, 522);
        }

        if (!(s.v[1414] != 0.0)) {
            s.copy_ad(125, 134);
        }

        if (!(s.v[1414] != 0.0)) {
            s.copy_ad(126, 135);
        }

        if (!(s.v[1414] != 0.0)) {
            s.copy_ad(367, 521);
        }

        s.store_sub(347, 125, 167);

        s.store_offset_ad(127, A::sqrt(A::offset(A::square(s.ad_value(126)), 0.01)), (-0.1));

        s.v[1415] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1415] != 0.0) {
            s.store_sub_ad_rhs(368, 367, A::scale(A::sub(s.ad_value(126), s.ad_value(127)), 0.5));
        }

        if (s.v[1415] != 0.0) {
            s.store_scale(369, 689, 0.95);
        }

        if (s.v[1415] != 0.0) {
            s.store_offset_ad(170, A::sub(s.ad_value(369), s.ad_value(368)), (-0.001));
        }

        if (s.v[1415] != 0.0) {
            s.store_sub_ad_rhs(370, 369, A::scale(A::add(s.ad_value(170), A::sqrt(A::add(A::square(s.ad_value(170)), A::scale(s.ad_value(369), 0.004)))), 0.5));
        }

        s.store_tanh_ad(168, A::div(A::scale(s.ad_value(135), 0.6), s.ad_value(179)));

        s.store_offset_scaled(186, 168, 0.5, 0.5);

        s.store_sub_from_scalar(187, 1.0, 186);

        s.v[1416] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1416] != 0.0) {
            s.store_add_ad(664, A::mul(s.ad_value(665), s.ad_value(187)), A::mul(s.ad_value(663), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(676, A::mul(s.ad_value(298), s.ad_value(187)), A::mul(s.ad_value(296), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(427, A::mul(s.ad_value(715), s.ad_value(187)), A::mul(s.ad_value(714), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(718, A::mul(s.ad_value(717), s.ad_value(187)), A::mul(s.ad_value(716), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(423, A::mul(s.ad_value(338), s.ad_value(187)), A::mul(s.ad_value(337), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(424, A::mul(s.ad_value(258), s.ad_value(187)), A::mul(s.ad_value(257), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(422, A::mul(s.ad_value(335), s.ad_value(187)), A::mul(s.ad_value(334), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(425, A::mul(s.ad_value(300), s.ad_value(187)), A::mul(s.ad_value(299), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(426, A::mul(s.ad_value(302), s.ad_value(187)), A::mul(s.ad_value(301), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(795, A::mul(s.ad_value(796), s.ad_value(187)), A::mul(s.ad_value(797), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(428, A::mul(s.ad_value(333), s.ad_value(187)), A::mul(s.ad_value(332), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(659, A::mul(s.ad_value(658), s.ad_value(187)), A::mul(s.ad_value(660), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(805, A::mul(s.ad_value(806), s.ad_value(187)), A::mul(s.ad_value(804), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(669, A::mul(s.ad_value(668), s.ad_value(187)), A::mul(s.ad_value(666), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(416, A::mul(s.ad_value(417), s.ad_value(187)), A::mul(s.ad_value(413), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(819, A::mul(s.ad_value(305), s.ad_value(187)), A::mul(s.ad_value(303), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(820, A::mul(s.ad_value(320), s.ad_value(187)), A::mul(s.ad_value(318), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(821, A::mul(s.ad_value(316), s.ad_value(187)), A::mul(s.ad_value(314), s.ad_value(186)));
        }

        if (s.v[1416] != 0.0) {
            s.store_add_ad(822, A::mul(s.ad_value(816), s.ad_value(187)), A::mul(s.ad_value(323), s.ad_value(186)));
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(664, 663);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(676, 296);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(427, 714);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(718, 716);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(423, 337);
        }

    }
}
