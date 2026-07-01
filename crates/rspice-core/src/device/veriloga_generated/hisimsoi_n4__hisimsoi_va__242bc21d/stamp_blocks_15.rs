#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_111(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30420_e43876, assign30420_e43876_d_n0, assign30420_e43876_d_n2, assign30420_e43876_d_n6, assign30420_e43876_d_n7, assign30420_e43876_d_n10, assign30420_e43876_d_n11, assign30420_e43876_d_n12, assign30420_e43876_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30420_e43870: f64 = (8.0 * locals.var_ac41__blk934);
        let assign30420_e43872: f64 = (assign30420_e43870 * locals.var_ac41__blk934);
        let assign30420_e43874: f64 = (assign30420_e43872 * locals.var_ac41__blk934);
        (assign30420_e43874, (((((8.0 * locals.var_ac41__blk934_dn0) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn0)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn0)), (((((8.0 * locals.var_ac41__blk934_dn2) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn2)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn2)), (((((8.0 * locals.var_ac41__blk934_dn6) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn6)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn6)), (((((8.0 * locals.var_ac41__blk934_dn7) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn7)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn7)), (((((8.0 * locals.var_ac41__blk934_dn10) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn10)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn10)), (((((8.0 * locals.var_ac41__blk934_dn11) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn11)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn11)), (((((8.0 * locals.var_ac41__blk934_dn12) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn12)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn12)), (((((8.0 * locals.var_ac41__blk934_dn17) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn17)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn17)),)
    } else {
        (locals.var_ac4__blk935, locals.var_ac4__blk935_dn0, locals.var_ac4__blk935_dn2, locals.var_ac4__blk935_dn6, locals.var_ac4__blk935_dn7, locals.var_ac4__blk935_dn10, locals.var_ac4__blk935_dn11, locals.var_ac4__blk935_dn12, locals.var_ac4__blk935_dn17,)
    }
};
        locals.var_ac4__blk935 = assign30420_e43876;
        locals.var_ac4__blk935_dn0 = assign30420_e43876_d_n0;
        locals.var_ac4__blk935_dn2 = assign30420_e43876_d_n2;
        locals.var_ac4__blk935_dn6 = assign30420_e43876_d_n6;
        locals.var_ac4__blk935_dn7 = assign30420_e43876_d_n7;
        locals.var_ac4__blk935_dn10 = assign30420_e43876_d_n10;
        locals.var_ac4__blk935_dn11 = assign30420_e43876_d_n11;
        locals.var_ac4__blk935_dn12 = assign30420_e43876_d_n12;
        locals.var_ac4__blk935_dn17 = assign30420_e43876_d_n17;
        locals.var_ac4__blk935_rv = 0.0;

        let (assign30430_e43889, assign30430_e43889_d_n0, assign30430_e43889_d_n2, assign30430_e43889_d_n6, assign30430_e43889_d_n7, assign30430_e43889_d_n10, assign30430_e43889_d_n11, assign30430_e43889_d_n12, assign30430_e43889_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30430_e43887: f64 = (locals.var_eg - locals.var_pb2over__blk932);
        (assign30430_e43887, (locals.var_eg_dn0 - locals.var_pb2over__blk932_dn0), (locals.var_eg_dn2 - locals.var_pb2over__blk932_dn2), (locals.var_eg_dn6 - locals.var_pb2over__blk932_dn6), (locals.var_eg_dn7 - locals.var_pb2over__blk932_dn7), (locals.var_eg_dn10 - locals.var_pb2over__blk932_dn10), (locals.var_eg_dn11 - locals.var_pb2over__blk932_dn11), (locals.var_eg_dn12 - locals.var_pb2over__blk932_dn12), (locals.var_eg_dn17 - locals.var_pb2over__blk932_dn17),)
    } else {
        (locals.var_ps0_min__blk936, locals.var_ps0_min__blk936_dn0, locals.var_ps0_min__blk936_dn2, locals.var_ps0_min__blk936_dn6, locals.var_ps0_min__blk936_dn7, locals.var_ps0_min__blk936_dn10, locals.var_ps0_min__blk936_dn11, locals.var_ps0_min__blk936_dn12, locals.var_ps0_min__blk936_dn17,)
    }
};
        locals.var_ps0_min__blk936 = assign30430_e43889;
        locals.var_ps0_min__blk936_dn0 = assign30430_e43889_d_n0;
        locals.var_ps0_min__blk936_dn2 = assign30430_e43889_d_n2;
        locals.var_ps0_min__blk936_dn6 = assign30430_e43889_d_n6;
        locals.var_ps0_min__blk936_dn7 = assign30430_e43889_d_n7;
        locals.var_ps0_min__blk936_dn10 = assign30430_e43889_d_n10;
        locals.var_ps0_min__blk936_dn11 = assign30430_e43889_d_n11;
        locals.var_ps0_min__blk936_dn12 = assign30430_e43889_d_n12;
        locals.var_ps0_min__blk936_dn17 = assign30430_e43889_d_n17;
        locals.var_ps0_min__blk936_rv = 0.0;

        let (assign30440_e43904, assign30440_e43904_d_n0, assign30440_e43904_d_n2, assign30440_e43904_d_n6, assign30440_e43904_d_n7, assign30440_e43904_d_n10, assign30440_e43904_d_n11, assign30440_e43904_d_n12, assign30440_e43904_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30440_e43901: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30440_e43902: f64 = (locals.var_beta * assign30440_e43901);
        (assign30440_e43902, (locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30440_e43901) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30440_e43904;
        locals.var_tx__blk904_dn0 = assign30440_e43904_d_n0;
        locals.var_tx__blk904_dn2 = assign30440_e43904_d_n2;
        locals.var_tx__blk904_dn6 = assign30440_e43904_d_n6;
        locals.var_tx__blk904_dn7 = assign30440_e43904_d_n7;
        locals.var_tx__blk904_dn10 = assign30440_e43904_d_n10;
        locals.var_tx__blk904_dn11 = assign30440_e43904_d_n11;
        locals.var_tx__blk904_dn12 = assign30440_e43904_d_n12;
        locals.var_tx__blk904_dn17 = assign30440_e43904_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign30450_e43925, assign30450_e43925_d_n0, assign30450_e43925_d_n2, assign30450_e43925_d_n6, assign30450_e43925_d_n7, assign30450_e43925_d_n10, assign30450_e43925_d_n11, assign30450_e43925_d_n12, assign30450_e43925_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30450_e43915: f64 = (7.0 * 1.414213562373095);
        let assign30450_e43918: f64 = (9.0 * locals.var_ty__blk905);
        let assign30450_e43921: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30450_e43922: f64 = (assign30450_e43918 * assign30450_e43921);
        let assign30450_e43923: f64 = (assign30450_e43915 - assign30450_e43922);
        (assign30450_e43923, (-(((9.0 * locals.var_ty__blk905_dn0) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn0))), (-(((9.0 * locals.var_ty__blk905_dn2) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn2))), (-(((9.0 * locals.var_ty__blk905_dn6) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn6))), (-(((9.0 * locals.var_ty__blk905_dn7) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn7))), (-(((9.0 * locals.var_ty__blk905_dn10) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn10))), (-(((9.0 * locals.var_ty__blk905_dn11) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn11))), (-(((9.0 * locals.var_ty__blk905_dn12) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn12))), (-(((9.0 * locals.var_ty__blk905_dn17) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac31__blk937, locals.var_ac31__blk937_dn0, locals.var_ac31__blk937_dn2, locals.var_ac31__blk937_dn6, locals.var_ac31__blk937_dn7, locals.var_ac31__blk937_dn10, locals.var_ac31__blk937_dn11, locals.var_ac31__blk937_dn12, locals.var_ac31__blk937_dn17,)
    }
};
        locals.var_ac31__blk937 = assign30450_e43925;
        locals.var_ac31__blk937_dn0 = assign30450_e43925_d_n0;
        locals.var_ac31__blk937_dn2 = assign30450_e43925_d_n2;
        locals.var_ac31__blk937_dn6 = assign30450_e43925_d_n6;
        locals.var_ac31__blk937_dn7 = assign30450_e43925_d_n7;
        locals.var_ac31__blk937_dn10 = assign30450_e43925_d_n10;
        locals.var_ac31__blk937_dn11 = assign30450_e43925_d_n11;
        locals.var_ac31__blk937_dn12 = assign30450_e43925_d_n12;
        locals.var_ac31__blk937_dn17 = assign30450_e43925_d_n17;
        locals.var_ac31__blk937_rv = 0.0;

        let (assign30460_e43938, assign30460_e43938_d_n0, assign30460_e43938_d_n2, assign30460_e43938_d_n6, assign30460_e43938_d_n7, assign30460_e43938_d_n10, assign30460_e43938_d_n11, assign30460_e43938_d_n12, assign30460_e43938_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30460_e43936: f64 = (locals.var_ac31__blk937 * locals.var_ac31__blk937);
        (assign30460_e43936, ((locals.var_ac31__blk937_dn0 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn0)), ((locals.var_ac31__blk937_dn2 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn2)), ((locals.var_ac31__blk937_dn6 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn6)), ((locals.var_ac31__blk937_dn7 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn7)), ((locals.var_ac31__blk937_dn10 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn10)), ((locals.var_ac31__blk937_dn11 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn11)), ((locals.var_ac31__blk937_dn12 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn12)), ((locals.var_ac31__blk937_dn17 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn17)),)
    } else {
        (locals.var_ac3__blk938, locals.var_ac3__blk938_dn0, locals.var_ac3__blk938_dn2, locals.var_ac3__blk938_dn6, locals.var_ac3__blk938_dn7, locals.var_ac3__blk938_dn10, locals.var_ac3__blk938_dn11, locals.var_ac3__blk938_dn12, locals.var_ac3__blk938_dn17,)
    }
};
        locals.var_ac3__blk938 = assign30460_e43938;
        locals.var_ac3__blk938_dn0 = assign30460_e43938_d_n0;
        locals.var_ac3__blk938_dn2 = assign30460_e43938_d_n2;
        locals.var_ac3__blk938_dn6 = assign30460_e43938_d_n6;
        locals.var_ac3__blk938_dn7 = assign30460_e43938_d_n7;
        locals.var_ac3__blk938_dn10 = assign30460_e43938_d_n10;
        locals.var_ac3__blk938_dn11 = assign30460_e43938_d_n11;
        locals.var_ac3__blk938_dn12 = assign30460_e43938_d_n12;
        locals.var_ac3__blk938_dn17 = assign30460_e43938_d_n17;
        locals.var_ac3__blk938_rv = 0.0;

        let assign30470_e43942: f64 = (locals.var_ac3__blk938 * 1e-8);
        let assign30470_e43943: f64 = if locals.var_ac4__blk935 < assign30470_e43942 { 1.0 } else { 0.0 };
        locals.var_guard1003 = assign30470_e43943;
        locals.var_guard1003_rv = 0.0;

        let (assign30480_e43975, assign30480_e43975_d_n0, assign30480_e43975_d_n2, assign30480_e43975_d_n6, assign30480_e43975_d_n7, assign30480_e43975_d_n10, assign30480_e43975_d_n11, assign30480_e43975_d_n12, assign30480_e43975_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30480_e43955: f64 = (-7.0);
        let assign30480_e43957: f64 = (assign30480_e43955 * 1.414213562373095);
        let assign30480_e43959: f64 = (assign30480_e43957 + locals.var_ac31__blk937);
        let assign30480_e43962: f64 = (0.5 * locals.var_ac4__blk935);
        let assign30480_e43964: f64 = (assign30480_e43962 / locals.var_ac31__blk937);
        let assign30480_e43965: f64 = (assign30480_e43959 + assign30480_e43964);
        let assign30480_e43968: f64 = (9.0 * locals.var_ty__blk905);
        let assign30480_e43971: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30480_e43972: f64 = (assign30480_e43968 * assign30480_e43971);
        let assign30480_e43973: f64 = (assign30480_e43965 + assign30480_e43972);
        (assign30480_e43973, ((locals.var_ac31__blk937_dn0 + ((((0.5 * locals.var_ac4__blk935_dn0) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn0)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn0) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn0))), ((locals.var_ac31__blk937_dn2 + ((((0.5 * locals.var_ac4__blk935_dn2) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn2)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn2) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn2))), ((locals.var_ac31__blk937_dn6 + ((((0.5 * locals.var_ac4__blk935_dn6) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn6)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn6) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn6))), ((locals.var_ac31__blk937_dn7 + ((((0.5 * locals.var_ac4__blk935_dn7) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn7)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn7) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn7))), ((locals.var_ac31__blk937_dn10 + ((((0.5 * locals.var_ac4__blk935_dn10) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn10)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn10) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn10))), ((locals.var_ac31__blk937_dn11 + ((((0.5 * locals.var_ac4__blk935_dn11) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn11)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn11) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn11))), ((locals.var_ac31__blk937_dn12 + ((((0.5 * locals.var_ac4__blk935_dn12) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn12)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn12) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn12))), ((locals.var_ac31__blk937_dn17 + ((((0.5 * locals.var_ac4__blk935_dn17) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn17)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn17) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign30480_e43975;
        locals.var_ac1__blk940_dn0 = assign30480_e43975_d_n0;
        locals.var_ac1__blk940_dn2 = assign30480_e43975_d_n2;
        locals.var_ac1__blk940_dn6 = assign30480_e43975_d_n6;
        locals.var_ac1__blk940_dn7 = assign30480_e43975_d_n7;
        locals.var_ac1__blk940_dn10 = assign30480_e43975_d_n10;
        locals.var_ac1__blk940_dn11 = assign30480_e43975_d_n11;
        locals.var_ac1__blk940_dn12 = assign30480_e43975_d_n12;
        locals.var_ac1__blk940_dn17 = assign30480_e43975_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign30490_e43992, assign30490_e43992_d_n0, assign30490_e43992_d_n2, assign30490_e43992_d_n6, assign30490_e43992_d_n7, assign30490_e43992_d_n10, assign30490_e43992_d_n11, assign30490_e43992_d_n12, assign30490_e43992_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 == 0.0)) {
        let assign30490_e43989: f64 = (locals.var_ac4__blk935 + locals.var_ac3__blk938);
        let assign30490_e43990: f64 = (assign30490_e43989).sqrt();
        (assign30490_e43990, ((locals.var_ac4__blk935_dn0 + locals.var_ac3__blk938_dn0) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn2 + locals.var_ac3__blk938_dn2) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn6 + locals.var_ac3__blk938_dn6) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn7 + locals.var_ac3__blk938_dn7) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn10 + locals.var_ac3__blk938_dn10) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn11 + locals.var_ac3__blk938_dn11) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn12 + locals.var_ac3__blk938_dn12) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn17 + locals.var_ac3__blk938_dn17) / (2.0 * assign30490_e43990)),)
    } else {
        (locals.var_ac2__blk939, locals.var_ac2__blk939_dn0, locals.var_ac2__blk939_dn2, locals.var_ac2__blk939_dn6, locals.var_ac2__blk939_dn7, locals.var_ac2__blk939_dn10, locals.var_ac2__blk939_dn11, locals.var_ac2__blk939_dn12, locals.var_ac2__blk939_dn17,)
    }
};
        locals.var_ac2__blk939 = assign30490_e43992;
        locals.var_ac2__blk939_dn0 = assign30490_e43992_d_n0;
        locals.var_ac2__blk939_dn2 = assign30490_e43992_d_n2;
        locals.var_ac2__blk939_dn6 = assign30490_e43992_d_n6;
        locals.var_ac2__blk939_dn7 = assign30490_e43992_d_n7;
        locals.var_ac2__blk939_dn10 = assign30490_e43992_d_n10;
        locals.var_ac2__blk939_dn11 = assign30490_e43992_d_n11;
        locals.var_ac2__blk939_dn12 = assign30490_e43992_d_n12;
        locals.var_ac2__blk939_dn17 = assign30490_e43992_d_n17;
        locals.var_ac2__blk939_rv = 0.0;

        let (assign30500_e44019, assign30500_e44019_d_n0, assign30500_e44019_d_n2, assign30500_e44019_d_n6, assign30500_e44019_d_n7, assign30500_e44019_d_n10, assign30500_e44019_d_n11, assign30500_e44019_d_n12, assign30500_e44019_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 == 0.0)) {
        let assign30500_e44005: f64 = (-7.0);
        let assign30500_e44007: f64 = (assign30500_e44005 * 1.414213562373095);
        let assign30500_e44009: f64 = (assign30500_e44007 + locals.var_ac2__blk939);
        let assign30500_e44012: f64 = (9.0 * locals.var_ty__blk905);
        let assign30500_e44015: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30500_e44016: f64 = (assign30500_e44012 * assign30500_e44015);
        let assign30500_e44017: f64 = (assign30500_e44009 + assign30500_e44016);
        (assign30500_e44017, (locals.var_ac2__blk939_dn0 + (((9.0 * locals.var_ty__blk905_dn0) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn0))), (locals.var_ac2__blk939_dn2 + (((9.0 * locals.var_ty__blk905_dn2) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn2))), (locals.var_ac2__blk939_dn6 + (((9.0 * locals.var_ty__blk905_dn6) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn6))), (locals.var_ac2__blk939_dn7 + (((9.0 * locals.var_ty__blk905_dn7) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn7))), (locals.var_ac2__blk939_dn10 + (((9.0 * locals.var_ty__blk905_dn10) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn10))), (locals.var_ac2__blk939_dn11 + (((9.0 * locals.var_ty__blk905_dn11) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn11))), (locals.var_ac2__blk939_dn12 + (((9.0 * locals.var_ty__blk905_dn12) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn12))), (locals.var_ac2__blk939_dn17 + (((9.0 * locals.var_ty__blk905_dn17) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign30500_e44019;
        locals.var_ac1__blk940_dn0 = assign30500_e44019_d_n0;
        locals.var_ac1__blk940_dn2 = assign30500_e44019_d_n2;
        locals.var_ac1__blk940_dn6 = assign30500_e44019_d_n6;
        locals.var_ac1__blk940_dn7 = assign30500_e44019_d_n7;
        locals.var_ac1__blk940_dn10 = assign30500_e44019_d_n10;
        locals.var_ac1__blk940_dn11 = assign30500_e44019_d_n11;
        locals.var_ac1__blk940_dn12 = assign30500_e44019_d_n12;
        locals.var_ac1__blk940_dn17 = assign30500_e44019_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign30510_e44032, assign30510_e44032_d_n0, assign30510_e44032_d_n2, assign30510_e44032_d_n6, assign30510_e44032_d_n7, assign30510_e44032_d_n10, assign30510_e44032_d_n11, assign30510_e44032_d_n12, assign30510_e44032_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30510_e44030: f64 = (locals.var_ac1__blk940).powf(0.3333333333333333);
        (assign30510_e44030, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn0)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn0 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn2)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn2 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn6)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn6 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn7)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn7 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn10)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn10 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn11)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn11 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn12)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn12 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn17)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn17 / locals.var_ac1__blk940))) },)
    } else {
        (locals.var_acd__blk941, locals.var_acd__blk941_dn0, locals.var_acd__blk941_dn2, locals.var_acd__blk941_dn6, locals.var_acd__blk941_dn7, locals.var_acd__blk941_dn10, locals.var_acd__blk941_dn11, locals.var_acd__blk941_dn12, locals.var_acd__blk941_dn17,)
    }
};
        locals.var_acd__blk941 = assign30510_e44032;
        locals.var_acd__blk941_dn0 = assign30510_e44032_d_n0;
        locals.var_acd__blk941_dn2 = assign30510_e44032_d_n2;
        locals.var_acd__blk941_dn6 = assign30510_e44032_d_n6;
        locals.var_acd__blk941_dn7 = assign30510_e44032_d_n7;
        locals.var_acd__blk941_dn10 = assign30510_e44032_d_n10;
        locals.var_acd__blk941_dn11 = assign30510_e44032_d_n11;
        locals.var_acd__blk941_dn12 = assign30510_e44032_d_n12;
        locals.var_acd__blk941_dn17 = assign30510_e44032_d_n17;
        locals.var_acd__blk941_rv = 0.0;

        let (assign30520_e44060, assign30520_e44060_d_n0, assign30520_e44060_d_n2, assign30520_e44060_d_n6, assign30520_e44060_d_n7, assign30520_e44060_d_n10, assign30520_e44060_d_n11, assign30520_e44060_d_n12, assign30520_e44060_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30520_e44042: f64 = (-4.0);
        let assign30520_e44044: f64 = (assign30520_e44042 * 1.414213562373095);
        let assign30520_e44047: f64 = (12.0 * locals.var_ty__blk905);
        let assign30520_e44048: f64 = (assign30520_e44044 - assign30520_e44047);
        let assign30520_e44051: f64 = (2.0 * locals.var_acd__blk941);
        let assign30520_e44052: f64 = (assign30520_e44048 + assign30520_e44051);
        let assign30520_e44055: f64 = (1.414213562373095 * locals.var_acd__blk941);
        let assign30520_e44057: f64 = (assign30520_e44055 * locals.var_acd__blk941);
        let assign30520_e44058: f64 = (assign30520_e44052 + assign30520_e44057);
        (assign30520_e44058, (((-(12.0 * locals.var_ty__blk905_dn0)) + (2.0 * locals.var_acd__blk941_dn0)) + (((1.414213562373095 * locals.var_acd__blk941_dn0) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn0))), (((-(12.0 * locals.var_ty__blk905_dn2)) + (2.0 * locals.var_acd__blk941_dn2)) + (((1.414213562373095 * locals.var_acd__blk941_dn2) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn2))), (((-(12.0 * locals.var_ty__blk905_dn6)) + (2.0 * locals.var_acd__blk941_dn6)) + (((1.414213562373095 * locals.var_acd__blk941_dn6) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn6))), (((-(12.0 * locals.var_ty__blk905_dn7)) + (2.0 * locals.var_acd__blk941_dn7)) + (((1.414213562373095 * locals.var_acd__blk941_dn7) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn7))), (((-(12.0 * locals.var_ty__blk905_dn10)) + (2.0 * locals.var_acd__blk941_dn10)) + (((1.414213562373095 * locals.var_acd__blk941_dn10) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn10))), (((-(12.0 * locals.var_ty__blk905_dn11)) + (2.0 * locals.var_acd__blk941_dn11)) + (((1.414213562373095 * locals.var_acd__blk941_dn11) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn11))), (((-(12.0 * locals.var_ty__blk905_dn12)) + (2.0 * locals.var_acd__blk941_dn12)) + (((1.414213562373095 * locals.var_acd__blk941_dn12) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn12))), (((-(12.0 * locals.var_ty__blk905_dn17)) + (2.0 * locals.var_acd__blk941_dn17)) + (((1.414213562373095 * locals.var_acd__blk941_dn17) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn17))),)
    } else {
        (locals.var_acn__blk942, locals.var_acn__blk942_dn0, locals.var_acn__blk942_dn2, locals.var_acn__blk942_dn6, locals.var_acn__blk942_dn7, locals.var_acn__blk942_dn10, locals.var_acn__blk942_dn11, locals.var_acn__blk942_dn12, locals.var_acn__blk942_dn17,)
    }
};
        locals.var_acn__blk942 = assign30520_e44060;
        locals.var_acn__blk942_dn0 = assign30520_e44060_d_n0;
        locals.var_acn__blk942_dn2 = assign30520_e44060_d_n2;
        locals.var_acn__blk942_dn6 = assign30520_e44060_d_n6;
        locals.var_acn__blk942_dn7 = assign30520_e44060_d_n7;
        locals.var_acn__blk942_dn10 = assign30520_e44060_d_n10;
        locals.var_acn__blk942_dn11 = assign30520_e44060_d_n11;
        locals.var_acn__blk942_dn12 = assign30520_e44060_d_n12;
        locals.var_acn__blk942_dn17 = assign30520_e44060_d_n17;
        locals.var_acn__blk942_rv = 0.0;

        let (assign30530_e44073, assign30530_e44073_d_n0, assign30530_e44073_d_n2, assign30530_e44073_d_n6, assign30530_e44073_d_n7, assign30530_e44073_d_n10, assign30530_e44073_d_n11, assign30530_e44073_d_n12, assign30530_e44073_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30530_e44071: f64 = (locals.var_acn__blk942 / locals.var_acd__blk941);
        (assign30530_e44071, (((locals.var_acn__blk942_dn0 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn0)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn2 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn2)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn6 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn6)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn7 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn7)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn10 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn10)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn11 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn11)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn12 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn12)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn17 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn17)) / (locals.var_acd__blk941 * locals.var_acd__blk941)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30530_e44073;
        locals.var_chi__blk943_dn0 = assign30530_e44073_d_n0;
        locals.var_chi__blk943_dn2 = assign30530_e44073_d_n2;
        locals.var_chi__blk943_dn6 = assign30530_e44073_d_n6;
        locals.var_chi__blk943_dn7 = assign30530_e44073_d_n7;
        locals.var_chi__blk943_dn10 = assign30530_e44073_d_n10;
        locals.var_chi__blk943_dn11 = assign30530_e44073_d_n11;
        locals.var_chi__blk943_dn12 = assign30530_e44073_d_n12;
        locals.var_chi__blk943_dn17 = assign30530_e44073_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign30540_e44088, assign30540_e44088_d_n0, assign30540_e44088_d_n2, assign30540_e44088_d_n6, assign30540_e44088_d_n7, assign30540_e44088_d_n10, assign30540_e44088_d_n11, assign30540_e44088_d_n12, assign30540_e44088_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30540_e44084: f64 = (locals.var_chi__blk943 * locals.var_beta_inv);
        let assign30540_e44086: f64 = (assign30540_e44084 - locals.var_vxbgmtcl__blk921);
        (assign30540_e44086, ((locals.var_chi__blk943_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_chi__blk943_dn10 * locals.var_beta_inv) + (locals.var_chi__blk943 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_psa__blk944, locals.var_psa__blk944_dn0, locals.var_psa__blk944_dn2, locals.var_psa__blk944_dn6, locals.var_psa__blk944_dn7, locals.var_psa__blk944_dn10, locals.var_psa__blk944_dn11, locals.var_psa__blk944_dn12, locals.var_psa__blk944_dn17,)
    }
};
        locals.var_psa__blk944 = assign30540_e44088;
        locals.var_psa__blk944_dn0 = assign30540_e44088_d_n0;
        locals.var_psa__blk944_dn2 = assign30540_e44088_d_n2;
        locals.var_psa__blk944_dn6 = assign30540_e44088_d_n6;
        locals.var_psa__blk944_dn7 = assign30540_e44088_d_n7;
        locals.var_psa__blk944_dn10 = assign30540_e44088_d_n10;
        locals.var_psa__blk944_dn11 = assign30540_e44088_d_n11;
        locals.var_psa__blk944_dn12 = assign30540_e44088_d_n12;
        locals.var_psa__blk944_dn17 = assign30540_e44088_d_n17;
        locals.var_psa__blk944_rv = 0.0;

        let (assign30550_e44101, assign30550_e44101_d_n0, assign30550_e44101_d_n2, assign30550_e44101_d_n6, assign30550_e44101_d_n7, assign30550_e44101_d_n10, assign30550_e44101_d_n11, assign30550_e44101_d_n12, assign30550_e44101_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30550_e44099: f64 = (locals.var_psa__blk944 + locals.var_vxbgmtcl__blk921);
        (assign30550_e44099, (locals.var_psa__blk944_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_psa__blk944_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_psa__blk944_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_psa__blk944_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_psa__blk944_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_psa__blk944_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_psa__blk944_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_psa__blk944_dn17 + locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign30550_e44101;
        locals.var_t1__blk896_dn0 = assign30550_e44101_d_n0;
        locals.var_t1__blk896_dn2 = assign30550_e44101_d_n2;
        locals.var_t1__blk896_dn6 = assign30550_e44101_d_n6;
        locals.var_t1__blk896_dn7 = assign30550_e44101_d_n7;
        locals.var_t1__blk896_dn10 = assign30550_e44101_d_n10;
        locals.var_t1__blk896_dn11 = assign30550_e44101_d_n11;
        locals.var_t1__blk896_dn12 = assign30550_e44101_d_n12;
        locals.var_t1__blk896_dn17 = assign30550_e44101_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign30560_e44114, assign30560_e44114_d_n0, assign30560_e44114_d_n2, assign30560_e44114_d_n6, assign30560_e44114_d_n7, assign30560_e44114_d_n10, assign30560_e44114_d_n11, assign30560_e44114_d_n12, assign30560_e44114_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30560_e44112: f64 = (locals.var_t1__blk896 / locals.var_ps0_min__blk936);
        (assign30560_e44112, (((locals.var_t1__blk896_dn0 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn0)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn2 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn2)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn6 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn6)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn7 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn7)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn10 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn10)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn11 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn11)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn12 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn12)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn17 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn17)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign30560_e44114;
        locals.var_t2__blk897_dn0 = assign30560_e44114_d_n0;
        locals.var_t2__blk897_dn2 = assign30560_e44114_d_n2;
        locals.var_t2__blk897_dn6 = assign30560_e44114_d_n6;
        locals.var_t2__blk897_dn7 = assign30560_e44114_d_n7;
        locals.var_t2__blk897_dn10 = assign30560_e44114_d_n10;
        locals.var_t2__blk897_dn11 = assign30560_e44114_d_n11;
        locals.var_t2__blk897_dn12 = assign30560_e44114_d_n12;
        locals.var_t2__blk897_dn17 = assign30560_e44114_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign30570_e44130, assign30570_e44130_d_n0, assign30570_e44130_d_n2, assign30570_e44130_d_n6, assign30570_e44130_d_n7, assign30570_e44130_d_n10, assign30570_e44130_d_n11, assign30570_e44130_d_n12, assign30570_e44130_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30570_e44126: f64 = (locals.var_t2__blk897 * locals.var_t2__blk897);
        let assign30570_e44127: f64 = (1.0 + assign30570_e44126);
        let assign30570_e44128: f64 = (assign30570_e44127).sqrt();
        (assign30570_e44128, (((locals.var_t2__blk897_dn0 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn0)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn2 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn2)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn6 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn6)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn7 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn7)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn10 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn10)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn11 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn11)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn12 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn12)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn17 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn17)) / (2.0 * assign30570_e44128)),)
    } else {
        (locals.var_t3__blk898, locals.var_t3__blk898_dn0, locals.var_t3__blk898_dn2, locals.var_t3__blk898_dn6, locals.var_t3__blk898_dn7, locals.var_t3__blk898_dn10, locals.var_t3__blk898_dn11, locals.var_t3__blk898_dn12, locals.var_t3__blk898_dn17,)
    }
};
        locals.var_t3__blk898 = assign30570_e44130;
        locals.var_t3__blk898_dn0 = assign30570_e44130_d_n0;
        locals.var_t3__blk898_dn2 = assign30570_e44130_d_n2;
        locals.var_t3__blk898_dn6 = assign30570_e44130_d_n6;
        locals.var_t3__blk898_dn7 = assign30570_e44130_d_n7;
        locals.var_t3__blk898_dn10 = assign30570_e44130_d_n10;
        locals.var_t3__blk898_dn11 = assign30570_e44130_d_n11;
        locals.var_t3__blk898_dn12 = assign30570_e44130_d_n12;
        locals.var_t3__blk898_dn17 = assign30570_e44130_d_n17;
        locals.var_t3__blk898_rv = 0.0;

        let (assign30580_e44145, assign30580_e44145_d_n0, assign30580_e44145_d_n2, assign30580_e44145_d_n6, assign30580_e44145_d_n7, assign30580_e44145_d_n10, assign30580_e44145_d_n11, assign30580_e44145_d_n12, assign30580_e44145_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30580_e44141: f64 = (locals.var_t1__blk896 / locals.var_t3__blk898);
        let assign30580_e44143: f64 = (assign30580_e44141 - locals.var_vxbgmtcl__blk921);
        (assign30580_e44143, ((((locals.var_t1__blk896_dn0 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn0)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn0), ((((locals.var_t1__blk896_dn2 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn2)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn2), ((((locals.var_t1__blk896_dn6 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn6)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn6), ((((locals.var_t1__blk896_dn7 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn7)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_t1__blk896_dn10 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn10)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn10), ((((locals.var_t1__blk896_dn11 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn11)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn11), ((((locals.var_t1__blk896_dn12 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn12)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn12), ((((locals.var_t1__blk896_dn17 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn17)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
        locals.var_ps0ld__blk945 = assign30580_e44145;
        locals.var_ps0ld__blk945_dn0 = assign30580_e44145_d_n0;
        locals.var_ps0ld__blk945_dn2 = assign30580_e44145_d_n2;
        locals.var_ps0ld__blk945_dn6 = assign30580_e44145_d_n6;
        locals.var_ps0ld__blk945_dn7 = assign30580_e44145_d_n7;
        locals.var_ps0ld__blk945_dn10 = assign30580_e44145_d_n10;
        locals.var_ps0ld__blk945_dn11 = assign30580_e44145_d_n11;
        locals.var_ps0ld__blk945_dn12 = assign30580_e44145_d_n12;
        locals.var_ps0ld__blk945_dn17 = assign30580_e44145_d_n17;
        locals.var_ps0ld__blk945_rv = 0.0;

        let (assign30590_e44158, assign30590_e44158_d_n0, assign30590_e44158_d_n2, assign30590_e44158_d_n6, assign30590_e44158_d_n7, assign30590_e44158_d_n10, assign30590_e44158_d_n11, assign30590_e44158_d_n12, assign30590_e44158_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30590_e44156: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        (assign30590_e44156, (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0), (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2), (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6), (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7), (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10), (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11), (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12), (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign30590_e44158;
        locals.var_t2__blk897_dn0 = assign30590_e44158_d_n0;
        locals.var_t2__blk897_dn2 = assign30590_e44158_d_n2;
        locals.var_t2__blk897_dn6 = assign30590_e44158_d_n6;
        locals.var_t2__blk897_dn7 = assign30590_e44158_d_n7;
        locals.var_t2__blk897_dn10 = assign30590_e44158_d_n10;
        locals.var_t2__blk897_dn11 = assign30590_e44158_d_n11;
        locals.var_t2__blk897_dn12 = assign30590_e44158_d_n12;
        locals.var_t2__blk897_dn17 = assign30590_e44158_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign30600_e44171, assign30600_e44171_d_n0, assign30600_e44171_d_n2, assign30600_e44171_d_n6, assign30600_e44171_d_n7, assign30600_e44171_d_n10, assign30600_e44171_d_n11, assign30600_e44171_d_n12, assign30600_e44171_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30600_e44169: f64 = (locals.var_cox0__blk906 * locals.var_t2__blk897);
        (assign30600_e44169, (locals.var_cox0__blk906 * locals.var_t2__blk897_dn0), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn2), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn6), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn7), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn10), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn11), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn12), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign30600_e44171;
        locals.var_qsuld_dn0 = assign30600_e44171_d_n0;
        locals.var_qsuld_dn2 = assign30600_e44171_d_n2;
        locals.var_qsuld_dn6 = assign30600_e44171_d_n6;
        locals.var_qsuld_dn7 = assign30600_e44171_d_n7;
        locals.var_qsuld_dn10 = assign30600_e44171_d_n10;
        locals.var_qsuld_dn11 = assign30600_e44171_d_n11;
        locals.var_qsuld_dn12 = assign30600_e44171_d_n12;
        locals.var_qsuld_dn17 = assign30600_e44171_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign30610_e44182, assign30610_e44182_d_n0, assign30610_e44182_d_n2, assign30610_e44182_d_n6, assign30610_e44182_d_n7, assign30610_e44182_d_n10, assign30610_e44182_d_n11, assign30610_e44182_d_n12, assign30610_e44182_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign30610_e44182;
        locals.var_qbuld_dn0 = assign30610_e44182_d_n0;
        locals.var_qbuld_dn2 = assign30610_e44182_d_n2;
        locals.var_qbuld_dn6 = assign30610_e44182_d_n6;
        locals.var_qbuld_dn7 = assign30610_e44182_d_n7;
        locals.var_qbuld_dn10 = assign30610_e44182_d_n10;
        locals.var_qbuld_dn11 = assign30610_e44182_d_n11;
        locals.var_qbuld_dn12 = assign30610_e44182_d_n12;
        locals.var_qbuld_dn17 = assign30610_e44182_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign30630_e44206, assign30630_e44206_d_n0, assign30630_e44206_d_n2, assign30630_e44206_d_n6, assign30630_e44206_d_n7, assign30630_e44206_d_n10, assign30630_e44206_d_n11, assign30630_e44206_d_n12, assign30630_e44206_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30630_e44206;
        locals.var_chi__blk943_dn0 = assign30630_e44206_d_n0;
        locals.var_chi__blk943_dn2 = assign30630_e44206_d_n2;
        locals.var_chi__blk943_dn6 = assign30630_e44206_d_n6;
        locals.var_chi__blk943_dn7 = assign30630_e44206_d_n7;
        locals.var_chi__blk943_dn10 = assign30630_e44206_d_n10;
        locals.var_chi__blk943_dn11 = assign30630_e44206_d_n11;
        locals.var_chi__blk943_dn12 = assign30630_e44206_d_n12;
        locals.var_chi__blk943_dn17 = assign30630_e44206_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign30640_e44222, assign30640_e44222_d_n0, assign30640_e44222_d_n2, assign30640_e44222_d_n6, assign30640_e44222_d_n7, assign30640_e44222_d_n10, assign30640_e44222_d_n11, assign30640_e44222_d_n12, assign30640_e44222_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30640_e44218: f64 = (locals.var_chi__blk943 / locals.var_beta);
        let assign30640_e44220: f64 = (assign30640_e44218 - locals.var_vxbgmtcl__blk921);
        (assign30640_e44220, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign30640_e44222;
        locals.var_ps0_inia__blk946_dn0 = assign30640_e44222_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign30640_e44222_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign30640_e44222_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign30640_e44222_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign30640_e44222_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign30640_e44222_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign30640_e44222_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign30640_e44222_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign30650_e44236, assign30650_e44236_d_n0, assign30650_e44236_d_n2, assign30650_e44236_d_n6, assign30650_e44236_d_n7, assign30650_e44236_d_n10, assign30650_e44236_d_n11, assign30650_e44236_d_n12, assign30650_e44236_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30650_e44233: f64 = (-locals.var_chi__blk943);
        let assign30650_e44234: f64 = (assign30650_e44233).exp();
        (assign30650_e44234, (assign30650_e44234 * (-locals.var_chi__blk943_dn0)), (assign30650_e44234 * (-locals.var_chi__blk943_dn2)), (assign30650_e44234 * (-locals.var_chi__blk943_dn6)), (assign30650_e44234 * (-locals.var_chi__blk943_dn7)), (assign30650_e44234 * (-locals.var_chi__blk943_dn10)), (assign30650_e44234 * (-locals.var_chi__blk943_dn11)), (assign30650_e44234 * (-locals.var_chi__blk943_dn12)), (assign30650_e44234 * (-locals.var_chi__blk943_dn17)),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30650_e44236;
        locals.var_ty__blk905_dn0 = assign30650_e44236_d_n0;
        locals.var_ty__blk905_dn2 = assign30650_e44236_d_n2;
        locals.var_ty__blk905_dn6 = assign30650_e44236_d_n6;
        locals.var_ty__blk905_dn7 = assign30650_e44236_d_n7;
        locals.var_ty__blk905_dn10 = assign30650_e44236_d_n10;
        locals.var_ty__blk905_dn11 = assign30650_e44236_d_n11;
        locals.var_ty__blk905_dn12 = assign30650_e44236_d_n12;
        locals.var_ty__blk905_dn17 = assign30650_e44236_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30660_e44264, assign30660_e44264_d_n0, assign30660_e44264_d_n2, assign30660_e44264_d_n6, assign30660_e44264_d_n7, assign30660_e44264_d_n10, assign30660_e44264_d_n11, assign30660_e44264_d_n12, assign30660_e44264_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30660_e44251: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30660_e44252: f64 = (locals.var_beta * assign30660_e44251);
        let assign30660_e44254: f64 = (assign30660_e44252 - 1.0);
        let assign30660_e44256: f64 = (assign30660_e44254 + locals.var_ty__blk905);
        let assign30660_e44257: f64 = (4.0 * assign30660_e44256);
        let assign30660_e44260: f64 = (locals.var_fac1p2__blk930 * locals.var_beta2);
        let assign30660_e44261: f64 = (assign30660_e44257 / assign30660_e44260);
        let assign30660_e44262: f64 = (1.0 + assign30660_e44261);
        (assign30660_e44262, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) + locals.var_ty__blk905_dn0)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn0 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) + locals.var_ty__blk905_dn2)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn2 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) + locals.var_ty__blk905_dn6)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn6 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) + locals.var_ty__blk905_dn7)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn7 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * (((locals.var_beta_dn10 * assign30660_e44251) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))) + locals.var_ty__blk905_dn10)) * assign30660_e44260) - (assign30660_e44257 * ((locals.var_fac1p2__blk930_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk930 * locals.var_beta2_dn10)))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) + locals.var_ty__blk905_dn11)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn11 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) + locals.var_ty__blk905_dn12)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn12 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) + locals.var_ty__blk905_dn17)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn17 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30660_e44264;
        locals.var_tx__blk904_dn0 = assign30660_e44264_d_n0;
        locals.var_tx__blk904_dn2 = assign30660_e44264_d_n2;
        locals.var_tx__blk904_dn6 = assign30660_e44264_d_n6;
        locals.var_tx__blk904_dn7 = assign30660_e44264_d_n7;
        locals.var_tx__blk904_dn10 = assign30660_e44264_d_n10;
        locals.var_tx__blk904_dn11 = assign30660_e44264_d_n11;
        locals.var_tx__blk904_dn12 = assign30660_e44264_d_n12;
        locals.var_tx__blk904_dn17 = assign30660_e44264_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let assign30670_e44268: f64 = (10.0 * 2.220446049250313e-16);
        let assign30670_e44269: f64 = if locals.var_tx__blk904 < assign30670_e44268 { 1.0 } else { 0.0 };
        locals.var_guard1004 = assign30670_e44269;
        locals.var_guard1004_rv = 0.0;

        let (assign30680_e44285, assign30680_e44285_d_n0, assign30680_e44285_d_n2, assign30680_e44285_d_n6, assign30680_e44285_d_n7, assign30680_e44285_d_n10, assign30680_e44285_d_n11, assign30680_e44285_d_n12, assign30680_e44285_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30680_e44283: f64 = (10.0 * 2.220446049250313e-16);
        (assign30680_e44283, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30680_e44285;
        locals.var_tx__blk904_dn0 = assign30680_e44285_d_n0;
        locals.var_tx__blk904_dn2 = assign30680_e44285_d_n2;
        locals.var_tx__blk904_dn6 = assign30680_e44285_d_n6;
        locals.var_tx__blk904_dn7 = assign30680_e44285_d_n7;
        locals.var_tx__blk904_dn10 = assign30680_e44285_d_n10;
        locals.var_tx__blk904_dn11 = assign30680_e44285_d_n11;
        locals.var_tx__blk904_dn12 = assign30680_e44285_d_n12;
        locals.var_tx__blk904_dn17 = assign30680_e44285_d_n17;
        locals.var_tx__blk904_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30690_e44308, assign30690_e44308_d_n0, assign30690_e44308_d_n2, assign30690_e44308_d_n6, assign30690_e44308_d_n7, assign30690_e44308_d_n10, assign30690_e44308_d_n11, assign30690_e44308_d_n12, assign30690_e44308_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30690_e44298: f64 = (locals.var_fac1p2__blk930 * locals.var_beta);
        let assign30690_e44300: f64 = (assign30690_e44298 / 2.0);
        let assign30690_e44303: f64 = (locals.var_tx__blk904).sqrt();
        let assign30690_e44304: f64 = (1.0 - assign30690_e44303);
        let assign30690_e44305: f64 = (assign30690_e44300 * assign30690_e44304);
        let assign30690_e44306: f64 = (locals.var_vgpld__blk931 + assign30690_e44305);
        (assign30690_e44306, (locals.var_vgpld__blk931_dn0 + ((((locals.var_fac1p2__blk930_dn0 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn0 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn2 + ((((locals.var_fac1p2__blk930_dn2 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn2 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn6 + ((((locals.var_fac1p2__blk930_dn6 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn6 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn7 + ((((locals.var_fac1p2__blk930_dn7 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn7 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn10 + (((((locals.var_fac1p2__blk930_dn10 * locals.var_beta) + (locals.var_fac1p2__blk930 * locals.var_beta_dn10)) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn10 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn11 + ((((locals.var_fac1p2__blk930_dn11 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn11 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn12 + ((((locals.var_fac1p2__blk930_dn12 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn12 / (2.0 * assign30690_e44303)))))), (locals.var_vgpld__blk931_dn17 + ((((locals.var_fac1p2__blk930_dn17 * locals.var_beta) / 2.0) * assign30690_e44304) + (assign30690_e44300 * (-(locals.var_tx__blk904_dn17 / (2.0 * assign30690_e44303)))))),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign30690_e44308;
        locals.var_ps0_inia__blk946_dn0 = assign30690_e44308_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign30690_e44308_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign30690_e44308_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign30690_e44308_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign30690_e44308_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign30690_e44308_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign30690_e44308_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign30690_e44308_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign30700_e44324, assign30700_e44324_d_n0, assign30700_e44324_d_n2, assign30700_e44324_d_n6, assign30700_e44324_d_n7, assign30700_e44324_d_n10, assign30700_e44324_d_n11, assign30700_e44324_d_n12, assign30700_e44324_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30700_e44321: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign30700_e44322: f64 = (locals.var_beta * assign30700_e44321);
        (assign30700_e44322, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30700_e44321) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30700_e44324;
        locals.var_chi__blk943_dn0 = assign30700_e44324_d_n0;
        locals.var_chi__blk943_dn2 = assign30700_e44324_d_n2;
        locals.var_chi__blk943_dn6 = assign30700_e44324_d_n6;
        locals.var_chi__blk943_dn7 = assign30700_e44324_d_n7;
        locals.var_chi__blk943_dn10 = assign30700_e44324_d_n10;
        locals.var_chi__blk943_dn11 = assign30700_e44324_d_n11;
        locals.var_chi__blk943_dn12 = assign30700_e44324_d_n12;
        locals.var_chi__blk943_dn17 = assign30700_e44324_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign30710_e44338, assign30710_e44338_d_n0, assign30710_e44338_d_n2, assign30710_e44338_d_n6, assign30710_e44338_d_n7, assign30710_e44338_d_n10, assign30710_e44338_d_n11, assign30710_e44338_d_n12, assign30710_e44338_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30710_e44335: f64 = (-locals.var_chi__blk943);
        let assign30710_e44336: f64 = (assign30710_e44335).exp();
        (assign30710_e44336, (assign30710_e44336 * (-locals.var_chi__blk943_dn0)), (assign30710_e44336 * (-locals.var_chi__blk943_dn2)), (assign30710_e44336 * (-locals.var_chi__blk943_dn6)), (assign30710_e44336 * (-locals.var_chi__blk943_dn7)), (assign30710_e44336 * (-locals.var_chi__blk943_dn10)), (assign30710_e44336 * (-locals.var_chi__blk943_dn11)), (assign30710_e44336 * (-locals.var_chi__blk943_dn12)), (assign30710_e44336 * (-locals.var_chi__blk943_dn17)),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30710_e44338;
        locals.var_ty__blk905_dn0 = assign30710_e44338_d_n0;
        locals.var_ty__blk905_dn2 = assign30710_e44338_d_n2;
        locals.var_ty__blk905_dn6 = assign30710_e44338_d_n6;
        locals.var_ty__blk905_dn7 = assign30710_e44338_d_n7;
        locals.var_ty__blk905_dn10 = assign30710_e44338_d_n10;
        locals.var_ty__blk905_dn11 = assign30710_e44338_d_n11;
        locals.var_ty__blk905_dn12 = assign30710_e44338_d_n12;
        locals.var_ty__blk905_dn17 = assign30710_e44338_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30720_e44366, assign30720_e44366_d_n0, assign30720_e44366_d_n2, assign30720_e44366_d_n6, assign30720_e44366_d_n7, assign30720_e44366_d_n10, assign30720_e44366_d_n11, assign30720_e44366_d_n12, assign30720_e44366_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30720_e44353: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30720_e44354: f64 = (locals.var_beta * assign30720_e44353);
        let assign30720_e44356: f64 = (assign30720_e44354 - 1.0);
        let assign30720_e44358: f64 = (assign30720_e44356 + locals.var_ty__blk905);
        let assign30720_e44359: f64 = (4.0 * assign30720_e44358);
        let assign30720_e44362: f64 = (locals.var_fac1p2__blk930 * locals.var_beta2);
        let assign30720_e44363: f64 = (assign30720_e44359 / assign30720_e44362);
        let assign30720_e44364: f64 = (1.0 + assign30720_e44363);
        (assign30720_e44364, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) + locals.var_ty__blk905_dn0)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn0 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) + locals.var_ty__blk905_dn2)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn2 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) + locals.var_ty__blk905_dn6)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn6 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) + locals.var_ty__blk905_dn7)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn7 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * (((locals.var_beta_dn10 * assign30720_e44353) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))) + locals.var_ty__blk905_dn10)) * assign30720_e44362) - (assign30720_e44359 * ((locals.var_fac1p2__blk930_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk930 * locals.var_beta2_dn10)))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) + locals.var_ty__blk905_dn11)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn11 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) + locals.var_ty__blk905_dn12)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn12 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) + locals.var_ty__blk905_dn17)) * assign30720_e44362) - (assign30720_e44359 * (locals.var_fac1p2__blk930_dn17 * locals.var_beta2))) / (assign30720_e44362 * assign30720_e44362)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30720_e44366;
        locals.var_tx__blk904_dn0 = assign30720_e44366_d_n0;
        locals.var_tx__blk904_dn2 = assign30720_e44366_d_n2;
        locals.var_tx__blk904_dn6 = assign30720_e44366_d_n6;
        locals.var_tx__blk904_dn7 = assign30720_e44366_d_n7;
        locals.var_tx__blk904_dn10 = assign30720_e44366_d_n10;
        locals.var_tx__blk904_dn11 = assign30720_e44366_d_n11;
        locals.var_tx__blk904_dn12 = assign30720_e44366_d_n12;
        locals.var_tx__blk904_dn17 = assign30720_e44366_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let assign30730_e44370: f64 = (10.0 * 2.220446049250313e-16);
        let assign30730_e44371: f64 = if locals.var_tx__blk904 < assign30730_e44370 { 1.0 } else { 0.0 };
        locals.var_guard1005 = assign30730_e44371;
        locals.var_guard1005_rv = 0.0;

        let (assign30740_e44387, assign30740_e44387_d_n0, assign30740_e44387_d_n2, assign30740_e44387_d_n6, assign30740_e44387_d_n7, assign30740_e44387_d_n10, assign30740_e44387_d_n11, assign30740_e44387_d_n12, assign30740_e44387_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30740_e44385: f64 = (10.0 * 2.220446049250313e-16);
        (assign30740_e44385, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30740_e44387;
        locals.var_tx__blk904_dn0 = assign30740_e44387_d_n0;
        locals.var_tx__blk904_dn2 = assign30740_e44387_d_n2;
        locals.var_tx__blk904_dn6 = assign30740_e44387_d_n6;
        locals.var_tx__blk904_dn7 = assign30740_e44387_d_n7;
        locals.var_tx__blk904_dn10 = assign30740_e44387_d_n10;
        locals.var_tx__blk904_dn11 = assign30740_e44387_d_n11;
        locals.var_tx__blk904_dn12 = assign30740_e44387_d_n12;
        locals.var_tx__blk904_dn17 = assign30740_e44387_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign30750_e44410, assign30750_e44410_d_n0, assign30750_e44410_d_n2, assign30750_e44410_d_n6, assign30750_e44410_d_n7, assign30750_e44410_d_n10, assign30750_e44410_d_n11, assign30750_e44410_d_n12, assign30750_e44410_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30750_e44400: f64 = (locals.var_fac1p2__blk930 * locals.var_beta);
        let assign30750_e44402: f64 = (assign30750_e44400 / 2.0);
        let assign30750_e44405: f64 = (locals.var_tx__blk904).sqrt();
        let assign30750_e44406: f64 = (1.0 - assign30750_e44405);
        let assign30750_e44407: f64 = (assign30750_e44402 * assign30750_e44406);
        let assign30750_e44408: f64 = (locals.var_vgpld__blk931 + assign30750_e44407);
        (assign30750_e44408, (locals.var_vgpld__blk931_dn0 + ((((locals.var_fac1p2__blk930_dn0 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn0 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn2 + ((((locals.var_fac1p2__blk930_dn2 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn2 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn6 + ((((locals.var_fac1p2__blk930_dn6 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn6 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn7 + ((((locals.var_fac1p2__blk930_dn7 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn7 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn10 + (((((locals.var_fac1p2__blk930_dn10 * locals.var_beta) + (locals.var_fac1p2__blk930 * locals.var_beta_dn10)) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn10 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn11 + ((((locals.var_fac1p2__blk930_dn11 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn11 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn12 + ((((locals.var_fac1p2__blk930_dn12 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn12 / (2.0 * assign30750_e44405)))))), (locals.var_vgpld__blk931_dn17 + ((((locals.var_fac1p2__blk930_dn17 * locals.var_beta) / 2.0) * assign30750_e44406) + (assign30750_e44402 * (-(locals.var_tx__blk904_dn17 / (2.0 * assign30750_e44405)))))),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign30750_e44410;
        locals.var_ps0_inia__blk946_dn0 = assign30750_e44410_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign30750_e44410_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign30750_e44410_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign30750_e44410_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign30750_e44410_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign30750_e44410_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign30750_e44410_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign30750_e44410_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign30760_e44426, assign30760_e44426_d_n0, assign30760_e44426_d_n2, assign30760_e44426_d_n6, assign30760_e44426_d_n7, assign30760_e44426_d_n10, assign30760_e44426_d_n11, assign30760_e44426_d_n12, assign30760_e44426_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30760_e44423: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign30760_e44424: f64 = (locals.var_beta * assign30760_e44423);
        (assign30760_e44424, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30760_e44423) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30760_e44426;
        locals.var_chi__blk943_dn0 = assign30760_e44426_d_n0;
        locals.var_chi__blk943_dn2 = assign30760_e44426_d_n2;
        locals.var_chi__blk943_dn6 = assign30760_e44426_d_n6;
        locals.var_chi__blk943_dn7 = assign30760_e44426_d_n7;
        locals.var_chi__blk943_dn10 = assign30760_e44426_d_n10;
        locals.var_chi__blk943_dn11 = assign30760_e44426_d_n11;
        locals.var_chi__blk943_dn12 = assign30760_e44426_d_n12;
        locals.var_chi__blk943_dn17 = assign30760_e44426_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let assign30770_e44429: f64 = if locals.var_chi__blk943 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1006 = assign30770_e44429;
        locals.var_guard1006_rv = 0.0;

        let (assign30790_e44474,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30790_e44458: f64 = (9.0 * 1.414213562373095);
        let assign30790_e44459: f64 = (1.0 / assign30790_e44458);
        let assign30790_e44463: f64 = (7.0 * 0.049787068367863944);
        let assign30790_e44464: f64 = (5.0 + assign30790_e44463);
        let assign30790_e44468: f64 = (2.0 + 0.049787068367863944);
        let assign30790_e44469: f64 = (assign30790_e44468).sqrt();
        let assign30790_e44470: f64 = (54.0 * assign30790_e44469);
        let assign30790_e44471: f64 = (assign30790_e44464 / assign30790_e44470);
        let assign30790_e44472: f64 = (assign30790_e44459 - assign30790_e44471);
        (assign30790_e44472,)
    } else {
        (locals.var_ta__blk947,)
    }
};
        locals.var_ta__blk947 = assign30790_e44474;
        locals.var_ta__blk947_rv = 0.0;

        let (assign30800_e44501,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30800_e44488: f64 = (1.0 + 0.049787068367863944);
        let assign30800_e44492: f64 = (2.0 + 0.049787068367863944);
        let assign30800_e44493: f64 = (assign30800_e44492).sqrt();
        let assign30800_e44494: f64 = (2.0 * assign30800_e44493);
        let assign30800_e44495: f64 = (assign30800_e44488 / assign30800_e44494);
        let assign30800_e44498: f64 = (1.414213562373095 / 3.0);
        let assign30800_e44499: f64 = (assign30800_e44495 - assign30800_e44498);
        (assign30800_e44499,)
    } else {
        (locals.var_tb__blk948,)
    }
};
        locals.var_tb__blk948 = assign30800_e44501;
        locals.var_tb__blk948_rv = 0.0;

        let (assign30810_e44523, assign30810_e44523_d_n0, assign30810_e44523_d_n2, assign30810_e44523_d_n6, assign30810_e44523_d_n7, assign30810_e44523_d_n10, assign30810_e44523_d_n11, assign30810_e44523_d_n12, assign30810_e44523_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30810_e44515: f64 = (1.0 / 1.414213562373095);
        let assign30810_e44519: f64 = (locals.var_beta * locals.var_fac1__blk929);
        let assign30810_e44520: f64 = (1.0 / assign30810_e44519);
        let assign30810_e44521: f64 = (assign30810_e44515 + assign30810_e44520);
        (assign30810_e44521, (-((locals.var_beta * locals.var_fac1__blk929_dn0) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn2) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn6) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn7) / (assign30810_e44519 * assign30810_e44519))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk929) + (locals.var_beta * locals.var_fac1__blk929_dn10)) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn11) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn12) / (assign30810_e44519 * assign30810_e44519))), (-((locals.var_beta * locals.var_fac1__blk929_dn17) / (assign30810_e44519 * assign30810_e44519))),)
    } else {
        (locals.var_tc__blk949, locals.var_tc__blk949_dn0, locals.var_tc__blk949_dn2, locals.var_tc__blk949_dn6, locals.var_tc__blk949_dn7, locals.var_tc__blk949_dn10, locals.var_tc__blk949_dn11, locals.var_tc__blk949_dn12, locals.var_tc__blk949_dn17,)
    }
};
        locals.var_tc__blk949 = assign30810_e44523;
        locals.var_tc__blk949_dn0 = assign30810_e44523_d_n0;
        locals.var_tc__blk949_dn2 = assign30810_e44523_d_n2;
        locals.var_tc__blk949_dn6 = assign30810_e44523_d_n6;
        locals.var_tc__blk949_dn7 = assign30810_e44523_d_n7;
        locals.var_tc__blk949_dn10 = assign30810_e44523_d_n10;
        locals.var_tc__blk949_dn11 = assign30810_e44523_d_n11;
        locals.var_tc__blk949_dn12 = assign30810_e44523_d_n12;
        locals.var_tc__blk949_dn17 = assign30810_e44523_d_n17;
        locals.var_tc__blk949_rv = 0.0;

        let (assign30820_e44542, assign30820_e44542_d_n0, assign30820_e44542_d_n2, assign30820_e44542_d_n6, assign30820_e44542_d_n7, assign30820_e44542_d_n10, assign30820_e44542_d_n11, assign30820_e44542_d_n12, assign30820_e44542_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30820_e44537: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30820_e44538: f64 = (-assign30820_e44537);
        let assign30820_e44540: f64 = (assign30820_e44538 / locals.var_fac1__blk929);
        (assign30820_e44540, ((((-(locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn0)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn2)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn6)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn7)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn10)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn11)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn12)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) * locals.var_fac1__blk929) - (assign30820_e44538 * locals.var_fac1__blk929_dn17)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)),)
    } else {
        (locals.var_td__blk950, locals.var_td__blk950_dn0, locals.var_td__blk950_dn2, locals.var_td__blk950_dn6, locals.var_td__blk950_dn7, locals.var_td__blk950_dn10, locals.var_td__blk950_dn11, locals.var_td__blk950_dn12, locals.var_td__blk950_dn17,)
    }
};
        locals.var_td__blk950 = assign30820_e44542;
        locals.var_td__blk950_dn0 = assign30820_e44542_d_n0;
        locals.var_td__blk950_dn2 = assign30820_e44542_d_n2;
        locals.var_td__blk950_dn6 = assign30820_e44542_d_n6;
        locals.var_td__blk950_dn7 = assign30820_e44542_d_n7;
        locals.var_td__blk950_dn10 = assign30820_e44542_d_n10;
        locals.var_td__blk950_dn11 = assign30820_e44542_d_n11;
        locals.var_td__blk950_dn12 = assign30820_e44542_d_n12;
        locals.var_td__blk950_dn17 = assign30820_e44542_d_n17;
        locals.var_td__blk950_rv = 0.0;

        let (assign30830_e44584, assign30830_e44584_d_n0, assign30830_e44584_d_n2, assign30830_e44584_d_n6, assign30830_e44584_d_n7, assign30830_e44584_d_n10, assign30830_e44584_d_n11, assign30830_e44584_d_n12, assign30830_e44584_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30830_e44556: f64 = (locals.var_tb__blk948 * locals.var_tb__blk948);
        let assign30830_e44558: f64 = (assign30830_e44556 * locals.var_tb__blk948);
        let assign30830_e44561: f64 = (27.0 * locals.var_ta__blk947);
        let assign30830_e44563: f64 = (assign30830_e44561 * locals.var_ta__blk947);
        let assign30830_e44565: f64 = (assign30830_e44563 * locals.var_ta__blk947);
        let assign30830_e44566: f64 = (assign30830_e44558 / assign30830_e44565);
        let assign30830_e44569: f64 = (locals.var_tb__blk948 * locals.var_tc__blk949);
        let assign30830_e44572: f64 = (6.0 * locals.var_ta__blk947);
        let assign30830_e44574: f64 = (assign30830_e44572 * locals.var_ta__blk947);
        let assign30830_e44575: f64 = (assign30830_e44569 / assign30830_e44574);
        let assign30830_e44576: f64 = (assign30830_e44566 - assign30830_e44575);
        let assign30830_e44580: f64 = (2.0 * locals.var_ta__blk947);
        let assign30830_e44581: f64 = (locals.var_td__blk950 / assign30830_e44580);
        let assign30830_e44582: f64 = (assign30830_e44576 + assign30830_e44581);
        (assign30830_e44582, ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn0) / assign30830_e44574)) + (locals.var_td__blk950_dn0 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn2) / assign30830_e44574)) + (locals.var_td__blk950_dn2 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn6) / assign30830_e44574)) + (locals.var_td__blk950_dn6 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn7) / assign30830_e44574)) + (locals.var_td__blk950_dn7 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn10) / assign30830_e44574)) + (locals.var_td__blk950_dn10 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn11) / assign30830_e44574)) + (locals.var_td__blk950_dn11 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn12) / assign30830_e44574)) + (locals.var_td__blk950_dn12 / assign30830_e44580)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn17) / assign30830_e44574)) + (locals.var_td__blk950_dn17 / assign30830_e44580)),)
    } else {
        (locals.var_tq__blk951, locals.var_tq__blk951_dn0, locals.var_tq__blk951_dn2, locals.var_tq__blk951_dn6, locals.var_tq__blk951_dn7, locals.var_tq__blk951_dn10, locals.var_tq__blk951_dn11, locals.var_tq__blk951_dn12, locals.var_tq__blk951_dn17,)
    }
};
        locals.var_tq__blk951 = assign30830_e44584;
        locals.var_tq__blk951_dn0 = assign30830_e44584_d_n0;
        locals.var_tq__blk951_dn2 = assign30830_e44584_d_n2;
        locals.var_tq__blk951_dn6 = assign30830_e44584_d_n6;
        locals.var_tq__blk951_dn7 = assign30830_e44584_d_n7;
        locals.var_tq__blk951_dn10 = assign30830_e44584_d_n10;
        locals.var_tq__blk951_dn11 = assign30830_e44584_d_n11;
        locals.var_tq__blk951_dn12 = assign30830_e44584_d_n12;
        locals.var_tq__blk951_dn17 = assign30830_e44584_d_n17;
        locals.var_tq__blk951_rv = 0.0;

        let (assign30840_e44612, assign30840_e44612_d_n0, assign30840_e44612_d_n2, assign30840_e44612_d_n6, assign30840_e44612_d_n7, assign30840_e44612_d_n10, assign30840_e44612_d_n11, assign30840_e44612_d_n12, assign30840_e44612_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30840_e44598: f64 = (3.0 * locals.var_ta__blk947);
        let assign30840_e44600: f64 = (assign30840_e44598 * locals.var_tc__blk949);
        let assign30840_e44603: f64 = (locals.var_tb__blk948 * locals.var_tb__blk948);
        let assign30840_e44604: f64 = (assign30840_e44600 - assign30840_e44603);
        let assign30840_e44607: f64 = (9.0 * locals.var_ta__blk947);
        let assign30840_e44609: f64 = (assign30840_e44607 * locals.var_ta__blk947);
        let assign30840_e44610: f64 = (assign30840_e44604 / assign30840_e44609);
        (assign30840_e44610, ((assign30840_e44598 * locals.var_tc__blk949_dn0) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn2) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn6) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn7) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn10) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn11) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn12) / assign30840_e44609), ((assign30840_e44598 * locals.var_tc__blk949_dn17) / assign30840_e44609),)
    } else {
        (locals.var_tp__blk952, locals.var_tp__blk952_dn0, locals.var_tp__blk952_dn2, locals.var_tp__blk952_dn6, locals.var_tp__blk952_dn7, locals.var_tp__blk952_dn10, locals.var_tp__blk952_dn11, locals.var_tp__blk952_dn12, locals.var_tp__blk952_dn17,)
    }
};
        locals.var_tp__blk952 = assign30840_e44612;
        locals.var_tp__blk952_dn0 = assign30840_e44612_d_n0;
        locals.var_tp__blk952_dn2 = assign30840_e44612_d_n2;
        locals.var_tp__blk952_dn6 = assign30840_e44612_d_n6;
        locals.var_tp__blk952_dn7 = assign30840_e44612_d_n7;
        locals.var_tp__blk952_dn10 = assign30840_e44612_d_n10;
        locals.var_tp__blk952_dn11 = assign30840_e44612_d_n11;
        locals.var_tp__blk952_dn12 = assign30840_e44612_d_n12;
        locals.var_tp__blk952_dn17 = assign30840_e44612_d_n17;
        locals.var_tp__blk952_rv = 0.0;

        let (assign30850_e44635, assign30850_e44635_d_n0, assign30850_e44635_d_n2, assign30850_e44635_d_n6, assign30850_e44635_d_n7, assign30850_e44635_d_n10, assign30850_e44635_d_n11, assign30850_e44635_d_n12, assign30850_e44635_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30850_e44626: f64 = (locals.var_tq__blk951 * locals.var_tq__blk951);
        let assign30850_e44629: f64 = (locals.var_tp__blk952 * locals.var_tp__blk952);
        let assign30850_e44631: f64 = (assign30850_e44629 * locals.var_tp__blk952);
        let assign30850_e44632: f64 = (assign30850_e44626 + assign30850_e44631);
        let assign30850_e44633: f64 = (assign30850_e44632).sqrt();
        (assign30850_e44633, ((((locals.var_tq__blk951_dn0 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn0)) + ((((locals.var_tp__blk952_dn0 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn0)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn0))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn2 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn2)) + ((((locals.var_tp__blk952_dn2 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn2)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn2))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn6 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn6)) + ((((locals.var_tp__blk952_dn6 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn6)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn6))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn7 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn7)) + ((((locals.var_tp__blk952_dn7 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn7)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn7))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn10 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn10)) + ((((locals.var_tp__blk952_dn10 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn10)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn10))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn11 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn11)) + ((((locals.var_tp__blk952_dn11 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn11)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn11))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn12 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn12)) + ((((locals.var_tp__blk952_dn12 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn12)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn12))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn17 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn17)) + ((((locals.var_tp__blk952_dn17 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn17)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn17))) / (2.0 * assign30850_e44633)),)
    } else {
        (locals.var_t5__blk900, locals.var_t5__blk900_dn0, locals.var_t5__blk900_dn2, locals.var_t5__blk900_dn6, locals.var_t5__blk900_dn7, locals.var_t5__blk900_dn10, locals.var_t5__blk900_dn11, locals.var_t5__blk900_dn12, locals.var_t5__blk900_dn17,)
    }
};
        locals.var_t5__blk900 = assign30850_e44635;
        locals.var_t5__blk900_dn0 = assign30850_e44635_d_n0;
        locals.var_t5__blk900_dn2 = assign30850_e44635_d_n2;
        locals.var_t5__blk900_dn6 = assign30850_e44635_d_n6;
        locals.var_t5__blk900_dn7 = assign30850_e44635_d_n7;
        locals.var_t5__blk900_dn10 = assign30850_e44635_d_n10;
        locals.var_t5__blk900_dn11 = assign30850_e44635_d_n11;
        locals.var_t5__blk900_dn12 = assign30850_e44635_d_n12;
        locals.var_t5__blk900_dn17 = assign30850_e44635_d_n17;
        locals.var_t5__blk900_rv = 0.0;

        let (assign30860_e44654, assign30860_e44654_d_n0, assign30860_e44654_d_n2, assign30860_e44654_d_n6, assign30860_e44654_d_n7, assign30860_e44654_d_n10, assign30860_e44654_d_n11, assign30860_e44654_d_n12, assign30860_e44654_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30860_e44648: f64 = (-locals.var_tq__blk951);
        let assign30860_e44650: f64 = (assign30860_e44648 + locals.var_t5__blk900);
        let assign30860_e44652: f64 = (assign30860_e44650).powf(0.3333333333333333);
        (assign30860_e44652, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17) / assign30860_e44650))) },)
    } else {
        (locals.var_tu__blk953, locals.var_tu__blk953_dn0, locals.var_tu__blk953_dn2, locals.var_tu__blk953_dn6, locals.var_tu__blk953_dn7, locals.var_tu__blk953_dn10, locals.var_tu__blk953_dn11, locals.var_tu__blk953_dn12, locals.var_tu__blk953_dn17,)
    }
};
        locals.var_tu__blk953 = assign30860_e44654;
        locals.var_tu__blk953_dn0 = assign30860_e44654_d_n0;
        locals.var_tu__blk953_dn2 = assign30860_e44654_d_n2;
        locals.var_tu__blk953_dn6 = assign30860_e44654_d_n6;
        locals.var_tu__blk953_dn7 = assign30860_e44654_d_n7;
        locals.var_tu__blk953_dn10 = assign30860_e44654_d_n10;
        locals.var_tu__blk953_dn11 = assign30860_e44654_d_n11;
        locals.var_tu__blk953_dn12 = assign30860_e44654_d_n12;
        locals.var_tu__blk953_dn17 = assign30860_e44654_d_n17;
        locals.var_tu__blk953_rv = 0.0;

        let (assign30870_e44673, assign30870_e44673_d_n0, assign30870_e44673_d_n2, assign30870_e44673_d_n6, assign30870_e44673_d_n7, assign30870_e44673_d_n10, assign30870_e44673_d_n11, assign30870_e44673_d_n12, assign30870_e44673_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30870_e44668: f64 = (locals.var_tq__blk951 + locals.var_t5__blk900);
        let assign30870_e44670: f64 = (assign30870_e44668).powf(0.3333333333333333);
        let assign30870_e44671: f64 = (-assign30870_e44670);
        (assign30870_e44671, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17) / assign30870_e44668))) }),)
    } else {
        (locals.var_tv__blk954, locals.var_tv__blk954_dn0, locals.var_tv__blk954_dn2, locals.var_tv__blk954_dn6, locals.var_tv__blk954_dn7, locals.var_tv__blk954_dn10, locals.var_tv__blk954_dn11, locals.var_tv__blk954_dn12, locals.var_tv__blk954_dn17,)
    }
};
        locals.var_tv__blk954 = assign30870_e44673;
        locals.var_tv__blk954_dn0 = assign30870_e44673_d_n0;
        locals.var_tv__blk954_dn2 = assign30870_e44673_d_n2;
        locals.var_tv__blk954_dn6 = assign30870_e44673_d_n6;
        locals.var_tv__blk954_dn7 = assign30870_e44673_d_n7;
        locals.var_tv__blk954_dn10 = assign30870_e44673_d_n10;
        locals.var_tv__blk954_dn11 = assign30870_e44673_d_n11;
        locals.var_tv__blk954_dn12 = assign30870_e44673_d_n12;
        locals.var_tv__blk954_dn17 = assign30870_e44673_d_n17;
        locals.var_tv__blk954_rv = 0.0;

        let (assign30880_e44695, assign30880_e44695_d_n0, assign30880_e44695_d_n2, assign30880_e44695_d_n6, assign30880_e44695_d_n7, assign30880_e44695_d_n10, assign30880_e44695_d_n11, assign30880_e44695_d_n12, assign30880_e44695_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30880_e44687: f64 = (locals.var_tu__blk953 + locals.var_tv__blk954);
        let assign30880_e44691: f64 = (3.0 * locals.var_ta__blk947);
        let assign30880_e44692: f64 = (locals.var_tb__blk948 / assign30880_e44691);
        let assign30880_e44693: f64 = (assign30880_e44687 - assign30880_e44692);
        (assign30880_e44693, (locals.var_tu__blk953_dn0 + locals.var_tv__blk954_dn0), (locals.var_tu__blk953_dn2 + locals.var_tv__blk954_dn2), (locals.var_tu__blk953_dn6 + locals.var_tv__blk954_dn6), (locals.var_tu__blk953_dn7 + locals.var_tv__blk954_dn7), (locals.var_tu__blk953_dn10 + locals.var_tv__blk954_dn10), (locals.var_tu__blk953_dn11 + locals.var_tv__blk954_dn11), (locals.var_tu__blk953_dn12 + locals.var_tv__blk954_dn12), (locals.var_tu__blk953_dn17 + locals.var_tv__blk954_dn17),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30880_e44695;
        locals.var_tx__blk904_dn0 = assign30880_e44695_d_n0;
        locals.var_tx__blk904_dn2 = assign30880_e44695_d_n2;
        locals.var_tx__blk904_dn6 = assign30880_e44695_d_n6;
        locals.var_tx__blk904_dn7 = assign30880_e44695_d_n7;
        locals.var_tx__blk904_dn10 = assign30880_e44695_d_n10;
        locals.var_tx__blk904_dn11 = assign30880_e44695_d_n11;
        locals.var_tx__blk904_dn12 = assign30880_e44695_d_n12;
        locals.var_tx__blk904_dn17 = assign30880_e44695_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign30890_e44713, assign30890_e44713_d_n0, assign30890_e44713_d_n2, assign30890_e44713_d_n6, assign30890_e44713_d_n7, assign30890_e44713_d_n10, assign30890_e44713_d_n11, assign30890_e44713_d_n12, assign30890_e44713_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30890_e44709: f64 = (locals.var_tx__blk904 * locals.var_beta_inv);
        let assign30890_e44711: f64 = (assign30890_e44709 - locals.var_vxbgmtcl__blk921);
        (assign30890_e44711, ((locals.var_tx__blk904_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_tx__blk904_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_tx__blk904_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_tx__blk904_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_tx__blk904_dn10 * locals.var_beta_inv) + (locals.var_tx__blk904 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_tx__blk904_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_tx__blk904_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_tx__blk904_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign30890_e44713;
        locals.var_ps0_inia__blk946_dn0 = assign30890_e44713_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign30890_e44713_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign30890_e44713_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign30890_e44713_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign30890_e44713_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign30890_e44713_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign30890_e44713_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign30890_e44713_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign30900_e44731, assign30900_e44731_d_n0, assign30900_e44731_d_n2, assign30900_e44731_d_n6, assign30900_e44731_d_n7, assign30900_e44731_d_n10, assign30900_e44731_d_n11, assign30900_e44731_d_n12, assign30900_e44731_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30900_e44728: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign30900_e44729: f64 = (locals.var_beta * assign30900_e44728);
        (assign30900_e44729, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30900_e44728) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30900_e44731;
        locals.var_chi__blk943_dn0 = assign30900_e44731_d_n0;
        locals.var_chi__blk943_dn2 = assign30900_e44731_d_n2;
        locals.var_chi__blk943_dn6 = assign30900_e44731_d_n6;
        locals.var_chi__blk943_dn7 = assign30900_e44731_d_n7;
        locals.var_chi__blk943_dn10 = assign30900_e44731_d_n10;
        locals.var_chi__blk943_dn11 = assign30900_e44731_d_n11;
        locals.var_chi__blk943_dn12 = assign30900_e44731_d_n12;
        locals.var_chi__blk943_dn17 = assign30900_e44731_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let assign30910_e44734: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1007 = assign30910_e44734;
        locals.var_guard1007_rv = 0.0;

        let (assign30930_e44768, assign30930_e44768_d_n0, assign30930_e44768_d_n2, assign30930_e44768_d_n6, assign30930_e44768_d_n7, assign30930_e44768_d_n10, assign30930_e44768_d_n11, assign30930_e44768_d_n12, assign30930_e44768_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30930_e44764: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30930_e44766: f64 = (assign30930_e44764 + 0.1);
        (assign30930_e44766, (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_vgpld_shift__blk955, locals.var_vgpld_shift__blk955_dn0, locals.var_vgpld_shift__blk955_dn2, locals.var_vgpld_shift__blk955_dn6, locals.var_vgpld_shift__blk955_dn7, locals.var_vgpld_shift__blk955_dn10, locals.var_vgpld_shift__blk955_dn11, locals.var_vgpld_shift__blk955_dn12, locals.var_vgpld_shift__blk955_dn17,)
    }
};
        locals.var_vgpld_shift__blk955 = assign30930_e44768;
        locals.var_vgpld_shift__blk955_dn0 = assign30930_e44768_d_n0;
        locals.var_vgpld_shift__blk955_dn2 = assign30930_e44768_d_n2;
        locals.var_vgpld_shift__blk955_dn6 = assign30930_e44768_d_n6;
        locals.var_vgpld_shift__blk955_dn7 = assign30930_e44768_d_n7;
        locals.var_vgpld_shift__blk955_dn10 = assign30930_e44768_d_n10;
        locals.var_vgpld_shift__blk955_dn11 = assign30930_e44768_d_n11;
        locals.var_vgpld_shift__blk955_dn12 = assign30930_e44768_d_n12;
        locals.var_vgpld_shift__blk955_dn17 = assign30930_e44768_d_n17;
        locals.var_vgpld_shift__blk955_rv = 0.0;

        let (assign30940_e44788, assign30940_e44788_d_n0, assign30940_e44788_d_n2, assign30940_e44788_d_n6, assign30940_e44788_d_n7, assign30940_e44788_d_n10, assign30940_e44788_d_n11, assign30940_e44788_d_n12, assign30940_e44788_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30940_e44782: f64 = (-locals.var_vxbgmtcl__blk921);
        let assign30940_e44783: f64 = (locals.var_beta * assign30940_e44782);
        let assign30940_e44784: f64 = (assign30940_e44783).exp();
        let assign30940_e44786: f64 = (assign30940_e44784 + 1e-50);
        (assign30940_e44786, (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign30940_e44784 * ((locals.var_beta_dn10 * assign30940_e44782) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17,)
    }
};
        locals.var_exp_bvbs__blk962 = assign30940_e44788;
        locals.var_exp_bvbs__blk962_dn0 = assign30940_e44788_d_n0;
        locals.var_exp_bvbs__blk962_dn2 = assign30940_e44788_d_n2;
        locals.var_exp_bvbs__blk962_dn6 = assign30940_e44788_d_n6;
        locals.var_exp_bvbs__blk962_dn7 = assign30940_e44788_d_n7;
        locals.var_exp_bvbs__blk962_dn10 = assign30940_e44788_d_n10;
        locals.var_exp_bvbs__blk962_dn11 = assign30940_e44788_d_n11;
        locals.var_exp_bvbs__blk962_dn12 = assign30940_e44788_d_n12;
        locals.var_exp_bvbs__blk962_dn17 = assign30940_e44788_d_n17;
        locals.var_exp_bvbs__blk962_rv = 0.0;

        let (assign30950_e44804, assign30950_e44804_d_n0, assign30950_e44804_d_n2, assign30950_e44804_d_n6, assign30950_e44804_d_n7, assign30950_e44804_d_n10, assign30950_e44804_d_n11, assign30950_e44804_d_n12, assign30950_e44804_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30950_e44802: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign30950_e44802, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign30950_e44804;
        locals.var_t0__blk895_dn0 = assign30950_e44804_d_n0;
        locals.var_t0__blk895_dn2 = assign30950_e44804_d_n2;
        locals.var_t0__blk895_dn6 = assign30950_e44804_d_n6;
        locals.var_t0__blk895_dn7 = assign30950_e44804_d_n7;
        locals.var_t0__blk895_dn10 = assign30950_e44804_d_n10;
        locals.var_t0__blk895_dn11 = assign30950_e44804_d_n11;
        locals.var_t0__blk895_dn12 = assign30950_e44804_d_n12;
        locals.var_t0__blk895_dn17 = assign30950_e44804_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign30960_e44820, assign30960_e44820_d_n0, assign30960_e44820_d_n2, assign30960_e44820_d_n6, assign30960_e44820_d_n7, assign30960_e44820_d_n10, assign30960_e44820_d_n11, assign30960_e44820_d_n12, assign30960_e44820_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30960_e44818: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
        (assign30960_e44818, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)),)
    } else {
        (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17,)
    }
};
        locals.var_cnst1over__blk956 = assign30960_e44820;
        locals.var_cnst1over__blk956_dn0 = assign30960_e44820_d_n0;
        locals.var_cnst1over__blk956_dn2 = assign30960_e44820_d_n2;
        locals.var_cnst1over__blk956_dn6 = assign30960_e44820_d_n6;
        locals.var_cnst1over__blk956_dn7 = assign30960_e44820_d_n7;
        locals.var_cnst1over__blk956_dn10 = assign30960_e44820_d_n10;
        locals.var_cnst1over__blk956_dn11 = assign30960_e44820_d_n11;
        locals.var_cnst1over__blk956_dn12 = assign30960_e44820_d_n12;
        locals.var_cnst1over__blk956_dn17 = assign30960_e44820_d_n17;
        locals.var_cnst1over__blk956_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30970_e44836, assign30970_e44836_d_n0, assign30970_e44836_d_n2, assign30970_e44836_d_n6, assign30970_e44836_d_n7, assign30970_e44836_d_n10, assign30970_e44836_d_n11, assign30970_e44836_d_n12, assign30970_e44836_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30970_e44834: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
        (assign30970_e44834, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)),)
    } else {
        (locals.var_gammachi__blk957, locals.var_gammachi__blk957_dn0, locals.var_gammachi__blk957_dn2, locals.var_gammachi__blk957_dn6, locals.var_gammachi__blk957_dn7, locals.var_gammachi__blk957_dn10, locals.var_gammachi__blk957_dn11, locals.var_gammachi__blk957_dn12, locals.var_gammachi__blk957_dn17,)
    }
};
        locals.var_gammachi__blk957 = assign30970_e44836;
        locals.var_gammachi__blk957_dn0 = assign30970_e44836_d_n0;
        locals.var_gammachi__blk957_dn2 = assign30970_e44836_d_n2;
        locals.var_gammachi__blk957_dn6 = assign30970_e44836_d_n6;
        locals.var_gammachi__blk957_dn7 = assign30970_e44836_d_n7;
        locals.var_gammachi__blk957_dn10 = assign30970_e44836_d_n10;
        locals.var_gammachi__blk957_dn11 = assign30970_e44836_d_n11;
        locals.var_gammachi__blk957_dn12 = assign30970_e44836_d_n12;
        locals.var_gammachi__blk957_dn17 = assign30970_e44836_d_n17;
        locals.var_gammachi__blk957_rv = 0.0;

        let (assign30980_e44852, assign30980_e44852_d_n0, assign30980_e44852_d_n2, assign30980_e44852_d_n6, assign30980_e44852_d_n7, assign30980_e44852_d_n10, assign30980_e44852_d_n11, assign30980_e44852_d_n12, assign30980_e44852_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30980_e44850: f64 = (locals.var_beta2 * locals.var_fac1p2__blk930);
        (assign30980_e44850, (locals.var_beta2 * locals.var_fac1p2__blk930_dn0), (locals.var_beta2 * locals.var_fac1p2__blk930_dn2), (locals.var_beta2 * locals.var_fac1p2__blk930_dn6), (locals.var_beta2 * locals.var_fac1p2__blk930_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk930) + (locals.var_beta2 * locals.var_fac1p2__blk930_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk930_dn11), (locals.var_beta2 * locals.var_fac1p2__blk930_dn12), (locals.var_beta2 * locals.var_fac1p2__blk930_dn17),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign30980_e44852;
        locals.var_t0__blk895_dn0 = assign30980_e44852_d_n0;
        locals.var_t0__blk895_dn2 = assign30980_e44852_d_n2;
        locals.var_t0__blk895_dn6 = assign30980_e44852_d_n6;
        locals.var_t0__blk895_dn7 = assign30980_e44852_d_n7;
        locals.var_t0__blk895_dn10 = assign30980_e44852_d_n10;
        locals.var_t0__blk895_dn11 = assign30980_e44852_d_n11;
        locals.var_t0__blk895_dn12 = assign30980_e44852_d_n12;
        locals.var_t0__blk895_dn17 = assign30980_e44852_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign30990_e44868, assign30990_e44868_d_n0, assign30990_e44868_d_n2, assign30990_e44868_d_n6, assign30990_e44868_d_n7, assign30990_e44868_d_n10, assign30990_e44868_d_n11, assign30990_e44868_d_n12, assign30990_e44868_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign30990_e44866: f64 = (locals.var_beta * locals.var_vgpld_shift__blk955);
        (assign30990_e44866, (locals.var_beta * locals.var_vgpld_shift__blk955_dn0), (locals.var_beta * locals.var_vgpld_shift__blk955_dn2), (locals.var_beta * locals.var_vgpld_shift__blk955_dn6), (locals.var_beta * locals.var_vgpld_shift__blk955_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk955) + (locals.var_beta * locals.var_vgpld_shift__blk955_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk955_dn11), (locals.var_beta * locals.var_vgpld_shift__blk955_dn12), (locals.var_beta * locals.var_vgpld_shift__blk955_dn17),)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign30990_e44868;
        locals.var_psi__blk958_dn0 = assign30990_e44868_d_n0;
        locals.var_psi__blk958_dn2 = assign30990_e44868_d_n2;
        locals.var_psi__blk958_dn6 = assign30990_e44868_d_n6;
        locals.var_psi__blk958_dn7 = assign30990_e44868_d_n7;
        locals.var_psi__blk958_dn10 = assign30990_e44868_d_n10;
        locals.var_psi__blk958_dn11 = assign30990_e44868_d_n11;
        locals.var_psi__blk958_dn12 = assign30990_e44868_d_n12;
        locals.var_psi__blk958_dn17 = assign30990_e44868_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign31000_e44898, assign31000_e44898_d_n0, assign31000_e44898_d_n2, assign31000_e44898_d_n6, assign31000_e44898_d_n7, assign31000_e44898_d_n10, assign31000_e44898_d_n11, assign31000_e44898_d_n12, assign31000_e44898_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31000_e44882: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
        let assign31000_e44885: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
        let assign31000_e44886: f64 = (assign31000_e44882 + assign31000_e44885);
        let assign31000_e44887: f64 = (assign31000_e44886).ln();
        let assign31000_e44890: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
        let assign31000_e44891: f64 = (assign31000_e44890).ln();
        let assign31000_e44892: f64 = (assign31000_e44887 - assign31000_e44891);
        let assign31000_e44895: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
        let assign31000_e44896: f64 = (assign31000_e44892 + assign31000_e44895);
        (assign31000_e44896, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign31000_e44890)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17,)
    }
};
        locals.var_chi_1__blk959 = assign31000_e44898;
        locals.var_chi_1__blk959_dn0 = assign31000_e44898_d_n0;
        locals.var_chi_1__blk959_dn2 = assign31000_e44898_d_n2;
        locals.var_chi_1__blk959_dn6 = assign31000_e44898_d_n6;
        locals.var_chi_1__blk959_dn7 = assign31000_e44898_d_n7;
        locals.var_chi_1__blk959_dn10 = assign31000_e44898_d_n10;
        locals.var_chi_1__blk959_dn11 = assign31000_e44898_d_n11;
        locals.var_chi_1__blk959_dn12 = assign31000_e44898_d_n12;
        locals.var_chi_1__blk959_dn17 = assign31000_e44898_d_n17;
        locals.var_chi_1__blk959_rv = 0.0;

        let (assign31010_e44916, assign31010_e44916_d_n0, assign31010_e44916_d_n2, assign31010_e44916_d_n6, assign31010_e44916_d_n7, assign31010_e44916_d_n10, assign31010_e44916_d_n11, assign31010_e44916_d_n12, assign31010_e44916_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31010_e44912: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
        let assign31010_e44914: f64 = (assign31010_e44912 - 1.0);
        (assign31010_e44914, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign31010_e44916;
        locals.var_tmf1_dn0 = assign31010_e44916_d_n0;
        locals.var_tmf1_dn2 = assign31010_e44916_d_n2;
        locals.var_tmf1_dn6 = assign31010_e44916_d_n6;
        locals.var_tmf1_dn7 = assign31010_e44916_d_n7;
        locals.var_tmf1_dn10 = assign31010_e44916_d_n10;
        locals.var_tmf1_dn11 = assign31010_e44916_d_n11;
        locals.var_tmf1_dn12 = assign31010_e44916_d_n12;
        locals.var_tmf1_dn17 = assign31010_e44916_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign31020_e44934, assign31020_e44934_d_n0, assign31020_e44934_d_n2, assign31020_e44934_d_n6, assign31020_e44934_d_n7, assign31020_e44934_d_n10, assign31020_e44934_d_n11, assign31020_e44934_d_n12, assign31020_e44934_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31020_e44930: f64 = (4.0 * locals.var_psi__blk958);
        let assign31020_e44932: f64 = assign31020_e44930;
        (assign31020_e44932, (4.0 * locals.var_psi__blk958_dn0), (4.0 * locals.var_psi__blk958_dn2), (4.0 * locals.var_psi__blk958_dn6), (4.0 * locals.var_psi__blk958_dn7), (4.0 * locals.var_psi__blk958_dn10), (4.0 * locals.var_psi__blk958_dn11), (4.0 * locals.var_psi__blk958_dn12), (4.0 * locals.var_psi__blk958_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31020_e44934;
        locals.var_tmf2_dn0 = assign31020_e44934_d_n0;
        locals.var_tmf2_dn2 = assign31020_e44934_d_n2;
        locals.var_tmf2_dn6 = assign31020_e44934_d_n6;
        locals.var_tmf2_dn7 = assign31020_e44934_d_n7;
        locals.var_tmf2_dn10 = assign31020_e44934_d_n10;
        locals.var_tmf2_dn11 = assign31020_e44934_d_n11;
        locals.var_tmf2_dn12 = assign31020_e44934_d_n12;
        locals.var_tmf2_dn17 = assign31020_e44934_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31030_e44954, assign31030_e44954_d_n0, assign31030_e44954_d_n2, assign31030_e44954_d_n6, assign31030_e44954_d_n7, assign31030_e44954_d_n10, assign31030_e44954_d_n11, assign31030_e44954_d_n12, assign31030_e44954_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let (assign31030_e44952, assign31030_e44952_d_n0, assign31030_e44952_d_n2, assign31030_e44952_d_n6, assign31030_e44952_d_n7, assign31030_e44952_d_n10, assign31030_e44952_d_n11, assign31030_e44952_d_n12, assign31030_e44952_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign31030_e44951: f64 = (-locals.var_tmf2);
                (assign31030_e44951, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign31030_e44952, assign31030_e44952_d_n0, assign31030_e44952_d_n2, assign31030_e44952_d_n6, assign31030_e44952_d_n7, assign31030_e44952_d_n10, assign31030_e44952_d_n11, assign31030_e44952_d_n12, assign31030_e44952_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31030_e44954;
        locals.var_tmf2_dn0 = assign31030_e44954_d_n0;
        locals.var_tmf2_dn2 = assign31030_e44954_d_n2;
        locals.var_tmf2_dn6 = assign31030_e44954_d_n6;
        locals.var_tmf2_dn7 = assign31030_e44954_d_n7;
        locals.var_tmf2_dn10 = assign31030_e44954_d_n10;
        locals.var_tmf2_dn11 = assign31030_e44954_d_n11;
        locals.var_tmf2_dn12 = assign31030_e44954_d_n12;
        locals.var_tmf2_dn17 = assign31030_e44954_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31040_e44973, assign31040_e44973_d_n0, assign31040_e44973_d_n2, assign31040_e44973_d_n6, assign31040_e44973_d_n7, assign31040_e44973_d_n10, assign31040_e44973_d_n11, assign31040_e44973_d_n12, assign31040_e44973_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31040_e44968: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign31040_e44970: f64 = (assign31040_e44968 + locals.var_tmf2);
        let assign31040_e44971: f64 = (assign31040_e44970).sqrt();
        (assign31040_e44971, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31040_e44971)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31040_e44973;
        locals.var_tmf2_dn0 = assign31040_e44973_d_n0;
        locals.var_tmf2_dn2 = assign31040_e44973_d_n2;
        locals.var_tmf2_dn6 = assign31040_e44973_d_n6;
        locals.var_tmf2_dn7 = assign31040_e44973_d_n7;
        locals.var_tmf2_dn10 = assign31040_e44973_d_n10;
        locals.var_tmf2_dn11 = assign31040_e44973_d_n11;
        locals.var_tmf2_dn12 = assign31040_e44973_d_n12;
        locals.var_tmf2_dn17 = assign31040_e44973_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31050_e44993, assign31050_e44993_d_n0, assign31050_e44993_d_n2, assign31050_e44993_d_n6, assign31050_e44993_d_n7, assign31050_e44993_d_n10, assign31050_e44993_d_n11, assign31050_e44993_d_n12, assign31050_e44993_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31050_e44989: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign31050_e44990: f64 = (1.0 + assign31050_e44989);
        let assign31050_e44991: f64 = (0.5 * assign31050_e44990);
        (assign31050_e44991, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31050_e44993;
        locals.var_t1__blk896_dn0 = assign31050_e44993_d_n0;
        locals.var_t1__blk896_dn2 = assign31050_e44993_d_n2;
        locals.var_t1__blk896_dn6 = assign31050_e44993_d_n6;
        locals.var_t1__blk896_dn7 = assign31050_e44993_d_n7;
        locals.var_t1__blk896_dn10 = assign31050_e44993_d_n10;
        locals.var_t1__blk896_dn11 = assign31050_e44993_d_n11;
        locals.var_t1__blk896_dn12 = assign31050_e44993_d_n12;
        locals.var_t1__blk896_dn17 = assign31050_e44993_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31060_e45017, assign31060_e45017_d_n0, assign31060_e45017_d_n2, assign31060_e45017_d_n6, assign31060_e45017_d_n7, assign31060_e45017_d_n10, assign31060_e45017_d_n11, assign31060_e45017_d_n12, assign31060_e45017_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31060_e45010: f64 = 2.0;
        let assign31060_e45011: f64 = (locals.var_tmf1 + assign31060_e45010);
        let assign31060_e45013: f64 = (assign31060_e45011 / locals.var_tmf2);
        let assign31060_e45014: f64 = (1.0 - assign31060_e45013);
        let assign31060_e45015: f64 = (0.5 * assign31060_e45014);
        (assign31060_e45015, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign31060_e45017;
        locals.var_t2__blk897_dn0 = assign31060_e45017_d_n0;
        locals.var_t2__blk897_dn2 = assign31060_e45017_d_n2;
        locals.var_t2__blk897_dn6 = assign31060_e45017_d_n6;
        locals.var_t2__blk897_dn7 = assign31060_e45017_d_n7;
        locals.var_t2__blk897_dn10 = assign31060_e45017_d_n10;
        locals.var_t2__blk897_dn11 = assign31060_e45017_d_n11;
        locals.var_t2__blk897_dn12 = assign31060_e45017_d_n12;
        locals.var_t2__blk897_dn17 = assign31060_e45017_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign31070_e45037, assign31070_e45037_d_n0, assign31070_e45037_d_n2, assign31070_e45037_d_n6, assign31070_e45037_d_n7, assign31070_e45037_d_n10, assign31070_e45037_d_n11, assign31070_e45037_d_n12, assign31070_e45037_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31070_e45033: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign31070_e45034: f64 = (0.5 * assign31070_e45033);
        let assign31070_e45035: f64 = (locals.var_psi__blk958 - assign31070_e45034);
        (assign31070_e45035, (locals.var_psi__blk958_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk958_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk958_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk958_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk958_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk958_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk958_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk958_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17,)
    }
};
        locals.var_chi_1__blk959 = assign31070_e45037;
        locals.var_chi_1__blk959_dn0 = assign31070_e45037_d_n0;
        locals.var_chi_1__blk959_dn2 = assign31070_e45037_d_n2;
        locals.var_chi_1__blk959_dn6 = assign31070_e45037_d_n6;
        locals.var_chi_1__blk959_dn7 = assign31070_e45037_d_n7;
        locals.var_chi_1__blk959_dn10 = assign31070_e45037_d_n10;
        locals.var_chi_1__blk959_dn11 = assign31070_e45037_d_n11;
        locals.var_chi_1__blk959_dn12 = assign31070_e45037_d_n12;
        locals.var_chi_1__blk959_dn17 = assign31070_e45037_d_n17;
        locals.var_chi_1__blk959_rv = 0.0;

        let (assign31080_e45053, assign31080_e45053_d_n0, assign31080_e45053_d_n2, assign31080_e45053_d_n6, assign31080_e45053_d_n7, assign31080_e45053_d_n10, assign31080_e45053_d_n11, assign31080_e45053_d_n12, assign31080_e45053_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31080_e45051: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
        (assign31080_e45051, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17),)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign31080_e45053;
        locals.var_psi__blk958_dn0 = assign31080_e45053_d_n0;
        locals.var_psi__blk958_dn2 = assign31080_e45053_d_n2;
        locals.var_psi__blk958_dn6 = assign31080_e45053_d_n6;
        locals.var_psi__blk958_dn7 = assign31080_e45053_d_n7;
        locals.var_psi__blk958_dn10 = assign31080_e45053_d_n10;
        locals.var_psi__blk958_dn11 = assign31080_e45053_d_n11;
        locals.var_psi__blk958_dn12 = assign31080_e45053_d_n12;
        locals.var_psi__blk958_dn17 = assign31080_e45053_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign31090_e45071, assign31090_e45071_d_n0, assign31090_e45071_d_n2, assign31090_e45071_d_n6, assign31090_e45071_d_n7, assign31090_e45071_d_n10, assign31090_e45071_d_n11, assign31090_e45071_d_n12, assign31090_e45071_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31090_e45068: f64 = (locals.var_beta * 0.1);
        let assign31090_e45069: f64 = (locals.var_psi__blk958 + assign31090_e45068);
        (assign31090_e45069, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, (locals.var_psi__blk958_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign31090_e45071;
        locals.var_psi__blk958_dn0 = assign31090_e45071_d_n0;
        locals.var_psi__blk958_dn2 = assign31090_e45071_d_n2;
        locals.var_psi__blk958_dn6 = assign31090_e45071_d_n6;
        locals.var_psi__blk958_dn7 = assign31090_e45071_d_n7;
        locals.var_psi__blk958_dn10 = assign31090_e45071_d_n10;
        locals.var_psi__blk958_dn11 = assign31090_e45071_d_n11;
        locals.var_psi__blk958_dn12 = assign31090_e45071_d_n12;
        locals.var_psi__blk958_dn17 = assign31090_e45071_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign31100_e45101, assign31100_e45101_d_n0, assign31100_e45101_d_n2, assign31100_e45101_d_n6, assign31100_e45101_d_n7, assign31100_e45101_d_n10, assign31100_e45101_d_n11, assign31100_e45101_d_n12, assign31100_e45101_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31100_e45085: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
        let assign31100_e45088: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
        let assign31100_e45089: f64 = (assign31100_e45085 + assign31100_e45088);
        let assign31100_e45090: f64 = (assign31100_e45089).ln();
        let assign31100_e45093: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
        let assign31100_e45094: f64 = (assign31100_e45093).ln();
        let assign31100_e45095: f64 = (assign31100_e45090 - assign31100_e45094);
        let assign31100_e45098: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
        let assign31100_e45099: f64 = (assign31100_e45095 + assign31100_e45098);
        (assign31100_e45099, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign31100_e45093)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi_b__blk960, locals.var_chi_b__blk960_dn0, locals.var_chi_b__blk960_dn2, locals.var_chi_b__blk960_dn6, locals.var_chi_b__blk960_dn7, locals.var_chi_b__blk960_dn10, locals.var_chi_b__blk960_dn11, locals.var_chi_b__blk960_dn12, locals.var_chi_b__blk960_dn17,)
    }
};
        locals.var_chi_b__blk960 = assign31100_e45101;
        locals.var_chi_b__blk960_dn0 = assign31100_e45101_d_n0;
        locals.var_chi_b__blk960_dn2 = assign31100_e45101_d_n2;
        locals.var_chi_b__blk960_dn6 = assign31100_e45101_d_n6;
        locals.var_chi_b__blk960_dn7 = assign31100_e45101_d_n7;
        locals.var_chi_b__blk960_dn10 = assign31100_e45101_d_n10;
        locals.var_chi_b__blk960_dn11 = assign31100_e45101_d_n11;
        locals.var_chi_b__blk960_dn12 = assign31100_e45101_d_n12;
        locals.var_chi_b__blk960_dn17 = assign31100_e45101_d_n17;
        locals.var_chi_b__blk960_rv = 0.0;

        let (assign31110_e45115, assign31110_e45115_d_n0, assign31110_e45115_d_n2, assign31110_e45115_d_n6, assign31110_e45115_d_n7, assign31110_e45115_d_n10, assign31110_e45115_d_n11, assign31110_e45115_d_n12, assign31110_e45115_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    } else {
        (locals.var_chi_a__blk961, locals.var_chi_a__blk961_dn0, locals.var_chi_a__blk961_dn2, locals.var_chi_a__blk961_dn6, locals.var_chi_a__blk961_dn7, locals.var_chi_a__blk961_dn10, locals.var_chi_a__blk961_dn11, locals.var_chi_a__blk961_dn12, locals.var_chi_a__blk961_dn17,)
    }
};
        locals.var_chi_a__blk961 = assign31110_e45115;
        locals.var_chi_a__blk961_dn0 = assign31110_e45115_d_n0;
        locals.var_chi_a__blk961_dn2 = assign31110_e45115_d_n2;
        locals.var_chi_a__blk961_dn6 = assign31110_e45115_d_n6;
        locals.var_chi_a__blk961_dn7 = assign31110_e45115_d_n7;
        locals.var_chi_a__blk961_dn10 = assign31110_e45115_d_n10;
        locals.var_chi_a__blk961_dn11 = assign31110_e45115_d_n11;
        locals.var_chi_a__blk961_dn12 = assign31110_e45115_d_n12;
        locals.var_chi_a__blk961_dn17 = assign31110_e45115_d_n17;
        locals.var_chi_a__blk961_rv = 0.0;

        let (assign31120_e45135, assign31120_e45135_d_n0, assign31120_e45135_d_n2, assign31120_e45135_d_n6, assign31120_e45135_d_n7, assign31120_e45135_d_n10, assign31120_e45135_d_n11, assign31120_e45135_d_n12, assign31120_e45135_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31120_e45129: f64 = (locals.var_chi_b__blk960 - locals.var_chi_a__blk961);
        let assign31120_e45132: f64 = (0.0008 * 75.0);
        let assign31120_e45133: f64 = (assign31120_e45129 - assign31120_e45132);
        (assign31120_e45133, (locals.var_chi_b__blk960_dn0 - locals.var_chi_a__blk961_dn0), (locals.var_chi_b__blk960_dn2 - locals.var_chi_a__blk961_dn2), (locals.var_chi_b__blk960_dn6 - locals.var_chi_a__blk961_dn6), (locals.var_chi_b__blk960_dn7 - locals.var_chi_a__blk961_dn7), (locals.var_chi_b__blk960_dn10 - locals.var_chi_a__blk961_dn10), (locals.var_chi_b__blk960_dn11 - locals.var_chi_a__blk961_dn11), (locals.var_chi_b__blk960_dn12 - locals.var_chi_a__blk961_dn12), (locals.var_chi_b__blk960_dn17 - locals.var_chi_a__blk961_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign31120_e45135;
        locals.var_tmf1_dn0 = assign31120_e45135_d_n0;
        locals.var_tmf1_dn2 = assign31120_e45135_d_n2;
        locals.var_tmf1_dn6 = assign31120_e45135_d_n6;
        locals.var_tmf1_dn7 = assign31120_e45135_d_n7;
        locals.var_tmf1_dn10 = assign31120_e45135_d_n10;
        locals.var_tmf1_dn11 = assign31120_e45135_d_n11;
        locals.var_tmf1_dn12 = assign31120_e45135_d_n12;
        locals.var_tmf1_dn17 = assign31120_e45135_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign31130_e45155, assign31130_e45155_d_n0, assign31130_e45155_d_n2, assign31130_e45155_d_n6, assign31130_e45155_d_n7, assign31130_e45155_d_n10, assign31130_e45155_d_n11, assign31130_e45155_d_n12, assign31130_e45155_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31130_e45149: f64 = (4.0 * locals.var_chi_b__blk960);
        let assign31130_e45152: f64 = (0.0008 * 75.0);
        let assign31130_e45153: f64 = (assign31130_e45149 * assign31130_e45152);
        (assign31130_e45153, ((4.0 * locals.var_chi_b__blk960_dn0) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn2) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn6) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn7) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn10) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn11) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn12) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn17) * assign31130_e45152),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31130_e45155;
        locals.var_tmf2_dn0 = assign31130_e45155_d_n0;
        locals.var_tmf2_dn2 = assign31130_e45155_d_n2;
        locals.var_tmf2_dn6 = assign31130_e45155_d_n6;
        locals.var_tmf2_dn7 = assign31130_e45155_d_n7;
        locals.var_tmf2_dn10 = assign31130_e45155_d_n10;
        locals.var_tmf2_dn11 = assign31130_e45155_d_n11;
        locals.var_tmf2_dn12 = assign31130_e45155_d_n12;
        locals.var_tmf2_dn17 = assign31130_e45155_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31140_e45175, assign31140_e45175_d_n0, assign31140_e45175_d_n2, assign31140_e45175_d_n6, assign31140_e45175_d_n7, assign31140_e45175_d_n10, assign31140_e45175_d_n11, assign31140_e45175_d_n12, assign31140_e45175_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let (assign31140_e45173, assign31140_e45173_d_n0, assign31140_e45173_d_n2, assign31140_e45173_d_n6, assign31140_e45173_d_n7, assign31140_e45173_d_n10, assign31140_e45173_d_n11, assign31140_e45173_d_n12, assign31140_e45173_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign31140_e45172: f64 = (-locals.var_tmf2);
                (assign31140_e45172, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign31140_e45173, assign31140_e45173_d_n0, assign31140_e45173_d_n2, assign31140_e45173_d_n6, assign31140_e45173_d_n7, assign31140_e45173_d_n10, assign31140_e45173_d_n11, assign31140_e45173_d_n12, assign31140_e45173_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31140_e45175;
        locals.var_tmf2_dn0 = assign31140_e45175_d_n0;
        locals.var_tmf2_dn2 = assign31140_e45175_d_n2;
        locals.var_tmf2_dn6 = assign31140_e45175_d_n6;
        locals.var_tmf2_dn7 = assign31140_e45175_d_n7;
        locals.var_tmf2_dn10 = assign31140_e45175_d_n10;
        locals.var_tmf2_dn11 = assign31140_e45175_d_n11;
        locals.var_tmf2_dn12 = assign31140_e45175_d_n12;
        locals.var_tmf2_dn17 = assign31140_e45175_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31150_e45194, assign31150_e45194_d_n0, assign31150_e45194_d_n2, assign31150_e45194_d_n6, assign31150_e45194_d_n7, assign31150_e45194_d_n10, assign31150_e45194_d_n11, assign31150_e45194_d_n12, assign31150_e45194_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31150_e45189: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign31150_e45191: f64 = (assign31150_e45189 + locals.var_tmf2);
        let assign31150_e45192: f64 = (assign31150_e45191).sqrt();
        (assign31150_e45192, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31150_e45192)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31150_e45194;
        locals.var_tmf2_dn0 = assign31150_e45194_d_n0;
        locals.var_tmf2_dn2 = assign31150_e45194_d_n2;
        locals.var_tmf2_dn6 = assign31150_e45194_d_n6;
        locals.var_tmf2_dn7 = assign31150_e45194_d_n7;
        locals.var_tmf2_dn10 = assign31150_e45194_d_n10;
        locals.var_tmf2_dn11 = assign31150_e45194_d_n11;
        locals.var_tmf2_dn12 = assign31150_e45194_d_n12;
        locals.var_tmf2_dn17 = assign31150_e45194_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31160_e45214, assign31160_e45214_d_n0, assign31160_e45214_d_n2, assign31160_e45214_d_n6, assign31160_e45214_d_n7, assign31160_e45214_d_n10, assign31160_e45214_d_n11, assign31160_e45214_d_n12, assign31160_e45214_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31160_e45210: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign31160_e45211: f64 = (1.0 + assign31160_e45210);
        let assign31160_e45212: f64 = (0.5 * assign31160_e45211);
        (assign31160_e45212, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31160_e45214;
        locals.var_t1__blk896_dn0 = assign31160_e45214_d_n0;
        locals.var_t1__blk896_dn2 = assign31160_e45214_d_n2;
        locals.var_t1__blk896_dn6 = assign31160_e45214_d_n6;
        locals.var_t1__blk896_dn7 = assign31160_e45214_d_n7;
        locals.var_t1__blk896_dn10 = assign31160_e45214_d_n10;
        locals.var_t1__blk896_dn11 = assign31160_e45214_d_n11;
        locals.var_t1__blk896_dn12 = assign31160_e45214_d_n12;
        locals.var_t1__blk896_dn17 = assign31160_e45214_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31170_e45240, assign31170_e45240_d_n0, assign31170_e45240_d_n2, assign31170_e45240_d_n6, assign31170_e45240_d_n7, assign31170_e45240_d_n10, assign31170_e45240_d_n11, assign31170_e45240_d_n12, assign31170_e45240_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31170_e45231: f64 = (2.0 * 0.0008);
        let assign31170_e45233: f64 = (assign31170_e45231 * 75.0);
        let assign31170_e45234: f64 = (locals.var_tmf1 + assign31170_e45233);
        let assign31170_e45236: f64 = (assign31170_e45234 / locals.var_tmf2);
        let assign31170_e45237: f64 = (1.0 - assign31170_e45236);
        let assign31170_e45238: f64 = (0.5 * assign31170_e45237);
        (assign31170_e45238, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign31170_e45240;
        locals.var_t2__blk897_dn0 = assign31170_e45240_d_n0;
        locals.var_t2__blk897_dn2 = assign31170_e45240_d_n2;
        locals.var_t2__blk897_dn6 = assign31170_e45240_d_n6;
        locals.var_t2__blk897_dn7 = assign31170_e45240_d_n7;
        locals.var_t2__blk897_dn10 = assign31170_e45240_d_n10;
        locals.var_t2__blk897_dn11 = assign31170_e45240_d_n11;
        locals.var_t2__blk897_dn12 = assign31170_e45240_d_n12;
        locals.var_t2__blk897_dn17 = assign31170_e45240_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign31180_e45260, assign31180_e45260_d_n0, assign31180_e45260_d_n2, assign31180_e45260_d_n6, assign31180_e45260_d_n7, assign31180_e45260_d_n10, assign31180_e45260_d_n11, assign31180_e45260_d_n12, assign31180_e45260_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
        let assign31180_e45256: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign31180_e45257: f64 = (0.5 * assign31180_e45256);
        let assign31180_e45258: f64 = (locals.var_chi_b__blk960 - assign31180_e45257);
        (assign31180_e45258, (locals.var_chi_b__blk960_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk960_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk960_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk960_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk960_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk960_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk960_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk960_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign31180_e45260;
        locals.var_chi__blk943_dn0 = assign31180_e45260_d_n0;
        locals.var_chi__blk943_dn2 = assign31180_e45260_d_n2;
        locals.var_chi__blk943_dn6 = assign31180_e45260_d_n6;
        locals.var_chi__blk943_dn7 = assign31180_e45260_d_n7;
        locals.var_chi__blk943_dn10 = assign31180_e45260_d_n10;
        locals.var_chi__blk943_dn11 = assign31180_e45260_d_n11;
        locals.var_chi__blk943_dn12 = assign31180_e45260_d_n12;
        locals.var_chi__blk943_dn17 = assign31180_e45260_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign31190_e45276, assign31190_e45276_d_n0, assign31190_e45276_d_n2, assign31190_e45276_d_n6, assign31190_e45276_d_n7, assign31190_e45276_d_n10, assign31190_e45276_d_n11, assign31190_e45276_d_n12, assign31190_e45276_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign31190_e45272: f64 = (locals.var_chi__blk943 / locals.var_beta);
        let assign31190_e45274: f64 = (assign31190_e45272 - locals.var_vxbgmtcl__blk921);
        (assign31190_e45274, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
        locals.var_ps0ld__blk945 = assign31190_e45276;
        locals.var_ps0ld__blk945_dn0 = assign31190_e45276_d_n0;
        locals.var_ps0ld__blk945_dn2 = assign31190_e45276_d_n2;
        locals.var_ps0ld__blk945_dn6 = assign31190_e45276_d_n6;
        locals.var_ps0ld__blk945_dn7 = assign31190_e45276_d_n7;
        locals.var_ps0ld__blk945_dn10 = assign31190_e45276_d_n10;
        locals.var_ps0ld__blk945_dn11 = assign31190_e45276_d_n11;
        locals.var_ps0ld__blk945_dn12 = assign31190_e45276_d_n12;
        locals.var_ps0ld__blk945_dn17 = assign31190_e45276_d_n17;
        locals.var_ps0ld__blk945_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31200_e45294, assign31200_e45294_d_n0, assign31200_e45294_d_n2, assign31200_e45294_d_n6, assign31200_e45294_d_n7, assign31200_e45294_d_n10, assign31200_e45294_d_n11, assign31200_e45294_d_n12, assign31200_e45294_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign31200_e45288: f64 = (locals.var_chi__blk943 - 1.0);
        let assign31200_e45290: f64 = (-locals.var_chi__blk943);
        let assign31200_e45291: f64 = (assign31200_e45290).exp();
        let assign31200_e45292: f64 = (assign31200_e45288 + assign31200_e45291);
        (assign31200_e45292, (locals.var_chi__blk943_dn0 + (assign31200_e45291 * (-locals.var_chi__blk943_dn0))), (locals.var_chi__blk943_dn2 + (assign31200_e45291 * (-locals.var_chi__blk943_dn2))), (locals.var_chi__blk943_dn6 + (assign31200_e45291 * (-locals.var_chi__blk943_dn6))), (locals.var_chi__blk943_dn7 + (assign31200_e45291 * (-locals.var_chi__blk943_dn7))), (locals.var_chi__blk943_dn10 + (assign31200_e45291 * (-locals.var_chi__blk943_dn10))), (locals.var_chi__blk943_dn11 + (assign31200_e45291 * (-locals.var_chi__blk943_dn11))), (locals.var_chi__blk943_dn12 + (assign31200_e45291 * (-locals.var_chi__blk943_dn12))), (locals.var_chi__blk943_dn17 + (assign31200_e45291 * (-locals.var_chi__blk943_dn17))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31200_e45294;
        locals.var_t1__blk896_dn0 = assign31200_e45294_d_n0;
        locals.var_t1__blk896_dn2 = assign31200_e45294_d_n2;
        locals.var_t1__blk896_dn6 = assign31200_e45294_d_n6;
        locals.var_t1__blk896_dn7 = assign31200_e45294_d_n7;
        locals.var_t1__blk896_dn10 = assign31200_e45294_d_n10;
        locals.var_t1__blk896_dn11 = assign31200_e45294_d_n11;
        locals.var_t1__blk896_dn12 = assign31200_e45294_d_n12;
        locals.var_t1__blk896_dn17 = assign31200_e45294_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let assign31210_e45298: f64 = (10.0 * 2.220446049250313e-16);
        let assign31210_e45299: f64 = if locals.var_t1__blk896 < assign31210_e45298 { 1.0 } else { 0.0 };
        locals.var_guard1008 = assign31210_e45299;
        locals.var_guard1008_rv = 0.0;

        let (assign31220_e45315, assign31220_e45315_d_n0, assign31220_e45315_d_n2, assign31220_e45315_d_n6, assign31220_e45315_d_n7, assign31220_e45315_d_n10, assign31220_e45315_d_n11, assign31220_e45315_d_n12, assign31220_e45315_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1008 != 0.0)) {
        let assign31220_e45313: f64 = (10.0 * 2.220446049250313e-16);
        (assign31220_e45313, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31220_e45315;
        locals.var_t1__blk896_dn0 = assign31220_e45315_d_n0;
        locals.var_t1__blk896_dn2 = assign31220_e45315_d_n2;
        locals.var_t1__blk896_dn6 = assign31220_e45315_d_n6;
        locals.var_t1__blk896_dn7 = assign31220_e45315_d_n7;
        locals.var_t1__blk896_dn10 = assign31220_e45315_d_n10;
        locals.var_t1__blk896_dn11 = assign31220_e45315_d_n11;
        locals.var_t1__blk896_dn12 = assign31220_e45315_d_n12;
        locals.var_t1__blk896_dn17 = assign31220_e45315_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31230_e45328, assign31230_e45328_d_n0, assign31230_e45328_d_n2, assign31230_e45328_d_n6, assign31230_e45328_d_n7, assign31230_e45328_d_n10, assign31230_e45328_d_n11, assign31230_e45328_d_n12, assign31230_e45328_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign31230_e45326: f64 = (locals.var_t1__blk896).sqrt();
        (assign31230_e45326, (locals.var_t1__blk896_dn0 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn2 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn6 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn7 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn10 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn11 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn12 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn17 / (2.0 * assign31230_e45326)),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign31230_e45328;
        locals.var_t2__blk897_dn0 = assign31230_e45328_d_n0;
        locals.var_t2__blk897_dn2 = assign31230_e45328_d_n2;
        locals.var_t2__blk897_dn6 = assign31230_e45328_d_n6;
        locals.var_t2__blk897_dn7 = assign31230_e45328_d_n7;
        locals.var_t2__blk897_dn10 = assign31230_e45328_d_n10;
        locals.var_t2__blk897_dn11 = assign31230_e45328_d_n11;
        locals.var_t2__blk897_dn12 = assign31230_e45328_d_n12;
        locals.var_t2__blk897_dn17 = assign31230_e45328_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign31240_e45342, assign31240_e45342_d_n0, assign31240_e45342_d_n2, assign31240_e45342_d_n6, assign31240_e45342_d_n7, assign31240_e45342_d_n10, assign31240_e45342_d_n11, assign31240_e45342_d_n12, assign31240_e45342_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign31240_e45340: f64 = (locals.var_cnst0over__blk928 * locals.var_t2__blk897);
        (assign31240_e45340, ((locals.var_cnst0over__blk928_dn0 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31240_e45342;
        locals.var_qbuld_dn0 = assign31240_e45342_d_n0;
        locals.var_qbuld_dn2 = assign31240_e45342_d_n2;
        locals.var_qbuld_dn6 = assign31240_e45342_d_n6;
        locals.var_qbuld_dn7 = assign31240_e45342_d_n7;
        locals.var_qbuld_dn10 = assign31240_e45342_d_n10;
        locals.var_qbuld_dn11 = assign31240_e45342_d_n11;
        locals.var_qbuld_dn12 = assign31240_e45342_d_n12;
        locals.var_qbuld_dn17 = assign31240_e45342_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign31250_e45358, assign31250_e45358_d_n0, assign31250_e45358_d_n2, assign31250_e45358_d_n6, assign31250_e45358_d_n7, assign31250_e45358_d_n10, assign31250_e45358_d_n11, assign31250_e45358_d_n12, assign31250_e45358_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign31250_e45355: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        let assign31250_e45356: f64 = (locals.var_cox0__blk906 * assign31250_e45355);
        (assign31250_e45356, (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31250_e45358;
        locals.var_qsuld_dn0 = assign31250_e45358_d_n0;
        locals.var_qsuld_dn2 = assign31250_e45358_d_n2;
        locals.var_qsuld_dn6 = assign31250_e45358_d_n6;
        locals.var_qsuld_dn7 = assign31250_e45358_d_n7;
        locals.var_qsuld_dn10 = assign31250_e45358_d_n10;
        locals.var_qsuld_dn11 = assign31250_e45358_d_n11;
        locals.var_qsuld_dn12 = assign31250_e45358_d_n12;
        locals.var_qsuld_dn17 = assign31250_e45358_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign31260_e45361: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1009 = assign31260_e45361;
        locals.var_guard1009_rv = 0.0;

        let (assign31270_e45379, assign31270_e45379_d_n0, assign31270_e45379_d_n2, assign31270_e45379_d_n6, assign31270_e45379_d_n7, assign31270_e45379_d_n10, assign31270_e45379_d_n11, assign31270_e45379_d_n12, assign31270_e45379_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31270_e45375: f64 = (-locals.var_vxbgmtcl__blk921);
        let assign31270_e45376: f64 = (locals.var_beta * assign31270_e45375);
        let assign31270_e45377: f64 = (assign31270_e45376).exp();
        (assign31270_e45377, (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign31270_e45377 * ((locals.var_beta_dn10 * assign31270_e45375) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17,)
    }
};
        locals.var_exp_bvbs__blk962 = assign31270_e45379;
        locals.var_exp_bvbs__blk962_dn0 = assign31270_e45379_d_n0;
        locals.var_exp_bvbs__blk962_dn2 = assign31270_e45379_d_n2;
        locals.var_exp_bvbs__blk962_dn6 = assign31270_e45379_d_n6;
        locals.var_exp_bvbs__blk962_dn7 = assign31270_e45379_d_n7;
        locals.var_exp_bvbs__blk962_dn10 = assign31270_e45379_d_n10;
        locals.var_exp_bvbs__blk962_dn11 = assign31270_e45379_d_n11;
        locals.var_exp_bvbs__blk962_dn12 = assign31270_e45379_d_n12;
        locals.var_exp_bvbs__blk962_dn17 = assign31270_e45379_d_n17;
        locals.var_exp_bvbs__blk962_rv = 0.0;

        let (assign31280_e45395, assign31280_e45395_d_n0, assign31280_e45395_d_n2, assign31280_e45395_d_n6, assign31280_e45395_d_n7, assign31280_e45395_d_n10, assign31280_e45395_d_n11, assign31280_e45395_d_n12, assign31280_e45395_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31280_e45393: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign31280_e45393, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign31280_e45395;
        locals.var_t0__blk895_dn0 = assign31280_e45395_d_n0;
        locals.var_t0__blk895_dn2 = assign31280_e45395_d_n2;
        locals.var_t0__blk895_dn6 = assign31280_e45395_d_n6;
        locals.var_t0__blk895_dn7 = assign31280_e45395_d_n7;
        locals.var_t0__blk895_dn10 = assign31280_e45395_d_n10;
        locals.var_t0__blk895_dn11 = assign31280_e45395_d_n11;
        locals.var_t0__blk895_dn12 = assign31280_e45395_d_n12;
        locals.var_t0__blk895_dn17 = assign31280_e45395_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign31290_e45411, assign31290_e45411_d_n0, assign31290_e45411_d_n2, assign31290_e45411_d_n6, assign31290_e45411_d_n7, assign31290_e45411_d_n10, assign31290_e45411_d_n11, assign31290_e45411_d_n12, assign31290_e45411_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31290_e45409: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
        (assign31290_e45409, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)),)
    } else {
        (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17,)
    }
};
        locals.var_cnst1over__blk956 = assign31290_e45411;
        locals.var_cnst1over__blk956_dn0 = assign31290_e45411_d_n0;
        locals.var_cnst1over__blk956_dn2 = assign31290_e45411_d_n2;
        locals.var_cnst1over__blk956_dn6 = assign31290_e45411_d_n6;
        locals.var_cnst1over__blk956_dn7 = assign31290_e45411_d_n7;
        locals.var_cnst1over__blk956_dn10 = assign31290_e45411_d_n10;
        locals.var_cnst1over__blk956_dn11 = assign31290_e45411_d_n11;
        locals.var_cnst1over__blk956_dn12 = assign31290_e45411_d_n12;
        locals.var_cnst1over__blk956_dn17 = assign31290_e45411_d_n17;
        locals.var_cnst1over__blk956_rv = 0.0;

        let (assign31300_e45427, assign31300_e45427_d_n0, assign31300_e45427_d_n2, assign31300_e45427_d_n6, assign31300_e45427_d_n7, assign31300_e45427_d_n10, assign31300_e45427_d_n11, assign31300_e45427_d_n12, assign31300_e45427_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31300_e45425: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
        (assign31300_e45425, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)),)
    } else {
        (locals.var_cfs1__blk971, locals.var_cfs1__blk971_dn0, locals.var_cfs1__blk971_dn2, locals.var_cfs1__blk971_dn6, locals.var_cfs1__blk971_dn7, locals.var_cfs1__blk971_dn10, locals.var_cfs1__blk971_dn11, locals.var_cfs1__blk971_dn12, locals.var_cfs1__blk971_dn17,)
    }
};
        locals.var_cfs1__blk971 = assign31300_e45427;
        locals.var_cfs1__blk971_dn0 = assign31300_e45427_d_n0;
        locals.var_cfs1__blk971_dn2 = assign31300_e45427_d_n2;
        locals.var_cfs1__blk971_dn6 = assign31300_e45427_d_n6;
        locals.var_cfs1__blk971_dn7 = assign31300_e45427_d_n7;
        locals.var_cfs1__blk971_dn10 = assign31300_e45427_d_n10;
        locals.var_cfs1__blk971_dn11 = assign31300_e45427_d_n11;
        locals.var_cfs1__blk971_dn12 = assign31300_e45427_d_n12;
        locals.var_cfs1__blk971_dn17 = assign31300_e45427_d_n17;
        locals.var_cfs1__blk971_rv = 0.0;

        let (assign31310_e45441,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk918,)
    }
};
        locals.var_flg_conv__blk918 = assign31310_e45441;
        locals.var_flg_conv__blk918_rv = 0.0;

        let (assign31320_e45455, assign31320_e45455_d_n0, assign31320_e45455_d_n2, assign31320_e45455_d_n6, assign31320_e45455_d_n7, assign31320_e45455_d_n10, assign31320_e45455_d_n11, assign31320_e45455_d_n12, assign31320_e45455_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
        locals.var_fs01__blk965 = assign31320_e45455;
        locals.var_fs01__blk965_dn0 = assign31320_e45455_d_n0;
        locals.var_fs01__blk965_dn2 = assign31320_e45455_d_n2;
        locals.var_fs01__blk965_dn6 = assign31320_e45455_d_n6;
        locals.var_fs01__blk965_dn7 = assign31320_e45455_d_n7;
        locals.var_fs01__blk965_dn10 = assign31320_e45455_d_n10;
        locals.var_fs01__blk965_dn11 = assign31320_e45455_d_n11;
        locals.var_fs01__blk965_dn12 = assign31320_e45455_d_n12;
        locals.var_fs01__blk965_dn17 = assign31320_e45455_d_n17;
        locals.var_fs01__blk965_rv = 0.0;

        let (assign31330_e45469, assign31330_e45469_d_n0, assign31330_e45469_d_n2, assign31330_e45469_d_n6, assign31330_e45469_d_n7, assign31330_e45469_d_n10, assign31330_e45469_d_n11, assign31330_e45469_d_n12, assign31330_e45469_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
        locals.var_fs02__blk969 = assign31330_e45469;
        locals.var_fs02__blk969_dn0 = assign31330_e45469_d_n0;
        locals.var_fs02__blk969_dn2 = assign31330_e45469_d_n2;
        locals.var_fs02__blk969_dn6 = assign31330_e45469_d_n6;
        locals.var_fs02__blk969_dn7 = assign31330_e45469_d_n7;
        locals.var_fs02__blk969_dn10 = assign31330_e45469_d_n10;
        locals.var_fs02__blk969_dn11 = assign31330_e45469_d_n11;
        locals.var_fs02__blk969_dn12 = assign31330_e45469_d_n12;
        locals.var_fs02__blk969_dn17 = assign31330_e45469_d_n17;
        locals.var_fs02__blk969_rv = 0.0;

        let (assign31340_e45483,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31340_e45483;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign31350_loop_guard: usize = 0;
        while {
            let assign31350_cond_e45498: f64 = (2.0 * 20.0);
            let assign31350_cond_e45500: f64 = (assign31350_cond_e45498 + 1.0);
            let assign31350_cond_e45502: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_lp_s0 <= assign31350_cond_e45500)) { 1.0 } else { 0.0 };
            assign31350_cond_e45502 != 0.0
        } {
            assign31350_loop_guard += 1;
            assert!(assign31350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31350_body0_e45516, assign31350_body0_e45516_d_n0, assign31350_body0_e45516_d_n2, assign31350_body0_e45516_d_n6, assign31350_body0_e45516_d_n7, assign31350_body0_e45516_d_n10, assign31350_body0_e45516_d_n11, assign31350_body0_e45516_d_n12, assign31350_body0_e45516_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    }
};
            locals.var_fb__blk967 = assign31350_body0_e45516;
            locals.var_fb__blk967_dn0 = assign31350_body0_e45516_d_n0;
            locals.var_fb__blk967_dn2 = assign31350_body0_e45516_d_n2;
            locals.var_fb__blk967_dn6 = assign31350_body0_e45516_d_n6;
            locals.var_fb__blk967_dn7 = assign31350_body0_e45516_d_n7;
            locals.var_fb__blk967_dn10 = assign31350_body0_e45516_d_n10;
            locals.var_fb__blk967_dn11 = assign31350_body0_e45516_d_n11;
            locals.var_fb__blk967_dn12 = assign31350_body0_e45516_d_n12;
            locals.var_fb__blk967_dn17 = assign31350_body0_e45516_d_n17;
            locals.var_fb__blk967_rv = 0.0;
            let (assign31350_body1_e45534, assign31350_body1_e45534_d_n0, assign31350_body1_e45534_d_n2, assign31350_body1_e45534_d_n6, assign31350_body1_e45534_d_n7, assign31350_body1_e45534_d_n10, assign31350_body1_e45534_d_n11, assign31350_body1_e45534_d_n12, assign31350_body1_e45534_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31350_body1_e45531: f64 = (locals.var_ps0ld__blk945 + locals.var_vxbgmtcl__blk921);
        let assign31350_body1_e45532: f64 = (locals.var_beta * assign31350_body1_e45531);
        (assign31350_body1_e45532, (locals.var_beta * (locals.var_ps0ld__blk945_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0ld__blk945_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0ld__blk945_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0ld__blk945_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign31350_body1_e45531) + (locals.var_beta * (locals.var_ps0ld__blk945_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0ld__blk945_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0ld__blk945_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0ld__blk945_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
            locals.var_chi__blk943 = assign31350_body1_e45534;
            locals.var_chi__blk943_dn0 = assign31350_body1_e45534_d_n0;
            locals.var_chi__blk943_dn2 = assign31350_body1_e45534_d_n2;
            locals.var_chi__blk943_dn6 = assign31350_body1_e45534_d_n6;
            locals.var_chi__blk943_dn7 = assign31350_body1_e45534_d_n7;
            locals.var_chi__blk943_dn10 = assign31350_body1_e45534_d_n10;
            locals.var_chi__blk943_dn11 = assign31350_body1_e45534_d_n11;
            locals.var_chi__blk943_dn12 = assign31350_body1_e45534_d_n12;
            locals.var_chi__blk943_dn17 = assign31350_body1_e45534_d_n17;
            locals.var_chi__blk943_rv = 0.0;
            let assign31350_body2_e45537: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1010 = assign31350_body2_e45537;
            locals.var_guard1010_rv = 0.0;
            let (assign31350_body3_e45568, assign31350_body3_e45568_d_n0, assign31350_body3_e45568_d_n2, assign31350_body3_e45568_d_n6, assign31350_body3_e45568_d_n7, assign31350_body3_e45568_d_n10, assign31350_body3_e45568_d_n11, assign31350_body3_e45568_d_n12, assign31350_body3_e45568_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body3_e45553: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
        let assign31350_body3_e45555: f64 = (assign31350_body3_e45553 * locals.var_chi__blk943);
        let assign31350_body3_e45559: f64 = (-0.07053654284009761);
        let assign31350_body3_e45562: f64 = (locals.var_chi__blk943 * 0.006115288895133179);
        let assign31350_body3_e45563: f64 = (assign31350_body3_e45559 + assign31350_body3_e45562);
        let assign31350_body3_e45564: f64 = (locals.var_chi__blk943 * assign31350_body3_e45563);
        let assign31350_body3_e45565: f64 = (0.29693154855771 + assign31350_body3_e45564);
        let assign31350_body3_e45566: f64 = (assign31350_body3_e45555 * assign31350_body3_e45565);
        (assign31350_body3_e45566, ((((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn0)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn0 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn2)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn2 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn6)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn6 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn7)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn7 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn10)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn10 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn11)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn11 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn12)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn12 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn17)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn17 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi__blk963, locals.var_fi__blk963_dn0, locals.var_fi__blk963_dn2, locals.var_fi__blk963_dn6, locals.var_fi__blk963_dn7, locals.var_fi__blk963_dn10, locals.var_fi__blk963_dn11, locals.var_fi__blk963_dn12, locals.var_fi__blk963_dn17,)
    }
};
            locals.var_fi__blk963 = assign31350_body3_e45568;
            locals.var_fi__blk963_dn0 = assign31350_body3_e45568_d_n0;
            locals.var_fi__blk963_dn2 = assign31350_body3_e45568_d_n2;
            locals.var_fi__blk963_dn6 = assign31350_body3_e45568_d_n6;
            locals.var_fi__blk963_dn7 = assign31350_body3_e45568_d_n7;
            locals.var_fi__blk963_dn10 = assign31350_body3_e45568_d_n10;
            locals.var_fi__blk963_dn11 = assign31350_body3_e45568_d_n11;
            locals.var_fi__blk963_dn12 = assign31350_body3_e45568_d_n12;
            locals.var_fi__blk963_dn17 = assign31350_body3_e45568_d_n17;
            locals.var_fi__blk963_rv = 0.0;
            let (assign31350_body4_e45603, assign31350_body4_e45603_d_n0, assign31350_body4_e45603_d_n2, assign31350_body4_e45603_d_n6, assign31350_body4_e45603_d_n7, assign31350_body4_e45603_d_n10, assign31350_body4_e45603_d_n11, assign31350_body4_e45603_d_n12, assign31350_body4_e45603_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body4_e45584: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
        let assign31350_body4_e45587: f64 = (3.0 * 0.29693154855771);
        let assign31350_body4_e45591: f64 = (-0.07053654284009761);
        let assign31350_body4_e45592: f64 = (4.0 * assign31350_body4_e45591);
        let assign31350_body4_e45595: f64 = (locals.var_chi__blk943 * 5.0);
        let assign31350_body4_e45597: f64 = (assign31350_body4_e45595 * 0.006115288895133179);
        let assign31350_body4_e45598: f64 = (assign31350_body4_e45592 + assign31350_body4_e45597);
        let assign31350_body4_e45599: f64 = (locals.var_chi__blk943 * assign31350_body4_e45598);
        let assign31350_body4_e45600: f64 = (assign31350_body4_e45587 + assign31350_body4_e45599);
        let assign31350_body4_e45601: f64 = (assign31350_body4_e45584 * assign31350_body4_e45600);
        (assign31350_body4_e45601, ((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn0 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn2 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn6 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn7 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn10 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn11 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn12 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn17 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi__blk964, locals.var_fi_dchi__blk964_dn0, locals.var_fi_dchi__blk964_dn2, locals.var_fi_dchi__blk964_dn6, locals.var_fi_dchi__blk964_dn7, locals.var_fi_dchi__blk964_dn10, locals.var_fi_dchi__blk964_dn11, locals.var_fi_dchi__blk964_dn12, locals.var_fi_dchi__blk964_dn17,)
    }
};
            locals.var_fi_dchi__blk964 = assign31350_body4_e45603;
            locals.var_fi_dchi__blk964_dn0 = assign31350_body4_e45603_d_n0;
            locals.var_fi_dchi__blk964_dn2 = assign31350_body4_e45603_d_n2;
            locals.var_fi_dchi__blk964_dn6 = assign31350_body4_e45603_d_n6;
            locals.var_fi_dchi__blk964_dn7 = assign31350_body4_e45603_d_n7;
            locals.var_fi_dchi__blk964_dn10 = assign31350_body4_e45603_d_n10;
            locals.var_fi_dchi__blk964_dn11 = assign31350_body4_e45603_d_n11;
            locals.var_fi_dchi__blk964_dn12 = assign31350_body4_e45603_d_n12;
            locals.var_fi_dchi__blk964_dn17 = assign31350_body4_e45603_d_n17;
            locals.var_fi_dchi__blk964_rv = 0.0;
            let (assign31350_body5_e45623, assign31350_body5_e45623_d_n0, assign31350_body5_e45623_d_n2, assign31350_body5_e45623_d_n6, assign31350_body5_e45623_d_n7, assign31350_body5_e45623_d_n10, assign31350_body5_e45623_d_n11, assign31350_body5_e45623_d_n12, assign31350_body5_e45623_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body5_e45619: f64 = (locals.var_cfs1__blk971 * locals.var_fi__blk963);
        let assign31350_body5_e45621: f64 = (assign31350_body5_e45619 * locals.var_fi__blk963);
        (assign31350_body5_e45621, ((((locals.var_cfs1__blk971_dn0 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn0)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn0)), ((((locals.var_cfs1__blk971_dn2 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn2)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn2)), ((((locals.var_cfs1__blk971_dn6 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn6)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn6)), ((((locals.var_cfs1__blk971_dn7 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn7)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn10)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn10)), ((((locals.var_cfs1__blk971_dn11 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn11)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn11)), ((((locals.var_cfs1__blk971_dn12 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn12)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn12)), ((((locals.var_cfs1__blk971_dn17 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn17)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn17)),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign31350_body5_e45623;
            locals.var_fs01__blk965_dn0 = assign31350_body5_e45623_d_n0;
            locals.var_fs01__blk965_dn2 = assign31350_body5_e45623_d_n2;
            locals.var_fs01__blk965_dn6 = assign31350_body5_e45623_d_n6;
            locals.var_fs01__blk965_dn7 = assign31350_body5_e45623_d_n7;
            locals.var_fs01__blk965_dn10 = assign31350_body5_e45623_d_n10;
            locals.var_fs01__blk965_dn11 = assign31350_body5_e45623_d_n11;
            locals.var_fs01__blk965_dn12 = assign31350_body5_e45623_d_n12;
            locals.var_fs01__blk965_dn17 = assign31350_body5_e45623_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign31350_body6_e45647, assign31350_body6_e45647_d_n0, assign31350_body6_e45647_d_n2, assign31350_body6_e45647_d_n6, assign31350_body6_e45647_d_n7, assign31350_body6_e45647_d_n10, assign31350_body6_e45647_d_n11, assign31350_body6_e45647_d_n12, assign31350_body6_e45647_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body6_e45639: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
        let assign31350_body6_e45641: f64 = (assign31350_body6_e45639 * 2.0);
        let assign31350_body6_e45643: f64 = (assign31350_body6_e45641 * locals.var_fi__blk963);
        let assign31350_body6_e45645: f64 = (assign31350_body6_e45643 * locals.var_fi_dchi__blk964);
        (assign31350_body6_e45645, ((((((locals.var_cfs1__blk971_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn0)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn0)), ((((((locals.var_cfs1__blk971_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn2)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn2)), ((((((locals.var_cfs1__blk971_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn6)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn6)), ((((((locals.var_cfs1__blk971_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn7)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn7)), (((((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn10)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn10)), ((((((locals.var_cfs1__blk971_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn11)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn11)), ((((((locals.var_cfs1__blk971_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn12)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn12)), ((((((locals.var_cfs1__blk971_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn17)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign31350_body6_e45647;
            locals.var_fs01_dps0__blk966_dn0 = assign31350_body6_e45647_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign31350_body6_e45647_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign31350_body6_e45647_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign31350_body6_e45647_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign31350_body6_e45647_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign31350_body6_e45647_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign31350_body6_e45647_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign31350_body6_e45647_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign31350_body7_e45683, assign31350_body7_e45683_d_n0, assign31350_body7_e45683_d_n2, assign31350_body7_e45683_d_n6, assign31350_body7_e45683_d_n7, assign31350_body7_e45683_d_n10, assign31350_body7_e45683_d_n11, assign31350_body7_e45683_d_n12, assign31350_body7_e45683_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body7_e45665: f64 = (-0.117851130197758);
        let assign31350_body7_e45670: f64 = (-0.00163730162779191);
        let assign31350_body7_e45673: f64 = (locals.var_chi__blk943 * 6.36964918866352e-5);
        let assign31350_body7_e45674: f64 = (assign31350_body7_e45670 + assign31350_body7_e45673);
        let assign31350_body7_e45675: f64 = (locals.var_chi__blk943 * assign31350_body7_e45674);
        let assign31350_body7_e45676: f64 = (0.0178800506338833 + assign31350_body7_e45675);
        let assign31350_body7_e45677: f64 = (locals.var_chi__blk943 * assign31350_body7_e45676);
        let assign31350_body7_e45678: f64 = (assign31350_body7_e45665 + assign31350_body7_e45677);
        let assign31350_body7_e45679: f64 = (locals.var_chi__blk943 * assign31350_body7_e45678);
        let assign31350_body7_e45680: f64 = (0.707106781186548 + assign31350_body7_e45679);
        let assign31350_body7_e45681: f64 = (locals.var_chi__blk943 * assign31350_body7_e45680);
        (assign31350_body7_e45681, ((locals.var_chi__blk943_dn0 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn2 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn6 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn7 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn10 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn11 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn12 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn17 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    }
};
            locals.var_fb__blk967 = assign31350_body7_e45683;
            locals.var_fb__blk967_dn0 = assign31350_body7_e45683_d_n0;
            locals.var_fb__blk967_dn2 = assign31350_body7_e45683_d_n2;
            locals.var_fb__blk967_dn6 = assign31350_body7_e45683_d_n6;
            locals.var_fb__blk967_dn7 = assign31350_body7_e45683_d_n7;
            locals.var_fb__blk967_dn10 = assign31350_body7_e45683_d_n10;
            locals.var_fb__blk967_dn11 = assign31350_body7_e45683_d_n11;
            locals.var_fb__blk967_dn12 = assign31350_body7_e45683_d_n12;
            locals.var_fb__blk967_dn17 = assign31350_body7_e45683_d_n17;
            locals.var_fb__blk967_rv = 0.0;
            let (assign31350_body8_e45725, assign31350_body8_e45725_d_n0, assign31350_body8_e45725_d_n2, assign31350_body8_e45725_d_n6, assign31350_body8_e45725_d_n7, assign31350_body8_e45725_d_n10, assign31350_body8_e45725_d_n11, assign31350_body8_e45725_d_n12, assign31350_body8_e45725_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body8_e45701: f64 = (-0.117851130197758);
        let assign31350_body8_e45702: f64 = (2.0 * assign31350_body8_e45701);
        let assign31350_body8_e45706: f64 = (3.0 * 0.0178800506338833);
        let assign31350_body8_e45710: f64 = (-0.00163730162779191);
        let assign31350_body8_e45711: f64 = (4.0 * assign31350_body8_e45710);
        let assign31350_body8_e45714: f64 = (locals.var_chi__blk943 * 5.0);
        let assign31350_body8_e45716: f64 = (assign31350_body8_e45714 * 6.36964918866352e-5);
        let assign31350_body8_e45717: f64 = (assign31350_body8_e45711 + assign31350_body8_e45716);
        let assign31350_body8_e45718: f64 = (locals.var_chi__blk943 * assign31350_body8_e45717);
        let assign31350_body8_e45719: f64 = (assign31350_body8_e45706 + assign31350_body8_e45718);
        let assign31350_body8_e45720: f64 = (locals.var_chi__blk943 * assign31350_body8_e45719);
        let assign31350_body8_e45721: f64 = (assign31350_body8_e45702 + assign31350_body8_e45720);
        let assign31350_body8_e45722: f64 = (locals.var_chi__blk943 * assign31350_body8_e45721);
        let assign31350_body8_e45723: f64 = (0.707106781186548 + assign31350_body8_e45722);
        (assign31350_body8_e45723, ((locals.var_chi__blk943_dn0 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn2 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn6 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn7 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn10 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn11 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn12 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn17 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi__blk968, locals.var_fb_dchi__blk968_dn0, locals.var_fb_dchi__blk968_dn2, locals.var_fb_dchi__blk968_dn6, locals.var_fb_dchi__blk968_dn7, locals.var_fb_dchi__blk968_dn10, locals.var_fb_dchi__blk968_dn11, locals.var_fb_dchi__blk968_dn12, locals.var_fb_dchi__blk968_dn17,)
    }
};
            locals.var_fb_dchi__blk968 = assign31350_body8_e45725;
            locals.var_fb_dchi__blk968_dn0 = assign31350_body8_e45725_d_n0;
            locals.var_fb_dchi__blk968_dn2 = assign31350_body8_e45725_d_n2;
            locals.var_fb_dchi__blk968_dn6 = assign31350_body8_e45725_d_n6;
            locals.var_fb_dchi__blk968_dn7 = assign31350_body8_e45725_d_n7;
            locals.var_fb_dchi__blk968_dn10 = assign31350_body8_e45725_d_n10;
            locals.var_fb_dchi__blk968_dn11 = assign31350_body8_e45725_d_n11;
            locals.var_fb_dchi__blk968_dn12 = assign31350_body8_e45725_d_n12;
            locals.var_fb_dchi__blk968_dn17 = assign31350_body8_e45725_d_n17;
            locals.var_fb_dchi__blk968_rv = 0.0;
            let (assign31350_body9_e45748, assign31350_body9_e45748_d_n0, assign31350_body9_e45748_d_n2, assign31350_body9_e45748_d_n6, assign31350_body9_e45748_d_n7, assign31350_body9_e45748_d_n10, assign31350_body9_e45748_d_n11, assign31350_body9_e45748_d_n12, assign31350_body9_e45748_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body9_e45741: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
        let assign31350_body9_e45743: f64 = (assign31350_body9_e45741 + locals.var_fs01__blk965);
        let assign31350_body9_e45745: f64 = (assign31350_body9_e45743 + 1e-50);
        let assign31350_body9_e45746: f64 = (assign31350_body9_e45745).sqrt();
        (assign31350_body9_e45746, ((((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)) + locals.var_fs01__blk965_dn0) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)) + locals.var_fs01__blk965_dn2) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)) + locals.var_fs01__blk965_dn6) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)) + locals.var_fs01__blk965_dn7) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)) + locals.var_fs01__blk965_dn10) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)) + locals.var_fs01__blk965_dn11) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)) + locals.var_fs01__blk965_dn12) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)) + locals.var_fs01__blk965_dn17) / (2.0 * assign31350_body9_e45746)),)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
            locals.var_fs02__blk969 = assign31350_body9_e45748;
            locals.var_fs02__blk969_dn0 = assign31350_body9_e45748_d_n0;
            locals.var_fs02__blk969_dn2 = assign31350_body9_e45748_d_n2;
            locals.var_fs02__blk969_dn6 = assign31350_body9_e45748_d_n6;
            locals.var_fs02__blk969_dn7 = assign31350_body9_e45748_d_n7;
            locals.var_fs02__blk969_dn10 = assign31350_body9_e45748_d_n10;
            locals.var_fs02__blk969_dn11 = assign31350_body9_e45748_d_n11;
            locals.var_fs02__blk969_dn12 = assign31350_body9_e45748_d_n12;
            locals.var_fs02__blk969_dn17 = assign31350_body9_e45748_d_n17;
            locals.var_fs02__blk969_rv = 0.0;
            let (assign31350_body10_e45776, assign31350_body10_e45776_d_n0, assign31350_body10_e45776_d_n2, assign31350_body10_e45776_d_n6, assign31350_body10_e45776_d_n7, assign31350_body10_e45776_d_n10, assign31350_body10_e45776_d_n11, assign31350_body10_e45776_d_n12, assign31350_body10_e45776_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign31350_body10_e45764: f64 = (locals.var_beta * locals.var_fb_dchi__blk968);
        let assign31350_body10_e45766: f64 = (assign31350_body10_e45764 * 2.0);
        let assign31350_body10_e45768: f64 = (assign31350_body10_e45766 * locals.var_fb__blk967);
        let assign31350_body10_e45770: f64 = (assign31350_body10_e45768 + locals.var_fs01_dps0__blk966);
        let assign31350_body10_e45773: f64 = (locals.var_fs02__blk969 + locals.var_fs02__blk969);
        let assign31350_body10_e45774: f64 = (assign31350_body10_e45770 / assign31350_body10_e45773);
        (assign31350_body10_e45774, ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn0) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn0)) + locals.var_fs01_dps0__blk966_dn0) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn0 + locals.var_fs02__blk969_dn0))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn2) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn2)) + locals.var_fs01_dps0__blk966_dn2) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn2 + locals.var_fs02__blk969_dn2))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn6) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn6)) + locals.var_fs01_dps0__blk966_dn6) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn6 + locals.var_fs02__blk969_dn6))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn7) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn7)) + locals.var_fs01_dps0__blk966_dn7) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn7 + locals.var_fs02__blk969_dn7))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk968) + (locals.var_beta * locals.var_fb_dchi__blk968_dn10)) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn10)) + locals.var_fs01_dps0__blk966_dn10) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn10 + locals.var_fs02__blk969_dn10))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn11) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn11)) + locals.var_fs01_dps0__blk966_dn11) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn11 + locals.var_fs02__blk969_dn11))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn12) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn12)) + locals.var_fs01_dps0__blk966_dn12) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn12 + locals.var_fs02__blk969_dn12))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn17) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn17)) + locals.var_fs01_dps0__blk966_dn17) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn17 + locals.var_fs02__blk969_dn17))) / (assign31350_body10_e45773 * assign31350_body10_e45773)),)
    } else {
        (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17,)
    }
};
            locals.var_fs02_dps0__blk970 = assign31350_body10_e45776;
            locals.var_fs02_dps0__blk970_dn0 = assign31350_body10_e45776_d_n0;
            locals.var_fs02_dps0__blk970_dn2 = assign31350_body10_e45776_d_n2;
            locals.var_fs02_dps0__blk970_dn6 = assign31350_body10_e45776_d_n6;
            locals.var_fs02_dps0__blk970_dn7 = assign31350_body10_e45776_d_n7;
            locals.var_fs02_dps0__blk970_dn10 = assign31350_body10_e45776_d_n10;
            locals.var_fs02_dps0__blk970_dn11 = assign31350_body10_e45776_d_n11;
            locals.var_fs02_dps0__blk970_dn12 = assign31350_body10_e45776_d_n12;
            locals.var_fs02_dps0__blk970_dn17 = assign31350_body10_e45776_d_n17;
            locals.var_fs02_dps0__blk970_rv = 0.0;
            let assign31350_body11_e45779: f64 = if locals.var_chi__blk943 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard1011 = assign31350_body11_e45779;
            locals.var_guard1011_rv = 0.0;
            let (assign31350_body12_e45799, assign31350_body12_e45799_d_n0, assign31350_body12_e45799_d_n2, assign31350_body12_e45799_d_n6, assign31350_body12_e45799_d_n7, assign31350_body12_e45799_d_n10, assign31350_body12_e45799_d_n11, assign31350_body12_e45799_d_n12, assign31350_body12_e45799_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31350_body12_e45797: f64 = (locals.var_chi__blk943).exp();
        (assign31350_body12_e45797, (assign31350_body12_e45797 * locals.var_chi__blk943_dn0), (assign31350_body12_e45797 * locals.var_chi__blk943_dn2), (assign31350_body12_e45797 * locals.var_chi__blk943_dn6), (assign31350_body12_e45797 * locals.var_chi__blk943_dn7), (assign31350_body12_e45797 * locals.var_chi__blk943_dn10), (assign31350_body12_e45797 * locals.var_chi__blk943_dn11), (assign31350_body12_e45797 * locals.var_chi__blk943_dn12), (assign31350_body12_e45797 * locals.var_chi__blk943_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign31350_body12_e45799;
            locals.var_exp_chi_dn0 = assign31350_body12_e45799_d_n0;
            locals.var_exp_chi_dn2 = assign31350_body12_e45799_d_n2;
            locals.var_exp_chi_dn6 = assign31350_body12_e45799_d_n6;
            locals.var_exp_chi_dn7 = assign31350_body12_e45799_d_n7;
            locals.var_exp_chi_dn10 = assign31350_body12_e45799_d_n10;
            locals.var_exp_chi_dn11 = assign31350_body12_e45799_d_n11;
            locals.var_exp_chi_dn12 = assign31350_body12_e45799_d_n12;
            locals.var_exp_chi_dn17 = assign31350_body12_e45799_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign31350_body13_e45822, assign31350_body13_e45822_d_n0, assign31350_body13_e45822_d_n2, assign31350_body13_e45822_d_n6, assign31350_body13_e45822_d_n7, assign31350_body13_e45822_d_n10, assign31350_body13_e45822_d_n11, assign31350_body13_e45822_d_n12, assign31350_body13_e45822_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31350_body13_e45819: f64 = (locals.var_exp_chi - 1.0);
        let assign31350_body13_e45820: f64 = (locals.var_cfs1__blk971 * assign31350_body13_e45819);
        (assign31350_body13_e45820, ((locals.var_cfs1__blk971_dn0 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk971_dn2 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk971_dn6 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk971_dn7 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk971_dn10 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk971_dn11 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk971_dn12 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk971_dn17 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign31350_body13_e45822;
            locals.var_fs01__blk965_dn0 = assign31350_body13_e45822_d_n0;
            locals.var_fs01__blk965_dn2 = assign31350_body13_e45822_d_n2;
            locals.var_fs01__blk965_dn6 = assign31350_body13_e45822_d_n6;
            locals.var_fs01__blk965_dn7 = assign31350_body13_e45822_d_n7;
            locals.var_fs01__blk965_dn10 = assign31350_body13_e45822_d_n10;
            locals.var_fs01__blk965_dn11 = assign31350_body13_e45822_d_n11;
            locals.var_fs01__blk965_dn12 = assign31350_body13_e45822_d_n12;
            locals.var_fs01__blk965_dn17 = assign31350_body13_e45822_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign31350_body14_e45845, assign31350_body14_e45845_d_n0, assign31350_body14_e45845_d_n2, assign31350_body14_e45845_d_n6, assign31350_body14_e45845_d_n7, assign31350_body14_e45845_d_n10, assign31350_body14_e45845_d_n11, assign31350_body14_e45845_d_n12, assign31350_body14_e45845_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31350_body14_e45841: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
        let assign31350_body14_e45843: f64 = (assign31350_body14_e45841 * locals.var_exp_chi);
        (assign31350_body14_e45843, (((locals.var_cfs1__blk971_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk971_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk971_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk971_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk971_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk971_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk971_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign31350_body14_e45845;
            locals.var_fs01_dps0__blk966_dn0 = assign31350_body14_e45845_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign31350_body14_e45845_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign31350_body14_e45845_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign31350_body14_e45845_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign31350_body14_e45845_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign31350_body14_e45845_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign31350_body14_e45845_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign31350_body14_e45845_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign31350_body15_e45868, assign31350_body15_e45868_d_n0, assign31350_body15_e45868_d_n2, assign31350_body15_e45868_d_n6, assign31350_body15_e45868_d_n7, assign31350_body15_e45868_d_n10, assign31350_body15_e45868_d_n11, assign31350_body15_e45868_d_n12, assign31350_body15_e45868_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
        let assign31350_body15_e45865: f64 = (locals.var_beta * locals.var_ps0ld__blk945);
        let assign31350_body15_e45866: f64 = (assign31350_body15_e45865).exp();
        (assign31350_body15_e45866, (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn0)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn2)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn6)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn7)), (assign31350_body15_e45866 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk945) + (locals.var_beta * locals.var_ps0ld__blk945_dn10))), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn11)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn12)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn17)),)
    } else {
        (locals.var_exp_bps0__blk972, locals.var_exp_bps0__blk972_dn0, locals.var_exp_bps0__blk972_dn2, locals.var_exp_bps0__blk972_dn6, locals.var_exp_bps0__blk972_dn7, locals.var_exp_bps0__blk972_dn10, locals.var_exp_bps0__blk972_dn11, locals.var_exp_bps0__blk972_dn12, locals.var_exp_bps0__blk972_dn17,)
    }
};
            locals.var_exp_bps0__blk972 = assign31350_body15_e45868;
            locals.var_exp_bps0__blk972_dn0 = assign31350_body15_e45868_d_n0;
            locals.var_exp_bps0__blk972_dn2 = assign31350_body15_e45868_d_n2;
            locals.var_exp_bps0__blk972_dn6 = assign31350_body15_e45868_d_n6;
            locals.var_exp_bps0__blk972_dn7 = assign31350_body15_e45868_d_n7;
            locals.var_exp_bps0__blk972_dn10 = assign31350_body15_e45868_d_n10;
            locals.var_exp_bps0__blk972_dn11 = assign31350_body15_e45868_d_n11;
            locals.var_exp_bps0__blk972_dn12 = assign31350_body15_e45868_d_n12;
            locals.var_exp_bps0__blk972_dn17 = assign31350_body15_e45868_d_n17;
            locals.var_exp_bps0__blk972_rv = 0.0;
            let (assign31350_body16_e45892, assign31350_body16_e45892_d_n0, assign31350_body16_e45892_d_n2, assign31350_body16_e45892_d_n6, assign31350_body16_e45892_d_n7, assign31350_body16_e45892_d_n10, assign31350_body16_e45892_d_n11, assign31350_body16_e45892_d_n12, assign31350_body16_e45892_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
        let assign31350_body16_e45889: f64 = (locals.var_exp_bps0__blk972 - locals.var_exp_bvbs__blk962);
        let assign31350_body16_e45890: f64 = (locals.var_cnst1over__blk956 * assign31350_body16_e45889);
        (assign31350_body16_e45890, ((locals.var_cnst1over__blk956_dn0 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn0 - locals.var_exp_bvbs__blk962_dn0))), ((locals.var_cnst1over__blk956_dn2 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn2 - locals.var_exp_bvbs__blk962_dn2))), ((locals.var_cnst1over__blk956_dn6 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn6 - locals.var_exp_bvbs__blk962_dn6))), ((locals.var_cnst1over__blk956_dn7 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn7 - locals.var_exp_bvbs__blk962_dn7))), ((locals.var_cnst1over__blk956_dn10 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn10 - locals.var_exp_bvbs__blk962_dn10))), ((locals.var_cnst1over__blk956_dn11 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn11 - locals.var_exp_bvbs__blk962_dn11))), ((locals.var_cnst1over__blk956_dn12 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn12 - locals.var_exp_bvbs__blk962_dn12))), ((locals.var_cnst1over__blk956_dn17 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn17 - locals.var_exp_bvbs__blk962_dn17))),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign31350_body16_e45892;
            locals.var_fs01__blk965_dn0 = assign31350_body16_e45892_d_n0;
            locals.var_fs01__blk965_dn2 = assign31350_body16_e45892_d_n2;
            locals.var_fs01__blk965_dn6 = assign31350_body16_e45892_d_n6;
            locals.var_fs01__blk965_dn7 = assign31350_body16_e45892_d_n7;
            locals.var_fs01__blk965_dn10 = assign31350_body16_e45892_d_n10;
            locals.var_fs01__blk965_dn11 = assign31350_body16_e45892_d_n11;
            locals.var_fs01__blk965_dn12 = assign31350_body16_e45892_d_n12;
            locals.var_fs01__blk965_dn17 = assign31350_body16_e45892_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign31350_body17_e45916, assign31350_body17_e45916_d_n0, assign31350_body17_e45916_d_n2, assign31350_body17_e45916_d_n6, assign31350_body17_e45916_d_n7, assign31350_body17_e45916_d_n10, assign31350_body17_e45916_d_n11, assign31350_body17_e45916_d_n12, assign31350_body17_e45916_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
        let assign31350_body17_e45912: f64 = (locals.var_cnst1over__blk956 * locals.var_beta);
        let assign31350_body17_e45914: f64 = (assign31350_body17_e45912 * locals.var_exp_bps0__blk972);
        (assign31350_body17_e45914, (((locals.var_cnst1over__blk956_dn0 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn0)), (((locals.var_cnst1over__blk956_dn2 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn2)), (((locals.var_cnst1over__blk956_dn6 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn6)), (((locals.var_cnst1over__blk956_dn7 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn7)), ((((locals.var_cnst1over__blk956_dn10 * locals.var_beta) + (locals.var_cnst1over__blk956 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn10)), (((locals.var_cnst1over__blk956_dn11 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn11)), (((locals.var_cnst1over__blk956_dn12 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn12)), (((locals.var_cnst1over__blk956_dn17 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign31350_body17_e45916;
            locals.var_fs01_dps0__blk966_dn0 = assign31350_body17_e45916_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign31350_body17_e45916_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign31350_body17_e45916_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign31350_body17_e45916_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign31350_body17_e45916_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign31350_body17_e45916_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign31350_body17_e45916_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign31350_body17_e45916_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign31350_body18_e45938, assign31350_body18_e45938_d_n0, assign31350_body18_e45938_d_n2, assign31350_body18_e45938_d_n6, assign31350_body18_e45938_d_n7, assign31350_body18_e45938_d_n10, assign31350_body18_e45938_d_n11, assign31350_body18_e45938_d_n12, assign31350_body18_e45938_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) {
        let assign31350_body18_e45933: f64 = (locals.var_chi__blk943 - 1.0);
        let assign31350_body18_e45935: f64 = (assign31350_body18_e45933 + locals.var_fs01__blk965);
        let assign31350_body18_e45936: f64 = (assign31350_body18_e45935).sqrt();
        (assign31350_body18_e45936, ((locals.var_chi__blk943_dn0 + locals.var_fs01__blk965_dn0) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn2 + locals.var_fs01__blk965_dn2) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn6 + locals.var_fs01__blk965_dn6) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn7 + locals.var_fs01__blk965_dn7) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn10 + locals.var_fs01__blk965_dn10) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn11 + locals.var_fs01__blk965_dn11) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn12 + locals.var_fs01__blk965_dn12) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn17 + locals.var_fs01__blk965_dn17) / (2.0 * assign31350_body18_e45936)),)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
            locals.var_fs02__blk969 = assign31350_body18_e45938;
            locals.var_fs02__blk969_dn0 = assign31350_body18_e45938_d_n0;
            locals.var_fs02__blk969_dn2 = assign31350_body18_e45938_d_n2;
            locals.var_fs02__blk969_dn6 = assign31350_body18_e45938_d_n6;
            locals.var_fs02__blk969_dn7 = assign31350_body18_e45938_d_n7;
            locals.var_fs02__blk969_dn10 = assign31350_body18_e45938_d_n10;
            locals.var_fs02__blk969_dn11 = assign31350_body18_e45938_d_n11;
            locals.var_fs02__blk969_dn12 = assign31350_body18_e45938_d_n12;
            locals.var_fs02__blk969_dn17 = assign31350_body18_e45938_d_n17;
            locals.var_fs02__blk969_rv = 0.0;
            let (assign31350_body19_e45961, assign31350_body19_e45961_d_n0, assign31350_body19_e45961_d_n2, assign31350_body19_e45961_d_n6, assign31350_body19_e45961_d_n7, assign31350_body19_e45961_d_n10, assign31350_body19_e45961_d_n11, assign31350_body19_e45961_d_n12, assign31350_body19_e45961_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) {
        let assign31350_body19_e45955: f64 = (locals.var_beta + locals.var_fs01_dps0__blk966);
        let assign31350_body19_e45957: f64 = (assign31350_body19_e45955 / locals.var_fs02__blk969);
        let assign31350_body19_e45959: f64 = (assign31350_body19_e45957 * 0.5);
        (assign31350_body19_e45959, ((((locals.var_fs01_dps0__blk966_dn0 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn0)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn2 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn2)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn6 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn6)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn7 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn7)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk966_dn10) * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn10)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn11 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn11)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn12 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn12)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn17 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn17)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17,)
    }
};
            locals.var_fs02_dps0__blk970 = assign31350_body19_e45961;
            locals.var_fs02_dps0__blk970_dn0 = assign31350_body19_e45961_d_n0;
            locals.var_fs02_dps0__blk970_dn2 = assign31350_body19_e45961_d_n2;
            locals.var_fs02_dps0__blk970_dn6 = assign31350_body19_e45961_d_n6;
            locals.var_fs02_dps0__blk970_dn7 = assign31350_body19_e45961_d_n7;
            locals.var_fs02_dps0__blk970_dn10 = assign31350_body19_e45961_d_n10;
            locals.var_fs02_dps0__blk970_dn11 = assign31350_body19_e45961_d_n11;
            locals.var_fs02_dps0__blk970_dn12 = assign31350_body19_e45961_d_n12;
            locals.var_fs02_dps0__blk970_dn17 = assign31350_body19_e45961_d_n17;
            locals.var_fs02_dps0__blk970_rv = 0.0;
            let (assign31350_body20_e45981, assign31350_body20_e45981_d_n0, assign31350_body20_e45981_d_n2, assign31350_body20_e45981_d_n6, assign31350_body20_e45981_d_n7, assign31350_body20_e45981_d_n10, assign31350_body20_e45981_d_n11, assign31350_body20_e45981_d_n12, assign31350_body20_e45981_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31350_body20_e45975: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        let assign31350_body20_e45978: f64 = (locals.var_fac1__blk929 * locals.var_fs02__blk969);
        let assign31350_body20_e45979: f64 = (assign31350_body20_e45975 - assign31350_body20_e45978);
        (assign31350_body20_e45979, ((locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0) - ((locals.var_fac1__blk929_dn0 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn0))), ((locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2) - ((locals.var_fac1__blk929_dn2 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn2))), ((locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6) - ((locals.var_fac1__blk929_dn6 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn6))), ((locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7) - ((locals.var_fac1__blk929_dn7 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn7))), ((locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10) - ((locals.var_fac1__blk929_dn10 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn10))), ((locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11) - ((locals.var_fac1__blk929_dn11 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn11))), ((locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12) - ((locals.var_fac1__blk929_dn12 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn12))), ((locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17) - ((locals.var_fac1__blk929_dn17 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn17))),)
    } else {
        (locals.var_fs0__blk973, locals.var_fs0__blk973_dn0, locals.var_fs0__blk973_dn2, locals.var_fs0__blk973_dn6, locals.var_fs0__blk973_dn7, locals.var_fs0__blk973_dn10, locals.var_fs0__blk973_dn11, locals.var_fs0__blk973_dn12, locals.var_fs0__blk973_dn17,)
    }
};
            locals.var_fs0__blk973 = assign31350_body20_e45981;
            locals.var_fs0__blk973_dn0 = assign31350_body20_e45981_d_n0;
            locals.var_fs0__blk973_dn2 = assign31350_body20_e45981_d_n2;
            locals.var_fs0__blk973_dn6 = assign31350_body20_e45981_d_n6;
            locals.var_fs0__blk973_dn7 = assign31350_body20_e45981_d_n7;
            locals.var_fs0__blk973_dn10 = assign31350_body20_e45981_d_n10;
            locals.var_fs0__blk973_dn11 = assign31350_body20_e45981_d_n11;
            locals.var_fs0__blk973_dn12 = assign31350_body20_e45981_d_n12;
            locals.var_fs0__blk973_dn17 = assign31350_body20_e45981_d_n17;
            locals.var_fs0__blk973_rv = 0.0;
            let (assign31350_body21_e46000, assign31350_body21_e46000_d_n0, assign31350_body21_e46000_d_n2, assign31350_body21_e46000_d_n6, assign31350_body21_e46000_d_n7, assign31350_body21_e46000_d_n10, assign31350_body21_e46000_d_n11, assign31350_body21_e46000_d_n12, assign31350_body21_e46000_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31350_body21_e45994: f64 = (-1.0);
        let assign31350_body21_e45997: f64 = (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970);
        let assign31350_body21_e45998: f64 = (assign31350_body21_e45994 - assign31350_body21_e45997);
        (assign31350_body21_e45998, (-((locals.var_fac1__blk929_dn0 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn0))), (-((locals.var_fac1__blk929_dn2 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn2))), (-((locals.var_fac1__blk929_dn6 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn6))), (-((locals.var_fac1__blk929_dn7 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn7))), (-((locals.var_fac1__blk929_dn10 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn10))), (-((locals.var_fac1__blk929_dn11 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn11))), (-((locals.var_fac1__blk929_dn12 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn12))), (-((locals.var_fac1__blk929_dn17 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk974, locals.var_fs0_dps0__blk974_dn0, locals.var_fs0_dps0__blk974_dn2, locals.var_fs0_dps0__blk974_dn6, locals.var_fs0_dps0__blk974_dn7, locals.var_fs0_dps0__blk974_dn10, locals.var_fs0_dps0__blk974_dn11, locals.var_fs0_dps0__blk974_dn12, locals.var_fs0_dps0__blk974_dn17,)
    }
};
            locals.var_fs0_dps0__blk974 = assign31350_body21_e46000;
            locals.var_fs0_dps0__blk974_dn0 = assign31350_body21_e46000_d_n0;
            locals.var_fs0_dps0__blk974_dn2 = assign31350_body21_e46000_d_n2;
            locals.var_fs0_dps0__blk974_dn6 = assign31350_body21_e46000_d_n6;
            locals.var_fs0_dps0__blk974_dn7 = assign31350_body21_e46000_d_n7;
            locals.var_fs0_dps0__blk974_dn10 = assign31350_body21_e46000_d_n10;
            locals.var_fs0_dps0__blk974_dn11 = assign31350_body21_e46000_d_n11;
            locals.var_fs0_dps0__blk974_dn12 = assign31350_body21_e46000_d_n12;
            locals.var_fs0_dps0__blk974_dn17 = assign31350_body21_e46000_d_n17;
            locals.var_fs0_dps0__blk974_rv = 0.0;
            let assign31350_body22_e46003: f64 = if locals.var_flg_conv__blk918 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1012 = assign31350_body22_e46003;
            locals.var_guard1012_rv = 0.0;
            let (assign31350_body23_e46023,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 != 0.0)) {
        let assign31350_body23_e46019: f64 = (2.0 * 20.0);
        let assign31350_body23_e46021: f64 = (assign31350_body23_e46019 + 1.0);
        (assign31350_body23_e46021,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31350_body23_e46023;
            locals.var_lp_s0_rv = 0.0;
            let (assign31350_body24_e46043, assign31350_body24_e46043_d_n0, assign31350_body24_e46043_d_n2, assign31350_body24_e46043_d_n6, assign31350_body24_e46043_d_n7, assign31350_body24_e46043_d_n10, assign31350_body24_e46043_d_n11, assign31350_body24_e46043_d_n12, assign31350_body24_e46043_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
        let assign31350_body24_e46039: f64 = (-locals.var_fs0__blk973);
        let assign31350_body24_e46041: f64 = (assign31350_body24_e46039 / locals.var_fs0_dps0__blk974);
        (assign31350_body24_e46041, ((((-locals.var_fs0__blk973_dn0) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn0)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn2) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn2)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn6) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn6)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn7) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn7)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn10) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn10)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn11) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn11)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn12) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn12)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn17) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn17)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign31350_body24_e46043;
            locals.var_dps0_dn0 = assign31350_body24_e46043_d_n0;
            locals.var_dps0_dn2 = assign31350_body24_e46043_d_n2;
            locals.var_dps0_dn6 = assign31350_body24_e46043_d_n6;
            locals.var_dps0_dn7 = assign31350_body24_e46043_d_n7;
            locals.var_dps0_dn10 = assign31350_body24_e46043_d_n10;
            locals.var_dps0_dn11 = assign31350_body24_e46043_d_n11;
            locals.var_dps0_dn12 = assign31350_body24_e46043_d_n12;
            locals.var_dps0_dn17 = assign31350_body24_e46043_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign31350_body25_e46073, assign31350_body25_e46073_d_n0, assign31350_body25_e46073_d_n2, assign31350_body25_e46073_d_n6, assign31350_body25_e46073_d_n7, assign31350_body25_e46073_d_n10, assign31350_body25_e46073_d_n11, assign31350_body25_e46073_d_n12, assign31350_body25_e46073_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
        let assign31350_body25_e46060: f64 = (0.5 * 0.1);
        let assign31350_body25_e46064: f64 = (locals.var_ps0ld__blk945).abs();
        let (assign31350_body25_e46069, assign31350_body25_e46069_d_n0, assign31350_body25_e46069_d_n2, assign31350_body25_e46069_d_n6, assign31350_body25_e46069_d_n7, assign31350_body25_e46069_d_n10, assign31350_body25_e46069_d_n11, assign31350_body25_e46069_d_n12, assign31350_body25_e46069_d_n17,) = {
            if (1.0 >= assign31350_body25_e46064) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31350_body25_e46068: f64 = (locals.var_ps0ld__blk945).abs();
                (assign31350_body25_e46068, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn0 } else { (-locals.var_ps0ld__blk945_dn0) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn2 } else { (-locals.var_ps0ld__blk945_dn2) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn6 } else { (-locals.var_ps0ld__blk945_dn6) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn7 } else { (-locals.var_ps0ld__blk945_dn7) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn10 } else { (-locals.var_ps0ld__blk945_dn10) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn11 } else { (-locals.var_ps0ld__blk945_dn11) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn12 } else { (-locals.var_ps0ld__blk945_dn12) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn17 } else { (-locals.var_ps0ld__blk945_dn17) },)
            }
        };
        let assign31350_body25_e46070: f64 = (1.0 + assign31350_body25_e46069);
        let assign31350_body25_e46071: f64 = (assign31350_body25_e46060 * assign31350_body25_e46070);
        (assign31350_body25_e46071, (assign31350_body25_e46060 * assign31350_body25_e46069_d_n0), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n2), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n6), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n7), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n10), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n11), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n12), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n17),)
    } else {
        (locals.var_dplim__blk975, locals.var_dplim__blk975_dn0, locals.var_dplim__blk975_dn2, locals.var_dplim__blk975_dn6, locals.var_dplim__blk975_dn7, locals.var_dplim__blk975_dn10, locals.var_dplim__blk975_dn11, locals.var_dplim__blk975_dn12, locals.var_dplim__blk975_dn17,)
    }
};
            locals.var_dplim__blk975 = assign31350_body25_e46073;
            locals.var_dplim__blk975_dn0 = assign31350_body25_e46073_d_n0;
            locals.var_dplim__blk975_dn2 = assign31350_body25_e46073_d_n2;
            locals.var_dplim__blk975_dn6 = assign31350_body25_e46073_d_n6;
            locals.var_dplim__blk975_dn7 = assign31350_body25_e46073_d_n7;
            locals.var_dplim__blk975_dn10 = assign31350_body25_e46073_d_n10;
            locals.var_dplim__blk975_dn11 = assign31350_body25_e46073_d_n11;
            locals.var_dplim__blk975_dn12 = assign31350_body25_e46073_d_n12;
            locals.var_dplim__blk975_dn17 = assign31350_body25_e46073_d_n17;
            locals.var_dplim__blk975_rv = 0.0;
            let assign31350_body26_e46075: f64 = (locals.var_dps0).abs();
            let assign31350_body26_e46077: f64 = if assign31350_body26_e46075 > locals.var_dplim__blk975 { 1.0 } else { 0.0 };
            locals.var_guard1013 = assign31350_body26_e46077;
            locals.var_guard1013_rv = 0.0;
            let (assign31350_body27_e46104, assign31350_body27_e46104_d_n0, assign31350_body27_e46104_d_n2, assign31350_body27_e46104_d_n6, assign31350_body27_e46104_d_n7, assign31350_body27_e46104_d_n10, assign31350_body27_e46104_d_n11, assign31350_body27_e46104_d_n12, assign31350_body27_e46104_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let (assign31350_body27_e46101,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign31350_body27_e46100: f64 = (-1.0);
                (assign31350_body27_e46100,)
            }
        };
        let assign31350_body27_e46102: f64 = (locals.var_dplim__blk975 * assign31350_body27_e46101);
        (assign31350_body27_e46102, (locals.var_dplim__blk975_dn0 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn2 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn6 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn7 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn10 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn11 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn12 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn17 * assign31350_body27_e46101),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign31350_body27_e46104;
            locals.var_dps0_dn0 = assign31350_body27_e46104_d_n0;
            locals.var_dps0_dn2 = assign31350_body27_e46104_d_n2;
            locals.var_dps0_dn6 = assign31350_body27_e46104_d_n6;
            locals.var_dps0_dn7 = assign31350_body27_e46104_d_n7;
            locals.var_dps0_dn10 = assign31350_body27_e46104_d_n10;
            locals.var_dps0_dn11 = assign31350_body27_e46104_d_n11;
            locals.var_dps0_dn12 = assign31350_body27_e46104_d_n12;
            locals.var_dps0_dn17 = assign31350_body27_e46104_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign31350_body28_e46123, assign31350_body28_e46123_d_n0, assign31350_body28_e46123_d_n2, assign31350_body28_e46123_d_n6, assign31350_body28_e46123_d_n7, assign31350_body28_e46123_d_n10, assign31350_body28_e46123_d_n11, assign31350_body28_e46123_d_n12, assign31350_body28_e46123_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
        let assign31350_body28_e46121: f64 = (locals.var_ps0ld__blk945 + locals.var_dps0);
        (assign31350_body28_e46121, (locals.var_ps0ld__blk945_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk945_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk945_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk945_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk945_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk945_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk945_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk945_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
            locals.var_ps0ld__blk945 = assign31350_body28_e46123;
            locals.var_ps0ld__blk945_dn0 = assign31350_body28_e46123_d_n0;
            locals.var_ps0ld__blk945_dn2 = assign31350_body28_e46123_d_n2;
            locals.var_ps0ld__blk945_dn6 = assign31350_body28_e46123_d_n6;
            locals.var_ps0ld__blk945_dn7 = assign31350_body28_e46123_d_n7;
            locals.var_ps0ld__blk945_dn10 = assign31350_body28_e46123_d_n10;
            locals.var_ps0ld__blk945_dn11 = assign31350_body28_e46123_d_n11;
            locals.var_ps0ld__blk945_dn12 = assign31350_body28_e46123_d_n12;
            locals.var_ps0ld__blk945_dn17 = assign31350_body28_e46123_d_n17;
            locals.var_ps0ld__blk945_rv = 0.0;
            let assign31350_body29_e46125: f64 = (locals.var_dps0).abs();
            let assign31350_body29_e46129: f64 = (locals.var_fs0__blk973).abs();
            let assign31350_body29_e46132: f64 = if ((assign31350_body29_e46125 <= 5e-12) && (assign31350_body29_e46129 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1014 = assign31350_body29_e46132;
            locals.var_guard1014_rv = 0.0;
            let (assign31350_body30_e46151,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1014 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk918,)
    }
};
            locals.var_flg_conv__blk918 = assign31350_body30_e46151;
            locals.var_flg_conv__blk918_rv = 0.0;
            let (assign31350_body31_e46167,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31350_body31_e46165: f64 = (locals.var_lp_s0 + 1.0);
        (assign31350_body31_e46165,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31350_body31_e46167;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31370_e46173: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign31370_e46173;
        locals.var_guard1016_rv = 0.0;

        let (assign31410_e46235, assign31410_e46235_d_n0, assign31410_e46235_d_n2, assign31410_e46235_d_n6, assign31410_e46235_d_n7, assign31410_e46235_d_n10, assign31410_e46235_d_n11, assign31410_e46235_d_n12, assign31410_e46235_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign31410_e46229: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
        let assign31410_e46232: f64 = (10.0 * 2.220446049250313e-16);
        let assign31410_e46233: f64 = (assign31410_e46229 + assign31410_e46232);
        (assign31410_e46233, ((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)), ((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)), ((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)), ((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)), ((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)), ((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)), ((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)), ((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)),)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign31410_e46235;
        locals.var_xi0__blk976_dn0 = assign31410_e46235_d_n0;
        locals.var_xi0__blk976_dn2 = assign31410_e46235_d_n2;
        locals.var_xi0__blk976_dn6 = assign31410_e46235_d_n6;
        locals.var_xi0__blk976_dn7 = assign31410_e46235_d_n7;
        locals.var_xi0__blk976_dn10 = assign31410_e46235_d_n10;
        locals.var_xi0__blk976_dn11 = assign31410_e46235_d_n11;
        locals.var_xi0__blk976_dn12 = assign31410_e46235_d_n12;
        locals.var_xi0__blk976_dn17 = assign31410_e46235_d_n17;
        locals.var_xi0__blk976_rv = 0.0;

        let (assign31420_e46255, assign31420_e46255_d_n0, assign31420_e46255_d_n2, assign31420_e46255_d_n6, assign31420_e46255_d_n7, assign31420_e46255_d_n10, assign31420_e46255_d_n11, assign31420_e46255_d_n12, assign31420_e46255_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign31420_e46252: f64 = (10.0 * 2.220446049250313e-16);
        let assign31420_e46253: f64 = (locals.var_fb__blk967 + assign31420_e46252);
        (assign31420_e46253, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign31420_e46255;
        locals.var_xi0p12__blk977_dn0 = assign31420_e46255_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign31420_e46255_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign31420_e46255_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign31420_e46255_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign31420_e46255_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign31420_e46255_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign31420_e46255_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign31420_e46255_d_n17;
        locals.var_xi0p12__blk977_rv = 0.0;

        let (assign31440_e46291, assign31440_e46291_d_n0, assign31440_e46291_d_n2, assign31440_e46291_d_n6, assign31440_e46291_d_n7, assign31440_e46291_d_n10, assign31440_e46291_d_n11, assign31440_e46291_d_n12, assign31440_e46291_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31440_e46289: f64 = (locals.var_chi__blk943 - 1.0);
        (assign31440_e46289, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign31440_e46291;
        locals.var_xi0__blk976_dn0 = assign31440_e46291_d_n0;
        locals.var_xi0__blk976_dn2 = assign31440_e46291_d_n2;
        locals.var_xi0__blk976_dn6 = assign31440_e46291_d_n6;
        locals.var_xi0__blk976_dn7 = assign31440_e46291_d_n7;
        locals.var_xi0__blk976_dn10 = assign31440_e46291_d_n10;
        locals.var_xi0__blk976_dn11 = assign31440_e46291_d_n11;
        locals.var_xi0__blk976_dn12 = assign31440_e46291_d_n12;
        locals.var_xi0__blk976_dn17 = assign31440_e46291_d_n17;
        locals.var_xi0__blk976_rv = 0.0;

        let (assign31450_e46309, assign31450_e46309_d_n0, assign31450_e46309_d_n2, assign31450_e46309_d_n6, assign31450_e46309_d_n7, assign31450_e46309_d_n10, assign31450_e46309_d_n11, assign31450_e46309_d_n12, assign31450_e46309_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31450_e46307: f64 = (locals.var_xi0__blk976).sqrt();
        (assign31450_e46307, (locals.var_xi0__blk976_dn0 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn2 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn6 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn7 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn10 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn11 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn12 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn17 / (2.0 * assign31450_e46307)),)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign31450_e46309;
        locals.var_xi0p12__blk977_dn0 = assign31450_e46309_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign31450_e46309_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign31450_e46309_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign31450_e46309_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign31450_e46309_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign31450_e46309_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign31450_e46309_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign31450_e46309_d_n17;
        locals.var_xi0p12__blk977_rv = 0.0;

        let (assign31460_e46325, assign31460_e46325_d_n0, assign31460_e46325_d_n2, assign31460_e46325_d_n6, assign31460_e46325_d_n7, assign31460_e46325_d_n10, assign31460_e46325_d_n11, assign31460_e46325_d_n12, assign31460_e46325_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31460_e46323: f64 = (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977);
        (assign31460_e46323, ((locals.var_cnst0over__blk928_dn0 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31460_e46325;
        locals.var_qbuld_dn0 = assign31460_e46325_d_n0;
        locals.var_qbuld_dn2 = assign31460_e46325_d_n2;
        locals.var_qbuld_dn6 = assign31460_e46325_d_n6;
        locals.var_qbuld_dn7 = assign31460_e46325_d_n7;
        locals.var_qbuld_dn10 = assign31460_e46325_d_n10;
        locals.var_qbuld_dn11 = assign31460_e46325_d_n11;
        locals.var_qbuld_dn12 = assign31460_e46325_d_n12;
        locals.var_qbuld_dn17 = assign31460_e46325_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign31470_e46343, assign31470_e46343_d_n0, assign31470_e46343_d_n2, assign31470_e46343_d_n6, assign31470_e46343_d_n7, assign31470_e46343_d_n10, assign31470_e46343_d_n11, assign31470_e46343_d_n12, assign31470_e46343_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31470_e46340: f64 = (locals.var_fs02__blk969 + locals.var_xi0p12__blk977);
        let assign31470_e46341: f64 = (1.0 / assign31470_e46340);
        (assign31470_e46341, (-((locals.var_fs02__blk969_dn0 + locals.var_xi0p12__blk977_dn0) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn2 + locals.var_xi0p12__blk977_dn2) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn6 + locals.var_xi0p12__blk977_dn6) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn7 + locals.var_xi0p12__blk977_dn7) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn10 + locals.var_xi0p12__blk977_dn10) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn11 + locals.var_xi0p12__blk977_dn11) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn12 + locals.var_xi0p12__blk977_dn12) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn17 + locals.var_xi0p12__blk977_dn17) / (assign31470_e46340 * assign31470_e46340))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31470_e46343;
        locals.var_t1__blk896_dn0 = assign31470_e46343_d_n0;
        locals.var_t1__blk896_dn2 = assign31470_e46343_d_n2;
        locals.var_t1__blk896_dn6 = assign31470_e46343_d_n6;
        locals.var_t1__blk896_dn7 = assign31470_e46343_d_n7;
        locals.var_t1__blk896_dn10 = assign31470_e46343_d_n10;
        locals.var_t1__blk896_dn11 = assign31470_e46343_d_n11;
        locals.var_t1__blk896_dn12 = assign31470_e46343_d_n12;
        locals.var_t1__blk896_dn17 = assign31470_e46343_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31480_e46361, assign31480_e46361_d_n0, assign31480_e46361_d_n2, assign31480_e46361_d_n6, assign31480_e46361_d_n7, assign31480_e46361_d_n10, assign31480_e46361_d_n11, assign31480_e46361_d_n12, assign31480_e46361_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31480_e46357: f64 = (locals.var_cnst0over__blk928 * locals.var_fs01__blk965);
        let assign31480_e46359: f64 = (assign31480_e46357 * locals.var_t1__blk896);
        (assign31480_e46359, ((((locals.var_cnst0over__blk928_dn0 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn0)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn0)), ((((locals.var_cnst0over__blk928_dn2 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn2)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn2)), ((((locals.var_cnst0over__blk928_dn6 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn6)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn6)), ((((locals.var_cnst0over__blk928_dn7 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn7)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn7)), ((((locals.var_cnst0over__blk928_dn10 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn10)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn10)), ((((locals.var_cnst0over__blk928_dn11 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn11)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn11)), ((((locals.var_cnst0over__blk928_dn12 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn12)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn12)), ((((locals.var_cnst0over__blk928_dn17 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn17)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31480_e46361;
        locals.var_qiuld_dn0 = assign31480_e46361_d_n0;
        locals.var_qiuld_dn2 = assign31480_e46361_d_n2;
        locals.var_qiuld_dn6 = assign31480_e46361_d_n6;
        locals.var_qiuld_dn7 = assign31480_e46361_d_n7;
        locals.var_qiuld_dn10 = assign31480_e46361_d_n10;
        locals.var_qiuld_dn11 = assign31480_e46361_d_n11;
        locals.var_qiuld_dn12 = assign31480_e46361_d_n12;
        locals.var_qiuld_dn17 = assign31480_e46361_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign31490_e46377, assign31490_e46377_d_n0, assign31490_e46377_d_n2, assign31490_e46377_d_n6, assign31490_e46377_d_n7, assign31490_e46377_d_n10, assign31490_e46377_d_n11, assign31490_e46377_d_n12, assign31490_e46377_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign31490_e46375: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign31490_e46375, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31490_e46377;
        locals.var_qsuld_dn0 = assign31490_e46377_d_n0;
        locals.var_qsuld_dn2 = assign31490_e46377_d_n2;
        locals.var_qsuld_dn6 = assign31490_e46377_d_n6;
        locals.var_qsuld_dn7 = assign31490_e46377_d_n7;
        locals.var_qsuld_dn10 = assign31490_e46377_d_n10;
        locals.var_qsuld_dn11 = assign31490_e46377_d_n11;
        locals.var_qsuld_dn12 = assign31490_e46377_d_n12;
        locals.var_qsuld_dn17 = assign31490_e46377_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign31500_e46388, assign31500_e46388_d_n0, assign31500_e46388_d_n2, assign31500_e46388_d_n6, assign31500_e46388_d_n7, assign31500_e46388_d_n10, assign31500_e46388_d_n11, assign31500_e46388_d_n12, assign31500_e46388_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign31500_e46386: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign31500_e46386, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31500_e46388;
        locals.var_qiuld_dn0 = assign31500_e46388_d_n0;
        locals.var_qiuld_dn2 = assign31500_e46388_d_n2;
        locals.var_qiuld_dn6 = assign31500_e46388_d_n6;
        locals.var_qiuld_dn7 = assign31500_e46388_d_n7;
        locals.var_qiuld_dn10 = assign31500_e46388_d_n10;
        locals.var_qiuld_dn11 = assign31500_e46388_d_n11;
        locals.var_qiuld_dn12 = assign31500_e46388_d_n12;
        locals.var_qiuld_dn17 = assign31500_e46388_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign31510_e46406, assign31510_e46406_d_n0, assign31510_e46406_d_n2, assign31510_e46406_d_n6, assign31510_e46406_d_n7, assign31510_e46406_d_n10, assign31510_e46406_d_n11, assign31510_e46406_d_n12, assign31510_e46406_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let (assign31510_e46404,) = {
            if (p.p43 == 1.0) {
                let assign31510_e46400: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign31510_e46400,)
            } else {
                let assign31510_e46403: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign31510_e46403,)
            }
        };
        (assign31510_e46404, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17,)
    }
};
        locals.var_t4__blk899 = assign31510_e46406;
        locals.var_t4__blk899_dn0 = assign31510_e46406_d_n0;
        locals.var_t4__blk899_dn2 = assign31510_e46406_d_n2;
        locals.var_t4__blk899_dn6 = assign31510_e46406_d_n6;
        locals.var_t4__blk899_dn7 = assign31510_e46406_d_n7;
        locals.var_t4__blk899_dn10 = assign31510_e46406_d_n10;
        locals.var_t4__blk899_dn11 = assign31510_e46406_d_n11;
        locals.var_t4__blk899_dn12 = assign31510_e46406_d_n12;
        locals.var_t4__blk899_dn17 = assign31510_e46406_d_n17;
        locals.var_t4__blk899_rv = 0.0;

        let assign31520_e46417: f64 = if (((locals.var_flg_overs__blk914 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk912 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign31520_e46417;
        locals.var_guard1018_rv = 0.0;

        let (assign31530_e46430, assign31530_e46430_d_n0, assign31530_e46430_d_n2, assign31530_e46430_d_n6, assign31530_e46430_d_n7, assign31530_e46430_d_n10, assign31530_e46430_d_n11, assign31530_e46430_d_n12, assign31530_e46430_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31530_e46428: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign31530_e46428, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign31530_e46430;
        locals.var_qovs_dn0 = assign31530_e46430_d_n0;
        locals.var_qovs_dn2 = assign31530_e46430_d_n2;
        locals.var_qovs_dn6 = assign31530_e46430_d_n6;
        locals.var_qovs_dn7 = assign31530_e46430_d_n7;
        locals.var_qovs_dn10 = assign31530_e46430_d_n10;
        locals.var_qovs_dn11 = assign31530_e46430_d_n11;
        locals.var_qovs_dn12 = assign31530_e46430_d_n12;
        locals.var_qovs_dn17 = assign31530_e46430_d_n17;
        locals.var_qovs_rv = 0.0;

        let (assign31540_e46443, assign31540_e46443_d_n0, assign31540_e46443_d_n2, assign31540_e46443_d_n6, assign31540_e46443_d_n7, assign31540_e46443_d_n10, assign31540_e46443_d_n11, assign31540_e46443_d_n12, assign31540_e46443_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign31540_e46441: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign31540_e46441, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign31540_e46443;
        locals.var_qbsld_dn0 = assign31540_e46443_d_n0;
        locals.var_qbsld_dn2 = assign31540_e46443_d_n2;
        locals.var_qbsld_dn6 = assign31540_e46443_d_n6;
        locals.var_qbsld_dn7 = assign31540_e46443_d_n7;
        locals.var_qbsld_dn10 = assign31540_e46443_d_n10;
        locals.var_qbsld_dn11 = assign31540_e46443_d_n11;
        locals.var_qbsld_dn12 = assign31540_e46443_d_n12;
        locals.var_qbsld_dn17 = assign31540_e46443_d_n17;
        locals.var_qbsld_rv = 0.0;

        let assign31550_e46454: f64 = if (((locals.var_flg_overd__blk915 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk913 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign31550_e46454;
        locals.var_guard1019_rv = 0.0;

        let (assign31560_e46467, assign31560_e46467_d_n0, assign31560_e46467_d_n2, assign31560_e46467_d_n6, assign31560_e46467_d_n7, assign31560_e46467_d_n10, assign31560_e46467_d_n11, assign31560_e46467_d_n12, assign31560_e46467_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
        let assign31560_e46465: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign31560_e46465, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign31560_e46467;
        locals.var_qovd_dn0 = assign31560_e46467_d_n0;
        locals.var_qovd_dn2 = assign31560_e46467_d_n2;
        locals.var_qovd_dn6 = assign31560_e46467_d_n6;
        locals.var_qovd_dn7 = assign31560_e46467_d_n7;
        locals.var_qovd_dn10 = assign31560_e46467_d_n10;
        locals.var_qovd_dn11 = assign31560_e46467_d_n11;
        locals.var_qovd_dn12 = assign31560_e46467_d_n12;
        locals.var_qovd_dn17 = assign31560_e46467_d_n17;
        locals.var_qovd_rv = 0.0;

        let (assign31570_e46480, assign31570_e46480_d_n0, assign31570_e46480_d_n2, assign31570_e46480_d_n6, assign31570_e46480_d_n7, assign31570_e46480_d_n10, assign31570_e46480_d_n11, assign31570_e46480_d_n12, assign31570_e46480_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
        let assign31570_e46478: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign31570_e46478, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign31570_e46480;
        locals.var_qbdld_dn0 = assign31570_e46480_d_n0;
        locals.var_qbdld_dn2 = assign31570_e46480_d_n2;
        locals.var_qbdld_dn6 = assign31570_e46480_d_n6;
        locals.var_qbdld_dn7 = assign31570_e46480_d_n7;
        locals.var_qbdld_dn10 = assign31570_e46480_d_n10;
        locals.var_qbdld_dn11 = assign31570_e46480_d_n11;
        locals.var_qbdld_dn12 = assign31570_e46480_d_n12;
        locals.var_qbdld_dn17 = assign31570_e46480_d_n17;
        locals.var_qbdld_rv = 0.0;

        let (assign31580_e46492,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        let assign31580_e46486: f64 = (locals.var_modervs * locals.var_cgso_given);
        let assign31580_e46489: f64 = (locals.var_modenml * locals.var_cgdo_given);
        let assign31580_e46490: f64 = (assign31580_e46486 + assign31580_e46489);
        (assign31580_e46490,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31580_e46492;
        locals.var_flg_overgiven_rv = 0.0;

        let (assign31590_e46506, assign31590_e46506_d_n0, assign31590_e46506_d_n2, assign31590_e46506_d_n6, assign31590_e46506_d_n7, assign31590_e46506_d_n10, assign31590_e46506_d_n11, assign31590_e46506_d_n12, assign31590_e46506_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31590_e46500: f64 = (locals.var_modervs * p.p170);
        let assign31590_e46503: f64 = (locals.var_modenml * p.p169);
        let assign31590_e46504: f64 = (assign31590_e46500 + assign31590_e46503);
        (assign31590_e46504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31590_e46506;
        locals.var_cgdoe_dn0 = assign31590_e46506_d_n0;
        locals.var_cgdoe_dn2 = assign31590_e46506_d_n2;
        locals.var_cgdoe_dn6 = assign31590_e46506_d_n6;
        locals.var_cgdoe_dn7 = assign31590_e46506_d_n7;
        locals.var_cgdoe_dn10 = assign31590_e46506_d_n10;
        locals.var_cgdoe_dn11 = assign31590_e46506_d_n11;
        locals.var_cgdoe_dn12 = assign31590_e46506_d_n12;
        locals.var_cgdoe_dn17 = assign31590_e46506_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let assign31600_e46509: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31600_e46509;
        locals.var_guard1020_rv = 0.0;

        let (assign31610_e46525, assign31610_e46525_d_n0, assign31610_e46525_d_n2, assign31610_e46525_d_n6, assign31610_e46525_d_n7, assign31610_e46525_d_n10, assign31610_e46525_d_n11, assign31610_e46525_d_n12, assign31610_e46525_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31610_e46519: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31610_e46522: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31610_e46523: f64 = (assign31610_e46519 + assign31610_e46522);
        (assign31610_e46523, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31610_e46525;
        locals.var_t1__blk896_dn0 = assign31610_e46525_d_n0;
        locals.var_t1__blk896_dn2 = assign31610_e46525_d_n2;
        locals.var_t1__blk896_dn6 = assign31610_e46525_d_n6;
        locals.var_t1__blk896_dn7 = assign31610_e46525_d_n7;
        locals.var_t1__blk896_dn10 = assign31610_e46525_d_n10;
        locals.var_t1__blk896_dn11 = assign31610_e46525_d_n11;
        locals.var_t1__blk896_dn12 = assign31610_e46525_d_n12;
        locals.var_t1__blk896_dn17 = assign31610_e46525_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31620_e46538, assign31620_e46538_d_n0, assign31620_e46538_d_n2, assign31620_e46538_d_n6, assign31620_e46538_d_n7, assign31620_e46538_d_n10, assign31620_e46538_d_n11, assign31620_e46538_d_n12, assign31620_e46538_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31620_e46535: f64 = (-locals.var_t1__blk896);
        let assign31620_e46536: f64 = (locals.var_cgdoe * assign31620_e46535);
        (assign31620_e46536, ((locals.var_cgdoe_dn0 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31620_e46538;
        locals.var_cgdoe_dn0 = assign31620_e46538_d_n0;
        locals.var_cgdoe_dn2 = assign31620_e46538_d_n2;
        locals.var_cgdoe_dn6 = assign31620_e46538_d_n6;
        locals.var_cgdoe_dn7 = assign31620_e46538_d_n7;
        locals.var_cgdoe_dn10 = assign31620_e46538_d_n10;
        locals.var_cgdoe_dn11 = assign31620_e46538_d_n11;
        locals.var_cgdoe_dn12 = assign31620_e46538_d_n12;
        locals.var_cgdoe_dn17 = assign31620_e46538_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31630_e46552, assign31630_e46552_d_n0, assign31630_e46552_d_n2, assign31630_e46552_d_n6, assign31630_e46552_d_n7, assign31630_e46552_d_n10, assign31630_e46552_d_n11, assign31630_e46552_d_n12, assign31630_e46552_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 == 0.0)) {
        let assign31630_e46549: f64 = (-locals.var_weffcv_nf);
        let assign31630_e46550: f64 = (locals.var_cgdoe * assign31630_e46549);
        (assign31630_e46550, (locals.var_cgdoe_dn0 * assign31630_e46549), (locals.var_cgdoe_dn2 * assign31630_e46549), (locals.var_cgdoe_dn6 * assign31630_e46549), (locals.var_cgdoe_dn7 * assign31630_e46549), (locals.var_cgdoe_dn10 * assign31630_e46549), (locals.var_cgdoe_dn11 * assign31630_e46549), (locals.var_cgdoe_dn12 * assign31630_e46549), (locals.var_cgdoe_dn17 * assign31630_e46549),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31630_e46552;
        locals.var_cgdoe_dn0 = assign31630_e46552_d_n0;
        locals.var_cgdoe_dn2 = assign31630_e46552_d_n2;
        locals.var_cgdoe_dn6 = assign31630_e46552_d_n6;
        locals.var_cgdoe_dn7 = assign31630_e46552_d_n7;
        locals.var_cgdoe_dn10 = assign31630_e46552_d_n10;
        locals.var_cgdoe_dn11 = assign31630_e46552_d_n11;
        locals.var_cgdoe_dn12 = assign31630_e46552_d_n12;
        locals.var_cgdoe_dn17 = assign31630_e46552_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31640_e46567, assign31640_e46567_d_n0, assign31640_e46567_d_n2, assign31640_e46567_d_n6, assign31640_e46567_d_n7, assign31640_e46567_d_n10, assign31640_e46567_d_n11, assign31640_e46567_d_n12, assign31640_e46567_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31640_e46560: f64 = (-locals.var_cgdoe);
        let assign31640_e46563: f64 = (locals.var_vgs - locals.var_vds);
        let assign31640_e46564: f64 = (assign31640_e46560 * assign31640_e46563);
        let assign31640_e46565: f64 = (locals.var_qgod + assign31640_e46564);
        (assign31640_e46565, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn17)))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31640_e46567;
        locals.var_qgod_dn0 = assign31640_e46567_d_n0;
        locals.var_qgod_dn2 = assign31640_e46567_d_n2;
        locals.var_qgod_dn6 = assign31640_e46567_d_n6;
        locals.var_qgod_dn7 = assign31640_e46567_d_n7;
        locals.var_qgod_dn10 = assign31640_e46567_d_n10;
        locals.var_qgod_dn11 = assign31640_e46567_d_n11;
        locals.var_qgod_dn12 = assign31640_e46567_d_n12;
        locals.var_qgod_dn17 = assign31640_e46567_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign31650_e46579,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        let assign31650_e46573: f64 = (locals.var_modenml * locals.var_cgso_given);
        let assign31650_e46576: f64 = (locals.var_modervs * locals.var_cgdo_given);
        let assign31650_e46577: f64 = (assign31650_e46573 + assign31650_e46576);
        (assign31650_e46577,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31650_e46579;
        locals.var_flg_overgiven_rv = 0.0;

        let (assign31660_e46593, assign31660_e46593_d_n0, assign31660_e46593_d_n2, assign31660_e46593_d_n6, assign31660_e46593_d_n7, assign31660_e46593_d_n10, assign31660_e46593_d_n11, assign31660_e46593_d_n12, assign31660_e46593_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31660_e46587: f64 = (locals.var_modenml * p.p170);
        let assign31660_e46590: f64 = (locals.var_modervs * p.p169);
        let assign31660_e46591: f64 = (assign31660_e46587 + assign31660_e46590);
        (assign31660_e46591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31660_e46593;
        locals.var_cgsoe_dn0 = assign31660_e46593_d_n0;
        locals.var_cgsoe_dn2 = assign31660_e46593_d_n2;
        locals.var_cgsoe_dn6 = assign31660_e46593_d_n6;
        locals.var_cgsoe_dn7 = assign31660_e46593_d_n7;
        locals.var_cgsoe_dn10 = assign31660_e46593_d_n10;
        locals.var_cgsoe_dn11 = assign31660_e46593_d_n11;
        locals.var_cgsoe_dn12 = assign31660_e46593_d_n12;
        locals.var_cgsoe_dn17 = assign31660_e46593_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let assign31670_e46596: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign31670_e46596;
        locals.var_guard1021_rv = 0.0;

        let (assign31680_e46612, assign31680_e46612_d_n0, assign31680_e46612_d_n2, assign31680_e46612_d_n6, assign31680_e46612_d_n7, assign31680_e46612_d_n10, assign31680_e46612_d_n11, assign31680_e46612_d_n12, assign31680_e46612_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31680_e46606: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31680_e46609: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31680_e46610: f64 = (assign31680_e46606 + assign31680_e46609);
        (assign31680_e46610, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31680_e46612;
        locals.var_t1__blk896_dn0 = assign31680_e46612_d_n0;
        locals.var_t1__blk896_dn2 = assign31680_e46612_d_n2;
        locals.var_t1__blk896_dn6 = assign31680_e46612_d_n6;
        locals.var_t1__blk896_dn7 = assign31680_e46612_d_n7;
        locals.var_t1__blk896_dn10 = assign31680_e46612_d_n10;
        locals.var_t1__blk896_dn11 = assign31680_e46612_d_n11;
        locals.var_t1__blk896_dn12 = assign31680_e46612_d_n12;
        locals.var_t1__blk896_dn17 = assign31680_e46612_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31690_e46625, assign31690_e46625_d_n0, assign31690_e46625_d_n2, assign31690_e46625_d_n6, assign31690_e46625_d_n7, assign31690_e46625_d_n10, assign31690_e46625_d_n11, assign31690_e46625_d_n12, assign31690_e46625_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
        let assign31690_e46622: f64 = (-locals.var_t1__blk896);
        let assign31690_e46623: f64 = (locals.var_cgsoe * assign31690_e46622);
        (assign31690_e46623, ((locals.var_cgsoe_dn0 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31690_e46625;
        locals.var_cgsoe_dn0 = assign31690_e46625_d_n0;
        locals.var_cgsoe_dn2 = assign31690_e46625_d_n2;
        locals.var_cgsoe_dn6 = assign31690_e46625_d_n6;
        locals.var_cgsoe_dn7 = assign31690_e46625_d_n7;
        locals.var_cgsoe_dn10 = assign31690_e46625_d_n10;
        locals.var_cgsoe_dn11 = assign31690_e46625_d_n11;
        locals.var_cgsoe_dn12 = assign31690_e46625_d_n12;
        locals.var_cgsoe_dn17 = assign31690_e46625_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31700_e46639, assign31700_e46639_d_n0, assign31700_e46639_d_n2, assign31700_e46639_d_n6, assign31700_e46639_d_n7, assign31700_e46639_d_n10, assign31700_e46639_d_n11, assign31700_e46639_d_n12, assign31700_e46639_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 == 0.0)) {
        let assign31700_e46636: f64 = (-locals.var_weffcv_nf);
        let assign31700_e46637: f64 = (locals.var_cgsoe * assign31700_e46636);
        (assign31700_e46637, (locals.var_cgsoe_dn0 * assign31700_e46636), (locals.var_cgsoe_dn2 * assign31700_e46636), (locals.var_cgsoe_dn6 * assign31700_e46636), (locals.var_cgsoe_dn7 * assign31700_e46636), (locals.var_cgsoe_dn10 * assign31700_e46636), (locals.var_cgsoe_dn11 * assign31700_e46636), (locals.var_cgsoe_dn12 * assign31700_e46636), (locals.var_cgsoe_dn17 * assign31700_e46636),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31700_e46639;
        locals.var_cgsoe_dn0 = assign31700_e46639_d_n0;
        locals.var_cgsoe_dn2 = assign31700_e46639_d_n2;
        locals.var_cgsoe_dn6 = assign31700_e46639_d_n6;
        locals.var_cgsoe_dn7 = assign31700_e46639_d_n7;
        locals.var_cgsoe_dn10 = assign31700_e46639_d_n10;
        locals.var_cgsoe_dn11 = assign31700_e46639_d_n11;
        locals.var_cgsoe_dn12 = assign31700_e46639_d_n12;
        locals.var_cgsoe_dn17 = assign31700_e46639_d_n17;
        locals.var_cgsoe_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31710_e46652, assign31710_e46652_d_n0, assign31710_e46652_d_n2, assign31710_e46652_d_n6, assign31710_e46652_d_n7, assign31710_e46652_d_n10, assign31710_e46652_d_n11, assign31710_e46652_d_n12, assign31710_e46652_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31710_e46647: f64 = (-locals.var_cgsoe);
        let assign31710_e46649: f64 = (assign31710_e46647 * locals.var_vgs);
        let assign31710_e46650: f64 = (locals.var_qgos + assign31710_e46649);
        (assign31710_e46650, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31710_e46652;
        locals.var_qgos_dn0 = assign31710_e46652_d_n0;
        locals.var_qgos_dn2 = assign31710_e46652_d_n2;
        locals.var_qgos_dn6 = assign31710_e46652_d_n6;
        locals.var_qgos_dn7 = assign31710_e46652_d_n7;
        locals.var_qgos_dn10 = assign31710_e46652_d_n10;
        locals.var_qgos_dn11 = assign31710_e46652_d_n11;
        locals.var_qgos_dn12 = assign31710_e46652_d_n12;
        locals.var_qgos_dn17 = assign31710_e46652_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign31720_e46665: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31720_e46665;
        locals.var_guard1022_rv = 0.0;

        let assign31730_e46668: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31730_e46668;
        locals.var_guard1023_rv = 0.0;

        let (assign31740_e46684, assign31740_e46684_d_n0, assign31740_e46684_d_n2, assign31740_e46684_d_n6, assign31740_e46684_d_n7, assign31740_e46684_d_n10, assign31740_e46684_d_n11, assign31740_e46684_d_n12, assign31740_e46684_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31740_e46678: f64 = (-locals.var_cox0__blk906);
        let assign31740_e46680: f64 = (assign31740_e46678 * p.p188);
        let assign31740_e46682: f64 = (assign31740_e46680 * locals.var_w_diodcv);
        (assign31740_e46682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31740_e46684;
        locals.var_cgdoe_dn0 = assign31740_e46684_d_n0;
        locals.var_cgdoe_dn2 = assign31740_e46684_d_n2;
        locals.var_cgdoe_dn6 = assign31740_e46684_d_n6;
        locals.var_cgdoe_dn7 = assign31740_e46684_d_n7;
        locals.var_cgdoe_dn10 = assign31740_e46684_d_n10;
        locals.var_cgdoe_dn11 = assign31740_e46684_d_n11;
        locals.var_cgdoe_dn12 = assign31740_e46684_d_n12;
        locals.var_cgdoe_dn17 = assign31740_e46684_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31750_e46701, assign31750_e46701_d_n0, assign31750_e46701_d_n2, assign31750_e46701_d_n6, assign31750_e46701_d_n7, assign31750_e46701_d_n10, assign31750_e46701_d_n11, assign31750_e46701_d_n12, assign31750_e46701_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) {
        let assign31750_e46695: f64 = (-locals.var_cox0__blk906);
        let assign31750_e46697: f64 = (assign31750_e46695 * p.p188);
        let assign31750_e46699: f64 = (assign31750_e46697 * locals.var_weffcv_nf);
        (assign31750_e46699, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31750_e46701;
        locals.var_cgdoe_dn0 = assign31750_e46701_d_n0;
        locals.var_cgdoe_dn2 = assign31750_e46701_d_n2;
        locals.var_cgdoe_dn6 = assign31750_e46701_d_n6;
        locals.var_cgdoe_dn7 = assign31750_e46701_d_n7;
        locals.var_cgdoe_dn10 = assign31750_e46701_d_n10;
        locals.var_cgdoe_dn11 = assign31750_e46701_d_n11;
        locals.var_cgdoe_dn12 = assign31750_e46701_d_n12;
        locals.var_cgdoe_dn17 = assign31750_e46701_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31760_e46717, assign31760_e46717_d_n0, assign31760_e46717_d_n2, assign31760_e46717_d_n6, assign31760_e46717_d_n7, assign31760_e46717_d_n10, assign31760_e46717_d_n11, assign31760_e46717_d_n12, assign31760_e46717_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) {
        let assign31760_e46711: f64 = (locals.var_modervs * p.p170);
        let assign31760_e46714: f64 = (locals.var_modenml * p.p169);
        let assign31760_e46715: f64 = (assign31760_e46711 + assign31760_e46714);
        (assign31760_e46715, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31760_e46717;
        locals.var_cgdoe_dn0 = assign31760_e46717_d_n0;
        locals.var_cgdoe_dn2 = assign31760_e46717_d_n2;
        locals.var_cgdoe_dn6 = assign31760_e46717_d_n6;
        locals.var_cgdoe_dn7 = assign31760_e46717_d_n7;
        locals.var_cgdoe_dn10 = assign31760_e46717_d_n10;
        locals.var_cgdoe_dn11 = assign31760_e46717_d_n11;
        locals.var_cgdoe_dn12 = assign31760_e46717_d_n12;
        locals.var_cgdoe_dn17 = assign31760_e46717_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let assign31770_e46720: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31770_e46720;
        locals.var_guard1024_rv = 0.0;

        let (assign31780_e46738, assign31780_e46738_d_n0, assign31780_e46738_d_n2, assign31780_e46738_d_n6, assign31780_e46738_d_n7, assign31780_e46738_d_n10, assign31780_e46738_d_n11, assign31780_e46738_d_n12, assign31780_e46738_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31780_e46732: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31780_e46735: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31780_e46736: f64 = (assign31780_e46732 + assign31780_e46735);
        (assign31780_e46736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31780_e46738;
        locals.var_t1__blk896_dn0 = assign31780_e46738_d_n0;
        locals.var_t1__blk896_dn2 = assign31780_e46738_d_n2;
        locals.var_t1__blk896_dn6 = assign31780_e46738_d_n6;
        locals.var_t1__blk896_dn7 = assign31780_e46738_d_n7;
        locals.var_t1__blk896_dn10 = assign31780_e46738_d_n10;
        locals.var_t1__blk896_dn11 = assign31780_e46738_d_n11;
        locals.var_t1__blk896_dn12 = assign31780_e46738_d_n12;
        locals.var_t1__blk896_dn17 = assign31780_e46738_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31790_e46753, assign31790_e46753_d_n0, assign31790_e46753_d_n2, assign31790_e46753_d_n6, assign31790_e46753_d_n7, assign31790_e46753_d_n10, assign31790_e46753_d_n11, assign31790_e46753_d_n12, assign31790_e46753_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31790_e46750: f64 = (-locals.var_t1__blk896);
        let assign31790_e46751: f64 = (locals.var_cgdoe * assign31790_e46750);
        (assign31790_e46751, ((locals.var_cgdoe_dn0 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31790_e46753;
        locals.var_cgdoe_dn0 = assign31790_e46753_d_n0;
        locals.var_cgdoe_dn2 = assign31790_e46753_d_n2;
        locals.var_cgdoe_dn6 = assign31790_e46753_d_n6;
        locals.var_cgdoe_dn7 = assign31790_e46753_d_n7;
        locals.var_cgdoe_dn10 = assign31790_e46753_d_n10;
        locals.var_cgdoe_dn11 = assign31790_e46753_d_n11;
        locals.var_cgdoe_dn12 = assign31790_e46753_d_n12;
        locals.var_cgdoe_dn17 = assign31790_e46753_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31800_e46769, assign31800_e46769_d_n0, assign31800_e46769_d_n2, assign31800_e46769_d_n6, assign31800_e46769_d_n7, assign31800_e46769_d_n10, assign31800_e46769_d_n11, assign31800_e46769_d_n12, assign31800_e46769_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 == 0.0)) {
        let assign31800_e46766: f64 = (-locals.var_weffcv_nf);
        let assign31800_e46767: f64 = (locals.var_cgdoe * assign31800_e46766);
        (assign31800_e46767, (locals.var_cgdoe_dn0 * assign31800_e46766), (locals.var_cgdoe_dn2 * assign31800_e46766), (locals.var_cgdoe_dn6 * assign31800_e46766), (locals.var_cgdoe_dn7 * assign31800_e46766), (locals.var_cgdoe_dn10 * assign31800_e46766), (locals.var_cgdoe_dn11 * assign31800_e46766), (locals.var_cgdoe_dn12 * assign31800_e46766), (locals.var_cgdoe_dn17 * assign31800_e46766),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31800_e46769;
        locals.var_cgdoe_dn0 = assign31800_e46769_d_n0;
        locals.var_cgdoe_dn2 = assign31800_e46769_d_n2;
        locals.var_cgdoe_dn6 = assign31800_e46769_d_n6;
        locals.var_cgdoe_dn7 = assign31800_e46769_d_n7;
        locals.var_cgdoe_dn10 = assign31800_e46769_d_n10;
        locals.var_cgdoe_dn11 = assign31800_e46769_d_n11;
        locals.var_cgdoe_dn12 = assign31800_e46769_d_n12;
        locals.var_cgdoe_dn17 = assign31800_e46769_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31810_e46781, assign31810_e46781_d_n0, assign31810_e46781_d_n2, assign31810_e46781_d_n6, assign31810_e46781_d_n7, assign31810_e46781_d_n10, assign31810_e46781_d_n11, assign31810_e46781_d_n12, assign31810_e46781_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
        let assign31810_e46775: f64 = (-locals.var_cgdoe);
        let assign31810_e46778: f64 = (locals.var_vgs - locals.var_vds);
        let assign31810_e46779: f64 = (assign31810_e46775 * assign31810_e46778);
        (assign31810_e46779, (((-locals.var_cgdoe_dn0) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31810_e46781;
        locals.var_qgod_dn0 = assign31810_e46781_d_n0;
        locals.var_qgod_dn2 = assign31810_e46781_d_n2;
        locals.var_qgod_dn6 = assign31810_e46781_d_n6;
        locals.var_qgod_dn7 = assign31810_e46781_d_n7;
        locals.var_qgod_dn10 = assign31810_e46781_d_n10;
        locals.var_qgod_dn11 = assign31810_e46781_d_n11;
        locals.var_qgod_dn12 = assign31810_e46781_d_n12;
        locals.var_qgod_dn17 = assign31810_e46781_d_n17;
        locals.var_qgod_rv = 0.0;

        let assign31820_e46794: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31820_e46794;
        locals.var_guard1025_rv = 0.0;

        let assign31830_e46797: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31830_e46797;
        locals.var_guard1026_rv = 0.0;

        let (assign31840_e46813, assign31840_e46813_d_n0, assign31840_e46813_d_n2, assign31840_e46813_d_n6, assign31840_e46813_d_n7, assign31840_e46813_d_n10, assign31840_e46813_d_n11, assign31840_e46813_d_n12, assign31840_e46813_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 != 0.0)) {
        let assign31840_e46807: f64 = (-locals.var_cox0__blk906);
        let assign31840_e46809: f64 = (assign31840_e46807 * p.p188);
        let assign31840_e46811: f64 = (assign31840_e46809 * locals.var_w_dioscv);
        (assign31840_e46811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31840_e46813;
        locals.var_cgsoe_dn0 = assign31840_e46813_d_n0;
        locals.var_cgsoe_dn2 = assign31840_e46813_d_n2;
        locals.var_cgsoe_dn6 = assign31840_e46813_d_n6;
        locals.var_cgsoe_dn7 = assign31840_e46813_d_n7;
        locals.var_cgsoe_dn10 = assign31840_e46813_d_n10;
        locals.var_cgsoe_dn11 = assign31840_e46813_d_n11;
        locals.var_cgsoe_dn12 = assign31840_e46813_d_n12;
        locals.var_cgsoe_dn17 = assign31840_e46813_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31850_e46830, assign31850_e46830_d_n0, assign31850_e46830_d_n2, assign31850_e46830_d_n6, assign31850_e46830_d_n7, assign31850_e46830_d_n10, assign31850_e46830_d_n11, assign31850_e46830_d_n12, assign31850_e46830_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 == 0.0)) {
        let assign31850_e46824: f64 = (-locals.var_cox0__blk906);
        let assign31850_e46826: f64 = (assign31850_e46824 * p.p188);
        let assign31850_e46828: f64 = (assign31850_e46826 * locals.var_weffcv_nf);
        (assign31850_e46828, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31850_e46830;
        locals.var_cgsoe_dn0 = assign31850_e46830_d_n0;
        locals.var_cgsoe_dn2 = assign31850_e46830_d_n2;
        locals.var_cgsoe_dn6 = assign31850_e46830_d_n6;
        locals.var_cgsoe_dn7 = assign31850_e46830_d_n7;
        locals.var_cgsoe_dn10 = assign31850_e46830_d_n10;
        locals.var_cgsoe_dn11 = assign31850_e46830_d_n11;
        locals.var_cgsoe_dn12 = assign31850_e46830_d_n12;
        locals.var_cgsoe_dn17 = assign31850_e46830_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31860_e46846, assign31860_e46846_d_n0, assign31860_e46846_d_n2, assign31860_e46846_d_n6, assign31860_e46846_d_n7, assign31860_e46846_d_n10, assign31860_e46846_d_n11, assign31860_e46846_d_n12, assign31860_e46846_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) {
        let assign31860_e46840: f64 = (locals.var_modenml * p.p170);
        let assign31860_e46843: f64 = (locals.var_modervs * p.p169);
        let assign31860_e46844: f64 = (assign31860_e46840 + assign31860_e46843);
        (assign31860_e46844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31860_e46846;
        locals.var_cgsoe_dn0 = assign31860_e46846_d_n0;
        locals.var_cgsoe_dn2 = assign31860_e46846_d_n2;
        locals.var_cgsoe_dn6 = assign31860_e46846_d_n6;
        locals.var_cgsoe_dn7 = assign31860_e46846_d_n7;
        locals.var_cgsoe_dn10 = assign31860_e46846_d_n10;
        locals.var_cgsoe_dn11 = assign31860_e46846_d_n11;
        locals.var_cgsoe_dn12 = assign31860_e46846_d_n12;
        locals.var_cgsoe_dn17 = assign31860_e46846_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let assign31870_e46849: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31870_e46849;
        locals.var_guard1027_rv = 0.0;

        let (assign31880_e46867, assign31880_e46867_d_n0, assign31880_e46867_d_n2, assign31880_e46867_d_n6, assign31880_e46867_d_n7, assign31880_e46867_d_n10, assign31880_e46867_d_n11, assign31880_e46867_d_n12, assign31880_e46867_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
        let assign31880_e46861: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31880_e46864: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31880_e46865: f64 = (assign31880_e46861 + assign31880_e46864);
        (assign31880_e46865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign31880_e46867;
        locals.var_t1__blk896_dn0 = assign31880_e46867_d_n0;
        locals.var_t1__blk896_dn2 = assign31880_e46867_d_n2;
        locals.var_t1__blk896_dn6 = assign31880_e46867_d_n6;
        locals.var_t1__blk896_dn7 = assign31880_e46867_d_n7;
        locals.var_t1__blk896_dn10 = assign31880_e46867_d_n10;
        locals.var_t1__blk896_dn11 = assign31880_e46867_d_n11;
        locals.var_t1__blk896_dn12 = assign31880_e46867_d_n12;
        locals.var_t1__blk896_dn17 = assign31880_e46867_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign31890_e46882, assign31890_e46882_d_n0, assign31890_e46882_d_n2, assign31890_e46882_d_n6, assign31890_e46882_d_n7, assign31890_e46882_d_n10, assign31890_e46882_d_n11, assign31890_e46882_d_n12, assign31890_e46882_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
        let assign31890_e46879: f64 = (-locals.var_t1__blk896);
        let assign31890_e46880: f64 = (locals.var_cgsoe * assign31890_e46879);
        (assign31890_e46880, ((locals.var_cgsoe_dn0 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31890_e46882;
        locals.var_cgsoe_dn0 = assign31890_e46882_d_n0;
        locals.var_cgsoe_dn2 = assign31890_e46882_d_n2;
        locals.var_cgsoe_dn6 = assign31890_e46882_d_n6;
        locals.var_cgsoe_dn7 = assign31890_e46882_d_n7;
        locals.var_cgsoe_dn10 = assign31890_e46882_d_n10;
        locals.var_cgsoe_dn11 = assign31890_e46882_d_n11;
        locals.var_cgsoe_dn12 = assign31890_e46882_d_n12;
        locals.var_cgsoe_dn17 = assign31890_e46882_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31900_e46898, assign31900_e46898_d_n0, assign31900_e46898_d_n2, assign31900_e46898_d_n6, assign31900_e46898_d_n7, assign31900_e46898_d_n10, assign31900_e46898_d_n11, assign31900_e46898_d_n12, assign31900_e46898_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 == 0.0)) {
        let assign31900_e46895: f64 = (-locals.var_weffcv_nf);
        let assign31900_e46896: f64 = (locals.var_cgsoe * assign31900_e46895);
        (assign31900_e46896, (locals.var_cgsoe_dn0 * assign31900_e46895), (locals.var_cgsoe_dn2 * assign31900_e46895), (locals.var_cgsoe_dn6 * assign31900_e46895), (locals.var_cgsoe_dn7 * assign31900_e46895), (locals.var_cgsoe_dn10 * assign31900_e46895), (locals.var_cgsoe_dn11 * assign31900_e46895), (locals.var_cgsoe_dn12 * assign31900_e46895), (locals.var_cgsoe_dn17 * assign31900_e46895),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31900_e46898;
        locals.var_cgsoe_dn0 = assign31900_e46898_d_n0;
        locals.var_cgsoe_dn2 = assign31900_e46898_d_n2;
        locals.var_cgsoe_dn6 = assign31900_e46898_d_n6;
        locals.var_cgsoe_dn7 = assign31900_e46898_d_n7;
        locals.var_cgsoe_dn10 = assign31900_e46898_d_n10;
        locals.var_cgsoe_dn11 = assign31900_e46898_d_n11;
        locals.var_cgsoe_dn12 = assign31900_e46898_d_n12;
        locals.var_cgsoe_dn17 = assign31900_e46898_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31910_e46908, assign31910_e46908_d_n0, assign31910_e46908_d_n2, assign31910_e46908_d_n6, assign31910_e46908_d_n7, assign31910_e46908_d_n10, assign31910_e46908_d_n11, assign31910_e46908_d_n12, assign31910_e46908_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
        let assign31910_e46904: f64 = (-locals.var_cgsoe);
        let assign31910_e46906: f64 = (assign31910_e46904 * locals.var_vgs);
        (assign31910_e46906, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31910_e46908;
        locals.var_qgos_dn0 = assign31910_e46908_d_n0;
        locals.var_qgos_dn2 = assign31910_e46908_d_n2;
        locals.var_qgos_dn6 = assign31910_e46908_d_n6;
        locals.var_qgos_dn7 = assign31910_e46908_d_n7;
        locals.var_qgos_dn10 = assign31910_e46908_d_n10;
        locals.var_qgos_dn11 = assign31910_e46908_d_n11;
        locals.var_qgos_dn12 = assign31910_e46908_d_n12;
        locals.var_qgos_dn17 = assign31910_e46908_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign31920_e46911: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31920_e46911;
        locals.var_guard1028_rv = 0.0;

        let (assign31930_e46915, assign31930_e46915_d_n6, assign31930_e46915_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    } else {
        (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12,)
    }
};
        locals.var_vbdj = assign31930_e46915;
        locals.var_vbdj_dn6 = assign31930_e46915_d_n6;
        locals.var_vbdj_dn12 = assign31930_e46915_d_n12;
        locals.var_vbdj_rv = 0.0;

        let (assign31940_e46919, assign31940_e46919_d_n7, assign31940_e46919_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    } else {
        (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12,)
    }
};
        locals.var_vbsj = assign31940_e46919;
        locals.var_vbsj_dn7 = assign31940_e46919_d_n7;
        locals.var_vbsj_dn12 = assign31940_e46919_d_n12;
        locals.var_vbsj_rv = 0.0;

        let (assign31950_e46941, assign31950_e46941_d_n0, assign31950_e46941_d_n2, assign31950_e46941_d_n6, assign31950_e46941_d_n7, assign31950_e46941_d_n10, assign31950_e46941_d_n11, assign31950_e46941_d_n12, assign31950_e46941_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31950_e46924: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31950_e46927: f64 = (locals.var_eg * locals.var_beta);
        let assign31950_e46928: f64 = (assign31950_e46924 - assign31950_e46927);
        let assign31950_e46932: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31950_e46933: f64 = (assign31950_e46932).ln();
        let assign31950_e46934: f64 = (p.p175 * assign31950_e46933);
        let assign31950_e46935: f64 = (assign31950_e46928 + assign31950_e46934);
        let assign31950_e46937: f64 = (assign31950_e46935 / p.p174);
        let assign31950_e46938: f64 = (assign31950_e46937).exp();
        let assign31950_e46939: f64 = (p.p173 * assign31950_e46938);
        (assign31950_e46939, (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31950_e46932))) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17,)
    }
};
        locals.var_js = assign31950_e46941;
        locals.var_js_dn0 = assign31950_e46941_d_n0;
        locals.var_js_dn2 = assign31950_e46941_d_n2;
        locals.var_js_dn6 = assign31950_e46941_d_n6;
        locals.var_js_dn7 = assign31950_e46941_d_n7;
        locals.var_js_dn10 = assign31950_e46941_d_n10;
        locals.var_js_dn11 = assign31950_e46941_d_n11;
        locals.var_js_dn12 = assign31950_e46941_d_n12;
        locals.var_js_dn17 = assign31950_e46941_d_n17;
        locals.var_js_rv = 0.0;

        let (assign31960_e46963, assign31960_e46963_d_n0, assign31960_e46963_d_n2, assign31960_e46963_d_n6, assign31960_e46963_d_n7, assign31960_e46963_d_n10, assign31960_e46963_d_n11, assign31960_e46963_d_n12, assign31960_e46963_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31960_e46946: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31960_e46949: f64 = (locals.var_eg * locals.var_beta);
        let assign31960_e46950: f64 = (assign31960_e46946 - assign31960_e46949);
        let assign31960_e46954: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31960_e46955: f64 = (assign31960_e46954).ln();
        let assign31960_e46956: f64 = (p.p176 * assign31960_e46955);
        let assign31960_e46957: f64 = (assign31960_e46950 + assign31960_e46956);
        let assign31960_e46959: f64 = (assign31960_e46957 / p.p174);
        let assign31960_e46960: f64 = (assign31960_e46959).exp();
        let assign31960_e46961: f64 = (p.p173 * assign31960_e46960);
        (assign31960_e46961, (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31960_e46954))) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17,)
    }
};
        locals.var_js2 = assign31960_e46963;
        locals.var_js2_dn0 = assign31960_e46963_d_n0;
        locals.var_js2_dn2 = assign31960_e46963_d_n2;
        locals.var_js2_dn6 = assign31960_e46963_d_n6;
        locals.var_js2_dn7 = assign31960_e46963_d_n7;
        locals.var_js2_dn10 = assign31960_e46963_d_n10;
        locals.var_js2_dn11 = assign31960_e46963_d_n11;
        locals.var_js2_dn12 = assign31960_e46963_d_n12;
        locals.var_js2_dn17 = assign31960_e46963_d_n17;
        locals.var_js2_rv = 0.0;

        let (assign31970_e46971, assign31970_e46971_d_n0, assign31970_e46971_d_n2, assign31970_e46971_d_n6, assign31970_e46971_d_n7, assign31970_e46971_d_n10, assign31970_e46971_d_n11, assign31970_e46971_d_n12, assign31970_e46971_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31970_e46967: f64 = (locals.var_w_diod * p.p237);
        let assign31970_e46969: f64 = (assign31970_e46967 * locals.var_js);
        (assign31970_e46969, (assign31970_e46967 * locals.var_js_dn0), (assign31970_e46967 * locals.var_js_dn2), (assign31970_e46967 * locals.var_js_dn6), (assign31970_e46967 * locals.var_js_dn7), (assign31970_e46967 * locals.var_js_dn10), (assign31970_e46967 * locals.var_js_dn11), (assign31970_e46967 * locals.var_js_dn12), (assign31970_e46967 * locals.var_js_dn17),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    }
};
        locals.var_isbd = assign31970_e46971;
        locals.var_isbd_dn0 = assign31970_e46971_d_n0;
        locals.var_isbd_dn2 = assign31970_e46971_d_n2;
        locals.var_isbd_dn6 = assign31970_e46971_d_n6;
        locals.var_isbd_dn7 = assign31970_e46971_d_n7;
        locals.var_isbd_dn10 = assign31970_e46971_d_n10;
        locals.var_isbd_dn11 = assign31970_e46971_d_n11;
        locals.var_isbd_dn12 = assign31970_e46971_d_n12;
        locals.var_isbd_dn17 = assign31970_e46971_d_n17;
        locals.var_isbd_rv = 0.0;

        let (assign31980_e46979, assign31980_e46979_d_n0, assign31980_e46979_d_n2, assign31980_e46979_d_n6, assign31980_e46979_d_n7, assign31980_e46979_d_n10, assign31980_e46979_d_n11, assign31980_e46979_d_n12, assign31980_e46979_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31980_e46975: f64 = (locals.var_w_diod * p.p237);
        let assign31980_e46977: f64 = (assign31980_e46975 * locals.var_js2);
        (assign31980_e46977, (assign31980_e46975 * locals.var_js2_dn0), (assign31980_e46975 * locals.var_js2_dn2), (assign31980_e46975 * locals.var_js2_dn6), (assign31980_e46975 * locals.var_js2_dn7), (assign31980_e46975 * locals.var_js2_dn10), (assign31980_e46975 * locals.var_js2_dn11), (assign31980_e46975 * locals.var_js2_dn12), (assign31980_e46975 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17,)
    }
};
        locals.var_isbd2 = assign31980_e46979;
        locals.var_isbd2_dn0 = assign31980_e46979_d_n0;
        locals.var_isbd2_dn2 = assign31980_e46979_d_n2;
        locals.var_isbd2_dn6 = assign31980_e46979_d_n6;
        locals.var_isbd2_dn7 = assign31980_e46979_d_n7;
        locals.var_isbd2_dn10 = assign31980_e46979_d_n10;
        locals.var_isbd2_dn11 = assign31980_e46979_d_n11;
        locals.var_isbd2_dn12 = assign31980_e46979_d_n12;
        locals.var_isbd2_dn17 = assign31980_e46979_d_n17;
        locals.var_isbd2_rv = 0.0;

        let (assign31990_e46987, assign31990_e46987_d_n0, assign31990_e46987_d_n2, assign31990_e46987_d_n6, assign31990_e46987_d_n7, assign31990_e46987_d_n10, assign31990_e46987_d_n11, assign31990_e46987_d_n12, assign31990_e46987_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign31990_e46983: f64 = (locals.var_w_dios * p.p237);
        let assign31990_e46985: f64 = (assign31990_e46983 * locals.var_js);
        (assign31990_e46985, (assign31990_e46983 * locals.var_js_dn0), (assign31990_e46983 * locals.var_js_dn2), (assign31990_e46983 * locals.var_js_dn6), (assign31990_e46983 * locals.var_js_dn7), (assign31990_e46983 * locals.var_js_dn10), (assign31990_e46983 * locals.var_js_dn11), (assign31990_e46983 * locals.var_js_dn12), (assign31990_e46983 * locals.var_js_dn17),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17,)
    }
};
        locals.var_isbs = assign31990_e46987;
        locals.var_isbs_dn0 = assign31990_e46987_d_n0;
        locals.var_isbs_dn2 = assign31990_e46987_d_n2;
        locals.var_isbs_dn6 = assign31990_e46987_d_n6;
        locals.var_isbs_dn7 = assign31990_e46987_d_n7;
        locals.var_isbs_dn10 = assign31990_e46987_d_n10;
        locals.var_isbs_dn11 = assign31990_e46987_d_n11;
        locals.var_isbs_dn12 = assign31990_e46987_d_n12;
        locals.var_isbs_dn17 = assign31990_e46987_d_n17;
        locals.var_isbs_rv = 0.0;

        let (assign32000_e46995, assign32000_e46995_d_n0, assign32000_e46995_d_n2, assign32000_e46995_d_n6, assign32000_e46995_d_n7, assign32000_e46995_d_n10, assign32000_e46995_d_n11, assign32000_e46995_d_n12, assign32000_e46995_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32000_e46991: f64 = (locals.var_w_dios * p.p237);
        let assign32000_e46993: f64 = (assign32000_e46991 * locals.var_js2);
        (assign32000_e46993, (assign32000_e46991 * locals.var_js2_dn0), (assign32000_e46991 * locals.var_js2_dn2), (assign32000_e46991 * locals.var_js2_dn6), (assign32000_e46991 * locals.var_js2_dn7), (assign32000_e46991 * locals.var_js2_dn10), (assign32000_e46991 * locals.var_js2_dn11), (assign32000_e46991 * locals.var_js2_dn12), (assign32000_e46991 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17,)
    }
};
        locals.var_isbs2 = assign32000_e46995;
        locals.var_isbs2_dn0 = assign32000_e46995_d_n0;
        locals.var_isbs2_dn2 = assign32000_e46995_d_n2;
        locals.var_isbs2_dn6 = assign32000_e46995_d_n6;
        locals.var_isbs2_dn7 = assign32000_e46995_d_n7;
        locals.var_isbs2_dn10 = assign32000_e46995_d_n10;
        locals.var_isbs2_dn11 = assign32000_e46995_d_n11;
        locals.var_isbs2_dn12 = assign32000_e46995_d_n12;
        locals.var_isbs2_dn17 = assign32000_e46995_d_n17;
        locals.var_isbs2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32010_e47001, assign32010_e47001_d_n6, assign32010_e47001_d_n7, assign32010_e47001_d_n10, assign32010_e47001_d_n12,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32010_e46999: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign32010_e46999, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32010_e47001;
        locals.var_t1__blk1030_dn6 = assign32010_e47001_d_n6;
        locals.var_t1__blk1030_dn7 = assign32010_e47001_d_n7;
        locals.var_t1__blk1030_dn10 = assign32010_e47001_d_n10;
        locals.var_t1__blk1030_dn12 = assign32010_e47001_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32030_e47013, assign32030_e47013_d_n0, assign32030_e47013_d_n2, assign32030_e47013_d_n6, assign32030_e47013_d_n7, assign32030_e47013_d_n10, assign32030_e47013_d_n11, assign32030_e47013_d_n12, assign32030_e47013_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32030_e47011: f64 = (locals.var_isbd + 1e-50);
        (assign32030_e47011, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32030_e47013;
        locals.var_t2__blk1031_dn0 = assign32030_e47013_d_n0;
        locals.var_t2__blk1031_dn2 = assign32030_e47013_d_n2;
        locals.var_t2__blk1031_dn6 = assign32030_e47013_d_n6;
        locals.var_t2__blk1031_dn7 = assign32030_e47013_d_n7;
        locals.var_t2__blk1031_dn10 = assign32030_e47013_d_n10;
        locals.var_t2__blk1031_dn11 = assign32030_e47013_d_n11;
        locals.var_t2__blk1031_dn12 = assign32030_e47013_d_n12;
        locals.var_t2__blk1031_dn17 = assign32030_e47013_d_n17;
        locals.var_t2__blk1031_rv = 0.0;

        let (assign32050_e47027, assign32050_e47027_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn10,)
    }
};
        locals.var_vbdt = assign32050_e47027;
        locals.var_vbdt_dn10 = assign32050_e47027_d_n10;
        locals.var_vbdt_rv = 0.0;

        let (assign32060_e47035, assign32060_e47035_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbst, locals.var_vbst_dn10,)
    }
};
        locals.var_vbst = assign32060_e47035;
        locals.var_vbst_dn10 = assign32060_e47035_d_n10;
        locals.var_vbst_rv = 0.0;

        let (assign32070_e47041, assign32070_e47041_d_n10,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32070_e47039: f64 = (p.p174 * locals.var_beta_inv);
        (assign32070_e47039, (p.p174 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn10,)
    }
};
        locals.var_nvtm = assign32070_e47041;
        locals.var_nvtm_dn10 = assign32070_e47041_d_n10;
        locals.var_nvtm_rv = 0.0;

        let assign32080_e47044: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign32080_e47044;
        locals.var_guard1057_rv = 0.0;

        let (assign32090_e47053, assign32090_e47053_d_n6, assign32090_e47053_d_n7, assign32090_e47053_d_n10, assign32090_e47053_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
        let assign32090_e47050: f64 = (locals.var_vbdj / locals.var_nvtm);
        let assign32090_e47051: f64 = (assign32090_e47050).exp();
        (assign32090_e47051, (assign32090_e47051 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32090_e47051 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32090_e47051 * (locals.var_vbdj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32090_e47053;
        locals.var_t1__blk1030_dn6 = assign32090_e47053_d_n6;
        locals.var_t1__blk1030_dn7 = assign32090_e47053_d_n7;
        locals.var_t1__blk1030_dn10 = assign32090_e47053_d_n10;
        locals.var_t1__blk1030_dn12 = assign32090_e47053_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32100_e47063, assign32100_e47063_d_n0, assign32100_e47063_d_n2, assign32100_e47063_d_n6, assign32100_e47063_d_n7, assign32100_e47063_d_n10, assign32100_e47063_d_n11, assign32100_e47063_d_n12, assign32100_e47063_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
        let assign32100_e47060: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32100_e47061: f64 = (locals.var_isbd * assign32100_e47060);
        (assign32100_e47061, (locals.var_isbd_dn0 * assign32100_e47060), (locals.var_isbd_dn2 * assign32100_e47060), ((locals.var_isbd_dn6 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn6)), ((locals.var_isbd_dn7 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn7)), ((locals.var_isbd_dn10 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn10)), (locals.var_isbd_dn11 * assign32100_e47060), ((locals.var_isbd_dn12 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn12)), (locals.var_isbd_dn17 * assign32100_e47060),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32100_e47063;
        locals.var_ibd_dn0 = assign32100_e47063_d_n0;
        locals.var_ibd_dn2 = assign32100_e47063_d_n2;
        locals.var_ibd_dn6 = assign32100_e47063_d_n6;
        locals.var_ibd_dn7 = assign32100_e47063_d_n7;
        locals.var_ibd_dn10 = assign32100_e47063_d_n10;
        locals.var_ibd_dn11 = assign32100_e47063_d_n11;
        locals.var_ibd_dn12 = assign32100_e47063_d_n12;
        locals.var_ibd_dn17 = assign32100_e47063_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32110_e47073, assign32110_e47073_d_n6, assign32110_e47073_d_n7, assign32110_e47073_d_n10, assign32110_e47073_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
        let assign32110_e47070: f64 = (locals.var_vbdt / locals.var_nvtm);
        let assign32110_e47071: f64 = (assign32110_e47070).exp();
        (assign32110_e47071, 0.0, 0.0, (assign32110_e47071 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32110_e47073;
        locals.var_t1__blk1030_dn6 = assign32110_e47073_d_n6;
        locals.var_t1__blk1030_dn7 = assign32110_e47073_d_n7;
        locals.var_t1__blk1030_dn10 = assign32110_e47073_d_n10;
        locals.var_t1__blk1030_dn12 = assign32110_e47073_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32120_e47094, assign32120_e47094_d_n0, assign32120_e47094_d_n2, assign32120_e47094_d_n6, assign32120_e47094_d_n7, assign32120_e47094_d_n10, assign32120_e47094_d_n11, assign32120_e47094_d_n12, assign32120_e47094_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
        let assign32120_e47081: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32120_e47082: f64 = (locals.var_isbd * assign32120_e47081);
        let assign32120_e47085: f64 = (locals.var_isbd / locals.var_nvtm);
        let assign32120_e47087: f64 = (assign32120_e47085 * locals.var_t1__blk1030);
        let assign32120_e47090: f64 = (locals.var_vbdj - locals.var_vbdt);
        let assign32120_e47091: f64 = (assign32120_e47087 * assign32120_e47090);
        let assign32120_e47092: f64 = (assign32120_e47082 + assign32120_e47091);
        (assign32120_e47092, ((locals.var_isbd_dn0 * assign32120_e47081) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), ((locals.var_isbd_dn2 * assign32120_e47081) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn6 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn6)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn7)) * assign32120_e47090)), (((locals.var_isbd_dn10 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn10)) * assign32120_e47090) + (assign32120_e47087 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32120_e47081) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn12 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn12)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32120_e47081) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32120_e47094;
        locals.var_ibd_dn0 = assign32120_e47094_d_n0;
        locals.var_ibd_dn2 = assign32120_e47094_d_n2;
        locals.var_ibd_dn6 = assign32120_e47094_d_n6;
        locals.var_ibd_dn7 = assign32120_e47094_d_n7;
        locals.var_ibd_dn10 = assign32120_e47094_d_n10;
        locals.var_ibd_dn11 = assign32120_e47094_d_n11;
        locals.var_ibd_dn12 = assign32120_e47094_d_n12;
        locals.var_ibd_dn17 = assign32120_e47094_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32130_e47104, assign32130_e47104_d_n0, assign32130_e47104_d_n2, assign32130_e47104_d_n6, assign32130_e47104_d_n7, assign32130_e47104_d_n10, assign32130_e47104_d_n11, assign32130_e47104_d_n12, assign32130_e47104_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32130_e47099: f64 = (p.p178 * locals.var_vbdj);
        let assign32130_e47101: f64 = (assign32130_e47099 * locals.var_isbd2);
        let assign32130_e47102: f64 = (locals.var_ibd + assign32130_e47101);
        (assign32130_e47102, (locals.var_ibd_dn0 + (assign32130_e47099 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32130_e47099 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32130_e47099 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32130_e47099 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32130_e47099 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32130_e47099 * locals.var_isbd2_dn17)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32130_e47104;
        locals.var_ibd_dn0 = assign32130_e47104_d_n0;
        locals.var_ibd_dn2 = assign32130_e47104_d_n2;
        locals.var_ibd_dn6 = assign32130_e47104_d_n6;
        locals.var_ibd_dn7 = assign32130_e47104_d_n7;
        locals.var_ibd_dn10 = assign32130_e47104_d_n10;
        locals.var_ibd_dn11 = assign32130_e47104_d_n11;
        locals.var_ibd_dn12 = assign32130_e47104_d_n12;
        locals.var_ibd_dn17 = assign32130_e47104_d_n17;
        locals.var_ibd_rv = 0.0;

        let assign32140_e47107: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign32140_e47107;
        locals.var_guard1058_rv = 0.0;

        let (assign32150_e47116, assign32150_e47116_d_n6, assign32150_e47116_d_n7, assign32150_e47116_d_n10, assign32150_e47116_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
        let assign32150_e47113: f64 = (locals.var_vbsj / locals.var_nvtm);
        let assign32150_e47114: f64 = (assign32150_e47113).exp();
        (assign32150_e47114, 0.0, (assign32150_e47114 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32150_e47114 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32150_e47114 * (locals.var_vbsj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32150_e47116;
        locals.var_t1__blk1030_dn6 = assign32150_e47116_d_n6;
        locals.var_t1__blk1030_dn7 = assign32150_e47116_d_n7;
        locals.var_t1__blk1030_dn10 = assign32150_e47116_d_n10;
        locals.var_t1__blk1030_dn12 = assign32150_e47116_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32160_e47126, assign32160_e47126_d_n0, assign32160_e47126_d_n2, assign32160_e47126_d_n6, assign32160_e47126_d_n7, assign32160_e47126_d_n10, assign32160_e47126_d_n11, assign32160_e47126_d_n12, assign32160_e47126_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
        let assign32160_e47123: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32160_e47124: f64 = (locals.var_isbs * assign32160_e47123);
        (assign32160_e47124, (locals.var_isbs_dn0 * assign32160_e47123), (locals.var_isbs_dn2 * assign32160_e47123), ((locals.var_isbs_dn6 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn6)), ((locals.var_isbs_dn7 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn7)), ((locals.var_isbs_dn10 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn10)), (locals.var_isbs_dn11 * assign32160_e47123), ((locals.var_isbs_dn12 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn12)), (locals.var_isbs_dn17 * assign32160_e47123),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32160_e47126;
        locals.var_ibs_dn0 = assign32160_e47126_d_n0;
        locals.var_ibs_dn2 = assign32160_e47126_d_n2;
        locals.var_ibs_dn6 = assign32160_e47126_d_n6;
        locals.var_ibs_dn7 = assign32160_e47126_d_n7;
        locals.var_ibs_dn10 = assign32160_e47126_d_n10;
        locals.var_ibs_dn11 = assign32160_e47126_d_n11;
        locals.var_ibs_dn12 = assign32160_e47126_d_n12;
        locals.var_ibs_dn17 = assign32160_e47126_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32170_e47136, assign32170_e47136_d_n6, assign32170_e47136_d_n7, assign32170_e47136_d_n10, assign32170_e47136_d_n12,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
        let assign32170_e47133: f64 = (locals.var_vbst / locals.var_nvtm);
        let assign32170_e47134: f64 = (assign32170_e47133).exp();
        (assign32170_e47134, 0.0, 0.0, (assign32170_e47134 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32170_e47136;
        locals.var_t1__blk1030_dn6 = assign32170_e47136_d_n6;
        locals.var_t1__blk1030_dn7 = assign32170_e47136_d_n7;
        locals.var_t1__blk1030_dn10 = assign32170_e47136_d_n10;
        locals.var_t1__blk1030_dn12 = assign32170_e47136_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32180_e47157, assign32180_e47157_d_n0, assign32180_e47157_d_n2, assign32180_e47157_d_n6, assign32180_e47157_d_n7, assign32180_e47157_d_n10, assign32180_e47157_d_n11, assign32180_e47157_d_n12, assign32180_e47157_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
        let assign32180_e47144: f64 = (locals.var_t1__blk1030 - 1.0);
        let assign32180_e47145: f64 = (locals.var_isbs * assign32180_e47144);
        let assign32180_e47148: f64 = (locals.var_isbs / locals.var_nvtm);
        let assign32180_e47150: f64 = (assign32180_e47148 * locals.var_t1__blk1030);
        let assign32180_e47153: f64 = (locals.var_vbsj - locals.var_vbst);
        let assign32180_e47154: f64 = (assign32180_e47150 * assign32180_e47153);
        let assign32180_e47155: f64 = (assign32180_e47145 + assign32180_e47154);
        (assign32180_e47155, ((locals.var_isbs_dn0 * assign32180_e47144) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), ((locals.var_isbs_dn2 * assign32180_e47144) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn6 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn6)) * assign32180_e47153)), (((locals.var_isbs_dn7 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn7)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn10)) * assign32180_e47153) + (assign32180_e47150 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32180_e47144) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn12 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn12)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32180_e47144) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32180_e47157;
        locals.var_ibs_dn0 = assign32180_e47157_d_n0;
        locals.var_ibs_dn2 = assign32180_e47157_d_n2;
        locals.var_ibs_dn6 = assign32180_e47157_d_n6;
        locals.var_ibs_dn7 = assign32180_e47157_d_n7;
        locals.var_ibs_dn10 = assign32180_e47157_d_n10;
        locals.var_ibs_dn11 = assign32180_e47157_d_n11;
        locals.var_ibs_dn12 = assign32180_e47157_d_n12;
        locals.var_ibs_dn17 = assign32180_e47157_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32190_e47167, assign32190_e47167_d_n0, assign32190_e47167_d_n2, assign32190_e47167_d_n6, assign32190_e47167_d_n7, assign32190_e47167_d_n10, assign32190_e47167_d_n11, assign32190_e47167_d_n12, assign32190_e47167_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32190_e47162: f64 = (p.p178 * locals.var_vbsj);
        let assign32190_e47164: f64 = (assign32190_e47162 * locals.var_isbs2);
        let assign32190_e47165: f64 = (locals.var_ibs + assign32190_e47164);
        (assign32190_e47165, (locals.var_ibs_dn0 + (assign32190_e47162 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32190_e47162 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32190_e47162 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32190_e47162 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32190_e47162 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32190_e47162 * locals.var_isbs2_dn17)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32190_e47167;
        locals.var_ibs_dn0 = assign32190_e47167_d_n0;
        locals.var_ibs_dn2 = assign32190_e47167_d_n2;
        locals.var_ibs_dn6 = assign32190_e47167_d_n6;
        locals.var_ibs_dn7 = assign32190_e47167_d_n7;
        locals.var_ibs_dn10 = assign32190_e47167_d_n10;
        locals.var_ibs_dn11 = assign32190_e47167_d_n11;
        locals.var_ibs_dn12 = assign32190_e47167_d_n12;
        locals.var_ibs_dn17 = assign32190_e47167_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32200_e47175, assign32200_e47175_d_n0, assign32200_e47175_d_n2, assign32200_e47175_d_n6, assign32200_e47175_d_n7, assign32200_e47175_d_n10, assign32200_e47175_d_n11, assign32200_e47175_d_n12, assign32200_e47175_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32200_e47172: f64 = (locals.var_gjmin * locals.var_vbdj);
        let assign32200_e47173: f64 = (locals.var_ibd + assign32200_e47172);
        (assign32200_e47173, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32200_e47175;
        locals.var_ibd_dn0 = assign32200_e47175_d_n0;
        locals.var_ibd_dn2 = assign32200_e47175_d_n2;
        locals.var_ibd_dn6 = assign32200_e47175_d_n6;
        locals.var_ibd_dn7 = assign32200_e47175_d_n7;
        locals.var_ibd_dn10 = assign32200_e47175_d_n10;
        locals.var_ibd_dn11 = assign32200_e47175_d_n11;
        locals.var_ibd_dn12 = assign32200_e47175_d_n12;
        locals.var_ibd_dn17 = assign32200_e47175_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32210_e47183, assign32210_e47183_d_n0, assign32210_e47183_d_n2, assign32210_e47183_d_n6, assign32210_e47183_d_n7, assign32210_e47183_d_n10, assign32210_e47183_d_n11, assign32210_e47183_d_n12, assign32210_e47183_d_n17,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32210_e47180: f64 = (locals.var_gjmin * locals.var_vbsj);
        let assign32210_e47181: f64 = (locals.var_ibs + assign32210_e47180);
        (assign32210_e47181, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32210_e47183;
        locals.var_ibs_dn0 = assign32210_e47183_d_n0;
        locals.var_ibs_dn2 = assign32210_e47183_d_n2;
        locals.var_ibs_dn6 = assign32210_e47183_d_n6;
        locals.var_ibs_dn7 = assign32210_e47183_d_n7;
        locals.var_ibs_dn10 = assign32210_e47183_d_n10;
        locals.var_ibs_dn11 = assign32210_e47183_d_n11;
        locals.var_ibs_dn12 = assign32210_e47183_d_n12;
        locals.var_ibs_dn17 = assign32210_e47183_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32220_e47189,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32220_e47187: f64 = (p.p179 * p.p2);
        (assign32220_e47187,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32220_e47189;
        locals.var_czbd_rv = 0.0;

        let (assign32230_e47195,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32230_e47193: f64 = (p.p179 * p.p3);
        (assign32230_e47193,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32230_e47195;
        locals.var_czbs_rv = 0.0;

        let (assign32240_e47201,) = {
    if (locals.var_guard1028 != 0.0) {
        let assign32240_e47199: f64 = (p.p237 - p.p238);
        (assign32240_e47199,)
    } else {
        (locals.var_xp_max,)
    }
};
        locals.var_xp_max = assign32240_e47201;
        locals.var_xp_max_rv = 0.0;

        let assign32250_e47204: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32250_e47204;
        locals.var_guard1059_rv = 0.0;

        let (assign32260_e47210,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32260_e47210;
        locals.var_czbd_rv = 0.0;

        let (assign32270_e47216,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32270_e47216;
        locals.var_czbs_rv = 0.0;

        let assign32280_e47219: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32280_e47219;
        locals.var_guard1060_rv = 0.0;

        let (assign32290_e47229,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32290_e47226: f64 = (p.p5 - locals.var_w_dioscv);
        let assign32290_e47227: f64 = (p.p180 * assign32290_e47226);
        (assign32290_e47227,)
    } else {
        (locals.var_czbssw,)
    }
};
        locals.var_czbssw = assign32290_e47229;
        locals.var_czbssw_rv = 0.0;

        let (assign32300_e47237,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
        let assign32300_e47235: f64 = (p.p181 * locals.var_w_dioscv);
        (assign32300_e47235,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32300_e47237;
        locals.var_czbsswg_rv = 0.0;

        let assign32310_e47240: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32310_e47240;
        locals.var_guard1061_rv = 0.0;

        let assign32320_e47243: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32320_e47243;
        locals.var_guard1062_rv = 0.0;

        let (assign32330_e47257, assign32330_e47257_d_n6, assign32330_e47257_d_n7, assign32330_e47257_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign32330_e47254: f64 = (locals.var_vbsj / p.p185);
        let assign32330_e47255: f64 = (1.0 - assign32330_e47254);
        (assign32330_e47255, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32330_e47257;
        locals.var_arg__blk1055_dn6 = assign32330_e47257_d_n6;
        locals.var_arg__blk1055_dn7 = assign32330_e47257_d_n7;
        locals.var_arg__blk1055_dn12 = assign32330_e47257_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32340_e47260: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32340_e47260;
        locals.var_guard1063_rv = 0.0;

        let (assign32350_e47275, assign32350_e47275_d_n6, assign32350_e47275_d_n7, assign32350_e47275_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
        let assign32350_e47272: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32350_e47273: f64 = (1.0 / assign32350_e47272);
        (assign32350_e47273, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32350_e47275;
        locals.var_sarg_dn6 = assign32350_e47275_d_n6;
        locals.var_sarg_dn7 = assign32350_e47275_d_n7;
        locals.var_sarg_dn12 = assign32350_e47275_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32360_e47291, assign32360_e47291_d_n6, assign32360_e47291_d_n7, assign32360_e47291_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let assign32360_e47288: f64 = (-p.p182);
        let assign32360_e47289: f64 = (locals.var_arg__blk1055).powf(assign32360_e47288);
        (assign32360_e47289, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32360_e47291;
        locals.var_sarg_dn6 = assign32360_e47291_d_n6;
        locals.var_sarg_dn7 = assign32360_e47291_d_n7;
        locals.var_sarg_dn12 = assign32360_e47291_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32370_e47313, assign32370_e47313_d_n0, assign32370_e47313_d_n2, assign32370_e47313_d_n6, assign32370_e47313_d_n7, assign32370_e47313_d_n10, assign32370_e47313_d_n11, assign32370_e47313_d_n12, assign32370_e47313_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign32370_e47301: f64 = (p.p185 * locals.var_czbs);
        let assign32370_e47305: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32370_e47306: f64 = (1.0 - assign32370_e47305);
        let assign32370_e47307: f64 = (assign32370_e47301 * assign32370_e47306);
        let assign32370_e47310: f64 = (1.0 - p.p182);
        let assign32370_e47311: f64 = (assign32370_e47307 / assign32370_e47310);
        (assign32370_e47311, 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32370_e47310), ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32370_e47310), 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32370_e47310), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32370_e47313;
        locals.var_qbs_dn0 = assign32370_e47313_d_n0;
        locals.var_qbs_dn2 = assign32370_e47313_d_n2;
        locals.var_qbs_dn6 = assign32370_e47313_d_n6;
        locals.var_qbs_dn7 = assign32370_e47313_d_n7;
        locals.var_qbs_dn10 = assign32370_e47313_d_n10;
        locals.var_qbs_dn11 = assign32370_e47313_d_n11;
        locals.var_qbs_dn12 = assign32370_e47313_d_n12;
        locals.var_qbs_dn17 = assign32370_e47313_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32380_e47324, assign32380_e47324_d_n0, assign32380_e47324_d_n2, assign32380_e47324_d_n6, assign32380_e47324_d_n7, assign32380_e47324_d_n10, assign32380_e47324_d_n11, assign32380_e47324_d_n12, assign32380_e47324_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32380_e47324;
        locals.var_qbs_dn0 = assign32380_e47324_d_n0;
        locals.var_qbs_dn2 = assign32380_e47324_d_n2;
        locals.var_qbs_dn6 = assign32380_e47324_d_n6;
        locals.var_qbs_dn7 = assign32380_e47324_d_n7;
        locals.var_qbs_dn10 = assign32380_e47324_d_n10;
        locals.var_qbs_dn11 = assign32380_e47324_d_n11;
        locals.var_qbs_dn12 = assign32380_e47324_d_n12;
        locals.var_qbs_dn17 = assign32380_e47324_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32390_e47327: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32390_e47327;
        locals.var_guard1064_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32400_e47341, assign32400_e47341_d_n6, assign32400_e47341_d_n7, assign32400_e47341_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32400_e47338: f64 = (locals.var_vbsj / p.p186);
        let assign32400_e47339: f64 = (1.0 - assign32400_e47338);
        (assign32400_e47339, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32400_e47341;
        locals.var_arg__blk1055_dn6 = assign32400_e47341_d_n6;
        locals.var_arg__blk1055_dn7 = assign32400_e47341_d_n7;
        locals.var_arg__blk1055_dn12 = assign32400_e47341_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32410_e47344: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32410_e47344;
        locals.var_guard1065_rv = 0.0;

        let (assign32420_e47359, assign32420_e47359_d_n6, assign32420_e47359_d_n7, assign32420_e47359_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        let assign32420_e47356: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32420_e47357: f64 = (1.0 / assign32420_e47356);
        (assign32420_e47357, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32420_e47359;
        locals.var_sarg_dn6 = assign32420_e47359_d_n6;
        locals.var_sarg_dn7 = assign32420_e47359_d_n7;
        locals.var_sarg_dn12 = assign32420_e47359_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32430_e47375, assign32430_e47375_d_n6, assign32430_e47375_d_n7, assign32430_e47375_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32430_e47372: f64 = (-p.p183);
        let assign32430_e47373: f64 = (locals.var_arg__blk1055).powf(assign32430_e47372);
        (assign32430_e47373, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32430_e47375;
        locals.var_sarg_dn6 = assign32430_e47375_d_n6;
        locals.var_sarg_dn7 = assign32430_e47375_d_n7;
        locals.var_sarg_dn12 = assign32430_e47375_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32440_e47399, assign32440_e47399_d_n0, assign32440_e47399_d_n2, assign32440_e47399_d_n6, assign32440_e47399_d_n7, assign32440_e47399_d_n10, assign32440_e47399_d_n11, assign32440_e47399_d_n12, assign32440_e47399_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign32440_e47386: f64 = (p.p186 * locals.var_czbssw);
        let assign32440_e47390: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32440_e47391: f64 = (1.0 - assign32440_e47390);
        let assign32440_e47392: f64 = (assign32440_e47386 * assign32440_e47391);
        let assign32440_e47395: f64 = (1.0 - p.p183);
        let assign32440_e47396: f64 = (assign32440_e47392 / assign32440_e47395);
        let assign32440_e47397: f64 = (locals.var_qbs + assign32440_e47396);
        (assign32440_e47397, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32440_e47395)), (locals.var_qbs_dn7 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32440_e47395)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32440_e47395)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32440_e47399;
        locals.var_qbs_dn0 = assign32440_e47399_d_n0;
        locals.var_qbs_dn2 = assign32440_e47399_d_n2;
        locals.var_qbs_dn6 = assign32440_e47399_d_n6;
        locals.var_qbs_dn7 = assign32440_e47399_d_n7;
        locals.var_qbs_dn10 = assign32440_e47399_d_n10;
        locals.var_qbs_dn11 = assign32440_e47399_d_n11;
        locals.var_qbs_dn12 = assign32440_e47399_d_n12;
        locals.var_qbs_dn17 = assign32440_e47399_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32450_e47402: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32450_e47402;
        locals.var_guard1066_rv = 0.0;

        let (assign32460_e47416, assign32460_e47416_d_n6, assign32460_e47416_d_n7, assign32460_e47416_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32460_e47413: f64 = (locals.var_vbsj / p.p187);
        let assign32460_e47414: f64 = (1.0 - assign32460_e47413);
        (assign32460_e47414, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32460_e47416;
        locals.var_arg__blk1055_dn6 = assign32460_e47416_d_n6;
        locals.var_arg__blk1055_dn7 = assign32460_e47416_d_n7;
        locals.var_arg__blk1055_dn12 = assign32460_e47416_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32470_e47419: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32470_e47419;
        locals.var_guard1067_rv = 0.0;

        let (assign32480_e47434, assign32480_e47434_d_n6, assign32480_e47434_d_n7, assign32480_e47434_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign32480_e47431: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32480_e47432: f64 = (1.0 / assign32480_e47431);
        (assign32480_e47432, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32480_e47434;
        locals.var_sarg_dn6 = assign32480_e47434_d_n6;
        locals.var_sarg_dn7 = assign32480_e47434_d_n7;
        locals.var_sarg_dn12 = assign32480_e47434_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32490_e47450, assign32490_e47450_d_n6, assign32490_e47450_d_n7, assign32490_e47450_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
        let assign32490_e47447: f64 = (-p.p184);
        let assign32490_e47448: f64 = (locals.var_arg__blk1055).powf(assign32490_e47447);
        (assign32490_e47448, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32490_e47450;
        locals.var_sarg_dn6 = assign32490_e47450_d_n6;
        locals.var_sarg_dn7 = assign32490_e47450_d_n7;
        locals.var_sarg_dn12 = assign32490_e47450_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32500_e47474, assign32500_e47474_d_n0, assign32500_e47474_d_n2, assign32500_e47474_d_n6, assign32500_e47474_d_n7, assign32500_e47474_d_n10, assign32500_e47474_d_n11, assign32500_e47474_d_n12, assign32500_e47474_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32500_e47461: f64 = (p.p187 * locals.var_czbsswg);
        let assign32500_e47465: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32500_e47466: f64 = (1.0 - assign32500_e47465);
        let assign32500_e47467: f64 = (assign32500_e47461 * assign32500_e47466);
        let assign32500_e47470: f64 = (1.0 - p.p184);
        let assign32500_e47471: f64 = (assign32500_e47467 / assign32500_e47470);
        let assign32500_e47472: f64 = (locals.var_qbs + assign32500_e47471);
        (assign32500_e47472, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32500_e47470)), (locals.var_qbs_dn7 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32500_e47470)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32500_e47470)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32500_e47474;
        locals.var_qbs_dn0 = assign32500_e47474_d_n0;
        locals.var_qbs_dn2 = assign32500_e47474_d_n2;
        locals.var_qbs_dn6 = assign32500_e47474_d_n6;
        locals.var_qbs_dn7 = assign32500_e47474_d_n7;
        locals.var_qbs_dn10 = assign32500_e47474_d_n10;
        locals.var_qbs_dn11 = assign32500_e47474_d_n11;
        locals.var_qbs_dn12 = assign32500_e47474_d_n12;
        locals.var_qbs_dn17 = assign32500_e47474_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32510_e47487, assign32510_e47487_d_n6, assign32510_e47487_d_n7, assign32510_e47487_d_n10, assign32510_e47487_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32510_e47483: f64 = (locals.var_czbs + locals.var_czbssw);
        let assign32510_e47485: f64 = (assign32510_e47483 + locals.var_czbsswg);
        (assign32510_e47485, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32510_e47487;
        locals.var_t1__blk1030_dn6 = assign32510_e47487_d_n6;
        locals.var_t1__blk1030_dn7 = assign32510_e47487_d_n7;
        locals.var_t1__blk1030_dn10 = assign32510_e47487_d_n10;
        locals.var_t1__blk1030_dn12 = assign32510_e47487_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32520_e47512, assign32520_e47512_d_n0, assign32520_e47512_d_n2, assign32520_e47512_d_n6, assign32520_e47512_d_n7, assign32520_e47512_d_n10, assign32520_e47512_d_n11, assign32520_e47512_d_n12, assign32520_e47512_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32520_e47496: f64 = (locals.var_czbs * p.p182);
        let assign32520_e47498: f64 = (assign32520_e47496 / p.p185);
        let assign32520_e47501: f64 = (locals.var_czbssw * p.p183);
        let assign32520_e47503: f64 = (assign32520_e47501 / p.p186);
        let assign32520_e47504: f64 = (assign32520_e47498 + assign32520_e47503);
        let assign32520_e47507: f64 = (locals.var_czbsswg * p.p184);
        let assign32520_e47509: f64 = (assign32520_e47507 / p.p187);
        let assign32520_e47510: f64 = (assign32520_e47504 + assign32520_e47509);
        (assign32520_e47510, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32520_e47512;
        locals.var_t2__blk1031_dn0 = assign32520_e47512_d_n0;
        locals.var_t2__blk1031_dn2 = assign32520_e47512_d_n2;
        locals.var_t2__blk1031_dn6 = assign32520_e47512_d_n6;
        locals.var_t2__blk1031_dn7 = assign32520_e47512_d_n7;
        locals.var_t2__blk1031_dn10 = assign32520_e47512_d_n10;
        locals.var_t2__blk1031_dn11 = assign32520_e47512_d_n11;
        locals.var_t2__blk1031_dn12 = assign32520_e47512_d_n12;
        locals.var_t2__blk1031_dn17 = assign32520_e47512_d_n17;
        locals.var_t2__blk1031_rv = 0.0;

        let (assign32530_e47529, assign32530_e47529_d_n0, assign32530_e47529_d_n2, assign32530_e47529_d_n6, assign32530_e47529_d_n7, assign32530_e47529_d_n10, assign32530_e47529_d_n11, assign32530_e47529_d_n12, assign32530_e47529_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
        let assign32530_e47523: f64 = (locals.var_vbsj * 0.5);
        let assign32530_e47525: f64 = (assign32530_e47523 * locals.var_t2__blk1031);
        let assign32530_e47526: f64 = (locals.var_t1__blk1030 + assign32530_e47525);
        let assign32530_e47527: f64 = (locals.var_vbsj * assign32530_e47526);
        (assign32530_e47527, (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32530_e47523 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32530_e47523 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32530_e47529;
        locals.var_qbs_dn0 = assign32530_e47529_d_n0;
        locals.var_qbs_dn2 = assign32530_e47529_d_n2;
        locals.var_qbs_dn6 = assign32530_e47529_d_n6;
        locals.var_qbs_dn7 = assign32530_e47529_d_n7;
        locals.var_qbs_dn10 = assign32530_e47529_d_n10;
        locals.var_qbs_dn11 = assign32530_e47529_d_n11;
        locals.var_qbs_dn12 = assign32530_e47529_d_n12;
        locals.var_qbs_dn17 = assign32530_e47529_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32540_e47538,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) {
        let assign32540_e47536: f64 = (p.p181 * p.p5);
        (assign32540_e47536,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32540_e47538;
        locals.var_czbsswg_rv = 0.0;

        let assign32550_e47541: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32550_e47541;
        locals.var_guard1068_rv = 0.0;

        let assign32560_e47544: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32560_e47544;
        locals.var_guard1069_rv = 0.0;

        let (assign32570_e47559, assign32570_e47559_d_n6, assign32570_e47559_d_n7, assign32570_e47559_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32570_e47556: f64 = (locals.var_vbsj / p.p185);
        let assign32570_e47557: f64 = (1.0 - assign32570_e47556);
        (assign32570_e47557, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32570_e47559;
        locals.var_arg__blk1055_dn6 = assign32570_e47559_d_n6;
        locals.var_arg__blk1055_dn7 = assign32570_e47559_d_n7;
        locals.var_arg__blk1055_dn12 = assign32570_e47559_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32580_e47562: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32580_e47562;
        locals.var_guard1070_rv = 0.0;

        let (assign32590_e47578, assign32590_e47578_d_n6, assign32590_e47578_d_n7, assign32590_e47578_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 != 0.0)) {
        let assign32590_e47575: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32590_e47576: f64 = (1.0 / assign32590_e47575);
        (assign32590_e47576, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32590_e47578;
        locals.var_sarg_dn6 = assign32590_e47578_d_n6;
        locals.var_sarg_dn7 = assign32590_e47578_d_n7;
        locals.var_sarg_dn12 = assign32590_e47578_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32600_e47595, assign32600_e47595_d_n6, assign32600_e47595_d_n7, assign32600_e47595_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) {
        let assign32600_e47592: f64 = (-p.p182);
        let assign32600_e47593: f64 = (locals.var_arg__blk1055).powf(assign32600_e47592);
        (assign32600_e47593, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32600_e47595;
        locals.var_sarg_dn6 = assign32600_e47595_d_n6;
        locals.var_sarg_dn7 = assign32600_e47595_d_n7;
        locals.var_sarg_dn12 = assign32600_e47595_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32610_e47618, assign32610_e47618_d_n0, assign32610_e47618_d_n2, assign32610_e47618_d_n6, assign32610_e47618_d_n7, assign32610_e47618_d_n10, assign32610_e47618_d_n11, assign32610_e47618_d_n12, assign32610_e47618_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32610_e47606: f64 = (p.p185 * locals.var_czbs);
        let assign32610_e47610: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32610_e47611: f64 = (1.0 - assign32610_e47610);
        let assign32610_e47612: f64 = (assign32610_e47606 * assign32610_e47611);
        let assign32610_e47615: f64 = (1.0 - p.p182);
        let assign32610_e47616: f64 = (assign32610_e47612 / assign32610_e47615);
        (assign32610_e47616, 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32610_e47615), ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32610_e47615), 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32610_e47615), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32610_e47618;
        locals.var_qbs_dn0 = assign32610_e47618_d_n0;
        locals.var_qbs_dn2 = assign32610_e47618_d_n2;
        locals.var_qbs_dn6 = assign32610_e47618_d_n6;
        locals.var_qbs_dn7 = assign32610_e47618_d_n7;
        locals.var_qbs_dn10 = assign32610_e47618_d_n10;
        locals.var_qbs_dn11 = assign32610_e47618_d_n11;
        locals.var_qbs_dn12 = assign32610_e47618_d_n12;
        locals.var_qbs_dn17 = assign32610_e47618_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32620_e47630, assign32620_e47630_d_n0, assign32620_e47630_d_n2, assign32620_e47630_d_n6, assign32620_e47630_d_n7, assign32620_e47630_d_n10, assign32620_e47630_d_n11, assign32620_e47630_d_n12, assign32620_e47630_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32620_e47630;
        locals.var_qbs_dn0 = assign32620_e47630_d_n0;
        locals.var_qbs_dn2 = assign32620_e47630_d_n2;
        locals.var_qbs_dn6 = assign32620_e47630_d_n6;
        locals.var_qbs_dn7 = assign32620_e47630_d_n7;
        locals.var_qbs_dn10 = assign32620_e47630_d_n10;
        locals.var_qbs_dn11 = assign32620_e47630_d_n11;
        locals.var_qbs_dn12 = assign32620_e47630_d_n12;
        locals.var_qbs_dn17 = assign32620_e47630_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32630_e47633: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32630_e47633;
        locals.var_guard1071_rv = 0.0;

        let (assign32640_e47648, assign32640_e47648_d_n6, assign32640_e47648_d_n7, assign32640_e47648_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32640_e47645: f64 = (locals.var_vbsj / p.p187);
        let assign32640_e47646: f64 = (1.0 - assign32640_e47645);
        (assign32640_e47646, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32640_e47648;
        locals.var_arg__blk1055_dn6 = assign32640_e47648_d_n6;
        locals.var_arg__blk1055_dn7 = assign32640_e47648_d_n7;
        locals.var_arg__blk1055_dn12 = assign32640_e47648_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32650_e47651: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32650_e47651;
        locals.var_guard1072_rv = 0.0;

        let (assign32660_e47667, assign32660_e47667_d_n6, assign32660_e47667_d_n7, assign32660_e47667_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
        let assign32660_e47664: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32660_e47665: f64 = (1.0 / assign32660_e47664);
        (assign32660_e47665, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32660_e47667;
        locals.var_sarg_dn6 = assign32660_e47667_d_n6;
        locals.var_sarg_dn7 = assign32660_e47667_d_n7;
        locals.var_sarg_dn12 = assign32660_e47667_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32670_e47684, assign32670_e47684_d_n6, assign32670_e47684_d_n7, assign32670_e47684_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32670_e47681: f64 = (-p.p184);
        let assign32670_e47682: f64 = (locals.var_arg__blk1055).powf(assign32670_e47681);
        (assign32670_e47682, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32670_e47684;
        locals.var_sarg_dn6 = assign32670_e47684_d_n6;
        locals.var_sarg_dn7 = assign32670_e47684_d_n7;
        locals.var_sarg_dn12 = assign32670_e47684_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32680_e47709, assign32680_e47709_d_n0, assign32680_e47709_d_n2, assign32680_e47709_d_n6, assign32680_e47709_d_n7, assign32680_e47709_d_n10, assign32680_e47709_d_n11, assign32680_e47709_d_n12, assign32680_e47709_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32680_e47696: f64 = (p.p187 * locals.var_czbsswg);
        let assign32680_e47700: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32680_e47701: f64 = (1.0 - assign32680_e47700);
        let assign32680_e47702: f64 = (assign32680_e47696 * assign32680_e47701);
        let assign32680_e47705: f64 = (1.0 - p.p184);
        let assign32680_e47706: f64 = (assign32680_e47702 / assign32680_e47705);
        let assign32680_e47707: f64 = (locals.var_qbs + assign32680_e47706);
        (assign32680_e47707, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32680_e47705)), (locals.var_qbs_dn7 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32680_e47705)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32680_e47705)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32680_e47709;
        locals.var_qbs_dn0 = assign32680_e47709_d_n0;
        locals.var_qbs_dn2 = assign32680_e47709_d_n2;
        locals.var_qbs_dn6 = assign32680_e47709_d_n6;
        locals.var_qbs_dn7 = assign32680_e47709_d_n7;
        locals.var_qbs_dn10 = assign32680_e47709_d_n10;
        locals.var_qbs_dn11 = assign32680_e47709_d_n11;
        locals.var_qbs_dn12 = assign32680_e47709_d_n12;
        locals.var_qbs_dn17 = assign32680_e47709_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32690_e47721, assign32690_e47721_d_n6, assign32690_e47721_d_n7, assign32690_e47721_d_n10, assign32690_e47721_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32690_e47719: f64 = (locals.var_czbs + locals.var_czbsswg);
        (assign32690_e47719, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32690_e47721;
        locals.var_t1__blk1030_dn6 = assign32690_e47721_d_n6;
        locals.var_t1__blk1030_dn7 = assign32690_e47721_d_n7;
        locals.var_t1__blk1030_dn10 = assign32690_e47721_d_n10;
        locals.var_t1__blk1030_dn12 = assign32690_e47721_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32700_e47741, assign32700_e47741_d_n0, assign32700_e47741_d_n2, assign32700_e47741_d_n6, assign32700_e47741_d_n7, assign32700_e47741_d_n10, assign32700_e47741_d_n11, assign32700_e47741_d_n12, assign32700_e47741_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32700_e47731: f64 = (locals.var_czbs * p.p182);
        let assign32700_e47733: f64 = (assign32700_e47731 / p.p185);
        let assign32700_e47736: f64 = (locals.var_czbsswg * p.p184);
        let assign32700_e47738: f64 = (assign32700_e47736 / p.p187);
        let assign32700_e47739: f64 = (assign32700_e47733 + assign32700_e47738);
        (assign32700_e47739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32700_e47741;
        locals.var_t2__blk1031_dn0 = assign32700_e47741_d_n0;
        locals.var_t2__blk1031_dn2 = assign32700_e47741_d_n2;
        locals.var_t2__blk1031_dn6 = assign32700_e47741_d_n6;
        locals.var_t2__blk1031_dn7 = assign32700_e47741_d_n7;
        locals.var_t2__blk1031_dn10 = assign32700_e47741_d_n10;
        locals.var_t2__blk1031_dn11 = assign32700_e47741_d_n11;
        locals.var_t2__blk1031_dn12 = assign32700_e47741_d_n12;
        locals.var_t2__blk1031_dn17 = assign32700_e47741_d_n17;
        locals.var_t2__blk1031_rv = 0.0;

        let (assign32710_e47759, assign32710_e47759_d_n0, assign32710_e47759_d_n2, assign32710_e47759_d_n6, assign32710_e47759_d_n7, assign32710_e47759_d_n10, assign32710_e47759_d_n11, assign32710_e47759_d_n12, assign32710_e47759_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
        let assign32710_e47753: f64 = (locals.var_vbsj * 0.5);
        let assign32710_e47755: f64 = (assign32710_e47753 * locals.var_t2__blk1031);
        let assign32710_e47756: f64 = (locals.var_t1__blk1030 + assign32710_e47755);
        let assign32710_e47757: f64 = (locals.var_vbsj * assign32710_e47756);
        (assign32710_e47757, (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32710_e47753 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32710_e47753 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32710_e47759;
        locals.var_qbs_dn0 = assign32710_e47759_d_n0;
        locals.var_qbs_dn2 = assign32710_e47759_d_n2;
        locals.var_qbs_dn6 = assign32710_e47759_d_n6;
        locals.var_qbs_dn7 = assign32710_e47759_d_n7;
        locals.var_qbs_dn10 = assign32710_e47759_d_n10;
        locals.var_qbs_dn11 = assign32710_e47759_d_n11;
        locals.var_qbs_dn12 = assign32710_e47759_d_n12;
        locals.var_qbs_dn17 = assign32710_e47759_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32720_e47762: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32720_e47762;
        locals.var_guard1073_rv = 0.0;

        let (assign32730_e47772,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
        let assign32730_e47769: f64 = (p.p4 - locals.var_w_diodcv);
        let assign32730_e47770: f64 = (p.p180 * assign32730_e47769);
        (assign32730_e47770,)
    } else {
        (locals.var_czbdsw,)
    }
};
        locals.var_czbdsw = assign32730_e47772;
        locals.var_czbdsw_rv = 0.0;

        let (assign32740_e47780,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
        let assign32740_e47778: f64 = (p.p181 * locals.var_w_diodcv);
        (assign32740_e47778,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32740_e47780;
        locals.var_czbdswg_rv = 0.0;

        let assign32750_e47783: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32750_e47783;
        locals.var_guard1074_rv = 0.0;

        let assign32760_e47786: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32760_e47786;
        locals.var_guard1075_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32770_e47800, assign32770_e47800_d_n6, assign32770_e47800_d_n7, assign32770_e47800_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32770_e47797: f64 = (locals.var_vbdj / p.p185);
        let assign32770_e47798: f64 = (1.0 - assign32770_e47797);
        (assign32770_e47798, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32770_e47800;
        locals.var_arg__blk1055_dn6 = assign32770_e47800_d_n6;
        locals.var_arg__blk1055_dn7 = assign32770_e47800_d_n7;
        locals.var_arg__blk1055_dn12 = assign32770_e47800_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32780_e47803: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32780_e47803;
        locals.var_guard1076_rv = 0.0;

        let (assign32790_e47818, assign32790_e47818_d_n6, assign32790_e47818_d_n7, assign32790_e47818_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
        let assign32790_e47815: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32790_e47816: f64 = (1.0 / assign32790_e47815);
        (assign32790_e47816, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32790_e47818;
        locals.var_sarg_dn6 = assign32790_e47818_d_n6;
        locals.var_sarg_dn7 = assign32790_e47818_d_n7;
        locals.var_sarg_dn12 = assign32790_e47818_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32800_e47834, assign32800_e47834_d_n6, assign32800_e47834_d_n7, assign32800_e47834_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32800_e47831: f64 = (-p.p182);
        let assign32800_e47832: f64 = (locals.var_arg__blk1055).powf(assign32800_e47831);
        (assign32800_e47832, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32800_e47834;
        locals.var_sarg_dn6 = assign32800_e47834_d_n6;
        locals.var_sarg_dn7 = assign32800_e47834_d_n7;
        locals.var_sarg_dn12 = assign32800_e47834_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32810_e47856, assign32810_e47856_d_n0, assign32810_e47856_d_n2, assign32810_e47856_d_n6, assign32810_e47856_d_n7, assign32810_e47856_d_n10, assign32810_e47856_d_n11, assign32810_e47856_d_n12, assign32810_e47856_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32810_e47844: f64 = (p.p185 * locals.var_czbd);
        let assign32810_e47848: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32810_e47849: f64 = (1.0 - assign32810_e47848);
        let assign32810_e47850: f64 = (assign32810_e47844 * assign32810_e47849);
        let assign32810_e47853: f64 = (1.0 - p.p182);
        let assign32810_e47854: f64 = (assign32810_e47850 / assign32810_e47853);
        (assign32810_e47854, 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32810_e47853), ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32810_e47853), 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32810_e47853), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32810_e47856;
        locals.var_qbd_dn0 = assign32810_e47856_d_n0;
        locals.var_qbd_dn2 = assign32810_e47856_d_n2;
        locals.var_qbd_dn6 = assign32810_e47856_d_n6;
        locals.var_qbd_dn7 = assign32810_e47856_d_n7;
        locals.var_qbd_dn10 = assign32810_e47856_d_n10;
        locals.var_qbd_dn11 = assign32810_e47856_d_n11;
        locals.var_qbd_dn12 = assign32810_e47856_d_n12;
        locals.var_qbd_dn17 = assign32810_e47856_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign32820_e47867, assign32820_e47867_d_n0, assign32820_e47867_d_n2, assign32820_e47867_d_n6, assign32820_e47867_d_n7, assign32820_e47867_d_n10, assign32820_e47867_d_n11, assign32820_e47867_d_n12, assign32820_e47867_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32820_e47867;
        locals.var_qbd_dn0 = assign32820_e47867_d_n0;
        locals.var_qbd_dn2 = assign32820_e47867_d_n2;
        locals.var_qbd_dn6 = assign32820_e47867_d_n6;
        locals.var_qbd_dn7 = assign32820_e47867_d_n7;
        locals.var_qbd_dn10 = assign32820_e47867_d_n10;
        locals.var_qbd_dn11 = assign32820_e47867_d_n11;
        locals.var_qbd_dn12 = assign32820_e47867_d_n12;
        locals.var_qbd_dn17 = assign32820_e47867_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign32830_e47870: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32830_e47870;
        locals.var_guard1077_rv = 0.0;

        let (assign32840_e47884, assign32840_e47884_d_n6, assign32840_e47884_d_n7, assign32840_e47884_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32840_e47881: f64 = (locals.var_vbdj / p.p186);
        let assign32840_e47882: f64 = (1.0 - assign32840_e47881);
        (assign32840_e47882, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32840_e47884;
        locals.var_arg__blk1055_dn6 = assign32840_e47884_d_n6;
        locals.var_arg__blk1055_dn7 = assign32840_e47884_d_n7;
        locals.var_arg__blk1055_dn12 = assign32840_e47884_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32850_e47887: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32850_e47887;
        locals.var_guard1078_rv = 0.0;

        let (assign32860_e47902, assign32860_e47902_d_n6, assign32860_e47902_d_n7, assign32860_e47902_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
        let assign32860_e47899: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32860_e47900: f64 = (1.0 / assign32860_e47899);
        (assign32860_e47900, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32860_e47902;
        locals.var_sarg_dn6 = assign32860_e47902_d_n6;
        locals.var_sarg_dn7 = assign32860_e47902_d_n7;
        locals.var_sarg_dn12 = assign32860_e47902_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32870_e47918, assign32870_e47918_d_n6, assign32870_e47918_d_n7, assign32870_e47918_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign32870_e47915: f64 = (-p.p183);
        let assign32870_e47916: f64 = (locals.var_arg__blk1055).powf(assign32870_e47915);
        (assign32870_e47916, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32870_e47918;
        locals.var_sarg_dn6 = assign32870_e47918_d_n6;
        locals.var_sarg_dn7 = assign32870_e47918_d_n7;
        locals.var_sarg_dn12 = assign32870_e47918_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32880_e47942, assign32880_e47942_d_n0, assign32880_e47942_d_n2, assign32880_e47942_d_n6, assign32880_e47942_d_n7, assign32880_e47942_d_n10, assign32880_e47942_d_n11, assign32880_e47942_d_n12, assign32880_e47942_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        let assign32880_e47929: f64 = (p.p186 * locals.var_czbdsw);
        let assign32880_e47933: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32880_e47934: f64 = (1.0 - assign32880_e47933);
        let assign32880_e47935: f64 = (assign32880_e47929 * assign32880_e47934);
        let assign32880_e47938: f64 = (1.0 - p.p183);
        let assign32880_e47939: f64 = (assign32880_e47935 / assign32880_e47938);
        let assign32880_e47940: f64 = (locals.var_qbd + assign32880_e47939);
        (assign32880_e47940, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32880_e47938)), (locals.var_qbd_dn7 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32880_e47938)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32880_e47938)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32880_e47942;
        locals.var_qbd_dn0 = assign32880_e47942_d_n0;
        locals.var_qbd_dn2 = assign32880_e47942_d_n2;
        locals.var_qbd_dn6 = assign32880_e47942_d_n6;
        locals.var_qbd_dn7 = assign32880_e47942_d_n7;
        locals.var_qbd_dn10 = assign32880_e47942_d_n10;
        locals.var_qbd_dn11 = assign32880_e47942_d_n11;
        locals.var_qbd_dn12 = assign32880_e47942_d_n12;
        locals.var_qbd_dn17 = assign32880_e47942_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign32890_e47945: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32890_e47945;
        locals.var_guard1079_rv = 0.0;

        let (assign32900_e47959, assign32900_e47959_d_n6, assign32900_e47959_d_n7, assign32900_e47959_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32900_e47956: f64 = (locals.var_vbdj / p.p187);
        let assign32900_e47957: f64 = (1.0 - assign32900_e47956);
        (assign32900_e47957, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign32900_e47959;
        locals.var_arg__blk1055_dn6 = assign32900_e47959_d_n6;
        locals.var_arg__blk1055_dn7 = assign32900_e47959_d_n7;
        locals.var_arg__blk1055_dn12 = assign32900_e47959_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign32910_e47962: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32910_e47962;
        locals.var_guard1080_rv = 0.0;

        let (assign32920_e47977, assign32920_e47977_d_n6, assign32920_e47977_d_n7, assign32920_e47977_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
        let assign32920_e47974: f64 = (locals.var_arg__blk1055).sqrt();
        let assign32920_e47975: f64 = (1.0 / assign32920_e47974);
        (assign32920_e47975, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32920_e47977;
        locals.var_sarg_dn6 = assign32920_e47977_d_n6;
        locals.var_sarg_dn7 = assign32920_e47977_d_n7;
        locals.var_sarg_dn12 = assign32920_e47977_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32930_e47993, assign32930_e47993_d_n6, assign32930_e47993_d_n7, assign32930_e47993_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
        let assign32930_e47990: f64 = (-p.p184);
        let assign32930_e47991: f64 = (locals.var_arg__blk1055).powf(assign32930_e47990);
        (assign32930_e47991, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32930_e47993;
        locals.var_sarg_dn6 = assign32930_e47993_d_n6;
        locals.var_sarg_dn7 = assign32930_e47993_d_n7;
        locals.var_sarg_dn12 = assign32930_e47993_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32940_e48017, assign32940_e48017_d_n0, assign32940_e48017_d_n2, assign32940_e48017_d_n6, assign32940_e48017_d_n7, assign32940_e48017_d_n10, assign32940_e48017_d_n11, assign32940_e48017_d_n12, assign32940_e48017_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32940_e48004: f64 = (p.p187 * locals.var_czbdswg);
        let assign32940_e48008: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign32940_e48009: f64 = (1.0 - assign32940_e48008);
        let assign32940_e48010: f64 = (assign32940_e48004 * assign32940_e48009);
        let assign32940_e48013: f64 = (1.0 - p.p184);
        let assign32940_e48014: f64 = (assign32940_e48010 / assign32940_e48013);
        let assign32940_e48015: f64 = (locals.var_qbd + assign32940_e48014);
        (assign32940_e48015, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32940_e48013)), (locals.var_qbd_dn7 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32940_e48013)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32940_e48013)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32940_e48017;
        locals.var_qbd_dn0 = assign32940_e48017_d_n0;
        locals.var_qbd_dn2 = assign32940_e48017_d_n2;
        locals.var_qbd_dn6 = assign32940_e48017_d_n6;
        locals.var_qbd_dn7 = assign32940_e48017_d_n7;
        locals.var_qbd_dn10 = assign32940_e48017_d_n10;
        locals.var_qbd_dn11 = assign32940_e48017_d_n11;
        locals.var_qbd_dn12 = assign32940_e48017_d_n12;
        locals.var_qbd_dn17 = assign32940_e48017_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign32950_e48030, assign32950_e48030_d_n6, assign32950_e48030_d_n7, assign32950_e48030_d_n10, assign32950_e48030_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32950_e48026: f64 = (locals.var_czbd + locals.var_czbdsw);
        let assign32950_e48028: f64 = (assign32950_e48026 + locals.var_czbdswg);
        (assign32950_e48028, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign32950_e48030;
        locals.var_t1__blk1030_dn6 = assign32950_e48030_d_n6;
        locals.var_t1__blk1030_dn7 = assign32950_e48030_d_n7;
        locals.var_t1__blk1030_dn10 = assign32950_e48030_d_n10;
        locals.var_t1__blk1030_dn12 = assign32950_e48030_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign32960_e48055, assign32960_e48055_d_n0, assign32960_e48055_d_n2, assign32960_e48055_d_n6, assign32960_e48055_d_n7, assign32960_e48055_d_n10, assign32960_e48055_d_n11, assign32960_e48055_d_n12, assign32960_e48055_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32960_e48039: f64 = (locals.var_czbd * p.p182);
        let assign32960_e48041: f64 = (assign32960_e48039 / p.p185);
        let assign32960_e48044: f64 = (locals.var_czbdsw * p.p183);
        let assign32960_e48046: f64 = (assign32960_e48044 / p.p186);
        let assign32960_e48047: f64 = (assign32960_e48041 + assign32960_e48046);
        let assign32960_e48050: f64 = (locals.var_czbdswg * p.p184);
        let assign32960_e48052: f64 = (assign32960_e48050 / p.p187);
        let assign32960_e48053: f64 = (assign32960_e48047 + assign32960_e48052);
        (assign32960_e48053, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign32960_e48055;
        locals.var_t2__blk1031_dn0 = assign32960_e48055_d_n0;
        locals.var_t2__blk1031_dn2 = assign32960_e48055_d_n2;
        locals.var_t2__blk1031_dn6 = assign32960_e48055_d_n6;
        locals.var_t2__blk1031_dn7 = assign32960_e48055_d_n7;
        locals.var_t2__blk1031_dn10 = assign32960_e48055_d_n10;
        locals.var_t2__blk1031_dn11 = assign32960_e48055_d_n11;
        locals.var_t2__blk1031_dn12 = assign32960_e48055_d_n12;
        locals.var_t2__blk1031_dn17 = assign32960_e48055_d_n17;
        locals.var_t2__blk1031_rv = 0.0;

        let (assign32970_e48072, assign32970_e48072_d_n0, assign32970_e48072_d_n2, assign32970_e48072_d_n6, assign32970_e48072_d_n7, assign32970_e48072_d_n10, assign32970_e48072_d_n11, assign32970_e48072_d_n12, assign32970_e48072_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32970_e48066: f64 = (locals.var_vbdj * 0.5);
        let assign32970_e48068: f64 = (assign32970_e48066 * locals.var_t2__blk1031);
        let assign32970_e48069: f64 = (locals.var_t1__blk1030 + assign32970_e48068);
        let assign32970_e48070: f64 = (locals.var_vbdj * assign32970_e48069);
        (assign32970_e48070, (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign32970_e48066 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign32970_e48066 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32970_e48072;
        locals.var_qbd_dn0 = assign32970_e48072_d_n0;
        locals.var_qbd_dn2 = assign32970_e48072_d_n2;
        locals.var_qbd_dn6 = assign32970_e48072_d_n6;
        locals.var_qbd_dn7 = assign32970_e48072_d_n7;
        locals.var_qbd_dn10 = assign32970_e48072_d_n10;
        locals.var_qbd_dn11 = assign32970_e48072_d_n11;
        locals.var_qbd_dn12 = assign32970_e48072_d_n12;
        locals.var_qbd_dn17 = assign32970_e48072_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign32980_e48081,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) {
        let assign32980_e48079: f64 = (p.p181 * p.p4);
        (assign32980_e48079,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32980_e48081;
        locals.var_czbdswg_rv = 0.0;

        let assign32990_e48084: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32990_e48084;
        locals.var_guard1081_rv = 0.0;

        let assign33000_e48087: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign33000_e48087;
        locals.var_guard1082_rv = 0.0;

        let (assign33010_e48102, assign33010_e48102_d_n6, assign33010_e48102_d_n7, assign33010_e48102_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign33010_e48099: f64 = (locals.var_vbdj / p.p185);
        let assign33010_e48100: f64 = (1.0 - assign33010_e48099);
        (assign33010_e48100, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign33010_e48102;
        locals.var_arg__blk1055_dn6 = assign33010_e48102_d_n6;
        locals.var_arg__blk1055_dn7 = assign33010_e48102_d_n7;
        locals.var_arg__blk1055_dn12 = assign33010_e48102_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign33020_e48105: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33020_e48105;
        locals.var_guard1083_rv = 0.0;

        let (assign33030_e48121, assign33030_e48121_d_n6, assign33030_e48121_d_n7, assign33030_e48121_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign33030_e48118: f64 = (locals.var_arg__blk1055).sqrt();
        let assign33030_e48119: f64 = (1.0 / assign33030_e48118);
        (assign33030_e48119, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33030_e48121;
        locals.var_sarg_dn6 = assign33030_e48121_d_n6;
        locals.var_sarg_dn7 = assign33030_e48121_d_n7;
        locals.var_sarg_dn12 = assign33030_e48121_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33040_e48138, assign33040_e48138_d_n6, assign33040_e48138_d_n7, assign33040_e48138_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign33040_e48135: f64 = (-p.p182);
        let assign33040_e48136: f64 = (locals.var_arg__blk1055).powf(assign33040_e48135);
        (assign33040_e48136, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33040_e48138;
        locals.var_sarg_dn6 = assign33040_e48138_d_n6;
        locals.var_sarg_dn7 = assign33040_e48138_d_n7;
        locals.var_sarg_dn12 = assign33040_e48138_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33050_e48161, assign33050_e48161_d_n0, assign33050_e48161_d_n2, assign33050_e48161_d_n6, assign33050_e48161_d_n7, assign33050_e48161_d_n10, assign33050_e48161_d_n11, assign33050_e48161_d_n12, assign33050_e48161_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign33050_e48149: f64 = (p.p185 * locals.var_czbd);
        let assign33050_e48153: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign33050_e48154: f64 = (1.0 - assign33050_e48153);
        let assign33050_e48155: f64 = (assign33050_e48149 * assign33050_e48154);
        let assign33050_e48158: f64 = (1.0 - p.p182);
        let assign33050_e48159: f64 = (assign33050_e48155 / assign33050_e48158);
        (assign33050_e48159, 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33050_e48158), ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33050_e48158), 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33050_e48158), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33050_e48161;
        locals.var_qbd_dn0 = assign33050_e48161_d_n0;
        locals.var_qbd_dn2 = assign33050_e48161_d_n2;
        locals.var_qbd_dn6 = assign33050_e48161_d_n6;
        locals.var_qbd_dn7 = assign33050_e48161_d_n7;
        locals.var_qbd_dn10 = assign33050_e48161_d_n10;
        locals.var_qbd_dn11 = assign33050_e48161_d_n11;
        locals.var_qbd_dn12 = assign33050_e48161_d_n12;
        locals.var_qbd_dn17 = assign33050_e48161_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign33060_e48173, assign33060_e48173_d_n0, assign33060_e48173_d_n2, assign33060_e48173_d_n6, assign33060_e48173_d_n7, assign33060_e48173_d_n10, assign33060_e48173_d_n11, assign33060_e48173_d_n12, assign33060_e48173_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33060_e48173;
        locals.var_qbd_dn0 = assign33060_e48173_d_n0;
        locals.var_qbd_dn2 = assign33060_e48173_d_n2;
        locals.var_qbd_dn6 = assign33060_e48173_d_n6;
        locals.var_qbd_dn7 = assign33060_e48173_d_n7;
        locals.var_qbd_dn10 = assign33060_e48173_d_n10;
        locals.var_qbd_dn11 = assign33060_e48173_d_n11;
        locals.var_qbd_dn12 = assign33060_e48173_d_n12;
        locals.var_qbd_dn17 = assign33060_e48173_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33070_e48176: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33070_e48176;
        locals.var_guard1084_rv = 0.0;

        let (assign33080_e48191, assign33080_e48191_d_n6, assign33080_e48191_d_n7, assign33080_e48191_d_n12,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33080_e48188: f64 = (locals.var_vbdj / p.p187);
        let assign33080_e48189: f64 = (1.0 - assign33080_e48188);
        (assign33080_e48189, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12,)
    }
};
        locals.var_arg__blk1055 = assign33080_e48191;
        locals.var_arg__blk1055_dn6 = assign33080_e48191_d_n6;
        locals.var_arg__blk1055_dn7 = assign33080_e48191_d_n7;
        locals.var_arg__blk1055_dn12 = assign33080_e48191_d_n12;
        locals.var_arg__blk1055_rv = 0.0;

        let assign33090_e48194: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33090_e48194;
        locals.var_guard1085_rv = 0.0;

        let (assign33100_e48210, assign33100_e48210_d_n6, assign33100_e48210_d_n7, assign33100_e48210_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
        let assign33100_e48207: f64 = (locals.var_arg__blk1055).sqrt();
        let assign33100_e48208: f64 = (1.0 / assign33100_e48207);
        (assign33100_e48208, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33100_e48210;
        locals.var_sarg_dn6 = assign33100_e48210_d_n6;
        locals.var_sarg_dn7 = assign33100_e48210_d_n7;
        locals.var_sarg_dn12 = assign33100_e48210_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33110_e48227, assign33110_e48227_d_n6, assign33110_e48227_d_n7, assign33110_e48227_d_n12,) = {
    if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33110_e48224: f64 = (-p.p184);
        let assign33110_e48225: f64 = (locals.var_arg__blk1055).powf(assign33110_e48224);
        (assign33110_e48225, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33110_e48227;
        locals.var_sarg_dn6 = assign33110_e48227_d_n6;
        locals.var_sarg_dn7 = assign33110_e48227_d_n7;
        locals.var_sarg_dn12 = assign33110_e48227_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33120_e48252, assign33120_e48252_d_n0, assign33120_e48252_d_n2, assign33120_e48252_d_n6, assign33120_e48252_d_n7, assign33120_e48252_d_n10, assign33120_e48252_d_n11, assign33120_e48252_d_n12, assign33120_e48252_d_n17,) = {
    if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign33120_e48239: f64 = (p.p187 * locals.var_czbdswg);
        let assign33120_e48243: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
        let assign33120_e48244: f64 = (1.0 - assign33120_e48243);
        let assign33120_e48245: f64 = (assign33120_e48239 * assign33120_e48244);
        let assign33120_e48248: f64 = (1.0 - p.p184);
        let assign33120_e48249: f64 = (assign33120_e48245 / assign33120_e48248);
        let assign33120_e48250: f64 = (locals.var_qbd + assign33120_e48249);
        (assign33120_e48250, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33120_e48248)), (locals.var_qbd_dn7 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33120_e48248)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33120_e48248)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33120_e48252;
        locals.var_qbd_dn0 = assign33120_e48252_d_n0;
        locals.var_qbd_dn2 = assign33120_e48252_d_n2;
        locals.var_qbd_dn6 = assign33120_e48252_d_n6;
        locals.var_qbd_dn7 = assign33120_e48252_d_n7;
        locals.var_qbd_dn10 = assign33120_e48252_d_n10;
        locals.var_qbd_dn11 = assign33120_e48252_d_n11;
        locals.var_qbd_dn12 = assign33120_e48252_d_n12;
        locals.var_qbd_dn17 = assign33120_e48252_d_n17;
        locals.var_qbd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33130_e48264, assign33130_e48264_d_n6, assign33130_e48264_d_n7, assign33130_e48264_d_n10, assign33130_e48264_d_n12,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33130_e48262: f64 = (locals.var_czbd + locals.var_czbdswg);
        (assign33130_e48262, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12,)
    }
};
        locals.var_t1__blk1030 = assign33130_e48264;
        locals.var_t1__blk1030_dn6 = assign33130_e48264_d_n6;
        locals.var_t1__blk1030_dn7 = assign33130_e48264_d_n7;
        locals.var_t1__blk1030_dn10 = assign33130_e48264_d_n10;
        locals.var_t1__blk1030_dn12 = assign33130_e48264_d_n12;
        locals.var_t1__blk1030_rv = 0.0;

        let (assign33140_e48284, assign33140_e48284_d_n0, assign33140_e48284_d_n2, assign33140_e48284_d_n6, assign33140_e48284_d_n7, assign33140_e48284_d_n10, assign33140_e48284_d_n11, assign33140_e48284_d_n12, assign33140_e48284_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33140_e48274: f64 = (locals.var_czbd * p.p182);
        let assign33140_e48276: f64 = (assign33140_e48274 / p.p185);
        let assign33140_e48279: f64 = (locals.var_czbdswg * p.p184);
        let assign33140_e48281: f64 = (assign33140_e48279 / p.p187);
        let assign33140_e48282: f64 = (assign33140_e48276 + assign33140_e48281);
        (assign33140_e48282, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17,)
    }
};
        locals.var_t2__blk1031 = assign33140_e48284;
        locals.var_t2__blk1031_dn0 = assign33140_e48284_d_n0;
        locals.var_t2__blk1031_dn2 = assign33140_e48284_d_n2;
        locals.var_t2__blk1031_dn6 = assign33140_e48284_d_n6;
        locals.var_t2__blk1031_dn7 = assign33140_e48284_d_n7;
        locals.var_t2__blk1031_dn10 = assign33140_e48284_d_n10;
        locals.var_t2__blk1031_dn11 = assign33140_e48284_d_n11;
        locals.var_t2__blk1031_dn12 = assign33140_e48284_d_n12;
        locals.var_t2__blk1031_dn17 = assign33140_e48284_d_n17;
        locals.var_t2__blk1031_rv = 0.0;

        let (assign33150_e48302, assign33150_e48302_d_n0, assign33150_e48302_d_n2, assign33150_e48302_d_n6, assign33150_e48302_d_n7, assign33150_e48302_d_n10, assign33150_e48302_d_n11, assign33150_e48302_d_n12, assign33150_e48302_d_n17,) = {
    if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
        let assign33150_e48296: f64 = (locals.var_vbdj * 0.5);
        let assign33150_e48298: f64 = (assign33150_e48296 * locals.var_t2__blk1031);
        let assign33150_e48299: f64 = (locals.var_t1__blk1030 + assign33150_e48298);
        let assign33150_e48300: f64 = (locals.var_vbdj * assign33150_e48299);
        (assign33150_e48300, (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign33150_e48296 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign33150_e48296 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33150_e48302;
        locals.var_qbd_dn0 = assign33150_e48302_d_n0;
        locals.var_qbd_dn2 = assign33150_e48302_d_n2;
        locals.var_qbd_dn6 = assign33150_e48302_d_n6;
        locals.var_qbd_dn7 = assign33150_e48302_d_n7;
        locals.var_qbd_dn10 = assign33150_e48302_d_n10;
        locals.var_qbd_dn11 = assign33150_e48302_d_n11;
        locals.var_qbd_dn12 = assign33150_e48302_d_n12;
        locals.var_qbd_dn17 = assign33150_e48302_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33160_e48305: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33160_e48305;
        locals.var_guard1086_rv = 0.0;

        let (assign33170_e48318, assign33170_e48318_d_n0, assign33170_e48318_d_n2, assign33170_e48318_d_n6, assign33170_e48318_d_n7, assign33170_e48318_d_n10, assign33170_e48318_d_n11, assign33170_e48318_d_n12, assign33170_e48318_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33170_e48310: f64 = (-1.6021918e-19);
        let assign33170_e48312: f64 = (assign33170_e48310 * locals.var_uc_nsubs);
        let assign33170_e48314: f64 = (assign33170_e48312 * locals.var_xp_max);
        let assign33170_e48316: f64 = (assign33170_e48314 * p.p3);
        (assign33170_e48316, (((assign33170_e48310 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3),)
    } else {
        (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17,)
    }
};
        locals.var_qbs_max = assign33170_e48318;
        locals.var_qbs_max_dn0 = assign33170_e48318_d_n0;
        locals.var_qbs_max_dn2 = assign33170_e48318_d_n2;
        locals.var_qbs_max_dn6 = assign33170_e48318_d_n6;
        locals.var_qbs_max_dn7 = assign33170_e48318_d_n7;
        locals.var_qbs_max_dn10 = assign33170_e48318_d_n10;
        locals.var_qbs_max_dn11 = assign33170_e48318_d_n11;
        locals.var_qbs_max_dn12 = assign33170_e48318_d_n12;
        locals.var_qbs_max_dn17 = assign33170_e48318_d_n17;
        locals.var_qbs_max_rv = 0.0;

        let (assign33180_e48327, assign33180_e48327_d_n0, assign33180_e48327_d_n2, assign33180_e48327_d_n6, assign33180_e48327_d_n7, assign33180_e48327_d_n10, assign33180_e48327_d_n11, assign33180_e48327_d_n12, assign33180_e48327_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33180_e48324: f64 = (-locals.var_qbs_max);
        let assign33180_e48325: f64 = (0.001 * assign33180_e48324);
        (assign33180_e48325, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)),)
    } else {
        (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17,)
    }
};
        locals.var_dlt_qbs = assign33180_e48327;
        locals.var_dlt_qbs_dn0 = assign33180_e48327_d_n0;
        locals.var_dlt_qbs_dn2 = assign33180_e48327_d_n2;
        locals.var_dlt_qbs_dn6 = assign33180_e48327_d_n6;
        locals.var_dlt_qbs_dn7 = assign33180_e48327_d_n7;
        locals.var_dlt_qbs_dn10 = assign33180_e48327_d_n10;
        locals.var_dlt_qbs_dn11 = assign33180_e48327_d_n11;
        locals.var_dlt_qbs_dn12 = assign33180_e48327_d_n12;
        locals.var_dlt_qbs_dn17 = assign33180_e48327_d_n17;
        locals.var_dlt_qbs_rv = 0.0;

        let (assign33190_e48339, assign33190_e48339_d_n0, assign33190_e48339_d_n2, assign33190_e48339_d_n6, assign33190_e48339_d_n7, assign33190_e48339_d_n10, assign33190_e48339_d_n11, assign33190_e48339_d_n12, assign33190_e48339_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33190_e48332: f64 = (-locals.var_qbs_max);
        let assign33190_e48334: f64 = (-locals.var_qbs);
        let assign33190_e48335: f64 = (assign33190_e48332 - assign33190_e48334);
        let assign33190_e48337: f64 = (assign33190_e48335 - locals.var_dlt_qbs);
        (assign33190_e48337, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33190_e48339;
        locals.var_tmf1_dn0 = assign33190_e48339_d_n0;
        locals.var_tmf1_dn2 = assign33190_e48339_d_n2;
        locals.var_tmf1_dn6 = assign33190_e48339_d_n6;
        locals.var_tmf1_dn7 = assign33190_e48339_d_n7;
        locals.var_tmf1_dn10 = assign33190_e48339_d_n10;
        locals.var_tmf1_dn11 = assign33190_e48339_d_n11;
        locals.var_tmf1_dn12 = assign33190_e48339_d_n12;
        locals.var_tmf1_dn17 = assign33190_e48339_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign33200_e48350, assign33200_e48350_d_n0, assign33200_e48350_d_n2, assign33200_e48350_d_n6, assign33200_e48350_d_n7, assign33200_e48350_d_n10, assign33200_e48350_d_n11, assign33200_e48350_d_n12, assign33200_e48350_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33200_e48345: f64 = (-locals.var_qbs_max);
        let assign33200_e48346: f64 = (4.0 * assign33200_e48345);
        let assign33200_e48348: f64 = (assign33200_e48346 * locals.var_dlt_qbs);
        (assign33200_e48348, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33200_e48350;
        locals.var_tmf2_dn0 = assign33200_e48350_d_n0;
        locals.var_tmf2_dn2 = assign33200_e48350_d_n2;
        locals.var_tmf2_dn6 = assign33200_e48350_d_n6;
        locals.var_tmf2_dn7 = assign33200_e48350_d_n7;
        locals.var_tmf2_dn10 = assign33200_e48350_d_n10;
        locals.var_tmf2_dn11 = assign33200_e48350_d_n11;
        locals.var_tmf2_dn12 = assign33200_e48350_d_n12;
        locals.var_tmf2_dn17 = assign33200_e48350_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33210_e48362, assign33210_e48362_d_n0, assign33210_e48362_d_n2, assign33210_e48362_d_n6, assign33210_e48362_d_n7, assign33210_e48362_d_n10, assign33210_e48362_d_n11, assign33210_e48362_d_n12, assign33210_e48362_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33210_e48359: f64 = (-locals.var_tmf2);
                (assign33210_e48359, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33210_e48362;
        locals.var_tmf2_dn0 = assign33210_e48362_d_n0;
        locals.var_tmf2_dn2 = assign33210_e48362_d_n2;
        locals.var_tmf2_dn6 = assign33210_e48362_d_n6;
        locals.var_tmf2_dn7 = assign33210_e48362_d_n7;
        locals.var_tmf2_dn10 = assign33210_e48362_d_n10;
        locals.var_tmf2_dn11 = assign33210_e48362_d_n11;
        locals.var_tmf2_dn12 = assign33210_e48362_d_n12;
        locals.var_tmf2_dn17 = assign33210_e48362_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33220_e48373, assign33220_e48373_d_n0, assign33220_e48373_d_n2, assign33220_e48373_d_n6, assign33220_e48373_d_n7, assign33220_e48373_d_n10, assign33220_e48373_d_n11, assign33220_e48373_d_n12, assign33220_e48373_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33220_e48368: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33220_e48370: f64 = (assign33220_e48368 + locals.var_tmf2);
        let assign33220_e48371: f64 = (assign33220_e48370).sqrt();
        (assign33220_e48371, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33220_e48371)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33220_e48373;
        locals.var_tmf2_dn0 = assign33220_e48373_d_n0;
        locals.var_tmf2_dn2 = assign33220_e48373_d_n2;
        locals.var_tmf2_dn6 = assign33220_e48373_d_n6;
        locals.var_tmf2_dn7 = assign33220_e48373_d_n7;
        locals.var_tmf2_dn10 = assign33220_e48373_d_n10;
        locals.var_tmf2_dn11 = assign33220_e48373_d_n11;
        locals.var_tmf2_dn12 = assign33220_e48373_d_n12;
        locals.var_tmf2_dn17 = assign33220_e48373_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33230_e48386, assign33230_e48386_d_n0, assign33230_e48386_d_n2, assign33230_e48386_d_n6, assign33230_e48386_d_n7, assign33230_e48386_d_n10, assign33230_e48386_d_n11, assign33230_e48386_d_n12, assign33230_e48386_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33230_e48378: f64 = (-locals.var_qbs_max);
        let assign33230_e48382: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33230_e48383: f64 = (0.5 * assign33230_e48382);
        let assign33230_e48384: f64 = (assign33230_e48378 - assign33230_e48383);
        (assign33230_e48384, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33230_e48386;
        locals.var_qbs_dn0 = assign33230_e48386_d_n0;
        locals.var_qbs_dn2 = assign33230_e48386_d_n2;
        locals.var_qbs_dn6 = assign33230_e48386_d_n6;
        locals.var_qbs_dn7 = assign33230_e48386_d_n7;
        locals.var_qbs_dn10 = assign33230_e48386_d_n10;
        locals.var_qbs_dn11 = assign33230_e48386_d_n11;
        locals.var_qbs_dn12 = assign33230_e48386_d_n12;
        locals.var_qbs_dn17 = assign33230_e48386_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign33240_e48395, assign33240_e48395_d_n0, assign33240_e48395_d_n2, assign33240_e48395_d_n6, assign33240_e48395_d_n7, assign33240_e48395_d_n10, assign33240_e48395_d_n11, assign33240_e48395_d_n12, assign33240_e48395_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
        let assign33240_e48392: f64 = (-1.0);
        let assign33240_e48393: f64 = (locals.var_qbs * assign33240_e48392);
        (assign33240_e48393, (locals.var_qbs_dn0 * assign33240_e48392), (locals.var_qbs_dn2 * assign33240_e48392), (locals.var_qbs_dn6 * assign33240_e48392), (locals.var_qbs_dn7 * assign33240_e48392), (locals.var_qbs_dn10 * assign33240_e48392), (locals.var_qbs_dn11 * assign33240_e48392), (locals.var_qbs_dn12 * assign33240_e48392), (locals.var_qbs_dn17 * assign33240_e48392),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33240_e48395;
        locals.var_qbs_dn0 = assign33240_e48395_d_n0;
        locals.var_qbs_dn2 = assign33240_e48395_d_n2;
        locals.var_qbs_dn6 = assign33240_e48395_d_n6;
        locals.var_qbs_dn7 = assign33240_e48395_d_n7;
        locals.var_qbs_dn10 = assign33240_e48395_d_n10;
        locals.var_qbs_dn11 = assign33240_e48395_d_n11;
        locals.var_qbs_dn12 = assign33240_e48395_d_n12;
        locals.var_qbs_dn17 = assign33240_e48395_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign33250_e48398: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33250_e48398;
        locals.var_guard1087_rv = 0.0;

        let (assign33260_e48411, assign33260_e48411_d_n0, assign33260_e48411_d_n2, assign33260_e48411_d_n6, assign33260_e48411_d_n7, assign33260_e48411_d_n10, assign33260_e48411_d_n11, assign33260_e48411_d_n12, assign33260_e48411_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33260_e48403: f64 = (-1.6021918e-19);
        let assign33260_e48405: f64 = (assign33260_e48403 * locals.var_uc_nsubs);
        let assign33260_e48407: f64 = (assign33260_e48405 * locals.var_xp_max);
        let assign33260_e48409: f64 = (assign33260_e48407 * p.p2);
        (assign33260_e48409, (((assign33260_e48403 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2),)
    } else {
        (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17,)
    }
};
        locals.var_qbd_max = assign33260_e48411;
        locals.var_qbd_max_dn0 = assign33260_e48411_d_n0;
        locals.var_qbd_max_dn2 = assign33260_e48411_d_n2;
        locals.var_qbd_max_dn6 = assign33260_e48411_d_n6;
        locals.var_qbd_max_dn7 = assign33260_e48411_d_n7;
        locals.var_qbd_max_dn10 = assign33260_e48411_d_n10;
        locals.var_qbd_max_dn11 = assign33260_e48411_d_n11;
        locals.var_qbd_max_dn12 = assign33260_e48411_d_n12;
        locals.var_qbd_max_dn17 = assign33260_e48411_d_n17;
        locals.var_qbd_max_rv = 0.0;

        let (assign33270_e48420, assign33270_e48420_d_n0, assign33270_e48420_d_n2, assign33270_e48420_d_n6, assign33270_e48420_d_n7, assign33270_e48420_d_n10, assign33270_e48420_d_n11, assign33270_e48420_d_n12, assign33270_e48420_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33270_e48417: f64 = (-locals.var_qbd_max);
        let assign33270_e48418: f64 = (0.001 * assign33270_e48417);
        (assign33270_e48418, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)),)
    } else {
        (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17,)
    }
};
        locals.var_dlt_qbd = assign33270_e48420;
        locals.var_dlt_qbd_dn0 = assign33270_e48420_d_n0;
        locals.var_dlt_qbd_dn2 = assign33270_e48420_d_n2;
        locals.var_dlt_qbd_dn6 = assign33270_e48420_d_n6;
        locals.var_dlt_qbd_dn7 = assign33270_e48420_d_n7;
        locals.var_dlt_qbd_dn10 = assign33270_e48420_d_n10;
        locals.var_dlt_qbd_dn11 = assign33270_e48420_d_n11;
        locals.var_dlt_qbd_dn12 = assign33270_e48420_d_n12;
        locals.var_dlt_qbd_dn17 = assign33270_e48420_d_n17;
        locals.var_dlt_qbd_rv = 0.0;

        let (assign33280_e48432, assign33280_e48432_d_n0, assign33280_e48432_d_n2, assign33280_e48432_d_n6, assign33280_e48432_d_n7, assign33280_e48432_d_n10, assign33280_e48432_d_n11, assign33280_e48432_d_n12, assign33280_e48432_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33280_e48425: f64 = (-locals.var_qbd_max);
        let assign33280_e48427: f64 = (-locals.var_qbd);
        let assign33280_e48428: f64 = (assign33280_e48425 - assign33280_e48427);
        let assign33280_e48430: f64 = (assign33280_e48428 - locals.var_dlt_qbd);
        (assign33280_e48430, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33280_e48432;
        locals.var_tmf1_dn0 = assign33280_e48432_d_n0;
        locals.var_tmf1_dn2 = assign33280_e48432_d_n2;
        locals.var_tmf1_dn6 = assign33280_e48432_d_n6;
        locals.var_tmf1_dn7 = assign33280_e48432_d_n7;
        locals.var_tmf1_dn10 = assign33280_e48432_d_n10;
        locals.var_tmf1_dn11 = assign33280_e48432_d_n11;
        locals.var_tmf1_dn12 = assign33280_e48432_d_n12;
        locals.var_tmf1_dn17 = assign33280_e48432_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign33290_e48443, assign33290_e48443_d_n0, assign33290_e48443_d_n2, assign33290_e48443_d_n6, assign33290_e48443_d_n7, assign33290_e48443_d_n10, assign33290_e48443_d_n11, assign33290_e48443_d_n12, assign33290_e48443_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33290_e48438: f64 = (-locals.var_qbd_max);
        let assign33290_e48439: f64 = (4.0 * assign33290_e48438);
        let assign33290_e48441: f64 = (assign33290_e48439 * locals.var_dlt_qbd);
        (assign33290_e48441, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33290_e48443;
        locals.var_tmf2_dn0 = assign33290_e48443_d_n0;
        locals.var_tmf2_dn2 = assign33290_e48443_d_n2;
        locals.var_tmf2_dn6 = assign33290_e48443_d_n6;
        locals.var_tmf2_dn7 = assign33290_e48443_d_n7;
        locals.var_tmf2_dn10 = assign33290_e48443_d_n10;
        locals.var_tmf2_dn11 = assign33290_e48443_d_n11;
        locals.var_tmf2_dn12 = assign33290_e48443_d_n12;
        locals.var_tmf2_dn17 = assign33290_e48443_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33300_e48455, assign33300_e48455_d_n0, assign33300_e48455_d_n2, assign33300_e48455_d_n6, assign33300_e48455_d_n7, assign33300_e48455_d_n10, assign33300_e48455_d_n11, assign33300_e48455_d_n12, assign33300_e48455_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33300_e48452: f64 = (-locals.var_tmf2);
                (assign33300_e48452, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33300_e48455;
        locals.var_tmf2_dn0 = assign33300_e48455_d_n0;
        locals.var_tmf2_dn2 = assign33300_e48455_d_n2;
        locals.var_tmf2_dn6 = assign33300_e48455_d_n6;
        locals.var_tmf2_dn7 = assign33300_e48455_d_n7;
        locals.var_tmf2_dn10 = assign33300_e48455_d_n10;
        locals.var_tmf2_dn11 = assign33300_e48455_d_n11;
        locals.var_tmf2_dn12 = assign33300_e48455_d_n12;
        locals.var_tmf2_dn17 = assign33300_e48455_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33310_e48466, assign33310_e48466_d_n0, assign33310_e48466_d_n2, assign33310_e48466_d_n6, assign33310_e48466_d_n7, assign33310_e48466_d_n10, assign33310_e48466_d_n11, assign33310_e48466_d_n12, assign33310_e48466_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33310_e48461: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33310_e48463: f64 = (assign33310_e48461 + locals.var_tmf2);
        let assign33310_e48464: f64 = (assign33310_e48463).sqrt();
        (assign33310_e48464, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33310_e48464)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33310_e48466;
        locals.var_tmf2_dn0 = assign33310_e48466_d_n0;
        locals.var_tmf2_dn2 = assign33310_e48466_d_n2;
        locals.var_tmf2_dn6 = assign33310_e48466_d_n6;
        locals.var_tmf2_dn7 = assign33310_e48466_d_n7;
        locals.var_tmf2_dn10 = assign33310_e48466_d_n10;
        locals.var_tmf2_dn11 = assign33310_e48466_d_n11;
        locals.var_tmf2_dn12 = assign33310_e48466_d_n12;
        locals.var_tmf2_dn17 = assign33310_e48466_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33320_e48479, assign33320_e48479_d_n0, assign33320_e48479_d_n2, assign33320_e48479_d_n6, assign33320_e48479_d_n7, assign33320_e48479_d_n10, assign33320_e48479_d_n11, assign33320_e48479_d_n12, assign33320_e48479_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33320_e48471: f64 = (-locals.var_qbd_max);
        let assign33320_e48475: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33320_e48476: f64 = (0.5 * assign33320_e48475);
        let assign33320_e48477: f64 = (assign33320_e48471 - assign33320_e48476);
        (assign33320_e48477, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33320_e48479;
        locals.var_qbd_dn0 = assign33320_e48479_d_n0;
        locals.var_qbd_dn2 = assign33320_e48479_d_n2;
        locals.var_qbd_dn6 = assign33320_e48479_d_n6;
        locals.var_qbd_dn7 = assign33320_e48479_d_n7;
        locals.var_qbd_dn10 = assign33320_e48479_d_n10;
        locals.var_qbd_dn11 = assign33320_e48479_d_n11;
        locals.var_qbd_dn12 = assign33320_e48479_d_n12;
        locals.var_qbd_dn17 = assign33320_e48479_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign33330_e48488, assign33330_e48488_d_n0, assign33330_e48488_d_n2, assign33330_e48488_d_n6, assign33330_e48488_d_n7, assign33330_e48488_d_n10, assign33330_e48488_d_n11, assign33330_e48488_d_n12, assign33330_e48488_d_n17,) = {
    if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
        let assign33330_e48485: f64 = (-1.0);
        let assign33330_e48486: f64 = (locals.var_qbd * assign33330_e48485);
        (assign33330_e48486, (locals.var_qbd_dn0 * assign33330_e48485), (locals.var_qbd_dn2 * assign33330_e48485), (locals.var_qbd_dn6 * assign33330_e48485), (locals.var_qbd_dn7 * assign33330_e48485), (locals.var_qbd_dn10 * assign33330_e48485), (locals.var_qbd_dn11 * assign33330_e48485), (locals.var_qbd_dn12 * assign33330_e48485), (locals.var_qbd_dn17 * assign33330_e48485),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33330_e48488;
        locals.var_qbd_dn0 = assign33330_e48488_d_n0;
        locals.var_qbd_dn2 = assign33330_e48488_d_n2;
        locals.var_qbd_dn6 = assign33330_e48488_d_n6;
        locals.var_qbd_dn7 = assign33330_e48488_d_n7;
        locals.var_qbd_dn10 = assign33330_e48488_d_n10;
        locals.var_qbd_dn11 = assign33330_e48488_d_n11;
        locals.var_qbd_dn12 = assign33330_e48488_d_n12;
        locals.var_qbd_dn17 = assign33330_e48488_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33560_e48742: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1120 = assign33560_e48742;
        locals.var_guard1120_rv = 0.0;

        let (assign33570_e48750, assign33570_e48750_d_n0, assign33570_e48750_d_n2, assign33570_e48750_d_n6, assign33570_e48750_d_n7, assign33570_e48750_d_n10, assign33570_e48750_d_n11, assign33570_e48750_d_n12, assign33570_e48750_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33570_e48746: f64 = (locals.var_psdl - locals.var_ps0);
        let assign33570_e48748: f64 = (assign33570_e48746 / locals.var_lch);
        (assign33570_e48748, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17,)
    }
};
        locals.var_eyd = assign33570_e48750;
        locals.var_eyd_dn0 = assign33570_e48750_d_n0;
        locals.var_eyd_dn2 = assign33570_e48750_d_n2;
        locals.var_eyd_dn6 = assign33570_e48750_d_n6;
        locals.var_eyd_dn7 = assign33570_e48750_d_n7;
        locals.var_eyd_dn10 = assign33570_e48750_d_n10;
        locals.var_eyd_dn11 = assign33570_e48750_d_n11;
        locals.var_eyd_dn12 = assign33570_e48750_d_n12;
        locals.var_eyd_dn17 = assign33570_e48750_d_n17;
        locals.var_eyd_rv = 0.0;

        let (assign33580_e48758, assign33580_e48758_d_n0, assign33580_e48758_d_n2, assign33580_e48758_d_n6, assign33580_e48758_d_n7, assign33580_e48758_d_n10, assign33580_e48758_d_n11, assign33580_e48758_d_n12, assign33580_e48758_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33580_e48754: f64 = (locals.var_muun * locals.var_eyd);
        let assign33580_e48756: f64 = (assign33580_e48754 / 100000.0);
        (assign33580_e48756, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0),)
    } else {
        (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17,)
    }
};
        locals.var_t12__blk1104 = assign33580_e48758;
        locals.var_t12__blk1104_dn0 = assign33580_e48758_d_n0;
        locals.var_t12__blk1104_dn2 = assign33580_e48758_d_n2;
        locals.var_t12__blk1104_dn6 = assign33580_e48758_d_n6;
        locals.var_t12__blk1104_dn7 = assign33580_e48758_d_n7;
        locals.var_t12__blk1104_dn10 = assign33580_e48758_d_n10;
        locals.var_t12__blk1104_dn11 = assign33580_e48758_d_n11;
        locals.var_t12__blk1104_dn12 = assign33580_e48758_d_n12;
        locals.var_t12__blk1104_dn17 = assign33580_e48758_d_n17;
        locals.var_t12__blk1104_rv = 0.0;

        let assign33590_e48762: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48763: f64 = (1.0 - assign33590_e48762);
        let assign33590_e48770: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48771: f64 = (1.0 + assign33590_e48770);
        let assign33590_e48773: f64 = if ((assign33590_e48763 <= p.p113) && (p.p113 <= assign33590_e48771)) { 1.0 } else { 0.0 };
        locals.var_guard1121 = assign33590_e48773;
        locals.var_guard1121_rv = 0.0;

        let (assign33600_e48779, assign33600_e48779_d_n0, assign33600_e48779_d_n2, assign33600_e48779_d_n6, assign33600_e48779_d_n7, assign33600_e48779_d_n10, assign33600_e48779_d_n11, assign33600_e48779_d_n12, assign33600_e48779_d_n17,) = {
    if ((locals.var_guard1120 != 0.0) && (locals.var_guard1121 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33600_e48779;
        locals.var_t7__blk1105_dn0 = assign33600_e48779_d_n0;
        locals.var_t7__blk1105_dn2 = assign33600_e48779_d_n2;
        locals.var_t7__blk1105_dn6 = assign33600_e48779_d_n6;
        locals.var_t7__blk1105_dn7 = assign33600_e48779_d_n7;
        locals.var_t7__blk1105_dn10 = assign33600_e48779_d_n10;
        locals.var_t7__blk1105_dn11 = assign33600_e48779_d_n11;
        locals.var_t7__blk1105_dn12 = assign33600_e48779_d_n12;
        locals.var_t7__blk1105_dn17 = assign33600_e48779_d_n17;
        locals.var_t7__blk1105_rv = 0.0;

        let assign33610_e48783: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48784: f64 = (2.0 - assign33610_e48783);
        let assign33610_e48791: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48792: f64 = (2.0 + assign33610_e48791);
        let assign33610_e48794: f64 = if ((assign33610_e48784 <= p.p113) && (p.p113 <= assign33610_e48792)) { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign33610_e48794;
        locals.var_guard1122_rv = 0.0;

        let (assign33620_e48803, assign33620_e48803_d_n0, assign33620_e48803_d_n2, assign33620_e48803_d_n6, assign33620_e48803_d_n7, assign33620_e48803_d_n10, assign33620_e48803_d_n11, assign33620_e48803_d_n12, assign33620_e48803_d_n17,) = {
    if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 != 0.0)) {
        (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17,)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33620_e48803;
        locals.var_t7__blk1105_dn0 = assign33620_e48803_d_n0;
        locals.var_t7__blk1105_dn2 = assign33620_e48803_d_n2;
        locals.var_t7__blk1105_dn6 = assign33620_e48803_d_n6;
        locals.var_t7__blk1105_dn7 = assign33620_e48803_d_n7;
        locals.var_t7__blk1105_dn10 = assign33620_e48803_d_n10;
        locals.var_t7__blk1105_dn11 = assign33620_e48803_d_n11;
        locals.var_t7__blk1105_dn12 = assign33620_e48803_d_n12;
        locals.var_t7__blk1105_dn17 = assign33620_e48803_d_n17;
        locals.var_t7__blk1105_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_122(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33630_e48817, assign33630_e48817_d_n0, assign33630_e48817_d_n2, assign33630_e48817_d_n6, assign33630_e48817_d_n7, assign33630_e48817_d_n10, assign33630_e48817_d_n11, assign33630_e48817_d_n12, assign33630_e48817_d_n17,) = {
    if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 == 0.0)) {
        let assign33630_e48814: f64 = (p.p113 - 1.0);
        let assign33630_e48815: f64 = (locals.var_t12__blk1104).powf(assign33630_e48814);
        (assign33630_e48815, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn0)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn0 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn2)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn2 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn6)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn6 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn7)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn7 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn10)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn10 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn11)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn11 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn12)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn12 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn17)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn17 / locals.var_t12__blk1104))) },)
    } else {
        (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17,)
    }
};
        locals.var_t7__blk1105 = assign33630_e48817;
        locals.var_t7__blk1105_dn0 = assign33630_e48817_d_n0;
        locals.var_t7__blk1105_dn2 = assign33630_e48817_d_n2;
        locals.var_t7__blk1105_dn6 = assign33630_e48817_d_n6;
        locals.var_t7__blk1105_dn7 = assign33630_e48817_d_n7;
        locals.var_t7__blk1105_dn10 = assign33630_e48817_d_n10;
        locals.var_t7__blk1105_dn11 = assign33630_e48817_d_n11;
        locals.var_t7__blk1105_dn12 = assign33630_e48817_d_n12;
        locals.var_t7__blk1105_dn17 = assign33630_e48817_d_n17;
        locals.var_t7__blk1105_rv = 0.0;

        let (assign33640_e48823, assign33640_e48823_d_n0, assign33640_e48823_d_n2, assign33640_e48823_d_n6, assign33640_e48823_d_n7, assign33640_e48823_d_n10, assign33640_e48823_d_n11, assign33640_e48823_d_n12, assign33640_e48823_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33640_e48821: f64 = (locals.var_t12__blk1104 * locals.var_t7__blk1105);
        (assign33640_e48821, ((locals.var_t12__blk1104_dn0 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn0)), ((locals.var_t12__blk1104_dn2 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn2)), ((locals.var_t12__blk1104_dn6 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn6)), ((locals.var_t12__blk1104_dn7 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn7)), ((locals.var_t12__blk1104_dn10 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn10)), ((locals.var_t12__blk1104_dn11 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn11)), ((locals.var_t12__blk1104_dn12 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn12)), ((locals.var_t12__blk1104_dn17 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn17)),)
    } else {
        (locals.var_t8__blk1106, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17,)
    }
};
        locals.var_t8__blk1106 = assign33640_e48823;
        locals.var_t8__blk1106_dn0 = assign33640_e48823_d_n0;
        locals.var_t8__blk1106_dn2 = assign33640_e48823_d_n2;
        locals.var_t8__blk1106_dn6 = assign33640_e48823_d_n6;
        locals.var_t8__blk1106_dn7 = assign33640_e48823_d_n7;
        locals.var_t8__blk1106_dn10 = assign33640_e48823_d_n10;
        locals.var_t8__blk1106_dn11 = assign33640_e48823_d_n11;
        locals.var_t8__blk1106_dn12 = assign33640_e48823_d_n12;
        locals.var_t8__blk1106_dn17 = assign33640_e48823_d_n17;
        locals.var_t8__blk1106_rv = 0.0;

        let (assign33650_e48829, assign33650_e48829_d_n0, assign33650_e48829_d_n2, assign33650_e48829_d_n6, assign33650_e48829_d_n7, assign33650_e48829_d_n10, assign33650_e48829_d_n11, assign33650_e48829_d_n12, assign33650_e48829_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33650_e48827: f64 = (1.0 + locals.var_t8__blk1106);
        (assign33650_e48827, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17,)
    } else {
        (locals.var_t9__blk1107, locals.var_t9__blk1107_dn0, locals.var_t9__blk1107_dn2, locals.var_t9__blk1107_dn6, locals.var_t9__blk1107_dn7, locals.var_t9__blk1107_dn10, locals.var_t9__blk1107_dn11, locals.var_t9__blk1107_dn12, locals.var_t9__blk1107_dn17,)
    }
};
        locals.var_t9__blk1107 = assign33650_e48829;
        locals.var_t9__blk1107_dn0 = assign33650_e48829_d_n0;
        locals.var_t9__blk1107_dn2 = assign33650_e48829_d_n2;
        locals.var_t9__blk1107_dn6 = assign33650_e48829_d_n6;
        locals.var_t9__blk1107_dn7 = assign33650_e48829_d_n7;
        locals.var_t9__blk1107_dn10 = assign33650_e48829_d_n10;
        locals.var_t9__blk1107_dn11 = assign33650_e48829_d_n11;
        locals.var_t9__blk1107_dn12 = assign33650_e48829_d_n12;
        locals.var_t9__blk1107_dn17 = assign33650_e48829_d_n17;
        locals.var_t9__blk1107_rv = 0.0;

        let (assign33660_e48840, assign33660_e48840_d_n0, assign33660_e48840_d_n2, assign33660_e48840_d_n6, assign33660_e48840_d_n7, assign33660_e48840_d_n10, assign33660_e48840_d_n11, assign33660_e48840_d_n12, assign33660_e48840_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33660_e48833: f64 = (-1.0);
        let assign33660_e48835: f64 = (assign33660_e48833 / p.p113);
        let assign33660_e48837: f64 = (assign33660_e48835 - 1.0);
        let assign33660_e48838: f64 = (locals.var_t9__blk1107).powf(assign33660_e48837);
        (assign33660_e48838, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn0)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn0 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn2)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn2 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn6)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn6 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn7)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn7 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn10)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn10 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn11)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn11 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn12)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn12 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn17)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn17 / locals.var_t9__blk1107))) },)
    } else {
        (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17,)
    }
};
        locals.var_t10__blk1108 = assign33660_e48840;
        locals.var_t10__blk1108_dn0 = assign33660_e48840_d_n0;
        locals.var_t10__blk1108_dn2 = assign33660_e48840_d_n2;
        locals.var_t10__blk1108_dn6 = assign33660_e48840_d_n6;
        locals.var_t10__blk1108_dn7 = assign33660_e48840_d_n7;
        locals.var_t10__blk1108_dn10 = assign33660_e48840_d_n10;
        locals.var_t10__blk1108_dn11 = assign33660_e48840_d_n11;
        locals.var_t10__blk1108_dn12 = assign33660_e48840_d_n12;
        locals.var_t10__blk1108_dn17 = assign33660_e48840_d_n17;
        locals.var_t10__blk1108_rv = 0.0;

        let (assign33670_e48846, assign33670_e48846_d_n0, assign33670_e48846_d_n2, assign33670_e48846_d_n6, assign33670_e48846_d_n7, assign33670_e48846_d_n10, assign33670_e48846_d_n11, assign33670_e48846_d_n12, assign33670_e48846_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33670_e48844: f64 = (locals.var_t9__blk1107 * locals.var_t10__blk1108);
        (assign33670_e48844, ((locals.var_t9__blk1107_dn0 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn0)), ((locals.var_t9__blk1107_dn2 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn2)), ((locals.var_t9__blk1107_dn6 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn6)), ((locals.var_t9__blk1107_dn7 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn7)), ((locals.var_t9__blk1107_dn10 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn10)), ((locals.var_t9__blk1107_dn11 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn11)), ((locals.var_t9__blk1107_dn12 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn12)), ((locals.var_t9__blk1107_dn17 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn17)),)
    } else {
        (locals.var_t11__blk1109, locals.var_t11__blk1109_dn0, locals.var_t11__blk1109_dn2, locals.var_t11__blk1109_dn6, locals.var_t11__blk1109_dn7, locals.var_t11__blk1109_dn10, locals.var_t11__blk1109_dn11, locals.var_t11__blk1109_dn12, locals.var_t11__blk1109_dn17,)
    }
};
        locals.var_t11__blk1109 = assign33670_e48846;
        locals.var_t11__blk1109_dn0 = assign33670_e48846_d_n0;
        locals.var_t11__blk1109_dn2 = assign33670_e48846_d_n2;
        locals.var_t11__blk1109_dn6 = assign33670_e48846_d_n6;
        locals.var_t11__blk1109_dn7 = assign33670_e48846_d_n7;
        locals.var_t11__blk1109_dn10 = assign33670_e48846_d_n10;
        locals.var_t11__blk1109_dn11 = assign33670_e48846_d_n11;
        locals.var_t11__blk1109_dn12 = assign33670_e48846_d_n12;
        locals.var_t11__blk1109_dn17 = assign33670_e48846_d_n17;
        locals.var_t11__blk1109_rv = 0.0;

        let (assign33680_e48852, assign33680_e48852_d_n0, assign33680_e48852_d_n2, assign33680_e48852_d_n6, assign33680_e48852_d_n7, assign33680_e48852_d_n10, assign33680_e48852_d_n11, assign33680_e48852_d_n12, assign33680_e48852_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33680_e48850: f64 = (locals.var_muun * locals.var_t11__blk1109);
        (assign33680_e48850, ((locals.var_muun_dn0 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn17)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17,)
    }
};
        locals.var_mud_hoso = assign33680_e48852;
        locals.var_mud_hoso_dn0 = assign33680_e48852_d_n0;
        locals.var_mud_hoso_dn2 = assign33680_e48852_d_n2;
        locals.var_mud_hoso_dn6 = assign33680_e48852_d_n6;
        locals.var_mud_hoso_dn7 = assign33680_e48852_d_n7;
        locals.var_mud_hoso_dn10 = assign33680_e48852_d_n10;
        locals.var_mud_hoso_dn11 = assign33680_e48852_d_n11;
        locals.var_mud_hoso_dn12 = assign33680_e48852_d_n12;
        locals.var_mud_hoso_dn17 = assign33680_e48852_d_n17;
        locals.var_mud_hoso_rv = 0.0;

        let (assign33690_e48860, assign33690_e48860_d_n0, assign33690_e48860_d_n2, assign33690_e48860_d_n6, assign33690_e48860_d_n7, assign33690_e48860_d_n10, assign33690_e48860_d_n11, assign33690_e48860_d_n12, assign33690_e48860_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33690_e48856: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign33690_e48858: f64 = (assign33690_e48856 / 2.0);
        (assign33690_e48858, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17,)
    }
};
        locals.var_mu_ave = assign33690_e48860;
        locals.var_mu_ave_dn0 = assign33690_e48860_d_n0;
        locals.var_mu_ave_dn2 = assign33690_e48860_d_n2;
        locals.var_mu_ave_dn6 = assign33690_e48860_d_n6;
        locals.var_mu_ave_dn7 = assign33690_e48860_d_n7;
        locals.var_mu_ave_dn10 = assign33690_e48860_d_n10;
        locals.var_mu_ave_dn11 = assign33690_e48860_d_n11;
        locals.var_mu_ave_dn12 = assign33690_e48860_d_n12;
        locals.var_mu_ave_dn17 = assign33690_e48860_d_n17;
        locals.var_mu_ave_rv = 0.0;

        let (assign33700_e48866, assign33700_e48866_d_n0, assign33700_e48866_d_n2, assign33700_e48866_d_n6, assign33700_e48866_d_n7, assign33700_e48866_d_n10, assign33700_e48866_d_n11, assign33700_e48866_d_n12, assign33700_e48866_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33700_e48864: f64 = (locals.var_alpha * locals.var_alpha);
        (assign33700_e48864, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_t0__blk1110, locals.var_t0__blk1110_dn0, locals.var_t0__blk1110_dn2, locals.var_t0__blk1110_dn6, locals.var_t0__blk1110_dn7, locals.var_t0__blk1110_dn10, locals.var_t0__blk1110_dn11, locals.var_t0__blk1110_dn12, locals.var_t0__blk1110_dn17,)
    }
};
        locals.var_t0__blk1110 = assign33700_e48866;
        locals.var_t0__blk1110_dn0 = assign33700_e48866_d_n0;
        locals.var_t0__blk1110_dn2 = assign33700_e48866_d_n2;
        locals.var_t0__blk1110_dn6 = assign33700_e48866_d_n6;
        locals.var_t0__blk1110_dn7 = assign33700_e48866_d_n7;
        locals.var_t0__blk1110_dn10 = assign33700_e48866_d_n10;
        locals.var_t0__blk1110_dn11 = assign33700_e48866_d_n11;
        locals.var_t0__blk1110_dn12 = assign33700_e48866_d_n12;
        locals.var_t0__blk1110_dn17 = assign33700_e48866_d_n17;
        locals.var_t0__blk1110_rv = 0.0;

        let (assign33710_e48928, assign33710_e48928_d_n0, assign33710_e48928_d_n2, assign33710_e48928_d_n6, assign33710_e48928_d_n7, assign33710_e48928_d_n10, assign33710_e48928_d_n11, assign33710_e48928_d_n12, assign33710_e48928_d_n17,) = {
    if (locals.var_guard1120 != 0.0) {
        let assign33710_e48870: f64 = (locals.var_weff_nf * locals.var_c_fox);
        let assign33710_e48872: f64 = (assign33710_e48870 * locals.var_vgvt);
        let assign33710_e48874: f64 = (assign33710_e48872 * locals.var_mu);
        let assign33710_e48878: f64 = (3.0 * locals.var_alpha);
        let assign33710_e48879: f64 = (1.0 + assign33710_e48878);
        let assign33710_e48882: f64 = (6.0 * locals.var_t0__blk1110);
        let assign33710_e48883: f64 = (assign33710_e48879 + assign33710_e48882);
        let assign33710_e48885: f64 = (assign33710_e48883 * locals.var_mud_hoso);
        let assign33710_e48887: f64 = (assign33710_e48885 * locals.var_mud_hoso);
        let assign33710_e48891: f64 = (4.0 * locals.var_alpha);
        let assign33710_e48892: f64 = (3.0 + assign33710_e48891);
        let assign33710_e48895: f64 = (3.0 * locals.var_t0__blk1110);
        let assign33710_e48896: f64 = (assign33710_e48892 + assign33710_e48895);
        let assign33710_e48898: f64 = (assign33710_e48896 * locals.var_mud_hoso);
        let assign33710_e48900: f64 = (assign33710_e48898 * locals.var_mu);
        let assign33710_e48901: f64 = (assign33710_e48887 + assign33710_e48900);
        let assign33710_e48905: f64 = (3.0 * locals.var_alpha);
        let assign33710_e48906: f64 = (6.0 + assign33710_e48905);
        let assign33710_e48908: f64 = (assign33710_e48906 + locals.var_t0__blk1110);
        let assign33710_e48910: f64 = (assign33710_e48908 * locals.var_mu);
        let assign33710_e48912: f64 = (assign33710_e48910 * locals.var_mu);
        let assign33710_e48913: f64 = (assign33710_e48901 + assign33710_e48912);
        let assign33710_e48914: f64 = (assign33710_e48874 * assign33710_e48913);
        let assign33710_e48917: f64 = (15.0 * locals.var_lch);
        let assign33710_e48920: f64 = (1.0 + locals.var_alpha);
        let assign33710_e48921: f64 = (assign33710_e48917 * assign33710_e48920);
        let assign33710_e48923: f64 = (assign33710_e48921 * locals.var_mu_ave);
        let assign33710_e48925: f64 = (assign33710_e48923 * locals.var_mu_ave);
        let assign33710_e48926: f64 = (assign33710_e48914 / assign33710_e48925);
        (assign33710_e48926, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn0)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1110_dn0) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn0)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn0))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn0) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn0)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn2)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1110_dn2) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn2)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn2))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn2) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn2)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn6)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1110_dn6) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn6)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn6))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn6) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn6)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn7)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1110_dn7) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn7)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn7))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn7) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn7)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn10)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1110_dn10) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn10)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn10))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn10) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn10)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn11)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1110_dn11) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn11)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn11))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn11) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn11)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn12)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1110_dn12) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn12)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn12))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn12) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn12)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn17)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1110_dn17) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn17)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn17))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn17) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn17)))) / (assign33710_e48925 * assign33710_e48925)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33710_e48928;
        locals.var_nthrml_dn0 = assign33710_e48928_d_n0;
        locals.var_nthrml_dn2 = assign33710_e48928_d_n2;
        locals.var_nthrml_dn6 = assign33710_e48928_d_n6;
        locals.var_nthrml_dn7 = assign33710_e48928_d_n7;
        locals.var_nthrml_dn10 = assign33710_e48928_d_n10;
        locals.var_nthrml_dn11 = assign33710_e48928_d_n11;
        locals.var_nthrml_dn12 = assign33710_e48928_d_n12;
        locals.var_nthrml_dn17 = assign33710_e48928_d_n17;
        locals.var_nthrml_rv = 0.0;

        let (assign33720_e48933, assign33720_e48933_d_n0, assign33720_e48933_d_n2, assign33720_e48933_d_n6, assign33720_e48933_d_n7, assign33720_e48933_d_n10, assign33720_e48933_d_n11, assign33720_e48933_d_n12, assign33720_e48933_d_n17,) = {
    if (locals.var_guard1120 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33720_e48933;
        locals.var_nthrml_dn0 = assign33720_e48933_d_n0;
        locals.var_nthrml_dn2 = assign33720_e48933_d_n2;
        locals.var_nthrml_dn6 = assign33720_e48933_d_n6;
        locals.var_nthrml_dn7 = assign33720_e48933_d_n7;
        locals.var_nthrml_dn10 = assign33720_e48933_d_n10;
        locals.var_nthrml_dn11 = assign33720_e48933_d_n11;
        locals.var_nthrml_dn12 = assign33720_e48933_d_n12;
        locals.var_nthrml_dn17 = assign33720_e48933_d_n17;
        locals.var_nthrml_rv = 0.0;

        let assign33730_e48947: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign33730_e48947;
        locals.var_guard1123_rv = 0.0;

        let (assign33740_e48952, assign33740_e48952_d_n0, assign33740_e48952_d_n2, assign33740_e48952_d_n6, assign33740_e48952_d_n7, assign33740_e48952_d_n10, assign33740_e48952_d_n11, assign33740_e48952_d_n12, assign33740_e48952_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33740_e48950: f64 = (locals.var_kusail).sqrt();
        (assign33740_e48950, (locals.var_kusail_dn0 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn2 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn6 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn7 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn10 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn11 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn12 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn17 / (2.0 * assign33740_e48950)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17,)
    }
};
        locals.var_sqrtkusail = assign33740_e48952;
        locals.var_sqrtkusail_dn0 = assign33740_e48952_d_n0;
        locals.var_sqrtkusail_dn2 = assign33740_e48952_d_n2;
        locals.var_sqrtkusail_dn6 = assign33740_e48952_d_n6;
        locals.var_sqrtkusail_dn7 = assign33740_e48952_d_n7;
        locals.var_sqrtkusail_dn10 = assign33740_e48952_d_n10;
        locals.var_sqrtkusail_dn11 = assign33740_e48952_d_n11;
        locals.var_sqrtkusail_dn12 = assign33740_e48952_d_n12;
        locals.var_sqrtkusail_dn17 = assign33740_e48952_d_n17;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign33750_e48958, assign33750_e48958_d_n0, assign33750_e48958_d_n2, assign33750_e48958_d_n6, assign33750_e48958_d_n7, assign33750_e48958_d_n10, assign33750_e48958_d_n11, assign33750_e48958_d_n12, assign33750_e48958_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33750_e48956: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign33750_e48956, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17),)
    } else {
        (locals.var_t2__blk1112, locals.var_t2__blk1112_dn0, locals.var_t2__blk1112_dn2, locals.var_t2__blk1112_dn6, locals.var_t2__blk1112_dn7, locals.var_t2__blk1112_dn10, locals.var_t2__blk1112_dn11, locals.var_t2__blk1112_dn12, locals.var_t2__blk1112_dn17,)
    }
};
        locals.var_t2__blk1112 = assign33750_e48958;
        locals.var_t2__blk1112_dn0 = assign33750_e48958_d_n0;
        locals.var_t2__blk1112_dn2 = assign33750_e48958_d_n2;
        locals.var_t2__blk1112_dn6 = assign33750_e48958_d_n6;
        locals.var_t2__blk1112_dn7 = assign33750_e48958_d_n7;
        locals.var_t2__blk1112_dn10 = assign33750_e48958_d_n10;
        locals.var_t2__blk1112_dn11 = assign33750_e48958_d_n11;
        locals.var_t2__blk1112_dn12 = assign33750_e48958_d_n12;
        locals.var_t2__blk1112_dn17 = assign33750_e48958_d_n17;
        locals.var_t2__blk1112_rv = 0.0;

        let (assign33760_e48964, assign33760_e48964_d_n0, assign33760_e48964_d_n2, assign33760_e48964_d_n6, assign33760_e48964_d_n7, assign33760_e48964_d_n10, assign33760_e48964_d_n11, assign33760_e48964_d_n12, assign33760_e48964_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33760_e48962: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign33760_e48962, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)),)
    } else {
        (locals.var_t3__blk1113, locals.var_t3__blk1113_dn0, locals.var_t3__blk1113_dn2, locals.var_t3__blk1113_dn6, locals.var_t3__blk1113_dn7, locals.var_t3__blk1113_dn10, locals.var_t3__blk1113_dn11, locals.var_t3__blk1113_dn12, locals.var_t3__blk1113_dn17,)
    }
};
        locals.var_t3__blk1113 = assign33760_e48964;
        locals.var_t3__blk1113_dn0 = assign33760_e48964_d_n0;
        locals.var_t3__blk1113_dn2 = assign33760_e48964_d_n2;
        locals.var_t3__blk1113_dn6 = assign33760_e48964_d_n6;
        locals.var_t3__blk1113_dn7 = assign33760_e48964_d_n7;
        locals.var_t3__blk1113_dn10 = assign33760_e48964_d_n10;
        locals.var_t3__blk1113_dn11 = assign33760_e48964_d_n11;
        locals.var_t3__blk1113_dn12 = assign33760_e48964_d_n12;
        locals.var_t3__blk1113_dn17 = assign33760_e48964_d_n17;
        locals.var_t3__blk1113_rv = 0.0;

        let (assign33770_e48970, assign33770_e48970_d_n0, assign33770_e48970_d_n2, assign33770_e48970_d_n6, assign33770_e48970_d_n7, assign33770_e48970_d_n10, assign33770_e48970_d_n11, assign33770_e48970_d_n12, assign33770_e48970_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33770_e48968: f64 = (locals.var_kusail * locals.var_kusail);
        (assign33770_e48968, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t4__blk1114, locals.var_t4__blk1114_dn0, locals.var_t4__blk1114_dn2, locals.var_t4__blk1114_dn6, locals.var_t4__blk1114_dn7, locals.var_t4__blk1114_dn10, locals.var_t4__blk1114_dn11, locals.var_t4__blk1114_dn12, locals.var_t4__blk1114_dn17,)
    }
};
        locals.var_t4__blk1114 = assign33770_e48970;
        locals.var_t4__blk1114_dn0 = assign33770_e48970_d_n0;
        locals.var_t4__blk1114_dn2 = assign33770_e48970_d_n2;
        locals.var_t4__blk1114_dn6 = assign33770_e48970_d_n6;
        locals.var_t4__blk1114_dn7 = assign33770_e48970_d_n7;
        locals.var_t4__blk1114_dn10 = assign33770_e48970_d_n10;
        locals.var_t4__blk1114_dn11 = assign33770_e48970_d_n11;
        locals.var_t4__blk1114_dn12 = assign33770_e48970_d_n12;
        locals.var_t4__blk1114_dn17 = assign33770_e48970_d_n17;
        locals.var_t4__blk1114_rv = 0.0;

        let (assign33780_e48978, assign33780_e48978_d_n0, assign33780_e48978_d_n2, assign33780_e48978_d_n6, assign33780_e48978_d_n7, assign33780_e48978_d_n10, assign33780_e48978_d_n11, assign33780_e48978_d_n12, assign33780_e48978_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33780_e48974: f64 = (42.0 * locals.var_kusai00);
        let assign33780_e48976: f64 = (assign33780_e48974 * locals.var_kusail);
        (assign33780_e48976, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33780_e48978;
        locals.var_t5__blk1115_dn0 = assign33780_e48978_d_n0;
        locals.var_t5__blk1115_dn2 = assign33780_e48978_d_n2;
        locals.var_t5__blk1115_dn6 = assign33780_e48978_d_n6;
        locals.var_t5__blk1115_dn7 = assign33780_e48978_d_n7;
        locals.var_t5__blk1115_dn10 = assign33780_e48978_d_n10;
        locals.var_t5__blk1115_dn11 = assign33780_e48978_d_n11;
        locals.var_t5__blk1115_dn12 = assign33780_e48978_d_n12;
        locals.var_t5__blk1115_dn17 = assign33780_e48978_d_n17;
        locals.var_t5__blk1115_rv = 0.0;

        let (assign33790_e48988, assign33790_e48988_d_n0, assign33790_e48988_d_n2, assign33790_e48988_d_n6, assign33790_e48988_d_n7, assign33790_e48988_d_n10, assign33790_e48988_d_n11, assign33790_e48988_d_n12, assign33790_e48988_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33790_e48984: f64 = (locals.var_t3__blk1113 + locals.var_t4__blk1114);
        let assign33790_e48985: f64 = (4.0 * assign33790_e48984);
        let assign33790_e48986: f64 = (locals.var_t5__blk1115 + assign33790_e48985);
        (assign33790_e48986, (locals.var_t5__blk1115_dn0 + (4.0 * (locals.var_t3__blk1113_dn0 + locals.var_t4__blk1114_dn0))), (locals.var_t5__blk1115_dn2 + (4.0 * (locals.var_t3__blk1113_dn2 + locals.var_t4__blk1114_dn2))), (locals.var_t5__blk1115_dn6 + (4.0 * (locals.var_t3__blk1113_dn6 + locals.var_t4__blk1114_dn6))), (locals.var_t5__blk1115_dn7 + (4.0 * (locals.var_t3__blk1113_dn7 + locals.var_t4__blk1114_dn7))), (locals.var_t5__blk1115_dn10 + (4.0 * (locals.var_t3__blk1113_dn10 + locals.var_t4__blk1114_dn10))), (locals.var_t5__blk1115_dn11 + (4.0 * (locals.var_t3__blk1113_dn11 + locals.var_t4__blk1114_dn11))), (locals.var_t5__blk1115_dn12 + (4.0 * (locals.var_t3__blk1113_dn12 + locals.var_t4__blk1114_dn12))), (locals.var_t5__blk1115_dn17 + (4.0 * (locals.var_t3__blk1113_dn17 + locals.var_t4__blk1114_dn17))),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33790_e48988;
        locals.var_t5__blk1115_dn0 = assign33790_e48988_d_n0;
        locals.var_t5__blk1115_dn2 = assign33790_e48988_d_n2;
        locals.var_t5__blk1115_dn6 = assign33790_e48988_d_n6;
        locals.var_t5__blk1115_dn7 = assign33790_e48988_d_n7;
        locals.var_t5__blk1115_dn10 = assign33790_e48988_d_n10;
        locals.var_t5__blk1115_dn11 = assign33790_e48988_d_n11;
        locals.var_t5__blk1115_dn12 = assign33790_e48988_d_n12;
        locals.var_t5__blk1115_dn17 = assign33790_e48988_d_n17;
        locals.var_t5__blk1115_rv = 0.0;

        let (assign33800_e49002, assign33800_e49002_d_n0, assign33800_e49002_d_n2, assign33800_e49002_d_n6, assign33800_e49002_d_n7, assign33800_e49002_d_n10, assign33800_e49002_d_n11, assign33800_e49002_d_n12, assign33800_e49002_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33800_e48993: f64 = (20.0 * locals.var_sqrtkusail);
        let assign33800_e48995: f64 = (assign33800_e48993 * locals.var_vgvt);
        let assign33800_e48998: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign33800_e48999: f64 = (assign33800_e48995 * assign33800_e48998);
        let assign33800_e49000: f64 = (locals.var_t5__blk1115 + assign33800_e48999);
        (assign33800_e49000, (locals.var_t5__blk1115_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn0)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1115_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn2)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1115_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn6)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1115_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn7)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1115_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn10)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1115_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn11)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1115_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn12)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1115_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn17)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))),)
    } else {
        (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17,)
    }
};
        locals.var_t5__blk1115 = assign33800_e49002;
        locals.var_t5__blk1115_dn0 = assign33800_e49002_d_n0;
        locals.var_t5__blk1115_dn2 = assign33800_e49002_d_n2;
        locals.var_t5__blk1115_dn6 = assign33800_e49002_d_n6;
        locals.var_t5__blk1115_dn7 = assign33800_e49002_d_n7;
        locals.var_t5__blk1115_dn10 = assign33800_e49002_d_n10;
        locals.var_t5__blk1115_dn11 = assign33800_e49002_d_n11;
        locals.var_t5__blk1115_dn12 = assign33800_e49002_d_n12;
        locals.var_t5__blk1115_dn17 = assign33800_e49002_d_n17;
        locals.var_t5__blk1115_rv = 0.0;

        let (assign33810_e49008, assign33810_e49008_d_n0, assign33810_e49008_d_n2, assign33810_e49008_d_n6, assign33810_e49008_d_n7, assign33810_e49008_d_n10, assign33810_e49008_d_n11, assign33810_e49008_d_n12, assign33810_e49008_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33810_e49006: f64 = (locals.var_t2__blk1112 * locals.var_t2__blk1112);
        (assign33810_e49006, ((locals.var_t2__blk1112_dn0 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn0)), ((locals.var_t2__blk1112_dn2 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn2)), ((locals.var_t2__blk1112_dn6 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn6)), ((locals.var_t2__blk1112_dn7 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn7)), ((locals.var_t2__blk1112_dn10 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn10)), ((locals.var_t2__blk1112_dn11 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn11)), ((locals.var_t2__blk1112_dn12 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn12)), ((locals.var_t2__blk1112_dn17 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn17)),)
    } else {
        (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17,)
    }
};
        locals.var_t10w = assign33810_e49008;
        locals.var_t10w_dn0 = assign33810_e49008_d_n0;
        locals.var_t10w_dn2 = assign33810_e49008_d_n2;
        locals.var_t10w_dn6 = assign33810_e49008_d_n6;
        locals.var_t10w_dn7 = assign33810_e49008_d_n7;
        locals.var_t10w_dn10 = assign33810_e49008_d_n10;
        locals.var_t10w_dn11 = assign33810_e49008_d_n11;
        locals.var_t10w_dn12 = assign33810_e49008_d_n12;
        locals.var_t10w_dn17 = assign33810_e49008_d_n17;
        locals.var_t10w_rv = 0.0;

        let (assign33820_e49014, assign33820_e49014_d_n0, assign33820_e49014_d_n2, assign33820_e49014_d_n6, assign33820_e49014_d_n7, assign33820_e49014_d_n10, assign33820_e49014_d_n11, assign33820_e49014_d_n12, assign33820_e49014_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33820_e49012: f64 = (locals.var_t10w * locals.var_t10w);
        (assign33820_e49012, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)),)
    } else {
        (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17,)
    }
};
        locals.var_t10__blk1108 = assign33820_e49014;
        locals.var_t10__blk1108_dn0 = assign33820_e49014_d_n0;
        locals.var_t10__blk1108_dn2 = assign33820_e49014_d_n2;
        locals.var_t10__blk1108_dn6 = assign33820_e49014_d_n6;
        locals.var_t10__blk1108_dn7 = assign33820_e49014_d_n7;
        locals.var_t10__blk1108_dn10 = assign33820_e49014_d_n10;
        locals.var_t10__blk1108_dn11 = assign33820_e49014_d_n11;
        locals.var_t10__blk1108_dn12 = assign33820_e49014_d_n12;
        locals.var_t10__blk1108_dn17 = assign33820_e49014_d_n17;
        locals.var_t10__blk1108_rv = 0.0;

        let (assign33830_e49022, assign33830_e49022_d_n0, assign33830_e49022_d_n2, assign33830_e49022_d_n6, assign33830_e49022_d_n7, assign33830_e49022_d_n10, assign33830_e49022_d_n11, assign33830_e49022_d_n12, assign33830_e49022_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33830_e49019: f64 = (locals.var_t10__blk1108 * locals.var_t2__blk1112);
        let assign33830_e49020: f64 = (locals.var_t5__blk1115 / assign33830_e49019);
        (assign33830_e49020, (((locals.var_t5__blk1115_dn0 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn0 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn0)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn2 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn2 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn2)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn6 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn6 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn6)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn7 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn7 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn7)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn10 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn10 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn10)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn11 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn11 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn11)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn12 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn12 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn12)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn17 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn17 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn17)))) / (assign33830_e49019 * assign33830_e49019)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17,)
    }
};
        locals.var_kusai_ig = assign33830_e49022;
        locals.var_kusai_ig_dn0 = assign33830_e49022_d_n0;
        locals.var_kusai_ig_dn2 = assign33830_e49022_d_n2;
        locals.var_kusai_ig_dn6 = assign33830_e49022_d_n6;
        locals.var_kusai_ig_dn7 = assign33830_e49022_d_n7;
        locals.var_kusai_ig_dn10 = assign33830_e49022_d_n10;
        locals.var_kusai_ig_dn11 = assign33830_e49022_d_n11;
        locals.var_kusai_ig_dn12 = assign33830_e49022_d_n12;
        locals.var_kusai_ig_dn17 = assign33830_e49022_d_n17;
        locals.var_kusai_ig_rv = 0.0;

        let (assign33840_e49032, assign33840_e49032_d_n0, assign33840_e49032_d_n2, assign33840_e49032_d_n6, assign33840_e49032_d_n7, assign33840_e49032_d_n10, assign33840_e49032_d_n11, assign33840_e49032_d_n12, assign33840_e49032_d_n17,) = {
    if (locals.var_guard1123 != 0.0) {
        let assign33840_e49026: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign33840_e49028: f64 = (assign33840_e49026 * locals.var_mu);
        let assign33840_e49030: f64 = (assign33840_e49028 * locals.var_c_fox);
        (assign33840_e49030, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17,)
    }
};
        locals.var_gds0_ign = assign33840_e49032;
        locals.var_gds0_ign_dn0 = assign33840_e49032_d_n0;
        locals.var_gds0_ign_dn2 = assign33840_e49032_d_n2;
        locals.var_gds0_ign_dn6 = assign33840_e49032_d_n6;
        locals.var_gds0_ign_dn7 = assign33840_e49032_d_n7;
        locals.var_gds0_ign_dn10 = assign33840_e49032_d_n10;
        locals.var_gds0_ign_dn11 = assign33840_e49032_d_n11;
        locals.var_gds0_ign_dn12 = assign33840_e49032_d_n12;
        locals.var_gds0_ign_dn17 = assign33840_e49032_d_n17;
        locals.var_gds0_ign_rv = 0.0;

        let assign33890_e49080: f64 = (locals.var_ids + locals.var_idsibpc);
        locals.var_ids = assign33890_e49080;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        locals.var_ids_dn2 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        locals.var_ids_dn6 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        locals.var_ids_dn7 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        locals.var_ids_dn10 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        locals.var_ids_dn11 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        locals.var_ids_dn12 = (locals.var_ids_dn12 + locals.var_idsibpc_dn12);
        locals.var_ids_dn17 = (locals.var_ids_dn17 + locals.var_idsibpc_dn17);
        locals.var_ids_rv = 0.0;

        let assign33900_e49083: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33900_e49083;
        locals.var_guard1124_rv = 0.0;

        let (assign33910_e49089,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33910_e49087: f64 = (locals.var_cbtp + locals.var_cbtn);
        (assign33910_e49087,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33910_e49089;
        locals.var_cgbe_rv = 0.0;

        let (assign33920_e49099,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign33920_e49096: f64 = (p.p168 * locals.var_lgleff);
        let assign33920_e49097: f64 = (locals.var_cgbe - assign33920_e49096);
        (assign33920_e49097,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33920_e49099;
        locals.var_cgbe_rv = 0.0;

        let (assign33930_e49108, assign33930_e49108_d_n0, assign33930_e49108_d_n2, assign33930_e49108_d_n6, assign33930_e49108_d_n7, assign33930_e49108_d_n10, assign33930_e49108_d_n11, assign33930_e49108_d_n12, assign33930_e49108_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33930_e49102: f64 = (-locals.var_cgbe);
        let assign33930_e49105: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33930_e49106: f64 = (assign33930_e49102 * assign33930_e49105);
        (assign33930_e49106, (assign33930_e49102 * (-locals.var_vbsp_dn0)), (assign33930_e49102 * (-locals.var_vbsp_dn2)), (assign33930_e49102 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33930_e49102 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33930_e49102 * (-locals.var_vbsp_dn10)), (assign33930_e49102 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33930_e49102 * (-locals.var_vbsp_dn12)), (assign33930_e49102 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign33930_e49108;
        locals.var_qgob_dn0 = assign33930_e49108_d_n0;
        locals.var_qgob_dn2 = assign33930_e49108_d_n2;
        locals.var_qgob_dn6 = assign33930_e49108_d_n6;
        locals.var_qgob_dn7 = assign33930_e49108_d_n7;
        locals.var_qgob_dn10 = assign33930_e49108_d_n10;
        locals.var_qgob_dn11 = assign33930_e49108_d_n11;
        locals.var_qgob_dn12 = assign33930_e49108_d_n12;
        locals.var_qgob_dn17 = assign33930_e49108_d_n17;
        locals.var_qgob_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33940_e49118,) = {
    if (locals.var_guard1124 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfu,)
    }
};
        locals.var_cfu = assign33940_e49118;
        locals.var_cfu_rv = 0.0;

        let (assign33950_e49128,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33950_e49122: f64 = (locals.var_cfu * p.p9);
        let assign33950_e49125: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
        let assign33950_e49126: f64 = (assign33950_e49122 * assign33950_e49125);
        (assign33950_e49126,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign33950_e49128;
        locals.var_cfd_rv = 0.0;

        let (assign33960_e49138,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33960_e49132: f64 = (locals.var_cfu * p.p9);
        let assign33960_e49135: f64 = (locals.var_wgate + locals.var_uc_psbcp);
        let assign33960_e49136: f64 = (assign33960_e49132 * assign33960_e49135);
        (assign33960_e49136,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign33960_e49138;
        locals.var_cfs_rv = 0.0;

        let (assign33970_e49146, assign33970_e49146_d_n0, assign33970_e49146_d_n2, assign33970_e49146_d_n6, assign33970_e49146_d_n7, assign33970_e49146_d_n10, assign33970_e49146_d_n11, assign33970_e49146_d_n12, assign33970_e49146_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33970_e49143: f64 = (locals.var_vgs - locals.var_vds);
        let assign33970_e49144: f64 = (locals.var_cfd * assign33970_e49143);
        (assign33970_e49144, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign33970_e49146;
        locals.var_qfd_dn0 = assign33970_e49146_d_n0;
        locals.var_qfd_dn2 = assign33970_e49146_d_n2;
        locals.var_qfd_dn6 = assign33970_e49146_d_n6;
        locals.var_qfd_dn7 = assign33970_e49146_d_n7;
        locals.var_qfd_dn10 = assign33970_e49146_d_n10;
        locals.var_qfd_dn11 = assign33970_e49146_d_n11;
        locals.var_qfd_dn12 = assign33970_e49146_d_n12;
        locals.var_qfd_dn17 = assign33970_e49146_d_n17;
        locals.var_qfd_rv = 0.0;

        let (assign33980_e49152, assign33980_e49152_d_n6, assign33980_e49152_d_n7, assign33980_e49152_d_n11,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33980_e49150: f64 = (locals.var_cfs * locals.var_vgs);
        (assign33980_e49150, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign33980_e49152;
        locals.var_qfs_dn6 = assign33980_e49152_d_n6;
        locals.var_qfs_dn7 = assign33980_e49152_d_n7;
        locals.var_qfs_dn11 = assign33980_e49152_d_n11;
        locals.var_qfs_rv = 0.0;

        let (assign33990_e49164, assign33990_e49164_d_n0, assign33990_e49164_d_n2, assign33990_e49164_d_n6, assign33990_e49164_d_n7, assign33990_e49164_d_n10, assign33990_e49164_d_n11, assign33990_e49164_d_n12, assign33990_e49164_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33990_e49156: f64 = (locals.var_cfu * p.p19);
        let assign33990_e49158: f64 = (assign33990_e49156 * p.p9);
        let assign33990_e49161: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33990_e49162: f64 = (assign33990_e49158 * assign33990_e49161);
        (assign33990_e49162, (assign33990_e49158 * (-locals.var_vbsp_dn0)), (assign33990_e49158 * (-locals.var_vbsp_dn2)), (assign33990_e49158 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33990_e49158 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33990_e49158 * (-locals.var_vbsp_dn10)), (assign33990_e49158 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33990_e49158 * (-locals.var_vbsp_dn12)), (assign33990_e49158 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17,)
    }
};
        locals.var_qfbc = assign33990_e49164;
        locals.var_qfbc_dn0 = assign33990_e49164_d_n0;
        locals.var_qfbc_dn2 = assign33990_e49164_d_n2;
        locals.var_qfbc_dn6 = assign33990_e49164_d_n6;
        locals.var_qfbc_dn7 = assign33990_e49164_d_n7;
        locals.var_qfbc_dn10 = assign33990_e49164_d_n10;
        locals.var_qfbc_dn11 = assign33990_e49164_d_n11;
        locals.var_qfbc_dn12 = assign33990_e49164_d_n12;
        locals.var_qfbc_dn17 = assign33990_e49164_d_n17;
        locals.var_qfbc_rv = 0.0;

        let (assign34000_e49170, assign34000_e49170_d_n0, assign34000_e49170_d_n2, assign34000_e49170_d_n6, assign34000_e49170_d_n7, assign34000_e49170_d_n10, assign34000_e49170_d_n11, assign34000_e49170_d_n12, assign34000_e49170_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34000_e49168: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34000_e49168, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34000_e49170;
        locals.var_qgod_dn0 = assign34000_e49170_d_n0;
        locals.var_qgod_dn2 = assign34000_e49170_d_n2;
        locals.var_qgod_dn6 = assign34000_e49170_d_n6;
        locals.var_qgod_dn7 = assign34000_e49170_d_n7;
        locals.var_qgod_dn10 = assign34000_e49170_d_n10;
        locals.var_qgod_dn11 = assign34000_e49170_d_n11;
        locals.var_qgod_dn12 = assign34000_e49170_d_n12;
        locals.var_qgod_dn17 = assign34000_e49170_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign34010_e49176, assign34010_e49176_d_n0, assign34010_e49176_d_n2, assign34010_e49176_d_n6, assign34010_e49176_d_n7, assign34010_e49176_d_n10, assign34010_e49176_d_n11, assign34010_e49176_d_n12, assign34010_e49176_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34010_e49174: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34010_e49174, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34010_e49176;
        locals.var_qgos_dn0 = assign34010_e49176_d_n0;
        locals.var_qgos_dn2 = assign34010_e49176_d_n2;
        locals.var_qgos_dn6 = assign34010_e49176_d_n6;
        locals.var_qgos_dn7 = assign34010_e49176_d_n7;
        locals.var_qgos_dn10 = assign34010_e49176_d_n10;
        locals.var_qgos_dn11 = assign34010_e49176_d_n11;
        locals.var_qgos_dn12 = assign34010_e49176_d_n12;
        locals.var_qgos_dn17 = assign34010_e49176_d_n17;
        locals.var_qgos_rv = 0.0;

        let (assign34020_e49182, assign34020_e49182_d_n0, assign34020_e49182_d_n2, assign34020_e49182_d_n6, assign34020_e49182_d_n7, assign34020_e49182_d_n10, assign34020_e49182_d_n11, assign34020_e49182_d_n12, assign34020_e49182_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign34020_e49180: f64 = (locals.var_qgob + locals.var_qfbc);
        (assign34020_e49180, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34020_e49182;
        locals.var_qgob_dn0 = assign34020_e49182_d_n0;
        locals.var_qgob_dn2 = assign34020_e49182_d_n2;
        locals.var_qgob_dn6 = assign34020_e49182_d_n6;
        locals.var_qgob_dn7 = assign34020_e49182_d_n7;
        locals.var_qgob_dn10 = assign34020_e49182_d_n10;
        locals.var_qgob_dn11 = assign34020_e49182_d_n11;
        locals.var_qgob_dn12 = assign34020_e49182_d_n12;
        locals.var_qgob_dn17 = assign34020_e49182_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34030_e49192,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34030_e49188: f64 = (-p.p168);
        let assign34030_e49190: f64 = (assign34030_e49188 * locals.var_lgleff);
        (assign34030_e49190,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34030_e49192;
        locals.var_cgbe_rv = 0.0;

        let (assign34040_e49204, assign34040_e49204_d_n0, assign34040_e49204_d_n2, assign34040_e49204_d_n6, assign34040_e49204_d_n7, assign34040_e49204_d_n10, assign34040_e49204_d_n11, assign34040_e49204_d_n12, assign34040_e49204_d_n17,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34040_e49198: f64 = (-locals.var_cgbe);
        let assign34040_e49201: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34040_e49202: f64 = (assign34040_e49198 * assign34040_e49201);
        (assign34040_e49202, (assign34040_e49198 * (-locals.var_vbsp_dn0)), (assign34040_e49198 * (-locals.var_vbsp_dn2)), (assign34040_e49198 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34040_e49198 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34040_e49198 * (-locals.var_vbsp_dn10)), (assign34040_e49198 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34040_e49198 * (-locals.var_vbsp_dn12)), (assign34040_e49198 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34040_e49204;
        locals.var_qgob_dn0 = assign34040_e49204_d_n0;
        locals.var_qgob_dn2 = assign34040_e49204_d_n2;
        locals.var_qgob_dn6 = assign34040_e49204_d_n6;
        locals.var_qgob_dn7 = assign34040_e49204_d_n7;
        locals.var_qgob_dn10 = assign34040_e49204_d_n10;
        locals.var_qgob_dn11 = assign34040_e49204_d_n11;
        locals.var_qgob_dn12 = assign34040_e49204_d_n12;
        locals.var_qgob_dn17 = assign34040_e49204_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34050_e49212,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34050_e49212;
        locals.var_cgbe_rv = 0.0;

        let (assign34060_e49220, assign34060_e49220_d_n0, assign34060_e49220_d_n2, assign34060_e49220_d_n6, assign34060_e49220_d_n7, assign34060_e49220_d_n10, assign34060_e49220_d_n11, assign34060_e49220_d_n12, assign34060_e49220_d_n17,) = {
    if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34060_e49220;
        locals.var_qgob_dn0 = assign34060_e49220_d_n0;
        locals.var_qgob_dn2 = assign34060_e49220_d_n2;
        locals.var_qgob_dn6 = assign34060_e49220_d_n6;
        locals.var_qgob_dn7 = assign34060_e49220_d_n7;
        locals.var_qgob_dn10 = assign34060_e49220_d_n10;
        locals.var_qgob_dn11 = assign34060_e49220_d_n11;
        locals.var_qgob_dn12 = assign34060_e49220_d_n12;
        locals.var_qgob_dn17 = assign34060_e49220_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34070_e49235,) = {
    if (locals.var_guard1124 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cf,)
    }
};
        locals.var_cf = assign34070_e49235;
        locals.var_cf_rv = 0.0;

        let (assign34080_e49240,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign34080_e49240;
        locals.var_cfd_rv = 0.0;

        let (assign34090_e49245,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign34090_e49245;
        locals.var_cfs_rv = 0.0;

        let (assign34100_e49254, assign34100_e49254_d_n0, assign34100_e49254_d_n2, assign34100_e49254_d_n6, assign34100_e49254_d_n7, assign34100_e49254_d_n10, assign34100_e49254_d_n11, assign34100_e49254_d_n12, assign34100_e49254_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34100_e49251: f64 = (locals.var_vgs - locals.var_vds);
        let assign34100_e49252: f64 = (locals.var_cfd * assign34100_e49251);
        (assign34100_e49252, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign34100_e49254;
        locals.var_qfd_dn0 = assign34100_e49254_d_n0;
        locals.var_qfd_dn2 = assign34100_e49254_d_n2;
        locals.var_qfd_dn6 = assign34100_e49254_d_n6;
        locals.var_qfd_dn7 = assign34100_e49254_d_n7;
        locals.var_qfd_dn10 = assign34100_e49254_d_n10;
        locals.var_qfd_dn11 = assign34100_e49254_d_n11;
        locals.var_qfd_dn12 = assign34100_e49254_d_n12;
        locals.var_qfd_dn17 = assign34100_e49254_d_n17;
        locals.var_qfd_rv = 0.0;

        let (assign34110_e49261, assign34110_e49261_d_n6, assign34110_e49261_d_n7, assign34110_e49261_d_n11,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34110_e49259: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34110_e49259, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34110_e49261;
        locals.var_qfs_dn6 = assign34110_e49261_d_n6;
        locals.var_qfs_dn7 = assign34110_e49261_d_n7;
        locals.var_qfs_dn11 = assign34110_e49261_d_n11;
        locals.var_qfs_rv = 0.0;

        let (assign34120_e49268, assign34120_e49268_d_n0, assign34120_e49268_d_n2, assign34120_e49268_d_n6, assign34120_e49268_d_n7, assign34120_e49268_d_n10, assign34120_e49268_d_n11, assign34120_e49268_d_n12, assign34120_e49268_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34120_e49266: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34120_e49266, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34120_e49268;
        locals.var_qgod_dn0 = assign34120_e49268_d_n0;
        locals.var_qgod_dn2 = assign34120_e49268_d_n2;
        locals.var_qgod_dn6 = assign34120_e49268_d_n6;
        locals.var_qgod_dn7 = assign34120_e49268_d_n7;
        locals.var_qgod_dn10 = assign34120_e49268_d_n10;
        locals.var_qgod_dn11 = assign34120_e49268_d_n11;
        locals.var_qgod_dn12 = assign34120_e49268_d_n12;
        locals.var_qgod_dn17 = assign34120_e49268_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign34130_e49275, assign34130_e49275_d_n0, assign34130_e49275_d_n2, assign34130_e49275_d_n6, assign34130_e49275_d_n7, assign34130_e49275_d_n10, assign34130_e49275_d_n11, assign34130_e49275_d_n12, assign34130_e49275_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        let assign34130_e49273: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34130_e49273, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34130_e49275;
        locals.var_qgos_dn0 = assign34130_e49275_d_n0;
        locals.var_qgos_dn2 = assign34130_e49275_d_n2;
        locals.var_qgos_dn6 = assign34130_e49275_d_n6;
        locals.var_qgos_dn7 = assign34130_e49275_d_n7;
        locals.var_qgos_dn10 = assign34130_e49275_d_n10;
        locals.var_qgos_dn11 = assign34130_e49275_d_n11;
        locals.var_qgos_dn12 = assign34130_e49275_d_n12;
        locals.var_qgos_dn17 = assign34130_e49275_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign34140_e49278: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign34140_e49278;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn12 = (locals.var_mfactor * locals.var_ids_dn12);
        locals.var_idse_dn17 = (locals.var_mfactor * locals.var_ids_dn17);
        locals.var_idse_rv = 0.0;

        let (assign34150_e49282, assign34150_e49282_d_n0, assign34150_e49282_d_n2, assign34150_e49282_d_n6, assign34150_e49282_d_n7, assign34150_e49282_d_n10, assign34150_e49282_d_n11, assign34150_e49282_d_n12, assign34150_e49282_d_n13, assign34150_e49282_d_n15, assign34150_e49282_d_n16, assign34150_e49282_d_n17, assign34150_e49282_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34150_e49282;
        locals.var_qde_dn0 = assign34150_e49282_d_n0;
        locals.var_qde_dn2 = assign34150_e49282_d_n2;
        locals.var_qde_dn6 = assign34150_e49282_d_n6;
        locals.var_qde_dn7 = assign34150_e49282_d_n7;
        locals.var_qde_dn10 = assign34150_e49282_d_n10;
        locals.var_qde_dn11 = assign34150_e49282_d_n11;
        locals.var_qde_dn12 = assign34150_e49282_d_n12;
        locals.var_qde_dn13 = assign34150_e49282_d_n13;
        locals.var_qde_dn15 = assign34150_e49282_d_n15;
        locals.var_qde_dn16 = assign34150_e49282_d_n16;
        locals.var_qde_dn17 = assign34150_e49282_d_n17;
        locals.var_qde_dn18 = assign34150_e49282_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34160_e49286, assign34160_e49286_d_n0, assign34160_e49286_d_n2, assign34160_e49286_d_n6, assign34160_e49286_d_n7, assign34160_e49286_d_n10, assign34160_e49286_d_n11, assign34160_e49286_d_n12, assign34160_e49286_d_n13, assign34160_e49286_d_n15, assign34160_e49286_d_n16, assign34160_e49286_d_n17, assign34160_e49286_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34160_e49286;
        locals.var_qge_dn0 = assign34160_e49286_d_n0;
        locals.var_qge_dn2 = assign34160_e49286_d_n2;
        locals.var_qge_dn6 = assign34160_e49286_d_n6;
        locals.var_qge_dn7 = assign34160_e49286_d_n7;
        locals.var_qge_dn10 = assign34160_e49286_d_n10;
        locals.var_qge_dn11 = assign34160_e49286_d_n11;
        locals.var_qge_dn12 = assign34160_e49286_d_n12;
        locals.var_qge_dn13 = assign34160_e49286_d_n13;
        locals.var_qge_dn15 = assign34160_e49286_d_n15;
        locals.var_qge_dn16 = assign34160_e49286_d_n16;
        locals.var_qge_dn17 = assign34160_e49286_d_n17;
        locals.var_qge_dn18 = assign34160_e49286_d_n18;
        locals.var_qge_rv = 0.0;

        let assign34170_e49289: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign34170_e49289;
        locals.var_guard1125_rv = 0.0;

        let (assign34180_e49295, assign34180_e49295_d_n0, assign34180_e49295_d_n2, assign34180_e49295_d_n6, assign34180_e49295_d_n7, assign34180_e49295_d_n10, assign34180_e49295_d_n11, assign34180_e49295_d_n12, assign34180_e49295_d_n13, assign34180_e49295_d_n15, assign34180_e49295_d_n16, assign34180_e49295_d_n17, assign34180_e49295_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34180_e49295;
        locals.var_qse_dn0 = assign34180_e49295_d_n0;
        locals.var_qse_dn2 = assign34180_e49295_d_n2;
        locals.var_qse_dn6 = assign34180_e49295_d_n6;
        locals.var_qse_dn7 = assign34180_e49295_d_n7;
        locals.var_qse_dn10 = assign34180_e49295_d_n10;
        locals.var_qse_dn11 = assign34180_e49295_d_n11;
        locals.var_qse_dn12 = assign34180_e49295_d_n12;
        locals.var_qse_dn13 = assign34180_e49295_d_n13;
        locals.var_qse_dn15 = assign34180_e49295_d_n15;
        locals.var_qse_dn16 = assign34180_e49295_d_n16;
        locals.var_qse_dn17 = assign34180_e49295_d_n17;
        locals.var_qse_dn18 = assign34180_e49295_d_n18;
        locals.var_qse_rv = 0.0;

        let (assign34190_e49301, assign34190_e49301_d_n0, assign34190_e49301_d_n2, assign34190_e49301_d_n6, assign34190_e49301_d_n7, assign34190_e49301_d_n10, assign34190_e49301_d_n11, assign34190_e49301_d_n12, assign34190_e49301_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    }
};
        locals.var_xd = assign34190_e49301;
        locals.var_xd_dn0 = assign34190_e49301_d_n0;
        locals.var_xd_dn2 = assign34190_e49301_d_n2;
        locals.var_xd_dn6 = assign34190_e49301_d_n6;
        locals.var_xd_dn7 = assign34190_e49301_d_n7;
        locals.var_xd_dn10 = assign34190_e49301_d_n10;
        locals.var_xd_dn11 = assign34190_e49301_d_n11;
        locals.var_xd_dn12 = assign34190_e49301_d_n12;
        locals.var_xd_dn17 = assign34190_e49301_d_n17;
        locals.var_xd_rv = 0.0;

        let (assign34220_e49324, assign34220_e49324_d_n0, assign34220_e49324_d_n2, assign34220_e49324_d_n6, assign34220_e49324_d_n7, assign34220_e49324_d_n10, assign34220_e49324_d_n11, assign34220_e49324_d_n12, assign34220_e49324_d_n13, assign34220_e49324_d_n15, assign34220_e49324_d_n16, assign34220_e49324_d_n17, assign34220_e49324_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign34220_e49324;
        locals.var_qbe_dn0 = assign34220_e49324_d_n0;
        locals.var_qbe_dn2 = assign34220_e49324_d_n2;
        locals.var_qbe_dn6 = assign34220_e49324_d_n6;
        locals.var_qbe_dn7 = assign34220_e49324_d_n7;
        locals.var_qbe_dn10 = assign34220_e49324_d_n10;
        locals.var_qbe_dn11 = assign34220_e49324_d_n11;
        locals.var_qbe_dn12 = assign34220_e49324_d_n12;
        locals.var_qbe_dn13 = assign34220_e49324_d_n13;
        locals.var_qbe_dn15 = assign34220_e49324_d_n15;
        locals.var_qbe_dn16 = assign34220_e49324_d_n16;
        locals.var_qbe_dn17 = assign34220_e49324_d_n17;
        locals.var_qbe_dn18 = assign34220_e49324_d_n18;
        locals.var_qbe_rv = 0.0;

        let assign34260_e49360: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign34260_e49360;
        locals.var_guard1126_rv = 0.0;

        let (assign34270_e49372, assign34270_e49372_d_n0, assign34270_e49372_d_n2, assign34270_e49372_d_n6, assign34270_e49372_d_n7, assign34270_e49372_d_n10, assign34270_e49372_d_n11, assign34270_e49372_d_n12, assign34270_e49372_d_n13, assign34270_e49372_d_n15, assign34270_e49372_d_n16, assign34270_e49372_d_n17, assign34270_e49372_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34270_e49367: f64 = (-locals.var_qb);
        let assign34270_e49369: f64 = (assign34270_e49367 - locals.var_qi);
        let assign34270_e49370: f64 = (locals.var_mfactor * assign34270_e49369);
        (assign34270_e49370, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34270_e49372;
        locals.var_qge_dn0 = assign34270_e49372_d_n0;
        locals.var_qge_dn2 = assign34270_e49372_d_n2;
        locals.var_qge_dn6 = assign34270_e49372_d_n6;
        locals.var_qge_dn7 = assign34270_e49372_d_n7;
        locals.var_qge_dn10 = assign34270_e49372_d_n10;
        locals.var_qge_dn11 = assign34270_e49372_d_n11;
        locals.var_qge_dn12 = assign34270_e49372_d_n12;
        locals.var_qge_dn13 = assign34270_e49372_d_n13;
        locals.var_qge_dn15 = assign34270_e49372_d_n15;
        locals.var_qge_dn16 = assign34270_e49372_d_n16;
        locals.var_qge_dn17 = assign34270_e49372_d_n17;
        locals.var_qge_dn18 = assign34270_e49372_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34280_e49381, assign34280_e49381_d_n0, assign34280_e49381_d_n2, assign34280_e49381_d_n6, assign34280_e49381_d_n7, assign34280_e49381_d_n10, assign34280_e49381_d_n11, assign34280_e49381_d_n12, assign34280_e49381_d_n13, assign34280_e49381_d_n15, assign34280_e49381_d_n16, assign34280_e49381_d_n17, assign34280_e49381_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34280_e49379: f64 = (locals.var_mfactor * locals.var_qd);
        (assign34280_e49379, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34280_e49381;
        locals.var_qde_dn0 = assign34280_e49381_d_n0;
        locals.var_qde_dn2 = assign34280_e49381_d_n2;
        locals.var_qde_dn6 = assign34280_e49381_d_n6;
        locals.var_qde_dn7 = assign34280_e49381_d_n7;
        locals.var_qde_dn10 = assign34280_e49381_d_n10;
        locals.var_qde_dn11 = assign34280_e49381_d_n11;
        locals.var_qde_dn12 = assign34280_e49381_d_n12;
        locals.var_qde_dn13 = assign34280_e49381_d_n13;
        locals.var_qde_dn15 = assign34280_e49381_d_n15;
        locals.var_qde_dn16 = assign34280_e49381_d_n16;
        locals.var_qde_dn17 = assign34280_e49381_d_n17;
        locals.var_qde_dn18 = assign34280_e49381_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34290_e49392, assign34290_e49392_d_n0, assign34290_e49392_d_n2, assign34290_e49392_d_n6, assign34290_e49392_d_n7, assign34290_e49392_d_n10, assign34290_e49392_d_n11, assign34290_e49392_d_n12, assign34290_e49392_d_n13, assign34290_e49392_d_n15, assign34290_e49392_d_n16, assign34290_e49392_d_n17, assign34290_e49392_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
        let assign34290_e49389: f64 = (locals.var_qi - locals.var_qd);
        let assign34290_e49390: f64 = (locals.var_mfactor * assign34290_e49389);
        (assign34290_e49390, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34290_e49392;
        locals.var_qse_dn0 = assign34290_e49392_d_n0;
        locals.var_qse_dn2 = assign34290_e49392_d_n2;
        locals.var_qse_dn6 = assign34290_e49392_d_n6;
        locals.var_qse_dn7 = assign34290_e49392_d_n7;
        locals.var_qse_dn10 = assign34290_e49392_d_n10;
        locals.var_qse_dn11 = assign34290_e49392_d_n11;
        locals.var_qse_dn12 = assign34290_e49392_d_n12;
        locals.var_qse_dn13 = assign34290_e49392_d_n13;
        locals.var_qse_dn15 = assign34290_e49392_d_n15;
        locals.var_qse_dn16 = assign34290_e49392_d_n16;
        locals.var_qse_dn17 = assign34290_e49392_d_n17;
        locals.var_qse_dn18 = assign34290_e49392_d_n18;
        locals.var_qse_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_124(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34300_e49409, assign34300_e49409_d_n0, assign34300_e49409_d_n2, assign34300_e49409_d_n6, assign34300_e49409_d_n7, assign34300_e49409_d_n10, assign34300_e49409_d_n11, assign34300_e49409_d_n12, assign34300_e49409_d_n13, assign34300_e49409_d_n15, assign34300_e49409_d_n16, assign34300_e49409_d_n17, assign34300_e49409_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34300_e49400: f64 = (-locals.var_qsub);
        let assign34300_e49402: f64 = (assign34300_e49400 - locals.var_qi);
        let assign34300_e49404: f64 = (assign34300_e49402 - locals.var_qs_fb);
        let assign34300_e49406: f64 = (assign34300_e49404 - locals.var_qd_fb);
        let assign34300_e49407: f64 = (locals.var_mfactor * assign34300_e49406);
        (assign34300_e49407, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34300_e49409;
        locals.var_qge_dn0 = assign34300_e49409_d_n0;
        locals.var_qge_dn2 = assign34300_e49409_d_n2;
        locals.var_qge_dn6 = assign34300_e49409_d_n6;
        locals.var_qge_dn7 = assign34300_e49409_d_n7;
        locals.var_qge_dn10 = assign34300_e49409_d_n10;
        locals.var_qge_dn11 = assign34300_e49409_d_n11;
        locals.var_qge_dn12 = assign34300_e49409_d_n12;
        locals.var_qge_dn13 = assign34300_e49409_d_n13;
        locals.var_qge_dn15 = assign34300_e49409_d_n15;
        locals.var_qge_dn16 = assign34300_e49409_d_n16;
        locals.var_qge_dn17 = assign34300_e49409_d_n17;
        locals.var_qge_dn18 = assign34300_e49409_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34310_e49421, assign34310_e49421_d_n0, assign34310_e49421_d_n2, assign34310_e49421_d_n6, assign34310_e49421_d_n7, assign34310_e49421_d_n10, assign34310_e49421_d_n11, assign34310_e49421_d_n12, assign34310_e49421_d_n13, assign34310_e49421_d_n15, assign34310_e49421_d_n16, assign34310_e49421_d_n17, assign34310_e49421_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34310_e49418: f64 = (locals.var_qd + locals.var_qd_fb);
        let assign34310_e49419: f64 = (locals.var_mfactor * assign34310_e49418);
        (assign34310_e49419, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34310_e49421;
        locals.var_qde_dn0 = assign34310_e49421_d_n0;
        locals.var_qde_dn2 = assign34310_e49421_d_n2;
        locals.var_qde_dn6 = assign34310_e49421_d_n6;
        locals.var_qde_dn7 = assign34310_e49421_d_n7;
        locals.var_qde_dn10 = assign34310_e49421_d_n10;
        locals.var_qde_dn11 = assign34310_e49421_d_n11;
        locals.var_qde_dn12 = assign34310_e49421_d_n12;
        locals.var_qde_dn13 = assign34310_e49421_d_n13;
        locals.var_qde_dn15 = assign34310_e49421_d_n15;
        locals.var_qde_dn16 = assign34310_e49421_d_n16;
        locals.var_qde_dn17 = assign34310_e49421_d_n17;
        locals.var_qde_dn18 = assign34310_e49421_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34320_e49435, assign34320_e49435_d_n0, assign34320_e49435_d_n2, assign34320_e49435_d_n6, assign34320_e49435_d_n7, assign34320_e49435_d_n10, assign34320_e49435_d_n11, assign34320_e49435_d_n12, assign34320_e49435_d_n13, assign34320_e49435_d_n15, assign34320_e49435_d_n16, assign34320_e49435_d_n17, assign34320_e49435_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
        let assign34320_e49430: f64 = (locals.var_qi - locals.var_qd);
        let assign34320_e49432: f64 = (assign34320_e49430 + locals.var_qs_fb);
        let assign34320_e49433: f64 = (locals.var_mfactor * assign34320_e49432);
        (assign34320_e49433, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34320_e49435;
        locals.var_qse_dn0 = assign34320_e49435_d_n0;
        locals.var_qse_dn2 = assign34320_e49435_d_n2;
        locals.var_qse_dn6 = assign34320_e49435_d_n6;
        locals.var_qse_dn7 = assign34320_e49435_d_n7;
        locals.var_qse_dn10 = assign34320_e49435_d_n10;
        locals.var_qse_dn11 = assign34320_e49435_d_n11;
        locals.var_qse_dn12 = assign34320_e49435_d_n12;
        locals.var_qse_dn13 = assign34320_e49435_d_n13;
        locals.var_qse_dn15 = assign34320_e49435_d_n15;
        locals.var_qse_dn16 = assign34320_e49435_d_n16;
        locals.var_qse_dn17 = assign34320_e49435_d_n17;
        locals.var_qse_dn18 = assign34320_e49435_d_n18;
        locals.var_qse_rv = 0.0;

        let assign34330_e49438: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign34330_e49438;
        locals.var_guard1132_rv = 0.0;

        let (assign34340_e49442, assign34340_e49442_d_n0, assign34340_e49442_d_n2, assign34340_e49442_d_n6, assign34340_e49442_d_n7, assign34340_e49442_d_n10, assign34340_e49442_d_n11, assign34340_e49442_d_n12, assign34340_e49442_d_n17,) = {
    if (locals.var_guard1132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34340_e49442;
        locals.var_qy_dn0 = assign34340_e49442_d_n0;
        locals.var_qy_dn2 = assign34340_e49442_d_n2;
        locals.var_qy_dn6 = assign34340_e49442_d_n6;
        locals.var_qy_dn7 = assign34340_e49442_d_n7;
        locals.var_qy_dn10 = assign34340_e49442_d_n10;
        locals.var_qy_dn11 = assign34340_e49442_d_n11;
        locals.var_qy_dn12 = assign34340_e49442_d_n12;
        locals.var_qy_dn17 = assign34340_e49442_d_n17;
        locals.var_qy_rv = 0.0;

        let (assign34350_e49451, assign34350_e49451_d_n0, assign34350_e49451_d_n2, assign34350_e49451_d_n6, assign34350_e49451_d_n7, assign34350_e49451_d_n10, assign34350_e49451_d_n11, assign34350_e49451_d_n12, assign34350_e49451_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34350_e49447: f64 = (locals.var_ec * locals.var_leff);
        let assign34350_e49449: f64 = (assign34350_e49447 + locals.var_ps0);
        (assign34350_e49449, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34350_e49451;
        locals.var_pslk_dn0 = assign34350_e49451_d_n0;
        locals.var_pslk_dn2 = assign34350_e49451_d_n2;
        locals.var_pslk_dn6 = assign34350_e49451_d_n6;
        locals.var_pslk_dn7 = assign34350_e49451_d_n7;
        locals.var_pslk_dn10 = assign34350_e49451_d_n10;
        locals.var_pslk_dn11 = assign34350_e49451_d_n11;
        locals.var_pslk_dn12 = assign34350_e49451_d_n12;
        locals.var_pslk_dn17 = assign34350_e49451_d_n17;
        locals.var_pslk_rv = 0.0;

        let assign34360_e49454: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign34360_e49454;
        locals.var_guard1133_rv = 0.0;

        let (assign34370_e49461, assign34370_e49461_d_n0, assign34370_e49461_d_n2, assign34370_e49461_d_n6, assign34370_e49461_d_n7, assign34370_e49461_d_n10, assign34370_e49461_d_n11, assign34370_e49461_d_n12, assign34370_e49461_d_n17,) = {
    if ((locals.var_guard1132 == 0.0) && (locals.var_guard1133 != 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34370_e49461;
        locals.var_pslk_dn0 = assign34370_e49461_d_n0;
        locals.var_pslk_dn2 = assign34370_e49461_d_n2;
        locals.var_pslk_dn6 = assign34370_e49461_d_n6;
        locals.var_pslk_dn7 = assign34370_e49461_d_n7;
        locals.var_pslk_dn10 = assign34370_e49461_d_n10;
        locals.var_pslk_dn11 = assign34370_e49461_d_n11;
        locals.var_pslk_dn12 = assign34370_e49461_d_n12;
        locals.var_pslk_dn17 = assign34370_e49461_d_n17;
        locals.var_pslk_rv = 0.0;

        let (assign34380_e49476, assign34380_e49476_d_n0, assign34380_e49476_d_n2, assign34380_e49476_d_n6, assign34380_e49476_d_n7, assign34380_e49476_d_n10, assign34380_e49476_d_n11, assign34380_e49476_d_n12, assign34380_e49476_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34380_e49467: f64 = (locals.var_vds + locals.var_ps0);
        let assign34380_e49468: f64 = (locals.var_aclm * assign34380_e49467);
        let assign34380_e49471: f64 = (1.0 - locals.var_aclm);
        let assign34380_e49473: f64 = (assign34380_e49471 * locals.var_pslk);
        let assign34380_e49474: f64 = (assign34380_e49468 + assign34380_e49473);
        (assign34380_e49474, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34380_e49471 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34380_e49471 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34380_e49471 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34380_e49471 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34380_e49471 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34380_e49471 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34380_e49471 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34380_e49471 * locals.var_pslk_dn17)),)
    } else {
        (locals.var_t1__blk1128, locals.var_t1__blk1128_dn0, locals.var_t1__blk1128_dn2, locals.var_t1__blk1128_dn6, locals.var_t1__blk1128_dn7, locals.var_t1__blk1128_dn10, locals.var_t1__blk1128_dn11, locals.var_t1__blk1128_dn12, locals.var_t1__blk1128_dn17,)
    }
};
        locals.var_t1__blk1128 = assign34380_e49476;
        locals.var_t1__blk1128_dn0 = assign34380_e49476_d_n0;
        locals.var_t1__blk1128_dn2 = assign34380_e49476_d_n2;
        locals.var_t1__blk1128_dn6 = assign34380_e49476_d_n6;
        locals.var_t1__blk1128_dn7 = assign34380_e49476_d_n7;
        locals.var_t1__blk1128_dn10 = assign34380_e49476_d_n10;
        locals.var_t1__blk1128_dn11 = assign34380_e49476_d_n11;
        locals.var_t1__blk1128_dn12 = assign34380_e49476_d_n12;
        locals.var_t1__blk1128_dn17 = assign34380_e49476_d_n17;
        locals.var_t1__blk1128_rv = 0.0;

        let (assign34390_e49486, assign34390_e49486_d_n0, assign34390_e49486_d_n2, assign34390_e49486_d_n6, assign34390_e49486_d_n7, assign34390_e49486_d_n10, assign34390_e49486_d_n11, assign34390_e49486_d_n12, assign34390_e49486_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34390_e49481: f64 = (2.0 * 1.034943e-10);
        let assign34390_e49483: f64 = (assign34390_e49481 / locals.var_q_nsub);
        let assign34390_e49484: f64 = (assign34390_e49483).sqrt();
        (assign34390_e49484, ((-((assign34390_e49481 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)),)
    } else {
        (locals.var_t10__blk1129, locals.var_t10__blk1129_dn0, locals.var_t10__blk1129_dn2, locals.var_t10__blk1129_dn6, locals.var_t10__blk1129_dn7, locals.var_t10__blk1129_dn10, locals.var_t10__blk1129_dn11, locals.var_t10__blk1129_dn12, locals.var_t10__blk1129_dn17,)
    }
};
        locals.var_t10__blk1129 = assign34390_e49486;
        locals.var_t10__blk1129_dn0 = assign34390_e49486_d_n0;
        locals.var_t10__blk1129_dn2 = assign34390_e49486_d_n2;
        locals.var_t10__blk1129_dn6 = assign34390_e49486_d_n6;
        locals.var_t10__blk1129_dn7 = assign34390_e49486_d_n7;
        locals.var_t10__blk1129_dn10 = assign34390_e49486_d_n10;
        locals.var_t10__blk1129_dn11 = assign34390_e49486_d_n11;
        locals.var_t10__blk1129_dn12 = assign34390_e49486_d_n12;
        locals.var_t10__blk1129_dn17 = assign34390_e49486_d_n17;
        locals.var_t10__blk1129_rv = 0.0;

        let (assign34400_e49493, assign34400_e49493_d_n0, assign34400_e49493_d_n2, assign34400_e49493_d_n6, assign34400_e49493_d_n7, assign34400_e49493_d_n10, assign34400_e49493_d_n11, assign34400_e49493_d_n12, assign34400_e49493_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34400_e49491: f64 = (locals.var_t10__blk1129 * 1.3);
        (assign34400_e49491, (locals.var_t10__blk1129_dn0 * 1.3), (locals.var_t10__blk1129_dn2 * 1.3), (locals.var_t10__blk1129_dn6 * 1.3), (locals.var_t10__blk1129_dn7 * 1.3), (locals.var_t10__blk1129_dn10 * 1.3), (locals.var_t10__blk1129_dn11 * 1.3), (locals.var_t10__blk1129_dn12 * 1.3), (locals.var_t10__blk1129_dn17 * 1.3),)
    } else {
        (locals.var_t3__blk1130, locals.var_t3__blk1130_dn0, locals.var_t3__blk1130_dn2, locals.var_t3__blk1130_dn6, locals.var_t3__blk1130_dn7, locals.var_t3__blk1130_dn10, locals.var_t3__blk1130_dn11, locals.var_t3__blk1130_dn12, locals.var_t3__blk1130_dn17,)
    }
};
        locals.var_t3__blk1130 = assign34400_e49493;
        locals.var_t3__blk1130_dn0 = assign34400_e49493_d_n0;
        locals.var_t3__blk1130_dn2 = assign34400_e49493_d_n2;
        locals.var_t3__blk1130_dn6 = assign34400_e49493_d_n6;
        locals.var_t3__blk1130_dn7 = assign34400_e49493_d_n7;
        locals.var_t3__blk1130_dn10 = assign34400_e49493_d_n10;
        locals.var_t3__blk1130_dn11 = assign34400_e49493_d_n11;
        locals.var_t3__blk1130_dn12 = assign34400_e49493_d_n12;
        locals.var_t3__blk1130_dn17 = assign34400_e49493_d_n17;
        locals.var_t3__blk1130_rv = 0.0;

        let (assign34410_e49502, assign34410_e49502_d_n0, assign34410_e49502_d_n2, assign34410_e49502_d_n6, assign34410_e49502_d_n7, assign34410_e49502_d_n10, assign34410_e49502_d_n11, assign34410_e49502_d_n12, assign34410_e49502_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34410_e49498: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign34410_e49500: f64 = (assign34410_e49498 * locals.var_t3__blk1130);
        (assign34410_e49500, (assign34410_e49498 * locals.var_t3__blk1130_dn0), (assign34410_e49498 * locals.var_t3__blk1130_dn2), (assign34410_e49498 * locals.var_t3__blk1130_dn6), (assign34410_e49498 * locals.var_t3__blk1130_dn7), (assign34410_e49498 * locals.var_t3__blk1130_dn10), (assign34410_e49498 * locals.var_t3__blk1130_dn11), (assign34410_e49498 * locals.var_t3__blk1130_dn12), (assign34410_e49498 * locals.var_t3__blk1130_dn17),)
    } else {
        (locals.var_t2__blk1131, locals.var_t2__blk1131_dn0, locals.var_t2__blk1131_dn2, locals.var_t2__blk1131_dn6, locals.var_t2__blk1131_dn7, locals.var_t2__blk1131_dn10, locals.var_t2__blk1131_dn11, locals.var_t2__blk1131_dn12, locals.var_t2__blk1131_dn17,)
    }
};
        locals.var_t2__blk1131 = assign34410_e49502;
        locals.var_t2__blk1131_dn0 = assign34410_e49502_d_n0;
        locals.var_t2__blk1131_dn2 = assign34410_e49502_d_n2;
        locals.var_t2__blk1131_dn6 = assign34410_e49502_d_n6;
        locals.var_t2__blk1131_dn7 = assign34410_e49502_d_n7;
        locals.var_t2__blk1131_dn10 = assign34410_e49502_d_n10;
        locals.var_t2__blk1131_dn11 = assign34410_e49502_d_n11;
        locals.var_t2__blk1131_dn12 = assign34410_e49502_d_n12;
        locals.var_t2__blk1131_dn17 = assign34410_e49502_d_n17;
        locals.var_t2__blk1131_rv = 0.0;

        let (assign34420_e49517, assign34420_e49517_d_n0, assign34420_e49517_d_n2, assign34420_e49517_d_n6, assign34420_e49517_d_n7, assign34420_e49517_d_n10, assign34420_e49517_d_n11, assign34420_e49517_d_n12, assign34420_e49517_d_n17,) = {
    if (locals.var_guard1132 == 0.0) {
        let assign34420_e49507: f64 = (locals.var_ps0 + locals.var_vds);
        let assign34420_e49509: f64 = (assign34420_e49507 - locals.var_t1__blk1128);
        let assign34420_e49511: f64 = (assign34420_e49509 / p.p64);
        let assign34420_e49513: f64 = (assign34420_e49511 - locals.var_ec);
        let assign34420_e49515: f64 = (assign34420_e49513 * locals.var_t2__blk1131);
        (assign34420_e49515, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1128_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1128_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1128_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1128_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1128_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1128_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1128_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1128_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34420_e49517;
        locals.var_qy_dn0 = assign34420_e49517_d_n0;
        locals.var_qy_dn2 = assign34420_e49517_d_n2;
        locals.var_qy_dn6 = assign34420_e49517_d_n6;
        locals.var_qy_dn7 = assign34420_e49517_d_n7;
        locals.var_qy_dn10 = assign34420_e49517_d_n10;
        locals.var_qy_dn11 = assign34420_e49517_d_n11;
        locals.var_qy_dn12 = assign34420_e49517_d_n12;
        locals.var_qy_dn17 = assign34420_e49517_d_n17;
        locals.var_qy_rv = 0.0;

        let assign34430_e49520: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34430_e49520;
        locals.var_guard1134_rv = 0.0;

        let (assign34440_e49528, assign34440_e49528_d_n0, assign34440_e49528_d_n2, assign34440_e49528_d_n6, assign34440_e49528_d_n7, assign34440_e49528_d_n10, assign34440_e49528_d_n11, assign34440_e49528_d_n12, assign34440_e49528_d_n17,) = {
    if (locals.var_guard1134 != 0.0) {
        let assign34440_e49525: f64 = (locals.var_cqyb0 * locals.var_vbsp);
        let assign34440_e49526: f64 = (locals.var_qy + assign34440_e49525);
        (assign34440_e49526, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34440_e49528;
        locals.var_qy_dn0 = assign34440_e49528_d_n0;
        locals.var_qy_dn2 = assign34440_e49528_d_n2;
        locals.var_qy_dn6 = assign34440_e49528_d_n6;
        locals.var_qy_dn7 = assign34440_e49528_d_n7;
        locals.var_qy_dn10 = assign34440_e49528_d_n10;
        locals.var_qy_dn11 = assign34440_e49528_d_n11;
        locals.var_qy_dn12 = assign34440_e49528_d_n12;
        locals.var_qy_dn17 = assign34440_e49528_d_n17;
        locals.var_qy_rv = 0.0;

        let assign34450_e49531: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34450_e49531;
        locals.var_guard1135_rv = 0.0;

        let assign34460_e49534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34460_e49534;
        locals.var_guard1136_rv = 0.0;

        let (assign34470_e49547, assign34470_e49547_d_n0, assign34470_e49547_d_n2, assign34470_e49547_d_n6, assign34470_e49547_d_n7, assign34470_e49547_d_n10, assign34470_e49547_d_n11, assign34470_e49547_d_n12, assign34470_e49547_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34470_e49539: f64 = (-locals.var_qbody_bt_p_sus);
        let assign34470_e49541: f64 = (assign34470_e49539 - locals.var_qbody_bt_p_sud);
        let assign34470_e49543: f64 = (assign34470_e49541 - locals.var_qbody_bt_n_sus);
        let assign34470_e49545: f64 = (assign34470_e49543 - locals.var_qbody_bt_n_sud);
        (assign34470_e49545, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17,)
    }
};
        locals.var_q_bt_ge = assign34470_e49547;
        locals.var_q_bt_ge_dn0 = assign34470_e49547_d_n0;
        locals.var_q_bt_ge_dn2 = assign34470_e49547_d_n2;
        locals.var_q_bt_ge_dn6 = assign34470_e49547_d_n6;
        locals.var_q_bt_ge_dn7 = assign34470_e49547_d_n7;
        locals.var_q_bt_ge_dn10 = assign34470_e49547_d_n10;
        locals.var_q_bt_ge_dn11 = assign34470_e49547_d_n11;
        locals.var_q_bt_ge_dn12 = assign34470_e49547_d_n12;
        locals.var_q_bt_ge_dn17 = assign34470_e49547_d_n17;
        locals.var_q_bt_ge_rv = 0.0;

        let (assign34480_e49555, assign34480_e49555_d_n0, assign34480_e49555_d_n2, assign34480_e49555_d_n6, assign34480_e49555_d_n7, assign34480_e49555_d_n10, assign34480_e49555_d_n11, assign34480_e49555_d_n12, assign34480_e49555_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34480_e49553: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
        (assign34480_e49553, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17,)
    }
};
        locals.var_q_bt_de = assign34480_e49555;
        locals.var_q_bt_de_dn0 = assign34480_e49555_d_n0;
        locals.var_q_bt_de_dn2 = assign34480_e49555_d_n2;
        locals.var_q_bt_de_dn6 = assign34480_e49555_d_n6;
        locals.var_q_bt_de_dn7 = assign34480_e49555_d_n7;
        locals.var_q_bt_de_dn10 = assign34480_e49555_d_n10;
        locals.var_q_bt_de_dn11 = assign34480_e49555_d_n11;
        locals.var_q_bt_de_dn12 = assign34480_e49555_d_n12;
        locals.var_q_bt_de_dn17 = assign34480_e49555_d_n17;
        locals.var_q_bt_de_rv = 0.0;

        let (assign34490_e49563, assign34490_e49563_d_n0, assign34490_e49563_d_n2, assign34490_e49563_d_n6, assign34490_e49563_d_n7, assign34490_e49563_d_n10, assign34490_e49563_d_n11, assign34490_e49563_d_n12, assign34490_e49563_d_n17,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34490_e49561: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
        (assign34490_e49561, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17),)
    } else {
        (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17,)
    }
};
        locals.var_q_bt_se = assign34490_e49563;
        locals.var_q_bt_se_dn0 = assign34490_e49563_d_n0;
        locals.var_q_bt_se_dn2 = assign34490_e49563_d_n2;
        locals.var_q_bt_se_dn6 = assign34490_e49563_d_n6;
        locals.var_q_bt_se_dn7 = assign34490_e49563_d_n7;
        locals.var_q_bt_se_dn10 = assign34490_e49563_d_n10;
        locals.var_q_bt_se_dn11 = assign34490_e49563_d_n11;
        locals.var_q_bt_se_dn12 = assign34490_e49563_d_n12;
        locals.var_q_bt_se_dn17 = assign34490_e49563_d_n17;
        locals.var_q_bt_se_rv = 0.0;

        let (assign34500_e49585, assign34500_e49585_d_n0, assign34500_e49585_d_n2, assign34500_e49585_d_n6, assign34500_e49585_d_n7, assign34500_e49585_d_n10, assign34500_e49585_d_n11, assign34500_e49585_d_n12, assign34500_e49585_d_n13, assign34500_e49585_d_n15, assign34500_e49585_d_n16, assign34500_e49585_d_n17, assign34500_e49585_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34500_e49571: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34500_e49573: f64 = (assign34500_e49571 + locals.var_qgob);
        let assign34500_e49575: f64 = (assign34500_e49573 - locals.var_qy);
        let assign34500_e49577: f64 = (assign34500_e49575 - locals.var_qovs);
        let assign34500_e49579: f64 = (assign34500_e49577 - locals.var_qovd);
        let assign34500_e49581: f64 = (assign34500_e49579 + locals.var_q_bt_ge);
        let assign34500_e49582: f64 = (locals.var_mfactor * assign34500_e49581);
        let assign34500_e49583: f64 = (locals.var_qge + assign34500_e49582);
        (assign34500_e49583, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34500_e49585;
        locals.var_qge_dn0 = assign34500_e49585_d_n0;
        locals.var_qge_dn2 = assign34500_e49585_d_n2;
        locals.var_qge_dn6 = assign34500_e49585_d_n6;
        locals.var_qge_dn7 = assign34500_e49585_d_n7;
        locals.var_qge_dn10 = assign34500_e49585_d_n10;
        locals.var_qge_dn11 = assign34500_e49585_d_n11;
        locals.var_qge_dn12 = assign34500_e49585_d_n12;
        locals.var_qge_dn13 = assign34500_e49585_d_n13;
        locals.var_qge_dn15 = assign34500_e49585_d_n15;
        locals.var_qge_dn16 = assign34500_e49585_d_n16;
        locals.var_qge_dn17 = assign34500_e49585_d_n17;
        locals.var_qge_dn18 = assign34500_e49585_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34510_e49602, assign34510_e49602_d_n0, assign34510_e49602_d_n2, assign34510_e49602_d_n6, assign34510_e49602_d_n7, assign34510_e49602_d_n10, assign34510_e49602_d_n11, assign34510_e49602_d_n12, assign34510_e49602_d_n13, assign34510_e49602_d_n15, assign34510_e49602_d_n16, assign34510_e49602_d_n17, assign34510_e49602_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34510_e49592: f64 = (-locals.var_qgod);
        let assign34510_e49594: f64 = (assign34510_e49592 + locals.var_qy);
        let assign34510_e49596: f64 = (assign34510_e49594 + locals.var_qbdld);
        let assign34510_e49598: f64 = (assign34510_e49596 + locals.var_q_bt_de);
        let assign34510_e49599: f64 = (locals.var_mfactor * assign34510_e49598);
        let assign34510_e49600: f64 = (locals.var_qde + assign34510_e49599);
        (assign34510_e49600, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34510_e49602;
        locals.var_qde_dn0 = assign34510_e49602_d_n0;
        locals.var_qde_dn2 = assign34510_e49602_d_n2;
        locals.var_qde_dn6 = assign34510_e49602_d_n6;
        locals.var_qde_dn7 = assign34510_e49602_d_n7;
        locals.var_qde_dn10 = assign34510_e49602_d_n10;
        locals.var_qde_dn11 = assign34510_e49602_d_n11;
        locals.var_qde_dn12 = assign34510_e49602_d_n12;
        locals.var_qde_dn13 = assign34510_e49602_d_n13;
        locals.var_qde_dn15 = assign34510_e49602_d_n15;
        locals.var_qde_dn16 = assign34510_e49602_d_n16;
        locals.var_qde_dn17 = assign34510_e49602_d_n17;
        locals.var_qde_dn18 = assign34510_e49602_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34520_e49617, assign34520_e49617_d_n0, assign34520_e49617_d_n2, assign34520_e49617_d_n6, assign34520_e49617_d_n7, assign34520_e49617_d_n10, assign34520_e49617_d_n11, assign34520_e49617_d_n12, assign34520_e49617_d_n13, assign34520_e49617_d_n15, assign34520_e49617_d_n16, assign34520_e49617_d_n17, assign34520_e49617_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
        let assign34520_e49609: f64 = (-locals.var_qgos);
        let assign34520_e49611: f64 = (assign34520_e49609 + locals.var_qbsld);
        let assign34520_e49613: f64 = (assign34520_e49611 + locals.var_q_bt_se);
        let assign34520_e49614: f64 = (locals.var_mfactor * assign34520_e49613);
        let assign34520_e49615: f64 = (locals.var_qse + assign34520_e49614);
        (assign34520_e49615, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34520_e49617;
        locals.var_qse_dn0 = assign34520_e49617_d_n0;
        locals.var_qse_dn2 = assign34520_e49617_d_n2;
        locals.var_qse_dn6 = assign34520_e49617_d_n6;
        locals.var_qse_dn7 = assign34520_e49617_d_n7;
        locals.var_qse_dn10 = assign34520_e49617_d_n10;
        locals.var_qse_dn11 = assign34520_e49617_d_n11;
        locals.var_qse_dn12 = assign34520_e49617_d_n12;
        locals.var_qse_dn13 = assign34520_e49617_d_n13;
        locals.var_qse_dn15 = assign34520_e49617_d_n15;
        locals.var_qse_dn16 = assign34520_e49617_d_n16;
        locals.var_qse_dn17 = assign34520_e49617_d_n17;
        locals.var_qse_dn18 = assign34520_e49617_d_n18;
        locals.var_qse_rv = 0.0;

        let (assign34530_e49638, assign34530_e49638_d_n0, assign34530_e49638_d_n2, assign34530_e49638_d_n6, assign34530_e49638_d_n7, assign34530_e49638_d_n10, assign34530_e49638_d_n11, assign34530_e49638_d_n12, assign34530_e49638_d_n13, assign34530_e49638_d_n15, assign34530_e49638_d_n16, assign34530_e49638_d_n17, assign34530_e49638_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34530_e49626: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34530_e49628: f64 = (assign34530_e49626 + locals.var_qgob);
        let assign34530_e49630: f64 = (assign34530_e49628 - locals.var_qy);
        let assign34530_e49632: f64 = (assign34530_e49630 - locals.var_qovs);
        let assign34530_e49634: f64 = (assign34530_e49632 - locals.var_qovd);
        let assign34530_e49635: f64 = (locals.var_mfactor * assign34530_e49634);
        let assign34530_e49636: f64 = (locals.var_qge + assign34530_e49635);
        (assign34530_e49636, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34530_e49638;
        locals.var_qge_dn0 = assign34530_e49638_d_n0;
        locals.var_qge_dn2 = assign34530_e49638_d_n2;
        locals.var_qge_dn6 = assign34530_e49638_d_n6;
        locals.var_qge_dn7 = assign34530_e49638_d_n7;
        locals.var_qge_dn10 = assign34530_e49638_d_n10;
        locals.var_qge_dn11 = assign34530_e49638_d_n11;
        locals.var_qge_dn12 = assign34530_e49638_d_n12;
        locals.var_qge_dn13 = assign34530_e49638_d_n13;
        locals.var_qge_dn15 = assign34530_e49638_d_n15;
        locals.var_qge_dn16 = assign34530_e49638_d_n16;
        locals.var_qge_dn17 = assign34530_e49638_d_n17;
        locals.var_qge_dn18 = assign34530_e49638_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34540_e49654, assign34540_e49654_d_n0, assign34540_e49654_d_n2, assign34540_e49654_d_n6, assign34540_e49654_d_n7, assign34540_e49654_d_n10, assign34540_e49654_d_n11, assign34540_e49654_d_n12, assign34540_e49654_d_n13, assign34540_e49654_d_n15, assign34540_e49654_d_n16, assign34540_e49654_d_n17, assign34540_e49654_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34540_e49646: f64 = (-locals.var_qgod);
        let assign34540_e49648: f64 = (assign34540_e49646 + locals.var_qy);
        let assign34540_e49650: f64 = (assign34540_e49648 + locals.var_qbdld);
        let assign34540_e49651: f64 = (locals.var_mfactor * assign34540_e49650);
        let assign34540_e49652: f64 = (locals.var_qde + assign34540_e49651);
        (assign34540_e49652, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34540_e49654;
        locals.var_qde_dn0 = assign34540_e49654_d_n0;
        locals.var_qde_dn2 = assign34540_e49654_d_n2;
        locals.var_qde_dn6 = assign34540_e49654_d_n6;
        locals.var_qde_dn7 = assign34540_e49654_d_n7;
        locals.var_qde_dn10 = assign34540_e49654_d_n10;
        locals.var_qde_dn11 = assign34540_e49654_d_n11;
        locals.var_qde_dn12 = assign34540_e49654_d_n12;
        locals.var_qde_dn13 = assign34540_e49654_d_n13;
        locals.var_qde_dn15 = assign34540_e49654_d_n15;
        locals.var_qde_dn16 = assign34540_e49654_d_n16;
        locals.var_qde_dn17 = assign34540_e49654_d_n17;
        locals.var_qde_dn18 = assign34540_e49654_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34550_e49668, assign34550_e49668_d_n0, assign34550_e49668_d_n2, assign34550_e49668_d_n6, assign34550_e49668_d_n7, assign34550_e49668_d_n10, assign34550_e49668_d_n11, assign34550_e49668_d_n12, assign34550_e49668_d_n13, assign34550_e49668_d_n15, assign34550_e49668_d_n16, assign34550_e49668_d_n17, assign34550_e49668_d_n18,) = {
    if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
        let assign34550_e49662: f64 = (-locals.var_qgos);
        let assign34550_e49664: f64 = (assign34550_e49662 + locals.var_qbsld);
        let assign34550_e49665: f64 = (locals.var_mfactor * assign34550_e49664);
        let assign34550_e49666: f64 = (locals.var_qse + assign34550_e49665);
        (assign34550_e49666, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34550_e49668;
        locals.var_qse_dn0 = assign34550_e49668_d_n0;
        locals.var_qse_dn2 = assign34550_e49668_d_n2;
        locals.var_qse_dn6 = assign34550_e49668_d_n6;
        locals.var_qse_dn7 = assign34550_e49668_d_n7;
        locals.var_qse_dn10 = assign34550_e49668_d_n10;
        locals.var_qse_dn11 = assign34550_e49668_d_n11;
        locals.var_qse_dn12 = assign34550_e49668_d_n12;
        locals.var_qse_dn13 = assign34550_e49668_d_n13;
        locals.var_qse_dn15 = assign34550_e49668_d_n15;
        locals.var_qse_dn16 = assign34550_e49668_d_n16;
        locals.var_qse_dn17 = assign34550_e49668_d_n17;
        locals.var_qse_dn18 = assign34550_e49668_d_n18;
        locals.var_qse_rv = 0.0;

        let assign34580_e49673: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34580_e49673;
        locals.var_guard1137_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34590_e49679, assign34590_e49679_d_n0, assign34590_e49679_d_n2, assign34590_e49679_d_n6, assign34590_e49679_d_n7, assign34590_e49679_d_n10, assign34590_e49679_d_n11, assign34590_e49679_d_n12, assign34590_e49679_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34590_e49677: f64 = (locals.var_mfactor * locals.var_ibs);
        (assign34590_e49677, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34590_e49679;
        locals.var_ibsb_dn0 = assign34590_e49679_d_n0;
        locals.var_ibsb_dn2 = assign34590_e49679_d_n2;
        locals.var_ibsb_dn6 = assign34590_e49679_d_n6;
        locals.var_ibsb_dn7 = assign34590_e49679_d_n7;
        locals.var_ibsb_dn10 = assign34590_e49679_d_n10;
        locals.var_ibsb_dn11 = assign34590_e49679_d_n11;
        locals.var_ibsb_dn12 = assign34590_e49679_d_n12;
        locals.var_ibsb_dn17 = assign34590_e49679_d_n17;
        locals.var_ibsb_rv = 0.0;

        let (assign34600_e49685, assign34600_e49685_d_n0, assign34600_e49685_d_n2, assign34600_e49685_d_n6, assign34600_e49685_d_n7, assign34600_e49685_d_n10, assign34600_e49685_d_n11, assign34600_e49685_d_n12, assign34600_e49685_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34600_e49683: f64 = (locals.var_mfactor * locals.var_ibd);
        (assign34600_e49683, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34600_e49685;
        locals.var_ibdb_dn0 = assign34600_e49685_d_n0;
        locals.var_ibdb_dn2 = assign34600_e49685_d_n2;
        locals.var_ibdb_dn6 = assign34600_e49685_d_n6;
        locals.var_ibdb_dn7 = assign34600_e49685_d_n7;
        locals.var_ibdb_dn10 = assign34600_e49685_d_n10;
        locals.var_ibdb_dn11 = assign34600_e49685_d_n11;
        locals.var_ibdb_dn12 = assign34600_e49685_d_n12;
        locals.var_ibdb_dn17 = assign34600_e49685_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign34610_e49691, assign34610_e49691_d_n0, assign34610_e49691_d_n2, assign34610_e49691_d_n6, assign34610_e49691_d_n7, assign34610_e49691_d_n10, assign34610_e49691_d_n11, assign34610_e49691_d_n12, assign34610_e49691_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34610_e49689: f64 = (locals.var_mfactor * locals.var_qbd);
        (assign34610_e49689, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17),)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34610_e49691;
        locals.var_qbd_s0_dn0 = assign34610_e49691_d_n0;
        locals.var_qbd_s0_dn2 = assign34610_e49691_d_n2;
        locals.var_qbd_s0_dn6 = assign34610_e49691_d_n6;
        locals.var_qbd_s0_dn7 = assign34610_e49691_d_n7;
        locals.var_qbd_s0_dn10 = assign34610_e49691_d_n10;
        locals.var_qbd_s0_dn11 = assign34610_e49691_d_n11;
        locals.var_qbd_s0_dn12 = assign34610_e49691_d_n12;
        locals.var_qbd_s0_dn17 = assign34610_e49691_d_n17;
        locals.var_qbd_s0_rv = 0.0;

        let (assign34620_e49697, assign34620_e49697_d_n0, assign34620_e49697_d_n2, assign34620_e49697_d_n6, assign34620_e49697_d_n7, assign34620_e49697_d_n10, assign34620_e49697_d_n11, assign34620_e49697_d_n12, assign34620_e49697_d_n17,) = {
    if (locals.var_guard1137 != 0.0) {
        let assign34620_e49695: f64 = (locals.var_mfactor * locals.var_qbs);
        (assign34620_e49695, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17),)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34620_e49697;
        locals.var_qbs_s0_dn0 = assign34620_e49697_d_n0;
        locals.var_qbs_s0_dn2 = assign34620_e49697_d_n2;
        locals.var_qbs_s0_dn6 = assign34620_e49697_d_n6;
        locals.var_qbs_s0_dn7 = assign34620_e49697_d_n7;
        locals.var_qbs_s0_dn10 = assign34620_e49697_d_n10;
        locals.var_qbs_s0_dn11 = assign34620_e49697_d_n11;
        locals.var_qbs_s0_dn12 = assign34620_e49697_d_n12;
        locals.var_qbs_s0_dn17 = assign34620_e49697_d_n17;
        locals.var_qbs_s0_rv = 0.0;

        let (assign34630_e49702, assign34630_e49702_d_n0, assign34630_e49702_d_n2, assign34630_e49702_d_n6, assign34630_e49702_d_n7, assign34630_e49702_d_n10, assign34630_e49702_d_n11, assign34630_e49702_d_n12, assign34630_e49702_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34630_e49702;
        locals.var_ibsb_dn0 = assign34630_e49702_d_n0;
        locals.var_ibsb_dn2 = assign34630_e49702_d_n2;
        locals.var_ibsb_dn6 = assign34630_e49702_d_n6;
        locals.var_ibsb_dn7 = assign34630_e49702_d_n7;
        locals.var_ibsb_dn10 = assign34630_e49702_d_n10;
        locals.var_ibsb_dn11 = assign34630_e49702_d_n11;
        locals.var_ibsb_dn12 = assign34630_e49702_d_n12;
        locals.var_ibsb_dn17 = assign34630_e49702_d_n17;
        locals.var_ibsb_rv = 0.0;

        let (assign34640_e49707, assign34640_e49707_d_n0, assign34640_e49707_d_n2, assign34640_e49707_d_n6, assign34640_e49707_d_n7, assign34640_e49707_d_n10, assign34640_e49707_d_n11, assign34640_e49707_d_n12, assign34640_e49707_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34640_e49707;
        locals.var_ibdb_dn0 = assign34640_e49707_d_n0;
        locals.var_ibdb_dn2 = assign34640_e49707_d_n2;
        locals.var_ibdb_dn6 = assign34640_e49707_d_n6;
        locals.var_ibdb_dn7 = assign34640_e49707_d_n7;
        locals.var_ibdb_dn10 = assign34640_e49707_d_n10;
        locals.var_ibdb_dn11 = assign34640_e49707_d_n11;
        locals.var_ibdb_dn12 = assign34640_e49707_d_n12;
        locals.var_ibdb_dn17 = assign34640_e49707_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign34650_e49712, assign34650_e49712_d_n0, assign34650_e49712_d_n2, assign34650_e49712_d_n6, assign34650_e49712_d_n7, assign34650_e49712_d_n10, assign34650_e49712_d_n11, assign34650_e49712_d_n12, assign34650_e49712_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34650_e49712;
        locals.var_qbd_s0_dn0 = assign34650_e49712_d_n0;
        locals.var_qbd_s0_dn2 = assign34650_e49712_d_n2;
        locals.var_qbd_s0_dn6 = assign34650_e49712_d_n6;
        locals.var_qbd_s0_dn7 = assign34650_e49712_d_n7;
        locals.var_qbd_s0_dn10 = assign34650_e49712_d_n10;
        locals.var_qbd_s0_dn11 = assign34650_e49712_d_n11;
        locals.var_qbd_s0_dn12 = assign34650_e49712_d_n12;
        locals.var_qbd_s0_dn17 = assign34650_e49712_d_n17;
        locals.var_qbd_s0_rv = 0.0;

        let (assign34660_e49717, assign34660_e49717_d_n0, assign34660_e49717_d_n2, assign34660_e49717_d_n6, assign34660_e49717_d_n7, assign34660_e49717_d_n10, assign34660_e49717_d_n11, assign34660_e49717_d_n12, assign34660_e49717_d_n17,) = {
    if (locals.var_guard1137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34660_e49717;
        locals.var_qbs_s0_dn0 = assign34660_e49717_d_n0;
        locals.var_qbs_s0_dn2 = assign34660_e49717_d_n2;
        locals.var_qbs_s0_dn6 = assign34660_e49717_d_n6;
        locals.var_qbs_s0_dn7 = assign34660_e49717_d_n7;
        locals.var_qbs_s0_dn10 = assign34660_e49717_d_n10;
        locals.var_qbs_s0_dn11 = assign34660_e49717_d_n11;
        locals.var_qbs_s0_dn12 = assign34660_e49717_d_n12;
        locals.var_qbs_s0_dn17 = assign34660_e49717_d_n17;
        locals.var_qbs_s0_rv = 0.0;

        let assign34670_e49720: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34670_e49720;
        locals.var_guard1138_rv = 0.0;

        let (assign34680_e49724, assign34680_e49724_d_n0, assign34680_e49724_d_n2, assign34680_e49724_d_n6, assign34680_e49724_d_n7, assign34680_e49724_d_n10, assign34680_e49724_d_n11, assign34680_e49724_d_n12, assign34680_e49724_d_n17,) = {
    if (locals.var_guard1138 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34680_e49724;
        locals.var_isube_dn0 = assign34680_e49724_d_n0;
        locals.var_isube_dn2 = assign34680_e49724_d_n2;
        locals.var_isube_dn6 = assign34680_e49724_d_n6;
        locals.var_isube_dn7 = assign34680_e49724_d_n7;
        locals.var_isube_dn10 = assign34680_e49724_d_n10;
        locals.var_isube_dn11 = assign34680_e49724_d_n11;
        locals.var_isube_dn12 = assign34680_e49724_d_n12;
        locals.var_isube_dn17 = assign34680_e49724_d_n17;
        locals.var_isube_rv = 0.0;

        let (assign34690_e49731, assign34690_e49731_d_n0, assign34690_e49731_d_n2, assign34690_e49731_d_n6, assign34690_e49731_d_n7, assign34690_e49731_d_n10, assign34690_e49731_d_n11, assign34690_e49731_d_n12, assign34690_e49731_d_n17,) = {
    if (locals.var_guard1138 == 0.0) {
        let assign34690_e49729: f64 = (locals.var_mfactor * locals.var_isub);
        (assign34690_e49729, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17),)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34690_e49731;
        locals.var_isube_dn0 = assign34690_e49731_d_n0;
        locals.var_isube_dn2 = assign34690_e49731_d_n2;
        locals.var_isube_dn6 = assign34690_e49731_d_n6;
        locals.var_isube_dn7 = assign34690_e49731_d_n7;
        locals.var_isube_dn10 = assign34690_e49731_d_n10;
        locals.var_isube_dn11 = assign34690_e49731_d_n11;
        locals.var_isube_dn12 = assign34690_e49731_d_n12;
        locals.var_isube_dn17 = assign34690_e49731_d_n17;
        locals.var_isube_rv = 0.0;

        let assign34800_e49813: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign34800_e49813;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn12 = (locals.var_mfactor * locals.var_nthrml_dn12);
        locals.var_noithrml_dn17 = (locals.var_mfactor * locals.var_nthrml_dn17);
        locals.var_noithrml_rv = 0.0;

        let assign34810_e49816: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign34810_e49816;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign34820_e49819: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign34820_e49819;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);
        locals.var_cgdbd_rv = 0.0;

        let assign34830_e49822: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign34830_e49822;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign34840_e49825: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign34840_e49825;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);
        locals.var_cgsbd_rv = 0.0;

        let (assign34850_e49831, assign34850_e49831_d_n0, assign34850_e49831_d_n2, assign34850_e49831_d_n6, assign34850_e49831_d_n7, assign34850_e49831_d_n10, assign34850_e49831_d_n11, assign34850_e49831_d_n12, assign34850_e49831_d_n13, assign34850_e49831_d_n15, assign34850_e49831_d_n16, assign34850_e49831_d_n17, assign34850_e49831_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        locals.var_cgsb = assign34850_e49831;
        locals.var_cgsb_dn0 = assign34850_e49831_d_n0;
        locals.var_cgsb_dn2 = assign34850_e49831_d_n2;
        locals.var_cgsb_dn6 = assign34850_e49831_d_n6;
        locals.var_cgsb_dn7 = assign34850_e49831_d_n7;
        locals.var_cgsb_dn10 = assign34850_e49831_d_n10;
        locals.var_cgsb_dn11 = assign34850_e49831_d_n11;
        locals.var_cgsb_dn12 = assign34850_e49831_d_n12;
        locals.var_cgsb_dn13 = assign34850_e49831_d_n13;
        locals.var_cgsb_dn15 = assign34850_e49831_d_n15;
        locals.var_cgsb_dn16 = assign34850_e49831_d_n16;
        locals.var_cgsb_dn17 = assign34850_e49831_d_n17;
        locals.var_cgsb_dn18 = assign34850_e49831_d_n18;
        locals.var_cgsb_rv = 0.0;

        let assign34860_e49845: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1147 = assign34860_e49845;
        locals.var_guard1147_rv = 0.0;

        let (assign34870_e49855, assign34870_e49855_d_n0, assign34870_e49855_d_n2, assign34870_e49855_d_n6, assign34870_e49855_d_n7, assign34870_e49855_d_n10, assign34870_e49855_d_n11, assign34870_e49855_d_n12, assign34870_e49855_d_n17,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34870_e49849: f64 = (1e-6 * locals.var_c_fox);
        let assign34870_e49851: f64 = (assign34870_e49849 * locals.var_weffcv_nf);
        let assign34870_e49853: f64 = (assign34870_e49851 * locals.var_leff_cv);
        (assign34870_e49853, (((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn7) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn17) * locals.var_weffcv_nf) * locals.var_leff_cv),)
    } else {
        (locals.var_t0__blk1141, locals.var_t0__blk1141_dn0, locals.var_t0__blk1141_dn2, locals.var_t0__blk1141_dn6, locals.var_t0__blk1141_dn7, locals.var_t0__blk1141_dn10, locals.var_t0__blk1141_dn11, locals.var_t0__blk1141_dn12, locals.var_t0__blk1141_dn17,)
    }
};
        locals.var_t0__blk1141 = assign34870_e49855;
        locals.var_t0__blk1141_dn0 = assign34870_e49855_d_n0;
        locals.var_t0__blk1141_dn2 = assign34870_e49855_d_n2;
        locals.var_t0__blk1141_dn6 = assign34870_e49855_d_n6;
        locals.var_t0__blk1141_dn7 = assign34870_e49855_d_n7;
        locals.var_t0__blk1141_dn10 = assign34870_e49855_d_n10;
        locals.var_t0__blk1141_dn11 = assign34870_e49855_d_n11;
        locals.var_t0__blk1141_dn12 = assign34870_e49855_d_n12;
        locals.var_t0__blk1141_dn17 = assign34870_e49855_d_n17;
        locals.var_t0__blk1141_rv = 0.0;

        let (assign34880_e49861, assign34880_e49861_d_n0, assign34880_e49861_d_n2, assign34880_e49861_d_n6, assign34880_e49861_d_n7, assign34880_e49861_d_n10, assign34880_e49861_d_n11, assign34880_e49861_d_n12, assign34880_e49861_d_n13, assign34880_e49861_d_n15, assign34880_e49861_d_n16, assign34880_e49861_d_n17, assign34880_e49861_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34880_e49859: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign34880_e49859, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor),)
    } else {
        (locals.var_t1__blk1142, locals.var_t1__blk1142_dn0, locals.var_t1__blk1142_dn2, locals.var_t1__blk1142_dn6, locals.var_t1__blk1142_dn7, locals.var_t1__blk1142_dn10, locals.var_t1__blk1142_dn11, locals.var_t1__blk1142_dn12, locals.var_t1__blk1142_dn13, locals.var_t1__blk1142_dn15, locals.var_t1__blk1142_dn16, locals.var_t1__blk1142_dn17, locals.var_t1__blk1142_dn18,)
    }
};
        locals.var_t1__blk1142 = assign34880_e49861;
        locals.var_t1__blk1142_dn0 = assign34880_e49861_d_n0;
        locals.var_t1__blk1142_dn2 = assign34880_e49861_d_n2;
        locals.var_t1__blk1142_dn6 = assign34880_e49861_d_n6;
        locals.var_t1__blk1142_dn7 = assign34880_e49861_d_n7;
        locals.var_t1__blk1142_dn10 = assign34880_e49861_d_n10;
        locals.var_t1__blk1142_dn11 = assign34880_e49861_d_n11;
        locals.var_t1__blk1142_dn12 = assign34880_e49861_d_n12;
        locals.var_t1__blk1142_dn13 = assign34880_e49861_d_n13;
        locals.var_t1__blk1142_dn15 = assign34880_e49861_d_n15;
        locals.var_t1__blk1142_dn16 = assign34880_e49861_d_n16;
        locals.var_t1__blk1142_dn17 = assign34880_e49861_d_n17;
        locals.var_t1__blk1142_dn18 = assign34880_e49861_d_n18;
        locals.var_t1__blk1142_rv = 0.0;

        let (assign34890_e49875, assign34890_e49875_d_n0, assign34890_e49875_d_n2, assign34890_e49875_d_n6, assign34890_e49875_d_n7, assign34890_e49875_d_n10, assign34890_e49875_d_n11, assign34890_e49875_d_n12, assign34890_e49875_d_n13, assign34890_e49875_d_n15, assign34890_e49875_d_n16, assign34890_e49875_d_n17, assign34890_e49875_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34890_e49865: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34890_e49867: f64 = (assign34890_e49865 * locals.var_beta_inv);
        let assign34890_e49869: f64 = (assign34890_e49867 * locals.var_t1__blk1142);
        let assign34890_e49871: f64 = (assign34890_e49869 * locals.var_t1__blk1142);
        let assign34890_e49873: f64 = (assign34890_e49871 / locals.var_gds0_ign);
        (assign34890_e49873, ((((((assign34890_e49867 * locals.var_t1__blk1142_dn0) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn0)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn2) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn2)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn6) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn6)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn7) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn7)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34890_e49865 * locals.var_beta_inv_dn10) * locals.var_t1__blk1142) + (assign34890_e49867 * locals.var_t1__blk1142_dn10)) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn10)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn11) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn11)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn12) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn12)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn13) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn13)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn15) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn15)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn16) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn16)) / locals.var_gds0_ign), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn17) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn17)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn18) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn18)) / locals.var_gds0_ign),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18,)
    }
};
        locals.var_nign0 = assign34890_e49875;
        locals.var_nign0_dn0 = assign34890_e49875_d_n0;
        locals.var_nign0_dn2 = assign34890_e49875_d_n2;
        locals.var_nign0_dn6 = assign34890_e49875_d_n6;
        locals.var_nign0_dn7 = assign34890_e49875_d_n7;
        locals.var_nign0_dn10 = assign34890_e49875_d_n10;
        locals.var_nign0_dn11 = assign34890_e49875_d_n11;
        locals.var_nign0_dn12 = assign34890_e49875_d_n12;
        locals.var_nign0_dn13 = assign34890_e49875_d_n13;
        locals.var_nign0_dn15 = assign34890_e49875_d_n15;
        locals.var_nign0_dn16 = assign34890_e49875_d_n16;
        locals.var_nign0_dn17 = assign34890_e49875_d_n17;
        locals.var_nign0_dn18 = assign34890_e49875_d_n18;
        locals.var_nign0_rv = 0.0;

        let assign34900_e49879: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49884: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49886: f64 = if ((locals.var_kusai00l > assign34900_e49879) && (locals.var_vds > assign34900_e49884)) { 1.0 } else { 0.0 };
        locals.var_guard1148 = assign34900_e49886;
        locals.var_guard1148_rv = 0.0;

        let (assign34910_e49894, assign34910_e49894_d_n0, assign34910_e49894_d_n2, assign34910_e49894_d_n6, assign34910_e49894_d_n7, assign34910_e49894_d_n10, assign34910_e49894_d_n11, assign34910_e49894_d_n12, assign34910_e49894_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34910_e49892: f64 = (locals.var_muun / locals.var_mu);
        (assign34910_e49892, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17,)
    }
};
        locals.var_mumoda = assign34910_e49894;
        locals.var_mumoda_dn0 = assign34910_e49894_d_n0;
        locals.var_mumoda_dn2 = assign34910_e49894_d_n2;
        locals.var_mumoda_dn6 = assign34910_e49894_d_n6;
        locals.var_mumoda_dn7 = assign34910_e49894_d_n7;
        locals.var_mumoda_dn10 = assign34910_e49894_d_n10;
        locals.var_mumoda_dn11 = assign34910_e49894_d_n11;
        locals.var_mumoda_dn12 = assign34910_e49894_d_n12;
        locals.var_mumoda_dn17 = assign34910_e49894_d_n17;
        locals.var_mumoda_rv = 0.0;

        let (assign34920_e49906, assign34920_e49906_d_n0, assign34920_e49906_d_n2, assign34920_e49906_d_n6, assign34920_e49906_d_n7, assign34920_e49906_d_n10, assign34920_e49906_d_n11, assign34920_e49906_d_n12, assign34920_e49906_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34920_e49900: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign34920_e49902: f64 = (assign34920_e49900 - locals.var_mumoda);
        let assign34920_e49904: f64 = (assign34920_e49902 / locals.var_vds);
        (assign34920_e49904, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17,)
    }
};
        locals.var_mumodb = assign34920_e49906;
        locals.var_mumodb_dn0 = assign34920_e49906_d_n0;
        locals.var_mumodb_dn2 = assign34920_e49906_d_n2;
        locals.var_mumodb_dn6 = assign34920_e49906_d_n6;
        locals.var_mumodb_dn7 = assign34920_e49906_d_n7;
        locals.var_mumodb_dn10 = assign34920_e49906_d_n10;
        locals.var_mumodb_dn11 = assign34920_e49906_d_n11;
        locals.var_mumodb_dn12 = assign34920_e49906_d_n12;
        locals.var_mumodb_dn17 = assign34920_e49906_d_n17;
        locals.var_mumodb_rv = 0.0;

        let (assign34930_e49928, assign34930_e49928_d_n0, assign34930_e49928_d_n2, assign34930_e49928_d_n6, assign34930_e49928_d_n7, assign34930_e49928_d_n10, assign34930_e49928_d_n11, assign34930_e49928_d_n12, assign34930_e49928_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
        let assign34930_e49913: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign34930_e49917: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign34930_e49918: f64 = (locals.var_kusai00 + assign34930_e49917);
        let assign34930_e49920: f64 = (assign34930_e49918 + locals.var_kusail);
        let assign34930_e49921: f64 = (assign34930_e49913 * assign34930_e49920);
        let assign34930_e49924: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign34930_e49925: f64 = (assign34930_e49921 / assign34930_e49924);
        let assign34930_e49926: f64 = (locals.var_mumoda + assign34930_e49925);
        (assign34930_e49926, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34930_e49924 * assign34930_e49924))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34930_e49928;
        locals.var_correct_w1_dn0 = assign34930_e49928_d_n0;
        locals.var_correct_w1_dn2 = assign34930_e49928_d_n2;
        locals.var_correct_w1_dn6 = assign34930_e49928_d_n6;
        locals.var_correct_w1_dn7 = assign34930_e49928_d_n7;
        locals.var_correct_w1_dn10 = assign34930_e49928_d_n10;
        locals.var_correct_w1_dn11 = assign34930_e49928_d_n11;
        locals.var_correct_w1_dn12 = assign34930_e49928_d_n12;
        locals.var_correct_w1_dn17 = assign34930_e49928_d_n17;
        locals.var_correct_w1_rv = 0.0;

        let (assign34940_e49937, assign34940_e49937_d_n0, assign34940_e49937_d_n2, assign34940_e49937_d_n6, assign34940_e49937_d_n7, assign34940_e49937_d_n10, assign34940_e49937_d_n11, assign34940_e49937_d_n12, assign34940_e49937_d_n17,) = {
    if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 == 0.0)) {
        let assign34940_e49935: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign34940_e49935, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34940_e49937;
        locals.var_correct_w1_dn0 = assign34940_e49937_d_n0;
        locals.var_correct_w1_dn2 = assign34940_e49937_d_n2;
        locals.var_correct_w1_dn6 = assign34940_e49937_d_n6;
        locals.var_correct_w1_dn7 = assign34940_e49937_d_n7;
        locals.var_correct_w1_dn10 = assign34940_e49937_d_n10;
        locals.var_correct_w1_dn11 = assign34940_e49937_d_n11;
        locals.var_correct_w1_dn12 = assign34940_e49937_d_n12;
        locals.var_correct_w1_dn17 = assign34940_e49937_d_n17;
        locals.var_correct_w1_rv = 0.0;

        let (assign34950_e49947, assign34950_e49947_d_n0, assign34950_e49947_d_n2, assign34950_e49947_d_n6, assign34950_e49947_d_n7, assign34950_e49947_d_n10, assign34950_e49947_d_n11, assign34950_e49947_d_n12, assign34950_e49947_d_n13, assign34950_e49947_d_n15, assign34950_e49947_d_n16, assign34950_e49947_d_n17, assign34950_e49947_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34950_e49941: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign34950_e49943: f64 = (assign34950_e49941 * locals.var_kusai_ig);
        let assign34950_e49945: f64 = (assign34950_e49943 * locals.var_correct_w1);
        (assign34950_e49945, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34950_e49947;
        locals.var_noiigate_dn0 = assign34950_e49947_d_n0;
        locals.var_noiigate_dn2 = assign34950_e49947_d_n2;
        locals.var_noiigate_dn6 = assign34950_e49947_d_n6;
        locals.var_noiigate_dn7 = assign34950_e49947_d_n7;
        locals.var_noiigate_dn10 = assign34950_e49947_d_n10;
        locals.var_noiigate_dn11 = assign34950_e49947_d_n11;
        locals.var_noiigate_dn12 = assign34950_e49947_d_n12;
        locals.var_noiigate_dn13 = assign34950_e49947_d_n13;
        locals.var_noiigate_dn15 = assign34950_e49947_d_n15;
        locals.var_noiigate_dn16 = assign34950_e49947_d_n16;
        locals.var_noiigate_dn17 = assign34950_e49947_d_n17;
        locals.var_noiigate_dn18 = assign34950_e49947_d_n18;
        locals.var_noiigate_rv = 0.0;

        let (assign34970_e49965, assign34970_e49965_d_n0, assign34970_e49965_d_n2, assign34970_e49965_d_n6, assign34970_e49965_d_n7, assign34970_e49965_d_n10, assign34970_e49965_d_n11, assign34970_e49965_d_n12, assign34970_e49965_d_n13, assign34970_e49965_d_n15, assign34970_e49965_d_n16, assign34970_e49965_d_n17, assign34970_e49965_d_n18,) = {
    if (locals.var_guard1147 != 0.0) {
        let assign34970_e49954: f64 = (-locals.var_t1__blk1142);
        let (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18,) = {
            if ((assign34970_e49954 > locals.var_t0__blk1141) && (locals.var_noiigate > 0.0)) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34970_e49965;
        locals.var_noiigate_dn0 = assign34970_e49965_d_n0;
        locals.var_noiigate_dn2 = assign34970_e49965_d_n2;
        locals.var_noiigate_dn6 = assign34970_e49965_d_n6;
        locals.var_noiigate_dn7 = assign34970_e49965_d_n7;
        locals.var_noiigate_dn10 = assign34970_e49965_d_n10;
        locals.var_noiigate_dn11 = assign34970_e49965_d_n11;
        locals.var_noiigate_dn12 = assign34970_e49965_d_n12;
        locals.var_noiigate_dn13 = assign34970_e49965_d_n13;
        locals.var_noiigate_dn15 = assign34970_e49965_d_n15;
        locals.var_noiigate_dn16 = assign34970_e49965_d_n16;
        locals.var_noiigate_dn17 = assign34970_e49965_d_n17;
        locals.var_noiigate_dn18 = assign34970_e49965_d_n18;
        locals.var_noiigate_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_126(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign34990_e49980, assign34990_e49980_d_n0, assign34990_e49980_d_n2, assign34990_e49980_d_n6, assign34990_e49980_d_n7, assign34990_e49980_d_n10, assign34990_e49980_d_n11, assign34990_e49980_d_n12, assign34990_e49980_d_n13, assign34990_e49980_d_n15, assign34990_e49980_d_n16, assign34990_e49980_d_n17, assign34990_e49980_d_n18,) = {
    if (locals.var_guard1147 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34990_e49980;
        locals.var_noiigate_dn0 = assign34990_e49980_d_n0;
        locals.var_noiigate_dn2 = assign34990_e49980_d_n2;
        locals.var_noiigate_dn6 = assign34990_e49980_d_n6;
        locals.var_noiigate_dn7 = assign34990_e49980_d_n7;
        locals.var_noiigate_dn10 = assign34990_e49980_d_n10;
        locals.var_noiigate_dn11 = assign34990_e49980_d_n11;
        locals.var_noiigate_dn12 = assign34990_e49980_d_n12;
        locals.var_noiigate_dn13 = assign34990_e49980_d_n13;
        locals.var_noiigate_dn15 = assign34990_e49980_d_n15;
        locals.var_noiigate_dn16 = assign34990_e49980_d_n16;
        locals.var_noiigate_dn17 = assign34990_e49980_d_n17;
        locals.var_noiigate_dn18 = assign34990_e49980_d_n18;
        locals.var_noiigate_rv = 0.0;

        let assign35050_e49992: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign35050_e49992;
        locals.var_guard1149_rv = 0.0;

        let (assign35060_e49996,) = {
    if (locals.var_guard1149 != 0.0) {
        (1.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35060_e49996;
        locals.var_rdmod_rv = 0.0;

        let assign35070_e49999: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign35070_e49999;
        locals.var_guard1169_rv = 0.0;

        let (assign35090_e50013,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35090_e50013;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35100_e50019,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35100_e50019;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35110_e50025, assign35110_e50025_d_n10,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35110_e50025;
        locals.var_rrdrbb_dn10 = assign35110_e50025_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35130_e50044,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35130_e50044;
        locals.var_ldrifte_rv = 0.0;

        let (assign35140_e50052, assign35140_e50052_d_n0, assign35140_e50052_d_n2, assign35140_e50052_d_n6, assign35140_e50052_d_n7,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
        let assign35140_e50050: f64 = (p.p50 * (nv7 - nv2));
        (assign35140_e50050, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35140_e50052;
        locals.var_vrdr_dn0 = assign35140_e50052_d_n0;
        locals.var_vrdr_dn2 = assign35140_e50052_d_n2;
        locals.var_vrdr_dn6 = assign35140_e50052_d_n6;
        locals.var_vrdr_dn7 = assign35140_e50052_d_n7;
        locals.var_vrdr_rv = 0.0;

        let (assign35160_e50068,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35160_e50068;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35170_e50075,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35170_e50075;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35180_e50082, assign35180_e50082_d_n10,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35180_e50082;
        locals.var_rrdrbb_dn10 = assign35180_e50082_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35200_e50103,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35200_e50103;
        locals.var_ldrifte_rv = 0.0;

        let (assign35210_e50112, assign35210_e50112_d_n0, assign35210_e50112_d_n2, assign35210_e50112_d_n6, assign35210_e50112_d_n7,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
        let assign35210_e50110: f64 = (p.p50 * (nv0 - nv6));
        (assign35210_e50110, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35210_e50112;
        locals.var_vrdr_dn0 = assign35210_e50112_d_n0;
        locals.var_vrdr_dn2 = assign35210_e50112_d_n2;
        locals.var_vrdr_dn6 = assign35210_e50112_d_n6;
        locals.var_vrdr_dn7 = assign35210_e50112_d_n7;
        locals.var_vrdr_rv = 0.0;

        let (assign35240_e50135,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35240_e50133: f64 = (locals.var_mks_rdrmue / 10000.0);
        (assign35240_e50133,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35240_e50135;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35250_e50141,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35250_e50139: f64 = (locals.var_mks_rdrvmax / 100.0);
        (assign35250_e50139,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35250_e50141;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35260_e50147, assign35260_e50147_d_n10,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35260_e50145: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35260_e50145, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn10,)
    }
};
        locals.var_tratio = assign35260_e50147;
        locals.var_tratio_dn10 = assign35260_e50147_d_n10;
        locals.var_tratio_rv = 0.0;

        let (assign35270_e50153, assign35270_e50153_d_n0, assign35270_e50153_d_n2, assign35270_e50153_d_n6, assign35270_e50153_d_n7, assign35270_e50153_d_n10, assign35270_e50153_d_n11, assign35270_e50153_d_n12, assign35270_e50153_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35270_e50151: f64 = (locals.var_tratio).powf(p.p269);
        (assign35270_e50151, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35270_e50151 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35270_e50153;
        locals.var_t1_dn0 = assign35270_e50153_d_n0;
        locals.var_t1_dn2 = assign35270_e50153_d_n2;
        locals.var_t1_dn6 = assign35270_e50153_d_n6;
        locals.var_t1_dn7 = assign35270_e50153_d_n7;
        locals.var_t1_dn10 = assign35270_e50153_d_n10;
        locals.var_t1_dn11 = assign35270_e50153_d_n11;
        locals.var_t1_dn12 = assign35270_e50153_d_n12;
        locals.var_t1_dn17 = assign35270_e50153_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign35280_e50159, assign35280_e50159_d_n0, assign35280_e50159_d_n2, assign35280_e50159_d_n6, assign35280_e50159_d_n7, assign35280_e50159_d_n10, assign35280_e50159_d_n11, assign35280_e50159_d_n12, assign35280_e50159_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35280_e50157: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign35280_e50157, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35280_e50159;
        locals.var_mu0_dn0 = assign35280_e50159_d_n0;
        locals.var_mu0_dn2 = assign35280_e50159_d_n2;
        locals.var_mu0_dn6 = assign35280_e50159_d_n6;
        locals.var_mu0_dn7 = assign35280_e50159_d_n7;
        locals.var_mu0_dn10 = assign35280_e50159_d_n10;
        locals.var_mu0_dn11 = assign35280_e50159_d_n11;
        locals.var_mu0_dn12 = assign35280_e50159_d_n12;
        locals.var_mu0_dn17 = assign35280_e50159_d_n17;
        locals.var_mu0_rv = 0.0;

        let (assign35290_e50179, assign35290_e50179_d_n0, assign35290_e50179_d_n2, assign35290_e50179_d_n6, assign35290_e50179_d_n7, assign35290_e50179_d_n10, assign35290_e50179_d_n11, assign35290_e50179_d_n12, assign35290_e50179_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35290_e50164: f64 = (0.4 * locals.var_tratio);
        let assign35290_e50165: f64 = (1.8 + assign35290_e50164);
        let assign35290_e50168: f64 = (0.1 * locals.var_tratio);
        let assign35290_e50170: f64 = (assign35290_e50168 * locals.var_tratio);
        let assign35290_e50171: f64 = (assign35290_e50165 + assign35290_e50170);
        let assign35290_e50175: f64 = (1.0 - locals.var_tratio);
        let assign35290_e50176: f64 = (p.p270 * assign35290_e50175);
        let assign35290_e50177: f64 = (assign35290_e50171 - assign35290_e50176);
        (assign35290_e50177, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35290_e50168 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35290_e50179;
        locals.var_t0_dn0 = assign35290_e50179_d_n0;
        locals.var_t0_dn2 = assign35290_e50179_d_n2;
        locals.var_t0_dn6 = assign35290_e50179_d_n6;
        locals.var_t0_dn7 = assign35290_e50179_d_n7;
        locals.var_t0_dn10 = assign35290_e50179_d_n10;
        locals.var_t0_dn11 = assign35290_e50179_d_n11;
        locals.var_t0_dn12 = assign35290_e50179_d_n12;
        locals.var_t0_dn17 = assign35290_e50179_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign35300_e50185, assign35300_e50185_d_n0, assign35300_e50185_d_n2, assign35300_e50185_d_n6, assign35300_e50185_d_n7, assign35300_e50185_d_n10, assign35300_e50185_d_n11, assign35300_e50185_d_n12, assign35300_e50185_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35300_e50183: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign35300_e50183, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17,)
    }
};
        locals.var_vmaxe__blk1162 = assign35300_e50185;
        locals.var_vmaxe__blk1162_dn0 = assign35300_e50185_d_n0;
        locals.var_vmaxe__blk1162_dn2 = assign35300_e50185_d_n2;
        locals.var_vmaxe__blk1162_dn6 = assign35300_e50185_d_n6;
        locals.var_vmaxe__blk1162_dn7 = assign35300_e50185_d_n7;
        locals.var_vmaxe__blk1162_dn10 = assign35300_e50185_d_n10;
        locals.var_vmaxe__blk1162_dn11 = assign35300_e50185_d_n11;
        locals.var_vmaxe__blk1162_dn12 = assign35300_e50185_d_n12;
        locals.var_vmaxe__blk1162_dn17 = assign35300_e50185_d_n17;
        locals.var_vmaxe__blk1162_rv = 0.0;

        let (assign35310_e50195, assign35310_e50195_d_n10,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35310_e50191: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35310_e50192: f64 = (p.p274 * assign35310_e50191);
        let assign35310_e50193: f64 = (locals.var_rrdrbb + assign35310_e50192);
        (assign35310_e50193, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35310_e50195;
        locals.var_rrdrbb_dn10 = assign35310_e50195_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35320_e50205,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35320_e50201: f64 = (locals.var_lgle).powf(p.p280);
        let assign35320_e50202: f64 = (p.p279 / assign35320_e50201);
        let assign35320_e50203: f64 = (1.0 + assign35320_e50202);
        (assign35320_e50203,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign35320_e50205;
        locals.var_rdrmuele_rv = 0.0;

        let (assign35330_e50215,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35330_e50211: f64 = (locals.var_lgle).powf(p.p278);
        let assign35330_e50212: f64 = (p.p277 / assign35330_e50211);
        let assign35330_e50213: f64 = (1.0 + assign35330_e50212);
        (assign35330_e50213,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign35330_e50215;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign35340_e50225,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35340_e50221: f64 = (locals.var_wg).powf(p.p276);
        let assign35340_e50222: f64 = (p.p275 / assign35340_e50221);
        let assign35340_e50223: f64 = (1.0 + assign35340_e50222);
        (assign35340_e50223,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign35340_e50225;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign35350_e50231, assign35350_e50231_d_n0, assign35350_e50231_d_n2, assign35350_e50231_d_n6, assign35350_e50231_d_n7, assign35350_e50231_d_n10, assign35350_e50231_d_n11, assign35350_e50231_d_n12, assign35350_e50231_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35350_e50229: f64 = (locals.var_mu0 * locals.var_rdrmuele);
        (assign35350_e50229, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35350_e50231;
        locals.var_mu0_dn0 = assign35350_e50231_d_n0;
        locals.var_mu0_dn2 = assign35350_e50231_d_n2;
        locals.var_mu0_dn6 = assign35350_e50231_d_n6;
        locals.var_mu0_dn7 = assign35350_e50231_d_n7;
        locals.var_mu0_dn10 = assign35350_e50231_d_n10;
        locals.var_mu0_dn11 = assign35350_e50231_d_n11;
        locals.var_mu0_dn12 = assign35350_e50231_d_n12;
        locals.var_mu0_dn17 = assign35350_e50231_d_n17;
        locals.var_mu0_rv = 0.0;

        let (assign35360_e50241, assign35360_e50241_d_n0, assign35360_e50241_d_n2, assign35360_e50241_d_n6, assign35360_e50241_d_n7, assign35360_e50241_d_n10, assign35360_e50241_d_n11, assign35360_e50241_d_n12, assign35360_e50241_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35360_e50235: f64 = (locals.var_vmaxe__blk1162 * locals.var_rdrvmaxwe);
        let assign35360_e50237: f64 = (assign35360_e50235 * locals.var_rdrvmaxle);
        let assign35360_e50239: f64 = (assign35360_e50237 + 1e-50);
        (assign35360_e50239, ((locals.var_vmaxe__blk1162_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17,)
    }
};
        locals.var_vmaxe__blk1162 = assign35360_e50241;
        locals.var_vmaxe__blk1162_dn0 = assign35360_e50241_d_n0;
        locals.var_vmaxe__blk1162_dn2 = assign35360_e50241_d_n2;
        locals.var_vmaxe__blk1162_dn6 = assign35360_e50241_d_n6;
        locals.var_vmaxe__blk1162_dn7 = assign35360_e50241_d_n7;
        locals.var_vmaxe__blk1162_dn10 = assign35360_e50241_d_n10;
        locals.var_vmaxe__blk1162_dn11 = assign35360_e50241_d_n11;
        locals.var_vmaxe__blk1162_dn12 = assign35360_e50241_d_n12;
        locals.var_vmaxe__blk1162_dn17 = assign35360_e50241_d_n17;
        locals.var_vmaxe__blk1162_rv = 0.0;

        let (assign35370_e50247, assign35370_e50247_d_n0, assign35370_e50247_d_n2, assign35370_e50247_d_n6, assign35370_e50247_d_n7,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35370_e50245: f64 = (locals.var_vrdr / locals.var_ldrifte);
        (assign35370_e50245, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7,)
    }
};
        locals.var_edri = assign35370_e50247;
        locals.var_edri_dn0 = assign35370_e50247_d_n0;
        locals.var_edri_dn2 = assign35370_e50247_d_n2;
        locals.var_edri_dn6 = assign35370_e50247_d_n6;
        locals.var_edri_dn7 = assign35370_e50247_d_n7;
        locals.var_edri_rv = 0.0;

        let (assign35380_e50253, assign35380_e50253_d_n0, assign35380_e50253_d_n2, assign35380_e50253_d_n6, assign35380_e50253_d_n7, assign35380_e50253_d_n10, assign35380_e50253_d_n11, assign35380_e50253_d_n12, assign35380_e50253_d_n17,) = {
    if (locals.var_guard1149 != 0.0) {
        let assign35380_e50251: f64 = (locals.var_mu0 * locals.var_edri);
        (assign35380_e50251, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17,)
    }
};
        locals.var_vdri = assign35380_e50253;
        locals.var_vdri_dn0 = assign35380_e50253_d_n0;
        locals.var_vdri_dn2 = assign35380_e50253_d_n2;
        locals.var_vdri_dn6 = assign35380_e50253_d_n6;
        locals.var_vdri_dn7 = assign35380_e50253_d_n7;
        locals.var_vdri_dn10 = assign35380_e50253_d_n10;
        locals.var_vdri_dn11 = assign35380_e50253_d_n11;
        locals.var_vdri_dn12 = assign35380_e50253_d_n12;
        locals.var_vdri_dn17 = assign35380_e50253_d_n17;
        locals.var_vdri_rv = 0.0;

        let assign35390_e50256: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign35390_e50256;
        locals.var_guard1170_rv = 0.0;

        let (assign35400_e50264, assign35400_e50264_d_n0, assign35400_e50264_d_n2, assign35400_e50264_d_n6, assign35400_e50264_d_n7, assign35400_e50264_d_n10, assign35400_e50264_d_n11, assign35400_e50264_d_n12, assign35400_e50264_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign35400_e50262: f64 = (locals.var_vdri / locals.var_vmaxe__blk1162);
        (assign35400_e50262, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35400_e50264;
        locals.var_t1_dn0 = assign35400_e50264_d_n0;
        locals.var_t1_dn2 = assign35400_e50264_d_n2;
        locals.var_t1_dn6 = assign35400_e50264_d_n6;
        locals.var_t1_dn7 = assign35400_e50264_d_n7;
        locals.var_t1_dn10 = assign35400_e50264_d_n10;
        locals.var_t1_dn11 = assign35400_e50264_d_n11;
        locals.var_t1_dn12 = assign35400_e50264_d_n12;
        locals.var_t1_dn17 = assign35400_e50264_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign35410_e50274, assign35410_e50274_d_n0, assign35410_e50274_d_n2, assign35410_e50274_d_n6, assign35410_e50274_d_n7, assign35410_e50274_d_n10, assign35410_e50274_d_n11, assign35410_e50274_d_n12, assign35410_e50274_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 == 0.0)) {
        let assign35410_e50270: f64 = (-locals.var_vdri);
        let assign35410_e50272: f64 = (assign35410_e50270 / locals.var_vmaxe__blk1162);
        (assign35410_e50272, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35410_e50274;
        locals.var_t1_dn0 = assign35410_e50274_d_n0;
        locals.var_t1_dn2 = assign35410_e50274_d_n2;
        locals.var_t1_dn6 = assign35410_e50274_d_n6;
        locals.var_t1_dn7 = assign35410_e50274_d_n7;
        locals.var_t1_dn10 = assign35410_e50274_d_n10;
        locals.var_t1_dn11 = assign35410_e50274_d_n11;
        locals.var_t1_dn12 = assign35410_e50274_d_n12;
        locals.var_t1_dn17 = assign35410_e50274_d_n17;
        locals.var_t1_rv = 0.0;

        let assign35420_e50278: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50279: f64 = (1.0 - assign35420_e50278);
        let assign35420_e50286: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50287: f64 = (1.0 + assign35420_e50286);
        let assign35420_e50289: f64 = if ((assign35420_e50279 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35420_e50287)) { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35420_e50289;
        locals.var_guard1171_rv = 0.0;

        let (assign35430_e50295, assign35430_e50295_d_n0, assign35430_e50295_d_n2, assign35430_e50295_d_n6, assign35430_e50295_d_n7, assign35430_e50295_d_n10, assign35430_e50295_d_n11, assign35430_e50295_d_n12, assign35430_e50295_d_n17,) = {
    if ((locals.var_guard1149 != 0.0) && (locals.var_guard1171 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35430_e50295;
        locals.var_t3_dn0 = assign35430_e50295_d_n0;
        locals.var_t3_dn2 = assign35430_e50295_d_n2;
        locals.var_t3_dn6 = assign35430_e50295_d_n6;
        locals.var_t3_dn7 = assign35430_e50295_d_n7;
        locals.var_t3_dn10 = assign35430_e50295_d_n10;
        locals.var_t3_dn11 = assign35430_e50295_d_n11;
        locals.var_t3_dn12 = assign35430_e50295_d_n12;
        locals.var_t3_dn17 = assign35430_e50295_d_n17;
        locals.var_t3_rv = 0.0;

        let assign35440_e50299: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50300: f64 = (2.0 - assign35440_e50299);
        let assign35440_e50307: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50308: f64 = (2.0 + assign35440_e50307);
        let assign35440_e50310: f64 = if ((assign35440_e50300 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50308)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35440_e50310;
        locals.var_guard1172_rv = 0.0;

        let (assign35450_e50319, assign35450_e50319_d_n0, assign35450_e50319_d_n2, assign35450_e50319_d_n6, assign35450_e50319_d_n7, assign35450_e50319_d_n10, assign35450_e50319_d_n11, assign35450_e50319_d_n12, assign35450_e50319_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35450_e50319;
        locals.var_t3_dn0 = assign35450_e50319_d_n0;
        locals.var_t3_dn2 = assign35450_e50319_d_n2;
        locals.var_t3_dn6 = assign35450_e50319_d_n6;
        locals.var_t3_dn7 = assign35450_e50319_d_n7;
        locals.var_t3_dn10 = assign35450_e50319_d_n10;
        locals.var_t3_dn11 = assign35450_e50319_d_n11;
        locals.var_t3_dn12 = assign35450_e50319_d_n12;
        locals.var_t3_dn17 = assign35450_e50319_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign35460_e50333, assign35460_e50333_d_n0, assign35460_e50333_d_n2, assign35460_e50333_d_n6, assign35460_e50333_d_n7, assign35460_e50333_d_n10, assign35460_e50333_d_n11, assign35460_e50333_d_n12, assign35460_e50333_d_n17,) = {
    if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 == 0.0)) {
        let assign35460_e50330: f64 = (locals.var_rrdrbb - 1.0);
        let assign35460_e50331: f64 = (locals.var_t1).powf(assign35460_e50330);
        (assign35460_e50331, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn0)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn2)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn6)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn7)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn10)) } } else { (assign35460_e50331 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35460_e50330 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn11)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn12)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn17)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35460_e50333;
        locals.var_t3_dn0 = assign35460_e50333_d_n0;
        locals.var_t3_dn2 = assign35460_e50333_d_n2;
        locals.var_t3_dn6 = assign35460_e50333_d_n6;
        locals.var_t3_dn7 = assign35460_e50333_d_n7;
        locals.var_t3_dn10 = assign35460_e50333_d_n10;
        locals.var_t3_dn11 = assign35460_e50333_d_n11;
        locals.var_t3_dn12 = assign35460_e50333_d_n12;
        locals.var_t3_dn17 = assign35460_e50333_d_n17;
        locals.var_t3_rv = 0.0;

    }
}
