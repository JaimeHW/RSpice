#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[1289] != 0.0) {
            scratch.values[156] = self.params.fnto;
            scratch.node_derivatives[156] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[156] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(157, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(103), self.params.fntexcl), scratch.ad_value(103)), scratch.ad_value(320)), scratch.ad_value(320)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(158, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(337), scratch.ad_value(320)), scratch.ad_value(319)), self.params.nfalw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(159, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(337), scratch.ad_value(320)), scratch.ad_value(319)), self.params.nfblw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(160, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(337), scratch.ad_value(320)), scratch.ad_value(319)), self.params.nfclw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[161] = self.params.efo;
            scratch.node_derivatives[161] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[161] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(338, &AdValue::offset(AdValue::scale(scratch.ad_value(314), self.params.wedgew), (2.0 * self.params.wedge)));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(339, &AdValue::div_from_scalar(1e-6, scratch.ad_value(338)));
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
            scratch.store_ad(168, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.stbetedgel), self.params.stbetedgeo), AdValue::scale(scratch.ad_value(320), self.params.stbetedgew)), AdValue::mul(AdValue::scale(scratch.ad_value(319), self.params.stbetedgelw), scratch.ad_value(320))));
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

        if (scratch.values[1289] != 0.0) {
            scratch.values[175] = self.params.fntedgeo;
            scratch.node_derivatives[175] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[175] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(176, &AdValue::scale(AdValue::mul(scratch.ad_value(339), scratch.ad_value(319)), self.params.nfaedgelw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(177, &AdValue::scale(AdValue::mul(scratch.ad_value(339), scratch.ad_value(319)), self.params.nfbedgelw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.store_ad(178, &AdValue::scale(AdValue::mul(scratch.ad_value(339), scratch.ad_value(319)), self.params.nfcedgelw));
        }

        if (scratch.values[1289] != 0.0) {
            scratch.values[179] = self.params.efedgeo;
            scratch.node_derivatives[179] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[179] = [0.0; Instance::BRANCH_COUNT];
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
            scratch.store_ad(126, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plalp1), self.params.poalp1), AdValue::scale(scratch.ad_value(320), self.params.pwalp1)), AdValue::scale(scratch.ad_value(344), self.params.plwalp1)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(127, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plalp2), self.params.poalp2), AdValue::scale(scratch.ad_value(320), self.params.pwalp2)), AdValue::scale(scratch.ad_value(344), self.params.plwalp2)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[128] = self.params.povp;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(129, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.pla1), self.params.poa1), AdValue::scale(scratch.ad_value(320), self.params.pwa1)), AdValue::scale(scratch.ad_value(344), self.params.plwa1)));
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

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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
            scratch.store_ad(157, &AdValue::mul(AdValue::square(scratch.ad_value(319)), AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plfntexc), self.params.pofntexc), AdValue::scale(scratch.ad_value(320), self.params.pwfntexc)), AdValue::scale(scratch.ad_value(344), self.params.plwfntexc))));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(158, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfa), self.params.ponfa), AdValue::scale(scratch.ad_value(320), self.params.pwnfa)), AdValue::scale(scratch.ad_value(344), self.params.plwnfa)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(159, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfb), self.params.ponfb), AdValue::scale(scratch.ad_value(320), self.params.pwnfb)), AdValue::scale(scratch.ad_value(344), self.params.plwnfb)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(160, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfc), self.params.ponfc), AdValue::scale(scratch.ad_value(320), self.params.pwnfc)), AdValue::scale(scratch.ad_value(344), self.params.plwnfc)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[161] = self.params.poef;
            scratch.node_derivatives[161] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[161] = [0.0; Instance::BRANCH_COUNT];
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
            scratch.store_ad(168, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plstbetedge), self.params.postbetedge), AdValue::scale(scratch.ad_value(320), self.params.pwstbetedge)), AdValue::scale(scratch.ad_value(344), self.params.plwstbetedge)));
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
            scratch.values[175] = self.params.pofntedge;
            scratch.node_derivatives[175] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[175] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(176, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfaedge), self.params.ponfaedge), AdValue::scale(scratch.ad_value(320), self.params.pwnfaedge)), AdValue::scale(scratch.ad_value(344), self.params.plwnfaedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(177, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfbedge), self.params.ponfbedge), AdValue::scale(scratch.ad_value(320), self.params.pwnfbedge)), AdValue::scale(scratch.ad_value(344), self.params.plwnfbedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(178, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plnfcedge), self.params.ponfcedge), AdValue::scale(scratch.ad_value(320), self.params.pwnfcedge)), AdValue::scale(scratch.ad_value(344), self.params.plwnfcedge)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.values[179] = self.params.poefedge;
            scratch.node_derivatives[179] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[179] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(341, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plkvthowe), self.params.pokvthowe), AdValue::scale(scratch.ad_value(320), self.params.pwkvthowe)), AdValue::scale(scratch.ad_value(344), self.params.plwkvthowe)));
        }

        if (scratch.values[1292] != 0.0) {
            scratch.store_ad(342, &AdValue::add(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(319), self.params.plkuowe), self.params.pokuowe), AdValue::scale(scratch.ad_value(320), self.params.pwkuowe)), AdValue::scale(scratch.ad_value(344), self.params.plwkuowe)));
        }

        scratch.values[1293] = if ((scratch.values[1] == 1.0) || (scratch.values[1] == 2.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(180, &AdValue::add(AdValue::add(AdValue::div(AdValue::scale(AdValue::add(AdValue::scale(scratch.ad_value(323), (0.3333333333333333 * 1.0 / (scratch.values[24]))), scratch.ad_value(324)), scratch.values[66]), AdValue::scale(scratch.ad_value(322), scratch.values[24])), AdValue::div_from_scalar((scratch.values[69] + scratch.values[70]), AdValue::mul(scratch.ad_value(323), scratch.ad_value(321)))), AdValue::scale(scratch.ad_value(11), self.params.rgo)));
        }

        scratch.values[1294] = if (self.params.swjunasym == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1293] != 0.0) && (scratch.values[1294] != 0.0)) {
            scratch.values[68] = scratch.values[67];
            scratch.node_derivatives[68] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[68] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1293] != 0.0) {
            scratch.values[181] = (self.params.nrs * scratch.values[67]);
            scratch.node_derivatives[181] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[181] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(182, &AdValue::scale(scratch.ad_value(68), self.params.nrd));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(183, &AdValue::scale(scratch.ad_value(11), self.params.rwello));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(184, &AdValue::scale(scratch.ad_value(11), self.params.rbulko));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(185, &AdValue::scale(scratch.ad_value(11), self.params.rjunso));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(186, &AdValue::scale(scratch.ad_value(11), self.params.rjundo));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(343, &AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(314), 1000000.0), AdValue::offset(AdValue::scale(scratch.ad_value(313), (self.params.rthlw * 1000000.0)), 1.0)), self.params.rthw2));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(343, &{
                if (scratch.values[343] > 1e-6) {
                    scratch.ad_value(343)
                } else {
                    AdValue::constant(1e-6)
                }
            });
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(187, &AdValue::offset(AdValue::div_from_scalar(self.params.rthw1, scratch.ad_value(343)), self.params.rtho));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.store_ad(188, &AdValue::offset(AdValue::scale(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(314), 1000000.0), AdValue::offset(AdValue::scale(scratch.ad_value(313), (self.params.cthlw * 1000000.0)), 1.0)), self.params.cthw2), self.params.cthw1), self.params.ctho));
        }

        if (scratch.values[1293] != 0.0) {
            scratch.values[189] = self.params.strtho;
            scratch.node_derivatives[189] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[189] = [0.0; Instance::BRANCH_COUNT];
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

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        if (scratch.values[126] > 0.0) {
            scratch.values[245] = scratch.values[126];
            scratch.node_derivatives[245] = scratch.node_derivatives[126];
            scratch.branch_derivatives[245] = scratch.branch_derivatives[126];
        } else {
            scratch.values[245] = 0.0;
            scratch.node_derivatives[245] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[245] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[127] > 0.0) {
            scratch.values[246] = scratch.values[127];
            scratch.node_derivatives[246] = scratch.node_derivatives[127];
            scratch.branch_derivatives[246] = scratch.branch_derivatives[127];
        } else {
            scratch.values[246] = 0.0;
            scratch.node_derivatives[246] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[246] = [0.0; Instance::BRANCH_COUNT];
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

        if (scratch.values[129] > 0.0) {
            scratch.values[248] = scratch.values[129];
            scratch.node_derivatives[248] = scratch.node_derivatives[129];
            scratch.branch_derivatives[248] = scratch.branch_derivatives[129];
        } else {
            scratch.values[248] = 0.0;
            scratch.node_derivatives[248] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[248] = [0.0; Instance::BRANCH_COUNT];
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

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        if (scratch.values[157] > 0.0) {
            scratch.values[276] = scratch.values[157];
            scratch.node_derivatives[276] = scratch.node_derivatives[157];
            scratch.branch_derivatives[276] = scratch.branch_derivatives[157];
        } else {
            scratch.values[276] = 0.0;
            scratch.node_derivatives[276] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[276] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[158] > 0.0) {
            scratch.values[277] = scratch.values[158];
            scratch.node_derivatives[277] = scratch.node_derivatives[158];
            scratch.branch_derivatives[277] = scratch.branch_derivatives[158];
        } else {
            scratch.values[277] = 0.0;
            scratch.node_derivatives[277] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[277] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[159] > 0.0) {
            scratch.values[278] = scratch.values[159];
            scratch.node_derivatives[278] = scratch.node_derivatives[159];
            scratch.branch_derivatives[278] = scratch.branch_derivatives[159];
        } else {
            scratch.values[278] = 0.0;
            scratch.node_derivatives[278] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[278] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[160] > 0.0) {
            scratch.values[279] = scratch.values[160];
            scratch.node_derivatives[279] = scratch.node_derivatives[160];
            scratch.branch_derivatives[279] = scratch.branch_derivatives[160];
        } else {
            scratch.values[279] = 0.0;
            scratch.node_derivatives[279] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[279] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[161] > 0.0) {
            scratch.values[280] = scratch.values[161];
            scratch.node_derivatives[280] = scratch.node_derivatives[161];
            scratch.branch_derivatives[280] = scratch.branch_derivatives[161];
        } else {
            scratch.values[280] = 0.0;
            scratch.node_derivatives[280] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[280] = [0.0; Instance::BRANCH_COUNT];
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

        scratch.values[287] = scratch.values[168];
        scratch.node_derivatives[287] = scratch.node_derivatives[168];
        scratch.branch_derivatives[287] = scratch.branch_derivatives[168];

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

        if (scratch.values[175] > 0.0) {
            scratch.values[294] = scratch.values[175];
            scratch.node_derivatives[294] = scratch.node_derivatives[175];
            scratch.branch_derivatives[294] = scratch.branch_derivatives[175];
        } else {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[176] > 0.0) {
            scratch.values[295] = scratch.values[176];
            scratch.node_derivatives[295] = scratch.node_derivatives[176];
            scratch.branch_derivatives[295] = scratch.branch_derivatives[176];
        } else {
            scratch.values[295] = 0.0;
            scratch.node_derivatives[295] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[295] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[177] > 0.0) {
            scratch.values[296] = scratch.values[177];
            scratch.node_derivatives[296] = scratch.node_derivatives[177];
            scratch.branch_derivatives[296] = scratch.branch_derivatives[177];
        } else {
            scratch.values[296] = 0.0;
            scratch.node_derivatives[296] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[296] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[178] > 0.0) {
            scratch.values[297] = scratch.values[178];
            scratch.node_derivatives[297] = scratch.node_derivatives[178];
            scratch.branch_derivatives[297] = scratch.branch_derivatives[178];
        } else {
            scratch.values[297] = 0.0;
            scratch.node_derivatives[297] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[297] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[179] > 0.0) {
            scratch.values[298] = scratch.values[179];
            scratch.node_derivatives[298] = scratch.node_derivatives[179];
            scratch.branch_derivatives[298] = scratch.branch_derivatives[179];
        } else {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[180] > 0.0) {
            scratch.values[299] = scratch.values[180];
            scratch.node_derivatives[299] = scratch.node_derivatives[180];
            scratch.branch_derivatives[299] = scratch.branch_derivatives[180];
        } else {
            scratch.values[299] = 0.0;
            scratch.node_derivatives[299] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[299] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[181] > 0.0) {
            scratch.values[300] = scratch.values[181];
            scratch.node_derivatives[300] = scratch.node_derivatives[181];
            scratch.branch_derivatives[300] = scratch.branch_derivatives[181];
        } else {
            scratch.values[300] = 0.0;
            scratch.node_derivatives[300] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[300] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[182] > 0.0) {
            scratch.values[301] = scratch.values[182];
            scratch.node_derivatives[301] = scratch.node_derivatives[182];
            scratch.branch_derivatives[301] = scratch.branch_derivatives[182];
        } else {
            scratch.values[301] = 0.0;
            scratch.node_derivatives[301] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[301] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[184] > 0.0) {
            scratch.values[302] = scratch.values[184];
            scratch.node_derivatives[302] = scratch.node_derivatives[184];
            scratch.branch_derivatives[302] = scratch.branch_derivatives[184];
        } else {
            scratch.values[302] = 0.0;
            scratch.node_derivatives[302] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[302] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[185] > 0.0) {
            scratch.values[303] = scratch.values[185];
            scratch.node_derivatives[303] = scratch.node_derivatives[185];
            scratch.branch_derivatives[303] = scratch.branch_derivatives[185];
        } else {
            scratch.values[303] = 0.0;
            scratch.node_derivatives[303] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[303] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[186] > 0.0) {
            scratch.values[304] = scratch.values[186];
            scratch.node_derivatives[304] = scratch.node_derivatives[186];
            scratch.branch_derivatives[304] = scratch.branch_derivatives[186];
        } else {
            scratch.values[304] = 0.0;
            scratch.node_derivatives[304] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[304] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[183] > 0.0) {
            scratch.values[305] = scratch.values[183];
            scratch.node_derivatives[305] = scratch.node_derivatives[183];
            scratch.branch_derivatives[305] = scratch.branch_derivatives[183];
        } else {
            scratch.values[305] = 0.0;
            scratch.node_derivatives[305] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[305] = [0.0; Instance::BRANCH_COUNT];
        }

    }
}
