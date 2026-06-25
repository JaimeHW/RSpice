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
        scratch.values[12] = self.multiplicity;

        scratch.values[11] = 0.0;

        scratch.values[13] = (((1.0 - (0.01 * self.params.shrink)) * self.params.scale) * 1000000.0);

        scratch.values[14] = (scratch.values[13] * scratch.values[13]);

        scratch.values[15] = (273.15 + self.params.tnom);

        scratch.values[23] = ((ctx.temperature() + self.params.trise) - 273.15);

        scratch.values[114] = if (scratch.values[23] < (self.params.tminclip + 1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[114] != 0.0) {
            scratch.values[23] = (self.params.tminclip + ((((scratch.values[23] - self.params.tminclip) - 1.0)) as f64).exp());
            scratch.node_derivatives[23] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[23] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[115] = if (scratch.values[23] > (self.params.tmaxclip - 1.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[114] != 0.0)) && (scratch.values[115] != 0.0)) {
            scratch.store_ad(23, &AdValue::sub_from_scalar(self.params.tmaxclip, AdValue::exp(AdValue::offset(AdValue::sub_from_scalar(self.params.tmaxclip, scratch.ad_value(23)), (-1.0)))));
        }

        if ((!(scratch.values[114] != 0.0)) && (!(scratch.values[115] != 0.0))) {
        }

        scratch.store_ad(24, &AdValue::offset(scratch.ad_value(23), 273.15));

        scratch.store_ad(71, &AdValue::scale(scratch.ad_value(24), (1.3806505e-23 * 6.241509479607718e18)));

        scratch.store_ad(68, &AdValue::scale(scratch.ad_value(24), 1.0 / (scratch.values[15])));

        scratch.store_ad(69, &AdValue::offset(scratch.ad_value(24), (-scratch.values[15])));

        scratch.values[26] = (self.params.w * scratch.values[13]);

        scratch.values[27] = (self.params.l * scratch.values[13]);

        scratch.values[30] = (self.params.wd * scratch.values[13]);

        scratch.values[31] = (self.params.a1 * scratch.values[14]);

        scratch.values[32] = (self.params.p1 * scratch.values[13]);

        scratch.values[33] = (self.params.a2 * scratch.values[14]);

        scratch.values[34] = (self.params.p2 * scratch.values[13]);

        scratch.values[35] = (scratch.values[27] * scratch.values[26]);

        scratch.values[36] = ((2.0 * scratch.values[27]) + ((if (self.params.c1 > 0.0) { 1.0 } else { 0.0 } + if (self.params.c2 > 0.0) { 1.0 } else { 0.0 }) * scratch.values[26]));

        scratch.values[25] = ((0.5 * (if (self.params.c1 > 0.0) { 1.0 } else { 0.0 } + if (self.params.c2 > 0.0) { 1.0 } else { 0.0 })) * (self.params.xl + (self.params.xlw / scratch.values[26])));

        scratch.values[4] = ((((scratch.values[26] + self.params.xw) + (self.params.nwxw / scratch.values[26])) + (self.params.fdxwinf * (1.0 - ((((-scratch.values[26]) / self.params.fdrw)) as f64).exp()))) / (1.0 - ((self.params.wexw * scratch.values[30]) / scratch.values[35])));

        scratch.values[3] = (scratch.values[27] + scratch.values[25]);

        if (self.params.sw_mmgeo != 0.0) {
            scratch.values[38] = scratch.values[4];
            scratch.node_derivatives[38] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[38] = [0.0; Instance::BRANCH_COUNT];
        }

        if (self.params.sw_mmgeo != 0.0) {
            scratch.values[37] = scratch.values[3];
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(self.params.sw_mmgeo != 0.0)) {
            scratch.values[38] = scratch.values[26];
            scratch.node_derivatives[38] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[38] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(self.params.sw_mmgeo != 0.0)) {
            scratch.values[37] = scratch.values[27];
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        if (self.params.sw_mman != 0.0) {
            scratch.store_ad(4, &AdValue::offset(AdValue::div_from_scalar((self.params.nsmm_w * self.params.smm_w), AdValue::sqrt(AdValue::scale(scratch.ad_value(37), scratch.values[12]))), (scratch.values[4] + (self.params.nsig_w * self.params.sig_w))));
        }

        if (self.params.sw_mman != 0.0) {
            scratch.store_ad(3, &AdValue::offset(AdValue::div_from_scalar((self.params.nsmm_l * self.params.smm_l), AdValue::sqrt(AdValue::scale(scratch.ad_value(38), scratch.values[12]))), (scratch.values[3] + (self.params.nsig_l * self.params.sig_l))));
        }

        if (self.params.sw_mman != 0.0) {
            scratch.store_ad(40, &AdValue::exp(AdValue::scale(AdValue::offset(AdValue::div_from_scalar((self.params.nsmm_rsh * self.params.smm_rsh), AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(37), scratch.values[12]), scratch.ad_value(38)))), (self.params.nsig_rsh * self.params.sig_rsh)), 0.01)));
        }

        scratch.values[120] = if ((self.params.nsig_w != 0.0) && ((self.params.smm_w > 0.0) || (self.params.sig_w > 0.0))) { 1.0 } else { 0.0 };

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[120] != 0.0)) {
            scratch.store_ad(39, &AdValue::div_from_scalar(self.params.smm_w, AdValue::sqrt(AdValue::scale(scratch.ad_value(37), scratch.values[12]))));
        }

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[120] != 0.0)) {
            scratch.store_ad(4, &AdValue::add(scratch.ad_value(4), AdValue::scale(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(39)), (self.params.sig_w * self.params.sig_w))), self.params.nsig_w)));
        }

        scratch.values[121] = if ((self.params.nsig_l != 0.0) && ((self.params.smm_l > 0.0) || (self.params.sig_l > 0.0))) { 1.0 } else { 0.0 };

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[121] != 0.0)) {
            scratch.store_ad(39, &AdValue::div_from_scalar(self.params.smm_l, AdValue::sqrt(AdValue::scale(scratch.ad_value(38), scratch.values[12]))));
        }

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[121] != 0.0)) {
            scratch.store_ad(3, &AdValue::add(scratch.ad_value(3), AdValue::scale(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(39)), (self.params.sig_l * self.params.sig_l))), self.params.nsig_l)));
        }

        scratch.values[122] = if ((self.params.nsig_rsh != 0.0) && ((self.params.smm_rsh > 0.0) || (self.params.sig_rsh > 0.0))) { 1.0 } else { 0.0 };

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[122] != 0.0)) {
            scratch.store_ad(39, &AdValue::div_from_scalar(self.params.smm_rsh, AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(37), scratch.values[12]), scratch.ad_value(38)))));
        }

        if ((!(self.params.sw_mman != 0.0)) && (scratch.values[122] != 0.0)) {
            scratch.store_ad(40, &AdValue::exp(AdValue::scale(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(39)), (self.params.sig_rsh * self.params.sig_rsh))), (0.01 * self.params.nsig_rsh))));
        }

        if ((!(self.params.sw_mman != 0.0)) && (!(scratch.values[122] != 0.0))) {
            scratch.values[40] = 1.0;
            scratch.node_derivatives[40] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[40] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(28, &AdValue::offset(scratch.ad_value(3), self.params.dxlsat));

        if (self.params.sw_dfgeo != 0.0) {
            scratch.values[38] = scratch.values[4];
            scratch.node_derivatives[38] = scratch.node_derivatives[4];
            scratch.branch_derivatives[38] = scratch.branch_derivatives[4];
        }

        if (self.params.sw_dfgeo != 0.0) {
            scratch.values[37] = scratch.values[3];
            scratch.node_derivatives[37] = scratch.node_derivatives[3];
            scratch.branch_derivatives[37] = scratch.branch_derivatives[3];
        }

        if (!(self.params.sw_dfgeo != 0.0)) {
            scratch.values[38] = scratch.values[26];
            scratch.node_derivatives[38] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[38] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(self.params.sw_dfgeo != 0.0)) {
            scratch.values[37] = scratch.values[27];
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(42, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(38), self.params.dpwe)));

        scratch.store_ad(43, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(37), self.params.dple)));

        scratch.store_ad(41, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(42), self.params.dpw), 1.0), self.params.dp), AdValue::offset(AdValue::scale(scratch.ad_value(43), self.params.dpl), 1.0)), AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(42), self.params.dpwl), scratch.ad_value(43)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(69), AdValue::offset(AdValue::scale(scratch.ad_value(69), self.params.tc2dp), self.params.tc1dp)), 1.0)));

        if !(scratch.values[41] > 0.1) {
            scratch.values[41] = 0.1;
            scratch.node_derivatives[41] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[41] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(44, &AdValue::div(AdValue::sqrt(scratch.ad_value(41)), AdValue::offset(scratch.ad_value(41), 10000.0)));

        if (self.params.sw_lin != 0.0) {
            scratch.values[45] = 0.0;
            scratch.node_derivatives[45] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[45] = [0.0; Instance::BRANCH_COUNT];
        } else {
            scratch.store_ad(45, &AdValue::offset(AdValue::div(AdValue::offset(AdValue::add(AdValue::scale(scratch.ad_value(37), self.params.dfw), AdValue::scale(scratch.ad_value(38), self.params.dfl)), self.params.dfwl), AdValue::mul(scratch.ad_value(37), scratch.ad_value(38))), self.params.dfinf));
        }

        scratch.values[126] = if (scratch.values[45] < scratch.values[44]) { 1.0 } else { 0.0 };

        if (scratch.values[126] != 0.0) {
            scratch.store_ad(45, &{
                if (scratch.values[45] > 0.0) {
                    scratch.ad_value(45)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[126] != 0.0) {
            scratch.store_ad(46, &AdValue::square(scratch.ad_value(44)));
        }

        if (!(scratch.values[126] != 0.0)) {
            scratch.store_ad(46, &AdValue::square(scratch.ad_value(45)));
        }

        scratch.store_ad(48, &AdValue::sub(AdValue::div_from_scalar(0.5, scratch.ad_value(46)), AdValue::scale(scratch.ad_value(41), 0.5)));

        scratch.values[127] = if (self.params.sw_accpo > 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[127] != 0.0) {
            scratch.store_ad(49, &AdValue::sub(scratch.ad_value(48), AdValue::div_from_scalar((2.0 * self.params.grpo), scratch.ad_value(46))));
        }

        if (scratch.values[127] != 0.0) {
            scratch.store_ad(50, &AdValue::sub(AdValue::div_from_scalar(0.1666666666666667, scratch.ad_value(46)), AdValue::scale(scratch.ad_value(41), 0.5)));
        }

        scratch.values[128] = if (self.params.sw_accpo > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[127] != 0.0)) && (scratch.values[128] != 0.0)) {
            scratch.store_ad(49, &AdValue::sub(scratch.ad_value(48), AdValue::sqrt(AdValue::div_from_scalar((2.0 * self.params.grpo), scratch.ad_value(46)))));
        }

        if ((!(scratch.values[127] != 0.0)) && (scratch.values[128] != 0.0)) {
            scratch.values[50] = 0.0;
            scratch.node_derivatives[50] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[50] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((!(scratch.values[127] != 0.0)) && (!(scratch.values[128] != 0.0))) {
            scratch.values[49] = scratch.values[48];
            scratch.node_derivatives[49] = scratch.node_derivatives[48];
            scratch.branch_derivatives[49] = scratch.branch_derivatives[48];
        }

        if ((!(scratch.values[127] != 0.0)) && (!(scratch.values[128] != 0.0))) {
            scratch.values[50] = 0.0;
            scratch.node_derivatives[50] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[50] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(106, &AdValue::div_from_scalar(self.params.ats, AdValue::offset(AdValue::div_from_scalar(self.params.atsl, scratch.ad_value(3)), 1.0)));

        scratch.values[129] = if (self.params.sw_accpo > 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[129] != 0.0) {
            scratch.store_ad(105, &AdValue::scale(scratch.ad_value(71), self.params.nst));
        }

        if (scratch.values[129] != 0.0) {
            scratch.store_ad(107, &{
                if (self.params.sw_accpo > 2.0) {
                    AdValue::mul(AdValue::scale(scratch.ad_value(71), 0.55), AdValue::offset(AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(106)), scratch.ad_value(71))), 1.0))
                } else {
                    AdValue::scale(scratch.ad_value(71), 1.1)
                }
            });
        }

        scratch.values[130] = if (self.params.sw_accpo > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[129] != 0.0)) && (scratch.values[130] != 0.0)) {
            scratch.store_ad(105, &AdValue::scale(scratch.ad_value(71), (2.0 * self.params.nst)));
        }

        if ((!(scratch.values[129] != 0.0)) && (scratch.values[130] != 0.0)) {
            scratch.store_ad(107, &AdValue::mul(AdValue::scale(scratch.ad_value(106), 4.0), scratch.ad_value(106)));
        }

        if ((!(scratch.values[129] != 0.0)) && (!(scratch.values[130] != 0.0))) {
            scratch.store_ad(105, &AdValue::scale(scratch.ad_value(71), self.params.nst));
        }

        if ((!(scratch.values[129] != 0.0)) && (!(scratch.values[130] != 0.0))) {
            scratch.store_ad(107, &AdValue::mul(AdValue::scale(scratch.ad_value(106), 4.0), scratch.ad_value(106)));
        }

        scratch.store_ad(5, &AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(40), self.params.rsh), AdValue::div(scratch.ad_value(3), scratch.ad_value(4))), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(45), AdValue::sqrt(scratch.ad_value(41))))));

        scratch.values[132] = if ((self.params.rc > 0.0) && (self.params.c1 > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[132] != 0.0) {
            scratch.values[54] = ((self.params.rc + (self.params.rcw / scratch.values[26])) / self.params.c1);
            scratch.node_derivatives[54] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[54] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[132] != 0.0)) {
            scratch.values[54] = 0.0;
            scratch.node_derivatives[54] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[54] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[133] = if ((self.params.rc > 0.0) && (self.params.c2 > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[133] != 0.0) {
            scratch.values[55] = ((self.params.rc + (self.params.rcw / scratch.values[26])) / self.params.c2);
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[133] != 0.0)) {
            scratch.values[55] = 0.0;
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        if (self.params.sw_lin != 0.0) {
            scratch.values[47] = 0.0;
            scratch.node_derivatives[47] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[47] = [0.0; Instance::BRANCH_COUNT];
        }

        if (self.params.sw_lin != 0.0) {
            scratch.values[9] = 0.0;
            scratch.node_derivatives[9] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[9] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(self.params.sw_lin != 0.0)) {
            scratch.store_ad(47, &AdValue::scale(AdValue::powf(scratch.ad_value(68), self.params.tegth), (((self.params.gth0 + (self.params.gthp * scratch.values[36])) + (self.params.gtha * scratch.values[35])) + (self.params.gthc * (self.params.c1 + self.params.c2)))));
        }

        if (!(self.params.sw_lin != 0.0)) {
            scratch.values[9] = (((self.params.cth0 + (self.params.cthp * scratch.values[36])) + (self.params.ctha * scratch.values[35])) + (self.params.cthc * (self.params.c1 + self.params.c2)));
            scratch.node_derivatives[9] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[9] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(52, &AdValue::add(AdValue::offset(AdValue::div_from_scalar(self.params.tc1w, scratch.ad_value(4)), self.params.tc1), AdValue::div(AdValue::scale(AdValue::offset(AdValue::div_from_scalar(self.params.tc1wl, scratch.ad_value(4)), self.params.tc1l), (0.5 * (if (self.params.c1 > 0.0) { 1.0 } else { 0.0 } + if (self.params.c2 > 0.0) { 1.0 } else { 0.0 }))), scratch.ad_value(3))));

        scratch.store_ad(53, &AdValue::add(AdValue::offset(AdValue::div_from_scalar(self.params.tc2w, scratch.ad_value(4)), self.params.tc2), AdValue::div(AdValue::scale(AdValue::offset(AdValue::div_from_scalar(self.params.tc2wl, scratch.ad_value(4)), self.params.tc2l), (0.5 * (if (self.params.c1 > 0.0) { 1.0 } else { 0.0 } + if (self.params.c2 > 0.0) { 1.0 } else { 0.0 }))), scratch.ad_value(3))));

        scratch.values[88] = ((self.params.ca * scratch.values[31]) + (self.params.cp * scratch.values[32]));

        scratch.values[89] = ((self.params.ca * scratch.values[33]) + (self.params.cp * scratch.values[34]));

        scratch.values[86] = ((self.params.cja * scratch.values[31]) + (self.params.cjp * scratch.values[32]));

        scratch.values[87] = ((self.params.cja * scratch.values[33]) + (self.params.cjp * scratch.values[34]));

        scratch.store_ad(10, &AdValue::voltage(ctx, &self.nodes, Some(3), None));

        scratch.store_ad(64, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(5), Some(4)), (-self.params.type_)));

        scratch.store_ad(65, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(1), Some(4)), (-self.params.type_)));

        scratch.store_ad(66, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(1), Some(5)), (-self.params.type_)));

        scratch.store_ad(23, &AdValue::offset(AdValue::offset(scratch.ad_value(10), (ctx.temperature() + self.params.trise)), (-273.15)));

        scratch.values[134] = if (scratch.values[23] < (self.params.tminclip + 1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[134] != 0.0) {
            scratch.store_ad(23, &AdValue::offset(AdValue::exp(AdValue::offset(AdValue::offset(scratch.ad_value(23), (-self.params.tminclip)), (-1.0))), self.params.tminclip));
        }

        scratch.values[135] = if (scratch.values[23] > (self.params.tmaxclip - 1.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[134] != 0.0)) && (scratch.values[135] != 0.0)) {
            scratch.store_ad(23, &AdValue::sub_from_scalar(self.params.tmaxclip, AdValue::exp(AdValue::offset(AdValue::sub_from_scalar(self.params.tmaxclip, scratch.ad_value(23)), (-1.0)))));
        }

        if ((!(scratch.values[134] != 0.0)) && (!(scratch.values[135] != 0.0))) {
        }

        scratch.store_ad(24, &AdValue::offset(scratch.ad_value(23), 273.15));

        scratch.store_ad(70, &AdValue::scale(scratch.ad_value(24), (1.3806505e-23 * 6.241509479607718e18)));

        scratch.store_ad(68, &AdValue::scale(scratch.ad_value(24), 1.0 / (scratch.values[15])));

        scratch.store_ad(69, &AdValue::offset(scratch.ad_value(24), (-scratch.values[15])));

        scratch.store_ad(57, &AdValue::offset(AdValue::mul(scratch.ad_value(69), AdValue::add(scratch.ad_value(52), AdValue::mul(scratch.ad_value(69), scratch.ad_value(53)))), 1.0));

        scratch.values[136] = if (scratch.values[57] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (scratch.values[136] != 0.0) {
            scratch.store_ad(57, &AdValue::offset(AdValue::scale(AdValue::exp(AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(57), (-0.01)), 10.0), (-1.0))), 0.1), 0.01));
        }

        if (!(scratch.values[136] != 0.0)) {
        }

        if (self.params.sw_accpo != 0.0) {
            scratch.store_ad(29, &AdValue::div_from_scalar(1.0, AdValue::mul(AdValue::mul(scratch.ad_value(5), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(45), AdValue::sqrt(scratch.ad_value(41))))), scratch.ad_value(57))));
        }

        if (!(self.params.sw_accpo != 0.0)) {
            scratch.store_ad(29, &AdValue::div_from_scalar(1.0, AdValue::mul(scratch.ad_value(5), scratch.ad_value(57))));
        }

        scratch.store_ad(58, &AdValue::offset(AdValue::mul(scratch.ad_value(69), AdValue::offset(AdValue::scale(scratch.ad_value(69), self.params.tc2rc), self.params.tc1rc)), 1.0));

        scratch.values[137] = if (scratch.values[58] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (scratch.values[137] != 0.0) {
            scratch.store_ad(58, &AdValue::offset(AdValue::scale(AdValue::exp(AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(58), (-0.01)), 10.0), (-1.0))), 0.1), 0.01));
        }

        if (!(scratch.values[137] != 0.0)) {
        }

        scratch.store_ad(59, &AdValue::powf(scratch.ad_value(68), self.params.xvsat));

        scratch.values[138] = if (self.params.isa > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[138] != 0.0) {
            scratch.store_ad(74, &AdValue::scale(AdValue::exp(AdValue::scale(AdValue::add(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(68)), (-self.params.ea)), scratch.ad_value(70)), AdValue::scale(AdValue::ln(scratch.ad_value(68)), self.params.xis)), 1.0 / (self.params.na))), self.params.isa));
        }

        if (scratch.values[138] != 0.0) {
            scratch.store_ad(61, &AdValue::mul(AdValue::scale(scratch.ad_value(70), self.params.na), AdValue::ln(AdValue::offset(AdValue::div_from_scalar(self.params.imax, scratch.ad_value(74)), 1.0))));
        }

        if (!(scratch.values[138] != 0.0)) {
            scratch.values[74] = 0.0;
            scratch.node_derivatives[74] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[74] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[138] != 0.0)) {
            scratch.values[61] = 0.0;
            scratch.node_derivatives[61] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[61] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[139] = if (self.params.isp > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[139] != 0.0) {
            scratch.store_ad(75, &AdValue::scale(AdValue::exp(AdValue::scale(AdValue::add(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(68)), (-self.params.ea)), scratch.ad_value(70)), AdValue::scale(AdValue::ln(scratch.ad_value(68)), self.params.xis)), 1.0 / (self.params.np))), self.params.isp));
        }

        if (scratch.values[139] != 0.0) {
            scratch.store_ad(60, &AdValue::mul(AdValue::scale(scratch.ad_value(70), self.params.np), AdValue::ln(AdValue::offset(AdValue::div_from_scalar(self.params.imax, scratch.ad_value(75)), 1.0))));
        }

        if (!(scratch.values[139] != 0.0)) {
            scratch.values[75] = 0.0;
            scratch.node_derivatives[75] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[75] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[139] != 0.0)) {
            scratch.values[60] = 0.0;
            scratch.node_derivatives[60] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[60] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(84, &AdValue::add(AdValue::scale(scratch.ad_value(74), scratch.values[31]), AdValue::scale(scratch.ad_value(75), scratch.values[32])));

        scratch.store_ad(85, &AdValue::add(AdValue::scale(scratch.ad_value(74), scratch.values[33]), AdValue::scale(scratch.ad_value(75), scratch.values[34])));

        scratch.values[140] = if (self.params.cja > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[140] != 0.0) {
            scratch.store_ad(141, &AdValue::mul(AdValue::scale(AdValue::div(scratch.ad_value(70), scratch.ad_value(68)), 2.0), AdValue::ln(AdValue::sub(AdValue::exp(AdValue::div(AdValue::scale(scratch.ad_value(68), (0.5 * self.params.pa)), scratch.ad_value(70))), AdValue::exp(AdValue::div(AdValue::scale(scratch.ad_value(68), ((-0.5) * self.params.pa)), scratch.ad_value(70)))))));
        }

        if (scratch.values[140] != 0.0) {
            scratch.store_ad(142, &AdValue::sub(AdValue::sub(AdValue::mul(scratch.ad_value(141), scratch.ad_value(68)), AdValue::mul(AdValue::scale(scratch.ad_value(70), 3.0), AdValue::ln(scratch.ad_value(68)))), AdValue::scale(AdValue::offset(scratch.ad_value(68), (-1.0)), self.params.ea)));
        }

        if (scratch.values[140] != 0.0) {
            scratch.store_ad(76, &AdValue::add(scratch.ad_value(142), AdValue::mul(AdValue::scale(scratch.ad_value(70), 2.0), AdValue::ln(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(142)), scratch.ad_value(70))), 4.0), 1.0)), 1.0), 0.5)))));
        }

        if (scratch.values[140] != 0.0) {
            scratch.store_ad(77, &AdValue::scale(AdValue::powf(AdValue::div_from_scalar(self.params.pa, scratch.ad_value(76)), self.params.ma), self.params.cja));
        }

        if (!(scratch.values[140] != 0.0)) {
            scratch.values[76] = self.params.pa;
            scratch.node_derivatives[76] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[76] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[140] != 0.0)) {
            scratch.values[77] = 0.0;
            scratch.node_derivatives[77] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[77] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[143] = if (self.params.cjp > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[143] != 0.0) {
            scratch.store_ad(144, &AdValue::mul(AdValue::scale(AdValue::div(scratch.ad_value(70), scratch.ad_value(68)), 2.0), AdValue::ln(AdValue::sub(AdValue::exp(AdValue::div(AdValue::scale(scratch.ad_value(68), (0.5 * self.params.pp)), scratch.ad_value(70))), AdValue::exp(AdValue::div(AdValue::scale(scratch.ad_value(68), ((-0.5) * self.params.pp)), scratch.ad_value(70)))))));
        }

        if (scratch.values[143] != 0.0) {
            scratch.store_ad(145, &AdValue::sub(AdValue::sub(AdValue::mul(scratch.ad_value(144), scratch.ad_value(68)), AdValue::mul(AdValue::scale(scratch.ad_value(70), 3.0), AdValue::ln(scratch.ad_value(68)))), AdValue::scale(AdValue::offset(scratch.ad_value(68), (-1.0)), self.params.ea)));
        }

        if (scratch.values[143] != 0.0) {
            scratch.store_ad(78, &AdValue::add(scratch.ad_value(145), AdValue::mul(AdValue::scale(scratch.ad_value(70), 2.0), AdValue::ln(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(145)), scratch.ad_value(70))), 4.0), 1.0)), 1.0), 0.5)))));
        }

        if (scratch.values[143] != 0.0) {
            scratch.store_ad(79, &AdValue::scale(AdValue::powf(AdValue::div_from_scalar(self.params.pp, scratch.ad_value(78)), self.params.mp), self.params.cjp));
        }

        if (!(scratch.values[143] != 0.0)) {
            scratch.values[78] = self.params.pp;
            scratch.node_derivatives[78] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[78] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[143] != 0.0)) {
            scratch.values[79] = 0.0;
            scratch.node_derivatives[79] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[79] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(80, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(69), self.params.tc1kfn), 1.0), self.params.kfn));

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if !(scratch.values[80] > 0.0) {
            scratch.values[80] = 0.0;
            scratch.node_derivatives[80] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[80] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[146] = if (self.params.vbv > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[146] != 0.0) {
            scratch.store_ad(103, &AdValue::scale(AdValue::offset(AdValue::mul(scratch.ad_value(69), AdValue::offset(AdValue::scale(scratch.ad_value(69), self.params.tc2vbv), self.params.tc1vbv)), 1.0), self.params.vbv));
        }

        if (scratch.values[146] != 0.0) {
            scratch.store_ad(103, &{
                if (scratch.values[103] > 0.0) {
                    scratch.ad_value(103)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[146] != 0.0) {
            scratch.store_ad(104, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(69), self.params.tc1nbv), 1.0), self.params.nbv));
        }

        if (scratch.values[146] != 0.0) {
            scratch.store_ad(62, &AdValue::mul(AdValue::mul(scratch.ad_value(104), scratch.ad_value(70)), AdValue::ln(AdValue::offset(AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(103)), AdValue::mul(scratch.ad_value(104), scratch.ad_value(70)))), (self.params.imax / self.params.ibv)))));
        }

        if (!(scratch.values[146] != 0.0)) {
            scratch.values[103] = self.params.vbv;
            scratch.node_derivatives[103] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[103] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[146] != 0.0)) {
            scratch.values[104] = self.params.nbv;
            scratch.node_derivatives[104] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[104] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[146] != 0.0)) {
            scratch.values[62] = 1.0;
            scratch.node_derivatives[62] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[62] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[147] = if ((self.params.ecrit > 0.0) && (!(self.params.sw_lin != 0.0))) { 1.0 } else { 0.0 };

        if ((scratch.values[147] != 0.0) && (self.params.sw_vsatt != 0.0)) {
            scratch.store_ad(72, &AdValue::mul(AdValue::scale(scratch.ad_value(59), self.params.ecorn), scratch.ad_value(57)));
        }

        if ((scratch.values[147] != 0.0) && (self.params.sw_vsatt != 0.0)) {
            scratch.store_ad(73, &AdValue::mul(AdValue::scale(scratch.ad_value(59), self.params.ecrit), scratch.ad_value(57)));
        }

        if ((scratch.values[147] != 0.0) && (!(self.params.sw_vsatt != 0.0))) {
            scratch.values[72] = self.params.ecorn;
            scratch.node_derivatives[72] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[72] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[147] != 0.0) && (!(self.params.sw_vsatt != 0.0))) {
            scratch.values[73] = self.params.ecrit;
            scratch.node_derivatives[73] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[73] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[147] != 0.0) {
            scratch.store_ad(19, &AdValue::sub(AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(72)), AdValue::mul(AdValue::scale(scratch.ad_value(73), ((4.0 * self.params.du) * self.params.du)), scratch.ad_value(73)))), AdValue::scale(scratch.ad_value(73), (2.0 * self.params.du))));
        }

        if (scratch.values[147] != 0.0) {
            scratch.store_ad(20, &AdValue::div(AdValue::scale(scratch.ad_value(19), self.params.du), scratch.ad_value(73)));
        }

        if (scratch.values[147] != 0.0) {
            scratch.store_ad(21, &AdValue::sqrt(AdValue::add(AdValue::div(AdValue::square(scratch.ad_value(19)), AdValue::square(scratch.ad_value(73))), AdValue::scale(scratch.ad_value(20), 4.0))));
        }

        if (scratch.values[147] != 0.0) {
            scratch.store_ad(22, &AdValue::sub(scratch.ad_value(73), scratch.ad_value(72)));
        }

        if (scratch.values[147] != 0.0) {
            scratch.store_ad(18, &AdValue::div_from_scalar(1.0, scratch.ad_value(73)));
        }

        if (!(scratch.values[147] != 0.0)) {
            scratch.values[19] = 0.0;
            scratch.node_derivatives[19] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[19] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[147] != 0.0)) {
            scratch.values[20] = 0.0;
            scratch.node_derivatives[20] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[20] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[147] != 0.0)) {
            scratch.values[21] = 0.0;
            scratch.node_derivatives[21] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[21] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[147] != 0.0)) {
            scratch.values[22] = 1000.0;
            scratch.node_derivatives[22] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[22] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[147] != 0.0)) {
            scratch.values[18] = 0.0;
            scratch.node_derivatives[18] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[18] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(51, &AdValue::mul(scratch.ad_value(28), scratch.ad_value(22)));

        scratch.values[148] = if (scratch.values[51] > 100000.0) { 1.0 } else { 0.0 };

        if (scratch.values[148] != 0.0) {
            scratch.values[51] = 100000.0;
            scratch.node_derivatives[51] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[51] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[199] = if (scratch.values[64] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[199] != 0.0) {
            scratch.values[149] = (-1.0);
            scratch.node_derivatives[149] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[149] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[199] != 0.0) {
            scratch.store_ad(150, &AdValue::neg(scratch.ad_value(66)));
        }

        if (scratch.values[199] != 0.0) {
            scratch.store_ad(151, &AdValue::neg(scratch.ad_value(64)));
        }

        if (!(scratch.values[199] != 0.0)) {
            scratch.values[149] = 1.0;
            scratch.node_derivatives[149] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[149] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[199] != 0.0)) {
            scratch.store_ad(150, &AdValue::neg(scratch.ad_value(65)));
        }

        if (!(scratch.values[199] != 0.0)) {
            scratch.values[151] = scratch.values[64];
            scratch.node_derivatives[151] = scratch.node_derivatives[64];
            scratch.branch_derivatives[151] = scratch.branch_derivatives[64];
        }

        scratch.values[200] = if (scratch.values[150] > scratch.values[49]) { 1.0 } else { 0.0 };

        if (scratch.values[200] != 0.0) {
            scratch.store_ad(152, &AdValue::sub(scratch.ad_value(49), AdValue::mul(scratch.ad_value(105), AdValue::ln(AdValue::offset(AdValue::exp(AdValue::div(AdValue::sub(scratch.ad_value(49), scratch.ad_value(150)), scratch.ad_value(105))), 1.0)))));
        }

        if (!(scratch.values[200] != 0.0)) {
            scratch.store_ad(152, &AdValue::sub(scratch.ad_value(150), AdValue::mul(scratch.ad_value(105), AdValue::ln(AdValue::offset(AdValue::exp(AdValue::div(AdValue::sub(scratch.ad_value(150), scratch.ad_value(49)), scratch.ad_value(105))), 1.0)))));
        }

        scratch.values[201] = if (scratch.values[152] < ((-0.4) * (scratch.values[41] + (if (scratch.values[151] < (scratch.values[49] - scratch.values[152])) { scratch.values[151] } else { (scratch.values[49] - scratch.values[152]) })))) { 1.0 } else { 0.0 };

        if ((self.params.sw_accpo != 0.0) && (scratch.values[201] != 0.0)) {
            scratch.store_ad(153, &AdValue::scale(AdValue::add(scratch.ad_value(41), {
                if (scratch.values[151] < (scratch.values[49] - scratch.values[152])) {
                    scratch.ad_value(151)
                } else {
                    AdValue::sub(scratch.ad_value(49), scratch.ad_value(152))
                }
            }), (-0.4)));
        }

        if ((self.params.sw_accpo != 0.0) && (!(scratch.values[201] != 0.0))) {
            scratch.values[153] = scratch.values[152];
            scratch.node_derivatives[153] = scratch.node_derivatives[152];
            scratch.branch_derivatives[153] = scratch.branch_derivatives[152];
        }

        scratch.values[202] = if (scratch.values[152] < ((-0.4) * scratch.values[41])) { 1.0 } else { 0.0 };

        if ((!(self.params.sw_accpo != 0.0)) && (scratch.values[202] != 0.0)) {
            scratch.store_ad(153, &AdValue::scale(scratch.ad_value(41), (-0.4)));
        }

        if ((!(self.params.sw_accpo != 0.0)) && (!(scratch.values[202] != 0.0))) {
            scratch.values[153] = scratch.values[152];
            scratch.node_derivatives[153] = scratch.node_derivatives[152];
            scratch.branch_derivatives[153] = scratch.branch_derivatives[152];
        }

        scratch.store_ad(154, &AdValue::add(scratch.ad_value(41), AdValue::scale(scratch.ad_value(153), 2.0)));

        scratch.values[203] = if (scratch.values[18] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(155, &AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(46), scratch.ad_value(154)), scratch.ad_value(154)), scratch.ad_value(154)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(156, &AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(46), 3.0), scratch.ad_value(154)), (-1.0)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(157, &AdValue::mul(scratch.ad_value(46), AdValue::offset(AdValue::div(scratch.ad_value(154), scratch.ad_value(51)), (9.0 / 4.0))));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(158, &AdValue::div(AdValue::scale(scratch.ad_value(46), 1.5), scratch.ad_value(51)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(159, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(51), 4.0), scratch.ad_value(51)), scratch.ad_value(46)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(160, &AdValue::mul(scratch.ad_value(155), scratch.ad_value(159)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(161, &AdValue::scale(scratch.ad_value(159), self.params.a1));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(162, &AdValue::scale(scratch.ad_value(159), self.params.a2));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(163, &AdValue::mul(scratch.ad_value(158), scratch.ad_value(159)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(164, &AdValue::square(scratch.ad_value(163)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(165, &AdValue::neg(scratch.ad_value(162)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(166, &AdValue::sub(AdValue::mul(scratch.ad_value(163), scratch.ad_value(161)), AdValue::scale(scratch.ad_value(160), 4.0)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(167, &AdValue::sub(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(162), 4.0), scratch.ad_value(160)), AdValue::square(scratch.ad_value(161))), AdValue::mul(scratch.ad_value(160), scratch.ad_value(164))));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(168, &AdValue::sub(scratch.ad_value(166), AdValue::scale(AdValue::square(scratch.ad_value(165)), 0.3333333333333333)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(169, &AdValue::sub(scratch.ad_value(167), AdValue::scale(AdValue::mul(scratch.ad_value(165), AdValue::add(scratch.ad_value(166), AdValue::scale(scratch.ad_value(168), 2.0))), 0.1111111111111111)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(170, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(168)), scratch.ad_value(168)), 0.037037037037037035));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(171, &AdValue::add(AdValue::mul(AdValue::scale(scratch.ad_value(169), 0.25), scratch.ad_value(169)), scratch.ad_value(170)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(172, &AdValue::sqrt(scratch.ad_value(171)));
        }

        scratch.values[204] = if (scratch.values[169] < 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[203] != 0.0) && (scratch.values[204] != 0.0)) {
            scratch.store_ad(173, &AdValue::add(AdValue::scale(scratch.ad_value(169), (-0.5)), scratch.ad_value(172)));
        }

        if ((scratch.values[203] != 0.0) && (scratch.values[204] != 0.0)) {
            scratch.store_ad(174, &AdValue::div(AdValue::neg(scratch.ad_value(170)), scratch.ad_value(173)));
        }

        if ((scratch.values[203] != 0.0) && (!(scratch.values[204] != 0.0))) {
            scratch.store_ad(174, &AdValue::sub(AdValue::scale(scratch.ad_value(169), (-0.5)), scratch.ad_value(172)));
        }

        if ((scratch.values[203] != 0.0) && (!(scratch.values[204] != 0.0))) {
            scratch.store_ad(173, &AdValue::div(AdValue::neg(scratch.ad_value(170)), scratch.ad_value(174)));
        }

        scratch.values[205] = if (scratch.values[173] > 1e-6) { 1.0 } else { 0.0 };

        if ((scratch.values[203] != 0.0) && (scratch.values[205] != 0.0)) {
            scratch.store_ad(175, &AdValue::powf(scratch.ad_value(173), 0.3333333333333333));
        }

        scratch.values[206] = if (scratch.values[173] < (-1e-6)) { 1.0 } else { 0.0 };

        if (((scratch.values[203] != 0.0) && (!(scratch.values[205] != 0.0))) && (scratch.values[206] != 0.0)) {
            scratch.store_ad(175, &AdValue::neg(AdValue::powf(AdValue::neg(scratch.ad_value(173)), 0.3333333333333333)));
        }

        if (((scratch.values[203] != 0.0) && (!(scratch.values[205] != 0.0))) && (!(scratch.values[206] != 0.0))) {
            scratch.store_ad(175, &AdValue::scale(scratch.ad_value(173), 10000.0));
        }

        scratch.values[207] = if (scratch.values[174] > 1e-6) { 1.0 } else { 0.0 };

        if ((scratch.values[203] != 0.0) && (scratch.values[207] != 0.0)) {
            scratch.store_ad(176, &AdValue::powf(scratch.ad_value(174), 0.3333333333333333));
        }

        scratch.values[208] = if (scratch.values[174] < (-1e-6)) { 1.0 } else { 0.0 };

        if (((scratch.values[203] != 0.0) && (!(scratch.values[207] != 0.0))) && (scratch.values[208] != 0.0)) {
            scratch.store_ad(176, &AdValue::neg(AdValue::powf(AdValue::neg(scratch.ad_value(174)), 0.3333333333333333)));
        }

        if (((scratch.values[203] != 0.0) && (!(scratch.values[207] != 0.0))) && (!(scratch.values[208] != 0.0))) {
            scratch.store_ad(176, &AdValue::scale(scratch.ad_value(174), 10000.0));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(177, &AdValue::sub(AdValue::add(scratch.ad_value(175), scratch.ad_value(176)), AdValue::scale(scratch.ad_value(165), 0.3333333333333333)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(167, &AdValue::sqrt(AdValue::add(AdValue::sub(AdValue::scale(scratch.ad_value(164), 0.25), scratch.ad_value(162)), scratch.ad_value(177))));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(178, &AdValue::sub(AdValue::sub(AdValue::scale(scratch.ad_value(164), 0.75), AdValue::square(scratch.ad_value(167))), AdValue::scale(scratch.ad_value(162), 2.0)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(179, &AdValue::div(AdValue::sub(AdValue::sub(AdValue::mul(scratch.ad_value(163), scratch.ad_value(162)), AdValue::scale(scratch.ad_value(161), 2.0)), AdValue::mul(AdValue::scale(scratch.ad_value(164), 0.25), scratch.ad_value(163))), scratch.ad_value(167)));
        }

        if (scratch.values[203] != 0.0) {
            scratch.store_ad(180, &AdValue::add(scratch.ad_value(178), scratch.ad_value(179)));
        }

        scratch.values[209] = if (scratch.values[180] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[203] != 0.0) && (scratch.values[209] != 0.0)) {
            scratch.store_ad(182, &AdValue::sqrt(scratch.ad_value(180)));
        }

        if ((scratch.values[203] != 0.0) && (scratch.values[209] != 0.0)) {
            scratch.store_ad(183, &AdValue::add(AdValue::scale(scratch.ad_value(163), (-0.25)), AdValue::scale(AdValue::add(scratch.ad_value(182), scratch.ad_value(167)), 0.5)));
        }

        if ((scratch.values[203] != 0.0) && (!(scratch.values[209] != 0.0))) {
            scratch.store_ad(181, &AdValue::sub(scratch.ad_value(178), scratch.ad_value(179)));
        }

        if ((scratch.values[203] != 0.0) && (!(scratch.values[209] != 0.0))) {
            scratch.store_ad(182, &AdValue::sqrt(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(181)), 0.0001))));
        }

        if ((scratch.values[203] != 0.0) && (!(scratch.values[209] != 0.0))) {
            scratch.store_ad(183, &AdValue::add(AdValue::scale(scratch.ad_value(163), (-0.25)), AdValue::scale(AdValue::sub(scratch.ad_value(182), scratch.ad_value(167)), 0.5)));
        }

        scratch.values[210] = if (scratch.values[153] > scratch.values[50]) { 1.0 } else { 0.0 };

        if ((!(scratch.values[203] != 0.0)) && (scratch.values[210] != 0.0)) {
            scratch.store_ad(198, &AdValue::mul(scratch.ad_value(46), AdValue::sub(scratch.ad_value(48), scratch.ad_value(153))));
        }

        if ((!(scratch.values[203] != 0.0)) && (scratch.values[210] != 0.0)) {
            scratch.store_ad(183, &AdValue::div(AdValue::mul(AdValue::scale(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(198), 2.0)), 2.0), AdValue::sub(scratch.ad_value(48), scratch.ad_value(153))), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(198), 3.0)), AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(198), 1.5))))));
        }

        if ((!(scratch.values[203] != 0.0)) && (!(scratch.values[210] != 0.0))) {
            scratch.store_ad(198, &AdValue::mul(AdValue::scale(scratch.ad_value(46), 3.0), scratch.ad_value(154)));
        }

        if ((!(scratch.values[203] != 0.0)) && (!(scratch.values[210] != 0.0))) {
            scratch.store_ad(183, &AdValue::div(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(198)), AdValue::sqrt(AdValue::offset(scratch.ad_value(198), 1.0))), AdValue::scale(scratch.ad_value(46), 4.5)));
        }

        scratch.values[211] = if ((self.params.sw_accpo > 1.0) && (scratch.values[45] > 1e-9)) { 1.0 } else { 0.0 };

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(193, &AdValue::add(scratch.ad_value(183), scratch.ad_value(71)));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(194, &AdValue::mul(scratch.ad_value(45), AdValue::sqrt(AdValue::add(scratch.ad_value(154), scratch.ad_value(183)))));
        }

        scratch.values[212] = if (scratch.values[18] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(185, &AdValue::mul(AdValue::scale(AdValue::sub(AdValue::div(scratch.ad_value(193), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(186, &AdValue::mul(AdValue::scale(AdValue::add(AdValue::div(scratch.ad_value(193), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(188, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(185)), scratch.ad_value(20))));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(187, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(186)), scratch.ad_value(20))));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(189, &AdValue::sub(AdValue::add(scratch.ad_value(188), scratch.ad_value(187)), scratch.ad_value(21)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::mul(AdValue::scale(AdValue::add(AdValue::div(scratch.ad_value(185), scratch.ad_value(188)), AdValue::div(scratch.ad_value(186), scratch.ad_value(187))), 0.5), scratch.ad_value(18)), scratch.ad_value(28)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[212] != 0.0)) {
            scratch.store_ad(196, &AdValue::sqrt(AdValue::div(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(194), 2.0), AdValue::sub_from_scalar(1.0, scratch.ad_value(194))), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::mul(scratch.ad_value(195), scratch.ad_value(193)), AdValue::offset(scratch.ad_value(189), 1.0)))), scratch.ad_value(193))));
        }

        if ((scratch.values[211] != 0.0) && (!(scratch.values[212] != 0.0))) {
            scratch.store_ad(196, &AdValue::sqrt(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(194), 2.0), AdValue::sub_from_scalar(1.0, scratch.ad_value(194))), scratch.ad_value(193))));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(197, &AdValue::sub(AdValue::div(AdValue::mul(scratch.ad_value(46), AdValue::add(scratch.ad_value(154), scratch.ad_value(183))), AdValue::square(scratch.ad_value(196))), scratch.ad_value(193)));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(191, &AdValue::add(scratch.ad_value(107), AdValue::div(AdValue::scale(scratch.ad_value(183), self.params.ats), AdValue::offset(scratch.ad_value(193), self.params.ats))));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(192, &AdValue::mul(AdValue::scale(scratch.ad_value(191), 4.0), scratch.ad_value(191)));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(184, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(151), 2.0), scratch.ad_value(193)), AdValue::add(AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(151), scratch.ad_value(193)), AdValue::sub(scratch.ad_value(151), scratch.ad_value(193))), scratch.ad_value(192))), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(151), scratch.ad_value(193)), AdValue::add(scratch.ad_value(151), scratch.ad_value(193))), scratch.ad_value(192))))));
        }

        scratch.values[213] = if (self.params.sw_accpo > 2.0) { 1.0 } else { 0.0 };

        if ((scratch.values[211] != 0.0) && (scratch.values[213] != 0.0)) {
            scratch.store_ad(191, &AdValue::add(scratch.ad_value(107), AdValue::div(AdValue::scale(scratch.ad_value(184), self.params.ats), AdValue::offset(scratch.ad_value(193), self.params.ats))));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[213] != 0.0)) {
            scratch.store_ad(192, &AdValue::mul(AdValue::scale(scratch.ad_value(191), 4.0), scratch.ad_value(191)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[213] != 0.0)) {
            scratch.store_ad(184, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(151), 2.0), scratch.ad_value(193)), AdValue::add(AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(151), scratch.ad_value(193)), AdValue::sub(scratch.ad_value(151), scratch.ad_value(193))), scratch.ad_value(192))), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(151), scratch.ad_value(193)), AdValue::add(scratch.ad_value(151), scratch.ad_value(193))), scratch.ad_value(192))))));
        }

        if (scratch.values[211] != 0.0) {
            scratch.store_ad(190, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(196), AdValue::sqrt(AdValue::add(scratch.ad_value(197), scratch.ad_value(184))))));
        }

        scratch.values[214] = if (scratch.values[18] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[211] != 0.0) && (scratch.values[214] != 0.0)) {
            scratch.store_ad(185, &AdValue::mul(AdValue::scale(AdValue::sub(AdValue::div(scratch.ad_value(184), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[214] != 0.0)) {
            scratch.store_ad(186, &AdValue::mul(AdValue::scale(AdValue::add(AdValue::div(scratch.ad_value(184), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[214] != 0.0)) {
            scratch.store_ad(188, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(185)), scratch.ad_value(20))));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[214] != 0.0)) {
            scratch.store_ad(187, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(186)), scratch.ad_value(20))));
        }

        if ((scratch.values[211] != 0.0) && (scratch.values[214] != 0.0)) {
            scratch.store_ad(189, &AdValue::sub(AdValue::add(scratch.ad_value(188), scratch.ad_value(187)), scratch.ad_value(21)));
        }

        if ((scratch.values[211] != 0.0) && (!(scratch.values[214] != 0.0))) {
            scratch.values[189] = 0.0;
            scratch.node_derivatives[189] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[189] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[211] != 0.0)) {
            scratch.store_ad(184, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(151), 2.0), scratch.ad_value(183)), AdValue::add(AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(151), scratch.ad_value(183)), AdValue::sub(scratch.ad_value(151), scratch.ad_value(183))), scratch.ad_value(107))), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(151), scratch.ad_value(183)), AdValue::add(scratch.ad_value(151), scratch.ad_value(183))), scratch.ad_value(107))))));
        }

        scratch.values[215] = if (scratch.values[18] > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[211] != 0.0)) && (scratch.values[215] != 0.0)) {
            scratch.store_ad(185, &AdValue::mul(AdValue::scale(AdValue::sub(AdValue::div(scratch.ad_value(184), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((!(scratch.values[211] != 0.0)) && (scratch.values[215] != 0.0)) {
            scratch.store_ad(186, &AdValue::mul(AdValue::scale(AdValue::add(AdValue::div(scratch.ad_value(184), scratch.ad_value(28)), scratch.ad_value(19)), 0.5), scratch.ad_value(18)));
        }

        if ((!(scratch.values[211] != 0.0)) && (scratch.values[215] != 0.0)) {
            scratch.store_ad(188, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(185)), scratch.ad_value(20))));
        }

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[211] != 0.0)) && (scratch.values[215] != 0.0)) {
            scratch.store_ad(187, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(186)), scratch.ad_value(20))));
        }

        if ((!(scratch.values[211] != 0.0)) && (scratch.values[215] != 0.0)) {
            scratch.store_ad(189, &AdValue::sub(AdValue::add(scratch.ad_value(188), scratch.ad_value(187)), scratch.ad_value(21)));
        }

        if ((!(scratch.values[211] != 0.0)) && (!(scratch.values[215] != 0.0))) {
            scratch.values[189] = 0.0;
            scratch.node_derivatives[189] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[189] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[211] != 0.0)) {
            scratch.store_ad(190, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(45), AdValue::sqrt(AdValue::add(scratch.ad_value(154), scratch.ad_value(184))))));
        }

        scratch.values[216] = if (scratch.values[190] < self.params.grpo) { 1.0 } else { 0.0 };

        if (scratch.values[216] != 0.0) {
            scratch.values[190] = self.params.grpo;
            scratch.node_derivatives[190] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[190] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(63, &AdValue::div(AdValue::mul(scratch.ad_value(29), scratch.ad_value(190)), AdValue::offset(scratch.ad_value(189), 1.0)));

        scratch.store_ad(81, &AdValue::mul(AdValue::mul(scratch.ad_value(149), scratch.ad_value(63)), scratch.ad_value(184)));

        scratch.values[217] = if (scratch.values[84] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[217] != 0.0) {
            scratch.store_ad(218, &AdValue::scale(scratch.ad_value(74), scratch.values[31]));
        }

        if (scratch.values[217] != 0.0) {
            scratch.store_ad(219, &AdValue::scale(scratch.ad_value(75), scratch.values[32]));
        }

        scratch.values[224] = if (scratch.values[218] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[217] != 0.0) && (scratch.values[224] != 0.0)) {
            scratch.store_ad(220, &AdValue::div_from_scalar(1.0, AdValue::scale(scratch.ad_value(70), self.params.na)));
        }

        scratch.values[225] = if (scratch.values[65] < scratch.values[61]) { 1.0 } else { 0.0 };

        if (((scratch.values[217] != 0.0) && (scratch.values[224] != 0.0)) && (scratch.values[225] != 0.0)) {
            scratch.store_ad(221, &AdValue::exp(AdValue::mul(scratch.ad_value(65), scratch.ad_value(220))));
        }

        if (((scratch.values[217] != 0.0) && (scratch.values[224] != 0.0)) && (!(scratch.values[225] != 0.0))) {
            scratch.store_ad(221, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(61), scratch.ad_value(220))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(65), scratch.ad_value(61)), scratch.ad_value(220)), 1.0)));
        }

        if ((scratch.values[217] != 0.0) && (scratch.values[224] != 0.0)) {
            scratch.store_ad(222, &AdValue::mul(scratch.ad_value(218), AdValue::offset(scratch.ad_value(221), (-1.0))));
        }

        if ((scratch.values[217] != 0.0) && (!(scratch.values[224] != 0.0))) {
            scratch.values[222] = 0.0;
            scratch.node_derivatives[222] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[222] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[226] = if (scratch.values[219] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[217] != 0.0) && (scratch.values[226] != 0.0)) {
            scratch.store_ad(220, &AdValue::div_from_scalar(1.0, AdValue::scale(scratch.ad_value(70), self.params.np)));
        }

        scratch.values[227] = if (scratch.values[65] < scratch.values[60]) { 1.0 } else { 0.0 };

        if (((scratch.values[217] != 0.0) && (scratch.values[226] != 0.0)) && (scratch.values[227] != 0.0)) {
            scratch.store_ad(221, &AdValue::exp(AdValue::mul(scratch.ad_value(65), scratch.ad_value(220))));
        }

        if (((scratch.values[217] != 0.0) && (scratch.values[226] != 0.0)) && (!(scratch.values[227] != 0.0))) {
            scratch.store_ad(221, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(60), scratch.ad_value(220))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(65), scratch.ad_value(60)), scratch.ad_value(220)), 1.0)));
        }

        if ((scratch.values[217] != 0.0) && (scratch.values[226] != 0.0)) {
            scratch.store_ad(223, &AdValue::mul(scratch.ad_value(219), AdValue::offset(scratch.ad_value(221), (-1.0))));
        }

        if ((scratch.values[217] != 0.0) && (!(scratch.values[226] != 0.0))) {
            scratch.values[223] = 0.0;
            scratch.node_derivatives[223] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[223] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[217] != 0.0) {
            scratch.store_ad(90, &AdValue::add(scratch.ad_value(222), scratch.ad_value(223)));
        }

        scratch.values[231] = if (scratch.values[103] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[217] != 0.0) && (scratch.values[231] != 0.0)) {
            scratch.store_ad(228, &AdValue::sub(AdValue::neg(scratch.ad_value(103)), scratch.ad_value(65)));
        }

        if ((scratch.values[217] != 0.0) && (scratch.values[231] != 0.0)) {
            scratch.store_ad(229, &AdValue::div_from_scalar(1.0, AdValue::mul(scratch.ad_value(104), scratch.ad_value(70))));
        }

        scratch.values[232] = if (scratch.values[228] < scratch.values[62]) { 1.0 } else { 0.0 };

        if (((scratch.values[217] != 0.0) && (scratch.values[231] != 0.0)) && (scratch.values[232] != 0.0)) {
            scratch.store_ad(230, &AdValue::exp(AdValue::mul(scratch.ad_value(228), scratch.ad_value(229))));
        }

        if (((scratch.values[217] != 0.0) && (scratch.values[231] != 0.0)) && (!(scratch.values[232] != 0.0))) {
            scratch.store_ad(230, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(62), scratch.ad_value(229))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(228), scratch.ad_value(62)), scratch.ad_value(229)), 1.0)));
        }

        if ((scratch.values[217] != 0.0) && (scratch.values[231] != 0.0)) {
            scratch.store_ad(92, &AdValue::scale(AdValue::sub(scratch.ad_value(230), AdValue::exp(AdValue::mul(AdValue::neg(scratch.ad_value(103)), scratch.ad_value(229)))), (-self.params.ibv)));
        }

        if ((scratch.values[217] != 0.0) && (!(scratch.values[231] != 0.0))) {
            scratch.values[92] = 0.0;
            scratch.node_derivatives[92] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[92] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[217] != 0.0) {
            scratch.store_ad(82, &AdValue::add(AdValue::add(scratch.ad_value(90), scratch.ad_value(92)), AdValue::scale(scratch.ad_value(65), scratch.values[11])));
        }

        if (!(scratch.values[217] != 0.0)) {
            scratch.values[90] = 0.0;
            scratch.node_derivatives[90] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[90] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[217] != 0.0)) {
            scratch.values[92] = 0.0;
            scratch.node_derivatives[92] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[92] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[217] != 0.0)) {
            scratch.values[82] = 0.0;
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[233] = if (scratch.values[85] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[233] != 0.0) {
            scratch.store_ad(234, &AdValue::scale(scratch.ad_value(74), scratch.values[33]));
        }

        if (scratch.values[233] != 0.0) {
            scratch.store_ad(235, &AdValue::scale(scratch.ad_value(75), scratch.values[34]));
        }

        scratch.values[240] = if (scratch.values[234] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[233] != 0.0) && (scratch.values[240] != 0.0)) {
            scratch.store_ad(236, &AdValue::div_from_scalar(1.0, AdValue::scale(scratch.ad_value(70), self.params.na)));
        }

        scratch.values[241] = if (scratch.values[66] < scratch.values[61]) { 1.0 } else { 0.0 };

        if (((scratch.values[233] != 0.0) && (scratch.values[240] != 0.0)) && (scratch.values[241] != 0.0)) {
            scratch.store_ad(237, &AdValue::exp(AdValue::mul(scratch.ad_value(66), scratch.ad_value(236))));
        }

        if (((scratch.values[233] != 0.0) && (scratch.values[240] != 0.0)) && (!(scratch.values[241] != 0.0))) {
            scratch.store_ad(237, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(61), scratch.ad_value(236))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(66), scratch.ad_value(61)), scratch.ad_value(236)), 1.0)));
        }

        if ((scratch.values[233] != 0.0) && (scratch.values[240] != 0.0)) {
            scratch.store_ad(238, &AdValue::mul(scratch.ad_value(234), AdValue::offset(scratch.ad_value(237), (-1.0))));
        }

        if ((scratch.values[233] != 0.0) && (!(scratch.values[240] != 0.0))) {
            scratch.values[238] = 0.0;
            scratch.node_derivatives[238] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[238] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[242] = if (scratch.values[235] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[233] != 0.0) && (scratch.values[242] != 0.0)) {
            scratch.store_ad(236, &AdValue::div_from_scalar(1.0, AdValue::scale(scratch.ad_value(70), self.params.np)));
        }

        scratch.values[243] = if (scratch.values[66] < scratch.values[60]) { 1.0 } else { 0.0 };

        if (((scratch.values[233] != 0.0) && (scratch.values[242] != 0.0)) && (scratch.values[243] != 0.0)) {
            scratch.store_ad(237, &AdValue::exp(AdValue::mul(scratch.ad_value(66), scratch.ad_value(236))));
        }

        if (((scratch.values[233] != 0.0) && (scratch.values[242] != 0.0)) && (!(scratch.values[243] != 0.0))) {
            scratch.store_ad(237, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(60), scratch.ad_value(236))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(66), scratch.ad_value(60)), scratch.ad_value(236)), 1.0)));
        }

        if ((scratch.values[233] != 0.0) && (scratch.values[242] != 0.0)) {
            scratch.store_ad(239, &AdValue::mul(scratch.ad_value(235), AdValue::offset(scratch.ad_value(237), (-1.0))));
        }

        if ((scratch.values[233] != 0.0) && (!(scratch.values[242] != 0.0))) {
            scratch.values[239] = 0.0;
            scratch.node_derivatives[239] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[239] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[233] != 0.0) {
            scratch.store_ad(91, &AdValue::add(scratch.ad_value(238), scratch.ad_value(239)));
        }

        scratch.values[247] = if (scratch.values[103] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[233] != 0.0) && (scratch.values[247] != 0.0)) {
            scratch.store_ad(244, &AdValue::sub(AdValue::neg(scratch.ad_value(103)), scratch.ad_value(66)));
        }

        if ((scratch.values[233] != 0.0) && (scratch.values[247] != 0.0)) {
            scratch.store_ad(245, &AdValue::div_from_scalar(1.0, AdValue::mul(scratch.ad_value(104), scratch.ad_value(70))));
        }

        scratch.values[248] = if (scratch.values[244] < scratch.values[62]) { 1.0 } else { 0.0 };

        if (((scratch.values[233] != 0.0) && (scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(246, &AdValue::exp(AdValue::mul(scratch.ad_value(244), scratch.ad_value(245))));
        }

        if (((scratch.values[233] != 0.0) && (scratch.values[247] != 0.0)) && (!(scratch.values[248] != 0.0))) {
            scratch.store_ad(246, &AdValue::mul(AdValue::exp(AdValue::mul(scratch.ad_value(62), scratch.ad_value(245))), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(244), scratch.ad_value(62)), scratch.ad_value(245)), 1.0)));
        }

        if ((scratch.values[233] != 0.0) && (scratch.values[247] != 0.0)) {
            scratch.store_ad(93, &AdValue::scale(AdValue::sub(scratch.ad_value(246), AdValue::exp(AdValue::mul(AdValue::neg(scratch.ad_value(103)), scratch.ad_value(245)))), (-self.params.ibv)));
        }

        if ((scratch.values[233] != 0.0) && (!(scratch.values[247] != 0.0))) {
            scratch.values[93] = 0.0;
            scratch.node_derivatives[93] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[93] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[233] != 0.0) {
            scratch.store_ad(83, &AdValue::add(AdValue::add(scratch.ad_value(91), scratch.ad_value(93)), AdValue::scale(scratch.ad_value(66), scratch.values[11])));
        }

        if (!(scratch.values[233] != 0.0)) {
            scratch.values[91] = 0.0;
            scratch.node_derivatives[91] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[91] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[233] != 0.0)) {
            scratch.values[93] = 0.0;
            scratch.node_derivatives[93] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[93] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[233] != 0.0)) {
            scratch.values[83] = 0.0;
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(2, &AdValue::add(AdValue::add(AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(81), scratch.ad_value(64)), AdValue::mul(scratch.ad_value(82), scratch.ad_value(65))), AdValue::mul(scratch.ad_value(83), scratch.ad_value(66))), AdValue::mul(AdValue::branch_current(ctx, &self.branches, 0), AdValue::voltage(ctx, &self.nodes, Some(0), Some(4)))), AdValue::mul(AdValue::branch_current(ctx, &self.branches, 1), AdValue::voltage(ctx, &self.nodes, Some(2), Some(5)))));

        scratch.values[249] = if (((scratch.values[47] > 0.0) && (self.params.sw_et != 0.0)) && (!(self.params.sw_lin != 0.0))) { 1.0 } else { 0.0 };

        if (scratch.values[249] != 0.0) {
            scratch.store_ad(94, &AdValue::neg(scratch.ad_value(2)));
        }

        scratch.values[250] = if (self.params.tegth == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[249] != 0.0) && (scratch.values[250] != 0.0)) {
            scratch.store_ad(95, &AdValue::mul(scratch.ad_value(47), scratch.ad_value(10)));
        }

        if ((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) {
            scratch.values[17] = ((ctx.temperature() + self.params.trise) - 273.15);
            scratch.node_derivatives[17] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[17] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[251] = if (scratch.values[17] < (self.params.tminclip + 1.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(17, &AdValue::offset(AdValue::exp(AdValue::offset(AdValue::offset(scratch.ad_value(17), (-self.params.tminclip)), (-1.0))), self.params.tminclip));
        }

        scratch.values[252] = if (scratch.values[17] > (self.params.tmaxclip - 1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) && (!(scratch.values[251] != 0.0))) && (scratch.values[252] != 0.0)) {
            scratch.store_ad(17, &AdValue::sub_from_scalar(self.params.tmaxclip, AdValue::exp(AdValue::offset(AdValue::sub_from_scalar(self.params.tmaxclip, scratch.ad_value(17)), (-1.0)))));
        }

        if ((((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) && (!(scratch.values[251] != 0.0))) && (!(scratch.values[252] != 0.0))) {
        }

        if ((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) {
            scratch.store_ad(16, &AdValue::offset(scratch.ad_value(17), 273.15));
        }

        scratch.values[253] = if ((((self.params.tegth + 1.0)) as f64).abs() > 0.1) { 1.0 } else { 0.0 };

        if (((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) && (scratch.values[253] != 0.0)) {
            scratch.store_ad(95, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(47), scratch.ad_value(16)), AdValue::offset(AdValue::powf(AdValue::offset(AdValue::div(scratch.ad_value(10), scratch.ad_value(16)), 1.0), (1.0 + self.params.tegth)), (-1.0))), 1.0 / ((1.0 + self.params.tegth))));
        }

        if (((scratch.values[249] != 0.0) && (!(scratch.values[250] != 0.0))) && (!(scratch.values[253] != 0.0))) {
            scratch.store_ad(95, &AdValue::mul(AdValue::mul(scratch.ad_value(47), scratch.ad_value(10)), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(10), (0.5 * self.params.tegth)), scratch.ad_value(16)), 1.0)));
        }

        if (!(scratch.values[249] != 0.0)) {
            scratch.values[94] = 0.0;
            scratch.node_derivatives[94] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[94] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[249] != 0.0)) {
            scratch.store_ad(95, &AdValue::scale(scratch.ad_value(10), 1000000.0));
        }

        scratch.store_ad(81, &AdValue::scale(scratch.ad_value(81), (-self.params.type_)));

        scratch.store_ad(82, &AdValue::scale(scratch.ad_value(82), (-self.params.type_)));

        scratch.store_ad(83, &AdValue::scale(scratch.ad_value(83), (-self.params.type_)));

        scratch.values[259] = if (scratch.values[86] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[259] != 0.0) && (self.params.sw_accpo != 0.0)) {
            scratch.store_ad(67, &AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(65), scratch.ad_value(48)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(65), scratch.ad_value(48)), AdValue::add(scratch.ad_value(65), scratch.ad_value(48))), 0.04))), 0.5));
        }

        if ((scratch.values[259] != 0.0) && (!(self.params.sw_accpo != 0.0))) {
            scratch.values[67] = scratch.values[65];
            scratch.node_derivatives[67] = scratch.node_derivatives[65];
            scratch.branch_derivatives[67] = scratch.branch_derivatives[65];
        }

        if (scratch.values[259] != 0.0) {
            scratch.store_ad(260, &AdValue::scale(scratch.ad_value(77), scratch.values[31]));
        }

        if (scratch.values[259] != 0.0) {
            scratch.store_ad(261, &AdValue::scale(scratch.ad_value(79), scratch.values[32]));
        }

        scratch.values[264] = if (scratch.values[260] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) {
            scratch.store_ad(265, &AdValue::scale(AdValue::neg(scratch.ad_value(76)), self.params.fc));
        }

        scratch.values[275] = if (self.params.aja <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) {
            scratch.store_ad(266, &AdValue::add(scratch.ad_value(67), scratch.ad_value(265)));
        }

        scratch.values[276] = if (scratch.values[266] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) && (scratch.values[276] != 0.0)) {
            scratch.values[267] = (((1.0 - self.params.fc)) as f64).powf((-self.params.ma));
            scratch.node_derivatives[267] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[267] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) && (scratch.values[276] != 0.0)) {
            scratch.store_ad(268, &AdValue::scale(AdValue::mul(scratch.ad_value(76), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(267), (1.0 - self.params.fc)))), 1.0 / ((1.0 - self.params.ma))));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) && (scratch.values[276] != 0.0)) {
            scratch.store_ad(269, &AdValue::mul(AdValue::mul(scratch.ad_value(266), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(266), (0.5 * self.params.ma)), AdValue::scale(scratch.ad_value(76), (1.0 - self.params.fc))), 1.0)), scratch.ad_value(267)));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) && (!(scratch.values[276] != 0.0))) {
            scratch.store_ad(268, &AdValue::scale(AdValue::mul(scratch.ad_value(76), AdValue::sub_from_scalar(1.0, AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(67), scratch.ad_value(76))), (1.0 - self.params.ma)))), 1.0 / ((1.0 - self.params.ma))));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) && (!(scratch.values[276] != 0.0))) {
            scratch.values[269] = 0.0;
            scratch.node_derivatives[269] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[269] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (scratch.values[275] != 0.0)) {
            scratch.store_ad(262, &AdValue::add(scratch.ad_value(268), scratch.ad_value(269)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(270, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(265)), ((4.0 * self.params.aja) * self.params.aja))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(271, &AdValue::scale(AdValue::add(scratch.ad_value(265), scratch.ad_value(270)), (-0.5)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(272, &AdValue::add(scratch.ad_value(67), scratch.ad_value(265)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(273, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(272)), ((4.0 * self.params.aja) * self.params.aja))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(274, &AdValue::sub(AdValue::scale(AdValue::sub(scratch.ad_value(272), scratch.ad_value(273)), 0.5), scratch.ad_value(265)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(268, &AdValue::scale(AdValue::mul(AdValue::neg(scratch.ad_value(76)), AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(274), scratch.ad_value(76))), (1.0 - self.params.ma))), 1.0 / ((1.0 - self.params.ma))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[264] != 0.0)) && (!(scratch.values[275] != 0.0))) {
            scratch.store_ad(262, &AdValue::add(scratch.ad_value(268), AdValue::mul(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(274)), scratch.ad_value(271)), (((1.0 - self.params.fc)) as f64).powf((-self.params.ma))), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(274)), scratch.ad_value(271)), (0.5 * self.params.ma)), AdValue::scale(scratch.ad_value(76), (1.0 - self.params.fc))), 1.0))));
        }

        if ((scratch.values[259] != 0.0) && (!(scratch.values[264] != 0.0))) {
            scratch.values[262] = 0.0;
            scratch.node_derivatives[262] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[262] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[277] = if (scratch.values[261] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) {
            scratch.store_ad(278, &AdValue::scale(AdValue::neg(scratch.ad_value(78)), self.params.fc));
        }

        scratch.values[288] = if (self.params.ajp <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) {
            scratch.store_ad(279, &AdValue::add(scratch.ad_value(67), scratch.ad_value(278)));
        }

        scratch.values[289] = if (scratch.values[279] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) && (scratch.values[289] != 0.0)) {
            scratch.values[280] = (((1.0 - self.params.fc)) as f64).powf((-self.params.mp));
            scratch.node_derivatives[280] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[280] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) && (scratch.values[289] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::mul(scratch.ad_value(78), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(280), (1.0 - self.params.fc)))), 1.0 / ((1.0 - self.params.mp))));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) && (scratch.values[289] != 0.0)) {
            scratch.store_ad(282, &AdValue::mul(AdValue::mul(scratch.ad_value(279), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(279), (0.5 * self.params.mp)), AdValue::scale(scratch.ad_value(78), (1.0 - self.params.fc))), 1.0)), scratch.ad_value(280)));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) && (!(scratch.values[289] != 0.0))) {
            scratch.store_ad(281, &AdValue::scale(AdValue::mul(scratch.ad_value(78), AdValue::sub_from_scalar(1.0, AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(67), scratch.ad_value(78))), (1.0 - self.params.mp)))), 1.0 / ((1.0 - self.params.mp))));
        }

        if ((((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) && (!(scratch.values[289] != 0.0))) {
            scratch.values[282] = 0.0;
            scratch.node_derivatives[282] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[282] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (scratch.values[288] != 0.0)) {
            scratch.store_ad(263, &AdValue::add(scratch.ad_value(281), scratch.ad_value(282)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(278)), ((4.0 * self.params.ajp) * self.params.ajp))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(284, &AdValue::scale(AdValue::add(scratch.ad_value(278), scratch.ad_value(283)), (-0.5)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(285, &AdValue::add(scratch.ad_value(67), scratch.ad_value(278)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(286, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(285)), ((4.0 * self.params.ajp) * self.params.ajp))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(287, &AdValue::sub(AdValue::scale(AdValue::sub(scratch.ad_value(285), scratch.ad_value(286)), 0.5), scratch.ad_value(278)));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(281, &AdValue::scale(AdValue::mul(AdValue::neg(scratch.ad_value(78)), AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(287), scratch.ad_value(78))), (1.0 - self.params.mp))), 1.0 / ((1.0 - self.params.mp))));
        }

        if (((scratch.values[259] != 0.0) && (scratch.values[277] != 0.0)) && (!(scratch.values[288] != 0.0))) {
            scratch.store_ad(263, &AdValue::add(scratch.ad_value(281), AdValue::mul(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(287)), scratch.ad_value(284)), (((1.0 - self.params.fc)) as f64).powf((-self.params.mp))), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(287)), scratch.ad_value(284)), (0.5 * self.params.mp)), AdValue::scale(scratch.ad_value(78), (1.0 - self.params.fc))), 1.0))));
        }

        if ((scratch.values[259] != 0.0) && (!(scratch.values[277] != 0.0))) {
            scratch.values[263] = 0.0;
            scratch.node_derivatives[263] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[263] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[259] != 0.0) {
            scratch.store_ad(96, &AdValue::add(AdValue::mul(scratch.ad_value(260), scratch.ad_value(262)), AdValue::mul(scratch.ad_value(261), scratch.ad_value(263))));
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (!(scratch.values[259] != 0.0)) {
            scratch.values[96] = 0.0;
            scratch.node_derivatives[96] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[96] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[290] = if (scratch.values[87] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[290] != 0.0) && (self.params.sw_accpo != 0.0)) {
            scratch.store_ad(67, &AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(66), scratch.ad_value(48)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(66), scratch.ad_value(48)), AdValue::add(scratch.ad_value(66), scratch.ad_value(48))), 0.04))), 0.5));
        }

        if ((scratch.values[290] != 0.0) && (!(self.params.sw_accpo != 0.0))) {
            scratch.values[67] = scratch.values[66];
            scratch.node_derivatives[67] = scratch.node_derivatives[66];
            scratch.branch_derivatives[67] = scratch.branch_derivatives[66];
        }

        if (scratch.values[290] != 0.0) {
            scratch.store_ad(291, &AdValue::scale(scratch.ad_value(77), scratch.values[33]));
        }

        if (scratch.values[290] != 0.0) {
            scratch.store_ad(292, &AdValue::scale(scratch.ad_value(79), scratch.values[34]));
        }

        scratch.values[295] = if (scratch.values[291] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) {
            scratch.store_ad(296, &AdValue::scale(AdValue::neg(scratch.ad_value(76)), self.params.fc));
        }

        scratch.values[306] = if (self.params.aja <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) {
            scratch.store_ad(297, &AdValue::add(scratch.ad_value(67), scratch.ad_value(296)));
        }

        scratch.values[307] = if (scratch.values[297] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) && (scratch.values[307] != 0.0)) {
            scratch.values[298] = (((1.0 - self.params.fc)) as f64).powf((-self.params.ma));
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) && (scratch.values[307] != 0.0)) {
            scratch.store_ad(299, &AdValue::scale(AdValue::mul(scratch.ad_value(76), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(298), (1.0 - self.params.fc)))), 1.0 / ((1.0 - self.params.ma))));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) && (scratch.values[307] != 0.0)) {
            scratch.store_ad(300, &AdValue::mul(AdValue::mul(scratch.ad_value(297), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(297), (0.5 * self.params.ma)), AdValue::scale(scratch.ad_value(76), (1.0 - self.params.fc))), 1.0)), scratch.ad_value(298)));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) && (!(scratch.values[307] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::mul(scratch.ad_value(76), AdValue::sub_from_scalar(1.0, AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(67), scratch.ad_value(76))), (1.0 - self.params.ma)))), 1.0 / ((1.0 - self.params.ma))));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) && (!(scratch.values[307] != 0.0))) {
            scratch.values[300] = 0.0;
            scratch.node_derivatives[300] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[300] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (scratch.values[306] != 0.0)) {
            scratch.store_ad(293, &AdValue::add(scratch.ad_value(299), scratch.ad_value(300)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(301, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(296)), ((4.0 * self.params.aja) * self.params.aja))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(302, &AdValue::scale(AdValue::add(scratch.ad_value(296), scratch.ad_value(301)), (-0.5)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(303, &AdValue::add(scratch.ad_value(67), scratch.ad_value(296)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(304, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(303)), ((4.0 * self.params.aja) * self.params.aja))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(305, &AdValue::sub(AdValue::scale(AdValue::sub(scratch.ad_value(303), scratch.ad_value(304)), 0.5), scratch.ad_value(296)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::mul(AdValue::neg(scratch.ad_value(76)), AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(305), scratch.ad_value(76))), (1.0 - self.params.ma))), 1.0 / ((1.0 - self.params.ma))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[295] != 0.0)) && (!(scratch.values[306] != 0.0))) {
            scratch.store_ad(293, &AdValue::add(scratch.ad_value(299), AdValue::mul(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(305)), scratch.ad_value(302)), (((1.0 - self.params.fc)) as f64).powf((-self.params.ma))), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(305)), scratch.ad_value(302)), (0.5 * self.params.ma)), AdValue::scale(scratch.ad_value(76), (1.0 - self.params.fc))), 1.0))));
        }

        if ((scratch.values[290] != 0.0) && (!(scratch.values[295] != 0.0))) {
            scratch.values[293] = 0.0;
            scratch.node_derivatives[293] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[293] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[308] = if (scratch.values[292] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) {
            scratch.store_ad(309, &AdValue::scale(AdValue::neg(scratch.ad_value(78)), self.params.fc));
        }

        scratch.values[319] = if (self.params.ajp <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) {
            scratch.store_ad(310, &AdValue::add(scratch.ad_value(67), scratch.ad_value(309)));
        }

        scratch.values[320] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) && (scratch.values[320] != 0.0)) {
            scratch.values[311] = (((1.0 - self.params.fc)) as f64).powf((-self.params.mp));
            scratch.node_derivatives[311] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[311] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) && (scratch.values[320] != 0.0)) {
            scratch.store_ad(312, &AdValue::scale(AdValue::mul(scratch.ad_value(78), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(311), (1.0 - self.params.fc)))), 1.0 / ((1.0 - self.params.mp))));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) && (scratch.values[320] != 0.0)) {
            scratch.store_ad(313, &AdValue::mul(AdValue::mul(scratch.ad_value(310), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(310), (0.5 * self.params.mp)), AdValue::scale(scratch.ad_value(78), (1.0 - self.params.fc))), 1.0)), scratch.ad_value(311)));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) && (!(scratch.values[320] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::mul(scratch.ad_value(78), AdValue::sub_from_scalar(1.0, AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(67), scratch.ad_value(78))), (1.0 - self.params.mp)))), 1.0 / ((1.0 - self.params.mp))));
        }

        if ((((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) && (!(scratch.values[320] != 0.0))) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (scratch.values[319] != 0.0)) {
            scratch.store_ad(294, &AdValue::add(scratch.ad_value(312), scratch.ad_value(313)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(314, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(309)), ((4.0 * self.params.ajp) * self.params.ajp))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(315, &AdValue::scale(AdValue::add(scratch.ad_value(309), scratch.ad_value(314)), (-0.5)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(316, &AdValue::add(scratch.ad_value(67), scratch.ad_value(309)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(317, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(316)), ((4.0 * self.params.ajp) * self.params.ajp))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(318, &AdValue::sub(AdValue::scale(AdValue::sub(scratch.ad_value(316), scratch.ad_value(317)), 0.5), scratch.ad_value(309)));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::mul(AdValue::neg(scratch.ad_value(78)), AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(318), scratch.ad_value(78))), (1.0 - self.params.mp))), 1.0 / ((1.0 - self.params.mp))));
        }

        if (((scratch.values[290] != 0.0) && (scratch.values[308] != 0.0)) && (!(scratch.values[319] != 0.0))) {
            scratch.store_ad(294, &AdValue::add(scratch.ad_value(312), AdValue::mul(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(318)), scratch.ad_value(315)), (((1.0 - self.params.fc)) as f64).powf((-self.params.mp))), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::sub(scratch.ad_value(67), scratch.ad_value(318)), scratch.ad_value(315)), (0.5 * self.params.mp)), AdValue::scale(scratch.ad_value(78), (1.0 - self.params.fc))), 1.0))));
        }

        if ((scratch.values[290] != 0.0) && (!(scratch.values[308] != 0.0))) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[290] != 0.0) {
            scratch.store_ad(97, &AdValue::add(AdValue::mul(scratch.ad_value(291), scratch.ad_value(293)), AdValue::mul(scratch.ad_value(292), scratch.ad_value(294))));
        }

        if (!(scratch.values[290] != 0.0)) {
            scratch.values[97] = 0.0;
            scratch.node_derivatives[97] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[97] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(96, &AdValue::add(scratch.ad_value(96), AdValue::scale(scratch.ad_value(65), scratch.values[88])));

        scratch.store_ad(97, &AdValue::add(scratch.ad_value(97), AdValue::scale(scratch.ad_value(66), scratch.values[89])));

        scratch.store_ad(96, &AdValue::scale(scratch.ad_value(96), (-self.params.type_)));

        scratch.store_ad(97, &AdValue::scale(scratch.ad_value(97), (-self.params.type_)));

        scratch.store_ad(98, &AdValue::mul(scratch.ad_value(10), scratch.ad_value(9)));

        scratch.values[321] = if ((scratch.values[54] / scratch.values[12]) <= self.params.rthresh) { 1.0 } else { 0.0 };

        scratch.values[322] = if ((scratch.values[55] / scratch.values[12]) <= self.params.rthresh) { 1.0 } else { 0.0 };

        if ((self.params.sw_noise != 0.0) && (self.params.sw_fngeo != 0.0)) {
            scratch.values[37] = scratch.values[3];
            scratch.node_derivatives[37] = scratch.node_derivatives[3];
            scratch.branch_derivatives[37] = scratch.branch_derivatives[3];
        }

        if ((self.params.sw_noise != 0.0) && (self.params.sw_fngeo != 0.0)) {
            scratch.values[38] = scratch.values[4];
            scratch.node_derivatives[38] = scratch.node_derivatives[4];
            scratch.branch_derivatives[38] = scratch.branch_derivatives[4];
        }

        if ((self.params.sw_noise != 0.0) && (!(self.params.sw_fngeo != 0.0))) {
            scratch.values[37] = scratch.values[27];
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((self.params.sw_noise != 0.0) && (!(self.params.sw_fngeo != 0.0))) {
            scratch.values[38] = scratch.values[26];
            scratch.node_derivatives[38] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[38] = [0.0; Instance::BRANCH_COUNT];
        }

        if (self.params.sw_noise != 0.0) {
            scratch.store_ad(99, &AdValue::mul(AdValue::scale(scratch.ad_value(24), (4.0 * 1.3806505e-23)), scratch.ad_value(63)));
        }

        if (self.params.sw_noise != 0.0) {
            scratch.store_ad(100, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(80), AdValue::powf(AdValue::abs(AdValue::div(scratch.ad_value(81), scratch.ad_value(38))), self.params.afn)), scratch.ad_value(38)), scratch.ad_value(37)));
        }

        scratch.values[323] = if (scratch.values[81] < 0.0) { 1.0 } else { 0.0 };

        if ((self.params.sw_noise != 0.0) && (scratch.values[323] != 0.0)) {
            scratch.store_ad(100, &AdValue::neg(scratch.ad_value(100)));
        }

        scratch.values[324] = if (scratch.values[54] > 0.0) { 1.0 } else { 0.0 };

        if ((self.params.sw_noise != 0.0) && (scratch.values[324] != 0.0)) {
            scratch.store_ad(56, &AdValue::div_from_scalar(1.0, AdValue::mul(scratch.ad_value(54), scratch.ad_value(58))));
        }

        if ((self.params.sw_noise != 0.0) && (!(scratch.values[324] != 0.0))) {
            scratch.values[56] = 0.0;
            scratch.node_derivatives[56] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[56] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[325] = if (scratch.values[55] > 0.0) { 1.0 } else { 0.0 };

        if ((self.params.sw_noise != 0.0) && (scratch.values[325] != 0.0)) {
            scratch.store_ad(56, &AdValue::div_from_scalar(1.0, AdValue::mul(scratch.ad_value(55), scratch.ad_value(58))));
        }

        if ((self.params.sw_noise != 0.0) && (!(scratch.values[325] != 0.0))) {
            scratch.values[56] = 0.0;
            scratch.node_derivatives[56] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[56] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[326] = if (scratch.values[84] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[327] = if (scratch.values[85] > 0.0) { 1.0 } else { 0.0 };

    }
}
