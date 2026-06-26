#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
    ) {
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
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.v[596] = 0.0;

        s.v[1052] = 0.0;

        s.b[1057] = (p.p60 == 1.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if s.b[1057] {
            s.store_scalar(114, 1.0);
        }

        if (!s.b[1057]) {
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

        s.b[1058] = (s.v[149] <= 0.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if s.b[1058] {
            s.store_scalar(149, p.p0);
        }

        s.store_powf(168, 149, (-p.p84));

        s.store_offset_scaled(150, 168, p.p83, s.v[877]);

        s.store_offset_scaled_ad(151, A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83, s.v[877]);

        s.store_offset_scaled(152, 168, p.p88, p.p85);

        s.store_sub_scaled_inputs(153, 149, 1.0, 150, 2.0);

        s.store_sub_scaled_ad_lhs(155, A::offset(s.ad_value(149), s.v[878]), 151, 2.0);

        s.store_sub_scaled_inputs(156, 149, 1.0, 152, 2.0);

        s.store_offset(157, 156, (-p.p86));

        s.b[1059] = (s.v[153] <= 0.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if s.b[1059] {
            s.copy_ad(153, 149);
        }

        s.b[1061] = (s.v[155] <= 0.0);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if s.b[1061] {
            s.copy_ad(155, 149);
        }

        s.b[1063] = (s.v[156] <= 0.0);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if s.b[1063] {
            s.copy_ad(156, 149);
        }

        s.b[1065] = (p.p61 != 0.0);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        s.b[1066] = (s.v[157] <= 0.0);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1065] && s.b[1066]) {
            s.copy_ad(157, 149);
        }

        s.b[1068] = (p.p62 == 5.0);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if s.b[1068] {
            s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));
            s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));
        }

        if (!s.b[1068]) {
            s.store_scalar(879, 0.0);
            s.store_scalar(880, 0.0);
        }

        s.store_offset(161, 879, p.p43);

        s.store_add(162, 161, 880);

        s.b[1069] = (p.p62 == 5.0);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        s.b[1070] = (s.v[162] <= 0.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1069] && s.b[1070]) {
            s.store_scalar(162, p.p43);
        }

        s.v[115] = (p.p5 * p.p59);

        s.store_div_from_scalar(635, 1e-6, 155);

        s.v[636] = (1.0 / p.p5);

        s.store_div_from_scalar_scaled_input(637, 1e-6, 155, p.p5);

        s.b[1072] = (p.p62 == 5.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if s.b[1072] {
            s.store_div_from_scalar(638, 1e-6, 162);
            s.store_div_from_scalar_mul_ad(639, 1e-12, s.ad_value(162), s.ad_value(155));
        }

        if (!s.b[1072]) {
            s.store_scalar(638, 0.0);
            s.store_scalar(639, 0.0);
        }

        s.store_add_scaled_ad_lhs(640, A::add_scaled_inputs3_offset(s.ad_value(635), p.p134, s.ad_value(637), p.p136, s.ad_value(638), 0.0, ((p.p133) + ((s.v[636] * p.p135)))), 639, 0.0);

        s.b[1073] = (p.p95 != 0.0);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if s.b[1073] {
            s.store_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1074] = (s.v[640] <= 0.0);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if s.b[1074] {
            s.store_scalar(640, 1e22);
        }

        s.b[1076] = (p.p62 == 0.0);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        s.b[1077] = (p.p62 == 1.0);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        s.b[1078] = (p.p62 == 2.0);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        s.b[1079] = (p.p62 == 3.0);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        s.b[1080] = (p.p62 == 4.0);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        s.b[1081] = (p.p62 == 5.0);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        s.b[1082] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1076] && s.b[1082]) {
            s.store_scalar(895, (2.0 * p.p92));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (s.b[1076] && (!s.b[1082])) {
            s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1083] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if ((s.b[1077] && (!s.b[1076])) && s.b[1083]) {
            s.store_scalar(895, ((2.0 * p.p92) + p.p3));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1077] && (!s.b[1076])) && (!s.b[1083])) {
            s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1084] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && s.b[1084]) {
            s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && (!s.b[1084])) {
            s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        if (s.b[1078] && (!(s.b[1076] || s.b[1077]))) {
            s.store_scalar(896, p.p1803);
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(895, (3.141592653589793 * p.p2));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));
            s.store_scalar(896, p.p2);
        }

        if (s.b[1080] && (!(((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]))) {
            s.store_scalar(895, p.p1801);
            s.store_scalar(893, p.p1800);
            s.store_scalar(894, p.p1799);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_offset_scaled(954, 161, 2.0, ((((p.p40) * (2.0))) + (p.p44)));
            s.store_offset_scaled(948, 161, p.p40, p.p45);
            s.copy_ad(895, 954);
            s.copy_ad(894, 948);
        }

        s.b[1085] = (p.p56 > 1.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1085]) {
            s.store_offset_scaled(955, 161, 2.0, ((((p.p40) * (2.0))) + (p.p46)));
            s.store_offset_scaled(949, 161, p.p40, p.p47);
            s.store_add(895, 954, 955);
            s.store_add(894, 948, 949);
        }

        s.b[1086] = (p.p56 > 2.0);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1086]) {
            s.store_offset_scaled(956, 161, 2.0, ((((p.p40) * (2.0))) + (p.p48)));
            s.store_offset_scaled(950, 161, p.p40, p.p49);
            s.store_ad_value(895, A::add_scaled_inputs3(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0));
            s.store_ad_value(894, A::add_scaled_inputs3(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0));
        }

        s.b[1087] = (p.p56 > 3.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1087]) {
            s.store_offset_scaled(957, 161, 2.0, ((((p.p40) * (2.0))) + (p.p50)));
            s.store_offset_scaled(951, 161, p.p40, p.p51);
            s.store_add_ad_lhs(895, A::add_scaled_inputs3(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0), 957);
            s.store_add_ad_lhs(894, A::add_scaled_inputs3(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0), 951);
        }

        s.b[1088] = (p.p56 > 4.0);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1088]) {
            s.store_offset_scaled(958, 161, 2.0, ((((p.p40) * (2.0))) + (p.p52)));
            s.store_offset_scaled(952, 161, p.p40, p.p53);
            s.store_ad_value(895, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0), 1.0, s.ad_value(957), 1.0, s.ad_value(958), 1.0));
            s.store_ad_value(894, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0), 1.0, s.ad_value(951), 1.0, s.ad_value(952), 1.0));
        }

        s.b[1089] = (p.p56 > 5.0);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1089]) {
            s.store_offset_scaled(959, 161, 2.0, ((((p.p40) * (2.0))) + (p.p54)));
            s.store_offset_scaled(953, 161, p.p40, p.p55);
            s.store_add_ad_lhs(895, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0), 1.0, s.ad_value(957), 1.0, s.ad_value(958), 1.0), 959);
            s.store_add_ad_lhs(894, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0), 1.0, s.ad_value(951), 1.0, s.ad_value(952), 1.0), 953);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_scalar(896, p.p43);
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        s.store_ad_value(898, A::div_scaled_inputs(s.ad_value(893), 2.0, A::div_scaled_inputs(A::square(s.ad_value(895)), s.v[143], s.ad_value(894), 1.0), 1.0));

        s.store_ad_value(903, A::div_scaled_product(s.ad_value(640), s.ad_value(894), (-1.60219e-19), s.ad_value(893), 1.0));

        s.store_div(163, 893, 895);

        s.b[1090] = (p.p61 != 0.0);
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if s.b[1090] {
            s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));
        }

        s.store_offset(158, 895, (-p.p93));

        s.store_offset(159, 895, (-p.p94));

        s.b[1091] = (p.p62 == 5.0);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if s.b[1091] {
            s.store_offset(160, 158, (-((2.0 * p.p56) * p.p87)));
        }

        if (!s.b[1091]) {
            s.copy_ad(160, 158);
        }

        s.b[1092] = (p.p62 == 5.0);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        s.b[1093] = (p.p61 != 0.0);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        s.b[1094] = (s.v[160] <= 0.0);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if ((s.b[1092] && s.b[1093]) && s.b[1094]) {
            s.copy_ad(160, 895);
        }

        s.v[446] = p.p1085;

        s.v[577] = p.p1680;

        s.store_add_scaled_ad_lhs(641, A::add_scaled_inputs3_offset(s.ad_value(635), p.p138, s.ad_value(637), p.p140, s.ad_value(638), p.p141, ((p.p137) + ((s.v[636] * p.p139)))), 639, p.p142);

        s.store_add_scaled_ad_lhs(666, A::add_scaled_inputs3_offset(s.ad_value(635), p.p189, s.ad_value(637), p.p191, s.ad_value(638), p.p192, ((p.p188) + ((s.v[636] * p.p190)))), 639, p.p193);

        s.store_add_scaled_ad_lhs(662, A::add_scaled_inputs3_offset(s.ad_value(635), p.p201, s.ad_value(637), p.p203, s.ad_value(638), p.p204, ((p.p200) + ((s.v[636] * p.p202)))), 639, p.p205);

        s.store_add_scaled_ad_lhs(663, A::add_scaled_inputs3_offset(s.ad_value(635), p.p207, s.ad_value(637), p.p209, s.ad_value(638), p.p210, ((p.p206) + ((s.v[636] * p.p208)))), 639, p.p211);

        s.store_add_scaled_ad_lhs(667, A::add_scaled_inputs3_offset(s.ad_value(635), p.p219, s.ad_value(637), p.p221, s.ad_value(638), p.p222, ((p.p218) + ((s.v[636] * p.p220)))), 639, p.p223);

        s.store_add_scaled_ad_lhs(670, A::add_scaled_inputs3_offset(s.ad_value(635), p.p225, s.ad_value(637), p.p227, s.ad_value(638), p.p228, ((p.p224) + ((s.v[636] * p.p226)))), 639, p.p229);

        s.store_add_scaled_ad_lhs(671, A::add_scaled_inputs3_offset(s.ad_value(635), p.p231, s.ad_value(637), p.p233, s.ad_value(638), p.p234, ((p.p230) + ((s.v[636] * p.p232)))), 639, p.p235);

        s.store_add_scaled_ad_lhs(672, A::add_scaled_inputs3_offset(s.ad_value(635), p.p237, s.ad_value(637), p.p239, s.ad_value(638), p.p240, ((p.p236) + ((s.v[636] * p.p238)))), 639, p.p241);

        s.store_add_scaled_ad_lhs(673, A::add_scaled_inputs3_offset(s.ad_value(635), p.p243, s.ad_value(637), p.p245, s.ad_value(638), p.p246, ((p.p242) + ((s.v[636] * p.p244)))), 639, p.p247);

        s.store_add_scaled_ad_lhs(674, A::add_scaled_inputs3_offset(s.ad_value(635), p.p249, s.ad_value(637), p.p251, s.ad_value(638), p.p252, ((p.p248) + ((s.v[636] * p.p250)))), 639, p.p253);

        s.store_add_scaled_ad_lhs(678, A::add_scaled_inputs3_offset(s.ad_value(635), p.p267, s.ad_value(637), p.p269, s.ad_value(638), p.p270, ((p.p266) + ((s.v[636] * p.p268)))), 639, p.p271);

        s.store_add_scaled_ad_lhs(802, A::add_scaled_inputs3_offset(s.ad_value(635), p.p273, s.ad_value(637), p.p275, s.ad_value(638), p.p276, ((p.p272) + ((s.v[636] * p.p274)))), 639, p.p277);

        s.store_add_scaled_ad_lhs(803, A::add_scaled_inputs3_offset(s.ad_value(635), p.p279, s.ad_value(637), p.p281, s.ad_value(638), p.p282, ((p.p278) + ((s.v[636] * p.p280)))), 639, p.p283);

        s.store_add_scaled_ad_lhs(804, A::add_scaled_inputs3_offset(s.ad_value(635), p.p285, s.ad_value(637), p.p287, s.ad_value(638), p.p288, ((p.p284) + ((s.v[636] * p.p286)))), 639, p.p289);

        s.store_add_scaled_ad_lhs(683, A::add_scaled_inputs3_offset(s.ad_value(635), p.p297, s.ad_value(637), p.p299, s.ad_value(638), p.p300, ((p.p296) + ((s.v[636] * p.p298)))), 639, p.p301);

        s.store_add_scaled_ad_lhs(684, A::add_scaled_inputs3_offset(s.ad_value(635), p.p303, s.ad_value(637), p.p305, s.ad_value(638), p.p306, ((p.p302) + ((s.v[636] * p.p304)))), 639, p.p307);

        s.store_add_scaled_ad_lhs(685, A::add_scaled_inputs3_offset(s.ad_value(635), p.p309, s.ad_value(637), p.p311, s.ad_value(638), p.p312, ((p.p308) + ((s.v[636] * p.p310)))), 639, p.p313);

        s.store_add_scaled_ad_lhs(686, A::add_scaled_inputs3_offset(s.ad_value(635), p.p315, s.ad_value(637), p.p317, s.ad_value(638), p.p318, ((p.p314) + ((s.v[636] * p.p316)))), 639, p.p319);

        s.store_add_scaled_ad_lhs(687, A::add_scaled_inputs3_offset(s.ad_value(635), p.p321, s.ad_value(637), p.p323, s.ad_value(638), p.p324, ((p.p320) + ((s.v[636] * p.p322)))), 639, p.p325);

        s.store_add_scaled_ad_lhs(688, A::add_scaled_inputs3_offset(s.ad_value(635), p.p327, s.ad_value(637), p.p329, s.ad_value(638), p.p330, ((p.p326) + ((s.v[636] * p.p328)))), 639, p.p331);

        s.store_add_scaled_ad_lhs(867, A::add_scaled_inputs3_offset(s.ad_value(635), p.p333, s.ad_value(637), p.p335, s.ad_value(638), p.p336, ((p.p332) + ((s.v[636] * p.p334)))), 639, p.p337);

        s.store_add_scaled_ad_lhs(868, A::add_scaled_inputs3_offset(s.ad_value(635), p.p339, s.ad_value(637), p.p341, s.ad_value(638), p.p342, ((p.p338) + ((s.v[636] * p.p340)))), 639, p.p343);

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_ad_lhs(869, A::add_scaled_inputs3_offset(s.ad_value(635), p.p345, s.ad_value(637), p.p347, s.ad_value(638), p.p348, ((p.p344) + ((s.v[636] * p.p346)))), 639, p.p349);

        s.store_add_scaled_ad_lhs(870, A::add_scaled_inputs3_offset(s.ad_value(635), p.p351, s.ad_value(637), p.p353, s.ad_value(638), p.p354, ((p.p350) + ((s.v[636] * p.p352)))), 639, p.p355);

        s.store_add_scaled_ad_lhs(654, A::add_scaled_inputs3_offset(s.ad_value(635), p.p404, s.ad_value(637), p.p406, s.ad_value(638), p.p407, ((p.p403) + ((s.v[636] * p.p405)))), 639, p.p408);

        s.store_add_scaled_ad_lhs(655, A::add_scaled_inputs3_offset(s.ad_value(635), p.p410, s.ad_value(637), p.p412, s.ad_value(638), p.p413, ((p.p409) + ((s.v[636] * p.p411)))), 639, p.p414);

        s.store_add_scaled_ad_lhs(656, A::add_scaled_inputs3_offset(s.ad_value(635), p.p416, s.ad_value(637), p.p418, s.ad_value(638), p.p419, ((p.p415) + ((s.v[636] * p.p417)))), 639, p.p420);

        s.store_add_scaled_ad_lhs(661, A::add_scaled_inputs3_offset(s.ad_value(635), p.p422, s.ad_value(637), p.p424, s.ad_value(638), p.p425, ((p.p421) + ((s.v[636] * p.p423)))), 639, p.p426);

        s.store_add_scaled_ad_lhs(679, A::add_scaled_inputs3_offset(s.ad_value(635), p.p456, s.ad_value(637), p.p458, s.ad_value(638), p.p459, ((p.p455) + ((s.v[636] * p.p457)))), 639, p.p460);

        s.store_add_scaled_ad_lhs(698, A::add_scaled_inputs3_offset(s.ad_value(635), p.p468, s.ad_value(637), p.p470, s.ad_value(638), p.p471, ((p.p467) + ((s.v[636] * p.p469)))), 639, p.p472);

        s.store_add_scaled_ad_lhs(702, A::add_scaled_inputs3_offset(s.ad_value(635), p.p507, s.ad_value(637), p.p509, s.ad_value(638), p.p510, ((p.p506) + ((s.v[636] * p.p508)))), 639, p.p511);

        s.store_add_scaled_ad_lhs(881, A::add_scaled_inputs3_offset(s.ad_value(635), p.p513, s.ad_value(637), p.p515, s.ad_value(638), p.p516, ((p.p512) + ((s.v[636] * p.p514)))), 639, p.p517);

        s.store_add_scaled_ad_lhs(694, A::add_scaled_inputs3_offset(s.ad_value(635), p.p480, s.ad_value(637), p.p482, s.ad_value(638), p.p483, ((p.p479) + ((s.v[636] * p.p481)))), 639, p.p484);

        s.store_add_scaled_ad_lhs(695, A::add_scaled_inputs3_offset(s.ad_value(635), p.p486, s.ad_value(637), p.p488, s.ad_value(638), p.p489, ((p.p485) + ((s.v[636] * p.p487)))), 639, p.p490);

        s.store_add_scaled_ad_lhs(696, A::add_scaled_inputs3_offset(s.ad_value(635), p.p519, s.ad_value(637), p.p521, s.ad_value(638), p.p522, ((p.p518) + ((s.v[636] * p.p520)))), 639, p.p523);

        s.store_add_scaled_ad_lhs(697, A::add_scaled_inputs3_offset(s.ad_value(635), p.p525, s.ad_value(637), p.p527, s.ad_value(638), p.p528, ((p.p524) + ((s.v[636] * p.p526)))), 639, p.p529);

        s.store_add_scaled_ad_lhs(657, A::add_scaled_inputs3_offset(s.ad_value(635), p.p493, s.ad_value(637), p.p495, s.ad_value(638), p.p496, ((p.p492) + ((s.v[636] * p.p494)))), 639, p.p497);

        s.store_add_scaled_ad_lhs(790, A::add_scaled_inputs3_offset(s.ad_value(635), p.p532, s.ad_value(637), p.p534, s.ad_value(638), p.p535, ((p.p531) + ((s.v[636] * p.p533)))), 639, p.p536);

        s.store_add_scaled_ad_lhs(700, A::add_scaled_inputs3_offset(s.ad_value(635), p.p544, s.ad_value(637), p.p546, s.ad_value(638), p.p547, ((p.p543) + ((s.v[636] * p.p545)))), 639, p.p548);

        s.store_add_scaled_ad_lhs(704, A::add_scaled_inputs3_offset(s.ad_value(635), p.p606, s.ad_value(637), p.p608, s.ad_value(638), p.p609, ((p.p605) + ((s.v[636] * p.p607)))), 639, p.p610);

        s.store_add_scaled_ad_lhs(707, A::add_scaled_inputs3_offset(s.ad_value(635), p.p624, s.ad_value(637), p.p626, s.ad_value(638), p.p627, ((p.p623) + ((s.v[636] * p.p625)))), 639, p.p628);

        s.store_add_scaled_ad_lhs(703, A::add_scaled_inputs3_offset(s.ad_value(635), p.p630, s.ad_value(637), p.p632, s.ad_value(638), p.p633, ((p.p629) + ((s.v[636] * p.p631)))), 639, p.p634);

        s.store_add_scaled_ad_lhs(807, A::add_scaled_inputs3_offset(s.ad_value(635), p.p642, s.ad_value(637), p.p644, s.ad_value(638), p.p645, ((p.p641) + ((s.v[636] * p.p643)))), 639, p.p646);

        s.store_add_scaled_ad_lhs(811, A::add_scaled_inputs3_offset(s.ad_value(635), p.p678, s.ad_value(637), p.p680, s.ad_value(638), p.p681, ((p.p677) + ((s.v[636] * p.p679)))), 639, p.p682);

        s.store_add_scaled_ad_lhs(812, A::add_scaled_inputs3_offset(s.ad_value(635), p.p690, s.ad_value(637), p.p692, s.ad_value(638), p.p693, ((p.p689) + ((s.v[636] * p.p691)))), 639, p.p694);

        s.store_add_scaled_ad_lhs(814, A::add_scaled_inputs3_offset(s.ad_value(635), p.p708, s.ad_value(637), p.p710, s.ad_value(638), p.p711, ((p.p707) + ((s.v[636] * p.p709)))), 639, p.p712);

        s.store_add_scaled_ad_lhs(325, A::add_scaled_inputs3_offset(s.ad_value(635), p.p714, s.ad_value(637), p.p716, s.ad_value(638), p.p717, ((p.p713) + ((s.v[636] * p.p715)))), 639, p.p718);

        s.store_add_scaled_ad_lhs(326, A::add_scaled_inputs3_offset(s.ad_value(635), p.p720, s.ad_value(637), p.p722, s.ad_value(638), p.p723, ((p.p719) + ((s.v[636] * p.p721)))), 639, p.p724);

        s.store_add_scaled_ad_lhs(328, A::add_scaled_inputs3_offset(s.ad_value(635), p.p726, s.ad_value(637), p.p728, s.ad_value(638), p.p729, ((p.p725) + ((s.v[636] * p.p727)))), 639, p.p730);

        s.store_add_scaled_ad_lhs(329, A::add_scaled_inputs3_offset(s.ad_value(635), p.p732, s.ad_value(637), p.p734, s.ad_value(638), p.p735, ((p.p731) + ((s.v[636] * p.p733)))), 639, p.p736);

        s.store_add_scaled_ad_lhs(792, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1027, s.ad_value(637), p.p1029, s.ad_value(638), p.p1030, ((p.p1025) + ((s.v[636] * p.p1028)))), 639, p.p1031);

        s.store_add_scaled_ad_lhs(793, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1039, s.ad_value(637), p.p1041, s.ad_value(638), p.p1042, ((p.p1038) + ((s.v[636] * p.p1040)))), 639, p.p1043);

        s.store_add_scaled_ad_lhs(794, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1045, s.ad_value(637), p.p1047, s.ad_value(638), p.p1048, ((p.p1044) + ((s.v[636] * p.p1046)))), 639, p.p1049);

        s.store_add_scaled_ad_lhs(798, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1051, s.ad_value(637), p.p1053, s.ad_value(638), p.p1054, ((p.p1050) + ((s.v[636] * p.p1052)))), 639, p.p1055);

        s.store_add_scaled_ad_lhs(800, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1057, s.ad_value(637), p.p1059, s.ad_value(638), p.p1060, ((p.p1056) + ((s.v[636] * p.p1058)))), 639, p.p1061);

        s.store_add_scaled_ad_lhs(799, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1063, s.ad_value(637), p.p1065, s.ad_value(638), p.p1066, ((p.p1062) + ((s.v[636] * p.p1064)))), 639, p.p1067);

        s.store_add_scaled_ad_lhs(801, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1069, s.ad_value(637), p.p1071, s.ad_value(638), p.p1072, ((p.p1068) + ((s.v[636] * p.p1070)))), 639, p.p1073);

        s.store_add_scaled_ad_lhs(709, A::add_scaled_inputs3_offset(s.ad_value(635), p.p926, s.ad_value(637), p.p928, s.ad_value(638), p.p929, ((p.p925) + ((s.v[636] * p.p927)))), 639, p.p930);

        s.store_add_scaled_ad_lhs(853, A::add_scaled_inputs3_offset(s.ad_value(635), p.p932, s.ad_value(637), p.p934, s.ad_value(638), p.p935, ((p.p931) + ((s.v[636] * p.p933)))), 639, p.p936);

        s.store_add_scaled_ad_lhs(852, A::add_scaled_inputs3_offset(s.ad_value(635), p.p938, s.ad_value(637), p.p940, s.ad_value(638), p.p941, ((p.p937) + ((s.v[636] * p.p939)))), 639, p.p942);

        s.store_add_scaled_ad_lhs(712, A::add_scaled_inputs3_offset(s.ad_value(635), p.p950, s.ad_value(637), p.p952, s.ad_value(638), p.p953, ((p.p949) + ((s.v[636] * p.p951)))), 639, p.p954);

        s.store_add_scaled_ad_lhs(711, A::add_scaled_inputs3_offset(s.ad_value(635), p.p944, s.ad_value(637), p.p946, s.ad_value(638), p.p947, ((p.p943) + ((s.v[636] * p.p945)))), 639, p.p948);

        s.store_add_scaled_ad_lhs(713, A::add_scaled_inputs3_offset(s.ad_value(635), p.p956, s.ad_value(637), p.p958, s.ad_value(638), p.p959, ((p.p955) + ((s.v[636] * p.p957)))), 639, p.p960);

        s.store_add_scaled_ad_lhs(714, A::add_scaled_inputs3_offset(s.ad_value(635), p.p986, s.ad_value(637), p.p988, s.ad_value(638), p.p989, ((p.p985) + ((s.v[636] * p.p987)))), 639, p.p990);

        s.store_add_scaled_ad_lhs(716, A::add_scaled_inputs3_offset(s.ad_value(635), p.p992, s.ad_value(637), p.p994, s.ad_value(638), p.p995, ((p.p991) + ((s.v[636] * p.p993)))), 639, p.p996);

        s.store_add_scaled_ad_lhs(719, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1010, s.ad_value(637), p.p1012, s.ad_value(638), p.p1013, ((p.p1009) + ((s.v[636] * p.p1011)))), 639, p.p1014);

        s.store_add_scaled_ad_lhs(720, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1016, s.ad_value(637), p.p1018, s.ad_value(638), p.p1019, ((p.p1015) + ((s.v[636] * p.p1017)))), 639, p.p1020);

        s.store_add_scaled_ad_lhs(721, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1120, s.ad_value(637), p.p1122, s.ad_value(638), p.p1123, ((p.p1119) + ((s.v[636] * p.p1121)))), 639, p.p1124);

        s.store_add_scaled_ad_lhs(722, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1126, s.ad_value(637), p.p1128, s.ad_value(638), p.p1129, ((p.p1125) + ((s.v[636] * p.p1127)))), 639, p.p1130);

        s.store_add_scaled_ad_lhs(723, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1132, s.ad_value(637), p.p1134, s.ad_value(638), p.p1135, ((p.p1131) + ((s.v[636] * p.p1133)))), 639, p.p1136);

        s.store_add_scaled_ad_lhs(724, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1138, s.ad_value(637), p.p1140, s.ad_value(638), p.p1141, ((p.p1137) + ((s.v[636] * p.p1139)))), 639, p.p1142);

        s.store_add_scaled_ad_lhs(725, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1144, s.ad_value(637), p.p1146, s.ad_value(638), p.p1147, ((p.p1143) + ((s.v[636] * p.p1145)))), 639, p.p1148);

        s.store_add_scaled_ad_lhs(726, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1150, s.ad_value(637), p.p1152, s.ad_value(638), p.p1153, ((p.p1149) + ((s.v[636] * p.p1151)))), 639, p.p1154);

        s.store_add_scaled_ad_lhs(727, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1156, s.ad_value(637), p.p1158, s.ad_value(638), p.p1159, ((p.p1155) + ((s.v[636] * p.p1157)))), 639, p.p1160);

        s.store_add_scaled_ad_lhs(728, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1162, s.ad_value(637), p.p1164, s.ad_value(638), p.p1165, ((p.p1161) + ((s.v[636] * p.p1163)))), 639, p.p1166);

        s.store_add_scaled_ad_lhs(729, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1168, s.ad_value(637), p.p1170, s.ad_value(638), p.p1171, ((p.p1167) + ((s.v[636] * p.p1169)))), 639, p.p1172);

        s.store_add_scaled_ad_lhs(730, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1174, s.ad_value(637), p.p1176, s.ad_value(638), p.p1177, ((p.p1173) + ((s.v[636] * p.p1175)))), 639, p.p1178);

        s.store_add_scaled_ad_lhs(731, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1180, s.ad_value(637), p.p1182, s.ad_value(638), p.p1183, ((p.p1179) + ((s.v[636] * p.p1181)))), 639, p.p1184);

        s.store_add_scaled_ad_lhs(732, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1186, s.ad_value(637), p.p1188, s.ad_value(638), p.p1189, ((p.p1185) + ((s.v[636] * p.p1187)))), 639, p.p1190);

        s.store_add_scaled_ad_lhs(733, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1192, s.ad_value(637), p.p1194, s.ad_value(638), p.p1195, ((p.p1191) + ((s.v[636] * p.p1193)))), 639, p.p1196);

        s.store_add_scaled_ad_lhs(734, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1198, s.ad_value(637), p.p1200, s.ad_value(638), p.p1201, ((p.p1197) + ((s.v[636] * p.p1199)))), 639, p.p1202);

        s.store_add_scaled_ad_lhs(735, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1204, s.ad_value(637), p.p1206, s.ad_value(638), p.p1207, ((p.p1203) + ((s.v[636] * p.p1205)))), 639, p.p1208);

        s.store_add_scaled_ad_lhs(736, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1210, s.ad_value(637), p.p1212, s.ad_value(638), p.p1213, ((p.p1209) + ((s.v[636] * p.p1211)))), 639, p.p1214);

        s.store_add_scaled_ad_lhs(737, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1216, s.ad_value(637), p.p1218, s.ad_value(638), p.p1219, ((p.p1215) + ((s.v[636] * p.p1217)))), 639, p.p1220);

        s.store_add_scaled_ad_lhs(738, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1222, s.ad_value(637), p.p1224, s.ad_value(638), p.p1225, ((p.p1221) + ((s.v[636] * p.p1223)))), 639, p.p1226);

        s.store_add_scaled_ad_lhs(739, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1228, s.ad_value(637), p.p1230, s.ad_value(638), p.p1231, ((p.p1227) + ((s.v[636] * p.p1229)))), 639, p.p1232);

        s.store_add_scaled_ad_lhs(740, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1234, s.ad_value(637), p.p1236, s.ad_value(638), p.p1237, ((p.p1233) + ((s.v[636] * p.p1235)))), 639, p.p1238);

        s.store_add_scaled_ad_lhs(743, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1240, s.ad_value(637), p.p1242, s.ad_value(638), p.p1243, ((p.p1239) + ((s.v[636] * p.p1241)))), 639, p.p1244);

        s.store_add_scaled_ad_lhs(744, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1246, s.ad_value(637), p.p1248, s.ad_value(638), p.p1249, ((p.p1245) + ((s.v[636] * p.p1247)))), 639, p.p1250);

        s.store_add_scaled_ad_lhs(745, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1252, s.ad_value(637), p.p1254, s.ad_value(638), p.p1255, ((p.p1251) + ((s.v[636] * p.p1253)))), 639, p.p1256);

        s.store_add_scaled_ad_lhs(746, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1258, s.ad_value(637), p.p1260, s.ad_value(638), p.p1261, ((p.p1257) + ((s.v[636] * p.p1259)))), 639, p.p1262);

        s.store_add_scaled_ad_lhs(741, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1114, s.ad_value(637), p.p1116, s.ad_value(638), p.p1117, ((p.p1113) + ((s.v[636] * p.p1115)))), 639, p.p1118);

        s.store_add_scaled_ad_lhs(742, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1264, s.ad_value(637), p.p1266, s.ad_value(638), p.p1267, ((p.p1263) + ((s.v[636] * p.p1265)))), 639, p.p1268);

        s.store_add_scaled_ad_lhs(747, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1270, s.ad_value(637), p.p1272, s.ad_value(638), p.p1273, ((p.p1269) + ((s.v[636] * p.p1271)))), 639, p.p1274);

        s.store_add_scaled_ad_lhs(748, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1276, s.ad_value(637), p.p1278, s.ad_value(638), p.p1279, ((p.p1275) + ((s.v[636] * p.p1277)))), 639, p.p1280);

        s.store_add_scaled_ad_lhs(749, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1282, s.ad_value(637), p.p1284, s.ad_value(638), p.p1285, ((p.p1281) + ((s.v[636] * p.p1283)))), 639, p.p1286);

        s.store_add_scaled_ad_lhs(750, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1288, s.ad_value(637), p.p1290, s.ad_value(638), p.p1291, ((p.p1287) + ((s.v[636] * p.p1289)))), 639, p.p1292);

        s.store_add_scaled_ad_lhs(751, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1294, s.ad_value(637), p.p1296, s.ad_value(638), p.p1297, ((p.p1293) + ((s.v[636] * p.p1295)))), 639, p.p1298);

        s.store_add_scaled_ad_lhs(752, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1330, s.ad_value(637), p.p1332, s.ad_value(638), p.p1333, ((p.p1329) + ((s.v[636] * p.p1331)))), 639, p.p1334);

        s.store_add_scaled_ad_lhs(753, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1336, s.ad_value(637), p.p1338, s.ad_value(638), p.p1339, ((p.p1335) + ((s.v[636] * p.p1337)))), 639, p.p1340);

        s.store_add_scaled_ad_lhs(754, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1342, s.ad_value(637), p.p1344, s.ad_value(638), p.p1345, ((p.p1341) + ((s.v[636] * p.p1343)))), 639, p.p1346);

        s.store_add_scaled_ad_lhs(755, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1348, s.ad_value(637), p.p1350, s.ad_value(638), p.p1351, ((p.p1347) + ((s.v[636] * p.p1349)))), 639, p.p1352);

        s.store_add_scaled_ad_lhs(761, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1300, s.ad_value(637), p.p1302, s.ad_value(638), p.p1303, ((p.p1299) + ((s.v[636] * p.p1301)))), 639, p.p1304);

        s.store_add_scaled_ad_lhs(762, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1306, s.ad_value(637), p.p1308, s.ad_value(638), p.p1309, ((p.p1305) + ((s.v[636] * p.p1307)))), 639, p.p1310);

        s.store_add_scaled_ad_lhs(763, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1312, s.ad_value(637), p.p1314, s.ad_value(638), p.p1315, ((p.p1311) + ((s.v[636] * p.p1313)))), 639, p.p1316);

        s.store_add_scaled_ad_lhs(764, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1318, s.ad_value(637), p.p1320, s.ad_value(638), p.p1321, ((p.p1317) + ((s.v[636] * p.p1319)))), 639, p.p1322);

        s.store_add_scaled_ad_lhs(765, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1324, s.ad_value(637), p.p1326, s.ad_value(638), p.p1327, ((p.p1323) + ((s.v[636] * p.p1325)))), 639, p.p1328);

        s.store_add_scaled_ad_lhs(766, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1354, s.ad_value(637), p.p1356, s.ad_value(638), p.p1357, ((p.p1353) + ((s.v[636] * p.p1355)))), 639, p.p1358);

        s.store_add_scaled_ad_lhs(767, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1360, s.ad_value(637), p.p1362, s.ad_value(638), p.p1363, ((p.p1359) + ((s.v[636] * p.p1361)))), 639, p.p1364);

        s.store_add_scaled_ad_lhs(768, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1366, s.ad_value(637), p.p1368, s.ad_value(638), p.p1369, ((p.p1365) + ((s.v[636] * p.p1367)))), 639, p.p1370);

        s.store_add_scaled_ad_lhs(769, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1372, s.ad_value(637), p.p1374, s.ad_value(638), p.p1375, ((p.p1371) + ((s.v[636] * p.p1373)))), 639, p.p1376);

        s.store_add_scaled_ad_lhs(775, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1445, s.ad_value(637), p.p1447, s.ad_value(638), p.p1448, ((p.p1444) + ((s.v[636] * p.p1446)))), 639, p.p1449);

        s.store_add_scaled_ad_lhs(776, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1451, s.ad_value(637), p.p1453, s.ad_value(638), p.p1454, ((p.p1450) + ((s.v[636] * p.p1452)))), 639, p.p1455);

        s.store_add_scaled_ad_lhs(777, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1463, s.ad_value(637), p.p1465, s.ad_value(638), p.p1466, ((p.p1462) + ((s.v[636] * p.p1464)))), 639, p.p1467);

        s.store_add_scaled_ad_lhs(778, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1469, s.ad_value(637), p.p1471, s.ad_value(638), p.p1472, ((p.p1468) + ((s.v[636] * p.p1470)))), 639, p.p1473);

        s.store_add_scaled_ad_lhs(779, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1457, s.ad_value(637), p.p1459, s.ad_value(638), p.p1460, ((p.p1456) + ((s.v[636] * p.p1458)))), 639, p.p1461);

        s.store_add_scaled_ad_lhs(780, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1475, s.ad_value(637), p.p1477, s.ad_value(638), p.p1478, ((p.p1474) + ((s.v[636] * p.p1476)))), 639, p.p1479);

        s.store_add_scaled_ad_lhs(781, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1481, s.ad_value(637), p.p1483, s.ad_value(638), p.p1484, ((p.p1480) + ((s.v[636] * p.p1482)))), 639, p.p1485);

        s.store_add_scaled_ad_lhs(782, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1487, s.ad_value(637), p.p1489, s.ad_value(638), p.p1490, ((p.p1486) + ((s.v[636] * p.p1488)))), 639, p.p1491);

        s.store_add_scaled_ad_lhs(783, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1493, s.ad_value(637), p.p1495, s.ad_value(638), p.p1496, ((p.p1492) + ((s.v[636] * p.p1494)))), 639, p.p1497);

        s.store_add_scaled_ad_lhs(784, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1499, s.ad_value(637), p.p1501, s.ad_value(638), p.p1502, ((p.p1498) + ((s.v[636] * p.p1500)))), 639, p.p1503);

        s.store_add_scaled_ad_lhs(785, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1505, s.ad_value(637), p.p1507, s.ad_value(638), p.p1508, ((p.p1504) + ((s.v[636] * p.p1506)))), 639, p.p1509);

        s.store_add_scaled_ad_lhs(786, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1511, s.ad_value(637), p.p1513, s.ad_value(638), p.p1514, ((p.p1510) + ((s.v[636] * p.p1512)))), 639, p.p1515);

        s.store_add_scaled_ad_lhs(787, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1517, s.ad_value(637), p.p1519, s.ad_value(638), p.p1520, ((p.p1516) + ((s.v[636] * p.p1518)))), 639, p.p1521);

        s.store_add_scaled_ad_lhs(788, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1523, s.ad_value(637), p.p1525, s.ad_value(638), p.p1526, ((p.p1522) + ((s.v[636] * p.p1524)))), 639, p.p1527);

        s.store_add_scaled_ad_lhs(789, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1763, s.ad_value(637), p.p1765, s.ad_value(638), p.p1766, ((p.p1762) + ((s.v[636] * p.p1764)))), 639, p.p1767);

        s.store_add_scaled_ad_lhs(643, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1531, s.ad_value(637), p.p1533, s.ad_value(638), p.p1534, ((p.p1530) + ((s.v[636] * p.p1532)))), 639, p.p1535);

        s.store_add_scaled_ad_lhs(642, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1537, s.ad_value(637), p.p1539, s.ad_value(638), p.p1540, ((p.p1536) + ((s.v[636] * p.p1538)))), 639, p.p1541);

        s.store_add_scaled_ad_lhs(644, A::add_scaled_inputs3_offset(s.ad_value(635), p.p29, s.ad_value(637), p.p31, s.ad_value(638), p.p32, ((p.p28) + ((s.v[636] * p.p30)))), 639, p.p33);

        s.store_add_scaled_ad_lhs(645, A::add_scaled_inputs3_offset(s.ad_value(635), p.p35, s.ad_value(637), p.p37, s.ad_value(638), p.p38, ((p.p34) + ((s.v[636] * p.p36)))), 639, p.p39);

        s.store_add_scaled_ad_lhs(648, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1548, s.ad_value(637), p.p1550, s.ad_value(638), p.p1551, ((p.p1547) + ((s.v[636] * p.p1549)))), 639, p.p1552);

        s.store_add_scaled_ad_lhs(649, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1554, s.ad_value(637), p.p1556, s.ad_value(638), p.p1557, ((p.p1553) + ((s.v[636] * p.p1555)))), 639, p.p1558);

        s.store_add_scaled_ad_lhs(650, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1560, s.ad_value(637), p.p1562, s.ad_value(638), p.p1563, ((p.p1559) + ((s.v[636] * p.p1561)))), 639, p.p1564);

        s.store_add_scaled_ad_lhs(651, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1566, s.ad_value(637), p.p1568, s.ad_value(638), p.p1569, ((p.p1565) + ((s.v[636] * p.p1567)))), 639, p.p1570);

        s.store_add_scaled_ad_lhs(652, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1572, s.ad_value(637), p.p1574, s.ad_value(638), p.p1575, ((p.p1571) + ((s.v[636] * p.p1573)))), 639, p.p1576);

        s.store_add_scaled_ad_lhs(653, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1578, s.ad_value(637), p.p1580, s.ad_value(638), p.p1581, ((p.p1577) + ((s.v[636] * p.p1579)))), 639, p.p1582);

        s.store_add_scaled_ad_lhs(864, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1651, s.ad_value(637), p.p1653, s.ad_value(638), p.p1654, ((p.p1650) + ((s.v[636] * p.p1652)))), 639, p.p1655);

        s.store_add_scaled_ad_lhs(865, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1657, s.ad_value(637), p.p1659, s.ad_value(638), p.p1660, ((p.p1656) + ((s.v[636] * p.p1658)))), 639, p.p1661);

        s.store_add_scaled_ad_lhs(866, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1663, s.ad_value(637), p.p1665, s.ad_value(638), p.p1666, ((p.p1662) + ((s.v[636] * p.p1664)))), 639, p.p1667);

        s.store_add_scaled_ad_lhs(836, A::add_scaled_inputs3_offset(s.ad_value(635), p.p738, s.ad_value(637), p.p740, s.ad_value(638), p.p741, ((p.p737) + ((s.v[636] * p.p739)))), 639, p.p742);

        s.store_add_scaled_ad_lhs(837, A::add_scaled_inputs3_offset(s.ad_value(635), p.p756, s.ad_value(637), p.p758, s.ad_value(638), p.p759, ((p.p755) + ((s.v[636] * p.p757)))), 639, p.p760);

        s.store_add_scaled_ad_lhs(838, A::add_scaled_inputs3_offset(s.ad_value(635), p.p768, s.ad_value(637), p.p770, s.ad_value(638), p.p771, ((p.p767) + ((s.v[636] * p.p769)))), 639, p.p772);

        s.store_add_scaled_ad_lhs(842, A::add_scaled_inputs3_offset(s.ad_value(635), p.p786, s.ad_value(637), p.p788, s.ad_value(638), p.p789, ((p.p785) + ((s.v[636] * p.p787)))), 639, p.p790);

        s.store_add_scaled_ad_lhs(823, A::add_scaled_inputs3_offset(s.ad_value(635), p.p792, s.ad_value(637), p.p794, s.ad_value(638), p.p795, ((p.p791) + ((s.v[636] * p.p793)))), 639, p.p796);

        s.store_add_scaled_ad_lhs(824, A::add_scaled_inputs3_offset(s.ad_value(635), p.p810, s.ad_value(637), p.p812, s.ad_value(638), p.p813, ((p.p809) + ((s.v[636] * p.p811)))), 639, p.p814);

        s.store_add_scaled_ad_lhs(847, A::add_scaled_inputs3_offset(s.ad_value(635), p.p822, s.ad_value(637), p.p824, s.ad_value(638), p.p825, ((p.p821) + ((s.v[636] * p.p823)))), 639, p.p826);

        s.store_add_scaled_ad_lhs(830, A::add_scaled_inputs3_offset(s.ad_value(635), p.p846, s.ad_value(637), p.p848, s.ad_value(638), p.p849, ((p.p845) + ((s.v[636] * p.p847)))), 639, p.p850);

        s.store_add_scaled_ad_lhs(831, A::add_scaled_inputs3_offset(s.ad_value(635), p.p864, s.ad_value(637), p.p866, s.ad_value(638), p.p867, ((p.p863) + ((s.v[636] * p.p865)))), 639, p.p868);

        s.store_add_scaled_ad_lhs(834, A::add_scaled_inputs3_offset(s.ad_value(635), p.p876, s.ad_value(637), p.p878, s.ad_value(638), p.p879, ((p.p875) + ((s.v[636] * p.p877)))), 639, p.p880);

        s.store_add_scaled_ad_lhs(835, A::add_scaled_inputs3_offset(s.ad_value(635), p.p882, s.ad_value(637), p.p884, s.ad_value(638), p.p885, ((p.p881) + ((s.v[636] * p.p883)))), 639, p.p886);

        s.store_add_scaled_ad_lhs(848, A::add_scaled_inputs3_offset(s.ad_value(635), p.p576, s.ad_value(637), p.p578, s.ad_value(638), p.p579, ((p.p575) + ((s.v[636] * p.p577)))), 639, p.p580);

        s.store_add_scaled_ad_lhs(849, A::add_scaled_inputs3_offset(s.ad_value(635), p.p556, s.ad_value(637), p.p558, s.ad_value(638), p.p559, ((p.p555) + ((s.v[636] * p.p557)))), 639, p.p560);

        s.store_add_scaled_ad_lhs(850, A::add_scaled_inputs3_offset(s.ad_value(635), p.p569, s.ad_value(637), p.p571, s.ad_value(638), p.p572, ((p.p568) + ((s.v[636] * p.p570)))), 639, p.p573);

        s.store_add_scaled_ad_lhs(854, A::add_scaled_inputs3_offset(s.ad_value(635), p.p962, s.ad_value(637), p.p964, s.ad_value(638), p.p965, ((p.p961) + ((s.v[636] * p.p963)))), 639, p.p966);

        s.store_add_scaled_ad_lhs(855, A::add_scaled_inputs3_offset(s.ad_value(635), p.p968, s.ad_value(637), p.p970, s.ad_value(638), p.p971, ((p.p967) + ((s.v[636] * p.p969)))), 639, p.p972);

        s.store_add_scaled_ad_lhs(856, A::add_scaled_inputs3_offset(s.ad_value(635), p.p974, s.ad_value(637), p.p976, s.ad_value(638), p.p977, ((p.p973) + ((s.v[636] * p.p975)))), 639, p.p978);

        s.store_add_scaled_ad_lhs(857, A::add_scaled_inputs3_offset(s.ad_value(635), p.p980, s.ad_value(637), p.p982, s.ad_value(638), p.p983, ((p.p979) + ((s.v[636] * p.p981)))), 639, p.p984);

        s.store_add_scaled_ad_lhs(858, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1742, s.ad_value(637), p.p1744, s.ad_value(638), p.p1745, ((p.p1741) + ((s.v[636] * p.p1743)))), 639, p.p1746);

        s.store_add_scaled_ad_lhs(859, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1751, s.ad_value(637), p.p1753, s.ad_value(638), p.p1754, ((p.p1750) + ((s.v[636] * p.p1752)))), 639, p.p1755);

        s.store_add_scaled_ad_lhs(860, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1757, s.ad_value(637), p.p1759, s.ad_value(638), p.p1760, ((p.p1756) + ((s.v[636] * p.p1758)))), 639, p.p1761);

        s.store_add_scaled_ad_lhs(862, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1769, s.ad_value(637), p.p1771, s.ad_value(638), p.p1772, ((p.p1768) + ((s.v[636] * p.p1770)))), 639, p.p1773);

        s.store_add_scaled_ad_lhs(863, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1775, s.ad_value(637), p.p1777, s.ad_value(638), p.p1778, ((p.p1774) + ((s.v[636] * p.p1776)))), 639, p.p1779);

        s.store_add_scaled_ad_lhs(861, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1781, s.ad_value(637), p.p1783, s.ad_value(638), p.p1784, ((p.p1780) + ((s.v[636] * p.p1782)))), 639, p.p1785);

        s.store_add_scaled_ad_lhs(681, A::add_scaled_inputs3_offset(s.ad_value(635), p.p177, s.ad_value(637), p.p179, s.ad_value(638), p.p180, ((p.p176) + ((s.v[636] * p.p178)))), 639, p.p181);

        s.store_add_scaled_ad_lhs(682, A::add_scaled_inputs3_offset(s.ad_value(635), p.p183, s.ad_value(637), p.p185, s.ad_value(638), p.p186, ((p.p182) + ((s.v[636] * p.p184)))), 639, p.p187);

        s.store_add_scaled_ad_lhs(574, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1690, s.ad_value(637), p.p1692, s.ad_value(638), p.p1693, ((p.p1689) + ((s.v[636] * p.p1691)))), 639, p.p1694);

        s.store_add_scaled_ad_lhs(576, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1702, s.ad_value(637), p.p1704, s.ad_value(638), p.p1705, ((p.p1701) + ((s.v[636] * p.p1703)))), 639, p.p1706);

        s.store_add_scaled_ad_lhs(575, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1696, s.ad_value(637), p.p1698, s.ad_value(638), p.p1699, ((p.p1695) + ((s.v[636] * p.p1697)))), 639, p.p1700);

        s.b[1096] = (p.p61 != 0.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if s.b[1096] {
            s.store_add_scaled_ad_lhs(689, A::add_scaled_inputs3_offset(s.ad_value(635), p.p357, s.ad_value(637), p.p359, s.ad_value(638), p.p360, ((p.p356) + ((s.v[636] * p.p358)))), 639, p.p361);
            s.store_add_scaled_ad_lhs(690, A::add_scaled_inputs3_offset(s.ad_value(635), p.p363, s.ad_value(637), p.p365, s.ad_value(638), p.p366, ((p.p362) + ((s.v[636] * p.p364)))), 639, p.p367);
            s.store_add_scaled_ad_lhs(691, A::add_scaled_inputs3_offset(s.ad_value(635), p.p369, s.ad_value(637), p.p371, s.ad_value(638), p.p372, ((p.p368) + ((s.v[636] * p.p370)))), 639, p.p373);
            s.store_add_scaled_ad_lhs(809, A::add_scaled_inputs3_offset(s.ad_value(635), p.p660, s.ad_value(637), p.p662, s.ad_value(638), p.p663, ((p.p659) + ((s.v[636] * p.p661)))), 639, p.p664);
            s.store_add_scaled_ad_lhs(828, A::add_scaled_inputs3_offset(s.ad_value(635), p.p828, s.ad_value(637), p.p830, s.ad_value(638), p.p831, ((p.p827) + ((s.v[636] * p.p829)))), 639, p.p832);
        }

        s.b[1097] = (p.p61 == 2.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1096] && s.b[1097]) {
            s.store_add_scaled_ad_lhs(871, A::add_scaled_inputs3_offset(s.ad_value(635), p.p387, s.ad_value(637), p.p389, s.ad_value(638), p.p390, ((p.p386) + ((s.v[636] * p.p388)))), 639, p.p391);
            s.store_add_scaled_ad_lhs(872, A::add_scaled_inputs3_offset(s.ad_value(635), p.p393, s.ad_value(637), p.p395, s.ad_value(638), p.p396, ((p.p392) + ((s.v[636] * p.p394)))), 639, p.p397);
            s.store_add_scaled_ad_lhs(692, A::add_scaled_inputs3_offset(s.ad_value(635), p.p375, s.ad_value(637), p.p377, s.ad_value(638), p.p378, ((p.p374) + ((s.v[636] * p.p376)))), 639, p.p379);
            s.store_add_scaled_ad_lhs(693, A::add_scaled_inputs3_offset(s.ad_value(635), p.p381, s.ad_value(637), p.p383, s.ad_value(638), p.p384, ((p.p380) + ((s.v[636] * p.p382)))), 639, p.p385);
        }

        s.b[1098] = (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1096] && s.b[1098]) {
            s.store_add_scaled_ad_lhs(756, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1378, s.ad_value(637), p.p1380, s.ad_value(638), p.p1381, ((p.p1377) + ((s.v[636] * p.p1379)))), 639, p.p1382);
            s.store_add_scaled_ad_lhs(757, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1384, s.ad_value(637), p.p1386, s.ad_value(638), p.p1387, ((p.p1383) + ((s.v[636] * p.p1385)))), 639, p.p1388);
            s.store_add_scaled_ad_lhs(758, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1390, s.ad_value(637), p.p1392, s.ad_value(638), p.p1393, ((p.p1389) + ((s.v[636] * p.p1391)))), 639, p.p1394);
            s.store_add_scaled_ad_lhs(759, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1396, s.ad_value(637), p.p1398, s.ad_value(638), p.p1399, ((p.p1395) + ((s.v[636] * p.p1397)))), 639, p.p1400);
            s.store_add_scaled_ad_lhs(760, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1402, s.ad_value(637), p.p1404, s.ad_value(638), p.p1405, ((p.p1401) + ((s.v[636] * p.p1403)))), 639, p.p1406);
            s.store_add_scaled_ad_lhs(770, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1408, s.ad_value(637), p.p1410, s.ad_value(638), p.p1411, ((p.p1407) + ((s.v[636] * p.p1409)))), 639, p.p1412);
            s.store_add_scaled_ad_lhs(771, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1414, s.ad_value(637), p.p1416, s.ad_value(638), p.p1417, ((p.p1413) + ((s.v[636] * p.p1415)))), 639, p.p1418);
            s.store_add_scaled_ad_lhs(772, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1420, s.ad_value(637), p.p1422, s.ad_value(638), p.p1423, ((p.p1419) + ((s.v[636] * p.p1421)))), 639, p.p1424);
            s.store_add_scaled_ad_lhs(773, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1426, s.ad_value(637), p.p1428, s.ad_value(638), p.p1429, ((p.p1425) + ((s.v[636] * p.p1427)))), 639, p.p1430);
            s.store_add_scaled_ad_lhs(774, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1432, s.ad_value(637), p.p1434, s.ad_value(638), p.p1435, ((p.p1431) + ((s.v[636] * p.p1433)))), 639, p.p1436);
        }

        s.b[1099] = (p.p66 != 0.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if s.b[1099] {
            s.store_add_scaled_ad_lhs(665, A::add_scaled_inputs3_offset(s.ad_value(635), p.p213, s.ad_value(637), p.p215, s.ad_value(638), p.p216, ((p.p212) + ((s.v[636] * p.p214)))), 639, p.p217);
            s.store_add_scaled_ad_lhs(668, A::add_scaled_inputs3_offset(s.ad_value(635), p.p195, s.ad_value(637), p.p197, s.ad_value(638), p.p198, ((p.p194) + ((s.v[636] * p.p196)))), 639, p.p199);
            s.store_add_scaled_ad_lhs(677, A::add_scaled_inputs3_offset(s.ad_value(635), p.p255, s.ad_value(637), p.p257, s.ad_value(638), p.p258, ((p.p254) + ((s.v[636] * p.p256)))), 639, p.p259);
            s.store_add_scaled_ad_lhs(699, A::add_scaled_inputs3_offset(s.ad_value(635), p.p474, s.ad_value(637), p.p476, s.ad_value(638), p.p477, ((p.p473) + ((s.v[636] * p.p475)))), 639, p.p478);
            s.store_add_scaled_ad_lhs(791, A::add_scaled_inputs3_offset(s.ad_value(635), p.p538, s.ad_value(637), p.p540, s.ad_value(638), p.p541, ((p.p537) + ((s.v[636] * p.p539)))), 639, p.p542);
            s.store_add_scaled_ad_lhs(701, A::add_scaled_inputs3_offset(s.ad_value(635), p.p550, s.ad_value(637), p.p552, s.ad_value(638), p.p553, ((p.p549) + ((s.v[636] * p.p551)))), 639, p.p554);
            s.store_add_scaled_ad_lhs(715, A::add_scaled_inputs3_offset(s.ad_value(635), p.p998, s.ad_value(637), p.p1000, s.ad_value(638), p.p1001, ((p.p997) + ((s.v[636] * p.p999)))), 639, p.p1002);
            s.store_add_scaled_ad_lhs(717, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1004, s.ad_value(637), p.p1006, s.ad_value(638), p.p1007, ((p.p1003) + ((s.v[636] * p.p1005)))), 639, p.p1008);
            s.store_add_scaled_ad_lhs(796, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1033, s.ad_value(637), p.p1035, s.ad_value(638), p.p1036, ((p.p1032) + ((s.v[636] * p.p1034)))), 639, p.p1037);
            s.store_add_scaled_ad_lhs(806, A::add_scaled_inputs3_offset(s.ad_value(635), p.p291, s.ad_value(637), p.p293, s.ad_value(638), p.p294, ((p.p290) + ((s.v[636] * p.p292)))), 639, p.p295);
            s.store_add_scaled_ad_lhs(680, A::add_scaled_inputs3_offset(s.ad_value(635), p.p462, s.ad_value(637), p.p464, s.ad_value(638), p.p465, ((p.p461) + ((s.v[636] * p.p463)))), 639, p.p466);
            s.store_add_scaled_ad_lhs(658, A::add_scaled_inputs3_offset(s.ad_value(635), p.p501, s.ad_value(637), p.p503, s.ad_value(638), p.p504, ((p.p500) + ((s.v[636] * p.p502)))), 639, p.p505);
            s.store_add_scaled_ad_lhs(706, A::add_scaled_inputs3_offset(s.ad_value(635), p.p612, s.ad_value(637), p.p614, s.ad_value(638), p.p615, ((p.p611) + ((s.v[636] * p.p613)))), 639, p.p616);
            s.store_add_scaled_ad_lhs(815, A::add_scaled_inputs3_offset(s.ad_value(635), p.p648, s.ad_value(637), p.p650, s.ad_value(638), p.p651, ((p.p647) + ((s.v[636] * p.p649)))), 639, p.p652);
            s.store_add_scaled_ad_lhs(710, A::add_scaled_inputs3_offset(s.ad_value(635), p.p636, s.ad_value(637), p.p638, s.ad_value(638), p.p639, ((p.p635) + ((s.v[636] * p.p637)))), 639, p.p640);
            s.store_add_scaled_ad_lhs(816, A::add_scaled_inputs3_offset(s.ad_value(635), p.p684, s.ad_value(637), p.p686, s.ad_value(638), p.p687, ((p.p683) + ((s.v[636] * p.p685)))), 639, p.p688);
            s.store_add_scaled_ad_lhs(818, A::add_scaled_inputs3_offset(s.ad_value(635), p.p696, s.ad_value(637), p.p698, s.ad_value(638), p.p699, ((p.p695) + ((s.v[636] * p.p697)))), 639, p.p700);
            s.store_add_scaled_ad_lhs(845, A::add_scaled_inputs3_offset(s.ad_value(635), p.p744, s.ad_value(637), p.p746, s.ad_value(638), p.p747, ((p.p743) + ((s.v[636] * p.p745)))), 639, p.p748);
            s.store_add_scaled_ad_lhs(846, A::add_scaled_inputs3_offset(s.ad_value(635), p.p774, s.ad_value(637), p.p776, s.ad_value(638), p.p777, ((p.p773) + ((s.v[636] * p.p775)))), 639, p.p778);
            s.store_add_scaled_ad_lhs(825, A::add_scaled_inputs3_offset(s.ad_value(635), p.p798, s.ad_value(637), p.p800, s.ad_value(638), p.p801, ((p.p797) + ((s.v[636] * p.p799)))), 639, p.p802);
            s.store_add_scaled_ad_lhs(844, A::add_scaled_inputs3_offset(s.ad_value(635), p.p852, s.ad_value(637), p.p854, s.ad_value(638), p.p855, ((p.p851) + ((s.v[636] * p.p853)))), 639, p.p856);
            s.store_add_scaled_ad_lhs(851, A::add_scaled_inputs3_offset(s.ad_value(635), p.p563, s.ad_value(637), p.p565, s.ad_value(638), p.p566, ((p.p562) + ((s.v[636] * p.p564)))), 639, p.p567);
        }

        s.b[1100] = (p.p61 != 0.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1099] && s.b[1100]) {
            s.store_add_scaled_ad_lhs(817, A::add_scaled_inputs3_offset(s.ad_value(635), p.p666, s.ad_value(637), p.p668, s.ad_value(638), p.p669, ((p.p665) + ((s.v[636] * p.p667)))), 639, p.p670);
            s.store_add_scaled_ad_lhs(843, A::add_scaled_inputs3_offset(s.ad_value(635), p.p834, s.ad_value(637), p.p836, s.ad_value(638), p.p837, ((p.p833) + ((s.v[636] * p.p835)))), 639, p.p838);
        }

        s.b[1101] = (p.p67 == 1.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if s.b[1101] {
            s.store_add_scaled_ad_lhs(705, A::add_scaled_inputs3_offset(s.ad_value(635), p.p618, s.ad_value(637), p.p620, s.ad_value(638), p.p621, ((p.p617) + ((s.v[636] * p.p619)))), 639, p.p622);
        }

        s.b[1102] = (p.p582 != 0.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1102]) {
            s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));
        }

        if s.b[1101] {
            s.store_add_scaled_ad_lhs(808, A::add_scaled_inputs3_offset(s.ad_value(635), p.p654, s.ad_value(637), p.p656, s.ad_value(638), p.p657, ((p.p653) + ((s.v[636] * p.p655)))), 639, p.p658);
            s.store_add_scaled_ad_lhs(813, A::add_scaled_inputs3_offset(s.ad_value(635), p.p702, s.ad_value(637), p.p704, s.ad_value(638), p.p705, ((p.p701) + ((s.v[636] * p.p703)))), 639, p.p706);
            s.store_add_scaled_ad_lhs(839, A::add_scaled_inputs3_offset(s.ad_value(635), p.p750, s.ad_value(637), p.p752, s.ad_value(638), p.p753, ((p.p749) + ((s.v[636] * p.p751)))), 639, p.p754);
            s.store_add_scaled_ad_lhs(840, A::add_scaled_inputs3_offset(s.ad_value(635), p.p762, s.ad_value(637), p.p764, s.ad_value(638), p.p765, ((p.p761) + ((s.v[636] * p.p763)))), 639, p.p766);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1101] {
            s.store_add_scaled_ad_lhs(841, A::add_scaled_inputs3_offset(s.ad_value(635), p.p780, s.ad_value(637), p.p782, s.ad_value(638), p.p783, ((p.p779) + ((s.v[636] * p.p781)))), 639, p.p784);
            s.store_add_scaled_ad_lhs(826, A::add_scaled_inputs3_offset(s.ad_value(635), p.p804, s.ad_value(637), p.p806, s.ad_value(638), p.p807, ((p.p803) + ((s.v[636] * p.p805)))), 639, p.p808);
            s.store_add_scaled_ad_lhs(827, A::add_scaled_inputs3_offset(s.ad_value(635), p.p816, s.ad_value(637), p.p818, s.ad_value(638), p.p819, ((p.p815) + ((s.v[636] * p.p817)))), 639, p.p820);
            s.store_add_scaled_ad_lhs(832, A::add_scaled_inputs3_offset(s.ad_value(635), p.p858, s.ad_value(637), p.p860, s.ad_value(638), p.p861, ((p.p857) + ((s.v[636] * p.p859)))), 639, p.p862);
            s.store_add_scaled_ad_lhs(833, A::add_scaled_inputs3_offset(s.ad_value(635), p.p870, s.ad_value(637), p.p872, s.ad_value(638), p.p873, ((p.p869) + ((s.v[636] * p.p871)))), 639, p.p874);
        }

        s.b[1103] = (p.p61 != 0.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1103]) {
            s.store_add_scaled_ad_lhs(810, A::add_scaled_inputs3_offset(s.ad_value(635), p.p672, s.ad_value(637), p.p674, s.ad_value(638), p.p675, ((p.p671) + ((s.v[636] * p.p673)))), 639, p.p676);
            s.store_add_scaled_ad_lhs(829, A::add_scaled_inputs3_offset(s.ad_value(635), p.p840, s.ad_value(637), p.p842, s.ad_value(638), p.p843, ((p.p839) + ((s.v[636] * p.p841)))), 639, p.p844);
        }

        if s.b[1101] {
            s.store_add_scaled_ad_lhs(675, A::add_scaled_inputs3_offset(s.ad_value(635), p.p261, s.ad_value(637), p.p263, s.ad_value(638), p.p264, ((p.p260) + ((s.v[636] * p.p262)))), 639, p.p265);
        }

        s.b[1104] = (p.p161 != 0.0);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1104]) {
            s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1105] = (p.p21 != 0.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1105]) {
            s.store_mul_ad_rhs(705, 705, A::scale_offset(s.ad_value(153), ((p.p5 - p.p21) * p.p588), 1.0));
            s.store_mul_ad_rhs(675, 675, A::scale_offset(s.ad_value(153), ((p.p5 - p.p21) * p.p163), 1.0));
        }

        s.b[1106] = ((p.p73 != 0.0) && (p.p1668 != 0.0));
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if s.b[1106] {
            s.store_add_scaled_ad_lhs(873, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1669, s.ad_value(637), p.p1671, s.ad_value(638), p.p1672, ((p.p1668) + ((s.v[636] * p.p1670)))), 639, p.p1673);
            s.store_add_scaled_ad_lhs(874, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1675, s.ad_value(637), p.p1677, s.ad_value(638), p.p1678, ((p.p1674) + ((s.v[636] * p.p1676)))), 639, p.p1679);
        }

        s.b[1107] = (p.p57 == 1.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if s.b[1107] {
            s.store_add_scaled_ad_lhs(882, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1808, s.ad_value(637), p.p1810, s.ad_value(638), p.p1811, ((p.p1807) + ((s.v[636] * p.p1809)))), 639, p.p1812);
            s.store_add_scaled_ad_lhs(883, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1815, s.ad_value(637), p.p1817, s.ad_value(638), p.p1818, ((p.p1814) + ((s.v[636] * p.p1816)))), 639, p.p1819);
            s.store_add_scaled_ad_lhs(884, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1822, s.ad_value(637), p.p1824, s.ad_value(638), p.p1825, ((p.p1821) + ((s.v[636] * p.p1823)))), 639, p.p1826);
            s.store_add_scaled_ad_lhs(885, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1830, s.ad_value(637), p.p1832, s.ad_value(638), p.p1833, ((p.p1829) + ((s.v[636] * p.p1831)))), 639, p.p1834);
            s.store_add_scaled_ad_lhs(886, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1836, s.ad_value(637), p.p1838, s.ad_value(638), p.p1839, ((p.p1835) + ((s.v[636] * p.p1837)))), 639, p.p1840);
            s.store_add_scaled_ad_lhs(887, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1842, s.ad_value(637), p.p1844, s.ad_value(638), p.p1845, ((p.p1841) + ((s.v[636] * p.p1843)))), 639, p.p1846);
            s.store_add_scaled_ad_lhs(888, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1854, s.ad_value(637), p.p1856, s.ad_value(638), p.p1857, ((p.p1853) + ((s.v[636] * p.p1855)))), 639, p.p1858);
            s.store_add_scaled_ad_lhs(889, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1860, s.ad_value(637), p.p1862, s.ad_value(638), p.p1863, ((p.p1859) + ((s.v[636] * p.p1861)))), 639, p.p1864);
            s.store_add_scaled_ad_lhs(890, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1870, s.ad_value(637), p.p1872, s.ad_value(638), p.p1873, ((p.p1869) + ((s.v[636] * p.p1871)))), 639, p.p1874);
            s.store_add_scaled_ad_lhs(891, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1876, s.ad_value(637), p.p1878, s.ad_value(638), p.p1879, ((p.p1875) + ((s.v[636] * p.p1877)))), 639, p.p1880);
            s.store_add_scaled_ad_lhs(892, A::add_scaled_inputs3_offset(s.ad_value(635), p.p1882, s.ad_value(637), p.p1884, s.ad_value(638), p.p1885, ((p.p1881) + ((s.v[636] * p.p1883)))), 639, p.p1886);
        }

        s.b[1108] = (p.p100 != 0.0);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if s.b[1108] {
            s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1109] = (p.p158 != 0.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if s.b[1109] {
            s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1110] = (p.p152 != 0.0);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if s.b[1110] {
            s.store_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1111] = (p.p154 != 0.0);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if s.b[1111] {
            s.store_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1112] = (p.p156 != 0.0);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if s.b[1112] {
            s.store_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1113] = (p.p428 != 0.0);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if s.b[1113] {
            s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1114] = (p.p432 != 0.0);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if s.b[1114] {
            s.store_scale(698, 698, (1.0 + ((p.p432 / p.p5) * (if (!((1.0 + (p.p5 / p.p433)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p433)) > 1e-38) { (((1.0 + (p.p5 / p.p433))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1115] = (p.p434 != 0.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if s.b[1115] {
            s.store_scale(699, 699, (1.0 + ((p.p434 / p.p5) * (if (!((1.0 + (p.p5 / p.p435)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p435)) > 1e-38) { (((1.0 + (p.p5 / p.p435))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1116] = (p.p581 != 0.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if s.b[1116] {
            s.store_scale(704, 704, (1.0 + ((p.p581 / p.p5) * (if (!((1.0 + (p.p5 / p.p584)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p584)) > 1e-38) { (((1.0 + (p.p5 / p.p584))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1117] = (p.p583 != 0.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if s.b[1117] {
            s.store_scale(706, 706, (1.0 + ((p.p583 / p.p5) * (if (!((1.0 + (p.p5 / p.p586)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p586)) > 1e-38) { (((1.0 + (p.p5 / p.p586))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1118] = (p.p21 != 0.0);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if s.b[1118] {
            s.store_mul_ad_rhs(641, 641, A::scale_offset(s.ad_value(153), ((p.p5 - p.p21) * p.p99), 1.0));
            s.store_mul_ad_rhs(673, 673, A::scale_offset(s.ad_value(153), ((p.p5 - p.p21) * p.p160), 1.0));
            s.store_mul_ad_rhs(704, 704, A::scale_offset(s.ad_value(153), ((p.p5 - p.p21) * p.p587), 1.0));
        }

        s.store_ln(154, 153);

        s.store_add_scaled_inputs(641, 641, 1.0, 153, p.p98);

        s.store_add_scaled_inputs(661, 661, 1.0, 153, p.p427);

        s.b[1119] = (p.p589 > 0.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if s.b[1119] {
            s.store_mul_sub_from_scalar_ad_rhs(704, 704, 1.0, A::mul(s.ad_value(703), A::exp_scaled_input(s.ad_value(154), (-p.p589))));
        }

        if (!s.b[1119]) {
            s.store_mul_sub_from_scalar_rhs(704, 704, 1.0, 703);
        }

        s.store_ad_value(807, A::add_scaled_inputs(s.ad_value(807), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p593))), p.p591));

        s.store_ad_value(812, A::add_scaled_inputs(s.ad_value(812), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p601))), p.p599));

        s.store_ad_value(811, A::add_scaled_inputs(s.ad_value(811), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p597))), p.p595));

        s.b[1120] = (p.p66 != 0.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if s.b[1120] {
            s.store_ad_value(815, A::add_scaled_inputs(s.ad_value(815), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p594))), p.p592));
            s.store_ad_value(818, A::add_scaled_inputs(s.ad_value(818), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p602))), p.p600));
            s.store_ad_value(816, A::add_scaled_inputs(s.ad_value(816), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p598))), p.p596));
        }

        s.b[1121] = (p.p590 > 0.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1120] && s.b[1121]) {
            s.store_mul_sub_from_scalar_ad_rhs(706, 706, 1.0, A::mul(s.ad_value(710), A::exp_scaled_input(s.ad_value(154), (-p.p590))));
        }

        if (s.b[1120] && (!s.b[1121])) {
            s.store_mul_sub_from_scalar_rhs(706, 706, 1.0, 710);
        }

        s.b[1122] = (p.p64 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if s.b[1122] {
            s.store_ad_value(853, A::add_scaled_inputs(s.ad_value(853), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p913))), p.p912));
            s.store_ad_value(852, A::add_scaled_inputs(s.ad_value(852), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p916))), p.p915));
        }

        if (!s.b[1122]) {
            s.store_ad_value(709, A::add_scaled_inputs(s.ad_value(709), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p910))), p.p909));
        }

        s.store_ad_value(792, A::add_scaled_inputs(s.ad_value(792), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p1023))), p.p1021));

        s.b[1123] = (p.p66 != 0.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if s.b[1123] {
            s.store_ad_value(796, A::add_scaled_inputs(s.ad_value(796), 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p1024)), p.p1022));
        }

        s.store_ad_value(790, A::add_scaled_inputs(s.ad_value(790), 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p445)), p.p444));

        s.b[1124] = (p.p66 != 0.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if s.b[1124] {
            s.store_ad_value(791, A::add_scaled_inputs(s.ad_value(791), 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p447)), p.p446));
        }

        s.store_ad_value(700, A::add_scaled_inputs(s.ad_value(700), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448));

        s.b[1125] = (p.p66 != 0.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if s.b[1125] {
            s.store_ad_value(701, A::add_scaled_inputs(s.ad_value(701), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448));
        }

        s.store_ad_value(679, A::add_scaled_inputs(s.ad_value(679), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p431))), p.p430));

        s.store_ad_value(698, A::add_scaled_inputs(s.ad_value(698), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436));

        s.b[1126] = (p.p66 != 0.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if s.b[1126] {
            s.store_ad_value(699, A::add_scaled_inputs(s.ad_value(699), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436));
        }

        s.store_ad_value(695, A::add_scaled_inputs(s.ad_value(695), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p439))), p.p438));

        s.store_ad_value(697, A::add_scaled_inputs(s.ad_value(697), 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p443))), p.p442));

        s.store_ad_value(702, A::add_scaled_inputs(s.ad_value(702), 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p441))), p.p440));

        s.store_ad_value(681, A::add_scaled_inputs(s.ad_value(681), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p168))), p.p167));

        s.store_ad_value(682, A::add_scaled_inputs(s.ad_value(682), 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p170))), p.p169));

        s.b[1127] = ((s.v[655] > 0.0) || (s.v[656] > 0.0));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if s.b[1127] {
            s.store_offset_scaled_ad(376, A::limited_exp_scaled_input(A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), (-1.0 / (p.p399))), p.p398, 1.0);
            s.store_mul_ad_lhs(373, A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), 376);
        }

        s.b[1130] = (s.v[576] <= 0.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.store_scalar(576, 0.05);
        }

        s.b[1135] = (s.v[641] <= 0.0);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if s.b[1135] {
            s.store_scalar(641, 4.61);
        }

        s.b[1136] = (p.p61 != 0.0);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1137] = (s.v[690] < 1e-6);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (s.b[1136] && s.b[1137]) {
            s.store_scalar(690, 1e-6);
        }

        s.b[1138] = (s.v[857] < 0.0);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if s.b[1138] {
            s.store_scalar(857, 0.01);
        }

        s.b[1139] = (s.v[576] < 0.0);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if s.b[1139] {
            s.store_scalar(576, 0.05);
        }

        s.b[1140] = (s.v[574] < 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_scalar(574, p.p1682);
        }

        s.b[1141] = (s.v[575] < 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_scalar(575, 1.2);
        }

        s.b[1142] = (s.v[644] < 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_scalar(644, 0.0);
        }

        s.b[1143] = (s.v[645] < 0.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_scalar(645, 0.0);
        }

        s.b[1144] = (s.v[679] <= 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_scalar(679, 85000.0);
        }

        s.b[1145] = (s.v[698] <= 0.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_scalar(698, 85000.0);
        }

        s.b[1146] = ((p.p66 != 0.0) && (s.v[699] <= 0.0));
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_scalar(699, 85000.0);
        }

        s.b[1147] = (s.v[670] <= 0.0);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_scalar(670, 0.6);
        }

        s.b[1148] = (s.v[671] <= 0.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_scalar(671, 0.6);
        }

        s.b[1152] = (s.v[678] <= 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_scalar(678, 1.06);
        }

        s.b[1153] = (s.v[673] < 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_scalar(673, 0.0);
        }

        s.b[1154] = (s.v[677] < 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_scalar(677, 0.0);
        }

        s.b[1155] = (s.v[803] < (-s.v[153]));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_scalar(803, 0.0);
        }

        s.b[1156] = (s.v[685] < 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scalar(685, 0.0);
        }

        s.b[1157] = (s.v[687] < 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scalar(687, 0.0);
        }

        s.b[1158] = ((p.p61 != 0.0) && (s.v[689] < 0.2));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_scalar(689, 0.2);
        }

        s.b[1159] = ((p.p61 != 0.0) && (s.v[689] > 1.2));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_scalar(689, 1.2);
        }

        s.b[1160] = (s.v[695] < 2.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_scalar(695, 2.0);
        }

        s.b[1161] = (s.v[697] < 2.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_scalar(697, 2.0);
        }

        s.b[1162] = (s.v[704] < 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if s.b[1162] {
            s.store_scalar(704, 0.03);
        }

        s.b[1163] = (s.v[807] < 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_scalar(807, 0.0);
        }

        s.b[1164] = (s.v[811] < 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_scalar(811, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1165] = (s.v[812] < 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.store_scalar(812, 0.0);
        }

        s.b[1166] = (s.v[814] < 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_scalar(814, 0.0);
        }

        s.b[1167] = (s.v[707] < 0.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_scalar(707, 0.0);
        }

        s.b[1168] = (s.v[709] < 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_scalar(709, 0.0);
        }

        s.b[1169] = (s.v[853] < 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if s.b[1169] {
            s.store_scalar(853, 0.0);
        }

        s.b[1170] = (s.v[852] < 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if s.b[1170] {
            s.store_scalar(852, 0.0);
        }

        s.b[1171] = (s.v[712] < 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if s.b[1171] {
            s.store_scalar(712, 0.0);
        }

        s.b[1172] = (s.v[711] < 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if s.b[1172] {
            s.store_scalar(711, 0.0);
        }

        s.b[1175] = (p.p66 != 0.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        s.b[1178] = (s.v[706] < 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1178]) {
            s.store_scalar(706, 0.0);
        }

        s.b[1179] = (s.v[815] < 0.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1179]) {
            s.store_scalar(815, 0.0);
        }

        s.b[1180] = (s.v[816] < 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1180]) {
            s.store_scalar(816, 0.0);
        }

        s.b[1181] = (s.v[818] < 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1181]) {
            s.store_scalar(818, 0.0);
        }

        s.b[1183] = (s.v[719] <= 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if s.b[1183] {
            s.store_scalar(719, 1.06);
        }

        s.b[1184] = (s.v[790] < 2.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if s.b[1184] {
            s.store_scalar(790, 2.0);
        }

        s.b[1185] = (p.p66 != 0.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        s.b[1186] = (s.v[791] < 2.0);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (s.b[1185] && s.b[1186]) {
            s.store_scalar(791, 2.0);
        }

        s.b[1187] = (s.v[700] < 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if s.b[1187] {
            s.store_scalar(700, 0.0);
        }

        s.b[1188] = (s.v[749] < 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if s.b[1188] {
            s.store_scalar(749, 0.0);
        }

        s.b[1189] = (s.v[763] < 0.0);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if s.b[1189] {
            s.store_scalar(763, 0.0);
        }

        s.b[1190] = (p.p69 != 0.0);
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        s.b[1191] = (s.v[726] <= 0.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if (s.b[1190] && s.b[1191]) {
            s.store_scalar(726, 3.0);
        }

        s.b[1192] = (s.v[731] <= 0.0);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if (s.b[1190] && s.b[1192]) {
            s.store_scalar(731, 1.0);
        }

        s.b[1193] = (p.p68 != 0.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        s.b[1194] = (s.v[742] <= 0.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if (s.b[1193] && s.b[1194]) {
            s.store_scalar(742, 1.0);
        }

        s.b[1195] = (s.v[736] <= 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (s.b[1193] && s.b[1195]) {
            s.store_scalar(736, 1.0);
        }

        s.b[1198] = (p.p1649 >= (s.v[153] / 2.0));
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.store_scalar(875, 0.0);
        }

        if (!s.b[1198]) {
            s.store_scalar(875, p.p1649);
        }

        s.b[1199] = (s.v[864] <= 0.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_scalar(864, 1.0);
        }

        s.b[1200] = ((p.p73 == 1.0) && (s.v[873] != 0.0));
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        s.b[1201] = (s.v[873] < 0.001);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if (s.b[1200] && s.b[1201]) {
            s.store_scalar(873, 0.0);
        }

        s.b[1211] = (p.p1680 <= 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if s.b[1211] {
            s.store_scalar(577, 1.0);
        }

        s.b[1212] = (p.p1680 > 2.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((!s.b[1211]) && s.b[1212]) {
            s.store_scalar(577, 1.0);
        }

        s.b[1213] = (s.v[648] < 0.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if s.b[1213] {
            s.store_scalar(648, 0.0);
        }

        s.b[1214] = (s.v[649] < 0.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if s.b[1214] {
            s.store_scalar(649, 0.0);
        }

        s.b[1215] = (s.v[643] < 0.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if s.b[1215] {
            s.store_scalar(643, 0.0);
        }

        s.b[1216] = (s.v[642] < 0.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if s.b[1216] {
            s.store_scalar(642, 0.0);
        }

        s.b[1217] = (s.v[650] < 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if s.b[1217] {
            s.store_scalar(650, 0.0);
        }

        s.b[1218] = (s.v[651] <= 0.02);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if s.b[1218] {
            s.store_scalar(651, 0.02);
        }

        s.b[1219] = (s.v[652] <= 0.02);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if s.b[1219] {
            s.store_scalar(652, 0.02);
        }

        s.b[1220] = (s.v[653] <= 0.02);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if s.b[1220] {
            s.store_scalar(653, 0.02);
        }

        s.b[1221] = (s.v[446] < (-p.p4));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if s.b[1221] {
            s.store_scalar(446, 0.0);
        }

        s.b[1222] = (p.p57 == 1.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = ((s.v[882] < 1.0) || (s.v[882] > 3.0));
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1223]) {
            s.store_scalar(882, 2.0);
        }

        s.b[1224] = ((s.v[883] < 1.0) || (s.v[883] > 3.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1224]) {
            s.store_scalar(883, 2.6);
        }

        s.b[1225] = ((s.v[884] < 1.0) || (s.v[884] > 3.0));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1225]) {
            s.store_scalar(884, 2.6);
        }

        s.b[1226] = (s.v[885] < 0.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1226]) {
            s.store_scalar(885, 14.0);
        }

        s.b[1227] = (s.v[886] < 0.0);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1227]) {
            s.store_scalar(886, 24.0);
        }

        s.b[1228] = (s.v[887] < 0.0);
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1228]) {
            s.store_scalar(887, 24.0);
        }

        s.b[1229] = (s.v[888] < 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1229]) {
            s.store_scalar(888, 0.139);
        }

        s.b[1230] = (s.v[889] < 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1230]) {
            s.store_scalar(889, 2.0);
        }

        s.b[1231] = (s.v[890] < 0.0);
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1231]) {
            s.store_scalar(890, 11.2);
        }

        s.b[1232] = (s.v[891] < 0.0);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1232]) {
            s.store_scalar(891, 8.02);
        }

        s.b[1233] = (s.v[892] < 0.0);
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1233]) {
            s.store_scalar(892, 6.18);
        }

        s.b[1234] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        s.b[1235] = (p.p1795 != 0.0);
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if (s.b[1234] && s.b[1235]) {
            s.store_scalar(169, (p.p1793 * ((p.p59) as f64).powf(p.p1795)));
        }

        if (s.b[1234] && (!s.b[1235])) {
            s.store_scalar(169, p.p1793);
        }

        s.b[1236] = (p.p1794 != 0.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if (s.b[1234] && s.b[1236]) {
            s.store_scalar(170, ((p.p1797 * p.p4) * ((s.v[115]) as f64).powf(p.p1794)));
        }

        if (s.b[1234] && (!s.b[1236])) {
            s.store_scalar(170, (p.p1797 * p.p4));
        }

        s.b[1237] = (p.p62 == 5.0);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        s.b[1238] = (p.p1796 != 0.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if ((s.b[1234] && s.b[1237]) && s.b[1238]) {
            s.store_scalar(171, (((p.p1798 * p.p59) * p.p43) * ((p.p56) as f64).powf(p.p1796)));
        }

        if ((s.b[1234] && s.b[1237]) && (!s.b[1238])) {
            s.store_scalar(171, ((p.p1798 * p.p59) * p.p43));
        }

        if (s.b[1234] && (!s.b[1237])) {
            s.store_scalar(171, 0.0);
        }

        if s.b[1234] {
            s.store_ad_value(633, A::add_scaled_inputs3(s.ad_value(169), 1.0 / (p.p1791), s.ad_value(170), 1.0 / (p.p1791), s.ad_value(171), 1.0 / (p.p1791)));
            s.store_ad_value(634, A::add_scaled_inputs3(s.ad_value(169), p.p1792, s.ad_value(170), p.p1792, s.ad_value(171), p.p1792));
        }

        s.b[1239] = (p.p76 != 0.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if s.b[1239] {
            s.store_scalar(457, (((p.p1074 / p.p6) + ((p.p1075 * p.p5) / (if (p.p6 == 2.0) { 12.0 } else { 3.0 }))) / p.p59));
        }

        if s.b[1239] {
            s.store_div_from_scalar_ad(456, 1.0, A::max_from_scalar(0.001, s.ad_value(457)));
        }

        s.b[1240] = (p.p76 == 2.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if (s.b[1239] && s.b[1240]) {
            s.store_scalar(458, (1.0 / (0.001_f64).max(p.p1076)));
            s.store_scalar(459, (1.0 / (0.001_f64).max(p.p1077)));
        }

        s.b[1241] = (p.p77 == 0.0);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if s.b[1241] {
            s.store_scalar(190, (p.p1078 * p.p18));
            s.store_scalar(191, (p.p1079 * p.p19));
        }

        s.b[1242] = (p.p1080 > 0.0);
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1242]) {
            s.store_scalar(444, ((p.p4 * p.p92) + ((p.p3 + ((p.p4 - p.p3) * p.p1084)) * p.p1080)));
        }

        if ((!s.b[1241]) && (!s.b[1242])) {
            s.store_scalar(444, (p.p4 * (1e-9_f64).max((p.p92 + p.p1080))));
        }

        if (!s.b[1241]) {
            s.store_offset(445, 446, p.p4);
        }

        s.b[1243] = param_given[1083];
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1243]) {
            s.store_scalar(431, p.p1083);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_scalar(429, (if (p.p60 == 1.0) { 1417.0 } else { 470.5 }));
        }

        s.b[1244] = (p.p60 == 1.0);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if (((!s.b[1241]) && (!s.b[1243])) && s.b[1244]) {
            s.store_scalar(168, (((p.p97 / 9.68e22)) as f64).powf(0.68));
            s.store_scalar(169, (3.43e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div(A::offset(s.ad_value(429), (-52.2)), A::offset(s.ad_value(168), 1.0)), 52.2), A::div_from_scalar(43.4, A::offset(A::square(s.ad_value(169)), 1.0)), 0.0001);
        }

        if (((!s.b[1241]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scalar(168, (((p.p97 / 2.23e22)) as f64).powf(0.719));
            s.store_scalar(169, (6.1e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div(A::offset(s.ad_value(429), (-44.9)), A::offset(s.ad_value(168), 1.0)), 44.9), A::div_from_scalar(29.0, A::offset(A::square(s.ad_value(169)), 1.0)), 0.0001);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_div_from_scalar_scaled_input(431, 1.0, 430, (1.60219e-19 * p.p97));
        }

        if (!s.b[1241]) {
            s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));
            s.store_min_with_scalar(432, 444, (1e-18_f64).max((p.p3 * (p.p92 + (0.0_f64).min(p.p1080)))));
            s.store_scaled_mul_ad(434, A::div(s.ad_value(431), A::tan(s.ad_value(433))), A::add_scaled_inputs3(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), 1.0, A::div_from_scalar(2.0, A::sqrt(s.ad_value(444))), (-1.0), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444)))), 1.0), 1.0 / ((((3.141592653589793) as f64).sqrt() * p.p5)));
            s.store_offset_scaled(436, 444, p.p5, p.p1092);
            s.store_offset_scaled(437, 445, p.p5, p.p1093);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[1241]) {
            s.store_sqrt_ad(435, A::div_scaled_inputs(s.ad_value(436), p.p1082, A::mul(s.ad_value(431), s.ad_value(437)), 1.0));
            s.store_div_from_scalar(438, p.p20, 435);
            s.store_limited_exp_scaled_input(168, 438, 2.0);
        }

        s.b[1245] = (p.p1086 == 1.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1245]) {
            s.store_scaled_mul(439, 431, 435, 1.0 / (p.p1082));
            s.store_mul_offset_rhs(169, 168, 439, 1.0);
            s.store_sub_ad_lhs(170, A::offset(s.ad_value(169), 1.0), 439);
            s.store_add_ad_lhs(171, A::offset(s.ad_value(169), (-1.0)), 439);
        }

        if ((!s.b[1241]) && (!s.b[1245])) {
            s.store_offset(170, 168, 1.0);
            s.store_offset(171, 168, (-1.0));
        }

        if (!s.b[1241]) {
            s.store_ad_value(440, A::div_scaled_product3(s.ad_value(431), s.ad_value(435), s.ad_value(170), 1.0, A::mul(s.ad_value(436), s.ad_value(171)), 1.0));
        }

        s.b[1246] = (p.p1080 < (-1e-10));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1246]) {
            s.store_scalar(441, (p.p1082 / (((-p.p1080) * p.p3) * p.p5)));
            s.store_ad_value(442, A::div_scaled_product(A::add(s.ad_value(440), s.ad_value(434)), s.ad_value(441), 1.0, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(434), 1.0, s.ad_value(441), 1.0), 1.0));
        }

        if ((!s.b[1241]) && (!s.b[1246])) {
            s.store_add(442, 440, 434);
        }

        if (!s.b[1241]) {
            s.store_scale(443, 442, (1.0 / (p.p59) * (0.0_f64).max(((((p.p1094 + (p.p1095 * p.p3)) + (p.p1096 * p.p4)) + (p.p1097 * p.p20)) + (p.p1098 * p.p1080)))));
            s.copy_ad(190, 443);
            s.copy_ad(191, 443);
        }

        s.b[1247] = (p.p64 == 0.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        s.b[1248] = (s.v[190] < p.p151);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (s.b[1247] && s.b[1248]) {
            s.store_scalar(190, 0.0);
        }

        s.b[1249] = (s.v[191] < p.p151);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if (s.b[1247] && s.b[1249]) {
            s.store_scalar(191, 0.0);
        }

        s.b[1250] = (s.v[190] <= p.p151);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((!s.b[1247]) && s.b[1250]) {
            s.store_scalar(190, p.p151);
        }

        s.b[1251] = (s.v[191] <= p.p151);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((!s.b[1247]) && s.b[1251]) {
            s.store_scalar(191, p.p151);
        }

        s.b[1252] = (p.p78 != 1.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        s.b[1253] = param_given[1542];
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (s.b[1252] && s.b[1253]) {
            s.store_scalar(646, p.p1542);
        }

        s.b[1254] = (param_given[85] && (p.p85 > 0.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((s.b[1252] && (!s.b[1253])) && s.b[1254]) {
            s.store_max_from_scalar_ad(646, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(648), 1.0));
        }

        s.b[1255] = (p.p78 == 3.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && s.b[1255]) {
            s.store_scale(646, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && (!s.b[1255])) {
            s.store_scale(646, 163, (0.3 * p.p3));
        }

        s.b[1256] = param_given[1543];
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (s.b[1252] && s.b[1256]) {
            s.store_scalar(647, p.p1543);
        }

        s.b[1257] = (param_given[85] && (p.p85 > 0.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((s.b[1252] && (!s.b[1256])) && s.b[1257]) {
            s.store_max_from_scalar_ad(647, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(649), 1.0));
        }

        s.b[1258] = (p.p78 == 3.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && s.b[1258]) {
            s.store_scale(647, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && (!s.b[1258])) {
            s.store_scale(647, 163, (0.3 * p.p3));
        }

        s.b[1259] = (p.p78 == 2.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if s.b[1259] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p3)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
        }

        s.b[1260] = (p.p1090 > 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1260]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1259] && s.b[1260]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(451, 168, 169, (p.p3 + ((p.p4 - p.p3) * p.p1084)));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(447), p.p90, 0.2), s.ad_value(450), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1261] = (s.v[933] > 80.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1260])) && s.b[1261]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1260])) && (!s.b[1261])) {
            let assign10160_ad_e12710: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(450), A::offset(s.ad_value(447), p.p90)), A::div(A::offset(s.ad_value(447), p.p90), s.ad_value(450))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1260])) {
            let assign10190_ad_e12788: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10190_ad_e12788, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_add(938, 934, 937, p.p3);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1259] && (!s.b[1260])) {
            let assign10280_ad_e12985: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign10280_ad_e12985, 928);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && (!s.b[1260])) {
            let assign10300_ad_e13041: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign10300_ad_e13041);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p3, s.ad_value(933), ((-0.5) * p.p3), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p3)));
            s.store_add(451, 938, 947);
        }

        s.b[1262] = (p.p1090 > 0.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1262]) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1263] = (s.v[933] > 80.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if ((s.b[1259] && s.b[1262]) && s.b[1263]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && s.b[1262]) && (!s.b[1263])) {
            let assign10470_ad_e13236: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1259] && s.b[1262]) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && s.b[1262]) {
            let assign10500_ad_e13311: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10500_ad_e13311, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1259] && s.b[1262]) {
            let assign10590_ad_e13499: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign10590_ad_e13499, 928);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && s.b[1262]) {
            let assign10610_ad_e13553: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign10610_ad_e13553);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p92, s.ad_value(933), ((-0.5) * p.p92), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p92)));
            s.store_add(452, 938, 947);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1264] = (s.v[933] > 80.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1262])) && s.b[1264]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1262])) && (!s.b[1264])) {
            let assign10770_ad_e13752: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1262])) {
            let assign10800_ad_e13830: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign10800_ad_e13830, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1259] && (!s.b[1262])) {
            let assign10890_ad_e14027: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign10890_ad_e14027, 928);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && (!s.b[1262])) {
            let assign10910_ad_e14083: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign10910_ad_e14083);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p92, s.ad_value(933), ((-0.5) * p.p92), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p92)));
            s.store_add(452, 938, 947);
        }

        s.b[1265] = (p.p1090 > 0.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1265]) {
            s.store_scalar(454, 0.0);
        }

        s.b[1266] = (p.p1080 > 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(454, ((p.p4 - p.p3) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1259] && (!s.b[1265])) && (!s.b[1266])) {
            s.store_scale(454, 450, (p.p4 - p.p3));
        }

        if s.b[1259] {
            s.store_offset_scaled(455, 454, ((p.p5) * ((s.v[144] * 1.0 / (p.p1087)))), ((((p.p1092) + (p.p1091))) * ((s.v[144] * 1.0 / (p.p1087)))));
            s.store_ad_value(453, A::add_scaled_inputs3(s.ad_value(455), p.p59, s.ad_value(451), (p.p5 * p.p59), s.ad_value(452), ((p.p1103 * (p.p5 * 2.0)) * p.p59)));
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p3)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.b[1267] = (p.p78 == 3.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if s.b[1267] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p43)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
            s.store_scalar(1031, (0.5 * p.p41));
        }

        s.b[1268] = (p.p1090 > 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1268]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1267] && s.b[1268]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(1034, 168, 169, (p.p43 + ((p.p4 - p.p43) * p.p1084)));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(447), p.p90, 0.2), s.ad_value(450), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1269] = (s.v[933] > 80.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1268])) && (!s.b[1269])) {
            let assign11250_ad_e14490: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(450), A::offset(s.ad_value(447), p.p90)), A::div(A::offset(s.ad_value(447), p.p90), s.ad_value(450))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1268])) {
            let assign11280_ad_e14568: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11280_ad_e14568, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1267] && (!s.b[1268])) {
            let assign11370_ad_e14765: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign11370_ad_e14765, 928);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1268])) {
            let assign11390_ad_e14821: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign11390_ad_e14821);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p43, s.ad_value(933), ((-0.5) * p.p43), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p43)));
            s.store_add(1034, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_from_scalar_ad(925, (0.2 * (p.p1089 + p.p90)), s.ad_value(1031), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub_from_scalar((p.p1089 + p.p90), s.ad_value(1031)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_with_scalar(929, 1031, (p.p1089 + p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1270] = (s.v[933] > 80.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1270]) {
            s.copy_ad(934, 932);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1270])) {
            let assign11550_ad_e14991: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p.p1089 + p.p90))), A::div_from_scalar((p.p1089 + p.p90), s.ad_value(1031))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            let assign11580_ad_e15060: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11580_ad_e15060, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_scale(930, 928, 1.0 / (p.p1089));
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_offset_ad(940, A::add_scaled_inputs(A::offset(A::mul(A::sqrt(A::scale_offset(s.ad_value(930), (p.p1089 * p.p1089), (((p.p1089 * p.p1089)) + (((p.p90 * p.p90) + ((2.0 * p.p1089) * p.p90)))))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(930), p.p1089), p.p1089);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if s.b[1267] {
            let assign11670_ad_e15230: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign11670_ad_e15230, 928);
        }

        if s.b[1267] {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            let assign11690_ad_e15280: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign11690_ad_e15280);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p43, s.ad_value(933), ((-0.5) * p.p43), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p43)));
            s.store_add(1035, 938, 947);
        }

        s.b[1271] = (p.p1090 > 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1271]) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1272] = (s.v[933] > 80.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if ((s.b[1267] && s.b[1271]) && s.b[1272]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && s.b[1271]) && (!s.b[1272])) {
            let assign11860_ad_e15463: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1267] && s.b[1271]) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && s.b[1271]) {
            let assign11890_ad_e15538: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign11890_ad_e15538, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1267] && s.b[1271]) {
            let assign11980_ad_e15726: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign11980_ad_e15726, 928);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && s.b[1271]) {
            let assign12000_ad_e15780: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign12000_ad_e15780);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p40, s.ad_value(933), ((-0.5) * p.p40), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p40)));
            s.store_add(1036, 938, 947);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1273] = (s.v[933] > 80.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1271])) && s.b[1273]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1271])) && (!s.b[1273])) {
            let assign12160_ad_e15979: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1271])) {
            let assign12190_ad_e16057: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12190_ad_e16057, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if (s.b[1267] && (!s.b[1271])) {
            let assign12280_ad_e16254: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign12280_ad_e16254, 928);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1271])) {
            let assign12300_ad_e16310: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign12300_ad_e16310);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p40, s.ad_value(933), ((-0.5) * p.p40), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p40)));
            s.store_add(1036, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1267] {
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1274] = (s.v[933] > 80.0);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1274]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1274])) {
            let assign12460_ad_e16480: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            let assign12490_ad_e16549: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12490_ad_e16549, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if s.b[1267] {
            let assign12580_ad_e16719: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign12580_ad_e16719, 928);
        }

        if s.b[1267] {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            let assign12600_ad_e16769: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign12600_ad_e16769);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p40, s.ad_value(933), ((-0.5) * p.p40), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p40)));
            s.store_add(1037, 938, 947);
            s.store_offset_div_ad(925, A::scaled_offset(s.ad_value(448), p.p90, 0.2), s.ad_value(449), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1275] = (s.v[933] > 80.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1275]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1275])) {
            let assign12760_ad_e16927: A = {
                if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(933))
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

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div(s.ad_value(449), A::offset(s.ad_value(448), p.p90)), A::div(A::offset(s.ad_value(448), p.p90), s.ad_value(449))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            let assign12790_ad_e16996: A = {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(937, assign12790_ad_e16996, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p42);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_product(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), A::offset(s.ad_value(930), 1.0), 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_ad_value(941, A::add_scaled_inputs(A::sqrt(A::mul(A::offset(s.ad_value(930), 1.0), A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0));
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
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
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
        }

        if s.b[1267] {
            let assign12880_ad_e17166: A = A::add_scaled_product(A::add_scaled_inputs(A::sqrt(A::mul(A::offset(A::square(s.ad_value(943)), 1.0), A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90), 1.0, A::square(s.ad_value(943)), s.ad_value(928), 1.0);
            s.store_add_ad_lhs(933, assign12880_ad_e17166, 928);
        }

        if s.b[1267] {
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            let assign12900_ad_e17216: A = A::mul(A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
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
            s.store_ad_value(946, assign12900_ad_e17216);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_ad_value(933, A::add_scaled_inputs3(s.ad_value(946), 1.0, s.ad_value(942), (-1.0), s.ad_value(627), -1.0));
            s.store_ad_value(947, A::add_scaled_inputs3(s.ad_value(946), p.p42, s.ad_value(933), ((-0.5) * p.p42), A::sqrt(A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0)), ((-0.5) * p.p42)));
            s.store_add(1038, 938, 947);
        }

        s.b[1276] = (p.p1090 > 0.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1276]) {
            s.store_scalar(1032, 0.0);
        }

        s.b[1277] = (p.p1080 > 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1276])) && s.b[1277]) {
            s.store_scalar(1032, ((p.p4 - p.p43) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1267] && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scale(1032, 450, (p.p4 - p.p43));
        }

        if s.b[1267] {
            s.store_scale(1033, 1031, (p.p4 - p.p43));
            s.store_scaled_offset_ad(455, A::add_scaled_inputs(s.ad_value(1032), p.p5, s.ad_value(1033), ((2.0 * p.p56) * p.p5)), ((p.p1092) + (p.p1091)), (s.v[144] * 1.0 / (p.p1087)));
            s.store_scaled_add_ad(453, A::add_scaled_inputs3(s.ad_value(455), 1.0, s.ad_value(1034), p.p5, s.ad_value(1035), ((2.0 * p.p56) * p.p5)), A::add_scaled_inputs3(s.ad_value(1036), (p.p1103 * (p.p5 * 2.0)), s.ad_value(1037), ((p.p56 - 1.0) * (p.p1103 * (p.p5 * 2.0))), s.ad_value(1038), (p.p1103 * (p.p5 * 2.0))), p.p59);
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p43)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.v[168] = (p.p1583 * (if (!((1.0 + (p.p92 / p.p91)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p92 / p.p91)) > 1e-38) { (((1.0 + (p.p92 / p.p91))) as f64).ln() } else { 0.0 }) }));

        s.v[515] = ((s.v[165] * p.p7) + (s.v[168] * (0.0_f64).max((p.p9 - (p.p4 * s.v[115])))));

        s.v[516] = ((s.v[165] * p.p8) + (s.v[168] * (0.0_f64).max((p.p10 - (p.p4 * s.v[115])))));

        s.b[1278] = (p.p62 != 5.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if s.b[1278] {
            s.store_scale(517, 149, (((p.p1544 * p.p59) * p.p6) + (p.p1545 * s.v[115])));
        }

        if (!s.b[1278]) {
            s.store_mul_ad_lhs(517, A::scale_offset(s.ad_value(161), ((p.p1546) * (s.v[115])), ((((p.p1545) * (s.v[115]))) + (((p.p1544 * p.p59) * p.p6)))), 149);
        }

        s.v[420] = (1e-8 / (s.v[145] * p.p89));

        s.store_div_from_scalar_scaled_ad(189, 1.0, A::pow(A::scale(s.ad_value(158), 1000000.0), s.ad_value(713)), s.v[115]);

        s.v[578] = (((((s.v[145] * p.p89) * 0.5) * p.p3)) as f64).sqrt();

        s.store_sqrt_mul_ad(351, A::div_scaled_inputs(s.ad_value(894), s.v[143], s.ad_value(893), 1.0), A::offset(A::div_scaled_product(s.ad_value(894), s.ad_value(893), 1.0, A::mul_scaled_lhs(s.ad_value(895), (2.0 * s.v[143]), s.ad_value(895)), 1.0), 1.0));

        s.b[1279] = (!param_given[172]);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if s.b[1279] {
            s.store_offset_ad(360, A::div_scaled_product(s.ad_value(670), s.ad_value(153), 1.0, s.ad_value(351), 1.0), 1e-6);
        }

        s.b[1280] = (s.v[360] < 40.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1279] && s.b[1280]) {
            s.store_div_from_scalar_offset_ad(361, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1279] && (!s.b[1280])) {
            s.store_limited_exp_neg_input(361, 360);
        }

        if (!s.b[1279]) {
            s.store_scalar(361, p.p172);
        }

        s.b[1281] = (!param_given[174]);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if s.b[1281] {
            s.store_offset_ad(360, A::div_scaled_product(s.ad_value(671), s.ad_value(153), 1.0, s.ad_value(351), 1.0), 1e-6);
        }

        s.b[1282] = (s.v[360] < 40.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if (s.b[1281] && s.b[1282]) {
            s.store_div_from_scalar_offset_ad(362, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1281] && (!s.b[1282])) {
            s.store_limited_exp_neg_input(362, 360);
        }

        if (!s.b[1281]) {
            s.store_scalar(362, p.p174);
        }

        s.b[1283] = (!param_given[173]);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if s.b[1283] {
            s.store_offset_ad(360, A::div_scaled_product(s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0), 1e-6);
        }

        s.b[1284] = (s.v[360] < 40.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1283] && s.b[1284]) {
            s.store_div_from_scalar_offset_ad(363, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1283] && (!s.b[1284])) {
            s.store_limited_exp_neg_input(363, 360);
        }

        if (!s.b[1283]) {
            s.store_scalar(363, p.p173);
        }

        s.store_offset_sqrt_ad(364, A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0), (-1.0));

        s.store_offset_ad(360, A::div_scaled_product(s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0), 1e-6);

        s.b[1285] = (s.v[360] < 40.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if s.b[1285] {
            s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(360)), p.p171, (((((-2.0)) * (p.p171))) + (1.0))), 1e-6));
        }

        if (!s.b[1285]) {
            s.store_div_ad(365, A::limited_exp_scaled_input(s.ad_value(360), -1.0), A::max_with_scalar(A::offset(A::limited_exp_scaled_input(s.ad_value(360), -1.0), p.p171), 1e-6));
        }

        s.store_ad_value(396, A::div_scaled_product(s.ad_value(640), s.ad_value(894), 1.60219e-19, s.ad_value(893), 1.0));

        s.b[1286] = (p.p60 == 1.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if s.b[1286] {
            s.store_scalar(484, 4.97232e-7);
            s.store_scalar(485, 745669000000.0);
        }

        if (!s.b[1286]) {
            s.store_scalar(484, 3.42537e-7);
            s.store_scalar(485, 1166450000000.0);
        }

        s.v[168] = (p.p1109 * p.p1109);

        s.store_scale(169, 742, p.p1109);

        s.store_square(170, 169);

        s.store_scale_ad(486, A::pow_from_scalar((p.p1108 / p.p1109), s.ad_value(741)), 1.0 / (s.v[168]));

        s.store_div_ad_lhs(487, A::pow(A::div_from_scalar(p.p1108, s.ad_value(169)), s.ad_value(741)), 170);

        s.store_mul3_lhs(463, 158, 484, 487);

        s.b[1287] = (p.p1717 < (-273.15));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1287] {
            s.store_scalar(228, 300.15);
        }

        if (!s.b[1287]) {
            s.store_scalar(228, (p.p1717 + 273.15));
        }

        s.b[1288] = (p.p57 == 1.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if s.b[1288] {
            s.store_ad_value(960, A::add_scaled_inputs(A::sub_from_scalar(p.p1806, s.ad_value(882)), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), s.ad_value(882), 1.0));
        }

        if s.b[1288] {
            s.store_ad_value(961, A::add_scaled_inputs(A::sub_from_scalar(p.p1813, s.ad_value(883)), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), s.ad_value(883), 1.0));
        }

        if s.b[1288] {
            s.store_ad_value(962, A::add_scaled_inputs(A::sub_from_scalar(p.p1820, s.ad_value(884)), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), s.ad_value(884), 1.0));
        }

        if s.b[1288] {
            let assign13560_ad_e17880: A = A::mul(A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));
            s.store_scaled_add_ad(963, A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::sqrt(A::offset(assign13560_ad_e17880, ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1288] {
            let assign13570_ad_e17951: A = A::mul(A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));
            s.store_scaled_add_ad(964, A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::sqrt(A::offset(assign13570_ad_e17951, ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1288] {
            let assign13580_ad_e18022: A = A::mul(A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));
            s.store_scaled_add_ad(965, A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), A::sqrt(A::offset(assign13580_ad_e18022, ((0.25 * 0.001) * 0.001))), 0.5);
        }

        if s.b[1288] {
            let assign13590_ad_e18138: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), A::scaled_offset(s.ad_value(960), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), A::scaled_offset(s.ad_value(960), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(966, A::add_scaled_inputs3_offset(s.ad_value(960), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), A::scaled_offset(s.ad_value(960), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13590_ad_e18138, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13600_ad_e18258: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), A::scaled_offset(s.ad_value(960), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), A::scaled_offset(s.ad_value(960), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(969, A::add_scaled_inputs3_offset(s.ad_value(960), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), A::scaled_offset(s.ad_value(960), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13600_ad_e18258, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13610_ad_e18378: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), A::scaled_offset(s.ad_value(961), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), A::scaled_offset(s.ad_value(961), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(967, A::add_scaled_inputs3_offset(s.ad_value(961), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), A::scaled_offset(s.ad_value(961), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13610_ad_e18378, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13620_ad_e18498: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), A::scaled_offset(s.ad_value(961), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), A::scaled_offset(s.ad_value(961), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(970, A::add_scaled_inputs3_offset(s.ad_value(961), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), A::scaled_offset(s.ad_value(961), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13620_ad_e18498, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13630_ad_e18618: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), A::scaled_offset(s.ad_value(962), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), A::scaled_offset(s.ad_value(962), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(968, A::add_scaled_inputs3_offset(s.ad_value(962), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), A::scaled_offset(s.ad_value(962), (-1.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13630_ad_e18618, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13640_ad_e18738: A = A::mul(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), A::scaled_offset(s.ad_value(962), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)), A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), A::scaled_offset(s.ad_value(962), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), 0.5), (-1.0)));
            s.store_offset_ad(971, A::add_scaled_inputs3_offset(s.ad_value(962), ((0.5 * 1.001) * 0.5), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), A::scaled_offset(s.ad_value(962), (-2.001), 1.001)), ((0.25 * 0.001) * 0.001))), (0.5 * 0.5), A::sqrt(A::offset(assign13640_ad_e18738, ((0.25 * 0.001) * 0.001))), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5)), (0.25 * 0.001));
        }

        if s.b[1288] {
            s.store_mul_ad(976, A::pow(s.ad_value(158), s.ad_value(966)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)));
            s.store_div(979, 976, 893);
            s.store_mul_ad(977, A::pow(s.ad_value(158), s.ad_value(967)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)));
            s.store_div(980, 977, 893);
            s.store_mul_ad(978, A::pow(s.ad_value(158), s.ad_value(968)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)));
            s.store_div(981, 978, 893);
        }

        if s.b[1288] {
            s.store_scalar(982, (0.5 * (((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) + 0.5) + ((((((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5) * ((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));
        }

        if s.b[1288] {
            s.store_add_ad_lhs(983, A::div_scaled_product(A::sub_from_scalar(1.0, s.ad_value(982)), A::sub(s.ad_value(960), s.ad_value(882)), 1.0, A::sub_from_scalar(p.p1806, s.ad_value(882)), 1.0), 982);
            s.store_div_from_scalar_offset_ad(984, 1.0, A::limited_exp_scaled_input(A::offset(s.ad_value(983), (-0.999)), 1.0 / (0.0001)), 1.0);
            s.store_scalar(1013, (((((0.5 * p.p40) * p.p40) * 1e18) - ((1.5 * p.p40) * 1000000000.0)) + 2.0));
            s.store_offset_ad(1014, A::sub_scaled_inputs(A::offset(s.ad_value(1013), 4.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1013), (-4.0)), A::offset(s.ad_value(1013), (-4.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            let assign13760_ad_e19004: A = A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0))))), ((0.25 * 0.01) * 0.01));
            let assign13760_ad_e19074: A = A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0))))), ((0.25 * 0.01) * 0.01));
            let assign13760_ad_e19144: A = A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0))))), ((0.25 * 0.01) * 0.01));
            let assign13760_ad_e19150: A = A::mul(A::offset(A::add_scaled_inputs(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + (18100.0))), 0.5, A::sqrt(assign13760_ad_e19074), 0.5), (-924000.0)), A::offset(A::add_scaled_inputs(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + (18100.0))), 0.5, A::sqrt(assign13760_ad_e19144), 0.5), (-924000.0)));
            s.store_offset_ad(974, A::add_scaled_inputs3_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893), ((0.5 * ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))) * 0.5), A::sqrt(assign13760_ad_e19004), (0.5 * 0.5), A::sqrt(A::offset(assign13760_ad_e19150, ((0.25 * 9240.0) * 9240.0))), (-0.5), ((924000.0 + (0.5 * ((s.v[168]) + (18100.0)))) * 0.5)), (0.25 * 9240.0));
        }

        if s.b[1288] {
            let assign13770_ad_e19234: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5)), ((0.25 * 0.01) * 0.01))));
            let assign13770_ad_e19304: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5)), ((0.25 * 0.01) * 0.01))));
            let assign13770_ad_e19374: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894), ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5)), ((0.25 * 0.01) * 0.01))));
            s.store_offset_ad(975, A::sub_scaled_inputs(A::scale_offset(assign13770_ad_e19234, 0.5, 8.0), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(assign13770_ad_e19304, 0.5, (-8.0)), A::scale_offset(assign13770_ad_e19374, 0.5, (-8.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p.p1895)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1895)));
            s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p.p1896)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1896)));
            s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p.p1897)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1897)));
        }

        if s.b[1288] {
            let assign13810_ad_e19486: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + (0.5))), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5))))), ((0.25 * 0.01) * 0.01))));
            let assign13810_ad_e19538: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + (0.5))), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5))))), ((0.25 * 0.01) * 0.01))));
            let assign13810_ad_e19590: A = A::add(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + (0.5))), A::sqrt(A::offset(A::mul(A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5)))), A::scale_offset(A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898), 0.1, ((0.7) + ((-0.5))))), ((0.25 * 0.01) * 0.01))));
            s.store_offset_ad(990, A::sub_scaled_inputs(A::scale_offset(assign13810_ad_e19486, 0.5, 1.0), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(assign13810_ad_e19538, 0.5, (-1.0)), A::scale_offset(assign13810_ad_e19590, 0.5, (-1.0))), ((0.25 * 0.01) * 0.01))), 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p.p1899)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1899)));
            s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p.p1900)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1900)));
            s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p.p1901)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1901)));
            s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p.p1902)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1902)));
            s.store_div_ad_rhs(987, 974, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(975), p.p1867)));
            s.store_div_ad_rhs(988, 972, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(973), p.p1868)));
        }

        if s.b[1288] {
            let assign13880_ad_e19743: A = A::mul(A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), p.p1865, s.ad_value(987), (-p.p1865)), A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), p.p1865, s.ad_value(987), (-p.p1865)));
            s.store_scaled_add_ad(985, A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867))), p.p1865, s.ad_value(987), (-p.p1865)), A::sqrt(A::offset(assign13880_ad_e19743, ((0.25 * 0.01) * 0.01))), 0.5);
        }

        if s.b[1288] {
            let assign13890_ad_e19808: A = A::mul(A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), p.p1866, s.ad_value(988), (-p.p1866)), A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), p.p1866, s.ad_value(988), (-p.p1866)));
            s.store_scaled_add_ad(986, A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868))), p.p1866, s.ad_value(988), (-p.p1866)), A::sqrt(A::offset(assign13890_ad_e19808, ((0.25 * 0.01) * 0.01))), 0.5);
        }

        if s.b[1288] {
            let assign13900_ad_e19881: A = A::add(A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5)), A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(995, assign13900_ad_e19881, 0.5);
        }

        if s.b[1288] {
            let assign13910_ad_e19946: A = A::add(A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5)), A::div(s.ad_value(989), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(996, assign13910_ad_e19946, 0.5);
        }

        if s.b[1288] {
            s.store_ad_value(997, A::add_scaled_inputs3(s.ad_value(890), 1.0, s.ad_value(995), p.p1887, s.ad_value(996), (-p.p1887)));
        }

        if s.b[1288] {
            let assign13930_ad_e20021: A = A::add(A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5)), A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(998, assign13930_ad_e20021, 0.5);
        }

        if s.b[1288] {
            let assign13940_ad_e20086: A = A::add(A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5)), A::div(s.ad_value(991), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(999, assign13940_ad_e20086, 0.5);
        }

        if s.b[1288] {
            s.store_ad_value(1000, A::add_scaled_inputs3(s.ad_value(891), 1.0, s.ad_value(998), p.p1888, s.ad_value(999), (-p.p1888)));
        }

        if s.b[1288] {
            let assign13960_ad_e20161: A = A::add(A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5)), A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(1001, assign13960_ad_e20161, 0.5);
        }

        if s.b[1288] {
            let assign13970_ad_e20226: A = A::add(A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5)), A::div(s.ad_value(993), A::powf(A::scale_offset(A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892)), 5.0, 1.0), 0.5))), ((0.25 * 0.1) * 0.1))));
            s.store_scale_ad(1002, assign13970_ad_e20226, 0.5);
        }

        if s.b[1288] {
            s.store_ad_value(1003, A::add_scaled_inputs3(s.ad_value(892), 1.0, s.ad_value(1001), p.p1889, s.ad_value(1002), (-p.p1889)));
        }

        if s.b[1288] {
            let assign13990_ad_e20305: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::exp_scaled_input(A::scale_offset(s.ad_value(960), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 6.0), 6.9583e-5), 1.0, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 5.0), 0.0006583);
            let assign13990_ad_e20359: A = A::add_scaled_inputs3_offset(A::add_scaled_inputs3(assign13990_ad_e20305, 1.0, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 4.0), 0.0065, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 3.0), (-0.026)), 1.0, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 2.0), 0.1371, s.ad_value(960), (-((0.5) * ((0.194 * 2.0)))), (-((((1.0) + ((-1.0)))) * ((0.194 * 2.0)))));
            s.store_mul_ad_lhs(1010, A::mul3_scaled_output(s.ad_value(960), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(960), 0.5)), A::offset(assign13990_ad_e20359, 0.959)), A::pow(A::scale(s.ad_value(997), 1000000.0), s.ad_value(960)), (1.0 / (2.0) * 1.60219e-19)), 979);
        }

        if s.b[1288] {
            let assign14000_ad_e20439: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::exp_scaled_input(A::scale_offset(s.ad_value(961), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 6.0), 6.9583e-5), 1.0, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 5.0), 0.0006583);
            let assign14000_ad_e20493: A = A::add_scaled_inputs3_offset(A::add_scaled_inputs3(assign14000_ad_e20439, 1.0, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 4.0), 0.0065, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 3.0), (-0.026)), 1.0, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 2.0), 0.1371, s.ad_value(961), (-((0.5) * ((0.194 * 2.0)))), (-((((1.0) + ((-1.0)))) * ((0.194 * 2.0)))));
            s.store_mul_ad_lhs(1011, A::mul3_scaled_output(s.ad_value(961), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(961), 0.5)), A::offset(assign14000_ad_e20493, 0.959)), A::pow(A::scale(s.ad_value(1000), 1000000.0), s.ad_value(961)), (1.0 / (2.0) * 1.60219e-19)), 980);
        }

        if s.b[1288] {
            let assign14010_ad_e20573: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::exp_scaled_input(A::scale_offset(s.ad_value(962), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 6.0), 6.9583e-5), 1.0, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 5.0), 0.0006583);
            let assign14010_ad_e20627: A = A::add_scaled_inputs3_offset(A::add_scaled_inputs3(assign14010_ad_e20573, 1.0, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 4.0), 0.0065, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 3.0), (-0.026)), 1.0, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0))), 2.0), 0.1371, s.ad_value(962), (-((0.5) * ((0.194 * 2.0)))), (-((((1.0) + ((-1.0)))) * ((0.194 * 2.0)))));
            s.store_mul_ad_lhs(1012, A::mul3_scaled_output(s.ad_value(962), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(assign14010_ad_e20627, 0.959)), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962)), (1.0 / (2.0) * 1.60219e-19)), 981);
        }

        s.b[1289] = (p.p58 == 1.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if s.b[1289] {
            let assign14030_ad_e20663: A = A::scale_offset(s.ad_value(707), 1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0)), (((((-p.p889)) * (1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0))))) + (p.p889)));
            s.store_ad_value(707, assign14030_ad_e20663);
        }

        if s.b[1289] {
            s.store_offset(1024, 807, (((-p.p892)) + ((-((p.p893 * 1000000000.0) * p.p894)))));
        }

        if s.b[1289] {
            s.store_scaled_offset(1025, 1024, ((p.p40 * 1000000000.0) * p.p894), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
        }

        if s.b[1289] {
            s.store_ad_value(807, A::add_scaled_inputs3_offset(s.ad_value(1025), 0.5, s.ad_value(807), 0.5, A::sqrt(A::offset(A::mul(A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2))), ((0.25 * 0.6) * 0.6))), (-0.5), ((p.p892 + 0.2) * 0.5)));
        }

        if s.b[1289] {
            let assign14070_ad_e20766: A = A::add_scaled_inputs3_offset(s.ad_value(811), (-(370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898)))), s.ad_value(811), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), s.ad_value(811), 1.0, (((370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898))) * p.p897) + (1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) * p.p897)));
            s.store_ad_value(1026, assign14070_ad_e20766);
        }

        if s.b[1289] {
            s.store_scaled_sub_ad(811, A::offset(s.ad_value(1026), p.p897), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1026), (-p.p897)), A::offset(s.ad_value(1026), (-p.p897))), ((0.25 * 0.2) * 0.2))), 0.5);
            s.store_scalar(1027, (p.p43 / (p.p43 + p.p40)));
            s.store_scalar(1028, ((((p.p905 * p.p40) * p.p40) * 1e18) - (p.p906 * 0.001)));
            s.store_scaled_add_ad_rhs(1029, 1028, A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p.p906) * 0.001) * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)), 0.5), 1.0 / (((((2.0 * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)));
        }

        if s.b[1289] {
            let assign14120_ad_e20896: A = A::sub_scaled_inputs(A::offset(A::div_from_scalar(0.0001, A::offset(s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))))), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(0.0001, A::offset(s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))))), (-1.0)), A::offset(A::div_from_scalar(0.0001, A::offset(s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))))), (-1.0))), ((0.25 * 0.06) * 0.06))), 0.5);
            s.store_ad_value(1030, assign14120_ad_e20896);
        }

        if s.b[1289] {
            s.store_mul_ad_product_lhs(704, s.ad_value(704), A::add_scaled_inputs(s.ad_value(1027), 1.0, A::sub_from_scalar(1.0, s.ad_value(1027)), p.p904), 1030);
            s.store_ad_value(812, A::add_scaled_inputs(A::sub_from_scalar(p.p901, s.ad_value(812)), (((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903), s.ad_value(812), 1.0));
        }

        s.b[1290] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if s.b[1290] {
            s.store_offset_voltage(116, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p22)));
        }

        if (!s.b[1290]) {
            s.store_scalar(116, (ctx_temp + p.p22));
        }

        s.store_div(229, 116, 228);

        s.store_offset(230, 229, (-1.0));

        s.store_sub(232, 116, 228);

        s.store_scale(179, 116, 8.617087e-5);

        s.store_scale(180, 228, 8.617087e-5);

        s.v[121] = p.p1786;

        s.b[1291] = (p.p80 != 0.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if s.b[1291] {
            s.store_scaled_add_ad(119, A::offset(s.ad_value(116), s.v[121]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(116), (-s.v[121])), A::offset(s.ad_value(116), (-s.v[121]))), ((0.25 * p.p1788) * p.p1788))), 0.5);
            s.store_scaled_add_ad(120, A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790)), A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790))), ((0.25 * p.p1789) * p.p1789))), 0.5);
        }

        s.b[1292] = (p.p80 == 1.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (s.b[1291] && s.b[1292]) {
            s.store_scaled_add_ad(169, A::offset(s.ad_value(228), s.v[121]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-s.v[121])), A::offset(s.ad_value(228), (-s.v[121]))), ((0.25 * p.p1788) * p.p1788))), 0.5);
            s.store_scaled_add_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790))), ((0.25 * p.p1789) * p.p1789))), 0.5);
        }

        s.b[1293] = (s.v[228] > s.v[121]);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((s.b[1291] && s.b[1292]) && s.b[1293]) {
            s.store_ad_value(171, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0), 1.0, s.ad_value(170), (-1.0), s.ad_value(228), 1.0));
        }

        if ((s.b[1291] && s.b[1292]) && (!s.b[1293])) {
            s.store_offset_sub_ad(171, A::add_scaled_inputs3(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0), s.ad_value(170), s.v[121]);
        }

        if (s.b[1291] && s.b[1292]) {
            s.store_ad_value(118, A::add_scaled_inputs3(s.ad_value(116), 0.5, s.ad_value(171), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2))), 0.5));
        }

        s.b[1294] = (s.v[121] > 210.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1292])) && s.b[1294]) {
            s.store_scalar(121, 210.0);
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_offset_scaled_ad(312, A::tanh_scaled_input(A::offset(s.ad_value(116), (-210.0)), 0.5), 0.5, 0.5);
            s.store_sub_from_scalar(313, 1.0, 312);
        }

        s.b[1295] = (s.v[228] > 210.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1292])) && s.b[1295]) {
            s.store_scaled_add_ad(169, A::offset(s.ad_value(121), 210.0), A::sqrt(A::offset(A::mul(A::sub_from_scalar(210.0, s.ad_value(121)), A::sub_from_scalar(210.0, s.ad_value(121))), ((0.25 * p.p1788) * p.p1788))), 0.5);
            s.store_scalar(170, (0.5 * (((-p.p1790) * (210.0 - p.p1787)) + ((((((-p.p1790) * (210.0 - p.p1787)) * ((-p.p1790) * (210.0 - p.p1787))) + ((0.25 * p.p1789) * p.p1789))) as f64).sqrt())));
            s.store_offset_sub_ad(171, A::add_scaled_inputs3(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0), s.ad_value(170), 210.0);
            s.store_ad_value(118, A::add_scaled_inputs3(s.ad_value(116), 0.5, s.ad_value(171), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2))), 0.5));
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_ad_value(169, A::add_scaled_inputs3(s.ad_value(228), 0.5, s.ad_value(121), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(228), s.ad_value(121)), A::sub(s.ad_value(228), s.ad_value(121))), ((0.25 * p.p1788) * p.p1788))), 0.5));
            s.store_scaled_add_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), A::sqrt(A::offset(A::mul(A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790))), ((0.25 * p.p1789) * p.p1789))), 0.5);
        }

        s.b[1296] = (s.v[228] > s.v[121]);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && s.b[1296]) {
            s.store_ad_value(171, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0), 1.0, s.ad_value(170), (-1.0), s.ad_value(228), 1.0));
        }

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_ad_value(171, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0), 1.0, s.ad_value(170), (-1.0), s.ad_value(121), 1.0));
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_ad_value(172, A::add_scaled_inputs3(s.ad_value(116), 0.5, s.ad_value(171), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(116), s.ad_value(171)), A::sub(s.ad_value(116), s.ad_value(171))), ((0.25 * 0.2) * 0.2))), 0.5));
            s.store_ad_value(118, A::add_scaled_products(s.ad_value(313), s.ad_value(172), 1.0, s.ad_value(312), s.ad_value(116), 1.0));
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_scaled_sub_ad(117, A::offset(s.ad_value(116), 210.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(116), (-210.0)), A::offset(s.ad_value(116), (-210.0))), ((0.25 * 0.2) * 0.2))), 0.5);
            s.store_ad_value(233, A::add_scaled_inputs3_offset(s.ad_value(117), 1.0, s.ad_value(228), (-0.5), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-210.0)), A::offset(s.ad_value(228), (-210.0))), ((0.25 * 0.2) * 0.2))), (-(-0.5)), ((-0.5) * 210.0)));
            s.store_div_ad_lhs(234, A::offset(s.ad_value(117), (-210.0)), 228);
        }

        if s.b[1291] {
            s.store_scale(182, 118, 8.617087e-5);
        }

        s.store_sub_from_scalar_ad(146, p.p106, A::div_scaled_product(s.ad_value(116), s.ad_value(116), p.p1718, A::offset(s.ad_value(116), p.p1719), 1.0));

        s.store_sub_from_scalar_ad(147, p.p106, A::div_scaled_product(s.ad_value(228), s.ad_value(228), p.p1718, A::offset(s.ad_value(228), p.p1719), 1.0));

        s.store_mul_scaled_ad_rhs(169, 116, 1.0 / (300.15), A::sqrt_scaled_input(s.ad_value(116), 1.0 / (300.15)));

        s.store_mul_scaled_ad_rhs(141, 169, p.p105, A::limited_exp(A::sub_from_scalar((p.p106 / ((2.0 * 8.617087e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0))));

        s.b[1297] = (p.p80 == 0.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if s.b[1297] {
            s.store_scale(148, 169, p.p107);
        }

        if (!s.b[1297]) {
            s.store_mul_scaled_ad_rhs(148, 118, (1.0 / (300.15) * p.p107), A::sqrt_scaled_input(s.ad_value(118), 1.0 / (300.15)));
        }

        if (!s.b[1297]) {
            let assign14610_ad_e21688: A = A::sub(A::offset({
                if (!((p.p105 * s.v[169]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((p.p105 * s.v[169]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(169), p.p105)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (p.p106 / ((2.0 * 8.617087e-5) * 300.15))), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0));
            s.store_ad_value(142, assign14610_ad_e21688);
        }

        if (!(((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_ad(235, A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5);
        } else {
            if (((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_offset_ad(235, ((-0.001) * 0.001), A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6))));
            } else {
                s.store_scalar(235, 0.0);
            }
        }

        s.store_scale(389, 179, 1.60219e-19);

        s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0));

        s.store_scaled_square(377, 168, 1.0 / ((2.0 * s.v[381])));

        s.store_scaled_square(378, 168, 1.0 / ((2.0 * s.v[382])));

        s.store_scale(379, 377, 4.0);

        s.store_scale(380, 378, 4.0);

        s.v[169] = ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383]));

        s.store_offset_scaled_ad(387, A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(378)), s.ad_value(389))), s.v[169], 1.0);

        s.store_ad_value(388, A::add_scaled_inputs3(s.ad_value(387), 1.0, A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(379)), s.ad_value(389))), 1.0, A::limited_exp(A::div(A::sub(s.ad_value(377), s.ad_value(380)), s.ad_value(389))), s.v[169]));

        let assign14720_ad_e21904: A = {
    if (!((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38)) {
        A::neg(A::constant(87.498233534))
    } else {
        let assign14720_ad_e21903: A = {
            if ((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38) {
                A::ln(A::mul(A::div_scaled_product(A::div_from_scalar((s.v[386] * s.v[383]), A::scale(s.ad_value(148), ((3.141592653589793 * 1.05457e-34) * 1.05457e-34))), s.ad_value(389), 1.0, A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), 1.0), s.ad_value(388)))
            } else {
                A::constant(0.0)
            }
        };
        assign14720_ad_e21903
    }
};
        s.store_mul_scaled_ad_rhs(170, 179, -1.0, assign14720_ad_e21904);

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_ad_rhs(375, 654, A::add_scaled_inputs(s.ad_value(377), 6.241457005723417e18, s.ad_value(170), 1.0));

        s.store_ln(418, 229);

        s.b[1298] = (p.p80 == 0.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(s.ad_value(836), s.ad_value(418)));
        }

        if s.b[1298] {
            let assign14770_ad_e21975: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(413, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign14770_ad_e21975, 0.5));
        }

        s.b[1299] = (p.p66 == 1.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1299]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(s.ad_value(845), s.ad_value(418)));
        }

        if (s.b[1298] && s.b[1299]) {
            let assign14800_ad_e22045: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(321, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign14800_ad_e22045, 0.5));
        }

        if (s.b[1298] && s.b[1299]) {
            s.copy_ad(417, 321);
        }

        if s.b[1298] {
            let assign14820_ad_e22096: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(807), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(303, A::add_scaled_inputs3(s.ad_value(807), 1.0, assign14820_ad_e22096, 1.0, s.ad_value(807), (-1.0)));
        }

        if s.b[1298] {
            s.copy_ad(323, 811);
        }

        s.b[1300] = (p.p66 != 0.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1300]) {
            let assign14850_ad_e22149: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(815), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(305, A::add_scaled_inputs3(s.ad_value(815), 1.0, assign14850_ad_e22149, 1.0, s.ad_value(815), (-1.0)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(s.ad_value(830), s.ad_value(418)));
        }

        s.b[1301] = (p.p66 != 0.0);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1301]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(s.ad_value(844), s.ad_value(418)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(s.ad_value(834), s.ad_value(418)));
        }

        if s.b[1298] {
            let assign14900_ad_e22258: A = {
                if (!(((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(194, assign14900_ad_e22258);
        }

        s.b[1302] = (p.p75 != 0.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1302]) {
            let assign14920_ad_e22308: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(679), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(332, A::add_scaled_inputs3(s.ad_value(679), 1.0, assign14920_ad_e22308, 1.0, s.ad_value(679), (-1.0)));
        }

        if (s.b[1298] && (!s.b[1302])) {
            let assign14930_ad_e22395: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign14930_ad_e22395);
        }

        s.b[1303] = (p.p66 != 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        s.b[1304] = (p.p75 != 0.0);
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1303]) && s.b[1304]) {
            let assign14960_ad_e22451: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(680), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(333, A::add_scaled_inputs3(s.ad_value(680), 1.0, assign14960_ad_e22451, 1.0, s.ad_value(680), (-1.0)));
        }

        if ((s.b[1298] && s.b[1303]) && (!s.b[1304])) {
            let assign14970_ad_e22540: A = {
                if (!(((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign14970_ad_e22540);
        }

        s.b[1305] = (s.v[333] < 1000.0);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1303]) && s.b[1305]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1306] = (p.p67 == 1.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1306]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(s.ad_value(839), s.ad_value(418)));
        }

        if (s.b[1298] && s.b[1306]) {
            let assign15020_ad_e22619: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(414, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign15020_ad_e22619, 0.5));
        }

        if (s.b[1298] && s.b[1306]) {
            let assign15030_ad_e22666: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(808), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(304, A::add_scaled_inputs3(s.ad_value(808), 1.0, assign15030_ad_e22666, 1.0, s.ad_value(808), (-1.0)));
        }

        if (s.b[1298] && s.b[1306]) {
            s.store_mul_exp_ad_rhs(319, 813, A::mul(s.ad_value(832), s.ad_value(418)));
        }

        s.b[1307] = (p.p75 != 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1307]) {
            let assign15060_ad_e22729: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(698), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(334, A::add_scaled_inputs3(s.ad_value(698), 1.0, assign15060_ad_e22729, 1.0, s.ad_value(698), (-1.0)));
        }

        if (s.b[1298] && (!s.b[1307])) {
            let assign15070_ad_e22816: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign15070_ad_e22816);
        }

        s.b[1308] = (p.p66 != 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        s.b[1309] = (p.p75 != 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1308]) && s.b[1309]) {
            let assign15100_ad_e22872: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(699), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(699), 1.0, assign15100_ad_e22872, 1.0, s.ad_value(699), (-1.0)));
        }

        if ((s.b[1298] && s.b[1308]) && (!s.b[1309])) {
            let assign15110_ad_e22961: A = {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign15110_ad_e22961);
        }

        s.b[1310] = (s.v[335] < 1000.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1308]) && s.b[1310]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1311] = (p.p75 != 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1311]) {
            let assign15150_ad_e23023: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(702), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(702), 1.0, assign15150_ad_e23023, 1.0, s.ad_value(702), (-1.0)));
        }

        if (s.b[1298] && (!s.b[1311])) {
            let assign15160_ad_e23110: A = {
                if (!(((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign15160_ad_e23110);
        }

        if s.b[1298] {
            let assign15170_ad_e23198: A = {
                if (!(((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(337, assign15170_ad_e23198, 2.0);
        }

        s.b[1312] = (p.p66 != 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1312]) {
            let assign15190_ad_e23292: A = {
                if (!(((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(338, assign15190_ad_e23292, 2.0);
        }

        if s.b[1298] {
            s.copy_ad(660, 657);
            s.copy_ad(797, 792);
            s.store_mul_add_ad_lhs(231, s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153)), 230);
        }

        s.b[1313] = (p.p80 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            let assign15250_ad_e23385: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(413, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign15250_ad_e23385, 0.5));
        }

        s.b[1314] = (p.p66 == 1.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {
            let assign15280_ad_e23465: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(321, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign15280_ad_e23465, 0.5));
        }

        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {
            s.copy_ad(417, 321);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(303, 807, A::mul(A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1315] = (p.p66 != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1315]) {
            s.store_mul_exp_ad_rhs(305, 815, A::mul(A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1316] = (p.p66 != 0.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1316]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_inputs(s.ad_value(834), 1.0, s.ad_value(229), p.p881), s.ad_value(418)));
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), (-1.0));
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), (-1.0));
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1317] = (p.p75 != 0.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1317]) {
            let assign15420_ad_e23663: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(811), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(323, A::add_scaled_inputs3(s.ad_value(811), 1.0, assign15420_ad_e23663, 1.0, s.ad_value(811), (-1.0)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1317])) {
            let assign15430_ad_e23747: A = {
                if (!(((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(323, 811, assign15430_ad_e23747);
        }

        s.b[1318] = (p.p67 == 1.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {
            let assign15460_ad_e23825: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(414, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign15460_ad_e23825, 0.5));
        }

        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {
            s.store_mul_exp_ad_rhs(304, 808, A::mul(A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1319] = (s.v[854] == s.v[855]);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1319]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1320] = (s.v[856] < s.v[228]);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_ad_value(196, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.b[1321] = (s.v[855] < s.v[854]);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && s.b[1321]) {
            let assign15560_ad_e23998: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(170, assign15560_ad_e23998);
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && (!s.b[1321])) {
            let assign15570_ad_e24053: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(170, assign15570_ad_e24053);
        }

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_offset_mul_ad(196, s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228)), 1.0);
            s.store_ad_value(195, A::add_scaled_product(A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.b[1322] = (s.v[855] < s.v[854]);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && s.b[1322]) {
            let assign15620_ad_e24174: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(196), s.ad_value(195)), A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(170, assign15620_ad_e24174);
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && (!s.b[1322])) {
            let assign15630_ad_e24230: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(196), s.ad_value(195)), A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(170, assign15630_ad_e24230);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            let assign15640_ad_e24284: A = {
                if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(170), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(170), (-1e-6)), A::offset(s.ad_value(170), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
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
            s.store_ad_value(194, assign15640_ad_e24284);
        }

        s.b[1323] = (p.p75 != 0.0);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1323]) {
            let assign15660_ad_e24352: A = A::add_scaled_product(s.ad_value(679), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(332, A::add_scaled_inputs3(s.ad_value(679), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(assign15660_ad_e24352), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(679), (-1.0)));
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1323])) {
            let assign15670_ad_e24481: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15670_ad_e24439: A = A::add(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15670_ad_e24439, 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign15670_ad_e24481);
        }

        s.b[1324] = (p.p66 != 0.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        s.b[1325] = (p.p75 != 0.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1325]) {
            let assign15700_ad_e24555: A = A::add_scaled_product(s.ad_value(680), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(333, A::add_scaled_inputs3(s.ad_value(680), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(assign15700_ad_e24555), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(680), (-1.0)));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && (!s.b[1325])) {
            let assign15710_ad_e24686: A = {
                if (!((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15710_ad_e24644: A = A::add(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15710_ad_e24644, 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign15710_ad_e24686);
        }

        s.b[1326] = (s.v[333] < 1000.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1326]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1327] = (p.p75 != 0.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1327]) {
            let assign15750_ad_e24769: A = A::add_scaled_product(s.ad_value(698), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(334, A::add_scaled_inputs3(s.ad_value(698), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(assign15750_ad_e24769), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(698), (-1.0)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1327])) {
            let assign15760_ad_e24898: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15760_ad_e24856: A = A::add(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15760_ad_e24856, 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign15760_ad_e24898);
        }

        s.b[1328] = (p.p66 != 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        s.b[1329] = (p.p75 != 0.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1329]) {
            let assign15790_ad_e24972: A = A::add_scaled_product(s.ad_value(699), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(699), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(assign15790_ad_e24972), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(699), (-1.0)));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && (!s.b[1329])) {
            let assign15800_ad_e25103: A = {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15800_ad_e25061: A = A::add(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15800_ad_e25061, 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign15800_ad_e25103);
        }

        s.b[1330] = (s.v[335] < 1000.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1330]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1331] = (p.p75 != 0.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1331]) {
            let assign15840_ad_e25186: A = A::add_scaled_product(s.ad_value(702), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(702), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(assign15840_ad_e25186), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(702), (-1.0)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1331])) {
            let assign15850_ad_e25315: A = {
                if (!((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15850_ad_e25273: A = A::add(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15850_ad_e25273, 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign15850_ad_e25315);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            let assign15860_ad_e25442: A = {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign15860_ad_e25398: A = A::add(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15860_ad_e25398, 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(337, assign15860_ad_e25442, 2.0);
        }

        s.b[1332] = (p.p66 != 0.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1332]) {
            let assign15880_ad_e25575: A = {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign15880_ad_e25531: A = A::add(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15880_ad_e25531, 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(338, assign15880_ad_e25575, 2.0);
        }

        s.b[1333] = (p.p75 != 0.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1333]) {
            let assign15900_ad_e25643: A = A::sqrt(A::add_scaled_product(s.ad_value(657), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6)), 1.0));
            s.store_ad_value(660, A::add_scaled_inputs3(s.ad_value(657), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 0.5, s.ad_value(657), 0.5, assign15900_ad_e25643, 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(657), (-1.0)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1333])) {
            let assign15910_ad_e25765: A = {
                if (!((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign15910_ad_e25725: A = A::add(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign15910_ad_e25725, 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(660, 657, assign15910_ad_e25765);
        }

        s.b[1334] = (p.p75 != 0.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1334]) {
            let assign15930_ad_e25818: A = A::add_scaled_inputs3(s.ad_value(792), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(792), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(792), (-1.0));
            s.store_ad_value(797, assign15930_ad_e25818);
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1334])) {
            let assign15940_ad_e25900: A = {
                if (!(((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(797, 792, assign15940_ad_e25900);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_sub_ad(231, A::add_scaled_product(A::div_from_scalar(p.p1747, A::offset(A::limited_exp_scaled_input(A::offset(s.ad_value(116), (-p.p1749)), p.p1748), 1.0)), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_from_scalar(p.p1747, A::offset(A::limited_exp_scaled_input(A::offset(s.ad_value(228), (-p.p1749)), p.p1748), 1.0)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            let assign15970_ad_e26008: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(413, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign15970_ad_e26008, 0.5));
        }

        s.b[1335] = (p.p66 == 1.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {
            let assign16000_ad_e26090: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(321, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign16000_ad_e26090, 0.5));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {
            s.copy_ad(417, 321);
        }

        s.b[1336] = (s.v[228] > 210.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            let assign16030_ad_e26152: A = A::sub(A::div(s.ad_value(823), A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0)), A::div_scaled_product(s.ad_value(824), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0));
            s.store_scale_ad(170, assign16030_ad_e26152, 210.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_div_ad(169, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)));
            s.store_mul_pow_ad_rhs(306, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_ad_value(307, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            let assign16070_ad_e26266: A = A::add_scaled_inputs(s.ad_value(823), 0.004761904761904762, A::div_scaled_product(s.ad_value(824), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0), 1.0);
            s.store_mul_ad_product_rhs(170, 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), assign16070_ad_e26266);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_ad_value(169, A::add_scaled_products(s.ad_value(807), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0, s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228)), (-1.0)));
            s.store_mul_pow_ad_rhs(306, 807, s.ad_value(229), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_ad_value(307, A::add_scaled_product(s.ad_value(169), 1.0, s.ad_value(170), s.ad_value(232), 1.0));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_ad_value(168, A::add_scaled_products(s.ad_value(313), s.ad_value(306), 1.0, s.ad_value(312), s.ad_value(307), 1.0));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_ad_value(303, {
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(s.ad_value(168), 0.5, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6))), 0.5)
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

        s.b[1337] = (p.p66 != 0.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        s.b[1338] = (s.v[228] > 210.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            let assign16150_ad_e26441: A = A::sub(A::div(s.ad_value(825), A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0)), A::div_scaled_product(s.ad_value(824), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0));
            s.store_scale_ad(170, assign16150_ad_e26441, 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_div_ad(169, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)));
            s.store_mul_pow_ad_rhs(310, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_ad_value(311, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            let assign16190_ad_e26563: A = A::add_scaled_inputs(s.ad_value(825), 0.004761904761904762, A::div_scaled_product(s.ad_value(824), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0), 1.0);
            s.store_mul_ad_product_rhs(170, 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), assign16190_ad_e26563);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_ad_value(169, A::add_scaled_products(s.ad_value(815), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0, s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228)), (-1.0)));
            s.store_mul_pow_ad_rhs(310, 815, s.ad_value(229), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_ad_value(311, A::add_scaled_product(s.ad_value(169), 1.0, s.ad_value(170), s.ad_value(232), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            s.store_ad_value(168, A::add_scaled_products(s.ad_value(313), s.ad_value(310), 1.0, s.ad_value(312), s.ad_value(311), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            s.store_ad_value(305, {
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(s.ad_value(168), 0.5, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6))), 0.5)
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

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1339] = (p.p66 != 0.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1339]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_product(s.ad_value(834), 1.0, s.ad_value(835), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1340] = (((((s.v[326] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1340]) {
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1340])) {
            s.store_ad_value(324, A::div_scaled_product(s.ad_value(325), A::offset(A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0)), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product(s.ad_value(326), A::offset(s.ad_value(228), (-210.0)), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0));
        }

        s.b[1341] = (((((s.v[329] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1341]) {
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1341])) {
            s.store_ad_value(327, A::div_scaled_product(s.ad_value(328), A::offset(A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0)), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product(s.ad_value(329), A::offset(s.ad_value(228), (-210.0)), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1342] = (p.p75 != 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16380_ad_e26935: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(811), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(323, A::add_scaled_inputs3(s.ad_value(811), 1.0, assign16380_ad_e26935, 1.0, s.ad_value(811), (-1.0)));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16390_ad_e27003: A = A::add_scaled_product(s.ad_value(679), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(332, A::add_scaled_inputs3(s.ad_value(679), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(assign16390_ad_e27003), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(679), (-1.0)));
        }

        s.b[1343] = (p.p66 != 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) {
            let assign16410_ad_e27079: A = A::add_scaled_product(s.ad_value(680), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(333, A::add_scaled_inputs3(s.ad_value(680), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(assign16410_ad_e27079), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(680), (-1.0)));
        }

        s.b[1344] = (s.v[333] < 1000.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) && s.b[1344]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16440_ad_e27167: A = A::add_scaled_product(s.ad_value(698), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(334, A::add_scaled_inputs3(s.ad_value(698), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(assign16440_ad_e27167), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(698), (-1.0)));
        }

        s.b[1345] = (p.p66 != 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) {
            let assign16460_ad_e27243: A = A::add_scaled_product(s.ad_value(699), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(699), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(assign16460_ad_e27243), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(699), (-1.0)));
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1346] = (s.v[335] < 1000.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) && s.b[1346]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16490_ad_e27331: A = A::add_scaled_product(s.ad_value(702), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6)), 1.0);
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(702), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(assign16490_ad_e27331), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(702), (-1.0)));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16500_ad_e27400: A = A::sqrt(A::add_scaled_product(s.ad_value(657), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6)), 1.0));
            s.store_ad_value(660, A::add_scaled_inputs3(s.ad_value(657), 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 0.5, s.ad_value(657), 0.5, assign16500_ad_e27400, 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(657), (-1.0)));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            let assign16510_ad_e27454: A = A::add_scaled_inputs3(s.ad_value(792), 1.0, A::add_scaled_inputs3_offset(s.ad_value(233), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(792), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(233), p.p1026, s.ad_value(792), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(233), p.p1026, s.ad_value(792), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(792), (-1.0));
            s.store_ad_value(797, assign16510_ad_e27454);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16520_ad_e27537: A = {
                if (!(((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(323, 811, assign16520_ad_e27537);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16530_ad_e27657: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16530_ad_e27617: A = A::add(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16530_ad_e27617, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(332, 679, assign16530_ad_e27657);
        }

        s.b[1347] = (p.p66 != 0.0);
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) {
            let assign16550_ad_e27782: A = {
                if (!((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16550_ad_e27742: A = A::add(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16550_ad_e27742, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(851), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(333, 680, assign16550_ad_e27782);
        }

        s.b[1348] = (s.v[333] < 1000.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) && s.b[1348]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16580_ad_e27920: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16580_ad_e27880: A = A::add(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16580_ad_e27880, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(334, 698, assign16580_ad_e27920);
        }

        s.b[1349] = (p.p66 != 0.0);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) {
            let assign16600_ad_e28045: A = {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16600_ad_e28005: A = A::add(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16600_ad_e28005, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(849), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(335, 699, assign16600_ad_e28045);
        }

        s.b[1350] = (s.v[335] < 1000.0);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) && s.b[1350]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16630_ad_e28183: A = {
                if (!((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16630_ad_e28143: A = A::add(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16630_ad_e28143, 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::sub_from_scalar(1.0, A::mul(s.ad_value(850), s.ad_value(232))), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(336, 702, assign16630_ad_e28183);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16640_ad_e28303: A = {
                if (!((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    let assign16640_ad_e28263: A = A::add(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16640_ad_e28263, 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(660, 657, assign16640_ad_e28303);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            let assign16650_ad_e28387: A = {
                if (!(((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(797, 792, assign16650_ad_e28387);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            let assign16660_ad_e28515: A = {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign16660_ad_e28471: A = A::add(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16660_ad_e28471, 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(337, assign16660_ad_e28515, 2.0);
        }

        s.b[1351] = (p.p66 != 0.0);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1351]) {
            let assign16680_ad_e28649: A = {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    let assign16680_ad_e28605: A = A::add(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0))), ((4.0 * 0.001) * 0.001))));
                    A::scale(assign16680_ad_e28605, 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(338, assign16680_ad_e28649, 2.0);
        }

        s.b[1352] = (p.p67 == 1.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            let assign16710_ad_e28730: A = A::add(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::sqrt(A::add_scaled_product(s.ad_value(169), (-((-0.9) * (4.0 * 0.0001))), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001)), 1.0)));
            s.store_ad_value(414, A::add_scaled_inputs3(s.ad_value(169), 1.0, s.ad_value(169), (-0.9), assign16710_ad_e28730, 0.5));
        }

        s.b[1353] = (s.v[228] > 210.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            let assign16730_ad_e28784: A = A::sub(A::div(s.ad_value(826), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0)), A::div_scaled_product(s.ad_value(827), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0));
            s.store_scale_ad(170, assign16730_ad_e28784, 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_div_ad(169, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)));
            s.store_mul_pow_ad_rhs(308, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_ad_value(309, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            let assign16770_ad_e28906: A = A::add_scaled_inputs(s.ad_value(826), 0.004761904761904762, A::div_scaled_product(s.ad_value(827), A::offset({
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
            }, 1.0), 1.0, s.ad_value(228), 1.0), 1.0);
            s.store_mul_ad_product_rhs(170, 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), assign16770_ad_e28906);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_ad_value(169, A::add_scaled_products(s.ad_value(808), A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0, s.ad_value(170), A::sub_from_scalar(210.0, s.ad_value(228)), (-1.0)));
            s.store_mul_pow_ad_rhs(308, 808, s.ad_value(229), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_ad_value(309, A::add_scaled_product(s.ad_value(169), 1.0, s.ad_value(170), s.ad_value(232), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_ad_value(168, A::add_scaled_products(s.ad_value(313), s.ad_value(308), 1.0, s.ad_value(312), s.ad_value(309), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_ad_value(304, {
                if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(s.ad_value(168), 0.5, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-6) * 1e-6))), 0.5)
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

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1354] = (s.v[854] == s.v[855]);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1354]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1355] = (s.v[856] < 210.0);
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        s.b[1356] = (s.v[228] > 210.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_ad_value(196, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
            s.store_offset_mul_ad(171, s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0);
            s.store_ad_value(172, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), 1.0, s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
        }

        s.b[1357] = (s.v[855] < s.v[854]);
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && s.b[1357]) {
            let assign16930_ad_e29233: A = A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(171), s.ad_value(172)), A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_ad_lhs(174, assign16930_ad_e29233, 171);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && s.b[1357]) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(174), 0.5, s.ad_value(195), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(174), s.ad_value(195)), A::sub(s.ad_value(174), s.ad_value(195))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && (!s.b[1357])) {
            let assign16950_ad_e29329: A = A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(171), s.ad_value(172)), A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)));
            s.store_add_ad_lhs(174, assign16950_ad_e29329, 171);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && (!s.b[1357])) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(174), 0.5, s.ad_value(195), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(174), s.ad_value(195)), A::sub(s.ad_value(174), s.ad_value(195))), ((0.25 * 0.001) * 0.001))), (-0.5)));
        }

        s.b[1358] = (s.v[228] > s.v[856]);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_ad_value(196, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
            s.store_offset_mul_ad(172, s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0);
            s.store_ad_value(174, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), 1.0, s.ad_value(854), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
        }

        s.b[1359] = (s.v[855] < s.v[854]);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {
            let assign17040_ad_e29564: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(175, assign17040_ad_e29564);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {
            let assign17050_ad_e29624: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(176, assign17050_ad_e29624);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {
            s.store_ad_value(177, A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(854), A::offset(s.ad_value(116), (-210.0)), 1.0));
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(175), 0.5, s.ad_value(177), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {
            let assign17080_ad_e29750: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(175, assign17080_ad_e29750);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {
            let assign17090_ad_e29811: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(176, assign17090_ad_e29811);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {
            s.store_ad_value(177, A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(854), A::offset(s.ad_value(116), (-210.0)), 1.0));
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(175), 0.5, s.ad_value(177), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001))), (-0.5)));
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_ad_value(195, A::add_scaled_product(A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
            s.store_offset_mul_ad(172, s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0);
            s.store_ad_value(174, A::add_scaled_product(A::offset(A::mul(s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(856))), 1.0), 1.0, s.ad_value(855), A::sub(s.ad_value(856), s.ad_value(228)), 1.0));
        }

        s.b[1360] = (s.v[855] < s.v[854]);
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {
            let assign17180_ad_e30077: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(175, assign17180_ad_e30077);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {
            let assign17190_ad_e30138: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5));
            s.store_ad_value(176, assign17190_ad_e30138);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {
            s.store_ad_value(177, A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(854), A::offset(s.ad_value(116), (-210.0)), 1.0));
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(175), 0.5, s.ad_value(177), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001))), 0.5));
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {
            let assign17220_ad_e30267: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(175, assign17220_ad_e30267);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {
            let assign17230_ad_e30329: A = A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_products(A::sub(s.ad_value(172), s.ad_value(174)), A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), 1.0, s.ad_value(171), (-0.5), A::sqrt(A::add_scaled_products(s.ad_value(171), s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-(-0.5)));
            s.store_ad_value(176, assign17230_ad_e30329);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {
            s.store_ad_value(177, A::add_scaled_product(s.ad_value(176), 1.0, s.ad_value(854), A::offset(s.ad_value(116), (-210.0)), 1.0));
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(175), 0.5, s.ad_value(177), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(175), s.ad_value(177)), A::sub(s.ad_value(175), s.ad_value(177))), ((0.25 * 0.001) * 0.001))), (-0.5)));
        }

        s.b[1361] = (s.v[228] > 210.0);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_ad_value(196, A::add_scaled_product(A::offset(A::mul(s.ad_value(855), A::offset(s.ad_value(116), (-210.0))), 1.0), 1.0, s.ad_value(854), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0));
        }

        s.b[1362] = (s.v[855] < s.v[854]);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && s.b[1362]) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01))), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && (!s.b[1362])) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01))), (-0.5)));
        }

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_ad_value(195, A::add_scaled_product(A::offset(A::mul(s.ad_value(854), A::offset(s.ad_value(116), (-210.0))), 1.0), 1.0, s.ad_value(855), A::sub_from_scalar(210.0, s.ad_value(228)), 1.0));
        }

        s.b[1363] = (s.v[855] < s.v[854]);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && s.b[1363]) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01))), 0.5));
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && (!s.b[1363])) {
            s.store_ad_value(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(195), s.ad_value(196)), A::sub(s.ad_value(195), s.ad_value(196))), ((0.25 * 0.01) * 0.01))), (-0.5)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            let assign17370_ad_e30712: A = {
                if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(170), (-1e-6)), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(170), (-1e-6)), A::offset(s.ad_value(170), (-1e-6))), ((4.0 * 0.001) * 0.001))), 0.5)
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
            s.store_ad_value(194, assign17370_ad_e30712);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_scaled_sub_ad(172, A::offset(s.ad_value(228), 210.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(228), (-210.0)), A::offset(s.ad_value(228), (-210.0))), ((0.25 * 0.2) * 0.2))), 0.5);
            s.store_sub_ad(231, A::add_scaled_product(A::div_from_scalar(p.p1747, A::offset(A::limited_exp_scaled_input(A::offset(s.ad_value(117), (-p.p1749)), p.p1748), 1.0)), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_from_scalar(p.p1747, A::offset(A::limited_exp_scaled_input(A::offset(s.ad_value(172), (-p.p1749)), p.p1748), 1.0)));
        }

        s.b[1364] = (s.v[332] < 1000.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if s.b[1364] {
            s.store_scalar(332, 1000.0);
        }

        s.b[1365] = (s.v[334] < 1000.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if s.b[1365] {
            s.store_scalar(334, 1000.0);
        }

        s.b[1366] = (s.v[336] < 1000.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if s.b[1366] {
            s.store_scalar(336, 1000.0);
        }

        s.b[1367] = (p.p61 != 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        s.b[1368] = (p.p75 == 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        s.b[1369] = (p.p75 != 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((s.b[1367] && s.b[1368]) && s.b[1369]) {
            let assign17490_ad_e30851: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(809), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(314, A::add_scaled_inputs3(s.ad_value(809), 1.0, assign17490_ad_e30851, 1.0, s.ad_value(809), (-1.0)));
        }

        if ((s.b[1367] && s.b[1368]) && (!s.b[1369])) {
            let assign17500_ad_e30934: A = {
                if (!(((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(314, 809, assign17500_ad_e30934);
        }

        s.b[1370] = (p.p67 == 1.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        s.b[1371] = (p.p75 != 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && s.b[1371]) {
            let assign17530_ad_e30989: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(810), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(315, A::add_scaled_inputs3(s.ad_value(810), 1.0, assign17530_ad_e30989, 1.0, s.ad_value(810), (-1.0)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && (!s.b[1371])) {
            let assign17540_ad_e31074: A = {
                if (!(((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(315, 810, assign17540_ad_e31074);
        }

        s.b[1372] = (p.p66 != 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        s.b[1373] = (p.p75 != 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && s.b[1373]) {
            let assign17570_ad_e31129: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(817), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(316, A::add_scaled_inputs3(s.ad_value(817), 1.0, assign17570_ad_e31129, 1.0, s.ad_value(817), (-1.0)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && (!s.b[1373])) {
            let assign17580_ad_e31214: A = {
                if (!(((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(316, 817, assign17580_ad_e31214);
        }

        if (s.b[1367] && (!s.b[1368])) {
            s.store_ad_value(314, A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0));
        }

        s.b[1374] = (p.p67 == 1.0);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((s.b[1367] && (!s.b[1368])) && s.b[1374]) {
            s.store_ad_value(315, A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0));
        }

        s.b[1375] = (p.p66 != 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((s.b[1367] && (!s.b[1368])) && s.b[1375]) {
            s.store_ad_value(316, A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0));
        }

        s.b[1376] = (p.p75 != 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if s.b[1376] {
            let assign17650_ad_e31305: A = A::add_scaled_inputs3(s.ad_value(673), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p164 * 0.5), s.ad_value(673), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(673), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p164, s.ad_value(673), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p164, s.ad_value(673), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(673), (-1.0));
            s.store_ad_value(296, assign17650_ad_e31305);
        }

        if (!s.b[1376]) {
            let assign17660_ad_e31382: A = {
                if (!(((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(296, 673, assign17660_ad_e31382);
        }

        s.b[1377] = (p.p67 == 1.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        s.b[1378] = (p.p75 != 0.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1378]) {
            let assign17690_ad_e31435: A = A::add_scaled_inputs3(s.ad_value(675), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p165 * 0.5), s.ad_value(675), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(675), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p165, s.ad_value(675), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p165, s.ad_value(675), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(675), (-1.0));
            s.store_ad_value(297, assign17690_ad_e31435);
        }

        if (s.b[1377] && (!s.b[1378])) {
            let assign17700_ad_e31514: A = {
                if (!(((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(297, 675, assign17700_ad_e31514);
        }

        s.b[1379] = (p.p75 != 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if s.b[1379] {
            let assign17720_ad_e31562: A = A::add_scaled_inputs3(s.ad_value(677), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p166 * 0.5), s.ad_value(677), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(677), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p166, s.ad_value(677), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p166, s.ad_value(677), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(677), (-1.0));
            s.store_ad_value(298, assign17720_ad_e31562);
        }

        if (!s.b[1379]) {
            let assign17730_ad_e31639: A = {
                if (!(((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(298, 677, assign17730_ad_e31639);
        }

        s.b[1380] = (p.p75 != 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if s.b[1380] {
            let assign17750_ad_e31685: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(707), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(322, A::add_scaled_inputs3(s.ad_value(707), 1.0, assign17750_ad_e31685, 1.0, s.ad_value(707), (-1.0)));
        }

        if (!s.b[1380]) {
            let assign17760_ad_e31764: A = {
                if (!(((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(322, 707, assign17760_ad_e31764);
        }

        s.b[1381] = (p.p75 != 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if s.b[1381] {
            s.store_offset_ad(299, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6))))), (-((4.0 * (-p.p917)) * 1e-6)))), 0.5), (((-p.p917)) + (p.p917)));
        }

        if (!s.b[1381]) {
            let assign17790_ad_e31889: A = {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(299, assign17790_ad_e31889, p.p917);
        }

        s.b[1382] = (p.p66 != 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        s.b[1383] = (p.p75 != 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if (s.b[1382] && s.b[1383]) {
            s.store_offset_ad(300, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6))))), (-((4.0 * (-p.p918)) * 1e-6)))), 0.5), (((-p.p918)) + (p.p918)));
        }

        if (s.b[1382] && (!s.b[1383])) {
            let assign17830_ad_e32021: A = {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(300, assign17830_ad_e32021, p.p918);
        }

        s.b[1384] = (p.p75 != 0.0);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if s.b[1384] {
            s.store_offset_ad(301, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6))))), (-((4.0 * (-p.p919)) * 1e-6)))), 0.5), (((-p.p919)) + (p.p919)));
        }

        if (!s.b[1384]) {
            let assign17860_ad_e32146: A = {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(301, assign17860_ad_e32146, p.p919);
        }

        s.b[1385] = (p.p66 != 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        s.b[1386] = (p.p75 != 0.0);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (s.b[1385] && s.b[1386]) {
            s.store_offset_ad(302, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6))))), (-((4.0 * (-p.p920)) * 1e-6)))), 0.5), (((-p.p920)) + (p.p920)));
        }

        if (s.b[1385] && (!s.b[1386])) {
            let assign17900_ad_e32278: A = {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(302, assign17900_ad_e32278, p.p920);
        }

        s.b[1387] = (p.p75 != 0.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if s.b[1387] {
            let assign17920_ad_e32327: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(700), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(257, A::add_scaled_inputs3(s.ad_value(700), 1.0, assign17920_ad_e32327, 1.0, s.ad_value(700), (-1.0)));
        }

        if (!s.b[1387]) {
            let assign17930_ad_e32412: A = {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(257, 700, assign17930_ad_e32412);
        }

        s.b[1388] = (p.p66 != 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        s.b[1389] = (p.p75 != 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if (s.b[1388] && s.b[1389]) {
            let assign17960_ad_e32466: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(701), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6)), 1.0)), 0.5);
            s.store_ad_value(258, A::add_scaled_inputs3(s.ad_value(701), 1.0, assign17960_ad_e32466, 1.0, s.ad_value(701), (-1.0)));
        }

        if (s.b[1388] && (!s.b[1389])) {
            let assign17970_ad_e32553: A = {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(258, 701, assign17970_ad_e32553);
        }

        s.store_mul_exp_ad_rhs(248, 779, A::mul(s.ad_value(860), s.ad_value(418)));

        let assign17990_ad_e32635: A = {
    if (!(((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_offset_ad_rhs(249, 785, assign17990_ad_e32635, 0.01);

        s.store_ad_value(236, A::add_scaled_product(s.ad_value(683), 1.0, s.ad_value(684), s.ad_value(232), 1.0));

        let assign18010_ad_e32682: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(685), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(237, A::add_scaled_inputs3(s.ad_value(685), 1.0, assign18010_ad_e32682, 1.0, s.ad_value(685), (-1.0)));

        let assign18020_ad_e32723: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(687), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(238, A::add_scaled_inputs3(s.ad_value(687), 1.0, assign18020_ad_e32723, 1.0, s.ad_value(687), (-1.0)));

        let assign18030_ad_e32764: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(690), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(239, A::add_scaled_inputs3(s.ad_value(690), 1.0, assign18030_ad_e32764, 1.0, s.ad_value(690), (-1.0)));

        s.store_ad_value(240, A::add_scaled_product(s.ad_value(692), 1.0, s.ad_value(693), s.ad_value(232), 1.0));

        s.store_ad_value(241, A::add_scaled_product(s.ad_value(798), 1.0, s.ad_value(800), s.ad_value(232), 1.0));

        s.store_ad_value(242, A::add_scaled_product(s.ad_value(799), 1.0, s.ad_value(801), s.ad_value(232), 1.0));

        let assign18070_ad_e32820: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(871), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(293, A::add_scaled_inputs3(s.ad_value(871), 1.0, assign18070_ad_e32820, 1.0, s.ad_value(871), (-1.0)));

        s.store_ad_value(294, A::add_scaled_product(s.ad_value(867), 1.0, s.ad_value(868), s.ad_value(232), 1.0));

        s.store_ad_value(295, A::add_scaled_product(s.ad_value(869), 1.0, s.ad_value(870), s.ad_value(232), 1.0));

        let assign18100_ad_e32871: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(721), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(243, A::add_scaled_inputs3(s.ad_value(721), 1.0, assign18100_ad_e32871, 1.0, s.ad_value(721), (-1.0)));

        let assign18110_ad_e32912: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(727), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(244, A::add_scaled_inputs3(s.ad_value(727), 1.0, assign18110_ad_e32912, 1.0, s.ad_value(727), (-1.0)));

        let assign18120_ad_e32953: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(732), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(245, A::add_scaled_inputs3(s.ad_value(732), 1.0, assign18120_ad_e32953, 1.0, s.ad_value(732), (-1.0)));

        let assign18130_ad_e32994: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(737), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(246, A::add_scaled_inputs3(s.ad_value(737), 1.0, assign18130_ad_e32994, 1.0, s.ad_value(737), (-1.0)));

        let assign18140_ad_e33035: A = A::add_scaled_inputs(A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6)), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(743), (-(-(4.0 * 1e-6))), A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6)), A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6)), 1.0)), 0.5);
        s.store_ad_value(247, A::add_scaled_inputs3(s.ad_value(743), 1.0, assign18140_ad_e33035, 1.0, s.ad_value(743), (-1.0)));

        let assign18150_ad_e33110: A = {
    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(252, 748, assign18150_ad_e33110);

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let assign18160_ad_e33184: A = {
    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(250, 762, assign18160_ad_e33184);

        let assign18170_ad_e33226: A = A::add_scaled_inputs3(s.ad_value(775), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1437 * 0.5), s.ad_value(775), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(775), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1437, s.ad_value(775), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1437, s.ad_value(775), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(775), (-1.0));
        s.store_ad_value(259, assign18170_ad_e33226);

        let assign18180_ad_e33267: A = A::add_scaled_inputs3(s.ad_value(776), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1438 * 0.5), s.ad_value(776), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(776), (-(-(4.0 * 1e-6))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1438, s.ad_value(776), -1.0), (-1e-6)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1438, s.ad_value(776), -1.0), (-1e-6)), 1.0)), 0.5, ((-1e-6) * 0.5)), 1.0, s.ad_value(776), (-1.0));
        s.store_ad_value(260, assign18180_ad_e33267);

        let assign18190_ad_e33308: A = A::add_scaled_inputs3(s.ad_value(777), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1439 * 0.5), s.ad_value(777), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(777), (-(-(4.0 * 1e-25))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1439, s.ad_value(777), -1.0), (-1e-25)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1439, s.ad_value(777), -1.0), (-1e-25)), 1.0)), 0.5, ((-1e-25) * 0.5)), 1.0, s.ad_value(777), (-1.0));
        s.store_ad_value(261, assign18190_ad_e33308);

        let assign18200_ad_e33349: A = A::add_scaled_inputs3(s.ad_value(778), 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1440 * 0.5), s.ad_value(778), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(778), (-(-(4.0 * 1e-20))), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1440, s.ad_value(778), -1.0), (-1e-20)), A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1440, s.ad_value(778), -1.0), (-1e-20)), 1.0)), 0.5, ((-1e-20) * 0.5)), 1.0, s.ad_value(778), (-1.0));
        s.store_ad_value(262, assign18200_ad_e33349);

        s.store_exp_mul(256, 861, 418);

        s.store_mul(462, 463, 256);

        s.b[1390] = (p.p61 != 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (p.p75 != 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1391]) {
            s.store_offset_ad(263, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6))))), (-((4.0 * (-p.p1584)) * 1e-6)))), 0.5), (((-p.p1584)) + (p.p1584)));
        }

        if (s.b[1390] && (!s.b[1391])) {
            let assign18260_ad_e33485: A = {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(263, assign18260_ad_e33485, p.p1584);
        }

        s.b[1392] = (p.p75 != 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1392]) {
            s.store_offset_ad(266, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6))))), (-((4.0 * (-p.p1585)) * 1e-6)))), 0.5), (((-p.p1585)) + (p.p1585)));
        }

        if (s.b[1390] && (!s.b[1392])) {
            let assign18290_ad_e33614: A = {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(266, assign18290_ad_e33614, p.p1585);
        }

        s.b[1393] = (p.p75 != 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1393]) {
            s.store_offset_ad(264, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6))))), (-((4.0 * (-p.p1586)) * 1e-6)))), 0.5), (((-p.p1586)) + (p.p1586)));
        }

        if (s.b[1390] && (!s.b[1393])) {
            let assign18320_ad_e33743: A = {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(264, assign18320_ad_e33743, p.p1586);
        }

        s.b[1394] = (p.p75 != 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1394]) {
            s.store_offset_ad(267, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6))))), (-((4.0 * (-p.p1587)) * 1e-6)))), 0.5), (((-p.p1587)) + (p.p1587)));
        }

        if (s.b[1390] && (!s.b[1394])) {
            let assign18350_ad_e33872: A = {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(267, assign18350_ad_e33872, p.p1587);
        }

        s.b[1395] = (p.p75 != 0.0);
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1395]) {
            s.store_offset_ad(268, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6))))), (-((4.0 * (-p.p1588)) * 1e-6)))), 0.5), (((-p.p1588)) + (p.p1588)));
        }

        if (s.b[1390] && (!s.b[1395])) {
            let assign18380_ad_e34001: A = {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(268, assign18380_ad_e34001, p.p1588);
        }

        s.b[1396] = (p.p75 != 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1396]) {
            s.store_offset_ad(265, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6))))), (-((4.0 * (-p.p1589)) * 1e-6)))), 0.5), (((-p.p1589)) + (p.p1589)));
        }

        if (s.b[1390] && (!s.b[1396])) {
            let assign18410_ad_e34130: A = {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(265, assign18410_ad_e34130, p.p1589);
        }

        if s.b[1390] {
            let assign18420_ad_e34206: A = {
                if (!(((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            let assign18430_ad_e34283: A = {
                if (!(((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            let assign18440_ad_e34360: A = {
                if (!(((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            let assign18450_ad_e34437: A = {
                if (!(((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            let assign18460_ad_e34514: A = {
                if (!(((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            let assign18470_ad_e34591: A = {
                if (!(((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if s.b[1390] {
            s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));
            s.store_ad_value(171, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1727), 1.0 / (p.p1620)));
            s.store_scale(275, 171, p.p1614);
            s.store_scale(276, 171, p.p1616);
            s.store_scale(277, 171, p.p1618);
            s.store_ad_value(171, A::limited_exp_scaled_input(A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1728), 1.0 / (p.p1621)));
            s.store_scale(278, 171, p.p1615);
            s.store_scale(279, 171, p.p1617);
            s.store_scale(280, 171, p.p1619);
            s.store_scaled_limited_exp_ad(281, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1729, s.ad_value(179), 1.0), p.p1630);
            s.store_scaled_limited_exp_ad(282, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1730, s.ad_value(179), 1.0), p.p1631);
            s.store_scaled_limited_exp_ad(283, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1731, s.ad_value(179), 1.0), p.p1632);
            s.store_scaled_limited_exp_ad(284, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1732, s.ad_value(179), 1.0), p.p1633);
            s.store_scaled_mul_ad(285, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1733, s.ad_value(179), 1.0)), p.p1634);
            s.store_scaled_mul_ad(286, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1734, s.ad_value(179), 1.0)), p.p1635);
        }

        if s.b[1390] {
            let assign18630_ad_e34840: A = {
                if (!(((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(287, assign18630_ad_e34840, 0.01);
        }

        if s.b[1390] {
            let assign18640_ad_e34929: A = {
                if (!(((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(288, assign18640_ad_e34929, 0.01);
        }

        if s.b[1390] {
            let assign18650_ad_e35018: A = {
                if (!(((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(289, assign18650_ad_e35018, 0.01);
        }

        if s.b[1390] {
            let assign18660_ad_e35107: A = {
                if (!(((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(290, assign18660_ad_e35107, 0.01);
        }

        if s.b[1390] {
            let assign18670_ad_e35196: A = {
                if (!(((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(291, assign18670_ad_e35196, 0.01);
        }

        if s.b[1390] {
            let assign18680_ad_e35285: A = {
                if (!(((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(292, assign18680_ad_e35285, 0.01);
        }

        s.b[1397] = (!param_given[1106]);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        s.b[1398] = (p.p145 > 0.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        s.b[1399] = (p.p80 == 0.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1397] && s.b[1398]) && s.b[1399]) {
            let assign18720_ad_e35490: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    let assign18720_ad_e35416: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)));
                    let assign18720_ad_e35424: A = A::add(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::sqrt(A::offset(assign18720_ad_e35416, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18720_ad_e35424, 0.5)
                } else {
                    let assign18720_ad_e35489: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18720_ad_e35487: A = A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                            }, (-1.0)));
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
                    let assign18720_ad_e35609: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)));
                    let assign18720_ad_e35617: A = A::add(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::sqrt(A::offset(assign18720_ad_e35609, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18720_ad_e35617, 0.5)
                } else {
                    let assign18720_ad_e35682: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18720_ad_e35680: A = A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                            }, (-1.0)));
                            assign18720_ad_e35680
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18720_ad_e35682
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, assign18720_ad_e35490, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18720_ad_e35683, 1.0), (-1.0)));
        }

        if ((s.b[1397] && s.b[1398]) && (!s.b[1399])) {
            let assign18730_ad_e35859: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18730_ad_e35793: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)));
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), 0.5, A::sqrt(A::offset(assign18730_ad_e35793, ((4.0 * 0.0001) * 0.0001))), 0.5)
                } else {
                    let assign18730_ad_e35858: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18730_ad_e35858
                }
            };
            let assign18730_ad_e36028: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18730_ad_e35962: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)));
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), 0.5, A::sqrt(A::offset(assign18730_ad_e35962, ((4.0 * 0.0001) * 0.0001))), 0.5)
                } else {
                    let assign18730_ad_e36027: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18730_ad_e36027
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, assign18730_ad_e35859, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18730_ad_e36028, 1.0), (-1.0)));
        }

        s.b[1400] = (p.p80 == 0.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (!s.b[1398])) && s.b[1400]) {
            let assign18750_ad_e36241: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    let assign18750_ad_e36167: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)));
                    let assign18750_ad_e36175: A = A::add(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                    }, (-1.0)), A::sqrt(A::offset(assign18750_ad_e36167, ((4.0 * 0.0001) * 0.0001))));
                    A::scale(assign18750_ad_e36175, 0.5)
                } else {
                    let assign18750_ad_e36240: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            let assign18750_ad_e36238: A = A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
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
                            }, (-1.0)));
                            assign18750_ad_e36238
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18750_ad_e36240
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18750_ad_e36241, 1.0), (-1.0)));
        }

        if ((s.b[1397] && (!s.b[1398])) && (!s.b[1400])) {
            let assign18760_ad_e36428: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    let assign18760_ad_e36362: A = A::mul(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)));
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)), 0.5, A::sqrt(A::offset(assign18760_ad_e36362, ((4.0 * 0.0001) * 0.0001))), 0.5)
                } else {
                    let assign18760_ad_e36427: A = {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), A::sub_from_scalar((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign18760_ad_e36427
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18760_ad_e36428, 1.0), (-1.0)));
        }

        if (!s.b[1397]) {
            s.store_scalar(479, p.p1106);
        }

        s.b[1401] = (!param_given[1107]);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if s.b[1401] {
            s.copy_ad(518, 479);
        }

        if (!s.b[1401]) {
            s.store_scalar(518, p.p1107);
        }

        s.b[1402] = (p.p80 == 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if s.b[1402] {
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

        if s.b[1402] {
            s.store_scaled_add_ad_rhs(166, 166, A::sqrt(A::offset(A::mul(s.ad_value(166), s.ad_value(166)), ((0.25 * 1e-10) * 1e-10))), 0.5);
        }

        if s.b[1402] {
            let assign18840_ad_e36537: A = {
                if (!(((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(640), p.p97, A::square(s.ad_value(141)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(352, 179, assign18840_ad_e36537);
        }

        if (!s.b[1402]) {
            s.store_mul_sub_ad_rhs(166, 179, {
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
            }, s.ad_value(142));
        }

        if (!s.b[1402]) {
            s.store_scaled_add_ad_rhs(166, 166, A::sqrt(A::offset(A::mul(s.ad_value(166), s.ad_value(166)), ((0.25 * 1e-10) * 1e-10))), 0.5);
        }

        if (!s.b[1402]) {
            s.store_mul_ad_rhs(352, 179, A::sub_scaled_inputs({
                if (!((s.v[640] * p.p97) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p.p97) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(640), p.p97)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, s.ad_value(142), 2.0));
        }

        s.store_mul_sub_ad_rhs(167, 114, s.ad_value(641), A::offset({
            if (p.p60 == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p.p104));

        s.store_scale(407, 322, 0.5);

        s.v[408] = 0.5;

        s.b[1403] = (p.p60 != 1.0);
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if s.b[1403] {
            s.store_scale(407, 322, 0.333333333);
            s.store_scalar(408, 0.333333333);
        }

        s.b[1404] = (p.p61 != 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if s.b[1404] {
            s.store_ad_value(537, A::add_scaled_inputs3(s.ad_value(275), p.p11, s.ad_value(276), p.p13, s.ad_value(277), (p.p3 * s.v[115])));
        }

        s.b[1405] = (s.v[537] > 0.0);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1405]) {
            s.store_scale(539, 179, p.p1620);
            s.store_scaled_limited_exp_ad(547, A::div_from_scalar((-p.p1626), s.ad_value(539)), p.p1628);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1622, s.ad_value(537)), 10.0);
            s.store_sub_ad_lhs(226, A::offset(s.ad_value(170), 1.0), 547);
        }

        if (s.b[1404] && s.b[1405]) {
            let assign19010_ad_e36758: A = {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38) {
                            A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(547), 4.0))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(546, 539, assign19010_ad_e36758);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_limited_exp_div(168, 546, 539);
            s.store_mul_offset_ad_rhs(545, 537, A::add_scaled_inputs3(s.ad_value(168), 1.0, A::div(s.ad_value(547), s.ad_value(168)), (-1.0), s.ad_value(547), 1.0), (-1.0));
            s.store_ad_value(544, A::div_scaled_product(s.ad_value(537), A::add(s.ad_value(168), A::div(s.ad_value(547), s.ad_value(168))), 1.0, s.ad_value(539), 1.0));
        }

        if (s.b[1404] && s.b[1405]) {
            let assign19050_ad_e36863: A = {
                if (!(((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0))), ((4.0 * 0.001) * 0.001))), 0.5)
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

        if (s.b[1404] && s.b[1405]) {
            s.store_sub_from_scalar_ad(543, (-p.p1626), A::mul(s.ad_value(539), {
                if (!(((s.v[170] - 1.0) / p.p1628) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1628) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1628))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_scaled_limited_exp_ad(169, A::div_scaled_inputs(A::offset(s.ad_value(543), p.p1626), -1.0, s.ad_value(539), 1.0), p.p1628);
            s.store_mul_offset_rhs(542, 537, 169, 1.0);
            s.store_ad_value(541, A::div_scaled_product(s.ad_value(537), s.ad_value(169), -1.0, s.ad_value(539), 1.0));
        }

        if s.b[1404] {
            s.store_ad_value(538, A::add_scaled_inputs3(s.ad_value(278), p.p12, s.ad_value(279), p.p14, s.ad_value(280), (p.p3 * s.v[115])));
        }

        s.b[1406] = (s.v[538] > 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1406]) {
            s.store_scale(540, 179, p.p1621);
            s.store_scaled_limited_exp_ad(554, A::div_from_scalar((-p.p1627), s.ad_value(540)), p.p1629);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1623, s.ad_value(538)), 10.0);
            s.store_sub_ad_lhs(226, A::offset(s.ad_value(170), 1.0), 554);
        }

    }
}
