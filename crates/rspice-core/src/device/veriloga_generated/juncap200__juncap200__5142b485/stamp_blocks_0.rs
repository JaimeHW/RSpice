#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1] = (8.8541878176e-12 * 11.8);

        scratch.values[2] = (if (self.params.trj > (-250.0)) { self.params.trj } else { (-250.0) });

        scratch.values[3] = (if (self.params.imax > 1e-12) { self.params.imax } else { 1e-12 });

        scratch.values[4] = (if (self.params.frev > 10.0) { (if (self.params.frev < 10000000000.0) { self.params.frev } else { 10000000000.0 }) } else { 10.0 });

        scratch.values[5] = (if (self.params.cjorbot > 1e-12) { self.params.cjorbot } else { 1e-12 });

        scratch.values[6] = (if (self.params.cjorsti > 1e-18) { self.params.cjorsti } else { 1e-18 });

        scratch.values[7] = (if (self.params.cjorgat > 1e-18) { self.params.cjorgat } else { 1e-18 });

        scratch.values[8] = (if (self.params.vbirbot > 0.05) { self.params.vbirbot } else { 0.05 });

        scratch.values[9] = (if (self.params.vbirsti > 0.05) { self.params.vbirsti } else { 0.05 });

        scratch.values[10] = (if (self.params.vbirgat > 0.05) { self.params.vbirgat } else { 0.05 });

        scratch.values[11] = (if (self.params.pbot > 0.05) { (if (self.params.pbot < 0.95) { self.params.pbot } else { 0.95 }) } else { 0.05 });

        scratch.values[12] = (if (self.params.psti > 0.05) { (if (self.params.psti < 0.95) { self.params.psti } else { 0.95 }) } else { 0.05 });

        scratch.values[13] = (if (self.params.pgat > 0.05) { (if (self.params.pgat < 0.95) { self.params.pgat } else { 0.95 }) } else { 0.05 });

        scratch.values[14] = self.params.phigbot;

        scratch.values[15] = self.params.phigsti;

        scratch.values[16] = self.params.phiggat;

        scratch.values[17] = (if (self.params.idsatrbot > 0.0) { self.params.idsatrbot } else { 0.0 });

        scratch.values[18] = (if (self.params.idsatrsti > 0.0) { self.params.idsatrsti } else { 0.0 });

        scratch.values[19] = (if (self.params.idsatrgat > 0.0) { self.params.idsatrgat } else { 0.0 });

        scratch.values[22] = (if (self.params.csrhbot > 0.0) { self.params.csrhbot } else { 0.0 });

        scratch.values[23] = (if (self.params.csrhsti > 0.0) { self.params.csrhsti } else { 0.0 });

        scratch.values[24] = (if (self.params.csrhgat > 0.0) { self.params.csrhgat } else { 0.0 });

        scratch.values[20] = (if (self.params.xjunsti > 1e-9) { self.params.xjunsti } else { 1e-9 });

        scratch.values[21] = (if (self.params.xjungat > 1e-9) { self.params.xjungat } else { 1e-9 });

        scratch.values[25] = (if (self.params.ctatbot > 0.0) { self.params.ctatbot } else { 0.0 });

        scratch.values[26] = (if (self.params.ctatsti > 0.0) { self.params.ctatsti } else { 0.0 });

        scratch.values[27] = (if (self.params.ctatgat > 0.0) { self.params.ctatgat } else { 0.0 });

        scratch.values[28] = (if (self.params.mefftatbot > 0.01) { self.params.mefftatbot } else { 0.01 });

        scratch.values[29] = (if (self.params.mefftatsti > 0.01) { self.params.mefftatsti } else { 0.01 });

        scratch.values[30] = (if (self.params.mefftatgat > 0.01) { self.params.mefftatgat } else { 0.01 });

        scratch.values[31] = (if (self.params.cbbtbot > 0.0) { self.params.cbbtbot } else { 0.0 });

        scratch.values[32] = (if (self.params.cbbtsti > 0.0) { self.params.cbbtsti } else { 0.0 });

        scratch.values[33] = (if (self.params.cbbtgat > 0.0) { self.params.cbbtgat } else { 0.0 });

        scratch.values[34] = self.params.fbbtrbot;

        scratch.values[35] = self.params.fbbtrsti;

        scratch.values[36] = self.params.fbbtrgat;

        scratch.values[37] = self.params.stfbbtbot;

        scratch.values[38] = self.params.stfbbtsti;

        scratch.values[39] = self.params.stfbbtgat;

        scratch.values[40] = (if (self.params.vbrbot > 0.1) { self.params.vbrbot } else { 0.1 });

        scratch.values[41] = (if (self.params.vbrsti > 0.1) { self.params.vbrsti } else { 0.1 });

        scratch.values[42] = (if (self.params.vbrgat > 0.1) { self.params.vbrgat } else { 0.1 });

        scratch.values[43] = (if (self.params.pbrbot > 0.1) { self.params.pbrbot } else { 0.1 });

        scratch.values[44] = (if (self.params.pbrsti > 0.1) { self.params.pbrsti } else { 0.1 });

        scratch.values[45] = (if (self.params.pbrgat > 0.1) { self.params.pbrgat } else { 0.1 });

        scratch.values[46] = 0.0;

        scratch.values[261] = if (self.params.swjunexp > 0.5) { 1.0 } else { 0.0 };

        if (scratch.values[261] != 0.0) {
            scratch.values[46] = 1.0;
            scratch.node_derivatives[46] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[46] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[261] != 0.0)) {
            scratch.values[46] = 0.0;
            scratch.node_derivatives[46] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[46] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[47] = (if (self.params.vjunref > 0.5) { self.params.vjunref } else { 0.5 });

        scratch.values[48] = (if (self.params.fjunq > 0.0) { self.params.fjunq } else { 0.0 });

        scratch.values[49] = (273.15 + scratch.values[2]);

        scratch.values[50] = ((ctx.temperature() + self.params.dta)).max((273.15 + (-250.0)));

        scratch.values[51] = (scratch.values[50] / scratch.values[49]);

        scratch.values[52] = (1.3806505e-23 / 1.6021918e-19);

        scratch.values[53] = (scratch.values[52] * scratch.values[49]);

        scratch.values[54] = (1.0 / scratch.values[53]);

        scratch.values[55] = (scratch.values[52] * scratch.values[50]);

        scratch.values[56] = (1.0 / scratch.values[55]);

        scratch.values[60] = ((-((0.000702 * scratch.values[49]) * scratch.values[49])) / (1108.0 + scratch.values[49]));

        scratch.values[63] = (scratch.values[14] + scratch.values[60]);

        scratch.values[64] = (scratch.values[15] + scratch.values[60]);

        scratch.values[65] = (scratch.values[16] + scratch.values[60]);

        scratch.values[61] = ((-((0.000702 * scratch.values[50]) * scratch.values[50])) / (1108.0 + scratch.values[50]));

        scratch.values[66] = (scratch.values[14] + scratch.values[61]);

        scratch.values[67] = (scratch.values[15] + scratch.values[61]);

        scratch.values[68] = (scratch.values[16] + scratch.values[61]);

        scratch.values[69] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[63] * scratch.values[54]) - (scratch.values[66] * scratch.values[56])))) as f64).exp());

        scratch.values[70] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[64] * scratch.values[54]) - (scratch.values[67] * scratch.values[56])))) as f64).exp());

        scratch.values[71] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[65] * scratch.values[54]) - (scratch.values[68] * scratch.values[56])))) as f64).exp());

        scratch.values[72] = ((scratch.values[17] * scratch.values[69]) * scratch.values[69]);

        scratch.values[73] = ((scratch.values[18] * scratch.values[70]) * scratch.values[70]);

        scratch.values[74] = ((scratch.values[19] * scratch.values[71]) * scratch.values[71]);

        scratch.values[75] = ((scratch.values[8] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[69]) as f64).ln()));

        scratch.values[76] = ((scratch.values[9] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[70]) as f64).ln()));

        scratch.values[77] = ((scratch.values[10] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[71]) as f64).ln()));

        scratch.values[78] = (scratch.values[75] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[75]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[79] = (scratch.values[76] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[76]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[80] = (scratch.values[77] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[77]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[90] = (1.0 / scratch.values[78]);

        scratch.values[91] = (1.0 / scratch.values[79]);

        scratch.values[92] = (1.0 / scratch.values[80]);

        scratch.values[93] = (1.0 - scratch.values[11]);

        scratch.values[94] = (1.0 - scratch.values[12]);

        scratch.values[95] = (1.0 - scratch.values[13]);

        scratch.values[96] = (1.0 / scratch.values[93]);

        scratch.values[97] = (1.0 / scratch.values[94]);

        scratch.values[98] = (1.0 / scratch.values[95]);

        scratch.values[99] = (scratch.values[5] * (((scratch.values[8] * scratch.values[90])) as f64).powf(scratch.values[11]));

        scratch.values[100] = (scratch.values[6] * (((scratch.values[9] * scratch.values[91])) as f64).powf(scratch.values[12]));

        scratch.values[101] = (scratch.values[7] * (((scratch.values[10] * scratch.values[92])) as f64).powf(scratch.values[13]));

        scratch.values[102] = ((scratch.values[99] * scratch.values[78]) * scratch.values[96]);

        scratch.values[103] = ((scratch.values[100] * scratch.values[79]) * scratch.values[97]);

        scratch.values[104] = ((scratch.values[101] * scratch.values[80]) * scratch.values[98]);

        scratch.values[105] = (2.0 * scratch.values[99]);

        scratch.values[106] = (2.0 * scratch.values[100]);

        scratch.values[107] = (2.0 * scratch.values[101]);

        scratch.values[108] = (scratch.values[1] / scratch.values[5]);

        scratch.values[109] = ((scratch.values[20] * scratch.values[1]) / scratch.values[6]);

        scratch.values[110] = ((scratch.values[21] * scratch.values[1]) / scratch.values[7]);

        scratch.values[111] = (1.0 / scratch.values[108]);

        scratch.values[112] = (1.0 / scratch.values[109]);

        scratch.values[113] = (1.0 / scratch.values[110]);

        scratch.values[114] = (1.0 / scratch.values[8]);

        scratch.values[115] = (1.0 / scratch.values[9]);

        scratch.values[116] = (1.0 / scratch.values[10]);

        scratch.values[57] = (1.772453850905516 * 0.29214664);

        scratch.values[58] = (((((-5.0) * 0.29214664) + 6.0) - ((scratch.values[57]) as f64).powf((-2.0))) / 3.0);

        scratch.values[59] = ((1.0 - 0.29214664) - scratch.values[58]);

        scratch.values[117] = ((0.5 * scratch.values[66])).max(scratch.values[55]);

        scratch.values[118] = ((0.5 * scratch.values[67])).max(scratch.values[55]);

        scratch.values[119] = ((0.5 * scratch.values[68])).max(scratch.values[55]);

        scratch.values[120] = (scratch.values[117] * scratch.values[56]);

        scratch.values[121] = (scratch.values[118] * scratch.values[56]);

        scratch.values[122] = (scratch.values[119] * scratch.values[56]);

        scratch.values[123] = (((((((32.0 * scratch.values[28]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[117] * scratch.values[117]) * scratch.values[117]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[124] = (((((((32.0 * scratch.values[29]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[118] * scratch.values[118]) * scratch.values[118]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[125] = (((((((32.0 * scratch.values[30]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[119] * scratch.values[119]) * scratch.values[119]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[126] = (scratch.values[34] * (1.0 + (scratch.values[37] * (scratch.values[50] - scratch.values[49]))));

        scratch.values[127] = (scratch.values[35] * (1.0 + (scratch.values[38] * (scratch.values[50] - scratch.values[49]))));

        scratch.values[128] = (scratch.values[36] * (1.0 + (scratch.values[39] * (scratch.values[50] - scratch.values[49]))));

        if !(scratch.values[126] > 0.0) {
            scratch.values[126] = 0.0;
            scratch.node_derivatives[126] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[126] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[127] > 0.0) {
            scratch.values[127] = 0.0;
            scratch.node_derivatives[127] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[127] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[128] > 0.0) {
            scratch.values[128] = 0.0;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[129] = (1.0 - (1.0 / scratch.values[4]));

        scratch.values[130] = (1.0 / (1.0 - ((scratch.values[129]) as f64).powf(scratch.values[43])));

        scratch.values[131] = (1.0 / (1.0 - ((scratch.values[129]) as f64).powf(scratch.values[44])));

        scratch.values[132] = (1.0 / (1.0 - ((scratch.values[129]) as f64).powf(scratch.values[45])));

        scratch.values[133] = (1.0 / scratch.values[40]);

        scratch.values[134] = (1.0 / scratch.values[41]);

        scratch.values[135] = (1.0 / scratch.values[42]);

        scratch.values[136] = (((-((scratch.values[130] * scratch.values[130]) * ((scratch.values[129]) as f64).powf((scratch.values[43] - 1.0)))) * scratch.values[43]) * scratch.values[133]);

        scratch.values[137] = (((-((scratch.values[131] * scratch.values[131]) * ((scratch.values[129]) as f64).powf((scratch.values[44] - 1.0)))) * scratch.values[44]) * scratch.values[134]);

        scratch.values[138] = (((-((scratch.values[132] * scratch.values[132]) * ((scratch.values[129]) as f64).powf((scratch.values[45] - 1.0)))) * scratch.values[45]) * scratch.values[135]);

        scratch.values[217] = (if (self.params.ab > 0.0) { self.params.ab } else { 0.0 });

        scratch.values[218] = (if (self.params.ls > 0.0) { self.params.ls } else { 0.0 });

        scratch.values[219] = (if (self.params.lg > 0.0) { self.params.lg } else { 0.0 });

        scratch.values[0] = (if (self.params.mult > 0.0) { self.params.mult } else { 0.0 });

        scratch.values[224] = 0.0;

        scratch.values[262] = if ((scratch.values[72] * scratch.values[217]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[262] != 0.0) {
            scratch.values[139] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[72] * scratch.values[217])) + 1.0)) as f64).ln());
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[262] != 0.0)) {
            scratch.values[139] = 100000000.0;
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[263] = if ((scratch.values[73] * scratch.values[218]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[263] != 0.0) {
            scratch.values[140] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[73] * scratch.values[218])) + 1.0)) as f64).ln());
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[263] != 0.0)) {
            scratch.values[140] = 100000000.0;
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[264] = if ((scratch.values[74] * scratch.values[219]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[264] != 0.0) {
            scratch.values[141] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[74] * scratch.values[219])) + 1.0)) as f64).ln());
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[264] != 0.0)) {
            scratch.values[141] = 100000000.0;
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(223, &AdValue::min(AdValue::min(scratch.ad_value(139), scratch.ad_value(140)), scratch.ad_value(141)));

        scratch.values[265] = if ((((scratch.values[223] * scratch.values[56])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (scratch.values[265] != 0.0) {
            scratch.store_ad(224, &AdValue::exp(AdValue::scale(scratch.ad_value(223), scratch.values[56])));
        }

        scratch.values[266] = if ((scratch.values[223] * scratch.values[56]) < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[265] != 0.0)) && (scratch.values[266] != 0.0)) {
            scratch.store_ad(224, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[265] != 0.0)) && (!(scratch.values[266] != 0.0))) {
            scratch.store_ad(224, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        scratch.values[81] = scratch.values[78];

        scratch.values[82] = scratch.values[79];

        scratch.values[83] = scratch.values[80];

        scratch.values[84] = scratch.values[11];

        scratch.values[85] = scratch.values[12];

        scratch.values[86] = scratch.values[13];

        scratch.values[87] = scratch.values[8];

        scratch.values[88] = scratch.values[9];

        scratch.values[89] = scratch.values[10];

        scratch.values[267] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[267] != 0.0) {
            scratch.values[81] = (scratch.values[79] + scratch.values[80]);
            scratch.node_derivatives[81] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[81] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[267] != 0.0) {
            scratch.values[84] = (0.9 * (scratch.values[12]).min(scratch.values[13]));
            scratch.node_derivatives[84] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[84] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[267] != 0.0) {
            scratch.values[87] = (scratch.values[9] + scratch.values[10]);
            scratch.node_derivatives[87] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[87] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[268] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[268] != 0.0) {
            scratch.values[82] = (scratch.values[78] + scratch.values[80]);
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[268] != 0.0) {
            scratch.values[85] = (0.9 * (scratch.values[11]).min(scratch.values[13]));
            scratch.node_derivatives[85] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[85] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[268] != 0.0) {
            scratch.values[88] = (scratch.values[8] + scratch.values[10]);
            scratch.node_derivatives[88] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[88] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[269] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[269] != 0.0) {
            scratch.values[83] = (scratch.values[78] + scratch.values[79]);
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[269] != 0.0) {
            scratch.values[86] = (0.9 * (scratch.values[11]).min(scratch.values[12]));
            scratch.node_derivatives[86] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[86] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[269] != 0.0) {
            scratch.values[89] = (scratch.values[8] + scratch.values[9]);
            scratch.node_derivatives[89] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[89] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(225, &AdValue::min(AdValue::min(scratch.ad_value(81), scratch.ad_value(82)), scratch.ad_value(83)));

        scratch.store_ad(226, &AdValue::scale(scratch.ad_value(225), 0.1));

        scratch.store_ad(62, &AdValue::max(AdValue::max(scratch.ad_value(84), scratch.ad_value(85)), scratch.ad_value(86)));

        scratch.store_ad(227, &AdValue::mul(scratch.ad_value(225), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(62))))));

        scratch.store_ad(228, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(87), scratch.ad_value(88)), scratch.ad_value(89)), (-0.05)));

        scratch.values[235] = 0.0;

        scratch.values[236] = 1.0;

        scratch.values[238] = 1.0;

        scratch.values[237] = 0.0;

        scratch.values[240] = 1.0;

        scratch.values[239] = 0.0;

        scratch.values[241] = 0.0;

        scratch.values[229] = 0.0;

        scratch.values[230] = 0.0;

        scratch.values[231] = 0.0;

        scratch.values[232] = 0.0;

        scratch.values[233] = 0.0;

        scratch.values[234] = 0.0;

        scratch.values[158] = 0.0;

        scratch.values[159] = 0.0;

        scratch.values[147] = 0.0;

        scratch.values[148] = 0.0;

        scratch.values[149] = 0.0;

        scratch.values[150] = 0.0;

        scratch.values[151] = 0.0;

        scratch.values[160] = 0.0;

        scratch.values[161] = 0.0;

        scratch.values[162] = 0.0;

        scratch.values[168] = 0.0;

        scratch.values[220] = 1.0;

        scratch.values[221] = 1.0;

        scratch.values[222] = 1.0;

        scratch.values[157] = 0.0;

        scratch.values[270] = if (scratch.values[46] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[270] != 0.0) {
            scratch.values[271] = 0.0;
            scratch.node_derivatives[271] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[271] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[270] != 0.0) {
            scratch.values[272] = 0.0;
            scratch.node_derivatives[272] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[272] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[273] = 0.0;
            scratch.node_derivatives[273] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[273] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[274] = 0.0;
            scratch.node_derivatives[274] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[274] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[275] = 0.0;
            scratch.node_derivatives[275] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[275] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[276] = 0.0;
            scratch.node_derivatives[276] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[276] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[277] = 0.0;
            scratch.node_derivatives[277] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[277] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[278] = 0.0;
            scratch.node_derivatives[278] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[278] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[279] = 0.0;
            scratch.node_derivatives[279] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[279] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[280] = 0.0;
            scratch.node_derivatives[280] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[280] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[281] = 0.0;
            scratch.node_derivatives[281] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[281] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[282] = 0.0;
            scratch.node_derivatives[282] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[282] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[283] = 0.0;
            scratch.node_derivatives[283] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[283] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[284] = 0.0;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[285] = 0.0;
            scratch.node_derivatives[285] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[285] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[286] = 0.0;
            scratch.node_derivatives[286] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[286] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[288] = 0.0;
            scratch.node_derivatives[288] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[288] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[289] = 0.0;
            scratch.node_derivatives[289] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[289] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[290] = 0.0;
            scratch.node_derivatives[290] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[290] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[292] = 0.0;
            scratch.node_derivatives[292] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[292] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[293] = 0.0;
            scratch.node_derivatives[293] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[293] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[295] = 0.0;
            scratch.node_derivatives[295] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[295] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[296] = 0.0;
            scratch.node_derivatives[296] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[296] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[297] = 0.0;
            scratch.node_derivatives[297] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[297] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[299] = 0.0;
            scratch.node_derivatives[299] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[299] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[300] = 0.0;
            scratch.node_derivatives[300] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[300] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[301] = 0.0;
            scratch.node_derivatives[301] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[301] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[302] = 0.0;
            scratch.node_derivatives[302] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[302] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[303] = 0.0;
            scratch.node_derivatives[303] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[303] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[304] = 0.0;
            scratch.node_derivatives[304] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[304] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[305] = 0.0;
            scratch.node_derivatives[305] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[305] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[306] = 0.0;
            scratch.node_derivatives[306] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[306] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[307] = 0.0;
            scratch.node_derivatives[307] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[307] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[308] = 0.0;
            scratch.node_derivatives[308] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[308] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[309] = 0.0;
            scratch.node_derivatives[309] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[309] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[310] = 0.0;
            scratch.node_derivatives[310] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[310] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[311] = 0.0;
            scratch.node_derivatives[311] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[311] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[312] = 0.0;
            scratch.node_derivatives[312] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[312] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[314] = 0.0;
            scratch.node_derivatives[314] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[314] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[315] = 0.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[165] = 0.4;
            scratch.node_derivatives[165] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[165] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[166] = 0.65;
            scratch.node_derivatives[166] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[166] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[167] = 0.8;
            scratch.node_derivatives[167] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[167] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(152, &AdValue::scale(AdValue::neg(scratch.ad_value(165)), scratch.values[47]));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(153, &AdValue::scale(AdValue::neg(scratch.ad_value(166)), scratch.values[47]));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(154, &AdValue::scale(AdValue::neg(scratch.ad_value(167)), scratch.values[47]));
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[155] = 0.1;
            scratch.node_derivatives[155] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[155] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[156] = 0.2;
            scratch.node_derivatives[156] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[156] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[284] = 0.0;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[322] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(274, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(275, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(276, &AdValue::add(scratch.ad_value(152), AdValue::mul(scratch.ad_value(226), scratch.ad_value(275))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(277, &AdValue::add(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(278, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(279, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(278)), scratch.ad_value(274))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(152), scratch.ad_value(227)), AdValue::add(scratch.ad_value(277), scratch.ad_value(279))), 2.0));
        }

        scratch.values[323] = if (scratch.values[152] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[324] = if ((((0.5 * (scratch.values[152] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (scratch.values[323] != 0.0)) && (scratch.values[324] != 0.0)) {
            scratch.store_ad(283, &AdValue::exp(AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5))));
        }

        scratch.values[325] = if ((0.5 * (scratch.values[152] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (scratch.values[323] != 0.0)) && (!(scratch.values[324] != 0.0))) && (scratch.values[325] != 0.0)) {
            let assign2790_ad_e1591: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(283, &assign2790_ad_e1591);
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (scratch.values[323] != 0.0)) && (!(scratch.values[324] != 0.0))) && (!(scratch.values[325] != 0.0))) {
            scratch.store_ad(283, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(152), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (scratch.values[323] != 0.0)) {
            scratch.store_ad(280, &AdValue::square(scratch.ad_value(283)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (!(scratch.values[323] != 0.0))) {
            scratch.store_ad(280, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(152), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (!(scratch.values[323] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(scratch.ad_value(280)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(280, &AdValue::offset(scratch.ad_value(280), (-1.0)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(282, &AdValue::div_from_scalar(1.0, scratch.ad_value(283)));
        }

        scratch.values[326] = if (scratch.values[152] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (scratch.values[326] != 0.0)) {
            scratch.store_ad(284, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(282), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(282), 1.0), AdValue::offset(scratch.ad_value(282), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) && (!(scratch.values[326] != 0.0))) {
            scratch.store_ad(284, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(283), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(283), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(283), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(152)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(285, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(284)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(286, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(152), scratch.ad_value(285)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(152), scratch.ad_value(285)), AdValue::sub(scratch.ad_value(152), scratch.ad_value(285))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(287, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(152), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(152), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(152), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[322] != 0.0)) {
            scratch.store_ad(288, &AdValue::scale(AdValue::sub(scratch.ad_value(152), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(152), scratch.ad_value(152)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[327] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[327] != 0.0)) {
            scratch.values[316] = 0.0;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[328] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (scratch.values[328] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[328] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[72]));
        }

        scratch.values[329] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (scratch.values[329] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[330] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) && (scratch.values[330] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) && (!(scratch.values[330] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[331] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) && (scratch.values[331] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) && (!(scratch.values[331] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[108]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[69]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[329] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[22]));
        }

        scratch.values[332] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (scratch.values[332] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[93]), scratch.ad_value(292)), scratch.values[123]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[333] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (scratch.values[333] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[333] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[120]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[334] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (scratch.values[334] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[334] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[335] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (scratch.values[335] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[335] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[336] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (scratch.values[336] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[337] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[336] != 0.0))) && (scratch.values[337] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[336] != 0.0))) && (!(scratch.values[337] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) && (!(scratch.values[336] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[120]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[332] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[25]));
        }

        scratch.values[338] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (scratch.values[338] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[339] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) && (scratch.values[339] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) && (!(scratch.values[339] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[111]), scratch.ad_value(289)), scratch.values[96]));
        }

        scratch.values[340] = if (((((-scratch.values[126]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) && (scratch.values[340] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))));
        }

        scratch.values[341] = if (((-scratch.values[126]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) && (!(scratch.values[340] != 0.0))) && (scratch.values[341] != 0.0)) {
            let assign3560_ad_e2739: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign3560_ad_e2739));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) && (!(scratch.values[340] != 0.0))) && (!(scratch.values[341] != 0.0))) {
            let assign3570_ad_e2787: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign3570_ad_e2787);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[338] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(152), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[31]));
        }

        scratch.values[342] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (scratch.values[342] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[343] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[344] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[342] != 0.0))) && (scratch.values[343] != 0.0)) && (scratch.values[344] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[133]), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[342] != 0.0))) && (scratch.values[343] != 0.0)) && (!(scratch.values[344] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[133])), scratch.values[43]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[342] != 0.0))) && (scratch.values[343] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) && (!(scratch.values[342] != 0.0))) && (!(scratch.values[343] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[327] != 0.0))) {
            scratch.store_ad(316, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[345] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[345] != 0.0)) {
            scratch.values[317] = 0.0;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[346] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (scratch.values[346] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[346] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[73]));
        }

        scratch.values[347] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (scratch.values[347] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[348] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) && (scratch.values[348] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) && (!(scratch.values[348] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[349] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) && (scratch.values[349] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) && (!(scratch.values[349] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[109]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[70]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[347] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[23]));
        }

        scratch.values[350] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (scratch.values[350] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[94]), scratch.ad_value(292)), scratch.values[124]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[351] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (scratch.values[351] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[351] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[121]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[352] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (scratch.values[352] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[352] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[353] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (scratch.values[353] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[353] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[354] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (scratch.values[354] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[355] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[354] != 0.0))) && (scratch.values[355] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[354] != 0.0))) && (!(scratch.values[355] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) && (!(scratch.values[354] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[121]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[350] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[26]));
        }

        scratch.values[356] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (scratch.values[356] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[357] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) && (scratch.values[357] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) && (!(scratch.values[357] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[112]), scratch.ad_value(289)), scratch.values[97]));
        }

        scratch.values[358] = if (((((-scratch.values[127]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) && (scratch.values[358] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))));
        }

        scratch.values[359] = if (((-scratch.values[127]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) && (!(scratch.values[358] != 0.0))) && (scratch.values[359] != 0.0)) {
            let assign4310_ad_e3831: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign4310_ad_e3831));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) && (!(scratch.values[358] != 0.0))) && (!(scratch.values[359] != 0.0))) {
            let assign4320_ad_e3879: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign4320_ad_e3879);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[356] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(152), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[32]));
        }

        scratch.values[360] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (scratch.values[360] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[361] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[362] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[360] != 0.0))) && (scratch.values[361] != 0.0)) && (scratch.values[362] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[134]), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[360] != 0.0))) && (scratch.values[361] != 0.0)) && (!(scratch.values[362] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[134])), scratch.values[44]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[360] != 0.0))) && (scratch.values[361] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) && (!(scratch.values[360] != 0.0))) && (!(scratch.values[361] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[345] != 0.0))) {
            scratch.store_ad(317, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[363] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[363] != 0.0)) {
            scratch.values[318] = 0.0;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[364] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (scratch.values[364] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[364] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[74]));
        }

        scratch.values[365] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (scratch.values[365] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[366] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) && (scratch.values[366] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) && (!(scratch.values[366] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[367] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) && (scratch.values[367] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) && (!(scratch.values[367] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[116]), scratch.values[13]));
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[110]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[71]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[365] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[24]));
        }

        scratch.values[368] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (scratch.values[368] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[95]), scratch.ad_value(292)), scratch.values[125]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[369] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (scratch.values[369] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[369] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[122]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[370] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (scratch.values[370] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[370] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[371] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (scratch.values[371] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[371] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[372] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (scratch.values[372] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[373] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[372] != 0.0))) && (scratch.values[373] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[372] != 0.0))) && (!(scratch.values[373] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) && (!(scratch.values[372] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[122]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[368] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[27]));
        }

        scratch.values[374] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (scratch.values[374] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[375] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) && (scratch.values[375] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) && (!(scratch.values[375] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[113]), scratch.ad_value(289)), scratch.values[98]));
        }

        scratch.values[376] = if (((((-scratch.values[128]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) && (scratch.values[376] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))));
        }

        scratch.values[377] = if (((-scratch.values[128]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) && (!(scratch.values[376] != 0.0))) && (scratch.values[377] != 0.0)) {
            let assign5060_ad_e4923: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign5060_ad_e4923));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) && (!(scratch.values[376] != 0.0))) && (!(scratch.values[377] != 0.0))) {
            let assign5070_ad_e4971: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign5070_ad_e4971);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[374] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(152), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[33]));
        }

        scratch.values[378] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (scratch.values[378] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[379] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[380] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[378] != 0.0))) && (scratch.values[379] != 0.0)) && (scratch.values[380] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[135]), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[378] != 0.0))) && (scratch.values[379] != 0.0)) && (!(scratch.values[380] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[135])), scratch.values[45]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[378] != 0.0))) && (scratch.values[379] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) && (!(scratch.values[378] != 0.0))) && (!(scratch.values[379] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[363] != 0.0))) {
            scratch.store_ad(318, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(142, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(316), scratch.values[217]), AdValue::scale(scratch.ad_value(317), scratch.values[218])), AdValue::scale(scratch.ad_value(318), scratch.values[219])));
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[284] = 0.0;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[381] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(274, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(275, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(276, &AdValue::add(scratch.ad_value(153), AdValue::mul(scratch.ad_value(226), scratch.ad_value(275))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(277, &AdValue::add(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(278, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(279, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(278)), scratch.ad_value(274))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(153), scratch.ad_value(227)), AdValue::add(scratch.ad_value(277), scratch.ad_value(279))), 2.0));
        }

        scratch.values[382] = if (scratch.values[153] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[383] = if ((((0.5 * (scratch.values[153] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (scratch.values[382] != 0.0)) && (scratch.values[383] != 0.0)) {
            scratch.store_ad(283, &AdValue::exp(AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5))));
        }

        scratch.values[384] = if ((0.5 * (scratch.values[153] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (scratch.values[382] != 0.0)) && (!(scratch.values[383] != 0.0))) && (scratch.values[384] != 0.0)) {
            let assign5330_ad_e5296: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(283, &assign5330_ad_e5296);
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (scratch.values[382] != 0.0)) && (!(scratch.values[383] != 0.0))) && (!(scratch.values[384] != 0.0))) {
            scratch.store_ad(283, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(153), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (scratch.values[382] != 0.0)) {
            scratch.store_ad(280, &AdValue::square(scratch.ad_value(283)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (!(scratch.values[382] != 0.0))) {
            scratch.store_ad(280, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(153), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (!(scratch.values[382] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(scratch.ad_value(280)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(280, &AdValue::offset(scratch.ad_value(280), (-1.0)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(282, &AdValue::div_from_scalar(1.0, scratch.ad_value(283)));
        }

        scratch.values[385] = if (scratch.values[153] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (scratch.values[385] != 0.0)) {
            scratch.store_ad(284, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(282), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(282), 1.0), AdValue::offset(scratch.ad_value(282), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) && (!(scratch.values[385] != 0.0))) {
            scratch.store_ad(284, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(283), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(283), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(283), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(153)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(285, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(284)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(286, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(153), scratch.ad_value(285)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(153), scratch.ad_value(285)), AdValue::sub(scratch.ad_value(153), scratch.ad_value(285))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(287, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(153), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(153), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(153), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[381] != 0.0)) {
            scratch.store_ad(288, &AdValue::scale(AdValue::sub(scratch.ad_value(153), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(153), scratch.ad_value(153)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[386] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[386] != 0.0)) {
            scratch.values[316] = 0.0;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[387] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (scratch.values[387] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[387] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[72]));
        }

        scratch.values[388] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (scratch.values[388] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[389] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) && (scratch.values[389] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) && (!(scratch.values[389] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[390] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) && (scratch.values[390] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) && (!(scratch.values[390] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[108]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[69]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[388] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[22]));
        }

        scratch.values[391] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (scratch.values[391] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[93]), scratch.ad_value(292)), scratch.values[123]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[392] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (scratch.values[392] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[392] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[120]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[393] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (scratch.values[393] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[393] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[394] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (scratch.values[394] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[394] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[395] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (scratch.values[395] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[396] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[395] != 0.0))) && (scratch.values[396] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[395] != 0.0))) && (!(scratch.values[396] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) && (!(scratch.values[395] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[120]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

    }
}
