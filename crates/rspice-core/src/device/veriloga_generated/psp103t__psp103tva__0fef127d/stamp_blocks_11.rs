#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[1263] = if (self.params.type_ >= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1263] != 0.0) {
            scratch.values[0] = 1.0;
            scratch.node_derivatives[0] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[0] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1263] != 0.0)) {
            scratch.values[0] = (-1.0);
            scratch.node_derivatives[0] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[0] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[806] = (8.8541878176e-12 * 11.8);

        scratch.values[1] = ((((if (self.params.swgeo > 0.0) { (if (self.params.swgeo < 2.0) { self.params.swgeo } else { 2.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[2] = ((((if (self.params.swigate > 0.0) { (if (self.params.swigate < 1.0) { self.params.swigate } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[3] = ((((if (self.params.swimpact > 0.0) { (if (self.params.swimpact < 1.0) { self.params.swimpact } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[4] = ((((if (self.params.swgidl > 0.0) { (if (self.params.swgidl < 1.0) { self.params.swgidl } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[5] = ((((if (self.params.swjuncap > 0.0) { (if (self.params.swjuncap < 3.0) { self.params.swjuncap } else { 3.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[6] = ((((if (self.params.swjunasym > 0.0) { (if (self.params.swjunasym < 1.0) { self.params.swjunasym } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[7] = ((((if (self.params.swnud > 0.0) { (if (self.params.swnud < 2.0) { self.params.swnud } else { 2.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[8] = ((((if (self.params.swedge > 0.0) { (if (self.params.swedge < 1.0) { self.params.swedge } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[9] = ((((if (self.params.swdelvtac > 0.0) { (if (self.params.swdelvtac < 1.0) { self.params.swdelvtac } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[10] = ((((if (self.params.swign > 0.0) { (if (self.params.swign < 1.0) { self.params.swign } else { 1.0 }) } else { 0.0 }) + 0.5)) as f64).floor();

        scratch.values[191] = (if (self.params.qmc > 0.0) { self.params.qmc } else { 0.0 });

        scratch.values[38] = (if (self.params.toxo > 1e-10) { self.params.toxo } else { 1e-10 });

        scratch.values[39] = (if (self.params.epsroxo > 1.0) { self.params.epsroxo } else { 1.0 });

        scratch.values[40] = (if (self.params.nsubo > 1e20) { self.params.nsubo } else { 1e20 });

        scratch.values[41] = (if (self.params.wseg > 1e-10) { self.params.wseg } else { 1e-10 });

        scratch.values[42] = (if (self.params.npck > 0.0) { self.params.npck } else { 0.0 });

        scratch.values[43] = (if (self.params.wsegp > 1e-10) { self.params.wsegp } else { 1e-10 });

        scratch.values[44] = (if (self.params.lpck > 1e-10) { self.params.lpck } else { 1e-10 });

        scratch.values[45] = (if (self.params.toxovo > 1e-10) { self.params.toxovo } else { 1e-10 });

        scratch.values[46] = (if (self.params.toxovdo > 1e-10) { self.params.toxovdo } else { 1e-10 });

        scratch.values[47] = (if (self.params.lov > 0.0) { self.params.lov } else { 0.0 });

        scratch.values[48] = (if (self.params.lovd > 0.0) { self.params.lovd } else { 0.0 });

        scratch.values[49] = (if (self.params.lp1 > 1e-10) { self.params.lp1 } else { 1e-10 });

        scratch.values[50] = (if (self.params.lp2 > 1e-10) { self.params.lp2 } else { 1e-10 });

        scratch.values[51] = (if (self.params.wbet > 1e-10) { self.params.wbet } else { 1e-10 });

        scratch.values[52] = (if (self.params.axl > 0.0) { self.params.axl } else { 0.0 });

        scratch.values[55] = (if (self.params.saref > 1e-9) { self.params.saref } else { 1e-9 });

        scratch.values[56] = (if (self.params.sbref > 1e-9) { self.params.sbref } else { 1e-9 });

        scratch.values[57] = (if (self.params.kvsat > (-1.0)) { (if (self.params.kvsat < 1.0) { self.params.kvsat } else { 1.0 }) } else { (-1.0) });

        scratch.values[58] = (if (self.params.llodkuo > 0.0) { self.params.llodkuo } else { 0.0 });

        scratch.values[59] = (if (self.params.wlodkuo > 0.0) { self.params.wlodkuo } else { 0.0 });

        scratch.values[60] = (if (self.params.llodvth > 0.0) { self.params.llodvth } else { 0.0 });

        scratch.values[61] = (if (self.params.wlodvth > 0.0) { self.params.wlodvth } else { 0.0 });

        scratch.values[62] = (if (self.params.lodetao > 0.0) { self.params.lodetao } else { 0.0 });

        scratch.values[63] = (if (self.params.scref > 0.0) { self.params.scref } else { 0.0 });

        scratch.values[64] = self.params.web;

        scratch.values[65] = self.params.wec;

        scratch.values[71] = (if (self.params.nsubedgeo > 1e20) { self.params.nsubedgeo } else { 1e20 });

        scratch.values[72] = (if (self.params.lpedge > 1e-10) { self.params.lpedge } else { 1e-10 });

        scratch.values[190] = (if (self.params.tr > (-273.0)) { self.params.tr } else { (-273.0) });

        scratch.values[359] = (273.15 + scratch.values[190]);

        scratch.values[360] = (ctx.temperature() + self.params.dta);

        scratch.values[361] = (scratch.values[360] / scratch.values[359]);

        scratch.values[362] = (scratch.values[360] - scratch.values[359]);

        scratch.values[363] = ((scratch.values[360] * 1.3806505e-23) / 1.6021918e-19);

        scratch.values[364] = (1.0 / scratch.values[363]);

        scratch.values[373] = (if (self.params.trj > (-250.0)) { self.params.trj } else { (-250.0) });

        scratch.values[374] = (if (self.params.imax > 1e-12) { self.params.imax } else { 1e-12 });

        scratch.values[375] = (if (self.params.frev > 10.0) { (if (self.params.frev < 10000000000.0) { self.params.frev } else { 10000000000.0 }) } else { 10.0 });

        scratch.values[376] = (if (self.params.cjorbot > 1e-12) { self.params.cjorbot } else { 1e-12 });

        scratch.values[377] = (if (self.params.cjorsti > 1e-18) { self.params.cjorsti } else { 1e-18 });

        scratch.values[378] = (if (self.params.cjorgat > 1e-18) { self.params.cjorgat } else { 1e-18 });

        scratch.values[379] = (if (self.params.vbirbot > 0.05) { self.params.vbirbot } else { 0.05 });

        scratch.values[380] = (if (self.params.vbirsti > 0.05) { self.params.vbirsti } else { 0.05 });

        scratch.values[381] = (if (self.params.vbirgat > 0.05) { self.params.vbirgat } else { 0.05 });

        scratch.values[382] = (if (self.params.pbot > 0.05) { (if (self.params.pbot < 0.95) { self.params.pbot } else { 0.95 }) } else { 0.05 });

        scratch.values[383] = (if (self.params.psti > 0.05) { (if (self.params.psti < 0.95) { self.params.psti } else { 0.95 }) } else { 0.05 });

        scratch.values[384] = (if (self.params.pgat > 0.05) { (if (self.params.pgat < 0.95) { self.params.pgat } else { 0.95 }) } else { 0.05 });

        scratch.values[385] = self.params.phigbot;

        scratch.values[386] = self.params.phigsti;

        scratch.values[387] = self.params.phiggat;

        scratch.values[388] = (if (self.params.idsatrbot > 0.0) { self.params.idsatrbot } else { 0.0 });

        scratch.values[389] = (if (self.params.idsatrsti > 0.0) { self.params.idsatrsti } else { 0.0 });

        scratch.values[390] = (if (self.params.idsatrgat > 0.0) { self.params.idsatrgat } else { 0.0 });

        scratch.values[393] = (if (self.params.csrhbot > 0.0) { self.params.csrhbot } else { 0.0 });

        scratch.values[394] = (if (self.params.csrhsti > 0.0) { self.params.csrhsti } else { 0.0 });

        scratch.values[395] = (if (self.params.csrhgat > 0.0) { self.params.csrhgat } else { 0.0 });

        scratch.values[391] = (if (self.params.xjunsti > 1e-9) { self.params.xjunsti } else { 1e-9 });

        scratch.values[392] = (if (self.params.xjungat > 1e-9) { self.params.xjungat } else { 1e-9 });

        scratch.values[396] = (if (self.params.ctatbot > 0.0) { self.params.ctatbot } else { 0.0 });

        scratch.values[397] = (if (self.params.ctatsti > 0.0) { self.params.ctatsti } else { 0.0 });

        scratch.values[398] = (if (self.params.ctatgat > 0.0) { self.params.ctatgat } else { 0.0 });

        scratch.values[399] = (if (self.params.mefftatbot > 0.01) { self.params.mefftatbot } else { 0.01 });

        scratch.values[400] = (if (self.params.mefftatsti > 0.01) { self.params.mefftatsti } else { 0.01 });

        scratch.values[401] = (if (self.params.mefftatgat > 0.01) { self.params.mefftatgat } else { 0.01 });

        scratch.values[402] = (if (self.params.cbbtbot > 0.0) { self.params.cbbtbot } else { 0.0 });

        scratch.values[403] = (if (self.params.cbbtsti > 0.0) { self.params.cbbtsti } else { 0.0 });

        scratch.values[404] = (if (self.params.cbbtgat > 0.0) { self.params.cbbtgat } else { 0.0 });

        scratch.values[405] = self.params.fbbtrbot;

        scratch.values[406] = self.params.fbbtrsti;

        scratch.values[407] = self.params.fbbtrgat;

        scratch.values[408] = self.params.stfbbtbot;

        scratch.values[409] = self.params.stfbbtsti;

        scratch.values[410] = self.params.stfbbtgat;

        scratch.values[411] = (if (self.params.vbrbot > 0.1) { self.params.vbrbot } else { 0.1 });

        scratch.values[412] = (if (self.params.vbrsti > 0.1) { self.params.vbrsti } else { 0.1 });

        scratch.values[413] = (if (self.params.vbrgat > 0.1) { self.params.vbrgat } else { 0.1 });

        scratch.values[414] = (if (self.params.pbrbot > 0.1) { self.params.pbrbot } else { 0.1 });

        scratch.values[415] = (if (self.params.pbrsti > 0.1) { self.params.pbrsti } else { 0.1 });

        scratch.values[416] = (if (self.params.pbrgat > 0.1) { self.params.pbrgat } else { 0.1 });

        scratch.values[417] = 0.0;

        scratch.values[1264] = if (self.params.swjunexp > 0.5) { 1.0 } else { 0.0 };

        if (scratch.values[1264] != 0.0) {
            scratch.values[417] = 1.0;
            scratch.node_derivatives[417] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[417] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1264] != 0.0)) {
            scratch.values[417] = 0.0;
            scratch.node_derivatives[417] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[417] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[419] = (if (self.params.fjunq > 0.0) { self.params.fjunq } else { 0.0 });

        scratch.values[1265] = if (self.params.swjunasym == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1265] != 0.0) {
            scratch.values[543] = scratch.values[376];
            scratch.node_derivatives[543] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[543] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[544] = scratch.values[377];
            scratch.node_derivatives[544] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[544] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[545] = scratch.values[378];
            scratch.node_derivatives[545] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[545] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[546] = scratch.values[379];
            scratch.node_derivatives[546] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[546] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[547] = scratch.values[380];
            scratch.node_derivatives[547] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[547] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[548] = scratch.values[381];
            scratch.node_derivatives[548] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[548] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[549] = scratch.values[382];
            scratch.node_derivatives[549] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[549] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[550] = scratch.values[383];
            scratch.node_derivatives[550] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[550] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[551] = scratch.values[384];
            scratch.node_derivatives[551] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[551] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[552] = scratch.values[385];
            scratch.node_derivatives[552] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[552] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[553] = scratch.values[386];
            scratch.node_derivatives[553] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[553] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[554] = scratch.values[387];
            scratch.node_derivatives[554] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[554] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[555] = scratch.values[388];
            scratch.node_derivatives[555] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[555] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[556] = scratch.values[389];
            scratch.node_derivatives[556] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[556] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[557] = scratch.values[390];
            scratch.node_derivatives[557] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[557] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[560] = scratch.values[393];
            scratch.node_derivatives[560] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[560] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[561] = scratch.values[394];
            scratch.node_derivatives[561] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[561] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[562] = scratch.values[395];
            scratch.node_derivatives[562] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[562] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[558] = scratch.values[391];
            scratch.node_derivatives[558] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[558] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[559] = scratch.values[392];
            scratch.node_derivatives[559] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[559] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[563] = scratch.values[396];
            scratch.node_derivatives[563] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[563] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[564] = scratch.values[397];
            scratch.node_derivatives[564] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[564] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[565] = scratch.values[398];
            scratch.node_derivatives[565] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[565] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[566] = scratch.values[399];
            scratch.node_derivatives[566] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[566] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[567] = scratch.values[400];
            scratch.node_derivatives[567] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[567] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[568] = scratch.values[401];
            scratch.node_derivatives[568] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[568] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[569] = scratch.values[402];
            scratch.node_derivatives[569] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[569] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[570] = scratch.values[403];
            scratch.node_derivatives[570] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[570] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[571] = scratch.values[404];
            scratch.node_derivatives[571] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[571] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[572] = scratch.values[405];
            scratch.node_derivatives[572] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[572] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[573] = scratch.values[406];
            scratch.node_derivatives[573] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[573] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[574] = scratch.values[407];
            scratch.node_derivatives[574] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[574] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[575] = scratch.values[408];
            scratch.node_derivatives[575] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[575] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[576] = scratch.values[409];
            scratch.node_derivatives[576] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[576] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[577] = scratch.values[410];
            scratch.node_derivatives[577] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[577] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[578] = scratch.values[411];
            scratch.node_derivatives[578] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[578] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[579] = scratch.values[412];
            scratch.node_derivatives[579] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[579] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[580] = scratch.values[413];
            scratch.node_derivatives[580] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[580] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[581] = scratch.values[414];
            scratch.node_derivatives[581] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[581] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[582] = scratch.values[415];
            scratch.node_derivatives[582] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[582] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[583] = scratch.values[416];
            scratch.node_derivatives[583] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[583] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1265] != 0.0) {
            scratch.values[585] = scratch.values[419];
            scratch.node_derivatives[585] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[585] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[543] = (if (self.params.cjorbotd > 1e-12) { self.params.cjorbotd } else { 1e-12 });
            scratch.node_derivatives[543] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[543] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[544] = (if (self.params.cjorstid > 1e-18) { self.params.cjorstid } else { 1e-18 });
            scratch.node_derivatives[544] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[544] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[545] = (if (self.params.cjorgatd > 1e-18) { self.params.cjorgatd } else { 1e-18 });
            scratch.node_derivatives[545] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[545] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[546] = (if (self.params.vbirbotd > 0.05) { self.params.vbirbotd } else { 0.05 });
            scratch.node_derivatives[546] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[546] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[547] = (if (self.params.vbirstid > 0.05) { self.params.vbirstid } else { 0.05 });
            scratch.node_derivatives[547] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[547] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[548] = (if (self.params.vbirgatd > 0.05) { self.params.vbirgatd } else { 0.05 });
            scratch.node_derivatives[548] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[548] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[549] = (if (self.params.pbotd > 0.05) { (if (self.params.pbotd < 0.95) { self.params.pbotd } else { 0.95 }) } else { 0.05 });
            scratch.node_derivatives[549] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[549] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[550] = (if (self.params.pstid > 0.05) { (if (self.params.pstid < 0.95) { self.params.pstid } else { 0.95 }) } else { 0.05 });
            scratch.node_derivatives[550] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[550] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[551] = (if (self.params.pgatd > 0.05) { (if (self.params.pgatd < 0.95) { self.params.pgatd } else { 0.95 }) } else { 0.05 });
            scratch.node_derivatives[551] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[551] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[552] = self.params.phigbotd;
            scratch.node_derivatives[552] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[552] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[553] = self.params.phigstid;
            scratch.node_derivatives[553] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[553] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[554] = self.params.phiggatd;
            scratch.node_derivatives[554] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[554] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[555] = (if (self.params.idsatrbotd > 0.0) { self.params.idsatrbotd } else { 0.0 });
            scratch.node_derivatives[555] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[555] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[556] = (if (self.params.idsatrstid > 0.0) { self.params.idsatrstid } else { 0.0 });
            scratch.node_derivatives[556] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[556] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[557] = (if (self.params.idsatrgatd > 0.0) { self.params.idsatrgatd } else { 0.0 });
            scratch.node_derivatives[557] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[557] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[560] = (if (self.params.csrhbotd > 0.0) { self.params.csrhbotd } else { 0.0 });
            scratch.node_derivatives[560] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[560] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[561] = (if (self.params.csrhstid > 0.0) { self.params.csrhstid } else { 0.0 });
            scratch.node_derivatives[561] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[561] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[562] = (if (self.params.csrhgatd > 0.0) { self.params.csrhgatd } else { 0.0 });
            scratch.node_derivatives[562] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[562] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[558] = (if (self.params.xjunstid > 1e-9) { self.params.xjunstid } else { 1e-9 });
            scratch.node_derivatives[558] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[558] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[559] = (if (self.params.xjungatd > 1e-9) { self.params.xjungatd } else { 1e-9 });
            scratch.node_derivatives[559] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[559] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[563] = (if (self.params.ctatbotd > 0.0) { self.params.ctatbotd } else { 0.0 });
            scratch.node_derivatives[563] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[563] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[564] = (if (self.params.ctatstid > 0.0) { self.params.ctatstid } else { 0.0 });
            scratch.node_derivatives[564] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[564] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[565] = (if (self.params.ctatgatd > 0.0) { self.params.ctatgatd } else { 0.0 });
            scratch.node_derivatives[565] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[565] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[566] = (if (self.params.mefftatbotd > 0.01) { self.params.mefftatbotd } else { 0.01 });
            scratch.node_derivatives[566] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[566] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[567] = (if (self.params.mefftatstid > 0.01) { self.params.mefftatstid } else { 0.01 });
            scratch.node_derivatives[567] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[567] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[568] = (if (self.params.mefftatgatd > 0.01) { self.params.mefftatgatd } else { 0.01 });
            scratch.node_derivatives[568] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[568] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[569] = (if (self.params.cbbtbotd > 0.0) { self.params.cbbtbotd } else { 0.0 });
            scratch.node_derivatives[569] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[569] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[570] = (if (self.params.cbbtstid > 0.0) { self.params.cbbtstid } else { 0.0 });
            scratch.node_derivatives[570] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[570] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[571] = (if (self.params.cbbtgatd > 0.0) { self.params.cbbtgatd } else { 0.0 });
            scratch.node_derivatives[571] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[571] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[572] = self.params.fbbtrbotd;
            scratch.node_derivatives[572] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[572] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[573] = self.params.fbbtrstid;
            scratch.node_derivatives[573] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[573] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[574] = self.params.fbbtrgatd;
            scratch.node_derivatives[574] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[574] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[575] = self.params.stfbbtbotd;
            scratch.node_derivatives[575] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[575] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[576] = self.params.stfbbtstid;
            scratch.node_derivatives[576] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[576] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[577] = self.params.stfbbtgatd;
            scratch.node_derivatives[577] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[577] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[578] = (if (self.params.vbrbotd > 0.1) { self.params.vbrbotd } else { 0.1 });
            scratch.node_derivatives[578] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[578] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[579] = (if (self.params.vbrstid > 0.1) { self.params.vbrstid } else { 0.1 });
            scratch.node_derivatives[579] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[579] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[580] = (if (self.params.vbrgatd > 0.1) { self.params.vbrgatd } else { 0.1 });
            scratch.node_derivatives[580] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[580] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[581] = (if (self.params.pbrbotd > 0.1) { self.params.pbrbotd } else { 0.1 });
            scratch.node_derivatives[581] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[581] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[582] = (if (self.params.pbrstid > 0.1) { self.params.pbrstid } else { 0.1 });
            scratch.node_derivatives[582] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[582] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[583] = (if (self.params.pbrgatd > 0.1) { self.params.pbrgatd } else { 0.1 });
            scratch.node_derivatives[583] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[583] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1265] != 0.0)) {
            scratch.values[585] = (if (self.params.fjunqd > 0.0) { self.params.fjunqd } else { 0.0 });
            scratch.node_derivatives[585] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[585] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[420] = (273.15 + scratch.values[373]);

        scratch.values[421] = ((ctx.temperature() + self.params.dta)).max((273.15 + (-250.0)));

        scratch.values[422] = (scratch.values[421] / scratch.values[420]);

        scratch.values[423] = (1.3806505e-23 / 1.6021918e-19);

        scratch.values[424] = (scratch.values[423] * scratch.values[420]);

        scratch.values[425] = (1.0 / scratch.values[424]);

        scratch.values[426] = (scratch.values[423] * scratch.values[421]);

        scratch.values[427] = (1.0 / scratch.values[426]);

        scratch.values[431] = ((-((0.000702 * scratch.values[420]) * scratch.values[420])) / (1108.0 + scratch.values[420]));

        scratch.values[434] = (scratch.values[385] + scratch.values[431]);

        scratch.values[435] = (scratch.values[386] + scratch.values[431]);

        scratch.values[436] = (scratch.values[387] + scratch.values[431]);

        scratch.values[432] = ((-((0.000702 * scratch.values[421]) * scratch.values[421])) / (1108.0 + scratch.values[421]));

        scratch.values[437] = (scratch.values[385] + scratch.values[432]);

        scratch.values[438] = (scratch.values[386] + scratch.values[432]);

        scratch.values[439] = (scratch.values[387] + scratch.values[432]);

        scratch.values[440] = (((scratch.values[422]) as f64).powf(1.5) * (((0.5 * ((scratch.values[434] * scratch.values[425]) - (scratch.values[437] * scratch.values[427])))) as f64).exp());

        scratch.values[441] = (((scratch.values[422]) as f64).powf(1.5) * (((0.5 * ((scratch.values[435] * scratch.values[425]) - (scratch.values[438] * scratch.values[427])))) as f64).exp());

        scratch.values[442] = (((scratch.values[422]) as f64).powf(1.5) * (((0.5 * ((scratch.values[436] * scratch.values[425]) - (scratch.values[439] * scratch.values[427])))) as f64).exp());

        scratch.values[443] = ((scratch.values[388] * scratch.values[440]) * scratch.values[440]);

        scratch.values[444] = ((scratch.values[389] * scratch.values[441]) * scratch.values[441]);

        scratch.values[445] = ((scratch.values[390] * scratch.values[442]) * scratch.values[442]);

        scratch.values[446] = ((scratch.values[379] * scratch.values[422]) - ((2.0 * scratch.values[426]) * ((scratch.values[440]) as f64).ln()));

        scratch.values[447] = ((scratch.values[380] * scratch.values[422]) - ((2.0 * scratch.values[426]) * ((scratch.values[441]) as f64).ln()));

        scratch.values[448] = ((scratch.values[381] * scratch.values[422]) - ((2.0 * scratch.values[426]) * ((scratch.values[442]) as f64).ln()));

        scratch.values[449] = (scratch.values[446] + (scratch.values[426] * (((1.0 + ((((0.05 - scratch.values[446]) * scratch.values[427])) as f64).exp())) as f64).ln()));

        scratch.values[450] = (scratch.values[447] + (scratch.values[426] * (((1.0 + ((((0.05 - scratch.values[447]) * scratch.values[427])) as f64).exp())) as f64).ln()));

        scratch.values[451] = (scratch.values[448] + (scratch.values[426] * (((1.0 + ((((0.05 - scratch.values[448]) * scratch.values[427])) as f64).exp())) as f64).ln()));

        scratch.values[461] = (1.0 / scratch.values[449]);

        scratch.values[462] = (1.0 / scratch.values[450]);

        scratch.values[463] = (1.0 / scratch.values[451]);

        scratch.values[464] = (1.0 - scratch.values[382]);

        scratch.values[465] = (1.0 - scratch.values[383]);

        scratch.values[466] = (1.0 - scratch.values[384]);

        scratch.values[467] = (1.0 / scratch.values[464]);

        scratch.values[468] = (1.0 / scratch.values[465]);

        scratch.values[469] = (1.0 / scratch.values[466]);

        scratch.values[470] = (scratch.values[376] * (((scratch.values[379] * scratch.values[461])) as f64).powf(scratch.values[382]));

        scratch.values[471] = (scratch.values[377] * (((scratch.values[380] * scratch.values[462])) as f64).powf(scratch.values[383]));

        scratch.values[472] = (scratch.values[378] * (((scratch.values[381] * scratch.values[463])) as f64).powf(scratch.values[384]));

        scratch.values[473] = ((scratch.values[470] * scratch.values[449]) * scratch.values[467]);

        scratch.values[474] = ((scratch.values[471] * scratch.values[450]) * scratch.values[468]);

        scratch.values[475] = ((scratch.values[472] * scratch.values[451]) * scratch.values[469]);

        scratch.values[476] = (2.0 * scratch.values[470]);

        scratch.values[477] = (2.0 * scratch.values[471]);

        scratch.values[478] = (2.0 * scratch.values[472]);

        scratch.values[479] = (scratch.values[806] / scratch.values[376]);

        scratch.values[480] = ((scratch.values[391] * scratch.values[806]) / scratch.values[377]);

        scratch.values[481] = ((scratch.values[392] * scratch.values[806]) / scratch.values[378]);

        scratch.values[482] = (1.0 / scratch.values[479]);

        scratch.values[483] = (1.0 / scratch.values[480]);

        scratch.values[484] = (1.0 / scratch.values[481]);

        scratch.values[485] = (1.0 / scratch.values[379]);

        scratch.values[486] = (1.0 / scratch.values[380]);

        scratch.values[487] = (1.0 / scratch.values[381]);

        scratch.values[488] = ((0.5 * scratch.values[437])).max(scratch.values[426]);

        scratch.values[489] = ((0.5 * scratch.values[438])).max(scratch.values[426]);

        scratch.values[490] = ((0.5 * scratch.values[439])).max(scratch.values[426]);

        scratch.values[491] = (scratch.values[488] * scratch.values[427]);

        scratch.values[492] = (scratch.values[489] * scratch.values[427]);

        scratch.values[493] = (scratch.values[490] * scratch.values[427]);

        scratch.values[494] = (((((((32.0 * scratch.values[399]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[488] * scratch.values[488]) * scratch.values[488]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[495] = (((((((32.0 * scratch.values[400]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[489] * scratch.values[489]) * scratch.values[489]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[496] = (((((((32.0 * scratch.values[401]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[490] * scratch.values[490]) * scratch.values[490]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[497] = (scratch.values[405] * (1.0 + (scratch.values[408] * (scratch.values[421] - scratch.values[420]))));

        scratch.values[498] = (scratch.values[406] * (1.0 + (scratch.values[409] * (scratch.values[421] - scratch.values[420]))));

        scratch.values[499] = (scratch.values[407] * (1.0 + (scratch.values[410] * (scratch.values[421] - scratch.values[420]))));

        if !(scratch.values[497] > 0.0) {
            scratch.values[497] = 0.0;
            scratch.node_derivatives[497] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[497] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[498] > 0.0) {
            scratch.values[498] = 0.0;
            scratch.node_derivatives[498] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[498] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[499] > 0.0) {
            scratch.values[499] = 0.0;
            scratch.node_derivatives[499] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[499] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[500] = (1.0 - (1.0 / scratch.values[375]));

        scratch.values[504] = (1.0 / scratch.values[411]);

        scratch.values[505] = (1.0 / scratch.values[412]);

        scratch.values[506] = (1.0 / scratch.values[413]);

        scratch.store_ad(586, &AdValue::offset(scratch.ad_value(552), scratch.values[431]));

        scratch.store_ad(587, &AdValue::offset(scratch.ad_value(553), scratch.values[431]));

        scratch.store_ad(588, &AdValue::offset(scratch.ad_value(554), scratch.values[431]));

        scratch.store_ad(589, &AdValue::offset(scratch.ad_value(552), scratch.values[432]));

        scratch.store_ad(590, &AdValue::offset(scratch.ad_value(553), scratch.values[432]));

        scratch.store_ad(591, &AdValue::offset(scratch.ad_value(554), scratch.values[432]));

        scratch.store_ad(592, &AdValue::scale(AdValue::exp(AdValue::scale(AdValue::sub(AdValue::scale(scratch.ad_value(586), scratch.values[425]), AdValue::scale(scratch.ad_value(589), scratch.values[427])), 0.5)), ((scratch.values[422]) as f64).powf(1.5)));

        scratch.store_ad(593, &AdValue::scale(AdValue::exp(AdValue::scale(AdValue::sub(AdValue::scale(scratch.ad_value(587), scratch.values[425]), AdValue::scale(scratch.ad_value(590), scratch.values[427])), 0.5)), ((scratch.values[422]) as f64).powf(1.5)));

        scratch.store_ad(594, &AdValue::scale(AdValue::exp(AdValue::scale(AdValue::sub(AdValue::scale(scratch.ad_value(588), scratch.values[425]), AdValue::scale(scratch.ad_value(591), scratch.values[427])), 0.5)), ((scratch.values[422]) as f64).powf(1.5)));

        scratch.store_ad(595, &AdValue::mul(AdValue::mul(scratch.ad_value(555), scratch.ad_value(592)), scratch.ad_value(592)));

        scratch.store_ad(596, &AdValue::mul(AdValue::mul(scratch.ad_value(556), scratch.ad_value(593)), scratch.ad_value(593)));

        scratch.store_ad(597, &AdValue::mul(AdValue::mul(scratch.ad_value(557), scratch.ad_value(594)), scratch.ad_value(594)));

        scratch.store_ad(598, &AdValue::sub(AdValue::scale(scratch.ad_value(546), scratch.values[422]), AdValue::scale(AdValue::ln(scratch.ad_value(592)), (2.0 * scratch.values[426]))));

        scratch.store_ad(599, &AdValue::sub(AdValue::scale(scratch.ad_value(547), scratch.values[422]), AdValue::scale(AdValue::ln(scratch.ad_value(593)), (2.0 * scratch.values[426]))));

        scratch.store_ad(600, &AdValue::sub(AdValue::scale(scratch.ad_value(548), scratch.values[422]), AdValue::scale(AdValue::ln(scratch.ad_value(594)), (2.0 * scratch.values[426]))));

        scratch.store_ad(601, &AdValue::add(scratch.ad_value(598), AdValue::scale(AdValue::ln(AdValue::offset(AdValue::exp(AdValue::scale(AdValue::sub_from_scalar(0.05, scratch.ad_value(598)), scratch.values[427])), 1.0)), scratch.values[426])));

        scratch.store_ad(602, &AdValue::add(scratch.ad_value(599), AdValue::scale(AdValue::ln(AdValue::offset(AdValue::exp(AdValue::scale(AdValue::sub_from_scalar(0.05, scratch.ad_value(599)), scratch.values[427])), 1.0)), scratch.values[426])));

        scratch.store_ad(603, &AdValue::add(scratch.ad_value(600), AdValue::scale(AdValue::ln(AdValue::offset(AdValue::exp(AdValue::scale(AdValue::sub_from_scalar(0.05, scratch.ad_value(600)), scratch.values[427])), 1.0)), scratch.values[426])));

        scratch.store_ad(604, &AdValue::div_from_scalar(1.0, scratch.ad_value(601)));

        scratch.store_ad(605, &AdValue::div_from_scalar(1.0, scratch.ad_value(602)));

        scratch.store_ad(606, &AdValue::div_from_scalar(1.0, scratch.ad_value(603)));

        scratch.store_ad(607, &AdValue::sub_from_scalar(1.0, scratch.ad_value(549)));

        scratch.store_ad(608, &AdValue::sub_from_scalar(1.0, scratch.ad_value(550)));

        scratch.store_ad(609, &AdValue::sub_from_scalar(1.0, scratch.ad_value(551)));

        scratch.store_ad(610, &AdValue::div_from_scalar(1.0, scratch.ad_value(607)));

        scratch.store_ad(611, &AdValue::div_from_scalar(1.0, scratch.ad_value(608)));

        scratch.store_ad(612, &AdValue::div_from_scalar(1.0, scratch.ad_value(609)));

        scratch.store_ad(613, &AdValue::mul(scratch.ad_value(543), AdValue::pow(AdValue::mul(scratch.ad_value(546), scratch.ad_value(604)), scratch.ad_value(549))));

        scratch.store_ad(614, &AdValue::mul(scratch.ad_value(544), AdValue::pow(AdValue::mul(scratch.ad_value(547), scratch.ad_value(605)), scratch.ad_value(550))));

        scratch.store_ad(615, &AdValue::mul(scratch.ad_value(545), AdValue::pow(AdValue::mul(scratch.ad_value(548), scratch.ad_value(606)), scratch.ad_value(551))));

        scratch.store_ad(616, &AdValue::mul(AdValue::mul(scratch.ad_value(613), scratch.ad_value(601)), scratch.ad_value(610)));

        scratch.store_ad(617, &AdValue::mul(AdValue::mul(scratch.ad_value(614), scratch.ad_value(602)), scratch.ad_value(611)));

        scratch.store_ad(618, &AdValue::mul(AdValue::mul(scratch.ad_value(615), scratch.ad_value(603)), scratch.ad_value(612)));

        scratch.store_ad(619, &AdValue::scale(scratch.ad_value(613), 2.0));

        scratch.store_ad(620, &AdValue::scale(scratch.ad_value(614), 2.0));

        scratch.store_ad(621, &AdValue::scale(scratch.ad_value(615), 2.0));

        scratch.store_ad(622, &AdValue::div_from_scalar(scratch.values[806], scratch.ad_value(543)));

        scratch.store_ad(623, &AdValue::div(AdValue::scale(scratch.ad_value(558), scratch.values[806]), scratch.ad_value(544)));

        scratch.store_ad(624, &AdValue::div(AdValue::scale(scratch.ad_value(559), scratch.values[806]), scratch.ad_value(545)));

        scratch.store_ad(625, &AdValue::div_from_scalar(1.0, scratch.ad_value(622)));

        scratch.store_ad(626, &AdValue::div_from_scalar(1.0, scratch.ad_value(623)));

        scratch.store_ad(627, &AdValue::div_from_scalar(1.0, scratch.ad_value(624)));

        scratch.store_ad(628, &AdValue::div_from_scalar(1.0, scratch.ad_value(546)));

        scratch.store_ad(629, &AdValue::div_from_scalar(1.0, scratch.ad_value(547)));

        scratch.store_ad(630, &AdValue::div_from_scalar(1.0, scratch.ad_value(548)));

        scratch.store_ad(631, &AdValue::max_with_scalar(AdValue::scale(scratch.ad_value(589), 0.5), scratch.values[426]));

        scratch.store_ad(632, &AdValue::max_with_scalar(AdValue::scale(scratch.ad_value(590), 0.5), scratch.values[426]));

        scratch.store_ad(633, &AdValue::max_with_scalar(AdValue::scale(scratch.ad_value(591), 0.5), scratch.values[426]));

        scratch.store_ad(634, &AdValue::scale(scratch.ad_value(631), scratch.values[427]));

        scratch.store_ad(635, &AdValue::scale(scratch.ad_value(632), scratch.values[427]));

        scratch.store_ad(636, &AdValue::scale(scratch.ad_value(633), scratch.values[427]));

        scratch.store_ad(637, &AdValue::scale(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(566), (32.0 * (9.1093826e-31 * 1.6021918e-19))), AdValue::mul(AdValue::square(scratch.ad_value(631)), scratch.ad_value(631)))), 1.0 / ((3.0 * 1.05457168e-34))));

        scratch.store_ad(638, &AdValue::scale(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(567), (32.0 * (9.1093826e-31 * 1.6021918e-19))), AdValue::mul(AdValue::square(scratch.ad_value(632)), scratch.ad_value(632)))), 1.0 / ((3.0 * 1.05457168e-34))));

        scratch.store_ad(639, &AdValue::scale(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(568), (32.0 * (9.1093826e-31 * 1.6021918e-19))), AdValue::mul(AdValue::square(scratch.ad_value(633)), scratch.ad_value(633)))), 1.0 / ((3.0 * 1.05457168e-34))));

        scratch.store_ad(640, &AdValue::mul(scratch.ad_value(572), AdValue::offset(AdValue::scale(scratch.ad_value(575), (scratch.values[421] - scratch.values[420])), 1.0)));

        scratch.store_ad(641, &AdValue::mul(scratch.ad_value(573), AdValue::offset(AdValue::scale(scratch.ad_value(576), (scratch.values[421] - scratch.values[420])), 1.0)));

        scratch.store_ad(642, &AdValue::mul(scratch.ad_value(574), AdValue::offset(AdValue::scale(scratch.ad_value(577), (scratch.values[421] - scratch.values[420])), 1.0)));

        if !(scratch.values[640] > 0.0) {
            scratch.values[640] = 0.0;
            scratch.node_derivatives[640] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[640] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[641] > 0.0) {
            scratch.values[641] = 0.0;
            scratch.node_derivatives[641] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[641] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[642] > 0.0) {
            scratch.values[642] = 0.0;
            scratch.node_derivatives[642] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[642] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(646, &AdValue::div_from_scalar(1.0, scratch.ad_value(578)));

        scratch.store_ad(647, &AdValue::div_from_scalar(1.0, scratch.ad_value(579)));

        scratch.store_ad(648, &AdValue::div_from_scalar(1.0, scratch.ad_value(580)));

        scratch.values[11] = 1.0;

        scratch.values[12] = 1.0;

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[313] = 0.0;

        scratch.values[314] = 0.0;

        scratch.values[13] = self.params.l;

        scratch.values[14] = self.params.w;

        scratch.values[15] = self.params.sa;

        scratch.values[16] = self.params.sb;

        scratch.values[17] = self.params.sd;

        scratch.values[18] = self.params.sc;

        scratch.values[697] = self.params.absource;

        scratch.values[698] = self.params.lssource;

        scratch.values[699] = self.params.lgsource;

        scratch.values[724] = self.params.abdrain;

        scratch.values[725] = self.params.lsdrain;

        scratch.values[726] = self.params.lgdrain;

        scratch.values[700] = self.params.as_;

        scratch.values[701] = self.params.ps;

        scratch.values[727] = self.params.ad;

        scratch.values[728] = self.params.pd;

        scratch.values[20] = self.params.jw;

        scratch.values[1287] = if ((scratch.values[1] == 1.0) || (scratch.values[1] == 2.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1287] != 0.0) {
            scratch.values[11] = (if (self.params.nf > 1.0) { self.params.nf } else { 1.0 });
            scratch.node_derivatives[11] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[11] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1287] != 0.0) {
            scratch.store_ad(11, &AdValue::floor(AdValue::offset(scratch.ad_value(11), 0.5)));
        }

        if (scratch.values[1287] != 0.0) {
            scratch.store_ad(12, &AdValue::div_from_scalar(1.0, scratch.ad_value(11)));
        }

        if !(scratch.values[13] > 1e-9) {
            scratch.values[13] = 1e-9;
            scratch.node_derivatives[13] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[13] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[14] * scratch.values[12]) > 1e-9) {
            scratch.store_ad(14, &AdValue::scale(scratch.ad_value(12), scratch.values[14]));
        } else {
            scratch.values[14] = 1e-9;
            scratch.node_derivatives[14] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[14] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[21] = (if (self.params.sca > 0.0) { self.params.sca } else { 0.0 });

        scratch.values[22] = (if (self.params.scb > 0.0) { self.params.scb } else { 0.0 });

        scratch.values[23] = (if (self.params.scc > 0.0) { self.params.scc } else { 0.0 });

        scratch.store_ad(309, &AdValue::div_from_scalar(1e-6, scratch.ad_value(13)));

        scratch.store_ad(310, &AdValue::div_from_scalar(1e-6, scratch.ad_value(14)));

        scratch.store_ad(311, &AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(309), self.params.lvarl), 1.0), self.params.lvaro), AdValue::offset(AdValue::scale(scratch.ad_value(310), self.params.lvarw), 1.0)));

        scratch.store_ad(312, &AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(309), self.params.wvarl), 1.0), self.params.wvaro), AdValue::offset(AdValue::scale(scratch.ad_value(310), self.params.wvarw), 1.0)));

        scratch.values[1288] = if (scratch.values[1] == 2.0) { 1.0 } else { 0.0 };

        if (scratch.values[1288] != 0.0) {
            scratch.store_ad(311, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(309), self.params.lvarl), 1.0), self.params.lvaro));
        }

        if (scratch.values[1288] != 0.0) {
            scratch.store_ad(312, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(310), self.params.wvarw), 1.0), self.params.wvaro));
        }

        if (((scratch.values[13] + scratch.values[311]) - (2.0 * self.params.lap)) > 1e-9) {
            scratch.store_ad(313, &AdValue::offset(AdValue::add(scratch.ad_value(13), scratch.ad_value(311)), (-(2.0 * self.params.lap))));
        } else {
            scratch.values[313] = 1e-9;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[14] + scratch.values[312]) - (2.0 * self.params.wot)) > 1e-9) {
            scratch.store_ad(314, &AdValue::offset(AdValue::add(scratch.ad_value(14), scratch.ad_value(312)), (-(2.0 * self.params.wot))));
        } else {
            scratch.values[314] = 1e-9;
            scratch.node_derivatives[314] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[314] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[13] + scratch.values[311]) - (2.0 * self.params.lap)) + self.params.dlq) > 1e-9) {
            scratch.store_ad(315, &AdValue::offset(AdValue::offset(AdValue::add(scratch.ad_value(13), scratch.ad_value(311)), (-(2.0 * self.params.lap))), self.params.dlq));
        } else {
            scratch.values[315] = 1e-9;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[14] + scratch.values[312]) - (2.0 * self.params.wot)) + self.params.dwq) > 1e-9) {
            scratch.store_ad(316, &AdValue::offset(AdValue::offset(AdValue::add(scratch.ad_value(14), scratch.ad_value(312)), (-(2.0 * self.params.wot))), self.params.dwq));
        } else {
            scratch.values[316] = 1e-9;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[13] + scratch.values[311]) + self.params.dlq) > 1e-9) {
            scratch.store_ad(317, &AdValue::offset(AdValue::add(scratch.ad_value(13), scratch.ad_value(311)), self.params.dlq));
        } else {
            scratch.values[317] = 1e-9;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[14] + scratch.values[312]) + self.params.dwq) > 1e-9) {
            scratch.store_ad(318, &AdValue::offset(AdValue::add(scratch.ad_value(14), scratch.ad_value(312)), self.params.dwq));
        } else {
            scratch.values[318] = 1e-9;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(319, &AdValue::div_from_scalar(1e-6, scratch.ad_value(313)));

        scratch.store_ad(320, &AdValue::div_from_scalar(1e-6, scratch.ad_value(314)));

        scratch.values[73] = self.params.vfb;

        scratch.values[74] = self.params.stvfb;

        scratch.values[75] = self.params.st2vfb;

        scratch.values[76] = self.params.tox;

        scratch.values[77] = self.params.epsrox;

        scratch.values[78] = self.params.neff;

        scratch.values[79] = self.params.facneffac;

        scratch.values[80] = self.params.gfacnud;

        scratch.values[81] = self.params.vsbnud;

        scratch.values[82] = self.params.dvsbnud;

        scratch.values[83] = self.params.vnsub;

        scratch.values[84] = self.params.nslp;

        scratch.values[85] = self.params.dnsub;

        scratch.values[86] = self.params.dphib;

        scratch.values[87] = self.params.delvtac;

        scratch.values[88] = self.params.np;

        scratch.values[93] = self.params.toxov;

        scratch.values[94] = self.params.toxovd;

        scratch.values[95] = self.params.nov;

        scratch.values[96] = self.params.novd;

        scratch.values[89] = self.params.ct;

        scratch.values[90] = self.params.ctg;

        scratch.values[91] = self.params.ctb;

        scratch.values[92] = self.params.stct;

        scratch.values[97] = self.params.psce;

        scratch.values[98] = self.params.psced;

        scratch.values[99] = self.params.psceb;

        scratch.values[100] = self.params.cf;

        scratch.values[101] = self.params.cfd;

        scratch.values[102] = self.params.cfb;

        scratch.values[103] = self.params.betn;

        scratch.values[104] = self.params.stbet;

        scratch.values[105] = self.params.mue;

        scratch.values[106] = self.params.stmue;

        scratch.values[107] = self.params.themu;

        scratch.values[108] = self.params.stthemu;

        scratch.values[109] = self.params.cs;

        scratch.values[110] = self.params.stcs;

        scratch.values[111] = self.params.thecs;

        scratch.values[112] = self.params.stthecs;

        scratch.values[113] = self.params.xcor;

        scratch.values[114] = self.params.stxcor;

        scratch.values[115] = self.params.feta;

        scratch.values[116] = self.params.rs;

        scratch.values[117] = self.params.strs;

        scratch.values[118] = self.params.rsb;

        scratch.values[119] = self.params.rsg;

        scratch.values[120] = self.params.thesat;

        scratch.values[121] = self.params.stthesat;

        scratch.values[122] = self.params.thesatb;

        scratch.values[123] = self.params.thesatg;

        scratch.values[124] = self.params.ax;

        scratch.values[125] = self.params.alp;

        scratch.values[128] = self.params.vp;

        scratch.values[130] = self.params.a2;

        scratch.values[131] = self.params.sta2;

        scratch.values[132] = self.params.a3;

        scratch.values[133] = self.params.a4;

        scratch.values[134] = self.params.gco;

        scratch.values[135] = self.params.iginv;

        scratch.values[136] = self.params.igov;

        scratch.values[137] = self.params.igovd;

        scratch.values[138] = self.params.stig;

        scratch.values[139] = self.params.gc2;

        scratch.values[140] = self.params.gc3;

        scratch.values[141] = self.params.chib;

        scratch.values[142] = self.params.agidl;

        scratch.values[143] = self.params.agidld;

        scratch.values[144] = self.params.bgidl;

        scratch.values[145] = self.params.bgidld;

        scratch.values[146] = self.params.stbgidl;

        scratch.values[147] = self.params.stbgidld;

        scratch.values[148] = self.params.cgidl;

        scratch.values[149] = self.params.cgidld;

        scratch.values[150] = self.params.cox;

        scratch.values[151] = self.params.cgov;

        scratch.values[152] = self.params.cgovd;

        scratch.values[153] = self.params.cgbov;

        scratch.values[154] = self.params.cfr;

        scratch.values[155] = self.params.cfrd;

        scratch.values[156] = self.params.fnt;

        scratch.values[162] = self.params.vfbedge;

        scratch.values[163] = self.params.stvfbedge;

        scratch.values[164] = self.params.dphibedge;

        scratch.values[165] = self.params.neffedge;

        scratch.values[166] = self.params.ctedge;

        scratch.values[167] = self.params.betnedge;

        scratch.values[169] = self.params.psceedge;

        scratch.values[170] = self.params.pscebedge;

        scratch.values[171] = self.params.pscededge;

        scratch.values[172] = self.params.cfedge;

        scratch.values[173] = self.params.cfdedge;

        scratch.values[174] = self.params.cfbedge;

        scratch.values[188] = self.params.cth;

        scratch.values[1289] = if (scratch.values[1] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(73, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.vfbl), self.params.vfbo), AdValue::scale(scratch.ad_value(320), self.params.vfbw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.vfblw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(74, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.stvfbl), self.params.stvfbo), AdValue::scale(scratch.ad_value(320), self.params.stvfbw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.stvfblw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[75] = self.params.st2vfbo;
            scratch.node_derivatives[75] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[75] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[76] = self.params.toxo;
            scratch.node_derivatives[76] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[76] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[77] = self.params.epsroxo;
            scratch.node_derivatives[77] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[77] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(325, &AdValue::scale({
                if ((1.0 + ((self.params.nsubw * scratch.values[320]) * (((1.0 + (scratch.values[314] / scratch.values[41]))) as f64).ln())) > 0.001) {
                    AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(320), self.params.nsubw), AdValue::ln(AdValue::offset(AdValue::scale(scratch.ad_value(314), 1.0 / (scratch.values[41])), 1.0))), 1.0)
                } else {
                    AdValue::constant(0.001)
                }
            }, scratch.values[40]));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(326, &AdValue::scale({
                if ((1.0 + ((self.params.npckw * scratch.values[320]) * (((1.0 + (scratch.values[314] / scratch.values[43]))) as f64).ln())) > 0.001) {
                    AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(320), self.params.npckw), AdValue::ln(AdValue::offset(AdValue::scale(scratch.ad_value(314), 1.0 / (scratch.values[43])), 1.0))), 1.0)
                } else {
                    AdValue::constant(0.001)
                }
            }, scratch.values[42]));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(327, &AdValue::scale({
                if ((1.0 + ((self.params.lpckw * scratch.values[320]) * (((1.0 + (scratch.values[314] / scratch.values[43]))) as f64).ln())) > 0.001) {
                    AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(320), self.params.lpckw), AdValue::ln(AdValue::offset(AdValue::scale(scratch.ad_value(314), 1.0 / (scratch.values[43])), 1.0))), 1.0)
                } else {
                    AdValue::constant(0.001)
                }
            }, scratch.values[44]));
        }

        scratch.values[1290] = if (scratch.values[313] > (2.0 * scratch.values[327])) { 1.0 } else { 0.0 };

        if ((scratch.values[1289] != 0.0) && (scratch.values[1290] != 0.0)) {
            scratch.values[328] = 75000000000.0;
            scratch.node_derivatives[328] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[328] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1289] != 0.0) && (scratch.values[1290] != 0.0)) {
            scratch.store_ad(329, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(325), AdValue::scale(scratch.ad_value(326), 0.5))), AdValue::sqrt(scratch.ad_value(325))));
        }

        if ((scratch.values[1289] != 0.0) && (scratch.values[1290] != 0.0)) {
            scratch.store_ad(330, &AdValue::add(AdValue::sqrt(scratch.ad_value(325)), AdValue::mul(scratch.ad_value(328), AdValue::ln(AdValue::offset(AdValue::mul(AdValue::div(AdValue::scale(scratch.ad_value(327), 2.0), scratch.ad_value(313)), AdValue::offset(AdValue::exp(AdValue::div(scratch.ad_value(329), scratch.ad_value(328))), (-1.0))), 1.0)))));
        }

        if ((scratch.values[1289] != 0.0) && (scratch.values[1290] != 0.0)) {
            scratch.store_ad(330, &AdValue::square(scratch.ad_value(330)));
        }

        scratch.values[1291] = if (scratch.values[313] >= scratch.values[327]) { 1.0 } else { 0.0 };

        if (((scratch.values[1289] != 0.0) && (!(scratch.values[1290] != 0.0))) && (scratch.values[1291] != 0.0)) {
            scratch.store_ad(330, &AdValue::add(scratch.ad_value(325), AdValue::div(AdValue::mul(scratch.ad_value(326), scratch.ad_value(327)), scratch.ad_value(313))));
        }

        if (((scratch.values[1289] != 0.0) && (!(scratch.values[1290] != 0.0))) && (!(scratch.values[1291] != 0.0))) {
            scratch.store_ad(330, &AdValue::add(scratch.ad_value(325), AdValue::mul(scratch.ad_value(326), AdValue::sub_from_scalar(2.0, AdValue::div(scratch.ad_value(313), scratch.ad_value(327))))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(78, &AdValue::mul(scratch.ad_value(330), AdValue::sub(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(319), self.params.fol1)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.fol2), scratch.ad_value(319)))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(79, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.facneffacl), self.params.facneffaco), AdValue::scale(scratch.ad_value(320), self.params.facneffacw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.facneffaclw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(80, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.gfacnudlexp), self.params.gfacnudl), self.params.gfacnudo), AdValue::scale(scratch.ad_value(320), self.params.gfacnudw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.gfacnudlw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[81] = self.params.vsbnudo;
            scratch.node_derivatives[81] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[81] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[82] = self.params.dvsbnudo;
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[83] = self.params.vnsubo;
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[84] = self.params.nslpo;
            scratch.node_derivatives[84] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[84] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[85] = self.params.dnsubo;
            scratch.node_derivatives[85] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[85] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(86, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.dphiblexp), self.params.dphibl), self.params.dphibo), AdValue::scale(scratch.ad_value(320), self.params.dphibw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.dphiblw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(87, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.delvtaclexp), self.params.delvtacl), self.params.delvtaco), AdValue::scale(scratch.ad_value(320), self.params.delvtacw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.delvtaclw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(88, &AdValue::scale({
                if (1e-6 > (1.0 + (self.params.npl * scratch.values[319]))) {
                    AdValue::constant(1e-6)
                } else {
                    AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.npl), 1.0)
                }
            }, self.params.npo));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[93] = self.params.toxovo;
            scratch.node_derivatives[93] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[93] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[94] = self.params.toxovdo;
            scratch.node_derivatives[94] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[94] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[95] = self.params.novo;
            scratch.node_derivatives[95] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[95] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[96] = self.params.novdo;
            scratch.node_derivatives[96] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[96] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(89, &AdValue::mul(AdValue::mul(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.ctlexp), self.params.ctl), self.params.cto), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.ctw), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.ctlw), scratch.ad_value(320)), 1.0)));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[1289] != 0.0) {
            scratch.values[90] = self.params.ctgo;
            scratch.node_derivatives[90] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[90] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[91] = self.params.ctbo;
            scratch.node_derivatives[91] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[91] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[92] = self.params.stcto;
            scratch.node_derivatives[92] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[92] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(100, &AdValue::mul(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.cflexp), self.params.cfl), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.cfw), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[101] = self.params.cfdo;
            scratch.node_derivatives[101] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[101] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[102] = self.params.cfbo;
            scratch.node_derivatives[102] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[102] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(97, &AdValue::mul(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.pscelexp), self.params.pscel), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.pscew), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[98] = self.params.pscedo;
            scratch.node_derivatives[98] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[98] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[99] = self.params.pscebo;
            scratch.node_derivatives[99] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[99] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(331, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.fbet1w), 1.0), self.params.fbet1));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(332, &AdValue::scale({
                if ((1.0 + (self.params.lp1w * scratch.values[320])) > 0.001) {
                    AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.lp1w), 1.0)
                } else {
                    AdValue::constant(0.001)
                }
            }, scratch.values[49]));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(333, &AdValue::add(AdValue::offset(AdValue::mul(AdValue::div(AdValue::mul(scratch.ad_value(331), scratch.ad_value(332)), scratch.ad_value(313)), AdValue::sub_from_scalar(1.0, AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(313)), scratch.ad_value(332))))), 1.0), AdValue::mul(AdValue::div_from_scalar((self.params.fbet2 * scratch.values[50]), scratch.ad_value(313)), AdValue::sub_from_scalar(1.0, AdValue::exp(AdValue::scale(AdValue::neg(scratch.ad_value(313)), 1.0 / (scratch.values[50])))))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(333, &{
                if (scratch.values[333] > 1e-15) {
                    scratch.ad_value(333)
                } else {
                    AdValue::constant(1e-15)
                }
            });
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(334, &AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.betw1), 1.0), AdValue::mul(AdValue::scale(scratch.ad_value(320), self.params.betw2), AdValue::ln(AdValue::offset(AdValue::scale(scratch.ad_value(314), 1.0 / (scratch.values[51])), 1.0)))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(103, &AdValue::mul(AdValue::div(AdValue::scale(scratch.ad_value(314), self.params.uo), AdValue::mul(scratch.ad_value(333), scratch.ad_value(313))), scratch.ad_value(334)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(104, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.stbetl), self.params.stbeto), AdValue::scale(scratch.ad_value(320), self.params.stbetw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.stbetlw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(105, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.muew), 1.0), self.params.mueo));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[106] = self.params.stmueo;
            scratch.node_derivatives[106] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[106] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[107] = self.params.themuo;
            scratch.node_derivatives[107] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[107] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[108] = self.params.stthemuo;
            scratch.node_derivatives[108] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[108] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(109, &AdValue::mul(AdValue::mul(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.cslexp), self.params.csl), self.params.cso), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.csw), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.cslw), scratch.ad_value(320)), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[110] = self.params.stcso;
            scratch.node_derivatives[110] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[110] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[111] = self.params.thecso;
            scratch.node_derivatives[111] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[111] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[112] = self.params.stthecso;
            scratch.node_derivatives[112] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[112] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(113, &AdValue::mul(AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.xcorl), 1.0), self.params.xcoro), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.xcorw), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.xcorlw), scratch.ad_value(320)), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[114] = self.params.stxcoro;
            scratch.node_derivatives[114] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[114] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[115] = self.params.fetao;
            scratch.node_derivatives[115] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[115] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(116, &AdValue::mul(AdValue::scale(scratch.ad_value(320), self.params.rsw1), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.rsw2), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[117] = self.params.strso;
            scratch.node_derivatives[117] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[117] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[118] = self.params.rsbo;
            scratch.node_derivatives[118] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[118] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[119] = self.params.rsgo;
            scratch.node_derivatives[119] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[119] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(120, &AdValue::mul(AdValue::mul(AdValue::offset(AdValue::mul(AdValue::div(AdValue::scale(scratch.ad_value(334), self.params.thesatl), scratch.ad_value(333)), AdValue::powf(scratch.ad_value(319), self.params.thesatlexp)), self.params.thesato), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.thesatw), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.thesatlw), scratch.ad_value(320)), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(121, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.stthesatl), self.params.stthesato), AdValue::scale(scratch.ad_value(320), self.params.stthesatw)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.stthesatlw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[122] = self.params.thesatbo;
            scratch.node_derivatives[122] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[122] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[123] = self.params.thesatgo;
            scratch.node_derivatives[123] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[123] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(124, &AdValue::div_from_scalar(self.params.axo, AdValue::offset(AdValue::scale(scratch.ad_value(319), scratch.values[52]), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(125, &AdValue::mul(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.alplexp), self.params.alpl), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.alpw), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[128] = self.params.vpo;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[130] = self.params.a2o;
            scratch.node_derivatives[130] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[130] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[131] = self.params.sta2o;
            scratch.node_derivatives[131] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[131] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(132, &AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.a3l), 1.0), self.params.a3o), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.a3w), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(133, &AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.a4l), 1.0), self.params.a4o), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.a4w), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[134] = self.params.gcoo;
            scratch.node_derivatives[134] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[134] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(135, &AdValue::div_from_scalar(self.params.iginvlw, AdValue::mul(scratch.ad_value(320), scratch.ad_value(319))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(136, &AdValue::div_from_scalar((self.params.igovw * scratch.values[47]), AdValue::scale(scratch.ad_value(320), 1e-6)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(137, &AdValue::div_from_scalar((self.params.igovdw * scratch.values[48]), AdValue::scale(scratch.ad_value(320), 1e-6)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[138] = self.params.stigo;
            scratch.node_derivatives[138] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[138] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[139] = self.params.gc2o;
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[140] = self.params.gc3o;
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[141] = self.params.chibo;
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(142, &AdValue::div_from_scalar((self.params.agidlw * scratch.values[47]), AdValue::scale(scratch.ad_value(320), 1e-6)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(143, &AdValue::div_from_scalar((self.params.agidldw * scratch.values[48]), AdValue::scale(scratch.ad_value(320), 1e-6)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[144] = self.params.bgidlo;
            scratch.node_derivatives[144] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[144] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[145] = self.params.bgidldo;
            scratch.node_derivatives[145] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[145] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[146] = self.params.stbgidlo;
            scratch.node_derivatives[146] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[146] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[147] = self.params.stbgidldo;
            scratch.node_derivatives[147] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[147] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[148] = self.params.cgidlo;
            scratch.node_derivatives[148] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[148] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[149] = self.params.cgidldo;
            scratch.node_derivatives[149] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[149] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(150, &AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(316), (8.8541878176e-12 * scratch.values[39])), scratch.ad_value(315)), 1.0 / (scratch.values[38])));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(151, &AdValue::scale(scratch.ad_value(316), ((8.8541878176e-12 * scratch.values[39]) * (scratch.values[47] * 1.0 / (scratch.values[45])))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(152, &AdValue::scale(scratch.ad_value(316), ((8.8541878176e-12 * scratch.values[39]) * (scratch.values[48] * 1.0 / (scratch.values[46])))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(153, &AdValue::scale(scratch.ad_value(317), (self.params.cgbovl * 1000000.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(154, &AdValue::scale(scratch.ad_value(318), (self.params.cfrw * 1000000.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(155, &AdValue::scale(scratch.ad_value(318), (self.params.cfrdw * 1000000.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(1274, &AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(319), ((2.0 * self.params.lintnoi) * 1000000.0))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[156] = self.params.fnto;
            scratch.node_derivatives[156] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[156] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(338, &AdValue::offset(AdValue::scale(scratch.ad_value(314), self.params.wedgew), (2.0 * self.params.wedge)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[162] = self.params.vfbedgeo;
            scratch.node_derivatives[162] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[162] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(163, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.stvfbedgel), self.params.stvfbedgeo), AdValue::scale(scratch.ad_value(320), self.params.stvfbedgew)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.stvfbedgelw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(164, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.dphibedgelexp), self.params.dphibedgel), self.params.dphibedgeo), AdValue::scale(scratch.ad_value(320), self.params.dphibedgew)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.dphibedgelw), scratch.ad_value(320))));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(165, &AdValue::mul(AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.nsubedgelexp), self.params.nsubedgel), 1.0), scratch.values[71]), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.nsubedgew), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.nsubedgelw), scratch.ad_value(320)), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(166, &AdValue::offset(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.ctedgelexp), self.params.ctedgel), self.params.ctedgeo));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(340, &AdValue::offset(AdValue::mul(AdValue::div_from_scalar((self.params.fbetedge * scratch.values[72]), scratch.ad_value(313)), AdValue::sub_from_scalar(1.0, AdValue::exp(AdValue::scale(AdValue::neg(scratch.ad_value(313)), 1.0 / (scratch.values[72]))))), 1.0));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(340, &{
                if (scratch.values[340] > 1e-15) {
                    scratch.ad_value(340)
                } else {
                    AdValue::constant(1e-15)
                }
            });
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(167, &AdValue::mul(AdValue::div(AdValue::scale(scratch.ad_value(338), self.params.uo), AdValue::mul(scratch.ad_value(340), scratch.ad_value(313))), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.betedgew), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(169, &AdValue::mul(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.psceedgelexp), self.params.psceedgel), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.psceedgew), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[170] = self.params.pscebedgeo;
            scratch.node_derivatives[170] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[170] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[171] = self.params.pscededgeo;
            scratch.node_derivatives[171] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[171] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(172, &AdValue::mul(AdValue::scale(AdValue::powf(scratch.ad_value(319), self.params.cfedgelexp), self.params.cfedgel), AdValue::offset(AdValue::scale(scratch.ad_value(320), self.params.cfedgew), 1.0)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[173] = self.params.cfdedgeo;
            scratch.node_derivatives[173] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[173] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[174] = self.params.cfbedgeo;
            scratch.node_derivatives[174] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[174] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(341, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.kvthowel), self.params.kvthoweo), AdValue::scale(scratch.ad_value(320), self.params.kvthowew)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.kvthowelw), scratch.ad_value(320))));

        scratch.store_ad(342, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.kuowel), self.params.kuoweo), AdValue::scale(scratch.ad_value(320), self.params.kuowew)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.kuowelw), scratch.ad_value(320))));

        scratch.values[1292] = if (scratch.values[1] == 2.0) { 1.0 } else { 0.0 };

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(344, &AdValue::mul(scratch.ad_value(319), scratch.ad_value(320)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(345, &AdValue::scale(scratch.ad_value(313), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(346, &AdValue::scale(scratch.ad_value(314), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(347, &AdValue::mul(scratch.ad_value(345), scratch.ad_value(346)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(348, &AdValue::div(scratch.ad_value(346), scratch.ad_value(345)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(350, &AdValue::scale(scratch.ad_value(315), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(351, &AdValue::scale(scratch.ad_value(316), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(352, &AdValue::mul(scratch.ad_value(350), scratch.ad_value(351)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(349, &AdValue::div_from_scalar(1e-6, scratch.ad_value(315)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(353, &AdValue::div(scratch.ad_value(351), scratch.ad_value(350)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(355, &AdValue::scale(scratch.ad_value(317), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(356, &AdValue::scale(scratch.ad_value(318), 1000000.0));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(357, &AdValue::mul(scratch.ad_value(355), scratch.ad_value(356)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(354, &AdValue::div_from_scalar(1e-6, scratch.ad_value(317)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(358, &AdValue::div(scratch.ad_value(356), scratch.ad_value(355)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(73, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plvfb), self.params.povfb), AdValue::scale(scratch.ad_value(320), self.params.pwvfb)), AdValue::scale(scratch.ad_value(344), self.params.plwvfb)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(74, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plstvfb), self.params.postvfb), AdValue::scale(scratch.ad_value(320), self.params.pwstvfb)), AdValue::scale(scratch.ad_value(344), self.params.plwstvfb)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[75] = self.params.post2vfb;
            scratch.node_derivatives[75] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[75] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[76] = self.params.potox;
            scratch.node_derivatives[76] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[76] = [0.0; Instance::BRANCH_COUNT];
        }

    }
}
