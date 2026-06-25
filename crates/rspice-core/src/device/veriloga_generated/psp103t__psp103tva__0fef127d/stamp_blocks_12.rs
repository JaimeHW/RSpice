#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[1292] != 0.0) {
            scratch.values[77] = self.params.poepsrox;
            scratch.node_derivatives[77] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[77] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(78, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plneff), self.params.poneff), AdValue::scale(scratch.ad_value(320), self.params.pwneff)), AdValue::scale(scratch.ad_value(344), self.params.plwneff)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(79, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plfacneffac), self.params.pofacneffac), AdValue::scale(scratch.ad_value(320), self.params.pwfacneffac)), AdValue::scale(scratch.ad_value(344), self.params.plwfacneffac)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(80, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plgfacnud), self.params.pogfacnud), AdValue::scale(scratch.ad_value(320), self.params.pwgfacnud)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.plwgfacnud), scratch.ad_value(320))));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[81] = self.params.povsbnud;
            scratch.node_derivatives[81] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[81] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[82] = self.params.podvsbnud;
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[83] = self.params.povnsub;
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[84] = self.params.ponslp;
            scratch.node_derivatives[84] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[84] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[85] = self.params.podnsub;
            scratch.node_derivatives[85] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[85] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(86, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pldphib), self.params.podphib), AdValue::scale(scratch.ad_value(320), self.params.pwdphib)), AdValue::scale(scratch.ad_value(344), self.params.plwdphib)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(87, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pldelvtac), self.params.podelvtac), AdValue::scale(scratch.ad_value(320), self.params.pwdelvtac)), AdValue::scale(scratch.ad_value(344), self.params.plwdelvtac)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(88, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnp), self.params.ponp), AdValue::scale(scratch.ad_value(320), self.params.pwnp)), AdValue::scale(scratch.ad_value(344), self.params.plwnp)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[93] = self.params.potoxov;
            scratch.node_derivatives[93] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[93] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[94] = self.params.potoxovd;
            scratch.node_derivatives[94] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[94] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(95, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnov), self.params.ponov), AdValue::scale(scratch.ad_value(320), self.params.pwnov)), AdValue::scale(scratch.ad_value(344), self.params.plwnov)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(96, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnovd), self.params.ponovd), AdValue::scale(scratch.ad_value(320), self.params.pwnovd)), AdValue::scale(scratch.ad_value(344), self.params.plwnovd)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(89, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plct), self.params.poct), AdValue::scale(scratch.ad_value(320), self.params.pwct)), AdValue::scale(scratch.ad_value(344), self.params.plwct)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[90] = self.params.poctg;
            scratch.node_derivatives[90] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[90] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[91] = self.params.poctb;
            scratch.node_derivatives[91] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[91] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[92] = self.params.postct;
            scratch.node_derivatives[92] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[92] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(100, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plcf), self.params.pocf), AdValue::scale(scratch.ad_value(320), self.params.pwcf)), AdValue::scale(scratch.ad_value(344), self.params.plwcf)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[101] = self.params.pocfd;
            scratch.node_derivatives[101] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[101] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[102] = self.params.pocfb;
            scratch.node_derivatives[102] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[102] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(97, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plpsce), self.params.popsce), AdValue::scale(scratch.ad_value(320), self.params.pwpsce)), AdValue::scale(scratch.ad_value(344), self.params.plwpsce)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[99] = self.params.popsceb;
            scratch.node_derivatives[99] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[99] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[98] = self.params.popsced;
            scratch.node_derivatives[98] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[98] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(103, &AdValue::mul(AdValue::mul(scratch.ad_value(346), scratch.ad_value(319)), AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plbetn), self.params.pobetn), AdValue::scale(scratch.ad_value(320), self.params.pwbetn)), AdValue::scale(scratch.ad_value(344), self.params.plwbetn))));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(104, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plstbet), self.params.postbet), AdValue::scale(scratch.ad_value(320), self.params.pwstbet)), AdValue::scale(scratch.ad_value(344), self.params.plwstbet)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(105, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plmue), self.params.pomue), AdValue::scale(scratch.ad_value(320), self.params.pwmue)), AdValue::scale(scratch.ad_value(344), self.params.plwmue)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[106] = self.params.postmue;
            scratch.node_derivatives[106] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[106] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[107] = self.params.pothemu;
            scratch.node_derivatives[107] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[107] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[108] = self.params.postthemu;
            scratch.node_derivatives[108] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[108] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(109, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plcs), self.params.pocs), AdValue::scale(scratch.ad_value(320), self.params.pwcs)), AdValue::scale(scratch.ad_value(344), self.params.plwcs)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[110] = self.params.postcs;
            scratch.node_derivatives[110] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[110] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[111] = self.params.pothecs;
            scratch.node_derivatives[111] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[111] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[112] = self.params.postthecs;
            scratch.node_derivatives[112] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[112] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(113, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plxcor), self.params.poxcor), AdValue::scale(scratch.ad_value(320), self.params.pwxcor)), AdValue::scale(scratch.ad_value(344), self.params.plwxcor)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[114] = self.params.postxcor;
            scratch.node_derivatives[114] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[114] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[115] = self.params.pofeta;
            scratch.node_derivatives[115] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[115] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(116, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plrs), self.params.pors), AdValue::scale(scratch.ad_value(320), self.params.pwrs)), AdValue::scale(scratch.ad_value(344), self.params.plwrs)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[117] = self.params.postrs;
            scratch.node_derivatives[117] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[117] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[118] = self.params.porsb;
            scratch.node_derivatives[118] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[118] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[119] = self.params.porsg;
            scratch.node_derivatives[119] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[119] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(120, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plthesat), self.params.pothesat), AdValue::scale(scratch.ad_value(320), self.params.pwthesat)), AdValue::scale(scratch.ad_value(344), self.params.plwthesat)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(121, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plstthesat), self.params.postthesat), AdValue::scale(scratch.ad_value(320), self.params.pwstthesat)), AdValue::scale(scratch.ad_value(344), self.params.plwstthesat)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(122, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plthesatb), self.params.pothesatb), AdValue::scale(scratch.ad_value(320), self.params.pwthesatb)), AdValue::scale(scratch.ad_value(344), self.params.plwthesatb)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(123, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plthesatg), self.params.pothesatg), AdValue::scale(scratch.ad_value(320), self.params.pwthesatg)), AdValue::scale(scratch.ad_value(344), self.params.plwthesatg)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(124, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plax), self.params.poax), AdValue::scale(scratch.ad_value(320), self.params.pwax)), AdValue::scale(scratch.ad_value(344), self.params.plwax)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(125, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plalp), self.params.poalp), AdValue::scale(scratch.ad_value(320), self.params.pwalp)), AdValue::scale(scratch.ad_value(344), self.params.plwalp)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[128] = self.params.povp;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[130] = self.params.poa2;
            scratch.node_derivatives[130] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[130] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[131] = self.params.posta2;
            scratch.node_derivatives[131] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[131] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(132, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pla3), self.params.poa3), AdValue::scale(scratch.ad_value(320), self.params.pwa3)), AdValue::scale(scratch.ad_value(344), self.params.plwa3)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(133, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pla4), self.params.poa4), AdValue::scale(scratch.ad_value(320), self.params.pwa4)), AdValue::scale(scratch.ad_value(344), self.params.plwa4)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[134] = self.params.pogco;
            scratch.node_derivatives[134] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[134] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(135, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(345), self.params.pliginv), self.params.poiginv), AdValue::scale(scratch.ad_value(346), self.params.pwiginv)), AdValue::scale(scratch.ad_value(347), self.params.plwiginv)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(136, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pligov), self.params.poigov), AdValue::scale(scratch.ad_value(346), self.params.pwigov)), AdValue::scale(scratch.ad_value(348), self.params.plwigov)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(137, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pligovd), self.params.poigovd), AdValue::scale(scratch.ad_value(346), self.params.pwigovd)), AdValue::scale(scratch.ad_value(348), self.params.plwigovd)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[138] = self.params.postig;
            scratch.node_derivatives[138] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[138] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[139] = self.params.pogc2;
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[140] = self.params.pogc3;
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[141] = self.params.pochib;
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(142, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plagidl), self.params.poagidl), AdValue::scale(scratch.ad_value(346), self.params.pwagidl)), AdValue::scale(scratch.ad_value(348), self.params.plwagidl)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(143, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plagidld), self.params.poagidld), AdValue::scale(scratch.ad_value(346), self.params.pwagidld)), AdValue::scale(scratch.ad_value(348), self.params.plwagidld)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[144] = self.params.pobgidl;
            scratch.node_derivatives[144] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[144] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[145] = self.params.pobgidld;
            scratch.node_derivatives[145] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[145] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[146] = self.params.postbgidl;
            scratch.node_derivatives[146] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[146] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[147] = self.params.postbgidld;
            scratch.node_derivatives[147] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[147] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[148] = self.params.pocgidl;
            scratch.node_derivatives[148] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[148] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[149] = self.params.pocgidld;
            scratch.node_derivatives[149] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[149] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(150, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(350), self.params.plcox), self.params.pocox), AdValue::scale(scratch.ad_value(351), self.params.pwcox)), AdValue::scale(scratch.ad_value(352), self.params.plwcox)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(151, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(349), self.params.plcgov), self.params.pocgov), AdValue::scale(scratch.ad_value(351), self.params.pwcgov)), AdValue::scale(scratch.ad_value(353), self.params.plwcgov)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(152, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(349), self.params.plcgovd), self.params.pocgovd), AdValue::scale(scratch.ad_value(351), self.params.pwcgovd)), AdValue::scale(scratch.ad_value(353), self.params.plwcgovd)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(153, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(355), self.params.plcgbov), self.params.pocgbov), AdValue::scale(scratch.ad_value(356), self.params.pwcgbov)), AdValue::scale(scratch.ad_value(357), self.params.plwcgbov)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(154, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(354), self.params.plcfr), self.params.pocfr), AdValue::scale(scratch.ad_value(356), self.params.pwcfr)), AdValue::scale(scratch.ad_value(358), self.params.plwcfr)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(155, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(354), self.params.plcfrd), self.params.pocfrd), AdValue::scale(scratch.ad_value(356), self.params.pwcfrd)), AdValue::scale(scratch.ad_value(358), self.params.plwcfrd)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[156] = self.params.pofnt;
            scratch.node_derivatives[156] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[156] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[162] = self.params.povfbedge;
            scratch.node_derivatives[162] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[162] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(163, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plstvfbedge), self.params.postvfbedge), AdValue::scale(scratch.ad_value(320), self.params.pwstvfbedge)), AdValue::scale(scratch.ad_value(344), self.params.plwstvfbedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(164, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pldphibedge), self.params.podphibedge), AdValue::scale(scratch.ad_value(320), self.params.pwdphibedge)), AdValue::scale(scratch.ad_value(344), self.params.plwdphibedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(165, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plneffedge), self.params.poneffedge), AdValue::scale(scratch.ad_value(320), self.params.pwneffedge)), AdValue::scale(scratch.ad_value(344), self.params.plwneffedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(166, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plctedge), self.params.poctedge), AdValue::scale(scratch.ad_value(320), self.params.pwctedge)), AdValue::scale(scratch.ad_value(344), self.params.plwctedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(167, &AdValue::mul(scratch.ad_value(319), AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plbetnedge), self.params.pobetnedge), AdValue::scale(scratch.ad_value(320), self.params.pwbetnedge)), AdValue::scale(scratch.ad_value(344), self.params.plwbetnedge))));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(169, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plpsceedge), self.params.popsceedge), AdValue::scale(scratch.ad_value(320), self.params.pwpsceedge)), AdValue::scale(scratch.ad_value(344), self.params.plwpsceedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[170] = self.params.popscebedge;
            scratch.node_derivatives[170] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[170] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[171] = self.params.popscededge;
            scratch.node_derivatives[171] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[171] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(172, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plcfedge), self.params.pocfedge), AdValue::scale(scratch.ad_value(320), self.params.pwcfedge)), AdValue::scale(scratch.ad_value(344), self.params.plwcfedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[173] = self.params.pocfdedge;
            scratch.node_derivatives[173] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[173] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[174] = self.params.pocfbedge;
            scratch.node_derivatives[174] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[174] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(341, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plkvthowe), self.params.pokvthowe), AdValue::scale(scratch.ad_value(320), self.params.pwkvthowe)), AdValue::scale(scratch.ad_value(344), self.params.plwkvthowe)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(342, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plkuowe), self.params.pokuowe), AdValue::scale(scratch.ad_value(320), self.params.pwkuowe)), AdValue::scale(scratch.ad_value(344), self.params.plwkuowe)));
        }

        scratch.values[1293] = if ((scratch.values[1] == 1.0) || (scratch.values[1] == 2.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(188, &AdValue::offset(AdValue::scale(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(314), 1000000.0), AdValue::offset(AdValue::scale(scratch.ad_value(313), (self.params.cthlw * 1000000.0)), 1.0)), self.params.cthw2), self.params.cthw1), self.params.ctho));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.values[1281] = 0.0;
            scratch.node_derivatives[1281] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1281] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1293] != 0.0) {
            scratch.values[1282] = 0.0;
            scratch.node_derivatives[1282] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1282] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1293] != 0.0) {
            scratch.values[1280] = 0.0;
            scratch.node_derivatives[1280] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1280] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1295] = if (((scratch.values[15] > 0.0) && (scratch.values[16] > 0.0)) && ((scratch.values[11] == 1.0) || ((scratch.values[11] > 1.0) && (scratch.values[17] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign7930_loop_guard: usize = 0;
        while {
            let assign7930_cond_e6662: f64 = (scratch.values[11] - 0.5);
            let assign7930_cond_e6664: f64 = if (((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1280] < assign7930_cond_e6662)) { 1.0 } else { 0.0 };
            assign7930_cond_e6664 != 0.0
        } {
            assign7930_loop_guard += 1;
            assert!(assign7930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
                scratch.store_ad(1281, &AdValue::add(scratch.ad_value(1281), AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[15]), AdValue::mul(scratch.ad_value(1280), AdValue::offset(scratch.ad_value(13), scratch.values[17]))))));
            }
            if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
                scratch.store_ad(1282, &AdValue::add(scratch.ad_value(1282), AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[16]), AdValue::mul(scratch.ad_value(1280), AdValue::offset(scratch.ad_value(13), scratch.values[17]))))));
            }
            if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
                scratch.store_ad(1280, &AdValue::offset(scratch.ad_value(1280), 1.0));
            }
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1266, &AdValue::mul(scratch.ad_value(1281), scratch.ad_value(12)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1267, &AdValue::mul(scratch.ad_value(1282), scratch.ad_value(12)));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1268, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[55])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1269, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[56])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1278, &{
                if ((scratch.values[13] + scratch.values[311]) > 1e-9) {
                    AdValue::add(scratch.ad_value(13), scratch.ad_value(311))
                } else {
                    AdValue::constant(1e-9)
                }
            });
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1279, &{
                if (((scratch.values[14] + scratch.values[312]) + self.params.wlod) > 1e-9) {
                    AdValue::offset(AdValue::add(scratch.ad_value(14), scratch.ad_value(312)), self.params.wlod)
                } else {
                    AdValue::constant(1e-9)
                }
            });
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1276, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1278), scratch.values[58])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1277, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1279), scratch.values[59])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1270, &AdValue::scale(AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1276), self.params.lkuo), 1.0), AdValue::scale(scratch.ad_value(1277), self.params.wkuo)), AdValue::mul(AdValue::scale(scratch.ad_value(1276), self.params.pkuo), scratch.ad_value(1277))), (1.0 + (self.params.tkuo * (scratch.values[361] - 1.0)))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1271, &AdValue::div(AdValue::scale(AdValue::add(scratch.ad_value(1266), scratch.ad_value(1267)), self.params.kuo), scratch.ad_value(1270)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1272, &AdValue::div(AdValue::scale(AdValue::add(scratch.ad_value(1268), scratch.ad_value(1269)), self.params.kuo), scratch.ad_value(1270)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1276, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1278), scratch.values[60])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1277, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1279), scratch.values[61])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1273, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1276), self.params.lkvtho), 1.0), AdValue::scale(scratch.ad_value(1277), self.params.wkvtho)), AdValue::mul(AdValue::scale(scratch.ad_value(1276), self.params.pkvtho), scratch.ad_value(1277))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1274, &AdValue::sub(AdValue::sub(AdValue::add(scratch.ad_value(1266), scratch.ad_value(1267)), scratch.ad_value(1268)), scratch.ad_value(1269)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(103, &AdValue::div(AdValue::mul(scratch.ad_value(103), AdValue::offset(scratch.ad_value(1271), 1.0)), AdValue::offset(scratch.ad_value(1272), 1.0)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(120, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(120), AdValue::offset(scratch.ad_value(1271), 1.0)), AdValue::offset(AdValue::scale(scratch.ad_value(1272), scratch.values[57]), 1.0)), AdValue::mul(AdValue::offset(scratch.ad_value(1272), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1271), scratch.values[57]), 1.0))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(73, &AdValue::add(scratch.ad_value(73), AdValue::div(AdValue::scale(scratch.ad_value(1274), self.params.kvtho), scratch.ad_value(1273))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(100, &AdValue::add(scratch.ad_value(100), AdValue::div(AdValue::scale(scratch.ad_value(1274), self.params.stetao), AdValue::powf(scratch.ad_value(1273), scratch.values[62]))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(167, &AdValue::div(AdValue::mul(scratch.ad_value(167), AdValue::offset(scratch.ad_value(1271), 1.0)), AdValue::offset(scratch.ad_value(1272), 1.0)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(162, &AdValue::add(scratch.ad_value(162), AdValue::div(AdValue::scale(scratch.ad_value(1274), self.params.kvtho), scratch.ad_value(1273))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(172, &AdValue::add(scratch.ad_value(172), AdValue::div(AdValue::scale(scratch.ad_value(1274), self.params.stetao), AdValue::powf(scratch.ad_value(1273), scratch.values[62]))));
        }

        scratch.values[1296] = if ((((scratch.values[21] > 0.0) || (scratch.values[22] > 0.0)) || (scratch.values[23] > 0.0)) || (scratch.values[18] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1297] = if (((scratch.values[21] == 0.0) && (scratch.values[22] == 0.0)) && (scratch.values[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) && (scratch.values[1297] != 0.0)) {
            scratch.store_ad(1274, &AdValue::offset(scratch.ad_value(14), scratch.values[18]));
        }

        if (((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) && (scratch.values[1297] != 0.0)) {
            scratch.values[1275] = (1.0 / scratch.values[63]);
            scratch.node_derivatives[1275] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1275] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) && (scratch.values[1297] != 0.0)) {
            scratch.store_ad(21, &AdValue::div_from_scalar((scratch.values[63] * scratch.values[63]), AdValue::scale(scratch.ad_value(1274), scratch.values[18])));
        }

        if (((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) && (scratch.values[1297] != 0.0)) {
            scratch.store_ad(22, &AdValue::div(AdValue::sub(AdValue::scale(AdValue::exp(AdValue::scale(scratch.ad_value(1275), ((-10.0) * scratch.values[18]))), ((0.1 * scratch.values[18]) + (0.01 * scratch.values[63]))), AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(1274), 0.1), (0.01 * scratch.values[63])), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(1274), (-10.0)), scratch.ad_value(1275))))), scratch.ad_value(14)));
        }

        if (((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) && (scratch.values[1297] != 0.0)) {
            scratch.store_ad(23, &AdValue::div(AdValue::sub(AdValue::scale(AdValue::exp(AdValue::scale(scratch.ad_value(1275), ((-20.0) * scratch.values[18]))), ((0.05 * scratch.values[18]) + (0.0025 * scratch.values[63]))), AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(1274), 0.05), (0.0025 * scratch.values[63])), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(1274), (-20.0)), scratch.ad_value(1275))))), scratch.ad_value(14)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(1274, &AdValue::add(AdValue::add(scratch.ad_value(21), AdValue::scale(scratch.ad_value(22), scratch.values[64])), AdValue::scale(scratch.ad_value(23), scratch.values[65])));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(73, &AdValue::add(scratch.ad_value(73), AdValue::mul(scratch.ad_value(341), scratch.ad_value(1274))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(103, &AdValue::mul(scratch.ad_value(103), AdValue::offset(AdValue::mul(scratch.ad_value(342), scratch.ad_value(1274)), 1.0)));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(162, &AdValue::add(scratch.ad_value(162), AdValue::mul(scratch.ad_value(341), scratch.ad_value(1274))));
        }

        if ((scratch.values[1293] != 0.0) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(167, &AdValue::mul(scratch.ad_value(167), AdValue::offset(AdValue::mul(scratch.ad_value(342), scratch.ad_value(1274)), 1.0)));
        }

        scratch.values[192] = scratch.values[73];
        scratch.node_derivatives[192] = scratch.node_derivatives[73];
        scratch.branch_derivatives[192] = scratch.branch_derivatives[73];

        scratch.values[193] = scratch.values[74];
        scratch.node_derivatives[193] = scratch.node_derivatives[74];
        scratch.branch_derivatives[193] = scratch.branch_derivatives[74];

        scratch.values[194] = scratch.values[75];
        scratch.node_derivatives[194] = scratch.node_derivatives[75];
        scratch.branch_derivatives[194] = scratch.branch_derivatives[75];

        if (scratch.values[76] > 1e-10) {
            scratch.values[196] = scratch.values[76];
            scratch.node_derivatives[196] = scratch.node_derivatives[76];
            scratch.branch_derivatives[196] = scratch.branch_derivatives[76];
        } else {
            scratch.values[196] = 1e-10;
            scratch.node_derivatives[196] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[196] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[77] > 1.0) {
            scratch.values[197] = scratch.values[77];
            scratch.node_derivatives[197] = scratch.node_derivatives[77];
            scratch.branch_derivatives[197] = scratch.branch_derivatives[77];
        } else {
            scratch.values[197] = 1.0;
            scratch.node_derivatives[197] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[197] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[78] > 1e20) {
            scratch.store_ad(198, &{
                if (scratch.values[78] < 1e26) {
                    scratch.ad_value(78)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[198] = 1e20;
            scratch.node_derivatives[198] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[198] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[79] > 0.0) {
            scratch.values[199] = scratch.values[79];
            scratch.node_derivatives[199] = scratch.node_derivatives[79];
            scratch.branch_derivatives[199] = scratch.branch_derivatives[79];
        } else {
            scratch.values[199] = 0.0;
            scratch.node_derivatives[199] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[199] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[80] > 0.01) {
            scratch.values[200] = scratch.values[80];
            scratch.node_derivatives[200] = scratch.node_derivatives[80];
            scratch.branch_derivatives[200] = scratch.branch_derivatives[80];
        } else {
            scratch.values[200] = 0.01;
            scratch.node_derivatives[200] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[200] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[81] > 0.0) {
            scratch.values[201] = scratch.values[81];
            scratch.node_derivatives[201] = scratch.node_derivatives[81];
            scratch.branch_derivatives[201] = scratch.branch_derivatives[81];
        } else {
            scratch.values[201] = 0.0;
            scratch.node_derivatives[201] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[201] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[82] > 0.1) {
            scratch.values[202] = scratch.values[82];
            scratch.node_derivatives[202] = scratch.node_derivatives[82];
            scratch.branch_derivatives[202] = scratch.branch_derivatives[82];
        } else {
            scratch.values[202] = 0.1;
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[203] = scratch.values[83];
        scratch.node_derivatives[203] = scratch.node_derivatives[83];
        scratch.branch_derivatives[203] = scratch.branch_derivatives[83];

        if (scratch.values[84] > 0.001) {
            scratch.values[204] = scratch.values[84];
            scratch.node_derivatives[204] = scratch.node_derivatives[84];
            scratch.branch_derivatives[204] = scratch.branch_derivatives[84];
        } else {
            scratch.values[204] = 0.001;
            scratch.node_derivatives[204] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[204] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[85] > 0.0) {
            scratch.store_ad(205, &{
                if (scratch.values[85] < 1.0) {
                    scratch.ad_value(85)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[205] = 0.0;
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[206] = scratch.values[86];
        scratch.node_derivatives[206] = scratch.node_derivatives[86];
        scratch.branch_derivatives[206] = scratch.branch_derivatives[86];

        scratch.values[207] = scratch.values[87];
        scratch.node_derivatives[207] = scratch.node_derivatives[87];
        scratch.branch_derivatives[207] = scratch.branch_derivatives[87];

        if (scratch.values[88] > 0.0) {
            scratch.values[208] = scratch.values[88];
            scratch.node_derivatives[208] = scratch.node_derivatives[88];
            scratch.branch_derivatives[208] = scratch.branch_derivatives[88];
        } else {
            scratch.values[208] = 0.0;
            scratch.node_derivatives[208] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[208] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[93] > 1e-10) {
            scratch.values[212] = scratch.values[93];
            scratch.node_derivatives[212] = scratch.node_derivatives[93];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[93];
        } else {
            scratch.values[212] = 1e-10;
            scratch.node_derivatives[212] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[212] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[94] > 1e-10) {
            scratch.values[213] = scratch.values[94];
            scratch.node_derivatives[213] = scratch.node_derivatives[94];
            scratch.branch_derivatives[213] = scratch.branch_derivatives[94];
        } else {
            scratch.values[213] = 1e-10;
            scratch.node_derivatives[213] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[213] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[95] > 1e23) {
            scratch.store_ad(214, &{
                if (scratch.values[95] < 1e27) {
                    scratch.ad_value(95)
                } else {
                    AdValue::constant(1e27)
                }
            });
        } else {
            scratch.values[214] = 1e23;
            scratch.node_derivatives[214] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[214] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[96] > 1e23) {
            scratch.store_ad(215, &{
                if (scratch.values[96] < 1e27) {
                    scratch.ad_value(96)
                } else {
                    AdValue::constant(1e27)
                }
            });
        } else {
            scratch.values[215] = 1e23;
            scratch.node_derivatives[215] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[215] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[89] > 0.0) {
            scratch.values[209] = scratch.values[89];
            scratch.node_derivatives[209] = scratch.node_derivatives[89];
            scratch.branch_derivatives[209] = scratch.branch_derivatives[89];
        } else {
            scratch.values[209] = 0.0;
            scratch.node_derivatives[209] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[209] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[90] > 0.0) {
            scratch.values[210] = scratch.values[90];
            scratch.node_derivatives[210] = scratch.node_derivatives[90];
            scratch.branch_derivatives[210] = scratch.branch_derivatives[90];
        } else {
            scratch.values[210] = 0.0;
            scratch.node_derivatives[210] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[210] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[211] = scratch.values[91];
        scratch.node_derivatives[211] = scratch.node_derivatives[91];
        scratch.branch_derivatives[211] = scratch.branch_derivatives[91];

        scratch.values[195] = scratch.values[92];
        scratch.node_derivatives[195] = scratch.node_derivatives[92];
        scratch.branch_derivatives[195] = scratch.branch_derivatives[92];

        if (scratch.values[100] > 0.0) {
            scratch.values[216] = scratch.values[100];
            scratch.node_derivatives[216] = scratch.node_derivatives[100];
            scratch.branch_derivatives[216] = scratch.branch_derivatives[100];
        } else {
            scratch.values[216] = 0.0;
            scratch.node_derivatives[216] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[216] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[101] > 0.0) {
            scratch.values[217] = scratch.values[101];
            scratch.node_derivatives[217] = scratch.node_derivatives[101];
            scratch.branch_derivatives[217] = scratch.branch_derivatives[101];
        } else {
            scratch.values[217] = 0.0;
            scratch.node_derivatives[217] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[217] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[102] > 0.0) {
            scratch.store_ad(218, &{
                if (scratch.values[102] < 1.0) {
                    scratch.ad_value(102)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[218] = 0.0;
            scratch.node_derivatives[218] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[218] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[97] > 0.0) {
            scratch.values[219] = scratch.values[97];
            scratch.node_derivatives[219] = scratch.node_derivatives[97];
            scratch.branch_derivatives[219] = scratch.branch_derivatives[97];
        } else {
            scratch.values[219] = 0.0;
            scratch.node_derivatives[219] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[219] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[99] > 0.0) {
            scratch.store_ad(220, &{
                if (scratch.values[99] < 1.0) {
                    scratch.ad_value(99)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[220] = 0.0;
            scratch.node_derivatives[220] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[220] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[98] > 0.0) {
            scratch.values[221] = scratch.values[98];
            scratch.node_derivatives[221] = scratch.node_derivatives[98];
            scratch.branch_derivatives[221] = scratch.branch_derivatives[98];
        } else {
            scratch.values[221] = 0.0;
            scratch.node_derivatives[221] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[221] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[103] > 0.0) {
            scratch.values[222] = scratch.values[103];
            scratch.node_derivatives[222] = scratch.node_derivatives[103];
            scratch.branch_derivatives[222] = scratch.branch_derivatives[103];
        } else {
            scratch.values[222] = 0.0;
            scratch.node_derivatives[222] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[222] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[223] = scratch.values[104];
        scratch.node_derivatives[223] = scratch.node_derivatives[104];
        scratch.branch_derivatives[223] = scratch.branch_derivatives[104];

        if (scratch.values[105] > 0.0) {
            scratch.values[224] = scratch.values[105];
            scratch.node_derivatives[224] = scratch.node_derivatives[105];
            scratch.branch_derivatives[224] = scratch.branch_derivatives[105];
        } else {
            scratch.values[224] = 0.0;
            scratch.node_derivatives[224] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[224] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[225] = scratch.values[106];
        scratch.node_derivatives[225] = scratch.node_derivatives[106];
        scratch.branch_derivatives[225] = scratch.branch_derivatives[106];

        if (scratch.values[107] > 0.0) {
            scratch.values[226] = scratch.values[107];
            scratch.node_derivatives[226] = scratch.node_derivatives[107];
            scratch.branch_derivatives[226] = scratch.branch_derivatives[107];
        } else {
            scratch.values[226] = 0.0;
            scratch.node_derivatives[226] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[226] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[227] = scratch.values[108];
        scratch.node_derivatives[227] = scratch.node_derivatives[108];
        scratch.branch_derivatives[227] = scratch.branch_derivatives[108];

        if (scratch.values[109] > 0.0) {
            scratch.values[228] = scratch.values[109];
            scratch.node_derivatives[228] = scratch.node_derivatives[109];
            scratch.branch_derivatives[228] = scratch.branch_derivatives[109];
        } else {
            scratch.values[228] = 0.0;
            scratch.node_derivatives[228] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[228] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[229] = scratch.values[110];
        scratch.node_derivatives[229] = scratch.node_derivatives[110];
        scratch.branch_derivatives[229] = scratch.branch_derivatives[110];

        if (scratch.values[111] > 0.0) {
            scratch.values[230] = scratch.values[111];
            scratch.node_derivatives[230] = scratch.node_derivatives[111];
            scratch.branch_derivatives[230] = scratch.branch_derivatives[111];
        } else {
            scratch.values[230] = 0.0;
            scratch.node_derivatives[230] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[230] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[231] = scratch.values[112];
        scratch.node_derivatives[231] = scratch.node_derivatives[112];
        scratch.branch_derivatives[231] = scratch.branch_derivatives[112];

        if (scratch.values[113] > 0.0) {
            scratch.values[232] = scratch.values[113];
            scratch.node_derivatives[232] = scratch.node_derivatives[113];
            scratch.branch_derivatives[232] = scratch.branch_derivatives[113];
        } else {
            scratch.values[232] = 0.0;
            scratch.node_derivatives[232] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[232] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[233] = scratch.values[114];
        scratch.node_derivatives[233] = scratch.node_derivatives[114];
        scratch.branch_derivatives[233] = scratch.branch_derivatives[114];

        if (scratch.values[115] > 0.0) {
            scratch.values[234] = scratch.values[115];
            scratch.node_derivatives[234] = scratch.node_derivatives[115];
            scratch.branch_derivatives[234] = scratch.branch_derivatives[115];
        } else {
            scratch.values[234] = 0.0;
            scratch.node_derivatives[234] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[234] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[116] > 0.0) {
            scratch.values[235] = scratch.values[116];
            scratch.node_derivatives[235] = scratch.node_derivatives[116];
            scratch.branch_derivatives[235] = scratch.branch_derivatives[116];
        } else {
            scratch.values[235] = 0.0;
            scratch.node_derivatives[235] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[235] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[236] = scratch.values[117];
        scratch.node_derivatives[236] = scratch.node_derivatives[117];
        scratch.branch_derivatives[236] = scratch.branch_derivatives[117];

        if (scratch.values[118] > (-0.5)) {
            scratch.store_ad(237, &{
                if (scratch.values[118] < 1.0) {
                    scratch.ad_value(118)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[237] = (-0.5);
            scratch.node_derivatives[237] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[237] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[119] > (-0.5)) {
            scratch.values[238] = scratch.values[119];
            scratch.node_derivatives[238] = scratch.node_derivatives[119];
            scratch.branch_derivatives[238] = scratch.branch_derivatives[119];
        } else {
            scratch.values[238] = (-0.5);
            scratch.node_derivatives[238] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[238] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[120] > 0.0) {
            scratch.values[239] = scratch.values[120];
            scratch.node_derivatives[239] = scratch.node_derivatives[120];
            scratch.branch_derivatives[239] = scratch.branch_derivatives[120];
        } else {
            scratch.values[239] = 0.0;
            scratch.node_derivatives[239] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[239] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[240] = scratch.values[121];
        scratch.node_derivatives[240] = scratch.node_derivatives[121];
        scratch.branch_derivatives[240] = scratch.branch_derivatives[121];

        if (scratch.values[122] > (-0.5)) {
            scratch.store_ad(241, &{
                if (scratch.values[122] < 1.0) {
                    scratch.ad_value(122)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[241] = (-0.5);
            scratch.node_derivatives[241] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[241] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[123] > (-0.5)) {
            scratch.values[242] = scratch.values[123];
            scratch.node_derivatives[242] = scratch.node_derivatives[123];
            scratch.branch_derivatives[242] = scratch.branch_derivatives[123];
        } else {
            scratch.values[242] = (-0.5);
            scratch.node_derivatives[242] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[242] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[124] > 2.0) {
            scratch.values[243] = scratch.values[124];
            scratch.node_derivatives[243] = scratch.node_derivatives[124];
            scratch.branch_derivatives[243] = scratch.branch_derivatives[124];
        } else {
            scratch.values[243] = 2.0;
            scratch.node_derivatives[243] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[243] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[125] > 0.0) {
            scratch.values[244] = scratch.values[125];
            scratch.node_derivatives[244] = scratch.node_derivatives[125];
            scratch.branch_derivatives[244] = scratch.branch_derivatives[125];
        } else {
            scratch.values[244] = 0.0;
            scratch.node_derivatives[244] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[244] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[128] > 1e-10) {
            scratch.values[247] = scratch.values[128];
            scratch.node_derivatives[247] = scratch.node_derivatives[128];
            scratch.branch_derivatives[247] = scratch.branch_derivatives[128];
        } else {
            scratch.values[247] = 1e-10;
            scratch.node_derivatives[247] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[247] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[130] > 0.0) {
            scratch.values[249] = scratch.values[130];
            scratch.node_derivatives[249] = scratch.node_derivatives[130];
            scratch.branch_derivatives[249] = scratch.branch_derivatives[130];
        } else {
            scratch.values[249] = 0.0;
            scratch.node_derivatives[249] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[249] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[250] = scratch.values[131];
        scratch.node_derivatives[250] = scratch.node_derivatives[131];
        scratch.branch_derivatives[250] = scratch.branch_derivatives[131];

        if (scratch.values[132] > 0.0) {
            scratch.values[251] = scratch.values[132];
            scratch.node_derivatives[251] = scratch.node_derivatives[132];
            scratch.branch_derivatives[251] = scratch.branch_derivatives[132];
        } else {
            scratch.values[251] = 0.0;
            scratch.node_derivatives[251] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[251] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[133] > 0.0) {
            scratch.values[252] = scratch.values[133];
            scratch.node_derivatives[252] = scratch.node_derivatives[133];
            scratch.branch_derivatives[252] = scratch.branch_derivatives[133];
        } else {
            scratch.values[252] = 0.0;
            scratch.node_derivatives[252] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[252] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[134] > (-10.0)) {
            scratch.store_ad(253, &{
                if (scratch.values[134] < 10.0) {
                    scratch.ad_value(134)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[253] = (-10.0);
            scratch.node_derivatives[253] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[253] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[135] > 0.0) {
            scratch.values[254] = scratch.values[135];
            scratch.node_derivatives[254] = scratch.node_derivatives[135];
            scratch.branch_derivatives[254] = scratch.branch_derivatives[135];
        } else {
            scratch.values[254] = 0.0;
            scratch.node_derivatives[254] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[254] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[136] > 0.0) {
            scratch.values[255] = scratch.values[136];
            scratch.node_derivatives[255] = scratch.node_derivatives[136];
            scratch.branch_derivatives[255] = scratch.branch_derivatives[136];
        } else {
            scratch.values[255] = 0.0;
            scratch.node_derivatives[255] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[255] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[137] > 0.0) {
            scratch.values[256] = scratch.values[137];
            scratch.node_derivatives[256] = scratch.node_derivatives[137];
            scratch.branch_derivatives[256] = scratch.branch_derivatives[137];
        } else {
            scratch.values[256] = 0.0;
            scratch.node_derivatives[256] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[256] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[257] = scratch.values[138];
        scratch.node_derivatives[257] = scratch.node_derivatives[138];
        scratch.branch_derivatives[257] = scratch.branch_derivatives[138];

        if (scratch.values[139] > 0.0) {
            scratch.store_ad(258, &{
                if (scratch.values[139] < 10.0) {
                    scratch.ad_value(139)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[258] = 0.0;
            scratch.node_derivatives[258] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[258] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[140] > (-10.0)) {
            scratch.store_ad(259, &{
                if (scratch.values[140] < 10.0) {
                    scratch.ad_value(140)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[259] = (-10.0);
            scratch.node_derivatives[259] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[259] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[141] > 1.0) {
            scratch.values[260] = scratch.values[141];
            scratch.node_derivatives[260] = scratch.node_derivatives[141];
            scratch.branch_derivatives[260] = scratch.branch_derivatives[141];
        } else {
            scratch.values[260] = 1.0;
            scratch.node_derivatives[260] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[260] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[142] > 0.0) {
            scratch.values[261] = scratch.values[142];
            scratch.node_derivatives[261] = scratch.node_derivatives[142];
            scratch.branch_derivatives[261] = scratch.branch_derivatives[142];
        } else {
            scratch.values[261] = 0.0;
            scratch.node_derivatives[261] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[261] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[143] > 0.0) {
            scratch.values[262] = scratch.values[143];
            scratch.node_derivatives[262] = scratch.node_derivatives[143];
            scratch.branch_derivatives[262] = scratch.branch_derivatives[143];
        } else {
            scratch.values[262] = 0.0;
            scratch.node_derivatives[262] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[262] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[144] > 0.0) {
            scratch.values[263] = scratch.values[144];
            scratch.node_derivatives[263] = scratch.node_derivatives[144];
            scratch.branch_derivatives[263] = scratch.branch_derivatives[144];
        } else {
            scratch.values[263] = 0.0;
            scratch.node_derivatives[263] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[263] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[145] > 0.0) {
            scratch.values[264] = scratch.values[145];
            scratch.node_derivatives[264] = scratch.node_derivatives[145];
            scratch.branch_derivatives[264] = scratch.branch_derivatives[145];
        } else {
            scratch.values[264] = 0.0;
            scratch.node_derivatives[264] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[264] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[265] = scratch.values[146];
        scratch.node_derivatives[265] = scratch.node_derivatives[146];
        scratch.branch_derivatives[265] = scratch.branch_derivatives[146];

        scratch.values[266] = scratch.values[147];
        scratch.node_derivatives[266] = scratch.node_derivatives[147];
        scratch.branch_derivatives[266] = scratch.branch_derivatives[147];

        scratch.values[267] = scratch.values[148];
        scratch.node_derivatives[267] = scratch.node_derivatives[148];
        scratch.branch_derivatives[267] = scratch.branch_derivatives[148];

        scratch.values[268] = scratch.values[149];
        scratch.node_derivatives[268] = scratch.node_derivatives[149];
        scratch.branch_derivatives[268] = scratch.branch_derivatives[149];

        if (scratch.values[150] > 0.0) {
            scratch.values[269] = scratch.values[150];
            scratch.node_derivatives[269] = scratch.node_derivatives[150];
            scratch.branch_derivatives[269] = scratch.branch_derivatives[150];
        } else {
            scratch.values[269] = 0.0;
            scratch.node_derivatives[269] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[269] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[151] > 0.0) {
            scratch.values[270] = scratch.values[151];
            scratch.node_derivatives[270] = scratch.node_derivatives[151];
            scratch.branch_derivatives[270] = scratch.branch_derivatives[151];
        } else {
            scratch.values[270] = 0.0;
            scratch.node_derivatives[270] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[270] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[152] > 0.0) {
            scratch.values[271] = scratch.values[152];
            scratch.node_derivatives[271] = scratch.node_derivatives[152];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[152];
        } else {
            scratch.values[271] = 0.0;
            scratch.node_derivatives[271] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[271] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[153] > 0.0) {
            scratch.values[272] = scratch.values[153];
            scratch.node_derivatives[272] = scratch.node_derivatives[153];
            scratch.branch_derivatives[272] = scratch.branch_derivatives[153];
        } else {
            scratch.values[272] = 0.0;
            scratch.node_derivatives[272] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[272] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[154] > 0.0) {
            scratch.values[273] = scratch.values[154];
            scratch.node_derivatives[273] = scratch.node_derivatives[154];
            scratch.branch_derivatives[273] = scratch.branch_derivatives[154];
        } else {
            scratch.values[273] = 0.0;
            scratch.node_derivatives[273] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[273] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[155] > 0.0) {
            scratch.values[274] = scratch.values[155];
            scratch.node_derivatives[274] = scratch.node_derivatives[155];
            scratch.branch_derivatives[274] = scratch.branch_derivatives[155];
        } else {
            scratch.values[274] = 0.0;
            scratch.node_derivatives[274] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[274] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[156] > 0.0) {
            scratch.values[275] = scratch.values[156];
            scratch.node_derivatives[275] = scratch.node_derivatives[156];
            scratch.branch_derivatives[275] = scratch.branch_derivatives[156];
        } else {
            scratch.values[275] = 0.0;
            scratch.node_derivatives[275] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[275] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[281] = scratch.values[162];
        scratch.node_derivatives[281] = scratch.node_derivatives[162];
        scratch.branch_derivatives[281] = scratch.branch_derivatives[162];

        scratch.values[282] = scratch.values[163];
        scratch.node_derivatives[282] = scratch.node_derivatives[163];
        scratch.branch_derivatives[282] = scratch.branch_derivatives[163];

        scratch.values[283] = scratch.values[164];
        scratch.node_derivatives[283] = scratch.node_derivatives[164];
        scratch.branch_derivatives[283] = scratch.branch_derivatives[164];

        if (scratch.values[165] > 1e20) {
            scratch.store_ad(284, &{
                if (scratch.values[165] < 1e26) {
                    scratch.ad_value(165)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[284] = 1e20;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[166] > 0.0) {
            scratch.values[285] = scratch.values[166];
            scratch.node_derivatives[285] = scratch.node_derivatives[166];
            scratch.branch_derivatives[285] = scratch.branch_derivatives[166];
        } else {
            scratch.values[285] = 0.0;
            scratch.node_derivatives[285] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[285] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[167] > 0.0) {
            scratch.values[286] = scratch.values[167];
            scratch.node_derivatives[286] = scratch.node_derivatives[167];
            scratch.branch_derivatives[286] = scratch.branch_derivatives[167];
        } else {
            scratch.values[286] = 0.0;
            scratch.node_derivatives[286] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[286] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[169] > 0.0) {
            scratch.values[288] = scratch.values[169];
            scratch.node_derivatives[288] = scratch.node_derivatives[169];
            scratch.branch_derivatives[288] = scratch.branch_derivatives[169];
        } else {
            scratch.values[288] = 0.0;
            scratch.node_derivatives[288] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[288] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[170] > 0.0) {
            scratch.store_ad(289, &{
                if (scratch.values[170] < 1.0) {
                    scratch.ad_value(170)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[289] = 0.0;
            scratch.node_derivatives[289] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[289] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[171] > 0.0) {
            scratch.values[290] = scratch.values[171];
            scratch.node_derivatives[290] = scratch.node_derivatives[171];
            scratch.branch_derivatives[290] = scratch.branch_derivatives[171];
        } else {
            scratch.values[290] = 0.0;
            scratch.node_derivatives[290] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[290] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[172] > 0.0) {
            scratch.values[291] = scratch.values[172];
            scratch.node_derivatives[291] = scratch.node_derivatives[172];
            scratch.branch_derivatives[291] = scratch.branch_derivatives[172];
        } else {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[173] > 0.0) {
            scratch.values[292] = scratch.values[173];
            scratch.node_derivatives[292] = scratch.node_derivatives[173];
            scratch.branch_derivatives[292] = scratch.branch_derivatives[173];
        } else {
            scratch.values[292] = 0.0;
            scratch.node_derivatives[292] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[292] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[174] > 0.0) {
            scratch.store_ad(293, &{
                if (scratch.values[174] < 1.0) {
                    scratch.ad_value(174)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[293] = 0.0;
            scratch.node_derivatives[293] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[293] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[188] > 0.0) {
            scratch.values[307] = scratch.values[188];
            scratch.node_derivatives[307] = scratch.node_derivatives[188];
            scratch.branch_derivatives[307] = scratch.branch_derivatives[188];
        } else {
            scratch.values[307] = 0.0;
            scratch.node_derivatives[307] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[307] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((self.params.mult * scratch.values[11]) > 0.0) {
            scratch.store_ad(25, &AdValue::scale(scratch.ad_value(11), self.params.mult));
        } else {
            scratch.values[25] = 0.0;
            scratch.node_derivatives[25] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[25] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[26] = (if (self.params.factuo > 0.0) { self.params.factuo } else { 0.0 });

        scratch.values[27] = self.params.delvto;

        scratch.values[29] = self.params.delvtoedge;

        scratch.values[1298] = if (scratch.values[6] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1298] != 0.0) {
            scratch.values[213] = scratch.values[212];
            scratch.node_derivatives[213] = scratch.node_derivatives[212];
            scratch.branch_derivatives[213] = scratch.branch_derivatives[212];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[215] = scratch.values[214];
            scratch.node_derivatives[215] = scratch.node_derivatives[214];
            scratch.branch_derivatives[215] = scratch.branch_derivatives[214];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[262] = scratch.values[261];
            scratch.node_derivatives[262] = scratch.node_derivatives[261];
            scratch.branch_derivatives[262] = scratch.branch_derivatives[261];
        }

    }

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[1298] != 0.0) {
            scratch.values[264] = scratch.values[263];
            scratch.node_derivatives[264] = scratch.node_derivatives[263];
            scratch.branch_derivatives[264] = scratch.branch_derivatives[263];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[266] = scratch.values[265];
            scratch.node_derivatives[266] = scratch.node_derivatives[265];
            scratch.branch_derivatives[266] = scratch.branch_derivatives[265];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[268] = scratch.values[267];
            scratch.node_derivatives[268] = scratch.node_derivatives[267];
            scratch.branch_derivatives[268] = scratch.branch_derivatives[267];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[256] = scratch.values[255];
            scratch.node_derivatives[256] = scratch.node_derivatives[255];
            scratch.branch_derivatives[256] = scratch.branch_derivatives[255];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[271] = scratch.values[270];
            scratch.node_derivatives[271] = scratch.node_derivatives[270];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[270];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[274] = scratch.values[273];
            scratch.node_derivatives[274] = scratch.node_derivatives[273];
            scratch.branch_derivatives[274] = scratch.branch_derivatives[273];
        }

        scratch.store_ad(807, &AdValue::scale(scratch.ad_value(197), 8.8541878176e-12));

        scratch.store_ad(808, &AdValue::div(scratch.ad_value(807), scratch.ad_value(196)));

        scratch.store_ad(809, &AdValue::square(scratch.ad_value(196)));

        scratch.store_ad(810, &AdValue::scale(scratch.ad_value(808), 6.241449993689894e18));

        scratch.store_ad(811, &AdValue::mul(scratch.ad_value(199), scratch.ad_value(198)));

        if (scratch.values[811] > 1e20) {
            scratch.store_ad(811, &{
                if (scratch.values[811] < 1e26) {
                    scratch.ad_value(811)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[811] = 1e20;
            scratch.node_derivatives[811] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[811] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[812] = 0.0;

        scratch.values[1299] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1299] != 0.0) {
            scratch.store_ad(812, &AdValue::scale(AdValue::powf(scratch.ad_value(808), 0.6666666666666666), ((0.4 * 5.951993) * scratch.values[191])));
        }

        scratch.values[1300] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[1299] != 0.0) && (scratch.values[1300] != 0.0)) {
            scratch.store_ad(812, &AdValue::scale(scratch.ad_value(812), (7.448711 / 5.951993)));
        }

        scratch.store_ad(813, &AdValue::scale(scratch.ad_value(808), (1e-8 * 1.0 / (scratch.values[806]))));

        scratch.store_ad(814, &AdValue::scale(scratch.ad_value(234), 0.5));

        scratch.values[815] = 0.5;

        scratch.values[1301] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1301] != 0.0) {
            scratch.store_ad(814, &AdValue::scale(scratch.ad_value(234), 0.3333333333333333));
        }

        if (scratch.values[1301] != 0.0) {
            scratch.values[815] = 0.3333333333333333;
            scratch.node_derivatives[815] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[815] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(816, &AdValue::div_from_scalar(1.0, scratch.ad_value(243)));

        scratch.store_ad(817, &AdValue::div_from_scalar(1.0, scratch.ad_value(247)));

        scratch.store_ad(818, &AdValue::div(scratch.ad_value(807), scratch.ad_value(212)));

        scratch.store_ad(819, &AdValue::div(scratch.ad_value(807), scratch.ad_value(213)));

        scratch.store_ad(820, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(214), ((2.0 * 1.6021918e-19) * (scratch.values[806] * scratch.values[364])))), scratch.ad_value(818)));

        scratch.store_ad(821, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(215), ((2.0 * 1.6021918e-19) * (scratch.values[806] * scratch.values[364])))), scratch.ad_value(819)));

        scratch.store_ad(822, &AdValue::square(scratch.ad_value(820)));

        scratch.store_ad(823, &AdValue::square(scratch.ad_value(821)));

        scratch.store_ad(958, &AdValue::div_from_scalar(1.0, scratch.ad_value(820)));

        scratch.store_ad(959, &AdValue::offset(AdValue::scale(scratch.ad_value(820), 3.1), 8.5));

        scratch.store_ad(824, &AdValue::square(scratch.ad_value(959)));

        scratch.store_ad(960, &AdValue::scale(scratch.ad_value(959), 0.5));

        scratch.values[1302] = if (scratch.values[958] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1302] != 0.0) {
            scratch.store_ad(825, &AdValue::scale(scratch.ad_value(958), 64.0));
        }

        scratch.values[1303] = if (scratch.values[958] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1302] != 0.0)) && (scratch.values[1303] != 0.0)) {
            scratch.store_ad(825, &AdValue::offset(AdValue::scale(scratch.ad_value(958), 22.0), 3.0));
        }

        scratch.values[1304] = if (scratch.values[958] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1302] != 0.0)) && (!(scratch.values[1303] != 0.0))) && (scratch.values[1304] != 0.0)) {
            scratch.store_ad(825, &AdValue::offset(AdValue::scale(scratch.ad_value(958), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1302] != 0.0)) && (!(scratch.values[1303] != 0.0))) && (!(scratch.values[1304] != 0.0))) {
            scratch.values[825] = scratch.values[820];
            scratch.node_derivatives[825] = scratch.node_derivatives[820];
            scratch.branch_derivatives[825] = scratch.branch_derivatives[820];
        }

        scratch.store_ad(826, &AdValue::sub(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(822), 0.5)), AdValue::mul(scratch.ad_value(820), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(822), 0.25)), scratch.ad_value(825))))));

        scratch.store_ad(958, &AdValue::div_from_scalar(1.0, scratch.ad_value(821)));

        scratch.store_ad(959, &AdValue::offset(AdValue::scale(scratch.ad_value(821), 3.1), 8.5));

        scratch.store_ad(827, &AdValue::square(scratch.ad_value(959)));

        scratch.store_ad(960, &AdValue::scale(scratch.ad_value(959), 0.5));

        scratch.values[1305] = if (scratch.values[958] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1305] != 0.0) {
            scratch.store_ad(828, &AdValue::scale(scratch.ad_value(958), 64.0));
        }

        scratch.values[1306] = if (scratch.values[958] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1305] != 0.0)) && (scratch.values[1306] != 0.0)) {
            scratch.store_ad(828, &AdValue::offset(AdValue::scale(scratch.ad_value(958), 22.0), 3.0));
        }

        scratch.values[1307] = if (scratch.values[958] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1305] != 0.0)) && (!(scratch.values[1306] != 0.0))) && (scratch.values[1307] != 0.0)) {
            scratch.store_ad(828, &AdValue::offset(AdValue::scale(scratch.ad_value(958), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1305] != 0.0)) && (!(scratch.values[1306] != 0.0))) && (!(scratch.values[1307] != 0.0))) {
            scratch.values[828] = scratch.values[821];
            scratch.node_derivatives[828] = scratch.node_derivatives[821];
            scratch.branch_derivatives[828] = scratch.branch_derivatives[821];
        }

        scratch.store_ad(829, &AdValue::sub(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(823), 0.5)), AdValue::mul(scratch.ad_value(821), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(823), 0.25)), scratch.ad_value(828))))));

        scratch.store_ad(830, &AdValue::div_from_scalar(1.0, scratch.ad_value(260)));

        scratch.store_ad(831, &AdValue::scale(AdValue::sqrt(AdValue::scale(scratch.ad_value(260), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33)));

        scratch.store_ad(832, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(196)));

        scratch.store_ad(833, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(212)));

        scratch.store_ad(834, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(213)));

        scratch.values[835] = 0.0;

        scratch.values[1308] = if (scratch.values[259] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(835, &AdValue::div(AdValue::scale(scratch.ad_value(258), (-0.495)), scratch.ad_value(259)));
        }

        scratch.store_ad(836, &AdValue::pow_from_scalar(scratch.values[361], scratch.ad_value(257)));

        scratch.store_ad(254, &AdValue::mul(scratch.ad_value(254), scratch.ad_value(836)));

        scratch.store_ad(255, &AdValue::mul(scratch.ad_value(255), scratch.ad_value(836)));

        scratch.store_ad(256, &AdValue::mul(scratch.ad_value(256), scratch.ad_value(836)));

        if ((1.0 + (scratch.values[265] * scratch.values[362])) > 0.0) {
            scratch.store_ad(831, &AdValue::offset(AdValue::scale(scratch.ad_value(265), scratch.values[362]), 1.0));
        } else {
            scratch.values[831] = 0.0;
            scratch.node_derivatives[831] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[831] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(761, &AdValue::mul(scratch.ad_value(263), scratch.ad_value(831)));

        scratch.store_ad(839, &AdValue::scale(AdValue::mul(scratch.ad_value(761), scratch.ad_value(212)), 500000000.0));

        if ((1.0 + (scratch.values[266] * scratch.values[362])) > 0.0) {
            scratch.store_ad(831, &AdValue::offset(AdValue::scale(scratch.ad_value(266), scratch.values[362]), 1.0));
        } else {
            scratch.values[831] = 0.0;
            scratch.node_derivatives[831] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[831] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(762, &AdValue::mul(scratch.ad_value(264), scratch.ad_value(831)));

        scratch.store_ad(840, &AdValue::scale(AdValue::mul(scratch.ad_value(762), scratch.ad_value(213)), 500000000.0));

        scratch.values[30] = 0.0;

        scratch.values[31] = 0.0;

        scratch.values[32] = 0.0;

        scratch.values[33] = 0.0;

        scratch.values[34] = 0.0;

        scratch.values[35] = 0.0;

        scratch.values[36] = 0.0;

        scratch.values[37] = scratch.values[314];
        scratch.node_derivatives[37] = scratch.node_derivatives[314];
        scratch.branch_derivatives[37] = scratch.branch_derivatives[314];

        scratch.values[1316] = if (scratch.values[1] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1316] != 0.0) {
            scratch.values[37] = (if (scratch.values[20] > 0.0) { scratch.values[20] } else { 0.0 });
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1317] = if (scratch.values[5] == 3.0) { 1.0 } else { 0.0 };

        if (scratch.values[1317] != 0.0) {
            scratch.values[36] = 1.0;
            scratch.node_derivatives[36] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[36] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[697]));

        scratch.store_ad(31, &AdValue::scale(scratch.ad_value(12), scratch.values[698]));

        scratch.store_ad(32, &AdValue::scale(scratch.ad_value(12), scratch.values[699]));

        scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[724]));

        scratch.store_ad(34, &AdValue::scale(scratch.ad_value(12), scratch.values[725]));

        scratch.store_ad(35, &AdValue::scale(scratch.ad_value(12), scratch.values[726]));

        scratch.values[1318] = if ((scratch.values[5] == 2.0) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[700]));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(31, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[701]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.values[32] = scratch.values[37];
            scratch.node_derivatives[32] = scratch.node_derivatives[37];
            scratch.branch_derivatives[32] = scratch.branch_derivatives[37];
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[727]));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(34, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[728]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.values[35] = scratch.values[37];
            scratch.node_derivatives[35] = scratch.node_derivatives[37];
            scratch.branch_derivatives[35] = scratch.branch_derivatives[37];
        }

        scratch.values[1319] = if (((scratch.values[5] == 1.0) || (scratch.values[5] == 2.0)) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(697, &{
                if (scratch.values[30] > 0.0) {
                    scratch.ad_value(30)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(698, &{
                if (scratch.values[31] > 0.0) {
                    scratch.ad_value(31)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(699, &{
                if (scratch.values[32] > 0.0) {
                    scratch.ad_value(32)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(724, &{
                if (scratch.values[33] > 0.0) {
                    scratch.ad_value(33)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(725, &{
                if (scratch.values[34] > 0.0) {
                    scratch.ad_value(34)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(726, &{
                if (scratch.values[35] > 0.0) {
                    scratch.ad_value(35)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[697] = 0.0;
            scratch.node_derivatives[697] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[697] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[698] = 0.0;
            scratch.node_derivatives[698] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[698] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[699] = 0.0;
            scratch.node_derivatives[699] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[699] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[724] = 0.0;
            scratch.node_derivatives[724] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[724] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[725] = 0.0;
            scratch.node_derivatives[725] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[725] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[726] = 0.0;
            scratch.node_derivatives[726] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[726] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[707] = 0.0;

        scratch.values[734] = 0.0;

        scratch.values[709] = 0.0;

        scratch.values[736] = 0.0;

        scratch.values[708] = 0.0;

        scratch.values[735] = 0.0;

        scratch.values[710] = 0.0;

        scratch.values[737] = 0.0;

        scratch.values[705] = 0.0;

        scratch.values[732] = 0.0;

        scratch.values[706] = 0.0;

        scratch.values[733] = 0.0;

        scratch.values[702] = 1.0;

        scratch.values[729] = 1.0;

        scratch.values[703] = 1.0;

        scratch.values[730] = 1.0;

        scratch.values[704] = 1.0;

        scratch.values[731] = 1.0;

        scratch.values[539] = 0.0;

        scratch.values[1320] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[1321] = if ((scratch.values[443] * scratch.values[697]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1321] != 0.0)) {
            scratch.store_ad(510, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(697), scratch.values[443])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1321] != 0.0))) {
            scratch.values[510] = 100000000.0;
            scratch.node_derivatives[510] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[510] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1322] = if ((scratch.values[444] * scratch.values[698]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1322] != 0.0)) {
            scratch.store_ad(511, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(698), scratch.values[444])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1322] != 0.0))) {
            scratch.values[511] = 100000000.0;
            scratch.node_derivatives[511] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[511] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1323] = if ((scratch.values[445] * scratch.values[699]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1323] != 0.0)) {
            scratch.store_ad(512, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(699), scratch.values[445])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1323] != 0.0))) {
            scratch.values[512] = 100000000.0;
            scratch.node_derivatives[512] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[512] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(705, &AdValue::min(AdValue::min(scratch.ad_value(510), scratch.ad_value(511)), scratch.ad_value(512)));
        }

        scratch.values[1324] = if ((((scratch.values[705] * scratch.values[427])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1324] != 0.0)) {
            scratch.store_ad(706, &AdValue::exp(AdValue::scale(scratch.ad_value(705), scratch.values[427])));
        }

        scratch.values[1325] = if ((scratch.values[705] * scratch.values[427]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1324] != 0.0))) && (scratch.values[1325] != 0.0)) {
            scratch.store_ad(706, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1324] != 0.0))) && (!(scratch.values[1325] != 0.0))) {
            scratch.store_ad(706, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[452] = scratch.values[449];
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[453] = scratch.values[450];
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

    }
}
