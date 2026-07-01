#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15210_e21051, assign15210_e21051_d_n0, assign15210_e21051_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15210_e21047: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15210_e21048: f64 = (1.0 + assign15210_e21047);
        let assign15210_e21049: f64 = (0.5 * assign15210_e21048);
        (assign15210_e21049, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign15210_e21051;
        locals.var_dfn_su_dn0 = assign15210_e21051_d_n0;
        locals.var_dfn_su_dn2 = assign15210_e21051_d_n2;

        let (assign15220_e21068, assign15220_e21068_d_n0, assign15220_e21068_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15220_e21064: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15220_e21065: f64 = (0.5 * assign15220_e21064);
        let assign15220_e21066: f64 = (p.p85 - assign15220_e21065);
        (assign15220_e21066, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign15220_e21068;
        locals.var_nja11_dn0 = assign15220_e21068_d_n0;
        locals.var_nja11_dn2 = assign15220_e21068_d_n2;

        let (assign15230_e21083, assign15230_e21083_d_n0, assign15230_e21083_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15230_e21079: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign15230_e21081: f64 = (assign15230_e21079 - 0.01);
        (assign15230_e21081, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15230_e21083;
        locals.var_tmf1_dn0 = assign15230_e21083_d_n0;
        locals.var_tmf1_dn2 = assign15230_e21083_d_n2;

        let (assign15240_e21098, assign15240_e21098_d_n0, assign15240_e21098_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15240_e21094: f64 = (4.0 * locals.var_nfasti_i);
        let assign15240_e21096: f64 = (assign15240_e21094 * 0.01);
        (assign15240_e21096, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15240_e21098;
        locals.var_tmf2_dn0 = assign15240_e21098_d_n0;
        locals.var_tmf2_dn2 = assign15240_e21098_d_n2;

        let (assign15250_e21115, assign15250_e21115_d_n0, assign15250_e21115_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15250_e21112: f64 = (-locals.var_tmf2);
                (assign15250_e21112, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15250_e21115;
        locals.var_tmf2_dn0 = assign15250_e21115_d_n0;
        locals.var_tmf2_dn2 = assign15250_e21115_d_n2;

        let (assign15260_e21131, assign15260_e21131_d_n0, assign15260_e21131_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15260_e21126: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15260_e21128: f64 = (assign15260_e21126 + locals.var_tmf2);
        let assign15260_e21129: f64 = (assign15260_e21128).sqrt();
        (assign15260_e21129, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15260_e21129)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15260_e21129)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15260_e21131;
        locals.var_tmf2_dn0 = assign15260_e21131_d_n0;
        locals.var_tmf2_dn2 = assign15260_e21131_d_n2;

        let (assign15270_e21148, assign15270_e21148_d_n0, assign15270_e21148_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15270_e21144: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15270_e21145: f64 = (1.0 + assign15270_e21144);
        let assign15270_e21146: f64 = (0.5 * assign15270_e21145);
        (assign15270_e21146, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign15270_e21148;
        locals.var_dfn_sl_dn0 = assign15270_e21148_d_n0;
        locals.var_dfn_sl_dn2 = assign15270_e21148_d_n2;

        let (assign15280_e21165, assign15280_e21165_d_n0, assign15280_e21165_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15280_e21161: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15280_e21162: f64 = (0.5 * assign15280_e21161);
        let assign15280_e21163: f64 = (locals.var_nfasti_i + assign15280_e21162);
        (assign15280_e21163, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign15280_e21165;
        locals.var_nj1_dn0 = assign15280_e21165_d_n0;
        locals.var_nj1_dn2 = assign15280_e21165_d_n2;

        let (assign15290_e21180, assign15290_e21180_d_n0, assign15290_e21180_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15290_e21176: f64 = (p.p85 - locals.var_nj0);
        let assign15290_e21178: f64 = (assign15290_e21176 - 0.01);
        (assign15290_e21178, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15290_e21180;
        locals.var_tmf1_dn0 = assign15290_e21180_d_n0;
        locals.var_tmf1_dn2 = assign15290_e21180_d_n2;

        let (assign15300_e21195, assign15300_e21195_d_n0, assign15300_e21195_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15300_e21191: f64 = (4.0 * p.p85);
        let assign15300_e21193: f64 = (assign15300_e21191 * 0.01);
        (assign15300_e21193, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15300_e21195;
        locals.var_tmf2_dn0 = assign15300_e21195_d_n0;
        locals.var_tmf2_dn2 = assign15300_e21195_d_n2;

        let (assign15310_e21212, assign15310_e21212_d_n0, assign15310_e21212_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15310_e21209: f64 = (-locals.var_tmf2);
                (assign15310_e21209, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15310_e21212;
        locals.var_tmf2_dn0 = assign15310_e21212_d_n0;
        locals.var_tmf2_dn2 = assign15310_e21212_d_n2;

        let (assign15320_e21228, assign15320_e21228_d_n0, assign15320_e21228_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15320_e21223: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15320_e21225: f64 = (assign15320_e21223 + locals.var_tmf2);
        let assign15320_e21226: f64 = (assign15320_e21225).sqrt();
        (assign15320_e21226, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15320_e21226)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15320_e21226)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15320_e21228;
        locals.var_tmf2_dn0 = assign15320_e21228_d_n0;
        locals.var_tmf2_dn2 = assign15320_e21228_d_n2;

        let (assign15330_e21245, assign15330_e21245_d_n0, assign15330_e21245_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15330_e21241: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15330_e21242: f64 = (0.5 * assign15330_e21241);
        let assign15330_e21243: f64 = (p.p85 - assign15330_e21242);
        (assign15330_e21243, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15330_e21245;
        locals.var_nj0_dn0 = assign15330_e21245_d_n0;
        locals.var_nj0_dn2 = assign15330_e21245_d_n2;

        let (assign15340_e21260, assign15340_e21260_d_n0, assign15340_e21260_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15340_e21256: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign15340_e21258: f64 = (assign15340_e21256 - 0.01);
        (assign15340_e21258, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15340_e21260;
        locals.var_tmf1_dn0 = assign15340_e21260_d_n0;
        locals.var_tmf1_dn2 = assign15340_e21260_d_n2;

        let (assign15350_e21275, assign15350_e21275_d_n0, assign15350_e21275_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15350_e21271: f64 = (4.0 * locals.var_nfasti_i);
        let assign15350_e21273: f64 = (assign15350_e21271 * 0.01);
        (assign15350_e21273, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15350_e21275;
        locals.var_tmf2_dn0 = assign15350_e21275_d_n0;
        locals.var_tmf2_dn2 = assign15350_e21275_d_n2;

        let (assign15360_e21292, assign15360_e21292_d_n0, assign15360_e21292_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15360_e21289: f64 = (-locals.var_tmf2);
                (assign15360_e21289, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15360_e21292;
        locals.var_tmf2_dn0 = assign15360_e21292_d_n0;
        locals.var_tmf2_dn2 = assign15360_e21292_d_n2;

        let (assign15370_e21308, assign15370_e21308_d_n0, assign15370_e21308_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15370_e21303: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15370_e21305: f64 = (assign15370_e21303 + locals.var_tmf2);
        let assign15370_e21306: f64 = (assign15370_e21305).sqrt();
        (assign15370_e21306, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15370_e21306)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15370_e21306)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15370_e21308;
        locals.var_tmf2_dn0 = assign15370_e21308_d_n0;
        locals.var_tmf2_dn2 = assign15370_e21308_d_n2;

        let (assign15380_e21325, assign15380_e21325_d_n0, assign15380_e21325_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15380_e21321: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15380_e21322: f64 = (0.5 * assign15380_e21321);
        let assign15380_e21323: f64 = (locals.var_nfasti_i + assign15380_e21322);
        (assign15380_e21323, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15380_e21325;
        locals.var_nj0_dn0 = assign15380_e21325_d_n0;
        locals.var_nj0_dn2 = assign15380_e21325_d_n2;

        let (assign15390_e21340, assign15390_e21340_d_n0, assign15390_e21340_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign15390_e21336: f64 = (p.p86 * locals.var_dfn_su);
        let assign15390_e21338: f64 = (assign15390_e21336 * locals.var_dfn_sl);
        (assign15390_e21338, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign15390_e21336 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign15390_e21336 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15390_e21340;
        locals.var_dnj1_dv_dn0 = assign15390_e21340_d_n0;
        locals.var_dnj1_dv_dn2 = assign15390_e21340_d_n2;

        let (assign15400_e21352, assign15400_e21352_d_n0, assign15400_e21352_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15400_e21352;
        locals.var_nj0_dn0 = assign15400_e21352_d_n0;
        locals.var_nj0_dn2 = assign15400_e21352_d_n2;

        let (assign15410_e21364, assign15410_e21364_d_n0, assign15410_e21364_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign15410_e21364;
        locals.var_nj1_dn0 = assign15410_e21364_d_n0;
        locals.var_nj1_dn2 = assign15410_e21364_d_n2;

        let (assign15420_e21376, assign15420_e21376_d_n0, assign15420_e21376_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard246 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15420_e21376;
        locals.var_dnj1_dv_dn0 = assign15420_e21376_d_n0;
        locals.var_dnj1_dv_dn2 = assign15420_e21376_d_n2;

        let (assign15480_e21625, assign15480_e21625_d_n0, assign15480_e21625_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15480_e21609: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign15480_e21610: f64 = (locals.var_nj1 - assign15480_e21609);
        let assign15480_e21613: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign15480_e21614: f64 = (assign15480_e21610 / assign15480_e21613);
        let assign15480_e21617: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign15480_e21620: f64 = (locals.var_nj0 * p.p85);
        let assign15480_e21621: f64 = (assign15480_e21617 / assign15480_e21620);
        let assign15480_e21622: f64 = (assign15480_e21614 + assign15480_e21621);
        let assign15480_e21623: f64 = (locals.var_phitdinv * assign15480_e21622);
        (assign15480_e21623, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign15480_e21613) - (assign15480_e21610 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign15480_e21613 * assign15480_e21613)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign15480_e21620) - (assign15480_e21617 * (locals.var_nj0_dn0 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign15480_e21613) - (assign15480_e21610 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign15480_e21613 * assign15480_e21613)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign15480_e21620) - (assign15480_e21617 * (locals.var_nj0_dn2 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign15480_e21625;
        locals.var_dvmax_over_phitd_dv_dn0 = assign15480_e21625_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign15480_e21625_d_n2;

        let (assign15500_e21655,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15500_e21651: f64 = (locals.var_nin * locals.var_nin);
        let assign15500_e21653: f64 = (assign15500_e21651 / locals.var_ndigat_i);
        (assign15500_e21653,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign15500_e21655;

        let (assign15510_e21671,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15510_e21664: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign15510_e21667: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign15510_e21668: f64 = (assign15510_e21667).ln();
        let assign15510_e21669: f64 = (assign15510_e21664 * assign15510_e21668);
        (assign15510_e21669,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign15510_e21671;

        let assign15520_e21674: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign15520_e21674;

        let (assign15530_e21691, assign15530_e21691_d_n0, assign15530_e21691_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15530_e21686: f64 = (locals.var_vmax - locals.var_vha1);
        let assign15530_e21687: f64 = (p.p86 * assign15530_e21686);
        let assign15530_e21689: f64 = (assign15530_e21687 + locals.var_nfagat_i);
        (assign15530_e21689, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign15530_e21691;
        locals.var_nja10_dn0 = assign15530_e21691_d_n0;
        locals.var_nja10_dn2 = assign15530_e21691_d_n2;

        let (assign15540_e21706, assign15540_e21706_d_n0, assign15540_e21706_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15540_e21703: f64 = (p.p86 * locals.var_vha1);
        let assign15540_e21704: f64 = (locals.var_nfagat_i - assign15540_e21703);
        (assign15540_e21704, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15540_e21706;
        locals.var_nj0_dn0 = assign15540_e21706_d_n0;
        locals.var_nj0_dn2 = assign15540_e21706_d_n2;

        let (assign15550_e21721, assign15550_e21721_d_n0, assign15550_e21721_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15550_e21717: f64 = (p.p85 - locals.var_nja10);
        let assign15550_e21719: f64 = (assign15550_e21717 - 0.01);
        (assign15550_e21719, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15550_e21721;
        locals.var_tmf1_dn0 = assign15550_e21721_d_n0;
        locals.var_tmf1_dn2 = assign15550_e21721_d_n2;

        let (assign15560_e21736, assign15560_e21736_d_n0, assign15560_e21736_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15560_e21732: f64 = (4.0 * p.p85);
        let assign15560_e21734: f64 = (assign15560_e21732 * 0.01);
        (assign15560_e21734, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15560_e21736;
        locals.var_tmf2_dn0 = assign15560_e21736_d_n0;
        locals.var_tmf2_dn2 = assign15560_e21736_d_n2;

        let (assign15570_e21753, assign15570_e21753_d_n0, assign15570_e21753_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15570_e21750: f64 = (-locals.var_tmf2);
                (assign15570_e21750, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15570_e21753;
        locals.var_tmf2_dn0 = assign15570_e21753_d_n0;
        locals.var_tmf2_dn2 = assign15570_e21753_d_n2;

        let (assign15580_e21769, assign15580_e21769_d_n0, assign15580_e21769_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15580_e21764: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15580_e21766: f64 = (assign15580_e21764 + locals.var_tmf2);
        let assign15580_e21767: f64 = (assign15580_e21766).sqrt();
        (assign15580_e21767, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15580_e21767)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15580_e21767)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15580_e21769;
        locals.var_tmf2_dn0 = assign15580_e21769_d_n0;
        locals.var_tmf2_dn2 = assign15580_e21769_d_n2;

        let (assign15590_e21786, assign15590_e21786_d_n0, assign15590_e21786_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15590_e21782: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15590_e21783: f64 = (1.0 + assign15590_e21782);
        let assign15590_e21784: f64 = (0.5 * assign15590_e21783);
        (assign15590_e21784, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign15590_e21786;
        locals.var_dfn_su_dn0 = assign15590_e21786_d_n0;
        locals.var_dfn_su_dn2 = assign15590_e21786_d_n2;

        let (assign15600_e21803, assign15600_e21803_d_n0, assign15600_e21803_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15600_e21799: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15600_e21800: f64 = (0.5 * assign15600_e21799);
        let assign15600_e21801: f64 = (p.p85 - assign15600_e21800);
        (assign15600_e21801, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign15600_e21803;
        locals.var_nja11_dn0 = assign15600_e21803_d_n0;
        locals.var_nja11_dn2 = assign15600_e21803_d_n2;

        let (assign15610_e21818, assign15610_e21818_d_n0, assign15610_e21818_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15610_e21814: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign15610_e21816: f64 = (assign15610_e21814 - 0.01);
        (assign15610_e21816, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15610_e21818;
        locals.var_tmf1_dn0 = assign15610_e21818_d_n0;
        locals.var_tmf1_dn2 = assign15610_e21818_d_n2;

        let (assign15620_e21833, assign15620_e21833_d_n0, assign15620_e21833_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15620_e21829: f64 = (4.0 * locals.var_nfagat_i);
        let assign15620_e21831: f64 = (assign15620_e21829 * 0.01);
        (assign15620_e21831, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15620_e21833;
        locals.var_tmf2_dn0 = assign15620_e21833_d_n0;
        locals.var_tmf2_dn2 = assign15620_e21833_d_n2;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15630_e21850, assign15630_e21850_d_n0, assign15630_e21850_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15630_e21847: f64 = (-locals.var_tmf2);
                (assign15630_e21847, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15630_e21850;
        locals.var_tmf2_dn0 = assign15630_e21850_d_n0;
        locals.var_tmf2_dn2 = assign15630_e21850_d_n2;

        let (assign15640_e21866, assign15640_e21866_d_n0, assign15640_e21866_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15640_e21861: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15640_e21863: f64 = (assign15640_e21861 + locals.var_tmf2);
        let assign15640_e21864: f64 = (assign15640_e21863).sqrt();
        (assign15640_e21864, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15640_e21864)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15640_e21864)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15640_e21866;
        locals.var_tmf2_dn0 = assign15640_e21866_d_n0;
        locals.var_tmf2_dn2 = assign15640_e21866_d_n2;

        let (assign15650_e21883, assign15650_e21883_d_n0, assign15650_e21883_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15650_e21879: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15650_e21880: f64 = (1.0 + assign15650_e21879);
        let assign15650_e21881: f64 = (0.5 * assign15650_e21880);
        (assign15650_e21881, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign15650_e21883;
        locals.var_dfn_sl_dn0 = assign15650_e21883_d_n0;
        locals.var_dfn_sl_dn2 = assign15650_e21883_d_n2;

        let (assign15660_e21900, assign15660_e21900_d_n0, assign15660_e21900_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15660_e21896: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15660_e21897: f64 = (0.5 * assign15660_e21896);
        let assign15660_e21898: f64 = (locals.var_nfagat_i + assign15660_e21897);
        (assign15660_e21898, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign15660_e21900;
        locals.var_nj1_dn0 = assign15660_e21900_d_n0;
        locals.var_nj1_dn2 = assign15660_e21900_d_n2;

        let (assign15670_e21915, assign15670_e21915_d_n0, assign15670_e21915_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15670_e21911: f64 = (p.p85 - locals.var_nj0);
        let assign15670_e21913: f64 = (assign15670_e21911 - 0.01);
        (assign15670_e21913, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15670_e21915;
        locals.var_tmf1_dn0 = assign15670_e21915_d_n0;
        locals.var_tmf1_dn2 = assign15670_e21915_d_n2;

        let (assign15680_e21930, assign15680_e21930_d_n0, assign15680_e21930_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15680_e21926: f64 = (4.0 * p.p85);
        let assign15680_e21928: f64 = (assign15680_e21926 * 0.01);
        (assign15680_e21928, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15680_e21930;
        locals.var_tmf2_dn0 = assign15680_e21930_d_n0;
        locals.var_tmf2_dn2 = assign15680_e21930_d_n2;

        let (assign15690_e21947, assign15690_e21947_d_n0, assign15690_e21947_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15690_e21944: f64 = (-locals.var_tmf2);
                (assign15690_e21944, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15690_e21947;
        locals.var_tmf2_dn0 = assign15690_e21947_d_n0;
        locals.var_tmf2_dn2 = assign15690_e21947_d_n2;

        let (assign15700_e21963, assign15700_e21963_d_n0, assign15700_e21963_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15700_e21958: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15700_e21960: f64 = (assign15700_e21958 + locals.var_tmf2);
        let assign15700_e21961: f64 = (assign15700_e21960).sqrt();
        (assign15700_e21961, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15700_e21961)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15700_e21961)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15700_e21963;
        locals.var_tmf2_dn0 = assign15700_e21963_d_n0;
        locals.var_tmf2_dn2 = assign15700_e21963_d_n2;

        let (assign15710_e21980, assign15710_e21980_d_n0, assign15710_e21980_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15710_e21976: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15710_e21977: f64 = (0.5 * assign15710_e21976);
        let assign15710_e21978: f64 = (p.p85 - assign15710_e21977);
        (assign15710_e21978, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15710_e21980;
        locals.var_nj0_dn0 = assign15710_e21980_d_n0;
        locals.var_nj0_dn2 = assign15710_e21980_d_n2;

        let (assign15720_e21995, assign15720_e21995_d_n0, assign15720_e21995_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15720_e21991: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign15720_e21993: f64 = (assign15720_e21991 - 0.01);
        (assign15720_e21993, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign15720_e21995;
        locals.var_tmf1_dn0 = assign15720_e21995_d_n0;
        locals.var_tmf1_dn2 = assign15720_e21995_d_n2;

        let (assign15730_e22010, assign15730_e22010_d_n0, assign15730_e22010_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15730_e22006: f64 = (4.0 * locals.var_nfagat_i);
        let assign15730_e22008: f64 = (assign15730_e22006 * 0.01);
        (assign15730_e22008, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15730_e22010;
        locals.var_tmf2_dn0 = assign15730_e22010_d_n0;
        locals.var_tmf2_dn2 = assign15730_e22010_d_n2;

        let (assign15740_e22027, assign15740_e22027_d_n0, assign15740_e22027_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign15740_e22024: f64 = (-locals.var_tmf2);
                (assign15740_e22024, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15740_e22027;
        locals.var_tmf2_dn0 = assign15740_e22027_d_n0;
        locals.var_tmf2_dn2 = assign15740_e22027_d_n2;

        let (assign15750_e22043, assign15750_e22043_d_n0, assign15750_e22043_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15750_e22038: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15750_e22040: f64 = (assign15750_e22038 + locals.var_tmf2);
        let assign15750_e22041: f64 = (assign15750_e22040).sqrt();
        (assign15750_e22041, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15750_e22041)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15750_e22041)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign15750_e22043;
        locals.var_tmf2_dn0 = assign15750_e22043_d_n0;
        locals.var_tmf2_dn2 = assign15750_e22043_d_n2;

        let (assign15760_e22060, assign15760_e22060_d_n0, assign15760_e22060_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15760_e22056: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15760_e22057: f64 = (0.5 * assign15760_e22056);
        let assign15760_e22058: f64 = (locals.var_nfagat_i + assign15760_e22057);
        (assign15760_e22058, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15760_e22060;
        locals.var_nj0_dn0 = assign15760_e22060_d_n0;
        locals.var_nj0_dn2 = assign15760_e22060_d_n2;

        let (assign15770_e22075, assign15770_e22075_d_n0, assign15770_e22075_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign15770_e22071: f64 = (p.p86 * locals.var_dfn_su);
        let assign15770_e22073: f64 = (assign15770_e22071 * locals.var_dfn_sl);
        (assign15770_e22073, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign15770_e22071 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign15770_e22071 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15770_e22075;
        locals.var_dnj1_dv_dn0 = assign15770_e22075_d_n0;
        locals.var_dnj1_dv_dn2 = assign15770_e22075_d_n2;

        let (assign15780_e22087, assign15780_e22087_d_n0, assign15780_e22087_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign15780_e22087;
        locals.var_nj0_dn0 = assign15780_e22087_d_n0;
        locals.var_nj0_dn2 = assign15780_e22087_d_n2;

        let (assign15790_e22099, assign15790_e22099_d_n0, assign15790_e22099_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign15790_e22099;
        locals.var_nj1_dn0 = assign15790_e22099_d_n0;
        locals.var_nj1_dn2 = assign15790_e22099_d_n2;

        let (assign15800_e22111, assign15800_e22111_d_n0, assign15800_e22111_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign15800_e22111;
        locals.var_dnj1_dv_dn0 = assign15800_e22111_d_n0;
        locals.var_dnj1_dv_dn2 = assign15800_e22111_d_n2;

        let (assign15860_e22360, assign15860_e22360_d_n0, assign15860_e22360_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign15860_e22344: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign15860_e22345: f64 = (locals.var_nj1 - assign15860_e22344);
        let assign15860_e22348: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign15860_e22349: f64 = (assign15860_e22345 / assign15860_e22348);
        let assign15860_e22352: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign15860_e22355: f64 = (locals.var_nj0 * p.p85);
        let assign15860_e22356: f64 = (assign15860_e22352 / assign15860_e22355);
        let assign15860_e22357: f64 = (assign15860_e22349 + assign15860_e22356);
        let assign15860_e22358: f64 = (locals.var_phitdinv * assign15860_e22357);
        (assign15860_e22358, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign15860_e22348) - (assign15860_e22345 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign15860_e22348 * assign15860_e22348)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign15860_e22355) - (assign15860_e22352 * (locals.var_nj0_dn0 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign15860_e22348) - (assign15860_e22345 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign15860_e22348 * assign15860_e22348)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign15860_e22355) - (assign15860_e22352 * (locals.var_nj0_dn2 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign15860_e22360;
        locals.var_dvmax_over_phitd_dv_dn0 = assign15860_e22360_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign15860_e22360_d_n2;

        let (assign15880_e22385, assign15880_e22385_d_n0, assign15880_e22385_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard230 != 0.0)) {
        let assign15880_e22383: f64 = (locals.var_idmultbot - 1.0);
        (assign15880_e22383, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign15880_e22385;
        locals.var_idmultbot_dn0 = assign15880_e22385_d_n0;
        locals.var_idmultbot_dn2 = assign15880_e22385_d_n2;

        let (assign15990_e22558, assign15990_e22558_d_n0, assign15990_e22558_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard230 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign15990_e22558;
        locals.var_idmultbot_dn0 = assign15990_e22558_d_n0;
        locals.var_idmultbot_dn2 = assign15990_e22558_d_n2;

        let assign18520_e26108: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard307 = assign18520_e26108;

        let assign18600_e26180: f64 = if locals.var_v4 < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard308 = assign18600_e26180;

        let (assign18660_e26321,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign18660_e26317: f64 = (locals.var_nin * locals.var_nin);
        let assign18660_e26319: f64 = (assign18660_e26317 / locals.var_ndibot_i);
        (assign18660_e26319,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign18660_e26321;

        let (assign18670_e26336,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign18670_e26329: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign18670_e26332: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign18670_e26333: f64 = (assign18670_e26332).ln();
        let assign18670_e26334: f64 = (assign18670_e26329 * assign18670_e26333);
        (assign18670_e26334,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign18670_e26336;

        let assign18680_e26339: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign18680_e26339;

        let (assign18690_e26355, assign18690_e26355_d_n0, assign18690_e26355_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18690_e26350: f64 = (locals.var_v4 - locals.var_vha1);
        let assign18690_e26351: f64 = (p.p86 * assign18690_e26350);
        let assign18690_e26353: f64 = (assign18690_e26351 + locals.var_nfabot_i);
        (assign18690_e26353, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign18690_e26355;
        locals.var_nja10_dn0 = assign18690_e26355_d_n0;
        locals.var_nja10_dn2 = assign18690_e26355_d_n2;

        let (assign18700_e26369, assign18700_e26369_d_n0, assign18700_e26369_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18700_e26366: f64 = (p.p86 * locals.var_vha1);
        let assign18700_e26367: f64 = (locals.var_nfabot_i - assign18700_e26366);
        (assign18700_e26367, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign18700_e26369;
        locals.var_nj0_dn0 = assign18700_e26369_d_n0;
        locals.var_nj0_dn2 = assign18700_e26369_d_n2;

        let (assign18710_e26383, assign18710_e26383_d_n0, assign18710_e26383_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18710_e26379: f64 = (p.p85 - locals.var_nja10);
        let assign18710_e26381: f64 = (assign18710_e26379 - 0.01);
        (assign18710_e26381, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign18710_e26383;
        locals.var_tmf1_dn0 = assign18710_e26383_d_n0;
        locals.var_tmf1_dn2 = assign18710_e26383_d_n2;

        let (assign18720_e26397, assign18720_e26397_d_n0, assign18720_e26397_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18720_e26393: f64 = (4.0 * p.p85);
        let assign18720_e26395: f64 = (assign18720_e26393 * 0.01);
        (assign18720_e26395, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18720_e26397;
        locals.var_tmf2_dn0 = assign18720_e26397_d_n0;
        locals.var_tmf2_dn2 = assign18720_e26397_d_n2;

        let (assign18730_e26413, assign18730_e26413_d_n0, assign18730_e26413_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign18730_e26410: f64 = (-locals.var_tmf2);
                (assign18730_e26410, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18730_e26413;
        locals.var_tmf2_dn0 = assign18730_e26413_d_n0;
        locals.var_tmf2_dn2 = assign18730_e26413_d_n2;

        let (assign18740_e26428, assign18740_e26428_d_n0, assign18740_e26428_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18740_e26423: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18740_e26425: f64 = (assign18740_e26423 + locals.var_tmf2);
        let assign18740_e26426: f64 = (assign18740_e26425).sqrt();
        (assign18740_e26426, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18740_e26426)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18740_e26426)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18740_e26428;
        locals.var_tmf2_dn0 = assign18740_e26428_d_n0;
        locals.var_tmf2_dn2 = assign18740_e26428_d_n2;

        let (assign18750_e26444, assign18750_e26444_d_n0, assign18750_e26444_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18750_e26440: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18750_e26441: f64 = (0.5 * assign18750_e26440);
        let assign18750_e26442: f64 = (p.p85 - assign18750_e26441);
        (assign18750_e26442, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign18750_e26444;
        locals.var_nja11_dn0 = assign18750_e26444_d_n0;
        locals.var_nja11_dn2 = assign18750_e26444_d_n2;

        let (assign18760_e26458, assign18760_e26458_d_n0, assign18760_e26458_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18760_e26454: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign18760_e26456: f64 = (assign18760_e26454 - 0.01);
        (assign18760_e26456, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign18760_e26458;
        locals.var_tmf1_dn0 = assign18760_e26458_d_n0;
        locals.var_tmf1_dn2 = assign18760_e26458_d_n2;

        let (assign18770_e26472, assign18770_e26472_d_n0, assign18770_e26472_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18770_e26468: f64 = (4.0 * locals.var_nfabot_i);
        let assign18770_e26470: f64 = (assign18770_e26468 * 0.01);
        (assign18770_e26470, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18770_e26472;
        locals.var_tmf2_dn0 = assign18770_e26472_d_n0;
        locals.var_tmf2_dn2 = assign18770_e26472_d_n2;

        let (assign18780_e26488, assign18780_e26488_d_n0, assign18780_e26488_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign18780_e26485: f64 = (-locals.var_tmf2);
                (assign18780_e26485, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18780_e26488;
        locals.var_tmf2_dn0 = assign18780_e26488_d_n0;
        locals.var_tmf2_dn2 = assign18780_e26488_d_n2;

        let (assign18790_e26503, assign18790_e26503_d_n0, assign18790_e26503_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18790_e26498: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18790_e26500: f64 = (assign18790_e26498 + locals.var_tmf2);
        let assign18790_e26501: f64 = (assign18790_e26500).sqrt();
        (assign18790_e26501, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18790_e26501)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18790_e26501)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18790_e26503;
        locals.var_tmf2_dn0 = assign18790_e26503_d_n0;
        locals.var_tmf2_dn2 = assign18790_e26503_d_n2;

        let (assign18800_e26519, assign18800_e26519_d_n0, assign18800_e26519_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18800_e26515: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18800_e26516: f64 = (0.5 * assign18800_e26515);
        let assign18800_e26517: f64 = (locals.var_nfabot_i + assign18800_e26516);
        (assign18800_e26517, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign18800_e26519;
        locals.var_nj1_dn0 = assign18800_e26519_d_n0;
        locals.var_nj1_dn2 = assign18800_e26519_d_n2;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18810_e26533, assign18810_e26533_d_n0, assign18810_e26533_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18810_e26529: f64 = (p.p85 - locals.var_nj0);
        let assign18810_e26531: f64 = (assign18810_e26529 - 0.01);
        (assign18810_e26531, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign18810_e26533;
        locals.var_tmf1_dn0 = assign18810_e26533_d_n0;
        locals.var_tmf1_dn2 = assign18810_e26533_d_n2;

        let (assign18820_e26547, assign18820_e26547_d_n0, assign18820_e26547_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18820_e26543: f64 = (4.0 * p.p85);
        let assign18820_e26545: f64 = (assign18820_e26543 * 0.01);
        (assign18820_e26545, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18820_e26547;
        locals.var_tmf2_dn0 = assign18820_e26547_d_n0;
        locals.var_tmf2_dn2 = assign18820_e26547_d_n2;

        let (assign18830_e26563, assign18830_e26563_d_n0, assign18830_e26563_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign18830_e26560: f64 = (-locals.var_tmf2);
                (assign18830_e26560, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18830_e26563;
        locals.var_tmf2_dn0 = assign18830_e26563_d_n0;
        locals.var_tmf2_dn2 = assign18830_e26563_d_n2;

        let (assign18840_e26578, assign18840_e26578_d_n0, assign18840_e26578_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18840_e26573: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18840_e26575: f64 = (assign18840_e26573 + locals.var_tmf2);
        let assign18840_e26576: f64 = (assign18840_e26575).sqrt();
        (assign18840_e26576, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18840_e26576)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18840_e26576)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18840_e26578;
        locals.var_tmf2_dn0 = assign18840_e26578_d_n0;
        locals.var_tmf2_dn2 = assign18840_e26578_d_n2;

        let (assign18850_e26594, assign18850_e26594_d_n0, assign18850_e26594_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18850_e26590: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18850_e26591: f64 = (0.5 * assign18850_e26590);
        let assign18850_e26592: f64 = (p.p85 - assign18850_e26591);
        (assign18850_e26592, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign18850_e26594;
        locals.var_nj0_dn0 = assign18850_e26594_d_n0;
        locals.var_nj0_dn2 = assign18850_e26594_d_n2;

        let (assign18860_e26608, assign18860_e26608_d_n0, assign18860_e26608_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18860_e26604: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign18860_e26606: f64 = (assign18860_e26604 - 0.01);
        (assign18860_e26606, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign18860_e26608;
        locals.var_tmf1_dn0 = assign18860_e26608_d_n0;
        locals.var_tmf1_dn2 = assign18860_e26608_d_n2;

        let (assign18870_e26622, assign18870_e26622_d_n0, assign18870_e26622_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18870_e26618: f64 = (4.0 * locals.var_nfabot_i);
        let assign18870_e26620: f64 = (assign18870_e26618 * 0.01);
        (assign18870_e26620, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18870_e26622;
        locals.var_tmf2_dn0 = assign18870_e26622_d_n0;
        locals.var_tmf2_dn2 = assign18870_e26622_d_n2;

        let (assign18880_e26638, assign18880_e26638_d_n0, assign18880_e26638_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign18880_e26635: f64 = (-locals.var_tmf2);
                (assign18880_e26635, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18880_e26638;
        locals.var_tmf2_dn0 = assign18880_e26638_d_n0;
        locals.var_tmf2_dn2 = assign18880_e26638_d_n2;

        let (assign18890_e26653, assign18890_e26653_d_n0, assign18890_e26653_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18890_e26648: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18890_e26650: f64 = (assign18890_e26648 + locals.var_tmf2);
        let assign18890_e26651: f64 = (assign18890_e26650).sqrt();
        (assign18890_e26651, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18890_e26651)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18890_e26651)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign18890_e26653;
        locals.var_tmf2_dn0 = assign18890_e26653_d_n0;
        locals.var_tmf2_dn2 = assign18890_e26653_d_n2;

        let (assign18900_e26669, assign18900_e26669_d_n0, assign18900_e26669_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign18900_e26665: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18900_e26666: f64 = (0.5 * assign18900_e26665);
        let assign18900_e26667: f64 = (locals.var_nfabot_i + assign18900_e26666);
        (assign18900_e26667, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign18900_e26669;
        locals.var_nj0_dn0 = assign18900_e26669_d_n0;
        locals.var_nj0_dn2 = assign18900_e26669_d_n2;

        let (assign18910_e26680, assign18910_e26680_d_n0, assign18910_e26680_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign18910_e26680;
        locals.var_nj0_dn0 = assign18910_e26680_d_n0;
        locals.var_nj0_dn2 = assign18910_e26680_d_n2;

        let (assign18920_e26691, assign18920_e26691_d_n0, assign18920_e26691_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard311 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign18920_e26691;
        locals.var_nj1_dn0 = assign18920_e26691_d_n0;
        locals.var_nj1_dn2 = assign18920_e26691_d_n2;

        let assign18930_e26695: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18930_e26699: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18930_e26700: f64 = (locals.var_vha1 * assign18930_e26699);
        let assign18930_e26703: f64 = (locals.var_nj0 * p.p85);
        let assign18930_e26704: f64 = (assign18930_e26700 / assign18930_e26703);
        let assign18930_e26705: f64 = (assign18930_e26695 + assign18930_e26704);
        let assign18930_e26706: f64 = (locals.var_phitdinv * assign18930_e26705);
        let assign18930_e26707: f64 = (assign18930_e26706).abs();
        let assign18930_e26709: f64 = if assign18930_e26707 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign18930_e26709;

        let (assign18940_e26734, assign18940_e26734_d_n0, assign18940_e26734_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign18940_e26720: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18940_e26724: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18940_e26725: f64 = (locals.var_vha1 * assign18940_e26724);
        let assign18940_e26728: f64 = (locals.var_nj0 * p.p85);
        let assign18940_e26729: f64 = (assign18940_e26725 / assign18940_e26728);
        let assign18940_e26730: f64 = (assign18940_e26720 + assign18940_e26729);
        let assign18940_e26731: f64 = (locals.var_phitdinv * assign18940_e26730);
        let assign18940_e26732: f64 = (assign18940_e26731).exp();
        (assign18940_e26732, (assign18940_e26732 * (locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18940_e26728) - (assign18940_e26725 * (locals.var_nj0_dn0 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18940_e26728) - (assign18940_e26725 * (locals.var_nj0_dn2 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign18940_e26734;
        locals.var_idmultbot_dn0 = assign18940_e26734_d_n0;
        locals.var_idmultbot_dn2 = assign18940_e26734_d_n2;

        let assign18950_e26738: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18950_e26742: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18950_e26743: f64 = (locals.var_vha1 * assign18950_e26742);
        let assign18950_e26746: f64 = (locals.var_nj0 * p.p85);
        let assign18950_e26747: f64 = (assign18950_e26743 / assign18950_e26746);
        let assign18950_e26748: f64 = (assign18950_e26738 + assign18950_e26747);
        let assign18950_e26749: f64 = (locals.var_phitdinv * assign18950_e26748);
        let assign18950_e26751: f64 = (-230.25850929940458);
        let assign18950_e26752: f64 = if assign18950_e26749 < assign18950_e26751 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign18950_e26752;

        let (assign18960_e26832, assign18960_e26832_d_n0, assign18960_e26832_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign18960_e26766: f64 = (-230.25850929940458);
        let assign18960_e26770: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18960_e26774: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18960_e26775: f64 = (locals.var_vha1 * assign18960_e26774);
        let assign18960_e26778: f64 = (locals.var_nj0 * p.p85);
        let assign18960_e26779: f64 = (assign18960_e26775 / assign18960_e26778);
        let assign18960_e26780: f64 = (assign18960_e26770 + assign18960_e26779);
        let assign18960_e26781: f64 = (locals.var_phitdinv * assign18960_e26780);
        let assign18960_e26782: f64 = (assign18960_e26766 - assign18960_e26781);
        let assign18960_e26786: f64 = (-230.25850929940458);
        let assign18960_e26790: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18960_e26794: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18960_e26795: f64 = (locals.var_vha1 * assign18960_e26794);
        let assign18960_e26798: f64 = (locals.var_nj0 * p.p85);
        let assign18960_e26799: f64 = (assign18960_e26795 / assign18960_e26798);
        let assign18960_e26800: f64 = (assign18960_e26790 + assign18960_e26799);
        let assign18960_e26801: f64 = (locals.var_phitdinv * assign18960_e26800);
        let assign18960_e26802: f64 = (assign18960_e26786 - assign18960_e26801);
        let assign18960_e26805: f64 = (-230.25850929940458);
        let assign18960_e26809: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18960_e26813: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18960_e26814: f64 = (locals.var_vha1 * assign18960_e26813);
        let assign18960_e26817: f64 = (locals.var_nj0 * p.p85);
        let assign18960_e26818: f64 = (assign18960_e26814 / assign18960_e26817);
        let assign18960_e26819: f64 = (assign18960_e26809 + assign18960_e26818);
        let assign18960_e26820: f64 = (locals.var_phitdinv * assign18960_e26819);
        let assign18960_e26821: f64 = (assign18960_e26805 - assign18960_e26820);
        let assign18960_e26823: f64 = (assign18960_e26821 * 0.3333333333333333);
        let assign18960_e26824: f64 = (1.0 + assign18960_e26823);
        let assign18960_e26825: f64 = (assign18960_e26802 * assign18960_e26824);
        let assign18960_e26826: f64 = (0.5 * assign18960_e26825);
        let assign18960_e26827: f64 = (1.0 + assign18960_e26826);
        let assign18960_e26828: f64 = (assign18960_e26782 * assign18960_e26827);
        let assign18960_e26829: f64 = (1.0 + assign18960_e26828);
        let assign18960_e26830: f64 = (1e-100 / assign18960_e26829);
        (assign18960_e26830, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18960_e26778) - (assign18960_e26775 * (locals.var_nj0_dn0 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18960_e26798) - (assign18960_e26795 * (locals.var_nj0_dn0 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18960_e26817) - (assign18960_e26814 * (locals.var_nj0_dn0 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18960_e26778) - (assign18960_e26775 * (locals.var_nj0_dn2 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18960_e26798) - (assign18960_e26795 * (locals.var_nj0_dn2 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18960_e26817) - (assign18960_e26814 * (locals.var_nj0_dn2 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign18960_e26832;
        locals.var_idmultbot_dn0 = assign18960_e26832_d_n0;
        locals.var_idmultbot_dn2 = assign18960_e26832_d_n2;

        let (assign18970_e26910, assign18970_e26910_d_n0, assign18970_e26910_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard313 == 0.0)) {
        let assign18970_e26849: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18970_e26853: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18970_e26854: f64 = (locals.var_vha1 * assign18970_e26853);
        let assign18970_e26857: f64 = (locals.var_nj0 * p.p85);
        let assign18970_e26858: f64 = (assign18970_e26854 / assign18970_e26857);
        let assign18970_e26859: f64 = (assign18970_e26849 + assign18970_e26858);
        let assign18970_e26860: f64 = (locals.var_phitdinv * assign18970_e26859);
        let assign18970_e26862: f64 = (assign18970_e26860 - 230.25850929940458);
        let assign18970_e26868: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18970_e26872: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18970_e26873: f64 = (locals.var_vha1 * assign18970_e26872);
        let assign18970_e26876: f64 = (locals.var_nj0 * p.p85);
        let assign18970_e26877: f64 = (assign18970_e26873 / assign18970_e26876);
        let assign18970_e26878: f64 = (assign18970_e26868 + assign18970_e26877);
        let assign18970_e26879: f64 = (locals.var_phitdinv * assign18970_e26878);
        let assign18970_e26881: f64 = (assign18970_e26879 - 230.25850929940458);
        let assign18970_e26886: f64 = (locals.var_v4 / locals.var_nj1);
        let assign18970_e26890: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign18970_e26891: f64 = (locals.var_vha1 * assign18970_e26890);
        let assign18970_e26894: f64 = (locals.var_nj0 * p.p85);
        let assign18970_e26895: f64 = (assign18970_e26891 / assign18970_e26894);
        let assign18970_e26896: f64 = (assign18970_e26886 + assign18970_e26895);
        let assign18970_e26897: f64 = (locals.var_phitdinv * assign18970_e26896);
        let assign18970_e26899: f64 = (assign18970_e26897 - 230.25850929940458);
        let assign18970_e26901: f64 = (assign18970_e26899 * 0.3333333333333333);
        let assign18970_e26902: f64 = (1.0 + assign18970_e26901);
        let assign18970_e26903: f64 = (assign18970_e26881 * assign18970_e26902);
        let assign18970_e26904: f64 = (0.5 * assign18970_e26903);
        let assign18970_e26905: f64 = (1.0 + assign18970_e26904);
        let assign18970_e26906: f64 = (assign18970_e26862 * assign18970_e26905);
        let assign18970_e26907: f64 = (1.0 + assign18970_e26906);
        let assign18970_e26908: f64 = (1e100 * assign18970_e26907);
        (assign18970_e26908, (1e100 * (((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18970_e26857) - (assign18970_e26854 * (locals.var_nj0_dn0 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18970_e26876) - (assign18970_e26873 * (locals.var_nj0_dn0 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign18970_e26894) - (assign18970_e26891 * (locals.var_nj0_dn0 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18970_e26857) - (assign18970_e26854 * (locals.var_nj0_dn2 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18970_e26876) - (assign18970_e26873 * (locals.var_nj0_dn2 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((locals.var_phitdinv * ((-((locals.var_v4 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign18970_e26894) - (assign18970_e26891 * (locals.var_nj0_dn2 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign18970_e26910;
        locals.var_idmultbot_dn0 = assign18970_e26910_d_n0;
        locals.var_idmultbot_dn2 = assign18970_e26910_d_n2;

        let (assign18980_e26922,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign18980_e26918: f64 = (locals.var_nin * locals.var_nin);
        let assign18980_e26920: f64 = (assign18980_e26918 / locals.var_ndisti_i);
        (assign18980_e26920,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign18980_e26922;

        let (assign18990_e26937,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign18990_e26930: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign18990_e26933: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign18990_e26934: f64 = (assign18990_e26933).ln();
        let assign18990_e26935: f64 = (assign18990_e26930 * assign18990_e26934);
        (assign18990_e26935,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign18990_e26937;

        let assign19000_e26940: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign19000_e26940;

        let (assign19010_e26956, assign19010_e26956_d_n0, assign19010_e26956_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19010_e26951: f64 = (locals.var_v4 - locals.var_vha1);
        let assign19010_e26952: f64 = (p.p86 * assign19010_e26951);
        let assign19010_e26954: f64 = (assign19010_e26952 + locals.var_nfasti_i);
        (assign19010_e26954, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign19010_e26956;
        locals.var_nja10_dn0 = assign19010_e26956_d_n0;
        locals.var_nja10_dn2 = assign19010_e26956_d_n2;

        let (assign19020_e26970, assign19020_e26970_d_n0, assign19020_e26970_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19020_e26967: f64 = (p.p86 * locals.var_vha1);
        let assign19020_e26968: f64 = (locals.var_nfasti_i - assign19020_e26967);
        (assign19020_e26968, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19020_e26970;
        locals.var_nj0_dn0 = assign19020_e26970_d_n0;
        locals.var_nj0_dn2 = assign19020_e26970_d_n2;

        let (assign19030_e26984, assign19030_e26984_d_n0, assign19030_e26984_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19030_e26980: f64 = (p.p85 - locals.var_nja10);
        let assign19030_e26982: f64 = (assign19030_e26980 - 0.01);
        (assign19030_e26982, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19030_e26984;
        locals.var_tmf1_dn0 = assign19030_e26984_d_n0;
        locals.var_tmf1_dn2 = assign19030_e26984_d_n2;

        let (assign19040_e26998, assign19040_e26998_d_n0, assign19040_e26998_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19040_e26994: f64 = (4.0 * p.p85);
        let assign19040_e26996: f64 = (assign19040_e26994 * 0.01);
        (assign19040_e26996, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19040_e26998;
        locals.var_tmf2_dn0 = assign19040_e26998_d_n0;
        locals.var_tmf2_dn2 = assign19040_e26998_d_n2;

        let (assign19050_e27014, assign19050_e27014_d_n0, assign19050_e27014_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19050_e27011: f64 = (-locals.var_tmf2);
                (assign19050_e27011, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19050_e27014;
        locals.var_tmf2_dn0 = assign19050_e27014_d_n0;
        locals.var_tmf2_dn2 = assign19050_e27014_d_n2;

        let (assign19060_e27029, assign19060_e27029_d_n0, assign19060_e27029_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19060_e27024: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19060_e27026: f64 = (assign19060_e27024 + locals.var_tmf2);
        let assign19060_e27027: f64 = (assign19060_e27026).sqrt();
        (assign19060_e27027, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19060_e27027)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19060_e27027)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19060_e27029;
        locals.var_tmf2_dn0 = assign19060_e27029_d_n0;
        locals.var_tmf2_dn2 = assign19060_e27029_d_n2;

        let (assign19070_e27045, assign19070_e27045_d_n0, assign19070_e27045_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19070_e27041: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19070_e27042: f64 = (0.5 * assign19070_e27041);
        let assign19070_e27043: f64 = (p.p85 - assign19070_e27042);
        (assign19070_e27043, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign19070_e27045;
        locals.var_nja11_dn0 = assign19070_e27045_d_n0;
        locals.var_nja11_dn2 = assign19070_e27045_d_n2;

        let (assign19080_e27059, assign19080_e27059_d_n0, assign19080_e27059_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19080_e27055: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign19080_e27057: f64 = (assign19080_e27055 - 0.01);
        (assign19080_e27057, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19080_e27059;
        locals.var_tmf1_dn0 = assign19080_e27059_d_n0;
        locals.var_tmf1_dn2 = assign19080_e27059_d_n2;

        let (assign19090_e27073, assign19090_e27073_d_n0, assign19090_e27073_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19090_e27069: f64 = (4.0 * locals.var_nfasti_i);
        let assign19090_e27071: f64 = (assign19090_e27069 * 0.01);
        (assign19090_e27071, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19090_e27073;
        locals.var_tmf2_dn0 = assign19090_e27073_d_n0;
        locals.var_tmf2_dn2 = assign19090_e27073_d_n2;

        let (assign19100_e27089, assign19100_e27089_d_n0, assign19100_e27089_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19100_e27086: f64 = (-locals.var_tmf2);
                (assign19100_e27086, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19100_e27089;
        locals.var_tmf2_dn0 = assign19100_e27089_d_n0;
        locals.var_tmf2_dn2 = assign19100_e27089_d_n2;

        let (assign19110_e27104, assign19110_e27104_d_n0, assign19110_e27104_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19110_e27099: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19110_e27101: f64 = (assign19110_e27099 + locals.var_tmf2);
        let assign19110_e27102: f64 = (assign19110_e27101).sqrt();
        (assign19110_e27102, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19110_e27102)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19110_e27102)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19110_e27104;
        locals.var_tmf2_dn0 = assign19110_e27104_d_n0;
        locals.var_tmf2_dn2 = assign19110_e27104_d_n2;

        let (assign19120_e27120, assign19120_e27120_d_n0, assign19120_e27120_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19120_e27116: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19120_e27117: f64 = (0.5 * assign19120_e27116);
        let assign19120_e27118: f64 = (locals.var_nfasti_i + assign19120_e27117);
        (assign19120_e27118, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19120_e27120;
        locals.var_nj1_dn0 = assign19120_e27120_d_n0;
        locals.var_nj1_dn2 = assign19120_e27120_d_n2;

        let (assign19130_e27134, assign19130_e27134_d_n0, assign19130_e27134_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19130_e27130: f64 = (p.p85 - locals.var_nj0);
        let assign19130_e27132: f64 = (assign19130_e27130 - 0.01);
        (assign19130_e27132, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19130_e27134;
        locals.var_tmf1_dn0 = assign19130_e27134_d_n0;
        locals.var_tmf1_dn2 = assign19130_e27134_d_n2;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19140_e27148, assign19140_e27148_d_n0, assign19140_e27148_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19140_e27144: f64 = (4.0 * p.p85);
        let assign19140_e27146: f64 = (assign19140_e27144 * 0.01);
        (assign19140_e27146, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19140_e27148;
        locals.var_tmf2_dn0 = assign19140_e27148_d_n0;
        locals.var_tmf2_dn2 = assign19140_e27148_d_n2;

        let (assign19150_e27164, assign19150_e27164_d_n0, assign19150_e27164_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19150_e27161: f64 = (-locals.var_tmf2);
                (assign19150_e27161, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19150_e27164;
        locals.var_tmf2_dn0 = assign19150_e27164_d_n0;
        locals.var_tmf2_dn2 = assign19150_e27164_d_n2;

        let (assign19160_e27179, assign19160_e27179_d_n0, assign19160_e27179_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19160_e27174: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19160_e27176: f64 = (assign19160_e27174 + locals.var_tmf2);
        let assign19160_e27177: f64 = (assign19160_e27176).sqrt();
        (assign19160_e27177, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19160_e27177)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19160_e27177)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19160_e27179;
        locals.var_tmf2_dn0 = assign19160_e27179_d_n0;
        locals.var_tmf2_dn2 = assign19160_e27179_d_n2;

        let (assign19170_e27195, assign19170_e27195_d_n0, assign19170_e27195_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19170_e27191: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19170_e27192: f64 = (0.5 * assign19170_e27191);
        let assign19170_e27193: f64 = (p.p85 - assign19170_e27192);
        (assign19170_e27193, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19170_e27195;
        locals.var_nj0_dn0 = assign19170_e27195_d_n0;
        locals.var_nj0_dn2 = assign19170_e27195_d_n2;

        let (assign19180_e27209, assign19180_e27209_d_n0, assign19180_e27209_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19180_e27205: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign19180_e27207: f64 = (assign19180_e27205 - 0.01);
        (assign19180_e27207, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19180_e27209;
        locals.var_tmf1_dn0 = assign19180_e27209_d_n0;
        locals.var_tmf1_dn2 = assign19180_e27209_d_n2;

        let (assign19190_e27223, assign19190_e27223_d_n0, assign19190_e27223_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19190_e27219: f64 = (4.0 * locals.var_nfasti_i);
        let assign19190_e27221: f64 = (assign19190_e27219 * 0.01);
        (assign19190_e27221, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19190_e27223;
        locals.var_tmf2_dn0 = assign19190_e27223_d_n0;
        locals.var_tmf2_dn2 = assign19190_e27223_d_n2;

        let (assign19200_e27239, assign19200_e27239_d_n0, assign19200_e27239_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19200_e27236: f64 = (-locals.var_tmf2);
                (assign19200_e27236, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19200_e27239;
        locals.var_tmf2_dn0 = assign19200_e27239_d_n0;
        locals.var_tmf2_dn2 = assign19200_e27239_d_n2;

        let (assign19210_e27254, assign19210_e27254_d_n0, assign19210_e27254_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19210_e27249: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19210_e27251: f64 = (assign19210_e27249 + locals.var_tmf2);
        let assign19210_e27252: f64 = (assign19210_e27251).sqrt();
        (assign19210_e27252, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19210_e27252)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19210_e27252)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19210_e27254;
        locals.var_tmf2_dn0 = assign19210_e27254_d_n0;
        locals.var_tmf2_dn2 = assign19210_e27254_d_n2;

        let (assign19220_e27270, assign19220_e27270_d_n0, assign19220_e27270_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign19220_e27266: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19220_e27267: f64 = (0.5 * assign19220_e27266);
        let assign19220_e27268: f64 = (locals.var_nfasti_i + assign19220_e27267);
        (assign19220_e27268, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19220_e27270;
        locals.var_nj0_dn0 = assign19220_e27270_d_n0;
        locals.var_nj0_dn2 = assign19220_e27270_d_n2;

        let (assign19230_e27281, assign19230_e27281_d_n0, assign19230_e27281_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19230_e27281;
        locals.var_nj0_dn0 = assign19230_e27281_d_n0;
        locals.var_nj0_dn2 = assign19230_e27281_d_n2;

        let (assign19240_e27292, assign19240_e27292_d_n0, assign19240_e27292_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard314 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19240_e27292;
        locals.var_nj1_dn0 = assign19240_e27292_d_n0;
        locals.var_nj1_dn2 = assign19240_e27292_d_n2;

        let (assign19300_e27523,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign19300_e27519: f64 = (locals.var_nin * locals.var_nin);
        let assign19300_e27521: f64 = (assign19300_e27519 / locals.var_ndigat_i);
        (assign19300_e27521,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign19300_e27523;

        let (assign19310_e27538,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign19310_e27531: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign19310_e27534: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign19310_e27535: f64 = (assign19310_e27534).ln();
        let assign19310_e27536: f64 = (assign19310_e27531 * assign19310_e27535);
        (assign19310_e27536,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign19310_e27538;

        let assign19320_e27541: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign19320_e27541;

        let (assign19330_e27557, assign19330_e27557_d_n0, assign19330_e27557_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19330_e27552: f64 = (locals.var_v4 - locals.var_vha1);
        let assign19330_e27553: f64 = (p.p86 * assign19330_e27552);
        let assign19330_e27555: f64 = (assign19330_e27553 + locals.var_nfagat_i);
        (assign19330_e27555, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign19330_e27557;
        locals.var_nja10_dn0 = assign19330_e27557_d_n0;
        locals.var_nja10_dn2 = assign19330_e27557_d_n2;

        let (assign19340_e27571, assign19340_e27571_d_n0, assign19340_e27571_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19340_e27568: f64 = (p.p86 * locals.var_vha1);
        let assign19340_e27569: f64 = (locals.var_nfagat_i - assign19340_e27568);
        (assign19340_e27569, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19340_e27571;
        locals.var_nj0_dn0 = assign19340_e27571_d_n0;
        locals.var_nj0_dn2 = assign19340_e27571_d_n2;

        let (assign19350_e27585, assign19350_e27585_d_n0, assign19350_e27585_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19350_e27581: f64 = (p.p85 - locals.var_nja10);
        let assign19350_e27583: f64 = (assign19350_e27581 - 0.01);
        (assign19350_e27583, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19350_e27585;
        locals.var_tmf1_dn0 = assign19350_e27585_d_n0;
        locals.var_tmf1_dn2 = assign19350_e27585_d_n2;

        let (assign19360_e27599, assign19360_e27599_d_n0, assign19360_e27599_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19360_e27595: f64 = (4.0 * p.p85);
        let assign19360_e27597: f64 = (assign19360_e27595 * 0.01);
        (assign19360_e27597, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19360_e27599;
        locals.var_tmf2_dn0 = assign19360_e27599_d_n0;
        locals.var_tmf2_dn2 = assign19360_e27599_d_n2;

        let (assign19370_e27615, assign19370_e27615_d_n0, assign19370_e27615_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19370_e27612: f64 = (-locals.var_tmf2);
                (assign19370_e27612, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19370_e27615;
        locals.var_tmf2_dn0 = assign19370_e27615_d_n0;
        locals.var_tmf2_dn2 = assign19370_e27615_d_n2;

        let (assign19380_e27630, assign19380_e27630_d_n0, assign19380_e27630_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19380_e27625: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19380_e27627: f64 = (assign19380_e27625 + locals.var_tmf2);
        let assign19380_e27628: f64 = (assign19380_e27627).sqrt();
        (assign19380_e27628, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19380_e27628)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19380_e27628)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19380_e27630;
        locals.var_tmf2_dn0 = assign19380_e27630_d_n0;
        locals.var_tmf2_dn2 = assign19380_e27630_d_n2;

        let (assign19390_e27646, assign19390_e27646_d_n0, assign19390_e27646_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19390_e27642: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19390_e27643: f64 = (0.5 * assign19390_e27642);
        let assign19390_e27644: f64 = (p.p85 - assign19390_e27643);
        (assign19390_e27644, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign19390_e27646;
        locals.var_nja11_dn0 = assign19390_e27646_d_n0;
        locals.var_nja11_dn2 = assign19390_e27646_d_n2;

        let (assign19400_e27660, assign19400_e27660_d_n0, assign19400_e27660_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19400_e27656: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign19400_e27658: f64 = (assign19400_e27656 - 0.01);
        (assign19400_e27658, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19400_e27660;
        locals.var_tmf1_dn0 = assign19400_e27660_d_n0;
        locals.var_tmf1_dn2 = assign19400_e27660_d_n2;

        let (assign19410_e27674, assign19410_e27674_d_n0, assign19410_e27674_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19410_e27670: f64 = (4.0 * locals.var_nfagat_i);
        let assign19410_e27672: f64 = (assign19410_e27670 * 0.01);
        (assign19410_e27672, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19410_e27674;
        locals.var_tmf2_dn0 = assign19410_e27674_d_n0;
        locals.var_tmf2_dn2 = assign19410_e27674_d_n2;

        let (assign19420_e27690, assign19420_e27690_d_n0, assign19420_e27690_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19420_e27687: f64 = (-locals.var_tmf2);
                (assign19420_e27687, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19420_e27690;
        locals.var_tmf2_dn0 = assign19420_e27690_d_n0;
        locals.var_tmf2_dn2 = assign19420_e27690_d_n2;

        let (assign19430_e27705, assign19430_e27705_d_n0, assign19430_e27705_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19430_e27700: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19430_e27702: f64 = (assign19430_e27700 + locals.var_tmf2);
        let assign19430_e27703: f64 = (assign19430_e27702).sqrt();
        (assign19430_e27703, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19430_e27703)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19430_e27703)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19430_e27705;
        locals.var_tmf2_dn0 = assign19430_e27705_d_n0;
        locals.var_tmf2_dn2 = assign19430_e27705_d_n2;

        let (assign19440_e27721, assign19440_e27721_d_n0, assign19440_e27721_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19440_e27717: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19440_e27718: f64 = (0.5 * assign19440_e27717);
        let assign19440_e27719: f64 = (locals.var_nfagat_i + assign19440_e27718);
        (assign19440_e27719, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19440_e27721;
        locals.var_nj1_dn0 = assign19440_e27721_d_n0;
        locals.var_nj1_dn2 = assign19440_e27721_d_n2;

        let (assign19450_e27735, assign19450_e27735_d_n0, assign19450_e27735_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19450_e27731: f64 = (p.p85 - locals.var_nj0);
        let assign19450_e27733: f64 = (assign19450_e27731 - 0.01);
        (assign19450_e27733, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19450_e27735;
        locals.var_tmf1_dn0 = assign19450_e27735_d_n0;
        locals.var_tmf1_dn2 = assign19450_e27735_d_n2;

        let (assign19460_e27749, assign19460_e27749_d_n0, assign19460_e27749_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19460_e27745: f64 = (4.0 * p.p85);
        let assign19460_e27747: f64 = (assign19460_e27745 * 0.01);
        (assign19460_e27747, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19460_e27749;
        locals.var_tmf2_dn0 = assign19460_e27749_d_n0;
        locals.var_tmf2_dn2 = assign19460_e27749_d_n2;

        let (assign19470_e27765, assign19470_e27765_d_n0, assign19470_e27765_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19470_e27762: f64 = (-locals.var_tmf2);
                (assign19470_e27762, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19470_e27765;
        locals.var_tmf2_dn0 = assign19470_e27765_d_n0;
        locals.var_tmf2_dn2 = assign19470_e27765_d_n2;

        let (assign19480_e27780, assign19480_e27780_d_n0, assign19480_e27780_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19480_e27775: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19480_e27777: f64 = (assign19480_e27775 + locals.var_tmf2);
        let assign19480_e27778: f64 = (assign19480_e27777).sqrt();
        (assign19480_e27778, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19480_e27778)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19480_e27778)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19480_e27780;
        locals.var_tmf2_dn0 = assign19480_e27780_d_n0;
        locals.var_tmf2_dn2 = assign19480_e27780_d_n2;

        let (assign19490_e27796, assign19490_e27796_d_n0, assign19490_e27796_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19490_e27792: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19490_e27793: f64 = (0.5 * assign19490_e27792);
        let assign19490_e27794: f64 = (p.p85 - assign19490_e27793);
        (assign19490_e27794, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19490_e27796;
        locals.var_nj0_dn0 = assign19490_e27796_d_n0;
        locals.var_nj0_dn2 = assign19490_e27796_d_n2;

        let (assign19500_e27810, assign19500_e27810_d_n0, assign19500_e27810_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19500_e27806: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign19500_e27808: f64 = (assign19500_e27806 - 0.01);
        (assign19500_e27808, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19500_e27810;
        locals.var_tmf1_dn0 = assign19500_e27810_d_n0;
        locals.var_tmf1_dn2 = assign19500_e27810_d_n2;

        let (assign19510_e27824, assign19510_e27824_d_n0, assign19510_e27824_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19510_e27820: f64 = (4.0 * locals.var_nfagat_i);
        let assign19510_e27822: f64 = (assign19510_e27820 * 0.01);
        (assign19510_e27822, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19510_e27824;
        locals.var_tmf2_dn0 = assign19510_e27824_d_n0;
        locals.var_tmf2_dn2 = assign19510_e27824_d_n2;

        let (assign19520_e27840, assign19520_e27840_d_n0, assign19520_e27840_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19520_e27837: f64 = (-locals.var_tmf2);
                (assign19520_e27837, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19520_e27840;
        locals.var_tmf2_dn0 = assign19520_e27840_d_n0;
        locals.var_tmf2_dn2 = assign19520_e27840_d_n2;

        let (assign19530_e27855, assign19530_e27855_d_n0, assign19530_e27855_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19530_e27850: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19530_e27852: f64 = (assign19530_e27850 + locals.var_tmf2);
        let assign19530_e27853: f64 = (assign19530_e27852).sqrt();
        (assign19530_e27853, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19530_e27853)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19530_e27853)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19530_e27855;
        locals.var_tmf2_dn0 = assign19530_e27855_d_n0;
        locals.var_tmf2_dn2 = assign19530_e27855_d_n2;

        let (assign19540_e27871, assign19540_e27871_d_n0, assign19540_e27871_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign19540_e27867: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19540_e27868: f64 = (0.5 * assign19540_e27867);
        let assign19540_e27869: f64 = (locals.var_nfagat_i + assign19540_e27868);
        (assign19540_e27869, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19540_e27871;
        locals.var_nj0_dn0 = assign19540_e27871_d_n0;
        locals.var_nj0_dn2 = assign19540_e27871_d_n2;

        let (assign19550_e27882, assign19550_e27882_d_n0, assign19550_e27882_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19550_e27882;
        locals.var_nj0_dn0 = assign19550_e27882_d_n0;
        locals.var_nj0_dn2 = assign19550_e27882_d_n2;

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19560_e27893, assign19560_e27893_d_n0, assign19560_e27893_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) && (locals.var_guard317 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19560_e27893;
        locals.var_nj1_dn0 = assign19560_e27893_d_n0;
        locals.var_nj1_dn2 = assign19560_e27893_d_n2;

        let (assign19630_e28143,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign19630_e28139: f64 = (locals.var_nin * locals.var_nin);
        let assign19630_e28141: f64 = (assign19630_e28139 / locals.var_ndibot_i);
        (assign19630_e28141,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign19630_e28143;

        let (assign19640_e28159,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign19640_e28152: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign19640_e28155: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign19640_e28156: f64 = (assign19640_e28155).ln();
        let assign19640_e28157: f64 = (assign19640_e28152 * assign19640_e28156);
        (assign19640_e28157,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign19640_e28159;

        let assign19650_e28162: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign19650_e28162;

        let (assign19660_e28179, assign19660_e28179_d_n0, assign19660_e28179_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19660_e28174: f64 = (locals.var_vmax - locals.var_vha1);
        let assign19660_e28175: f64 = (p.p86 * assign19660_e28174);
        let assign19660_e28177: f64 = (assign19660_e28175 + locals.var_nfabot_i);
        (assign19660_e28177, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign19660_e28179;
        locals.var_nja10_dn0 = assign19660_e28179_d_n0;
        locals.var_nja10_dn2 = assign19660_e28179_d_n2;

        let (assign19670_e28194, assign19670_e28194_d_n0, assign19670_e28194_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19670_e28191: f64 = (p.p86 * locals.var_vha1);
        let assign19670_e28192: f64 = (locals.var_nfabot_i - assign19670_e28191);
        (assign19670_e28192, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19670_e28194;
        locals.var_nj0_dn0 = assign19670_e28194_d_n0;
        locals.var_nj0_dn2 = assign19670_e28194_d_n2;

        let (assign19680_e28209, assign19680_e28209_d_n0, assign19680_e28209_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19680_e28205: f64 = (p.p85 - locals.var_nja10);
        let assign19680_e28207: f64 = (assign19680_e28205 - 0.01);
        (assign19680_e28207, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19680_e28209;
        locals.var_tmf1_dn0 = assign19680_e28209_d_n0;
        locals.var_tmf1_dn2 = assign19680_e28209_d_n2;

        let (assign19690_e28224, assign19690_e28224_d_n0, assign19690_e28224_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19690_e28220: f64 = (4.0 * p.p85);
        let assign19690_e28222: f64 = (assign19690_e28220 * 0.01);
        (assign19690_e28222, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19690_e28224;
        locals.var_tmf2_dn0 = assign19690_e28224_d_n0;
        locals.var_tmf2_dn2 = assign19690_e28224_d_n2;

        let (assign19700_e28241, assign19700_e28241_d_n0, assign19700_e28241_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19700_e28238: f64 = (-locals.var_tmf2);
                (assign19700_e28238, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19700_e28241;
        locals.var_tmf2_dn0 = assign19700_e28241_d_n0;
        locals.var_tmf2_dn2 = assign19700_e28241_d_n2;

        let (assign19710_e28257, assign19710_e28257_d_n0, assign19710_e28257_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19710_e28252: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19710_e28254: f64 = (assign19710_e28252 + locals.var_tmf2);
        let assign19710_e28255: f64 = (assign19710_e28254).sqrt();
        (assign19710_e28255, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19710_e28255)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19710_e28255)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19710_e28257;
        locals.var_tmf2_dn0 = assign19710_e28257_d_n0;
        locals.var_tmf2_dn2 = assign19710_e28257_d_n2;

        let (assign19720_e28274, assign19720_e28274_d_n0, assign19720_e28274_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19720_e28270: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19720_e28271: f64 = (1.0 + assign19720_e28270);
        let assign19720_e28272: f64 = (0.5 * assign19720_e28271);
        (assign19720_e28272, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign19720_e28274;
        locals.var_dfn_su_dn0 = assign19720_e28274_d_n0;
        locals.var_dfn_su_dn2 = assign19720_e28274_d_n2;

        let (assign19730_e28291, assign19730_e28291_d_n0, assign19730_e28291_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19730_e28287: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19730_e28288: f64 = (0.5 * assign19730_e28287);
        let assign19730_e28289: f64 = (p.p85 - assign19730_e28288);
        (assign19730_e28289, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign19730_e28291;
        locals.var_nja11_dn0 = assign19730_e28291_d_n0;
        locals.var_nja11_dn2 = assign19730_e28291_d_n2;

        let (assign19740_e28306, assign19740_e28306_d_n0, assign19740_e28306_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19740_e28302: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign19740_e28304: f64 = (assign19740_e28302 - 0.01);
        (assign19740_e28304, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19740_e28306;
        locals.var_tmf1_dn0 = assign19740_e28306_d_n0;
        locals.var_tmf1_dn2 = assign19740_e28306_d_n2;

        let (assign19750_e28321, assign19750_e28321_d_n0, assign19750_e28321_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19750_e28317: f64 = (4.0 * locals.var_nfabot_i);
        let assign19750_e28319: f64 = (assign19750_e28317 * 0.01);
        (assign19750_e28319, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19750_e28321;
        locals.var_tmf2_dn0 = assign19750_e28321_d_n0;
        locals.var_tmf2_dn2 = assign19750_e28321_d_n2;

        let (assign19760_e28338, assign19760_e28338_d_n0, assign19760_e28338_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19760_e28335: f64 = (-locals.var_tmf2);
                (assign19760_e28335, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19760_e28338;
        locals.var_tmf2_dn0 = assign19760_e28338_d_n0;
        locals.var_tmf2_dn2 = assign19760_e28338_d_n2;

        let (assign19770_e28354, assign19770_e28354_d_n0, assign19770_e28354_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19770_e28349: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19770_e28351: f64 = (assign19770_e28349 + locals.var_tmf2);
        let assign19770_e28352: f64 = (assign19770_e28351).sqrt();
        (assign19770_e28352, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19770_e28352)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19770_e28352)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19770_e28354;
        locals.var_tmf2_dn0 = assign19770_e28354_d_n0;
        locals.var_tmf2_dn2 = assign19770_e28354_d_n2;

        let (assign19780_e28371, assign19780_e28371_d_n0, assign19780_e28371_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19780_e28367: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19780_e28368: f64 = (1.0 + assign19780_e28367);
        let assign19780_e28369: f64 = (0.5 * assign19780_e28368);
        (assign19780_e28369, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign19780_e28371;
        locals.var_dfn_sl_dn0 = assign19780_e28371_d_n0;
        locals.var_dfn_sl_dn2 = assign19780_e28371_d_n2;

        let (assign19790_e28388, assign19790_e28388_d_n0, assign19790_e28388_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19790_e28384: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19790_e28385: f64 = (0.5 * assign19790_e28384);
        let assign19790_e28386: f64 = (locals.var_nfabot_i + assign19790_e28385);
        (assign19790_e28386, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19790_e28388;
        locals.var_nj1_dn0 = assign19790_e28388_d_n0;
        locals.var_nj1_dn2 = assign19790_e28388_d_n2;

        let (assign19800_e28403, assign19800_e28403_d_n0, assign19800_e28403_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19800_e28399: f64 = (p.p85 - locals.var_nj0);
        let assign19800_e28401: f64 = (assign19800_e28399 - 0.01);
        (assign19800_e28401, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19800_e28403;
        locals.var_tmf1_dn0 = assign19800_e28403_d_n0;
        locals.var_tmf1_dn2 = assign19800_e28403_d_n2;

        let (assign19810_e28418, assign19810_e28418_d_n0, assign19810_e28418_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19810_e28414: f64 = (4.0 * p.p85);
        let assign19810_e28416: f64 = (assign19810_e28414 * 0.01);
        (assign19810_e28416, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19810_e28418;
        locals.var_tmf2_dn0 = assign19810_e28418_d_n0;
        locals.var_tmf2_dn2 = assign19810_e28418_d_n2;

        let (assign19820_e28435, assign19820_e28435_d_n0, assign19820_e28435_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19820_e28432: f64 = (-locals.var_tmf2);
                (assign19820_e28432, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19820_e28435;
        locals.var_tmf2_dn0 = assign19820_e28435_d_n0;
        locals.var_tmf2_dn2 = assign19820_e28435_d_n2;

        let (assign19830_e28451, assign19830_e28451_d_n0, assign19830_e28451_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19830_e28446: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19830_e28448: f64 = (assign19830_e28446 + locals.var_tmf2);
        let assign19830_e28449: f64 = (assign19830_e28448).sqrt();
        (assign19830_e28449, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19830_e28449)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19830_e28449)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19830_e28451;
        locals.var_tmf2_dn0 = assign19830_e28451_d_n0;
        locals.var_tmf2_dn2 = assign19830_e28451_d_n2;

        let (assign19840_e28468, assign19840_e28468_d_n0, assign19840_e28468_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19840_e28464: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19840_e28465: f64 = (0.5 * assign19840_e28464);
        let assign19840_e28466: f64 = (p.p85 - assign19840_e28465);
        (assign19840_e28466, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19840_e28468;
        locals.var_nj0_dn0 = assign19840_e28468_d_n0;
        locals.var_nj0_dn2 = assign19840_e28468_d_n2;

        let (assign19850_e28483, assign19850_e28483_d_n0, assign19850_e28483_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19850_e28479: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign19850_e28481: f64 = (assign19850_e28479 - 0.01);
        (assign19850_e28481, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign19850_e28483;
        locals.var_tmf1_dn0 = assign19850_e28483_d_n0;
        locals.var_tmf1_dn2 = assign19850_e28483_d_n2;

        let (assign19860_e28498, assign19860_e28498_d_n0, assign19860_e28498_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19860_e28494: f64 = (4.0 * locals.var_nfabot_i);
        let assign19860_e28496: f64 = (assign19860_e28494 * 0.01);
        (assign19860_e28496, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19860_e28498;
        locals.var_tmf2_dn0 = assign19860_e28498_d_n0;
        locals.var_tmf2_dn2 = assign19860_e28498_d_n2;

        let (assign19870_e28515, assign19870_e28515_d_n0, assign19870_e28515_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign19870_e28512: f64 = (-locals.var_tmf2);
                (assign19870_e28512, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19870_e28515;
        locals.var_tmf2_dn0 = assign19870_e28515_d_n0;
        locals.var_tmf2_dn2 = assign19870_e28515_d_n2;

        let (assign19880_e28531, assign19880_e28531_d_n0, assign19880_e28531_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19880_e28526: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19880_e28528: f64 = (assign19880_e28526 + locals.var_tmf2);
        let assign19880_e28529: f64 = (assign19880_e28528).sqrt();
        (assign19880_e28529, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19880_e28529)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19880_e28529)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign19880_e28531;
        locals.var_tmf2_dn0 = assign19880_e28531_d_n0;
        locals.var_tmf2_dn2 = assign19880_e28531_d_n2;

        let (assign19890_e28548, assign19890_e28548_d_n0, assign19890_e28548_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19890_e28544: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19890_e28545: f64 = (0.5 * assign19890_e28544);
        let assign19890_e28546: f64 = (locals.var_nfabot_i + assign19890_e28545);
        (assign19890_e28546, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19890_e28548;
        locals.var_nj0_dn0 = assign19890_e28548_d_n0;
        locals.var_nj0_dn2 = assign19890_e28548_d_n2;

        let (assign19900_e28563, assign19900_e28563_d_n0, assign19900_e28563_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign19900_e28559: f64 = (p.p86 * locals.var_dfn_su);
        let assign19900_e28561: f64 = (assign19900_e28559 * locals.var_dfn_sl);
        (assign19900_e28561, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign19900_e28559 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign19900_e28559 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign19900_e28563;
        locals.var_dnj1_dv_dn0 = assign19900_e28563_d_n0;
        locals.var_dnj1_dv_dn2 = assign19900_e28563_d_n2;

        let (assign19910_e28575, assign19910_e28575_d_n0, assign19910_e28575_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign19910_e28575;
        locals.var_nj0_dn0 = assign19910_e28575_d_n0;
        locals.var_nj0_dn2 = assign19910_e28575_d_n2;

        let (assign19920_e28587, assign19920_e28587_d_n0, assign19920_e28587_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign19920_e28587;
        locals.var_nj1_dn0 = assign19920_e28587_d_n0;
        locals.var_nj1_dn2 = assign19920_e28587_d_n2;

        let (assign19930_e28599, assign19930_e28599_d_n0, assign19930_e28599_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign19930_e28599;
        locals.var_dnj1_dv_dn0 = assign19930_e28599_d_n0;
        locals.var_dnj1_dv_dn2 = assign19930_e28599_d_n2;

        let assign19940_e28603: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19940_e28607: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19940_e28608: f64 = (locals.var_vha1 * assign19940_e28607);
        let assign19940_e28611: f64 = (locals.var_nj0 * p.p85);
        let assign19940_e28612: f64 = (assign19940_e28608 / assign19940_e28611);
        let assign19940_e28613: f64 = (assign19940_e28603 + assign19940_e28612);
        let assign19940_e28614: f64 = (locals.var_phitdinv * assign19940_e28613);
        let assign19940_e28615: f64 = (assign19940_e28614).abs();
        let assign19940_e28617: f64 = if assign19940_e28615 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign19940_e28617;

        let (assign19950_e28643, assign19950_e28643_d_n0, assign19950_e28643_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign19950_e28629: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19950_e28633: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19950_e28634: f64 = (locals.var_vha1 * assign19950_e28633);
        let assign19950_e28637: f64 = (locals.var_nj0 * p.p85);
        let assign19950_e28638: f64 = (assign19950_e28634 / assign19950_e28637);
        let assign19950_e28639: f64 = (assign19950_e28629 + assign19950_e28638);
        let assign19950_e28640: f64 = (locals.var_phitdinv * assign19950_e28639);
        let assign19950_e28641: f64 = (assign19950_e28640).exp();
        (assign19950_e28641, (assign19950_e28641 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19950_e28637) - (assign19950_e28634 * (locals.var_nj0_dn0 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19950_e28637) - (assign19950_e28634 * (locals.var_nj0_dn2 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign19950_e28643;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign19950_e28643_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign19950_e28643_d_n2;

        let assign19960_e28647: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19960_e28651: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19960_e28652: f64 = (locals.var_vha1 * assign19960_e28651);
        let assign19960_e28655: f64 = (locals.var_nj0 * p.p85);
        let assign19960_e28656: f64 = (assign19960_e28652 / assign19960_e28655);
        let assign19960_e28657: f64 = (assign19960_e28647 + assign19960_e28656);
        let assign19960_e28658: f64 = (locals.var_phitdinv * assign19960_e28657);
        let assign19960_e28660: f64 = (-230.25850929940458);
        let assign19960_e28661: f64 = if assign19960_e28658 < assign19960_e28660 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign19960_e28661;

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19970_e28742, assign19970_e28742_d_n0, assign19970_e28742_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign19970_e28676: f64 = (-230.25850929940458);
        let assign19970_e28680: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19970_e28684: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19970_e28685: f64 = (locals.var_vha1 * assign19970_e28684);
        let assign19970_e28688: f64 = (locals.var_nj0 * p.p85);
        let assign19970_e28689: f64 = (assign19970_e28685 / assign19970_e28688);
        let assign19970_e28690: f64 = (assign19970_e28680 + assign19970_e28689);
        let assign19970_e28691: f64 = (locals.var_phitdinv * assign19970_e28690);
        let assign19970_e28692: f64 = (assign19970_e28676 - assign19970_e28691);
        let assign19970_e28696: f64 = (-230.25850929940458);
        let assign19970_e28700: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19970_e28704: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19970_e28705: f64 = (locals.var_vha1 * assign19970_e28704);
        let assign19970_e28708: f64 = (locals.var_nj0 * p.p85);
        let assign19970_e28709: f64 = (assign19970_e28705 / assign19970_e28708);
        let assign19970_e28710: f64 = (assign19970_e28700 + assign19970_e28709);
        let assign19970_e28711: f64 = (locals.var_phitdinv * assign19970_e28710);
        let assign19970_e28712: f64 = (assign19970_e28696 - assign19970_e28711);
        let assign19970_e28715: f64 = (-230.25850929940458);
        let assign19970_e28719: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19970_e28723: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19970_e28724: f64 = (locals.var_vha1 * assign19970_e28723);
        let assign19970_e28727: f64 = (locals.var_nj0 * p.p85);
        let assign19970_e28728: f64 = (assign19970_e28724 / assign19970_e28727);
        let assign19970_e28729: f64 = (assign19970_e28719 + assign19970_e28728);
        let assign19970_e28730: f64 = (locals.var_phitdinv * assign19970_e28729);
        let assign19970_e28731: f64 = (assign19970_e28715 - assign19970_e28730);
        let assign19970_e28733: f64 = (assign19970_e28731 * 0.3333333333333333);
        let assign19970_e28734: f64 = (1.0 + assign19970_e28733);
        let assign19970_e28735: f64 = (assign19970_e28712 * assign19970_e28734);
        let assign19970_e28736: f64 = (0.5 * assign19970_e28735);
        let assign19970_e28737: f64 = (1.0 + assign19970_e28736);
        let assign19970_e28738: f64 = (assign19970_e28692 * assign19970_e28737);
        let assign19970_e28739: f64 = (1.0 + assign19970_e28738);
        let assign19970_e28740: f64 = (1e-100 / assign19970_e28739);
        (assign19970_e28740, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19970_e28688) - (assign19970_e28685 * (locals.var_nj0_dn0 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19970_e28708) - (assign19970_e28705 * (locals.var_nj0_dn0 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19970_e28727) - (assign19970_e28724 * (locals.var_nj0_dn0 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19970_e28688) - (assign19970_e28685 * (locals.var_nj0_dn2 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19970_e28708) - (assign19970_e28705 * (locals.var_nj0_dn2 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19970_e28727) - (assign19970_e28724 * (locals.var_nj0_dn2 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign19970_e28742;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign19970_e28742_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign19970_e28742_d_n2;

        let (assign19980_e28821, assign19980_e28821_d_n0, assign19980_e28821_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign19980_e28760: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19980_e28764: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19980_e28765: f64 = (locals.var_vha1 * assign19980_e28764);
        let assign19980_e28768: f64 = (locals.var_nj0 * p.p85);
        let assign19980_e28769: f64 = (assign19980_e28765 / assign19980_e28768);
        let assign19980_e28770: f64 = (assign19980_e28760 + assign19980_e28769);
        let assign19980_e28771: f64 = (locals.var_phitdinv * assign19980_e28770);
        let assign19980_e28773: f64 = (assign19980_e28771 - 230.25850929940458);
        let assign19980_e28779: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19980_e28783: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19980_e28784: f64 = (locals.var_vha1 * assign19980_e28783);
        let assign19980_e28787: f64 = (locals.var_nj0 * p.p85);
        let assign19980_e28788: f64 = (assign19980_e28784 / assign19980_e28787);
        let assign19980_e28789: f64 = (assign19980_e28779 + assign19980_e28788);
        let assign19980_e28790: f64 = (locals.var_phitdinv * assign19980_e28789);
        let assign19980_e28792: f64 = (assign19980_e28790 - 230.25850929940458);
        let assign19980_e28797: f64 = (locals.var_vmax / locals.var_nj1);
        let assign19980_e28801: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign19980_e28802: f64 = (locals.var_vha1 * assign19980_e28801);
        let assign19980_e28805: f64 = (locals.var_nj0 * p.p85);
        let assign19980_e28806: f64 = (assign19980_e28802 / assign19980_e28805);
        let assign19980_e28807: f64 = (assign19980_e28797 + assign19980_e28806);
        let assign19980_e28808: f64 = (locals.var_phitdinv * assign19980_e28807);
        let assign19980_e28810: f64 = (assign19980_e28808 - 230.25850929940458);
        let assign19980_e28812: f64 = (assign19980_e28810 * 0.3333333333333333);
        let assign19980_e28813: f64 = (1.0 + assign19980_e28812);
        let assign19980_e28814: f64 = (assign19980_e28792 * assign19980_e28813);
        let assign19980_e28815: f64 = (0.5 * assign19980_e28814);
        let assign19980_e28816: f64 = (1.0 + assign19980_e28815);
        let assign19980_e28817: f64 = (assign19980_e28773 * assign19980_e28816);
        let assign19980_e28818: f64 = (1.0 + assign19980_e28817);
        let assign19980_e28819: f64 = (1e100 * assign19980_e28818);
        (assign19980_e28819, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19980_e28768) - (assign19980_e28765 * (locals.var_nj0_dn0 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19980_e28787) - (assign19980_e28784 * (locals.var_nj0_dn0 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign19980_e28805) - (assign19980_e28802 * (locals.var_nj0_dn0 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19980_e28768) - (assign19980_e28765 * (locals.var_nj0_dn2 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19980_e28787) - (assign19980_e28784 * (locals.var_nj0_dn2 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign19980_e28805) - (assign19980_e28802 * (locals.var_nj0_dn2 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign19980_e28821;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign19980_e28821_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign19980_e28821_d_n2;

        let (assign19990_e28848, assign19990_e28848_d_n0, assign19990_e28848_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign19990_e28832: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign19990_e28833: f64 = (locals.var_nj1 - assign19990_e28832);
        let assign19990_e28836: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign19990_e28837: f64 = (assign19990_e28833 / assign19990_e28836);
        let assign19990_e28840: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign19990_e28843: f64 = (locals.var_nj0 * p.p85);
        let assign19990_e28844: f64 = (assign19990_e28840 / assign19990_e28843);
        let assign19990_e28845: f64 = (assign19990_e28837 + assign19990_e28844);
        let assign19990_e28846: f64 = (locals.var_phitdinv * assign19990_e28845);
        (assign19990_e28846, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign19990_e28836) - (assign19990_e28833 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign19990_e28836 * assign19990_e28836)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign19990_e28843) - (assign19990_e28840 * (locals.var_nj0_dn0 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign19990_e28836) - (assign19990_e28833 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign19990_e28836 * assign19990_e28836)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign19990_e28843) - (assign19990_e28840 * (locals.var_nj0_dn2 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign19990_e28848;
        locals.var_dvmax_over_phitd_dv_dn0 = assign19990_e28848_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign19990_e28848_d_n2;

        let (assign20000_e28865, assign20000_e28865_d_n0, assign20000_e28865_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20000_e28858: f64 = (locals.var_v4 - locals.var_vmax);
        let assign20000_e28860: f64 = (assign20000_e28858 * locals.var_dvmax_over_phitd_dv);
        let assign20000_e28861: f64 = (1.0 + assign20000_e28860);
        let assign20000_e28863: f64 = (assign20000_e28861 * locals.var_exp_vmax_over_phitd_bot);
        (assign20000_e28863, (((assign20000_e28858 * locals.var_dvmax_over_phitd_dv_dn0) * locals.var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * locals.var_exp_vmax_over_phitd_bot_dn0)), (((assign20000_e28858 * locals.var_dvmax_over_phitd_dv_dn2) * locals.var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * locals.var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign20000_e28865;
        locals.var_idmultbot_dn0 = assign20000_e28865_d_n0;
        locals.var_idmultbot_dn2 = assign20000_e28865_d_n2;

        let (assign20010_e28878,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20010_e28874: f64 = (locals.var_nin * locals.var_nin);
        let assign20010_e28876: f64 = (assign20010_e28874 / locals.var_ndisti_i);
        (assign20010_e28876,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign20010_e28878;

        let (assign20020_e28894,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20020_e28887: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign20020_e28890: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign20020_e28891: f64 = (assign20020_e28890).ln();
        let assign20020_e28892: f64 = (assign20020_e28887 * assign20020_e28891);
        (assign20020_e28892,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign20020_e28894;

        let assign20030_e28897: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign20030_e28897;

        let (assign20040_e28914, assign20040_e28914_d_n0, assign20040_e28914_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20040_e28909: f64 = (locals.var_vmax - locals.var_vha1);
        let assign20040_e28910: f64 = (p.p86 * assign20040_e28909);
        let assign20040_e28912: f64 = (assign20040_e28910 + locals.var_nfasti_i);
        (assign20040_e28912, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign20040_e28914;
        locals.var_nja10_dn0 = assign20040_e28914_d_n0;
        locals.var_nja10_dn2 = assign20040_e28914_d_n2;

        let (assign20050_e28929, assign20050_e28929_d_n0, assign20050_e28929_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20050_e28926: f64 = (p.p86 * locals.var_vha1);
        let assign20050_e28927: f64 = (locals.var_nfasti_i - assign20050_e28926);
        (assign20050_e28927, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20050_e28929;
        locals.var_nj0_dn0 = assign20050_e28929_d_n0;
        locals.var_nj0_dn2 = assign20050_e28929_d_n2;

        let (assign20060_e28944, assign20060_e28944_d_n0, assign20060_e28944_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20060_e28940: f64 = (p.p85 - locals.var_nja10);
        let assign20060_e28942: f64 = (assign20060_e28940 - 0.01);
        (assign20060_e28942, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20060_e28944;
        locals.var_tmf1_dn0 = assign20060_e28944_d_n0;
        locals.var_tmf1_dn2 = assign20060_e28944_d_n2;

        let (assign20070_e28959, assign20070_e28959_d_n0, assign20070_e28959_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20070_e28955: f64 = (4.0 * p.p85);
        let assign20070_e28957: f64 = (assign20070_e28955 * 0.01);
        (assign20070_e28957, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20070_e28959;
        locals.var_tmf2_dn0 = assign20070_e28959_d_n0;
        locals.var_tmf2_dn2 = assign20070_e28959_d_n2;

        let (assign20080_e28976, assign20080_e28976_d_n0, assign20080_e28976_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20080_e28973: f64 = (-locals.var_tmf2);
                (assign20080_e28973, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20080_e28976;
        locals.var_tmf2_dn0 = assign20080_e28976_d_n0;
        locals.var_tmf2_dn2 = assign20080_e28976_d_n2;

        let (assign20090_e28992, assign20090_e28992_d_n0, assign20090_e28992_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20090_e28987: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20090_e28989: f64 = (assign20090_e28987 + locals.var_tmf2);
        let assign20090_e28990: f64 = (assign20090_e28989).sqrt();
        (assign20090_e28990, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20090_e28990)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20090_e28990)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20090_e28992;
        locals.var_tmf2_dn0 = assign20090_e28992_d_n0;
        locals.var_tmf2_dn2 = assign20090_e28992_d_n2;

        let (assign20100_e29009, assign20100_e29009_d_n0, assign20100_e29009_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20100_e29005: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20100_e29006: f64 = (1.0 + assign20100_e29005);
        let assign20100_e29007: f64 = (0.5 * assign20100_e29006);
        (assign20100_e29007, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign20100_e29009;
        locals.var_dfn_su_dn0 = assign20100_e29009_d_n0;
        locals.var_dfn_su_dn2 = assign20100_e29009_d_n2;

        let (assign20110_e29026, assign20110_e29026_d_n0, assign20110_e29026_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20110_e29022: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20110_e29023: f64 = (0.5 * assign20110_e29022);
        let assign20110_e29024: f64 = (p.p85 - assign20110_e29023);
        (assign20110_e29024, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign20110_e29026;
        locals.var_nja11_dn0 = assign20110_e29026_d_n0;
        locals.var_nja11_dn2 = assign20110_e29026_d_n2;

        let (assign20120_e29041, assign20120_e29041_d_n0, assign20120_e29041_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20120_e29037: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign20120_e29039: f64 = (assign20120_e29037 - 0.01);
        (assign20120_e29039, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20120_e29041;
        locals.var_tmf1_dn0 = assign20120_e29041_d_n0;
        locals.var_tmf1_dn2 = assign20120_e29041_d_n2;

        let (assign20130_e29056, assign20130_e29056_d_n0, assign20130_e29056_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20130_e29052: f64 = (4.0 * locals.var_nfasti_i);
        let assign20130_e29054: f64 = (assign20130_e29052 * 0.01);
        (assign20130_e29054, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20130_e29056;
        locals.var_tmf2_dn0 = assign20130_e29056_d_n0;
        locals.var_tmf2_dn2 = assign20130_e29056_d_n2;

        let (assign20140_e29073, assign20140_e29073_d_n0, assign20140_e29073_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20140_e29070: f64 = (-locals.var_tmf2);
                (assign20140_e29070, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20140_e29073;
        locals.var_tmf2_dn0 = assign20140_e29073_d_n0;
        locals.var_tmf2_dn2 = assign20140_e29073_d_n2;

        let (assign20150_e29089, assign20150_e29089_d_n0, assign20150_e29089_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20150_e29084: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20150_e29086: f64 = (assign20150_e29084 + locals.var_tmf2);
        let assign20150_e29087: f64 = (assign20150_e29086).sqrt();
        (assign20150_e29087, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20150_e29087)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20150_e29087)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20150_e29089;
        locals.var_tmf2_dn0 = assign20150_e29089_d_n0;
        locals.var_tmf2_dn2 = assign20150_e29089_d_n2;

        let (assign20160_e29106, assign20160_e29106_d_n0, assign20160_e29106_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20160_e29102: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20160_e29103: f64 = (1.0 + assign20160_e29102);
        let assign20160_e29104: f64 = (0.5 * assign20160_e29103);
        (assign20160_e29104, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign20160_e29106;
        locals.var_dfn_sl_dn0 = assign20160_e29106_d_n0;
        locals.var_dfn_sl_dn2 = assign20160_e29106_d_n2;

        let (assign20170_e29123, assign20170_e29123_d_n0, assign20170_e29123_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20170_e29119: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20170_e29120: f64 = (0.5 * assign20170_e29119);
        let assign20170_e29121: f64 = (locals.var_nfasti_i + assign20170_e29120);
        (assign20170_e29121, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign20170_e29123;
        locals.var_nj1_dn0 = assign20170_e29123_d_n0;
        locals.var_nj1_dn2 = assign20170_e29123_d_n2;

        let (assign20180_e29138, assign20180_e29138_d_n0, assign20180_e29138_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20180_e29134: f64 = (p.p85 - locals.var_nj0);
        let assign20180_e29136: f64 = (assign20180_e29134 - 0.01);
        (assign20180_e29136, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20180_e29138;
        locals.var_tmf1_dn0 = assign20180_e29138_d_n0;
        locals.var_tmf1_dn2 = assign20180_e29138_d_n2;

        let (assign20190_e29153, assign20190_e29153_d_n0, assign20190_e29153_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20190_e29149: f64 = (4.0 * p.p85);
        let assign20190_e29151: f64 = (assign20190_e29149 * 0.01);
        (assign20190_e29151, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20190_e29153;
        locals.var_tmf2_dn0 = assign20190_e29153_d_n0;
        locals.var_tmf2_dn2 = assign20190_e29153_d_n2;

        let (assign20200_e29170, assign20200_e29170_d_n0, assign20200_e29170_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20200_e29167: f64 = (-locals.var_tmf2);
                (assign20200_e29167, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20200_e29170;
        locals.var_tmf2_dn0 = assign20200_e29170_d_n0;
        locals.var_tmf2_dn2 = assign20200_e29170_d_n2;

        let (assign20210_e29186, assign20210_e29186_d_n0, assign20210_e29186_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20210_e29181: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20210_e29183: f64 = (assign20210_e29181 + locals.var_tmf2);
        let assign20210_e29184: f64 = (assign20210_e29183).sqrt();
        (assign20210_e29184, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20210_e29184)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20210_e29184)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20210_e29186;
        locals.var_tmf2_dn0 = assign20210_e29186_d_n0;
        locals.var_tmf2_dn2 = assign20210_e29186_d_n2;

        let (assign20220_e29203, assign20220_e29203_d_n0, assign20220_e29203_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20220_e29199: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20220_e29200: f64 = (0.5 * assign20220_e29199);
        let assign20220_e29201: f64 = (p.p85 - assign20220_e29200);
        (assign20220_e29201, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20220_e29203;
        locals.var_nj0_dn0 = assign20220_e29203_d_n0;
        locals.var_nj0_dn2 = assign20220_e29203_d_n2;

        let (assign20230_e29218, assign20230_e29218_d_n0, assign20230_e29218_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20230_e29214: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign20230_e29216: f64 = (assign20230_e29214 - 0.01);
        (assign20230_e29216, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20230_e29218;
        locals.var_tmf1_dn0 = assign20230_e29218_d_n0;
        locals.var_tmf1_dn2 = assign20230_e29218_d_n2;

        let (assign20240_e29233, assign20240_e29233_d_n0, assign20240_e29233_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20240_e29229: f64 = (4.0 * locals.var_nfasti_i);
        let assign20240_e29231: f64 = (assign20240_e29229 * 0.01);
        (assign20240_e29231, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20240_e29233;
        locals.var_tmf2_dn0 = assign20240_e29233_d_n0;
        locals.var_tmf2_dn2 = assign20240_e29233_d_n2;

        let (assign20250_e29250, assign20250_e29250_d_n0, assign20250_e29250_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20250_e29247: f64 = (-locals.var_tmf2);
                (assign20250_e29247, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20250_e29250;
        locals.var_tmf2_dn0 = assign20250_e29250_d_n0;
        locals.var_tmf2_dn2 = assign20250_e29250_d_n2;

        let (assign20260_e29266, assign20260_e29266_d_n0, assign20260_e29266_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20260_e29261: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20260_e29263: f64 = (assign20260_e29261 + locals.var_tmf2);
        let assign20260_e29264: f64 = (assign20260_e29263).sqrt();
        (assign20260_e29264, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20260_e29264)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20260_e29264)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20260_e29266;
        locals.var_tmf2_dn0 = assign20260_e29266_d_n0;
        locals.var_tmf2_dn2 = assign20260_e29266_d_n2;

        let (assign20270_e29283, assign20270_e29283_d_n0, assign20270_e29283_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20270_e29279: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20270_e29280: f64 = (0.5 * assign20270_e29279);
        let assign20270_e29281: f64 = (locals.var_nfasti_i + assign20270_e29280);
        (assign20270_e29281, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20270_e29283;
        locals.var_nj0_dn0 = assign20270_e29283_d_n0;
        locals.var_nj0_dn2 = assign20270_e29283_d_n2;

        let (assign20280_e29298, assign20280_e29298_d_n0, assign20280_e29298_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign20280_e29294: f64 = (p.p86 * locals.var_dfn_su);
        let assign20280_e29296: f64 = (assign20280_e29294 * locals.var_dfn_sl);
        (assign20280_e29296, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign20280_e29294 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign20280_e29294 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign20280_e29298;
        locals.var_dnj1_dv_dn0 = assign20280_e29298_d_n0;
        locals.var_dnj1_dv_dn2 = assign20280_e29298_d_n2;

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20290_e29310, assign20290_e29310_d_n0, assign20290_e29310_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20290_e29310;
        locals.var_nj0_dn0 = assign20290_e29310_d_n0;
        locals.var_nj0_dn2 = assign20290_e29310_d_n2;

        let (assign20300_e29322, assign20300_e29322_d_n0, assign20300_e29322_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign20300_e29322;
        locals.var_nj1_dn0 = assign20300_e29322_d_n0;
        locals.var_nj1_dn2 = assign20300_e29322_d_n2;

        let (assign20310_e29334, assign20310_e29334_d_n0, assign20310_e29334_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard323 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign20310_e29334;
        locals.var_dnj1_dv_dn0 = assign20310_e29334_d_n0;
        locals.var_dnj1_dv_dn2 = assign20310_e29334_d_n2;

        let (assign20370_e29583, assign20370_e29583_d_n0, assign20370_e29583_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20370_e29567: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign20370_e29568: f64 = (locals.var_nj1 - assign20370_e29567);
        let assign20370_e29571: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign20370_e29572: f64 = (assign20370_e29568 / assign20370_e29571);
        let assign20370_e29575: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign20370_e29578: f64 = (locals.var_nj0 * p.p85);
        let assign20370_e29579: f64 = (assign20370_e29575 / assign20370_e29578);
        let assign20370_e29580: f64 = (assign20370_e29572 + assign20370_e29579);
        let assign20370_e29581: f64 = (locals.var_phitdinv * assign20370_e29580);
        (assign20370_e29581, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign20370_e29571) - (assign20370_e29568 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign20370_e29571 * assign20370_e29571)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign20370_e29578) - (assign20370_e29575 * (locals.var_nj0_dn0 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign20370_e29571) - (assign20370_e29568 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign20370_e29571 * assign20370_e29571)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign20370_e29578) - (assign20370_e29575 * (locals.var_nj0_dn2 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign20370_e29583;
        locals.var_dvmax_over_phitd_dv_dn0 = assign20370_e29583_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign20370_e29583_d_n2;

        let (assign20390_e29613,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20390_e29609: f64 = (locals.var_nin * locals.var_nin);
        let assign20390_e29611: f64 = (assign20390_e29609 / locals.var_ndigat_i);
        (assign20390_e29611,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign20390_e29613;

        let (assign20400_e29629,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20400_e29622: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign20400_e29625: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign20400_e29626: f64 = (assign20400_e29625).ln();
        let assign20400_e29627: f64 = (assign20400_e29622 * assign20400_e29626);
        (assign20400_e29627,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign20400_e29629;

        let assign20410_e29632: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign20410_e29632;

        let (assign20420_e29649, assign20420_e29649_d_n0, assign20420_e29649_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20420_e29644: f64 = (locals.var_vmax - locals.var_vha1);
        let assign20420_e29645: f64 = (p.p86 * assign20420_e29644);
        let assign20420_e29647: f64 = (assign20420_e29645 + locals.var_nfagat_i);
        (assign20420_e29647, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign20420_e29649;
        locals.var_nja10_dn0 = assign20420_e29649_d_n0;
        locals.var_nja10_dn2 = assign20420_e29649_d_n2;

        let (assign20430_e29664, assign20430_e29664_d_n0, assign20430_e29664_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20430_e29661: f64 = (p.p86 * locals.var_vha1);
        let assign20430_e29662: f64 = (locals.var_nfagat_i - assign20430_e29661);
        (assign20430_e29662, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20430_e29664;
        locals.var_nj0_dn0 = assign20430_e29664_d_n0;
        locals.var_nj0_dn2 = assign20430_e29664_d_n2;

        let (assign20440_e29679, assign20440_e29679_d_n0, assign20440_e29679_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20440_e29675: f64 = (p.p85 - locals.var_nja10);
        let assign20440_e29677: f64 = (assign20440_e29675 - 0.01);
        (assign20440_e29677, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20440_e29679;
        locals.var_tmf1_dn0 = assign20440_e29679_d_n0;
        locals.var_tmf1_dn2 = assign20440_e29679_d_n2;

        let (assign20450_e29694, assign20450_e29694_d_n0, assign20450_e29694_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20450_e29690: f64 = (4.0 * p.p85);
        let assign20450_e29692: f64 = (assign20450_e29690 * 0.01);
        (assign20450_e29692, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20450_e29694;
        locals.var_tmf2_dn0 = assign20450_e29694_d_n0;
        locals.var_tmf2_dn2 = assign20450_e29694_d_n2;

        let (assign20460_e29711, assign20460_e29711_d_n0, assign20460_e29711_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20460_e29708: f64 = (-locals.var_tmf2);
                (assign20460_e29708, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20460_e29711;
        locals.var_tmf2_dn0 = assign20460_e29711_d_n0;
        locals.var_tmf2_dn2 = assign20460_e29711_d_n2;

        let (assign20470_e29727, assign20470_e29727_d_n0, assign20470_e29727_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20470_e29722: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20470_e29724: f64 = (assign20470_e29722 + locals.var_tmf2);
        let assign20470_e29725: f64 = (assign20470_e29724).sqrt();
        (assign20470_e29725, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20470_e29725)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20470_e29725)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20470_e29727;
        locals.var_tmf2_dn0 = assign20470_e29727_d_n0;
        locals.var_tmf2_dn2 = assign20470_e29727_d_n2;

        let (assign20480_e29744, assign20480_e29744_d_n0, assign20480_e29744_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20480_e29740: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20480_e29741: f64 = (1.0 + assign20480_e29740);
        let assign20480_e29742: f64 = (0.5 * assign20480_e29741);
        (assign20480_e29742, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign20480_e29744;
        locals.var_dfn_su_dn0 = assign20480_e29744_d_n0;
        locals.var_dfn_su_dn2 = assign20480_e29744_d_n2;

        let (assign20490_e29761, assign20490_e29761_d_n0, assign20490_e29761_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20490_e29757: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20490_e29758: f64 = (0.5 * assign20490_e29757);
        let assign20490_e29759: f64 = (p.p85 - assign20490_e29758);
        (assign20490_e29759, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign20490_e29761;
        locals.var_nja11_dn0 = assign20490_e29761_d_n0;
        locals.var_nja11_dn2 = assign20490_e29761_d_n2;

        let (assign20500_e29776, assign20500_e29776_d_n0, assign20500_e29776_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20500_e29772: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign20500_e29774: f64 = (assign20500_e29772 - 0.01);
        (assign20500_e29774, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20500_e29776;
        locals.var_tmf1_dn0 = assign20500_e29776_d_n0;
        locals.var_tmf1_dn2 = assign20500_e29776_d_n2;

        let (assign20510_e29791, assign20510_e29791_d_n0, assign20510_e29791_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20510_e29787: f64 = (4.0 * locals.var_nfagat_i);
        let assign20510_e29789: f64 = (assign20510_e29787 * 0.01);
        (assign20510_e29789, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20510_e29791;
        locals.var_tmf2_dn0 = assign20510_e29791_d_n0;
        locals.var_tmf2_dn2 = assign20510_e29791_d_n2;

        let (assign20520_e29808, assign20520_e29808_d_n0, assign20520_e29808_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20520_e29805: f64 = (-locals.var_tmf2);
                (assign20520_e29805, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20520_e29808;
        locals.var_tmf2_dn0 = assign20520_e29808_d_n0;
        locals.var_tmf2_dn2 = assign20520_e29808_d_n2;

        let (assign20530_e29824, assign20530_e29824_d_n0, assign20530_e29824_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20530_e29819: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20530_e29821: f64 = (assign20530_e29819 + locals.var_tmf2);
        let assign20530_e29822: f64 = (assign20530_e29821).sqrt();
        (assign20530_e29822, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20530_e29822)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20530_e29822)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20530_e29824;
        locals.var_tmf2_dn0 = assign20530_e29824_d_n0;
        locals.var_tmf2_dn2 = assign20530_e29824_d_n2;

        let (assign20540_e29841, assign20540_e29841_d_n0, assign20540_e29841_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20540_e29837: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20540_e29838: f64 = (1.0 + assign20540_e29837);
        let assign20540_e29839: f64 = (0.5 * assign20540_e29838);
        (assign20540_e29839, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign20540_e29841;
        locals.var_dfn_sl_dn0 = assign20540_e29841_d_n0;
        locals.var_dfn_sl_dn2 = assign20540_e29841_d_n2;

        let (assign20550_e29858, assign20550_e29858_d_n0, assign20550_e29858_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20550_e29854: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20550_e29855: f64 = (0.5 * assign20550_e29854);
        let assign20550_e29856: f64 = (locals.var_nfagat_i + assign20550_e29855);
        (assign20550_e29856, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign20550_e29858;
        locals.var_nj1_dn0 = assign20550_e29858_d_n0;
        locals.var_nj1_dn2 = assign20550_e29858_d_n2;

        let (assign20560_e29873, assign20560_e29873_d_n0, assign20560_e29873_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20560_e29869: f64 = (p.p85 - locals.var_nj0);
        let assign20560_e29871: f64 = (assign20560_e29869 - 0.01);
        (assign20560_e29871, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20560_e29873;
        locals.var_tmf1_dn0 = assign20560_e29873_d_n0;
        locals.var_tmf1_dn2 = assign20560_e29873_d_n2;

        let (assign20570_e29888, assign20570_e29888_d_n0, assign20570_e29888_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20570_e29884: f64 = (4.0 * p.p85);
        let assign20570_e29886: f64 = (assign20570_e29884 * 0.01);
        (assign20570_e29886, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20570_e29888;
        locals.var_tmf2_dn0 = assign20570_e29888_d_n0;
        locals.var_tmf2_dn2 = assign20570_e29888_d_n2;

        let (assign20580_e29905, assign20580_e29905_d_n0, assign20580_e29905_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20580_e29902: f64 = (-locals.var_tmf2);
                (assign20580_e29902, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20580_e29905;
        locals.var_tmf2_dn0 = assign20580_e29905_d_n0;
        locals.var_tmf2_dn2 = assign20580_e29905_d_n2;

        let (assign20590_e29921, assign20590_e29921_d_n0, assign20590_e29921_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20590_e29916: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20590_e29918: f64 = (assign20590_e29916 + locals.var_tmf2);
        let assign20590_e29919: f64 = (assign20590_e29918).sqrt();
        (assign20590_e29919, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20590_e29919)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20590_e29919)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20590_e29921;
        locals.var_tmf2_dn0 = assign20590_e29921_d_n0;
        locals.var_tmf2_dn2 = assign20590_e29921_d_n2;

        let (assign20600_e29938, assign20600_e29938_d_n0, assign20600_e29938_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20600_e29934: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20600_e29935: f64 = (0.5 * assign20600_e29934);
        let assign20600_e29936: f64 = (p.p85 - assign20600_e29935);
        (assign20600_e29936, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20600_e29938;
        locals.var_nj0_dn0 = assign20600_e29938_d_n0;
        locals.var_nj0_dn2 = assign20600_e29938_d_n2;

        let (assign20610_e29953, assign20610_e29953_d_n0, assign20610_e29953_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20610_e29949: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign20610_e29951: f64 = (assign20610_e29949 - 0.01);
        (assign20610_e29951, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign20610_e29953;
        locals.var_tmf1_dn0 = assign20610_e29953_d_n0;
        locals.var_tmf1_dn2 = assign20610_e29953_d_n2;

        let (assign20620_e29968, assign20620_e29968_d_n0, assign20620_e29968_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20620_e29964: f64 = (4.0 * locals.var_nfagat_i);
        let assign20620_e29966: f64 = (assign20620_e29964 * 0.01);
        (assign20620_e29966, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20620_e29968;
        locals.var_tmf2_dn0 = assign20620_e29968_d_n0;
        locals.var_tmf2_dn2 = assign20620_e29968_d_n2;

        let (assign20630_e29985, assign20630_e29985_d_n0, assign20630_e29985_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign20630_e29982: f64 = (-locals.var_tmf2);
                (assign20630_e29982, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20630_e29985;
        locals.var_tmf2_dn0 = assign20630_e29985_d_n0;
        locals.var_tmf2_dn2 = assign20630_e29985_d_n2;

        let (assign20640_e30001, assign20640_e30001_d_n0, assign20640_e30001_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20640_e29996: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20640_e29998: f64 = (assign20640_e29996 + locals.var_tmf2);
        let assign20640_e29999: f64 = (assign20640_e29998).sqrt();
        (assign20640_e29999, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20640_e29999)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20640_e29999)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign20640_e30001;
        locals.var_tmf2_dn0 = assign20640_e30001_d_n0;
        locals.var_tmf2_dn2 = assign20640_e30001_d_n2;

        let (assign20650_e30018, assign20650_e30018_d_n0, assign20650_e30018_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20650_e30014: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20650_e30015: f64 = (0.5 * assign20650_e30014);
        let assign20650_e30016: f64 = (locals.var_nfagat_i + assign20650_e30015);
        (assign20650_e30016, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20650_e30018;
        locals.var_nj0_dn0 = assign20650_e30018_d_n0;
        locals.var_nj0_dn2 = assign20650_e30018_d_n2;

        let (assign20660_e30033, assign20660_e30033_d_n0, assign20660_e30033_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 != 0.0)) {
        let assign20660_e30029: f64 = (p.p86 * locals.var_dfn_su);
        let assign20660_e30031: f64 = (assign20660_e30029 * locals.var_dfn_sl);
        (assign20660_e30031, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign20660_e30029 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign20660_e30029 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign20660_e30033;
        locals.var_dnj1_dv_dn0 = assign20660_e30033_d_n0;
        locals.var_dnj1_dv_dn2 = assign20660_e30033_d_n2;

        let (assign20670_e30045, assign20670_e30045_d_n0, assign20670_e30045_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign20670_e30045;
        locals.var_nj0_dn0 = assign20670_e30045_d_n0;
        locals.var_nj0_dn2 = assign20670_e30045_d_n2;

        let (assign20680_e30057, assign20680_e30057_d_n0, assign20680_e30057_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign20680_e30057;
        locals.var_nj1_dn0 = assign20680_e30057_d_n0;
        locals.var_nj1_dn2 = assign20680_e30057_d_n2;

        let (assign20690_e30069, assign20690_e30069_d_n0, assign20690_e30069_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) && (locals.var_guard326 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign20690_e30069;
        locals.var_dnj1_dv_dn0 = assign20690_e30069_d_n0;
        locals.var_dnj1_dv_dn2 = assign20690_e30069_d_n2;

        let (assign20750_e30318, assign20750_e30318_d_n0, assign20750_e30318_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign20750_e30302: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign20750_e30303: f64 = (locals.var_nj1 - assign20750_e30302);
        let assign20750_e30306: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign20750_e30307: f64 = (assign20750_e30303 / assign20750_e30306);
        let assign20750_e30310: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign20750_e30313: f64 = (locals.var_nj0 * p.p85);
        let assign20750_e30314: f64 = (assign20750_e30310 / assign20750_e30313);
        let assign20750_e30315: f64 = (assign20750_e30307 + assign20750_e30314);
        let assign20750_e30316: f64 = (locals.var_phitdinv * assign20750_e30315);
        (assign20750_e30316, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign20750_e30306) - (assign20750_e30303 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign20750_e30306 * assign20750_e30306)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign20750_e30313) - (assign20750_e30310 * (locals.var_nj0_dn0 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign20750_e30306) - (assign20750_e30303 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign20750_e30306 * assign20750_e30306)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign20750_e30313) - (assign20750_e30310 * (locals.var_nj0_dn2 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign20750_e30318;
        locals.var_dvmax_over_phitd_dv_dn0 = assign20750_e30318_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign20750_e30318_d_n2;

        let (assign20770_e30343, assign20770_e30343_d_n0, assign20770_e30343_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard307 != 0.0)) {
        let assign20770_e30341: f64 = (locals.var_idmultbot - 1.0);
        (assign20770_e30341, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign20770_e30343;
        locals.var_idmultbot_dn0 = assign20770_e30343_d_n0;
        locals.var_idmultbot_dn2 = assign20770_e30343_d_n2;

    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20880_e30516, assign20880_e30516_d_n0, assign20880_e30516_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard307 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign20880_e30516;
        locals.var_idmultbot_dn0 = assign20880_e30516_d_n0;
        locals.var_idmultbot_dn2 = assign20880_e30516_d_n2;

        let assign23410_e34066: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign23410_e34066;

        let assign23490_e34138: f64 = if locals.var_v5 < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard385 = assign23490_e34138;

        let (assign23550_e34279,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign23550_e34275: f64 = (locals.var_nin * locals.var_nin);
        let assign23550_e34277: f64 = (assign23550_e34275 / locals.var_ndibot_i);
        (assign23550_e34277,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign23550_e34279;

        let (assign23560_e34294,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign23560_e34287: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign23560_e34290: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign23560_e34291: f64 = (assign23560_e34290).ln();
        let assign23560_e34292: f64 = (assign23560_e34287 * assign23560_e34291);
        (assign23560_e34292,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign23560_e34294;

        let assign23570_e34297: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign23570_e34297;

        let (assign23580_e34313, assign23580_e34313_d_n0, assign23580_e34313_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23580_e34308: f64 = (locals.var_v5 - locals.var_vha1);
        let assign23580_e34309: f64 = (p.p86 * assign23580_e34308);
        let assign23580_e34311: f64 = (assign23580_e34309 + locals.var_nfabot_i);
        (assign23580_e34311, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign23580_e34313;
        locals.var_nja10_dn0 = assign23580_e34313_d_n0;
        locals.var_nja10_dn2 = assign23580_e34313_d_n2;

        let (assign23590_e34327, assign23590_e34327_d_n0, assign23590_e34327_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23590_e34324: f64 = (p.p86 * locals.var_vha1);
        let assign23590_e34325: f64 = (locals.var_nfabot_i - assign23590_e34324);
        (assign23590_e34325, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign23590_e34327;
        locals.var_nj0_dn0 = assign23590_e34327_d_n0;
        locals.var_nj0_dn2 = assign23590_e34327_d_n2;

        let (assign23600_e34341, assign23600_e34341_d_n0, assign23600_e34341_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23600_e34337: f64 = (p.p85 - locals.var_nja10);
        let assign23600_e34339: f64 = (assign23600_e34337 - 0.01);
        (assign23600_e34339, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23600_e34341;
        locals.var_tmf1_dn0 = assign23600_e34341_d_n0;
        locals.var_tmf1_dn2 = assign23600_e34341_d_n2;

        let (assign23610_e34355, assign23610_e34355_d_n0, assign23610_e34355_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23610_e34351: f64 = (4.0 * p.p85);
        let assign23610_e34353: f64 = (assign23610_e34351 * 0.01);
        (assign23610_e34353, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23610_e34355;
        locals.var_tmf2_dn0 = assign23610_e34355_d_n0;
        locals.var_tmf2_dn2 = assign23610_e34355_d_n2;

        let (assign23620_e34371, assign23620_e34371_d_n0, assign23620_e34371_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23620_e34368: f64 = (-locals.var_tmf2);
                (assign23620_e34368, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23620_e34371;
        locals.var_tmf2_dn0 = assign23620_e34371_d_n0;
        locals.var_tmf2_dn2 = assign23620_e34371_d_n2;

        let (assign23630_e34386, assign23630_e34386_d_n0, assign23630_e34386_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23630_e34381: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23630_e34383: f64 = (assign23630_e34381 + locals.var_tmf2);
        let assign23630_e34384: f64 = (assign23630_e34383).sqrt();
        (assign23630_e34384, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23630_e34384)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23630_e34384)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23630_e34386;
        locals.var_tmf2_dn0 = assign23630_e34386_d_n0;
        locals.var_tmf2_dn2 = assign23630_e34386_d_n2;

        let (assign23640_e34402, assign23640_e34402_d_n0, assign23640_e34402_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23640_e34398: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23640_e34399: f64 = (0.5 * assign23640_e34398);
        let assign23640_e34400: f64 = (p.p85 - assign23640_e34399);
        (assign23640_e34400, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign23640_e34402;
        locals.var_nja11_dn0 = assign23640_e34402_d_n0;
        locals.var_nja11_dn2 = assign23640_e34402_d_n2;

        let (assign23650_e34416, assign23650_e34416_d_n0, assign23650_e34416_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23650_e34412: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign23650_e34414: f64 = (assign23650_e34412 - 0.01);
        (assign23650_e34414, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23650_e34416;
        locals.var_tmf1_dn0 = assign23650_e34416_d_n0;
        locals.var_tmf1_dn2 = assign23650_e34416_d_n2;

        let (assign23660_e34430, assign23660_e34430_d_n0, assign23660_e34430_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23660_e34426: f64 = (4.0 * locals.var_nfabot_i);
        let assign23660_e34428: f64 = (assign23660_e34426 * 0.01);
        (assign23660_e34428, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23660_e34430;
        locals.var_tmf2_dn0 = assign23660_e34430_d_n0;
        locals.var_tmf2_dn2 = assign23660_e34430_d_n2;

        let (assign23670_e34446, assign23670_e34446_d_n0, assign23670_e34446_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23670_e34443: f64 = (-locals.var_tmf2);
                (assign23670_e34443, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23670_e34446;
        locals.var_tmf2_dn0 = assign23670_e34446_d_n0;
        locals.var_tmf2_dn2 = assign23670_e34446_d_n2;

        let (assign23680_e34461, assign23680_e34461_d_n0, assign23680_e34461_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23680_e34456: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23680_e34458: f64 = (assign23680_e34456 + locals.var_tmf2);
        let assign23680_e34459: f64 = (assign23680_e34458).sqrt();
        (assign23680_e34459, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23680_e34459)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23680_e34459)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23680_e34461;
        locals.var_tmf2_dn0 = assign23680_e34461_d_n0;
        locals.var_tmf2_dn2 = assign23680_e34461_d_n2;

        let (assign23690_e34477, assign23690_e34477_d_n0, assign23690_e34477_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23690_e34473: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23690_e34474: f64 = (0.5 * assign23690_e34473);
        let assign23690_e34475: f64 = (locals.var_nfabot_i + assign23690_e34474);
        (assign23690_e34475, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign23690_e34477;
        locals.var_nj1_dn0 = assign23690_e34477_d_n0;
        locals.var_nj1_dn2 = assign23690_e34477_d_n2;

        let (assign23700_e34491, assign23700_e34491_d_n0, assign23700_e34491_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23700_e34487: f64 = (p.p85 - locals.var_nj0);
        let assign23700_e34489: f64 = (assign23700_e34487 - 0.01);
        (assign23700_e34489, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23700_e34491;
        locals.var_tmf1_dn0 = assign23700_e34491_d_n0;
        locals.var_tmf1_dn2 = assign23700_e34491_d_n2;

        let (assign23710_e34505, assign23710_e34505_d_n0, assign23710_e34505_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23710_e34501: f64 = (4.0 * p.p85);
        let assign23710_e34503: f64 = (assign23710_e34501 * 0.01);
        (assign23710_e34503, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23710_e34505;
        locals.var_tmf2_dn0 = assign23710_e34505_d_n0;
        locals.var_tmf2_dn2 = assign23710_e34505_d_n2;

        let (assign23720_e34521, assign23720_e34521_d_n0, assign23720_e34521_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23720_e34518: f64 = (-locals.var_tmf2);
                (assign23720_e34518, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23720_e34521;
        locals.var_tmf2_dn0 = assign23720_e34521_d_n0;
        locals.var_tmf2_dn2 = assign23720_e34521_d_n2;

        let (assign23730_e34536, assign23730_e34536_d_n0, assign23730_e34536_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23730_e34531: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23730_e34533: f64 = (assign23730_e34531 + locals.var_tmf2);
        let assign23730_e34534: f64 = (assign23730_e34533).sqrt();
        (assign23730_e34534, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23730_e34534)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23730_e34534)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23730_e34536;
        locals.var_tmf2_dn0 = assign23730_e34536_d_n0;
        locals.var_tmf2_dn2 = assign23730_e34536_d_n2;

        let (assign23740_e34552, assign23740_e34552_d_n0, assign23740_e34552_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23740_e34548: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23740_e34549: f64 = (0.5 * assign23740_e34548);
        let assign23740_e34550: f64 = (p.p85 - assign23740_e34549);
        (assign23740_e34550, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign23740_e34552;
        locals.var_nj0_dn0 = assign23740_e34552_d_n0;
        locals.var_nj0_dn2 = assign23740_e34552_d_n2;

        let (assign23750_e34566, assign23750_e34566_d_n0, assign23750_e34566_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23750_e34562: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign23750_e34564: f64 = (assign23750_e34562 - 0.01);
        (assign23750_e34564, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23750_e34566;
        locals.var_tmf1_dn0 = assign23750_e34566_d_n0;
        locals.var_tmf1_dn2 = assign23750_e34566_d_n2;

        let (assign23760_e34580, assign23760_e34580_d_n0, assign23760_e34580_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23760_e34576: f64 = (4.0 * locals.var_nfabot_i);
        let assign23760_e34578: f64 = (assign23760_e34576 * 0.01);
        (assign23760_e34578, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23760_e34580;
        locals.var_tmf2_dn0 = assign23760_e34580_d_n0;
        locals.var_tmf2_dn2 = assign23760_e34580_d_n2;

        let (assign23770_e34596, assign23770_e34596_d_n0, assign23770_e34596_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23770_e34593: f64 = (-locals.var_tmf2);
                (assign23770_e34593, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23770_e34596;
        locals.var_tmf2_dn0 = assign23770_e34596_d_n0;
        locals.var_tmf2_dn2 = assign23770_e34596_d_n2;

        let (assign23780_e34611, assign23780_e34611_d_n0, assign23780_e34611_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23780_e34606: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23780_e34608: f64 = (assign23780_e34606 + locals.var_tmf2);
        let assign23780_e34609: f64 = (assign23780_e34608).sqrt();
        (assign23780_e34609, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23780_e34609)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23780_e34609)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23780_e34611;
        locals.var_tmf2_dn0 = assign23780_e34611_d_n0;
        locals.var_tmf2_dn2 = assign23780_e34611_d_n2;

        let (assign23790_e34627, assign23790_e34627_d_n0, assign23790_e34627_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign23790_e34623: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23790_e34624: f64 = (0.5 * assign23790_e34623);
        let assign23790_e34625: f64 = (locals.var_nfabot_i + assign23790_e34624);
        (assign23790_e34625, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign23790_e34627;
        locals.var_nj0_dn0 = assign23790_e34627_d_n0;
        locals.var_nj0_dn2 = assign23790_e34627_d_n2;

        let (assign23800_e34638, assign23800_e34638_d_n0, assign23800_e34638_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign23800_e34638;
        locals.var_nj0_dn0 = assign23800_e34638_d_n0;
        locals.var_nj0_dn2 = assign23800_e34638_d_n2;

        let (assign23810_e34649, assign23810_e34649_d_n0, assign23810_e34649_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign23810_e34649;
        locals.var_nj1_dn0 = assign23810_e34649_d_n0;
        locals.var_nj1_dn2 = assign23810_e34649_d_n2;

        let assign23820_e34653: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23820_e34657: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23820_e34658: f64 = (locals.var_vha1 * assign23820_e34657);
        let assign23820_e34661: f64 = (locals.var_nj0 * p.p85);
        let assign23820_e34662: f64 = (assign23820_e34658 / assign23820_e34661);
        let assign23820_e34663: f64 = (assign23820_e34653 + assign23820_e34662);
        let assign23820_e34664: f64 = (locals.var_phitdinv * assign23820_e34663);
        let assign23820_e34665: f64 = (assign23820_e34664).abs();
        let assign23820_e34667: f64 = if assign23820_e34665 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign23820_e34667;

        let (assign23830_e34692, assign23830_e34692_d_n0, assign23830_e34692_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign23830_e34678: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23830_e34682: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23830_e34683: f64 = (locals.var_vha1 * assign23830_e34682);
        let assign23830_e34686: f64 = (locals.var_nj0 * p.p85);
        let assign23830_e34687: f64 = (assign23830_e34683 / assign23830_e34686);
        let assign23830_e34688: f64 = (assign23830_e34678 + assign23830_e34687);
        let assign23830_e34689: f64 = (locals.var_phitdinv * assign23830_e34688);
        let assign23830_e34690: f64 = (assign23830_e34689).exp();
        (assign23830_e34690, (assign23830_e34690 * (locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23830_e34686) - (assign23830_e34683 * (locals.var_nj0_dn0 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23830_e34686) - (assign23830_e34683 * (locals.var_nj0_dn2 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign23830_e34692;
        locals.var_idmultbot_dn0 = assign23830_e34692_d_n0;
        locals.var_idmultbot_dn2 = assign23830_e34692_d_n2;

        let assign23840_e34696: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23840_e34700: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23840_e34701: f64 = (locals.var_vha1 * assign23840_e34700);
        let assign23840_e34704: f64 = (locals.var_nj0 * p.p85);
        let assign23840_e34705: f64 = (assign23840_e34701 / assign23840_e34704);
        let assign23840_e34706: f64 = (assign23840_e34696 + assign23840_e34705);
        let assign23840_e34707: f64 = (locals.var_phitdinv * assign23840_e34706);
        let assign23840_e34709: f64 = (-230.25850929940458);
        let assign23840_e34710: f64 = if assign23840_e34707 < assign23840_e34709 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign23840_e34710;

        let (assign23850_e34790, assign23850_e34790_d_n0, assign23850_e34790_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign23850_e34724: f64 = (-230.25850929940458);
        let assign23850_e34728: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23850_e34732: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23850_e34733: f64 = (locals.var_vha1 * assign23850_e34732);
        let assign23850_e34736: f64 = (locals.var_nj0 * p.p85);
        let assign23850_e34737: f64 = (assign23850_e34733 / assign23850_e34736);
        let assign23850_e34738: f64 = (assign23850_e34728 + assign23850_e34737);
        let assign23850_e34739: f64 = (locals.var_phitdinv * assign23850_e34738);
        let assign23850_e34740: f64 = (assign23850_e34724 - assign23850_e34739);
        let assign23850_e34744: f64 = (-230.25850929940458);
        let assign23850_e34748: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23850_e34752: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23850_e34753: f64 = (locals.var_vha1 * assign23850_e34752);
        let assign23850_e34756: f64 = (locals.var_nj0 * p.p85);
        let assign23850_e34757: f64 = (assign23850_e34753 / assign23850_e34756);
        let assign23850_e34758: f64 = (assign23850_e34748 + assign23850_e34757);
        let assign23850_e34759: f64 = (locals.var_phitdinv * assign23850_e34758);
        let assign23850_e34760: f64 = (assign23850_e34744 - assign23850_e34759);
        let assign23850_e34763: f64 = (-230.25850929940458);
        let assign23850_e34767: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23850_e34771: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23850_e34772: f64 = (locals.var_vha1 * assign23850_e34771);
        let assign23850_e34775: f64 = (locals.var_nj0 * p.p85);
        let assign23850_e34776: f64 = (assign23850_e34772 / assign23850_e34775);
        let assign23850_e34777: f64 = (assign23850_e34767 + assign23850_e34776);
        let assign23850_e34778: f64 = (locals.var_phitdinv * assign23850_e34777);
        let assign23850_e34779: f64 = (assign23850_e34763 - assign23850_e34778);
        let assign23850_e34781: f64 = (assign23850_e34779 * 0.3333333333333333);
        let assign23850_e34782: f64 = (1.0 + assign23850_e34781);
        let assign23850_e34783: f64 = (assign23850_e34760 * assign23850_e34782);
        let assign23850_e34784: f64 = (0.5 * assign23850_e34783);
        let assign23850_e34785: f64 = (1.0 + assign23850_e34784);
        let assign23850_e34786: f64 = (assign23850_e34740 * assign23850_e34785);
        let assign23850_e34787: f64 = (1.0 + assign23850_e34786);
        let assign23850_e34788: f64 = (1e-100 / assign23850_e34787);
        (assign23850_e34788, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23850_e34736) - (assign23850_e34733 * (locals.var_nj0_dn0 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23850_e34756) - (assign23850_e34753 * (locals.var_nj0_dn0 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23850_e34775) - (assign23850_e34772 * (locals.var_nj0_dn0 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23850_e34736) - (assign23850_e34733 * (locals.var_nj0_dn2 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23850_e34756) - (assign23850_e34753 * (locals.var_nj0_dn2 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23850_e34775) - (assign23850_e34772 * (locals.var_nj0_dn2 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign23850_e34790;
        locals.var_idmultbot_dn0 = assign23850_e34790_d_n0;
        locals.var_idmultbot_dn2 = assign23850_e34790_d_n2;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23860_e34868, assign23860_e34868_d_n0, assign23860_e34868_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign23860_e34807: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23860_e34811: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23860_e34812: f64 = (locals.var_vha1 * assign23860_e34811);
        let assign23860_e34815: f64 = (locals.var_nj0 * p.p85);
        let assign23860_e34816: f64 = (assign23860_e34812 / assign23860_e34815);
        let assign23860_e34817: f64 = (assign23860_e34807 + assign23860_e34816);
        let assign23860_e34818: f64 = (locals.var_phitdinv * assign23860_e34817);
        let assign23860_e34820: f64 = (assign23860_e34818 - 230.25850929940458);
        let assign23860_e34826: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23860_e34830: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23860_e34831: f64 = (locals.var_vha1 * assign23860_e34830);
        let assign23860_e34834: f64 = (locals.var_nj0 * p.p85);
        let assign23860_e34835: f64 = (assign23860_e34831 / assign23860_e34834);
        let assign23860_e34836: f64 = (assign23860_e34826 + assign23860_e34835);
        let assign23860_e34837: f64 = (locals.var_phitdinv * assign23860_e34836);
        let assign23860_e34839: f64 = (assign23860_e34837 - 230.25850929940458);
        let assign23860_e34844: f64 = (locals.var_v5 / locals.var_nj1);
        let assign23860_e34848: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign23860_e34849: f64 = (locals.var_vha1 * assign23860_e34848);
        let assign23860_e34852: f64 = (locals.var_nj0 * p.p85);
        let assign23860_e34853: f64 = (assign23860_e34849 / assign23860_e34852);
        let assign23860_e34854: f64 = (assign23860_e34844 + assign23860_e34853);
        let assign23860_e34855: f64 = (locals.var_phitdinv * assign23860_e34854);
        let assign23860_e34857: f64 = (assign23860_e34855 - 230.25850929940458);
        let assign23860_e34859: f64 = (assign23860_e34857 * 0.3333333333333333);
        let assign23860_e34860: f64 = (1.0 + assign23860_e34859);
        let assign23860_e34861: f64 = (assign23860_e34839 * assign23860_e34860);
        let assign23860_e34862: f64 = (0.5 * assign23860_e34861);
        let assign23860_e34863: f64 = (1.0 + assign23860_e34862);
        let assign23860_e34864: f64 = (assign23860_e34820 * assign23860_e34863);
        let assign23860_e34865: f64 = (1.0 + assign23860_e34864);
        let assign23860_e34866: f64 = (1e100 * assign23860_e34865);
        (assign23860_e34866, (1e100 * (((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23860_e34815) - (assign23860_e34812 * (locals.var_nj0_dn0 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23860_e34834) - (assign23860_e34831 * (locals.var_nj0_dn0 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign23860_e34852) - (assign23860_e34849 * (locals.var_nj0_dn0 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23860_e34815) - (assign23860_e34812 * (locals.var_nj0_dn2 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23860_e34834) - (assign23860_e34831 * (locals.var_nj0_dn2 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((locals.var_phitdinv * ((-((locals.var_v5 * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign23860_e34852) - (assign23860_e34849 * (locals.var_nj0_dn2 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign23860_e34868;
        locals.var_idmultbot_dn0 = assign23860_e34868_d_n0;
        locals.var_idmultbot_dn2 = assign23860_e34868_d_n2;

        let (assign23870_e34880,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign23870_e34876: f64 = (locals.var_nin * locals.var_nin);
        let assign23870_e34878: f64 = (assign23870_e34876 / locals.var_ndisti_i);
        (assign23870_e34878,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign23870_e34880;

        let (assign23880_e34895,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign23880_e34888: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign23880_e34891: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign23880_e34892: f64 = (assign23880_e34891).ln();
        let assign23880_e34893: f64 = (assign23880_e34888 * assign23880_e34892);
        (assign23880_e34893,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign23880_e34895;

        let assign23890_e34898: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign23890_e34898;

        let (assign23900_e34914, assign23900_e34914_d_n0, assign23900_e34914_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23900_e34909: f64 = (locals.var_v5 - locals.var_vha1);
        let assign23900_e34910: f64 = (p.p86 * assign23900_e34909);
        let assign23900_e34912: f64 = (assign23900_e34910 + locals.var_nfasti_i);
        (assign23900_e34912, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign23900_e34914;
        locals.var_nja10_dn0 = assign23900_e34914_d_n0;
        locals.var_nja10_dn2 = assign23900_e34914_d_n2;

        let (assign23910_e34928, assign23910_e34928_d_n0, assign23910_e34928_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23910_e34925: f64 = (p.p86 * locals.var_vha1);
        let assign23910_e34926: f64 = (locals.var_nfasti_i - assign23910_e34925);
        (assign23910_e34926, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign23910_e34928;
        locals.var_nj0_dn0 = assign23910_e34928_d_n0;
        locals.var_nj0_dn2 = assign23910_e34928_d_n2;

        let (assign23920_e34942, assign23920_e34942_d_n0, assign23920_e34942_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23920_e34938: f64 = (p.p85 - locals.var_nja10);
        let assign23920_e34940: f64 = (assign23920_e34938 - 0.01);
        (assign23920_e34940, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23920_e34942;
        locals.var_tmf1_dn0 = assign23920_e34942_d_n0;
        locals.var_tmf1_dn2 = assign23920_e34942_d_n2;

        let (assign23930_e34956, assign23930_e34956_d_n0, assign23930_e34956_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23930_e34952: f64 = (4.0 * p.p85);
        let assign23930_e34954: f64 = (assign23930_e34952 * 0.01);
        (assign23930_e34954, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23930_e34956;
        locals.var_tmf2_dn0 = assign23930_e34956_d_n0;
        locals.var_tmf2_dn2 = assign23930_e34956_d_n2;

        let (assign23940_e34972, assign23940_e34972_d_n0, assign23940_e34972_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23940_e34969: f64 = (-locals.var_tmf2);
                (assign23940_e34969, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23940_e34972;
        locals.var_tmf2_dn0 = assign23940_e34972_d_n0;
        locals.var_tmf2_dn2 = assign23940_e34972_d_n2;

        let (assign23950_e34987, assign23950_e34987_d_n0, assign23950_e34987_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23950_e34982: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23950_e34984: f64 = (assign23950_e34982 + locals.var_tmf2);
        let assign23950_e34985: f64 = (assign23950_e34984).sqrt();
        (assign23950_e34985, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23950_e34985)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23950_e34985)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23950_e34987;
        locals.var_tmf2_dn0 = assign23950_e34987_d_n0;
        locals.var_tmf2_dn2 = assign23950_e34987_d_n2;

        let (assign23960_e35003, assign23960_e35003_d_n0, assign23960_e35003_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23960_e34999: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23960_e35000: f64 = (0.5 * assign23960_e34999);
        let assign23960_e35001: f64 = (p.p85 - assign23960_e35000);
        (assign23960_e35001, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign23960_e35003;
        locals.var_nja11_dn0 = assign23960_e35003_d_n0;
        locals.var_nja11_dn2 = assign23960_e35003_d_n2;

        let (assign23970_e35017, assign23970_e35017_d_n0, assign23970_e35017_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23970_e35013: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign23970_e35015: f64 = (assign23970_e35013 - 0.01);
        (assign23970_e35015, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign23970_e35017;
        locals.var_tmf1_dn0 = assign23970_e35017_d_n0;
        locals.var_tmf1_dn2 = assign23970_e35017_d_n2;

        let (assign23980_e35031, assign23980_e35031_d_n0, assign23980_e35031_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign23980_e35027: f64 = (4.0 * locals.var_nfasti_i);
        let assign23980_e35029: f64 = (assign23980_e35027 * 0.01);
        (assign23980_e35029, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23980_e35031;
        locals.var_tmf2_dn0 = assign23980_e35031_d_n0;
        locals.var_tmf2_dn2 = assign23980_e35031_d_n2;

        let (assign23990_e35047, assign23990_e35047_d_n0, assign23990_e35047_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign23990_e35044: f64 = (-locals.var_tmf2);
                (assign23990_e35044, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign23990_e35047;
        locals.var_tmf2_dn0 = assign23990_e35047_d_n0;
        locals.var_tmf2_dn2 = assign23990_e35047_d_n2;

        let (assign24000_e35062, assign24000_e35062_d_n0, assign24000_e35062_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24000_e35057: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24000_e35059: f64 = (assign24000_e35057 + locals.var_tmf2);
        let assign24000_e35060: f64 = (assign24000_e35059).sqrt();
        (assign24000_e35060, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24000_e35060)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24000_e35060)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24000_e35062;
        locals.var_tmf2_dn0 = assign24000_e35062_d_n0;
        locals.var_tmf2_dn2 = assign24000_e35062_d_n2;

        let (assign24010_e35078, assign24010_e35078_d_n0, assign24010_e35078_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24010_e35074: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24010_e35075: f64 = (0.5 * assign24010_e35074);
        let assign24010_e35076: f64 = (locals.var_nfasti_i + assign24010_e35075);
        (assign24010_e35076, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24010_e35078;
        locals.var_nj1_dn0 = assign24010_e35078_d_n0;
        locals.var_nj1_dn2 = assign24010_e35078_d_n2;

        let (assign24020_e35092, assign24020_e35092_d_n0, assign24020_e35092_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24020_e35088: f64 = (p.p85 - locals.var_nj0);
        let assign24020_e35090: f64 = (assign24020_e35088 - 0.01);
        (assign24020_e35090, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24020_e35092;
        locals.var_tmf1_dn0 = assign24020_e35092_d_n0;
        locals.var_tmf1_dn2 = assign24020_e35092_d_n2;

        let (assign24030_e35106, assign24030_e35106_d_n0, assign24030_e35106_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24030_e35102: f64 = (4.0 * p.p85);
        let assign24030_e35104: f64 = (assign24030_e35102 * 0.01);
        (assign24030_e35104, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24030_e35106;
        locals.var_tmf2_dn0 = assign24030_e35106_d_n0;
        locals.var_tmf2_dn2 = assign24030_e35106_d_n2;

        let (assign24040_e35122, assign24040_e35122_d_n0, assign24040_e35122_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24040_e35119: f64 = (-locals.var_tmf2);
                (assign24040_e35119, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24040_e35122;
        locals.var_tmf2_dn0 = assign24040_e35122_d_n0;
        locals.var_tmf2_dn2 = assign24040_e35122_d_n2;

        let (assign24050_e35137, assign24050_e35137_d_n0, assign24050_e35137_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24050_e35132: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24050_e35134: f64 = (assign24050_e35132 + locals.var_tmf2);
        let assign24050_e35135: f64 = (assign24050_e35134).sqrt();
        (assign24050_e35135, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24050_e35135)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24050_e35135)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24050_e35137;
        locals.var_tmf2_dn0 = assign24050_e35137_d_n0;
        locals.var_tmf2_dn2 = assign24050_e35137_d_n2;

        let (assign24060_e35153, assign24060_e35153_d_n0, assign24060_e35153_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24060_e35149: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24060_e35150: f64 = (0.5 * assign24060_e35149);
        let assign24060_e35151: f64 = (p.p85 - assign24060_e35150);
        (assign24060_e35151, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24060_e35153;
        locals.var_nj0_dn0 = assign24060_e35153_d_n0;
        locals.var_nj0_dn2 = assign24060_e35153_d_n2;

        let (assign24070_e35167, assign24070_e35167_d_n0, assign24070_e35167_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24070_e35163: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign24070_e35165: f64 = (assign24070_e35163 - 0.01);
        (assign24070_e35165, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24070_e35167;
        locals.var_tmf1_dn0 = assign24070_e35167_d_n0;
        locals.var_tmf1_dn2 = assign24070_e35167_d_n2;

        let (assign24080_e35181, assign24080_e35181_d_n0, assign24080_e35181_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24080_e35177: f64 = (4.0 * locals.var_nfasti_i);
        let assign24080_e35179: f64 = (assign24080_e35177 * 0.01);
        (assign24080_e35179, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24080_e35181;
        locals.var_tmf2_dn0 = assign24080_e35181_d_n0;
        locals.var_tmf2_dn2 = assign24080_e35181_d_n2;

        let (assign24090_e35197, assign24090_e35197_d_n0, assign24090_e35197_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24090_e35194: f64 = (-locals.var_tmf2);
                (assign24090_e35194, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24090_e35197;
        locals.var_tmf2_dn0 = assign24090_e35197_d_n0;
        locals.var_tmf2_dn2 = assign24090_e35197_d_n2;

        let (assign24100_e35212, assign24100_e35212_d_n0, assign24100_e35212_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24100_e35207: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24100_e35209: f64 = (assign24100_e35207 + locals.var_tmf2);
        let assign24100_e35210: f64 = (assign24100_e35209).sqrt();
        (assign24100_e35210, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24100_e35210)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24100_e35210)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24100_e35212;
        locals.var_tmf2_dn0 = assign24100_e35212_d_n0;
        locals.var_tmf2_dn2 = assign24100_e35212_d_n2;

        let (assign24110_e35228, assign24110_e35228_d_n0, assign24110_e35228_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign24110_e35224: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24110_e35225: f64 = (0.5 * assign24110_e35224);
        let assign24110_e35226: f64 = (locals.var_nfasti_i + assign24110_e35225);
        (assign24110_e35226, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24110_e35228;
        locals.var_nj0_dn0 = assign24110_e35228_d_n0;
        locals.var_nj0_dn2 = assign24110_e35228_d_n2;

        let (assign24120_e35239, assign24120_e35239_d_n0, assign24120_e35239_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24120_e35239;
        locals.var_nj0_dn0 = assign24120_e35239_d_n0;
        locals.var_nj0_dn2 = assign24120_e35239_d_n2;

        let (assign24130_e35250, assign24130_e35250_d_n0, assign24130_e35250_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard391 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24130_e35250;
        locals.var_nj1_dn0 = assign24130_e35250_d_n0;
        locals.var_nj1_dn2 = assign24130_e35250_d_n2;

        let (assign24190_e35481,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign24190_e35477: f64 = (locals.var_nin * locals.var_nin);
        let assign24190_e35479: f64 = (assign24190_e35477 / locals.var_ndigat_i);
        (assign24190_e35479,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign24190_e35481;

        let (assign24200_e35496,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign24200_e35489: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign24200_e35492: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign24200_e35493: f64 = (assign24200_e35492).ln();
        let assign24200_e35494: f64 = (assign24200_e35489 * assign24200_e35493);
        (assign24200_e35494,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign24200_e35496;

        let assign24210_e35499: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign24210_e35499;

        let (assign24220_e35515, assign24220_e35515_d_n0, assign24220_e35515_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24220_e35510: f64 = (locals.var_v5 - locals.var_vha1);
        let assign24220_e35511: f64 = (p.p86 * assign24220_e35510);
        let assign24220_e35513: f64 = (assign24220_e35511 + locals.var_nfagat_i);
        (assign24220_e35513, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign24220_e35515;
        locals.var_nja10_dn0 = assign24220_e35515_d_n0;
        locals.var_nja10_dn2 = assign24220_e35515_d_n2;

        let (assign24230_e35529, assign24230_e35529_d_n0, assign24230_e35529_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24230_e35526: f64 = (p.p86 * locals.var_vha1);
        let assign24230_e35527: f64 = (locals.var_nfagat_i - assign24230_e35526);
        (assign24230_e35527, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24230_e35529;
        locals.var_nj0_dn0 = assign24230_e35529_d_n0;
        locals.var_nj0_dn2 = assign24230_e35529_d_n2;

        let (assign24240_e35543, assign24240_e35543_d_n0, assign24240_e35543_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24240_e35539: f64 = (p.p85 - locals.var_nja10);
        let assign24240_e35541: f64 = (assign24240_e35539 - 0.01);
        (assign24240_e35541, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24240_e35543;
        locals.var_tmf1_dn0 = assign24240_e35543_d_n0;
        locals.var_tmf1_dn2 = assign24240_e35543_d_n2;

        let (assign24250_e35557, assign24250_e35557_d_n0, assign24250_e35557_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24250_e35553: f64 = (4.0 * p.p85);
        let assign24250_e35555: f64 = (assign24250_e35553 * 0.01);
        (assign24250_e35555, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24250_e35557;
        locals.var_tmf2_dn0 = assign24250_e35557_d_n0;
        locals.var_tmf2_dn2 = assign24250_e35557_d_n2;

        let (assign24260_e35573, assign24260_e35573_d_n0, assign24260_e35573_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24260_e35570: f64 = (-locals.var_tmf2);
                (assign24260_e35570, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24260_e35573;
        locals.var_tmf2_dn0 = assign24260_e35573_d_n0;
        locals.var_tmf2_dn2 = assign24260_e35573_d_n2;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24270_e35588, assign24270_e35588_d_n0, assign24270_e35588_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24270_e35583: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24270_e35585: f64 = (assign24270_e35583 + locals.var_tmf2);
        let assign24270_e35586: f64 = (assign24270_e35585).sqrt();
        (assign24270_e35586, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24270_e35586)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24270_e35586)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24270_e35588;
        locals.var_tmf2_dn0 = assign24270_e35588_d_n0;
        locals.var_tmf2_dn2 = assign24270_e35588_d_n2;

        let (assign24280_e35604, assign24280_e35604_d_n0, assign24280_e35604_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24280_e35600: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24280_e35601: f64 = (0.5 * assign24280_e35600);
        let assign24280_e35602: f64 = (p.p85 - assign24280_e35601);
        (assign24280_e35602, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign24280_e35604;
        locals.var_nja11_dn0 = assign24280_e35604_d_n0;
        locals.var_nja11_dn2 = assign24280_e35604_d_n2;

        let (assign24290_e35618, assign24290_e35618_d_n0, assign24290_e35618_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24290_e35614: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign24290_e35616: f64 = (assign24290_e35614 - 0.01);
        (assign24290_e35616, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24290_e35618;
        locals.var_tmf1_dn0 = assign24290_e35618_d_n0;
        locals.var_tmf1_dn2 = assign24290_e35618_d_n2;

        let (assign24300_e35632, assign24300_e35632_d_n0, assign24300_e35632_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24300_e35628: f64 = (4.0 * locals.var_nfagat_i);
        let assign24300_e35630: f64 = (assign24300_e35628 * 0.01);
        (assign24300_e35630, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24300_e35632;
        locals.var_tmf2_dn0 = assign24300_e35632_d_n0;
        locals.var_tmf2_dn2 = assign24300_e35632_d_n2;

        let (assign24310_e35648, assign24310_e35648_d_n0, assign24310_e35648_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24310_e35645: f64 = (-locals.var_tmf2);
                (assign24310_e35645, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24310_e35648;
        locals.var_tmf2_dn0 = assign24310_e35648_d_n0;
        locals.var_tmf2_dn2 = assign24310_e35648_d_n2;

        let (assign24320_e35663, assign24320_e35663_d_n0, assign24320_e35663_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24320_e35658: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24320_e35660: f64 = (assign24320_e35658 + locals.var_tmf2);
        let assign24320_e35661: f64 = (assign24320_e35660).sqrt();
        (assign24320_e35661, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24320_e35661)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24320_e35661)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24320_e35663;
        locals.var_tmf2_dn0 = assign24320_e35663_d_n0;
        locals.var_tmf2_dn2 = assign24320_e35663_d_n2;

        let (assign24330_e35679, assign24330_e35679_d_n0, assign24330_e35679_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24330_e35675: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24330_e35676: f64 = (0.5 * assign24330_e35675);
        let assign24330_e35677: f64 = (locals.var_nfagat_i + assign24330_e35676);
        (assign24330_e35677, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24330_e35679;
        locals.var_nj1_dn0 = assign24330_e35679_d_n0;
        locals.var_nj1_dn2 = assign24330_e35679_d_n2;

        let (assign24340_e35693, assign24340_e35693_d_n0, assign24340_e35693_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24340_e35689: f64 = (p.p85 - locals.var_nj0);
        let assign24340_e35691: f64 = (assign24340_e35689 - 0.01);
        (assign24340_e35691, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24340_e35693;
        locals.var_tmf1_dn0 = assign24340_e35693_d_n0;
        locals.var_tmf1_dn2 = assign24340_e35693_d_n2;

        let (assign24350_e35707, assign24350_e35707_d_n0, assign24350_e35707_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24350_e35703: f64 = (4.0 * p.p85);
        let assign24350_e35705: f64 = (assign24350_e35703 * 0.01);
        (assign24350_e35705, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24350_e35707;
        locals.var_tmf2_dn0 = assign24350_e35707_d_n0;
        locals.var_tmf2_dn2 = assign24350_e35707_d_n2;

        let (assign24360_e35723, assign24360_e35723_d_n0, assign24360_e35723_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24360_e35720: f64 = (-locals.var_tmf2);
                (assign24360_e35720, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24360_e35723;
        locals.var_tmf2_dn0 = assign24360_e35723_d_n0;
        locals.var_tmf2_dn2 = assign24360_e35723_d_n2;

        let (assign24370_e35738, assign24370_e35738_d_n0, assign24370_e35738_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24370_e35733: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24370_e35735: f64 = (assign24370_e35733 + locals.var_tmf2);
        let assign24370_e35736: f64 = (assign24370_e35735).sqrt();
        (assign24370_e35736, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24370_e35736)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24370_e35736)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24370_e35738;
        locals.var_tmf2_dn0 = assign24370_e35738_d_n0;
        locals.var_tmf2_dn2 = assign24370_e35738_d_n2;

        let (assign24380_e35754, assign24380_e35754_d_n0, assign24380_e35754_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24380_e35750: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24380_e35751: f64 = (0.5 * assign24380_e35750);
        let assign24380_e35752: f64 = (p.p85 - assign24380_e35751);
        (assign24380_e35752, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24380_e35754;
        locals.var_nj0_dn0 = assign24380_e35754_d_n0;
        locals.var_nj0_dn2 = assign24380_e35754_d_n2;

        let (assign24390_e35768, assign24390_e35768_d_n0, assign24390_e35768_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24390_e35764: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign24390_e35766: f64 = (assign24390_e35764 - 0.01);
        (assign24390_e35766, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24390_e35768;
        locals.var_tmf1_dn0 = assign24390_e35768_d_n0;
        locals.var_tmf1_dn2 = assign24390_e35768_d_n2;

        let (assign24400_e35782, assign24400_e35782_d_n0, assign24400_e35782_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24400_e35778: f64 = (4.0 * locals.var_nfagat_i);
        let assign24400_e35780: f64 = (assign24400_e35778 * 0.01);
        (assign24400_e35780, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24400_e35782;
        locals.var_tmf2_dn0 = assign24400_e35782_d_n0;
        locals.var_tmf2_dn2 = assign24400_e35782_d_n2;

        let (assign24410_e35798, assign24410_e35798_d_n0, assign24410_e35798_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24410_e35795: f64 = (-locals.var_tmf2);
                (assign24410_e35795, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24410_e35798;
        locals.var_tmf2_dn0 = assign24410_e35798_d_n0;
        locals.var_tmf2_dn2 = assign24410_e35798_d_n2;

        let (assign24420_e35813, assign24420_e35813_d_n0, assign24420_e35813_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24420_e35808: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24420_e35810: f64 = (assign24420_e35808 + locals.var_tmf2);
        let assign24420_e35811: f64 = (assign24420_e35810).sqrt();
        (assign24420_e35811, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24420_e35811)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24420_e35811)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24420_e35813;
        locals.var_tmf2_dn0 = assign24420_e35813_d_n0;
        locals.var_tmf2_dn2 = assign24420_e35813_d_n2;

        let (assign24430_e35829, assign24430_e35829_d_n0, assign24430_e35829_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign24430_e35825: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24430_e35826: f64 = (0.5 * assign24430_e35825);
        let assign24430_e35827: f64 = (locals.var_nfagat_i + assign24430_e35826);
        (assign24430_e35827, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24430_e35829;
        locals.var_nj0_dn0 = assign24430_e35829_d_n0;
        locals.var_nj0_dn2 = assign24430_e35829_d_n2;

        let (assign24440_e35840, assign24440_e35840_d_n0, assign24440_e35840_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24440_e35840;
        locals.var_nj0_dn0 = assign24440_e35840_d_n0;
        locals.var_nj0_dn2 = assign24440_e35840_d_n2;

        let (assign24450_e35851, assign24450_e35851_d_n0, assign24450_e35851_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard394 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24450_e35851;
        locals.var_nj1_dn0 = assign24450_e35851_d_n0;
        locals.var_nj1_dn2 = assign24450_e35851_d_n2;

        let (assign24520_e36101,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24520_e36097: f64 = (locals.var_nin * locals.var_nin);
        let assign24520_e36099: f64 = (assign24520_e36097 / locals.var_ndibot_i);
        (assign24520_e36099,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign24520_e36101;

        let (assign24530_e36117,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24530_e36110: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign24530_e36113: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign24530_e36114: f64 = (assign24530_e36113).ln();
        let assign24530_e36115: f64 = (assign24530_e36110 * assign24530_e36114);
        (assign24530_e36115,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign24530_e36117;

        let assign24540_e36120: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard397 = assign24540_e36120;

        let (assign24550_e36137, assign24550_e36137_d_n0, assign24550_e36137_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24550_e36132: f64 = (locals.var_vmax - locals.var_vha1);
        let assign24550_e36133: f64 = (p.p86 * assign24550_e36132);
        let assign24550_e36135: f64 = (assign24550_e36133 + locals.var_nfabot_i);
        (assign24550_e36135, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign24550_e36137;
        locals.var_nja10_dn0 = assign24550_e36137_d_n0;
        locals.var_nja10_dn2 = assign24550_e36137_d_n2;

        let (assign24560_e36152, assign24560_e36152_d_n0, assign24560_e36152_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24560_e36149: f64 = (p.p86 * locals.var_vha1);
        let assign24560_e36150: f64 = (locals.var_nfabot_i - assign24560_e36149);
        (assign24560_e36150, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24560_e36152;
        locals.var_nj0_dn0 = assign24560_e36152_d_n0;
        locals.var_nj0_dn2 = assign24560_e36152_d_n2;

        let (assign24570_e36167, assign24570_e36167_d_n0, assign24570_e36167_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24570_e36163: f64 = (p.p85 - locals.var_nja10);
        let assign24570_e36165: f64 = (assign24570_e36163 - 0.01);
        (assign24570_e36165, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24570_e36167;
        locals.var_tmf1_dn0 = assign24570_e36167_d_n0;
        locals.var_tmf1_dn2 = assign24570_e36167_d_n2;

        let (assign24580_e36182, assign24580_e36182_d_n0, assign24580_e36182_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24580_e36178: f64 = (4.0 * p.p85);
        let assign24580_e36180: f64 = (assign24580_e36178 * 0.01);
        (assign24580_e36180, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24580_e36182;
        locals.var_tmf2_dn0 = assign24580_e36182_d_n0;
        locals.var_tmf2_dn2 = assign24580_e36182_d_n2;

        let (assign24590_e36199, assign24590_e36199_d_n0, assign24590_e36199_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24590_e36196: f64 = (-locals.var_tmf2);
                (assign24590_e36196, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24590_e36199;
        locals.var_tmf2_dn0 = assign24590_e36199_d_n0;
        locals.var_tmf2_dn2 = assign24590_e36199_d_n2;

        let (assign24600_e36215, assign24600_e36215_d_n0, assign24600_e36215_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24600_e36210: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24600_e36212: f64 = (assign24600_e36210 + locals.var_tmf2);
        let assign24600_e36213: f64 = (assign24600_e36212).sqrt();
        (assign24600_e36213, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24600_e36213)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24600_e36213)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24600_e36215;
        locals.var_tmf2_dn0 = assign24600_e36215_d_n0;
        locals.var_tmf2_dn2 = assign24600_e36215_d_n2;

        let (assign24610_e36232, assign24610_e36232_d_n0, assign24610_e36232_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24610_e36228: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign24610_e36229: f64 = (1.0 + assign24610_e36228);
        let assign24610_e36230: f64 = (0.5 * assign24610_e36229);
        (assign24610_e36230, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign24610_e36232;
        locals.var_dfn_su_dn0 = assign24610_e36232_d_n0;
        locals.var_dfn_su_dn2 = assign24610_e36232_d_n2;

        let (assign24620_e36249, assign24620_e36249_d_n0, assign24620_e36249_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24620_e36245: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24620_e36246: f64 = (0.5 * assign24620_e36245);
        let assign24620_e36247: f64 = (p.p85 - assign24620_e36246);
        (assign24620_e36247, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign24620_e36249;
        locals.var_nja11_dn0 = assign24620_e36249_d_n0;
        locals.var_nja11_dn2 = assign24620_e36249_d_n2;

        let (assign24630_e36264, assign24630_e36264_d_n0, assign24630_e36264_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24630_e36260: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign24630_e36262: f64 = (assign24630_e36260 - 0.01);
        (assign24630_e36262, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24630_e36264;
        locals.var_tmf1_dn0 = assign24630_e36264_d_n0;
        locals.var_tmf1_dn2 = assign24630_e36264_d_n2;

        let (assign24640_e36279, assign24640_e36279_d_n0, assign24640_e36279_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24640_e36275: f64 = (4.0 * locals.var_nfabot_i);
        let assign24640_e36277: f64 = (assign24640_e36275 * 0.01);
        (assign24640_e36277, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24640_e36279;
        locals.var_tmf2_dn0 = assign24640_e36279_d_n0;
        locals.var_tmf2_dn2 = assign24640_e36279_d_n2;

        let (assign24650_e36296, assign24650_e36296_d_n0, assign24650_e36296_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24650_e36293: f64 = (-locals.var_tmf2);
                (assign24650_e36293, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24650_e36296;
        locals.var_tmf2_dn0 = assign24650_e36296_d_n0;
        locals.var_tmf2_dn2 = assign24650_e36296_d_n2;

        let (assign24660_e36312, assign24660_e36312_d_n0, assign24660_e36312_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24660_e36307: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24660_e36309: f64 = (assign24660_e36307 + locals.var_tmf2);
        let assign24660_e36310: f64 = (assign24660_e36309).sqrt();
        (assign24660_e36310, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24660_e36310)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24660_e36310)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24660_e36312;
        locals.var_tmf2_dn0 = assign24660_e36312_d_n0;
        locals.var_tmf2_dn2 = assign24660_e36312_d_n2;

        let (assign24670_e36329, assign24670_e36329_d_n0, assign24670_e36329_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24670_e36325: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign24670_e36326: f64 = (1.0 + assign24670_e36325);
        let assign24670_e36327: f64 = (0.5 * assign24670_e36326);
        (assign24670_e36327, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign24670_e36329;
        locals.var_dfn_sl_dn0 = assign24670_e36329_d_n0;
        locals.var_dfn_sl_dn2 = assign24670_e36329_d_n2;

        let (assign24680_e36346, assign24680_e36346_d_n0, assign24680_e36346_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24680_e36342: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24680_e36343: f64 = (0.5 * assign24680_e36342);
        let assign24680_e36344: f64 = (locals.var_nfabot_i + assign24680_e36343);
        (assign24680_e36344, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24680_e36346;
        locals.var_nj1_dn0 = assign24680_e36346_d_n0;
        locals.var_nj1_dn2 = assign24680_e36346_d_n2;

        let (assign24690_e36361, assign24690_e36361_d_n0, assign24690_e36361_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24690_e36357: f64 = (p.p85 - locals.var_nj0);
        let assign24690_e36359: f64 = (assign24690_e36357 - 0.01);
        (assign24690_e36359, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24690_e36361;
        locals.var_tmf1_dn0 = assign24690_e36361_d_n0;
        locals.var_tmf1_dn2 = assign24690_e36361_d_n2;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24700_e36376, assign24700_e36376_d_n0, assign24700_e36376_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24700_e36372: f64 = (4.0 * p.p85);
        let assign24700_e36374: f64 = (assign24700_e36372 * 0.01);
        (assign24700_e36374, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24700_e36376;
        locals.var_tmf2_dn0 = assign24700_e36376_d_n0;
        locals.var_tmf2_dn2 = assign24700_e36376_d_n2;

        let (assign24710_e36393, assign24710_e36393_d_n0, assign24710_e36393_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24710_e36390: f64 = (-locals.var_tmf2);
                (assign24710_e36390, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24710_e36393;
        locals.var_tmf2_dn0 = assign24710_e36393_d_n0;
        locals.var_tmf2_dn2 = assign24710_e36393_d_n2;

        let (assign24720_e36409, assign24720_e36409_d_n0, assign24720_e36409_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24720_e36404: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24720_e36406: f64 = (assign24720_e36404 + locals.var_tmf2);
        let assign24720_e36407: f64 = (assign24720_e36406).sqrt();
        (assign24720_e36407, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24720_e36407)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24720_e36407)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24720_e36409;
        locals.var_tmf2_dn0 = assign24720_e36409_d_n0;
        locals.var_tmf2_dn2 = assign24720_e36409_d_n2;

        let (assign24730_e36426, assign24730_e36426_d_n0, assign24730_e36426_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24730_e36422: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24730_e36423: f64 = (0.5 * assign24730_e36422);
        let assign24730_e36424: f64 = (p.p85 - assign24730_e36423);
        (assign24730_e36424, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24730_e36426;
        locals.var_nj0_dn0 = assign24730_e36426_d_n0;
        locals.var_nj0_dn2 = assign24730_e36426_d_n2;

        let (assign24740_e36441, assign24740_e36441_d_n0, assign24740_e36441_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24740_e36437: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign24740_e36439: f64 = (assign24740_e36437 - 0.01);
        (assign24740_e36439, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24740_e36441;
        locals.var_tmf1_dn0 = assign24740_e36441_d_n0;
        locals.var_tmf1_dn2 = assign24740_e36441_d_n2;

        let (assign24750_e36456, assign24750_e36456_d_n0, assign24750_e36456_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24750_e36452: f64 = (4.0 * locals.var_nfabot_i);
        let assign24750_e36454: f64 = (assign24750_e36452 * 0.01);
        (assign24750_e36454, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24750_e36456;
        locals.var_tmf2_dn0 = assign24750_e36456_d_n0;
        locals.var_tmf2_dn2 = assign24750_e36456_d_n2;

        let (assign24760_e36473, assign24760_e36473_d_n0, assign24760_e36473_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24760_e36470: f64 = (-locals.var_tmf2);
                (assign24760_e36470, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24760_e36473;
        locals.var_tmf2_dn0 = assign24760_e36473_d_n0;
        locals.var_tmf2_dn2 = assign24760_e36473_d_n2;

        let (assign24770_e36489, assign24770_e36489_d_n0, assign24770_e36489_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24770_e36484: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24770_e36486: f64 = (assign24770_e36484 + locals.var_tmf2);
        let assign24770_e36487: f64 = (assign24770_e36486).sqrt();
        (assign24770_e36487, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24770_e36487)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24770_e36487)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24770_e36489;
        locals.var_tmf2_dn0 = assign24770_e36489_d_n0;
        locals.var_tmf2_dn2 = assign24770_e36489_d_n2;

        let (assign24780_e36506, assign24780_e36506_d_n0, assign24780_e36506_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24780_e36502: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign24780_e36503: f64 = (0.5 * assign24780_e36502);
        let assign24780_e36504: f64 = (locals.var_nfabot_i + assign24780_e36503);
        (assign24780_e36504, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24780_e36506;
        locals.var_nj0_dn0 = assign24780_e36506_d_n0;
        locals.var_nj0_dn2 = assign24780_e36506_d_n2;

        let (assign24790_e36521, assign24790_e36521_d_n0, assign24790_e36521_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 != 0.0)) {
        let assign24790_e36517: f64 = (p.p86 * locals.var_dfn_su);
        let assign24790_e36519: f64 = (assign24790_e36517 * locals.var_dfn_sl);
        (assign24790_e36519, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign24790_e36517 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign24790_e36517 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign24790_e36521;
        locals.var_dnj1_dv_dn0 = assign24790_e36521_d_n0;
        locals.var_dnj1_dv_dn2 = assign24790_e36521_d_n2;

        let (assign24800_e36533, assign24800_e36533_d_n0, assign24800_e36533_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24800_e36533;
        locals.var_nj0_dn0 = assign24800_e36533_d_n0;
        locals.var_nj0_dn2 = assign24800_e36533_d_n2;

        let (assign24810_e36545, assign24810_e36545_d_n0, assign24810_e36545_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign24810_e36545;
        locals.var_nj1_dn0 = assign24810_e36545_d_n0;
        locals.var_nj1_dn2 = assign24810_e36545_d_n2;

        let (assign24820_e36557, assign24820_e36557_d_n0, assign24820_e36557_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard397 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign24820_e36557;
        locals.var_dnj1_dv_dn0 = assign24820_e36557_d_n0;
        locals.var_dnj1_dv_dn2 = assign24820_e36557_d_n2;

        let assign24830_e36561: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24830_e36565: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24830_e36566: f64 = (locals.var_vha1 * assign24830_e36565);
        let assign24830_e36569: f64 = (locals.var_nj0 * p.p85);
        let assign24830_e36570: f64 = (assign24830_e36566 / assign24830_e36569);
        let assign24830_e36571: f64 = (assign24830_e36561 + assign24830_e36570);
        let assign24830_e36572: f64 = (locals.var_phitdinv * assign24830_e36571);
        let assign24830_e36573: f64 = (assign24830_e36572).abs();
        let assign24830_e36575: f64 = if assign24830_e36573 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign24830_e36575;

        let (assign24840_e36601, assign24840_e36601_d_n0, assign24840_e36601_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign24840_e36587: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24840_e36591: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24840_e36592: f64 = (locals.var_vha1 * assign24840_e36591);
        let assign24840_e36595: f64 = (locals.var_nj0 * p.p85);
        let assign24840_e36596: f64 = (assign24840_e36592 / assign24840_e36595);
        let assign24840_e36597: f64 = (assign24840_e36587 + assign24840_e36596);
        let assign24840_e36598: f64 = (locals.var_phitdinv * assign24840_e36597);
        let assign24840_e36599: f64 = (assign24840_e36598).exp();
        (assign24840_e36599, (assign24840_e36599 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24840_e36595) - (assign24840_e36592 * (locals.var_nj0_dn0 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24840_e36595) - (assign24840_e36592 * (locals.var_nj0_dn2 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign24840_e36601;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign24840_e36601_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign24840_e36601_d_n2;

        let assign24850_e36605: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24850_e36609: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24850_e36610: f64 = (locals.var_vha1 * assign24850_e36609);
        let assign24850_e36613: f64 = (locals.var_nj0 * p.p85);
        let assign24850_e36614: f64 = (assign24850_e36610 / assign24850_e36613);
        let assign24850_e36615: f64 = (assign24850_e36605 + assign24850_e36614);
        let assign24850_e36616: f64 = (locals.var_phitdinv * assign24850_e36615);
        let assign24850_e36618: f64 = (-230.25850929940458);
        let assign24850_e36619: f64 = if assign24850_e36616 < assign24850_e36618 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign24850_e36619;

        let (assign24860_e36700, assign24860_e36700_d_n0, assign24860_e36700_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 != 0.0)) {
        let assign24860_e36634: f64 = (-230.25850929940458);
        let assign24860_e36638: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24860_e36642: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24860_e36643: f64 = (locals.var_vha1 * assign24860_e36642);
        let assign24860_e36646: f64 = (locals.var_nj0 * p.p85);
        let assign24860_e36647: f64 = (assign24860_e36643 / assign24860_e36646);
        let assign24860_e36648: f64 = (assign24860_e36638 + assign24860_e36647);
        let assign24860_e36649: f64 = (locals.var_phitdinv * assign24860_e36648);
        let assign24860_e36650: f64 = (assign24860_e36634 - assign24860_e36649);
        let assign24860_e36654: f64 = (-230.25850929940458);
        let assign24860_e36658: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24860_e36662: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24860_e36663: f64 = (locals.var_vha1 * assign24860_e36662);
        let assign24860_e36666: f64 = (locals.var_nj0 * p.p85);
        let assign24860_e36667: f64 = (assign24860_e36663 / assign24860_e36666);
        let assign24860_e36668: f64 = (assign24860_e36658 + assign24860_e36667);
        let assign24860_e36669: f64 = (locals.var_phitdinv * assign24860_e36668);
        let assign24860_e36670: f64 = (assign24860_e36654 - assign24860_e36669);
        let assign24860_e36673: f64 = (-230.25850929940458);
        let assign24860_e36677: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24860_e36681: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24860_e36682: f64 = (locals.var_vha1 * assign24860_e36681);
        let assign24860_e36685: f64 = (locals.var_nj0 * p.p85);
        let assign24860_e36686: f64 = (assign24860_e36682 / assign24860_e36685);
        let assign24860_e36687: f64 = (assign24860_e36677 + assign24860_e36686);
        let assign24860_e36688: f64 = (locals.var_phitdinv * assign24860_e36687);
        let assign24860_e36689: f64 = (assign24860_e36673 - assign24860_e36688);
        let assign24860_e36691: f64 = (assign24860_e36689 * 0.3333333333333333);
        let assign24860_e36692: f64 = (1.0 + assign24860_e36691);
        let assign24860_e36693: f64 = (assign24860_e36670 * assign24860_e36692);
        let assign24860_e36694: f64 = (0.5 * assign24860_e36693);
        let assign24860_e36695: f64 = (1.0 + assign24860_e36694);
        let assign24860_e36696: f64 = (assign24860_e36650 * assign24860_e36695);
        let assign24860_e36697: f64 = (1.0 + assign24860_e36696);
        let assign24860_e36698: f64 = (1e-100 / assign24860_e36697);
        (assign24860_e36698, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24860_e36646) - (assign24860_e36643 * (locals.var_nj0_dn0 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24860_e36666) - (assign24860_e36663 * (locals.var_nj0_dn0 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24860_e36685) - (assign24860_e36682 * (locals.var_nj0_dn0 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24860_e36646) - (assign24860_e36643 * (locals.var_nj0_dn2 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24860_e36666) - (assign24860_e36663 * (locals.var_nj0_dn2 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24860_e36685) - (assign24860_e36682 * (locals.var_nj0_dn2 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign24860_e36700;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign24860_e36700_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign24860_e36700_d_n2;

        let (assign24870_e36779, assign24870_e36779_d_n0, assign24870_e36779_d_n2,) = {
    if (((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 == 0.0)) {
        let assign24870_e36718: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24870_e36722: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24870_e36723: f64 = (locals.var_vha1 * assign24870_e36722);
        let assign24870_e36726: f64 = (locals.var_nj0 * p.p85);
        let assign24870_e36727: f64 = (assign24870_e36723 / assign24870_e36726);
        let assign24870_e36728: f64 = (assign24870_e36718 + assign24870_e36727);
        let assign24870_e36729: f64 = (locals.var_phitdinv * assign24870_e36728);
        let assign24870_e36731: f64 = (assign24870_e36729 - 230.25850929940458);
        let assign24870_e36737: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24870_e36741: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24870_e36742: f64 = (locals.var_vha1 * assign24870_e36741);
        let assign24870_e36745: f64 = (locals.var_nj0 * p.p85);
        let assign24870_e36746: f64 = (assign24870_e36742 / assign24870_e36745);
        let assign24870_e36747: f64 = (assign24870_e36737 + assign24870_e36746);
        let assign24870_e36748: f64 = (locals.var_phitdinv * assign24870_e36747);
        let assign24870_e36750: f64 = (assign24870_e36748 - 230.25850929940458);
        let assign24870_e36755: f64 = (locals.var_vmax / locals.var_nj1);
        let assign24870_e36759: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign24870_e36760: f64 = (locals.var_vha1 * assign24870_e36759);
        let assign24870_e36763: f64 = (locals.var_nj0 * p.p85);
        let assign24870_e36764: f64 = (assign24870_e36760 / assign24870_e36763);
        let assign24870_e36765: f64 = (assign24870_e36755 + assign24870_e36764);
        let assign24870_e36766: f64 = (locals.var_phitdinv * assign24870_e36765);
        let assign24870_e36768: f64 = (assign24870_e36766 - 230.25850929940458);
        let assign24870_e36770: f64 = (assign24870_e36768 * 0.3333333333333333);
        let assign24870_e36771: f64 = (1.0 + assign24870_e36770);
        let assign24870_e36772: f64 = (assign24870_e36750 * assign24870_e36771);
        let assign24870_e36773: f64 = (0.5 * assign24870_e36772);
        let assign24870_e36774: f64 = (1.0 + assign24870_e36773);
        let assign24870_e36775: f64 = (assign24870_e36731 * assign24870_e36774);
        let assign24870_e36776: f64 = (1.0 + assign24870_e36775);
        let assign24870_e36777: f64 = (1e100 * assign24870_e36776);
        (assign24870_e36777, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24870_e36726) - (assign24870_e36723 * (locals.var_nj0_dn0 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24870_e36745) - (assign24870_e36742 * (locals.var_nj0_dn0 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign24870_e36763) - (assign24870_e36760 * (locals.var_nj0_dn0 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24870_e36726) - (assign24870_e36723 * (locals.var_nj0_dn2 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24870_e36745) - (assign24870_e36742 * (locals.var_nj0_dn2 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign24870_e36763) - (assign24870_e36760 * (locals.var_nj0_dn2 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign24870_e36779;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign24870_e36779_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign24870_e36779_d_n2;

        let (assign24880_e36806, assign24880_e36806_d_n0, assign24880_e36806_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24880_e36790: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign24880_e36791: f64 = (locals.var_nj1 - assign24880_e36790);
        let assign24880_e36794: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign24880_e36795: f64 = (assign24880_e36791 / assign24880_e36794);
        let assign24880_e36798: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign24880_e36801: f64 = (locals.var_nj0 * p.p85);
        let assign24880_e36802: f64 = (assign24880_e36798 / assign24880_e36801);
        let assign24880_e36803: f64 = (assign24880_e36795 + assign24880_e36802);
        let assign24880_e36804: f64 = (locals.var_phitdinv * assign24880_e36803);
        (assign24880_e36804, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign24880_e36794) - (assign24880_e36791 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign24880_e36794 * assign24880_e36794)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign24880_e36801) - (assign24880_e36798 * (locals.var_nj0_dn0 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign24880_e36794) - (assign24880_e36791 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign24880_e36794 * assign24880_e36794)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign24880_e36801) - (assign24880_e36798 * (locals.var_nj0_dn2 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign24880_e36806;
        locals.var_dvmax_over_phitd_dv_dn0 = assign24880_e36806_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign24880_e36806_d_n2;

        let (assign24890_e36823, assign24890_e36823_d_n0, assign24890_e36823_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24890_e36816: f64 = (locals.var_v5 - locals.var_vmax);
        let assign24890_e36818: f64 = (assign24890_e36816 * locals.var_dvmax_over_phitd_dv);
        let assign24890_e36819: f64 = (1.0 + assign24890_e36818);
        let assign24890_e36821: f64 = (assign24890_e36819 * locals.var_exp_vmax_over_phitd_bot);
        (assign24890_e36821, (((assign24890_e36816 * locals.var_dvmax_over_phitd_dv_dn0) * locals.var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * locals.var_exp_vmax_over_phitd_bot_dn0)), (((assign24890_e36816 * locals.var_dvmax_over_phitd_dv_dn2) * locals.var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * locals.var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign24890_e36823;
        locals.var_idmultbot_dn0 = assign24890_e36823_d_n0;
        locals.var_idmultbot_dn2 = assign24890_e36823_d_n2;

        let (assign24900_e36836,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24900_e36832: f64 = (locals.var_nin * locals.var_nin);
        let assign24900_e36834: f64 = (assign24900_e36832 / locals.var_ndisti_i);
        (assign24900_e36834,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign24900_e36836;

        let (assign24910_e36852,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign24910_e36845: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign24910_e36848: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign24910_e36849: f64 = (assign24910_e36848).ln();
        let assign24910_e36850: f64 = (assign24910_e36845 * assign24910_e36849);
        (assign24910_e36850,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign24910_e36852;

        let assign24920_e36855: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign24920_e36855;

        let (assign24930_e36872, assign24930_e36872_d_n0, assign24930_e36872_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24930_e36867: f64 = (locals.var_vmax - locals.var_vha1);
        let assign24930_e36868: f64 = (p.p86 * assign24930_e36867);
        let assign24930_e36870: f64 = (assign24930_e36868 + locals.var_nfasti_i);
        (assign24930_e36870, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign24930_e36872;
        locals.var_nja10_dn0 = assign24930_e36872_d_n0;
        locals.var_nja10_dn2 = assign24930_e36872_d_n2;

        let (assign24940_e36887, assign24940_e36887_d_n0, assign24940_e36887_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24940_e36884: f64 = (p.p86 * locals.var_vha1);
        let assign24940_e36885: f64 = (locals.var_nfasti_i - assign24940_e36884);
        (assign24940_e36885, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign24940_e36887;
        locals.var_nj0_dn0 = assign24940_e36887_d_n0;
        locals.var_nj0_dn2 = assign24940_e36887_d_n2;

        let (assign24950_e36902, assign24950_e36902_d_n0, assign24950_e36902_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24950_e36898: f64 = (p.p85 - locals.var_nja10);
        let assign24950_e36900: f64 = (assign24950_e36898 - 0.01);
        (assign24950_e36900, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign24950_e36902;
        locals.var_tmf1_dn0 = assign24950_e36902_d_n0;
        locals.var_tmf1_dn2 = assign24950_e36902_d_n2;

        let (assign24960_e36917, assign24960_e36917_d_n0, assign24960_e36917_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24960_e36913: f64 = (4.0 * p.p85);
        let assign24960_e36915: f64 = (assign24960_e36913 * 0.01);
        (assign24960_e36915, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24960_e36917;
        locals.var_tmf2_dn0 = assign24960_e36917_d_n0;
        locals.var_tmf2_dn2 = assign24960_e36917_d_n2;

        let (assign24970_e36934, assign24970_e36934_d_n0, assign24970_e36934_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign24970_e36931: f64 = (-locals.var_tmf2);
                (assign24970_e36931, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24970_e36934;
        locals.var_tmf2_dn0 = assign24970_e36934_d_n0;
        locals.var_tmf2_dn2 = assign24970_e36934_d_n2;

        let (assign24980_e36950, assign24980_e36950_d_n0, assign24980_e36950_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24980_e36945: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign24980_e36947: f64 = (assign24980_e36945 + locals.var_tmf2);
        let assign24980_e36948: f64 = (assign24980_e36947).sqrt();
        (assign24980_e36948, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign24980_e36948)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign24980_e36948)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign24980_e36950;
        locals.var_tmf2_dn0 = assign24980_e36950_d_n0;
        locals.var_tmf2_dn2 = assign24980_e36950_d_n2;

        let (assign24990_e36967, assign24990_e36967_d_n0, assign24990_e36967_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign24990_e36963: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign24990_e36964: f64 = (1.0 + assign24990_e36963);
        let assign24990_e36965: f64 = (0.5 * assign24990_e36964);
        (assign24990_e36965, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign24990_e36967;
        locals.var_dfn_su_dn0 = assign24990_e36967_d_n0;
        locals.var_dfn_su_dn2 = assign24990_e36967_d_n2;

        let (assign25000_e36984, assign25000_e36984_d_n0, assign25000_e36984_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25000_e36980: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25000_e36981: f64 = (0.5 * assign25000_e36980);
        let assign25000_e36982: f64 = (p.p85 - assign25000_e36981);
        (assign25000_e36982, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign25000_e36984;
        locals.var_nja11_dn0 = assign25000_e36984_d_n0;
        locals.var_nja11_dn2 = assign25000_e36984_d_n2;

        let (assign25010_e36999, assign25010_e36999_d_n0, assign25010_e36999_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25010_e36995: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign25010_e36997: f64 = (assign25010_e36995 - 0.01);
        (assign25010_e36997, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25010_e36999;
        locals.var_tmf1_dn0 = assign25010_e36999_d_n0;
        locals.var_tmf1_dn2 = assign25010_e36999_d_n2;

        let (assign25020_e37014, assign25020_e37014_d_n0, assign25020_e37014_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25020_e37010: f64 = (4.0 * locals.var_nfasti_i);
        let assign25020_e37012: f64 = (assign25020_e37010 * 0.01);
        (assign25020_e37012, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25020_e37014;
        locals.var_tmf2_dn0 = assign25020_e37014_d_n0;
        locals.var_tmf2_dn2 = assign25020_e37014_d_n2;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25030_e37031, assign25030_e37031_d_n0, assign25030_e37031_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25030_e37028: f64 = (-locals.var_tmf2);
                (assign25030_e37028, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25030_e37031;
        locals.var_tmf2_dn0 = assign25030_e37031_d_n0;
        locals.var_tmf2_dn2 = assign25030_e37031_d_n2;

        let (assign25040_e37047, assign25040_e37047_d_n0, assign25040_e37047_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25040_e37042: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25040_e37044: f64 = (assign25040_e37042 + locals.var_tmf2);
        let assign25040_e37045: f64 = (assign25040_e37044).sqrt();
        (assign25040_e37045, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25040_e37045)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25040_e37045)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25040_e37047;
        locals.var_tmf2_dn0 = assign25040_e37047_d_n0;
        locals.var_tmf2_dn2 = assign25040_e37047_d_n2;

        let (assign25050_e37064, assign25050_e37064_d_n0, assign25050_e37064_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25050_e37060: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25050_e37061: f64 = (1.0 + assign25050_e37060);
        let assign25050_e37062: f64 = (0.5 * assign25050_e37061);
        (assign25050_e37062, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign25050_e37064;
        locals.var_dfn_sl_dn0 = assign25050_e37064_d_n0;
        locals.var_dfn_sl_dn2 = assign25050_e37064_d_n2;

        let (assign25060_e37081, assign25060_e37081_d_n0, assign25060_e37081_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25060_e37077: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25060_e37078: f64 = (0.5 * assign25060_e37077);
        let assign25060_e37079: f64 = (locals.var_nfasti_i + assign25060_e37078);
        (assign25060_e37079, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign25060_e37081;
        locals.var_nj1_dn0 = assign25060_e37081_d_n0;
        locals.var_nj1_dn2 = assign25060_e37081_d_n2;

        let (assign25070_e37096, assign25070_e37096_d_n0, assign25070_e37096_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25070_e37092: f64 = (p.p85 - locals.var_nj0);
        let assign25070_e37094: f64 = (assign25070_e37092 - 0.01);
        (assign25070_e37094, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25070_e37096;
        locals.var_tmf1_dn0 = assign25070_e37096_d_n0;
        locals.var_tmf1_dn2 = assign25070_e37096_d_n2;

        let (assign25080_e37111, assign25080_e37111_d_n0, assign25080_e37111_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25080_e37107: f64 = (4.0 * p.p85);
        let assign25080_e37109: f64 = (assign25080_e37107 * 0.01);
        (assign25080_e37109, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25080_e37111;
        locals.var_tmf2_dn0 = assign25080_e37111_d_n0;
        locals.var_tmf2_dn2 = assign25080_e37111_d_n2;

        let (assign25090_e37128, assign25090_e37128_d_n0, assign25090_e37128_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25090_e37125: f64 = (-locals.var_tmf2);
                (assign25090_e37125, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25090_e37128;
        locals.var_tmf2_dn0 = assign25090_e37128_d_n0;
        locals.var_tmf2_dn2 = assign25090_e37128_d_n2;

        let (assign25100_e37144, assign25100_e37144_d_n0, assign25100_e37144_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25100_e37139: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25100_e37141: f64 = (assign25100_e37139 + locals.var_tmf2);
        let assign25100_e37142: f64 = (assign25100_e37141).sqrt();
        (assign25100_e37142, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25100_e37142)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25100_e37142)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25100_e37144;
        locals.var_tmf2_dn0 = assign25100_e37144_d_n0;
        locals.var_tmf2_dn2 = assign25100_e37144_d_n2;

        let (assign25110_e37161, assign25110_e37161_d_n0, assign25110_e37161_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25110_e37157: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25110_e37158: f64 = (0.5 * assign25110_e37157);
        let assign25110_e37159: f64 = (p.p85 - assign25110_e37158);
        (assign25110_e37159, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25110_e37161;
        locals.var_nj0_dn0 = assign25110_e37161_d_n0;
        locals.var_nj0_dn2 = assign25110_e37161_d_n2;

        let (assign25120_e37176, assign25120_e37176_d_n0, assign25120_e37176_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25120_e37172: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign25120_e37174: f64 = (assign25120_e37172 - 0.01);
        (assign25120_e37174, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25120_e37176;
        locals.var_tmf1_dn0 = assign25120_e37176_d_n0;
        locals.var_tmf1_dn2 = assign25120_e37176_d_n2;

        let (assign25130_e37191, assign25130_e37191_d_n0, assign25130_e37191_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25130_e37187: f64 = (4.0 * locals.var_nfasti_i);
        let assign25130_e37189: f64 = (assign25130_e37187 * 0.01);
        (assign25130_e37189, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25130_e37191;
        locals.var_tmf2_dn0 = assign25130_e37191_d_n0;
        locals.var_tmf2_dn2 = assign25130_e37191_d_n2;

        let (assign25140_e37208, assign25140_e37208_d_n0, assign25140_e37208_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25140_e37205: f64 = (-locals.var_tmf2);
                (assign25140_e37205, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25140_e37208;
        locals.var_tmf2_dn0 = assign25140_e37208_d_n0;
        locals.var_tmf2_dn2 = assign25140_e37208_d_n2;

        let (assign25150_e37224, assign25150_e37224_d_n0, assign25150_e37224_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25150_e37219: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25150_e37221: f64 = (assign25150_e37219 + locals.var_tmf2);
        let assign25150_e37222: f64 = (assign25150_e37221).sqrt();
        (assign25150_e37222, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25150_e37222)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25150_e37222)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25150_e37224;
        locals.var_tmf2_dn0 = assign25150_e37224_d_n0;
        locals.var_tmf2_dn2 = assign25150_e37224_d_n2;

        let (assign25160_e37241, assign25160_e37241_d_n0, assign25160_e37241_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25160_e37237: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25160_e37238: f64 = (0.5 * assign25160_e37237);
        let assign25160_e37239: f64 = (locals.var_nfasti_i + assign25160_e37238);
        (assign25160_e37239, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25160_e37241;
        locals.var_nj0_dn0 = assign25160_e37241_d_n0;
        locals.var_nj0_dn2 = assign25160_e37241_d_n2;

        let (assign25170_e37256, assign25170_e37256_d_n0, assign25170_e37256_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign25170_e37252: f64 = (p.p86 * locals.var_dfn_su);
        let assign25170_e37254: f64 = (assign25170_e37252 * locals.var_dfn_sl);
        (assign25170_e37254, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign25170_e37252 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign25170_e37252 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign25170_e37256;
        locals.var_dnj1_dv_dn0 = assign25170_e37256_d_n0;
        locals.var_dnj1_dv_dn2 = assign25170_e37256_d_n2;

        let (assign25180_e37268, assign25180_e37268_d_n0, assign25180_e37268_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25180_e37268;
        locals.var_nj0_dn0 = assign25180_e37268_d_n0;
        locals.var_nj0_dn2 = assign25180_e37268_d_n2;

        let (assign25190_e37280, assign25190_e37280_d_n0, assign25190_e37280_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign25190_e37280;
        locals.var_nj1_dn0 = assign25190_e37280_d_n0;
        locals.var_nj1_dn2 = assign25190_e37280_d_n2;

        let (assign25200_e37292, assign25200_e37292_d_n0, assign25200_e37292_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard400 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign25200_e37292;
        locals.var_dnj1_dv_dn0 = assign25200_e37292_d_n0;
        locals.var_dnj1_dv_dn2 = assign25200_e37292_d_n2;

        let (assign25260_e37541, assign25260_e37541_d_n0, assign25260_e37541_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign25260_e37525: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign25260_e37526: f64 = (locals.var_nj1 - assign25260_e37525);
        let assign25260_e37529: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign25260_e37530: f64 = (assign25260_e37526 / assign25260_e37529);
        let assign25260_e37533: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign25260_e37536: f64 = (locals.var_nj0 * p.p85);
        let assign25260_e37537: f64 = (assign25260_e37533 / assign25260_e37536);
        let assign25260_e37538: f64 = (assign25260_e37530 + assign25260_e37537);
        let assign25260_e37539: f64 = (locals.var_phitdinv * assign25260_e37538);
        (assign25260_e37539, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign25260_e37529) - (assign25260_e37526 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign25260_e37529 * assign25260_e37529)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign25260_e37536) - (assign25260_e37533 * (locals.var_nj0_dn0 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign25260_e37529) - (assign25260_e37526 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign25260_e37529 * assign25260_e37529)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign25260_e37536) - (assign25260_e37533 * (locals.var_nj0_dn2 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign25260_e37541;
        locals.var_dvmax_over_phitd_dv_dn0 = assign25260_e37541_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign25260_e37541_d_n2;

        let (assign25280_e37571,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign25280_e37567: f64 = (locals.var_nin * locals.var_nin);
        let assign25280_e37569: f64 = (assign25280_e37567 / locals.var_ndigat_i);
        (assign25280_e37569,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign25280_e37571;

        let (assign25290_e37587,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign25290_e37580: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign25290_e37583: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign25290_e37584: f64 = (assign25290_e37583).ln();
        let assign25290_e37585: f64 = (assign25290_e37580 * assign25290_e37584);
        (assign25290_e37585,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign25290_e37587;

        let assign25300_e37590: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign25300_e37590;

        let (assign25310_e37607, assign25310_e37607_d_n0, assign25310_e37607_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25310_e37602: f64 = (locals.var_vmax - locals.var_vha1);
        let assign25310_e37603: f64 = (p.p86 * assign25310_e37602);
        let assign25310_e37605: f64 = (assign25310_e37603 + locals.var_nfagat_i);
        (assign25310_e37605, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign25310_e37607;
        locals.var_nja10_dn0 = assign25310_e37607_d_n0;
        locals.var_nja10_dn2 = assign25310_e37607_d_n2;

        let (assign25320_e37622, assign25320_e37622_d_n0, assign25320_e37622_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25320_e37619: f64 = (p.p86 * locals.var_vha1);
        let assign25320_e37620: f64 = (locals.var_nfagat_i - assign25320_e37619);
        (assign25320_e37620, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25320_e37622;
        locals.var_nj0_dn0 = assign25320_e37622_d_n0;
        locals.var_nj0_dn2 = assign25320_e37622_d_n2;

        let (assign25330_e37637, assign25330_e37637_d_n0, assign25330_e37637_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25330_e37633: f64 = (p.p85 - locals.var_nja10);
        let assign25330_e37635: f64 = (assign25330_e37633 - 0.01);
        (assign25330_e37635, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25330_e37637;
        locals.var_tmf1_dn0 = assign25330_e37637_d_n0;
        locals.var_tmf1_dn2 = assign25330_e37637_d_n2;

        let (assign25340_e37652, assign25340_e37652_d_n0, assign25340_e37652_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25340_e37648: f64 = (4.0 * p.p85);
        let assign25340_e37650: f64 = (assign25340_e37648 * 0.01);
        (assign25340_e37650, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25340_e37652;
        locals.var_tmf2_dn0 = assign25340_e37652_d_n0;
        locals.var_tmf2_dn2 = assign25340_e37652_d_n2;

        let (assign25350_e37669, assign25350_e37669_d_n0, assign25350_e37669_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25350_e37666: f64 = (-locals.var_tmf2);
                (assign25350_e37666, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25350_e37669;
        locals.var_tmf2_dn0 = assign25350_e37669_d_n0;
        locals.var_tmf2_dn2 = assign25350_e37669_d_n2;

        let (assign25360_e37685, assign25360_e37685_d_n0, assign25360_e37685_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25360_e37680: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25360_e37682: f64 = (assign25360_e37680 + locals.var_tmf2);
        let assign25360_e37683: f64 = (assign25360_e37682).sqrt();
        (assign25360_e37683, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25360_e37683)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25360_e37683)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25360_e37685;
        locals.var_tmf2_dn0 = assign25360_e37685_d_n0;
        locals.var_tmf2_dn2 = assign25360_e37685_d_n2;

        let (assign25370_e37702, assign25370_e37702_d_n0, assign25370_e37702_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25370_e37698: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25370_e37699: f64 = (1.0 + assign25370_e37698);
        let assign25370_e37700: f64 = (0.5 * assign25370_e37699);
        (assign25370_e37700, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign25370_e37702;
        locals.var_dfn_su_dn0 = assign25370_e37702_d_n0;
        locals.var_dfn_su_dn2 = assign25370_e37702_d_n2;

        let (assign25380_e37719, assign25380_e37719_d_n0, assign25380_e37719_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25380_e37715: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25380_e37716: f64 = (0.5 * assign25380_e37715);
        let assign25380_e37717: f64 = (p.p85 - assign25380_e37716);
        (assign25380_e37717, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign25380_e37719;
        locals.var_nja11_dn0 = assign25380_e37719_d_n0;
        locals.var_nja11_dn2 = assign25380_e37719_d_n2;

        let (assign25390_e37734, assign25390_e37734_d_n0, assign25390_e37734_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25390_e37730: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign25390_e37732: f64 = (assign25390_e37730 - 0.01);
        (assign25390_e37732, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25390_e37734;
        locals.var_tmf1_dn0 = assign25390_e37734_d_n0;
        locals.var_tmf1_dn2 = assign25390_e37734_d_n2;

        let (assign25400_e37749, assign25400_e37749_d_n0, assign25400_e37749_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25400_e37745: f64 = (4.0 * locals.var_nfagat_i);
        let assign25400_e37747: f64 = (assign25400_e37745 * 0.01);
        (assign25400_e37747, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25400_e37749;
        locals.var_tmf2_dn0 = assign25400_e37749_d_n0;
        locals.var_tmf2_dn2 = assign25400_e37749_d_n2;

        let (assign25410_e37766, assign25410_e37766_d_n0, assign25410_e37766_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25410_e37763: f64 = (-locals.var_tmf2);
                (assign25410_e37763, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25410_e37766;
        locals.var_tmf2_dn0 = assign25410_e37766_d_n0;
        locals.var_tmf2_dn2 = assign25410_e37766_d_n2;

        let (assign25420_e37782, assign25420_e37782_d_n0, assign25420_e37782_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25420_e37777: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25420_e37779: f64 = (assign25420_e37777 + locals.var_tmf2);
        let assign25420_e37780: f64 = (assign25420_e37779).sqrt();
        (assign25420_e37780, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25420_e37780)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25420_e37780)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25420_e37782;
        locals.var_tmf2_dn0 = assign25420_e37782_d_n0;
        locals.var_tmf2_dn2 = assign25420_e37782_d_n2;

        let (assign25430_e37799, assign25430_e37799_d_n0, assign25430_e37799_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25430_e37795: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25430_e37796: f64 = (1.0 + assign25430_e37795);
        let assign25430_e37797: f64 = (0.5 * assign25430_e37796);
        (assign25430_e37797, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign25430_e37799;
        locals.var_dfn_sl_dn0 = assign25430_e37799_d_n0;
        locals.var_dfn_sl_dn2 = assign25430_e37799_d_n2;

        let (assign25440_e37816, assign25440_e37816_d_n0, assign25440_e37816_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25440_e37812: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25440_e37813: f64 = (0.5 * assign25440_e37812);
        let assign25440_e37814: f64 = (locals.var_nfagat_i + assign25440_e37813);
        (assign25440_e37814, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign25440_e37816;
        locals.var_nj1_dn0 = assign25440_e37816_d_n0;
        locals.var_nj1_dn2 = assign25440_e37816_d_n2;

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (assign25450_e37831, assign25450_e37831_d_n0, assign25450_e37831_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25450_e37827: f64 = (p.p85 - locals.var_nj0);
        let assign25450_e37829: f64 = (assign25450_e37827 - 0.01);
        (assign25450_e37829, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25450_e37831;
        locals.var_tmf1_dn0 = assign25450_e37831_d_n0;
        locals.var_tmf1_dn2 = assign25450_e37831_d_n2;

        let (assign25460_e37846, assign25460_e37846_d_n0, assign25460_e37846_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25460_e37842: f64 = (4.0 * p.p85);
        let assign25460_e37844: f64 = (assign25460_e37842 * 0.01);
        (assign25460_e37844, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25460_e37846;
        locals.var_tmf2_dn0 = assign25460_e37846_d_n0;
        locals.var_tmf2_dn2 = assign25460_e37846_d_n2;

        let (assign25470_e37863, assign25470_e37863_d_n0, assign25470_e37863_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25470_e37860: f64 = (-locals.var_tmf2);
                (assign25470_e37860, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25470_e37863;
        locals.var_tmf2_dn0 = assign25470_e37863_d_n0;
        locals.var_tmf2_dn2 = assign25470_e37863_d_n2;

        let (assign25480_e37879, assign25480_e37879_d_n0, assign25480_e37879_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25480_e37874: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25480_e37876: f64 = (assign25480_e37874 + locals.var_tmf2);
        let assign25480_e37877: f64 = (assign25480_e37876).sqrt();
        (assign25480_e37877, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25480_e37877)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25480_e37877)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25480_e37879;
        locals.var_tmf2_dn0 = assign25480_e37879_d_n0;
        locals.var_tmf2_dn2 = assign25480_e37879_d_n2;

        let (assign25490_e37896, assign25490_e37896_d_n0, assign25490_e37896_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25490_e37892: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25490_e37893: f64 = (0.5 * assign25490_e37892);
        let assign25490_e37894: f64 = (p.p85 - assign25490_e37893);
        (assign25490_e37894, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25490_e37896;
        locals.var_nj0_dn0 = assign25490_e37896_d_n0;
        locals.var_nj0_dn2 = assign25490_e37896_d_n2;

        let (assign25500_e37911, assign25500_e37911_d_n0, assign25500_e37911_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25500_e37907: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign25500_e37909: f64 = (assign25500_e37907 - 0.01);
        (assign25500_e37909, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign25500_e37911;
        locals.var_tmf1_dn0 = assign25500_e37911_d_n0;
        locals.var_tmf1_dn2 = assign25500_e37911_d_n2;

        let (assign25510_e37926, assign25510_e37926_d_n0, assign25510_e37926_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25510_e37922: f64 = (4.0 * locals.var_nfagat_i);
        let assign25510_e37924: f64 = (assign25510_e37922 * 0.01);
        (assign25510_e37924, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25510_e37926;
        locals.var_tmf2_dn0 = assign25510_e37926_d_n0;
        locals.var_tmf2_dn2 = assign25510_e37926_d_n2;

        let (assign25520_e37943, assign25520_e37943_d_n0, assign25520_e37943_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign25520_e37940: f64 = (-locals.var_tmf2);
                (assign25520_e37940, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25520_e37943;
        locals.var_tmf2_dn0 = assign25520_e37943_d_n0;
        locals.var_tmf2_dn2 = assign25520_e37943_d_n2;

        let (assign25530_e37959, assign25530_e37959_d_n0, assign25530_e37959_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25530_e37954: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25530_e37956: f64 = (assign25530_e37954 + locals.var_tmf2);
        let assign25530_e37957: f64 = (assign25530_e37956).sqrt();
        (assign25530_e37957, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25530_e37957)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25530_e37957)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign25530_e37959;
        locals.var_tmf2_dn0 = assign25530_e37959_d_n0;
        locals.var_tmf2_dn2 = assign25530_e37959_d_n2;

        let (assign25540_e37976, assign25540_e37976_d_n0, assign25540_e37976_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25540_e37972: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25540_e37973: f64 = (0.5 * assign25540_e37972);
        let assign25540_e37974: f64 = (locals.var_nfagat_i + assign25540_e37973);
        (assign25540_e37974, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25540_e37976;
        locals.var_nj0_dn0 = assign25540_e37976_d_n0;
        locals.var_nj0_dn2 = assign25540_e37976_d_n2;

        let (assign25550_e37991, assign25550_e37991_d_n0, assign25550_e37991_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 != 0.0)) {
        let assign25550_e37987: f64 = (p.p86 * locals.var_dfn_su);
        let assign25550_e37989: f64 = (assign25550_e37987 * locals.var_dfn_sl);
        (assign25550_e37989, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign25550_e37987 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign25550_e37987 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign25550_e37991;
        locals.var_dnj1_dv_dn0 = assign25550_e37991_d_n0;
        locals.var_dnj1_dv_dn2 = assign25550_e37991_d_n2;

        let (assign25560_e38003, assign25560_e38003_d_n0, assign25560_e38003_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign25560_e38003;
        locals.var_nj0_dn0 = assign25560_e38003_d_n0;
        locals.var_nj0_dn2 = assign25560_e38003_d_n2;

        let (assign25570_e38015, assign25570_e38015_d_n0, assign25570_e38015_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign25570_e38015;
        locals.var_nj1_dn0 = assign25570_e38015_d_n0;
        locals.var_nj1_dn2 = assign25570_e38015_d_n2;

        let (assign25580_e38027, assign25580_e38027_d_n0, assign25580_e38027_d_n2,) = {
    if ((((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) && (locals.var_guard403 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign25580_e38027;
        locals.var_dnj1_dv_dn0 = assign25580_e38027_d_n0;
        locals.var_dnj1_dv_dn2 = assign25580_e38027_d_n2;

        let (assign25640_e38276, assign25640_e38276_d_n0, assign25640_e38276_d_n2,) = {
    if (((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign25640_e38260: f64 = (locals.var_vmax * locals.var_dnj1_dv);
        let assign25640_e38261: f64 = (locals.var_nj1 - assign25640_e38260);
        let assign25640_e38264: f64 = (locals.var_nj1 * locals.var_nj1);
        let assign25640_e38265: f64 = (assign25640_e38261 / assign25640_e38264);
        let assign25640_e38268: f64 = (locals.var_vha1 * locals.var_dnj1_dv);
        let assign25640_e38271: f64 = (locals.var_nj0 * p.p85);
        let assign25640_e38272: f64 = (assign25640_e38268 / assign25640_e38271);
        let assign25640_e38273: f64 = (assign25640_e38265 + assign25640_e38272);
        let assign25640_e38274: f64 = (locals.var_phitdinv * assign25640_e38273);
        (assign25640_e38274, (locals.var_phitdinv * (((((locals.var_nj1_dn0 - (locals.var_vmax * locals.var_dnj1_dv_dn0)) * assign25640_e38264) - (assign25640_e38261 * ((locals.var_nj1_dn0 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn0)))) / (assign25640_e38264 * assign25640_e38264)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn0) * assign25640_e38271) - (assign25640_e38268 * (locals.var_nj0_dn0 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (locals.var_phitdinv * (((((locals.var_nj1_dn2 - (locals.var_vmax * locals.var_dnj1_dv_dn2)) * assign25640_e38264) - (assign25640_e38261 * ((locals.var_nj1_dn2 * locals.var_nj1) + (locals.var_nj1 * locals.var_nj1_dn2)))) / (assign25640_e38264 * assign25640_e38264)) + ((((locals.var_vha1 * locals.var_dnj1_dv_dn2) * assign25640_e38271) - (assign25640_e38268 * (locals.var_nj0_dn2 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))),)
    } else {
        (locals.var_dvmax_over_phitd_dv, locals.var_dvmax_over_phitd_dv_dn0, locals.var_dvmax_over_phitd_dv_dn2,)
    }
};
        locals.var_dvmax_over_phitd_dv = assign25640_e38276;
        locals.var_dvmax_over_phitd_dv_dn0 = assign25640_e38276_d_n0;
        locals.var_dvmax_over_phitd_dv_dn2 = assign25640_e38276_d_n2;

        let (assign25660_e38301, assign25660_e38301_d_n0, assign25660_e38301_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard384 != 0.0)) {
        let assign25660_e38299: f64 = (locals.var_idmultbot - 1.0);
        (assign25660_e38299, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign25660_e38301;
        locals.var_idmultbot_dn0 = assign25660_e38301_d_n0;
        locals.var_idmultbot_dn2 = assign25660_e38301_d_n2;

        let (assign25770_e38474, assign25770_e38474_d_n0, assign25770_e38474_d_n2,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_guard384 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign25770_e38474;
        locals.var_idmultbot_dn0 = assign25770_e38474_d_n0;
        locals.var_idmultbot_dn2 = assign25770_e38474_d_n2;

        locals.var_vak = (nv0 - nv2);
        locals.var_vak_dn0 = 1.0;
        locals.var_vak_dn2 = -1.0;

        let assign28760_e42618: f64 = if locals.var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign28760_e42618;

        let assign29220_e43059: f64 = if (!(((locals.var_ab_i == 0.0) && (locals.var_ls_i == 0.0)) && (locals.var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard479 = assign29220_e43059;

        let assign29300_e43138: f64 = if locals.var_vak < locals.var_vmax { 1.0 } else { 0.0 };
        locals.var_guard480 = assign29300_e43138;

        let (assign29360_e43283,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29360_e43279: f64 = (locals.var_nin * locals.var_nin);
        let assign29360_e43281: f64 = (assign29360_e43279 / locals.var_ndibot_i);
        (assign29360_e43281,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign29360_e43283;

        let (assign29370_e43299,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29370_e43292: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign29370_e43295: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign29370_e43296: f64 = (assign29370_e43295).ln();
        let assign29370_e43297: f64 = (assign29370_e43292 * assign29370_e43296);
        (assign29370_e43297,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign29370_e43299;

        let assign29380_e43302: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard483 = assign29380_e43302;

        let (assign29390_e43319, assign29390_e43319_d_n0, assign29390_e43319_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29390_e43314: f64 = (locals.var_vak - locals.var_vha1);
        let assign29390_e43315: f64 = (p.p86 * assign29390_e43314);
        let assign29390_e43317: f64 = (assign29390_e43315 + locals.var_nfabot_i);
        (assign29390_e43317, (p.p86 * locals.var_vak_dn0), (p.p86 * locals.var_vak_dn2),)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign29390_e43319;
        locals.var_nja10_dn0 = assign29390_e43319_d_n0;
        locals.var_nja10_dn2 = assign29390_e43319_d_n2;

        let (assign29400_e43334, assign29400_e43334_d_n0, assign29400_e43334_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29400_e43331: f64 = (p.p86 * locals.var_vha1);
        let assign29400_e43332: f64 = (locals.var_nfabot_i - assign29400_e43331);
        (assign29400_e43332, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29400_e43334;
        locals.var_nj0_dn0 = assign29400_e43334_d_n0;
        locals.var_nj0_dn2 = assign29400_e43334_d_n2;

        let (assign29410_e43349, assign29410_e43349_d_n0, assign29410_e43349_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29410_e43345: f64 = (p.p85 - locals.var_nja10);
        let assign29410_e43347: f64 = (assign29410_e43345 - 0.01);
        (assign29410_e43347, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29410_e43349;
        locals.var_tmf1_dn0 = assign29410_e43349_d_n0;
        locals.var_tmf1_dn2 = assign29410_e43349_d_n2;

        let (assign29420_e43364, assign29420_e43364_d_n0, assign29420_e43364_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29420_e43360: f64 = (4.0 * p.p85);
        let assign29420_e43362: f64 = (assign29420_e43360 * 0.01);
        (assign29420_e43362, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29420_e43364;
        locals.var_tmf2_dn0 = assign29420_e43364_d_n0;
        locals.var_tmf2_dn2 = assign29420_e43364_d_n2;

        let (assign29430_e43381, assign29430_e43381_d_n0, assign29430_e43381_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29430_e43378: f64 = (-locals.var_tmf2);
                (assign29430_e43378, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29430_e43381;
        locals.var_tmf2_dn0 = assign29430_e43381_d_n0;
        locals.var_tmf2_dn2 = assign29430_e43381_d_n2;

        let (assign29440_e43397, assign29440_e43397_d_n0, assign29440_e43397_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29440_e43392: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29440_e43394: f64 = (assign29440_e43392 + locals.var_tmf2);
        let assign29440_e43395: f64 = (assign29440_e43394).sqrt();
        (assign29440_e43395, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29440_e43395)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29440_e43395)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29440_e43397;
        locals.var_tmf2_dn0 = assign29440_e43397_d_n0;
        locals.var_tmf2_dn2 = assign29440_e43397_d_n2;

        let (assign29450_e43414, assign29450_e43414_d_n0, assign29450_e43414_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29450_e43410: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29450_e43411: f64 = (0.5 * assign29450_e43410);
        let assign29450_e43412: f64 = (p.p85 - assign29450_e43411);
        (assign29450_e43412, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign29450_e43414;
        locals.var_nja11_dn0 = assign29450_e43414_d_n0;
        locals.var_nja11_dn2 = assign29450_e43414_d_n2;

        let (assign29460_e43429, assign29460_e43429_d_n0, assign29460_e43429_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29460_e43425: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign29460_e43427: f64 = (assign29460_e43425 - 0.01);
        (assign29460_e43427, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29460_e43429;
        locals.var_tmf1_dn0 = assign29460_e43429_d_n0;
        locals.var_tmf1_dn2 = assign29460_e43429_d_n2;

        let (assign29470_e43444, assign29470_e43444_d_n0, assign29470_e43444_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29470_e43440: f64 = (4.0 * locals.var_nfabot_i);
        let assign29470_e43442: f64 = (assign29470_e43440 * 0.01);
        (assign29470_e43442, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29470_e43444;
        locals.var_tmf2_dn0 = assign29470_e43444_d_n0;
        locals.var_tmf2_dn2 = assign29470_e43444_d_n2;

        let (assign29480_e43461, assign29480_e43461_d_n0, assign29480_e43461_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29480_e43458: f64 = (-locals.var_tmf2);
                (assign29480_e43458, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29480_e43461;
        locals.var_tmf2_dn0 = assign29480_e43461_d_n0;
        locals.var_tmf2_dn2 = assign29480_e43461_d_n2;

        let (assign29490_e43477, assign29490_e43477_d_n0, assign29490_e43477_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29490_e43472: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29490_e43474: f64 = (assign29490_e43472 + locals.var_tmf2);
        let assign29490_e43475: f64 = (assign29490_e43474).sqrt();
        (assign29490_e43475, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29490_e43475)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29490_e43475)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29490_e43477;
        locals.var_tmf2_dn0 = assign29490_e43477_d_n0;
        locals.var_tmf2_dn2 = assign29490_e43477_d_n2;

        let (assign29500_e43494, assign29500_e43494_d_n0, assign29500_e43494_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29500_e43490: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29500_e43491: f64 = (0.5 * assign29500_e43490);
        let assign29500_e43492: f64 = (locals.var_nfabot_i + assign29500_e43491);
        (assign29500_e43492, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign29500_e43494;
        locals.var_nj1_dn0 = assign29500_e43494_d_n0;
        locals.var_nj1_dn2 = assign29500_e43494_d_n2;

        let (assign29510_e43509, assign29510_e43509_d_n0, assign29510_e43509_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29510_e43505: f64 = (p.p85 - locals.var_nj0);
        let assign29510_e43507: f64 = (assign29510_e43505 - 0.01);
        (assign29510_e43507, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29510_e43509;
        locals.var_tmf1_dn0 = assign29510_e43509_d_n0;
        locals.var_tmf1_dn2 = assign29510_e43509_d_n2;

        let (assign29520_e43524, assign29520_e43524_d_n0, assign29520_e43524_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29520_e43520: f64 = (4.0 * p.p85);
        let assign29520_e43522: f64 = (assign29520_e43520 * 0.01);
        (assign29520_e43522, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29520_e43524;
        locals.var_tmf2_dn0 = assign29520_e43524_d_n0;
        locals.var_tmf2_dn2 = assign29520_e43524_d_n2;

        let (assign29530_e43541, assign29530_e43541_d_n0, assign29530_e43541_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29530_e43538: f64 = (-locals.var_tmf2);
                (assign29530_e43538, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29530_e43541;
        locals.var_tmf2_dn0 = assign29530_e43541_d_n0;
        locals.var_tmf2_dn2 = assign29530_e43541_d_n2;

        let (assign29540_e43557, assign29540_e43557_d_n0, assign29540_e43557_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29540_e43552: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29540_e43554: f64 = (assign29540_e43552 + locals.var_tmf2);
        let assign29540_e43555: f64 = (assign29540_e43554).sqrt();
        (assign29540_e43555, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29540_e43555)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29540_e43555)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29540_e43557;
        locals.var_tmf2_dn0 = assign29540_e43557_d_n0;
        locals.var_tmf2_dn2 = assign29540_e43557_d_n2;

    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29550_e43574, assign29550_e43574_d_n0, assign29550_e43574_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29550_e43570: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29550_e43571: f64 = (0.5 * assign29550_e43570);
        let assign29550_e43572: f64 = (p.p85 - assign29550_e43571);
        (assign29550_e43572, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29550_e43574;
        locals.var_nj0_dn0 = assign29550_e43574_d_n0;
        locals.var_nj0_dn2 = assign29550_e43574_d_n2;

        let (assign29560_e43589, assign29560_e43589_d_n0, assign29560_e43589_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29560_e43585: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign29560_e43587: f64 = (assign29560_e43585 - 0.01);
        (assign29560_e43587, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29560_e43589;
        locals.var_tmf1_dn0 = assign29560_e43589_d_n0;
        locals.var_tmf1_dn2 = assign29560_e43589_d_n2;

        let (assign29570_e43604, assign29570_e43604_d_n0, assign29570_e43604_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29570_e43600: f64 = (4.0 * locals.var_nfabot_i);
        let assign29570_e43602: f64 = (assign29570_e43600 * 0.01);
        (assign29570_e43602, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29570_e43604;
        locals.var_tmf2_dn0 = assign29570_e43604_d_n0;
        locals.var_tmf2_dn2 = assign29570_e43604_d_n2;

        let (assign29580_e43621, assign29580_e43621_d_n0, assign29580_e43621_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29580_e43618: f64 = (-locals.var_tmf2);
                (assign29580_e43618, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29580_e43621;
        locals.var_tmf2_dn0 = assign29580_e43621_d_n0;
        locals.var_tmf2_dn2 = assign29580_e43621_d_n2;

        let (assign29590_e43637, assign29590_e43637_d_n0, assign29590_e43637_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29590_e43632: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29590_e43634: f64 = (assign29590_e43632 + locals.var_tmf2);
        let assign29590_e43635: f64 = (assign29590_e43634).sqrt();
        (assign29590_e43635, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29590_e43635)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29590_e43635)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29590_e43637;
        locals.var_tmf2_dn0 = assign29590_e43637_d_n0;
        locals.var_tmf2_dn2 = assign29590_e43637_d_n2;

        let (assign29600_e43654, assign29600_e43654_d_n0, assign29600_e43654_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29600_e43650: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29600_e43651: f64 = (0.5 * assign29600_e43650);
        let assign29600_e43652: f64 = (locals.var_nfabot_i + assign29600_e43651);
        (assign29600_e43652, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29600_e43654;
        locals.var_nj0_dn0 = assign29600_e43654_d_n0;
        locals.var_nj0_dn2 = assign29600_e43654_d_n2;

        let (assign29610_e43666, assign29610_e43666_d_n0, assign29610_e43666_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29610_e43666;
        locals.var_nj0_dn0 = assign29610_e43666_d_n0;
        locals.var_nj0_dn2 = assign29610_e43666_d_n2;

        let (assign29620_e43678, assign29620_e43678_d_n0, assign29620_e43678_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard483 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign29620_e43678;
        locals.var_nj1_dn0 = assign29620_e43678_d_n0;
        locals.var_nj1_dn2 = assign29620_e43678_d_n2;

        let assign29630_e43682: f64 = (locals.var_vak / locals.var_nj1);
        let assign29630_e43686: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29630_e43687: f64 = (locals.var_vha1 * assign29630_e43686);
        let assign29630_e43690: f64 = (locals.var_nj0 * p.p85);
        let assign29630_e43691: f64 = (assign29630_e43687 / assign29630_e43690);
        let assign29630_e43692: f64 = (assign29630_e43682 + assign29630_e43691);
        let assign29630_e43693: f64 = (locals.var_phitdinv * assign29630_e43692);
        let assign29630_e43694: f64 = (assign29630_e43693).abs();
        let assign29630_e43696: f64 = if assign29630_e43694 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign29630_e43696;

        let (assign29640_e43722, assign29640_e43722_d_n0, assign29640_e43722_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign29640_e43708: f64 = (locals.var_vak / locals.var_nj1);
        let assign29640_e43712: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29640_e43713: f64 = (locals.var_vha1 * assign29640_e43712);
        let assign29640_e43716: f64 = (locals.var_nj0 * p.p85);
        let assign29640_e43717: f64 = (assign29640_e43713 / assign29640_e43716);
        let assign29640_e43718: f64 = (assign29640_e43708 + assign29640_e43717);
        let assign29640_e43719: f64 = (locals.var_phitdinv * assign29640_e43718);
        let assign29640_e43720: f64 = (assign29640_e43719).exp();
        (assign29640_e43720, (assign29640_e43720 * (locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29640_e43716) - (assign29640_e43713 * (locals.var_nj0_dn0 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29640_e43716) - (assign29640_e43713 * (locals.var_nj0_dn2 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign29640_e43722;
        locals.var_idmultbot_dn0 = assign29640_e43722_d_n0;
        locals.var_idmultbot_dn2 = assign29640_e43722_d_n2;

        let assign29650_e43726: f64 = (locals.var_vak / locals.var_nj1);
        let assign29650_e43730: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29650_e43731: f64 = (locals.var_vha1 * assign29650_e43730);
        let assign29650_e43734: f64 = (locals.var_nj0 * p.p85);
        let assign29650_e43735: f64 = (assign29650_e43731 / assign29650_e43734);
        let assign29650_e43736: f64 = (assign29650_e43726 + assign29650_e43735);
        let assign29650_e43737: f64 = (locals.var_phitdinv * assign29650_e43736);
        let assign29650_e43739: f64 = (-230.25850929940458);
        let assign29650_e43740: f64 = if assign29650_e43737 < assign29650_e43739 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign29650_e43740;

        let (assign29660_e43821, assign29660_e43821_d_n0, assign29660_e43821_d_n2,) = {
    if (((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign29660_e43755: f64 = (-230.25850929940458);
        let assign29660_e43759: f64 = (locals.var_vak / locals.var_nj1);
        let assign29660_e43763: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29660_e43764: f64 = (locals.var_vha1 * assign29660_e43763);
        let assign29660_e43767: f64 = (locals.var_nj0 * p.p85);
        let assign29660_e43768: f64 = (assign29660_e43764 / assign29660_e43767);
        let assign29660_e43769: f64 = (assign29660_e43759 + assign29660_e43768);
        let assign29660_e43770: f64 = (locals.var_phitdinv * assign29660_e43769);
        let assign29660_e43771: f64 = (assign29660_e43755 - assign29660_e43770);
        let assign29660_e43775: f64 = (-230.25850929940458);
        let assign29660_e43779: f64 = (locals.var_vak / locals.var_nj1);
        let assign29660_e43783: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29660_e43784: f64 = (locals.var_vha1 * assign29660_e43783);
        let assign29660_e43787: f64 = (locals.var_nj0 * p.p85);
        let assign29660_e43788: f64 = (assign29660_e43784 / assign29660_e43787);
        let assign29660_e43789: f64 = (assign29660_e43779 + assign29660_e43788);
        let assign29660_e43790: f64 = (locals.var_phitdinv * assign29660_e43789);
        let assign29660_e43791: f64 = (assign29660_e43775 - assign29660_e43790);
        let assign29660_e43794: f64 = (-230.25850929940458);
        let assign29660_e43798: f64 = (locals.var_vak / locals.var_nj1);
        let assign29660_e43802: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29660_e43803: f64 = (locals.var_vha1 * assign29660_e43802);
        let assign29660_e43806: f64 = (locals.var_nj0 * p.p85);
        let assign29660_e43807: f64 = (assign29660_e43803 / assign29660_e43806);
        let assign29660_e43808: f64 = (assign29660_e43798 + assign29660_e43807);
        let assign29660_e43809: f64 = (locals.var_phitdinv * assign29660_e43808);
        let assign29660_e43810: f64 = (assign29660_e43794 - assign29660_e43809);
        let assign29660_e43812: f64 = (assign29660_e43810 * 0.3333333333333333);
        let assign29660_e43813: f64 = (1.0 + assign29660_e43812);
        let assign29660_e43814: f64 = (assign29660_e43791 * assign29660_e43813);
        let assign29660_e43815: f64 = (0.5 * assign29660_e43814);
        let assign29660_e43816: f64 = (1.0 + assign29660_e43815);
        let assign29660_e43817: f64 = (assign29660_e43771 * assign29660_e43816);
        let assign29660_e43818: f64 = (1.0 + assign29660_e43817);
        let assign29660_e43819: f64 = (1e-100 / assign29660_e43818);
        (assign29660_e43819, (-((1e-100 * (((-(locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29660_e43767) - (assign29660_e43764 * (locals.var_nj0_dn0 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29660_e43787) - (assign29660_e43784 * (locals.var_nj0_dn0 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29660_e43806) - (assign29660_e43803 * (locals.var_nj0_dn0 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29660_e43767) - (assign29660_e43764 * (locals.var_nj0_dn2 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29660_e43787) - (assign29660_e43784 * (locals.var_nj0_dn2 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29660_e43806) - (assign29660_e43803 * (locals.var_nj0_dn2 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign29660_e43821;
        locals.var_idmultbot_dn0 = assign29660_e43821_d_n0;
        locals.var_idmultbot_dn2 = assign29660_e43821_d_n2;

        let (assign29670_e43900, assign29670_e43900_d_n0, assign29670_e43900_d_n2,) = {
    if (((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard484 == 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign29670_e43839: f64 = (locals.var_vak / locals.var_nj1);
        let assign29670_e43843: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29670_e43844: f64 = (locals.var_vha1 * assign29670_e43843);
        let assign29670_e43847: f64 = (locals.var_nj0 * p.p85);
        let assign29670_e43848: f64 = (assign29670_e43844 / assign29670_e43847);
        let assign29670_e43849: f64 = (assign29670_e43839 + assign29670_e43848);
        let assign29670_e43850: f64 = (locals.var_phitdinv * assign29670_e43849);
        let assign29670_e43852: f64 = (assign29670_e43850 - 230.25850929940458);
        let assign29670_e43858: f64 = (locals.var_vak / locals.var_nj1);
        let assign29670_e43862: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29670_e43863: f64 = (locals.var_vha1 * assign29670_e43862);
        let assign29670_e43866: f64 = (locals.var_nj0 * p.p85);
        let assign29670_e43867: f64 = (assign29670_e43863 / assign29670_e43866);
        let assign29670_e43868: f64 = (assign29670_e43858 + assign29670_e43867);
        let assign29670_e43869: f64 = (locals.var_phitdinv * assign29670_e43868);
        let assign29670_e43871: f64 = (assign29670_e43869 - 230.25850929940458);
        let assign29670_e43876: f64 = (locals.var_vak / locals.var_nj1);
        let assign29670_e43880: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign29670_e43881: f64 = (locals.var_vha1 * assign29670_e43880);
        let assign29670_e43884: f64 = (locals.var_nj0 * p.p85);
        let assign29670_e43885: f64 = (assign29670_e43881 / assign29670_e43884);
        let assign29670_e43886: f64 = (assign29670_e43876 + assign29670_e43885);
        let assign29670_e43887: f64 = (locals.var_phitdinv * assign29670_e43886);
        let assign29670_e43889: f64 = (assign29670_e43887 - 230.25850929940458);
        let assign29670_e43891: f64 = (assign29670_e43889 * 0.3333333333333333);
        let assign29670_e43892: f64 = (1.0 + assign29670_e43891);
        let assign29670_e43893: f64 = (assign29670_e43871 * assign29670_e43892);
        let assign29670_e43894: f64 = (0.5 * assign29670_e43893);
        let assign29670_e43895: f64 = (1.0 + assign29670_e43894);
        let assign29670_e43896: f64 = (assign29670_e43852 * assign29670_e43895);
        let assign29670_e43897: f64 = (1.0 + assign29670_e43896);
        let assign29670_e43898: f64 = (1e100 * assign29670_e43897);
        (assign29670_e43898, (1e100 * (((locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29670_e43847) - (assign29670_e43844 * (locals.var_nj0_dn0 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29670_e43866) - (assign29670_e43863 * (locals.var_nj0_dn0 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((locals.var_phitdinv * ((((locals.var_vak_dn0 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn0)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign29670_e43884) - (assign29670_e43881 * (locals.var_nj0_dn0 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29670_e43847) - (assign29670_e43844 * (locals.var_nj0_dn2 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29670_e43866) - (assign29670_e43863 * (locals.var_nj0_dn2 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((locals.var_phitdinv * ((((locals.var_vak_dn2 * locals.var_nj1) - (locals.var_vak * locals.var_nj1_dn2)) / (locals.var_nj1 * locals.var_nj1)) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign29670_e43884) - (assign29670_e43881 * (locals.var_nj0_dn2 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_idmultbot, locals.var_idmultbot_dn0, locals.var_idmultbot_dn2,)
    }
};
        locals.var_idmultbot = assign29670_e43900;
        locals.var_idmultbot_dn0 = assign29670_e43900_d_n0;
        locals.var_idmultbot_dn2 = assign29670_e43900_d_n2;

        let (assign29680_e43913,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29680_e43909: f64 = (locals.var_nin * locals.var_nin);
        let assign29680_e43911: f64 = (assign29680_e43909 / locals.var_ndisti_i);
        (assign29680_e43911,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign29680_e43913;

        let (assign29690_e43929,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29690_e43922: f64 = (locals.var_nfasti_i / locals.var_phitdinv);
        let assign29690_e43925: f64 = (locals.var_ndisti_i / locals.var_pnn0);
        let assign29690_e43926: f64 = (assign29690_e43925).ln();
        let assign29690_e43927: f64 = (assign29690_e43922 * assign29690_e43926);
        (assign29690_e43927,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign29690_e43929;

        let assign29700_e43932: f64 = if locals.var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign29700_e43932;

        let (assign29710_e43949, assign29710_e43949_d_n0, assign29710_e43949_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29710_e43944: f64 = (locals.var_vak - locals.var_vha1);
        let assign29710_e43945: f64 = (p.p86 * assign29710_e43944);
        let assign29710_e43947: f64 = (assign29710_e43945 + locals.var_nfasti_i);
        (assign29710_e43947, (p.p86 * locals.var_vak_dn0), (p.p86 * locals.var_vak_dn2),)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign29710_e43949;
        locals.var_nja10_dn0 = assign29710_e43949_d_n0;
        locals.var_nja10_dn2 = assign29710_e43949_d_n2;

        let (assign29720_e43964, assign29720_e43964_d_n0, assign29720_e43964_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29720_e43961: f64 = (p.p86 * locals.var_vha1);
        let assign29720_e43962: f64 = (locals.var_nfasti_i - assign29720_e43961);
        (assign29720_e43962, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29720_e43964;
        locals.var_nj0_dn0 = assign29720_e43964_d_n0;
        locals.var_nj0_dn2 = assign29720_e43964_d_n2;

        let (assign29730_e43979, assign29730_e43979_d_n0, assign29730_e43979_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29730_e43975: f64 = (p.p85 - locals.var_nja10);
        let assign29730_e43977: f64 = (assign29730_e43975 - 0.01);
        (assign29730_e43977, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29730_e43979;
        locals.var_tmf1_dn0 = assign29730_e43979_d_n0;
        locals.var_tmf1_dn2 = assign29730_e43979_d_n2;

        let (assign29740_e43994, assign29740_e43994_d_n0, assign29740_e43994_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29740_e43990: f64 = (4.0 * p.p85);
        let assign29740_e43992: f64 = (assign29740_e43990 * 0.01);
        (assign29740_e43992, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29740_e43994;
        locals.var_tmf2_dn0 = assign29740_e43994_d_n0;
        locals.var_tmf2_dn2 = assign29740_e43994_d_n2;

        let (assign29750_e44011, assign29750_e44011_d_n0, assign29750_e44011_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29750_e44008: f64 = (-locals.var_tmf2);
                (assign29750_e44008, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29750_e44011;
        locals.var_tmf2_dn0 = assign29750_e44011_d_n0;
        locals.var_tmf2_dn2 = assign29750_e44011_d_n2;

        let (assign29760_e44027, assign29760_e44027_d_n0, assign29760_e44027_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29760_e44022: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29760_e44024: f64 = (assign29760_e44022 + locals.var_tmf2);
        let assign29760_e44025: f64 = (assign29760_e44024).sqrt();
        (assign29760_e44025, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29760_e44025)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29760_e44025)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29760_e44027;
        locals.var_tmf2_dn0 = assign29760_e44027_d_n0;
        locals.var_tmf2_dn2 = assign29760_e44027_d_n2;

        let (assign29770_e44044, assign29770_e44044_d_n0, assign29770_e44044_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29770_e44040: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29770_e44041: f64 = (0.5 * assign29770_e44040);
        let assign29770_e44042: f64 = (p.p85 - assign29770_e44041);
        (assign29770_e44042, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign29770_e44044;
        locals.var_nja11_dn0 = assign29770_e44044_d_n0;
        locals.var_nja11_dn2 = assign29770_e44044_d_n2;

        let (assign29780_e44059, assign29780_e44059_d_n0, assign29780_e44059_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29780_e44055: f64 = (locals.var_nja11 - locals.var_nfasti_i);
        let assign29780_e44057: f64 = (assign29780_e44055 - 0.01);
        (assign29780_e44057, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29780_e44059;
        locals.var_tmf1_dn0 = assign29780_e44059_d_n0;
        locals.var_tmf1_dn2 = assign29780_e44059_d_n2;

        let (assign29790_e44074, assign29790_e44074_d_n0, assign29790_e44074_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29790_e44070: f64 = (4.0 * locals.var_nfasti_i);
        let assign29790_e44072: f64 = (assign29790_e44070 * 0.01);
        (assign29790_e44072, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29790_e44074;
        locals.var_tmf2_dn0 = assign29790_e44074_d_n0;
        locals.var_tmf2_dn2 = assign29790_e44074_d_n2;

        let (assign29800_e44091, assign29800_e44091_d_n0, assign29800_e44091_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29800_e44088: f64 = (-locals.var_tmf2);
                (assign29800_e44088, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29800_e44091;
        locals.var_tmf2_dn0 = assign29800_e44091_d_n0;
        locals.var_tmf2_dn2 = assign29800_e44091_d_n2;

        let (assign29810_e44107, assign29810_e44107_d_n0, assign29810_e44107_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29810_e44102: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29810_e44104: f64 = (assign29810_e44102 + locals.var_tmf2);
        let assign29810_e44105: f64 = (assign29810_e44104).sqrt();
        (assign29810_e44105, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29810_e44105)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29810_e44105)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29810_e44107;
        locals.var_tmf2_dn0 = assign29810_e44107_d_n0;
        locals.var_tmf2_dn2 = assign29810_e44107_d_n2;

        let (assign29820_e44124, assign29820_e44124_d_n0, assign29820_e44124_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29820_e44120: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29820_e44121: f64 = (0.5 * assign29820_e44120);
        let assign29820_e44122: f64 = (locals.var_nfasti_i + assign29820_e44121);
        (assign29820_e44122, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign29820_e44124;
        locals.var_nj1_dn0 = assign29820_e44124_d_n0;
        locals.var_nj1_dn2 = assign29820_e44124_d_n2;

        let (assign29830_e44139, assign29830_e44139_d_n0, assign29830_e44139_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29830_e44135: f64 = (p.p85 - locals.var_nj0);
        let assign29830_e44137: f64 = (assign29830_e44135 - 0.01);
        (assign29830_e44137, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29830_e44139;
        locals.var_tmf1_dn0 = assign29830_e44139_d_n0;
        locals.var_tmf1_dn2 = assign29830_e44139_d_n2;

        let (assign29840_e44154, assign29840_e44154_d_n0, assign29840_e44154_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29840_e44150: f64 = (4.0 * p.p85);
        let assign29840_e44152: f64 = (assign29840_e44150 * 0.01);
        (assign29840_e44152, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29840_e44154;
        locals.var_tmf2_dn0 = assign29840_e44154_d_n0;
        locals.var_tmf2_dn2 = assign29840_e44154_d_n2;

        let (assign29850_e44171, assign29850_e44171_d_n0, assign29850_e44171_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29850_e44168: f64 = (-locals.var_tmf2);
                (assign29850_e44168, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29850_e44171;
        locals.var_tmf2_dn0 = assign29850_e44171_d_n0;
        locals.var_tmf2_dn2 = assign29850_e44171_d_n2;

        let (assign29860_e44187, assign29860_e44187_d_n0, assign29860_e44187_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29860_e44182: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29860_e44184: f64 = (assign29860_e44182 + locals.var_tmf2);
        let assign29860_e44185: f64 = (assign29860_e44184).sqrt();
        (assign29860_e44185, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29860_e44185)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29860_e44185)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29860_e44187;
        locals.var_tmf2_dn0 = assign29860_e44187_d_n0;
        locals.var_tmf2_dn2 = assign29860_e44187_d_n2;

        let (assign29870_e44204, assign29870_e44204_d_n0, assign29870_e44204_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29870_e44200: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29870_e44201: f64 = (0.5 * assign29870_e44200);
        let assign29870_e44202: f64 = (p.p85 - assign29870_e44201);
        (assign29870_e44202, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29870_e44204;
        locals.var_nj0_dn0 = assign29870_e44204_d_n0;
        locals.var_nj0_dn2 = assign29870_e44204_d_n2;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29880_e44219, assign29880_e44219_d_n0, assign29880_e44219_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29880_e44215: f64 = (locals.var_nj0 - locals.var_nfasti_i);
        let assign29880_e44217: f64 = (assign29880_e44215 - 0.01);
        (assign29880_e44217, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign29880_e44219;
        locals.var_tmf1_dn0 = assign29880_e44219_d_n0;
        locals.var_tmf1_dn2 = assign29880_e44219_d_n2;

        let (assign29890_e44234, assign29890_e44234_d_n0, assign29890_e44234_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29890_e44230: f64 = (4.0 * locals.var_nfasti_i);
        let assign29890_e44232: f64 = (assign29890_e44230 * 0.01);
        (assign29890_e44232, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29890_e44234;
        locals.var_tmf2_dn0 = assign29890_e44234_d_n0;
        locals.var_tmf2_dn2 = assign29890_e44234_d_n2;

        let (assign29900_e44251, assign29900_e44251_d_n0, assign29900_e44251_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign29900_e44248: f64 = (-locals.var_tmf2);
                (assign29900_e44248, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29900_e44251;
        locals.var_tmf2_dn0 = assign29900_e44251_d_n0;
        locals.var_tmf2_dn2 = assign29900_e44251_d_n2;

        let (assign29910_e44267, assign29910_e44267_d_n0, assign29910_e44267_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29910_e44262: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29910_e44264: f64 = (assign29910_e44262 + locals.var_tmf2);
        let assign29910_e44265: f64 = (assign29910_e44264).sqrt();
        (assign29910_e44265, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29910_e44265)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29910_e44265)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign29910_e44267;
        locals.var_tmf2_dn0 = assign29910_e44267_d_n0;
        locals.var_tmf2_dn2 = assign29910_e44267_d_n2;

        let (assign29920_e44284, assign29920_e44284_d_n0, assign29920_e44284_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign29920_e44280: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29920_e44281: f64 = (0.5 * assign29920_e44280);
        let assign29920_e44282: f64 = (locals.var_nfasti_i + assign29920_e44281);
        (assign29920_e44282, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29920_e44284;
        locals.var_nj0_dn0 = assign29920_e44284_d_n0;
        locals.var_nj0_dn2 = assign29920_e44284_d_n2;

        let (assign29930_e44296, assign29930_e44296_d_n0, assign29930_e44296_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign29930_e44296;
        locals.var_nj0_dn0 = assign29930_e44296_d_n0;
        locals.var_nj0_dn2 = assign29930_e44296_d_n2;

        let (assign29940_e44308, assign29940_e44308_d_n0, assign29940_e44308_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard486 == 0.0)) {
        (locals.var_nfasti_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign29940_e44308;
        locals.var_nj1_dn0 = assign29940_e44308_d_n0;
        locals.var_nj1_dn2 = assign29940_e44308_d_n2;

        let (assign30000_e44543,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign30000_e44539: f64 = (locals.var_nin * locals.var_nin);
        let assign30000_e44541: f64 = (assign30000_e44539 / locals.var_ndigat_i);
        (assign30000_e44541,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign30000_e44543;

        let (assign30010_e44559,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign30010_e44552: f64 = (locals.var_nfagat_i / locals.var_phitdinv);
        let assign30010_e44555: f64 = (locals.var_ndigat_i / locals.var_pnn0);
        let assign30010_e44556: f64 = (assign30010_e44555).ln();
        let assign30010_e44557: f64 = (assign30010_e44552 * assign30010_e44556);
        (assign30010_e44557,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign30010_e44559;

        let assign30020_e44562: f64 = if locals.var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard489 = assign30020_e44562;

        let (assign30030_e44579, assign30030_e44579_d_n0, assign30030_e44579_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30030_e44574: f64 = (locals.var_vak - locals.var_vha1);
        let assign30030_e44575: f64 = (p.p86 * assign30030_e44574);
        let assign30030_e44577: f64 = (assign30030_e44575 + locals.var_nfagat_i);
        (assign30030_e44577, (p.p86 * locals.var_vak_dn0), (p.p86 * locals.var_vak_dn2),)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign30030_e44579;
        locals.var_nja10_dn0 = assign30030_e44579_d_n0;
        locals.var_nja10_dn2 = assign30030_e44579_d_n2;

        let (assign30040_e44594, assign30040_e44594_d_n0, assign30040_e44594_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30040_e44591: f64 = (p.p86 * locals.var_vha1);
        let assign30040_e44592: f64 = (locals.var_nfagat_i - assign30040_e44591);
        (assign30040_e44592, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30040_e44594;
        locals.var_nj0_dn0 = assign30040_e44594_d_n0;
        locals.var_nj0_dn2 = assign30040_e44594_d_n2;

        let (assign30050_e44609, assign30050_e44609_d_n0, assign30050_e44609_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30050_e44605: f64 = (p.p85 - locals.var_nja10);
        let assign30050_e44607: f64 = (assign30050_e44605 - 0.01);
        (assign30050_e44607, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30050_e44609;
        locals.var_tmf1_dn0 = assign30050_e44609_d_n0;
        locals.var_tmf1_dn2 = assign30050_e44609_d_n2;

        let (assign30060_e44624, assign30060_e44624_d_n0, assign30060_e44624_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30060_e44620: f64 = (4.0 * p.p85);
        let assign30060_e44622: f64 = (assign30060_e44620 * 0.01);
        (assign30060_e44622, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30060_e44624;
        locals.var_tmf2_dn0 = assign30060_e44624_d_n0;
        locals.var_tmf2_dn2 = assign30060_e44624_d_n2;

        let (assign30070_e44641, assign30070_e44641_d_n0, assign30070_e44641_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30070_e44638: f64 = (-locals.var_tmf2);
                (assign30070_e44638, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30070_e44641;
        locals.var_tmf2_dn0 = assign30070_e44641_d_n0;
        locals.var_tmf2_dn2 = assign30070_e44641_d_n2;

        let (assign30080_e44657, assign30080_e44657_d_n0, assign30080_e44657_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30080_e44652: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30080_e44654: f64 = (assign30080_e44652 + locals.var_tmf2);
        let assign30080_e44655: f64 = (assign30080_e44654).sqrt();
        (assign30080_e44655, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30080_e44655)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30080_e44655)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30080_e44657;
        locals.var_tmf2_dn0 = assign30080_e44657_d_n0;
        locals.var_tmf2_dn2 = assign30080_e44657_d_n2;

        let (assign30090_e44674, assign30090_e44674_d_n0, assign30090_e44674_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30090_e44670: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30090_e44671: f64 = (0.5 * assign30090_e44670);
        let assign30090_e44672: f64 = (p.p85 - assign30090_e44671);
        (assign30090_e44672, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign30090_e44674;
        locals.var_nja11_dn0 = assign30090_e44674_d_n0;
        locals.var_nja11_dn2 = assign30090_e44674_d_n2;

        let (assign30100_e44689, assign30100_e44689_d_n0, assign30100_e44689_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30100_e44685: f64 = (locals.var_nja11 - locals.var_nfagat_i);
        let assign30100_e44687: f64 = (assign30100_e44685 - 0.01);
        (assign30100_e44687, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30100_e44689;
        locals.var_tmf1_dn0 = assign30100_e44689_d_n0;
        locals.var_tmf1_dn2 = assign30100_e44689_d_n2;

        let (assign30110_e44704, assign30110_e44704_d_n0, assign30110_e44704_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30110_e44700: f64 = (4.0 * locals.var_nfagat_i);
        let assign30110_e44702: f64 = (assign30110_e44700 * 0.01);
        (assign30110_e44702, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30110_e44704;
        locals.var_tmf2_dn0 = assign30110_e44704_d_n0;
        locals.var_tmf2_dn2 = assign30110_e44704_d_n2;

        let (assign30120_e44721, assign30120_e44721_d_n0, assign30120_e44721_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30120_e44718: f64 = (-locals.var_tmf2);
                (assign30120_e44718, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30120_e44721;
        locals.var_tmf2_dn0 = assign30120_e44721_d_n0;
        locals.var_tmf2_dn2 = assign30120_e44721_d_n2;

        let (assign30130_e44737, assign30130_e44737_d_n0, assign30130_e44737_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30130_e44732: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30130_e44734: f64 = (assign30130_e44732 + locals.var_tmf2);
        let assign30130_e44735: f64 = (assign30130_e44734).sqrt();
        (assign30130_e44735, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30130_e44735)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30130_e44735)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30130_e44737;
        locals.var_tmf2_dn0 = assign30130_e44737_d_n0;
        locals.var_tmf2_dn2 = assign30130_e44737_d_n2;

        let (assign30140_e44754, assign30140_e44754_d_n0, assign30140_e44754_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30140_e44750: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30140_e44751: f64 = (0.5 * assign30140_e44750);
        let assign30140_e44752: f64 = (locals.var_nfagat_i + assign30140_e44751);
        (assign30140_e44752, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign30140_e44754;
        locals.var_nj1_dn0 = assign30140_e44754_d_n0;
        locals.var_nj1_dn2 = assign30140_e44754_d_n2;

        let (assign30150_e44769, assign30150_e44769_d_n0, assign30150_e44769_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30150_e44765: f64 = (p.p85 - locals.var_nj0);
        let assign30150_e44767: f64 = (assign30150_e44765 - 0.01);
        (assign30150_e44767, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30150_e44769;
        locals.var_tmf1_dn0 = assign30150_e44769_d_n0;
        locals.var_tmf1_dn2 = assign30150_e44769_d_n2;

        let (assign30160_e44784, assign30160_e44784_d_n0, assign30160_e44784_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30160_e44780: f64 = (4.0 * p.p85);
        let assign30160_e44782: f64 = (assign30160_e44780 * 0.01);
        (assign30160_e44782, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30160_e44784;
        locals.var_tmf2_dn0 = assign30160_e44784_d_n0;
        locals.var_tmf2_dn2 = assign30160_e44784_d_n2;

        let (assign30170_e44801, assign30170_e44801_d_n0, assign30170_e44801_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30170_e44798: f64 = (-locals.var_tmf2);
                (assign30170_e44798, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30170_e44801;
        locals.var_tmf2_dn0 = assign30170_e44801_d_n0;
        locals.var_tmf2_dn2 = assign30170_e44801_d_n2;

        let (assign30180_e44817, assign30180_e44817_d_n0, assign30180_e44817_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30180_e44812: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30180_e44814: f64 = (assign30180_e44812 + locals.var_tmf2);
        let assign30180_e44815: f64 = (assign30180_e44814).sqrt();
        (assign30180_e44815, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30180_e44815)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30180_e44815)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30180_e44817;
        locals.var_tmf2_dn0 = assign30180_e44817_d_n0;
        locals.var_tmf2_dn2 = assign30180_e44817_d_n2;

        let (assign30190_e44834, assign30190_e44834_d_n0, assign30190_e44834_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30190_e44830: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30190_e44831: f64 = (0.5 * assign30190_e44830);
        let assign30190_e44832: f64 = (p.p85 - assign30190_e44831);
        (assign30190_e44832, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30190_e44834;
        locals.var_nj0_dn0 = assign30190_e44834_d_n0;
        locals.var_nj0_dn2 = assign30190_e44834_d_n2;

        let (assign30200_e44849, assign30200_e44849_d_n0, assign30200_e44849_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30200_e44845: f64 = (locals.var_nj0 - locals.var_nfagat_i);
        let assign30200_e44847: f64 = (assign30200_e44845 - 0.01);
        (assign30200_e44847, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30200_e44849;
        locals.var_tmf1_dn0 = assign30200_e44849_d_n0;
        locals.var_tmf1_dn2 = assign30200_e44849_d_n2;

        let (assign30210_e44864, assign30210_e44864_d_n0, assign30210_e44864_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30210_e44860: f64 = (4.0 * locals.var_nfagat_i);
        let assign30210_e44862: f64 = (assign30210_e44860 * 0.01);
        (assign30210_e44862, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30210_e44864;
        locals.var_tmf2_dn0 = assign30210_e44864_d_n0;
        locals.var_tmf2_dn2 = assign30210_e44864_d_n2;

        let (assign30220_e44881, assign30220_e44881_d_n0, assign30220_e44881_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30220_e44878: f64 = (-locals.var_tmf2);
                (assign30220_e44878, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30220_e44881;
        locals.var_tmf2_dn0 = assign30220_e44881_d_n0;
        locals.var_tmf2_dn2 = assign30220_e44881_d_n2;

        let (assign30230_e44897, assign30230_e44897_d_n0, assign30230_e44897_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30230_e44892: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30230_e44894: f64 = (assign30230_e44892 + locals.var_tmf2);
        let assign30230_e44895: f64 = (assign30230_e44894).sqrt();
        (assign30230_e44895, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30230_e44895)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30230_e44895)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30230_e44897;
        locals.var_tmf2_dn0 = assign30230_e44897_d_n0;
        locals.var_tmf2_dn2 = assign30230_e44897_d_n2;

        let (assign30240_e44914, assign30240_e44914_d_n0, assign30240_e44914_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 != 0.0)) {
        let assign30240_e44910: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30240_e44911: f64 = (0.5 * assign30240_e44910);
        let assign30240_e44912: f64 = (locals.var_nfagat_i + assign30240_e44911);
        (assign30240_e44912, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30240_e44914;
        locals.var_nj0_dn0 = assign30240_e44914_d_n0;
        locals.var_nj0_dn2 = assign30240_e44914_d_n2;

        let (assign30250_e44926, assign30250_e44926_d_n0, assign30250_e44926_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30250_e44926;
        locals.var_nj0_dn0 = assign30250_e44926_d_n0;
        locals.var_nj0_dn2 = assign30250_e44926_d_n2;

        let (assign30260_e44938, assign30260_e44938_d_n0, assign30260_e44938_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) && (locals.var_guard489 == 0.0)) {
        (locals.var_nfagat_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign30260_e44938;
        locals.var_nj1_dn0 = assign30260_e44938_d_n0;
        locals.var_nj1_dn2 = assign30260_e44938_d_n2;

        let (assign30330_e45193,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) {
        let assign30330_e45189: f64 = (locals.var_nin * locals.var_nin);
        let assign30330_e45191: f64 = (assign30330_e45189 / locals.var_ndibot_i);
        (assign30330_e45191,)
    } else {
        (locals.var_pnn0,)
    }
};
        locals.var_pnn0 = assign30330_e45193;

        let (assign30340_e45210,) = {
    if (((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) {
        let assign30340_e45203: f64 = (locals.var_nfabot_i / locals.var_phitdinv);
        let assign30340_e45206: f64 = (locals.var_ndibot_i / locals.var_pnn0);
        let assign30340_e45207: f64 = (assign30340_e45206).ln();
        let assign30340_e45208: f64 = (assign30340_e45203 * assign30340_e45207);
        (assign30340_e45208,)
    } else {
        (locals.var_vha1,)
    }
};
        locals.var_vha1 = assign30340_e45210;

        let assign30350_e45213: f64 = if locals.var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        locals.var_guard492 = assign30350_e45213;

        let (assign30360_e45231, assign30360_e45231_d_n0, assign30360_e45231_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30360_e45226: f64 = (locals.var_vmax - locals.var_vha1);
        let assign30360_e45227: f64 = (p.p86 * assign30360_e45226);
        let assign30360_e45229: f64 = (assign30360_e45227 + locals.var_nfabot_i);
        (assign30360_e45229, 0.0, 0.0,)
    } else {
        (locals.var_nja10, locals.var_nja10_dn0, locals.var_nja10_dn2,)
    }
};
        locals.var_nja10 = assign30360_e45231;
        locals.var_nja10_dn0 = assign30360_e45231_d_n0;
        locals.var_nja10_dn2 = assign30360_e45231_d_n2;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30370_e45247, assign30370_e45247_d_n0, assign30370_e45247_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30370_e45244: f64 = (p.p86 * locals.var_vha1);
        let assign30370_e45245: f64 = (locals.var_nfabot_i - assign30370_e45244);
        (assign30370_e45245, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30370_e45247;
        locals.var_nj0_dn0 = assign30370_e45247_d_n0;
        locals.var_nj0_dn2 = assign30370_e45247_d_n2;

        let (assign30380_e45263, assign30380_e45263_d_n0, assign30380_e45263_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30380_e45259: f64 = (p.p85 - locals.var_nja10);
        let assign30380_e45261: f64 = (assign30380_e45259 - 0.01);
        (assign30380_e45261, (-locals.var_nja10_dn0), (-locals.var_nja10_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30380_e45263;
        locals.var_tmf1_dn0 = assign30380_e45263_d_n0;
        locals.var_tmf1_dn2 = assign30380_e45263_d_n2;

        let (assign30390_e45279, assign30390_e45279_d_n0, assign30390_e45279_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30390_e45275: f64 = (4.0 * p.p85);
        let assign30390_e45277: f64 = (assign30390_e45275 * 0.01);
        (assign30390_e45277, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30390_e45279;
        locals.var_tmf2_dn0 = assign30390_e45279_d_n0;
        locals.var_tmf2_dn2 = assign30390_e45279_d_n2;

        let (assign30400_e45297, assign30400_e45297_d_n0, assign30400_e45297_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30400_e45294: f64 = (-locals.var_tmf2);
                (assign30400_e45294, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30400_e45297;
        locals.var_tmf2_dn0 = assign30400_e45297_d_n0;
        locals.var_tmf2_dn2 = assign30400_e45297_d_n2;

        let (assign30410_e45314, assign30410_e45314_d_n0, assign30410_e45314_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30410_e45309: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30410_e45311: f64 = (assign30410_e45309 + locals.var_tmf2);
        let assign30410_e45312: f64 = (assign30410_e45311).sqrt();
        (assign30410_e45312, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30410_e45312)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30410_e45312)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30410_e45314;
        locals.var_tmf2_dn0 = assign30410_e45314_d_n0;
        locals.var_tmf2_dn2 = assign30410_e45314_d_n2;

        let (assign30420_e45332, assign30420_e45332_d_n0, assign30420_e45332_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30420_e45328: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign30420_e45329: f64 = (1.0 + assign30420_e45328);
        let assign30420_e45330: f64 = (0.5 * assign30420_e45329);
        (assign30420_e45330, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_su, locals.var_dfn_su_dn0, locals.var_dfn_su_dn2,)
    }
};
        locals.var_dfn_su = assign30420_e45332;
        locals.var_dfn_su_dn0 = assign30420_e45332_d_n0;
        locals.var_dfn_su_dn2 = assign30420_e45332_d_n2;

        let (assign30430_e45350, assign30430_e45350_d_n0, assign30430_e45350_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30430_e45346: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30430_e45347: f64 = (0.5 * assign30430_e45346);
        let assign30430_e45348: f64 = (p.p85 - assign30430_e45347);
        (assign30430_e45348, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nja11, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    }
};
        locals.var_nja11 = assign30430_e45350;
        locals.var_nja11_dn0 = assign30430_e45350_d_n0;
        locals.var_nja11_dn2 = assign30430_e45350_d_n2;

        let (assign30440_e45366, assign30440_e45366_d_n0, assign30440_e45366_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30440_e45362: f64 = (locals.var_nja11 - locals.var_nfabot_i);
        let assign30440_e45364: f64 = (assign30440_e45362 - 0.01);
        (assign30440_e45364, locals.var_nja11_dn0, locals.var_nja11_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30440_e45366;
        locals.var_tmf1_dn0 = assign30440_e45366_d_n0;
        locals.var_tmf1_dn2 = assign30440_e45366_d_n2;

        let (assign30450_e45382, assign30450_e45382_d_n0, assign30450_e45382_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30450_e45378: f64 = (4.0 * locals.var_nfabot_i);
        let assign30450_e45380: f64 = (assign30450_e45378 * 0.01);
        (assign30450_e45380, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30450_e45382;
        locals.var_tmf2_dn0 = assign30450_e45382_d_n0;
        locals.var_tmf2_dn2 = assign30450_e45382_d_n2;

        let (assign30460_e45400, assign30460_e45400_d_n0, assign30460_e45400_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30460_e45397: f64 = (-locals.var_tmf2);
                (assign30460_e45397, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30460_e45400;
        locals.var_tmf2_dn0 = assign30460_e45400_d_n0;
        locals.var_tmf2_dn2 = assign30460_e45400_d_n2;

        let (assign30470_e45417, assign30470_e45417_d_n0, assign30470_e45417_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30470_e45412: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30470_e45414: f64 = (assign30470_e45412 + locals.var_tmf2);
        let assign30470_e45415: f64 = (assign30470_e45414).sqrt();
        (assign30470_e45415, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30470_e45415)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30470_e45415)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30470_e45417;
        locals.var_tmf2_dn0 = assign30470_e45417_d_n0;
        locals.var_tmf2_dn2 = assign30470_e45417_d_n2;

        let (assign30480_e45435, assign30480_e45435_d_n0, assign30480_e45435_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30480_e45431: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign30480_e45432: f64 = (1.0 + assign30480_e45431);
        let assign30480_e45433: f64 = (0.5 * assign30480_e45432);
        (assign30480_e45433, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_dfn_sl, locals.var_dfn_sl_dn0, locals.var_dfn_sl_dn2,)
    }
};
        locals.var_dfn_sl = assign30480_e45435;
        locals.var_dfn_sl_dn0 = assign30480_e45435_d_n0;
        locals.var_dfn_sl_dn2 = assign30480_e45435_d_n2;

        let (assign30490_e45453, assign30490_e45453_d_n0, assign30490_e45453_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30490_e45449: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30490_e45450: f64 = (0.5 * assign30490_e45449);
        let assign30490_e45451: f64 = (locals.var_nfabot_i + assign30490_e45450);
        (assign30490_e45451, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign30490_e45453;
        locals.var_nj1_dn0 = assign30490_e45453_d_n0;
        locals.var_nj1_dn2 = assign30490_e45453_d_n2;

        let (assign30500_e45469, assign30500_e45469_d_n0, assign30500_e45469_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30500_e45465: f64 = (p.p85 - locals.var_nj0);
        let assign30500_e45467: f64 = (assign30500_e45465 - 0.01);
        (assign30500_e45467, (-locals.var_nj0_dn0), (-locals.var_nj0_dn2),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30500_e45469;
        locals.var_tmf1_dn0 = assign30500_e45469_d_n0;
        locals.var_tmf1_dn2 = assign30500_e45469_d_n2;

        let (assign30510_e45485, assign30510_e45485_d_n0, assign30510_e45485_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30510_e45481: f64 = (4.0 * p.p85);
        let assign30510_e45483: f64 = (assign30510_e45481 * 0.01);
        (assign30510_e45483, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30510_e45485;
        locals.var_tmf2_dn0 = assign30510_e45485_d_n0;
        locals.var_tmf2_dn2 = assign30510_e45485_d_n2;

        let (assign30520_e45503, assign30520_e45503_d_n0, assign30520_e45503_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30520_e45500: f64 = (-locals.var_tmf2);
                (assign30520_e45500, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30520_e45503;
        locals.var_tmf2_dn0 = assign30520_e45503_d_n0;
        locals.var_tmf2_dn2 = assign30520_e45503_d_n2;

        let (assign30530_e45520, assign30530_e45520_d_n0, assign30530_e45520_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30530_e45515: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30530_e45517: f64 = (assign30530_e45515 + locals.var_tmf2);
        let assign30530_e45518: f64 = (assign30530_e45517).sqrt();
        (assign30530_e45518, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30530_e45518)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30530_e45518)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30530_e45520;
        locals.var_tmf2_dn0 = assign30530_e45520_d_n0;
        locals.var_tmf2_dn2 = assign30530_e45520_d_n2;

        let (assign30540_e45538, assign30540_e45538_d_n0, assign30540_e45538_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30540_e45534: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30540_e45535: f64 = (0.5 * assign30540_e45534);
        let assign30540_e45536: f64 = (p.p85 - assign30540_e45535);
        (assign30540_e45536, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30540_e45538;
        locals.var_nj0_dn0 = assign30540_e45538_d_n0;
        locals.var_nj0_dn2 = assign30540_e45538_d_n2;

        let (assign30550_e45554, assign30550_e45554_d_n0, assign30550_e45554_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30550_e45550: f64 = (locals.var_nj0 - locals.var_nfabot_i);
        let assign30550_e45552: f64 = (assign30550_e45550 - 0.01);
        (assign30550_e45552, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2,)
    }
};
        locals.var_tmf1 = assign30550_e45554;
        locals.var_tmf1_dn0 = assign30550_e45554_d_n0;
        locals.var_tmf1_dn2 = assign30550_e45554_d_n2;

        let (assign30560_e45570, assign30560_e45570_d_n0, assign30560_e45570_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30560_e45566: f64 = (4.0 * locals.var_nfabot_i);
        let assign30560_e45568: f64 = (assign30560_e45566 * 0.01);
        (assign30560_e45568, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30560_e45570;
        locals.var_tmf2_dn0 = assign30560_e45570_d_n0;
        locals.var_tmf2_dn2 = assign30560_e45570_d_n2;

        let (assign30570_e45588, assign30570_e45588_d_n0, assign30570_e45588_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n2,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
            } else {
                let assign30570_e45585: f64 = (-locals.var_tmf2);
                (assign30570_e45585, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2),)
            }
        };
        (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n2,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30570_e45588;
        locals.var_tmf2_dn0 = assign30570_e45588_d_n0;
        locals.var_tmf2_dn2 = assign30570_e45588_d_n2;

        let (assign30580_e45605, assign30580_e45605_d_n0, assign30580_e45605_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30580_e45600: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign30580_e45602: f64 = (assign30580_e45600 + locals.var_tmf2);
        let assign30580_e45603: f64 = (assign30580_e45602).sqrt();
        (assign30580_e45603, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign30580_e45603)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign30580_e45603)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2,)
    }
};
        locals.var_tmf2 = assign30580_e45605;
        locals.var_tmf2_dn0 = assign30580_e45605_d_n0;
        locals.var_tmf2_dn2 = assign30580_e45605_d_n2;

        let (assign30590_e45623, assign30590_e45623_d_n0, assign30590_e45623_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30590_e45619: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign30590_e45620: f64 = (0.5 * assign30590_e45619);
        let assign30590_e45621: f64 = (locals.var_nfabot_i + assign30590_e45620);
        (assign30590_e45621, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)),)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30590_e45623;
        locals.var_nj0_dn0 = assign30590_e45623_d_n0;
        locals.var_nj0_dn2 = assign30590_e45623_d_n2;

        let (assign30600_e45639, assign30600_e45639_d_n0, assign30600_e45639_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign30600_e45635: f64 = (p.p86 * locals.var_dfn_su);
        let assign30600_e45637: f64 = (assign30600_e45635 * locals.var_dfn_sl);
        (assign30600_e45637, (((p.p86 * locals.var_dfn_su_dn0) * locals.var_dfn_sl) + (assign30600_e45635 * locals.var_dfn_sl_dn0)), (((p.p86 * locals.var_dfn_su_dn2) * locals.var_dfn_sl) + (assign30600_e45635 * locals.var_dfn_sl_dn2)),)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign30600_e45639;
        locals.var_dnj1_dv_dn0 = assign30600_e45639_d_n0;
        locals.var_dnj1_dv_dn2 = assign30600_e45639_d_n2;

        let (assign30610_e45652, assign30610_e45652_d_n0, assign30610_e45652_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj0, locals.var_nj0_dn0, locals.var_nj0_dn2,)
    }
};
        locals.var_nj0 = assign30610_e45652;
        locals.var_nj0_dn0 = assign30610_e45652_d_n0;
        locals.var_nj0_dn2 = assign30610_e45652_d_n2;

        let (assign30620_e45665, assign30620_e45665_d_n0, assign30620_e45665_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 == 0.0)) {
        (locals.var_nfabot_i, 0.0, 0.0,)
    } else {
        (locals.var_nj1, locals.var_nj1_dn0, locals.var_nj1_dn2,)
    }
};
        locals.var_nj1 = assign30620_e45665;
        locals.var_nj1_dn0 = assign30620_e45665_d_n0;
        locals.var_nj1_dn2 = assign30620_e45665_d_n2;

        let (assign30630_e45678, assign30630_e45678_d_n0, assign30630_e45678_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard492 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnj1_dv, locals.var_dnj1_dv_dn0, locals.var_dnj1_dv_dn2,)
    }
};
        locals.var_dnj1_dv = assign30630_e45678;
        locals.var_dnj1_dv_dn0 = assign30630_e45678_d_n0;
        locals.var_dnj1_dv_dn2 = assign30630_e45678_d_n2;

        let assign30640_e45682: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30640_e45686: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30640_e45687: f64 = (locals.var_vha1 * assign30640_e45686);
        let assign30640_e45690: f64 = (locals.var_nj0 * p.p85);
        let assign30640_e45691: f64 = (assign30640_e45687 / assign30640_e45690);
        let assign30640_e45692: f64 = (assign30640_e45682 + assign30640_e45691);
        let assign30640_e45693: f64 = (locals.var_phitdinv * assign30640_e45692);
        let assign30640_e45694: f64 = (assign30640_e45693).abs();
        let assign30640_e45696: f64 = if assign30640_e45694 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard493 = assign30640_e45696;

        let (assign30650_e45723, assign30650_e45723_d_n0, assign30650_e45723_d_n2,) = {
    if ((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard493 != 0.0)) {
        let assign30650_e45709: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30650_e45713: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30650_e45714: f64 = (locals.var_vha1 * assign30650_e45713);
        let assign30650_e45717: f64 = (locals.var_nj0 * p.p85);
        let assign30650_e45718: f64 = (assign30650_e45714 / assign30650_e45717);
        let assign30650_e45719: f64 = (assign30650_e45709 + assign30650_e45718);
        let assign30650_e45720: f64 = (locals.var_phitdinv * assign30650_e45719);
        let assign30650_e45721: f64 = (assign30650_e45720).exp();
        (assign30650_e45721, (assign30650_e45721 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30650_e45717) - (assign30650_e45714 * (locals.var_nj0_dn0 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30650_e45717) - (assign30650_e45714 * (locals.var_nj0_dn2 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign30650_e45723;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign30650_e45723_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign30650_e45723_d_n2;

        let assign30660_e45727: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30660_e45731: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30660_e45732: f64 = (locals.var_vha1 * assign30660_e45731);
        let assign30660_e45735: f64 = (locals.var_nj0 * p.p85);
        let assign30660_e45736: f64 = (assign30660_e45732 / assign30660_e45735);
        let assign30660_e45737: f64 = (assign30660_e45727 + assign30660_e45736);
        let assign30660_e45738: f64 = (locals.var_phitdinv * assign30660_e45737);
        let assign30660_e45740: f64 = (-230.25850929940458);
        let assign30660_e45741: f64 = if assign30660_e45738 < assign30660_e45740 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign30660_e45741;

        let (assign30670_e45823, assign30670_e45823_d_n0, assign30670_e45823_d_n2,) = {
    if (((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard493 == 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign30670_e45757: f64 = (-230.25850929940458);
        let assign30670_e45761: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30670_e45765: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30670_e45766: f64 = (locals.var_vha1 * assign30670_e45765);
        let assign30670_e45769: f64 = (locals.var_nj0 * p.p85);
        let assign30670_e45770: f64 = (assign30670_e45766 / assign30670_e45769);
        let assign30670_e45771: f64 = (assign30670_e45761 + assign30670_e45770);
        let assign30670_e45772: f64 = (locals.var_phitdinv * assign30670_e45771);
        let assign30670_e45773: f64 = (assign30670_e45757 - assign30670_e45772);
        let assign30670_e45777: f64 = (-230.25850929940458);
        let assign30670_e45781: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30670_e45785: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30670_e45786: f64 = (locals.var_vha1 * assign30670_e45785);
        let assign30670_e45789: f64 = (locals.var_nj0 * p.p85);
        let assign30670_e45790: f64 = (assign30670_e45786 / assign30670_e45789);
        let assign30670_e45791: f64 = (assign30670_e45781 + assign30670_e45790);
        let assign30670_e45792: f64 = (locals.var_phitdinv * assign30670_e45791);
        let assign30670_e45793: f64 = (assign30670_e45777 - assign30670_e45792);
        let assign30670_e45796: f64 = (-230.25850929940458);
        let assign30670_e45800: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30670_e45804: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30670_e45805: f64 = (locals.var_vha1 * assign30670_e45804);
        let assign30670_e45808: f64 = (locals.var_nj0 * p.p85);
        let assign30670_e45809: f64 = (assign30670_e45805 / assign30670_e45808);
        let assign30670_e45810: f64 = (assign30670_e45800 + assign30670_e45809);
        let assign30670_e45811: f64 = (locals.var_phitdinv * assign30670_e45810);
        let assign30670_e45812: f64 = (assign30670_e45796 - assign30670_e45811);
        let assign30670_e45814: f64 = (assign30670_e45812 * 0.3333333333333333);
        let assign30670_e45815: f64 = (1.0 + assign30670_e45814);
        let assign30670_e45816: f64 = (assign30670_e45793 * assign30670_e45815);
        let assign30670_e45817: f64 = (0.5 * assign30670_e45816);
        let assign30670_e45818: f64 = (1.0 + assign30670_e45817);
        let assign30670_e45819: f64 = (assign30670_e45773 * assign30670_e45818);
        let assign30670_e45820: f64 = (1.0 + assign30670_e45819);
        let assign30670_e45821: f64 = (1e-100 / assign30670_e45820);
        (assign30670_e45821, (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30670_e45769) - (assign30670_e45766 * (locals.var_nj0_dn0 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30670_e45789) - (assign30670_e45786 * (locals.var_nj0_dn0 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30670_e45808) - (assign30670_e45805 * (locals.var_nj0_dn0 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30670_e45769) - (assign30670_e45766 * (locals.var_nj0_dn2 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30670_e45789) - (assign30670_e45786 * (locals.var_nj0_dn2 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30670_e45808) - (assign30670_e45805 * (locals.var_nj0_dn2 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign30670_e45823;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign30670_e45823_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign30670_e45823_d_n2;

        let (assign30680_e45903, assign30680_e45903_d_n0, assign30680_e45903_d_n2,) = {
    if (((((locals.var_guard471 == 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard493 == 0.0)) && (locals.var_guard494 == 0.0)) {
        let assign30680_e45842: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30680_e45846: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30680_e45847: f64 = (locals.var_vha1 * assign30680_e45846);
        let assign30680_e45850: f64 = (locals.var_nj0 * p.p85);
        let assign30680_e45851: f64 = (assign30680_e45847 / assign30680_e45850);
        let assign30680_e45852: f64 = (assign30680_e45842 + assign30680_e45851);
        let assign30680_e45853: f64 = (locals.var_phitdinv * assign30680_e45852);
        let assign30680_e45855: f64 = (assign30680_e45853 - 230.25850929940458);
        let assign30680_e45861: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30680_e45865: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30680_e45866: f64 = (locals.var_vha1 * assign30680_e45865);
        let assign30680_e45869: f64 = (locals.var_nj0 * p.p85);
        let assign30680_e45870: f64 = (assign30680_e45866 / assign30680_e45869);
        let assign30680_e45871: f64 = (assign30680_e45861 + assign30680_e45870);
        let assign30680_e45872: f64 = (locals.var_phitdinv * assign30680_e45871);
        let assign30680_e45874: f64 = (assign30680_e45872 - 230.25850929940458);
        let assign30680_e45879: f64 = (locals.var_vmax / locals.var_nj1);
        let assign30680_e45883: f64 = (locals.var_nj1 - locals.var_nj0);
        let assign30680_e45884: f64 = (locals.var_vha1 * assign30680_e45883);
        let assign30680_e45887: f64 = (locals.var_nj0 * p.p85);
        let assign30680_e45888: f64 = (assign30680_e45884 / assign30680_e45887);
        let assign30680_e45889: f64 = (assign30680_e45879 + assign30680_e45888);
        let assign30680_e45890: f64 = (locals.var_phitdinv * assign30680_e45889);
        let assign30680_e45892: f64 = (assign30680_e45890 - 230.25850929940458);
        let assign30680_e45894: f64 = (assign30680_e45892 * 0.3333333333333333);
        let assign30680_e45895: f64 = (1.0 + assign30680_e45894);
        let assign30680_e45896: f64 = (assign30680_e45874 * assign30680_e45895);
        let assign30680_e45897: f64 = (0.5 * assign30680_e45896);
        let assign30680_e45898: f64 = (1.0 + assign30680_e45897);
        let assign30680_e45899: f64 = (assign30680_e45855 * assign30680_e45898);
        let assign30680_e45900: f64 = (1.0 + assign30680_e45899);
        let assign30680_e45901: f64 = (1e100 * assign30680_e45900);
        (assign30680_e45901, (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30680_e45850) - (assign30680_e45847 * (locals.var_nj0_dn0 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30680_e45869) - (assign30680_e45866 * (locals.var_nj0_dn0 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn0) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn0 - locals.var_nj0_dn0)) * assign30680_e45887) - (assign30680_e45884 * (locals.var_nj0_dn0 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30680_e45850) - (assign30680_e45847 * (locals.var_nj0_dn2 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30680_e45869) - (assign30680_e45866 * (locals.var_nj0_dn2 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((locals.var_phitdinv * ((-((locals.var_vmax * locals.var_nj1_dn2) / (locals.var_nj1 * locals.var_nj1))) + ((((locals.var_vha1 * (locals.var_nj1_dn2 - locals.var_nj0_dn2)) * assign30680_e45887) - (assign30680_e45884 * (locals.var_nj0_dn2 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))),)
    } else {
        (locals.var_exp_vmax_over_phitd_bot, locals.var_exp_vmax_over_phitd_bot_dn0, locals.var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        locals.var_exp_vmax_over_phitd_bot = assign30680_e45903;
        locals.var_exp_vmax_over_phitd_bot_dn0 = assign30680_e45903_d_n0;
        locals.var_exp_vmax_over_phitd_bot_dn2 = assign30680_e45903_d_n2;

    }
}
