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
        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(73, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plvfb), self.params.povfb), AdValue::scale(scratch.ad_value(314), self.params.pwvfb)), AdValue::scale(scratch.ad_value(337), self.params.plwvfb)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(74, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plstvfb), self.params.postvfb), AdValue::scale(scratch.ad_value(314), self.params.pwstvfb)), AdValue::scale(scratch.ad_value(337), self.params.plwstvfb)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[75] = self.params.post2vfb;
            scratch.node_derivatives[75] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[75] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[76] = self.params.potox;
            scratch.node_derivatives[76] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[76] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[77] = self.params.poepsrox;
            scratch.node_derivatives[77] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[77] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(78, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plneff), self.params.poneff), AdValue::scale(scratch.ad_value(314), self.params.pwneff)), AdValue::scale(scratch.ad_value(337), self.params.plwneff)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(79, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plfacneffac), self.params.pofacneffac), AdValue::scale(scratch.ad_value(314), self.params.pwfacneffac)), AdValue::scale(scratch.ad_value(337), self.params.plwfacneffac)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(80, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plgfacnud), self.params.pogfacnud), AdValue::scale(scratch.ad_value(314), self.params.pwgfacnud)), AdValue::mul(AdValue::scale(scratch.ad_value(313), self.params.plwgfacnud), scratch.ad_value(314))));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[81] = self.params.povsbnud;
            scratch.node_derivatives[81] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[81] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[82] = self.params.podvsbnud;
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[83] = self.params.povnsub;
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[84] = self.params.ponslp;
            scratch.node_derivatives[84] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[84] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[85] = self.params.podnsub;
            scratch.node_derivatives[85] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[85] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(86, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pldphib), self.params.podphib), AdValue::scale(scratch.ad_value(314), self.params.pwdphib)), AdValue::scale(scratch.ad_value(337), self.params.plwdphib)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(87, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pldelvtac), self.params.podelvtac), AdValue::scale(scratch.ad_value(314), self.params.pwdelvtac)), AdValue::scale(scratch.ad_value(337), self.params.plwdelvtac)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(88, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plnp), self.params.ponp), AdValue::scale(scratch.ad_value(314), self.params.pwnp)), AdValue::scale(scratch.ad_value(337), self.params.plwnp)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[93] = self.params.potoxov;
            scratch.node_derivatives[93] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[93] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[94] = self.params.potoxovd;
            scratch.node_derivatives[94] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[94] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(95, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plnov), self.params.ponov), AdValue::scale(scratch.ad_value(314), self.params.pwnov)), AdValue::scale(scratch.ad_value(337), self.params.plwnov)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(96, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plnovd), self.params.ponovd), AdValue::scale(scratch.ad_value(314), self.params.pwnovd)), AdValue::scale(scratch.ad_value(337), self.params.plwnovd)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(89, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plct), self.params.poct), AdValue::scale(scratch.ad_value(314), self.params.pwct)), AdValue::scale(scratch.ad_value(337), self.params.plwct)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[90] = self.params.poctg;
            scratch.node_derivatives[90] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[90] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[91] = self.params.poctb;
            scratch.node_derivatives[91] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[91] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[92] = self.params.postct;
            scratch.node_derivatives[92] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[92] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(100, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plcf), self.params.pocf), AdValue::scale(scratch.ad_value(314), self.params.pwcf)), AdValue::scale(scratch.ad_value(337), self.params.plwcf)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[101] = self.params.pocfd;
            scratch.node_derivatives[101] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[101] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[102] = self.params.pocfb;
            scratch.node_derivatives[102] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[102] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(97, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plpsce), self.params.popsce), AdValue::scale(scratch.ad_value(314), self.params.pwpsce)), AdValue::scale(scratch.ad_value(337), self.params.plwpsce)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[99] = self.params.popsceb;
            scratch.node_derivatives[99] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[99] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[98] = self.params.popsced;
            scratch.node_derivatives[98] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[98] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(103, &AdValue::mul(AdValue::mul(scratch.ad_value(339), scratch.ad_value(313)), AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plbetn), self.params.pobetn), AdValue::scale(scratch.ad_value(314), self.params.pwbetn)), AdValue::scale(scratch.ad_value(337), self.params.plwbetn))));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(104, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plstbet), self.params.postbet), AdValue::scale(scratch.ad_value(314), self.params.pwstbet)), AdValue::scale(scratch.ad_value(337), self.params.plwstbet)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(105, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plmue), self.params.pomue), AdValue::scale(scratch.ad_value(314), self.params.pwmue)), AdValue::scale(scratch.ad_value(337), self.params.plwmue)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[106] = self.params.postmue;
            scratch.node_derivatives[106] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[106] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[107] = self.params.pothemu;
            scratch.node_derivatives[107] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[107] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[108] = self.params.postthemu;
            scratch.node_derivatives[108] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[108] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(109, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plcs), self.params.pocs), AdValue::scale(scratch.ad_value(314), self.params.pwcs)), AdValue::scale(scratch.ad_value(337), self.params.plwcs)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[110] = self.params.postcs;
            scratch.node_derivatives[110] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[110] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[111] = self.params.pothecs;
            scratch.node_derivatives[111] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[111] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[112] = self.params.postthecs;
            scratch.node_derivatives[112] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[112] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(113, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plxcor), self.params.poxcor), AdValue::scale(scratch.ad_value(314), self.params.pwxcor)), AdValue::scale(scratch.ad_value(337), self.params.plwxcor)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[114] = self.params.postxcor;
            scratch.node_derivatives[114] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[114] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[115] = self.params.pofeta;
            scratch.node_derivatives[115] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[115] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(116, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plrs), self.params.pors), AdValue::scale(scratch.ad_value(314), self.params.pwrs)), AdValue::scale(scratch.ad_value(337), self.params.plwrs)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[117] = self.params.postrs;
            scratch.node_derivatives[117] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[117] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[118] = self.params.porsb;
            scratch.node_derivatives[118] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[118] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[119] = self.params.porsg;
            scratch.node_derivatives[119] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[119] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(120, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plthesat), self.params.pothesat), AdValue::scale(scratch.ad_value(314), self.params.pwthesat)), AdValue::scale(scratch.ad_value(337), self.params.plwthesat)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(121, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plstthesat), self.params.postthesat), AdValue::scale(scratch.ad_value(314), self.params.pwstthesat)), AdValue::scale(scratch.ad_value(337), self.params.plwstthesat)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(122, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plthesatb), self.params.pothesatb), AdValue::scale(scratch.ad_value(314), self.params.pwthesatb)), AdValue::scale(scratch.ad_value(337), self.params.plwthesatb)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(123, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plthesatg), self.params.pothesatg), AdValue::scale(scratch.ad_value(314), self.params.pwthesatg)), AdValue::scale(scratch.ad_value(337), self.params.plwthesatg)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(124, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plax), self.params.poax), AdValue::scale(scratch.ad_value(314), self.params.pwax)), AdValue::scale(scratch.ad_value(337), self.params.plwax)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(125, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plalp), self.params.poalp), AdValue::scale(scratch.ad_value(314), self.params.pwalp)), AdValue::scale(scratch.ad_value(337), self.params.plwalp)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[128] = self.params.povp;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[130] = self.params.poa2;
            scratch.node_derivatives[130] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[130] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[131] = self.params.posta2;
            scratch.node_derivatives[131] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[131] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(132, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pla3), self.params.poa3), AdValue::scale(scratch.ad_value(314), self.params.pwa3)), AdValue::scale(scratch.ad_value(337), self.params.plwa3)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(133, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pla4), self.params.poa4), AdValue::scale(scratch.ad_value(314), self.params.pwa4)), AdValue::scale(scratch.ad_value(337), self.params.plwa4)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[134] = self.params.pogco;
            scratch.node_derivatives[134] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[134] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(135, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(338), self.params.pliginv), self.params.poiginv), AdValue::scale(scratch.ad_value(339), self.params.pwiginv)), AdValue::scale(scratch.ad_value(340), self.params.plwiginv)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(136, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pligov), self.params.poigov), AdValue::scale(scratch.ad_value(339), self.params.pwigov)), AdValue::scale(scratch.ad_value(341), self.params.plwigov)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(137, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pligovd), self.params.poigovd), AdValue::scale(scratch.ad_value(339), self.params.pwigovd)), AdValue::scale(scratch.ad_value(341), self.params.plwigovd)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[138] = self.params.postig;
            scratch.node_derivatives[138] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[138] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[139] = self.params.pogc2;
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[140] = self.params.pogc3;
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[141] = self.params.pochib;
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(142, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plagidl), self.params.poagidl), AdValue::scale(scratch.ad_value(339), self.params.pwagidl)), AdValue::scale(scratch.ad_value(341), self.params.plwagidl)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(143, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plagidld), self.params.poagidld), AdValue::scale(scratch.ad_value(339), self.params.pwagidld)), AdValue::scale(scratch.ad_value(341), self.params.plwagidld)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[144] = self.params.pobgidl;
            scratch.node_derivatives[144] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[144] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[145] = self.params.pobgidld;
            scratch.node_derivatives[145] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[145] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[146] = self.params.postbgidl;
            scratch.node_derivatives[146] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[146] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[147] = self.params.postbgidld;
            scratch.node_derivatives[147] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[147] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[148] = self.params.pocgidl;
            scratch.node_derivatives[148] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[148] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[149] = self.params.pocgidld;
            scratch.node_derivatives[149] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[149] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(150, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(343), self.params.plcox), self.params.pocox), AdValue::scale(scratch.ad_value(344), self.params.pwcox)), AdValue::scale(scratch.ad_value(345), self.params.plwcox)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(151, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(342), self.params.plcgov), self.params.pocgov), AdValue::scale(scratch.ad_value(344), self.params.pwcgov)), AdValue::scale(scratch.ad_value(346), self.params.plwcgov)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(152, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(342), self.params.plcgovd), self.params.pocgovd), AdValue::scale(scratch.ad_value(344), self.params.pwcgovd)), AdValue::scale(scratch.ad_value(346), self.params.plwcgovd)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(153, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(348), self.params.plcgbov), self.params.pocgbov), AdValue::scale(scratch.ad_value(349), self.params.pwcgbov)), AdValue::scale(scratch.ad_value(350), self.params.plwcgbov)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(154, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(347), self.params.plcfr), self.params.pocfr), AdValue::scale(scratch.ad_value(349), self.params.pwcfr)), AdValue::scale(scratch.ad_value(351), self.params.plwcfr)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(155, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(347), self.params.plcfrd), self.params.pocfrd), AdValue::scale(scratch.ad_value(349), self.params.pwcfrd)), AdValue::scale(scratch.ad_value(351), self.params.plwcfrd)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[156] = self.params.pofnt;
            scratch.node_derivatives[156] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[156] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[162] = self.params.povfbedge;
            scratch.node_derivatives[162] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[162] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(163, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plstvfbedge), self.params.postvfbedge), AdValue::scale(scratch.ad_value(314), self.params.pwstvfbedge)), AdValue::scale(scratch.ad_value(337), self.params.plwstvfbedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(164, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.pldphibedge), self.params.podphibedge), AdValue::scale(scratch.ad_value(314), self.params.pwdphibedge)), AdValue::scale(scratch.ad_value(337), self.params.plwdphibedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(165, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plneffedge), self.params.poneffedge), AdValue::scale(scratch.ad_value(314), self.params.pwneffedge)), AdValue::scale(scratch.ad_value(337), self.params.plwneffedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(166, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plctedge), self.params.poctedge), AdValue::scale(scratch.ad_value(314), self.params.pwctedge)), AdValue::scale(scratch.ad_value(337), self.params.plwctedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(167, &AdValue::mul(scratch.ad_value(313), AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plbetnedge), self.params.pobetnedge), AdValue::scale(scratch.ad_value(314), self.params.pwbetnedge)), AdValue::scale(scratch.ad_value(337), self.params.plwbetnedge))));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(169, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plpsceedge), self.params.popsceedge), AdValue::scale(scratch.ad_value(314), self.params.pwpsceedge)), AdValue::scale(scratch.ad_value(337), self.params.plwpsceedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[170] = self.params.popscebedge;
            scratch.node_derivatives[170] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[170] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[171] = self.params.popscededge;
            scratch.node_derivatives[171] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[171] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(172, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plcfedge), self.params.pocfedge), AdValue::scale(scratch.ad_value(314), self.params.pwcfedge)), AdValue::scale(scratch.ad_value(337), self.params.plwcfedge)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[173] = self.params.pocfdedge;
            scratch.node_derivatives[173] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[173] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.values[174] = self.params.pocfbedge;
            scratch.node_derivatives[174] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[174] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(335, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plkvthowe), self.params.pokvthowe), AdValue::scale(scratch.ad_value(314), self.params.pwkvthowe)), AdValue::scale(scratch.ad_value(337), self.params.plwkvthowe)));
        }

        if (scratch.values[1291] != 0.0) {
            scratch.store_ad(336, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(313), self.params.plkuowe), self.params.pokuowe), AdValue::scale(scratch.ad_value(314), self.params.pwkuowe)), AdValue::scale(scratch.ad_value(337), self.params.plwkuowe)));
        }

        scratch.values[1292] = if ((scratch.values[1] == 1.0) || (scratch.values[1] == 2.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1292] != 0.0) {
            scratch.values[1280] = 0.0;
            scratch.node_derivatives[1280] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1280] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[1281] = 0.0;
            scratch.node_derivatives[1281] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1281] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[1279] = 0.0;
            scratch.node_derivatives[1279] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1279] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1294] = if (((scratch.values[15] > 0.0) && (scratch.values[16] > 0.0)) && ((scratch.values[11] == 1.0) || ((scratch.values[11] > 1.0) && (scratch.values[17] > 0.0)))) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let mut assign7960_loop_guard: usize = 0;
        while {
            let assign7960_cond_e6634: f64 = (scratch.values[11] - 0.5);
            let assign7960_cond_e6636: f64 = if (((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) && (scratch.values[1279] < assign7960_cond_e6634)) { 1.0 } else { 0.0 };
            assign7960_cond_e6636 != 0.0
        } {
            assign7960_loop_guard += 1;
            assert!(assign7960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
                scratch.store_ad(1280, &AdValue::add(scratch.ad_value(1280), AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[15]), AdValue::mul(scratch.ad_value(1279), AdValue::offset(scratch.ad_value(13), scratch.values[17]))))));
            }
            if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
                scratch.store_ad(1281, &AdValue::add(scratch.ad_value(1281), AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[16]), AdValue::mul(scratch.ad_value(1279), AdValue::offset(scratch.ad_value(13), scratch.values[17]))))));
            }
            if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
                scratch.store_ad(1279, &AdValue::offset(scratch.ad_value(1279), 1.0));
            }
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1265, &AdValue::mul(scratch.ad_value(1280), scratch.ad_value(12)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1266, &AdValue::mul(scratch.ad_value(1281), scratch.ad_value(12)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1267, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[55])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1268, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(13), 0.5), scratch.values[56])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1277, &{
                if ((scratch.values[13] + scratch.values[305]) > 1e-9) {
                    AdValue::add(scratch.ad_value(13), scratch.ad_value(305))
                } else {
                    AdValue::constant(1e-9)
                }
            });
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1278, &{
                if (((scratch.values[14] + scratch.values[306]) + self.params.wlod) > 1e-9) {
                    AdValue::offset(AdValue::add(scratch.ad_value(14), scratch.ad_value(306)), self.params.wlod)
                } else {
                    AdValue::constant(1e-9)
                }
            });
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1275, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1277), scratch.values[58])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1276, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1278), scratch.values[59])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1269, &AdValue::scale(AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1275), self.params.lkuo), 1.0), AdValue::scale(scratch.ad_value(1276), self.params.wkuo)), AdValue::mul(AdValue::scale(scratch.ad_value(1275), self.params.pkuo), scratch.ad_value(1276))), (1.0 + (self.params.tkuo * (scratch.values[354] - 1.0)))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1270, &AdValue::div(AdValue::scale(AdValue::add(scratch.ad_value(1265), scratch.ad_value(1266)), self.params.kuo), scratch.ad_value(1269)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1271, &AdValue::div(AdValue::scale(AdValue::add(scratch.ad_value(1267), scratch.ad_value(1268)), self.params.kuo), scratch.ad_value(1269)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1275, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1277), scratch.values[60])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1276, &AdValue::div_from_scalar(1.0, AdValue::powf(scratch.ad_value(1278), scratch.values[61])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1272, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1275), self.params.lkvtho), 1.0), AdValue::scale(scratch.ad_value(1276), self.params.wkvtho)), AdValue::mul(AdValue::scale(scratch.ad_value(1275), self.params.pkvtho), scratch.ad_value(1276))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(1273, &AdValue::sub(AdValue::sub(AdValue::add(scratch.ad_value(1265), scratch.ad_value(1266)), scratch.ad_value(1267)), scratch.ad_value(1268)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(103, &AdValue::div(AdValue::mul(scratch.ad_value(103), AdValue::offset(scratch.ad_value(1270), 1.0)), AdValue::offset(scratch.ad_value(1271), 1.0)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(120, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(120), AdValue::offset(scratch.ad_value(1270), 1.0)), AdValue::offset(AdValue::scale(scratch.ad_value(1271), scratch.values[57]), 1.0)), AdValue::mul(AdValue::offset(scratch.ad_value(1271), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1270), scratch.values[57]), 1.0))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(73, &AdValue::add(scratch.ad_value(73), AdValue::div(AdValue::scale(scratch.ad_value(1273), self.params.kvtho), scratch.ad_value(1272))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(100, &AdValue::add(scratch.ad_value(100), AdValue::div(AdValue::scale(scratch.ad_value(1273), self.params.stetao), AdValue::powf(scratch.ad_value(1272), scratch.values[62]))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(167, &AdValue::div(AdValue::mul(scratch.ad_value(167), AdValue::offset(scratch.ad_value(1270), 1.0)), AdValue::offset(scratch.ad_value(1271), 1.0)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(162, &AdValue::add(scratch.ad_value(162), AdValue::div(AdValue::scale(scratch.ad_value(1273), self.params.kvtho), scratch.ad_value(1272))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.store_ad(172, &AdValue::add(scratch.ad_value(172), AdValue::div(AdValue::scale(scratch.ad_value(1273), self.params.stetao), AdValue::powf(scratch.ad_value(1272), scratch.values[62]))));
        }

        scratch.values[1295] = if ((((scratch.values[21] > 0.0) || (scratch.values[22] > 0.0)) || (scratch.values[23] > 0.0)) || (scratch.values[18] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1296] = if (((scratch.values[21] == 0.0) && (scratch.values[22] == 0.0)) && (scratch.values[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(1273, &AdValue::offset(scratch.ad_value(14), scratch.values[18]));
        }

        if (((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1296] != 0.0)) {
            scratch.values[1274] = (1.0 / scratch.values[63]);
            scratch.node_derivatives[1274] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1274] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(21, &AdValue::div_from_scalar((scratch.values[63] * scratch.values[63]), AdValue::scale(scratch.ad_value(1273), scratch.values[18])));
        }

        if (((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(22, &AdValue::div(AdValue::sub(AdValue::scale(AdValue::exp(AdValue::scale(scratch.ad_value(1274), ((-10.0) * scratch.values[18]))), ((0.1 * scratch.values[18]) + (0.01 * scratch.values[63]))), AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(1273), 0.1), (0.01 * scratch.values[63])), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(1273), (-10.0)), scratch.ad_value(1274))))), scratch.ad_value(14)));
        }

        if (((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) && (scratch.values[1296] != 0.0)) {
            scratch.store_ad(23, &AdValue::div(AdValue::sub(AdValue::scale(AdValue::exp(AdValue::scale(scratch.ad_value(1274), ((-20.0) * scratch.values[18]))), ((0.05 * scratch.values[18]) + (0.0025 * scratch.values[63]))), AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(1273), 0.05), (0.0025 * scratch.values[63])), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(1273), (-20.0)), scratch.ad_value(1274))))), scratch.ad_value(14)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(1273, &AdValue::add(AdValue::add(scratch.ad_value(21), AdValue::scale(scratch.ad_value(22), scratch.values[64])), AdValue::scale(scratch.ad_value(23), scratch.values[65])));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(73, &AdValue::add(scratch.ad_value(73), AdValue::mul(scratch.ad_value(335), scratch.ad_value(1273))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(103, &AdValue::mul(scratch.ad_value(103), AdValue::offset(AdValue::mul(scratch.ad_value(336), scratch.ad_value(1273)), 1.0)));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(162, &AdValue::add(scratch.ad_value(162), AdValue::mul(scratch.ad_value(335), scratch.ad_value(1273))));
        }

        if ((scratch.values[1292] != 0.0) && (scratch.values[1295] != 0.0)) {
            scratch.store_ad(167, &AdValue::mul(scratch.ad_value(167), AdValue::offset(AdValue::mul(scratch.ad_value(336), scratch.ad_value(1273)), 1.0)));
        }

        scratch.values[189] = scratch.values[73];
        scratch.node_derivatives[189] = scratch.node_derivatives[73];
        scratch.branch_derivatives[189] = scratch.branch_derivatives[73];

        scratch.values[190] = scratch.values[74];
        scratch.node_derivatives[190] = scratch.node_derivatives[74];
        scratch.branch_derivatives[190] = scratch.branch_derivatives[74];

        scratch.values[191] = scratch.values[75];
        scratch.node_derivatives[191] = scratch.node_derivatives[75];
        scratch.branch_derivatives[191] = scratch.branch_derivatives[75];

        if (scratch.values[76] > 1e-10) {
            scratch.values[193] = scratch.values[76];
            scratch.node_derivatives[193] = scratch.node_derivatives[76];
            scratch.branch_derivatives[193] = scratch.branch_derivatives[76];
        } else {
            scratch.values[193] = 1e-10;
            scratch.node_derivatives[193] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[193] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[77] > 1.0) {
            scratch.values[194] = scratch.values[77];
            scratch.node_derivatives[194] = scratch.node_derivatives[77];
            scratch.branch_derivatives[194] = scratch.branch_derivatives[77];
        } else {
            scratch.values[194] = 1.0;
            scratch.node_derivatives[194] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[194] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[78] > 1e20) {
            scratch.store_ad(195, &{
                if (scratch.values[78] < 1e26) {
                    scratch.ad_value(78)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[195] = 1e20;
            scratch.node_derivatives[195] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[195] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[79] > 0.0) {
            scratch.values[196] = scratch.values[79];
            scratch.node_derivatives[196] = scratch.node_derivatives[79];
            scratch.branch_derivatives[196] = scratch.branch_derivatives[79];
        } else {
            scratch.values[196] = 0.0;
            scratch.node_derivatives[196] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[196] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[80] > 0.01) {
            scratch.values[197] = scratch.values[80];
            scratch.node_derivatives[197] = scratch.node_derivatives[80];
            scratch.branch_derivatives[197] = scratch.branch_derivatives[80];
        } else {
            scratch.values[197] = 0.01;
            scratch.node_derivatives[197] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[197] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[81] > 0.0) {
            scratch.values[198] = scratch.values[81];
            scratch.node_derivatives[198] = scratch.node_derivatives[81];
            scratch.branch_derivatives[198] = scratch.branch_derivatives[81];
        } else {
            scratch.values[198] = 0.0;
            scratch.node_derivatives[198] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[198] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[82] > 0.1) {
            scratch.values[199] = scratch.values[82];
            scratch.node_derivatives[199] = scratch.node_derivatives[82];
            scratch.branch_derivatives[199] = scratch.branch_derivatives[82];
        } else {
            scratch.values[199] = 0.1;
            scratch.node_derivatives[199] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[199] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[200] = scratch.values[83];
        scratch.node_derivatives[200] = scratch.node_derivatives[83];
        scratch.branch_derivatives[200] = scratch.branch_derivatives[83];

        if (scratch.values[84] > 0.001) {
            scratch.values[201] = scratch.values[84];
            scratch.node_derivatives[201] = scratch.node_derivatives[84];
            scratch.branch_derivatives[201] = scratch.branch_derivatives[84];
        } else {
            scratch.values[201] = 0.001;
            scratch.node_derivatives[201] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[201] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[85] > 0.0) {
            scratch.store_ad(202, &{
                if (scratch.values[85] < 1.0) {
                    scratch.ad_value(85)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[202] = 0.0;
            scratch.node_derivatives[202] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[202] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[203] = scratch.values[86];
        scratch.node_derivatives[203] = scratch.node_derivatives[86];
        scratch.branch_derivatives[203] = scratch.branch_derivatives[86];

        scratch.values[204] = scratch.values[87];
        scratch.node_derivatives[204] = scratch.node_derivatives[87];
        scratch.branch_derivatives[204] = scratch.branch_derivatives[87];

        if (scratch.values[88] > 0.0) {
            scratch.values[205] = scratch.values[88];
            scratch.node_derivatives[205] = scratch.node_derivatives[88];
            scratch.branch_derivatives[205] = scratch.branch_derivatives[88];
        } else {
            scratch.values[205] = 0.0;
            scratch.node_derivatives[205] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[205] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[93] > 1e-10) {
            scratch.values[209] = scratch.values[93];
            scratch.node_derivatives[209] = scratch.node_derivatives[93];
            scratch.branch_derivatives[209] = scratch.branch_derivatives[93];
        } else {
            scratch.values[209] = 1e-10;
            scratch.node_derivatives[209] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[209] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[94] > 1e-10) {
            scratch.values[210] = scratch.values[94];
            scratch.node_derivatives[210] = scratch.node_derivatives[94];
            scratch.branch_derivatives[210] = scratch.branch_derivatives[94];
        } else {
            scratch.values[210] = 1e-10;
            scratch.node_derivatives[210] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[210] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[95] > 1e23) {
            scratch.store_ad(211, &{
                if (scratch.values[95] < 1e27) {
                    scratch.ad_value(95)
                } else {
                    AdValue::constant(1e27)
                }
            });
        } else {
            scratch.values[211] = 1e23;
            scratch.node_derivatives[211] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[211] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[96] > 1e23) {
            scratch.store_ad(212, &{
                if (scratch.values[96] < 1e27) {
                    scratch.ad_value(96)
                } else {
                    AdValue::constant(1e27)
                }
            });
        } else {
            scratch.values[212] = 1e23;
            scratch.node_derivatives[212] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[212] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[89] > 0.0) {
            scratch.values[206] = scratch.values[89];
            scratch.node_derivatives[206] = scratch.node_derivatives[89];
            scratch.branch_derivatives[206] = scratch.branch_derivatives[89];
        } else {
            scratch.values[206] = 0.0;
            scratch.node_derivatives[206] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[206] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[90] > 0.0) {
            scratch.values[207] = scratch.values[90];
            scratch.node_derivatives[207] = scratch.node_derivatives[90];
            scratch.branch_derivatives[207] = scratch.branch_derivatives[90];
        } else {
            scratch.values[207] = 0.0;
            scratch.node_derivatives[207] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[207] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[208] = scratch.values[91];
        scratch.node_derivatives[208] = scratch.node_derivatives[91];
        scratch.branch_derivatives[208] = scratch.branch_derivatives[91];

        scratch.values[192] = scratch.values[92];
        scratch.node_derivatives[192] = scratch.node_derivatives[92];
        scratch.branch_derivatives[192] = scratch.branch_derivatives[92];

        if (scratch.values[100] > 0.0) {
            scratch.values[213] = scratch.values[100];
            scratch.node_derivatives[213] = scratch.node_derivatives[100];
            scratch.branch_derivatives[213] = scratch.branch_derivatives[100];
        } else {
            scratch.values[213] = 0.0;
            scratch.node_derivatives[213] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[213] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[101] > 0.0) {
            scratch.values[214] = scratch.values[101];
            scratch.node_derivatives[214] = scratch.node_derivatives[101];
            scratch.branch_derivatives[214] = scratch.branch_derivatives[101];
        } else {
            scratch.values[214] = 0.0;
            scratch.node_derivatives[214] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[214] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[102] > 0.0) {
            scratch.store_ad(215, &{
                if (scratch.values[102] < 1.0) {
                    scratch.ad_value(102)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[215] = 0.0;
            scratch.node_derivatives[215] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[215] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[97] > 0.0) {
            scratch.values[216] = scratch.values[97];
            scratch.node_derivatives[216] = scratch.node_derivatives[97];
            scratch.branch_derivatives[216] = scratch.branch_derivatives[97];
        } else {
            scratch.values[216] = 0.0;
            scratch.node_derivatives[216] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[216] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[99] > 0.0) {
            scratch.store_ad(217, &{
                if (scratch.values[99] < 1.0) {
                    scratch.ad_value(99)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[217] = 0.0;
            scratch.node_derivatives[217] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[217] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[98] > 0.0) {
            scratch.values[218] = scratch.values[98];
            scratch.node_derivatives[218] = scratch.node_derivatives[98];
            scratch.branch_derivatives[218] = scratch.branch_derivatives[98];
        } else {
            scratch.values[218] = 0.0;
            scratch.node_derivatives[218] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[218] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[103] > 0.0) {
            scratch.values[219] = scratch.values[103];
            scratch.node_derivatives[219] = scratch.node_derivatives[103];
            scratch.branch_derivatives[219] = scratch.branch_derivatives[103];
        } else {
            scratch.values[219] = 0.0;
            scratch.node_derivatives[219] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[219] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[220] = scratch.values[104];
        scratch.node_derivatives[220] = scratch.node_derivatives[104];
        scratch.branch_derivatives[220] = scratch.branch_derivatives[104];

        if (scratch.values[105] > 0.0) {
            scratch.values[221] = scratch.values[105];
            scratch.node_derivatives[221] = scratch.node_derivatives[105];
            scratch.branch_derivatives[221] = scratch.branch_derivatives[105];
        } else {
            scratch.values[221] = 0.0;
            scratch.node_derivatives[221] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[221] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[222] = scratch.values[106];
        scratch.node_derivatives[222] = scratch.node_derivatives[106];
        scratch.branch_derivatives[222] = scratch.branch_derivatives[106];

        if (scratch.values[107] > 0.0) {
            scratch.values[223] = scratch.values[107];
            scratch.node_derivatives[223] = scratch.node_derivatives[107];
            scratch.branch_derivatives[223] = scratch.branch_derivatives[107];
        } else {
            scratch.values[223] = 0.0;
            scratch.node_derivatives[223] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[223] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[224] = scratch.values[108];
        scratch.node_derivatives[224] = scratch.node_derivatives[108];
        scratch.branch_derivatives[224] = scratch.branch_derivatives[108];

        if (scratch.values[109] > 0.0) {
            scratch.values[225] = scratch.values[109];
            scratch.node_derivatives[225] = scratch.node_derivatives[109];
            scratch.branch_derivatives[225] = scratch.branch_derivatives[109];
        } else {
            scratch.values[225] = 0.0;
            scratch.node_derivatives[225] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[225] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[226] = scratch.values[110];
        scratch.node_derivatives[226] = scratch.node_derivatives[110];
        scratch.branch_derivatives[226] = scratch.branch_derivatives[110];

        if (scratch.values[111] > 0.0) {
            scratch.values[227] = scratch.values[111];
            scratch.node_derivatives[227] = scratch.node_derivatives[111];
            scratch.branch_derivatives[227] = scratch.branch_derivatives[111];
        } else {
            scratch.values[227] = 0.0;
            scratch.node_derivatives[227] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[227] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[228] = scratch.values[112];
        scratch.node_derivatives[228] = scratch.node_derivatives[112];
        scratch.branch_derivatives[228] = scratch.branch_derivatives[112];

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[113] > 0.0) {
            scratch.values[229] = scratch.values[113];
            scratch.node_derivatives[229] = scratch.node_derivatives[113];
            scratch.branch_derivatives[229] = scratch.branch_derivatives[113];
        } else {
            scratch.values[229] = 0.0;
            scratch.node_derivatives[229] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[229] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[230] = scratch.values[114];
        scratch.node_derivatives[230] = scratch.node_derivatives[114];
        scratch.branch_derivatives[230] = scratch.branch_derivatives[114];

        if (scratch.values[115] > 0.0) {
            scratch.values[231] = scratch.values[115];
            scratch.node_derivatives[231] = scratch.node_derivatives[115];
            scratch.branch_derivatives[231] = scratch.branch_derivatives[115];
        } else {
            scratch.values[231] = 0.0;
            scratch.node_derivatives[231] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[231] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[116] > 0.0) {
            scratch.values[232] = scratch.values[116];
            scratch.node_derivatives[232] = scratch.node_derivatives[116];
            scratch.branch_derivatives[232] = scratch.branch_derivatives[116];
        } else {
            scratch.values[232] = 0.0;
            scratch.node_derivatives[232] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[232] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[233] = scratch.values[117];
        scratch.node_derivatives[233] = scratch.node_derivatives[117];
        scratch.branch_derivatives[233] = scratch.branch_derivatives[117];

        if (scratch.values[118] > (-0.5)) {
            scratch.store_ad(234, &{
                if (scratch.values[118] < 1.0) {
                    scratch.ad_value(118)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[234] = (-0.5);
            scratch.node_derivatives[234] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[234] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[119] > (-0.5)) {
            scratch.values[235] = scratch.values[119];
            scratch.node_derivatives[235] = scratch.node_derivatives[119];
            scratch.branch_derivatives[235] = scratch.branch_derivatives[119];
        } else {
            scratch.values[235] = (-0.5);
            scratch.node_derivatives[235] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[235] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[120] > 0.0) {
            scratch.values[236] = scratch.values[120];
            scratch.node_derivatives[236] = scratch.node_derivatives[120];
            scratch.branch_derivatives[236] = scratch.branch_derivatives[120];
        } else {
            scratch.values[236] = 0.0;
            scratch.node_derivatives[236] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[236] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[237] = scratch.values[121];
        scratch.node_derivatives[237] = scratch.node_derivatives[121];
        scratch.branch_derivatives[237] = scratch.branch_derivatives[121];

        if (scratch.values[122] > (-0.5)) {
            scratch.store_ad(238, &{
                if (scratch.values[122] < 1.0) {
                    scratch.ad_value(122)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[238] = (-0.5);
            scratch.node_derivatives[238] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[238] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[123] > (-0.5)) {
            scratch.values[239] = scratch.values[123];
            scratch.node_derivatives[239] = scratch.node_derivatives[123];
            scratch.branch_derivatives[239] = scratch.branch_derivatives[123];
        } else {
            scratch.values[239] = (-0.5);
            scratch.node_derivatives[239] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[239] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[124] > 2.0) {
            scratch.values[240] = scratch.values[124];
            scratch.node_derivatives[240] = scratch.node_derivatives[124];
            scratch.branch_derivatives[240] = scratch.branch_derivatives[124];
        } else {
            scratch.values[240] = 2.0;
            scratch.node_derivatives[240] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[240] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[125] > 0.0) {
            scratch.values[241] = scratch.values[125];
            scratch.node_derivatives[241] = scratch.node_derivatives[125];
            scratch.branch_derivatives[241] = scratch.branch_derivatives[125];
        } else {
            scratch.values[241] = 0.0;
            scratch.node_derivatives[241] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[241] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[128] > 1e-10) {
            scratch.values[244] = scratch.values[128];
            scratch.node_derivatives[244] = scratch.node_derivatives[128];
            scratch.branch_derivatives[244] = scratch.branch_derivatives[128];
        } else {
            scratch.values[244] = 1e-10;
            scratch.node_derivatives[244] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[244] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[130] > 0.0) {
            scratch.values[246] = scratch.values[130];
            scratch.node_derivatives[246] = scratch.node_derivatives[130];
            scratch.branch_derivatives[246] = scratch.branch_derivatives[130];
        } else {
            scratch.values[246] = 0.0;
            scratch.node_derivatives[246] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[246] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[247] = scratch.values[131];
        scratch.node_derivatives[247] = scratch.node_derivatives[131];
        scratch.branch_derivatives[247] = scratch.branch_derivatives[131];

        if (scratch.values[132] > 0.0) {
            scratch.values[248] = scratch.values[132];
            scratch.node_derivatives[248] = scratch.node_derivatives[132];
            scratch.branch_derivatives[248] = scratch.branch_derivatives[132];
        } else {
            scratch.values[248] = 0.0;
            scratch.node_derivatives[248] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[248] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[133] > 0.0) {
            scratch.values[249] = scratch.values[133];
            scratch.node_derivatives[249] = scratch.node_derivatives[133];
            scratch.branch_derivatives[249] = scratch.branch_derivatives[133];
        } else {
            scratch.values[249] = 0.0;
            scratch.node_derivatives[249] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[249] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[134] > (-10.0)) {
            scratch.store_ad(250, &{
                if (scratch.values[134] < 10.0) {
                    scratch.ad_value(134)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[250] = (-10.0);
            scratch.node_derivatives[250] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[250] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[135] > 0.0) {
            scratch.values[251] = scratch.values[135];
            scratch.node_derivatives[251] = scratch.node_derivatives[135];
            scratch.branch_derivatives[251] = scratch.branch_derivatives[135];
        } else {
            scratch.values[251] = 0.0;
            scratch.node_derivatives[251] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[251] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[136] > 0.0) {
            scratch.values[252] = scratch.values[136];
            scratch.node_derivatives[252] = scratch.node_derivatives[136];
            scratch.branch_derivatives[252] = scratch.branch_derivatives[136];
        } else {
            scratch.values[252] = 0.0;
            scratch.node_derivatives[252] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[252] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[137] > 0.0) {
            scratch.values[253] = scratch.values[137];
            scratch.node_derivatives[253] = scratch.node_derivatives[137];
            scratch.branch_derivatives[253] = scratch.branch_derivatives[137];
        } else {
            scratch.values[253] = 0.0;
            scratch.node_derivatives[253] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[253] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[254] = scratch.values[138];
        scratch.node_derivatives[254] = scratch.node_derivatives[138];
        scratch.branch_derivatives[254] = scratch.branch_derivatives[138];

        if (scratch.values[139] > 0.0) {
            scratch.store_ad(255, &{
                if (scratch.values[139] < 10.0) {
                    scratch.ad_value(139)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[255] = 0.0;
            scratch.node_derivatives[255] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[255] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[140] > (-10.0)) {
            scratch.store_ad(256, &{
                if (scratch.values[140] < 10.0) {
                    scratch.ad_value(140)
                } else {
                    AdValue::constant(10.0)
                }
            });
        } else {
            scratch.values[256] = (-10.0);
            scratch.node_derivatives[256] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[256] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[141] > 1.0) {
            scratch.values[257] = scratch.values[141];
            scratch.node_derivatives[257] = scratch.node_derivatives[141];
            scratch.branch_derivatives[257] = scratch.branch_derivatives[141];
        } else {
            scratch.values[257] = 1.0;
            scratch.node_derivatives[257] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[257] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[142] > 0.0) {
            scratch.values[258] = scratch.values[142];
            scratch.node_derivatives[258] = scratch.node_derivatives[142];
            scratch.branch_derivatives[258] = scratch.branch_derivatives[142];
        } else {
            scratch.values[258] = 0.0;
            scratch.node_derivatives[258] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[258] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[143] > 0.0) {
            scratch.values[259] = scratch.values[143];
            scratch.node_derivatives[259] = scratch.node_derivatives[143];
            scratch.branch_derivatives[259] = scratch.branch_derivatives[143];
        } else {
            scratch.values[259] = 0.0;
            scratch.node_derivatives[259] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[259] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[144] > 0.0) {
            scratch.values[260] = scratch.values[144];
            scratch.node_derivatives[260] = scratch.node_derivatives[144];
            scratch.branch_derivatives[260] = scratch.branch_derivatives[144];
        } else {
            scratch.values[260] = 0.0;
            scratch.node_derivatives[260] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[260] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[145] > 0.0) {
            scratch.values[261] = scratch.values[145];
            scratch.node_derivatives[261] = scratch.node_derivatives[145];
            scratch.branch_derivatives[261] = scratch.branch_derivatives[145];
        } else {
            scratch.values[261] = 0.0;
            scratch.node_derivatives[261] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[261] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[262] = scratch.values[146];
        scratch.node_derivatives[262] = scratch.node_derivatives[146];
        scratch.branch_derivatives[262] = scratch.branch_derivatives[146];

        scratch.values[263] = scratch.values[147];
        scratch.node_derivatives[263] = scratch.node_derivatives[147];
        scratch.branch_derivatives[263] = scratch.branch_derivatives[147];

        scratch.values[264] = scratch.values[148];
        scratch.node_derivatives[264] = scratch.node_derivatives[148];
        scratch.branch_derivatives[264] = scratch.branch_derivatives[148];

        scratch.values[265] = scratch.values[149];
        scratch.node_derivatives[265] = scratch.node_derivatives[149];
        scratch.branch_derivatives[265] = scratch.branch_derivatives[149];

        if (scratch.values[150] > 0.0) {
            scratch.values[266] = scratch.values[150];
            scratch.node_derivatives[266] = scratch.node_derivatives[150];
            scratch.branch_derivatives[266] = scratch.branch_derivatives[150];
        } else {
            scratch.values[266] = 0.0;
            scratch.node_derivatives[266] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[266] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[151] > 0.0) {
            scratch.values[267] = scratch.values[151];
            scratch.node_derivatives[267] = scratch.node_derivatives[151];
            scratch.branch_derivatives[267] = scratch.branch_derivatives[151];
        } else {
            scratch.values[267] = 0.0;
            scratch.node_derivatives[267] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[267] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[152] > 0.0) {
            scratch.values[268] = scratch.values[152];
            scratch.node_derivatives[268] = scratch.node_derivatives[152];
            scratch.branch_derivatives[268] = scratch.branch_derivatives[152];
        } else {
            scratch.values[268] = 0.0;
            scratch.node_derivatives[268] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[268] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[153] > 0.0) {
            scratch.values[269] = scratch.values[153];
            scratch.node_derivatives[269] = scratch.node_derivatives[153];
            scratch.branch_derivatives[269] = scratch.branch_derivatives[153];
        } else {
            scratch.values[269] = 0.0;
            scratch.node_derivatives[269] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[269] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[154] > 0.0) {
            scratch.values[270] = scratch.values[154];
            scratch.node_derivatives[270] = scratch.node_derivatives[154];
            scratch.branch_derivatives[270] = scratch.branch_derivatives[154];
        } else {
            scratch.values[270] = 0.0;
            scratch.node_derivatives[270] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[270] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[155] > 0.0) {
            scratch.values[271] = scratch.values[155];
            scratch.node_derivatives[271] = scratch.node_derivatives[155];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[155];
        } else {
            scratch.values[271] = 0.0;
            scratch.node_derivatives[271] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[271] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[156] > 0.0) {
            scratch.values[272] = scratch.values[156];
            scratch.node_derivatives[272] = scratch.node_derivatives[156];
            scratch.branch_derivatives[272] = scratch.branch_derivatives[156];
        } else {
            scratch.values[272] = 0.0;
            scratch.node_derivatives[272] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[272] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[278] = scratch.values[162];
        scratch.node_derivatives[278] = scratch.node_derivatives[162];
        scratch.branch_derivatives[278] = scratch.branch_derivatives[162];

        scratch.values[279] = scratch.values[163];
        scratch.node_derivatives[279] = scratch.node_derivatives[163];
        scratch.branch_derivatives[279] = scratch.branch_derivatives[163];

        scratch.values[280] = scratch.values[164];
        scratch.node_derivatives[280] = scratch.node_derivatives[164];
        scratch.branch_derivatives[280] = scratch.branch_derivatives[164];

        if (scratch.values[165] > 1e20) {
            scratch.store_ad(281, &{
                if (scratch.values[165] < 1e26) {
                    scratch.ad_value(165)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[281] = 1e20;
            scratch.node_derivatives[281] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[281] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[166] > 0.0) {
            scratch.values[282] = scratch.values[166];
            scratch.node_derivatives[282] = scratch.node_derivatives[166];
            scratch.branch_derivatives[282] = scratch.branch_derivatives[166];
        } else {
            scratch.values[282] = 0.0;
            scratch.node_derivatives[282] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[282] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[167] > 0.0) {
            scratch.values[283] = scratch.values[167];
            scratch.node_derivatives[283] = scratch.node_derivatives[167];
            scratch.branch_derivatives[283] = scratch.branch_derivatives[167];
        } else {
            scratch.values[283] = 0.0;
            scratch.node_derivatives[283] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[283] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[169] > 0.0) {
            scratch.values[285] = scratch.values[169];
            scratch.node_derivatives[285] = scratch.node_derivatives[169];
            scratch.branch_derivatives[285] = scratch.branch_derivatives[169];
        } else {
            scratch.values[285] = 0.0;
            scratch.node_derivatives[285] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[285] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[170] > 0.0) {
            scratch.store_ad(286, &{
                if (scratch.values[170] < 1.0) {
                    scratch.ad_value(170)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[286] = 0.0;
            scratch.node_derivatives[286] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[286] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[171] > 0.0) {
            scratch.values[287] = scratch.values[171];
            scratch.node_derivatives[287] = scratch.node_derivatives[171];
            scratch.branch_derivatives[287] = scratch.branch_derivatives[171];
        } else {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[172] > 0.0) {
            scratch.values[288] = scratch.values[172];
            scratch.node_derivatives[288] = scratch.node_derivatives[172];
            scratch.branch_derivatives[288] = scratch.branch_derivatives[172];
        } else {
            scratch.values[288] = 0.0;
            scratch.node_derivatives[288] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[288] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[173] > 0.0) {
            scratch.values[289] = scratch.values[173];
            scratch.node_derivatives[289] = scratch.node_derivatives[173];
            scratch.branch_derivatives[289] = scratch.branch_derivatives[173];
        } else {
            scratch.values[289] = 0.0;
            scratch.node_derivatives[289] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[289] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[174] > 0.0) {
            scratch.store_ad(290, &{
                if (scratch.values[174] < 1.0) {
                    scratch.ad_value(174)
                } else {
                    AdValue::constant(1.0)
                }
            });
        } else {
            scratch.values[290] = 0.0;
            scratch.node_derivatives[290] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[290] = [0.0; Instance::BRANCH_COUNT];
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

        scratch.values[1297] = if (scratch.values[6] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1297] != 0.0) {
            scratch.values[210] = scratch.values[209];
            scratch.node_derivatives[210] = scratch.node_derivatives[209];
            scratch.branch_derivatives[210] = scratch.branch_derivatives[209];
        }

    }

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[1297] != 0.0) {
            scratch.values[212] = scratch.values[211];
            scratch.node_derivatives[212] = scratch.node_derivatives[211];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[211];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[259] = scratch.values[258];
            scratch.node_derivatives[259] = scratch.node_derivatives[258];
            scratch.branch_derivatives[259] = scratch.branch_derivatives[258];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[261] = scratch.values[260];
            scratch.node_derivatives[261] = scratch.node_derivatives[260];
            scratch.branch_derivatives[261] = scratch.branch_derivatives[260];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[263] = scratch.values[262];
            scratch.node_derivatives[263] = scratch.node_derivatives[262];
            scratch.branch_derivatives[263] = scratch.branch_derivatives[262];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[265] = scratch.values[264];
            scratch.node_derivatives[265] = scratch.node_derivatives[264];
            scratch.branch_derivatives[265] = scratch.branch_derivatives[264];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[253] = scratch.values[252];
            scratch.node_derivatives[253] = scratch.node_derivatives[252];
            scratch.branch_derivatives[253] = scratch.branch_derivatives[252];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[268] = scratch.values[267];
            scratch.node_derivatives[268] = scratch.node_derivatives[267];
            scratch.branch_derivatives[268] = scratch.branch_derivatives[267];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[271] = scratch.values[270];
            scratch.node_derivatives[271] = scratch.node_derivatives[270];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[270];
        }

        scratch.store_ad(811, &AdValue::scale(scratch.ad_value(194), 8.8541878176e-12));

        scratch.store_ad(812, &AdValue::div(scratch.ad_value(811), scratch.ad_value(193)));

        scratch.store_ad(813, &AdValue::square(scratch.ad_value(193)));

        scratch.store_ad(814, &AdValue::scale(scratch.ad_value(812), 6.241449993689894e18));

        scratch.store_ad(815, &AdValue::mul(scratch.ad_value(196), scratch.ad_value(195)));

        if (scratch.values[815] > 1e20) {
            scratch.store_ad(815, &{
                if (scratch.values[815] < 1e26) {
                    scratch.ad_value(815)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[815] = 1e20;
            scratch.node_derivatives[815] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[815] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[816] = 0.0;

        scratch.values[1298] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1298] != 0.0) {
            scratch.store_ad(816, &AdValue::scale(AdValue::powf(scratch.ad_value(812), 0.6666666666666666), ((0.4 * 5.951993) * scratch.values[188])));
        }

        scratch.values[1299] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[1298] != 0.0) && (scratch.values[1299] != 0.0)) {
            scratch.store_ad(816, &AdValue::scale(scratch.ad_value(816), (7.448711 / 5.951993)));
        }

        scratch.store_ad(817, &AdValue::scale(scratch.ad_value(812), (1e-8 * 1.0 / (scratch.values[810]))));

        scratch.store_ad(818, &AdValue::scale(scratch.ad_value(231), 0.5));

        scratch.values[819] = 0.5;

        scratch.values[1300] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1300] != 0.0) {
            scratch.store_ad(818, &AdValue::scale(scratch.ad_value(231), 0.3333333333333333));
        }

        if (scratch.values[1300] != 0.0) {
            scratch.values[819] = 0.3333333333333333;
            scratch.node_derivatives[819] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[819] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(820, &AdValue::div_from_scalar(1.0, scratch.ad_value(240)));

        scratch.store_ad(821, &AdValue::div_from_scalar(1.0, scratch.ad_value(244)));

        scratch.store_ad(822, &AdValue::div(scratch.ad_value(811), scratch.ad_value(209)));

        scratch.store_ad(823, &AdValue::div(scratch.ad_value(811), scratch.ad_value(210)));

        scratch.store_ad(824, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(211), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[357])))), scratch.ad_value(822)));

        scratch.store_ad(825, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(212), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[357])))), scratch.ad_value(823)));

        scratch.store_ad(826, &AdValue::square(scratch.ad_value(824)));

        scratch.store_ad(827, &AdValue::square(scratch.ad_value(825)));

        scratch.store_ad(962, &AdValue::div_from_scalar(1.0, scratch.ad_value(824)));

        scratch.store_ad(963, &AdValue::offset(AdValue::scale(scratch.ad_value(824), 3.1), 8.5));

        scratch.store_ad(828, &AdValue::square(scratch.ad_value(963)));

        scratch.store_ad(964, &AdValue::scale(scratch.ad_value(963), 0.5));

        scratch.values[1301] = if (scratch.values[962] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1301] != 0.0) {
            scratch.store_ad(829, &AdValue::scale(scratch.ad_value(962), 64.0));
        }

        scratch.values[1302] = if (scratch.values[962] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1301] != 0.0)) && (scratch.values[1302] != 0.0)) {
            scratch.store_ad(829, &AdValue::offset(AdValue::scale(scratch.ad_value(962), 22.0), 3.0));
        }

        scratch.values[1303] = if (scratch.values[962] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1301] != 0.0)) && (!(scratch.values[1302] != 0.0))) && (scratch.values[1303] != 0.0)) {
            scratch.store_ad(829, &AdValue::offset(AdValue::scale(scratch.ad_value(962), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1301] != 0.0)) && (!(scratch.values[1302] != 0.0))) && (!(scratch.values[1303] != 0.0))) {
            scratch.values[829] = scratch.values[824];
            scratch.node_derivatives[829] = scratch.node_derivatives[824];
            scratch.branch_derivatives[829] = scratch.branch_derivatives[824];
        }

        scratch.store_ad(830, &AdValue::sub(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(826), 0.5)), AdValue::mul(scratch.ad_value(824), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(826), 0.25)), scratch.ad_value(829))))));

        scratch.store_ad(962, &AdValue::div_from_scalar(1.0, scratch.ad_value(825)));

        scratch.store_ad(963, &AdValue::offset(AdValue::scale(scratch.ad_value(825), 3.1), 8.5));

        scratch.store_ad(831, &AdValue::square(scratch.ad_value(963)));

        scratch.store_ad(964, &AdValue::scale(scratch.ad_value(963), 0.5));

        scratch.values[1304] = if (scratch.values[962] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1304] != 0.0) {
            scratch.store_ad(832, &AdValue::scale(scratch.ad_value(962), 64.0));
        }

        scratch.values[1305] = if (scratch.values[962] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1304] != 0.0)) && (scratch.values[1305] != 0.0)) {
            scratch.store_ad(832, &AdValue::offset(AdValue::scale(scratch.ad_value(962), 22.0), 3.0));
        }

        scratch.values[1306] = if (scratch.values[962] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1304] != 0.0)) && (!(scratch.values[1305] != 0.0))) && (scratch.values[1306] != 0.0)) {
            scratch.store_ad(832, &AdValue::offset(AdValue::scale(scratch.ad_value(962), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1304] != 0.0)) && (!(scratch.values[1305] != 0.0))) && (!(scratch.values[1306] != 0.0))) {
            scratch.values[832] = scratch.values[825];
            scratch.node_derivatives[832] = scratch.node_derivatives[825];
            scratch.branch_derivatives[832] = scratch.branch_derivatives[825];
        }

        scratch.store_ad(833, &AdValue::sub(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(827), 0.5)), AdValue::mul(scratch.ad_value(825), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(827), 0.25)), scratch.ad_value(832))))));

        scratch.store_ad(771, &AdValue::add(AdValue::offset(scratch.ad_value(203), scratch.values[364]), AdValue::scale(AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(195), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)), (2.0 * scratch.values[759]))));

        if !(scratch.values[771] > 0.05) {
            scratch.values[771] = 0.05;
            scratch.node_derivatives[771] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[771] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(772, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(195), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));

        scratch.values[773] = 0.0;

        scratch.values[774] = 0.0;

        scratch.values[1307] = if (scratch.values[205] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(775, &AdValue::div_from_scalar(80000000.0, scratch.ad_value(813)));
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(774, &{
                if (scratch.values[205] > scratch.values[775]) {
                    scratch.ad_value(205)
                } else {
                    scratch.ad_value(775)
                }
            });
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(774, &{
                if (5e24 > scratch.values[774]) {
                    AdValue::constant(5e24)
                } else {
                    scratch.ad_value(774)
                }
            });
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(773, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(812), 2.0), scratch.ad_value(812)), scratch.values[759]), AdValue::scale(scratch.ad_value(774), (1.6021918e-19 * scratch.values[810]))));
        }

        scratch.values[776] = ((100.0 * scratch.values[759]) * scratch.values[759]);

        scratch.values[1308] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(777, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(772), scratch.values[759]), scratch.ad_value(772)), scratch.ad_value(771))));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(778, &AdValue::mul(AdValue::scale(scratch.ad_value(816), 0.75), AdValue::powf(scratch.ad_value(777), 0.6666666666666666)));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(771, &AdValue::add(scratch.ad_value(771), scratch.ad_value(778)));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(772, &AdValue::mul(scratch.ad_value(772), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(778), (2.0 * 0.6666666666666666)), scratch.ad_value(777)), 1.0)));
        }

        scratch.store_ad(779, &AdValue::sqrt(scratch.ad_value(771)));

        scratch.store_ad(780, &AdValue::scale(scratch.ad_value(771), 0.95));

        scratch.store_ad(781, &AdValue::mul(AdValue::scale(scratch.ad_value(771), 0.0025), scratch.ad_value(771)));

        scratch.values[782] = scratch.values[781];
        scratch.node_derivatives[782] = scratch.node_derivatives[781];
        scratch.branch_derivatives[782] = scratch.branch_derivatives[781];

        scratch.store_ad(783, &AdValue::scale(AdValue::sqrt(scratch.ad_value(782)), 0.5));

        scratch.store_ad(784, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(780), scratch.ad_value(783)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(780), scratch.ad_value(783)), AdValue::sub(scratch.ad_value(780), scratch.ad_value(783))), scratch.ad_value(781)))), 0.5));

        scratch.store_ad(785, &AdValue::scale(AdValue::offset(scratch.ad_value(771), scratch.values[364]), 0.5));

        scratch.store_ad(786, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(198), scratch.ad_value(771))), scratch.ad_value(779)));

        scratch.store_ad(787, &AdValue::sub(AdValue::sub(AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(198), scratch.ad_value(199)), scratch.ad_value(771))), scratch.ad_value(779)), scratch.ad_value(786)));

        scratch.store_ad(788, &AdValue::add(AdValue::add(AdValue::offset(scratch.ad_value(203), scratch.values[364]), scratch.ad_value(204)), AdValue::scale(AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(815), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)), (2.0 * scratch.values[759]))));

        if !(scratch.values[788] > 0.05) {
            scratch.values[788] = 0.05;
            scratch.node_derivatives[788] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[788] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(789, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(815), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));

        scratch.values[1309] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(777, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(789), scratch.values[759]), scratch.ad_value(789)), scratch.ad_value(788))));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(778, &AdValue::mul(AdValue::scale(scratch.ad_value(816), 0.75), AdValue::powf(scratch.ad_value(777), 0.6666666666666666)));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(788, &AdValue::add(scratch.ad_value(788), scratch.ad_value(778)));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(789, &AdValue::mul(scratch.ad_value(789), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(778), (2.0 * 0.6666666666666666)), scratch.ad_value(777)), 1.0)));
        }

        scratch.store_ad(790, &AdValue::scale(scratch.ad_value(788), 0.95));

        scratch.store_ad(791, &AdValue::mul(AdValue::scale(scratch.ad_value(788), 0.0025), scratch.ad_value(788)));

        scratch.values[792] = scratch.values[791];
        scratch.node_derivatives[792] = scratch.node_derivatives[791];
        scratch.branch_derivatives[792] = scratch.branch_derivatives[791];

        scratch.store_ad(783, &AdValue::scale(AdValue::sqrt(scratch.ad_value(792)), 0.5));

        scratch.store_ad(793, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(790), scratch.ad_value(783)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(790), scratch.ad_value(783)), AdValue::sub(scratch.ad_value(790), scratch.ad_value(783))), scratch.ad_value(791)))), 0.5));

        scratch.store_ad(744, &AdValue::offset(AdValue::add(scratch.ad_value(189), AdValue::mul(AdValue::scale(scratch.ad_value(190), scratch.values[360]), AdValue::offset(AdValue::scale(scratch.ad_value(191), scratch.values[360]), 1.0))), scratch.values[27]));

        scratch.store_ad(794, &AdValue::exp(AdValue::scale(scratch.ad_value(192), scratch.values[362])));

        scratch.store_ad(745, &AdValue::mul(scratch.ad_value(206), scratch.ad_value(794)));

        scratch.store_ad(746, &AdValue::scale(scratch.ad_value(207), 1.0 / (scratch.values[361])));

        scratch.store_ad(795, &AdValue::exp(AdValue::scale(scratch.ad_value(220), scratch.values[362])));

        scratch.store_ad(747, &AdValue::mul(scratch.ad_value(219), scratch.ad_value(795)));

        scratch.store_ad(760, &AdValue::mul(AdValue::scale(scratch.ad_value(747), scratch.values[26]), scratch.ad_value(812)));

        scratch.store_ad(749, &AdValue::mul(scratch.ad_value(223), AdValue::exp(AdValue::scale(scratch.ad_value(224), scratch.values[362]))));

        scratch.store_ad(796, &AdValue::exp(AdValue::scale(scratch.ad_value(222), scratch.values[362])));

        scratch.store_ad(748, &AdValue::mul(scratch.ad_value(221), scratch.ad_value(796)));

        scratch.store_ad(751, &AdValue::mul(scratch.ad_value(227), AdValue::exp(AdValue::scale(scratch.ad_value(228), scratch.values[362]))));

        scratch.store_ad(797, &AdValue::exp(AdValue::scale(scratch.ad_value(226), scratch.values[362])));

        scratch.store_ad(750, &AdValue::mul(scratch.ad_value(225), scratch.ad_value(797)));

        scratch.store_ad(798, &AdValue::exp(AdValue::scale(scratch.ad_value(230), scratch.values[362])));

        scratch.store_ad(752, &AdValue::mul(scratch.ad_value(229), scratch.ad_value(798)));

        scratch.store_ad(799, &AdValue::exp(AdValue::scale(scratch.ad_value(233), scratch.values[362])));

        scratch.store_ad(753, &AdValue::mul(scratch.ad_value(232), scratch.ad_value(799)));

        scratch.store_ad(800, &AdValue::mul(AdValue::scale(scratch.ad_value(760), 2.0), scratch.ad_value(753)));

        scratch.store_ad(801, &AdValue::exp(AdValue::scale(scratch.ad_value(237), scratch.values[362])));

        scratch.store_ad(764, &AdValue::mul(scratch.ad_value(236), scratch.ad_value(801)));

        scratch.store_ad(756, &AdValue::mul(scratch.ad_value(246), AdValue::exp(AdValue::scale(AdValue::neg(scratch.ad_value(247)), scratch.values[362]))));

        scratch.store_ad(763, &AdValue::scale(scratch.ad_value(272), (4.0 * (1.3806505e-23 * scratch.values[358]))));

        scratch.values[1310] = if ((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(757, &AdValue::offset(AdValue::add(scratch.ad_value(278), AdValue::scale(scratch.ad_value(279), scratch.values[360])), scratch.values[29]));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(766, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(282), scratch.values[361]), 1.0), scratch.values[759]));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(803, &AdValue::add(AdValue::offset(scratch.ad_value(280), scratch.values[364]), AdValue::mul(AdValue::scale(scratch.ad_value(766), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(281), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)))));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(803, &{
                if (scratch.values[803] > 0.05) {
                    scratch.ad_value(803)
                } else {
                    AdValue::constant(0.05)
                }
            });
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(804, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(281), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(767, &AdValue::square(scratch.ad_value(804)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(768, &AdValue::ln(scratch.ad_value(767)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(805, &AdValue::scale(scratch.ad_value(803), 0.95));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(806, &AdValue::mul(AdValue::scale(scratch.ad_value(803), 0.0025), scratch.ad_value(803)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.values[807] = scratch.values[806];
            scratch.node_derivatives[807] = scratch.node_derivatives[806];
            scratch.branch_derivatives[807] = scratch.branch_derivatives[806];
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(808, &AdValue::scale(AdValue::sqrt(scratch.ad_value(807)), 0.5));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(809, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(805), scratch.ad_value(808)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(805), scratch.ad_value(808)), AdValue::sub(scratch.ad_value(805), scratch.ad_value(808))), scratch.ad_value(806)))), 0.5));
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[757] = 0.0;
            scratch.node_derivatives[757] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[757] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[766] = scratch.values[759];
            scratch.node_derivatives[766] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[766] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[803] = 0.0;
            scratch.node_derivatives[803] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[803] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[804] = 1.0;
            scratch.node_derivatives[804] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[804] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[767] = 1.0;
            scratch.node_derivatives[767] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[767] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[768] = 0.0;
            scratch.node_derivatives[768] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[768] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[805] = 0.0;
            scratch.node_derivatives[805] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[805] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[806] = 0.0;
            scratch.node_derivatives[806] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[806] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[807] = 0.0;
            scratch.node_derivatives[807] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[807] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[808] = 0.0;
            scratch.node_derivatives[808] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[808] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[809] = 0.0;
            scratch.node_derivatives[809] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[809] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(834, &AdValue::div_from_scalar(1.0, scratch.ad_value(257)));

        scratch.store_ad(835, &AdValue::scale(AdValue::sqrt(AdValue::scale(scratch.ad_value(257), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33)));

        scratch.store_ad(836, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(193)));

        scratch.store_ad(837, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(209)));

        scratch.store_ad(838, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(210)));

        scratch.values[839] = 0.0;

        scratch.values[1311] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1311] != 0.0) {
            scratch.store_ad(839, &AdValue::div(AdValue::scale(scratch.ad_value(255), (-0.495)), scratch.ad_value(256)));
        }

        scratch.store_ad(840, &AdValue::pow_from_scalar(scratch.values[354], scratch.ad_value(254)));

        scratch.store_ad(251, &AdValue::mul(scratch.ad_value(251), scratch.ad_value(840)));

        scratch.store_ad(252, &AdValue::mul(scratch.ad_value(252), scratch.ad_value(840)));

        scratch.store_ad(253, &AdValue::mul(scratch.ad_value(253), scratch.ad_value(840)));

    }
}
