#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[199] = (11.7 * 8.8541879239442e-12);

        scratch.values[157] = 0.0;

        scratch.values[6] = 0.0;

        scratch.values[175] = 0.0;

        scratch.values[31] = (scratch.values[199] / self.params.cox);

        scratch.values[34] = (((scratch.values[31] * self.params.xj)) as f64).sqrt();

        scratch.values[35] = (scratch.values[34] * self.params.lambda);

        scratch.values[32] = ((3.0 * scratch.values[31]) * self.params.weta);

        scratch.values[33] = (scratch.values[31] * self.params.leta);

        scratch.values[36] = (self.params.ibn + self.params.ibn);

        scratch.values[37] = (self.params.cox / (scratch.values[199] * self.params.e0));

        scratch.values[182] = ((self.params.q0 + self.params.q0) / self.params.cox);

        scratch.values[39] = (if (self.params.type_ > 0.0) { 0.5 } else { 0.3333333333333 });

        scratch.values[238] = if (self.params.temp == (-(-1e21))) { 1.0 } else { 0.0 };

        if (scratch.values[238] != 0.0) {
            scratch.values[49] = (ctx.temperature() + self.params.trise);
            scratch.node_derivatives[49] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[49] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[238] != 0.0)) {
            scratch.values[49] = (self.params.temp + 273.15);
            scratch.node_derivatives[49] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[49] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[239] = if (self.params.tnom == (-(-1e21))) { 1.0 } else { 0.0 };

        if (scratch.values[239] != 0.0) {
            scratch.values[55] = (25.0 + 273.15);
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[239] != 0.0)) {
            scratch.values[55] = (self.params.tnom + 273.15);
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(17, &AdValue::scale(scratch.ad_value(49), THERMAL_VOLTAGE_PER_K));

        scratch.store_ad(25, &AdValue::scale(scratch.ad_value(17), 0.1));

        scratch.store_ad(24, &AdValue::div_from_scalar(1.0, scratch.ad_value(17)));

        scratch.store_ad(26, &AdValue::scale(scratch.ad_value(17), 2.0));

        scratch.store_ad(27, &AdValue::scale(scratch.ad_value(26), 2.0));

        scratch.store_ad(28, &AdValue::square(scratch.ad_value(17)));

        scratch.store_ad(29, &AdValue::scale(scratch.ad_value(28), 2.0));

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(28), 16.0));

        scratch.store_ad(51, &AdValue::sub_from_scalar(1.16, AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(49), 0.000702), scratch.ad_value(49)), AdValue::offset(scratch.ad_value(49), 1108.0))));

        scratch.store_ad(52, &AdValue::sub_from_scalar(1.16, AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(55), 0.000702), scratch.ad_value(55)), AdValue::offset(scratch.ad_value(55), 1108.0))));

        scratch.store_ad(53, &AdValue::sub(scratch.ad_value(49), scratch.ad_value(55)));

        scratch.store_ad(54, &AdValue::div(scratch.ad_value(49), scratch.ad_value(55)));

        scratch.store_ad(56, &AdValue::sub_from_scalar(self.params.vto, AdValue::scale(scratch.ad_value(53), self.params.tcv)));

        scratch.store_ad(58, &AdValue::scale(AdValue::powf(scratch.ad_value(54), self.params.bex), self.params.kp));

        scratch.store_ad(59, &AdValue::scale(AdValue::powf(scratch.ad_value(54), self.params.ucex), self.params.ucrit));

        scratch.store_ad(60, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.ibbt), 1.0), self.params.ibb));

        scratch.store_ad(61, &AdValue::add(AdValue::sub(AdValue::sub(AdValue::scale(scratch.ad_value(54), self.params.phi), AdValue::mul(AdValue::scale(scratch.ad_value(17), 3.0), AdValue::ln(scratch.ad_value(54)))), AdValue::mul(scratch.ad_value(52), scratch.ad_value(54))), scratch.ad_value(51)));

        scratch.values[0] = 0.2;

        scratch.store_ad(1, &AdValue::offset(scratch.ad_value(61), (-scratch.values[0])));

        scratch.store_ad(61, &AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(1), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1)), AdValue::square(scratch.ad_value(17))))), 0.5), scratch.values[0]));

        scratch.store_ad(71, &AdValue::sqrt(scratch.ad_value(61)));

        scratch.store_ad(40, &AdValue::div_from_scalar(1.0, scratch.ad_value(59)));

        scratch.store_ad(41, &AdValue::scale(scratch.ad_value(59), scratch.values[34]));

        scratch.store_ad(42, &AdValue::scale(scratch.ad_value(60), scratch.values[34]));

        scratch.store_ad(43, &AdValue::div_from_scalar(self.params.iba, scratch.ad_value(60)));

        scratch.values[191] = (self.params.l + self.params.dl);

        scratch.values[192] = (self.params.w + self.params.dw);

        scratch.store_ad(158, &AdValue::scale(scratch.ad_value(59), scratch.values[191]));

        scratch.store_ad(173, &AdValue::mul(scratch.ad_value(17), AdValue::offset(AdValue::ln(AdValue::mul(AdValue::scale(scratch.ad_value(158), 0.5), scratch.ad_value(24))), (-0.6))));

        scratch.values[48] = (1.0 / (((scratch.values[192] * scratch.values[191])) as f64).sqrt());

        scratch.values[240] = if (self.params.type_ > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[240] != 0.0) {
            scratch.store_ad(57, &{
                if (self.params.avto != 1e-6) {
                    AdValue::offset(scratch.ad_value(56), (scratch.values[48] * (self.params.avto - 1e-6)))
                } else {
                    scratch.ad_value(56)
                }
            });
        }

        if (!(scratch.values[240] != 0.0)) {
            scratch.store_ad(57, &{
                if (self.params.avto != 1e-6) {
                    AdValue::sub_from_scalar((scratch.values[48] * (1e-6 - self.params.avto)), scratch.ad_value(56))
                } else {
                    AdValue::neg(scratch.ad_value(56))
                }
            });
        }

        scratch.store_ad(50, &AdValue::scale({
    if (self.params.akp != 1e-6) {
        AdValue::scale(scratch.ad_value(58), (1.0 + ((self.params.akp - 1e-6) * scratch.values[48])))
    } else {
        scratch.ad_value(58)
    }
}, scratch.values[192]));

        scratch.values[62] = (if (self.params.agamma != 1e-6) { (self.params.gamma + ((self.params.agamma - 1e-6) * scratch.values[48])) } else { self.params.gamma });

        scratch.store_ad(153, &AdValue::scale(scratch.ad_value(71), scratch.values[62]));

        scratch.values[241] = if (scratch.values[182] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[241] != 0.0) {
            scratch.values[183] = 0.0;
            scratch.node_derivatives[183] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[183] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.values[184] = (0.28 * ((scratch.values[191] / (self.params.lk * self.params.ns)) - 0.1));
            scratch.node_derivatives[184] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[184] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.store_ad(242, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(184), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(184)), 0.001936))), 0.5), 1.0)));
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.store_ad(183, &AdValue::mul(AdValue::scale(scratch.ad_value(242), scratch.values[182]), scratch.ad_value(242)));
        }

        scratch.store_ad(145, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(1), Some(3)), self.params.type_));

        scratch.store_ad(147, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(2), Some(3)), self.params.type_));

        scratch.store_ad(146, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(3)), self.params.type_));

        scratch.values[243] = if ((scratch.values[146] - scratch.values[147]) < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[243] != 0.0) {
            scratch.values[44] = (-1.0);
            scratch.node_derivatives[44] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[44] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[38] = scratch.values[147];
            scratch.node_derivatives[38] = scratch.node_derivatives[147];
            scratch.branch_derivatives[38] = scratch.branch_derivatives[147];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[147] = scratch.values[146];
            scratch.node_derivatives[147] = scratch.node_derivatives[146];
            scratch.branch_derivatives[147] = scratch.branch_derivatives[146];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[146] = scratch.values[38];
            scratch.node_derivatives[146] = scratch.node_derivatives[38];
            scratch.branch_derivatives[146] = scratch.branch_derivatives[38];
        }

        if (!(scratch.values[243] != 0.0)) {
            scratch.values[44] = 1.0;
            scratch.node_derivatives[44] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[44] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(143, &AdValue::add(AdValue::add(AdValue::sub(AdValue::sub(scratch.ad_value(145), scratch.ad_value(57)), scratch.ad_value(183)), scratch.ad_value(61)), scratch.ad_value(153)));

        scratch.store_ad(144, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(143)), AdValue::scale(scratch.ad_value(30), 2.0))));

        scratch.store_ad(3, &AdValue::scale(AdValue::add(scratch.ad_value(143), scratch.ad_value(144)), 0.5));

        scratch.store_ad(70, &AdValue::add(scratch.ad_value(61), scratch.ad_value(147)));

        scratch.store_ad(76, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(70)), scratch.ad_value(30))));

        scratch.store_ad(74, &AdValue::sqrt(AdValue::scale(AdValue::add(scratch.ad_value(70), scratch.ad_value(76)), 0.5)));

        scratch.store_ad(69, &AdValue::add(scratch.ad_value(61), scratch.ad_value(146)));

        scratch.store_ad(75, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(69)), scratch.ad_value(30))));

        scratch.store_ad(73, &AdValue::sqrt(AdValue::scale(AdValue::add(scratch.ad_value(69), scratch.ad_value(75)), 0.5)));

        scratch.values[45] = ((scratch.values[32] * self.params.m) / scratch.values[192]);

        scratch.values[46] = ((scratch.values[33] * self.params.ns) / scratch.values[191]);

        scratch.store_ad(67, &AdValue::sqrt(AdValue::offset(scratch.ad_value(3), ((0.25 * scratch.values[62]) * scratch.values[62]))));

        scratch.store_ad(68, &AdValue::sub(AdValue::sub(scratch.ad_value(3), scratch.ad_value(61)), AdValue::scale(AdValue::offset(scratch.ad_value(67), (-(0.5 * scratch.values[62]))), scratch.values[62])));

        scratch.store_ad(174, &AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(68), scratch.ad_value(61)), scratch.ad_value(25))));

        scratch.store_ad(64, &AdValue::add(AdValue::sub_from_scalar(scratch.values[62], AdValue::scale(AdValue::add(scratch.ad_value(74), scratch.ad_value(73)), scratch.values[46])), AdValue::scale(scratch.ad_value(174), scratch.values[45])));

        scratch.store_ad(65, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(64)), scratch.ad_value(25))));

        scratch.store_ad(4, &AdValue::scale(AdValue::add(scratch.ad_value(64), scratch.ad_value(65)), 0.5));

        scratch.store_ad(66, &AdValue::sqrt(AdValue::add(scratch.ad_value(3), AdValue::mul(AdValue::scale(scratch.ad_value(4), 0.25), scratch.ad_value(4)))));

        scratch.store_ad(5, &AdValue::sub(AdValue::sub(scratch.ad_value(3), scratch.ad_value(61)), AdValue::mul(scratch.ad_value(4), AdValue::sub(scratch.ad_value(66), AdValue::scale(scratch.ad_value(4), 0.5)))));

        scratch.store_ad(0, &AdValue::mul(AdValue::sub(scratch.ad_value(5), scratch.ad_value(147)), scratch.ad_value(24)));

        scratch.values[244] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[245] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[246] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[244] != 0.0)) && (!(scratch.values[245] != 0.0))) && (scratch.values[246] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[244] != 0.0)) && (!(scratch.values[245] != 0.0))) && (!(scratch.values[246] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(7, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.store_ad(87, &AdValue::sqrt(scratch.ad_value(7)));

        scratch.values[90] = scratch.values[195];
        scratch.node_derivatives[90] = scratch.node_derivatives[195];
        scratch.branch_derivatives[90] = scratch.branch_derivatives[195];

        scratch.store_ad(160, &AdValue::div(scratch.ad_value(17), scratch.ad_value(158)));

        scratch.store_ad(80, &AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(87), scratch.ad_value(160)), 0.25)));

        scratch.store_ad(10, &AdValue::mul(scratch.ad_value(158), AdValue::offset(scratch.ad_value(80), (-0.5))));

        scratch.store_ad(77, &AdValue::scale(AdValue::sub(scratch.ad_value(146), scratch.ad_value(147)), 0.5));

        scratch.store_ad(78, &AdValue::mul(scratch.ad_value(30), AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(87), AdValue::mul(scratch.ad_value(10), scratch.ad_value(24))), self.params.lambda), 0.015625)));

        scratch.store_ad(81, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(10)), scratch.ad_value(78))));

        scratch.store_ad(82, &AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(77), scratch.ad_value(10)), AdValue::sub(scratch.ad_value(77), scratch.ad_value(10))), scratch.ad_value(78))));

        scratch.store_ad(79, &AdValue::sub(scratch.ad_value(81), scratch.ad_value(82)));

        scratch.store_ad(83, &AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(87), AdValue::scale(AdValue::ln(scratch.ad_value(7)), 0.75)), scratch.ad_value(160)), 0.25)));

        scratch.store_ad(11, &AdValue::add(AdValue::mul(scratch.ad_value(158), AdValue::offset(scratch.ad_value(83), (-0.5))), scratch.ad_value(173)));

        scratch.store_ad(159, &AdValue::sub(scratch.ad_value(77), scratch.ad_value(11)));

        scratch.store_ad(84, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(11)), scratch.ad_value(78))));

        scratch.store_ad(85, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(159)), scratch.ad_value(78))));

        scratch.store_ad(0, &AdValue::mul(AdValue::add(AdValue::sub(AdValue::sub(AdValue::sub(scratch.ad_value(5), scratch.ad_value(77)), scratch.ad_value(147)), scratch.ad_value(84)), scratch.ad_value(85)), scratch.ad_value(24)));

        scratch.values[247] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[248] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[249] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[247] != 0.0)) && (!(scratch.values[248] != 0.0))) && (scratch.values[249] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[247] != 0.0)) && (!(scratch.values[248] != 0.0))) && (!(scratch.values[249] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(9, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.values[92] = scratch.values[195];
        scratch.node_derivatives[92] = scratch.node_derivatives[195];
        scratch.branch_derivatives[92] = scratch.branch_derivatives[195];

        scratch.store_ad(12, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(AdValue::sub(scratch.ad_value(77), scratch.ad_value(79)), scratch.ad_value(41)), 1.0)), scratch.values[35]));

        scratch.store_ad(155, &AdValue::add(AdValue::sub_from_scalar(scratch.values[191], scratch.ad_value(12)), AdValue::mul(AdValue::add(scratch.ad_value(77), scratch.ad_value(79)), scratch.ad_value(40))));

        scratch.values[154] = (0.1 * scratch.values[191]);

        scratch.store_ad(63, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(155)), (scratch.values[154] * scratch.values[154]))));

        scratch.store_ad(13, &AdValue::scale(AdValue::add(scratch.ad_value(155), scratch.ad_value(63)), 0.5));

        scratch.store_ad(0, &AdValue::mul(AdValue::sub(scratch.ad_value(5), scratch.ad_value(146)), scratch.ad_value(24)));

        scratch.values[250] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[251] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[252] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[250] != 0.0)) && (!(scratch.values[251] != 0.0))) && (scratch.values[252] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[250] != 0.0)) && (!(scratch.values[251] != 0.0))) && (!(scratch.values[252] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(8, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.values[91] = scratch.values[195];
        scratch.node_derivatives[91] = scratch.node_derivatives[195];
        scratch.branch_derivatives[91] = scratch.branch_derivatives[195];

        scratch.store_ad(95, &AdValue::offset(scratch.ad_value(7), 0.25));

        scratch.store_ad(96, &AdValue::offset(scratch.ad_value(8), 0.25));

        scratch.store_ad(93, &AdValue::sqrt(scratch.ad_value(95)));

        scratch.store_ad(94, &AdValue::sqrt(scratch.ad_value(96)));

        scratch.store_ad(99, &AdValue::mul(AdValue::add(scratch.ad_value(93), scratch.ad_value(94)), AdValue::add(scratch.ad_value(93), scratch.ad_value(94))));

        scratch.store_ad(107, &AdValue::offset(AdValue::add(scratch.ad_value(5), scratch.ad_value(61)), 1e-6));

        scratch.store_ad(108, &AdValue::scale(AdValue::sqrt(scratch.ad_value(107)), 2.0));

        scratch.store_ad(111, &AdValue::div_from_scalar(scratch.values[62], scratch.ad_value(108)));

        scratch.store_ad(112, &AdValue::div_from_scalar(scratch.values[62], AdValue::offset(scratch.ad_value(108), scratch.values[62])));

        scratch.store_ad(100, &AdValue::mul(AdValue::mul(AdValue::neg(AdValue::offset(scratch.ad_value(111), 1.0)), scratch.ad_value(17)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(96), AdValue::mul(scratch.ad_value(94), scratch.ad_value(93))), scratch.ad_value(95)), (0.66666666 + 0.66666666)), AdValue::add(scratch.ad_value(93), scratch.ad_value(94))), (-1.0))));

        scratch.store_ad(101, &AdValue::sub(AdValue::scale(scratch.ad_value(108), ((-0.5) * scratch.values[62])), AdValue::mul(scratch.ad_value(112), scratch.ad_value(100))));

        scratch.values[253] = if (self.params.e0 == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(175, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(5)), scratch.ad_value(29))));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(6, &AdValue::scale(AdValue::add(scratch.ad_value(5), scratch.ad_value(175)), 0.5));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(157, &AdValue::offset(AdValue::scale(scratch.ad_value(6), self.params.theta), 1.0));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(14, &AdValue::div(scratch.ad_value(50), AdValue::mul(scratch.ad_value(13), scratch.ad_value(157))));
        }

        scratch.values[254] = if ((scratch.values[101] + (scratch.values[39] * scratch.values[100])) > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[253] != 0.0)) && (scratch.values[254] != 0.0)) {
            scratch.store_ad(47, &AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(101), AdValue::scale(scratch.ad_value(100), scratch.values[39])), scratch.values[37]), 1.0));
        }

        if ((!(scratch.values[253] != 0.0)) && (!(scratch.values[254] != 0.0))) {
            scratch.store_ad(47, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::add(scratch.ad_value(101), AdValue::scale(scratch.ad_value(100), scratch.values[39])), scratch.values[37])));
        }

        if (!(scratch.values[253] != 0.0)) {
            scratch.store_ad(156, &AdValue::offset(AdValue::scale(scratch.ad_value(153), scratch.values[37]), 1.0));
        }

        if (!(scratch.values[253] != 0.0)) {
            scratch.store_ad(14, &AdValue::div(AdValue::mul(scratch.ad_value(50), scratch.ad_value(156)), AdValue::mul(scratch.ad_value(13), scratch.ad_value(47))));
        }

        scratch.store_ad(72, &AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(61), scratch.ad_value(5)), scratch.ad_value(27))));

        scratch.store_ad(15, &AdValue::offset(AdValue::div_from_scalar(scratch.values[62], AdValue::scale(scratch.ad_value(72), 2.0)), 1.0));

        scratch.store_ad(86, &AdValue::sub(scratch.ad_value(7), scratch.ad_value(9)));

        scratch.store_ad(16, &AdValue::mul(AdValue::mul(scratch.ad_value(29), scratch.ad_value(15)), scratch.ad_value(14)));

        scratch.store_ad(150, &AdValue::mul(scratch.ad_value(16), scratch.ad_value(86)));

        scratch.store_ad(152, &AdValue::mul(scratch.ad_value(14), AdValue::abs(scratch.ad_value(100))));

        scratch.store_ad(0, &AdValue::div(scratch.ad_value(4), AdValue::scale(scratch.ad_value(65), 2.0)));

        scratch.store_ad(1, &AdValue::div(scratch.ad_value(3), scratch.ad_value(144)));

        scratch.store_ad(161, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(0), (-scratch.values[46])), scratch.ad_value(73)), scratch.ad_value(75)));

        scratch.store_ad(163, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(0), (-scratch.values[46])), scratch.ad_value(74)), scratch.ad_value(76)));

        scratch.store_ad(162, &AdValue::mul(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(0), scratch.values[45]), AdValue::offset(scratch.ad_value(67), (-(0.5 * scratch.values[62])))), AdValue::mul(scratch.ad_value(67), scratch.ad_value(174))), scratch.ad_value(1)));

        scratch.store_ad(2, &AdValue::div(AdValue::add(scratch.ad_value(5), scratch.ad_value(61)), scratch.ad_value(66)));

        scratch.store_ad(113, &AdValue::mul(AdValue::neg(scratch.ad_value(2)), scratch.ad_value(161)));

        scratch.store_ad(115, &AdValue::mul(AdValue::neg(scratch.ad_value(2)), scratch.ad_value(163)));

        scratch.store_ad(114, &AdValue::add(AdValue::mul(AdValue::neg(scratch.ad_value(2)), scratch.ad_value(162)), AdValue::mul(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(4), AdValue::scale(scratch.ad_value(66), 2.0))), scratch.ad_value(1))));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(90), scratch.ad_value(24)));

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.store_ad(116, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));

        scratch.store_ad(117, &AdValue::mul(scratch.ad_value(0), AdValue::offset(scratch.ad_value(115), (-1.0))));

        scratch.store_ad(118, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)));

        scratch.store_ad(0, &AdValue::div(scratch.ad_value(17), AdValue::mul(AdValue::scale(scratch.ad_value(80), 4.0), scratch.ad_value(87))));

        scratch.store_ad(122, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(116)));

        scratch.store_ad(124, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(117)));

        scratch.store_ad(123, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(118)));

        scratch.store_ad(0, &AdValue::scale(scratch.ad_value(27), (2.0 * self.params.lambda)));

        scratch.store_ad(1, &AdValue::div(scratch.ad_value(17), AdValue::scale(scratch.ad_value(87), 2.0)));

        scratch.store_ad(125, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(116), scratch.ad_value(1)), scratch.ad_value(122))));

        scratch.store_ad(127, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(117), scratch.ad_value(1)), scratch.ad_value(124))));

        scratch.store_ad(126, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(118), scratch.ad_value(1)), scratch.ad_value(123))));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, scratch.ad_value(81)));

        scratch.store_ad(1, &AdValue::div_from_scalar(1.0, scratch.ad_value(82)));

        scratch.store_ad(2, &AdValue::sub(scratch.ad_value(77), scratch.ad_value(10)));

        scratch.store_ad(128, &AdValue::sub(AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(10), scratch.ad_value(122)), scratch.ad_value(125)), scratch.ad_value(0)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(2), AdValue::sub_from_scalar(0.5, scratch.ad_value(122))), scratch.ad_value(125)), scratch.ad_value(1))));

        scratch.store_ad(130, &AdValue::sub(AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(10), scratch.ad_value(124)), scratch.ad_value(127)), scratch.ad_value(0)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(2), AdValue::sub_from_scalar((-0.5), scratch.ad_value(124))), scratch.ad_value(127)), scratch.ad_value(1))));

        scratch.store_ad(129, &AdValue::sub(AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(10), scratch.ad_value(123)), scratch.ad_value(126)), scratch.ad_value(0)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(2), AdValue::neg(scratch.ad_value(123))), scratch.ad_value(126)), scratch.ad_value(1))));

        scratch.store_ad(0, &AdValue::div(AdValue::mul(scratch.ad_value(17), AdValue::offset(scratch.ad_value(87), (-1.5))), AdValue::mul(AdValue::scale(scratch.ad_value(83), 4.0), scratch.ad_value(7))));

        scratch.store_ad(131, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(116)));

        scratch.store_ad(133, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(117)));

        scratch.store_ad(132, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(118)));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(92), scratch.ad_value(24)));

        scratch.store_ad(1, &AdValue::div_from_scalar(1.0, scratch.ad_value(84)));

        scratch.store_ad(2, &AdValue::div_from_scalar(1.0, scratch.ad_value(85)));

        scratch.store_ad(134, &AdValue::mul(scratch.ad_value(0), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(113), (-0.5)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(11), scratch.ad_value(131)), scratch.ad_value(125)), scratch.ad_value(1))), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(159), AdValue::sub_from_scalar(0.5, scratch.ad_value(131))), scratch.ad_value(125)), scratch.ad_value(2)))));

        scratch.store_ad(136, &AdValue::mul(scratch.ad_value(0), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(115), (-0.5)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(11), scratch.ad_value(133)), scratch.ad_value(127)), scratch.ad_value(1))), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(159), AdValue::sub_from_scalar((-0.5), scratch.ad_value(133))), scratch.ad_value(127)), scratch.ad_value(2)))));

        scratch.store_ad(135, &AdValue::mul(scratch.ad_value(0), AdValue::add(AdValue::sub(scratch.ad_value(114), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(11), scratch.ad_value(132)), scratch.ad_value(126)), scratch.ad_value(1))), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(159), AdValue::neg(scratch.ad_value(132))), scratch.ad_value(126)), scratch.ad_value(2)))));

        scratch.store_ad(0, &AdValue::div_from_scalar(scratch.values[35], AdValue::sub(AdValue::add(scratch.ad_value(41), scratch.ad_value(77)), scratch.ad_value(79))));

        scratch.store_ad(167, &AdValue::mul(scratch.ad_value(0), AdValue::sub_from_scalar(0.5, scratch.ad_value(128))));

        scratch.store_ad(169, &AdValue::mul(scratch.ad_value(0), AdValue::sub_from_scalar((-0.5), scratch.ad_value(130))));

        scratch.store_ad(168, &AdValue::mul(AdValue::neg(scratch.ad_value(0)), scratch.ad_value(129)));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, scratch.ad_value(63)));

        scratch.store_ad(137, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(AdValue::offset(scratch.ad_value(128), 0.5), scratch.ad_value(40)), scratch.ad_value(167))));

        scratch.store_ad(139, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(AdValue::offset(scratch.ad_value(130), (-0.5)), scratch.ad_value(40)), scratch.ad_value(169))));

        scratch.store_ad(138, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(129), scratch.ad_value(40)), scratch.ad_value(168))));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(91), scratch.ad_value(24)));

        scratch.store_ad(119, &AdValue::mul(scratch.ad_value(0), AdValue::offset(scratch.ad_value(113), (-1.0))));

        scratch.store_ad(120, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));

        scratch.store_ad(121, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)));

        scratch.store_ad(0, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::neg(AdValue::offset(scratch.ad_value(111), 1.0)), scratch.ad_value(17)), 0.66666666), scratch.ad_value(99)));

        scratch.store_ad(1, &AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(93), AdValue::scale(scratch.ad_value(94), 2.0))));

        scratch.store_ad(2, &AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(94), AdValue::scale(scratch.ad_value(93), 2.0))));

        scratch.store_ad(0, &AdValue::div(AdValue::mul(AdValue::neg(scratch.ad_value(111)), scratch.ad_value(100)), AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(111), 2.0), scratch.ad_value(111)), scratch.ad_value(107))));

        scratch.store_ad(185, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)), AdValue::mul(scratch.ad_value(1), scratch.ad_value(116))), AdValue::mul(scratch.ad_value(2), scratch.ad_value(119))));

        scratch.store_ad(186, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)), AdValue::mul(scratch.ad_value(1), scratch.ad_value(117))), AdValue::mul(scratch.ad_value(2), scratch.ad_value(120))));

        scratch.store_ad(187, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)), AdValue::mul(scratch.ad_value(1), scratch.ad_value(118))), AdValue::mul(scratch.ad_value(2), scratch.ad_value(121))));

        scratch.store_ad(0, &AdValue::sub(AdValue::offset(scratch.ad_value(111), 1.0), AdValue::div(scratch.ad_value(100), AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(111), 1.0), 2.0), scratch.ad_value(107)))));

        scratch.store_ad(188, &AdValue::mul(AdValue::neg(scratch.ad_value(112)), AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)), scratch.ad_value(185))));

        scratch.store_ad(189, &AdValue::mul(AdValue::neg(scratch.ad_value(112)), AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)), scratch.ad_value(186))));

        scratch.store_ad(190, &AdValue::mul(AdValue::neg(scratch.ad_value(112)), AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)), scratch.ad_value(187))));

        scratch.values[255] = if (self.params.e0 == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(0, &AdValue::div(AdValue::scale(scratch.ad_value(6), self.params.theta), AdValue::mul(scratch.ad_value(157), scratch.ad_value(175))));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(164, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(166, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(165, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(140, &AdValue::sub(AdValue::neg(scratch.ad_value(137)), scratch.ad_value(164)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(142, &AdValue::sub(AdValue::neg(scratch.ad_value(139)), scratch.ad_value(166)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(141, &AdValue::sub(AdValue::neg(scratch.ad_value(138)), scratch.ad_value(165)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(0, &AdValue::div_from_scalar(scratch.values[37], scratch.ad_value(47)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(140, &AdValue::sub(AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(188), AdValue::scale(scratch.ad_value(185), scratch.values[39]))), scratch.ad_value(137)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(142, &AdValue::sub(AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(189), AdValue::scale(scratch.ad_value(186), scratch.values[39]))), scratch.ad_value(139)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(141, &AdValue::sub(AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(190), AdValue::scale(scratch.ad_value(187), scratch.values[39]))), scratch.ad_value(138)));
        }

        scratch.store_ad(0, &AdValue::div_from_scalar((-scratch.values[62]), AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(15), 4.0), scratch.ad_value(72)), AdValue::add(AdValue::add(scratch.ad_value(61), scratch.ad_value(5)), scratch.ad_value(27)))));

        scratch.store_ad(170, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));

        scratch.store_ad(172, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));

        scratch.store_ad(171, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(114)));

        scratch.store_ad(21, &AdValue::mul(scratch.ad_value(16), AdValue::sub(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(170), scratch.ad_value(140)), scratch.ad_value(86)), scratch.ad_value(116)), scratch.ad_value(134))));

        scratch.store_ad(19, &AdValue::mul(AdValue::neg(scratch.ad_value(16)), AdValue::sub(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(172), scratch.ad_value(142)), scratch.ad_value(86)), scratch.ad_value(117)), scratch.ad_value(136))));

        scratch.store_ad(18, &AdValue::mul(scratch.ad_value(16), AdValue::sub(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(171), scratch.ad_value(141)), scratch.ad_value(86)), scratch.ad_value(118)), scratch.ad_value(135))));

        scratch.values[193] = ((self.params.rsh * self.params.hdif) / (scratch.values[192] - self.params.dw));

        scratch.values[194] = ((self.params.rsh * self.params.hdif) / (scratch.values[192] - self.params.dw));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(19), scratch.values[193]), 1.0), AdValue::scale(scratch.ad_value(21), scratch.values[194]))));

        scratch.store_ad(150, &AdValue::mul(scratch.ad_value(150), scratch.ad_value(0)));

        scratch.store_ad(177, &AdValue::sub(AdValue::sub(scratch.ad_value(146), scratch.ad_value(147)), AdValue::scale(scratch.ad_value(10), scratch.values[36])));

        scratch.values[256] = if ((scratch.values[177] > 0.0) && (scratch.values[43] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[256] != 0.0) {
            scratch.store_ad(180, &AdValue::div_from_scalar(1.0, scratch.ad_value(177)));
        }

        if (scratch.values[256] != 0.0) {
            scratch.store_ad(176, &AdValue::mul(AdValue::neg(scratch.ad_value(42)), scratch.ad_value(180)));
        }

        scratch.values[257] = if (scratch.values[176] < (-35.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[256] != 0.0) && (scratch.values[257] != 0.0)) {
            scratch.values[176] = (-35.0);
            scratch.node_derivatives[176] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[176] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[256] != 0.0) {
            scratch.store_ad(179, &AdValue::exp(scratch.ad_value(176)));
        }

        if (scratch.values[256] != 0.0) {
            scratch.store_ad(22, &AdValue::mul(AdValue::mul(scratch.ad_value(43), scratch.ad_value(177)), scratch.ad_value(179)));
        }

        if (scratch.values[256] != 0.0) {
            scratch.store_ad(23, &AdValue::mul(scratch.ad_value(22), scratch.ad_value(150)));
        }

        if (!(scratch.values[256] != 0.0)) {
            scratch.values[176] = 0.0;
            scratch.node_derivatives[176] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[176] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[256] != 0.0)) {
            scratch.values[23] = 0.0;
            scratch.node_derivatives[23] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[23] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[109] = ((scratch.values[192] * scratch.values[191]) * self.params.cox);

        scratch.store_ad(97, &AdValue::mul(scratch.ad_value(93), scratch.ad_value(95)));

        scratch.store_ad(98, &AdValue::mul(scratch.ad_value(94), scratch.ad_value(96)));

        scratch.store_ad(0, &AdValue::sqrt(AdValue::add(scratch.ad_value(61), AdValue::scale(scratch.ad_value(5), 0.5))));

        scratch.store_ad(181, &AdValue::scale(scratch.ad_value(0), 2.0));

        scratch.store_ad(110, &AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(scratch.ad_value(4), scratch.ad_value(181)), 1.0), scratch.ad_value(17)), scratch.values[109]));

        scratch.store_ad(102, &AdValue::mul(AdValue::neg(scratch.ad_value(110)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(98), 3.0), AdValue::mul(AdValue::scale(scratch.ad_value(96), 6.0), scratch.ad_value(93))), AdValue::mul(AdValue::scale(scratch.ad_value(94), 4.0), scratch.ad_value(95))), AdValue::scale(scratch.ad_value(97), 2.0)), 0.266666666), scratch.ad_value(99)), (-0.5))));

        scratch.store_ad(103, &AdValue::mul(AdValue::neg(scratch.ad_value(110)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(97), 3.0), AdValue::mul(AdValue::scale(scratch.ad_value(95), 6.0), scratch.ad_value(94))), AdValue::mul(AdValue::scale(scratch.ad_value(93), 4.0), scratch.ad_value(96))), AdValue::scale(scratch.ad_value(98), 2.0)), 0.266666666), scratch.ad_value(99)), (-0.5))));

        scratch.store_ad(104, &AdValue::add(scratch.ad_value(103), scratch.ad_value(102)));

        scratch.store_ad(105, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(AdValue::mul(AdValue::scale(scratch.ad_value(4), (-0.5)), scratch.ad_value(108)), scratch.ad_value(3)), scratch.ad_value(143)), scratch.values[109]), AdValue::div(AdValue::mul(scratch.ad_value(104), scratch.ad_value(4)), AdValue::add(scratch.ad_value(4), scratch.ad_value(181)))));

        scratch.store_ad(106, &AdValue::sub(AdValue::neg(scratch.ad_value(104)), scratch.ad_value(105)));

        let assign2910_ad_e2277: AdValue = AdValue::ddt(scratch.ad_value(102), self.ddt_jacobian(1.0), self.eval_ddt(0, scratch.ad_value(102).value));
        scratch.store_ad(200, &assign2910_ad_e2277);

        let assign2920_ad_e2279: AdValue = AdValue::ddt(scratch.ad_value(103), self.ddt_jacobian(1.0), self.eval_ddt(1, scratch.ad_value(103).value));
        scratch.store_ad(201, &assign2920_ad_e2279);

        scratch.values[258] = if (scratch.values[44] == 1.0) { 1.0 } else { 0.0 };

        if (self.params.noise != 0.0) {
            scratch.store_ad(260, &AdValue::mul(AdValue::scale(scratch.ad_value(49), (4.0 * 1.3806226e-23)), scratch.ad_value(152)));
        }

        if (self.params.noise != 0.0) {
            scratch.store_ad(259, &AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(18), self.params.kf), scratch.ad_value(18)), 1.0 / ((((scratch.values[192] * self.params.ns) * scratch.values[191]) * self.params.cox))));
        }

        scratch.values[261] = if ((self.params.as_ == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[261] != 0.0) {
            scratch.values[202] = ((2.0 * self.params.hdif) * scratch.values[192]);
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[261] != 0.0)) {
            scratch.values[202] = self.params.as_;
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[262] = if ((self.params.ps == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[262] != 0.0) {
            scratch.values[204] = ((4.0 * self.params.hdif) + scratch.values[192]);
            scratch.node_derivatives[204] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[204] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[262] != 0.0)) {
            scratch.values[204] = self.params.ps;
            scratch.node_derivatives[204] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[204] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[263] = if ((self.params.ad == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[263] != 0.0) {
            scratch.values[203] = ((2.0 * self.params.hdif) * scratch.values[192]);
            scratch.node_derivatives[203] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[203] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[263] != 0.0)) {
            scratch.values[203] = self.params.ad;
            scratch.node_derivatives[203] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[203] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[264] = if ((self.params.pd == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[264] != 0.0) {
            scratch.values[205] = ((4.0 * self.params.hdif) + scratch.values[192]);
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[264] != 0.0)) {
            scratch.values[205] = self.params.pd;
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(208, &AdValue::exp(AdValue::scale(AdValue::add(AdValue::sub(AdValue::div(scratch.ad_value(52), AdValue::scale(scratch.ad_value(55), THERMAL_VOLTAGE_PER_K)), AdValue::div(scratch.ad_value(51), scratch.ad_value(17))), AdValue::scale(AdValue::ln(scratch.ad_value(54)), self.params.tp_xti)), 1.0 / (self.params.xd_n))));

        scratch.store_ad(210, &AdValue::scale(scratch.ad_value(208), self.params.xd_js));

        scratch.store_ad(211, &AdValue::scale(scratch.ad_value(208), self.params.xd_jsw));

        scratch.store_ad(212, &AdValue::scale(scratch.ad_value(208), self.params.xd_jswg));

        scratch.store_ad(213, &AdValue::sub_from_scalar(self.params.xd_pb, AdValue::scale(scratch.ad_value(53), self.params.tp_pb)));

        scratch.store_ad(214, &AdValue::sub_from_scalar(self.params.xd_pbsw, AdValue::scale(scratch.ad_value(53), self.params.tp_pbsw)));

        scratch.store_ad(215, &AdValue::sub_from_scalar(self.params.xd_pbswg, AdValue::scale(scratch.ad_value(53), self.params.tp_pbswg)));

        scratch.store_ad(216, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cj), 1.0), self.params.xd_cj));

        scratch.store_ad(217, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cjsw), 1.0), self.params.xd_cjsw));

        scratch.store_ad(218, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cjswg), 1.0), self.params.xd_cjswg));

        scratch.store_ad(219, &AdValue::scale(AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(54), (-1.0)), self.params.tp_njts), 1.0), self.params.xd_njts));

        scratch.store_ad(220, &AdValue::scale(AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(54), (-1.0)), self.params.tp_njtssw), 1.0), self.params.xd_njtssw));

        scratch.store_ad(221, &AdValue::scale(AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(54), (-1.0)), self.params.tp_njtsswg), 1.0), self.params.xd_njtsswg));

        scratch.store_ad(206, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(3)), self.params.type_));

        scratch.store_ad(207, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(2), Some(3)), self.params.type_));

        scratch.store_ad(222, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(210), scratch.ad_value(203)), AdValue::mul(scratch.ad_value(211), scratch.ad_value(205))), AdValue::scale(scratch.ad_value(212), scratch.values[192])));

        scratch.store_ad(223, &AdValue::div(AdValue::mul(AdValue::neg(scratch.ad_value(206)), scratch.ad_value(54)), AdValue::scale(scratch.ad_value(17), self.params.xd_n)));

        scratch.values[265] = if (scratch.values[223] < (-40.0)) { 1.0 } else { 0.0 };

        if (scratch.values[265] != 0.0) {
            scratch.values[223] = (-40.0);
            scratch.node_derivatives[223] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[223] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(209, &AdValue::div(AdValue::mul(AdValue::sub_from_scalar(self.params.xd_bv, scratch.ad_value(206)), scratch.ad_value(54)), AdValue::scale(scratch.ad_value(17), self.params.xd_n)));

        scratch.values[266] = if (scratch.values[209] > 70.0) { 1.0 } else { 0.0 };

        if (scratch.values[266] != 0.0) {
            scratch.values[226] = 1.0;
            scratch.node_derivatives[226] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[226] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[266] != 0.0)) {
            scratch.store_ad(226, &AdValue::offset(AdValue::scale(AdValue::exp(AdValue::neg(scratch.ad_value(209))), self.params.xd_xjbv), 1.0));
        }

        scratch.store_ad(228, &AdValue::mul(AdValue::scale(scratch.ad_value(212), (-scratch.values[192])), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(206), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(221))), self.params.xd_vtsswg), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(206), self.params.xd_vtsswg), 0.001))), (-1.0))));

        scratch.store_ad(228, &AdValue::sub(scratch.ad_value(228), AdValue::mul(AdValue::mul(scratch.ad_value(205), scratch.ad_value(211)), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(206), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(220))), self.params.xd_vtssw), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(206), self.params.xd_vtssw), 0.001))), (-1.0)))));

        scratch.store_ad(228, &AdValue::sub(scratch.ad_value(228), AdValue::mul(AdValue::mul(scratch.ad_value(203), scratch.ad_value(210)), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(206), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(219))), self.params.xd_vts), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(206), self.params.xd_vts), 0.001))), (-1.0)))));

        scratch.store_ad(224, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(210), scratch.ad_value(202)), AdValue::mul(scratch.ad_value(211), scratch.ad_value(204))), AdValue::scale(scratch.ad_value(212), scratch.values[192])));

        scratch.store_ad(225, &AdValue::div(AdValue::mul(AdValue::neg(scratch.ad_value(207)), scratch.ad_value(54)), AdValue::scale(scratch.ad_value(17), self.params.xd_n)));

        scratch.values[267] = if (scratch.values[225] < (-40.0)) { 1.0 } else { 0.0 };

        if (scratch.values[267] != 0.0) {
            scratch.values[225] = (-40.0);
            scratch.node_derivatives[225] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[225] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(209, &AdValue::div(AdValue::mul(AdValue::sub_from_scalar(self.params.xd_bv, scratch.ad_value(207)), scratch.ad_value(54)), AdValue::scale(scratch.ad_value(17), self.params.xd_n)));

        scratch.values[268] = if (scratch.values[209] > 70.0) { 1.0 } else { 0.0 };

        if (scratch.values[268] != 0.0) {
            scratch.values[227] = 1.0;
            scratch.node_derivatives[227] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[227] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[268] != 0.0)) {
            scratch.store_ad(227, &AdValue::offset(AdValue::scale(AdValue::exp(AdValue::neg(scratch.ad_value(209))), self.params.xd_xjbv), 1.0));
        }

        scratch.store_ad(229, &AdValue::mul(AdValue::scale(scratch.ad_value(212), (-scratch.values[192])), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(207), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(221))), self.params.xd_vtsswg), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(207), self.params.xd_vtsswg), 0.001))), (-1.0))));

        scratch.store_ad(229, &AdValue::sub(scratch.ad_value(229), AdValue::mul(AdValue::mul(scratch.ad_value(204), scratch.ad_value(211)), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(207), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(220))), self.params.xd_vtssw), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(207), self.params.xd_vtssw), 0.001))), (-1.0)))));

        scratch.store_ad(229, &AdValue::sub(scratch.ad_value(229), AdValue::mul(AdValue::mul(scratch.ad_value(202), scratch.ad_value(210)), AdValue::offset(AdValue::exp(AdValue::div(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(207), scratch.ad_value(54)), AdValue::mul(scratch.ad_value(17), scratch.ad_value(219))), self.params.xd_vts), AdValue::max_with_scalar(AdValue::offset(scratch.ad_value(207), self.params.xd_vts), 0.001))), (-1.0)))));

        scratch.values[269] = if (scratch.values[206] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(230, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(203)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(213)), 1.0)), (-self.params.xd_mj)))));
        }

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(231, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(205)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(214)), 1.0)), (-self.params.xd_mjsw)))));
        }

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(232, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(215)), 1.0)), (-self.params.xd_mjswg)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(230, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(203)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mj), scratch.ad_value(213)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(231, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(205)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mjsw), scratch.ad_value(214)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(232, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mjswg), scratch.ad_value(215)))));
        }

        scratch.store_ad(236, &AdValue::mul(AdValue::add(AdValue::add(scratch.ad_value(230), scratch.ad_value(231)), scratch.ad_value(232)), scratch.ad_value(206)));

        scratch.values[270] = if (scratch.values[207] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(233, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(202)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(213)), 1.0)), (-self.params.xd_mj)))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(234, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(204)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(214)), 1.0)), (-self.params.xd_mjsw)))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(235, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(215)), 1.0)), (-self.params.xd_mjswg)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(233, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(202)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mj), scratch.ad_value(213)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(234, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(204)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mjsw), scratch.ad_value(214)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(235, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mjswg), scratch.ad_value(215)))));
        }

        scratch.store_ad(237, &AdValue::mul(AdValue::add(AdValue::add(scratch.ad_value(233), scratch.ad_value(234)), scratch.ad_value(235)), scratch.ad_value(207)));

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[199] = (11.7 * 8.8541879239442e-12);

        scratch.values[157] = 0.0;

        scratch.values[6] = 0.0;

        scratch.values[175] = 0.0;

        scratch.values[31] = (scratch.values[199] / self.params.cox);

        scratch.values[34] = (((scratch.values[31] * self.params.xj)) as f64).sqrt();

        scratch.values[35] = (scratch.values[34] * self.params.lambda);

        scratch.values[32] = ((3.0 * scratch.values[31]) * self.params.weta);

        scratch.values[33] = (scratch.values[31] * self.params.leta);

        scratch.values[37] = (self.params.cox / (scratch.values[199] * self.params.e0));

        scratch.values[182] = ((self.params.q0 + self.params.q0) / self.params.cox);

        scratch.values[39] = (if (self.params.type_ > 0.0) { 0.5 } else { 0.3333333333333 });

        scratch.values[238] = if (self.params.temp == (-(-1e21))) { 1.0 } else { 0.0 };

        if (scratch.values[238] != 0.0) {
            scratch.values[49] = (ctx.temperature() + self.params.trise);
            scratch.node_derivatives[49] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[49] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[238] != 0.0)) {
            scratch.values[49] = (self.params.temp + 273.15);
            scratch.node_derivatives[49] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[49] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[239] = if (self.params.tnom == (-(-1e21))) { 1.0 } else { 0.0 };

        if (scratch.values[239] != 0.0) {
            scratch.values[55] = (25.0 + 273.15);
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[239] != 0.0)) {
            scratch.values[55] = (self.params.tnom + 273.15);
            scratch.node_derivatives[55] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[55] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(17, &AdValue::scale(scratch.ad_value(49), THERMAL_VOLTAGE_PER_K));

        scratch.store_ad(25, &AdValue::scale(scratch.ad_value(17), 0.1));

        scratch.store_ad(24, &AdValue::div_from_scalar(1.0, scratch.ad_value(17)));

        scratch.store_ad(26, &AdValue::scale(scratch.ad_value(17), 2.0));

        scratch.store_ad(27, &AdValue::scale(scratch.ad_value(26), 2.0));

        scratch.store_ad(28, &AdValue::square(scratch.ad_value(17)));

        scratch.store_ad(29, &AdValue::scale(scratch.ad_value(28), 2.0));

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(28), 16.0));

        scratch.store_ad(51, &AdValue::sub_from_scalar(1.16, AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(49), 0.000702), scratch.ad_value(49)), AdValue::offset(scratch.ad_value(49), 1108.0))));

        scratch.store_ad(52, &AdValue::sub_from_scalar(1.16, AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(55), 0.000702), scratch.ad_value(55)), AdValue::offset(scratch.ad_value(55), 1108.0))));

        scratch.store_ad(53, &AdValue::sub(scratch.ad_value(49), scratch.ad_value(55)));

        scratch.store_ad(54, &AdValue::div(scratch.ad_value(49), scratch.ad_value(55)));

        scratch.store_ad(56, &AdValue::sub_from_scalar(self.params.vto, AdValue::scale(scratch.ad_value(53), self.params.tcv)));

        scratch.store_ad(58, &AdValue::scale(AdValue::powf(scratch.ad_value(54), self.params.bex), self.params.kp));

        scratch.store_ad(59, &AdValue::scale(AdValue::powf(scratch.ad_value(54), self.params.ucex), self.params.ucrit));

        scratch.store_ad(61, &AdValue::add(AdValue::sub(AdValue::sub(AdValue::scale(scratch.ad_value(54), self.params.phi), AdValue::mul(AdValue::scale(scratch.ad_value(17), 3.0), AdValue::ln(scratch.ad_value(54)))), AdValue::mul(scratch.ad_value(52), scratch.ad_value(54))), scratch.ad_value(51)));

        scratch.values[0] = 0.2;

        scratch.store_ad(1, &AdValue::offset(scratch.ad_value(61), (-scratch.values[0])));

        scratch.store_ad(61, &AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(1), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1)), AdValue::square(scratch.ad_value(17))))), 0.5), scratch.values[0]));

        scratch.store_ad(71, &AdValue::sqrt(scratch.ad_value(61)));

        scratch.store_ad(40, &AdValue::div_from_scalar(1.0, scratch.ad_value(59)));

        scratch.store_ad(41, &AdValue::scale(scratch.ad_value(59), scratch.values[34]));

        scratch.values[191] = (self.params.l + self.params.dl);

        scratch.values[192] = (self.params.w + self.params.dw);

        scratch.store_ad(158, &AdValue::scale(scratch.ad_value(59), scratch.values[191]));

        scratch.store_ad(173, &AdValue::mul(scratch.ad_value(17), AdValue::offset(AdValue::ln(AdValue::mul(AdValue::scale(scratch.ad_value(158), 0.5), scratch.ad_value(24))), (-0.6))));

        scratch.values[48] = (1.0 / (((scratch.values[192] * scratch.values[191])) as f64).sqrt());

        scratch.values[240] = if (self.params.type_ > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[240] != 0.0) {
            scratch.store_ad(57, &{
                if (self.params.avto != 1e-6) {
                    AdValue::offset(scratch.ad_value(56), (scratch.values[48] * (self.params.avto - 1e-6)))
                } else {
                    scratch.ad_value(56)
                }
            });
        }

        if (!(scratch.values[240] != 0.0)) {
            scratch.store_ad(57, &{
                if (self.params.avto != 1e-6) {
                    AdValue::sub_from_scalar((scratch.values[48] * (1e-6 - self.params.avto)), scratch.ad_value(56))
                } else {
                    AdValue::neg(scratch.ad_value(56))
                }
            });
        }

        scratch.store_ad(50, &AdValue::scale({
    if (self.params.akp != 1e-6) {
        AdValue::scale(scratch.ad_value(58), (1.0 + ((self.params.akp - 1e-6) * scratch.values[48])))
    } else {
        scratch.ad_value(58)
    }
}, scratch.values[192]));

        scratch.values[62] = (if (self.params.agamma != 1e-6) { (self.params.gamma + ((self.params.agamma - 1e-6) * scratch.values[48])) } else { self.params.gamma });

        scratch.store_ad(153, &AdValue::scale(scratch.ad_value(71), scratch.values[62]));

        scratch.values[241] = if (scratch.values[182] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[241] != 0.0) {
            scratch.values[183] = 0.0;
            scratch.node_derivatives[183] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[183] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.values[184] = (0.28 * ((scratch.values[191] / (self.params.lk * self.params.ns)) - 0.1));
            scratch.node_derivatives[184] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[184] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.store_ad(242, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(184), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(184)), 0.001936))), 0.5), 1.0)));
        }

        if (!(scratch.values[241] != 0.0)) {
            scratch.store_ad(183, &AdValue::mul(AdValue::scale(scratch.ad_value(242), scratch.values[182]), scratch.ad_value(242)));
        }

        scratch.store_ad(145, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(1), Some(3)), self.params.type_));

        scratch.store_ad(147, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(2), Some(3)), self.params.type_));

        scratch.store_ad(146, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(3)), self.params.type_));

        scratch.values[243] = if ((scratch.values[146] - scratch.values[147]) < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[243] != 0.0) {
            scratch.values[44] = (-1.0);
            scratch.node_derivatives[44] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[44] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[38] = scratch.values[147];
            scratch.node_derivatives[38] = scratch.node_derivatives[147];
            scratch.branch_derivatives[38] = scratch.branch_derivatives[147];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[147] = scratch.values[146];
            scratch.node_derivatives[147] = scratch.node_derivatives[146];
            scratch.branch_derivatives[147] = scratch.branch_derivatives[146];
        }

        if (scratch.values[243] != 0.0) {
            scratch.values[146] = scratch.values[38];
            scratch.node_derivatives[146] = scratch.node_derivatives[38];
            scratch.branch_derivatives[146] = scratch.branch_derivatives[38];
        }

        if (!(scratch.values[243] != 0.0)) {
            scratch.values[44] = 1.0;
            scratch.node_derivatives[44] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[44] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(143, &AdValue::add(AdValue::add(AdValue::sub(AdValue::sub(scratch.ad_value(145), scratch.ad_value(57)), scratch.ad_value(183)), scratch.ad_value(61)), scratch.ad_value(153)));

        scratch.store_ad(144, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(143)), AdValue::scale(scratch.ad_value(30), 2.0))));

        scratch.store_ad(3, &AdValue::scale(AdValue::add(scratch.ad_value(143), scratch.ad_value(144)), 0.5));

        scratch.store_ad(70, &AdValue::add(scratch.ad_value(61), scratch.ad_value(147)));

        scratch.store_ad(76, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(70)), scratch.ad_value(30))));

        scratch.store_ad(74, &AdValue::sqrt(AdValue::scale(AdValue::add(scratch.ad_value(70), scratch.ad_value(76)), 0.5)));

        scratch.store_ad(69, &AdValue::add(scratch.ad_value(61), scratch.ad_value(146)));

        scratch.store_ad(75, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(69)), scratch.ad_value(30))));

        scratch.store_ad(73, &AdValue::sqrt(AdValue::scale(AdValue::add(scratch.ad_value(69), scratch.ad_value(75)), 0.5)));

        scratch.values[45] = ((scratch.values[32] * self.params.m) / scratch.values[192]);

        scratch.values[46] = ((scratch.values[33] * self.params.ns) / scratch.values[191]);

        scratch.store_ad(67, &AdValue::sqrt(AdValue::offset(scratch.ad_value(3), ((0.25 * scratch.values[62]) * scratch.values[62]))));

        scratch.store_ad(68, &AdValue::sub(AdValue::sub(scratch.ad_value(3), scratch.ad_value(61)), AdValue::scale(AdValue::offset(scratch.ad_value(67), (-(0.5 * scratch.values[62]))), scratch.values[62])));

        scratch.store_ad(174, &AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(68), scratch.ad_value(61)), scratch.ad_value(25))));

        scratch.store_ad(64, &AdValue::add(AdValue::sub_from_scalar(scratch.values[62], AdValue::scale(AdValue::add(scratch.ad_value(74), scratch.ad_value(73)), scratch.values[46])), AdValue::scale(scratch.ad_value(174), scratch.values[45])));

        scratch.store_ad(65, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(64)), scratch.ad_value(25))));

        scratch.store_ad(4, &AdValue::scale(AdValue::add(scratch.ad_value(64), scratch.ad_value(65)), 0.5));

        scratch.store_ad(66, &AdValue::sqrt(AdValue::add(scratch.ad_value(3), AdValue::mul(AdValue::scale(scratch.ad_value(4), 0.25), scratch.ad_value(4)))));

        scratch.store_ad(5, &AdValue::sub(AdValue::sub(scratch.ad_value(3), scratch.ad_value(61)), AdValue::mul(scratch.ad_value(4), AdValue::sub(scratch.ad_value(66), AdValue::scale(scratch.ad_value(4), 0.5)))));

        scratch.store_ad(0, &AdValue::mul(AdValue::sub(scratch.ad_value(5), scratch.ad_value(147)), scratch.ad_value(24)));

        scratch.values[244] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[244] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[245] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[244] != 0.0)) && (scratch.values[245] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[246] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[244] != 0.0)) && (!(scratch.values[245] != 0.0))) && (scratch.values[246] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[244] != 0.0)) && (!(scratch.values[245] != 0.0))) && (!(scratch.values[246] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(7, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.store_ad(87, &AdValue::sqrt(scratch.ad_value(7)));

        scratch.values[90] = scratch.values[195];
        scratch.node_derivatives[90] = scratch.node_derivatives[195];
        scratch.branch_derivatives[90] = scratch.branch_derivatives[195];

        scratch.store_ad(160, &AdValue::div(scratch.ad_value(17), scratch.ad_value(158)));

        scratch.store_ad(80, &AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(87), scratch.ad_value(160)), 0.25)));

        scratch.store_ad(10, &AdValue::mul(scratch.ad_value(158), AdValue::offset(scratch.ad_value(80), (-0.5))));

        scratch.store_ad(77, &AdValue::scale(AdValue::sub(scratch.ad_value(146), scratch.ad_value(147)), 0.5));

        scratch.store_ad(78, &AdValue::mul(scratch.ad_value(30), AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(87), AdValue::mul(scratch.ad_value(10), scratch.ad_value(24))), self.params.lambda), 0.015625)));

        scratch.store_ad(81, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(10)), scratch.ad_value(78))));

        scratch.store_ad(82, &AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(77), scratch.ad_value(10)), AdValue::sub(scratch.ad_value(77), scratch.ad_value(10))), scratch.ad_value(78))));

        scratch.store_ad(79, &AdValue::sub(scratch.ad_value(81), scratch.ad_value(82)));

        scratch.store_ad(83, &AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(87), AdValue::scale(AdValue::ln(scratch.ad_value(7)), 0.75)), scratch.ad_value(160)), 0.25)));

        scratch.store_ad(11, &AdValue::add(AdValue::mul(scratch.ad_value(158), AdValue::offset(scratch.ad_value(83), (-0.5))), scratch.ad_value(173)));

        scratch.store_ad(159, &AdValue::sub(scratch.ad_value(77), scratch.ad_value(11)));

        scratch.store_ad(84, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(11)), scratch.ad_value(78))));

        scratch.store_ad(85, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(159)), scratch.ad_value(78))));

        scratch.store_ad(0, &AdValue::mul(AdValue::add(AdValue::sub(AdValue::sub(AdValue::sub(scratch.ad_value(5), scratch.ad_value(77)), scratch.ad_value(147)), scratch.ad_value(84)), scratch.ad_value(85)), scratch.ad_value(24)));

        scratch.values[247] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[247] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[248] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[247] != 0.0)) && (scratch.values[248] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[249] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[247] != 0.0)) && (!(scratch.values[248] != 0.0))) && (scratch.values[249] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[247] != 0.0)) && (!(scratch.values[248] != 0.0))) && (!(scratch.values[249] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(9, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.values[92] = scratch.values[195];
        scratch.node_derivatives[92] = scratch.node_derivatives[195];
        scratch.branch_derivatives[92] = scratch.branch_derivatives[195];

        scratch.store_ad(12, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(AdValue::sub(scratch.ad_value(77), scratch.ad_value(79)), scratch.ad_value(41)), 1.0)), scratch.values[35]));

        scratch.store_ad(155, &AdValue::add(AdValue::sub_from_scalar(scratch.values[191], scratch.ad_value(12)), AdValue::mul(AdValue::add(scratch.ad_value(77), scratch.ad_value(79)), scratch.ad_value(40))));

        scratch.values[154] = (0.1 * scratch.values[191]);

        scratch.store_ad(63, &AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(155)), (scratch.values[154] * scratch.values[154]))));

        scratch.store_ad(13, &AdValue::scale(AdValue::add(scratch.ad_value(155), scratch.ad_value(63)), 0.5));

        scratch.store_ad(0, &AdValue::mul(AdValue::sub(scratch.ad_value(5), scratch.ad_value(146)), scratch.ad_value(24)));

        scratch.values[250] = if (scratch.values[0] > (-0.35)) { 1.0 } else { 0.0 };

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(196, &AdValue::div_from_scalar(2.0, AdValue::sub(AdValue::offset(scratch.ad_value(0), 1.3), AdValue::ln(AdValue::offset(scratch.ad_value(0), 1.6)))));
        }

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if (scratch.values[250] != 0.0) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[251] = if (scratch.values[0] > (-15.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(196, &AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 1.55));
        }

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(197, &AdValue::div(AdValue::offset(scratch.ad_value(196), 2.0), AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(196)))));
        }

        if ((!(scratch.values[250] != 0.0)) && (scratch.values[251] != 0.0)) {
            scratch.store_ad(195, &AdValue::div(AdValue::add(AdValue::offset(scratch.ad_value(0), 1.0), AdValue::ln(scratch.ad_value(197))), AdValue::offset(scratch.ad_value(197), 2.0)));
        }

        scratch.values[252] = if (scratch.values[0] > (-23.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[250] != 0.0)) && (!(scratch.values[251] != 0.0))) && (scratch.values[252] != 0.0)) {
            scratch.store_ad(195, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::exp(AdValue::neg(scratch.ad_value(0))), 2.0)));
        }

        if (((!(scratch.values[250] != 0.0)) && (!(scratch.values[251] != 0.0))) && (!(scratch.values[252] != 0.0))) {
            scratch.store_ad(195, &AdValue::offset(AdValue::exp(scratch.ad_value(0)), 1e-64));
        }

        scratch.store_ad(8, &AdValue::mul(scratch.ad_value(195), AdValue::offset(scratch.ad_value(195), 1.0)));

        scratch.values[91] = scratch.values[195];
        scratch.node_derivatives[91] = scratch.node_derivatives[195];
        scratch.branch_derivatives[91] = scratch.branch_derivatives[195];

        scratch.store_ad(95, &AdValue::offset(scratch.ad_value(7), 0.25));

        scratch.store_ad(96, &AdValue::offset(scratch.ad_value(8), 0.25));

        scratch.store_ad(93, &AdValue::sqrt(scratch.ad_value(95)));

        scratch.store_ad(94, &AdValue::sqrt(scratch.ad_value(96)));

        scratch.store_ad(99, &AdValue::mul(AdValue::add(scratch.ad_value(93), scratch.ad_value(94)), AdValue::add(scratch.ad_value(93), scratch.ad_value(94))));

        scratch.store_ad(107, &AdValue::offset(AdValue::add(scratch.ad_value(5), scratch.ad_value(61)), 1e-6));

        scratch.store_ad(108, &AdValue::scale(AdValue::sqrt(scratch.ad_value(107)), 2.0));

        scratch.store_ad(111, &AdValue::div_from_scalar(scratch.values[62], scratch.ad_value(108)));

        scratch.store_ad(112, &AdValue::div_from_scalar(scratch.values[62], AdValue::offset(scratch.ad_value(108), scratch.values[62])));

        scratch.store_ad(100, &AdValue::mul(AdValue::mul(AdValue::neg(AdValue::offset(scratch.ad_value(111), 1.0)), scratch.ad_value(17)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(96), AdValue::mul(scratch.ad_value(94), scratch.ad_value(93))), scratch.ad_value(95)), (0.66666666 + 0.66666666)), AdValue::add(scratch.ad_value(93), scratch.ad_value(94))), (-1.0))));

        scratch.store_ad(101, &AdValue::sub(AdValue::scale(scratch.ad_value(108), ((-0.5) * scratch.values[62])), AdValue::mul(scratch.ad_value(112), scratch.ad_value(100))));

        scratch.values[253] = if (self.params.e0 == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(175, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(5)), scratch.ad_value(29))));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(6, &AdValue::scale(AdValue::add(scratch.ad_value(5), scratch.ad_value(175)), 0.5));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(157, &AdValue::offset(AdValue::scale(scratch.ad_value(6), self.params.theta), 1.0));
        }

        if (scratch.values[253] != 0.0) {
            scratch.store_ad(14, &AdValue::div(scratch.ad_value(50), AdValue::mul(scratch.ad_value(13), scratch.ad_value(157))));
        }

        scratch.values[254] = if ((scratch.values[101] + (scratch.values[39] * scratch.values[100])) > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[253] != 0.0)) && (scratch.values[254] != 0.0)) {
            scratch.store_ad(47, &AdValue::offset(AdValue::scale(AdValue::add(scratch.ad_value(101), AdValue::scale(scratch.ad_value(100), scratch.values[39])), scratch.values[37]), 1.0));
        }

        if ((!(scratch.values[253] != 0.0)) && (!(scratch.values[254] != 0.0))) {
            scratch.store_ad(47, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::add(scratch.ad_value(101), AdValue::scale(scratch.ad_value(100), scratch.values[39])), scratch.values[37])));
        }

        if (!(scratch.values[253] != 0.0)) {
            scratch.store_ad(156, &AdValue::offset(AdValue::scale(scratch.ad_value(153), scratch.values[37]), 1.0));
        }

        if (!(scratch.values[253] != 0.0)) {
            scratch.store_ad(14, &AdValue::div(AdValue::mul(scratch.ad_value(50), scratch.ad_value(156)), AdValue::mul(scratch.ad_value(13), scratch.ad_value(47))));
        }

        scratch.store_ad(72, &AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(61), scratch.ad_value(5)), scratch.ad_value(27))));

        scratch.store_ad(15, &AdValue::offset(AdValue::div_from_scalar(scratch.values[62], AdValue::scale(scratch.ad_value(72), 2.0)), 1.0));

        scratch.store_ad(86, &AdValue::sub(scratch.ad_value(7), scratch.ad_value(9)));

        scratch.store_ad(16, &AdValue::mul(AdValue::mul(scratch.ad_value(29), scratch.ad_value(15)), scratch.ad_value(14)));

        scratch.store_ad(0, &AdValue::div(scratch.ad_value(4), AdValue::scale(scratch.ad_value(65), 2.0)));

        scratch.store_ad(1, &AdValue::div(scratch.ad_value(3), scratch.ad_value(144)));

        scratch.store_ad(161, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(0), (-scratch.values[46])), scratch.ad_value(73)), scratch.ad_value(75)));

        scratch.store_ad(163, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(0), (-scratch.values[46])), scratch.ad_value(74)), scratch.ad_value(76)));

        scratch.store_ad(2, &AdValue::div(AdValue::add(scratch.ad_value(5), scratch.ad_value(61)), scratch.ad_value(66)));

        scratch.store_ad(113, &AdValue::mul(AdValue::neg(scratch.ad_value(2)), scratch.ad_value(161)));

        scratch.store_ad(115, &AdValue::mul(AdValue::neg(scratch.ad_value(2)), scratch.ad_value(163)));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(90), scratch.ad_value(24)));

        scratch.store_ad(116, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));

        scratch.store_ad(117, &AdValue::mul(scratch.ad_value(0), AdValue::offset(scratch.ad_value(115), (-1.0))));

        scratch.store_ad(0, &AdValue::div(scratch.ad_value(17), AdValue::mul(AdValue::scale(scratch.ad_value(80), 4.0), scratch.ad_value(87))));

        scratch.store_ad(122, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(116)));

        scratch.store_ad(124, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(117)));

        scratch.store_ad(0, &AdValue::scale(scratch.ad_value(27), (2.0 * self.params.lambda)));

        scratch.store_ad(1, &AdValue::div(scratch.ad_value(17), AdValue::scale(scratch.ad_value(87), 2.0)));

        scratch.store_ad(125, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(116), scratch.ad_value(1)), scratch.ad_value(122))));

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.store_ad(127, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(scratch.ad_value(117), scratch.ad_value(1)), scratch.ad_value(124))));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, scratch.ad_value(81)));

        scratch.store_ad(1, &AdValue::div_from_scalar(1.0, scratch.ad_value(82)));

        scratch.store_ad(2, &AdValue::sub(scratch.ad_value(77), scratch.ad_value(10)));

        scratch.store_ad(128, &AdValue::sub(AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(10), scratch.ad_value(122)), scratch.ad_value(125)), scratch.ad_value(0)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(2), AdValue::sub_from_scalar(0.5, scratch.ad_value(122))), scratch.ad_value(125)), scratch.ad_value(1))));

        scratch.store_ad(130, &AdValue::sub(AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(10), scratch.ad_value(124)), scratch.ad_value(127)), scratch.ad_value(0)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(2), AdValue::sub_from_scalar((-0.5), scratch.ad_value(124))), scratch.ad_value(127)), scratch.ad_value(1))));

        scratch.store_ad(0, &AdValue::div(AdValue::mul(scratch.ad_value(17), AdValue::offset(scratch.ad_value(87), (-1.5))), AdValue::mul(AdValue::scale(scratch.ad_value(83), 4.0), scratch.ad_value(7))));

        scratch.store_ad(131, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(116)));

        scratch.store_ad(133, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(117)));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(92), scratch.ad_value(24)));

        scratch.store_ad(1, &AdValue::div_from_scalar(1.0, scratch.ad_value(84)));

        scratch.store_ad(2, &AdValue::div_from_scalar(1.0, scratch.ad_value(85)));

        scratch.store_ad(134, &AdValue::mul(scratch.ad_value(0), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(113), (-0.5)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(11), scratch.ad_value(131)), scratch.ad_value(125)), scratch.ad_value(1))), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(159), AdValue::sub_from_scalar(0.5, scratch.ad_value(131))), scratch.ad_value(125)), scratch.ad_value(2)))));

        scratch.store_ad(136, &AdValue::mul(scratch.ad_value(0), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(115), (-0.5)), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(11), scratch.ad_value(133)), scratch.ad_value(127)), scratch.ad_value(1))), AdValue::mul(AdValue::add(AdValue::mul(scratch.ad_value(159), AdValue::sub_from_scalar((-0.5), scratch.ad_value(133))), scratch.ad_value(127)), scratch.ad_value(2)))));

        scratch.store_ad(0, &AdValue::div_from_scalar(scratch.values[35], AdValue::sub(AdValue::add(scratch.ad_value(41), scratch.ad_value(77)), scratch.ad_value(79))));

        scratch.store_ad(167, &AdValue::mul(scratch.ad_value(0), AdValue::sub_from_scalar(0.5, scratch.ad_value(128))));

        scratch.store_ad(169, &AdValue::mul(scratch.ad_value(0), AdValue::sub_from_scalar((-0.5), scratch.ad_value(130))));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, scratch.ad_value(63)));

        scratch.store_ad(137, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(AdValue::offset(scratch.ad_value(128), 0.5), scratch.ad_value(40)), scratch.ad_value(167))));

        scratch.store_ad(139, &AdValue::mul(scratch.ad_value(0), AdValue::sub(AdValue::mul(AdValue::offset(scratch.ad_value(130), (-0.5)), scratch.ad_value(40)), scratch.ad_value(169))));

        scratch.store_ad(0, &AdValue::mul(scratch.ad_value(91), scratch.ad_value(24)));

        scratch.store_ad(119, &AdValue::mul(scratch.ad_value(0), AdValue::offset(scratch.ad_value(113), (-1.0))));

        scratch.store_ad(120, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));

        scratch.store_ad(0, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::neg(AdValue::offset(scratch.ad_value(111), 1.0)), scratch.ad_value(17)), 0.66666666), scratch.ad_value(99)));

        scratch.store_ad(1, &AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(93), AdValue::scale(scratch.ad_value(94), 2.0))));

        scratch.store_ad(2, &AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(94), AdValue::scale(scratch.ad_value(93), 2.0))));

        scratch.store_ad(0, &AdValue::div(AdValue::mul(AdValue::neg(scratch.ad_value(111)), scratch.ad_value(100)), AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(111), 2.0), scratch.ad_value(111)), scratch.ad_value(107))));

        scratch.store_ad(185, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)), AdValue::mul(scratch.ad_value(1), scratch.ad_value(116))), AdValue::mul(scratch.ad_value(2), scratch.ad_value(119))));

        scratch.store_ad(186, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)), AdValue::mul(scratch.ad_value(1), scratch.ad_value(117))), AdValue::mul(scratch.ad_value(2), scratch.ad_value(120))));

        scratch.store_ad(0, &AdValue::sub(AdValue::offset(scratch.ad_value(111), 1.0), AdValue::div(scratch.ad_value(100), AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(111), 1.0), 2.0), scratch.ad_value(107)))));

        scratch.store_ad(188, &AdValue::mul(AdValue::neg(scratch.ad_value(112)), AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)), scratch.ad_value(185))));

        scratch.store_ad(189, &AdValue::mul(AdValue::neg(scratch.ad_value(112)), AdValue::add(AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)), scratch.ad_value(186))));

        scratch.values[255] = if (self.params.e0 == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(0, &AdValue::div(AdValue::scale(scratch.ad_value(6), self.params.theta), AdValue::mul(scratch.ad_value(157), scratch.ad_value(175))));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(164, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(166, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(140, &AdValue::sub(AdValue::neg(scratch.ad_value(137)), scratch.ad_value(164)));
        }

        if (scratch.values[255] != 0.0) {
            scratch.store_ad(142, &AdValue::sub(AdValue::neg(scratch.ad_value(139)), scratch.ad_value(166)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(0, &AdValue::div_from_scalar(scratch.values[37], scratch.ad_value(47)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(140, &AdValue::sub(AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(188), AdValue::scale(scratch.ad_value(185), scratch.values[39]))), scratch.ad_value(137)));
        }

        if (!(scratch.values[255] != 0.0)) {
            scratch.store_ad(142, &AdValue::sub(AdValue::mul(scratch.ad_value(0), AdValue::add(scratch.ad_value(189), AdValue::scale(scratch.ad_value(186), scratch.values[39]))), scratch.ad_value(139)));
        }

        scratch.store_ad(0, &AdValue::div_from_scalar((-scratch.values[62]), AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(15), 4.0), scratch.ad_value(72)), AdValue::add(AdValue::add(scratch.ad_value(61), scratch.ad_value(5)), scratch.ad_value(27)))));

        scratch.store_ad(170, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(113)));

        scratch.store_ad(172, &AdValue::mul(scratch.ad_value(0), scratch.ad_value(115)));

        scratch.store_ad(21, &AdValue::mul(scratch.ad_value(16), AdValue::sub(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(170), scratch.ad_value(140)), scratch.ad_value(86)), scratch.ad_value(116)), scratch.ad_value(134))));

        scratch.store_ad(19, &AdValue::mul(AdValue::neg(scratch.ad_value(16)), AdValue::sub(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(172), scratch.ad_value(142)), scratch.ad_value(86)), scratch.ad_value(117)), scratch.ad_value(136))));

        scratch.values[193] = ((self.params.rsh * self.params.hdif) / (scratch.values[192] - self.params.dw));

        scratch.values[194] = ((self.params.rsh * self.params.hdif) / (scratch.values[192] - self.params.dw));

        scratch.store_ad(0, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(19), scratch.values[193]), 1.0), AdValue::scale(scratch.ad_value(21), scratch.values[194]))));

        scratch.values[109] = ((scratch.values[192] * scratch.values[191]) * self.params.cox);

        scratch.store_ad(97, &AdValue::mul(scratch.ad_value(93), scratch.ad_value(95)));

        scratch.store_ad(98, &AdValue::mul(scratch.ad_value(94), scratch.ad_value(96)));

        scratch.store_ad(0, &AdValue::sqrt(AdValue::add(scratch.ad_value(61), AdValue::scale(scratch.ad_value(5), 0.5))));

        scratch.store_ad(181, &AdValue::scale(scratch.ad_value(0), 2.0));

        scratch.store_ad(110, &AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(scratch.ad_value(4), scratch.ad_value(181)), 1.0), scratch.ad_value(17)), scratch.values[109]));

        scratch.store_ad(102, &AdValue::mul(AdValue::neg(scratch.ad_value(110)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(98), 3.0), AdValue::mul(AdValue::scale(scratch.ad_value(96), 6.0), scratch.ad_value(93))), AdValue::mul(AdValue::scale(scratch.ad_value(94), 4.0), scratch.ad_value(95))), AdValue::scale(scratch.ad_value(97), 2.0)), 0.266666666), scratch.ad_value(99)), (-0.5))));

        scratch.store_ad(103, &AdValue::mul(AdValue::neg(scratch.ad_value(110)), AdValue::offset(AdValue::div(AdValue::scale(AdValue::add(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(97), 3.0), AdValue::mul(AdValue::scale(scratch.ad_value(95), 6.0), scratch.ad_value(94))), AdValue::mul(AdValue::scale(scratch.ad_value(93), 4.0), scratch.ad_value(96))), AdValue::scale(scratch.ad_value(98), 2.0)), 0.266666666), scratch.ad_value(99)), (-0.5))));

        scratch.store_ad(104, &AdValue::add(scratch.ad_value(103), scratch.ad_value(102)));

        scratch.store_ad(105, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(AdValue::mul(AdValue::scale(scratch.ad_value(4), (-0.5)), scratch.ad_value(108)), scratch.ad_value(3)), scratch.ad_value(143)), scratch.values[109]), AdValue::div(AdValue::mul(scratch.ad_value(104), scratch.ad_value(4)), AdValue::add(scratch.ad_value(4), scratch.ad_value(181)))));

        scratch.store_ad(106, &AdValue::sub(AdValue::neg(scratch.ad_value(104)), scratch.ad_value(105)));

        let assign2910_e2277_q: f64 = scratch.values[102];
        scratch.values[200] = scratch.values[102];
        scratch.node_derivatives[200][0] = scratch.node_derivatives[102][0];
        scratch.node_derivatives[200][1] = scratch.node_derivatives[102][1];
        scratch.node_derivatives[200][2] = scratch.node_derivatives[102][2];
        scratch.node_derivatives[200][3] = scratch.node_derivatives[102][3];
        scratch.reactive_values[200] = assign2910_e2277_q;
        scratch.reactive_node_derivatives[200][0] = scratch.node_derivatives[102][0];
        scratch.reactive_node_derivatives[200][1] = scratch.node_derivatives[102][1];
        scratch.reactive_node_derivatives[200][2] = scratch.node_derivatives[102][2];
        scratch.reactive_node_derivatives[200][3] = scratch.node_derivatives[102][3];

        let assign2920_e2279_q: f64 = scratch.values[103];
        scratch.values[201] = scratch.values[103];
        scratch.node_derivatives[201][0] = scratch.node_derivatives[103][0];
        scratch.node_derivatives[201][1] = scratch.node_derivatives[103][1];
        scratch.node_derivatives[201][2] = scratch.node_derivatives[103][2];
        scratch.node_derivatives[201][3] = scratch.node_derivatives[103][3];
        scratch.reactive_values[201] = assign2920_e2279_q;
        scratch.reactive_node_derivatives[201][0] = scratch.node_derivatives[103][0];
        scratch.reactive_node_derivatives[201][1] = scratch.node_derivatives[103][1];
        scratch.reactive_node_derivatives[201][2] = scratch.node_derivatives[103][2];
        scratch.reactive_node_derivatives[201][3] = scratch.node_derivatives[103][3];

        scratch.values[258] = if (scratch.values[44] == 1.0) { 1.0 } else { 0.0 };

        scratch.values[261] = if ((self.params.as_ == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[261] != 0.0) {
            scratch.values[202] = ((2.0 * self.params.hdif) * scratch.values[192]);
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[261] != 0.0)) {
            scratch.values[202] = self.params.as_;
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[262] = if ((self.params.ps == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[262] != 0.0) {
            scratch.values[204] = ((4.0 * self.params.hdif) + scratch.values[192]);
            scratch.node_derivatives[204] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[204] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[262] != 0.0)) {
            scratch.values[204] = self.params.ps;
            scratch.node_derivatives[204] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[204] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[263] = if ((self.params.ad == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[263] != 0.0) {
            scratch.values[203] = ((2.0 * self.params.hdif) * scratch.values[192]);
            scratch.node_derivatives[203] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[203] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[263] != 0.0)) {
            scratch.values[203] = self.params.ad;
            scratch.node_derivatives[203] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[203] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[264] = if ((self.params.pd == 0.0) && (self.params.hdif > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[264] != 0.0) {
            scratch.values[205] = ((4.0 * self.params.hdif) + scratch.values[192]);
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[264] != 0.0)) {
            scratch.values[205] = self.params.pd;
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(213, &AdValue::sub_from_scalar(self.params.xd_pb, AdValue::scale(scratch.ad_value(53), self.params.tp_pb)));

        scratch.store_ad(214, &AdValue::sub_from_scalar(self.params.xd_pbsw, AdValue::scale(scratch.ad_value(53), self.params.tp_pbsw)));

        scratch.store_ad(215, &AdValue::sub_from_scalar(self.params.xd_pbswg, AdValue::scale(scratch.ad_value(53), self.params.tp_pbswg)));

        scratch.store_ad(216, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cj), 1.0), self.params.xd_cj));

        scratch.store_ad(217, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cjsw), 1.0), self.params.xd_cjsw));

        scratch.store_ad(218, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(53), self.params.tp_cjswg), 1.0), self.params.xd_cjswg));

        scratch.store_ad(206, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(3)), self.params.type_));

        scratch.store_ad(207, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(2), Some(3)), self.params.type_));

        scratch.values[269] = if (scratch.values[206] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(230, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(203)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(213)), 1.0)), (-self.params.xd_mj)))));
        }

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(231, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(205)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(214)), 1.0)), (-self.params.xd_mjsw)))));
        }

        if (scratch.values[269] != 0.0) {
            scratch.store_ad(232, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(206), scratch.ad_value(215)), 1.0)), (-self.params.xd_mjswg)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(230, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(203)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mj), scratch.ad_value(213)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(231, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(205)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mjsw), scratch.ad_value(214)))));
        }

        if (!(scratch.values[269] != 0.0)) {
            scratch.store_ad(232, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(206), self.params.xd_mjswg), scratch.ad_value(215)))));
        }

        scratch.store_ad(236, &AdValue::mul(AdValue::add(AdValue::add(scratch.ad_value(230), scratch.ad_value(231)), scratch.ad_value(232)), scratch.ad_value(206)));

        scratch.values[270] = if (scratch.values[207] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(233, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(202)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(213)), 1.0)), (-self.params.xd_mj)))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(234, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(204)), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(214)), 1.0)), (-self.params.xd_mjsw)))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(235, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::exp(AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div(scratch.ad_value(207), scratch.ad_value(215)), 1.0)), (-self.params.xd_mjswg)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(233, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(202)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mj), scratch.ad_value(213)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(234, &AdValue::mul(AdValue::mul(scratch.ad_value(217), scratch.ad_value(204)), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mjsw), scratch.ad_value(214)))));
        }

        if (!(scratch.values[270] != 0.0)) {
            scratch.store_ad(235, &AdValue::mul(AdValue::scale(scratch.ad_value(218), scratch.values[192]), AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(207), self.params.xd_mjswg), scratch.ad_value(215)))));
        }

        scratch.store_ad(237, &AdValue::mul(AdValue::add(AdValue::add(scratch.ad_value(233), scratch.ad_value(234)), scratch.ad_value(235)), scratch.ad_value(207)));

    }
}
