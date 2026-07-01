#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        locals: &mut StampLocals,
    ) {
        let (assign41600_e54498, assign41600_e54498_d_n5, assign41600_e54498_d_n6, assign41600_e54498_d_n7, assign41600_e54498_d_n8,) = {
    if (locals.var_guard1182 != 0.0) {
        let assign41600_e54492: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign41600_e54494: f64 = (assign41600_e54492 * 0.16666666666666666);
        let assign41600_e54496: f64 = (assign41600_e54494 * 0.7071067811865475);
        (assign41600_e54496, ((((locals.var_inv_xi_dn5 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8,)
    }
};
        locals.var_sp_s_temp1 = assign41600_e54498;
        locals.var_sp_s_temp1_dn5 = assign41600_e54498_d_n5;
        locals.var_sp_s_temp1_dn6 = assign41600_e54498_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41600_e54498_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41600_e54498_d_n8;

        let (assign41610_e54516, assign41610_e54516_d_n5, assign41610_e54516_d_n6, assign41610_e54516_d_n7, assign41610_e54516_d_n8,) = {
    if (locals.var_guard1182 != 0.0) {
        let assign41610_e54502: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign41610_e54507: f64 = (1.0 - locals.var_delta_ns);
        let assign41610_e54508: f64 = (locals.var_xg * assign41610_e54507);
        let assign41610_e54510: f64 = (assign41610_e54508 * locals.var_gf);
        let assign41610_e54512: f64 = (assign41610_e54510 * locals.var_sp_s_temp1);
        let assign41610_e54513: f64 = (1.0 + assign41610_e54512);
        let assign41610_e54514: f64 = (assign41610_e54502 * assign41610_e54513);
        (assign41610_e54514, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn5 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn5))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn5)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn6 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn6))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn7 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn7))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn8 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn8))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn8)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8,)
    }
};
        locals.var_x_s = assign41610_e54516;
        locals.var_x_s_dn5 = assign41610_e54516_d_n5;
        locals.var_x_s_dn6 = assign41610_e54516_d_n6;
        locals.var_x_s_dn7 = assign41610_e54516_d_n7;
        locals.var_x_s_dn8 = assign41610_e54516_d_n8;

        let assign41620_e54519: f64 = (-locals.var_margin);
        let assign41620_e54520: f64 = if locals.var_xg < assign41620_e54519 { 1.0 } else { 0.0 };
        locals.var_guard1183 = assign41620_e54520;

        let (assign41630_e54528, assign41630_e54528_d_n5, assign41630_e54528_d_n6, assign41630_e54528_d_n7, assign41630_e54528_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41630_e54526: f64 = (-locals.var_xg);
        (assign41630_e54526, (-locals.var_xg_dn5), (-locals.var_xg_dn6), (-locals.var_xg_dn7), (-locals.var_xg_dn8),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn5, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8,)
    }
};
        locals.var_sp_s_yg = assign41630_e54528;
        locals.var_sp_s_yg_dn5 = assign41630_e54528_d_n5;
        locals.var_sp_s_yg_dn6 = assign41630_e54528_d_n6;
        locals.var_sp_s_yg_dn7 = assign41630_e54528_d_n7;
        locals.var_sp_s_yg_dn8 = assign41630_e54528_d_n8;

        let (assign41640_e54539, assign41640_e54539_d_n5, assign41640_e54539_d_n6, assign41640_e54539_d_n7, assign41640_e54539_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41640_e54536: f64 = (locals.var_sp_s_yg * locals.var_inv_xi);
        let assign41640_e54537: f64 = (1.25 * assign41640_e54536);
        (assign41640_e54537, (1.25 * ((locals.var_sp_s_yg_dn5 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn5))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn8))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn5, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8,)
    }
};
        locals.var_sp_s_ysub = assign41640_e54539;
        locals.var_sp_s_ysub_dn5 = assign41640_e54539_d_n5;
        locals.var_sp_s_ysub_dn6 = assign41640_e54539_d_n6;
        locals.var_sp_s_ysub_dn7 = assign41640_e54539_d_n7;
        locals.var_sp_s_ysub_dn8 = assign41640_e54539_d_n8;

        let (assign41650_e54561, assign41650_e54561_d_n5, assign41650_e54561_d_n6, assign41650_e54561_d_n7, assign41650_e54561_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41650_e54547: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign41650_e54550: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41650_e54553: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41650_e54554: f64 = (assign41650_e54550 * assign41650_e54553);
        let assign41650_e54556: f64 = (assign41650_e54554 + 64.0);
        let assign41650_e54557: f64 = (assign41650_e54556).sqrt();
        let assign41650_e54558: f64 = (assign41650_e54547 - assign41650_e54557);
        let assign41650_e54559: f64 = (0.5 * assign41650_e54558);
        (assign41650_e54559, (0.5 * (locals.var_sp_s_ysub_dn5 - (((locals.var_sp_s_ysub_dn5 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn5)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign41650_e54557)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8,)
    }
};
        locals.var_sp_s_eta = assign41650_e54561;
        locals.var_sp_s_eta_dn5 = assign41650_e54561_d_n5;
        locals.var_sp_s_eta_dn6 = assign41650_e54561_d_n6;
        locals.var_sp_s_eta_dn7 = assign41650_e54561_d_n7;
        locals.var_sp_s_eta_dn8 = assign41650_e54561_d_n8;

        let (assign41660_e54570, assign41660_e54570_d_n5, assign41660_e54570_d_n6, assign41660_e54570_d_n7, assign41660_e54570_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41660_e54568: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign41660_e54568, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41660_e54570;
        locals.var_sp_s_temp_dn5 = assign41660_e54570_d_n5;
        locals.var_sp_s_temp_dn6 = assign41660_e54570_d_n6;
        locals.var_sp_s_temp_dn7 = assign41660_e54570_d_n7;
        locals.var_sp_s_temp_dn8 = assign41660_e54570_d_n8;

        let (assign41670_e54585, assign41670_e54585_d_n5, assign41670_e54585_d_n6, assign41670_e54585_d_n7, assign41670_e54585_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41670_e54577: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign41670_e54581: f64 = (locals.var_sp_s_eta + 1.0);
        let assign41670_e54582: f64 = (locals.var_gf2 * assign41670_e54581);
        let assign41670_e54583: f64 = (assign41670_e54577 + assign41670_e54582);
        (assign41670_e54583, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) + ((locals.var_gf2_dn5 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn5))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gf2_dn6 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gf2_dn7 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gf2_dn8 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn8))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8,)
    }
};
        locals.var_sp_s_a = assign41670_e54585;
        locals.var_sp_s_a_dn5 = assign41670_e54585_d_n5;
        locals.var_sp_s_a_dn6 = assign41670_e54585_d_n6;
        locals.var_sp_s_a_dn7 = assign41670_e54585_d_n7;
        locals.var_sp_s_a_dn8 = assign41670_e54585_d_n8;

        let (assign41680_e54596, assign41680_e54596_d_n5, assign41680_e54596_d_n6, assign41680_e54596_d_n7, assign41680_e54596_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41680_e54592: f64 = (2.0 * locals.var_sp_s_temp);
        let assign41680_e54594: f64 = (assign41680_e54592 - locals.var_gf2);
        (assign41680_e54594, ((2.0 * locals.var_sp_s_temp_dn5) - locals.var_gf2_dn5), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gf2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gf2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gf2_dn8),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8,)
    }
};
        locals.var_sp_s_c = assign41680_e54596;
        locals.var_sp_s_c_dn5 = assign41680_e54596_d_n5;
        locals.var_sp_s_c_dn6 = assign41680_e54596_d_n6;
        locals.var_sp_s_c_dn7 = assign41680_e54596_d_n7;
        locals.var_sp_s_c_dn8 = assign41680_e54596_d_n8;

        let (assign41690_e54609, assign41690_e54609_d_n5, assign41690_e54609_d_n6, assign41690_e54609_d_n7, assign41690_e54609_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41690_e54602: f64 = (-locals.var_sp_s_eta);
        let assign41690_e54605: f64 = (locals.var_sp_s_a * locals.var_inv_gf2);
        let assign41690_e54606: f64 = (assign41690_e54605).ln();
        let assign41690_e54607: f64 = (assign41690_e54602 + assign41690_e54606);
        (assign41690_e54607, ((-locals.var_sp_s_eta_dn5) + (((locals.var_sp_s_a_dn5 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn5)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn6) + (((locals.var_sp_s_a_dn6 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn6)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn7) + (((locals.var_sp_s_a_dn7 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn7)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn8) + (((locals.var_sp_s_a_dn8 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn8)) / assign41690_e54605)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8,)
    }
};
        locals.var_sp_s_tau = assign41690_e54609;
        locals.var_sp_s_tau_dn5 = assign41690_e54609_d_n5;
        locals.var_sp_s_tau_dn6 = assign41690_e54609_d_n6;
        locals.var_sp_s_tau_dn7 = assign41690_e54609_d_n7;
        locals.var_sp_s_tau_dn8 = assign41690_e54609_d_n8;

        let (assign41700_e54618, assign41700_e54618_d_n5, assign41700_e54618_d_n6, assign41700_e54618_d_n7, assign41700_e54618_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41700_e54616: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign41700_e54616, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign41700_e54618;
        locals.var_nu_dn5 = assign41700_e54618_d_n5;
        locals.var_nu_dn6 = assign41700_e54618_d_n6;
        locals.var_nu_dn7 = assign41700_e54618_d_n7;
        locals.var_nu_dn8 = assign41700_e54618_d_n8;

        let (assign41710_e54637, assign41710_e54637_d_n5, assign41710_e54637_d_n6, assign41710_e54637_d_n7, assign41710_e54637_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41710_e54625: f64 = (locals.var_nu * locals.var_nu);
        let assign41710_e54630: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41710_e54631: f64 = (0.5 * assign41710_e54630);
        let assign41710_e54633: f64 = (assign41710_e54631 - locals.var_sp_s_a);
        let assign41710_e54634: f64 = (locals.var_sp_s_tau * assign41710_e54633);
        let assign41710_e54635: f64 = (assign41710_e54625 + assign41710_e54634);
        (assign41710_e54635, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - locals.var_sp_s_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign41710_e54637;
        locals.var_mutau_dn5 = assign41710_e54637_d_n5;
        locals.var_mutau_dn6 = assign41710_e54637_d_n6;
        locals.var_mutau_dn7 = assign41710_e54637_d_n7;
        locals.var_mutau_dn8 = assign41710_e54637_d_n8;

        let (assign41720_e54670, assign41720_e54670_d_n5, assign41720_e54670_d_n6, assign41720_e54670_d_n7, assign41720_e54670_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41720_e54645: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign41720_e54647: f64 = (assign41720_e54645 * locals.var_sp_s_tau);
        let assign41720_e54651: f64 = (locals.var_nu / locals.var_mutau);
        let assign41720_e54653: f64 = (assign41720_e54651 * locals.var_sp_s_tau);
        let assign41720_e54655: f64 = (assign41720_e54653 * locals.var_sp_s_tau);
        let assign41720_e54657: f64 = (assign41720_e54655 * locals.var_sp_s_c);
        let assign41720_e54660: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41720_e54662: f64 = (assign41720_e54660 * 0.3333333333333333);
        let assign41720_e54664: f64 = (assign41720_e54662 - locals.var_sp_s_a);
        let assign41720_e54665: f64 = (assign41720_e54657 * assign41720_e54664);
        let assign41720_e54666: f64 = (locals.var_mutau + assign41720_e54665);
        let assign41720_e54667: f64 = (assign41720_e54647 / assign41720_e54666);
        let assign41720_e54668: f64 = (locals.var_sp_s_eta + assign41720_e54667);
        (assign41720_e54668, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn5)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn5)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - locals.var_sp_s_a_dn5)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn6)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn6)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn7)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn7)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn8)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn8)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign41720_e54666 * assign41720_e54666))),)
    } else {
        (locals.var_sp_s_y0, locals.var_sp_s_y0_dn5, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8,)
    }
};
        locals.var_sp_s_y0 = assign41720_e54670;
        locals.var_sp_s_y0_dn5 = assign41720_e54670_d_n5;
        locals.var_sp_s_y0_dn6 = assign41720_e54670_d_n6;
        locals.var_sp_s_y0_dn7 = assign41720_e54670_d_n7;
        locals.var_sp_s_y0_dn8 = assign41720_e54670_d_n8;

        let assign41730_e54673: f64 = if locals.var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1184 = assign41730_e54673;

        let (assign41740_e54683, assign41740_e54683_d_n5, assign41740_e54683_d_n6, assign41740_e54683_d_n7, assign41740_e54683_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign41740_e54681: f64 = (locals.var_sp_s_y0).exp();
        (assign41740_e54681, (assign41740_e54681 * locals.var_sp_s_y0_dn5), (assign41740_e54681 * locals.var_sp_s_y0_dn6), (assign41740_e54681 * locals.var_sp_s_y0_dn7), (assign41740_e54681 * locals.var_sp_s_y0_dn8),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign41740_e54683;
        locals.var_sp_s_delta0_dn5 = assign41740_e54683_d_n5;
        locals.var_sp_s_delta0_dn6 = assign41740_e54683_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41740_e54683_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41740_e54683_d_n8;

        let (assign41750_e54715, assign41750_e54715_d_n5, assign41750_e54715_d_n6, assign41750_e54715_d_n7, assign41750_e54715_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign41750_e54695: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54700: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54704: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54706: f64 = (assign41750_e54704 * 0.3333333333333333);
        let assign41750_e54707: f64 = (1.0 + assign41750_e54706);
        let assign41750_e54708: f64 = (assign41750_e54700 * assign41750_e54707);
        let assign41750_e54709: f64 = (0.5 * assign41750_e54708);
        let assign41750_e54710: f64 = (1.0 + assign41750_e54709);
        let assign41750_e54711: f64 = (assign41750_e54695 * assign41750_e54710);
        let assign41750_e54712: f64 = (1.0 + assign41750_e54711);
        let assign41750_e54713: f64 = (1e100 * assign41750_e54712);
        (assign41750_e54713, (1e100 * ((locals.var_sp_s_y0_dn5 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn5 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn6 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn6 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn7 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn7 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn8 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn8 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign41750_e54715;
        locals.var_sp_s_delta0_dn5 = assign41750_e54715_d_n5;
        locals.var_sp_s_delta0_dn6 = assign41750_e54715_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41750_e54715_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41750_e54715_d_n8;

        let (assign41760_e54724, assign41760_e54724_d_n5, assign41760_e54724_d_n6, assign41760_e54724_d_n7, assign41760_e54724_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41760_e54722: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign41760_e54722, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign41760_e54724;
        locals.var_sp_s_delta1_dn5 = assign41760_e54724_d_n5;
        locals.var_sp_s_delta1_dn6 = assign41760_e54724_d_n6;
        locals.var_sp_s_delta1_dn7 = assign41760_e54724_d_n7;
        locals.var_sp_s_delta1_dn8 = assign41760_e54724_d_n8;

        let (assign41770_e54737, assign41770_e54737_d_n5, assign41770_e54737_d_n6, assign41770_e54737_d_n7, assign41770_e54737_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41770_e54733: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41770_e54734: f64 = (2.0 + assign41770_e54733);
        let assign41770_e54735: f64 = (1.0 / assign41770_e54734);
        (assign41770_e54735, (-(((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign41770_e54734 * assign41770_e54734))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41770_e54737;
        locals.var_sp_s_temp_dn5 = assign41770_e54737_d_n5;
        locals.var_sp_s_temp_dn6 = assign41770_e54737_d_n6;
        locals.var_sp_s_temp_dn7 = assign41770_e54737_d_n7;
        locals.var_sp_s_temp_dn8 = assign41770_e54737_d_n8;

        let (assign41780_e54748, assign41780_e54748_d_n5, assign41780_e54748_d_n6, assign41780_e54748_d_n7, assign41780_e54748_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41780_e54744: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41780_e54746: f64 = (assign41780_e54744 * locals.var_sp_s_temp);
        (assign41780_e54746, ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8,)
    }
};
        locals.var_sp_s_xi0 = assign41780_e54748;
        locals.var_sp_s_xi0_dn5 = assign41780_e54748_d_n5;
        locals.var_sp_s_xi0_dn6 = assign41780_e54748_d_n6;
        locals.var_sp_s_xi0_dn7 = assign41780_e54748_d_n7;
        locals.var_sp_s_xi0_dn8 = assign41780_e54748_d_n8;

        let (assign41790_e54761, assign41790_e54761_d_n5, assign41790_e54761_d_n6, assign41790_e54761_d_n7, assign41790_e54761_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41790_e54756: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
        let assign41790_e54758: f64 = (assign41790_e54756 * locals.var_sp_s_temp);
        let assign41790_e54759: f64 = (4.0 * assign41790_e54758);
        (assign41790_e54759, (4.0 * ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn8))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8,)
    }
};
        locals.var_sp_s_xi1 = assign41790_e54761;
        locals.var_sp_s_xi1_dn5 = assign41790_e54761_d_n5;
        locals.var_sp_s_xi1_dn6 = assign41790_e54761_d_n6;
        locals.var_sp_s_xi1_dn7 = assign41790_e54761_d_n7;
        locals.var_sp_s_xi1_dn8 = assign41790_e54761_d_n8;

        let (assign41800_e54778, assign41800_e54778_d_n5, assign41800_e54778_d_n6, assign41800_e54778_d_n7, assign41800_e54778_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41800_e54768: f64 = (8.0 * locals.var_sp_s_temp);
        let assign41800_e54771: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign41800_e54772: f64 = (assign41800_e54768 - assign41800_e54771);
        let assign41800_e54774: f64 = (assign41800_e54772 * locals.var_sp_s_temp);
        let assign41800_e54776: f64 = (assign41800_e54774 * locals.var_sp_s_temp);
        (assign41800_e54776, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8,)
    }
};
        locals.var_sp_s_xi2 = assign41800_e54778;
        locals.var_sp_s_xi2_dn5 = assign41800_e54778_d_n5;
        locals.var_sp_s_xi2_dn6 = assign41800_e54778_d_n6;
        locals.var_sp_s_xi2_dn7 = assign41800_e54778_d_n7;
        locals.var_sp_s_xi2_dn8 = assign41800_e54778_d_n8;

        let (assign41810_e54787, assign41810_e54787_d_n5, assign41810_e54787_d_n6, assign41810_e54787_d_n7, assign41810_e54787_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41810_e54785: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
        (assign41810_e54785, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_y0_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41810_e54787;
        locals.var_sp_s_temp_dn5 = assign41810_e54787_d_n5;
        locals.var_sp_s_temp_dn6 = assign41810_e54787_d_n6;
        locals.var_sp_s_temp_dn7 = assign41810_e54787_d_n7;
        locals.var_sp_s_temp_dn8 = assign41810_e54787_d_n8;

        let (assign41820_e54796, assign41820_e54796_d_n5, assign41820_e54796_d_n6, assign41820_e54796_d_n7, assign41820_e54796_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41820_e54794: f64 = (locals.var_delta_ns * locals.var_sp_s_delta1);
        (assign41820_e54794, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn8)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8,)
    }
};
        locals.var_sp_s_temp1 = assign41820_e54796;
        locals.var_sp_s_temp1_dn5 = assign41820_e54796_d_n5;
        locals.var_sp_s_temp1_dn6 = assign41820_e54796_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41820_e54796_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41820_e54796_d_n8;

        let (assign41830_e54819, assign41830_e54819_d_n5, assign41830_e54819_d_n6, assign41830_e54819_d_n7, assign41830_e54819_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41830_e54803: f64 = (2.0 * locals.var_sp_s_temp);
        let assign41830_e54807: f64 = (locals.var_sp_s_delta0 - 1.0);
        let assign41830_e54809: f64 = (assign41830_e54807 - locals.var_sp_s_temp1);
        let assign41830_e54813: f64 = (1.0 - locals.var_sp_s_xi1);
        let assign41830_e54814: f64 = (locals.var_delta_ns * assign41830_e54813);
        let assign41830_e54815: f64 = (assign41830_e54809 + assign41830_e54814);
        let assign41830_e54816: f64 = (locals.var_gf2 * assign41830_e54815);
        let assign41830_e54817: f64 = (assign41830_e54803 + assign41830_e54816);
        (assign41830_e54817, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn5))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn8))))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8,)
    }
};
        locals.var_sp_s_pc = assign41830_e54819;
        locals.var_sp_s_pc_dn5 = assign41830_e54819_d_n5;
        locals.var_sp_s_pc_dn6 = assign41830_e54819_d_n6;
        locals.var_sp_s_pc_dn7 = assign41830_e54819_d_n7;
        locals.var_sp_s_pc_dn8 = assign41830_e54819_d_n8;

        let (assign41840_e54846, assign41840_e54846_d_n5, assign41840_e54846_d_n6, assign41840_e54846_d_n7, assign41840_e54846_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41840_e54826: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign41840_e54830: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
        let assign41840_e54832: f64 = (assign41840_e54830 - 1.0);
        let assign41840_e54834: f64 = (assign41840_e54832 + locals.var_sp_s_temp1);
        let assign41840_e54838: f64 = (locals.var_sp_s_y0 - 1.0);
        let assign41840_e54840: f64 = (assign41840_e54838 - locals.var_sp_s_xi0);
        let assign41840_e54841: f64 = (locals.var_delta_ns * assign41840_e54840);
        let assign41840_e54842: f64 = (assign41840_e54834 + assign41840_e54841);
        let assign41840_e54843: f64 = (locals.var_gf2 * assign41840_e54842);
        let assign41840_e54844: f64 = (assign41840_e54826 - assign41840_e54843);
        (assign41840_e54844, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_y0_dn5) + locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn5 - locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8,)
    }
};
        locals.var_sp_s_qc = assign41840_e54846;
        locals.var_sp_s_qc_dn5 = assign41840_e54846_d_n5;
        locals.var_sp_s_qc_dn6 = assign41840_e54846_d_n6;
        locals.var_sp_s_qc_dn7 = assign41840_e54846_d_n7;
        locals.var_sp_s_qc_dn8 = assign41840_e54846_d_n8;

        let (assign41850_e54863, assign41850_e54863_d_n5, assign41850_e54863_d_n6, assign41850_e54863_d_n7, assign41850_e54863_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41850_e54855: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
        let assign41850_e54858: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign41850_e54859: f64 = (assign41850_e54855 - assign41850_e54858);
        let assign41850_e54860: f64 = (locals.var_gf2 * assign41850_e54859);
        let assign41850_e54861: f64 = (2.0 - assign41850_e54860);
        (assign41850_e54861, (-((locals.var_gf2_dn5 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 + locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41850_e54863;
        locals.var_sp_s_temp_dn5 = assign41850_e54863_d_n5;
        locals.var_sp_s_temp_dn6 = assign41850_e54863_d_n6;
        locals.var_sp_s_temp_dn7 = assign41850_e54863_d_n7;
        locals.var_sp_s_temp_dn8 = assign41850_e54863_d_n8;

        let (assign41860_e54878, assign41860_e54878_d_n5, assign41860_e54878_d_n6, assign41860_e54878_d_n7, assign41860_e54878_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41860_e54870: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign41860_e54874: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign41860_e54875: f64 = (2.0 * assign41860_e54874);
        let assign41860_e54876: f64 = (assign41860_e54870 - assign41860_e54875);
        (assign41860_e54876, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41860_e54878;
        locals.var_sp_s_temp_dn5 = assign41860_e54878_d_n5;
        locals.var_sp_s_temp_dn6 = assign41860_e54878_d_n6;
        locals.var_sp_s_temp_dn7 = assign41860_e54878_d_n7;
        locals.var_sp_s_temp_dn8 = assign41860_e54878_d_n8;

        let (assign41870_e54895, assign41870_e54895_d_n5, assign41870_e54895_d_n6, assign41870_e54895_d_n7, assign41870_e54895_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign41870_e54884: f64 = (-locals.var_sp_s_y0);
        let assign41870_e54889: f64 = (locals.var_sp_s_temp).sqrt();
        let assign41870_e54890: f64 = (locals.var_sp_s_pc + assign41870_e54889);
        let assign41870_e54891: f64 = (locals.var_sp_s_qc / assign41870_e54890);
        let assign41870_e54892: f64 = (2.0 * assign41870_e54891);
        let assign41870_e54893: f64 = (assign41870_e54884 - assign41870_e54892);
        (assign41870_e54893, ((-locals.var_sp_s_y0_dn5) - (2.0 * (((locals.var_sp_s_qc_dn5 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8,)
    }
};
        locals.var_x_s = assign41870_e54895;
        locals.var_x_s_dn5 = assign41870_e54895_d_n5;
        locals.var_x_s_dn6 = assign41870_e54895_d_n6;
        locals.var_x_s_dn7 = assign41870_e54895_d_n7;
        locals.var_x_s_dn8 = assign41870_e54895_d_n8;

        let (assign41880_e54909, assign41880_e54909_d_n5, assign41880_e54909_d_n6, assign41880_e54909_d_n7, assign41880_e54909_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41880_e54905: f64 = (locals.var_gf * 0.7324648775608221);
        let assign41880_e54906: f64 = (1.25 + assign41880_e54905);
        let assign41880_e54907: f64 = (1.0 / assign41880_e54906);
        (assign41880_e54907, (-((locals.var_gf_dn5 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn6 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn7 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn8 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))),)
    } else {
        (locals.var_sp_xg1, locals.var_sp_xg1_dn5, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8,)
    }
};
        locals.var_sp_xg1 = assign41880_e54909;
        locals.var_sp_xg1_dn5 = assign41880_e54909_d_n5;
        locals.var_sp_xg1_dn6 = assign41880_e54909_d_n6;
        locals.var_sp_xg1_dn7 = assign41880_e54909_d_n7;
        locals.var_sp_xg1_dn8 = assign41880_e54909_d_n8;

        let (assign41890_e54925, assign41890_e54925_d_n5, assign41890_e54925_d_n6, assign41890_e54925_d_n7, assign41890_e54925_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41890_e54917: f64 = (locals.var_xi * 1.25);
        let assign41890_e54919: f64 = (assign41890_e54917 * locals.var_sp_xg1);
        let assign41890_e54921: f64 = (assign41890_e54919 - 1.0);
        let assign41890_e54923: f64 = (assign41890_e54921 * locals.var_sp_xg1);
        (assign41890_e54923, (((((locals.var_xi_dn5 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn5)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn5)), (((((locals.var_xi_dn6 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn6)), (((((locals.var_xi_dn7 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn7)), (((((locals.var_xi_dn8 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn8)),)
    } else {
        (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn5, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8,)
    }
};
        locals.var_sp_s_a_fac = assign41890_e54925;
        locals.var_sp_s_a_fac_dn5 = assign41890_e54925_d_n5;
        locals.var_sp_s_a_fac_dn6 = assign41890_e54925_d_n6;
        locals.var_sp_s_a_fac_dn7 = assign41890_e54925_d_n7;
        locals.var_sp_s_a_fac_dn8 = assign41890_e54925_d_n8;

        let (assign41900_e54941, assign41900_e54941_d_n5, assign41900_e54941_d_n6, assign41900_e54941_d_n7, assign41900_e54941_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41900_e54933: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign41900_e54937: f64 = (locals.var_sp_s_a_fac * locals.var_xg);
        let assign41900_e54938: f64 = (1.0 + assign41900_e54937);
        let assign41900_e54939: f64 = (assign41900_e54933 * assign41900_e54938);
        (assign41900_e54939, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn5 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn6 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn7 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn8 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn8)))),)
    } else {
        (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn5, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8,)
    }
};
        locals.var_sp_s_xbar = assign41900_e54941;
        locals.var_sp_s_xbar_dn5 = assign41900_e54941_d_n5;
        locals.var_sp_s_xbar_dn6 = assign41900_e54941_d_n6;
        locals.var_sp_s_xbar_dn7 = assign41900_e54941_d_n7;
        locals.var_sp_s_xbar_dn8 = assign41900_e54941_d_n8;

    }

    pub(super) fn stamp_transient_block_17(
        locals: &mut StampLocals,
    ) {
        let assign41910_e54943: f64 = (-locals.var_sp_s_xbar);
        let assign41910_e54945: f64 = (-230.25850929940458);
        let assign41910_e54946: f64 = if assign41910_e54943 > assign41910_e54945 { 1.0 } else { 0.0 };
        locals.var_guard1185 = assign41910_e54946;

        let (assign41920_e54958, assign41920_e54958_d_n5, assign41920_e54958_d_n6, assign41920_e54958_d_n7, assign41920_e54958_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1185 != 0.0)) {
        let assign41920_e54955: f64 = (-locals.var_sp_s_xbar);
        let assign41920_e54956: f64 = (assign41920_e54955).exp();
        (assign41920_e54956, (assign41920_e54956 * (-locals.var_sp_s_xbar_dn5)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn6)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn7)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn8)),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41920_e54958;
        locals.var_sp_s_temp_dn5 = assign41920_e54958_d_n5;
        locals.var_sp_s_temp_dn6 = assign41920_e54958_d_n6;
        locals.var_sp_s_temp_dn7 = assign41920_e54958_d_n7;
        locals.var_sp_s_temp_dn8 = assign41920_e54958_d_n8;

        let (assign41930_e54997, assign41930_e54997_d_n5, assign41930_e54997_d_n6, assign41930_e54997_d_n7, assign41930_e54997_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1185 == 0.0)) {
        let assign41930_e54970: f64 = (-230.25850929940458);
        let assign41930_e54972: f64 = (-locals.var_sp_s_xbar);
        let assign41930_e54973: f64 = (assign41930_e54970 - assign41930_e54972);
        let assign41930_e54977: f64 = (-230.25850929940458);
        let assign41930_e54979: f64 = (-locals.var_sp_s_xbar);
        let assign41930_e54980: f64 = (assign41930_e54977 - assign41930_e54979);
        let assign41930_e54983: f64 = (-230.25850929940458);
        let assign41930_e54985: f64 = (-locals.var_sp_s_xbar);
        let assign41930_e54986: f64 = (assign41930_e54983 - assign41930_e54985);
        let assign41930_e54988: f64 = (assign41930_e54986 * 0.3333333333333333);
        let assign41930_e54989: f64 = (1.0 + assign41930_e54988);
        let assign41930_e54990: f64 = (assign41930_e54980 * assign41930_e54989);
        let assign41930_e54991: f64 = (0.5 * assign41930_e54990);
        let assign41930_e54992: f64 = (1.0 + assign41930_e54991);
        let assign41930_e54993: f64 = (assign41930_e54973 * assign41930_e54992);
        let assign41930_e54994: f64 = (1.0 + assign41930_e54993);
        let assign41930_e54995: f64 = (1e-100 / assign41930_e54994);
        (assign41930_e54995, (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn5)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn5)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn5)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn6)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn6)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn7)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn7)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn8)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn8)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41930_e54997;
        locals.var_sp_s_temp_dn5 = assign41930_e54997_d_n5;
        locals.var_sp_s_temp_dn6 = assign41930_e54997_d_n6;
        locals.var_sp_s_temp_dn7 = assign41930_e54997_d_n7;
        locals.var_sp_s_temp_dn8 = assign41930_e54997_d_n8;

        let (assign41940_e55007, assign41940_e55007_d_n5, assign41940_e55007_d_n6, assign41940_e55007_d_n7, assign41940_e55007_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41940_e55005: f64 = (1.0 - locals.var_sp_s_temp);
        (assign41940_e55005, (-locals.var_sp_s_temp_dn5), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8),)
    } else {
        (locals.var_sp_s_w, locals.var_sp_s_w_dn5, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8,)
    }
};
        locals.var_sp_s_w = assign41940_e55007;
        locals.var_sp_s_w_dn5 = assign41940_e55007_d_n5;
        locals.var_sp_s_w_dn6 = assign41940_e55007_d_n6;
        locals.var_sp_s_w_dn7 = assign41940_e55007_d_n7;
        locals.var_sp_s_w_dn8 = assign41940_e55007_d_n8;

        let (assign41950_e55030, assign41950_e55030_d_n5, assign41950_e55030_d_n6, assign41950_e55030_d_n7, assign41950_e55030_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41950_e55016: f64 = (locals.var_gf2 * 0.5);
        let assign41950_e55017: f64 = (locals.var_xg + assign41950_e55016);
        let assign41950_e55022: f64 = (locals.var_gf2 * 0.25);
        let assign41950_e55023: f64 = (locals.var_xg + assign41950_e55022);
        let assign41950_e55025: f64 = (assign41950_e55023 - locals.var_sp_s_w);
        let assign41950_e55026: f64 = (assign41950_e55025).sqrt();
        let assign41950_e55027: f64 = (locals.var_gf * assign41950_e55026);
        let assign41950_e55028: f64 = (assign41950_e55017 - assign41950_e55027);
        (assign41950_e55028, ((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.5)) - ((locals.var_gf_dn5 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.25)) - locals.var_sp_s_w_dn5) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.5)) - ((locals.var_gf_dn6 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.5)) - ((locals.var_gf_dn7 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.5)) - ((locals.var_gf_dn8 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign41950_e55026))))),)
    } else {
        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8,)
    }
};
        locals.var_sp_s_x1 = assign41950_e55030;
        locals.var_sp_s_x1_dn5 = assign41950_e55030_d_n5;
        locals.var_sp_s_x1_dn6 = assign41950_e55030_d_n6;
        locals.var_sp_s_x1_dn7 = assign41950_e55030_d_n7;
        locals.var_sp_s_x1_dn8 = assign41950_e55030_d_n8;

        let (assign41960_e55040, assign41960_e55040_d_n5, assign41960_e55040_d_n6, assign41960_e55040_d_n7, assign41960_e55040_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41960_e55038: f64 = (locals.var_xn_s + 3.0);
        (assign41960_e55038, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8,)
    }
};
        locals.var_sp_s_bx = assign41960_e55040;
        locals.var_sp_s_bx_dn5 = assign41960_e55040_d_n5;
        locals.var_sp_s_bx_dn6 = assign41960_e55040_d_n6;
        locals.var_sp_s_bx_dn7 = assign41960_e55040_d_n7;
        locals.var_sp_s_bx_dn8 = assign41960_e55040_d_n8;

        let (assign41970_e55074, assign41970_e55074_d_n5, assign41970_e55074_d_n6, assign41970_e55074_d_n7, assign41970_e55074_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41970_e55049: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign41970_e55052: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign41970_e55055: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign41970_e55056: f64 = (assign41970_e55052 * assign41970_e55055);
        let assign41970_e55058: f64 = (assign41970_e55056 + 5.0);
        let assign41970_e55059: f64 = (assign41970_e55058).sqrt();
        let assign41970_e55060: f64 = (assign41970_e55049 - assign41970_e55059);
        let assign41970_e55061: f64 = (0.5 * assign41970_e55060);
        let assign41970_e55066: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign41970_e55068: f64 = (assign41970_e55066 + 5.0);
        let assign41970_e55069: f64 = (assign41970_e55068).sqrt();
        let assign41970_e55070: f64 = (locals.var_sp_s_bx - assign41970_e55069);
        let assign41970_e55071: f64 = (0.5 * assign41970_e55070);
        let assign41970_e55072: f64 = (assign41970_e55061 - assign41970_e55071);
        (assign41970_e55072, ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign41970_e55069))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8,)
    }
};
        locals.var_sp_s_eta = assign41970_e55074;
        locals.var_sp_s_eta_dn5 = assign41970_e55074_d_n5;
        locals.var_sp_s_eta_dn6 = assign41970_e55074_d_n6;
        locals.var_sp_s_eta_dn7 = assign41970_e55074_d_n7;
        locals.var_sp_s_eta_dn8 = assign41970_e55074_d_n8;

        let (assign41980_e55084, assign41980_e55084_d_n5, assign41980_e55084_d_n6, assign41980_e55084_d_n7, assign41980_e55084_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41980_e55082: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign41980_e55082, (locals.var_xg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign41980_e55084;
        locals.var_sp_s_temp_dn5 = assign41980_e55084_d_n5;
        locals.var_sp_s_temp_dn6 = assign41980_e55084_d_n6;
        locals.var_sp_s_temp_dn7 = assign41980_e55084_d_n7;
        locals.var_sp_s_temp_dn8 = assign41980_e55084_d_n8;

        let (assign41990_e55094, assign41990_e55094_d_n5, assign41990_e55094_d_n6, assign41990_e55094_d_n7, assign41990_e55094_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign41990_e55091: f64 = (-locals.var_sp_s_eta);
        let assign41990_e55092: f64 = (assign41990_e55091).exp();
        (assign41990_e55092, (assign41990_e55092 * (-locals.var_sp_s_eta_dn5)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn6)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn7)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn8)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8,)
    }
};
        locals.var_sp_s_temp1 = assign41990_e55094;
        locals.var_sp_s_temp1_dn5 = assign41990_e55094_d_n5;
        locals.var_sp_s_temp1_dn6 = assign41990_e55094_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41990_e55094_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41990_e55094_d_n8;

        let (assign42000_e55108, assign42000_e55108_d_n5, assign42000_e55108_d_n6, assign42000_e55108_d_n7, assign42000_e55108_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42000_e55104: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42000_e55105: f64 = (2.0 + assign42000_e55104);
        let assign42000_e55106: f64 = (1.0 / assign42000_e55105);
        (assign42000_e55106, (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign42000_e55105 * assign42000_e55105))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8,)
    }
};
        locals.var_sp_s_temp2 = assign42000_e55108;
        locals.var_sp_s_temp2_dn5 = assign42000_e55108_d_n5;
        locals.var_sp_s_temp2_dn6 = assign42000_e55108_d_n6;
        locals.var_sp_s_temp2_dn7 = assign42000_e55108_d_n7;
        locals.var_sp_s_temp2_dn8 = assign42000_e55108_d_n8;

        let (assign42010_e55120, assign42010_e55120_d_n5, assign42010_e55120_d_n6, assign42010_e55120_d_n7, assign42010_e55120_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42010_e55116: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42010_e55118: f64 = (assign42010_e55116 * locals.var_sp_s_temp2);
        (assign42010_e55118, ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn8)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8,)
    }
};
        locals.var_sp_s_xi0 = assign42010_e55120;
        locals.var_sp_s_xi0_dn5 = assign42010_e55120_d_n5;
        locals.var_sp_s_xi0_dn6 = assign42010_e55120_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42010_e55120_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42010_e55120_d_n8;

        let (assign42020_e55134, assign42020_e55134_d_n5, assign42020_e55134_d_n6, assign42020_e55134_d_n7, assign42020_e55134_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42020_e55129: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign42020_e55131: f64 = (assign42020_e55129 * locals.var_sp_s_temp2);
        let assign42020_e55132: f64 = (4.0 * assign42020_e55131);
        (assign42020_e55132, (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn8))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8,)
    }
};
        locals.var_sp_s_xi1 = assign42020_e55134;
        locals.var_sp_s_xi1_dn5 = assign42020_e55134_d_n5;
        locals.var_sp_s_xi1_dn6 = assign42020_e55134_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42020_e55134_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42020_e55134_d_n8;

        let (assign42030_e55152, assign42030_e55152_d_n5, assign42030_e55152_d_n6, assign42030_e55152_d_n7, assign42030_e55152_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42030_e55142: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign42030_e55145: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42030_e55146: f64 = (assign42030_e55142 - assign42030_e55145);
        let assign42030_e55148: f64 = (assign42030_e55146 * locals.var_sp_s_temp2);
        let assign42030_e55150: f64 = (assign42030_e55148 * locals.var_sp_s_temp2);
        (assign42030_e55150, ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn8)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8,)
    }
};
        locals.var_sp_s_xi2 = assign42030_e55152;
        locals.var_sp_s_xi2_dn5 = assign42030_e55152_d_n5;
        locals.var_sp_s_xi2_dn6 = assign42030_e55152_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42030_e55152_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42030_e55152_d_n8;

        let (assign42040_e55201, assign42040_e55201_d_n5, assign42040_e55201_d_n6, assign42040_e55201_d_n7, assign42040_e55201_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42040_e55161: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42040_e55165: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign42040_e55167: f64 = (assign42040_e55165 - 1.0);
        let assign42040_e55171: f64 = (locals.var_sp_s_eta + 1.0);
        let assign42040_e55173: f64 = (assign42040_e55171 + locals.var_sp_s_xi0);
        let assign42040_e55174: f64 = (locals.var_delta_ns * assign42040_e55173);
        let assign42040_e55175: f64 = (assign42040_e55167 - assign42040_e55174);
        let assign42040_e55176: f64 = (locals.var_gf2 * assign42040_e55175);
        let assign42040_e55177: f64 = (assign42040_e55161 - assign42040_e55176);
        let (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,) = {
            if (1e-40 > assign42040_e55177) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42040_e55182: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign42040_e55186: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign42040_e55188: f64 = (assign42040_e55186 - 1.0);
                let assign42040_e55192: f64 = (locals.var_sp_s_eta + 1.0);
                let assign42040_e55194: f64 = (assign42040_e55192 + locals.var_sp_s_xi0);
                let assign42040_e55195: f64 = (locals.var_delta_ns * assign42040_e55194);
                let assign42040_e55196: f64 = (assign42040_e55188 - assign42040_e55195);
                let assign42040_e55197: f64 = (locals.var_gf2 * assign42040_e55196);
                let assign42040_e55198: f64 = (assign42040_e55182 - assign42040_e55197);
                (assign42040_e55198, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_delta_ns_dn5 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_ns_dn6 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_ns_dn7 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_ns_dn8 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8,)
    }
};
        locals.var_sp_s_a = assign42040_e55201;
        locals.var_sp_s_a_dn5 = assign42040_e55201_d_n5;
        locals.var_sp_s_a_dn6 = assign42040_e55201_d_n6;
        locals.var_sp_s_a_dn7 = assign42040_e55201_d_n7;
        locals.var_sp_s_a_dn8 = assign42040_e55201_d_n8;

        let (assign42050_e55219, assign42050_e55219_d_n5, assign42050_e55219_d_n6, assign42050_e55219_d_n7, assign42050_e55219_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42050_e55213: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42050_e55214: f64 = (locals.var_sp_s_temp1 - assign42050_e55213);
        let assign42050_e55215: f64 = (locals.var_gf2 * assign42050_e55214);
        let assign42050_e55216: f64 = (0.5 * assign42050_e55215);
        let assign42050_e55217: f64 = (1.0 - assign42050_e55216);
        (assign42050_e55217, (-(0.5 * ((locals.var_gf2_dn5 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8,)
    }
};
        locals.var_sp_s_b = assign42050_e55219;
        locals.var_sp_s_b_dn5 = assign42050_e55219_d_n5;
        locals.var_sp_s_b_dn6 = assign42050_e55219_d_n6;
        locals.var_sp_s_b_dn7 = assign42050_e55219_d_n7;
        locals.var_sp_s_b_dn8 = assign42050_e55219_d_n8;

        let (assign42060_e55241, assign42060_e55241_d_n5, assign42060_e55241_d_n6, assign42060_e55241_d_n7, assign42060_e55241_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42060_e55227: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42060_e55231: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign42060_e55235: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42060_e55236: f64 = (locals.var_delta_ns * assign42060_e55235);
        let assign42060_e55237: f64 = (assign42060_e55231 - assign42060_e55236);
        let assign42060_e55238: f64 = (locals.var_gf2 * assign42060_e55237);
        let assign42060_e55239: f64 = (assign42060_e55227 + assign42060_e55238);
        (assign42060_e55239, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8,)
    }
};
        locals.var_sp_s_c = assign42060_e55241;
        locals.var_sp_s_c_dn5 = assign42060_e55241_d_n5;
        locals.var_sp_s_c_dn6 = assign42060_e55241_d_n6;
        locals.var_sp_s_c_dn7 = assign42060_e55241_d_n7;
        locals.var_sp_s_c_dn8 = assign42060_e55241_d_n8;

        let (assign42070_e55256, assign42070_e55256_d_n5, assign42070_e55256_d_n6, assign42070_e55256_d_n7, assign42070_e55256_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42070_e55249: f64 = (locals.var_xn_s - locals.var_sp_s_eta);
        let assign42070_e55252: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign42070_e55253: f64 = (assign42070_e55252).ln();
        let assign42070_e55254: f64 = (assign42070_e55249 + assign42070_e55253);
        (assign42070_e55254, ((locals.var_xn_s_dn5 - locals.var_sp_s_eta_dn5) + ((((locals.var_sp_s_a_dn5 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn5)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8,)
    }
};
        locals.var_sp_s_tau = assign42070_e55256;
        locals.var_sp_s_tau_dn5 = assign42070_e55256_d_n5;
        locals.var_sp_s_tau_dn6 = assign42070_e55256_d_n6;
        locals.var_sp_s_tau_dn7 = assign42070_e55256_d_n7;
        locals.var_sp_s_tau_dn8 = assign42070_e55256_d_n8;

        let (assign42080_e55266, assign42080_e55266_d_n5, assign42080_e55266_d_n6, assign42080_e55266_d_n7, assign42080_e55266_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42080_e55264: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign42080_e55264, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign42080_e55266;
        locals.var_nu_dn5 = assign42080_e55266_d_n5;
        locals.var_nu_dn6 = assign42080_e55266_d_n6;
        locals.var_nu_dn7 = assign42080_e55266_d_n7;
        locals.var_nu_dn8 = assign42080_e55266_d_n8;

        let (assign42090_e55288, assign42090_e55288_d_n5, assign42090_e55288_d_n6, assign42090_e55288_d_n7, assign42090_e55288_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42090_e55274: f64 = (locals.var_nu * locals.var_nu);
        let assign42090_e55279: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42090_e55280: f64 = (0.5 * assign42090_e55279);
        let assign42090_e55283: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42090_e55284: f64 = (assign42090_e55280 - assign42090_e55283);
        let assign42090_e55285: f64 = (locals.var_sp_s_tau * assign42090_e55284);
        let assign42090_e55286: f64 = (assign42090_e55274 + assign42090_e55285);
        (assign42090_e55286, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign42090_e55288;
        locals.var_mutau_dn5 = assign42090_e55288_d_n5;
        locals.var_mutau_dn6 = assign42090_e55288_d_n6;
        locals.var_mutau_dn7 = assign42090_e55288_d_n7;
        locals.var_mutau_dn8 = assign42090_e55288_d_n8;

        let (assign42100_e55324, assign42100_e55324_d_n5, assign42100_e55324_d_n6, assign42100_e55324_d_n7, assign42100_e55324_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42100_e55297: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign42100_e55299: f64 = (assign42100_e55297 * locals.var_sp_s_tau);
        let assign42100_e55303: f64 = (locals.var_nu / locals.var_mutau);
        let assign42100_e55305: f64 = (assign42100_e55303 * locals.var_sp_s_tau);
        let assign42100_e55307: f64 = (assign42100_e55305 * locals.var_sp_s_tau);
        let assign42100_e55309: f64 = (assign42100_e55307 * locals.var_sp_s_c);
        let assign42100_e55312: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42100_e55314: f64 = (assign42100_e55312 * 0.3333333333333333);
        let assign42100_e55317: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42100_e55318: f64 = (assign42100_e55314 - assign42100_e55317);
        let assign42100_e55319: f64 = (assign42100_e55309 * assign42100_e55318);
        let assign42100_e55320: f64 = (locals.var_mutau + assign42100_e55319);
        let assign42100_e55321: f64 = (assign42100_e55299 / assign42100_e55320);
        let assign42100_e55322: f64 = (locals.var_sp_s_eta + assign42100_e55321);
        (assign42100_e55322, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn5)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn5)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn6)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn6)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn7)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn7)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn8)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn8)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign42100_e55320 * assign42100_e55320))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8,)
    }
};
        locals.var_sp_s_x0 = assign42100_e55324;
        locals.var_sp_s_x0_dn5 = assign42100_e55324_d_n5;
        locals.var_sp_s_x0_dn6 = assign42100_e55324_d_n6;
        locals.var_sp_s_x0_dn7 = assign42100_e55324_d_n7;
        locals.var_sp_s_x0_dn8 = assign42100_e55324_d_n8;

        let assign42110_e55327: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1186 = assign42110_e55327;

        let (assign42120_e55338, assign42120_e55338_d_n5, assign42120_e55338_d_n6, assign42120_e55338_d_n7, assign42120_e55338_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign42120_e55336: f64 = (locals.var_sp_s_x0).exp();
        (assign42120_e55336, (assign42120_e55336 * locals.var_sp_s_x0_dn5), (assign42120_e55336 * locals.var_sp_s_x0_dn6), (assign42120_e55336 * locals.var_sp_s_x0_dn7), (assign42120_e55336 * locals.var_sp_s_x0_dn8),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign42120_e55338;
        locals.var_sp_s_delta0_dn5 = assign42120_e55338_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42120_e55338_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42120_e55338_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42120_e55338_d_n8;

        let (assign42130_e55350, assign42130_e55350_d_n5, assign42130_e55350_d_n6, assign42130_e55350_d_n7, assign42130_e55350_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign42130_e55348: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign42130_e55348, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign42130_e55350;
        locals.var_sp_s_delta1_dn5 = assign42130_e55350_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42130_e55350_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42130_e55350_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42130_e55350_d_n8;

        let (assign42140_e55362, assign42140_e55362_d_n5, assign42140_e55362_d_n6, assign42140_e55362_d_n7, assign42140_e55362_d_n8,) = {
    if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign42140_e55360: f64 = (locals.var_delta_ns * locals.var_sp_s_delta0);
        (assign42140_e55360, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign42140_e55362;
        locals.var_sp_s_delta0_dn5 = assign42140_e55362_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42140_e55362_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42140_e55362_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42140_e55362_d_n8;

        let assign42150_e55366: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42150_e55367: f64 = if locals.var_sp_s_x0 > assign42150_e55366 { 1.0 } else { 0.0 };
        locals.var_guard1187 = assign42150_e55367;

        let (assign42160_e55383, assign42160_e55383_d_n5, assign42160_e55383_d_n6, assign42160_e55383_d_n7, assign42160_e55383_d_n8,) = {
    if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 != 0.0)) {
        let assign42160_e55380: f64 = (locals.var_sp_s_x0 - locals.var_xn_s);
        let assign42160_e55381: f64 = (assign42160_e55380).exp();
        (assign42160_e55381, (assign42160_e55381 * (locals.var_sp_s_x0_dn5 - locals.var_xn_s_dn5)), (assign42160_e55381 * (locals.var_sp_s_x0_dn6 - locals.var_xn_s_dn6)), (assign42160_e55381 * (locals.var_sp_s_x0_dn7 - locals.var_xn_s_dn7)), (assign42160_e55381 * (locals.var_sp_s_x0_dn8 - locals.var_xn_s_dn8)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign42160_e55383;
        locals.var_sp_s_delta0_dn5 = assign42160_e55383_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42160_e55383_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42160_e55383_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42160_e55383_d_n8;

        let (assign42170_e55398, assign42170_e55398_d_n5, assign42170_e55398_d_n6, assign42170_e55398_d_n7, assign42170_e55398_d_n8,) = {
    if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 != 0.0)) {
        let assign42170_e55396: f64 = (locals.var_delta_ns / locals.var_sp_s_delta0);
        (assign42170_e55396, (((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign42170_e55398;
        locals.var_sp_s_delta1_dn5 = assign42170_e55398_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42170_e55398_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42170_e55398_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42170_e55398_d_n8;

        let (assign42180_e55440, assign42180_e55440_d_n5, assign42180_e55440_d_n6, assign42180_e55440_d_n7, assign42180_e55440_d_n8,) = {
    if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 == 0.0)) {
        let assign42180_e55414: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42180_e55416: f64 = (assign42180_e55414 - 230.25850929940458);
        let assign42180_e55421: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42180_e55423: f64 = (assign42180_e55421 - 230.25850929940458);
        let assign42180_e55427: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42180_e55429: f64 = (assign42180_e55427 - 230.25850929940458);
        let assign42180_e55431: f64 = (assign42180_e55429 * 0.3333333333333333);
        let assign42180_e55432: f64 = (1.0 + assign42180_e55431);
        let assign42180_e55433: f64 = (assign42180_e55423 * assign42180_e55432);
        let assign42180_e55434: f64 = (0.5 * assign42180_e55433);
        let assign42180_e55435: f64 = (1.0 + assign42180_e55434);
        let assign42180_e55436: f64 = (assign42180_e55416 * assign42180_e55435);
        let assign42180_e55437: f64 = (1.0 + assign42180_e55436);
        let assign42180_e55438: f64 = (1e-100 / assign42180_e55437);
        (assign42180_e55438, (-((1e-100 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign42180_e55440;
        locals.var_sp_s_delta0_dn5 = assign42180_e55440_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42180_e55440_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42180_e55440_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42180_e55440_d_n8;

        let (assign42190_e55476, assign42190_e55476_d_n5, assign42190_e55476_d_n6, assign42190_e55476_d_n7, assign42190_e55476_d_n8,) = {
    if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 == 0.0)) {
        let assign42190_e55456: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55461: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55465: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55467: f64 = (assign42190_e55465 * 0.3333333333333333);
        let assign42190_e55468: f64 = (1.0 + assign42190_e55467);
        let assign42190_e55469: f64 = (assign42190_e55461 * assign42190_e55468);
        let assign42190_e55470: f64 = (0.5 * assign42190_e55469);
        let assign42190_e55471: f64 = (1.0 + assign42190_e55470);
        let assign42190_e55472: f64 = (assign42190_e55456 * assign42190_e55471);
        let assign42190_e55473: f64 = (1.0 + assign42190_e55472);
        let assign42190_e55474: f64 = (1e-100 / assign42190_e55473);
        (assign42190_e55474, (-((1e-100 * ((locals.var_sp_s_x0_dn5 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn5 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign42190_e55476;
        locals.var_sp_s_delta1_dn5 = assign42190_e55476_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42190_e55476_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42190_e55476_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42190_e55476_d_n8;

    }

    pub(super) fn stamp_transient_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign42200_e55490, assign42200_e55490_d_n5, assign42200_e55490_d_n6, assign42200_e55490_d_n7, assign42200_e55490_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42200_e55486: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42200_e55487: f64 = (2.0 + assign42200_e55486);
        let assign42200_e55488: f64 = (1.0 / assign42200_e55487);
        (assign42200_e55488, (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign42200_e55487 * assign42200_e55487))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign42200_e55490;
        locals.var_sp_s_temp_dn5 = assign42200_e55490_d_n5;
        locals.var_sp_s_temp_dn6 = assign42200_e55490_d_n6;
        locals.var_sp_s_temp_dn7 = assign42200_e55490_d_n7;
        locals.var_sp_s_temp_dn8 = assign42200_e55490_d_n8;

        let (assign42210_e55502, assign42210_e55502_d_n5, assign42210_e55502_d_n6, assign42210_e55502_d_n7, assign42210_e55502_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42210_e55498: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42210_e55500: f64 = (assign42210_e55498 * locals.var_sp_s_temp);
        (assign42210_e55500, ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8,)
    }
};
        locals.var_sp_s_xi0 = assign42210_e55502;
        locals.var_sp_s_xi0_dn5 = assign42210_e55502_d_n5;
        locals.var_sp_s_xi0_dn6 = assign42210_e55502_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42210_e55502_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42210_e55502_d_n8;

        let (assign42220_e55516, assign42220_e55516_d_n5, assign42220_e55516_d_n6, assign42220_e55516_d_n7, assign42220_e55516_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42220_e55511: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign42220_e55513: f64 = (assign42220_e55511 * locals.var_sp_s_temp);
        let assign42220_e55514: f64 = (4.0 * assign42220_e55513);
        (assign42220_e55514, (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn8))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8,)
    }
};
        locals.var_sp_s_xi1 = assign42220_e55516;
        locals.var_sp_s_xi1_dn5 = assign42220_e55516_d_n5;
        locals.var_sp_s_xi1_dn6 = assign42220_e55516_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42220_e55516_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42220_e55516_d_n8;

        let (assign42230_e55534, assign42230_e55534_d_n5, assign42230_e55534_d_n6, assign42230_e55534_d_n7, assign42230_e55534_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42230_e55524: f64 = (8.0 * locals.var_sp_s_temp);
        let assign42230_e55527: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42230_e55528: f64 = (assign42230_e55524 - assign42230_e55527);
        let assign42230_e55530: f64 = (assign42230_e55528 * locals.var_sp_s_temp);
        let assign42230_e55532: f64 = (assign42230_e55530 * locals.var_sp_s_temp);
        (assign42230_e55532, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8,)
    }
};
        locals.var_sp_s_xi2 = assign42230_e55534;
        locals.var_sp_s_xi2_dn5 = assign42230_e55534_d_n5;
        locals.var_sp_s_xi2_dn6 = assign42230_e55534_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42230_e55534_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42230_e55534_d_n8;

        let (assign42240_e55544, assign42240_e55544_d_n5, assign42240_e55544_d_n6, assign42240_e55544_d_n7, assign42240_e55544_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42240_e55542: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign42240_e55542, (locals.var_xg_dn5 - locals.var_sp_s_x0_dn5), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign42240_e55544;
        locals.var_sp_s_temp_dn5 = assign42240_e55544_d_n5;
        locals.var_sp_s_temp_dn6 = assign42240_e55544_d_n6;
        locals.var_sp_s_temp_dn7 = assign42240_e55544_d_n7;
        locals.var_sp_s_temp_dn8 = assign42240_e55544_d_n8;

        let (assign42250_e55568, assign42250_e55568_d_n5, assign42250_e55568_d_n6, assign42250_e55568_d_n7, assign42250_e55568_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42250_e55552: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42250_e55556: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign42250_e55558: f64 = (assign42250_e55556 + locals.var_sp_s_delta0);
        let assign42250_e55562: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42250_e55563: f64 = (locals.var_delta_ns * assign42250_e55562);
        let assign42250_e55564: f64 = (assign42250_e55558 - assign42250_e55563);
        let assign42250_e55565: f64 = (locals.var_gf2 * assign42250_e55564);
        let assign42250_e55566: f64 = (assign42250_e55552 + assign42250_e55565);
        (assign42250_e55566, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8,)
    }
};
        locals.var_sp_s_pc = assign42250_e55568;
        locals.var_sp_s_pc_dn5 = assign42250_e55568_d_n5;
        locals.var_sp_s_pc_dn6 = assign42250_e55568_d_n6;
        locals.var_sp_s_pc_dn7 = assign42250_e55568_d_n7;
        locals.var_sp_s_pc_dn8 = assign42250_e55568_d_n8;

        let (assign42260_e55596, assign42260_e55596_d_n5, assign42260_e55596_d_n6, assign42260_e55596_d_n7, assign42260_e55596_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42260_e55576: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42260_e55580: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign42260_e55582: f64 = (assign42260_e55580 - 1.0);
        let assign42260_e55584: f64 = (assign42260_e55582 + locals.var_sp_s_delta0);
        let assign42260_e55588: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign42260_e55590: f64 = (assign42260_e55588 + locals.var_sp_s_xi0);
        let assign42260_e55591: f64 = (locals.var_delta_ns * assign42260_e55590);
        let assign42260_e55592: f64 = (assign42260_e55584 - assign42260_e55591);
        let assign42260_e55593: f64 = (locals.var_gf2 * assign42260_e55592);
        let assign42260_e55594: f64 = (assign42260_e55576 - assign42260_e55593);
        (assign42260_e55594, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8,)
    }
};
        locals.var_sp_s_qc = assign42260_e55596;
        locals.var_sp_s_qc_dn5 = assign42260_e55596_d_n5;
        locals.var_sp_s_qc_dn6 = assign42260_e55596_d_n6;
        locals.var_sp_s_qc_dn7 = assign42260_e55596_d_n7;
        locals.var_sp_s_qc_dn8 = assign42260_e55596_d_n8;

        let (assign42270_e55614, assign42270_e55614_d_n5, assign42270_e55614_d_n6, assign42270_e55614_d_n7, assign42270_e55614_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42270_e55606: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign42270_e55609: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42270_e55610: f64 = (assign42270_e55606 - assign42270_e55609);
        let assign42270_e55611: f64 = (locals.var_gf2 * assign42270_e55610);
        let assign42270_e55612: f64 = (2.0 - assign42270_e55611);
        (assign42270_e55612, (-((locals.var_gf2_dn5 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign42270_e55614;
        locals.var_sp_s_temp_dn5 = assign42270_e55614_d_n5;
        locals.var_sp_s_temp_dn6 = assign42270_e55614_d_n6;
        locals.var_sp_s_temp_dn7 = assign42270_e55614_d_n7;
        locals.var_sp_s_temp_dn8 = assign42270_e55614_d_n8;

        let (assign42280_e55630, assign42280_e55630_d_n5, assign42280_e55630_d_n6, assign42280_e55630_d_n7, assign42280_e55630_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42280_e55622: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42280_e55626: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42280_e55627: f64 = (2.0 * assign42280_e55626);
        let assign42280_e55628: f64 = (assign42280_e55622 - assign42280_e55627);
        (assign42280_e55628, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign42280_e55630;
        locals.var_sp_s_temp_dn5 = assign42280_e55630_d_n5;
        locals.var_sp_s_temp_dn6 = assign42280_e55630_d_n6;
        locals.var_sp_s_temp_dn7 = assign42280_e55630_d_n7;
        locals.var_sp_s_temp_dn8 = assign42280_e55630_d_n8;

        let (assign42290_e55647, assign42290_e55647_d_n5, assign42290_e55647_d_n6, assign42290_e55647_d_n7, assign42290_e55647_d_n8,) = {
    if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
        let assign42290_e55641: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42290_e55642: f64 = (locals.var_sp_s_pc + assign42290_e55641);
        let assign42290_e55643: f64 = (locals.var_sp_s_qc / assign42290_e55642);
        let assign42290_e55644: f64 = (2.0 * assign42290_e55643);
        let assign42290_e55645: f64 = (locals.var_sp_s_x0 + assign42290_e55644);
        (assign42290_e55645, (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8,)
    }
};
        locals.var_x_s = assign42290_e55647;
        locals.var_x_s_dn5 = assign42290_e55647_d_n5;
        locals.var_x_s_dn6 = assign42290_e55647_d_n6;
        locals.var_x_s_dn7 = assign42290_e55647_d_n7;
        locals.var_x_s_dn8 = assign42290_e55647_d_n8;

        locals.var_xi1s = 0.0;
        locals.var_xi1s_dn5 = 0.0;
        locals.var_xi1s_dn6 = 0.0;
        locals.var_xi1s_dn7 = 0.0;
        locals.var_xi1s_dn8 = 0.0;

        locals.var_xi2s = 0.0;
        locals.var_xi2s_dn5 = 0.0;
        locals.var_xi2s_dn6 = 0.0;
        locals.var_xi2s_dn7 = 0.0;
        locals.var_xi2s_dn8 = 0.0;

        locals.var_delta_1s = 0.0;
        locals.var_delta_1s_dn5 = 0.0;
        locals.var_delta_1s_dn6 = 0.0;
        locals.var_delta_1s_dn7 = 0.0;
        locals.var_delta_1s_dn8 = 0.0;

        locals.var_es = 0.0;
        locals.var_es_dn5 = 0.0;
        locals.var_es_dn6 = 0.0;
        locals.var_es_dn7 = 0.0;
        locals.var_es_dn8 = 0.0;

        locals.var_ds = 0.0;
        locals.var_ds_dn5 = 0.0;
        locals.var_ds_dn6 = 0.0;
        locals.var_ds_dn7 = 0.0;
        locals.var_ds_dn8 = 0.0;

        locals.var_ps = 0.0;
        locals.var_ps_dn5 = 0.0;
        locals.var_ps_dn6 = 0.0;
        locals.var_ps_dn7 = 0.0;
        locals.var_ps_dn8 = 0.0;

        locals.var_sqs = 0.0;
        locals.var_sqs_dn5 = 0.0;
        locals.var_sqs_dn6 = 0.0;
        locals.var_sqs_dn7 = 0.0;
        locals.var_sqs_dn8 = 0.0;

        locals.var_alphas = 1.0;
        locals.var_alphas_dn5 = 0.0;
        locals.var_alphas_dn6 = 0.0;
        locals.var_alphas_dn7 = 0.0;
        locals.var_alphas_dn8 = 0.0;

        locals.var_rxcor = 1.0;
        locals.var_rxcor_dn5 = 0.0;
        locals.var_rxcor_dn6 = 0.0;
        locals.var_rxcor_dn7 = 0.0;
        locals.var_rxcor_dn8 = 0.0;

        let assign42390_e55659: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgs = assign42390_e55659;
        locals.var_xgs_dn5 = (locals.var_xg_dn5 - locals.var_x_s_dn5);
        locals.var_xgs_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgs_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgs_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);

        locals.var_qis = 0.0;
        locals.var_qis_dn5 = 0.0;
        locals.var_qis_dn6 = 0.0;
        locals.var_qis_dn7 = 0.0;
        locals.var_qis_dn8 = 0.0;

        let assign42410_e55663: f64 = (locals.var_phit1 * locals.var_xgs);
        locals.var_qbs = assign42410_e55663;
        locals.var_qbs_dn5 = ((locals.var_phit1_dn5 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn5));
        locals.var_qbs_dn6 = ((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6));
        locals.var_qbs_dn7 = ((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7));
        locals.var_qbs_dn8 = ((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8));

        locals.var_rhob = 1.0;
        locals.var_rhob_dn5 = 0.0;
        locals.var_rhob_dn6 = 0.0;
        locals.var_rhob_dn7 = 0.0;
        locals.var_rhob_dn8 = 0.0;

        locals.var_rhog = 1.0;
        locals.var_rhog_dn5 = 0.0;
        locals.var_rhog_dn6 = 0.0;
        locals.var_rhog_dn7 = 0.0;
        locals.var_rhog_dn8 = 0.0;

        locals.var_gmobs = 1.0;
        locals.var_gmobs_dn5 = 0.0;
        locals.var_gmobs_dn6 = 0.0;
        locals.var_gmobs_dn7 = 0.0;
        locals.var_gmobs_dn8 = 0.0;

        locals.var_xitsb = 1.0;
        locals.var_xitsb_dn5 = 0.0;
        locals.var_xitsb_dn6 = 0.0;
        locals.var_xitsb_dn7 = 0.0;
        locals.var_xitsb_dn8 = 0.0;

        locals.var_factheta = 1.0;
        locals.var_factheta_dn5 = 0.0;
        locals.var_factheta_dn6 = 0.0;
        locals.var_factheta_dn7 = 0.0;
        locals.var_factheta_dn8 = 0.0;

        let assign42470_e55671: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1188 = assign42470_e55671;

        let (assign42480_e55681, assign42480_e55681_d_n5, assign42480_e55681_d_n6, assign42480_e55681_d_n7, assign42480_e55681_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42480_e55677: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42480_e55678: f64 = (2.0 + assign42480_e55677);
        let assign42480_e55679: f64 = (1.0 / assign42480_e55678);
        (assign42480_e55679, (-(((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) / (assign42480_e55678 * assign42480_e55678))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign42480_e55681;
        locals.var_temp__blk936_dn5 = assign42480_e55681_d_n5;
        locals.var_temp__blk936_dn6 = assign42480_e55681_d_n6;
        locals.var_temp__blk936_dn7 = assign42480_e55681_d_n7;
        locals.var_temp__blk936_dn8 = assign42480_e55681_d_n8;

        let (assign42490_e55689, assign42490_e55689_d_n5, assign42490_e55689_d_n6, assign42490_e55689_d_n7, assign42490_e55689_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42490_e55685: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42490_e55687: f64 = (assign42490_e55685 * locals.var_temp__blk936);
        (assign42490_e55687, ((((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn5)), ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn6)), ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn7)), ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi0s, locals.var_xi0s_dn5, locals.var_xi0s_dn6, locals.var_xi0s_dn7, locals.var_xi0s_dn8,)
    }
};
        locals.var_xi0s = assign42490_e55689;
        locals.var_xi0s_dn5 = assign42490_e55689_d_n5;
        locals.var_xi0s_dn6 = assign42490_e55689_d_n6;
        locals.var_xi0s_dn7 = assign42490_e55689_d_n7;
        locals.var_xi0s_dn8 = assign42490_e55689_d_n8;

        let (assign42500_e55699, assign42500_e55699_d_n5, assign42500_e55699_d_n6, assign42500_e55699_d_n7, assign42500_e55699_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42500_e55694: f64 = (locals.var_x_s * locals.var_temp__blk936);
        let assign42500_e55696: f64 = (assign42500_e55694 * locals.var_temp__blk936);
        let assign42500_e55697: f64 = (4.0 * assign42500_e55696);
        (assign42500_e55697, (4.0 * ((((locals.var_x_s_dn5 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn5))), (4.0 * ((((locals.var_x_s_dn6 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn6))), (4.0 * ((((locals.var_x_s_dn7 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn7))), (4.0 * ((((locals.var_x_s_dn8 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_xi1s, locals.var_xi1s_dn5, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8,)
    }
};
        locals.var_xi1s = assign42500_e55699;
        locals.var_xi1s_dn5 = assign42500_e55699_d_n5;
        locals.var_xi1s_dn6 = assign42500_e55699_d_n6;
        locals.var_xi1s_dn7 = assign42500_e55699_d_n7;
        locals.var_xi1s_dn8 = assign42500_e55699_d_n8;

        let (assign42510_e55713, assign42510_e55713_d_n5, assign42510_e55713_d_n6, assign42510_e55713_d_n7, assign42510_e55713_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42510_e55703: f64 = (8.0 * locals.var_temp__blk936);
        let assign42510_e55706: f64 = (12.0 * locals.var_xi0s);
        let assign42510_e55707: f64 = (assign42510_e55703 - assign42510_e55706);
        let assign42510_e55709: f64 = (assign42510_e55707 * locals.var_temp__blk936);
        let assign42510_e55711: f64 = (assign42510_e55709 * locals.var_temp__blk936);
        (assign42510_e55711, ((((((8.0 * locals.var_temp__blk936_dn5) - (12.0 * locals.var_xi0s_dn5)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn5)), ((((((8.0 * locals.var_temp__blk936_dn6) - (12.0 * locals.var_xi0s_dn6)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn6)), ((((((8.0 * locals.var_temp__blk936_dn7) - (12.0 * locals.var_xi0s_dn7)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn7)), ((((((8.0 * locals.var_temp__blk936_dn8) - (12.0 * locals.var_xi0s_dn8)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi2s, locals.var_xi2s_dn5, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8,)
    }
};
        locals.var_xi2s = assign42510_e55713;
        locals.var_xi2s_dn5 = assign42510_e55713_d_n5;
        locals.var_xi2s_dn6 = assign42510_e55713_d_n6;
        locals.var_xi2s_dn7 = assign42510_e55713_d_n7;
        locals.var_xi2s_dn8 = assign42510_e55713_d_n8;

        let (assign42520_e55717, assign42520_e55717_d_n5, assign42520_e55717_d_n6, assign42520_e55717_d_n7, assign42520_e55717_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8,)
    }
};
        locals.var_delta_1s = assign42520_e55717;
        locals.var_delta_1s_dn5 = assign42520_e55717_d_n5;
        locals.var_delta_1s_dn6 = assign42520_e55717_d_n6;
        locals.var_delta_1s_dn7 = assign42520_e55717_d_n7;
        locals.var_delta_1s_dn8 = assign42520_e55717_d_n8;

        let assign42530_e55720: f64 = if locals.var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign42530_e55720;

        let (assign42540_e55727, assign42540_e55727_d_n5, assign42540_e55727_d_n6, assign42540_e55727_d_n7, assign42540_e55727_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
        let assign42540_e55725: f64 = (locals.var_x_s).exp();
        (assign42540_e55725, (assign42540_e55725 * locals.var_x_s_dn5), (assign42540_e55725 * locals.var_x_s_dn6), (assign42540_e55725 * locals.var_x_s_dn7), (assign42540_e55725 * locals.var_x_s_dn8),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8,)
    }
};
        locals.var_delta_1s = assign42540_e55727;
        locals.var_delta_1s_dn5 = assign42540_e55727_d_n5;
        locals.var_delta_1s_dn6 = assign42540_e55727_d_n6;
        locals.var_delta_1s_dn7 = assign42540_e55727_d_n7;
        locals.var_delta_1s_dn8 = assign42540_e55727_d_n8;

        let (assign42550_e55735, assign42550_e55735_d_n5, assign42550_e55735_d_n6, assign42550_e55735_d_n7, assign42550_e55735_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
        let assign42550_e55733: f64 = (1.0 / locals.var_delta_1s);
        (assign42550_e55733, (-(locals.var_delta_1s_dn5 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn6 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn7 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn8 / (locals.var_delta_1s * locals.var_delta_1s))),)
    } else {
        (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8,)
    }
};
        locals.var_es = assign42550_e55735;
        locals.var_es_dn5 = assign42550_e55735_d_n5;
        locals.var_es_dn6 = assign42550_e55735_d_n6;
        locals.var_es_dn7 = assign42550_e55735_d_n7;
        locals.var_es_dn8 = assign42550_e55735_d_n8;

        let (assign42560_e55743, assign42560_e55743_d_n5, assign42560_e55743_d_n6, assign42560_e55743_d_n7, assign42560_e55743_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
        let assign42560_e55741: f64 = (locals.var_delta_ns * locals.var_delta_1s);
        (assign42560_e55741, ((locals.var_delta_ns_dn5 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn5)), ((locals.var_delta_ns_dn6 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn6)), ((locals.var_delta_ns_dn7 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn7)), ((locals.var_delta_ns_dn8 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn8)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8,)
    }
};
        locals.var_delta_1s = assign42560_e55743;
        locals.var_delta_1s_dn5 = assign42560_e55743_d_n5;
        locals.var_delta_1s_dn6 = assign42560_e55743_d_n6;
        locals.var_delta_1s_dn7 = assign42560_e55743_d_n7;
        locals.var_delta_1s_dn8 = assign42560_e55743_d_n8;

        let assign42570_e55747: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42570_e55748: f64 = if locals.var_x_s > assign42570_e55747 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign42570_e55748;

        let (assign42580_e55760, assign42580_e55760_d_n5, assign42580_e55760_d_n6, assign42580_e55760_d_n7, assign42580_e55760_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 != 0.0)) {
        let assign42580_e55757: f64 = (locals.var_x_s - locals.var_xn_s);
        let assign42580_e55758: f64 = (assign42580_e55757).exp();
        (assign42580_e55758, (assign42580_e55758 * (locals.var_x_s_dn5 - locals.var_xn_s_dn5)), (assign42580_e55758 * (locals.var_x_s_dn6 - locals.var_xn_s_dn6)), (assign42580_e55758 * (locals.var_x_s_dn7 - locals.var_xn_s_dn7)), (assign42580_e55758 * (locals.var_x_s_dn8 - locals.var_xn_s_dn8)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8,)
    }
};
        locals.var_delta_1s = assign42580_e55760;
        locals.var_delta_1s_dn5 = assign42580_e55760_d_n5;
        locals.var_delta_1s_dn6 = assign42580_e55760_d_n6;
        locals.var_delta_1s_dn7 = assign42580_e55760_d_n7;
        locals.var_delta_1s_dn8 = assign42580_e55760_d_n8;

        let (assign42590_e55771, assign42590_e55771_d_n5, assign42590_e55771_d_n6, assign42590_e55771_d_n7, assign42590_e55771_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 != 0.0)) {
        let assign42590_e55769: f64 = (locals.var_delta_ns / locals.var_delta_1s);
        (assign42590_e55769, (((locals.var_delta_ns_dn5 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn5)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn6 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn6)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn7 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn7)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn8 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn8)) / (locals.var_delta_1s * locals.var_delta_1s)),)
    } else {
        (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8,)
    }
};
        locals.var_es = assign42590_e55771;
        locals.var_es_dn5 = assign42590_e55771_d_n5;
        locals.var_es_dn6 = assign42590_e55771_d_n6;
        locals.var_es_dn7 = assign42590_e55771_d_n7;
        locals.var_es_dn8 = assign42590_e55771_d_n8;

        let (assign42600_e55809, assign42600_e55809_d_n5, assign42600_e55809_d_n6, assign42600_e55809_d_n7, assign42600_e55809_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 == 0.0)) {
        let assign42600_e55783: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42600_e55785: f64 = (assign42600_e55783 - 230.25850929940458);
        let assign42600_e55790: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42600_e55792: f64 = (assign42600_e55790 - 230.25850929940458);
        let assign42600_e55796: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42600_e55798: f64 = (assign42600_e55796 - 230.25850929940458);
        let assign42600_e55800: f64 = (assign42600_e55798 * 0.3333333333333333);
        let assign42600_e55801: f64 = (1.0 + assign42600_e55800);
        let assign42600_e55802: f64 = (assign42600_e55792 * assign42600_e55801);
        let assign42600_e55803: f64 = (0.5 * assign42600_e55802);
        let assign42600_e55804: f64 = (1.0 + assign42600_e55803);
        let assign42600_e55805: f64 = (assign42600_e55785 * assign42600_e55804);
        let assign42600_e55806: f64 = (1.0 + assign42600_e55805);
        let assign42600_e55807: f64 = (1e-100 / assign42600_e55806);
        (assign42600_e55807, (-((1e-100 * (((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8,)
    }
};
        locals.var_delta_1s = assign42600_e55809;
        locals.var_delta_1s_dn5 = assign42600_e55809_d_n5;
        locals.var_delta_1s_dn6 = assign42600_e55809_d_n6;
        locals.var_delta_1s_dn7 = assign42600_e55809_d_n7;
        locals.var_delta_1s_dn8 = assign42600_e55809_d_n8;

        let (assign42610_e55841, assign42610_e55841_d_n5, assign42610_e55841_d_n6, assign42610_e55841_d_n7, assign42610_e55841_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 == 0.0)) {
        let assign42610_e55821: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42610_e55826: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42610_e55830: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42610_e55832: f64 = (assign42610_e55830 * 0.3333333333333333);
        let assign42610_e55833: f64 = (1.0 + assign42610_e55832);
        let assign42610_e55834: f64 = (assign42610_e55826 * assign42610_e55833);
        let assign42610_e55835: f64 = (0.5 * assign42610_e55834);
        let assign42610_e55836: f64 = (1.0 + assign42610_e55835);
        let assign42610_e55837: f64 = (assign42610_e55821 * assign42610_e55836);
        let assign42610_e55838: f64 = (1.0 + assign42610_e55837);
        let assign42610_e55839: f64 = (1e-100 / assign42610_e55838);
        (assign42610_e55839, (-((1e-100 * ((locals.var_x_s_dn5 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn5 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn5 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn6 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn6 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn6 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn7 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn7 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn7 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn8 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn8 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn8 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))),)
    } else {
        (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8,)
    }
};
        locals.var_es = assign42610_e55841;
        locals.var_es_dn5 = assign42610_e55841_d_n5;
        locals.var_es_dn6 = assign42610_e55841_d_n6;
        locals.var_es_dn7 = assign42610_e55841_d_n7;
        locals.var_es_dn8 = assign42610_e55841_d_n8;

        let (assign42620_e55853, assign42620_e55853_d_n5, assign42620_e55853_d_n6, assign42620_e55853_d_n7, assign42620_e55853_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42620_e55847: f64 = (locals.var_x_s + 1.0);
        let assign42620_e55849: f64 = (assign42620_e55847 + locals.var_xi0s);
        let assign42620_e55850: f64 = (locals.var_delta_ns * assign42620_e55849);
        let assign42620_e55851: f64 = (locals.var_delta_1s - assign42620_e55850);
        (assign42620_e55851, (locals.var_delta_1s_dn5 - ((locals.var_delta_ns_dn5 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn5 + locals.var_xi0s_dn5)))), (locals.var_delta_1s_dn6 - ((locals.var_delta_ns_dn6 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn6 + locals.var_xi0s_dn6)))), (locals.var_delta_1s_dn7 - ((locals.var_delta_ns_dn7 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn7 + locals.var_xi0s_dn7)))), (locals.var_delta_1s_dn8 - ((locals.var_delta_ns_dn8 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn8 + locals.var_xi0s_dn8)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8,)
    }
};
        locals.var_ds = assign42620_e55853;
        locals.var_ds_dn5 = assign42620_e55853_d_n5;
        locals.var_ds_dn6 = assign42620_e55853_d_n6;
        locals.var_ds_dn7 = assign42620_e55853_d_n7;
        locals.var_ds_dn8 = assign42620_e55853_d_n8;

        let assign42630_e55856: f64 = if locals.var_x_s < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign42630_e55856;

    }

    pub(super) fn stamp_transient_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign42640_e55878, assign42640_e55878_d_n5, assign42640_e55878_d_n6, assign42640_e55878_d_n7, assign42640_e55878_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign42640_e55863: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42640_e55870: f64 = (0.25 * locals.var_x_s);
        let assign42640_e55871: f64 = (1.0 - assign42640_e55870);
        let assign42640_e55872: f64 = (locals.var_x_s * assign42640_e55871);
        let assign42640_e55873: f64 = (0.3333333333333333 * assign42640_e55872);
        let assign42640_e55874: f64 = (1.0 - assign42640_e55873);
        let assign42640_e55875: f64 = (assign42640_e55863 * assign42640_e55874);
        let assign42640_e55876: f64 = (0.5 * assign42640_e55875);
        (assign42640_e55876, (0.5 * ((((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn5 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn5))))))))), (0.5 * ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6))))))))), (0.5 * ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7))))))))), (0.5 * ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8))))))))),)
    } else {
        (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8,)
    }
};
        locals.var_ps = assign42640_e55878;
        locals.var_ps_dn5 = assign42640_e55878_d_n5;
        locals.var_ps_dn6 = assign42640_e55878_d_n6;
        locals.var_ps_dn7 = assign42640_e55878_d_n7;
        locals.var_ps_dn8 = assign42640_e55878_d_n8;

        let (assign42650_e55898, assign42650_e55898_d_n5, assign42650_e55898_d_n6, assign42650_e55898_d_n7, assign42650_e55898_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign42650_e55885: f64 = (locals.var_delta_ns * locals.var_x_s);
        let assign42650_e55887: f64 = (assign42650_e55885 * locals.var_x_s);
        let assign42650_e55889: f64 = (assign42650_e55887 * locals.var_x_s);
        let assign42650_e55893: f64 = (1.75 * locals.var_x_s);
        let assign42650_e55894: f64 = (1.0 + assign42650_e55893);
        let assign42650_e55895: f64 = (assign42650_e55889 * assign42650_e55894);
        let assign42650_e55896: f64 = (0.16666666666666666 * assign42650_e55895);
        (assign42650_e55896, (0.16666666666666666 * ((((((((locals.var_delta_ns_dn5 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn5)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn5)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn5)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn5)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn6 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn6)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn7 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn7)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn8 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn8)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn8)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8,)
    }
};
        locals.var_ds = assign42650_e55898;
        locals.var_ds_dn5 = assign42650_e55898_d_n5;
        locals.var_ds_dn6 = assign42650_e55898_d_n6;
        locals.var_ds_dn7 = assign42650_e55898_d_n7;
        locals.var_ds_dn8 = assign42650_e55898_d_n8;

        let (assign42660_e55915, assign42660_e55915_d_n5, assign42660_e55915_d_n6, assign42660_e55915_d_n7, assign42660_e55915_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign42660_e55908: f64 = (0.25 * locals.var_x_s);
        let assign42660_e55909: f64 = (1.0 - assign42660_e55908);
        let assign42660_e55910: f64 = (locals.var_x_s * assign42660_e55909);
        let assign42660_e55911: f64 = (0.3333333333333333 * assign42660_e55910);
        let assign42660_e55912: f64 = (1.0 - assign42660_e55911);
        let assign42660_e55913: f64 = (assign42660_e55912).sqrt();
        (assign42660_e55913, ((-(0.3333333333333333 * ((locals.var_x_s_dn5 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn5)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8)))))) / (2.0 * assign42660_e55913)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign42660_e55915;
        locals.var_temp__blk936_dn5 = assign42660_e55915_d_n5;
        locals.var_temp__blk936_dn6 = assign42660_e55915_d_n6;
        locals.var_temp__blk936_dn7 = assign42660_e55915_d_n7;
        locals.var_temp__blk936_dn8 = assign42660_e55915_d_n8;

        let (assign42670_e55925, assign42670_e55925_d_n5, assign42670_e55925_d_n6, assign42670_e55925_d_n7, assign42670_e55925_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign42670_e55922: f64 = (locals.var_x_s * locals.var_temp__blk936);
        let assign42670_e55923: f64 = (0.7071067811865475 * assign42670_e55922);
        (assign42670_e55923, (0.7071067811865475 * ((locals.var_x_s_dn5 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_s_dn6 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_s_dn7 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_s_dn8 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8,)
    }
};
        locals.var_sqs = assign42670_e55925;
        locals.var_sqs_dn5 = assign42670_e55925_d_n5;
        locals.var_sqs_dn6 = assign42670_e55925_d_n6;
        locals.var_sqs_dn7 = assign42670_e55925_d_n7;
        locals.var_sqs_dn8 = assign42670_e55925_d_n8;

        let (assign42680_e55949, assign42680_e55949_d_n5, assign42680_e55949_d_n6, assign42680_e55949_d_n7, assign42680_e55949_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign42680_e55935: f64 = (0.5 * locals.var_x_s);
        let assign42680_e55936: f64 = (1.0 - assign42680_e55935);
        let assign42680_e55940: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42680_e55941: f64 = (0.16666666666666666 * assign42680_e55940);
        let assign42680_e55942: f64 = (assign42680_e55936 + assign42680_e55941);
        let assign42680_e55943: f64 = (locals.var_gf * assign42680_e55942);
        let assign42680_e55945: f64 = (assign42680_e55943 / locals.var_temp__blk936);
        let assign42680_e55946: f64 = (0.7071067811865475 * assign42680_e55945);
        let assign42680_e55947: f64 = (1.0 + assign42680_e55946);
        (assign42680_e55947, (0.7071067811865475 * (((((locals.var_gf_dn5 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn5)) + (0.16666666666666666 * ((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn6 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn6)) + (0.16666666666666666 * ((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn7 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn7)) + (0.16666666666666666 * ((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn8 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn8)) + (0.16666666666666666 * ((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8,)
    }
};
        locals.var_alphas = assign42680_e55949;
        locals.var_alphas_dn5 = assign42680_e55949_d_n5;
        locals.var_alphas_dn6 = assign42680_e55949_d_n6;
        locals.var_alphas_dn7 = assign42680_e55949_d_n7;
        locals.var_alphas_dn8 = assign42680_e55949_d_n8;

        let (assign42690_e55960, assign42690_e55960_d_n5, assign42690_e55960_d_n6, assign42690_e55960_d_n7, assign42690_e55960_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
        let assign42690_e55956: f64 = (locals.var_x_s - 1.0);
        let assign42690_e55958: f64 = (assign42690_e55956 + locals.var_es);
        (assign42690_e55958, (locals.var_x_s_dn5 + locals.var_es_dn5), (locals.var_x_s_dn6 + locals.var_es_dn6), (locals.var_x_s_dn7 + locals.var_es_dn7), (locals.var_x_s_dn8 + locals.var_es_dn8),)
    } else {
        (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8,)
    }
};
        locals.var_ps = assign42690_e55960;
        locals.var_ps_dn5 = assign42690_e55960_d_n5;
        locals.var_ps_dn6 = assign42690_e55960_d_n6;
        locals.var_ps_dn7 = assign42690_e55960_d_n7;
        locals.var_ps_dn8 = assign42690_e55960_d_n8;

        let (assign42700_e55968, assign42700_e55968_d_n5, assign42700_e55968_d_n6, assign42700_e55968_d_n7, assign42700_e55968_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
        let assign42700_e55966: f64 = (locals.var_ps).sqrt();
        (assign42700_e55966, (locals.var_ps_dn5 / (2.0 * assign42700_e55966)), (locals.var_ps_dn6 / (2.0 * assign42700_e55966)), (locals.var_ps_dn7 / (2.0 * assign42700_e55966)), (locals.var_ps_dn8 / (2.0 * assign42700_e55966)),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8,)
    }
};
        locals.var_sqs = assign42700_e55968;
        locals.var_sqs_dn5 = assign42700_e55968_d_n5;
        locals.var_sqs_dn6 = assign42700_e55968_d_n6;
        locals.var_sqs_dn7 = assign42700_e55968_d_n7;
        locals.var_sqs_dn8 = assign42700_e55968_d_n8;

        let (assign42710_e55985, assign42710_e55985_d_n5, assign42710_e55985_d_n6, assign42710_e55985_d_n7, assign42710_e55985_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
        let assign42710_e55978: f64 = (1.0 - locals.var_es);
        let assign42710_e55979: f64 = (locals.var_gf * assign42710_e55978);
        let assign42710_e55981: f64 = (assign42710_e55979 / locals.var_sqs);
        let assign42710_e55982: f64 = (0.5 * assign42710_e55981);
        let assign42710_e55983: f64 = (1.0 + assign42710_e55982);
        (assign42710_e55983, (0.5 * (((((locals.var_gf_dn5 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn5))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn5)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn6 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn6))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn6)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn7 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn7))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn7)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn8 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn8))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn8)) / (locals.var_sqs * locals.var_sqs))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8,)
    }
};
        locals.var_alphas = assign42710_e55985;
        locals.var_alphas_dn5 = assign42710_e55985_d_n5;
        locals.var_alphas_dn6 = assign42710_e55985_d_n6;
        locals.var_alphas_dn7 = assign42710_e55985_d_n7;
        locals.var_alphas_dn8 = assign42710_e55985_d_n8;

        let (assign42720_e56001, assign42720_e56001_d_n5, assign42720_e56001_d_n6, assign42720_e56001_d_n7, assign42720_e56001_d_n8,) = {
    if (locals.var_guard1188 != 0.0) {
        let assign42720_e55990: f64 = (0.2 * locals.var_xcor_t);
        let assign42720_e55992: f64 = (assign42720_e55990 * locals.var_vsbx);
        let assign42720_e55993: f64 = (1.0 + assign42720_e55992);
        let assign42720_e55997: f64 = (locals.var_xcor_t * locals.var_vsbx);
        let assign42720_e55998: f64 = (1.0 + assign42720_e55997);
        let assign42720_e55999: f64 = (assign42720_e55993 / assign42720_e55998);
        (assign42720_e55999, ((((assign42720_e55990 * locals.var_vsbx_dn5) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn5))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn6) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn6))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn7) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn7))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn8) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn8))) / (assign42720_e55998 * assign42720_e55998)),)
    } else {
        (locals.var_rxcor, locals.var_rxcor_dn5, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8,)
    }
};
        locals.var_rxcor = assign42720_e56001;
        locals.var_rxcor_dn5 = assign42720_e56001_d_n5;
        locals.var_rxcor_dn6 = assign42720_e56001_d_n6;
        locals.var_rxcor_dn7 = assign42720_e56001_d_n7;
        locals.var_rxcor_dn8 = assign42720_e56001_d_n8;

        let assign42730_e56004: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign42730_e56004;

        let (assign42740_e56015, assign42740_e56015_d_n5, assign42740_e56015_d_n6, assign42740_e56015_d_n7, assign42740_e56015_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42740_e56011: f64 = (locals.var_ps + locals.var_ds);
        let assign42740_e56012: f64 = (assign42740_e56011).sqrt();
        let assign42740_e56013: f64 = (locals.var_gf * assign42740_e56012);
        (assign42740_e56013, ((locals.var_gf_dn5 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn5 + locals.var_ds_dn5) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn6 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn6 + locals.var_ds_dn6) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn7 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn7 + locals.var_ds_dn7) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn8 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn8 + locals.var_ds_dn8) / (2.0 * assign42740_e56012)))),)
    } else {
        (locals.var_xgs, locals.var_xgs_dn5, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8,)
    }
};
        locals.var_xgs = assign42740_e56015;
        locals.var_xgs_dn5 = assign42740_e56015_d_n5;
        locals.var_xgs_dn6 = assign42740_e56015_d_n6;
        locals.var_xgs_dn7 = assign42740_e56015_d_n7;
        locals.var_xgs_dn8 = assign42740_e56015_d_n8;

        let (assign42750_e56031, assign42750_e56031_d_n5, assign42750_e56031_d_n6, assign42750_e56031_d_n7, assign42750_e56031_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42750_e56021: f64 = (locals.var_gf2 * locals.var_ds);
        let assign42750_e56023: f64 = (assign42750_e56021 * locals.var_phit1);
        let assign42750_e56027: f64 = (locals.var_gf * locals.var_sqs);
        let assign42750_e56028: f64 = (locals.var_xgs + assign42750_e56027);
        let assign42750_e56029: f64 = (assign42750_e56023 / assign42750_e56028);
        (assign42750_e56029, (((((((locals.var_gf2_dn5 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn5)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn5)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn5 + ((locals.var_gf_dn5 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn5))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn6 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn6)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn6)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn6 + ((locals.var_gf_dn6 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn6))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn7 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn7)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn7)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn7 + ((locals.var_gf_dn7 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn7))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn8 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn8)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn8)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn8 + ((locals.var_gf_dn8 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn8))))) / (assign42750_e56028 * assign42750_e56028)),)
    } else {
        (locals.var_qis, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8,)
    }
};
        locals.var_qis = assign42750_e56031;
        locals.var_qis_dn5 = assign42750_e56031_d_n5;
        locals.var_qis_dn6 = assign42750_e56031_d_n6;
        locals.var_qis_dn7 = assign42750_e56031_d_n7;
        locals.var_qis_dn8 = assign42750_e56031_d_n8;

        let (assign42760_e56041, assign42760_e56041_d_n5, assign42760_e56041_d_n6, assign42760_e56041_d_n7, assign42760_e56041_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42760_e56037: f64 = (locals.var_sqs * locals.var_gf);
        let assign42760_e56039: f64 = (assign42760_e56037 * locals.var_phit1);
        (assign42760_e56039, ((((locals.var_sqs_dn5 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn5)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn5)), ((((locals.var_sqs_dn6 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn6)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn6)), ((((locals.var_sqs_dn7 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn7)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn7)), ((((locals.var_sqs_dn8 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn8)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn8)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8,)
    }
};
        locals.var_qbs = assign42760_e56041;
        locals.var_qbs_dn5 = assign42760_e56041_d_n5;
        locals.var_qbs_dn6 = assign42760_e56041_d_n6;
        locals.var_qbs_dn7 = assign42760_e56041_d_n7;
        locals.var_qbs_dn8 = assign42760_e56041_d_n8;

        let assign42770_e56044: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign42770_e56044;

        let (assign42780_e56058, assign42780_e56058_d_n5, assign42780_e56058_d_n6, assign42780_e56058_d_n7, assign42780_e56058_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign42780_e56054: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42780_e56055: f64 = (1.0 - assign42780_e56054);
        let assign42780_e56056: f64 = (1.0 / assign42780_e56055);
        (assign42780_e56056, (-((-(locals.var_rsb_i * locals.var_vsbx_dn5)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn6)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn7)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn8)) / (assign42780_e56055 * assign42780_e56055))),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8,)
    }
};
        locals.var_rhob = assign42780_e56058;
        locals.var_rhob_dn5 = assign42780_e56058_d_n5;
        locals.var_rhob_dn6 = assign42780_e56058_d_n6;
        locals.var_rhob_dn7 = assign42780_e56058_d_n7;
        locals.var_rhob_dn8 = assign42780_e56058_d_n8;

        let (assign42790_e56071, assign42790_e56071_d_n5, assign42790_e56071_d_n6, assign42790_e56071_d_n7, assign42790_e56071_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1193 == 0.0)) {
        let assign42790_e56068: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42790_e56069: f64 = (1.0 + assign42790_e56068);
        (assign42790_e56069, (locals.var_rsb_i * locals.var_vsbx_dn5), (locals.var_rsb_i * locals.var_vsbx_dn6), (locals.var_rsb_i * locals.var_vsbx_dn7), (locals.var_rsb_i * locals.var_vsbx_dn8),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8,)
    }
};
        locals.var_rhob = assign42790_e56071;
        locals.var_rhob_dn5 = assign42790_e56071_d_n5;
        locals.var_rhob_dn6 = assign42790_e56071_d_n6;
        locals.var_rhob_dn7 = assign42790_e56071_d_n7;
        locals.var_rhob_dn8 = assign42790_e56071_d_n8;

        let assign42800_e56074: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign42800_e56074;

        let (assign42810_e56086, assign42810_e56086_d_n5, assign42810_e56086_d_n6, assign42810_e56086_d_n7, assign42810_e56086_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1194 != 0.0)) {
        let assign42810_e56083: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign42810_e56084: f64 = (1.0 - assign42810_e56083);
        (assign42810_e56084, (-(locals.var_rsg_i * locals.var_qis_dn5)), (-(locals.var_rsg_i * locals.var_qis_dn6)), (-(locals.var_rsg_i * locals.var_qis_dn7)), (-(locals.var_rsg_i * locals.var_qis_dn8)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign42810_e56086;
        locals.var_rhog_dn5 = assign42810_e56086_d_n5;
        locals.var_rhog_dn6 = assign42810_e56086_d_n6;
        locals.var_rhog_dn7 = assign42810_e56086_d_n7;
        locals.var_rhog_dn8 = assign42810_e56086_d_n8;

        let (assign42820_e56101, assign42820_e56101_d_n5, assign42820_e56101_d_n6, assign42820_e56101_d_n7, assign42820_e56101_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1194 == 0.0)) {
        let assign42820_e56097: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign42820_e56098: f64 = (1.0 + assign42820_e56097);
        let assign42820_e56099: f64 = (1.0 / assign42820_e56098);
        (assign42820_e56099, (-((locals.var_rsg_i * locals.var_qis_dn5) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn6) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn7) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn8) / (assign42820_e56098 * assign42820_e56098))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign42820_e56101;
        locals.var_rhog_dn5 = assign42820_e56101_d_n5;
        locals.var_rhog_dn6 = assign42820_e56101_d_n6;
        locals.var_rhog_dn7 = assign42820_e56101_d_n7;
        locals.var_rhog_dn8 = assign42820_e56101_d_n8;

        let (assign42830_e56113, assign42830_e56113_d_n5, assign42830_e56113_d_n6, assign42830_e56113_d_n7, assign42830_e56113_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42830_e56107: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign42830_e56109: f64 = (assign42830_e56107 * locals.var_rhog);
        let assign42830_e56111: f64 = (assign42830_e56109 * locals.var_qis);
        (assign42830_e56111, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn5)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn6)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn7)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn8)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn8)),)
    } else {
        (locals.var_gr, locals.var_gr_dn5, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8,)
    }
};
        locals.var_gr = assign42830_e56113;
        locals.var_gr_dn5 = assign42830_e56113_d_n5;
        locals.var_gr_dn6 = assign42830_e56113_d_n6;
        locals.var_gr_dn7 = assign42830_e56113_d_n7;
        locals.var_gr_dn8 = assign42830_e56113_d_n8;

        let (assign42840_e56125, assign42840_e56125_d_n5, assign42840_e56125_d_n6, assign42840_e56125_d_n7, assign42840_e56125_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42840_e56121: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign42840_e56122: f64 = (locals.var_qbs + assign42840_e56121);
        let assign42840_e56123: f64 = (locals.var_e_eff0 * assign42840_e56122);
        (assign42840_e56123, (locals.var_e_eff0 * (locals.var_qbs_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_e_eff0 * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_e_eff0 * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_e_eff0 * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn5, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8,)
    }
};
        locals.var_eeffs = assign42840_e56125;
        locals.var_eeffs_dn5 = assign42840_e56125_d_n5;
        locals.var_eeffs_dn6 = assign42840_e56125_d_n6;
        locals.var_eeffs_dn7 = assign42840_e56125_d_n7;
        locals.var_eeffs_dn8 = assign42840_e56125_d_n8;

        let (assign42850_e56138, assign42850_e56138_d_n5, assign42850_e56138_d_n6, assign42850_e56138_d_n7, assign42850_e56138_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42850_e56132: f64 = (locals.var_ps + locals.var_ds);
        let assign42850_e56134: f64 = (assign42850_e56132 + 1e-14);
        let assign42850_e56135: f64 = (locals.var_ps / assign42850_e56134);
        let assign42850_e56136: f64 = (assign42850_e56135).ln();
        (assign42850_e56136, ((((locals.var_ps_dn5 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn5 + locals.var_ds_dn5))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn6 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn6 + locals.var_ds_dn6))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn7 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn7 + locals.var_ds_dn7))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn8 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn8 + locals.var_ds_dn8))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign42850_e56138;
        locals.var_temp1_dn5 = assign42850_e56138_d_n5;
        locals.var_temp1_dn6 = assign42850_e56138_d_n6;
        locals.var_temp1_dn7 = assign42850_e56138_d_n7;
        locals.var_temp1_dn8 = assign42850_e56138_d_n8;

        let (assign42860_e56157, assign42860_e56157_d_n5, assign42860_e56157_d_n6, assign42860_e56157_d_n7, assign42860_e56157_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42860_e56144: f64 = (locals.var_eeffs * locals.var_mue_t);
        let assign42860_e56146: f64 = (assign42860_e56144).powf(locals.var_themu_t);
        let assign42860_e56150: f64 = (0.5 * locals.var_thecs_t);
        let assign42860_e56152: f64 = (assign42860_e56150 * locals.var_temp1);
        let assign42860_e56153: f64 = (assign42860_e56152).exp();
        let assign42860_e56154: f64 = (locals.var_cs_t * assign42860_e56153);
        let assign42860_e56155: f64 = (assign42860_e56146 + assign42860_e56154);
        (assign42860_e56155, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn5 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn5 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn6 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn6 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn7 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn7 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn8 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn8 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn5, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8,)
    }
};
        locals.var_mutmp = assign42860_e56157;
        locals.var_mutmp_dn5 = assign42860_e56157_d_n5;
        locals.var_mutmp_dn6 = assign42860_e56157_d_n6;
        locals.var_mutmp_dn7 = assign42860_e56157_d_n7;
        locals.var_mutmp_dn8 = assign42860_e56157_d_n8;

        let (assign42870_e56169, assign42870_e56169_d_n5, assign42870_e56169_d_n6, assign42870_e56169_d_n7, assign42870_e56169_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42870_e56163: f64 = (1.0 + locals.var_mutmp);
        let assign42870_e56165: f64 = (assign42870_e56163 + locals.var_gr);
        let assign42870_e56167: f64 = (assign42870_e56165 * locals.var_rxcor);
        (assign42870_e56167, (((locals.var_mutmp_dn5 + locals.var_gr_dn5) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn5)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn8)),)
    } else {
        (locals.var_gmobs, locals.var_gmobs_dn5, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8,)
    }
};
        locals.var_gmobs = assign42870_e56169;
        locals.var_gmobs_dn5 = assign42870_e56169_d_n5;
        locals.var_gmobs_dn6 = assign42870_e56169_d_n6;
        locals.var_gmobs_dn7 = assign42870_e56169_d_n7;
        locals.var_gmobs_dn8 = assign42870_e56169_d_n8;

        let assign42880_e56172: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign42880_e56172;

        let (assign42890_e56186, assign42890_e56186_d_n5, assign42890_e56186_d_n6, assign42890_e56186_d_n7, assign42890_e56186_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1195 != 0.0)) {
        let assign42890_e56182: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign42890_e56183: f64 = (1.0 - assign42890_e56182);
        let assign42890_e56184: f64 = (1.0 / assign42890_e56183);
        (assign42890_e56184, (-((-(locals.var_thesatb_i * locals.var_vsbx_dn5)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn6)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn7)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn8)) / (assign42890_e56183 * assign42890_e56183))),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8,)
    }
};
        locals.var_xitsb = assign42890_e56186;
        locals.var_xitsb_dn5 = assign42890_e56186_d_n5;
        locals.var_xitsb_dn6 = assign42890_e56186_d_n6;
        locals.var_xitsb_dn7 = assign42890_e56186_d_n7;
        locals.var_xitsb_dn8 = assign42890_e56186_d_n8;

        let (assign42900_e56199, assign42900_e56199_d_n5, assign42900_e56199_d_n6, assign42900_e56199_d_n7, assign42900_e56199_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1195 == 0.0)) {
        let assign42900_e56196: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign42900_e56197: f64 = (1.0 + assign42900_e56196);
        (assign42900_e56197, (locals.var_thesatb_i * locals.var_vsbx_dn5), (locals.var_thesatb_i * locals.var_vsbx_dn6), (locals.var_thesatb_i * locals.var_vsbx_dn7), (locals.var_thesatb_i * locals.var_vsbx_dn8),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8,)
    }
};
        locals.var_xitsb = assign42900_e56199;
        locals.var_xitsb_dn5 = assign42900_e56199_d_n5;
        locals.var_xitsb_dn6 = assign42900_e56199_d_n6;
        locals.var_xitsb_dn7 = assign42900_e56199_d_n7;
        locals.var_xitsb_dn8 = assign42900_e56199_d_n8;

        let (assign42910_e56207, assign42910_e56207_d_n5, assign42910_e56207_d_n6, assign42910_e56207_d_n7, assign42910_e56207_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42910_e56205: f64 = (locals.var_qis * locals.var_xitsb);
        (assign42910_e56205, ((locals.var_qis_dn5 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn5)), ((locals.var_qis_dn6 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn6)), ((locals.var_qis_dn7 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn7)), ((locals.var_qis_dn8 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign42910_e56207;
        locals.var_temp2_dn5 = assign42910_e56207_d_n5;
        locals.var_temp2_dn6 = assign42910_e56207_d_n6;
        locals.var_temp2_dn7 = assign42910_e56207_d_n7;
        locals.var_temp2_dn8 = assign42910_e56207_d_n8;

        let (assign42920_e56217, assign42920_e56217_d_n5, assign42920_e56217_d_n6, assign42920_e56217_d_n7, assign42920_e56217_d_n8,) = {
    if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign42920_e56214: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign42920_e56215: f64 = (locals.var_temp2 / assign42920_e56214);
        (assign42920_e56215, (((locals.var_temp2_dn5 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn6 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn7 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn8 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign42920_e56214 * assign42920_e56214)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8,)
    }
};
        locals.var_wsat = assign42920_e56217;
        locals.var_wsat_dn5 = assign42920_e56217_d_n5;
        locals.var_wsat_dn6 = assign42920_e56217_d_n6;
        locals.var_wsat_dn7 = assign42920_e56217_d_n7;
        locals.var_wsat_dn8 = assign42920_e56217_d_n8;

        let assign42930_e56220: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign42930_e56220;

        let (assign42940_e56234, assign42940_e56234_d_n5, assign42940_e56234_d_n6, assign42940_e56234_d_n7, assign42940_e56234_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1196 != 0.0)) {
        let assign42940_e56230: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign42940_e56231: f64 = (1.0 - assign42940_e56230);
        let assign42940_e56232: f64 = (1.0 / assign42940_e56231);
        (assign42940_e56232, (-((-(locals.var_thesatg_i * locals.var_wsat_dn5)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign42940_e56231 * assign42940_e56231))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign42940_e56234;
        locals.var_factheta_dn5 = assign42940_e56234_d_n5;
        locals.var_factheta_dn6 = assign42940_e56234_d_n6;
        locals.var_factheta_dn7 = assign42940_e56234_d_n7;
        locals.var_factheta_dn8 = assign42940_e56234_d_n8;

        let (assign42950_e56247, assign42950_e56247_d_n5, assign42950_e56247_d_n6, assign42950_e56247_d_n7, assign42950_e56247_d_n8,) = {
    if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1196 == 0.0)) {
        let assign42950_e56244: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign42950_e56245: f64 = (1.0 + assign42950_e56244);
        (assign42950_e56245, (locals.var_thesatg_i * locals.var_wsat_dn5), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign42950_e56247;
        locals.var_factheta_dn5 = assign42950_e56247_d_n5;
        locals.var_factheta_dn6 = assign42950_e56247_d_n6;
        locals.var_factheta_dn7 = assign42950_e56247_d_n7;
        locals.var_factheta_dn8 = assign42950_e56247_d_n8;

        locals.var_vgb1_dc = locals.var_vgb1;
        locals.var_vgb1_dc_dn5 = locals.var_vgb1_dn5;
        locals.var_vgb1_dc_dn6 = locals.var_vgb1_dn6;
        locals.var_vgb1_dc_dn7 = locals.var_vgb1_dn7;
        locals.var_vgb1_dc_dn8 = locals.var_vgb1_dn8;

        locals.var_vsbx_dc = locals.var_vsbx;
        locals.var_vsbx_dc_dn5 = locals.var_vsbx_dn5;
        locals.var_vsbx_dc_dn6 = locals.var_vsbx_dn6;
        locals.var_vsbx_dc_dn7 = locals.var_vsbx_dn7;
        locals.var_vsbx_dc_dn8 = locals.var_vsbx_dn8;

        locals.var_phit1_dc = locals.var_phit1;
        locals.var_phit1_dc_dn5 = locals.var_phit1_dn5;
        locals.var_phit1_dc_dn6 = locals.var_phit1_dn6;
        locals.var_phit1_dc_dn7 = locals.var_phit1_dn7;
        locals.var_phit1_dc_dn8 = locals.var_phit1_dn8;

        locals.var_inv_phit1_dc = locals.var_inv_phit1;
        locals.var_inv_phit1_dc_dn5 = locals.var_inv_phit1_dn5;
        locals.var_inv_phit1_dc_dn6 = locals.var_inv_phit1_dn6;
        locals.var_inv_phit1_dc_dn7 = locals.var_inv_phit1_dn7;
        locals.var_inv_phit1_dc_dn8 = locals.var_inv_phit1_dn8;

        locals.var_gf_dc = locals.var_gf;
        locals.var_gf_dc_dn5 = locals.var_gf_dn5;
        locals.var_gf_dc_dn6 = locals.var_gf_dn6;
        locals.var_gf_dc_dn7 = locals.var_gf_dn7;
        locals.var_gf_dc_dn8 = locals.var_gf_dn8;

        locals.var_gf2_dc = locals.var_gf2;
        locals.var_gf2_dc_dn5 = locals.var_gf2_dn5;
        locals.var_gf2_dc_dn6 = locals.var_gf2_dn6;
        locals.var_gf2_dc_dn7 = locals.var_gf2_dn7;
        locals.var_gf2_dc_dn8 = locals.var_gf2_dn8;

        locals.var_inv_gf2_dc = locals.var_inv_gf2;
        locals.var_inv_gf2_dc_dn5 = locals.var_inv_gf2_dn5;
        locals.var_inv_gf2_dc_dn6 = locals.var_inv_gf2_dn6;
        locals.var_inv_gf2_dc_dn7 = locals.var_inv_gf2_dn7;
        locals.var_inv_gf2_dc_dn8 = locals.var_inv_gf2_dn8;

    }

    pub(super) fn stamp_transient_block_20(
        locals: &mut StampLocals,
    ) {
        locals.var_xg_dc = locals.var_xg;
        locals.var_xg_dc_dn5 = locals.var_xg_dn5;
        locals.var_xg_dc_dn6 = locals.var_xg_dn6;
        locals.var_xg_dc_dn7 = locals.var_xg_dn7;
        locals.var_xg_dc_dn8 = locals.var_xg_dn8;

        locals.var_xno_s_dc = locals.var_xno_s;
        locals.var_xno_s_dc_dn5 = locals.var_xno_s_dn5;
        locals.var_xno_s_dc_dn6 = locals.var_xno_s_dn6;
        locals.var_xno_s_dc_dn7 = locals.var_xno_s_dn7;
        locals.var_xno_s_dc_dn8 = locals.var_xno_s_dn8;

        locals.var_xn_s_dc = locals.var_xn_s;
        locals.var_xn_s_dc_dn5 = locals.var_xn_s_dn5;
        locals.var_xn_s_dc_dn6 = locals.var_xn_s_dn6;
        locals.var_xn_s_dc_dn7 = locals.var_xn_s_dn7;
        locals.var_xn_s_dc_dn8 = locals.var_xn_s_dn8;

        locals.var_xi_dc = locals.var_xi;
        locals.var_xi_dc_dn5 = locals.var_xi_dn5;
        locals.var_xi_dc_dn6 = locals.var_xi_dn6;
        locals.var_xi_dc_dn7 = locals.var_xi_dn7;
        locals.var_xi_dc_dn8 = locals.var_xi_dn8;

        locals.var_margin_dc = locals.var_margin;

        locals.var_inv_xi_dc = locals.var_inv_xi;
        locals.var_inv_xi_dc_dn5 = locals.var_inv_xi_dn5;
        locals.var_inv_xi_dc_dn6 = locals.var_inv_xi_dn6;
        locals.var_inv_xi_dc_dn7 = locals.var_inv_xi_dn7;
        locals.var_inv_xi_dc_dn8 = locals.var_inv_xi_dn8;

        locals.var_sp_s_x1_dc = locals.var_sp_s_x1;
        locals.var_sp_s_x1_dc_dn5 = locals.var_sp_s_x1_dn5;
        locals.var_sp_s_x1_dc_dn6 = locals.var_sp_s_x1_dn6;
        locals.var_sp_s_x1_dc_dn7 = locals.var_sp_s_x1_dn7;
        locals.var_sp_s_x1_dc_dn8 = locals.var_sp_s_x1_dn8;

        locals.var_delta_ns_dc = locals.var_delta_ns;
        locals.var_delta_ns_dc_dn5 = locals.var_delta_ns_dn5;
        locals.var_delta_ns_dc_dn6 = locals.var_delta_ns_dn6;
        locals.var_delta_ns_dc_dn7 = locals.var_delta_ns_dn7;
        locals.var_delta_ns_dc_dn8 = locals.var_delta_ns_dn8;

        locals.var_x_s_dc = locals.var_x_s;
        locals.var_x_s_dc_dn5 = locals.var_x_s_dn5;
        locals.var_x_s_dc_dn6 = locals.var_x_s_dn6;
        locals.var_x_s_dc_dn7 = locals.var_x_s_dn7;
        locals.var_x_s_dc_dn8 = locals.var_x_s_dn8;

        locals.var_xi1s_dc = locals.var_xi1s;
        locals.var_xi1s_dc_dn5 = locals.var_xi1s_dn5;
        locals.var_xi1s_dc_dn6 = locals.var_xi1s_dn6;
        locals.var_xi1s_dc_dn7 = locals.var_xi1s_dn7;
        locals.var_xi1s_dc_dn8 = locals.var_xi1s_dn8;

        locals.var_xi2s_dc = locals.var_xi2s;
        locals.var_xi2s_dc_dn5 = locals.var_xi2s_dn5;
        locals.var_xi2s_dc_dn6 = locals.var_xi2s_dn6;
        locals.var_xi2s_dc_dn7 = locals.var_xi2s_dn7;
        locals.var_xi2s_dc_dn8 = locals.var_xi2s_dn8;

        locals.var_delta_1s_dc = locals.var_delta_1s;
        locals.var_delta_1s_dc_dn5 = locals.var_delta_1s_dn5;
        locals.var_delta_1s_dc_dn6 = locals.var_delta_1s_dn6;
        locals.var_delta_1s_dc_dn7 = locals.var_delta_1s_dn7;
        locals.var_delta_1s_dc_dn8 = locals.var_delta_1s_dn8;

        locals.var_es_dc = locals.var_es;
        locals.var_es_dc_dn5 = locals.var_es_dn5;
        locals.var_es_dc_dn6 = locals.var_es_dn6;
        locals.var_es_dc_dn7 = locals.var_es_dn7;
        locals.var_es_dc_dn8 = locals.var_es_dn8;

        locals.var_ps_dc = locals.var_ps;
        locals.var_ps_dc_dn5 = locals.var_ps_dn5;
        locals.var_ps_dc_dn6 = locals.var_ps_dn6;
        locals.var_ps_dc_dn7 = locals.var_ps_dn7;
        locals.var_ps_dc_dn8 = locals.var_ps_dn8;

        locals.var_ds_dc = locals.var_ds;
        locals.var_ds_dc_dn5 = locals.var_ds_dn5;
        locals.var_ds_dc_dn6 = locals.var_ds_dn6;
        locals.var_ds_dc_dn7 = locals.var_ds_dn7;
        locals.var_ds_dc_dn8 = locals.var_ds_dn8;

        locals.var_sqs_dc = locals.var_sqs;
        locals.var_sqs_dc_dn5 = locals.var_sqs_dn5;
        locals.var_sqs_dc_dn6 = locals.var_sqs_dn6;
        locals.var_sqs_dc_dn7 = locals.var_sqs_dn7;
        locals.var_sqs_dc_dn8 = locals.var_sqs_dn8;

        locals.var_alphas_dc = locals.var_alphas;
        locals.var_alphas_dc_dn5 = locals.var_alphas_dn5;
        locals.var_alphas_dc_dn6 = locals.var_alphas_dn6;
        locals.var_alphas_dc_dn7 = locals.var_alphas_dn7;
        locals.var_alphas_dc_dn8 = locals.var_alphas_dn8;

        locals.var_rxcor_dc = locals.var_rxcor;
        locals.var_rxcor_dc_dn5 = locals.var_rxcor_dn5;
        locals.var_rxcor_dc_dn6 = locals.var_rxcor_dn6;
        locals.var_rxcor_dc_dn7 = locals.var_rxcor_dn7;
        locals.var_rxcor_dc_dn8 = locals.var_rxcor_dn8;

        locals.var_xgs_dc = locals.var_xgs;
        locals.var_xgs_dc_dn5 = locals.var_xgs_dn5;
        locals.var_xgs_dc_dn6 = locals.var_xgs_dn6;
        locals.var_xgs_dc_dn7 = locals.var_xgs_dn7;
        locals.var_xgs_dc_dn8 = locals.var_xgs_dn8;

        locals.var_qis_dc = locals.var_qis;
        locals.var_qis_dc_dn5 = locals.var_qis_dn5;
        locals.var_qis_dc_dn6 = locals.var_qis_dn6;
        locals.var_qis_dc_dn7 = locals.var_qis_dn7;
        locals.var_qis_dc_dn8 = locals.var_qis_dn8;

        locals.var_qbs_dc = locals.var_qbs;
        locals.var_qbs_dc_dn5 = locals.var_qbs_dn5;
        locals.var_qbs_dc_dn6 = locals.var_qbs_dn6;
        locals.var_qbs_dc_dn7 = locals.var_qbs_dn7;
        locals.var_qbs_dc_dn8 = locals.var_qbs_dn8;

        locals.var_rhob_dc = locals.var_rhob;
        locals.var_rhob_dc_dn5 = locals.var_rhob_dn5;
        locals.var_rhob_dc_dn6 = locals.var_rhob_dn6;
        locals.var_rhob_dc_dn7 = locals.var_rhob_dn7;
        locals.var_rhob_dc_dn8 = locals.var_rhob_dn8;

        locals.var_rhog_dc = locals.var_rhog;
        locals.var_rhog_dc_dn5 = locals.var_rhog_dn5;
        locals.var_rhog_dc_dn6 = locals.var_rhog_dn6;
        locals.var_rhog_dc_dn7 = locals.var_rhog_dn7;
        locals.var_rhog_dc_dn8 = locals.var_rhog_dn8;

        locals.var_gmobs_dc = locals.var_gmobs;
        locals.var_gmobs_dc_dn5 = locals.var_gmobs_dn5;
        locals.var_gmobs_dc_dn6 = locals.var_gmobs_dn6;
        locals.var_gmobs_dc_dn7 = locals.var_gmobs_dn7;
        locals.var_gmobs_dc_dn8 = locals.var_gmobs_dn8;

        locals.var_xitsb_dc = locals.var_xitsb;
        locals.var_xitsb_dc_dn5 = locals.var_xitsb_dn5;
        locals.var_xitsb_dc_dn6 = locals.var_xitsb_dn6;
        locals.var_xitsb_dc_dn7 = locals.var_xitsb_dn7;
        locals.var_xitsb_dc_dn8 = locals.var_xitsb_dn8;

        locals.var_factheta_dc = locals.var_factheta;
        locals.var_factheta_dc_dn5 = locals.var_factheta_dn5;
        locals.var_factheta_dc_dn6 = locals.var_factheta_dn6;
        locals.var_factheta_dc_dn7 = locals.var_factheta_dn7;
        locals.var_factheta_dc_dn8 = locals.var_factheta_dn8;

        locals.var_thesat1 = 0.0;
        locals.var_thesat1_dn5 = 0.0;
        locals.var_thesat1_dn6 = 0.0;
        locals.var_thesat1_dn7 = 0.0;
        locals.var_thesat1_dn8 = 0.0;

        let assign43300_e56284: f64 = (locals.var_phit1 * 4.60517018598809);
        locals.var_vdsat_lim = assign43300_e56284;
        locals.var_vdsat_lim_dn5 = (locals.var_phit1_dn5 * 4.60517018598809);
        locals.var_vdsat_lim_dn6 = (locals.var_phit1_dn6 * 4.60517018598809);
        locals.var_vdsat_lim_dn7 = (locals.var_phit1_dn7 * 4.60517018598809);
        locals.var_vdsat_lim_dn8 = (locals.var_phit1_dn8 * 4.60517018598809);

        locals.var_v_dsat = locals.var_vdsat_lim;
        locals.var_v_dsat_dn5 = locals.var_vdsat_lim_dn5;
        locals.var_v_dsat_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_v_dsat_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_v_dsat_dn8 = locals.var_vdsat_lim_dn8;

        locals.var_vdse = locals.var_v_ds;
        locals.var_vdse_dn5 = 0.0;
        locals.var_vdse_dn6 = locals.var_v_ds_dn6;
        locals.var_vdse_dn7 = locals.var_v_ds_dn7;
        locals.var_vdse_dn8 = 0.0;

        let assign43330_e56289: f64 = (locals.var_v_ds * locals.var_inv_phit1);
        locals.var_udse = assign43330_e56289;
        locals.var_udse_dn5 = (locals.var_v_ds * locals.var_inv_phit1_dn5);
        locals.var_udse_dn6 = ((locals.var_v_ds_dn6 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn6));
        locals.var_udse_dn7 = ((locals.var_v_ds_dn7 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn7));
        locals.var_udse_dn8 = (locals.var_v_ds * locals.var_inv_phit1_dn8);

        locals.var_x_d = locals.var_x_s;
        locals.var_x_d_dn5 = locals.var_x_s_dn5;
        locals.var_x_d_dn6 = locals.var_x_s_dn6;
        locals.var_x_d_dn7 = locals.var_x_s_dn7;
        locals.var_x_d_dn8 = locals.var_x_s_dn8;

        locals.var_x_ds = 0.0;
        locals.var_x_ds_dn5 = 0.0;
        locals.var_x_ds_dn6 = 0.0;
        locals.var_x_ds_dn7 = 0.0;
        locals.var_x_ds_dn8 = 0.0;

        locals.var_dps = 0.0;
        locals.var_dps_dn5 = 0.0;
        locals.var_dps_dn6 = 0.0;
        locals.var_dps_dn7 = 0.0;
        locals.var_dps_dn8 = 0.0;

        locals.var_ed = locals.var_es;
        locals.var_ed_dn5 = locals.var_es_dn5;
        locals.var_ed_dn6 = locals.var_es_dn6;
        locals.var_ed_dn7 = locals.var_es_dn7;
        locals.var_ed_dn8 = locals.var_es_dn8;

        locals.var_pd = locals.var_ps;
        locals.var_pd_dn5 = locals.var_ps_dn5;
        locals.var_pd_dn6 = locals.var_ps_dn6;
        locals.var_pd_dn7 = locals.var_ps_dn7;
        locals.var_pd_dn8 = locals.var_ps_dn8;

        locals.var_dd = locals.var_ds;
        locals.var_dd_dn5 = locals.var_ds_dn5;
        locals.var_dd_dn6 = locals.var_ds_dn6;
        locals.var_dd_dn7 = locals.var_ds_dn7;
        locals.var_dd_dn8 = locals.var_ds_dn8;

        locals.var_qbd = locals.var_qbs;
        locals.var_qbd_dn5 = locals.var_qbs_dn5;
        locals.var_qbd_dn6 = locals.var_qbs_dn6;
        locals.var_qbd_dn7 = locals.var_qbs_dn7;
        locals.var_qbd_dn8 = locals.var_qbs_dn8;

        locals.var_x_m = locals.var_x_s;
        locals.var_x_m_dn5 = locals.var_x_s_dn5;
        locals.var_x_m_dn6 = locals.var_x_s_dn6;
        locals.var_x_m_dn7 = locals.var_x_s_dn7;
        locals.var_x_m_dn8 = locals.var_x_s_dn8;

        locals.var_em = locals.var_es;
        locals.var_em_dn5 = locals.var_es_dn5;
        locals.var_em_dn6 = locals.var_es_dn6;
        locals.var_em_dn7 = locals.var_es_dn7;
        locals.var_em_dn8 = locals.var_es_dn8;

        locals.var_dm = locals.var_ds;
        locals.var_dm_dn5 = locals.var_ds_dn5;
        locals.var_dm_dn6 = locals.var_ds_dn6;
        locals.var_dm_dn7 = locals.var_ds_dn7;
        locals.var_dm_dn8 = locals.var_ds_dn8;

        locals.var_pm = locals.var_ps;
        locals.var_pm_dn5 = locals.var_ps_dn5;
        locals.var_pm_dn6 = locals.var_ps_dn6;
        locals.var_pm_dn7 = locals.var_ps_dn7;
        locals.var_pm_dn8 = locals.var_ps_dn8;

        let assign43450_e56303: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgm = assign43450_e56303;
        locals.var_xgm_dn5 = (locals.var_xg_dn5 - locals.var_x_s_dn5);
        locals.var_xgm_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgm_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgm_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);

        locals.var_eta_p = 1.0;
        locals.var_eta_p_dn5 = 0.0;
        locals.var_eta_p_dn6 = 0.0;
        locals.var_eta_p_dn7 = 0.0;
        locals.var_eta_p_dn8 = 0.0;

        locals.var_alpha = 1.0;
        locals.var_alpha_dn5 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;

        locals.var_sqm = 0.0;
        locals.var_sqm_dn5 = 0.0;
        locals.var_sqm_dn6 = 0.0;
        locals.var_sqm_dn7 = 0.0;
        locals.var_sqm_dn8 = 0.0;

        locals.var_qim = locals.var_qis;
        locals.var_qim_dn5 = locals.var_qis_dn5;
        locals.var_qim_dn6 = locals.var_qis_dn6;
        locals.var_qim_dn7 = locals.var_qis_dn7;
        locals.var_qim_dn8 = locals.var_qis_dn8;

        let assign43500_e56310: f64 = (locals.var_xgm * locals.var_phit1);
        locals.var_qeff1 = assign43500_e56310;
        locals.var_qeff1_dn5 = ((locals.var_xgm_dn5 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn5));
        locals.var_qeff1_dn6 = ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6));
        locals.var_qeff1_dn7 = ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7));
        locals.var_qeff1_dn8 = ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8));

        locals.var_qim1 = 0.0;
        locals.var_qim1_dn5 = 0.0;
        locals.var_qim1_dn6 = 0.0;
        locals.var_qim1_dn7 = 0.0;
        locals.var_qim1_dn8 = 0.0;

        locals.var_qbm = locals.var_qbs;
        locals.var_qbm_dn5 = locals.var_qbs_dn5;
        locals.var_qbm_dn6 = locals.var_qbs_dn6;
        locals.var_qbm_dn7 = locals.var_qbs_dn7;
        locals.var_qbm_dn8 = locals.var_qbs_dn8;

        locals.var_s1 = 0.0;
        locals.var_s1_dn5 = 0.0;
        locals.var_s1_dn6 = 0.0;
        locals.var_s1_dn7 = 0.0;
        locals.var_s1_dn8 = 0.0;

        locals.var_gmob = 1.0;
        locals.var_gmob_dn5 = 0.0;
        locals.var_gmob_dn6 = 0.0;
        locals.var_gmob_dn7 = 0.0;
        locals.var_gmob_dn8 = 0.0;

        locals.var_thesateff = locals.var_thesatloc;
        locals.var_thesateff_dn5 = 0.0;
        locals.var_thesateff_dn6 = 0.0;
        locals.var_thesateff_dn7 = 0.0;
        locals.var_thesateff_dn8 = 0.0;

        locals.var_voxm = locals.var_qeff1;
        locals.var_voxm_dn5 = locals.var_qeff1_dn5;
        locals.var_voxm_dn6 = locals.var_qeff1_dn6;
        locals.var_voxm_dn7 = locals.var_qeff1_dn7;
        locals.var_voxm_dn8 = locals.var_qeff1_dn8;

        let assign43570_e56319: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign43570_e56319;

        let assign43580_e56322: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign43580_e56322;

        let (assign43590_e56330, assign43590_e56330_d_n5, assign43590_e56330_d_n6, assign43590_e56330_d_n7, assign43590_e56330_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43590_e56328: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign43590_e56328, (locals.var_thesatloc * locals.var_factheta_dn5), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8,)
    }
};
        locals.var_thesateff = assign43590_e56330;
        locals.var_thesateff_dn5 = assign43590_e56330_d_n5;
        locals.var_thesateff_dn6 = assign43590_e56330_d_n6;
        locals.var_thesateff_dn7 = assign43590_e56330_d_n7;
        locals.var_thesateff_dn8 = assign43590_e56330_d_n8;

        let (assign43600_e56338, assign43600_e56338_d_n5, assign43600_e56338_d_n6, assign43600_e56338_d_n7, assign43600_e56338_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43600_e56336: f64 = (locals.var_thesateff / locals.var_gmobs);
        (assign43600_e56336, (((locals.var_thesateff_dn5 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn5)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn6 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn7 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn8 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)),)
    } else {
        (locals.var_thesat1, locals.var_thesat1_dn5, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8,)
    }
};
        locals.var_thesat1 = assign43600_e56338;
        locals.var_thesat1_dn5 = assign43600_e56338_d_n5;
        locals.var_thesat1_dn6 = assign43600_e56338_d_n6;
        locals.var_thesat1_dn7 = assign43600_e56338_d_n7;
        locals.var_thesat1_dn8 = assign43600_e56338_d_n8;

        let (assign43610_e56348, assign43610_e56348_d_n5, assign43610_e56348_d_n6, assign43610_e56348_d_n7, assign43610_e56348_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43610_e56345: f64 = (0.5 * locals.var_gf2);
        let assign43610_e56346: f64 = (locals.var_xgs + assign43610_e56345);
        (assign43610_e56346, (locals.var_xgs_dn5 + (0.5 * locals.var_gf2_dn5)), (locals.var_xgs_dn6 + (0.5 * locals.var_gf2_dn6)), (locals.var_xgs_dn7 + (0.5 * locals.var_gf2_dn7)), (locals.var_xgs_dn8 + (0.5 * locals.var_gf2_dn8)),)
    } else {
        (locals.var_asat, locals.var_asat_dn5, locals.var_asat_dn6, locals.var_asat_dn7, locals.var_asat_dn8,)
    }
};
        locals.var_asat = assign43610_e56348;
        locals.var_asat_dn5 = assign43610_e56348_d_n5;
        locals.var_asat_dn6 = assign43610_e56348_d_n6;
        locals.var_asat_dn7 = assign43610_e56348_d_n7;
        locals.var_asat_dn8 = assign43610_e56348_d_n8;

        let (assign43620_e56360, assign43620_e56360_d_n5, assign43620_e56360_d_n6, assign43620_e56360_d_n7, assign43620_e56360_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43620_e56354: f64 = (locals.var_gf2 * locals.var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat;
        let assign43620_e56356: f64 = (assign43620_e56354 * __rspice_inv_cse_0);
        let assign43620_e56358: f64 = (assign43620_e56356 * __rspice_inv_cse_0);
        (assign43620_e56358, ((((((((locals.var_gf2_dn5 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn5)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn5)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn5)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn6 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn6)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn7 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn7)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn8 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn8)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43620_e56360;
        locals.var_temp__blk936_dn5 = assign43620_e56360_d_n5;
        locals.var_temp__blk936_dn6 = assign43620_e56360_d_n6;
        locals.var_temp__blk936_dn7 = assign43620_e56360_d_n7;
        locals.var_temp__blk936_dn8 = assign43620_e56360_d_n8;

        let assign43630_e56363: f64 = if locals.var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign43630_e56363;

        let (assign43640_e56373, assign43640_e56373_d_n5, assign43640_e56373_d_n6, assign43640_e56373_d_n7, assign43640_e56373_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign43640_e56371: f64 = (1.0 - locals.var_temp__blk936);
        (assign43640_e56371, (-locals.var_temp__blk936_dn5), (-locals.var_temp__blk936_dn6), (-locals.var_temp__blk936_dn7), (-locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign43640_e56373;
        locals.var_temp1_dn5 = assign43640_e56373_d_n5;
        locals.var_temp1_dn6 = assign43640_e56373_d_n6;
        locals.var_temp1_dn7 = assign43640_e56373_d_n7;
        locals.var_temp1_dn8 = assign43640_e56373_d_n8;

        let assign43650_e56376: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign43650_e56376;

        let (assign43660_e56386, assign43660_e56386_d_n5, assign43660_e56386_d_n6, assign43660_e56386_d_n7, assign43660_e56386_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) && (locals.var_guard1200 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign43660_e56386;
        locals.var_temp2_dn5 = assign43660_e56386_d_n5;
        locals.var_temp2_dn6 = assign43660_e56386_d_n6;
        locals.var_temp2_dn7 = assign43660_e56386_d_n7;
        locals.var_temp2_dn8 = assign43660_e56386_d_n8;

        let (assign43670_e56400, assign43670_e56400_d_n5, assign43670_e56400_d_n6, assign43670_e56400_d_n7, assign43670_e56400_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign43670_e56397: f64 = (locals.var_temp1).sqrt();
        let assign43670_e56398: f64 = (1.0 - assign43670_e56397);
        (assign43670_e56398, (-(locals.var_temp1_dn5 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn6 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn7 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn8 / (2.0 * assign43670_e56397))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign43670_e56400;
        locals.var_temp2_dn5 = assign43670_e56400_d_n5;
        locals.var_temp2_dn6 = assign43670_e56400_d_n6;
        locals.var_temp2_dn7 = assign43670_e56400_d_n7;
        locals.var_temp2_dn8 = assign43670_e56400_d_n8;

        let (assign43680_e56411, assign43680_e56411_d_n5, assign43680_e56411_d_n6, assign43680_e56411_d_n7, assign43680_e56411_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 == 0.0)) {
        let assign43680_e56409: f64 = (0.5 * locals.var_temp__blk936);
        (assign43680_e56409, (0.5 * locals.var_temp__blk936_dn5), (0.5 * locals.var_temp__blk936_dn6), (0.5 * locals.var_temp__blk936_dn7), (0.5 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign43680_e56411;
        locals.var_temp2_dn5 = assign43680_e56411_d_n5;
        locals.var_temp2_dn6 = assign43680_e56411_d_n6;
        locals.var_temp2_dn7 = assign43680_e56411_d_n7;
        locals.var_temp2_dn8 = assign43680_e56411_d_n8;

        let (assign43690_e56419, assign43690_e56419_d_n5, assign43690_e56419_d_n6, assign43690_e56419_d_n7, assign43690_e56419_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43690_e56417: f64 = (locals.var_temp2 * locals.var_asat);
        (assign43690_e56417, ((locals.var_temp2_dn5 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn5)), ((locals.var_temp2_dn6 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn6)), ((locals.var_temp2_dn7 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn7)), ((locals.var_temp2_dn8 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn8)),)
    } else {
        (locals.var_x_inf0, locals.var_x_inf0_dn5, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8,)
    }
};
        locals.var_x_inf0 = assign43690_e56419;
        locals.var_x_inf0_dn5 = assign43690_e56419_d_n5;
        locals.var_x_inf0_dn6 = assign43690_e56419_d_n6;
        locals.var_x_inf0_dn7 = assign43690_e56419_d_n7;
        locals.var_x_inf0_dn8 = assign43690_e56419_d_n8;

        let assign43700_e56426: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign43700_e56426;

        let (assign43710_e56438, assign43710_e56438_d_n5, assign43710_e56438_d_n6, assign43710_e56438_d_n7, assign43710_e56438_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43710_e56434: f64 = (0.475 * locals.var_phit1);
        let assign43710_e56436: f64 = (assign43710_e56434 * locals.var_x_inf0);
        (assign43710_e56436, (((0.475 * locals.var_phit1_dn5) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn5)), (((0.475 * locals.var_phit1_dn6) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn6)), (((0.475 * locals.var_phit1_dn7) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn7)), (((0.475 * locals.var_phit1_dn8) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn8)),)
    } else {
        (locals.var_midphi0, locals.var_midphi0_dn5, locals.var_midphi0_dn6, locals.var_midphi0_dn7, locals.var_midphi0_dn8,)
    }
};
        locals.var_midphi0 = assign43710_e56438;
        locals.var_midphi0_dn5 = assign43710_e56438_d_n5;
        locals.var_midphi0_dn6 = assign43710_e56438_d_n6;
        locals.var_midphi0_dn7 = assign43710_e56438_d_n7;
        locals.var_midphi0_dn8 = assign43710_e56438_d_n8;

        let (assign43720_e56450, assign43720_e56450_d_n5, assign43720_e56450_d_n6, assign43720_e56450_d_n7, assign43720_e56450_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43720_e56447: f64 = (locals.var_alphas * locals.var_midphi0);
        let assign43720_e56448: f64 = (locals.var_qis - assign43720_e56447);
        (assign43720_e56448, (locals.var_qis_dn5 - ((locals.var_alphas_dn5 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn5))), (locals.var_qis_dn6 - ((locals.var_alphas_dn6 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn6))), (locals.var_qis_dn7 - ((locals.var_alphas_dn7 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn7))), (locals.var_qis_dn8 - ((locals.var_alphas_dn8 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn8))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43720_e56450;
        locals.var_temp__blk936_dn5 = assign43720_e56450_d_n5;
        locals.var_temp__blk936_dn6 = assign43720_e56450_d_n6;
        locals.var_temp__blk936_dn7 = assign43720_e56450_d_n7;
        locals.var_temp__blk936_dn8 = assign43720_e56450_d_n8;

    }

    pub(super) fn stamp_transient_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign43730_e56467, assign43730_e56467_d_n5, assign43730_e56467_d_n6, assign43730_e56467_d_n7, assign43730_e56467_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43730_e56460: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign43730_e56462: f64 = (assign43730_e56460 + 1e-12);
        let assign43730_e56463: f64 = (assign43730_e56462).sqrt();
        let assign43730_e56464: f64 = (locals.var_temp__blk936 + assign43730_e56463);
        let assign43730_e56465: f64 = (0.5 * assign43730_e56464);
        (assign43730_e56465, (0.5 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign43730_e56463)))),)
    } else {
        (locals.var_qisat, locals.var_qisat_dn5, locals.var_qisat_dn6, locals.var_qisat_dn7, locals.var_qisat_dn8,)
    }
};
        locals.var_qisat = assign43730_e56467;
        locals.var_qisat_dn5 = assign43730_e56467_d_n5;
        locals.var_qisat_dn6 = assign43730_e56467_d_n6;
        locals.var_qisat_dn7 = assign43730_e56467_d_n7;
        locals.var_qisat_dn8 = assign43730_e56467_d_n8;

        let (assign43740_e56485, assign43740_e56485_d_n5, assign43740_e56485_d_n6, assign43740_e56485_d_n7, assign43740_e56485_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43740_e56475: f64 = (locals.var_phit1 * locals.var_xgs);
        let assign43740_e56477: f64 = (assign43740_e56475 - locals.var_qis);
        let assign43740_e56480: f64 = (locals.var_alphas - 1.0);
        let assign43740_e56482: f64 = (assign43740_e56480 * locals.var_midphi0);
        let assign43740_e56483: f64 = (assign43740_e56477 + assign43740_e56482);
        (assign43740_e56483, ((((locals.var_phit1_dn5 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn5)) - locals.var_qis_dn5) + ((locals.var_alphas_dn5 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn5))), ((((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6)) - locals.var_qis_dn6) + ((locals.var_alphas_dn6 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn6))), ((((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7)) - locals.var_qis_dn7) + ((locals.var_alphas_dn7 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn7))), ((((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8)) - locals.var_qis_dn8) + ((locals.var_alphas_dn8 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn8))),)
    } else {
        (locals.var_qbsat, locals.var_qbsat_dn5, locals.var_qbsat_dn6, locals.var_qbsat_dn7, locals.var_qbsat_dn8,)
    }
};
        locals.var_qbsat = assign43740_e56485;
        locals.var_qbsat_dn5 = assign43740_e56485_d_n5;
        locals.var_qbsat_dn6 = assign43740_e56485_d_n6;
        locals.var_qbsat_dn7 = assign43740_e56485_d_n7;
        locals.var_qbsat_dn8 = assign43740_e56485_d_n8;

        let (assign43750_e56501, assign43750_e56501_d_n5, assign43750_e56501_d_n6, assign43750_e56501_d_n7, assign43750_e56501_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43750_e56494: f64 = (0.5 * locals.var_gf2);
        let assign43750_e56496: f64 = (assign43750_e56494 * locals.var_phit1);
        let assign43750_e56498: f64 = (assign43750_e56496 / locals.var_qbsat);
        let assign43750_e56499: f64 = (1.0 + assign43750_e56498);
        (assign43750_e56499, ((((((0.5 * locals.var_gf2_dn5) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn5)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn6) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn6)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn7) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn7)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn8) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn8)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_alphasat, locals.var_alphasat_dn5, locals.var_alphasat_dn6, locals.var_alphasat_dn7, locals.var_alphasat_dn8,)
    }
};
        locals.var_alphasat = assign43750_e56501;
        locals.var_alphasat_dn5 = assign43750_e56501_d_n5;
        locals.var_alphasat_dn6 = assign43750_e56501_d_n6;
        locals.var_alphasat_dn7 = assign43750_e56501_d_n7;
        locals.var_alphasat_dn8 = assign43750_e56501_d_n8;

        let (assign43760_e56513, assign43760_e56513_d_n5, assign43760_e56513_d_n6, assign43760_e56513_d_n7, assign43760_e56513_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43760_e56510: f64 = (locals.var_eta_mu * locals.var_qisat);
        let assign43760_e56511: f64 = (locals.var_qbsat + assign43760_e56510);
        (assign43760_e56511, (locals.var_qbsat_dn5 + (locals.var_eta_mu * locals.var_qisat_dn5)), (locals.var_qbsat_dn6 + (locals.var_eta_mu * locals.var_qisat_dn6)), (locals.var_qbsat_dn7 + (locals.var_eta_mu * locals.var_qisat_dn7)), (locals.var_qbsat_dn8 + (locals.var_eta_mu * locals.var_qisat_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43760_e56513;
        locals.var_temp__blk936_dn5 = assign43760_e56513_d_n5;
        locals.var_temp__blk936_dn6 = assign43760_e56513_d_n6;
        locals.var_temp__blk936_dn7 = assign43760_e56513_d_n7;
        locals.var_temp__blk936_dn8 = assign43760_e56513_d_n8;

        let (assign43770_e56527, assign43770_e56527_d_n5, assign43770_e56527_d_n6, assign43770_e56527_d_n7, assign43770_e56527_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43770_e56521: f64 = (locals.var_e_eff0 * locals.var_temp__blk936);
        let assign43770_e56523: f64 = (assign43770_e56521 * locals.var_mue_t);
        let assign43770_e56525: f64 = (assign43770_e56523).powf(locals.var_themu_t);
        (assign43770_e56525, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t) / assign43770_e56523))) },)
    } else {
        (locals.var_gmobmusat, locals.var_gmobmusat_dn5, locals.var_gmobmusat_dn6, locals.var_gmobmusat_dn7, locals.var_gmobmusat_dn8,)
    }
};
        locals.var_gmobmusat = assign43770_e56527;
        locals.var_gmobmusat_dn5 = assign43770_e56527_d_n5;
        locals.var_gmobmusat_dn6 = assign43770_e56527_d_n6;
        locals.var_gmobmusat_dn7 = assign43770_e56527_d_n7;
        locals.var_gmobmusat_dn8 = assign43770_e56527_d_n8;

        let (assign43780_e56547, assign43780_e56547_d_n5, assign43780_e56547_d_n6, assign43780_e56547_d_n7, assign43780_e56547_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43780_e56537: f64 = (1.0 - locals.var_eta_mu);
        let assign43780_e56538: f64 = (locals.var_alphasat * assign43780_e56537);
        let assign43780_e56540: f64 = (assign43780_e56538 - 1.0);
        let assign43780_e56541: f64 = (locals.var_themu_t * assign43780_e56540);
        let assign43780_e56543: f64 = (assign43780_e56541 / locals.var_temp__blk936);
        let assign43780_e56545: f64 = (assign43780_e56543 * locals.var_gmobmusat);
        (assign43780_e56545, ((((((locals.var_themu_t * (locals.var_alphasat_dn5 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn5)), ((((((locals.var_themu_t * (locals.var_alphasat_dn6 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat_dn7 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat_dn8 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn8)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign43780_e56547;
        locals.var_temp1_dn5 = assign43780_e56547_d_n5;
        locals.var_temp1_dn6 = assign43780_e56547_d_n6;
        locals.var_temp1_dn7 = assign43780_e56547_d_n7;
        locals.var_temp1_dn8 = assign43780_e56547_d_n8;

        let (assign43790_e56557, assign43790_e56557_d_n5, assign43790_e56557_d_n6, assign43790_e56557_d_n7, assign43790_e56557_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43790_e56555: f64 = (locals.var_qisat / locals.var_qbsat);
        (assign43790_e56555, (((locals.var_qisat_dn5 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn6 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn7 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn8 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43790_e56557;
        locals.var_temp__blk936_dn5 = assign43790_e56557_d_n5;
        locals.var_temp__blk936_dn6 = assign43790_e56557_d_n6;
        locals.var_temp__blk936_dn7 = assign43790_e56557_d_n7;
        locals.var_temp__blk936_dn8 = assign43790_e56557_d_n8;

        let (assign43800_e56572, assign43800_e56572_d_n5, assign43800_e56572_d_n6, assign43800_e56572_d_n7, assign43800_e56572_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43800_e56566: f64 = (1.0 + locals.var_temp__blk936);
        let assign43800_e56568: f64 = (-locals.var_thecs_t);
        let assign43800_e56569: f64 = (assign43800_e56566).powf(assign43800_e56568);
        let assign43800_e56570: f64 = (locals.var_cs_t * assign43800_e56569);
        (assign43800_e56570, (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn5)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn5 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn6)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn6 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn7)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn7 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn8)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn8 / assign43800_e56566))) }),)
    } else {
        (locals.var_gmobcssat, locals.var_gmobcssat_dn5, locals.var_gmobcssat_dn6, locals.var_gmobcssat_dn7, locals.var_gmobcssat_dn8,)
    }
};
        locals.var_gmobcssat = assign43800_e56572;
        locals.var_gmobcssat_dn5 = assign43800_e56572_d_n5;
        locals.var_gmobcssat_dn6 = assign43800_e56572_d_n6;
        locals.var_gmobcssat_dn7 = assign43800_e56572_d_n7;
        locals.var_gmobcssat_dn8 = assign43800_e56572_d_n8;

        let (assign43810_e56594, assign43810_e56594_d_n5, assign43810_e56594_d_n6, assign43810_e56594_d_n7, assign43810_e56594_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43810_e56581: f64 = (locals.var_alphasat - 1.0);
        let assign43810_e56585: f64 = (locals.var_temp__blk936 + 1.0);
        let assign43810_e56586: f64 = (1.0 / assign43810_e56585);
        let assign43810_e56587: f64 = (assign43810_e56581 + assign43810_e56586);
        let assign43810_e56588: f64 = (locals.var_thecs_t * assign43810_e56587);
        let assign43810_e56590: f64 = (assign43810_e56588 / locals.var_qbsat);
        let assign43810_e56592: f64 = (assign43810_e56590 * locals.var_gmobcssat);
        (assign43810_e56592, ((((((locals.var_thecs_t * (locals.var_alphasat_dn5 + (-(locals.var_temp__blk936_dn5 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn5)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn6 + (-(locals.var_temp__blk936_dn6 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn7 + (-(locals.var_temp__blk936_dn7 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn8 + (-(locals.var_temp__blk936_dn8 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign43810_e56594;
        locals.var_temp2_dn5 = assign43810_e56594_d_n5;
        locals.var_temp2_dn6 = assign43810_e56594_d_n6;
        locals.var_temp2_dn7 = assign43810_e56594_d_n7;
        locals.var_temp2_dn8 = assign43810_e56594_d_n8;

        let (assign43820_e56608, assign43820_e56608_d_n5, assign43820_e56608_d_n6, assign43820_e56608_d_n7, assign43820_e56608_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43820_e56602: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign43820_e56604: f64 = (assign43820_e56602 * locals.var_rhog);
        let assign43820_e56606: f64 = (assign43820_e56604 * locals.var_qisat);
        (assign43820_e56606, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn5)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn6)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn7)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn8)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn8)),)
    } else {
        (locals.var_grsat, locals.var_grsat_dn5, locals.var_grsat_dn6, locals.var_grsat_dn7, locals.var_grsat_dn8,)
    }
};
        locals.var_grsat = assign43820_e56608;
        locals.var_grsat_dn5 = assign43820_e56608_d_n5;
        locals.var_grsat_dn6 = assign43820_e56608_d_n6;
        locals.var_grsat_dn7 = assign43820_e56608_d_n7;
        locals.var_grsat_dn8 = assign43820_e56608_d_n8;

        let (assign43830_e56628, assign43830_e56628_d_n5, assign43830_e56628_d_n6, assign43830_e56628_d_n7, assign43830_e56628_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43830_e56618: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign43830_e56620: f64 = (assign43830_e56618 * locals.var_rhog);
        let assign43830_e56622: f64 = (assign43830_e56620 * locals.var_alphasat);
        let assign43830_e56623: f64 = (locals.var_temp1 - assign43830_e56622);
        let assign43830_e56625: f64 = (assign43830_e56623 / locals.var_temp2);
        let assign43830_e56626: f64 = (1.0 + assign43830_e56625);
        (assign43830_e56626, ((((locals.var_temp1_dn5 - (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn5)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn5))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn5)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn6)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn6))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn7)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn7))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn8)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn8))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43830_e56628;
        locals.var_temp__blk936_dn5 = assign43830_e56628_d_n5;
        locals.var_temp__blk936_dn6 = assign43830_e56628_d_n6;
        locals.var_temp__blk936_dn7 = assign43830_e56628_d_n7;
        locals.var_temp__blk936_dn8 = assign43830_e56628_d_n8;

        let assign43840_e56631: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign43840_e56631;

        let (assign43850_e56649, assign43850_e56649_d_n5, assign43850_e56649_d_n6, assign43850_e56649_d_n7, assign43850_e56649_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign43850_e56643: f64 = (2.0 * locals.var_temp__blk936);
        let assign43850_e56644: f64 = (assign43850_e56643).exp();
        let assign43850_e56645: f64 = (1.0 + assign43850_e56644);
        let assign43850_e56646: f64 = (assign43850_e56645).ln();
        let assign43850_e56647: f64 = (0.5 * assign43850_e56646);
        (assign43850_e56647, (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn5)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn6)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn7)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn8)) / assign43850_e56645)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign43850_e56649;
        locals.var_temp1_dn5 = assign43850_e56649_d_n5;
        locals.var_temp1_dn6 = assign43850_e56649_d_n6;
        locals.var_temp1_dn7 = assign43850_e56649_d_n7;
        locals.var_temp1_dn8 = assign43850_e56649_d_n8;

        let (assign43860_e56660, assign43860_e56660_d_n5, assign43860_e56660_d_n6, assign43860_e56660_d_n7, assign43860_e56660_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) && (locals.var_guard1202 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign43860_e56660;
        locals.var_temp1_dn5 = assign43860_e56660_d_n5;
        locals.var_temp1_dn6 = assign43860_e56660_d_n6;
        locals.var_temp1_dn7 = assign43860_e56660_d_n7;
        locals.var_temp1_dn8 = assign43860_e56660_d_n8;

        let (assign43870_e56681, assign43870_e56681_d_n5, assign43870_e56681_d_n6, assign43870_e56681_d_n7, assign43870_e56681_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43870_e56667: f64 = (-locals.var_midphi0);
        let assign43870_e56669: f64 = (assign43870_e56667 * locals.var_temp2);
        let assign43870_e56671: f64 = (assign43870_e56669 * locals.var_temp1);
        let assign43870_e56674: f64 = (1.0 + locals.var_gmobmusat);
        let assign43870_e56676: f64 = (assign43870_e56674 + locals.var_gmobcssat);
        let assign43870_e56678: f64 = (assign43870_e56676 + locals.var_grsat);
        let assign43870_e56679: f64 = (assign43870_e56671 / assign43870_e56678);
        (assign43870_e56679, ((((((((-locals.var_midphi0_dn5) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn5)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn5)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn5 + locals.var_gmobcssat_dn5) + locals.var_grsat_dn5))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn6) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn6)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn6 + locals.var_gmobcssat_dn6) + locals.var_grsat_dn6))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn7) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn7)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn7 + locals.var_gmobcssat_dn7) + locals.var_grsat_dn7))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn8) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn8)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn8 + locals.var_gmobcssat_dn8) + locals.var_grsat_dn8))) / (assign43870_e56678 * assign43870_e56678)),)
    } else {
        (locals.var_delta_gmob, locals.var_delta_gmob_dn5, locals.var_delta_gmob_dn6, locals.var_delta_gmob_dn7, locals.var_delta_gmob_dn8,)
    }
};
        locals.var_delta_gmob = assign43870_e56681;
        locals.var_delta_gmob_dn5 = assign43870_e56681_d_n5;
        locals.var_delta_gmob_dn6 = assign43870_e56681_d_n6;
        locals.var_delta_gmob_dn7 = assign43870_e56681_d_n7;
        locals.var_delta_gmob_dn8 = assign43870_e56681_d_n8;

        let (assign43880_e56702, assign43880_e56702_d_n5, assign43880_e56702_d_n6, assign43880_e56702_d_n7, assign43880_e56702_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign43880_e56694: f64 = (locals.var_delta_gmob * locals.var_delta_gmob);
        let assign43880_e56695: f64 = (1.0 + assign43880_e56694);
        let assign43880_e56696: f64 = (assign43880_e56695).sqrt();
        let assign43880_e56697: f64 = (1.0 + assign43880_e56696);
        let assign43880_e56698: f64 = (locals.var_delta_gmob / assign43880_e56697);
        let assign43880_e56699: f64 = (1.0 + assign43880_e56698);
        let assign43880_e56700: f64 = (locals.var_x_inf0 * assign43880_e56699);
        (assign43880_e56700, ((locals.var_x_inf0_dn5 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn5 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn5 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn5)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn6 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn6 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn6 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn6)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn7 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn7 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn7 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn7)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn8 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn8 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn8 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn8)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))),)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn5, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8,)
    }
};
        locals.var_x_inf = assign43880_e56702;
        locals.var_x_inf_dn5 = assign43880_e56702_d_n5;
        locals.var_x_inf_dn6 = assign43880_e56702_d_n6;
        locals.var_x_inf_dn7 = assign43880_e56702_d_n7;
        locals.var_x_inf_dn8 = assign43880_e56702_d_n8;

        let (assign43890_e56711, assign43890_e56711_d_n5, assign43890_e56711_d_n6, assign43890_e56711_d_n7, assign43890_e56711_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 == 0.0)) {
        (locals.var_x_inf0, locals.var_x_inf0_dn5, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8,)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn5, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8,)
    }
};
        locals.var_x_inf = assign43890_e56711;
        locals.var_x_inf_dn5 = assign43890_e56711_d_n5;
        locals.var_x_inf_dn6 = assign43890_e56711_d_n6;
        locals.var_x_inf_dn7 = assign43890_e56711_d_n7;
        locals.var_x_inf_dn8 = assign43890_e56711_d_n8;

        let (assign43900_e56723, assign43900_e56723_d_n5, assign43900_e56723_d_n6, assign43900_e56723_d_n7, assign43900_e56723_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43900_e56717: f64 = (locals.var_phit1 * locals.var_thesat1);
        let assign43900_e56719: f64 = (assign43900_e56717 * locals.var_x_inf);
        let assign43900_e56721: f64 = (assign43900_e56719 * 0.7071067811865475);
        (assign43900_e56721, (((((locals.var_phit1_dn5 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn5)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn5)) * 0.7071067811865475), (((((locals.var_phit1_dn6 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn6)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn6)) * 0.7071067811865475), (((((locals.var_phit1_dn7 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn7)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn7)) * 0.7071067811865475), (((((locals.var_phit1_dn8 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn8)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn8)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn5, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8,)
    }
};
        locals.var_ysat = assign43900_e56723;
        locals.var_ysat_dn5 = assign43900_e56723_d_n5;
        locals.var_ysat_dn6 = assign43900_e56723_d_n6;
        locals.var_ysat_dn7 = assign43900_e56723_d_n7;
        locals.var_ysat_dn8 = assign43900_e56723_d_n8;

        let assign43910_e56726: f64 = (-1.0);
        let assign43910_e56727: f64 = if locals.var_chnl_type == assign43910_e56726 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign43910_e56727;

        let (assign43920_e56740, assign43920_e56740_d_n5, assign43920_e56740_d_n6, assign43920_e56740_d_n7, assign43920_e56740_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign43920_e56736: f64 = (1.0 + locals.var_ysat);
        let assign43920_e56737: f64 = (assign43920_e56736).sqrt();
        let assign43920_e56738: f64 = (locals.var_ysat / assign43920_e56737);
        (assign43920_e56738, (((locals.var_ysat_dn5 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn5 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn6 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn6 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn7 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn7 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn8 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn8 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn5, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8,)
    }
};
        locals.var_ysat = assign43920_e56740;
        locals.var_ysat_dn5 = assign43920_e56740_d_n5;
        locals.var_ysat_dn6 = assign43920_e56740_d_n6;
        locals.var_ysat_dn7 = assign43920_e56740_d_n7;
        locals.var_ysat_dn8 = assign43920_e56740_d_n8;

        let (assign43930_e56755, assign43930_e56755_d_n5, assign43930_e56755_d_n6, assign43930_e56755_d_n7, assign43930_e56755_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43930_e56749: f64 = (4.0 * locals.var_ysat);
        let assign43930_e56750: f64 = (1.0 + assign43930_e56749);
        let assign43930_e56751: f64 = (assign43930_e56750).sqrt();
        let assign43930_e56752: f64 = (1.0 + assign43930_e56751);
        let assign43930_e56753: f64 = (2.0 / assign43930_e56752);
        (assign43930_e56753, (-((2.0 * ((4.0 * locals.var_ysat_dn5) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn6) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn7) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn8) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))),)
    } else {
        (locals.var_za, locals.var_za_dn5, locals.var_za_dn6, locals.var_za_dn7, locals.var_za_dn8,)
    }
};
        locals.var_za = assign43930_e56755;
        locals.var_za_dn5 = assign43930_e56755_d_n5;
        locals.var_za_dn6 = assign43930_e56755_d_n6;
        locals.var_za_dn7 = assign43930_e56755_d_n7;
        locals.var_za_dn8 = assign43930_e56755_d_n8;

        let (assign43940_e56763, assign43940_e56763_d_n5, assign43940_e56763_d_n6, assign43940_e56763_d_n7, assign43940_e56763_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43940_e56761: f64 = (locals.var_za * locals.var_ysat);
        (assign43940_e56761, ((locals.var_za_dn5 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn5)), ((locals.var_za_dn6 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn6)), ((locals.var_za_dn7 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn7)), ((locals.var_za_dn8 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43940_e56763;
        locals.var_temp__blk936_dn5 = assign43940_e56763_d_n5;
        locals.var_temp__blk936_dn6 = assign43940_e56763_d_n6;
        locals.var_temp__blk936_dn7 = assign43940_e56763_d_n7;
        locals.var_temp__blk936_dn8 = assign43940_e56763_d_n8;

        let (assign43950_e56793, assign43950_e56793_d_n5, assign43950_e56793_d_n6, assign43950_e56793_d_n7, assign43950_e56793_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43950_e56769: f64 = (locals.var_x_inf * locals.var_za);
        let assign43950_e56773: f64 = (0.86 * locals.var_temp__blk936);
        let assign43950_e56777: f64 = (locals.var_temp__blk936 * locals.var_za);
        let assign43950_e56778: f64 = (1.0 - assign43950_e56777);
        let assign43950_e56779: f64 = (assign43950_e56773 * assign43950_e56778);
        let assign43950_e56783: f64 = (4.0 * locals.var_temp__blk936);
        let assign43950_e56785: f64 = (assign43950_e56783 * locals.var_temp__blk936);
        let assign43950_e56787: f64 = (assign43950_e56785 * locals.var_za);
        let assign43950_e56788: f64 = (1.0 + assign43950_e56787);
        let assign43950_e56789: f64 = (assign43950_e56779 / assign43950_e56788);
        let assign43950_e56790: f64 = (1.0 + assign43950_e56789);
        let assign43950_e56791: f64 = (assign43950_e56769 * assign43950_e56790);
        (assign43950_e56791, ((((locals.var_x_inf_dn5 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn5)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn5) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn5 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn5))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn5)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn5)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn6 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn6)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn6) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn6 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn6))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn6)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn6)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn7 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn7)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn7) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn7 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn7))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn7)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn7)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn8 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn8)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn8) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn8 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn8))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn8)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn8)))) / (assign43950_e56788 * assign43950_e56788)))),)
    } else {
        (locals.var_x_0, locals.var_x_0_dn5, locals.var_x_0_dn6, locals.var_x_0_dn7, locals.var_x_0_dn8,)
    }
};
        locals.var_x_0 = assign43950_e56793;
        locals.var_x_0_dn5 = assign43950_e56793_d_n5;
        locals.var_x_0_dn6 = assign43950_e56793_d_n6;
        locals.var_x_0_dn7 = assign43950_e56793_d_n7;
        locals.var_x_0_dn8 = assign43950_e56793_d_n8;

        let (assign43960_e56801, assign43960_e56801_d_n5, assign43960_e56801_d_n6, assign43960_e56801_d_n7, assign43960_e56801_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43960_e56799: f64 = (0.99 * locals.var_x_0);
        (assign43960_e56799, (0.99 * locals.var_x_0_dn5), (0.99 * locals.var_x_0_dn6), (0.99 * locals.var_x_0_dn7), (0.99 * locals.var_x_0_dn8),)
    } else {
        (locals.var_x_sat, locals.var_x_sat_dn5, locals.var_x_sat_dn6, locals.var_x_sat_dn7, locals.var_x_sat_dn8,)
    }
};
        locals.var_x_sat = assign43960_e56801;
        locals.var_x_sat_dn5 = assign43960_e56801_d_n5;
        locals.var_x_sat_dn6 = assign43960_e56801_d_n6;
        locals.var_x_sat_dn7 = assign43960_e56801_d_n7;
        locals.var_x_sat_dn8 = assign43960_e56801_d_n8;

        let (assign43970_e56817, assign43970_e56817_d_n5, assign43970_e56817_d_n6, assign43970_e56817_d_n7, assign43970_e56817_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43970_e56809: f64 = (2.0 * locals.var_asat);
        let assign43970_e56810: f64 = (locals.var_x_sat - assign43970_e56809);
        let assign43970_e56811: f64 = (locals.var_x_sat * assign43970_e56810);
        let assign43970_e56813: f64 = (assign43970_e56811 * locals.var_inv_gf2);
        let assign43970_e56815: f64 = (assign43970_e56813 / locals.var_ds);
        (assign43970_e56815, (((((((locals.var_x_sat_dn5 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn5 - (2.0 * locals.var_asat_dn5)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn5)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn5)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn6 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn6 - (2.0 * locals.var_asat_dn6)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn6)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn7 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn7 - (2.0 * locals.var_asat_dn7)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn7)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn8 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn8 - (2.0 * locals.var_asat_dn8)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn8)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign43970_e56817;
        locals.var_temp__blk936_dn5 = assign43970_e56817_d_n5;
        locals.var_temp__blk936_dn6 = assign43970_e56817_d_n6;
        locals.var_temp__blk936_dn7 = assign43970_e56817_d_n7;
        locals.var_temp__blk936_dn8 = assign43970_e56817_d_n8;

        let (assign43980_e56837, assign43980_e56837_d_n5, assign43980_e56837_d_n6, assign43980_e56837_d_n7, assign43980_e56837_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
        let assign43980_e56826: f64 = (-0.99);
        let (assign43980_e56831, assign43980_e56831_d_n5, assign43980_e56831_d_n6, assign43980_e56831_d_n7, assign43980_e56831_d_n8,) = {
            if (locals.var_temp__blk936 > assign43980_e56826) {
                (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
            } else {
                let assign43980_e56830: f64 = (-0.99);
                (assign43980_e56830, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign43980_e56832: f64 = (1.0 + assign43980_e56831);
        let assign43980_e56833: f64 = (assign43980_e56832).ln();
        let assign43980_e56834: f64 = (locals.var_x_sat - assign43980_e56833);
        let assign43980_e56835: f64 = (locals.var_phit1 * assign43980_e56834);
        (assign43980_e56835, ((locals.var_phit1_dn5 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn5 - (assign43980_e56831_d_n5 / assign43980_e56832)))), ((locals.var_phit1_dn6 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn6 - (assign43980_e56831_d_n6 / assign43980_e56832)))), ((locals.var_phit1_dn7 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn7 - (assign43980_e56831_d_n7 / assign43980_e56832)))), ((locals.var_phit1_dn8 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn8 - (assign43980_e56831_d_n8 / assign43980_e56832)))),)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn5, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8,)
    }
};
        locals.var_v_dsat = assign43980_e56837;
        locals.var_v_dsat_dn5 = assign43980_e56837_d_n5;
        locals.var_v_dsat_dn6 = assign43980_e56837_d_n6;
        locals.var_v_dsat_dn7 = assign43980_e56837_d_n7;
        locals.var_v_dsat_dn8 = assign43980_e56837_d_n8;

        let (assign43990_e56844, assign43990_e56844_d_n5, assign43990_e56844_d_n6, assign43990_e56844_d_n7, assign43990_e56844_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 == 0.0)) {
        (locals.var_vdsat_lim, locals.var_vdsat_lim_dn5, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8,)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn5, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8,)
    }
};
        locals.var_v_dsat = assign43990_e56844;
        locals.var_v_dsat_dn5 = assign43990_e56844_d_n5;
        locals.var_v_dsat_dn6 = assign43990_e56844_d_n6;
        locals.var_v_dsat_dn7 = assign43990_e56844_d_n7;
        locals.var_v_dsat_dn8 = assign43990_e56844_d_n8;

        let (assign44000_e56850, assign44000_e56850_d_n5, assign44000_e56850_d_n6, assign44000_e56850_d_n7, assign44000_e56850_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44000_e56848: f64 = (1.0 + locals.var_arloc);
        (assign44000_e56848, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44000_e56850;
        locals.var_temp__blk936_dn5 = assign44000_e56850_d_n5;
        locals.var_temp__blk936_dn6 = assign44000_e56850_d_n6;
        locals.var_temp__blk936_dn7 = assign44000_e56850_d_n7;
        locals.var_temp__blk936_dn8 = assign44000_e56850_d_n8;

        let (assign44010_e56859, assign44010_e56859_d_n5, assign44010_e56859_d_n6, assign44010_e56859_d_n7, assign44010_e56859_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44010_e56853: f64 = (locals.var_temp__blk936).sqrt();
        let assign44010_e56855: f64 = (assign44010_e56853 * locals.var_v_ds);
        let assign44010_e56857: f64 = (assign44010_e56855 / locals.var_v_dsat);
        (assign44010_e56857, (((((locals.var_temp__blk936_dn5 / (2.0 * assign44010_e56853)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn5)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign44010_e56853)) * locals.var_v_ds) + (assign44010_e56853 * locals.var_v_ds_dn6)) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn6)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign44010_e56853)) * locals.var_v_ds) + (assign44010_e56853 * locals.var_v_ds_dn7)) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn7)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign44010_e56853)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn8)) / (locals.var_v_dsat * locals.var_v_dsat)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign44010_e56859;
        locals.var_temp1_dn5 = assign44010_e56859_d_n5;
        locals.var_temp1_dn6 = assign44010_e56859_d_n6;
        locals.var_temp1_dn7 = assign44010_e56859_d_n7;
        locals.var_temp1_dn8 = assign44010_e56859_d_n8;

        let (assign44020_e56867, assign44020_e56867_d_n5, assign44020_e56867_d_n6, assign44020_e56867_d_n7, assign44020_e56867_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44020_e56863: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign44020_e56865: f64 = (assign44020_e56863 + locals.var_temp__blk936);
        (assign44020_e56865, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign44020_e56867;
        locals.var_temp2_dn5 = assign44020_e56867_d_n5;
        locals.var_temp2_dn6 = assign44020_e56867_d_n6;
        locals.var_temp2_dn7 = assign44020_e56867_d_n7;
        locals.var_temp2_dn8 = assign44020_e56867_d_n8;

        let (assign44030_e56873, assign44030_e56873_d_n5, assign44030_e56873_d_n6, assign44030_e56873_d_n7, assign44030_e56873_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44030_e56871: f64 = (2.0 * locals.var_temp1);
        (assign44030_e56871, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44030_e56873;
        locals.var_temp__blk936_dn5 = assign44030_e56873_d_n5;
        locals.var_temp__blk936_dn6 = assign44030_e56873_d_n6;
        locals.var_temp__blk936_dn7 = assign44030_e56873_d_n7;
        locals.var_temp__blk936_dn8 = assign44030_e56873_d_n8;

    }

    pub(super) fn stamp_transient_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign44040_e56889, assign44040_e56889_d_n5, assign44040_e56889_d_n6, assign44040_e56889_d_n7, assign44040_e56889_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44040_e56877: f64 = (locals.var_v_dsat * locals.var_temp__blk936);
        let assign44040_e56880: f64 = (locals.var_temp2 - locals.var_temp__blk936);
        let assign44040_e56881: f64 = (assign44040_e56880).sqrt();
        let assign44040_e56884: f64 = (locals.var_temp2 + locals.var_temp__blk936);
        let assign44040_e56885: f64 = (assign44040_e56884).sqrt();
        let assign44040_e56886: f64 = (assign44040_e56881 + assign44040_e56885);
        let assign44040_e56887: f64 = (assign44040_e56877 / assign44040_e56886);
        (assign44040_e56887, (((((locals.var_v_dsat_dn5 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn5)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn6 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn6)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn7 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn7)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn8 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn8)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn5, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8,)
    }
};
        locals.var_vdse = assign44040_e56889;
        locals.var_vdse_dn5 = assign44040_e56889_d_n5;
        locals.var_vdse_dn6 = assign44040_e56889_d_n6;
        locals.var_vdse_dn7 = assign44040_e56889_d_n7;
        locals.var_vdse_dn8 = assign44040_e56889_d_n8;

        let (assign44050_e56895, assign44050_e56895_d_n5, assign44050_e56895_d_n6, assign44050_e56895_d_n7, assign44050_e56895_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44050_e56893: f64 = (locals.var_vdse * locals.var_inv_phit1);
        (assign44050_e56893, ((locals.var_vdse_dn5 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn5)), ((locals.var_vdse_dn6 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn6)), ((locals.var_vdse_dn7 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn7)), ((locals.var_vdse_dn8 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn8)),)
    } else {
        (locals.var_udse, locals.var_udse_dn5, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8,)
    }
};
        locals.var_udse = assign44050_e56895;
        locals.var_udse_dn5 = assign44050_e56895_d_n5;
        locals.var_udse_dn6 = assign44050_e56895_d_n6;
        locals.var_udse_dn7 = assign44050_e56895_d_n7;
        locals.var_udse_dn8 = assign44050_e56895_d_n8;

        let (assign44060_e56901, assign44060_e56901_d_n5, assign44060_e56901_d_n6, assign44060_e56901_d_n7, assign44060_e56901_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44060_e56899: f64 = (locals.var_xn_s + locals.var_udse);
        (assign44060_e56899, (locals.var_xn_s_dn5 + locals.var_udse_dn5), (locals.var_xn_s_dn6 + locals.var_udse_dn6), (locals.var_xn_s_dn7 + locals.var_udse_dn7), (locals.var_xn_s_dn8 + locals.var_udse_dn8),)
    } else {
        (locals.var_xn_d, locals.var_xn_d_dn5, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8,)
    }
};
        locals.var_xn_d = assign44060_e56901;
        locals.var_xn_d_dn5 = assign44060_e56901_d_n5;
        locals.var_xn_d_dn6 = assign44060_e56901_d_n6;
        locals.var_xn_d_dn7 = assign44060_e56901_d_n7;
        locals.var_xn_d_dn8 = assign44060_e56901_d_n8;

        let assign44070_e56904: f64 = if locals.var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign44070_e56904;

        let (assign44080_e56912, assign44080_e56912_d_n5, assign44080_e56912_d_n6, assign44080_e56912_d_n7, assign44080_e56912_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1204 != 0.0)) {
        let assign44080_e56909: f64 = (-locals.var_udse);
        let assign44080_e56910: f64 = (assign44080_e56909).exp();
        (assign44080_e56910, (assign44080_e56910 * (-locals.var_udse_dn5)), (assign44080_e56910 * (-locals.var_udse_dn6)), (assign44080_e56910 * (-locals.var_udse_dn7)), (assign44080_e56910 * (-locals.var_udse_dn8)),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn5, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8,)
    }
};
        locals.var_k_ds = assign44080_e56912;
        locals.var_k_ds_dn5 = assign44080_e56912_d_n5;
        locals.var_k_ds_dn6 = assign44080_e56912_d_n6;
        locals.var_k_ds_dn7 = assign44080_e56912_d_n7;
        locals.var_k_ds_dn8 = assign44080_e56912_d_n8;

        let (assign44090_e56941, assign44090_e56941_d_n5, assign44090_e56941_d_n6, assign44090_e56941_d_n7, assign44090_e56941_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1204 == 0.0)) {
        let assign44090_e56921: f64 = (locals.var_udse - 460.51701859880916);
        let assign44090_e56926: f64 = (locals.var_udse - 460.51701859880916);
        let assign44090_e56930: f64 = (locals.var_udse - 460.51701859880916);
        let assign44090_e56932: f64 = (assign44090_e56930 * 0.3333333333333333);
        let assign44090_e56933: f64 = (1.0 + assign44090_e56932);
        let assign44090_e56934: f64 = (assign44090_e56926 * assign44090_e56933);
        let assign44090_e56935: f64 = (0.5 * assign44090_e56934);
        let assign44090_e56936: f64 = (1.0 + assign44090_e56935);
        let assign44090_e56937: f64 = (assign44090_e56921 * assign44090_e56936);
        let assign44090_e56938: f64 = (1.0 + assign44090_e56937);
        let assign44090_e56939: f64 = (1e-200 / assign44090_e56938);
        (assign44090_e56939, (-((1e-200 * ((locals.var_udse_dn5 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn5 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn5 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn6 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn6 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn6 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn7 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn7 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn7 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn8 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn8 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn8 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn5, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8,)
    }
};
        locals.var_k_ds = assign44090_e56941;
        locals.var_k_ds_dn5 = assign44090_e56941_d_n5;
        locals.var_k_ds_dn6 = assign44090_e56941_d_n6;
        locals.var_k_ds_dn7 = assign44090_e56941_d_n7;
        locals.var_k_ds_dn8 = assign44090_e56941_d_n8;

        let (assign44100_e56947, assign44100_e56947_d_n5, assign44100_e56947_d_n6, assign44100_e56947_d_n7, assign44100_e56947_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44100_e56945: f64 = (locals.var_delta_ns * locals.var_k_ds);
        (assign44100_e56945, ((locals.var_delta_ns_dn5 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn5)), ((locals.var_delta_ns_dn6 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn6)), ((locals.var_delta_ns_dn7 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn7)), ((locals.var_delta_ns_dn8 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn8)),)
    } else {
        (locals.var_delta_nd, locals.var_delta_nd_dn5, locals.var_delta_nd_dn6, locals.var_delta_nd_dn7, locals.var_delta_nd_dn8,)
    }
};
        locals.var_delta_nd = assign44100_e56947;
        locals.var_delta_nd_dn5 = assign44100_e56947_d_n5;
        locals.var_delta_nd_dn6 = assign44100_e56947_d_n6;
        locals.var_delta_nd_dn7 = assign44100_e56947_d_n7;
        locals.var_delta_nd_dn8 = assign44100_e56947_d_n8;

        let assign44110_e56949: f64 = (locals.var_xg).abs();
        let assign44110_e56951: f64 = if assign44110_e56949 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign44110_e56951;

        let (assign44120_e56963, assign44120_e56963_d_n5, assign44120_e56963_d_n6, assign44120_e56963_d_n7, assign44120_e56963_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 != 0.0)) {
        let assign44120_e56957: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign44120_e56959: f64 = (assign44120_e56957 * 0.16666666666666666);
        let assign44120_e56961: f64 = (assign44120_e56959 * 0.7071067811865475);
        (assign44120_e56961, ((((locals.var_inv_xi_dn5 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8,)
    }
};
        locals.var_sp_s_temp1 = assign44120_e56963;
        locals.var_sp_s_temp1_dn5 = assign44120_e56963_d_n5;
        locals.var_sp_s_temp1_dn6 = assign44120_e56963_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44120_e56963_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44120_e56963_d_n8;

        let (assign44130_e56983, assign44130_e56983_d_n5, assign44130_e56983_d_n6, assign44130_e56983_d_n7, assign44130_e56983_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 != 0.0)) {
        let assign44130_e56969: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign44130_e56974: f64 = (1.0 - locals.var_delta_nd);
        let assign44130_e56975: f64 = (locals.var_xg * assign44130_e56974);
        let assign44130_e56977: f64 = (assign44130_e56975 * locals.var_gf);
        let assign44130_e56979: f64 = (assign44130_e56977 * locals.var_sp_s_temp1);
        let assign44130_e56980: f64 = (1.0 + assign44130_e56979);
        let assign44130_e56981: f64 = (assign44130_e56969 * assign44130_e56980);
        (assign44130_e56981, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn5 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn5))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn5)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn6 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn6))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn7 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn7))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn8 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn8))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn8)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8,)
    }
};
        locals.var_x_d = assign44130_e56983;
        locals.var_x_d_dn5 = assign44130_e56983_d_n5;
        locals.var_x_d_dn6 = assign44130_e56983_d_n6;
        locals.var_x_d_dn7 = assign44130_e56983_d_n7;
        locals.var_x_d_dn8 = assign44130_e56983_d_n8;

        let (assign44140_e56992, assign44140_e56992_d_n5, assign44140_e56992_d_n6, assign44140_e56992_d_n7, assign44140_e56992_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44140_e56990: f64 = (locals.var_xn_d + 3.0);
        (assign44140_e56990, locals.var_xn_d_dn5, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8,)
    }
};
        locals.var_sp_s_bx = assign44140_e56992;
        locals.var_sp_s_bx_dn5 = assign44140_e56992_d_n5;
        locals.var_sp_s_bx_dn6 = assign44140_e56992_d_n6;
        locals.var_sp_s_bx_dn7 = assign44140_e56992_d_n7;
        locals.var_sp_s_bx_dn8 = assign44140_e56992_d_n8;

        let (assign44150_e57025, assign44150_e57025_d_n5, assign44150_e57025_d_n6, assign44150_e57025_d_n7, assign44150_e57025_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44150_e57000: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign44150_e57003: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44150_e57006: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44150_e57007: f64 = (assign44150_e57003 * assign44150_e57006);
        let assign44150_e57009: f64 = (assign44150_e57007 + 5.0);
        let assign44150_e57010: f64 = (assign44150_e57009).sqrt();
        let assign44150_e57011: f64 = (assign44150_e57000 - assign44150_e57010);
        let assign44150_e57012: f64 = (0.5 * assign44150_e57011);
        let assign44150_e57017: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign44150_e57019: f64 = (assign44150_e57017 + 5.0);
        let assign44150_e57020: f64 = (assign44150_e57019).sqrt();
        let assign44150_e57021: f64 = (locals.var_sp_s_bx - assign44150_e57020);
        let assign44150_e57022: f64 = (0.5 * assign44150_e57021);
        let assign44150_e57023: f64 = (assign44150_e57012 - assign44150_e57022);
        (assign44150_e57023, ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign44150_e57020))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8,)
    }
};
        locals.var_sp_s_eta = assign44150_e57025;
        locals.var_sp_s_eta_dn5 = assign44150_e57025_d_n5;
        locals.var_sp_s_eta_dn6 = assign44150_e57025_d_n6;
        locals.var_sp_s_eta_dn7 = assign44150_e57025_d_n7;
        locals.var_sp_s_eta_dn8 = assign44150_e57025_d_n8;

        let (assign44160_e57034, assign44160_e57034_d_n5, assign44160_e57034_d_n6, assign44160_e57034_d_n7, assign44160_e57034_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44160_e57032: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign44160_e57032, (locals.var_xg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign44160_e57034;
        locals.var_sp_s_temp_dn5 = assign44160_e57034_d_n5;
        locals.var_sp_s_temp_dn6 = assign44160_e57034_d_n6;
        locals.var_sp_s_temp_dn7 = assign44160_e57034_d_n7;
        locals.var_sp_s_temp_dn8 = assign44160_e57034_d_n8;

        let (assign44170_e57043, assign44170_e57043_d_n5, assign44170_e57043_d_n6, assign44170_e57043_d_n7, assign44170_e57043_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44170_e57040: f64 = (-locals.var_sp_s_eta);
        let assign44170_e57041: f64 = (assign44170_e57040).exp();
        (assign44170_e57041, (assign44170_e57041 * (-locals.var_sp_s_eta_dn5)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn6)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn7)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn8)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8,)
    }
};
        locals.var_sp_s_temp1 = assign44170_e57043;
        locals.var_sp_s_temp1_dn5 = assign44170_e57043_d_n5;
        locals.var_sp_s_temp1_dn6 = assign44170_e57043_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44170_e57043_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44170_e57043_d_n8;

        let (assign44180_e57056, assign44180_e57056_d_n5, assign44180_e57056_d_n6, assign44180_e57056_d_n7, assign44180_e57056_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44180_e57052: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44180_e57053: f64 = (2.0 + assign44180_e57052);
        let assign44180_e57054: f64 = (1.0 / assign44180_e57053);
        (assign44180_e57054, (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign44180_e57053 * assign44180_e57053))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8,)
    }
};
        locals.var_sp_s_temp2 = assign44180_e57056;
        locals.var_sp_s_temp2_dn5 = assign44180_e57056_d_n5;
        locals.var_sp_s_temp2_dn6 = assign44180_e57056_d_n6;
        locals.var_sp_s_temp2_dn7 = assign44180_e57056_d_n7;
        locals.var_sp_s_temp2_dn8 = assign44180_e57056_d_n8;

        let (assign44190_e57067, assign44190_e57067_d_n5, assign44190_e57067_d_n6, assign44190_e57067_d_n7, assign44190_e57067_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44190_e57063: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44190_e57065: f64 = (assign44190_e57063 * locals.var_sp_s_temp2);
        (assign44190_e57065, ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn8)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8,)
    }
};
        locals.var_sp_s_xi0 = assign44190_e57067;
        locals.var_sp_s_xi0_dn5 = assign44190_e57067_d_n5;
        locals.var_sp_s_xi0_dn6 = assign44190_e57067_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44190_e57067_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44190_e57067_d_n8;

        let (assign44200_e57080, assign44200_e57080_d_n5, assign44200_e57080_d_n6, assign44200_e57080_d_n7, assign44200_e57080_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44200_e57075: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign44200_e57077: f64 = (assign44200_e57075 * locals.var_sp_s_temp2);
        let assign44200_e57078: f64 = (4.0 * assign44200_e57077);
        (assign44200_e57078, (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn8))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8,)
    }
};
        locals.var_sp_s_xi1 = assign44200_e57080;
        locals.var_sp_s_xi1_dn5 = assign44200_e57080_d_n5;
        locals.var_sp_s_xi1_dn6 = assign44200_e57080_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44200_e57080_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44200_e57080_d_n8;

        let (assign44210_e57097, assign44210_e57097_d_n5, assign44210_e57097_d_n6, assign44210_e57097_d_n7, assign44210_e57097_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44210_e57087: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign44210_e57090: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44210_e57091: f64 = (assign44210_e57087 - assign44210_e57090);
        let assign44210_e57093: f64 = (assign44210_e57091 * locals.var_sp_s_temp2);
        let assign44210_e57095: f64 = (assign44210_e57093 * locals.var_sp_s_temp2);
        (assign44210_e57095, ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn8)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8,)
    }
};
        locals.var_sp_s_xi2 = assign44210_e57097;
        locals.var_sp_s_xi2_dn5 = assign44210_e57097_d_n5;
        locals.var_sp_s_xi2_dn6 = assign44210_e57097_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44210_e57097_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44210_e57097_d_n8;

        let (assign44220_e57145, assign44220_e57145_d_n5, assign44220_e57145_d_n6, assign44220_e57145_d_n7, assign44220_e57145_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44220_e57105: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44220_e57109: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign44220_e57111: f64 = (assign44220_e57109 - 1.0);
        let assign44220_e57115: f64 = (locals.var_sp_s_eta + 1.0);
        let assign44220_e57117: f64 = (assign44220_e57115 + locals.var_sp_s_xi0);
        let assign44220_e57118: f64 = (locals.var_delta_nd * assign44220_e57117);
        let assign44220_e57119: f64 = (assign44220_e57111 - assign44220_e57118);
        let assign44220_e57120: f64 = (locals.var_gf2 * assign44220_e57119);
        let assign44220_e57121: f64 = (assign44220_e57105 - assign44220_e57120);
        let (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,) = {
            if (1e-40 > assign44220_e57121) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44220_e57126: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign44220_e57130: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign44220_e57132: f64 = (assign44220_e57130 - 1.0);
                let assign44220_e57136: f64 = (locals.var_sp_s_eta + 1.0);
                let assign44220_e57138: f64 = (assign44220_e57136 + locals.var_sp_s_xi0);
                let assign44220_e57139: f64 = (locals.var_delta_nd * assign44220_e57138);
                let assign44220_e57140: f64 = (assign44220_e57132 - assign44220_e57139);
                let assign44220_e57141: f64 = (locals.var_gf2 * assign44220_e57140);
                let assign44220_e57142: f64 = (assign44220_e57126 - assign44220_e57141);
                (assign44220_e57142, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_delta_nd_dn5 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_nd_dn6 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_nd_dn7 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_nd_dn8 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8,)
    }
};
        locals.var_sp_s_a = assign44220_e57145;
        locals.var_sp_s_a_dn5 = assign44220_e57145_d_n5;
        locals.var_sp_s_a_dn6 = assign44220_e57145_d_n6;
        locals.var_sp_s_a_dn7 = assign44220_e57145_d_n7;
        locals.var_sp_s_a_dn8 = assign44220_e57145_d_n8;

        let (assign44230_e57162, assign44230_e57162_d_n5, assign44230_e57162_d_n6, assign44230_e57162_d_n7, assign44230_e57162_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44230_e57156: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44230_e57157: f64 = (locals.var_sp_s_temp1 - assign44230_e57156);
        let assign44230_e57158: f64 = (locals.var_gf2 * assign44230_e57157);
        let assign44230_e57159: f64 = (0.5 * assign44230_e57158);
        let assign44230_e57160: f64 = (1.0 - assign44230_e57159);
        (assign44230_e57160, (-(0.5 * ((locals.var_gf2_dn5 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_delta_nd_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8,)
    }
};
        locals.var_sp_s_b = assign44230_e57162;
        locals.var_sp_s_b_dn5 = assign44230_e57162_d_n5;
        locals.var_sp_s_b_dn6 = assign44230_e57162_d_n6;
        locals.var_sp_s_b_dn7 = assign44230_e57162_d_n7;
        locals.var_sp_s_b_dn8 = assign44230_e57162_d_n8;

        let (assign44240_e57183, assign44240_e57183_d_n5, assign44240_e57183_d_n6, assign44240_e57183_d_n7, assign44240_e57183_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44240_e57169: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44240_e57173: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign44240_e57177: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44240_e57178: f64 = (locals.var_delta_nd * assign44240_e57177);
        let assign44240_e57179: f64 = (assign44240_e57173 - assign44240_e57178);
        let assign44240_e57180: f64 = (locals.var_gf2 * assign44240_e57179);
        let assign44240_e57181: f64 = (assign44240_e57169 + assign44240_e57180);
        (assign44240_e57181, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_delta_nd_dn5 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_nd_dn6 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_nd_dn7 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_nd_dn8 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8,)
    }
};
        locals.var_sp_s_c = assign44240_e57183;
        locals.var_sp_s_c_dn5 = assign44240_e57183_d_n5;
        locals.var_sp_s_c_dn6 = assign44240_e57183_d_n6;
        locals.var_sp_s_c_dn7 = assign44240_e57183_d_n7;
        locals.var_sp_s_c_dn8 = assign44240_e57183_d_n8;

        let (assign44250_e57197, assign44250_e57197_d_n5, assign44250_e57197_d_n6, assign44250_e57197_d_n7, assign44250_e57197_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44250_e57190: f64 = (locals.var_xn_d - locals.var_sp_s_eta);
        let assign44250_e57193: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign44250_e57194: f64 = (assign44250_e57193).ln();
        let assign44250_e57195: f64 = (assign44250_e57190 + assign44250_e57194);
        (assign44250_e57195, ((locals.var_xn_d_dn5 - locals.var_sp_s_eta_dn5) + ((((locals.var_sp_s_a_dn5 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn5)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8,)
    }
};
        locals.var_sp_s_tau = assign44250_e57197;
        locals.var_sp_s_tau_dn5 = assign44250_e57197_d_n5;
        locals.var_sp_s_tau_dn6 = assign44250_e57197_d_n6;
        locals.var_sp_s_tau_dn7 = assign44250_e57197_d_n7;
        locals.var_sp_s_tau_dn8 = assign44250_e57197_d_n8;

        let (assign44260_e57206, assign44260_e57206_d_n5, assign44260_e57206_d_n6, assign44260_e57206_d_n7, assign44260_e57206_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44260_e57204: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign44260_e57204, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign44260_e57206;
        locals.var_nu_dn5 = assign44260_e57206_d_n5;
        locals.var_nu_dn6 = assign44260_e57206_d_n6;
        locals.var_nu_dn7 = assign44260_e57206_d_n7;
        locals.var_nu_dn8 = assign44260_e57206_d_n8;

        let (assign44270_e57227, assign44270_e57227_d_n5, assign44270_e57227_d_n6, assign44270_e57227_d_n7, assign44270_e57227_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44270_e57213: f64 = (locals.var_nu * locals.var_nu);
        let assign44270_e57218: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44270_e57219: f64 = (0.5 * assign44270_e57218);
        let assign44270_e57222: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44270_e57223: f64 = (assign44270_e57219 - assign44270_e57222);
        let assign44270_e57224: f64 = (locals.var_sp_s_tau * assign44270_e57223);
        let assign44270_e57225: f64 = (assign44270_e57213 + assign44270_e57224);
        (assign44270_e57225, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign44270_e57227;
        locals.var_mutau_dn5 = assign44270_e57227_d_n5;
        locals.var_mutau_dn6 = assign44270_e57227_d_n6;
        locals.var_mutau_dn7 = assign44270_e57227_d_n7;
        locals.var_mutau_dn8 = assign44270_e57227_d_n8;

        let (assign44280_e57262, assign44280_e57262_d_n5, assign44280_e57262_d_n6, assign44280_e57262_d_n7, assign44280_e57262_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44280_e57235: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign44280_e57237: f64 = (assign44280_e57235 * locals.var_sp_s_tau);
        let assign44280_e57241: f64 = (locals.var_nu / locals.var_mutau);
        let assign44280_e57243: f64 = (assign44280_e57241 * locals.var_sp_s_tau);
        let assign44280_e57245: f64 = (assign44280_e57243 * locals.var_sp_s_tau);
        let assign44280_e57247: f64 = (assign44280_e57245 * locals.var_sp_s_c);
        let assign44280_e57250: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44280_e57252: f64 = (assign44280_e57250 * 0.3333333333333333);
        let assign44280_e57255: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44280_e57256: f64 = (assign44280_e57252 - assign44280_e57255);
        let assign44280_e57257: f64 = (assign44280_e57247 * assign44280_e57256);
        let assign44280_e57258: f64 = (locals.var_mutau + assign44280_e57257);
        let assign44280_e57259: f64 = (assign44280_e57237 / assign44280_e57258);
        let assign44280_e57260: f64 = (locals.var_sp_s_eta + assign44280_e57259);
        (assign44280_e57260, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn5)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn5)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn6)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn6)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn7)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn7)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn8)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn8)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign44280_e57258 * assign44280_e57258))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8,)
    }
};
        locals.var_sp_s_x0 = assign44280_e57262;
        locals.var_sp_s_x0_dn5 = assign44280_e57262_d_n5;
        locals.var_sp_s_x0_dn6 = assign44280_e57262_d_n6;
        locals.var_sp_s_x0_dn7 = assign44280_e57262_d_n7;
        locals.var_sp_s_x0_dn8 = assign44280_e57262_d_n8;

        let assign44290_e57265: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign44290_e57265;

        let (assign44300_e57275, assign44300_e57275_d_n5, assign44300_e57275_d_n6, assign44300_e57275_d_n7, assign44300_e57275_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
        let assign44300_e57273: f64 = (locals.var_sp_s_x0).exp();
        (assign44300_e57273, (assign44300_e57273 * locals.var_sp_s_x0_dn5), (assign44300_e57273 * locals.var_sp_s_x0_dn6), (assign44300_e57273 * locals.var_sp_s_x0_dn7), (assign44300_e57273 * locals.var_sp_s_x0_dn8),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign44300_e57275;
        locals.var_sp_s_delta0_dn5 = assign44300_e57275_d_n5;
        locals.var_sp_s_delta0_dn6 = assign44300_e57275_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44300_e57275_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44300_e57275_d_n8;

        let (assign44310_e57286, assign44310_e57286_d_n5, assign44310_e57286_d_n6, assign44310_e57286_d_n7, assign44310_e57286_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
        let assign44310_e57284: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign44310_e57284, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign44310_e57286;
        locals.var_sp_s_delta1_dn5 = assign44310_e57286_d_n5;
        locals.var_sp_s_delta1_dn6 = assign44310_e57286_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44310_e57286_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44310_e57286_d_n8;

        let (assign44320_e57297, assign44320_e57297_d_n5, assign44320_e57297_d_n6, assign44320_e57297_d_n7, assign44320_e57297_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
        let assign44320_e57295: f64 = (locals.var_delta_nd * locals.var_sp_s_delta0);
        (assign44320_e57295, ((locals.var_delta_nd_dn5 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn5)), ((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign44320_e57297;
        locals.var_sp_s_delta0_dn5 = assign44320_e57297_d_n5;
        locals.var_sp_s_delta0_dn6 = assign44320_e57297_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44320_e57297_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44320_e57297_d_n8;

        let assign44330_e57301: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44330_e57302: f64 = if locals.var_sp_s_x0 > assign44330_e57301 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign44330_e57302;

        let (assign44340_e57317, assign44340_e57317_d_n5, assign44340_e57317_d_n6, assign44340_e57317_d_n7, assign44340_e57317_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign44340_e57314: f64 = (locals.var_sp_s_x0 - locals.var_xn_d);
        let assign44340_e57315: f64 = (assign44340_e57314).exp();
        (assign44340_e57315, (assign44340_e57315 * (locals.var_sp_s_x0_dn5 - locals.var_xn_d_dn5)), (assign44340_e57315 * (locals.var_sp_s_x0_dn6 - locals.var_xn_d_dn6)), (assign44340_e57315 * (locals.var_sp_s_x0_dn7 - locals.var_xn_d_dn7)), (assign44340_e57315 * (locals.var_sp_s_x0_dn8 - locals.var_xn_d_dn8)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign44340_e57317;
        locals.var_sp_s_delta0_dn5 = assign44340_e57317_d_n5;
        locals.var_sp_s_delta0_dn6 = assign44340_e57317_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44340_e57317_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44340_e57317_d_n8;

        let (assign44350_e57331, assign44350_e57331_d_n5, assign44350_e57331_d_n6, assign44350_e57331_d_n7, assign44350_e57331_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign44350_e57329: f64 = (locals.var_delta_nd / locals.var_sp_s_delta0);
        (assign44350_e57329, (((locals.var_delta_nd_dn5 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn5)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign44350_e57331;
        locals.var_sp_s_delta1_dn5 = assign44350_e57331_d_n5;
        locals.var_sp_s_delta1_dn6 = assign44350_e57331_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44350_e57331_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44350_e57331_d_n8;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign44360_e57372, assign44360_e57372_d_n5, assign44360_e57372_d_n6, assign44360_e57372_d_n7, assign44360_e57372_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign44360_e57346: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44360_e57348: f64 = (assign44360_e57346 - 230.25850929940458);
        let assign44360_e57353: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44360_e57355: f64 = (assign44360_e57353 - 230.25850929940458);
        let assign44360_e57359: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44360_e57361: f64 = (assign44360_e57359 - 230.25850929940458);
        let assign44360_e57363: f64 = (assign44360_e57361 * 0.3333333333333333);
        let assign44360_e57364: f64 = (1.0 + assign44360_e57363);
        let assign44360_e57365: f64 = (assign44360_e57355 * assign44360_e57364);
        let assign44360_e57366: f64 = (0.5 * assign44360_e57365);
        let assign44360_e57367: f64 = (1.0 + assign44360_e57366);
        let assign44360_e57368: f64 = (assign44360_e57348 * assign44360_e57367);
        let assign44360_e57369: f64 = (1.0 + assign44360_e57368);
        let assign44360_e57370: f64 = (1e-100 / assign44360_e57369);
        (assign44360_e57370, (-((1e-100 * (((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8,)
    }
};
        locals.var_sp_s_delta0 = assign44360_e57372;
        locals.var_sp_s_delta0_dn5 = assign44360_e57372_d_n5;
        locals.var_sp_s_delta0_dn6 = assign44360_e57372_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44360_e57372_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44360_e57372_d_n8;

        let (assign44370_e57407, assign44370_e57407_d_n5, assign44370_e57407_d_n6, assign44370_e57407_d_n7, assign44370_e57407_d_n8,) = {
    if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign44370_e57387: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57392: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57396: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57398: f64 = (assign44370_e57396 * 0.3333333333333333);
        let assign44370_e57399: f64 = (1.0 + assign44370_e57398);
        let assign44370_e57400: f64 = (assign44370_e57392 * assign44370_e57399);
        let assign44370_e57401: f64 = (0.5 * assign44370_e57400);
        let assign44370_e57402: f64 = (1.0 + assign44370_e57401);
        let assign44370_e57403: f64 = (assign44370_e57387 * assign44370_e57402);
        let assign44370_e57404: f64 = (1.0 + assign44370_e57403);
        let assign44370_e57405: f64 = (1e-100 / assign44370_e57404);
        (assign44370_e57405, (-((1e-100 * ((locals.var_sp_s_x0_dn5 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn5 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8,)
    }
};
        locals.var_sp_s_delta1 = assign44370_e57407;
        locals.var_sp_s_delta1_dn5 = assign44370_e57407_d_n5;
        locals.var_sp_s_delta1_dn6 = assign44370_e57407_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44370_e57407_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44370_e57407_d_n8;

        let (assign44380_e57420, assign44380_e57420_d_n5, assign44380_e57420_d_n6, assign44380_e57420_d_n7, assign44380_e57420_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44380_e57416: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44380_e57417: f64 = (2.0 + assign44380_e57416);
        let assign44380_e57418: f64 = (1.0 / assign44380_e57417);
        (assign44380_e57418, (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign44380_e57417 * assign44380_e57417))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign44380_e57420;
        locals.var_sp_s_temp_dn5 = assign44380_e57420_d_n5;
        locals.var_sp_s_temp_dn6 = assign44380_e57420_d_n6;
        locals.var_sp_s_temp_dn7 = assign44380_e57420_d_n7;
        locals.var_sp_s_temp_dn8 = assign44380_e57420_d_n8;

        let (assign44390_e57431, assign44390_e57431_d_n5, assign44390_e57431_d_n6, assign44390_e57431_d_n7, assign44390_e57431_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44390_e57427: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44390_e57429: f64 = (assign44390_e57427 * locals.var_sp_s_temp);
        (assign44390_e57429, ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8,)
    }
};
        locals.var_sp_s_xi0 = assign44390_e57431;
        locals.var_sp_s_xi0_dn5 = assign44390_e57431_d_n5;
        locals.var_sp_s_xi0_dn6 = assign44390_e57431_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44390_e57431_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44390_e57431_d_n8;

        let (assign44400_e57444, assign44400_e57444_d_n5, assign44400_e57444_d_n6, assign44400_e57444_d_n7, assign44400_e57444_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44400_e57439: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign44400_e57441: f64 = (assign44400_e57439 * locals.var_sp_s_temp);
        let assign44400_e57442: f64 = (4.0 * assign44400_e57441);
        (assign44400_e57442, (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn8))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8,)
    }
};
        locals.var_sp_s_xi1 = assign44400_e57444;
        locals.var_sp_s_xi1_dn5 = assign44400_e57444_d_n5;
        locals.var_sp_s_xi1_dn6 = assign44400_e57444_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44400_e57444_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44400_e57444_d_n8;

        let (assign44410_e57461, assign44410_e57461_d_n5, assign44410_e57461_d_n6, assign44410_e57461_d_n7, assign44410_e57461_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44410_e57451: f64 = (8.0 * locals.var_sp_s_temp);
        let assign44410_e57454: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44410_e57455: f64 = (assign44410_e57451 - assign44410_e57454);
        let assign44410_e57457: f64 = (assign44410_e57455 * locals.var_sp_s_temp);
        let assign44410_e57459: f64 = (assign44410_e57457 * locals.var_sp_s_temp);
        (assign44410_e57459, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn8)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8,)
    }
};
        locals.var_sp_s_xi2 = assign44410_e57461;
        locals.var_sp_s_xi2_dn5 = assign44410_e57461_d_n5;
        locals.var_sp_s_xi2_dn6 = assign44410_e57461_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44410_e57461_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44410_e57461_d_n8;

        let (assign44420_e57470, assign44420_e57470_d_n5, assign44420_e57470_d_n6, assign44420_e57470_d_n7, assign44420_e57470_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44420_e57468: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign44420_e57468, (locals.var_xg_dn5 - locals.var_sp_s_x0_dn5), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign44420_e57470;
        locals.var_sp_s_temp_dn5 = assign44420_e57470_d_n5;
        locals.var_sp_s_temp_dn6 = assign44420_e57470_d_n6;
        locals.var_sp_s_temp_dn7 = assign44420_e57470_d_n7;
        locals.var_sp_s_temp_dn8 = assign44420_e57470_d_n8;

        let (assign44430_e57493, assign44430_e57493_d_n5, assign44430_e57493_d_n6, assign44430_e57493_d_n7, assign44430_e57493_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44430_e57477: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44430_e57481: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign44430_e57483: f64 = (assign44430_e57481 + locals.var_sp_s_delta0);
        let assign44430_e57487: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44430_e57488: f64 = (locals.var_delta_nd * assign44430_e57487);
        let assign44430_e57489: f64 = (assign44430_e57483 - assign44430_e57488);
        let assign44430_e57490: f64 = (locals.var_gf2 * assign44430_e57489);
        let assign44430_e57491: f64 = (assign44430_e57477 + assign44430_e57490);
        (assign44430_e57491, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8,)
    }
};
        locals.var_sp_s_pc = assign44430_e57493;
        locals.var_sp_s_pc_dn5 = assign44430_e57493_d_n5;
        locals.var_sp_s_pc_dn6 = assign44430_e57493_d_n6;
        locals.var_sp_s_pc_dn7 = assign44430_e57493_d_n7;
        locals.var_sp_s_pc_dn8 = assign44430_e57493_d_n8;

        let (assign44440_e57520, assign44440_e57520_d_n5, assign44440_e57520_d_n6, assign44440_e57520_d_n7, assign44440_e57520_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44440_e57500: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44440_e57504: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign44440_e57506: f64 = (assign44440_e57504 - 1.0);
        let assign44440_e57508: f64 = (assign44440_e57506 + locals.var_sp_s_delta0);
        let assign44440_e57512: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign44440_e57514: f64 = (assign44440_e57512 + locals.var_sp_s_xi0);
        let assign44440_e57515: f64 = (locals.var_delta_nd * assign44440_e57514);
        let assign44440_e57516: f64 = (assign44440_e57508 - assign44440_e57515);
        let assign44440_e57517: f64 = (locals.var_gf2 * assign44440_e57516);
        let assign44440_e57518: f64 = (assign44440_e57500 - assign44440_e57517);
        (assign44440_e57518, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8,)
    }
};
        locals.var_sp_s_qc = assign44440_e57520;
        locals.var_sp_s_qc_dn5 = assign44440_e57520_d_n5;
        locals.var_sp_s_qc_dn6 = assign44440_e57520_d_n6;
        locals.var_sp_s_qc_dn7 = assign44440_e57520_d_n7;
        locals.var_sp_s_qc_dn8 = assign44440_e57520_d_n8;

        let (assign44450_e57537, assign44450_e57537_d_n5, assign44450_e57537_d_n6, assign44450_e57537_d_n7, assign44450_e57537_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44450_e57529: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign44450_e57532: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44450_e57533: f64 = (assign44450_e57529 - assign44450_e57532);
        let assign44450_e57534: f64 = (locals.var_gf2 * assign44450_e57533);
        let assign44450_e57535: f64 = (2.0 - assign44450_e57534);
        (assign44450_e57535, (-((locals.var_gf2_dn5 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign44450_e57537;
        locals.var_sp_s_temp_dn5 = assign44450_e57537_d_n5;
        locals.var_sp_s_temp_dn6 = assign44450_e57537_d_n6;
        locals.var_sp_s_temp_dn7 = assign44450_e57537_d_n7;
        locals.var_sp_s_temp_dn8 = assign44450_e57537_d_n8;

        let (assign44460_e57552, assign44460_e57552_d_n5, assign44460_e57552_d_n6, assign44460_e57552_d_n7, assign44460_e57552_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44460_e57544: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign44460_e57548: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign44460_e57549: f64 = (2.0 * assign44460_e57548);
        let assign44460_e57550: f64 = (assign44460_e57544 - assign44460_e57549);
        (assign44460_e57550, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8,)
    }
};
        locals.var_sp_s_temp = assign44460_e57552;
        locals.var_sp_s_temp_dn5 = assign44460_e57552_d_n5;
        locals.var_sp_s_temp_dn6 = assign44460_e57552_d_n6;
        locals.var_sp_s_temp_dn7 = assign44460_e57552_d_n7;
        locals.var_sp_s_temp_dn8 = assign44460_e57552_d_n8;

        let (assign44470_e57568, assign44470_e57568_d_n5, assign44470_e57568_d_n6, assign44470_e57568_d_n7, assign44470_e57568_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign44470_e57562: f64 = (locals.var_sp_s_temp).sqrt();
        let assign44470_e57563: f64 = (locals.var_sp_s_pc + assign44470_e57562);
        let assign44470_e57564: f64 = (locals.var_sp_s_qc / assign44470_e57563);
        let assign44470_e57565: f64 = (2.0 * assign44470_e57564);
        let assign44470_e57566: f64 = (locals.var_sp_s_x0 + assign44470_e57565);
        (assign44470_e57566, (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8,)
    }
};
        locals.var_x_d = assign44470_e57568;
        locals.var_x_d_dn5 = assign44470_e57568_d_n5;
        locals.var_x_d_dn6 = assign44470_e57568_d_n6;
        locals.var_x_d_dn7 = assign44470_e57568_d_n7;
        locals.var_x_d_dn8 = assign44470_e57568_d_n8;

        let (assign44480_e57574, assign44480_e57574_d_n5, assign44480_e57574_d_n6, assign44480_e57574_d_n7, assign44480_e57574_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44480_e57572: f64 = (locals.var_x_d - locals.var_x_s);
        (assign44480_e57572, (locals.var_x_d_dn5 - locals.var_x_s_dn5), (locals.var_x_d_dn6 - locals.var_x_s_dn6), (locals.var_x_d_dn7 - locals.var_x_s_dn7), (locals.var_x_d_dn8 - locals.var_x_s_dn8),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8,)
    }
};
        locals.var_x_ds = assign44480_e57574;
        locals.var_x_ds_dn5 = assign44480_e57574_d_n5;
        locals.var_x_ds_dn6 = assign44480_e57574_d_n6;
        locals.var_x_ds_dn7 = assign44480_e57574_d_n7;
        locals.var_x_ds_dn8 = assign44480_e57574_d_n8;

        let assign44490_e57577: f64 = if locals.var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign44490_e57577;

        let (assign44500_e57603, assign44500_e57603_d_n5, assign44500_e57603_d_n6, assign44500_e57603_d_n7, assign44500_e57603_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44500_e57584: f64 = (locals.var_xg - locals.var_x_s);
        let assign44500_e57585: f64 = (2.0 * assign44500_e57584);
        let assign44500_e57589: f64 = (1.0 - locals.var_es);
        let assign44500_e57592: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44500_e57593: f64 = (assign44500_e57589 + assign44500_e57592);
        let assign44500_e57597: f64 = (1.0 + locals.var_xi1s);
        let assign44500_e57598: f64 = (locals.var_delta_nd * assign44500_e57597);
        let assign44500_e57599: f64 = (assign44500_e57593 - assign44500_e57598);
        let assign44500_e57600: f64 = (locals.var_gf2 * assign44500_e57599);
        let assign44500_e57601: f64 = (assign44500_e57585 + assign44500_e57600);
        (assign44500_e57601, ((2.0 * (locals.var_xg_dn5 - locals.var_x_s_dn5)) + ((locals.var_gf2_dn5 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn5) + ((locals.var_delta_1s_dn5 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn5))) - ((locals.var_delta_nd_dn5 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn5)))))), ((2.0 * (locals.var_xg_dn6 - locals.var_x_s_dn6)) + ((locals.var_gf2_dn6 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn6) + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn6)))))), ((2.0 * (locals.var_xg_dn7 - locals.var_x_s_dn7)) + ((locals.var_gf2_dn7 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn7) + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn7)))))), ((2.0 * (locals.var_xg_dn8 - locals.var_x_s_dn8)) + ((locals.var_gf2_dn8 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn8) + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn8)))))),)
    } else {
        (locals.var_pc, locals.var_pc_dn5, locals.var_pc_dn6, locals.var_pc_dn7, locals.var_pc_dn8,)
    }
};
        locals.var_pc = assign44500_e57603;
        locals.var_pc_dn5 = assign44500_e57603_d_n5;
        locals.var_pc_dn6 = assign44500_e57603_d_n6;
        locals.var_pc_dn7 = assign44500_e57603_d_n7;
        locals.var_pc_dn8 = assign44500_e57603_d_n8;

        let (assign44510_e57615, assign44510_e57615_d_n5, assign44510_e57615_d_n6, assign44510_e57615_d_n7, assign44510_e57615_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44510_e57610: f64 = (1.0 - locals.var_k_ds);
        let assign44510_e57611: f64 = (locals.var_gf2 * assign44510_e57610);
        let assign44510_e57613: f64 = (assign44510_e57611 * locals.var_ds);
        (assign44510_e57613, ((((locals.var_gf2_dn5 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn5))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn5)), ((((locals.var_gf2_dn6 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn6))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn6)), ((((locals.var_gf2_dn7 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn7))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn7)), ((((locals.var_gf2_dn8 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn8))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn8)),)
    } else {
        (locals.var_qc, locals.var_qc_dn5, locals.var_qc_dn6, locals.var_qc_dn7, locals.var_qc_dn8,)
    }
};
        locals.var_qc = assign44510_e57615;
        locals.var_qc_dn5 = assign44510_e57615_d_n5;
        locals.var_qc_dn6 = assign44510_e57615_d_n6;
        locals.var_qc_dn7 = assign44510_e57615_d_n7;
        locals.var_qc_dn8 = assign44510_e57615_d_n8;

        let (assign44520_e57633, assign44520_e57633_d_n5, assign44520_e57633_d_n6, assign44520_e57633_d_n7, assign44520_e57633_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44520_e57624: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44520_e57625: f64 = (locals.var_es + assign44520_e57624);
        let assign44520_e57628: f64 = (locals.var_delta_nd * locals.var_xi2s);
        let assign44520_e57629: f64 = (assign44520_e57625 - assign44520_e57628);
        let assign44520_e57630: f64 = (locals.var_gf2 * assign44520_e57629);
        let assign44520_e57631: f64 = (2.0 - assign44520_e57630);
        (assign44520_e57631, (-((locals.var_gf2_dn5 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn5 + ((locals.var_delta_1s_dn5 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn5))) - ((locals.var_delta_nd_dn5 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn5)))))), (-((locals.var_gf2_dn6 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn6 + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn6)))))), (-((locals.var_gf2_dn7 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn7 + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn7)))))), (-((locals.var_gf2_dn8 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn8 + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn8)))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44520_e57633;
        locals.var_temp__blk936_dn5 = assign44520_e57633_d_n5;
        locals.var_temp__blk936_dn6 = assign44520_e57633_d_n6;
        locals.var_temp__blk936_dn7 = assign44520_e57633_d_n7;
        locals.var_temp__blk936_dn8 = assign44520_e57633_d_n8;

        let (assign44530_e57647, assign44530_e57647_d_n5, assign44530_e57647_d_n6, assign44530_e57647_d_n7, assign44530_e57647_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44530_e57639: f64 = (locals.var_pc * locals.var_pc);
        let assign44530_e57643: f64 = (locals.var_temp__blk936 * locals.var_qc);
        let assign44530_e57644: f64 = (2.0 * assign44530_e57643);
        let assign44530_e57645: f64 = (assign44530_e57639 - assign44530_e57644);
        (assign44530_e57645, (((locals.var_pc_dn5 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn5)) - (2.0 * ((locals.var_temp__blk936_dn5 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn5)))), (((locals.var_pc_dn6 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn6)) - (2.0 * ((locals.var_temp__blk936_dn6 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn6)))), (((locals.var_pc_dn7 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn7)) - (2.0 * ((locals.var_temp__blk936_dn7 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn7)))), (((locals.var_pc_dn8 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn8)) - (2.0 * ((locals.var_temp__blk936_dn8 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44530_e57647;
        locals.var_temp__blk936_dn5 = assign44530_e57647_d_n5;
        locals.var_temp__blk936_dn6 = assign44530_e57647_d_n6;
        locals.var_temp__blk936_dn7 = assign44530_e57647_d_n7;
        locals.var_temp__blk936_dn8 = assign44530_e57647_d_n8;

        let (assign44540_e57660, assign44540_e57660_d_n5, assign44540_e57660_d_n6, assign44540_e57660_d_n7, assign44540_e57660_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44540_e57655: f64 = (locals.var_temp__blk936).sqrt();
        let assign44540_e57656: f64 = (locals.var_pc + assign44540_e57655);
        let assign44540_e57657: f64 = (locals.var_qc / assign44540_e57656);
        let assign44540_e57658: f64 = (2.0 * assign44540_e57657);
        (assign44540_e57658, (2.0 * (((locals.var_qc_dn5 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn5 + (locals.var_temp__blk936_dn5 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn6 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn6 + (locals.var_temp__blk936_dn6 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn7 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn7 + (locals.var_temp__blk936_dn7 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn8 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn8 + (locals.var_temp__blk936_dn8 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8,)
    }
};
        locals.var_x_ds = assign44540_e57660;
        locals.var_x_ds_dn5 = assign44540_e57660_d_n5;
        locals.var_x_ds_dn6 = assign44540_e57660_d_n6;
        locals.var_x_ds_dn7 = assign44540_e57660_d_n7;
        locals.var_x_ds_dn8 = assign44540_e57660_d_n8;

        let (assign44550_e57668, assign44550_e57668_d_n5, assign44550_e57668_d_n6, assign44550_e57668_d_n7, assign44550_e57668_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign44550_e57666: f64 = (locals.var_x_s + locals.var_x_ds);
        (assign44550_e57666, (locals.var_x_s_dn5 + locals.var_x_ds_dn5), (locals.var_x_s_dn6 + locals.var_x_ds_dn6), (locals.var_x_s_dn7 + locals.var_x_ds_dn7), (locals.var_x_s_dn8 + locals.var_x_ds_dn8),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8,)
    }
};
        locals.var_x_d = assign44550_e57668;
        locals.var_x_d_dn5 = assign44550_e57668_d_n5;
        locals.var_x_d_dn6 = assign44550_e57668_d_n6;
        locals.var_x_d_dn7 = assign44550_e57668_d_n7;
        locals.var_x_d_dn8 = assign44550_e57668_d_n8;

        let (assign44560_e57674, assign44560_e57674_d_n5, assign44560_e57674_d_n6, assign44560_e57674_d_n7, assign44560_e57674_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44560_e57672: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign44560_e57672, ((locals.var_x_ds_dn5 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn5)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)),)
    } else {
        (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8,)
    }
};
        locals.var_dps = assign44560_e57674;
        locals.var_dps_dn5 = assign44560_e57674_d_n5;
        locals.var_dps_dn6 = assign44560_e57674_d_n6;
        locals.var_dps_dn7 = assign44560_e57674_d_n7;
        locals.var_dps_dn8 = assign44560_e57674_d_n8;

        let (assign44570_e57686, assign44570_e57686_d_n5, assign44570_e57686_d_n6, assign44570_e57686_d_n7, assign44570_e57686_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44570_e57678: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44570_e57682: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44570_e57683: f64 = (2.0 + assign44570_e57682);
        let assign44570_e57684: f64 = (assign44570_e57678 / assign44570_e57683);
        (assign44570_e57684, (((((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)))) / (assign44570_e57683 * assign44570_e57683)),)
    } else {
        (locals.var_xi0d, locals.var_xi0d_dn5, locals.var_xi0d_dn6, locals.var_xi0d_dn7, locals.var_xi0d_dn8,)
    }
};
        locals.var_xi0d = assign44570_e57686;
        locals.var_xi0d_dn5 = assign44570_e57686_d_n5;
        locals.var_xi0d_dn6 = assign44570_e57686_d_n6;
        locals.var_xi0d_dn7 = assign44570_e57686_d_n7;
        locals.var_xi0d_dn8 = assign44570_e57686_d_n8;

        let assign44580_e57689: f64 = if locals.var_x_d < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign44580_e57689;

        let (assign44590_e57697, assign44590_e57697_d_n5, assign44590_e57697_d_n6, assign44590_e57697_d_n7, assign44590_e57697_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign44590_e57694: f64 = (-locals.var_x_d);
        let assign44590_e57695: f64 = (assign44590_e57694).exp();
        (assign44590_e57695, (assign44590_e57695 * (-locals.var_x_d_dn5)), (assign44590_e57695 * (-locals.var_x_d_dn6)), (assign44590_e57695 * (-locals.var_x_d_dn7)), (assign44590_e57695 * (-locals.var_x_d_dn8)),)
    } else {
        (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8,)
    }
};
        locals.var_ed = assign44590_e57697;
        locals.var_ed_dn5 = assign44590_e57697_d_n5;
        locals.var_ed_dn6 = assign44590_e57697_d_n6;
        locals.var_ed_dn7 = assign44590_e57697_d_n7;
        locals.var_ed_dn8 = assign44590_e57697_d_n8;

        let assign44600_e57700: f64 = if locals.var_x_d < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign44600_e57700;

        let (assign44610_e57724, assign44610_e57724_d_n5, assign44610_e57724_d_n6, assign44610_e57724_d_n7, assign44610_e57724_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign44610_e57709: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44610_e57716: f64 = (0.25 * locals.var_x_d);
        let assign44610_e57717: f64 = (1.0 - assign44610_e57716);
        let assign44610_e57718: f64 = (locals.var_x_d * assign44610_e57717);
        let assign44610_e57719: f64 = (0.3333333333333333 * assign44610_e57718);
        let assign44610_e57720: f64 = (1.0 - assign44610_e57719);
        let assign44610_e57721: f64 = (assign44610_e57709 * assign44610_e57720);
        let assign44610_e57722: f64 = (0.5 * assign44610_e57721);
        (assign44610_e57722, (0.5 * ((((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn5 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn5))))))))), (0.5 * ((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6))))))))), (0.5 * ((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7))))))))), (0.5 * ((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8))))))))),)
    } else {
        (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8,)
    }
};
        locals.var_pd = assign44610_e57724;
        locals.var_pd_dn5 = assign44610_e57724_d_n5;
        locals.var_pd_dn6 = assign44610_e57724_d_n6;
        locals.var_pd_dn7 = assign44610_e57724_d_n7;
        locals.var_pd_dn8 = assign44610_e57724_d_n8;

        let (assign44620_e57743, assign44620_e57743_d_n5, assign44620_e57743_d_n6, assign44620_e57743_d_n7, assign44620_e57743_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign44620_e57736: f64 = (0.25 * locals.var_x_d);
        let assign44620_e57737: f64 = (1.0 - assign44620_e57736);
        let assign44620_e57738: f64 = (locals.var_x_d * assign44620_e57737);
        let assign44620_e57739: f64 = (0.3333333333333333 * assign44620_e57738);
        let assign44620_e57740: f64 = (1.0 - assign44620_e57739);
        let assign44620_e57741: f64 = (assign44620_e57740).sqrt();
        (assign44620_e57741, ((-(0.3333333333333333 * ((locals.var_x_d_dn5 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn5)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8)))))) / (2.0 * assign44620_e57741)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44620_e57743;
        locals.var_temp__blk936_dn5 = assign44620_e57743_d_n5;
        locals.var_temp__blk936_dn6 = assign44620_e57743_d_n6;
        locals.var_temp__blk936_dn7 = assign44620_e57743_d_n7;
        locals.var_temp__blk936_dn8 = assign44620_e57743_d_n8;

        let (assign44630_e57755, assign44630_e57755_d_n5, assign44630_e57755_d_n6, assign44630_e57755_d_n7, assign44630_e57755_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign44630_e57752: f64 = (locals.var_x_d * locals.var_temp__blk936);
        let assign44630_e57753: f64 = (0.7071067811865475 * assign44630_e57752);
        (assign44630_e57753, (0.7071067811865475 * ((locals.var_x_d_dn5 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_d_dn6 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_d_dn7 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_d_dn8 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8,)
    }
};
        locals.var_sqd = assign44630_e57755;
        locals.var_sqd_dn5 = assign44630_e57755_d_n5;
        locals.var_sqd_dn6 = assign44630_e57755_d_n6;
        locals.var_sqd_dn7 = assign44630_e57755_d_n7;
        locals.var_sqd_dn8 = assign44630_e57755_d_n8;

        let (assign44640_e57777, assign44640_e57777_d_n5, assign44640_e57777_d_n6, assign44640_e57777_d_n7, assign44640_e57777_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign44640_e57763: f64 = (0.16666666666666666 * locals.var_delta_nd);
        let assign44640_e57765: f64 = (assign44640_e57763 * locals.var_x_d);
        let assign44640_e57767: f64 = (assign44640_e57765 * locals.var_x_d);
        let assign44640_e57769: f64 = (assign44640_e57767 * locals.var_x_d);
        let assign44640_e57773: f64 = (1.75 * locals.var_x_d);
        let assign44640_e57774: f64 = (1.0 + assign44640_e57773);
        let assign44640_e57775: f64 = (assign44640_e57769 * assign44640_e57774);
        (assign44640_e57775, (((((((((0.16666666666666666 * locals.var_delta_nd_dn5) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn5)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn5)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn5)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn5))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn6) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn6)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn7) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn7)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn8) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn8)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn8))),)
    } else {
        (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8,)
    }
};
        locals.var_dd = assign44640_e57777;
        locals.var_dd_dn5 = assign44640_e57777_d_n5;
        locals.var_dd_dn6 = assign44640_e57777_d_n6;
        locals.var_dd_dn7 = assign44640_e57777_d_n7;
        locals.var_dd_dn8 = assign44640_e57777_d_n8;

        let (assign44650_e57790, assign44650_e57790_d_n5, assign44650_e57790_d_n6, assign44650_e57790_d_n7, assign44650_e57790_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign44650_e57786: f64 = (locals.var_x_d - 1.0);
        let assign44650_e57788: f64 = (assign44650_e57786 + locals.var_ed);
        (assign44650_e57788, (locals.var_x_d_dn5 + locals.var_ed_dn5), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8),)
    } else {
        (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8,)
    }
};
        locals.var_pd = assign44650_e57790;
        locals.var_pd_dn5 = assign44650_e57790_d_n5;
        locals.var_pd_dn6 = assign44650_e57790_d_n6;
        locals.var_pd_dn7 = assign44650_e57790_d_n7;
        locals.var_pd_dn8 = assign44650_e57790_d_n8;

        let (assign44660_e57800, assign44660_e57800_d_n5, assign44660_e57800_d_n6, assign44660_e57800_d_n7, assign44660_e57800_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign44660_e57798: f64 = (locals.var_pd).sqrt();
        (assign44660_e57798, (locals.var_pd_dn5 / (2.0 * assign44660_e57798)), (locals.var_pd_dn6 / (2.0 * assign44660_e57798)), (locals.var_pd_dn7 / (2.0 * assign44660_e57798)), (locals.var_pd_dn8 / (2.0 * assign44660_e57798)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8,)
    }
};
        locals.var_sqd = assign44660_e57800;
        locals.var_sqd_dn5 = assign44660_e57800_d_n5;
        locals.var_sqd_dn6 = assign44660_e57800_d_n6;
        locals.var_sqd_dn7 = assign44660_e57800_d_n7;
        locals.var_sqd_dn8 = assign44660_e57800_d_n8;

    }

    pub(super) fn stamp_transient_block_24(
        locals: &mut StampLocals,
    ) {
        let (assign44670_e57819, assign44670_e57819_d_n5, assign44670_e57819_d_n6, assign44670_e57819_d_n7, assign44670_e57819_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign44670_e57810: f64 = (1.0 / locals.var_ed);
        let assign44670_e57812: f64 = (assign44670_e57810 - locals.var_x_d);
        let assign44670_e57814: f64 = (assign44670_e57812 - 1.0);
        let assign44670_e57816: f64 = (assign44670_e57814 - locals.var_xi0d);
        let assign44670_e57817: f64 = (locals.var_delta_nd * assign44670_e57816);
        (assign44670_e57817, ((locals.var_delta_nd_dn5 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn5 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn5) - locals.var_xi0d_dn5))), ((locals.var_delta_nd_dn6 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn6 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn6) - locals.var_xi0d_dn6))), ((locals.var_delta_nd_dn7 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn7 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn7) - locals.var_xi0d_dn7))), ((locals.var_delta_nd_dn8 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn8 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn8) - locals.var_xi0d_dn8))),)
    } else {
        (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8,)
    }
};
        locals.var_dd = assign44670_e57819;
        locals.var_dd_dn5 = assign44670_e57819_d_n5;
        locals.var_dd_dn6 = assign44670_e57819_d_n6;
        locals.var_dd_dn7 = assign44670_e57819_d_n7;
        locals.var_dd_dn8 = assign44670_e57819_d_n8;

        let assign44680_e57823: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44680_e57824: f64 = if locals.var_x_d > assign44680_e57823 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign44680_e57824;

        let (assign44690_e57836, assign44690_e57836_d_n5, assign44690_e57836_d_n6, assign44690_e57836_d_n7, assign44690_e57836_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign44690_e57833: f64 = (locals.var_x_d - locals.var_xn_d);
        let assign44690_e57834: f64 = (assign44690_e57833).exp();
        (assign44690_e57834, (assign44690_e57834 * (locals.var_x_d_dn5 - locals.var_xn_d_dn5)), (assign44690_e57834 * (locals.var_x_d_dn6 - locals.var_xn_d_dn6)), (assign44690_e57834 * (locals.var_x_d_dn7 - locals.var_xn_d_dn7)), (assign44690_e57834 * (locals.var_x_d_dn8 - locals.var_xn_d_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44690_e57836;
        locals.var_temp__blk936_dn5 = assign44690_e57836_d_n5;
        locals.var_temp__blk936_dn6 = assign44690_e57836_d_n6;
        locals.var_temp__blk936_dn7 = assign44690_e57836_d_n7;
        locals.var_temp__blk936_dn8 = assign44690_e57836_d_n8;

        let (assign44700_e57847, assign44700_e57847_d_n5, assign44700_e57847_d_n6, assign44700_e57847_d_n7, assign44700_e57847_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign44700_e57845: f64 = (locals.var_delta_nd / locals.var_temp__blk936);
        (assign44700_e57845, (((locals.var_delta_nd_dn5 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn6 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn7 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn8 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)),)
    } else {
        (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8,)
    }
};
        locals.var_ed = assign44700_e57847;
        locals.var_ed_dn5 = assign44700_e57847_d_n5;
        locals.var_ed_dn6 = assign44700_e57847_d_n6;
        locals.var_ed_dn7 = assign44700_e57847_d_n7;
        locals.var_ed_dn8 = assign44700_e57847_d_n8;

        let (assign44710_e57864, assign44710_e57864_d_n5, assign44710_e57864_d_n6, assign44710_e57864_d_n7, assign44710_e57864_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign44710_e57858: f64 = (locals.var_x_d + 1.0);
        let assign44710_e57860: f64 = (assign44710_e57858 + locals.var_xi0d);
        let assign44710_e57861: f64 = (locals.var_delta_nd * assign44710_e57860);
        let assign44710_e57862: f64 = (locals.var_temp__blk936 - assign44710_e57861);
        (assign44710_e57862, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd_dn5 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn5 + locals.var_xi0d_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd_dn6 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd_dn7 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd_dn8 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8,)
    }
};
        locals.var_dd = assign44710_e57864;
        locals.var_dd_dn5 = assign44710_e57864_d_n5;
        locals.var_dd_dn6 = assign44710_e57864_d_n6;
        locals.var_dd_dn7 = assign44710_e57864_d_n7;
        locals.var_dd_dn8 = assign44710_e57864_d_n8;

        let (assign44720_e57896, assign44720_e57896_d_n5, assign44720_e57896_d_n6, assign44720_e57896_d_n7, assign44720_e57896_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
        let assign44720_e57876: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44720_e57881: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44720_e57885: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44720_e57887: f64 = (assign44720_e57885 * 0.3333333333333333);
        let assign44720_e57888: f64 = (1.0 + assign44720_e57887);
        let assign44720_e57889: f64 = (assign44720_e57881 * assign44720_e57888);
        let assign44720_e57890: f64 = (0.5 * assign44720_e57889);
        let assign44720_e57891: f64 = (1.0 + assign44720_e57890);
        let assign44720_e57892: f64 = (assign44720_e57876 * assign44720_e57891);
        let assign44720_e57893: f64 = (1.0 + assign44720_e57892);
        let assign44720_e57894: f64 = (1e-100 / assign44720_e57893);
        (assign44720_e57894, (-((1e-100 * ((locals.var_x_d_dn5 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn5 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn5 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn6 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn6 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn6 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn7 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn7 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn7 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn8 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn8 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn8 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))),)
    } else {
        (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8,)
    }
};
        locals.var_ed = assign44720_e57896;
        locals.var_ed_dn5 = assign44720_e57896_d_n5;
        locals.var_ed_dn6 = assign44720_e57896_d_n6;
        locals.var_ed_dn7 = assign44720_e57896_d_n7;
        locals.var_ed_dn8 = assign44720_e57896_d_n8;

        let (assign44730_e57934, assign44730_e57934_d_n5, assign44730_e57934_d_n6, assign44730_e57934_d_n7, assign44730_e57934_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
        let assign44730_e57908: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44730_e57910: f64 = (assign44730_e57908 - 230.25850929940458);
        let assign44730_e57915: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44730_e57917: f64 = (assign44730_e57915 - 230.25850929940458);
        let assign44730_e57921: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44730_e57923: f64 = (assign44730_e57921 - 230.25850929940458);
        let assign44730_e57925: f64 = (assign44730_e57923 * 0.3333333333333333);
        let assign44730_e57926: f64 = (1.0 + assign44730_e57925);
        let assign44730_e57927: f64 = (assign44730_e57917 * assign44730_e57926);
        let assign44730_e57928: f64 = (0.5 * assign44730_e57927);
        let assign44730_e57929: f64 = (1.0 + assign44730_e57928);
        let assign44730_e57930: f64 = (assign44730_e57910 * assign44730_e57929);
        let assign44730_e57931: f64 = (1.0 + assign44730_e57930);
        let assign44730_e57932: f64 = (1e-100 / assign44730_e57931);
        (assign44730_e57932, (-((1e-100 * (((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44730_e57934;
        locals.var_temp__blk936_dn5 = assign44730_e57934_d_n5;
        locals.var_temp__blk936_dn6 = assign44730_e57934_d_n6;
        locals.var_temp__blk936_dn7 = assign44730_e57934_d_n7;
        locals.var_temp__blk936_dn8 = assign44730_e57934_d_n8;

        let (assign44740_e57952, assign44740_e57952_d_n5, assign44740_e57952_d_n6, assign44740_e57952_d_n7, assign44740_e57952_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
        let assign44740_e57946: f64 = (locals.var_x_d + 1.0);
        let assign44740_e57948: f64 = (assign44740_e57946 + locals.var_xi0d);
        let assign44740_e57949: f64 = (locals.var_delta_nd * assign44740_e57948);
        let assign44740_e57950: f64 = (locals.var_temp__blk936 - assign44740_e57949);
        (assign44740_e57950, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd_dn5 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn5 + locals.var_xi0d_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd_dn6 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd_dn7 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd_dn8 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8,)
    }
};
        locals.var_dd = assign44740_e57952;
        locals.var_dd_dn5 = assign44740_e57952_d_n5;
        locals.var_dd_dn6 = assign44740_e57952_d_n6;
        locals.var_dd_dn7 = assign44740_e57952_d_n7;
        locals.var_dd_dn8 = assign44740_e57952_d_n8;

        let (assign44750_e57963, assign44750_e57963_d_n5, assign44750_e57963_d_n6, assign44750_e57963_d_n7, assign44750_e57963_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) {
        let assign44750_e57959: f64 = (locals.var_x_d - 1.0);
        let assign44750_e57961: f64 = (assign44750_e57959 + locals.var_ed);
        (assign44750_e57961, (locals.var_x_d_dn5 + locals.var_ed_dn5), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8),)
    } else {
        (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8,)
    }
};
        locals.var_pd = assign44750_e57963;
        locals.var_pd_dn5 = assign44750_e57963_d_n5;
        locals.var_pd_dn6 = assign44750_e57963_d_n6;
        locals.var_pd_dn7 = assign44750_e57963_d_n7;
        locals.var_pd_dn8 = assign44750_e57963_d_n8;

        let (assign44760_e57971, assign44760_e57971_d_n5, assign44760_e57971_d_n6, assign44760_e57971_d_n7, assign44760_e57971_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) {
        let assign44760_e57969: f64 = (locals.var_pd).sqrt();
        (assign44760_e57969, (locals.var_pd_dn5 / (2.0 * assign44760_e57969)), (locals.var_pd_dn6 / (2.0 * assign44760_e57969)), (locals.var_pd_dn7 / (2.0 * assign44760_e57969)), (locals.var_pd_dn8 / (2.0 * assign44760_e57969)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8,)
    }
};
        locals.var_sqd = assign44760_e57971;
        locals.var_sqd_dn5 = assign44760_e57971_d_n5;
        locals.var_sqd_dn6 = assign44760_e57971_d_n6;
        locals.var_sqd_dn7 = assign44760_e57971_d_n7;
        locals.var_sqd_dn8 = assign44760_e57971_d_n8;

        let (assign44770_e57979, assign44770_e57979_d_n5, assign44770_e57979_d_n6, assign44770_e57979_d_n7, assign44770_e57979_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44770_e57975: f64 = (locals.var_sqd * locals.var_gf);
        let assign44770_e57977: f64 = (assign44770_e57975 * locals.var_phit1);
        (assign44770_e57977, ((((locals.var_sqd_dn5 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn5)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn5)), ((((locals.var_sqd_dn6 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn6)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn6)), ((((locals.var_sqd_dn7 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn7)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn7)), ((((locals.var_sqd_dn8 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn8)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn8)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8,)
    }
};
        locals.var_qbd = assign44770_e57979;
        locals.var_qbd_dn5 = assign44770_e57979_d_n5;
        locals.var_qbd_dn6 = assign44770_e57979_d_n6;
        locals.var_qbd_dn7 = assign44770_e57979_d_n7;
        locals.var_qbd_dn8 = assign44770_e57979_d_n8;

        let (assign44780_e57987, assign44780_e57987_d_n5, assign44780_e57987_d_n6, assign44780_e57987_d_n7, assign44780_e57987_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44780_e57984: f64 = (locals.var_x_s + locals.var_x_d);
        let assign44780_e57985: f64 = (0.5 * assign44780_e57984);
        (assign44780_e57985, (0.5 * (locals.var_x_s_dn5 + locals.var_x_d_dn5)), (0.5 * (locals.var_x_s_dn6 + locals.var_x_d_dn6)), (0.5 * (locals.var_x_s_dn7 + locals.var_x_d_dn7)), (0.5 * (locals.var_x_s_dn8 + locals.var_x_d_dn8)),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8,)
    }
};
        locals.var_x_m = assign44780_e57987;
        locals.var_x_m_dn5 = assign44780_e57987_d_n5;
        locals.var_x_m_dn6 = assign44780_e57987_d_n6;
        locals.var_x_m_dn7 = assign44780_e57987_d_n7;
        locals.var_x_m_dn8 = assign44780_e57987_d_n8;

        let (assign44790_e57991, assign44790_e57991_d_n5, assign44790_e57991_d_n6, assign44790_e57991_d_n7, assign44790_e57991_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8,)
    }
};
        locals.var_em = assign44790_e57991;
        locals.var_em_dn5 = assign44790_e57991_d_n5;
        locals.var_em_dn6 = assign44790_e57991_d_n6;
        locals.var_em_dn7 = assign44790_e57991_d_n7;
        locals.var_em_dn8 = assign44790_e57991_d_n8;

        let (assign44800_e57997, assign44800_e57997_d_n5, assign44800_e57997_d_n6, assign44800_e57997_d_n7, assign44800_e57997_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44800_e57995: f64 = (locals.var_ed * locals.var_es);
        (assign44800_e57995, ((locals.var_ed_dn5 * locals.var_es) + (locals.var_ed * locals.var_es_dn5)), ((locals.var_ed_dn6 * locals.var_es) + (locals.var_ed * locals.var_es_dn6)), ((locals.var_ed_dn7 * locals.var_es) + (locals.var_ed * locals.var_es_dn7)), ((locals.var_ed_dn8 * locals.var_es) + (locals.var_ed * locals.var_es_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44800_e57997;
        locals.var_temp__blk936_dn5 = assign44800_e57997_d_n5;
        locals.var_temp__blk936_dn6 = assign44800_e57997_d_n6;
        locals.var_temp__blk936_dn7 = assign44800_e57997_d_n7;
        locals.var_temp__blk936_dn8 = assign44800_e57997_d_n8;

        let assign44810_e58000: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign44810_e58000;

        let (assign44820_e58007, assign44820_e58007_d_n5, assign44820_e58007_d_n6, assign44820_e58007_d_n7, assign44820_e58007_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1212 != 0.0)) {
        let assign44820_e58005: f64 = (locals.var_temp__blk936).sqrt();
        (assign44820_e58005, (locals.var_temp__blk936_dn5 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn6 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn7 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn8 / (2.0 * assign44820_e58005)),)
    } else {
        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8,)
    }
};
        locals.var_em = assign44820_e58007;
        locals.var_em_dn5 = assign44820_e58007_d_n5;
        locals.var_em_dn6 = assign44820_e58007_d_n6;
        locals.var_em_dn7 = assign44820_e58007_d_n7;
        locals.var_em_dn8 = assign44820_e58007_d_n8;

        let (assign44830_e58015, assign44830_e58015_d_n5, assign44830_e58015_d_n6, assign44830_e58015_d_n7, assign44830_e58015_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44830_e58012: f64 = (locals.var_ds + locals.var_dd);
        let assign44830_e58013: f64 = (0.5 * assign44830_e58012);
        (assign44830_e58013, (0.5 * (locals.var_ds_dn5 + locals.var_dd_dn5)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)),)
    } else {
        (locals.var_d_bar, locals.var_d_bar_dn5, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8,)
    }
};
        locals.var_d_bar = assign44830_e58015;
        locals.var_d_bar_dn5 = assign44830_e58015_d_n5;
        locals.var_d_bar_dn6 = assign44830_e58015_d_n6;
        locals.var_d_bar_dn7 = assign44830_e58015_d_n7;
        locals.var_d_bar_dn8 = assign44830_e58015_d_n8;

        let (assign44840_e58031, assign44840_e58031_d_n5, assign44840_e58031_d_n6, assign44840_e58031_d_n7, assign44840_e58031_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign44840_e58021: f64 = (locals.var_x_ds * locals.var_x_ds);
        let assign44840_e58025: f64 = (2.0 * locals.var_inv_gf2);
        let assign44840_e58026: f64 = (locals.var_em - assign44840_e58025);
        let assign44840_e58027: f64 = (assign44840_e58021 * assign44840_e58026);
        let assign44840_e58028: f64 = (0.125 * assign44840_e58027);
        let assign44840_e58029: f64 = (locals.var_d_bar + assign44840_e58028);
        (assign44840_e58029, (locals.var_d_bar_dn5 + (0.125 * ((((locals.var_x_ds_dn5 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn5)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn5 - (2.0 * locals.var_inv_gf2_dn5)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gf2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gf2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gf2_dn8)))))),)
    } else {
        (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8,)
    }
};
        locals.var_dm = assign44840_e58031;
        locals.var_dm_dn5 = assign44840_e58031_d_n5;
        locals.var_dm_dn6 = assign44840_e58031_d_n6;
        locals.var_dm_dn7 = assign44840_e58031_d_n7;
        locals.var_dm_dn8 = assign44840_e58031_d_n8;

        let assign44850_e58034: f64 = if locals.var_x_m < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign44850_e58034;

        let (assign44860_e58056, assign44860_e58056_d_n5, assign44860_e58056_d_n6, assign44860_e58056_d_n7, assign44860_e58056_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44860_e58041: f64 = (locals.var_x_m * locals.var_x_m);
        let assign44860_e58048: f64 = (0.25 * locals.var_x_m);
        let assign44860_e58049: f64 = (1.0 - assign44860_e58048);
        let assign44860_e58050: f64 = (locals.var_x_m * assign44860_e58049);
        let assign44860_e58051: f64 = (0.3333333333333333 * assign44860_e58050);
        let assign44860_e58052: f64 = (1.0 - assign44860_e58051);
        let assign44860_e58053: f64 = (assign44860_e58041 * assign44860_e58052);
        let assign44860_e58054: f64 = (0.5 * assign44860_e58053);
        (assign44860_e58054, (0.5 * ((((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5))))))))), (0.5 * ((((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6))))))))), (0.5 * ((((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7))))))))), (0.5 * ((((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8))))))))),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign44860_e58056;
        locals.var_pm_dn5 = assign44860_e58056_d_n5;
        locals.var_pm_dn6 = assign44860_e58056_d_n6;
        locals.var_pm_dn7 = assign44860_e58056_d_n7;
        locals.var_pm_dn8 = assign44860_e58056_d_n8;

        let (assign44870_e58067, assign44870_e58067_d_n5, assign44870_e58067_d_n6, assign44870_e58067_d_n7, assign44870_e58067_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44870_e58063: f64 = (locals.var_dm + locals.var_pm);
        let assign44870_e58064: f64 = (assign44870_e58063).sqrt();
        let assign44870_e58065: f64 = (locals.var_gf * assign44870_e58064);
        (assign44870_e58065, ((locals.var_gf_dn5 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn6 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn7 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn8 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44870_e58064)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign44870_e58067;
        locals.var_xgm_dn5 = assign44870_e58067_d_n5;
        locals.var_xgm_dn6 = assign44870_e58067_d_n6;
        locals.var_xgm_dn7 = assign44870_e58067_d_n7;
        locals.var_xgm_dn8 = assign44870_e58067_d_n8;

        let assign44880_e58070: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign44880_e58070;

        let (assign44890_e58085, assign44890_e58085_d_n5, assign44890_e58085_d_n6, assign44890_e58085_d_n7, assign44890_e58085_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
        let assign44890_e58080: f64 = (locals.var_kp * locals.var_xgm);
        let assign44890_e58081: f64 = (1.0 + assign44890_e58080);
        let assign44890_e58082: f64 = (assign44890_e58081).sqrt();
        let assign44890_e58083: f64 = (1.0 / assign44890_e58082);
        (assign44890_e58083, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8,)
    }
};
        locals.var_eta_p = assign44890_e58085;
        locals.var_eta_p_dn5 = assign44890_e58085_d_n5;
        locals.var_eta_p_dn6 = assign44890_e58085_d_n6;
        locals.var_eta_p_dn7 = assign44890_e58085_d_n7;
        locals.var_eta_p_dn8 = assign44890_e58085_d_n8;

        let (assign44900_e58102, assign44900_e58102_d_n5, assign44900_e58102_d_n6, assign44900_e58102_d_n7, assign44900_e58102_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44900_e58095: f64 = (0.25 * locals.var_x_m);
        let assign44900_e58096: f64 = (1.0 - assign44900_e58095);
        let assign44900_e58097: f64 = (locals.var_x_m * assign44900_e58096);
        let assign44900_e58098: f64 = (0.3333333333333333 * assign44900_e58097);
        let assign44900_e58099: f64 = (1.0 - assign44900_e58098);
        let assign44900_e58100: f64 = (assign44900_e58099).sqrt();
        (assign44900_e58100, ((-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8)))))) / (2.0 * assign44900_e58100)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44900_e58102;
        locals.var_temp__blk936_dn5 = assign44900_e58102_d_n5;
        locals.var_temp__blk936_dn6 = assign44900_e58102_d_n6;
        locals.var_temp__blk936_dn7 = assign44900_e58102_d_n7;
        locals.var_temp__blk936_dn8 = assign44900_e58102_d_n8;

        let (assign44910_e58112, assign44910_e58112_d_n5, assign44910_e58112_d_n6, assign44910_e58112_d_n7, assign44910_e58112_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44910_e58109: f64 = (locals.var_x_m * locals.var_temp__blk936);
        let assign44910_e58110: f64 = (0.7071067811865475 * assign44910_e58109);
        (assign44910_e58110, (0.7071067811865475 * ((locals.var_x_m_dn5 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m_dn6 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m_dn7 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m_dn8 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8,)
    }
};
        locals.var_sqm = assign44910_e58112;
        locals.var_sqm_dn5 = assign44910_e58112_d_n5;
        locals.var_sqm_dn6 = assign44910_e58112_d_n6;
        locals.var_sqm_dn7 = assign44910_e58112_d_n7;
        locals.var_sqm_dn8 = assign44910_e58112_d_n8;

        let (assign44920_e58136, assign44920_e58136_d_n5, assign44920_e58136_d_n6, assign44920_e58136_d_n7, assign44920_e58136_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
        let assign44920_e58122: f64 = (0.5 * locals.var_x_m);
        let assign44920_e58123: f64 = (1.0 - assign44920_e58122);
        let assign44920_e58127: f64 = (locals.var_x_m * locals.var_x_m);
        let assign44920_e58128: f64 = (0.16666666666666666 * assign44920_e58127);
        let assign44920_e58129: f64 = (assign44920_e58123 + assign44920_e58128);
        let assign44920_e58130: f64 = (locals.var_gf * assign44920_e58129);
        let assign44920_e58132: f64 = (assign44920_e58130 / locals.var_temp__blk936);
        let assign44920_e58133: f64 = (0.7071067811865475 * assign44920_e58132);
        let assign44920_e58134: f64 = (locals.var_eta_p + assign44920_e58133);
        (assign44920_e58134, (locals.var_eta_p_dn5 + (0.7071067811865475 * (((((locals.var_gf_dn5 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn5)) + (0.16666666666666666 * ((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn6 + (0.7071067811865475 * (((((locals.var_gf_dn6 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn6)) + (0.16666666666666666 * ((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn7 + (0.7071067811865475 * (((((locals.var_gf_dn7 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn7)) + (0.16666666666666666 * ((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn8 + (0.7071067811865475 * (((((locals.var_gf_dn8 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn8)) + (0.16666666666666666 * ((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8,)
    }
};
        locals.var_alpha = assign44920_e58136;
        locals.var_alpha_dn5 = assign44920_e58136_d_n5;
        locals.var_alpha_dn6 = assign44920_e58136_d_n6;
        locals.var_alpha_dn7 = assign44920_e58136_d_n7;
        locals.var_alpha_dn8 = assign44920_e58136_d_n8;

        let (assign44930_e58147, assign44930_e58147_d_n5, assign44930_e58147_d_n6, assign44930_e58147_d_n7, assign44930_e58147_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign44930_e58143: f64 = (locals.var_x_m - 1.0);
        let assign44930_e58145: f64 = (assign44930_e58143 + locals.var_em);
        (assign44930_e58145, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign44930_e58147;
        locals.var_pm_dn5 = assign44930_e58147_d_n5;
        locals.var_pm_dn6 = assign44930_e58147_d_n6;
        locals.var_pm_dn7 = assign44930_e58147_d_n7;
        locals.var_pm_dn8 = assign44930_e58147_d_n8;

        let (assign44940_e58159, assign44940_e58159_d_n5, assign44940_e58159_d_n6, assign44940_e58159_d_n7, assign44940_e58159_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign44940_e58155: f64 = (locals.var_dm + locals.var_pm);
        let assign44940_e58156: f64 = (assign44940_e58155).sqrt();
        let assign44940_e58157: f64 = (locals.var_gf * assign44940_e58156);
        (assign44940_e58157, ((locals.var_gf_dn5 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn6 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn7 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn8 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44940_e58156)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign44940_e58159;
        locals.var_xgm_dn5 = assign44940_e58159_d_n5;
        locals.var_xgm_dn6 = assign44940_e58159_d_n6;
        locals.var_xgm_dn7 = assign44940_e58159_d_n7;
        locals.var_xgm_dn8 = assign44940_e58159_d_n8;

        let assign44950_e58162: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign44950_e58162;

        let (assign44960_e58179, assign44960_e58179_d_n5, assign44960_e58179_d_n6, assign44960_e58179_d_n7, assign44960_e58179_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44960_e58171: f64 = (1.0 - locals.var_em);
        let assign44960_e58175: f64 = (locals.var_xgm * locals.var_inv_gf2);
        let assign44960_e58176: f64 = (2.0 * assign44960_e58175);
        let assign44960_e58177: f64 = (assign44960_e58171 + assign44960_e58176);
        (assign44960_e58177, ((-locals.var_em_dn5) + (2.0 * ((locals.var_xgm_dn5 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn8)))),)
    } else {
        (locals.var_d0, locals.var_d0_dn5, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8,)
    }
};
        locals.var_d0 = assign44960_e58179;
        locals.var_d0_dn5 = assign44960_e58179_d_n5;
        locals.var_d0_dn6 = assign44960_e58179_d_n6;
        locals.var_d0_dn7 = assign44960_e58179_d_n7;
        locals.var_d0_dn8 = assign44960_e58179_d_n8;

        let (assign44970_e58195, assign44970_e58195_d_n5, assign44970_e58195_d_n6, assign44970_e58195_d_n7, assign44970_e58195_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44970_e58190: f64 = (locals.var_kp * locals.var_xgm);
        let assign44970_e58191: f64 = (1.0 + assign44970_e58190);
        let assign44970_e58192: f64 = (assign44970_e58191).sqrt();
        let assign44970_e58193: f64 = (1.0 / assign44970_e58192);
        (assign44970_e58193, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8,)
    }
};
        locals.var_eta_p = assign44970_e58195;
        locals.var_eta_p_dn5 = assign44970_e58195_d_n5;
        locals.var_eta_p_dn6 = assign44970_e58195_d_n6;
        locals.var_eta_p_dn7 = assign44970_e58195_d_n7;
        locals.var_eta_p_dn8 = assign44970_e58195_d_n8;

        let (assign44980_e58208, assign44980_e58208_d_n5, assign44980_e58208_d_n6, assign44980_e58208_d_n7, assign44980_e58208_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44980_e58205: f64 = (locals.var_eta_p + 1.0);
        let assign44980_e58206: f64 = (locals.var_eta_p / assign44980_e58205);
        (assign44980_e58206, (((locals.var_eta_p_dn5 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn5)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn6 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn7 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn8 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign44980_e58205 * assign44980_e58205)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign44980_e58208;
        locals.var_temp__blk936_dn5 = assign44980_e58208_d_n5;
        locals.var_temp__blk936_dn6 = assign44980_e58208_d_n6;
        locals.var_temp__blk936_dn7 = assign44980_e58208_d_n7;
        locals.var_temp__blk936_dn8 = assign44980_e58208_d_n8;

        let (assign44990_e58225, assign44990_e58225_d_n5, assign44990_e58225_d_n6, assign44990_e58225_d_n7, assign44990_e58225_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign44990_e58218: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign44990_e58220: f64 = (assign44990_e58218 * locals.var_gf2);
        let assign44990_e58222: f64 = (assign44990_e58220 * locals.var_dm);
        let assign44990_e58223: f64 = (locals.var_kp * assign44990_e58222);
        (assign44990_e58223, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn5)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn6)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn7)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn8)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn8))),)
    } else {
        (locals.var_x_pm, locals.var_x_pm_dn5, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8,)
    }
};
        locals.var_x_pm = assign44990_e58225;
        locals.var_x_pm_dn5 = assign44990_e58225_d_n5;
        locals.var_x_pm_dn6 = assign44990_e58225_d_n6;
        locals.var_x_pm_dn7 = assign44990_e58225_d_n7;
        locals.var_x_pm_dn8 = assign44990_e58225_d_n8;

        let (assign45000_e58246, assign45000_e58246_d_n5, assign45000_e58246_d_n6, assign45000_e58246_d_n7, assign45000_e58246_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45000_e58235: f64 = (locals.var_xgm - locals.var_x_pm);
        let assign45000_e58236: f64 = (2.0 * assign45000_e58235);
        let assign45000_e58240: f64 = (1.0 - locals.var_em);
        let assign45000_e58242: f64 = (assign45000_e58240 + locals.var_dm);
        let assign45000_e58243: f64 = (locals.var_gf2 * assign45000_e58242);
        let assign45000_e58244: f64 = (assign45000_e58236 + assign45000_e58243);
        (assign45000_e58244, ((2.0 * (locals.var_xgm_dn5 - locals.var_x_pm_dn5)) + ((locals.var_gf2_dn5 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn5) + locals.var_dm_dn5)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gf2_dn6 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gf2_dn7 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gf2_dn8 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))),)
    } else {
        (locals.var_p_pd, locals.var_p_pd_dn5, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8,)
    }
};
        locals.var_p_pd = assign45000_e58246;
        locals.var_p_pd_dn5 = assign45000_e58246_d_n5;
        locals.var_p_pd_dn6 = assign45000_e58246_d_n6;
        locals.var_p_pd_dn7 = assign45000_e58246_d_n7;
        locals.var_p_pd_dn8 = assign45000_e58246_d_n8;

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign45010_e58261, assign45010_e58261_d_n5, assign45010_e58261_d_n6, assign45010_e58261_d_n7, assign45010_e58261_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45010_e58257: f64 = (2.0 * locals.var_xgm);
        let assign45010_e58258: f64 = (locals.var_x_pm - assign45010_e58257);
        let assign45010_e58259: f64 = (locals.var_x_pm * assign45010_e58258);
        (assign45010_e58259, ((locals.var_x_pm_dn5 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn5 - (2.0 * locals.var_xgm_dn5)))), ((locals.var_x_pm_dn6 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))),)
    } else {
        (locals.var_q_pd, locals.var_q_pd_dn5, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8,)
    }
};
        locals.var_q_pd = assign45010_e58261;
        locals.var_q_pd_dn5 = assign45010_e58261_d_n5;
        locals.var_q_pd_dn6 = assign45010_e58261_d_n6;
        locals.var_q_pd_dn7 = assign45010_e58261_d_n7;
        locals.var_q_pd_dn8 = assign45010_e58261_d_n8;

        let (assign45020_e58278, assign45020_e58278_d_n5, assign45020_e58278_d_n6, assign45020_e58278_d_n7, assign45020_e58278_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45020_e58273: f64 = (locals.var_em + locals.var_dm);
        let assign45020_e58274: f64 = (locals.var_gf2 * assign45020_e58273);
        let assign45020_e58275: f64 = (0.5 * assign45020_e58274);
        let assign45020_e58276: f64 = (1.0 - assign45020_e58275);
        (assign45020_e58276, (-(0.5 * ((locals.var_gf2_dn5 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn5 + locals.var_dm_dn5))))), (-(0.5 * ((locals.var_gf2_dn6 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gf2_dn7 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gf2_dn8 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn8 + locals.var_dm_dn8))))),)
    } else {
        (locals.var_xi_pd, locals.var_xi_pd_dn5, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8,)
    }
};
        locals.var_xi_pd = assign45020_e58278;
        locals.var_xi_pd_dn5 = assign45020_e58278_d_n5;
        locals.var_xi_pd_dn6 = assign45020_e58278_d_n6;
        locals.var_xi_pd_dn7 = assign45020_e58278_d_n7;
        locals.var_xi_pd_dn8 = assign45020_e58278_d_n8;

        let (assign45030_e58297, assign45030_e58297_d_n5, assign45030_e58297_d_n6, assign45030_e58297_d_n7, assign45030_e58297_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45030_e58287: f64 = (locals.var_q_pd * locals.var_p_pd);
        let assign45030_e58290: f64 = (locals.var_p_pd * locals.var_p_pd);
        let assign45030_e58293: f64 = (locals.var_xi_pd * locals.var_q_pd);
        let assign45030_e58294: f64 = (assign45030_e58290 - assign45030_e58293);
        let assign45030_e58295: f64 = (assign45030_e58287 / assign45030_e58294);
        (assign45030_e58295, (((((locals.var_q_pd_dn5 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn5)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn5 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn5)) - ((locals.var_xi_pd_dn5 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn5))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign45030_e58294 * assign45030_e58294)),)
    } else {
        (locals.var_u_pd, locals.var_u_pd_dn5, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8,)
    }
};
        locals.var_u_pd = assign45030_e58297;
        locals.var_u_pd_dn5 = assign45030_e58297_d_n5;
        locals.var_u_pd_dn6 = assign45030_e58297_d_n6;
        locals.var_u_pd_dn7 = assign45030_e58297_d_n7;
        locals.var_u_pd_dn8 = assign45030_e58297_d_n8;

        let (assign45040_e58308, assign45040_e58308_d_n5, assign45040_e58308_d_n6, assign45040_e58308_d_n7, assign45040_e58308_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45040_e58306: f64 = (locals.var_x_m + locals.var_u_pd);
        (assign45040_e58306, (locals.var_x_m_dn5 + locals.var_u_pd_dn5), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8,)
    }
};
        locals.var_x_m = assign45040_e58308;
        locals.var_x_m_dn5 = assign45040_e58308_d_n5;
        locals.var_x_m_dn6 = assign45040_e58308_d_n6;
        locals.var_x_m_dn7 = assign45040_e58308_d_n7;
        locals.var_x_m_dn8 = assign45040_e58308_d_n8;

        let (assign45050_e58318, assign45050_e58318_d_n5, assign45050_e58318_d_n6, assign45050_e58318_d_n7, assign45050_e58318_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45050_e58316: f64 = (locals.var_u_pd).exp();
        (assign45050_e58316, (assign45050_e58316 * locals.var_u_pd_dn5), (assign45050_e58316 * locals.var_u_pd_dn6), (assign45050_e58316 * locals.var_u_pd_dn7), (assign45050_e58316 * locals.var_u_pd_dn8),)
    } else {
        (locals.var_km, locals.var_km_dn5, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8,)
    }
};
        locals.var_km = assign45050_e58318;
        locals.var_km_dn5 = assign45050_e58318_d_n5;
        locals.var_km_dn6 = assign45050_e58318_d_n6;
        locals.var_km_dn7 = assign45050_e58318_d_n7;
        locals.var_km_dn8 = assign45050_e58318_d_n8;

        let (assign45060_e58329, assign45060_e58329_d_n5, assign45060_e58329_d_n6, assign45060_e58329_d_n7, assign45060_e58329_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45060_e58327: f64 = (locals.var_em / locals.var_km);
        (assign45060_e58327, (((locals.var_em_dn5 * locals.var_km) - (locals.var_em * locals.var_km_dn5)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)),)
    } else {
        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8,)
    }
};
        locals.var_em = assign45060_e58329;
        locals.var_em_dn5 = assign45060_e58329_d_n5;
        locals.var_em_dn6 = assign45060_e58329_d_n6;
        locals.var_em_dn7 = assign45060_e58329_d_n7;
        locals.var_em_dn8 = assign45060_e58329_d_n8;

        let (assign45070_e58340, assign45070_e58340_d_n5, assign45070_e58340_d_n6, assign45070_e58340_d_n7, assign45070_e58340_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45070_e58338: f64 = (locals.var_dm * locals.var_km);
        (assign45070_e58338, ((locals.var_dm_dn5 * locals.var_km) + (locals.var_dm * locals.var_km_dn5)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)),)
    } else {
        (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8,)
    }
};
        locals.var_dm = assign45070_e58340;
        locals.var_dm_dn5 = assign45070_e58340_d_n5;
        locals.var_dm_dn6 = assign45070_e58340_d_n6;
        locals.var_dm_dn7 = assign45070_e58340_d_n7;
        locals.var_dm_dn8 = assign45070_e58340_d_n8;

        let (assign45080_e58353, assign45080_e58353_d_n5, assign45080_e58353_d_n6, assign45080_e58353_d_n7, assign45080_e58353_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45080_e58349: f64 = (locals.var_x_m - 1.0);
        let assign45080_e58351: f64 = (assign45080_e58349 + locals.var_em);
        (assign45080_e58351, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8),)
    } else {
        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8,)
    }
};
        locals.var_pm = assign45080_e58353;
        locals.var_pm_dn5 = assign45080_e58353_d_n5;
        locals.var_pm_dn6 = assign45080_e58353_d_n6;
        locals.var_pm_dn7 = assign45080_e58353_d_n7;
        locals.var_pm_dn8 = assign45080_e58353_d_n8;

        let (assign45090_e58367, assign45090_e58367_d_n5, assign45090_e58367_d_n6, assign45090_e58367_d_n7, assign45090_e58367_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45090_e58363: f64 = (locals.var_dm + locals.var_pm);
        let assign45090_e58364: f64 = (assign45090_e58363).sqrt();
        let assign45090_e58365: f64 = (locals.var_gf * assign45090_e58364);
        (assign45090_e58365, ((locals.var_gf_dn5 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn6 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn7 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn8 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45090_e58364)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8,)
    }
};
        locals.var_xgm = assign45090_e58367;
        locals.var_xgm_dn5 = assign45090_e58367_d_n5;
        locals.var_xgm_dn6 = assign45090_e58367_d_n6;
        locals.var_xgm_dn7 = assign45090_e58367_d_n7;
        locals.var_xgm_dn8 = assign45090_e58367_d_n8;

        let (assign45100_e58386, assign45100_e58386_d_n5, assign45100_e58386_d_n6, assign45100_e58386_d_n7, assign45100_e58386_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45100_e58376: f64 = (1.0 - locals.var_em);
        let assign45100_e58380: f64 = (locals.var_xgm * locals.var_eta_p);
        let assign45100_e58382: f64 = (assign45100_e58380 * locals.var_inv_gf2);
        let assign45100_e58383: f64 = (2.0 * assign45100_e58382);
        let assign45100_e58384: f64 = (assign45100_e58376 + assign45100_e58383);
        (assign45100_e58384, ((-locals.var_em_dn5) + (2.0 * ((((locals.var_xgm_dn5 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn5)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn8)))),)
    } else {
        (locals.var_km0, locals.var_km0_dn5, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8,)
    }
};
        locals.var_km0 = assign45100_e58386;
        locals.var_km0_dn5 = assign45100_e58386_d_n5;
        locals.var_km0_dn6 = assign45100_e58386_d_n6;
        locals.var_km0_dn7 = assign45100_e58386_d_n7;
        locals.var_km0_dn8 = assign45100_e58386_d_n8;

        let (assign45110_e58407, assign45110_e58407_d_n5, assign45110_e58407_d_n6, assign45110_e58407_d_n7, assign45110_e58407_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45110_e58395: f64 = (locals.var_x_ds * locals.var_km);
        let assign45110_e58398: f64 = (locals.var_d0 + locals.var_d_bar);
        let assign45110_e58399: f64 = (assign45110_e58395 * assign45110_e58398);
        let assign45110_e58403: f64 = (locals.var_km * locals.var_d_bar);
        let assign45110_e58404: f64 = (locals.var_km0 + assign45110_e58403);
        let assign45110_e58405: f64 = (assign45110_e58399 / assign45110_e58404);
        (assign45110_e58405, (((((((locals.var_x_ds_dn5 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn5)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn5 + locals.var_d_bar_dn5))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn5 + ((locals.var_km_dn5 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn5))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign45110_e58404 * assign45110_e58404)),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8,)
    }
};
        locals.var_x_ds = assign45110_e58407;
        locals.var_x_ds_dn5 = assign45110_e58407_d_n5;
        locals.var_x_ds_dn6 = assign45110_e58407_d_n6;
        locals.var_x_ds_dn7 = assign45110_e58407_d_n7;
        locals.var_x_ds_dn8 = assign45110_e58407_d_n8;

        let (assign45120_e58418, assign45120_e58418_d_n5, assign45120_e58418_d_n6, assign45120_e58418_d_n7, assign45120_e58418_d_n8,) = {
    if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign45120_e58416: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign45120_e58416, ((locals.var_x_ds_dn5 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn5)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)),)
    } else {
        (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8,)
    }
};
        locals.var_dps = assign45120_e58418;
        locals.var_dps_dn5 = assign45120_e58418_d_n5;
        locals.var_dps_dn6 = assign45120_e58418_d_n6;
        locals.var_dps_dn7 = assign45120_e58418_d_n7;
        locals.var_dps_dn8 = assign45120_e58418_d_n8;

        let (assign45130_e58426, assign45130_e58426_d_n5, assign45130_e58426_d_n6, assign45130_e58426_d_n7, assign45130_e58426_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign45130_e58424: f64 = (locals.var_pm).sqrt();
        (assign45130_e58424, (locals.var_pm_dn5 / (2.0 * assign45130_e58424)), (locals.var_pm_dn6 / (2.0 * assign45130_e58424)), (locals.var_pm_dn7 / (2.0 * assign45130_e58424)), (locals.var_pm_dn8 / (2.0 * assign45130_e58424)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8,)
    }
};
        locals.var_sqm = assign45130_e58426;
        locals.var_sqm_dn5 = assign45130_e58426_d_n5;
        locals.var_sqm_dn6 = assign45130_e58426_d_n6;
        locals.var_sqm_dn7 = assign45130_e58426_d_n7;
        locals.var_sqm_dn8 = assign45130_e58426_d_n8;

        let (assign45140_e58443, assign45140_e58443_d_n5, assign45140_e58443_d_n6, assign45140_e58443_d_n7, assign45140_e58443_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
        let assign45140_e58436: f64 = (1.0 - locals.var_em);
        let assign45140_e58437: f64 = (locals.var_gf * assign45140_e58436);
        let assign45140_e58439: f64 = (assign45140_e58437 / locals.var_sqm);
        let assign45140_e58440: f64 = (0.5 * assign45140_e58439);
        let assign45140_e58441: f64 = (locals.var_eta_p + assign45140_e58440);
        (assign45140_e58441, (locals.var_eta_p_dn5 + (0.5 * (((((locals.var_gf_dn5 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn5))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn5)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn6 + (0.5 * (((((locals.var_gf_dn6 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn6))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn6)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn7 + (0.5 * (((((locals.var_gf_dn7 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn7))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn7)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn8 + (0.5 * (((((locals.var_gf_dn8 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn8))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn8)) / (locals.var_sqm * locals.var_sqm)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8,)
    }
};
        locals.var_alpha = assign45140_e58443;
        locals.var_alpha_dn5 = assign45140_e58443_d_n5;
        locals.var_alpha_dn6 = assign45140_e58443_d_n6;
        locals.var_alpha_dn7 = assign45140_e58443_d_n7;
        locals.var_alpha_dn8 = assign45140_e58443_d_n8;

        let (assign45150_e58457, assign45150_e58457_d_n5, assign45150_e58457_d_n6, assign45150_e58457_d_n7, assign45150_e58457_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45150_e58448: f64 = (locals.var_gf2 * locals.var_dm);
        let assign45150_e58452: f64 = (locals.var_gf * locals.var_sqm);
        let assign45150_e58453: f64 = (locals.var_xgm + assign45150_e58452);
        let assign45150_e58454: f64 = (assign45150_e58448 / assign45150_e58453);
        let assign45150_e58455: f64 = (locals.var_phit1 * assign45150_e58454);
        (assign45150_e58455, ((locals.var_phit1_dn5 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn5 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn5)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn5 + ((locals.var_gf_dn5 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn5))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn6 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn6 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn6)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn6 + ((locals.var_gf_dn6 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn6))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn7 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn7 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn7)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn7 + ((locals.var_gf_dn7 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn7))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn8 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn8 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn8)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn8 + ((locals.var_gf_dn8 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn8))))) / (assign45150_e58453 * assign45150_e58453)))),)
    } else {
        (locals.var_qim, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8,)
    }
};
        locals.var_qim = assign45150_e58457;
        locals.var_qim_dn5 = assign45150_e58457_d_n5;
        locals.var_qim_dn6 = assign45150_e58457_d_n6;
        locals.var_qim_dn7 = assign45150_e58457_d_n7;
        locals.var_qim_dn8 = assign45150_e58457_d_n8;

        let (assign45160_e58465, assign45160_e58465_d_n5, assign45160_e58465_d_n6, assign45160_e58465_d_n7, assign45160_e58465_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45160_e58462: f64 = (locals.var_phit1 * locals.var_alpha);
        let assign45160_e58463: f64 = (locals.var_qim + assign45160_e58462);
        (assign45160_e58463, (locals.var_qim_dn5 + ((locals.var_phit1_dn5 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn5))), (locals.var_qim_dn6 + ((locals.var_phit1_dn6 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn6))), (locals.var_qim_dn7 + ((locals.var_phit1_dn7 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn7))), (locals.var_qim_dn8 + ((locals.var_phit1_dn8 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn8))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8,)
    }
};
        locals.var_qim1 = assign45160_e58465;
        locals.var_qim1_dn5 = assign45160_e58465_d_n5;
        locals.var_qim1_dn6 = assign45160_e58465_d_n6;
        locals.var_qim1_dn7 = assign45160_e58465_d_n7;
        locals.var_qim1_dn8 = assign45160_e58465_d_n8;

        let (assign45170_e58473, assign45170_e58473_d_n5, assign45170_e58473_d_n6, assign45170_e58473_d_n7, assign45170_e58473_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45170_e58469: f64 = (locals.var_sqm * locals.var_gf);
        let assign45170_e58471: f64 = (assign45170_e58469 * locals.var_phit1);
        (assign45170_e58471, ((((locals.var_sqm_dn5 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn5)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn5)), ((((locals.var_sqm_dn6 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn6)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn6)), ((((locals.var_sqm_dn7 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn7)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn7)), ((((locals.var_sqm_dn8 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn8)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn8)),)
    } else {
        (locals.var_qbm, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8,)
    }
};
        locals.var_qbm = assign45170_e58473;
        locals.var_qbm_dn5 = assign45170_e58473_d_n5;
        locals.var_qbm_dn6 = assign45170_e58473_d_n6;
        locals.var_qbm_dn7 = assign45170_e58473_d_n7;
        locals.var_qbm_dn8 = assign45170_e58473_d_n8;

        let assign45180_e58476: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign45180_e58476;

        let (assign45190_e58486, assign45190_e58486_d_n5, assign45190_e58486_d_n6, assign45190_e58486_d_n7, assign45190_e58486_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 != 0.0)) {
        let assign45190_e58483: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45190_e58484: f64 = (1.0 - assign45190_e58483);
        (assign45190_e58484, (-(locals.var_rsg_i * locals.var_qim_dn5)), (-(locals.var_rsg_i * locals.var_qim_dn6)), (-(locals.var_rsg_i * locals.var_qim_dn7)), (-(locals.var_rsg_i * locals.var_qim_dn8)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign45190_e58486;
        locals.var_rhog_dn5 = assign45190_e58486_d_n5;
        locals.var_rhog_dn6 = assign45190_e58486_d_n6;
        locals.var_rhog_dn7 = assign45190_e58486_d_n7;
        locals.var_rhog_dn8 = assign45190_e58486_d_n8;

        let (assign45200_e58499, assign45200_e58499_d_n5, assign45200_e58499_d_n6, assign45200_e58499_d_n7, assign45200_e58499_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 == 0.0)) {
        let assign45200_e58495: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45200_e58496: f64 = (1.0 + assign45200_e58495);
        let assign45200_e58497: f64 = (1.0 / assign45200_e58496);
        (assign45200_e58497, (-((locals.var_rsg_i * locals.var_qim_dn5) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn6) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn7) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn8) / (assign45200_e58496 * assign45200_e58496))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8,)
    }
};
        locals.var_rhog = assign45200_e58499;
        locals.var_rhog_dn5 = assign45200_e58499_d_n5;
        locals.var_rhog_dn6 = assign45200_e58499_d_n6;
        locals.var_rhog_dn7 = assign45200_e58499_d_n7;
        locals.var_rhog_dn8 = assign45200_e58499_d_n8;

        let (assign45210_e58509, assign45210_e58509_d_n5, assign45210_e58509_d_n6, assign45210_e58509_d_n7, assign45210_e58509_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45210_e58503: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign45210_e58505: f64 = (assign45210_e58503 * locals.var_rhog);
        let assign45210_e58507: f64 = (assign45210_e58505 * locals.var_qim);
        (assign45210_e58507, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn5)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn6)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn7)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn8)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn8)),)
    } else {
        (locals.var_gr, locals.var_gr_dn5, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8,)
    }
};
        locals.var_gr = assign45210_e58509;
        locals.var_gr_dn5 = assign45210_e58509_d_n5;
        locals.var_gr_dn6 = assign45210_e58509_d_n6;
        locals.var_gr_dn7 = assign45210_e58509_d_n7;
        locals.var_gr_dn8 = assign45210_e58509_d_n8;

        let (assign45220_e58517, assign45220_e58517_d_n5, assign45220_e58517_d_n6, assign45220_e58517_d_n7, assign45220_e58517_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45220_e58514: f64 = (locals.var_eta_mu * locals.var_qim);
        let assign45220_e58515: f64 = (locals.var_qbm + assign45220_e58514);
        (assign45220_e58515, (locals.var_qbm_dn5 + (locals.var_eta_mu * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8)),)
    } else {
        (locals.var_qeff, locals.var_qeff_dn5, locals.var_qeff_dn6, locals.var_qeff_dn7, locals.var_qeff_dn8,)
    }
};
        locals.var_qeff = assign45220_e58517;
        locals.var_qeff_dn5 = assign45220_e58517_d_n5;
        locals.var_qeff_dn6 = assign45220_e58517_d_n6;
        locals.var_qeff_dn7 = assign45220_e58517_d_n7;
        locals.var_qeff_dn8 = assign45220_e58517_d_n8;

        let (assign45230_e58525, assign45230_e58525_d_n5, assign45230_e58525_d_n6, assign45230_e58525_d_n7, assign45230_e58525_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45230_e58522: f64 = (locals.var_eta_mu1 * locals.var_qim);
        let assign45230_e58523: f64 = (locals.var_qbm + assign45230_e58522);
        (assign45230_e58523, (locals.var_qbm_dn5 + (locals.var_eta_mu1 * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu1 * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu1 * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu1 * locals.var_qim_dn8)),)
    } else {
        (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8,)
    }
};
        locals.var_qeff1 = assign45230_e58525;
        locals.var_qeff1_dn5 = assign45230_e58525_d_n5;
        locals.var_qeff1_dn6 = assign45230_e58525_d_n6;
        locals.var_qeff1_dn7 = assign45230_e58525_d_n7;
        locals.var_qeff1_dn8 = assign45230_e58525_d_n8;

        let (assign45240_e58531, assign45240_e58531_d_n5, assign45240_e58531_d_n6, assign45240_e58531_d_n7, assign45240_e58531_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45240_e58529: f64 = (locals.var_e_eff0 * locals.var_qeff);
        (assign45240_e58529, (locals.var_e_eff0 * locals.var_qeff_dn5), (locals.var_e_eff0 * locals.var_qeff_dn6), (locals.var_e_eff0 * locals.var_qeff_dn7), (locals.var_e_eff0 * locals.var_qeff_dn8),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8,)
    }
};
        locals.var_eeffm = assign45240_e58531;
        locals.var_eeffm_dn5 = assign45240_e58531_d_n5;
        locals.var_eeffm_dn6 = assign45240_e58531_d_n6;
        locals.var_eeffm_dn7 = assign45240_e58531_d_n7;
        locals.var_eeffm_dn8 = assign45240_e58531_d_n8;

        let (assign45250_e58542, assign45250_e58542_d_n5, assign45250_e58542_d_n6, assign45250_e58542_d_n7, assign45250_e58542_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45250_e58536: f64 = (locals.var_pm + locals.var_dm);
        let assign45250_e58538: f64 = (assign45250_e58536 + 1e-14);
        let assign45250_e58539: f64 = (locals.var_pm / assign45250_e58538);
        let assign45250_e58540: f64 = (assign45250_e58539).ln();
        (assign45250_e58540, ((((locals.var_pm_dn5 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn5 + locals.var_dm_dn5))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn6 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn6 + locals.var_dm_dn6))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn7 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn7 + locals.var_dm_dn7))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn8 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn8 + locals.var_dm_dn8))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign45250_e58542;
        locals.var_temp1_dn5 = assign45250_e58542_d_n5;
        locals.var_temp1_dn6 = assign45250_e58542_d_n6;
        locals.var_temp1_dn7 = assign45250_e58542_d_n7;
        locals.var_temp1_dn8 = assign45250_e58542_d_n8;

        let (assign45260_e58559, assign45260_e58559_d_n5, assign45260_e58559_d_n6, assign45260_e58559_d_n7, assign45260_e58559_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45260_e58546: f64 = (locals.var_eeffm * locals.var_mue_t);
        let assign45260_e58548: f64 = (assign45260_e58546).powf(locals.var_themu_t);
        let assign45260_e58552: f64 = (0.5 * locals.var_thecs_t);
        let assign45260_e58554: f64 = (assign45260_e58552 * locals.var_temp1);
        let assign45260_e58555: f64 = (assign45260_e58554).exp();
        let assign45260_e58556: f64 = (locals.var_cs_t * assign45260_e58555);
        let assign45260_e58557: f64 = (assign45260_e58548 + assign45260_e58556);
        (assign45260_e58557, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn5 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn5 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn6 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn6 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn7 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn7 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn8 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn8 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn5, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8,)
    }
};
        locals.var_mutmp = assign45260_e58559;
        locals.var_mutmp_dn5 = assign45260_e58559_d_n5;
        locals.var_mutmp_dn6 = assign45260_e58559_d_n6;
        locals.var_mutmp_dn7 = assign45260_e58559_d_n7;
        locals.var_mutmp_dn8 = assign45260_e58559_d_n8;

        let (assign45270_e58569, assign45270_e58569_d_n5, assign45270_e58569_d_n6, assign45270_e58569_d_n7, assign45270_e58569_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45270_e58563: f64 = (1.0 + locals.var_mutmp);
        let assign45270_e58565: f64 = (assign45270_e58563 + locals.var_gr);
        let assign45270_e58567: f64 = (assign45270_e58565 * locals.var_rxcor);
        (assign45270_e58567, (((locals.var_mutmp_dn5 + locals.var_gr_dn5) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn5)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn8)),)
    } else {
        (locals.var_gmob, locals.var_gmob_dn5, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8,)
    }
};
        locals.var_gmob = assign45270_e58569;
        locals.var_gmob_dn5 = assign45270_e58569_d_n5;
        locals.var_gmob_dn6 = assign45270_e58569_d_n6;
        locals.var_gmob_dn7 = assign45270_e58569_d_n7;
        locals.var_gmob_dn8 = assign45270_e58569_d_n8;

        let (assign45280_e58588, assign45280_e58588_d_n5, assign45280_e58588_d_n6, assign45280_e58588_d_n7, assign45280_e58588_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45280_e58574: f64 = (locals.var_v_ds - locals.var_dps);
        let assign45280_e58576: f64 = (assign45280_e58574 * locals.var_inv_vp);
        let assign45280_e58577: f64 = (1.0 + assign45280_e58576);
        let assign45280_e58581: f64 = (locals.var_vdse - locals.var_dps);
        let assign45280_e58583: f64 = (assign45280_e58581 * locals.var_inv_vp);
        let assign45280_e58584: f64 = (1.0 + assign45280_e58583);
        let assign45280_e58585: f64 = (assign45280_e58577 / assign45280_e58584);
        let assign45280_e58586: f64 = (assign45280_e58585).ln();
        (assign45280_e58586, ((((((-locals.var_dps_dn5) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn5 - locals.var_dps_dn5) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn6 - locals.var_dps_dn6) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((-locals.var_dps_dn8) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585),)
    } else {
        (locals.var_s1, locals.var_s1_dn5, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8,)
    }
};
        locals.var_s1 = assign45280_e58588;
        locals.var_s1_dn5 = assign45280_e58588_d_n5;
        locals.var_s1_dn6 = assign45280_e58588_d_n6;
        locals.var_s1_dn7 = assign45280_e58588_d_n7;
        locals.var_s1_dn8 = assign45280_e58588_d_n8;

        let (assign45290_e58594, assign45290_e58594_d_n5, assign45290_e58594_d_n6, assign45290_e58594_d_n7, assign45290_e58594_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45290_e58592: f64 = (locals.var_qim * locals.var_xitsb);
        (assign45290_e58592, ((locals.var_qim_dn5 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn5)), ((locals.var_qim_dn6 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn6)), ((locals.var_qim_dn7 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn7)), ((locals.var_qim_dn8 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign45290_e58594;
        locals.var_temp2_dn5 = assign45290_e58594_d_n5;
        locals.var_temp2_dn6 = assign45290_e58594_d_n6;
        locals.var_temp2_dn7 = assign45290_e58594_d_n7;
        locals.var_temp2_dn8 = assign45290_e58594_d_n8;

        let (assign45300_e58602, assign45300_e58602_d_n5, assign45300_e58602_d_n6, assign45300_e58602_d_n7, assign45300_e58602_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45300_e58599: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign45300_e58600: f64 = (locals.var_temp2 / assign45300_e58599);
        (assign45300_e58600, (((locals.var_temp2_dn5 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn6 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn7 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn8 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign45300_e58599 * assign45300_e58599)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8,)
    }
};
        locals.var_wsat = assign45300_e58602;
        locals.var_wsat_dn5 = assign45300_e58602_d_n5;
        locals.var_wsat_dn6 = assign45300_e58602_d_n6;
        locals.var_wsat_dn7 = assign45300_e58602_d_n7;
        locals.var_wsat_dn8 = assign45300_e58602_d_n8;

        let assign45310_e58605: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign45310_e58605;

        let (assign45320_e58617, assign45320_e58617_d_n5, assign45320_e58617_d_n6, assign45320_e58617_d_n7, assign45320_e58617_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign45320_e58613: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45320_e58614: f64 = (1.0 - assign45320_e58613);
        let assign45320_e58615: f64 = (1.0 / assign45320_e58614);
        (assign45320_e58615, (-((-(locals.var_thesatg_i * locals.var_wsat_dn5)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign45320_e58614 * assign45320_e58614))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign45320_e58617;
        locals.var_factheta_dn5 = assign45320_e58617_d_n5;
        locals.var_factheta_dn6 = assign45320_e58617_d_n6;
        locals.var_factheta_dn7 = assign45320_e58617_d_n7;
        locals.var_factheta_dn8 = assign45320_e58617_d_n8;

        let (assign45330_e58628, assign45330_e58628_d_n5, assign45330_e58628_d_n6, assign45330_e58628_d_n7, assign45330_e58628_d_n8,) = {
    if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 == 0.0)) {
        let assign45330_e58625: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45330_e58626: f64 = (1.0 + assign45330_e58625);
        (assign45330_e58626, (locals.var_thesatg_i * locals.var_wsat_dn5), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8,)
    }
};
        locals.var_factheta = assign45330_e58628;
        locals.var_factheta_dn5 = assign45330_e58628_d_n5;
        locals.var_factheta_dn6 = assign45330_e58628_d_n6;
        locals.var_factheta_dn7 = assign45330_e58628_d_n7;
        locals.var_factheta_dn8 = assign45330_e58628_d_n8;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign45340_e58634, assign45340_e58634_d_n5, assign45340_e58634_d_n6, assign45340_e58634_d_n7, assign45340_e58634_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45340_e58632: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign45340_e58632, (locals.var_thesatloc * locals.var_factheta_dn5), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8,)
    }
};
        locals.var_thesateff = assign45340_e58634;
        locals.var_thesateff_dn5 = assign45340_e58634_d_n5;
        locals.var_thesateff_dn6 = assign45340_e58634_d_n6;
        locals.var_thesateff_dn7 = assign45340_e58634_d_n7;
        locals.var_thesateff_dn8 = assign45340_e58634_d_n8;

        let (assign45350_e58640, assign45350_e58640_d_n5, assign45350_e58640_d_n6, assign45350_e58640_d_n7, assign45350_e58640_d_n8,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign45350_e58638: f64 = (locals.var_xgm * locals.var_phit1);
        (assign45350_e58638, ((locals.var_xgm_dn5 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn5)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8,)
    }
};
        locals.var_voxm = assign45350_e58640;
        locals.var_voxm_dn5 = assign45350_e58640_d_n5;
        locals.var_voxm_dn6 = assign45350_e58640_d_n6;
        locals.var_voxm_dn7 = assign45350_e58640_d_n7;
        locals.var_voxm_dn8 = assign45350_e58640_d_n8;

        locals.var_vdsat_lim_dc = locals.var_vdsat_lim;
        locals.var_vdsat_lim_dc_dn5 = locals.var_vdsat_lim_dn5;
        locals.var_vdsat_lim_dc_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_vdsat_lim_dc_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_vdsat_lim_dc_dn8 = locals.var_vdsat_lim_dn8;

        locals.var_vdse_dc = locals.var_vdse;
        locals.var_vdse_dc_dn5 = locals.var_vdse_dn5;
        locals.var_vdse_dc_dn6 = locals.var_vdse_dn6;
        locals.var_vdse_dc_dn7 = locals.var_vdse_dn7;
        locals.var_vdse_dc_dn8 = locals.var_vdse_dn8;

        locals.var_udse_dc = locals.var_udse;
        locals.var_udse_dc_dn5 = locals.var_udse_dn5;
        locals.var_udse_dc_dn6 = locals.var_udse_dn6;
        locals.var_udse_dc_dn7 = locals.var_udse_dn7;
        locals.var_udse_dc_dn8 = locals.var_udse_dn8;

        locals.var_x_ds_dc = locals.var_x_ds;
        locals.var_x_ds_dc_dn5 = locals.var_x_ds_dn5;
        locals.var_x_ds_dc_dn6 = locals.var_x_ds_dn6;
        locals.var_x_ds_dc_dn7 = locals.var_x_ds_dn7;
        locals.var_x_ds_dc_dn8 = locals.var_x_ds_dn8;

        locals.var_dps_dc = locals.var_dps;
        locals.var_dps_dc_dn5 = locals.var_dps_dn5;
        locals.var_dps_dc_dn6 = locals.var_dps_dn6;
        locals.var_dps_dc_dn7 = locals.var_dps_dn7;
        locals.var_dps_dc_dn8 = locals.var_dps_dn8;

        locals.var_x_m_dc = locals.var_x_m;
        locals.var_x_m_dc_dn5 = locals.var_x_m_dn5;
        locals.var_x_m_dc_dn6 = locals.var_x_m_dn6;
        locals.var_x_m_dc_dn7 = locals.var_x_m_dn7;
        locals.var_x_m_dc_dn8 = locals.var_x_m_dn8;

        locals.var_qbd_dc = locals.var_qbd;
        locals.var_qbd_dc_dn5 = locals.var_qbd_dn5;
        locals.var_qbd_dc_dn6 = locals.var_qbd_dn6;
        locals.var_qbd_dc_dn7 = locals.var_qbd_dn7;
        locals.var_qbd_dc_dn8 = locals.var_qbd_dn8;

        locals.var_eta_p_dc = locals.var_eta_p;
        locals.var_eta_p_dc_dn5 = locals.var_eta_p_dn5;
        locals.var_eta_p_dc_dn6 = locals.var_eta_p_dn6;
        locals.var_eta_p_dc_dn7 = locals.var_eta_p_dn7;
        locals.var_eta_p_dc_dn8 = locals.var_eta_p_dn8;

        locals.var_alpha_dc = locals.var_alpha;
        locals.var_alpha_dc_dn5 = locals.var_alpha_dn5;
        locals.var_alpha_dc_dn6 = locals.var_alpha_dn6;
        locals.var_alpha_dc_dn7 = locals.var_alpha_dn7;
        locals.var_alpha_dc_dn8 = locals.var_alpha_dn8;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn5 = locals.var_qim_dn5;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;

        locals.var_qim1_dc = locals.var_qim1;
        locals.var_qim1_dc_dn5 = locals.var_qim1_dn5;
        locals.var_qim1_dc_dn6 = locals.var_qim1_dn6;
        locals.var_qim1_dc_dn7 = locals.var_qim1_dn7;
        locals.var_qim1_dc_dn8 = locals.var_qim1_dn8;

        locals.var_qbm_dc = locals.var_qbm;
        locals.var_qbm_dc_dn5 = locals.var_qbm_dn5;
        locals.var_qbm_dc_dn6 = locals.var_qbm_dn6;
        locals.var_qbm_dc_dn7 = locals.var_qbm_dn7;
        locals.var_qbm_dc_dn8 = locals.var_qbm_dn8;

        locals.var_qeff1_dc = locals.var_qeff1;
        locals.var_qeff1_dc_dn5 = locals.var_qeff1_dn5;
        locals.var_qeff1_dc_dn6 = locals.var_qeff1_dn6;
        locals.var_qeff1_dc_dn7 = locals.var_qeff1_dn7;
        locals.var_qeff1_dc_dn8 = locals.var_qeff1_dn8;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn5 = locals.var_gmob_dn5;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;

        locals.var_s1_dc = locals.var_s1;
        locals.var_s1_dc_dn5 = locals.var_s1_dn5;
        locals.var_s1_dc_dn6 = locals.var_s1_dn6;
        locals.var_s1_dc_dn7 = locals.var_s1_dn7;
        locals.var_s1_dc_dn8 = locals.var_s1_dn8;

        locals.var_thesateff_dc = locals.var_thesateff;
        locals.var_thesateff_dc_dn5 = locals.var_thesateff_dn5;
        locals.var_thesateff_dc_dn6 = locals.var_thesateff_dn6;
        locals.var_thesateff_dc_dn7 = locals.var_thesateff_dn7;
        locals.var_thesateff_dc_dn8 = locals.var_thesateff_dn8;

        locals.var_voxm_dc = locals.var_voxm;
        locals.var_voxm_dc_dn5 = locals.var_voxm_dn5;
        locals.var_voxm_dc_dn6 = locals.var_voxm_dn6;
        locals.var_voxm_dc_dn7 = locals.var_voxm_dn7;
        locals.var_voxm_dc_dn8 = locals.var_voxm_dn8;

        locals.var_gdl_dc = 1.0;
        locals.var_gdl_dc_dn5 = 0.0;
        locals.var_gdl_dc_dn6 = 0.0;
        locals.var_gdl_dc_dn7 = 0.0;
        locals.var_gdl_dc_dn8 = 0.0;

        locals.var_gmob_dl_dc = 1.0;
        locals.var_gmob_dl_dc_dn5 = 0.0;
        locals.var_gmob_dl_dc_dn6 = 0.0;
        locals.var_gmob_dl_dc_dn7 = 0.0;
        locals.var_gmob_dl_dc_dn8 = 0.0;

        locals.var_gvsatinv_dc = 1.0;
        locals.var_gvsatinv_dc_dn5 = 0.0;
        locals.var_gvsatinv_dc_dn6 = 0.0;
        locals.var_gvsatinv_dc_dn7 = 0.0;
        locals.var_gvsatinv_dc_dn8 = 0.0;

        locals.var_h_dc = 1.0;
        locals.var_h_dc_dn5 = 0.0;
        locals.var_h_dc_dn6 = 0.0;
        locals.var_h_dc_dn7 = 0.0;
        locals.var_h_dc_dn8 = 0.0;

        locals.var_i_ds = 0.0;
        locals.var_i_ds_dn5 = 0.0;
        locals.var_i_ds_dn6 = 0.0;
        locals.var_i_ds_dn7 = 0.0;
        locals.var_i_ds_dn8 = 0.0;

        let assign45690_e58714: f64 = if locals.var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign45690_e58714;

        let (assign45700_e58723, assign45700_e58723_d_n6, assign45700_e58723_d_n7,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45700_e58719: f64 = (locals.var_vdsx * locals.var_inv_vp);
        let assign45700_e58720: f64 = (1.0 + assign45700_e58719);
        let assign45700_e58721: f64 = (assign45700_e58720).ln();
        (assign45700_e58721, ((locals.var_vdsx_dn6 * locals.var_inv_vp) / assign45700_e58720), ((locals.var_vdsx_dn7 * locals.var_inv_vp) / assign45700_e58720),)
    } else {
        (locals.var_s2, locals.var_s2_dn6, locals.var_s2_dn7,)
    }
};
        locals.var_s2 = assign45700_e58723;
        locals.var_s2_dn6 = assign45700_e58723_d_n6;
        locals.var_s2_dn7 = assign45700_e58723_d_n7;

        let (assign45710_e58731, assign45710_e58731_d_n5, assign45710_e58731_d_n6, assign45710_e58731_d_n7, assign45710_e58731_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45710_e58727: f64 = (locals.var_phit1_dc * locals.var_alpha_dc);
        let assign45710_e58729: f64 = (assign45710_e58727 / locals.var_qim1_dc);
        (assign45710_e58729, (((((locals.var_phit1_dc_dn5 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn5)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn6 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn6)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn7 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn7)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn8 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn8)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign45710_e58731;
        locals.var_temp__blk936_dn5 = assign45710_e58731_d_n5;
        locals.var_temp__blk936_dn6 = assign45710_e58731_d_n6;
        locals.var_temp__blk936_dn7 = assign45710_e58731_d_n7;
        locals.var_temp__blk936_dn8 = assign45710_e58731_d_n8;

        let (assign45720_e58755, assign45720_e58755_d_n5, assign45720_e58755_d_n6, assign45720_e58755_d_n7, assign45720_e58755_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45720_e58736: f64 = (locals.var_alp1_i / locals.var_qim1_dc);
        let assign45720_e58737: f64 = (locals.var_alp_i + assign45720_e58736);
        let assign45720_e58739: f64 = (assign45720_e58737 * locals.var_qim_dc);
        let assign45720_e58741: f64 = (assign45720_e58739 / locals.var_qim1_dc);
        let assign45720_e58743: f64 = (assign45720_e58741 * locals.var_s1_dc);
        let assign45720_e58746: f64 = (locals.var_alp2_i * locals.var_qbm_dc);
        let assign45720_e58748: f64 = (assign45720_e58746 * locals.var_temp__blk936);
        let assign45720_e58750: f64 = (assign45720_e58748 * locals.var_temp__blk936);
        let assign45720_e58752: f64 = (assign45720_e58750 * locals.var_s2);
        let assign45720_e58753: f64 = (assign45720_e58743 + assign45720_e58752);
        (assign45720_e58753, (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn5) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn5)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn5)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn5) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn5)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn6) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn6)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn6)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn6) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn6)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn6))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn7) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn7)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn7)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn7) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn7)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn7))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn8) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn8)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn8)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn8) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn8)) * locals.var_s2)),)
    } else {
        (locals.var_dl, locals.var_dl_dn5, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8,)
    }
};
        locals.var_dl = assign45720_e58755;
        locals.var_dl_dn5 = assign45720_e58755_d_n5;
        locals.var_dl_dn6 = assign45720_e58755_d_n6;
        locals.var_dl_dn7 = assign45720_e58755_d_n7;
        locals.var_dl_dn8 = assign45720_e58755_d_n8;

        let (assign45730_e58767, assign45730_e58767_d_n5, assign45730_e58767_d_n6, assign45730_e58767_d_n7, assign45730_e58767_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45730_e58760: f64 = (1.0 + locals.var_dl);
        let assign45730_e58763: f64 = (locals.var_dl * locals.var_dl);
        let assign45730_e58764: f64 = (assign45730_e58760 + assign45730_e58763);
        let assign45730_e58765: f64 = (1.0 / assign45730_e58764);
        (assign45730_e58765, (-((locals.var_dl_dn5 + ((locals.var_dl_dn5 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn5))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign45730_e58764 * assign45730_e58764))),)
    } else {
        (locals.var_gdl_dc, locals.var_gdl_dc_dn5, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8,)
    }
};
        locals.var_gdl_dc = assign45730_e58767;
        locals.var_gdl_dc_dn5 = assign45730_e58767_d_n5;
        locals.var_gdl_dc_dn6 = assign45730_e58767_d_n6;
        locals.var_gdl_dc_dn7 = assign45730_e58767_d_n7;
        locals.var_gdl_dc_dn8 = assign45730_e58767_d_n8;

        let (assign45740_e58773, assign45740_e58773_d_n5, assign45740_e58773_d_n6, assign45740_e58773_d_n7, assign45740_e58773_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45740_e58771: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        (assign45740_e58771, ((locals.var_gmob_dc_dn5 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn5)), ((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)), ((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)), ((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)),)
    } else {
        (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn5, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8,)
    }
};
        locals.var_gmob_dl_dc = assign45740_e58773;
        locals.var_gmob_dl_dc_dn5 = assign45740_e58773_d_n5;
        locals.var_gmob_dl_dc_dn6 = assign45740_e58773_d_n6;
        locals.var_gmob_dl_dc_dn7 = assign45740_e58773_d_n7;
        locals.var_gmob_dl_dc_dn8 = assign45740_e58773_d_n8;

        let (assign45750_e58779, assign45750_e58779_d_n5, assign45750_e58779_d_n6, assign45750_e58779_d_n7, assign45750_e58779_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45750_e58777: f64 = (locals.var_thesateff_dc / locals.var_gmob_dl_dc);
        (assign45750_e58777, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn5)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn6)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn7)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn8)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)),)
    } else {
        (locals.var_thesat1_dc, locals.var_thesat1_dc_dn5, locals.var_thesat1_dc_dn6, locals.var_thesat1_dc_dn7, locals.var_thesat1_dc_dn8,)
    }
};
        locals.var_thesat1_dc = assign45750_e58779;
        locals.var_thesat1_dc_dn5 = assign45750_e58779_d_n5;
        locals.var_thesat1_dc_dn6 = assign45750_e58779_d_n6;
        locals.var_thesat1_dc_dn7 = assign45750_e58779_d_n7;
        locals.var_thesat1_dc_dn8 = assign45750_e58779_d_n8;

        let (assign45760_e58789, assign45760_e58789_d_n5, assign45760_e58789_d_n6, assign45760_e58789_d_n7, assign45760_e58789_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45760_e58783: f64 = (locals.var_thesat1_dc * locals.var_thesat1_dc);
        let assign45760_e58785: f64 = (assign45760_e58783 * locals.var_dps_dc);
        let assign45760_e58787: f64 = (assign45760_e58785 * locals.var_dps_dc);
        (assign45760_e58787, ((((((locals.var_thesat1_dc_dn5 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_dc_dn6 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_dc_dn7 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_dc_dn8 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn8)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8,)
    }
};
        locals.var_zsat = assign45760_e58789;
        locals.var_zsat_dn5 = assign45760_e58789_d_n5;
        locals.var_zsat_dn6 = assign45760_e58789_d_n6;
        locals.var_zsat_dn7 = assign45760_e58789_d_n7;
        locals.var_zsat_dn8 = assign45760_e58789_d_n8;

        let assign45770_e58792: f64 = (-1.0);
        let assign45770_e58793: f64 = if locals.var_chnl_type == assign45770_e58792 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign45770_e58793;

        let (assign45780_e58805, assign45780_e58805_d_n5, assign45780_e58805_d_n6, assign45780_e58805_d_n7, assign45780_e58805_d_n8,) = {
    if ((locals.var_guard1218 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign45780_e58801: f64 = (locals.var_thesat1_dc * locals.var_dps_dc);
        let assign45780_e58802: f64 = (1.0 + assign45780_e58801);
        let assign45780_e58803: f64 = (locals.var_zsat / assign45780_e58802);
        (assign45780_e58803, (((locals.var_zsat_dn5 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn5)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn6 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn6)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn7 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn7)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn8 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn8)))) / (assign45780_e58802 * assign45780_e58802)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8,)
    }
};
        locals.var_zsat = assign45780_e58805;
        locals.var_zsat_dn5 = assign45780_e58805_d_n5;
        locals.var_zsat_dn6 = assign45780_e58805_d_n6;
        locals.var_zsat_dn7 = assign45780_e58805_d_n7;
        locals.var_zsat_dn8 = assign45780_e58805_d_n8;

        let (assign45790_e58820, assign45790_e58820_d_n5, assign45790_e58820_d_n6, assign45790_e58820_d_n7, assign45790_e58820_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45790_e58813: f64 = (2.0 * locals.var_zsat);
        let assign45790_e58814: f64 = (1.0 + assign45790_e58813);
        let assign45790_e58815: f64 = (assign45790_e58814).sqrt();
        let assign45790_e58816: f64 = (1.0 + assign45790_e58815);
        let assign45790_e58817: f64 = (locals.var_gmob_dl_dc * assign45790_e58816);
        let assign45790_e58818: f64 = (0.5 * assign45790_e58817);
        (assign45790_e58818, (0.5 * ((locals.var_gmob_dl_dc_dn5 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn5) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn6 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn7 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn8 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign45790_e58815))))),)
    } else {
        (locals.var_gvsat, locals.var_gvsat_dn5, locals.var_gvsat_dn6, locals.var_gvsat_dn7, locals.var_gvsat_dn8,)
    }
};
        locals.var_gvsat = assign45790_e58820;
        locals.var_gvsat_dn5 = assign45790_e58820_d_n5;
        locals.var_gvsat_dn6 = assign45790_e58820_d_n6;
        locals.var_gvsat_dn7 = assign45790_e58820_d_n7;
        locals.var_gvsat_dn8 = assign45790_e58820_d_n8;

        let (assign45800_e58826, assign45800_e58826_d_n5, assign45800_e58826_d_n6, assign45800_e58826_d_n7, assign45800_e58826_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45800_e58824: f64 = (1.0 / locals.var_gvsat);
        (assign45800_e58824, (-(locals.var_gvsat_dn5 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn6 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn7 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn8 / (locals.var_gvsat * locals.var_gvsat))),)
    } else {
        (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn5, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8,)
    }
};
        locals.var_gvsatinv_dc = assign45800_e58826;
        locals.var_gvsatinv_dc_dn5 = assign45800_e58826_d_n5;
        locals.var_gvsatinv_dc_dn6 = assign45800_e58826_d_n6;
        locals.var_gvsatinv_dc_dn7 = assign45800_e58826_d_n7;
        locals.var_gvsatinv_dc_dn8 = assign45800_e58826_d_n8;

        let (assign45810_e58832, assign45810_e58832_d_n5, assign45810_e58832_d_n6, assign45810_e58832_d_n7, assign45810_e58832_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45810_e58830: f64 = (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc);
        (assign45810_e58830, ((locals.var_gmob_dl_dc_dn5 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn5)), ((locals.var_gmob_dl_dc_dn6 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn6)), ((locals.var_gmob_dl_dc_dn7 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn7)), ((locals.var_gmob_dl_dc_dn8 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign45810_e58832;
        locals.var_temp__blk936_dn5 = assign45810_e58832_d_n5;
        locals.var_temp__blk936_dn6 = assign45810_e58832_d_n6;
        locals.var_temp__blk936_dn7 = assign45810_e58832_d_n7;
        locals.var_temp__blk936_dn8 = assign45810_e58832_d_n8;

        let (assign45820_e58846, assign45820_e58846_d_n5, assign45820_e58846_d_n6, assign45820_e58846_d_n7, assign45820_e58846_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45820_e58839: f64 = (locals.var_zsat * locals.var_temp__blk936);
        let assign45820_e58841: f64 = (assign45820_e58839 * locals.var_temp__blk936);
        let assign45820_e58842: f64 = (0.5 * assign45820_e58841);
        let assign45820_e58843: f64 = (1.0 + assign45820_e58842);
        let assign45820_e58844: f64 = (locals.var_alpha_dc * assign45820_e58843);
        (assign45820_e58844, ((locals.var_alpha_dc_dn5 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn5 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_dc_dn6 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn6 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_dc_dn7 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn7 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_dc_dn8 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn8 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn8))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8,)
    }
};
        locals.var_alpha1 = assign45820_e58846;
        locals.var_alpha1_dn5 = assign45820_e58846_d_n5;
        locals.var_alpha1_dn6 = assign45820_e58846_d_n6;
        locals.var_alpha1_dn7 = assign45820_e58846_d_n7;
        locals.var_alpha1_dn8 = assign45820_e58846_d_n8;

        let (assign45830_e58854, assign45830_e58854_d_n5, assign45830_e58854_d_n6, assign45830_e58854_d_n7, assign45830_e58854_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45830_e58850: f64 = (locals.var_temp__blk936 * locals.var_qim1_dc);
        let assign45830_e58852: f64 = (assign45830_e58850 / locals.var_alpha1);
        (assign45830_e58852, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn5)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn5)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn6)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn7)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn8)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)),)
    } else {
        (locals.var_h_dc, locals.var_h_dc_dn5, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8,)
    }
};
        locals.var_h_dc = assign45830_e58854;
        locals.var_h_dc_dn5 = assign45830_e58854_d_n5;
        locals.var_h_dc_dn6 = assign45830_e58854_d_n6;
        locals.var_h_dc_dn7 = assign45830_e58854_d_n7;
        locals.var_h_dc_dn8 = assign45830_e58854_d_n8;

        let (assign45840_e58864, assign45840_e58864_d_n5, assign45840_e58864_d_n6, assign45840_e58864_d_n7, assign45840_e58864_d_n8,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign45840_e58858: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign45840_e58860: f64 = (assign45840_e58858 * locals.var_dps_dc);
        let assign45840_e58862: f64 = (assign45840_e58860 * locals.var_gvsatinv_dc);
        (assign45840_e58862, (((((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn5)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn5)), (((((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn6)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn6)), (((((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn7)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn7)), (((((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn8)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn8)),)
    } else {
        (locals.var_i_ds, locals.var_i_ds_dn5, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8,)
    }
};
        locals.var_i_ds = assign45840_e58864;
        locals.var_i_ds_dn5 = assign45840_e58864_d_n5;
        locals.var_i_ds_dn6 = assign45840_e58864_d_n6;
        locals.var_i_ds_dn7 = assign45840_e58864_d_n7;
        locals.var_i_ds_dn8 = assign45840_e58864_d_n8;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn5 = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn5 = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;

        locals.var_vovs = 0.0;
        locals.var_vovs_dn5 = 0.0;
        locals.var_vovs_dn6 = 0.0;
        locals.var_vovs_dn7 = 0.0;

        locals.var_vovd = 0.0;
        locals.var_vovd_dn5 = 0.0;
        locals.var_vovd_dn6 = 0.0;
        locals.var_vovd_dn7 = 0.0;

        let assign45890_e58899: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign45890_e58899;

        let (assign45900_e58912, assign45900_e58912_d_n5, assign45900_e58912_d_n6, assign45900_e58912_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45900_e58905: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
        let assign45900_e58907: f64 = (assign45900_e58905 + locals.var_sp_ov_eps2_s);
        let assign45900_e58908: f64 = (assign45900_e58907).sqrt();
        let assign45900_e58909: f64 = (locals.var_xgs_ov + assign45900_e58908);
        let assign45900_e58910: f64 = (0.5 * assign45900_e58909);
        (assign45900_e58910, (0.5 * (locals.var_xgs_ov_dn5 + (((locals.var_xgs_ov_dn5 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn5)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign45900_e58908)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7,)
    }
};
        locals.var_sp_ov_xg = assign45900_e58912;
        locals.var_sp_ov_xg_dn5 = assign45900_e58912_d_n5;
        locals.var_sp_ov_xg_dn6 = assign45900_e58912_d_n6;
        locals.var_sp_ov_xg_dn7 = assign45900_e58912_d_n7;

        let (assign45910_e58934, assign45910_e58934_d_n5, assign45910_e58934_d_n6, assign45910_e58934_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45910_e58915: f64 = (-locals.var_sp_ov_xg);
        let assign45910_e58918: f64 = (locals.var_gov2_s * 0.5);
        let assign45910_e58919: f64 = (assign45910_e58915 - assign45910_e58918);
        let assign45910_e58924: f64 = (locals.var_gov2_s * 0.25);
        let assign45910_e58925: f64 = (locals.var_sp_ov_xg + assign45910_e58924);
        let assign45910_e58927: f64 = (assign45910_e58925 + locals.var_sp_ov_a_s);
        let assign45910_e58928: f64 = (assign45910_e58927).sqrt();
        let assign45910_e58929: f64 = (locals.var_gov_s * assign45910_e58928);
        let assign45910_e58930: f64 = (assign45910_e58919 + assign45910_e58929);
        let assign45910_e58932: f64 = (assign45910_e58930 + locals.var_sp_ov_delta1_s);
        (assign45910_e58932, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45910_e58928)))),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7,)
    }
};
        locals.var_xs_ov = assign45910_e58934;
        locals.var_xs_ov_dn5 = assign45910_e58934_d_n5;
        locals.var_xs_ov_dn6 = assign45910_e58934_d_n6;
        locals.var_xs_ov_dn7 = assign45910_e58934_d_n7;

        let (assign45920_e58947, assign45920_e58947_d_n5, assign45920_e58947_d_n6, assign45920_e58947_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45920_e58940: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
        let assign45920_e58942: f64 = (assign45920_e58940 + locals.var_sp_ov_eps2_d);
        let assign45920_e58943: f64 = (assign45920_e58942).sqrt();
        let assign45920_e58944: f64 = (locals.var_xgd_ov + assign45920_e58943);
        let assign45920_e58945: f64 = (0.5 * assign45920_e58944);
        (assign45920_e58945, (0.5 * (locals.var_xgd_ov_dn5 + (((locals.var_xgd_ov_dn5 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn5)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign45920_e58943)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7,)
    }
};
        locals.var_sp_ov_xg = assign45920_e58947;
        locals.var_sp_ov_xg_dn5 = assign45920_e58947_d_n5;
        locals.var_sp_ov_xg_dn6 = assign45920_e58947_d_n6;
        locals.var_sp_ov_xg_dn7 = assign45920_e58947_d_n7;

        let (assign45930_e58969, assign45930_e58969_d_n5, assign45930_e58969_d_n6, assign45930_e58969_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45930_e58950: f64 = (-locals.var_sp_ov_xg);
        let assign45930_e58953: f64 = (locals.var_gov2_d * 0.5);
        let assign45930_e58954: f64 = (assign45930_e58950 - assign45930_e58953);
        let assign45930_e58959: f64 = (locals.var_gov2_d * 0.25);
        let assign45930_e58960: f64 = (locals.var_sp_ov_xg + assign45930_e58959);
        let assign45930_e58962: f64 = (assign45930_e58960 + locals.var_sp_ov_a_d);
        let assign45930_e58963: f64 = (assign45930_e58962).sqrt();
        let assign45930_e58964: f64 = (locals.var_gov_d * assign45930_e58963);
        let assign45930_e58965: f64 = (assign45930_e58954 + assign45930_e58964);
        let assign45930_e58967: f64 = (assign45930_e58965 + locals.var_sp_ov_delta1_d);
        (assign45930_e58967, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45930_e58963)))),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7,)
    }
};
        locals.var_xd_ov = assign45930_e58969;
        locals.var_xd_ov_dn5 = assign45930_e58969_d_n5;
        locals.var_xd_ov_dn6 = assign45930_e58969_d_n6;
        locals.var_xd_ov_dn7 = assign45930_e58969_d_n7;

        let (assign45940_e58978, assign45940_e58978_d_n5, assign45940_e58978_d_n6, assign45940_e58978_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45940_e58972: f64 = (-locals.var_phita);
        let assign45940_e58975: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign45940_e58976: f64 = (assign45940_e58972 * assign45940_e58975);
        (assign45940_e58976, (assign45940_e58972 * (locals.var_xgs_ov_dn5 + locals.var_xs_ov_dn5)), (assign45940_e58972 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign45940_e58972 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)),)
    } else {
        (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7,)
    }
};
        locals.var_vovs = assign45940_e58978;
        locals.var_vovs_dn5 = assign45940_e58978_d_n5;
        locals.var_vovs_dn6 = assign45940_e58978_d_n6;
        locals.var_vovs_dn7 = assign45940_e58978_d_n7;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign45950_e58987, assign45950_e58987_d_n5, assign45950_e58987_d_n6, assign45950_e58987_d_n7,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign45950_e58981: f64 = (-locals.var_phita);
        let assign45950_e58984: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign45950_e58985: f64 = (assign45950_e58981 * assign45950_e58984);
        (assign45950_e58985, (assign45950_e58981 * (locals.var_xgd_ov_dn5 + locals.var_xd_ov_dn5)), (assign45950_e58981 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign45950_e58981 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)),)
    } else {
        (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7,)
    }
};
        locals.var_vovd = assign45950_e58987;
        locals.var_vovd_dn5 = assign45950_e58987_d_n5;
        locals.var_vovd_dn6 = assign45950_e58987_d_n6;
        locals.var_vovd_dn7 = assign45950_e58987_d_n7;

        locals.var_igsov = 0.0;
        locals.var_igsov_dn5 = 0.0;
        locals.var_igsov_dn6 = 0.0;
        locals.var_igsov_dn7 = 0.0;
        locals.var_igsov_dn8 = 0.0;

        locals.var_igdov = 0.0;
        locals.var_igdov_dn5 = 0.0;
        locals.var_igdov_dn6 = 0.0;
        locals.var_igdov_dn7 = 0.0;
        locals.var_igdov_dn8 = 0.0;

        locals.var_igc_1 = 0.0;
        locals.var_igc_1_dn5 = 0.0;
        locals.var_igc_1_dn6 = 0.0;
        locals.var_igc_1_dn7 = 0.0;
        locals.var_igc_1_dn8 = 0.0;

        locals.var_i_gb = 0.0;
        locals.var_i_gb_dn5 = 0.0;
        locals.var_i_gb_dn6 = 0.0;
        locals.var_i_gb_dn7 = 0.0;
        locals.var_i_gb_dn8 = 0.0;

        locals.var_i_gcs = 0.0;
        locals.var_i_gcs_dn5 = 0.0;
        locals.var_i_gcs_dn6 = 0.0;
        locals.var_i_gcs_dn7 = 0.0;
        locals.var_i_gcs_dn8 = 0.0;

        locals.var_i_gcd = 0.0;
        locals.var_i_gcd_dn5 = 0.0;
        locals.var_i_gcd_dn6 = 0.0;
        locals.var_i_gcd_dn7 = 0.0;
        locals.var_i_gcd_dn8 = 0.0;

        let assign46020_e58996: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign46020_e58996;

        let assign46030_e58999: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign46030_e58999;

        let (assign46040_e59012, assign46040_e59012_d_n5, assign46040_e59012_d_n6, assign46040_e59012_d_n7, assign46040_e59012_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46040_e59005: f64 = (locals.var_vovs * locals.var_vovs);
        let assign46040_e59007: f64 = (assign46040_e59005 + 1e-6);
        let assign46040_e59008: f64 = (assign46040_e59007).sqrt();
        let assign46040_e59010: f64 = (assign46040_e59008 * locals.var_inv_chib);
        (assign46040_e59010, ((((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46040_e59012;
        locals.var_zg_dn5 = assign46040_e59012_d_n5;
        locals.var_zg_dn6 = assign46040_e59012_d_n6;
        locals.var_zg_dn7 = assign46040_e59012_d_n7;
        locals.var_zg_dn8 = assign46040_e59012_d_n8;

        let assign46050_e59015: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign46050_e59015;

        let (assign46060_e59038, assign46060_e59038_d_n5, assign46060_e59038_d_n6, assign46060_e59038_d_n7, assign46060_e59038_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign46060_e59024: f64 = (locals.var_zg + locals.var_gcqov);
        let assign46060_e59027: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46060_e59030: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46060_e59031: f64 = (assign46060_e59027 * assign46060_e59030);
        let assign46060_e59033: f64 = (assign46060_e59031 + 1e-6);
        let assign46060_e59034: f64 = (assign46060_e59033).sqrt();
        let assign46060_e59035: f64 = (assign46060_e59024 - assign46060_e59034);
        let assign46060_e59036: f64 = (0.5 * assign46060_e59035);
        (assign46060_e59036, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn5)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn6)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn7)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn8)) / (2.0 * assign46060_e59034)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46060_e59038;
        locals.var_zg_dn5 = assign46060_e59038_d_n5;
        locals.var_zg_dn6 = assign46060_e59038_d_n6;
        locals.var_zg_dn7 = assign46060_e59038_d_n7;
        locals.var_zg_dn8 = assign46060_e59038_d_n8;

        let (assign46070_e59055, assign46070_e59055_d_n5, assign46070_e59055_d_n6, assign46070_e59055_d_n7, assign46070_e59055_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46070_e59044: f64 = (-1.5);
        let assign46070_e59049: f64 = (locals.var_gc3ov_i * locals.var_zg);
        let assign46070_e59050: f64 = (locals.var_gc2ov_i + assign46070_e59049);
        let assign46070_e59051: f64 = (locals.var_zg * assign46070_e59050);
        let assign46070_e59052: f64 = (assign46070_e59044 + assign46070_e59051);
        let assign46070_e59053: f64 = (locals.var_bov * assign46070_e59052);
        (assign46070_e59053, (locals.var_bov * ((locals.var_zg_dn5 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn5)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46070_e59055;
        locals.var_temp__blk936_dn5 = assign46070_e59055_d_n5;
        locals.var_temp__blk936_dn6 = assign46070_e59055_d_n6;
        locals.var_temp__blk936_dn7 = assign46070_e59055_d_n7;
        locals.var_temp__blk936_dn8 = assign46070_e59055_d_n8;

        let assign46080_e59058: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign46080_e59058;

        let (assign46090_e59080, assign46090_e59080_d_n5, assign46090_e59080_d_n6, assign46090_e59080_d_n7, assign46090_e59080_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 != 0.0)) {
        let assign46090_e59072: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
        let assign46090_e59073: f64 = (1.0 + assign46090_e59072);
        let assign46090_e59074: f64 = (locals.var_temp__blk936 * assign46090_e59073);
        let assign46090_e59075: f64 = (0.5 * assign46090_e59074);
        let assign46090_e59076: f64 = (1.0 + assign46090_e59075);
        let assign46090_e59077: f64 = (locals.var_temp__blk936 * assign46090_e59076);
        let assign46090_e59078: f64 = (1.0 + assign46090_e59077);
        (assign46090_e59078, ((locals.var_temp__blk936_dn5 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46090_e59080;
        locals.var_tp_dn5 = assign46090_e59080_d_n5;
        locals.var_tp_dn6 = assign46090_e59080_d_n6;
        locals.var_tp_dn7 = assign46090_e59080_d_n7;
        locals.var_tp_dn8 = assign46090_e59080_d_n8;

        let assign46100_e59083: f64 = (-230.25850929940458);
        let assign46100_e59084: f64 = if locals.var_temp__blk936 > assign46100_e59083 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign46100_e59084;

        let (assign46110_e59096, assign46110_e59096_d_n5, assign46110_e59096_d_n6, assign46110_e59096_d_n7, assign46110_e59096_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 == 0.0)) && (locals.var_guard1225 != 0.0)) {
        let assign46110_e59094: f64 = (locals.var_temp__blk936).exp();
        (assign46110_e59094, (assign46110_e59094 * locals.var_temp__blk936_dn5), (assign46110_e59094 * locals.var_temp__blk936_dn6), (assign46110_e59094 * locals.var_temp__blk936_dn7), (assign46110_e59094 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46110_e59096;
        locals.var_tp_dn5 = assign46110_e59096_d_n5;
        locals.var_tp_dn6 = assign46110_e59096_d_n6;
        locals.var_tp_dn7 = assign46110_e59096_d_n7;
        locals.var_tp_dn8 = assign46110_e59096_d_n8;

        let (assign46120_e59133, assign46120_e59133_d_n5, assign46120_e59133_d_n6, assign46120_e59133_d_n7, assign46120_e59133_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 == 0.0)) && (locals.var_guard1225 == 0.0)) {
        let assign46120_e59109: f64 = (-230.25850929940458);
        let assign46120_e59111: f64 = (assign46120_e59109 - locals.var_temp__blk936);
        let assign46120_e59115: f64 = (-230.25850929940458);
        let assign46120_e59117: f64 = (assign46120_e59115 - locals.var_temp__blk936);
        let assign46120_e59120: f64 = (-230.25850929940458);
        let assign46120_e59122: f64 = (assign46120_e59120 - locals.var_temp__blk936);
        let assign46120_e59124: f64 = (assign46120_e59122 * 0.3333333333333333);
        let assign46120_e59125: f64 = (1.0 + assign46120_e59124);
        let assign46120_e59126: f64 = (assign46120_e59117 * assign46120_e59125);
        let assign46120_e59127: f64 = (0.5 * assign46120_e59126);
        let assign46120_e59128: f64 = (1.0 + assign46120_e59127);
        let assign46120_e59129: f64 = (assign46120_e59111 * assign46120_e59128);
        let assign46120_e59130: f64 = (1.0 + assign46120_e59129);
        let assign46120_e59131: f64 = (1e-100 / assign46120_e59130);
        (assign46120_e59131, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46120_e59133;
        locals.var_tp_dn5 = assign46120_e59133_d_n5;
        locals.var_tp_dn6 = assign46120_e59133_d_n6;
        locals.var_tp_dn7 = assign46120_e59133_d_n7;
        locals.var_tp_dn8 = assign46120_e59133_d_n8;

        let (assign46130_e59141, assign46130_e59141_d_n5, assign46130_e59141_d_n6, assign46130_e59141_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46130_e59139: f64 = (3.0 + locals.var_xs_ov);
        (assign46130_e59139, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7,)
    }
};
        locals.var_fs1 = assign46130_e59141;
        locals.var_fs1_dn5 = assign46130_e59141_d_n5;
        locals.var_fs1_dn6 = assign46130_e59141_d_n6;
        locals.var_fs1_dn7 = assign46130_e59141_d_n7;

        let (assign46140_e59150,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46140_e59146: f64 = (-3.0);
        let assign46140_e59148: f64 = (assign46140_e59146 - locals.var_gco_i);
        (assign46140_e59148,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46140_e59150;

        let (assign46150_e59158, assign46150_e59158_d_n5, assign46150_e59158_d_n6, assign46150_e59158_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46150_e59156: f64 = (30.0 * locals.var_vgsprime);
        (assign46150_e59156, (30.0 * locals.var_vgsprime_dn5), (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7,)
    }
};
        locals.var_fs3 = assign46150_e59158;
        locals.var_fs3_dn5 = assign46150_e59158_d_n5;
        locals.var_fs3_dn6 = assign46150_e59158_d_n6;
        locals.var_fs3_dn7 = assign46150_e59158_d_n7;

        let (assign46160_e59166,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46160_e59164: f64 = (4.0 - 0.9);
        (assign46160_e59164,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46160_e59166;

        let (assign46170_e59174, assign46170_e59174_d_n5, assign46170_e59174_d_n6, assign46170_e59174_d_n7, assign46170_e59174_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46170_e59172: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46170_e59172, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46170_e59174;
        locals.var_tme2_dn5 = assign46170_e59174_d_n5;
        locals.var_tme2_dn6 = assign46170_e59174_d_n6;
        locals.var_tme2_dn7 = assign46170_e59174_d_n7;
        locals.var_tme2_dn8 = assign46170_e59174_d_n8;

        let (assign46180_e59195, assign46180_e59195_d_n5, assign46180_e59195_d_n6, assign46180_e59195_d_n7, assign46180_e59195_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46180_e59180: f64 = (2.0 / locals.var_tme1);
        let assign46180_e59184: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46180_e59187: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46180_e59189: f64 = (assign46180_e59187 * locals.var_fs3);
        let assign46180_e59190: f64 = (assign46180_e59184 - assign46180_e59189);
        let assign46180_e59191: f64 = (assign46180_e59190).sqrt();
        let assign46180_e59192: f64 = (locals.var_tme2 - assign46180_e59191);
        let assign46180_e59193: f64 = (assign46180_e59180 * assign46180_e59192);
        (assign46180_e59193, (assign46180_e59180 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn5))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn6))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn7))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46180_e59191)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46180_e59195;
        locals.var_temp__blk936_dn5 = assign46180_e59195_d_n5;
        locals.var_temp__blk936_dn6 = assign46180_e59195_d_n6;
        locals.var_temp__blk936_dn7 = assign46180_e59195_d_n7;
        locals.var_temp__blk936_dn8 = assign46180_e59195_d_n8;

        let (assign46190_e59203,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46190_e59201: f64 = (4.0 - 0.3);
        (assign46190_e59201,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46190_e59203;

        let (assign46200_e59211, assign46200_e59211_d_n5, assign46200_e59211_d_n6, assign46200_e59211_d_n7, assign46200_e59211_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46200_e59209: f64 = (locals.var_fs2 + locals.var_temp__blk936);
        (assign46200_e59209, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46200_e59211;
        locals.var_tme2_dn5 = assign46200_e59211_d_n5;
        locals.var_tme2_dn6 = assign46200_e59211_d_n6;
        locals.var_tme2_dn7 = assign46200_e59211_d_n7;
        locals.var_tme2_dn8 = assign46200_e59211_d_n8;

        let (assign46210_e59232, assign46210_e59232_d_n5, assign46210_e59232_d_n6, assign46210_e59232_d_n7, assign46210_e59232_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46210_e59217: f64 = (2.0 / locals.var_tme1);
        let assign46210_e59221: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46210_e59224: f64 = (locals.var_tme1 * locals.var_fs2);
        let assign46210_e59226: f64 = (assign46210_e59224 * locals.var_temp__blk936);
        let assign46210_e59227: f64 = (assign46210_e59221 - assign46210_e59226);
        let assign46210_e59228: f64 = (assign46210_e59227).sqrt();
        let assign46210_e59229: f64 = (locals.var_tme2 + assign46210_e59228);
        let assign46210_e59230: f64 = (assign46210_e59217 * assign46210_e59229);
        (assign46210_e59230, (assign46210_e59217 * (locals.var_tme2_dn5 + ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (assign46210_e59224 * locals.var_temp__blk936_dn5)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46210_e59224 * locals.var_temp__blk936_dn6)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46210_e59224 * locals.var_temp__blk936_dn7)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46210_e59224 * locals.var_temp__blk936_dn8)) / (2.0 * assign46210_e59228)))),)
    } else {
        (locals.var_fs, locals.var_fs_dn5, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8,)
    }
};
        locals.var_fs = assign46210_e59232;
        locals.var_fs_dn5 = assign46210_e59232_d_n5;
        locals.var_fs_dn6 = assign46210_e59232_d_n6;
        locals.var_fs_dn7 = assign46210_e59232_d_n7;
        locals.var_fs_dn8 = assign46210_e59232_d_n8;

        let (assign46220_e59242, assign46220_e59242_d_n5, assign46220_e59242_d_n6, assign46220_e59242_d_n7, assign46220_e59242_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign46220_e59239: f64 = (locals.var_tp * locals.var_fs);
        let assign46220_e59240: f64 = (locals.var_igov_i * assign46220_e59239);
        (assign46220_e59240, (locals.var_igov_i * ((locals.var_tp_dn5 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn5))), (locals.var_igov_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igov_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igov_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))),)
    } else {
        (locals.var_igsov, locals.var_igsov_dn5, locals.var_igsov_dn6, locals.var_igsov_dn7, locals.var_igsov_dn8,)
    }
};
        locals.var_igsov = assign46220_e59242;
        locals.var_igsov_dn5 = assign46220_e59242_d_n5;
        locals.var_igsov_dn6 = assign46220_e59242_d_n6;
        locals.var_igsov_dn7 = assign46220_e59242_d_n7;
        locals.var_igsov_dn8 = assign46220_e59242_d_n8;

        let assign46230_e59245: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign46230_e59245;

        let (assign46240_e59258, assign46240_e59258_d_n5, assign46240_e59258_d_n6, assign46240_e59258_d_n7, assign46240_e59258_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46240_e59251: f64 = (locals.var_vovd * locals.var_vovd);
        let assign46240_e59253: f64 = (assign46240_e59251 + 1e-6);
        let assign46240_e59254: f64 = (assign46240_e59253).sqrt();
        let assign46240_e59256: f64 = (assign46240_e59254 * locals.var_inv_chib);
        (assign46240_e59256, ((((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46240_e59258;
        locals.var_zg_dn5 = assign46240_e59258_d_n5;
        locals.var_zg_dn6 = assign46240_e59258_d_n6;
        locals.var_zg_dn7 = assign46240_e59258_d_n7;
        locals.var_zg_dn8 = assign46240_e59258_d_n8;

        let assign46250_e59261: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign46250_e59261;

        let (assign46260_e59284, assign46260_e59284_d_n5, assign46260_e59284_d_n6, assign46260_e59284_d_n7, assign46260_e59284_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign46260_e59270: f64 = (locals.var_zg + locals.var_gcqovd);
        let assign46260_e59273: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46260_e59276: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46260_e59277: f64 = (assign46260_e59273 * assign46260_e59276);
        let assign46260_e59279: f64 = (assign46260_e59277 + 1e-6);
        let assign46260_e59280: f64 = (assign46260_e59279).sqrt();
        let assign46260_e59281: f64 = (assign46260_e59270 - assign46260_e59280);
        let assign46260_e59282: f64 = (0.5 * assign46260_e59281);
        (assign46260_e59282, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn5)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn6)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn7)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn8)) / (2.0 * assign46260_e59280)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46260_e59284;
        locals.var_zg_dn5 = assign46260_e59284_d_n5;
        locals.var_zg_dn6 = assign46260_e59284_d_n6;
        locals.var_zg_dn7 = assign46260_e59284_d_n7;
        locals.var_zg_dn8 = assign46260_e59284_d_n8;

        let (assign46270_e59301, assign46270_e59301_d_n5, assign46270_e59301_d_n6, assign46270_e59301_d_n7, assign46270_e59301_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46270_e59290: f64 = (-1.5);
        let assign46270_e59295: f64 = (locals.var_gc3ovd_i * locals.var_zg);
        let assign46270_e59296: f64 = (locals.var_gc2ovd_i + assign46270_e59295);
        let assign46270_e59297: f64 = (locals.var_zg * assign46270_e59296);
        let assign46270_e59298: f64 = (assign46270_e59290 + assign46270_e59297);
        let assign46270_e59299: f64 = (locals.var_bov_d * assign46270_e59298);
        (assign46270_e59299, (locals.var_bov_d * ((locals.var_zg_dn5 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn5)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46270_e59301;
        locals.var_temp__blk936_dn5 = assign46270_e59301_d_n5;
        locals.var_temp__blk936_dn6 = assign46270_e59301_d_n6;
        locals.var_temp__blk936_dn7 = assign46270_e59301_d_n7;
        locals.var_temp__blk936_dn8 = assign46270_e59301_d_n8;

        let assign46280_e59304: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign46280_e59304;

        let (assign46290_e59326, assign46290_e59326_d_n5, assign46290_e59326_d_n6, assign46290_e59326_d_n7, assign46290_e59326_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign46290_e59318: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
        let assign46290_e59319: f64 = (1.0 + assign46290_e59318);
        let assign46290_e59320: f64 = (locals.var_temp__blk936 * assign46290_e59319);
        let assign46290_e59321: f64 = (0.5 * assign46290_e59320);
        let assign46290_e59322: f64 = (1.0 + assign46290_e59321);
        let assign46290_e59323: f64 = (locals.var_temp__blk936 * assign46290_e59322);
        let assign46290_e59324: f64 = (1.0 + assign46290_e59323);
        (assign46290_e59324, ((locals.var_temp__blk936_dn5 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46290_e59326;
        locals.var_tp_dn5 = assign46290_e59326_d_n5;
        locals.var_tp_dn6 = assign46290_e59326_d_n6;
        locals.var_tp_dn7 = assign46290_e59326_d_n7;
        locals.var_tp_dn8 = assign46290_e59326_d_n8;

        let assign46300_e59329: f64 = (-230.25850929940458);
        let assign46300_e59330: f64 = if locals.var_temp__blk936 > assign46300_e59329 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign46300_e59330;

        let (assign46310_e59342, assign46310_e59342_d_n5, assign46310_e59342_d_n6, assign46310_e59342_d_n7, assign46310_e59342_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 == 0.0)) && (locals.var_guard1229 != 0.0)) {
        let assign46310_e59340: f64 = (locals.var_temp__blk936).exp();
        (assign46310_e59340, (assign46310_e59340 * locals.var_temp__blk936_dn5), (assign46310_e59340 * locals.var_temp__blk936_dn6), (assign46310_e59340 * locals.var_temp__blk936_dn7), (assign46310_e59340 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46310_e59342;
        locals.var_tp_dn5 = assign46310_e59342_d_n5;
        locals.var_tp_dn6 = assign46310_e59342_d_n6;
        locals.var_tp_dn7 = assign46310_e59342_d_n7;
        locals.var_tp_dn8 = assign46310_e59342_d_n8;

        let (assign46320_e59379, assign46320_e59379_d_n5, assign46320_e59379_d_n6, assign46320_e59379_d_n7, assign46320_e59379_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 == 0.0)) && (locals.var_guard1229 == 0.0)) {
        let assign46320_e59355: f64 = (-230.25850929940458);
        let assign46320_e59357: f64 = (assign46320_e59355 - locals.var_temp__blk936);
        let assign46320_e59361: f64 = (-230.25850929940458);
        let assign46320_e59363: f64 = (assign46320_e59361 - locals.var_temp__blk936);
        let assign46320_e59366: f64 = (-230.25850929940458);
        let assign46320_e59368: f64 = (assign46320_e59366 - locals.var_temp__blk936);
        let assign46320_e59370: f64 = (assign46320_e59368 * 0.3333333333333333);
        let assign46320_e59371: f64 = (1.0 + assign46320_e59370);
        let assign46320_e59372: f64 = (assign46320_e59363 * assign46320_e59371);
        let assign46320_e59373: f64 = (0.5 * assign46320_e59372);
        let assign46320_e59374: f64 = (1.0 + assign46320_e59373);
        let assign46320_e59375: f64 = (assign46320_e59357 * assign46320_e59374);
        let assign46320_e59376: f64 = (1.0 + assign46320_e59375);
        let assign46320_e59377: f64 = (1e-100 / assign46320_e59376);
        (assign46320_e59377, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46320_e59379;
        locals.var_tp_dn5 = assign46320_e59379_d_n5;
        locals.var_tp_dn6 = assign46320_e59379_d_n6;
        locals.var_tp_dn7 = assign46320_e59379_d_n7;
        locals.var_tp_dn8 = assign46320_e59379_d_n8;

        let (assign46330_e59387, assign46330_e59387_d_n5, assign46330_e59387_d_n6, assign46330_e59387_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46330_e59385: f64 = (3.0 + locals.var_xd_ov);
        (assign46330_e59385, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7,)
    }
};
        locals.var_fs1 = assign46330_e59387;
        locals.var_fs1_dn5 = assign46330_e59387_d_n5;
        locals.var_fs1_dn6 = assign46330_e59387_d_n6;
        locals.var_fs1_dn7 = assign46330_e59387_d_n7;

        let (assign46340_e59396,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46340_e59392: f64 = (-3.0);
        let assign46340_e59394: f64 = (assign46340_e59392 - locals.var_gco_i);
        (assign46340_e59394,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46340_e59396;

        let (assign46350_e59404, assign46350_e59404_d_n5, assign46350_e59404_d_n6, assign46350_e59404_d_n7,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46350_e59402: f64 = (30.0 * locals.var_vgdprime);
        (assign46350_e59402, (30.0 * locals.var_vgdprime_dn5), (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7,)
    }
};
        locals.var_fs3 = assign46350_e59404;
        locals.var_fs3_dn5 = assign46350_e59404_d_n5;
        locals.var_fs3_dn6 = assign46350_e59404_d_n6;
        locals.var_fs3_dn7 = assign46350_e59404_d_n7;

        let (assign46360_e59412,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46360_e59410: f64 = (4.0 - 0.9);
        (assign46360_e59410,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46360_e59412;

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign46370_e59420, assign46370_e59420_d_n5, assign46370_e59420_d_n6, assign46370_e59420_d_n7, assign46370_e59420_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46370_e59418: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46370_e59418, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46370_e59420;
        locals.var_tme2_dn5 = assign46370_e59420_d_n5;
        locals.var_tme2_dn6 = assign46370_e59420_d_n6;
        locals.var_tme2_dn7 = assign46370_e59420_d_n7;
        locals.var_tme2_dn8 = assign46370_e59420_d_n8;

        let (assign46380_e59441, assign46380_e59441_d_n5, assign46380_e59441_d_n6, assign46380_e59441_d_n7, assign46380_e59441_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46380_e59426: f64 = (2.0 / locals.var_tme1);
        let assign46380_e59430: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46380_e59433: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46380_e59435: f64 = (assign46380_e59433 * locals.var_fs3);
        let assign46380_e59436: f64 = (assign46380_e59430 - assign46380_e59435);
        let assign46380_e59437: f64 = (assign46380_e59436).sqrt();
        let assign46380_e59438: f64 = (locals.var_tme2 - assign46380_e59437);
        let assign46380_e59439: f64 = (assign46380_e59426 * assign46380_e59438);
        (assign46380_e59439, (assign46380_e59426 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn5))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn6))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn7))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46380_e59437)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46380_e59441;
        locals.var_temp__blk936_dn5 = assign46380_e59441_d_n5;
        locals.var_temp__blk936_dn6 = assign46380_e59441_d_n6;
        locals.var_temp__blk936_dn7 = assign46380_e59441_d_n7;
        locals.var_temp__blk936_dn8 = assign46380_e59441_d_n8;

        let (assign46390_e59449,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46390_e59447: f64 = (4.0 - 0.3);
        (assign46390_e59447,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46390_e59449;

        let (assign46400_e59457, assign46400_e59457_d_n5, assign46400_e59457_d_n6, assign46400_e59457_d_n7, assign46400_e59457_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46400_e59455: f64 = (locals.var_fs2 + locals.var_temp__blk936);
        (assign46400_e59455, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8,)
    }
};
        locals.var_tme2 = assign46400_e59457;
        locals.var_tme2_dn5 = assign46400_e59457_d_n5;
        locals.var_tme2_dn6 = assign46400_e59457_d_n6;
        locals.var_tme2_dn7 = assign46400_e59457_d_n7;
        locals.var_tme2_dn8 = assign46400_e59457_d_n8;

        let (assign46410_e59478, assign46410_e59478_d_n5, assign46410_e59478_d_n6, assign46410_e59478_d_n7, assign46410_e59478_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46410_e59463: f64 = (2.0 / locals.var_tme1);
        let assign46410_e59467: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46410_e59470: f64 = (locals.var_tme1 * locals.var_fs2);
        let assign46410_e59472: f64 = (assign46410_e59470 * locals.var_temp__blk936);
        let assign46410_e59473: f64 = (assign46410_e59467 - assign46410_e59472);
        let assign46410_e59474: f64 = (assign46410_e59473).sqrt();
        let assign46410_e59475: f64 = (locals.var_tme2 + assign46410_e59474);
        let assign46410_e59476: f64 = (assign46410_e59463 * assign46410_e59475);
        (assign46410_e59476, (assign46410_e59463 * (locals.var_tme2_dn5 + ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (assign46410_e59470 * locals.var_temp__blk936_dn5)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46410_e59470 * locals.var_temp__blk936_dn6)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46410_e59470 * locals.var_temp__blk936_dn7)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46410_e59470 * locals.var_temp__blk936_dn8)) / (2.0 * assign46410_e59474)))),)
    } else {
        (locals.var_fs, locals.var_fs_dn5, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8,)
    }
};
        locals.var_fs = assign46410_e59478;
        locals.var_fs_dn5 = assign46410_e59478_d_n5;
        locals.var_fs_dn6 = assign46410_e59478_d_n6;
        locals.var_fs_dn7 = assign46410_e59478_d_n7;
        locals.var_fs_dn8 = assign46410_e59478_d_n8;

        let (assign46420_e59488, assign46420_e59488_d_n5, assign46420_e59488_d_n6, assign46420_e59488_d_n7, assign46420_e59488_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign46420_e59485: f64 = (locals.var_tp * locals.var_fs);
        let assign46420_e59486: f64 = (locals.var_igovd_i * assign46420_e59485);
        (assign46420_e59486, (locals.var_igovd_i * ((locals.var_tp_dn5 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn5))), (locals.var_igovd_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igovd_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igovd_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))),)
    } else {
        (locals.var_igdov, locals.var_igdov_dn5, locals.var_igdov_dn6, locals.var_igdov_dn7, locals.var_igdov_dn8,)
    }
};
        locals.var_igdov = assign46420_e59488;
        locals.var_igdov_dn5 = assign46420_e59488_d_n5;
        locals.var_igdov_dn6 = assign46420_e59488_d_n6;
        locals.var_igdov_dn7 = assign46420_e59488_d_n7;
        locals.var_igdov_dn8 = assign46420_e59488_d_n8;

        let assign46430_e59491: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign46430_e59491;

        let assign46440_e59494: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign46440_e59494;

        let (assign46450_e59504, assign46450_e59504_d_n5, assign46450_e59504_d_n6, assign46450_e59504_d_n7, assign46450_e59504_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46450_e59502: f64 = (1.0 + locals.var_ar);
        (assign46450_e59502, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46450_e59504;
        locals.var_temp__blk936_dn5 = assign46450_e59504_d_n5;
        locals.var_temp__blk936_dn6 = assign46450_e59504_d_n6;
        locals.var_temp__blk936_dn7 = assign46450_e59504_d_n7;
        locals.var_temp__blk936_dn8 = assign46450_e59504_d_n8;

        let (assign46460_e59517, assign46460_e59517_d_n5, assign46460_e59517_d_n6, assign46460_e59517_d_n7, assign46460_e59517_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46460_e59511: f64 = (locals.var_temp__blk936).sqrt();
        let assign46460_e59513: f64 = (assign46460_e59511 * locals.var_v_ds);
        let assign46460_e59515: f64 = (assign46460_e59513 / locals.var_vdsat_lim_dc);
        (assign46460_e59515, (((((locals.var_temp__blk936_dn5 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn5)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn6)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign46460_e59517;
        locals.var_temp1_dn5 = assign46460_e59517_d_n5;
        locals.var_temp1_dn6 = assign46460_e59517_d_n6;
        locals.var_temp1_dn7 = assign46460_e59517_d_n7;
        locals.var_temp1_dn8 = assign46460_e59517_d_n8;

        let (assign46470_e59529, assign46470_e59529_d_n5, assign46470_e59529_d_n6, assign46470_e59529_d_n7, assign46470_e59529_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46470_e59525: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign46470_e59527: f64 = (assign46470_e59525 + locals.var_temp__blk936);
        (assign46470_e59527, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign46470_e59529;
        locals.var_temp2_dn5 = assign46470_e59529_d_n5;
        locals.var_temp2_dn6 = assign46470_e59529_d_n6;
        locals.var_temp2_dn7 = assign46470_e59529_d_n7;
        locals.var_temp2_dn8 = assign46470_e59529_d_n8;

        let (assign46480_e59539, assign46480_e59539_d_n5, assign46480_e59539_d_n6, assign46480_e59539_d_n7, assign46480_e59539_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46480_e59537: f64 = (2.0 * locals.var_temp1);
        (assign46480_e59537, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46480_e59539;
        locals.var_temp__blk936_dn5 = assign46480_e59539_d_n5;
        locals.var_temp__blk936_dn6 = assign46480_e59539_d_n6;
        locals.var_temp__blk936_dn7 = assign46480_e59539_d_n7;
        locals.var_temp__blk936_dn8 = assign46480_e59539_d_n8;

        let (assign46490_e59561, assign46490_e59561_d_n5, assign46490_e59561_d_n6, assign46490_e59561_d_n7, assign46490_e59561_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign46490_e59547: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
        let assign46490_e59549: f64 = (assign46490_e59547 * locals.var_temp__blk936);
        let assign46490_e59552: f64 = (locals.var_temp2 - locals.var_temp__blk936);
        let assign46490_e59553: f64 = (assign46490_e59552).sqrt();
        let assign46490_e59556: f64 = (locals.var_temp2 + locals.var_temp__blk936);
        let assign46490_e59557: f64 = (assign46490_e59556).sqrt();
        let assign46490_e59558: f64 = (assign46490_e59553 + assign46490_e59557);
        let assign46490_e59559: f64 = (assign46490_e59549 / assign46490_e59558);
        (assign46490_e59559, (((((((locals.var_vdsat_lim_dc_dn5 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn5)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn5)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn6)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn7)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn8)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)),)
    } else {
        (locals.var_udse_dc, locals.var_udse_dc_dn5, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8,)
    }
};
        locals.var_udse_dc = assign46490_e59561;
        locals.var_udse_dc_dn5 = assign46490_e59561_d_n5;
        locals.var_udse_dc_dn6 = assign46490_e59561_d_n6;
        locals.var_udse_dc_dn7 = assign46490_e59561_d_n7;
        locals.var_udse_dc_dn8 = assign46490_e59561_d_n8;

        let assign46500_e59564: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46500_e59566: f64 = (-230.25850929940458);
        let assign46500_e59567: f64 = if assign46500_e59564 > assign46500_e59566 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign46500_e59567;

        let (assign46510_e59578, assign46510_e59578_d_n5, assign46510_e59578_d_n6, assign46510_e59578_d_n7, assign46510_e59578_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign46510_e59575: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46510_e59576: f64 = (assign46510_e59575).exp();
        (assign46510_e59576, (assign46510_e59576 * (locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)), (assign46510_e59576 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46510_e59576 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46510_e59576 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46510_e59578;
        locals.var_temp__blk936_dn5 = assign46510_e59578_d_n5;
        locals.var_temp__blk936_dn6 = assign46510_e59578_d_n6;
        locals.var_temp__blk936_dn7 = assign46510_e59578_d_n7;
        locals.var_temp__blk936_dn8 = assign46510_e59578_d_n8;

        let (assign46520_e59618, assign46520_e59618_d_n5, assign46520_e59618_d_n6, assign46520_e59618_d_n7, assign46520_e59618_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 == 0.0)) {
        let assign46520_e59588: f64 = (-230.25850929940458);
        let assign46520_e59591: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59592: f64 = (assign46520_e59588 - assign46520_e59591);
        let assign46520_e59596: f64 = (-230.25850929940458);
        let assign46520_e59599: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59600: f64 = (assign46520_e59596 - assign46520_e59599);
        let assign46520_e59603: f64 = (-230.25850929940458);
        let assign46520_e59606: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46520_e59607: f64 = (assign46520_e59603 - assign46520_e59606);
        let assign46520_e59609: f64 = (assign46520_e59607 * 0.3333333333333333);
        let assign46520_e59610: f64 = (1.0 + assign46520_e59609);
        let assign46520_e59611: f64 = (assign46520_e59600 * assign46520_e59610);
        let assign46520_e59612: f64 = (0.5 * assign46520_e59611);
        let assign46520_e59613: f64 = (1.0 + assign46520_e59612);
        let assign46520_e59614: f64 = (assign46520_e59592 * assign46520_e59613);
        let assign46520_e59615: f64 = (1.0 + assign46520_e59614);
        let assign46520_e59616: f64 = (1e-100 / assign46520_e59615);
        (assign46520_e59616, (-((1e-100 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46520_e59618;
        locals.var_temp__blk936_dn5 = assign46520_e59618_d_n5;
        locals.var_temp__blk936_dn6 = assign46520_e59618_d_n6;
        locals.var_temp__blk936_dn7 = assign46520_e59618_d_n7;
        locals.var_temp__blk936_dn8 = assign46520_e59618_d_n8;

        let (assign46530_e59637, assign46530_e59637_d_n5, assign46530_e59637_d_n6, assign46530_e59637_d_n7, assign46530_e59637_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46530_e59626: f64 = (0.5 * locals.var_x_ds_dc);
        let assign46530_e59630: f64 = (1.0 + locals.var_temp__blk936);
        let assign46530_e59631: f64 = (0.5 * assign46530_e59630);
        let assign46530_e59632: f64 = (assign46530_e59631).ln();
        let assign46530_e59633: f64 = (assign46530_e59626 - assign46530_e59632);
        let assign46530_e59634: f64 = (locals.var_phit1_dc * assign46530_e59633);
        let assign46530_e59635: f64 = (locals.var_vsbstar_dc + assign46530_e59634);
        (assign46530_e59635, (locals.var_vsbstar_dc_dn5 + ((locals.var_phit1_dc_dn5 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn5) - ((0.5 * locals.var_temp__blk936_dn5) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk936_dn6) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk936_dn7) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk936_dn8) / assign46530_e59631))))),)
    } else {
        (locals.var_vm, locals.var_vm_dn5, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8,)
    }
};
        locals.var_vm = assign46530_e59637;
        locals.var_vm_dn5 = assign46530_e59637_d_n5;
        locals.var_vm_dn6 = assign46530_e59637_d_n6;
        locals.var_vm_dn7 = assign46530_e59637_d_n7;
        locals.var_vm_dn8 = assign46530_e59637_d_n8;

        let (assign46540_e59645, assign46540_e59645_d_n5, assign46540_e59645_d_n6, assign46540_e59645_d_n7, assign46540_e59645_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46540_e59643: f64 = (locals.var_gco_i * locals.var_phit1_dc);
        (assign46540_e59643, (locals.var_gco_i * locals.var_phit1_dc_dn5), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8),)
    } else {
        (locals.var_dch, locals.var_dch_dn5, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8,)
    }
};
        locals.var_dch = assign46540_e59645;
        locals.var_dch_dn5 = assign46540_e59645_d_n5;
        locals.var_dch_dn6 = assign46540_e59645_d_n6;
        locals.var_dch_dn7 = assign46540_e59645_d_n7;
        locals.var_dch_dn8 = assign46540_e59645_d_n8;

        let (assign46550_e59653, assign46550_e59653_d_n5, assign46550_e59653_d_n6, assign46550_e59653_d_n7, assign46550_e59653_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46550_e59651: f64 = (locals.var_voxm_dc + locals.var_dch);
        (assign46550_e59651, (locals.var_voxm_dc_dn5 + locals.var_dch_dn5), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn5, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8,)
    }
};
        locals.var_arg2mina = assign46550_e59653;
        locals.var_arg2mina_dn5 = assign46550_e59653_d_n5;
        locals.var_arg2mina_dn6 = assign46550_e59653_d_n6;
        locals.var_arg2mina_dn7 = assign46550_e59653_d_n7;
        locals.var_arg2mina_dn8 = assign46550_e59653_d_n8;

        let (assign46560_e59674, assign46560_e59674_d_n5, assign46560_e59674_d_n6, assign46560_e59674_d_n7, assign46560_e59674_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46560_e59660: f64 = locals.var_arg2mina;
        let assign46560_e59663: f64 = (-locals.var_arg2mina);
        let assign46560_e59666: f64 = (-locals.var_arg2mina);
        let assign46560_e59667: f64 = (assign46560_e59663 * assign46560_e59666);
        let assign46560_e59669: f64 = (assign46560_e59667 + 0.01);
        let assign46560_e59670: f64 = (assign46560_e59669).sqrt();
        let assign46560_e59671: f64 = (assign46560_e59660 - assign46560_e59670);
        let assign46560_e59672: f64 = (0.5 * assign46560_e59671);
        (assign46560_e59672, (0.5 * (locals.var_arg2mina_dn5 - ((((-locals.var_arg2mina_dn5) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn5))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46560_e59670)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn5, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8,)
    }
};
        locals.var_psi_t = assign46560_e59674;
        locals.var_psi_t_dn5 = assign46560_e59674_d_n5;
        locals.var_psi_t_dn6 = assign46560_e59674_d_n6;
        locals.var_psi_t_dn7 = assign46560_e59674_d_n7;
        locals.var_psi_t_dn8 = assign46560_e59674_d_n8;

        let (assign46570_e59687, assign46570_e59687_d_n5, assign46570_e59687_d_n6, assign46570_e59687_d_n7, assign46570_e59687_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46570_e59680: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
        let assign46570_e59682: f64 = (assign46570_e59680 + 1e-6);
        let assign46570_e59683: f64 = (assign46570_e59682).sqrt();
        let assign46570_e59685: f64 = (assign46570_e59683 * locals.var_inv_chib);
        (assign46570_e59685, ((((locals.var_voxm_dc_dn5 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn5)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46570_e59687;
        locals.var_zg_dn5 = assign46570_e59687_d_n5;
        locals.var_zg_dn6 = assign46570_e59687_d_n6;
        locals.var_zg_dn7 = assign46570_e59687_d_n7;
        locals.var_zg_dn8 = assign46570_e59687_d_n8;

        let assign46580_e59690: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign46580_e59690;

        let (assign46590_e59713, assign46590_e59713_d_n5, assign46590_e59713_d_n6, assign46590_e59713_d_n7, assign46590_e59713_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign46590_e59699: f64 = (locals.var_zg + locals.var_gcq);
        let assign46590_e59702: f64 = (locals.var_zg - locals.var_gcq);
        let assign46590_e59705: f64 = (locals.var_zg - locals.var_gcq);
        let assign46590_e59706: f64 = (assign46590_e59702 * assign46590_e59705);
        let assign46590_e59708: f64 = (assign46590_e59706 + 1e-6);
        let assign46590_e59709: f64 = (assign46590_e59708).sqrt();
        let assign46590_e59710: f64 = (assign46590_e59699 - assign46590_e59709);
        let assign46590_e59711: f64 = (0.5 * assign46590_e59710);
        (assign46590_e59711, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn5)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn6)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn7)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn8)) / (2.0 * assign46590_e59709)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8,)
    }
};
        locals.var_zg = assign46590_e59713;
        locals.var_zg_dn5 = assign46590_e59713_d_n5;
        locals.var_zg_dn6 = assign46590_e59713_d_n6;
        locals.var_zg_dn7 = assign46590_e59713_d_n7;
        locals.var_zg_dn8 = assign46590_e59713_d_n8;

        let (assign46600_e59727, assign46600_e59727_d_n5, assign46600_e59727_d_n6, assign46600_e59727_d_n7, assign46600_e59727_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46600_e59720: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign46600_e59722: f64 = (assign46600_e59720 - locals.var_vm);
        let assign46600_e59724: f64 = (assign46600_e59722 * locals.var_inv_phit1_dc);
        let assign46600_e59725: f64 = (locals.var_x_m_dc + assign46600_e59724);
        (assign46600_e59725, (locals.var_x_m_dc_dn5 + (((locals.var_psi_t_dn5 - locals.var_vm_dn5) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn5))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn8))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8,)
    }
};
        locals.var_arg1 = assign46600_e59727;
        locals.var_arg1_dn5 = assign46600_e59727_d_n5;
        locals.var_arg1_dn6 = assign46600_e59727_d_n6;
        locals.var_arg1_dn7 = assign46600_e59727_d_n7;
        locals.var_arg1_dn8 = assign46600_e59727_d_n8;

        let assign46610_e59729: f64 = (locals.var_arg1).abs();
        let assign46610_e59731: f64 = if assign46610_e59729 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign46610_e59731;

        let (assign46620_e59740, assign46620_e59740_d_n5, assign46620_e59740_d_n6, assign46620_e59740_d_n7, assign46620_e59740_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 != 0.0)) {
        let assign46620_e59738: f64 = (locals.var_arg1).exp();
        (assign46620_e59738, (assign46620_e59738 * locals.var_arg1_dn5), (assign46620_e59738 * locals.var_arg1_dn6), (assign46620_e59738 * locals.var_arg1_dn7), (assign46620_e59738 * locals.var_arg1_dn8),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8,)
    }
};
        locals.var_dsi = assign46620_e59740;
        locals.var_dsi_dn5 = assign46620_e59740_d_n5;
        locals.var_dsi_dn6 = assign46620_e59740_d_n6;
        locals.var_dsi_dn7 = assign46620_e59740_d_n7;
        locals.var_dsi_dn8 = assign46620_e59740_d_n8;

        let assign46630_e59743: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign46630_e59743;

        let (assign46640_e59779, assign46640_e59779_d_n5, assign46640_e59779_d_n6, assign46640_e59779_d_n7, assign46640_e59779_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 == 0.0)) && (locals.var_guard1235 != 0.0)) {
        let assign46640_e59755: f64 = (-230.25850929940458);
        let assign46640_e59757: f64 = (assign46640_e59755 - locals.var_arg1);
        let assign46640_e59761: f64 = (-230.25850929940458);
        let assign46640_e59763: f64 = (assign46640_e59761 - locals.var_arg1);
        let assign46640_e59766: f64 = (-230.25850929940458);
        let assign46640_e59768: f64 = (assign46640_e59766 - locals.var_arg1);
        let assign46640_e59770: f64 = (assign46640_e59768 * 0.3333333333333333);
        let assign46640_e59771: f64 = (1.0 + assign46640_e59770);
        let assign46640_e59772: f64 = (assign46640_e59763 * assign46640_e59771);
        let assign46640_e59773: f64 = (0.5 * assign46640_e59772);
        let assign46640_e59774: f64 = (1.0 + assign46640_e59773);
        let assign46640_e59775: f64 = (assign46640_e59757 * assign46640_e59774);
        let assign46640_e59776: f64 = (1.0 + assign46640_e59775);
        let assign46640_e59777: f64 = (1e-100 / assign46640_e59776);
        (assign46640_e59777, (-((1e-100 * (((-locals.var_arg1_dn5) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn5) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn5) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn6) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn7) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn8) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8,)
    }
};
        locals.var_dsi = assign46640_e59779;
        locals.var_dsi_dn5 = assign46640_e59779_d_n5;
        locals.var_dsi_dn6 = assign46640_e59779_d_n6;
        locals.var_dsi_dn7 = assign46640_e59779_d_n7;
        locals.var_dsi_dn8 = assign46640_e59779_d_n8;

        let (assign46650_e59813, assign46650_e59813_d_n5, assign46650_e59813_d_n6, assign46650_e59813_d_n7, assign46650_e59813_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 == 0.0)) && (locals.var_guard1235 == 0.0)) {
        let assign46650_e59793: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46650_e59798: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46650_e59802: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46650_e59804: f64 = (assign46650_e59802 * 0.3333333333333333);
        let assign46650_e59805: f64 = (1.0 + assign46650_e59804);
        let assign46650_e59806: f64 = (assign46650_e59798 * assign46650_e59805);
        let assign46650_e59807: f64 = (0.5 * assign46650_e59806);
        let assign46650_e59808: f64 = (1.0 + assign46650_e59807);
        let assign46650_e59809: f64 = (assign46650_e59793 * assign46650_e59808);
        let assign46650_e59810: f64 = (1.0 + assign46650_e59809);
        let assign46650_e59811: f64 = (1e100 * assign46650_e59810);
        (assign46650_e59811, (1e100 * ((locals.var_arg1_dn5 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn5 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn6 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn7 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn8 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8,)
    }
};
        locals.var_dsi = assign46650_e59813;
        locals.var_dsi_dn5 = assign46650_e59813_d_n5;
        locals.var_dsi_dn6 = assign46650_e59813_d_n6;
        locals.var_dsi_dn7 = assign46650_e59813_d_n7;
        locals.var_dsi_dn8 = assign46650_e59813_d_n8;

        let (assign46660_e59826, assign46660_e59826_d_n5, assign46660_e59826_d_n6, assign46660_e59826_d_n7, assign46660_e59826_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46660_e59819: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
        let assign46660_e59821: f64 = (assign46660_e59819 - locals.var_vm);
        let assign46660_e59822: f64 = (-assign46660_e59821);
        let assign46660_e59824: f64 = (assign46660_e59822 * locals.var_inv_phit1_dc);
        (assign46660_e59824, (((-((locals.var_v_gs_dn5 + locals.var_vsbstar_dc_dn5) - locals.var_vm_dn5)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn5)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn7)), (((-(locals.var_vsbstar_dc_dn8 - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn8)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8,)
    }
};
        locals.var_arg1 = assign46660_e59826;
        locals.var_arg1_dn5 = assign46660_e59826_d_n5;
        locals.var_arg1_dn6 = assign46660_e59826_d_n6;
        locals.var_arg1_dn7 = assign46660_e59826_d_n7;
        locals.var_arg1_dn8 = assign46660_e59826_d_n8;

        let assign46670_e59828: f64 = (locals.var_arg1).abs();
        let assign46670_e59830: f64 = if assign46670_e59828 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign46670_e59830;

        let (assign46680_e59839, assign46680_e59839_d_n5, assign46680_e59839_d_n6, assign46680_e59839_d_n7, assign46680_e59839_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 != 0.0)) {
        let assign46680_e59837: f64 = (locals.var_arg1).exp();
        (assign46680_e59837, (assign46680_e59837 * locals.var_arg1_dn5), (assign46680_e59837 * locals.var_arg1_dn6), (assign46680_e59837 * locals.var_arg1_dn7), (assign46680_e59837 * locals.var_arg1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46680_e59839;
        locals.var_temp__blk936_dn5 = assign46680_e59839_d_n5;
        locals.var_temp__blk936_dn6 = assign46680_e59839_d_n6;
        locals.var_temp__blk936_dn7 = assign46680_e59839_d_n7;
        locals.var_temp__blk936_dn8 = assign46680_e59839_d_n8;

        let assign46690_e59842: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign46690_e59842;

        let (assign46700_e59878, assign46700_e59878_d_n5, assign46700_e59878_d_n6, assign46700_e59878_d_n7, assign46700_e59878_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 != 0.0)) {
        let assign46700_e59854: f64 = (-230.25850929940458);
        let assign46700_e59856: f64 = (assign46700_e59854 - locals.var_arg1);
        let assign46700_e59860: f64 = (-230.25850929940458);
        let assign46700_e59862: f64 = (assign46700_e59860 - locals.var_arg1);
        let assign46700_e59865: f64 = (-230.25850929940458);
        let assign46700_e59867: f64 = (assign46700_e59865 - locals.var_arg1);
        let assign46700_e59869: f64 = (assign46700_e59867 * 0.3333333333333333);
        let assign46700_e59870: f64 = (1.0 + assign46700_e59869);
        let assign46700_e59871: f64 = (assign46700_e59862 * assign46700_e59870);
        let assign46700_e59872: f64 = (0.5 * assign46700_e59871);
        let assign46700_e59873: f64 = (1.0 + assign46700_e59872);
        let assign46700_e59874: f64 = (assign46700_e59856 * assign46700_e59873);
        let assign46700_e59875: f64 = (1.0 + assign46700_e59874);
        let assign46700_e59876: f64 = (1e-100 / assign46700_e59875);
        (assign46700_e59876, (-((1e-100 * (((-locals.var_arg1_dn5) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn5) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn5) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn6) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn7) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn8) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46700_e59878;
        locals.var_temp__blk936_dn5 = assign46700_e59878_d_n5;
        locals.var_temp__blk936_dn6 = assign46700_e59878_d_n6;
        locals.var_temp__blk936_dn7 = assign46700_e59878_d_n7;
        locals.var_temp__blk936_dn8 = assign46700_e59878_d_n8;

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign46710_e59912, assign46710_e59912_d_n5, assign46710_e59912_d_n6, assign46710_e59912_d_n7, assign46710_e59912_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 == 0.0)) {
        let assign46710_e59892: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59897: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59901: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46710_e59903: f64 = (assign46710_e59901 * 0.3333333333333333);
        let assign46710_e59904: f64 = (1.0 + assign46710_e59903);
        let assign46710_e59905: f64 = (assign46710_e59897 * assign46710_e59904);
        let assign46710_e59906: f64 = (0.5 * assign46710_e59905);
        let assign46710_e59907: f64 = (1.0 + assign46710_e59906);
        let assign46710_e59908: f64 = (assign46710_e59892 * assign46710_e59907);
        let assign46710_e59909: f64 = (1.0 + assign46710_e59908);
        let assign46710_e59910: f64 = (1e100 * assign46710_e59909);
        (assign46710_e59910, (1e100 * ((locals.var_arg1_dn5 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn5 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn6 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn7 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn8 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46710_e59912;
        locals.var_temp__blk936_dn5 = assign46710_e59912_d_n5;
        locals.var_temp__blk936_dn6 = assign46710_e59912_d_n6;
        locals.var_temp__blk936_dn7 = assign46710_e59912_d_n7;
        locals.var_temp__blk936_dn8 = assign46710_e59912_d_n8;

        let (assign46720_e59920, assign46720_e59920_d_n5, assign46720_e59920_d_n6, assign46720_e59920_d_n7, assign46720_e59920_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46720_e59918: f64 = (locals.var_dsi * locals.var_temp__blk936);
        (assign46720_e59918, ((locals.var_dsi_dn5 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn5)), ((locals.var_dsi_dn6 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn6)), ((locals.var_dsi_dn7 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn7)), ((locals.var_dsi_dn8 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn5, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8,)
    }
};
        locals.var_dgate = assign46720_e59920;
        locals.var_dgate_dn5 = assign46720_e59920_d_n5;
        locals.var_dgate_dn6 = assign46720_e59920_d_n6;
        locals.var_dgate_dn7 = assign46720_e59920_d_n7;
        locals.var_dgate_dn8 = assign46720_e59920_d_n8;

        let (assign46730_e59937, assign46730_e59937_d_n5, assign46730_e59937_d_n6, assign46730_e59937_d_n7, assign46730_e59937_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46730_e59926: f64 = (-1.5);
        let assign46730_e59931: f64 = (locals.var_gc3_i * locals.var_zg);
        let assign46730_e59932: f64 = (locals.var_gc2_i + assign46730_e59931);
        let assign46730_e59933: f64 = (locals.var_zg * assign46730_e59932);
        let assign46730_e59934: f64 = (assign46730_e59926 + assign46730_e59933);
        let assign46730_e59935: f64 = (locals.var_bch * assign46730_e59934);
        (assign46730_e59935, (locals.var_bch * ((locals.var_zg_dn5 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn5)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46730_e59937;
        locals.var_temp__blk936_dn5 = assign46730_e59937_d_n5;
        locals.var_temp__blk936_dn6 = assign46730_e59937_d_n6;
        locals.var_temp__blk936_dn7 = assign46730_e59937_d_n7;
        locals.var_temp__blk936_dn8 = assign46730_e59937_d_n8;

        let assign46740_e59940: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1238 = assign46740_e59940;

        let (assign46750_e59962, assign46750_e59962_d_n5, assign46750_e59962_d_n6, assign46750_e59962_d_n7, assign46750_e59962_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 != 0.0)) {
        let assign46750_e59954: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
        let assign46750_e59955: f64 = (1.0 + assign46750_e59954);
        let assign46750_e59956: f64 = (locals.var_temp__blk936 * assign46750_e59955);
        let assign46750_e59957: f64 = (0.5 * assign46750_e59956);
        let assign46750_e59958: f64 = (1.0 + assign46750_e59957);
        let assign46750_e59959: f64 = (locals.var_temp__blk936 * assign46750_e59958);
        let assign46750_e59960: f64 = (1.0 + assign46750_e59959);
        (assign46750_e59960, ((locals.var_temp__blk936_dn5 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46750_e59962;
        locals.var_tp_dn5 = assign46750_e59962_d_n5;
        locals.var_tp_dn6 = assign46750_e59962_d_n6;
        locals.var_tp_dn7 = assign46750_e59962_d_n7;
        locals.var_tp_dn8 = assign46750_e59962_d_n8;

        let assign46760_e59965: f64 = (-230.25850929940458);
        let assign46760_e59966: f64 = if locals.var_temp__blk936 > assign46760_e59965 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign46760_e59966;

        let (assign46770_e59978, assign46770_e59978_d_n5, assign46770_e59978_d_n6, assign46770_e59978_d_n7, assign46770_e59978_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 == 0.0)) && (locals.var_guard1239 != 0.0)) {
        let assign46770_e59976: f64 = (locals.var_temp__blk936).exp();
        (assign46770_e59976, (assign46770_e59976 * locals.var_temp__blk936_dn5), (assign46770_e59976 * locals.var_temp__blk936_dn6), (assign46770_e59976 * locals.var_temp__blk936_dn7), (assign46770_e59976 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46770_e59978;
        locals.var_tp_dn5 = assign46770_e59978_d_n5;
        locals.var_tp_dn6 = assign46770_e59978_d_n6;
        locals.var_tp_dn7 = assign46770_e59978_d_n7;
        locals.var_tp_dn8 = assign46770_e59978_d_n8;

        let (assign46780_e60015, assign46780_e60015_d_n5, assign46780_e60015_d_n6, assign46780_e60015_d_n7, assign46780_e60015_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 == 0.0)) && (locals.var_guard1239 == 0.0)) {
        let assign46780_e59991: f64 = (-230.25850929940458);
        let assign46780_e59993: f64 = (assign46780_e59991 - locals.var_temp__blk936);
        let assign46780_e59997: f64 = (-230.25850929940458);
        let assign46780_e59999: f64 = (assign46780_e59997 - locals.var_temp__blk936);
        let assign46780_e60002: f64 = (-230.25850929940458);
        let assign46780_e60004: f64 = (assign46780_e60002 - locals.var_temp__blk936);
        let assign46780_e60006: f64 = (assign46780_e60004 * 0.3333333333333333);
        let assign46780_e60007: f64 = (1.0 + assign46780_e60006);
        let assign46780_e60008: f64 = (assign46780_e59999 * assign46780_e60007);
        let assign46780_e60009: f64 = (0.5 * assign46780_e60008);
        let assign46780_e60010: f64 = (1.0 + assign46780_e60009);
        let assign46780_e60011: f64 = (assign46780_e59993 * assign46780_e60010);
        let assign46780_e60012: f64 = (1.0 + assign46780_e60011);
        let assign46780_e60013: f64 = (1e-100 / assign46780_e60012);
        (assign46780_e60013, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))),)
    } else {
        (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8,)
    }
};
        locals.var_tp = assign46780_e60015;
        locals.var_tp_dn5 = assign46780_e60015_d_n5;
        locals.var_tp_dn6 = assign46780_e60015_d_n6;
        locals.var_tp_dn7 = assign46780_e60015_d_n7;
        locals.var_tp_dn8 = assign46780_e60015_d_n8;

        let (assign46790_e60032, assign46790_e60032_d_n5, assign46790_e60032_d_n6, assign46790_e60032_d_n7, assign46790_e60032_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign46790_e60023: f64 = (1.0 + locals.var_dsi);
        let assign46790_e60026: f64 = (1.0 + locals.var_dgate);
        let assign46790_e60027: f64 = (assign46790_e60023 / assign46790_e60026);
        let assign46790_e60028: f64 = (assign46790_e60027).ln();
        let assign46790_e60029: f64 = (locals.var_tp * assign46790_e60028);
        let assign46790_e60030: f64 = (locals.var_iginv_i * assign46790_e60029);
        (assign46790_e60030, (locals.var_iginv_i * ((locals.var_tp_dn5 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn5 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn5)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn6 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn6 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn6)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn7 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn7 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn7)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn8 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn8 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn8)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))),)
    } else {
        (locals.var_igc0, locals.var_igc0_dn5, locals.var_igc0_dn6, locals.var_igc0_dn7, locals.var_igc0_dn8,)
    }
};
        locals.var_igc0 = assign46790_e60032;
        locals.var_igc0_dn5 = assign46790_e60032_d_n5;
        locals.var_igc0_dn6 = assign46790_e60032_d_n6;
        locals.var_igc0_dn7 = assign46790_e60032_d_n7;
        locals.var_igc0_dn8 = assign46790_e60032_d_n8;

        let assign46800_e60043: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign46800_e60043;

        let (assign46810_e60051, assign46810_e60051_d_n5, assign46810_e60051_d_n6, assign46810_e60051_d_n7, assign46810_e60051_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8,)
    }
};
        locals.var_igc = assign46810_e60051;
        locals.var_igc_dn5 = assign46810_e60051_d_n5;
        locals.var_igc_dn6 = assign46810_e60051_d_n6;
        locals.var_igc_dn7 = assign46810_e60051_d_n7;
        locals.var_igc_dn8 = assign46810_e60051_d_n8;

        let (assign46820_e60059, assign46820_e60059_d_n5, assign46820_e60059_d_n6, assign46820_e60059_d_n7, assign46820_e60059_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8,)
    }
};
        locals.var_igcd_h = assign46820_e60059;
        locals.var_igcd_h_dn5 = assign46820_e60059_d_n5;
        locals.var_igcd_h_dn6 = assign46820_e60059_d_n6;
        locals.var_igcd_h_dn7 = assign46820_e60059_d_n7;
        locals.var_igcd_h_dn8 = assign46820_e60059_d_n8;

        let (assign46830_e60074, assign46830_e60074_d_n5, assign46830_e60074_d_n6, assign46830_e60074_d_n7, assign46830_e60074_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46830_e60069: f64 = (2.0 * locals.var_gc3_i);
        let assign46830_e60071: f64 = (assign46830_e60069 * locals.var_zg);
        let assign46830_e60072: f64 = (locals.var_gc2_i + assign46830_e60071);
        (assign46830_e60072, (assign46830_e60069 * locals.var_zg_dn5), (assign46830_e60069 * locals.var_zg_dn6), (assign46830_e60069 * locals.var_zg_dn7), (assign46830_e60069 * locals.var_zg_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign46830_e60074;
        locals.var_temp__blk936_dn5 = assign46830_e60074_d_n5;
        locals.var_temp__blk936_dn6 = assign46830_e60074_d_n6;
        locals.var_temp__blk936_dn7 = assign46830_e60074_d_n7;
        locals.var_temp__blk936_dn8 = assign46830_e60074_d_n8;

        let (assign46840_e60087, assign46840_e60087_d_n5, assign46840_e60087_d_n6, assign46840_e60087_d_n7, assign46840_e60087_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46840_e60084: f64 = (locals.var_temp__blk936 * locals.var_bch);
        let assign46840_e60085: f64 = (locals.var_chib_i / assign46840_e60084);
        (assign46840_e60085, (-((locals.var_chib_i * (locals.var_temp__blk936_dn5 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn6 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn7 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn8 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))),)
    } else {
        (locals.var_u0, locals.var_u0_dn5, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8,)
    }
};
        locals.var_u0 = assign46840_e60087;
        locals.var_u0_dn5 = assign46840_e60087_d_n5;
        locals.var_u0_dn6 = assign46840_e60087_d_n6;
        locals.var_u0_dn7 = assign46840_e60087_d_n7;
        locals.var_u0_dn8 = assign46840_e60087_d_n8;

        let (assign46850_e60100, assign46850_e60100_d_n5, assign46850_e60100_d_n6, assign46850_e60100_d_n7, assign46850_e60100_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46850_e60097: f64 = (locals.var_dps_dc / locals.var_u0);
        let assign46850_e60098: f64 = (0.5 * assign46850_e60097);
        (assign46850_e60098, (0.5 * (((locals.var_dps_dc_dn5 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn5)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))),)
    } else {
        (locals.var_x, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8,)
    }
};
        locals.var_x = assign46850_e60100;
        locals.var_x_dn5 = assign46850_e60100_d_n5;
        locals.var_x_dn6 = assign46850_e60100_d_n6;
        locals.var_x_dn7 = assign46850_e60100_d_n7;
        locals.var_x_dn8 = assign46850_e60100_d_n8;

        let (assign46860_e60111, assign46860_e60111_d_n5, assign46860_e60111_d_n6, assign46860_e60111_d_n7, assign46860_e60111_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46860_e60109: f64 = (locals.var_u0 / locals.var_h_dc);
        (assign46860_e60109, (((locals.var_u0_dn5 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn5)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn6 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn7 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn8 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)),)
    } else {
        (locals.var_u0_div_h, locals.var_u0_div_h_dn5, locals.var_u0_div_h_dn6, locals.var_u0_div_h_dn7, locals.var_u0_div_h_dn8,)
    }
};
        locals.var_u0_div_h = assign46860_e60111;
        locals.var_u0_div_h_dn5 = assign46860_e60111_d_n5;
        locals.var_u0_div_h_dn6 = assign46860_e60111_d_n6;
        locals.var_u0_div_h_dn7 = assign46860_e60111_d_n7;
        locals.var_u0_div_h_dn8 = assign46860_e60111_d_n8;

        let (assign46870_e60126, assign46870_e60126_d_n5, assign46870_e60126_d_n6, assign46870_e60126_d_n7, assign46870_e60126_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46870_e60121: f64 = (1.0 - locals.var_u0_div_h);
        let assign46870_e60122: f64 = (locals.var_u0_div_h * assign46870_e60121);
        let assign46870_e60124: f64 = (assign46870_e60122 * 0.5);
        (assign46870_e60124, (((locals.var_u0_div_h_dn5 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn5))) * 0.5), (((locals.var_u0_div_h_dn6 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn6))) * 0.5), (((locals.var_u0_div_h_dn7 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn7))) * 0.5), (((locals.var_u0_div_h_dn8 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn8))) * 0.5),)
    } else {
        (locals.var_bg, locals.var_bg_dn5, locals.var_bg_dn6, locals.var_bg_dn7, locals.var_bg_dn8,)
    }
};
        locals.var_bg = assign46870_e60126;
        locals.var_bg_dn5 = assign46870_e60126_d_n5;
        locals.var_bg_dn6 = assign46870_e60126_d_n6;
        locals.var_bg_dn7 = assign46870_e60126_d_n7;
        locals.var_bg_dn8 = assign46870_e60126_d_n8;

        let (assign46880_e60139, assign46880_e60139_d_n5, assign46880_e60139_d_n6, assign46880_e60139_d_n7, assign46880_e60139_d_n8,) = {
    if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
        let assign46880_e60136: f64 = (3.0 * locals.var_bg);
        let assign46880_e60137: f64 = (0.5 - assign46880_e60136);
        (assign46880_e60137, (-(3.0 * locals.var_bg_dn5)), (-(3.0 * locals.var_bg_dn6)), (-(3.0 * locals.var_bg_dn7)), (-(3.0 * locals.var_bg_dn8)),)
    } else {
        (locals.var_ag, locals.var_ag_dn5, locals.var_ag_dn6, locals.var_ag_dn7, locals.var_ag_dn8,)
    }
};
        locals.var_ag = assign46880_e60139;
        locals.var_ag_dn5 = assign46880_e60139_d_n5;
        locals.var_ag_dn6 = assign46880_e60139_d_n6;
        locals.var_ag_dn7 = assign46880_e60139_d_n7;
        locals.var_ag_dn8 = assign46880_e60139_d_n8;

        let assign46890_e60142: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1241 = assign46890_e60142;

        let (assign46900_e60155, assign46900_e60155_d_n5, assign46900_e60155_d_n6, assign46900_e60155_d_n7, assign46900_e60155_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
        let assign46900_e60153: f64 = (locals.var_x * locals.var_x);
        (assign46900_e60153, ((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)), ((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)), ((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)), ((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)),)
    } else {
        (locals.var_xsq, locals.var_xsq_dn5, locals.var_xsq_dn6, locals.var_xsq_dn7, locals.var_xsq_dn8,)
    }
};
        locals.var_xsq = assign46900_e60155;
        locals.var_xsq_dn5 = assign46900_e60155_d_n5;
        locals.var_xsq_dn6 = assign46900_e60155_d_n6;
        locals.var_xsq_dn7 = assign46900_e60155_d_n7;
        locals.var_xsq_dn8 = assign46900_e60155_d_n8;

        let (assign46910_e60184, assign46910_e60184_d_n5, assign46910_e60184_d_n6, assign46910_e60184_d_n7, assign46910_e60184_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
        let assign46910_e60169: f64 = (locals.var_u0_div_h * 0.3333333333333333);
        let assign46910_e60170: f64 = (0.16666666666666666 + assign46910_e60169);
        let assign46910_e60176: f64 = (0.2 * locals.var_u0_div_h);
        let assign46910_e60177: f64 = (0.05 + assign46910_e60176);
        let assign46910_e60178: f64 = (locals.var_xsq * assign46910_e60177);
        let assign46910_e60179: f64 = (0.16666666666666666 * assign46910_e60178);
        let assign46910_e60180: f64 = (assign46910_e60170 + assign46910_e60179);
        let assign46910_e60181: f64 = (locals.var_xsq * assign46910_e60180);
        let assign46910_e60182: f64 = (1.0 + assign46910_e60181);
        (assign46910_e60182, ((locals.var_xsq_dn5 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn5 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn5 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn5))))))), ((locals.var_xsq_dn6 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn6 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn6 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn6))))))), ((locals.var_xsq_dn7 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn7 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn7 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn7))))))), ((locals.var_xsq_dn8 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn8 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn8 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn8))))))),)
    } else {
        (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8,)
    }
};
        locals.var_igc = assign46910_e60184;
        locals.var_igc_dn5 = assign46910_e60184_d_n5;
        locals.var_igc_dn6 = assign46910_e60184_d_n6;
        locals.var_igc_dn7 = assign46910_e60184_d_n7;
        locals.var_igc_dn8 = assign46910_e60184_d_n8;

        let (assign46920_e60219, assign46920_e60219_d_n5, assign46920_e60219_d_n6, assign46920_e60219_d_n7, assign46920_e60219_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
        let assign46920_e60195: f64 = (0.5 * locals.var_igc);
        let assign46920_e60203: f64 = (locals.var_bg + 0.25);
        let assign46920_e60204: f64 = (0.4 * assign46920_e60203);
        let assign46920_e60209: f64 = (0.125 + locals.var_bg);
        let assign46920_e60210: f64 = (locals.var_xsq * assign46920_e60209);
        let assign46920_e60211: f64 = (0.0285714285714 * assign46920_e60210);
        let assign46920_e60212: f64 = (assign46920_e60204 + assign46920_e60211);
        let assign46920_e60213: f64 = (locals.var_xsq * assign46920_e60212);
        let assign46920_e60214: f64 = (1.0 + assign46920_e60213);
        let assign46920_e60215: f64 = (locals.var_x * assign46920_e60214);
        let assign46920_e60216: f64 = (0.16666666666666666 * assign46920_e60215);
        let assign46920_e60217: f64 = (assign46920_e60195 - assign46920_e60216);
        (assign46920_e60217, ((0.5 * locals.var_igc_dn5) - (0.16666666666666666 * ((locals.var_x_dn5 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn5 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn5) + (0.0285714285714 * ((locals.var_xsq_dn5 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn5)))))))))), ((0.5 * locals.var_igc_dn6) - (0.16666666666666666 * ((locals.var_x_dn6 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn6 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn6) + (0.0285714285714 * ((locals.var_xsq_dn6 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn6)))))))))), ((0.5 * locals.var_igc_dn7) - (0.16666666666666666 * ((locals.var_x_dn7 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn7 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn7) + (0.0285714285714 * ((locals.var_xsq_dn7 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn7)))))))))), ((0.5 * locals.var_igc_dn8) - (0.16666666666666666 * ((locals.var_x_dn8 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn8 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn8) + (0.0285714285714 * ((locals.var_xsq_dn8 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn8)))))))))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8,)
    }
};
        locals.var_igcd_h = assign46920_e60219;
        locals.var_igcd_h_dn5 = assign46920_e60219_d_n5;
        locals.var_igcd_h_dn6 = assign46920_e60219_d_n6;
        locals.var_igcd_h_dn7 = assign46920_e60219_d_n7;
        locals.var_igcd_h_dn8 = assign46920_e60219_d_n8;

        let (assign46930_e60233, assign46930_e60233_d_n5, assign46930_e60233_d_n6, assign46930_e60233_d_n7, assign46930_e60233_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign46930_e60231: f64 = (1.0 / locals.var_x);
        (assign46930_e60231, (-(locals.var_x_dn5 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn6 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn7 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn8 / (locals.var_x * locals.var_x))),)
    } else {
        (locals.var_inv_x, locals.var_inv_x_dn5, locals.var_inv_x_dn6, locals.var_inv_x_dn7, locals.var_inv_x_dn8,)
    }
};
        locals.var_inv_x = assign46930_e60233;
        locals.var_inv_x_dn5 = assign46930_e60233_d_n5;
        locals.var_inv_x_dn6 = assign46930_e60233_d_n6;
        locals.var_inv_x_dn7 = assign46930_e60233_d_n7;
        locals.var_inv_x_dn8 = assign46930_e60233_d_n8;

        let assign46940_e60235: f64 = (locals.var_x).abs();
        let assign46940_e60237: f64 = if assign46940_e60235 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1242 = assign46940_e60237;

        let (assign46950_e60252, assign46950_e60252_d_n5, assign46950_e60252_d_n6, assign46950_e60252_d_n7, assign46950_e60252_d_n8,) = {
    if (((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 != 0.0)) {
        let assign46950_e60250: f64 = (locals.var_x).exp();
        (assign46950_e60250, (assign46950_e60250 * locals.var_x_dn5), (assign46950_e60250 * locals.var_x_dn6), (assign46950_e60250 * locals.var_x_dn7), (assign46950_e60250 * locals.var_x_dn8),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46950_e60252;
        locals.var_ex_dn5 = assign46950_e60252_d_n5;
        locals.var_ex_dn6 = assign46950_e60252_d_n6;
        locals.var_ex_dn7 = assign46950_e60252_d_n7;
        locals.var_ex_dn8 = assign46950_e60252_d_n8;

        let assign46960_e60255: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46960_e60255;

        let (assign46970_e60297, assign46970_e60297_d_n5, assign46970_e60297_d_n6, assign46970_e60297_d_n7, assign46970_e60297_d_n8,) = {
    if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 != 0.0)) {
        let assign46970_e60273: f64 = (-230.25850929940458);
        let assign46970_e60275: f64 = (assign46970_e60273 - locals.var_x);
        let assign46970_e60279: f64 = (-230.25850929940458);
        let assign46970_e60281: f64 = (assign46970_e60279 - locals.var_x);
        let assign46970_e60284: f64 = (-230.25850929940458);
        let assign46970_e60286: f64 = (assign46970_e60284 - locals.var_x);
        let assign46970_e60288: f64 = (assign46970_e60286 * 0.3333333333333333);
        let assign46970_e60289: f64 = (1.0 + assign46970_e60288);
        let assign46970_e60290: f64 = (assign46970_e60281 * assign46970_e60289);
        let assign46970_e60291: f64 = (0.5 * assign46970_e60290);
        let assign46970_e60292: f64 = (1.0 + assign46970_e60291);
        let assign46970_e60293: f64 = (assign46970_e60275 * assign46970_e60292);
        let assign46970_e60294: f64 = (1.0 + assign46970_e60293);
        let assign46970_e60295: f64 = (1e-100 / assign46970_e60294);
        (assign46970_e60295, (-((1e-100 * (((-locals.var_x_dn5) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn5) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn5) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn6) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn6) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn7) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn7) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn8) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn8) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46970_e60297;
        locals.var_ex_dn5 = assign46970_e60297_d_n5;
        locals.var_ex_dn6 = assign46970_e60297_d_n6;
        locals.var_ex_dn7 = assign46970_e60297_d_n7;
        locals.var_ex_dn8 = assign46970_e60297_d_n8;

        let (assign46980_e60337, assign46980_e60337_d_n5, assign46980_e60337_d_n6, assign46980_e60337_d_n7, assign46980_e60337_d_n8,) = {
    if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 == 0.0)) {
        let assign46980_e60317: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60322: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60326: f64 = (locals.var_x - 230.25850929940458);
        let assign46980_e60328: f64 = (assign46980_e60326 * 0.3333333333333333);
        let assign46980_e60329: f64 = (1.0 + assign46980_e60328);
        let assign46980_e60330: f64 = (assign46980_e60322 * assign46980_e60329);
        let assign46980_e60331: f64 = (0.5 * assign46980_e60330);
        let assign46980_e60332: f64 = (1.0 + assign46980_e60331);
        let assign46980_e60333: f64 = (assign46980_e60317 * assign46980_e60332);
        let assign46980_e60334: f64 = (1.0 + assign46980_e60333);
        let assign46980_e60335: f64 = (1e100 * assign46980_e60334);
        (assign46980_e60335, (1e100 * ((locals.var_x_dn5 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn5 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn6 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn7 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn8 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8,)
    }
};
        locals.var_ex = assign46980_e60337;
        locals.var_ex_dn5 = assign46980_e60337_d_n5;
        locals.var_ex_dn6 = assign46980_e60337_d_n6;
        locals.var_ex_dn7 = assign46980_e60337_d_n7;
        locals.var_ex_dn8 = assign46980_e60337_d_n8;

        let (assign46990_e60351, assign46990_e60351_d_n5, assign46990_e60351_d_n6, assign46990_e60351_d_n7, assign46990_e60351_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign46990_e60349: f64 = (1.0 / locals.var_ex);
        (assign46990_e60349, (-(locals.var_ex_dn5 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn5, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8,)
    }
};
        locals.var_inv_ex = assign46990_e60351;
        locals.var_inv_ex_dn5 = assign46990_e60351_d_n5;
        locals.var_inv_ex_dn6 = assign46990_e60351_d_n6;
        locals.var_inv_ex_dn7 = assign46990_e60351_d_n7;
        locals.var_inv_ex_dn8 = assign46990_e60351_d_n8;

        let (assign47000_e60365, assign47000_e60365_d_n5, assign47000_e60365_d_n6, assign47000_e60365_d_n7, assign47000_e60365_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47000_e60363: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign47000_e60363, (locals.var_ex_dn5 - locals.var_inv_ex_dn5), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47000_e60365;
        locals.var_temp__blk936_dn5 = assign47000_e60365_d_n5;
        locals.var_temp__blk936_dn6 = assign47000_e60365_d_n6;
        locals.var_temp__blk936_dn7 = assign47000_e60365_d_n7;
        locals.var_temp__blk936_dn8 = assign47000_e60365_d_n8;

        let (assign47010_e60379, assign47010_e60379_d_n5, assign47010_e60379_d_n6, assign47010_e60379_d_n7, assign47010_e60379_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47010_e60377: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign47010_e60377, (locals.var_ex_dn5 + locals.var_inv_ex_dn5), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47010_e60379;
        locals.var_temp2_dn5 = assign47010_e60379_d_n5;
        locals.var_temp2_dn6 = assign47010_e60379_d_n6;
        locals.var_temp2_dn7 = assign47010_e60379_d_n7;
        locals.var_temp2_dn8 = assign47010_e60379_d_n8;

        let (assign47020_e60403, assign47020_e60403_d_n5, assign47020_e60403_d_n6, assign47020_e60403_d_n7, assign47020_e60403_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47020_e60392: f64 = (1.0 - locals.var_u0_div_h);
        let assign47020_e60394: f64 = (assign47020_e60392 * locals.var_temp__blk936);
        let assign47020_e60396: f64 = (assign47020_e60394 * locals.var_inv_x);
        let assign47020_e60399: f64 = (locals.var_u0_div_h * locals.var_temp2);
        let assign47020_e60400: f64 = (assign47020_e60396 + assign47020_e60399);
        let assign47020_e60401: f64 = (0.5 * assign47020_e60400);
        (assign47020_e60401, (0.5 * ((((((-locals.var_u0_div_h_dn5) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn5)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn5)) + ((locals.var_u0_div_h_dn5 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn5)))), (0.5 * ((((((-locals.var_u0_div_h_dn6) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn6)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn6)) + ((locals.var_u0_div_h_dn6 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn6)))), (0.5 * ((((((-locals.var_u0_div_h_dn7) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn7)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn7)) + ((locals.var_u0_div_h_dn7 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn7)))), (0.5 * ((((((-locals.var_u0_div_h_dn8) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn8)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn8)) + ((locals.var_u0_div_h_dn8 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn8)))),)
    } else {
        (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8,)
    }
};
        locals.var_igc = assign47020_e60403;
        locals.var_igc_dn5 = assign47020_e60403_d_n5;
        locals.var_igc_dn6 = assign47020_e60403_d_n6;
        locals.var_igc_dn7 = assign47020_e60403_d_n7;
        locals.var_igc_dn8 = assign47020_e60403_d_n8;

        let (assign47030_e60433, assign47030_e60433_d_n5, assign47030_e60433_d_n6, assign47030_e60433_d_n7, assign47030_e60433_d_n8,) = {
    if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
        let assign47030_e60419: f64 = (locals.var_ag * locals.var_inv_x);
        let assign47030_e60421: f64 = (assign47030_e60419 * locals.var_inv_x);
        let assign47030_e60422: f64 = (locals.var_bg - assign47030_e60421);
        let assign47030_e60423: f64 = (locals.var_temp__blk936 * assign47030_e60422);
        let assign47030_e60424: f64 = (locals.var_igc - assign47030_e60423);
        let assign47030_e60427: f64 = (locals.var_ag * locals.var_temp2);
        let assign47030_e60429: f64 = (assign47030_e60427 * locals.var_inv_x);
        let assign47030_e60430: f64 = (assign47030_e60424 - assign47030_e60429);
        let assign47030_e60431: f64 = (0.5 * assign47030_e60430);
        (assign47030_e60431, (0.5 * ((locals.var_igc_dn5 - ((locals.var_temp__blk936_dn5 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn5 - ((((locals.var_ag_dn5 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn5)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn5)))))) - ((((locals.var_ag_dn5 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn5)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn5)))), (0.5 * ((locals.var_igc_dn6 - ((locals.var_temp__blk936_dn6 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn6 - ((((locals.var_ag_dn6 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn6)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn6)))))) - ((((locals.var_ag_dn6 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn6)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn6)))), (0.5 * ((locals.var_igc_dn7 - ((locals.var_temp__blk936_dn7 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn7 - ((((locals.var_ag_dn7 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn7)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn7)))))) - ((((locals.var_ag_dn7 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn7)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn7)))), (0.5 * ((locals.var_igc_dn8 - ((locals.var_temp__blk936_dn8 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn8 - ((((locals.var_ag_dn8 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn8)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn8)))))) - ((((locals.var_ag_dn8 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn8)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn8)))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8,)
    }
};
        locals.var_igcd_h = assign47030_e60433;
        locals.var_igcd_h_dn5 = assign47030_e60433_d_n5;
        locals.var_igcd_h_dn6 = assign47030_e60433_d_n6;
        locals.var_igcd_h_dn7 = assign47030_e60433_d_n7;
        locals.var_igcd_h_dn8 = assign47030_e60433_d_n8;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47040_e60450, assign47040_e60450_d_n5, assign47040_e60450_d_n6, assign47040_e60450_d_n7, assign47040_e60450_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign47040_e60442: f64 = (locals.var_xg_dc * locals.var_xg_dc);
        let assign47040_e60444: f64 = (assign47040_e60442 + 1e-6);
        let assign47040_e60445: f64 = (assign47040_e60444).sqrt();
        let assign47040_e60446: f64 = (locals.var_xg_dc / assign47040_e60445);
        let assign47040_e60447: f64 = (1.0 + assign47040_e60446);
        let assign47040_e60448: f64 = (0.5 * assign47040_e60447);
        (assign47040_e60448, (0.5 * (((locals.var_xg_dc_dn5 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn5 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn5)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn6 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn6 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn6)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn7 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn7 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn7)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn8 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn8 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn8)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))),)
    } else {
        (locals.var_sg, locals.var_sg_dn5, locals.var_sg_dn6, locals.var_sg_dn7, locals.var_sg_dn8,)
    }
};
        locals.var_sg = assign47040_e60450;
        locals.var_sg_dn5 = assign47040_e60450_d_n5;
        locals.var_sg_dn6 = assign47040_e60450_d_n6;
        locals.var_sg_dn7 = assign47040_e60450_d_n7;
        locals.var_sg_dn8 = assign47040_e60450_d_n8;

        let (assign47050_e60460, assign47050_e60460_d_n5, assign47050_e60460_d_n6, assign47050_e60460_d_n7, assign47050_e60460_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign47050_e60456: f64 = (locals.var_igc0 * locals.var_igc);
        let assign47050_e60458: f64 = (assign47050_e60456 * locals.var_sg);
        (assign47050_e60458, ((((locals.var_igc0_dn5 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn5)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn5)), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn8)),)
    } else {
        (locals.var_igc_1, locals.var_igc_1_dn5, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8,)
    }
};
        locals.var_igc_1 = assign47050_e60460;
        locals.var_igc_1_dn5 = assign47050_e60460_d_n5;
        locals.var_igc_1_dn6 = assign47050_e60460_d_n6;
        locals.var_igc_1_dn7 = assign47050_e60460_d_n7;
        locals.var_igc_1_dn8 = assign47050_e60460_d_n8;

        let (assign47060_e60470, assign47060_e60470_d_n5, assign47060_e60470_d_n6, assign47060_e60470_d_n7, assign47060_e60470_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign47060_e60466: f64 = (locals.var_igc0 * locals.var_igcd_h);
        let assign47060_e60468: f64 = (assign47060_e60466 * locals.var_sg);
        (assign47060_e60468, ((((locals.var_igc0_dn5 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn5)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn5)), ((((locals.var_igc0_dn6 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn6)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn7)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn8)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn8)),)
    } else {
        (locals.var_i_gcd, locals.var_i_gcd_dn5, locals.var_i_gcd_dn6, locals.var_i_gcd_dn7, locals.var_i_gcd_dn8,)
    }
};
        locals.var_i_gcd = assign47060_e60470;
        locals.var_i_gcd_dn5 = assign47060_e60470_d_n5;
        locals.var_i_gcd_dn6 = assign47060_e60470_d_n6;
        locals.var_i_gcd_dn7 = assign47060_e60470_d_n7;
        locals.var_i_gcd_dn8 = assign47060_e60470_d_n8;

        let (assign47070_e60478, assign47070_e60478_d_n5, assign47070_e60478_d_n6, assign47070_e60478_d_n7, assign47070_e60478_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign47070_e60476: f64 = (locals.var_igc_1 - locals.var_i_gcd);
        (assign47070_e60476, (locals.var_igc_1_dn5 - locals.var_i_gcd_dn5), (locals.var_igc_1_dn6 - locals.var_i_gcd_dn6), (locals.var_igc_1_dn7 - locals.var_i_gcd_dn7), (locals.var_igc_1_dn8 - locals.var_i_gcd_dn8),)
    } else {
        (locals.var_i_gcs, locals.var_i_gcs_dn5, locals.var_i_gcs_dn6, locals.var_i_gcs_dn7, locals.var_i_gcs_dn8,)
    }
};
        locals.var_i_gcs = assign47070_e60478;
        locals.var_i_gcs_dn5 = assign47070_e60478_d_n5;
        locals.var_i_gcs_dn6 = assign47070_e60478_d_n6;
        locals.var_i_gcs_dn7 = assign47070_e60478_d_n7;
        locals.var_i_gcs_dn8 = assign47070_e60478_d_n8;

        let (assign47080_e60490, assign47080_e60490_d_n5, assign47080_e60490_d_n6, assign47080_e60490_d_n7, assign47080_e60490_d_n8,) = {
    if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign47080_e60484: f64 = (locals.var_igc0 * locals.var_igc);
        let assign47080_e60487: f64 = (1.0 - locals.var_sg);
        let assign47080_e60488: f64 = (assign47080_e60484 * assign47080_e60487);
        (assign47080_e60488, ((((locals.var_igc0_dn5 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn5)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn5))), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn6))), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn7))), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn8))),)
    } else {
        (locals.var_i_gb, locals.var_i_gb_dn5, locals.var_i_gb_dn6, locals.var_i_gb_dn7, locals.var_i_gb_dn8,)
    }
};
        locals.var_i_gb = assign47080_e60490;
        locals.var_i_gb_dn5 = assign47080_e60490_d_n5;
        locals.var_i_gb_dn6 = assign47080_e60490_d_n6;
        locals.var_i_gb_dn7 = assign47080_e60490_d_n7;
        locals.var_i_gb_dn8 = assign47080_e60490_d_n8;

        locals.var_i_gidl = 0.0;
        locals.var_i_gidl_dn5 = 0.0;
        locals.var_i_gidl_dn6 = 0.0;
        locals.var_i_gidl_dn7 = 0.0;
        locals.var_i_gidl_dn8 = 0.0;

        locals.var_i_gisl = 0.0;
        locals.var_i_gisl_dn5 = 0.0;
        locals.var_i_gisl_dn6 = 0.0;
        locals.var_i_gisl_dn7 = 0.0;
        locals.var_i_gisl_dn8 = 0.0;

        let assign47110_e60495: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign47110_e60495;

        let assign47120_e60502: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign47120_e60502;

        let (assign47130_e60521, assign47130_e60521_d_n5, assign47130_e60521_d_n6, assign47130_e60521_d_n7, assign47130_e60521_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
        let assign47130_e60508: f64 = (locals.var_vovd * locals.var_vovd);
        let assign47130_e60511: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign47130_e60514: f64 = (locals.var_vdbprime * locals.var_vdbprime);
        let assign47130_e60515: f64 = (assign47130_e60511 * assign47130_e60514);
        let assign47130_e60516: f64 = (assign47130_e60508 + assign47130_e60515);
        let assign47130_e60518: f64 = (assign47130_e60516 + 1e-6);
        let assign47130_e60519: f64 = (assign47130_e60518).sqrt();
        (assign47130_e60519, (((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) + (assign47130_e60511 * ((locals.var_vdbprime_dn6 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn6)))) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47130_e60511 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47130_e60519)), ((assign47130_e60511 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8))) / (2.0 * assign47130_e60519)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn5, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8,)
    }
};
        locals.var_vtovd = assign47130_e60521;
        locals.var_vtovd_dn5 = assign47130_e60521_d_n5;
        locals.var_vtovd_dn6 = assign47130_e60521_d_n6;
        locals.var_vtovd_dn7 = assign47130_e60521_d_n7;
        locals.var_vtovd_dn8 = assign47130_e60521_d_n8;

        let (assign47140_e60530, assign47140_e60530_d_n5, assign47140_e60530_d_n6, assign47140_e60530_d_n7, assign47140_e60530_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
        let assign47140_e60526: f64 = (-locals.var_bgidlds);
        let assign47140_e60528: f64 = (assign47140_e60526 / locals.var_vtovd);
        (assign47140_e60528, (-((assign47140_e60526 * locals.var_vtovd_dn5) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47140_e60530;
        locals.var_temp__blk936_dn5 = assign47140_e60530_d_n5;
        locals.var_temp__blk936_dn6 = assign47140_e60530_d_n6;
        locals.var_temp__blk936_dn7 = assign47140_e60530_d_n7;
        locals.var_temp__blk936_dn8 = assign47140_e60530_d_n8;

        let assign47150_e60533: f64 = (-230.25850929940458);
        let assign47150_e60534: f64 = if locals.var_temp__blk936 > assign47150_e60533 { 1.0 } else { 0.0 };
        locals.var_guard1246 = assign47150_e60534;

        let (assign47160_e60543, assign47160_e60543_d_n5, assign47160_e60543_d_n6, assign47160_e60543_d_n7, assign47160_e60543_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 != 0.0)) {
        let assign47160_e60541: f64 = (locals.var_temp__blk936).exp();
        (assign47160_e60541, (assign47160_e60541 * locals.var_temp__blk936_dn5), (assign47160_e60541 * locals.var_temp__blk936_dn6), (assign47160_e60541 * locals.var_temp__blk936_dn7), (assign47160_e60541 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47160_e60543;
        locals.var_temp2_dn5 = assign47160_e60543_d_n5;
        locals.var_temp2_dn6 = assign47160_e60543_d_n6;
        locals.var_temp2_dn7 = assign47160_e60543_d_n7;
        locals.var_temp2_dn8 = assign47160_e60543_d_n8;

        let (assign47170_e60577, assign47170_e60577_d_n5, assign47170_e60577_d_n6, assign47170_e60577_d_n7, assign47170_e60577_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign47170_e60553: f64 = (-230.25850929940458);
        let assign47170_e60555: f64 = (assign47170_e60553 - locals.var_temp__blk936);
        let assign47170_e60559: f64 = (-230.25850929940458);
        let assign47170_e60561: f64 = (assign47170_e60559 - locals.var_temp__blk936);
        let assign47170_e60564: f64 = (-230.25850929940458);
        let assign47170_e60566: f64 = (assign47170_e60564 - locals.var_temp__blk936);
        let assign47170_e60568: f64 = (assign47170_e60566 * 0.3333333333333333);
        let assign47170_e60569: f64 = (1.0 + assign47170_e60568);
        let assign47170_e60570: f64 = (assign47170_e60561 * assign47170_e60569);
        let assign47170_e60571: f64 = (0.5 * assign47170_e60570);
        let assign47170_e60572: f64 = (1.0 + assign47170_e60571);
        let assign47170_e60573: f64 = (assign47170_e60555 * assign47170_e60572);
        let assign47170_e60574: f64 = (1.0 + assign47170_e60573);
        let assign47170_e60575: f64 = (1e-100 / assign47170_e60574);
        (assign47170_e60575, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47170_e60577;
        locals.var_temp2_dn5 = assign47170_e60577_d_n5;
        locals.var_temp2_dn6 = assign47170_e60577_d_n6;
        locals.var_temp2_dn7 = assign47170_e60577_d_n7;
        locals.var_temp2_dn8 = assign47170_e60577_d_n8;

        let (assign47180_e60592, assign47180_e60592_d_n5, assign47180_e60592_d_n6, assign47180_e60592_d_n7, assign47180_e60592_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
        let assign47180_e60582: f64 = (-locals.var_agidlds);
        let assign47180_e60585: f64 = (locals.var_vdbprime * locals.var_vovd);
        let assign47180_e60587: f64 = (assign47180_e60585 * locals.var_vtovd);
        let assign47180_e60589: f64 = (assign47180_e60587 * locals.var_temp2);
        let assign47180_e60590: f64 = (assign47180_e60582 * assign47180_e60589);
        (assign47180_e60590, (assign47180_e60582 * (((((locals.var_vdbprime * locals.var_vovd_dn5) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn5)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn5))), (assign47180_e60582 * ((((((locals.var_vdbprime_dn6 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn6)) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn6)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn6))), (assign47180_e60582 * ((((((locals.var_vdbprime_dn7 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn7)) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn7)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn7))), (assign47180_e60582 * (((((locals.var_vdbprime_dn8 * locals.var_vovd) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn8)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn8))),)
    } else {
        (locals.var_i_gidl, locals.var_i_gidl_dn5, locals.var_i_gidl_dn6, locals.var_i_gidl_dn7, locals.var_i_gidl_dn8,)
    }
};
        locals.var_i_gidl = assign47180_e60592;
        locals.var_i_gidl_dn5 = assign47180_e60592_d_n5;
        locals.var_i_gidl_dn6 = assign47180_e60592_d_n6;
        locals.var_i_gidl_dn7 = assign47180_e60592_d_n7;
        locals.var_i_gidl_dn8 = assign47180_e60592_d_n8;

        let assign47190_e60599: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign47190_e60599;

        let (assign47200_e60618, assign47200_e60618_d_n5, assign47200_e60618_d_n6, assign47200_e60618_d_n7, assign47200_e60618_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47200_e60605: f64 = (locals.var_vovs * locals.var_vovs);
        let assign47200_e60608: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign47200_e60611: f64 = (locals.var_vsbprime * locals.var_vsbprime);
        let assign47200_e60612: f64 = (assign47200_e60608 * assign47200_e60611);
        let assign47200_e60613: f64 = (assign47200_e60605 + assign47200_e60612);
        let assign47200_e60615: f64 = (assign47200_e60613 + 1e-6);
        let assign47200_e60616: f64 = (assign47200_e60615).sqrt();
        (assign47200_e60616, (((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) + (assign47200_e60608 * ((locals.var_vsbprime_dn6 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn6)))) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47200_e60608 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47200_e60616)), ((assign47200_e60608 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8))) / (2.0 * assign47200_e60616)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn5, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8,)
    }
};
        locals.var_vtovs = assign47200_e60618;
        locals.var_vtovs_dn5 = assign47200_e60618_d_n5;
        locals.var_vtovs_dn6 = assign47200_e60618_d_n6;
        locals.var_vtovs_dn7 = assign47200_e60618_d_n7;
        locals.var_vtovs_dn8 = assign47200_e60618_d_n8;

        let (assign47210_e60627, assign47210_e60627_d_n5, assign47210_e60627_d_n6, assign47210_e60627_d_n7, assign47210_e60627_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47210_e60623: f64 = (-locals.var_bgidls);
        let assign47210_e60625: f64 = (assign47210_e60623 / locals.var_vtovs);
        (assign47210_e60625, (-((assign47210_e60623 * locals.var_vtovs_dn5) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47210_e60627;
        locals.var_temp__blk936_dn5 = assign47210_e60627_d_n5;
        locals.var_temp__blk936_dn6 = assign47210_e60627_d_n6;
        locals.var_temp__blk936_dn7 = assign47210_e60627_d_n7;
        locals.var_temp__blk936_dn8 = assign47210_e60627_d_n8;

        let assign47220_e60630: f64 = (-230.25850929940458);
        let assign47220_e60631: f64 = if locals.var_temp__blk936 > assign47220_e60630 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign47220_e60631;

        let (assign47230_e60640, assign47230_e60640_d_n5, assign47230_e60640_d_n6, assign47230_e60640_d_n7, assign47230_e60640_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign47230_e60638: f64 = (locals.var_temp__blk936).exp();
        (assign47230_e60638, (assign47230_e60638 * locals.var_temp__blk936_dn5), (assign47230_e60638 * locals.var_temp__blk936_dn6), (assign47230_e60638 * locals.var_temp__blk936_dn7), (assign47230_e60638 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47230_e60640;
        locals.var_temp2_dn5 = assign47230_e60640_d_n5;
        locals.var_temp2_dn6 = assign47230_e60640_d_n6;
        locals.var_temp2_dn7 = assign47230_e60640_d_n7;
        locals.var_temp2_dn8 = assign47230_e60640_d_n8;

        let (assign47240_e60674, assign47240_e60674_d_n5, assign47240_e60674_d_n6, assign47240_e60674_d_n7, assign47240_e60674_d_n8,) = {
    if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign47240_e60650: f64 = (-230.25850929940458);
        let assign47240_e60652: f64 = (assign47240_e60650 - locals.var_temp__blk936);
        let assign47240_e60656: f64 = (-230.25850929940458);
        let assign47240_e60658: f64 = (assign47240_e60656 - locals.var_temp__blk936);
        let assign47240_e60661: f64 = (-230.25850929940458);
        let assign47240_e60663: f64 = (assign47240_e60661 - locals.var_temp__blk936);
        let assign47240_e60665: f64 = (assign47240_e60663 * 0.3333333333333333);
        let assign47240_e60666: f64 = (1.0 + assign47240_e60665);
        let assign47240_e60667: f64 = (assign47240_e60658 * assign47240_e60666);
        let assign47240_e60668: f64 = (0.5 * assign47240_e60667);
        let assign47240_e60669: f64 = (1.0 + assign47240_e60668);
        let assign47240_e60670: f64 = (assign47240_e60652 * assign47240_e60669);
        let assign47240_e60671: f64 = (1.0 + assign47240_e60670);
        let assign47240_e60672: f64 = (1e-100 / assign47240_e60671);
        (assign47240_e60672, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign47240_e60674;
        locals.var_temp2_dn5 = assign47240_e60674_d_n5;
        locals.var_temp2_dn6 = assign47240_e60674_d_n6;
        locals.var_temp2_dn7 = assign47240_e60674_d_n7;
        locals.var_temp2_dn8 = assign47240_e60674_d_n8;

        let (assign47250_e60689, assign47250_e60689_d_n5, assign47250_e60689_d_n6, assign47250_e60689_d_n7, assign47250_e60689_d_n8,) = {
    if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47250_e60679: f64 = (-locals.var_agidls);
        let assign47250_e60682: f64 = (locals.var_vsbprime * locals.var_vovs);
        let assign47250_e60684: f64 = (assign47250_e60682 * locals.var_vtovs);
        let assign47250_e60686: f64 = (assign47250_e60684 * locals.var_temp2);
        let assign47250_e60687: f64 = (assign47250_e60679 * assign47250_e60686);
        (assign47250_e60687, (assign47250_e60679 * (((((locals.var_vsbprime * locals.var_vovs_dn5) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn5)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn5))), (assign47250_e60679 * ((((((locals.var_vsbprime_dn6 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn6)) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn6)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn6))), (assign47250_e60679 * ((((((locals.var_vsbprime_dn7 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn7)) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn7)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn7))), (assign47250_e60679 * (((((locals.var_vsbprime_dn8 * locals.var_vovs) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn8)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn8))),)
    } else {
        (locals.var_i_gisl, locals.var_i_gisl_dn5, locals.var_i_gisl_dn6, locals.var_i_gisl_dn7, locals.var_i_gisl_dn8,)
    }
};
        locals.var_i_gisl = assign47250_e60689;
        locals.var_i_gisl_dn5 = assign47250_e60689_d_n5;
        locals.var_i_gisl_dn6 = assign47250_e60689_d_n6;
        locals.var_i_gisl_dn7 = assign47250_e60689_d_n7;
        locals.var_i_gisl_dn8 = assign47250_e60689_d_n8;

        locals.var_phit1edge = locals.var_phit;
        locals.var_phit1edge_dn5 = 0.0;
        locals.var_phit1edge_dn6 = 0.0;
        locals.var_phit1edge_dn7 = 0.0;
        locals.var_phit1edge_dn8 = 0.0;

        locals.var_xgedge = 0.0;
        locals.var_xgedge_dn5 = 0.0;
        locals.var_xgedge_dn6 = 0.0;
        locals.var_xgedge_dn7 = 0.0;
        locals.var_xgedge_dn8 = 0.0;

        locals.var_qdseffedge = 0.0;
        locals.var_qdseffedge_dn5 = 0.0;
        locals.var_qdseffedge_dn6 = 0.0;
        locals.var_qdseffedge_dn7 = 0.0;
        locals.var_qdseffedge_dn8 = 0.0;

        locals.var_qmeffedge = 0.0;
        locals.var_qmeffedge_dn5 = 0.0;
        locals.var_qmeffedge_dn6 = 0.0;
        locals.var_qmeffedge_dn7 = 0.0;
        locals.var_qmeffedge_dn8 = 0.0;

        locals.var_dsqredge = 1e-40;
        locals.var_dsqredge_dn5 = 0.0;
        locals.var_dsqredge_dn6 = 0.0;
        locals.var_dsqredge_dn7 = 0.0;
        locals.var_dsqredge_dn8 = 0.0;

        locals.var_alphabmedge = 1.0;
        locals.var_alphabmedge_dn5 = 0.0;
        locals.var_alphabmedge_dn6 = 0.0;
        locals.var_alphabmedge_dn7 = 0.0;
        locals.var_alphabmedge_dn8 = 0.0;

        locals.var_i_dsedge = 0.0;
        locals.var_i_dsedge_dn5 = 0.0;
        locals.var_i_dsedge_dn6 = 0.0;
        locals.var_i_dsedge_dn7 = 0.0;
        locals.var_i_dsedge_dn8 = 0.0;

        let assign47330_e60703: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign47330_e60703;

        let (assign47340_e60724, assign47340_e60724_d_n5, assign47340_e60724_d_n6, assign47340_e60724_d_n7, assign47340_e60724_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47340_e60708: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign47340_e60711: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47340_e60714: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47340_e60715: f64 = (assign47340_e60711 * assign47340_e60714);
        let assign47340_e60717: f64 = (assign47340_e60715 + locals.var_bphiedge);
        let assign47340_e60718: f64 = (assign47340_e60717).sqrt();
        let assign47340_e60719: f64 = (assign47340_e60708 - assign47340_e60718);
        let assign47340_e60720: f64 = (0.5 * assign47340_e60719);
        let assign47340_e60722: f64 = (assign47340_e60720 + locals.var_phixedge);
        (assign47340_e60722, 0.0, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47340_e60718)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47340_e60724;
        locals.var_temp__blk936_dn5 = assign47340_e60724_d_n5;
        locals.var_temp__blk936_dn6 = assign47340_e60724_d_n6;
        locals.var_temp__blk936_dn7 = assign47340_e60724_d_n7;
        locals.var_temp__blk936_dn8 = assign47340_e60724_d_n8;

        let (assign47350_e60747, assign47350_e60747_d_n5, assign47350_e60747_d_n6, assign47350_e60747_d_n7, assign47350_e60747_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47350_e60730: f64 = locals.var_temp__blk936;
        let assign47350_e60733: f64 = locals.var_temp__blk936;
        let assign47350_e60736: f64 = locals.var_temp__blk936;
        let assign47350_e60737: f64 = (assign47350_e60733 * assign47350_e60736);
        let assign47350_e60739: f64 = (assign47350_e60737 + locals.var_aphiedge);
        let assign47350_e60740: f64 = (assign47350_e60739).sqrt();
        let assign47350_e60741: f64 = (assign47350_e60730 - assign47350_e60740);
        let assign47350_e60742: f64 = (0.5 * assign47350_e60741);
        let assign47350_e60743: f64 = (locals.var_v_sb - assign47350_e60742);
        let assign47350_e60745: f64 = (assign47350_e60743 + locals.var_phix1edge);
        (assign47350_e60745, (-(0.5 * (locals.var_temp__blk936_dn5 - (((locals.var_temp__blk936_dn5 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn5)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn6 - (0.5 * (locals.var_temp__blk936_dn6 - (((locals.var_temp__blk936_dn6 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn6)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk936_dn7 - (((locals.var_temp__blk936_dn7 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn7)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk936_dn8 - (((locals.var_temp__blk936_dn8 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn8)) / (2.0 * assign47350_e60740))))),)
    } else {
        (locals.var_vsbstaredge, locals.var_vsbstaredge_dn5, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8,)
    }
};
        locals.var_vsbstaredge = assign47350_e60747;
        locals.var_vsbstaredge_dn5 = assign47350_e60747_d_n5;
        locals.var_vsbstaredge_dn6 = assign47350_e60747_d_n6;
        locals.var_vsbstaredge_dn7 = assign47350_e60747_d_n7;
        locals.var_vsbstaredge_dn8 = assign47350_e60747_d_n8;

        let (assign47360_e60757, assign47360_e60757_d_n5, assign47360_e60757_d_n6, assign47360_e60757_d_n7, assign47360_e60757_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47360_e60753: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign47360_e60754: f64 = (0.5 * assign47360_e60753);
        let assign47360_e60755: f64 = (locals.var_vsbstaredge + assign47360_e60754);
        (assign47360_e60755, locals.var_vsbstaredge_dn5, (locals.var_vsbstaredge_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstaredge_dn8,)
    } else {
        (locals.var_vsbxedge, locals.var_vsbxedge_dn5, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8,)
    }
};
        locals.var_vsbxedge = assign47360_e60757;
        locals.var_vsbxedge_dn5 = assign47360_e60757_d_n5;
        locals.var_vsbxedge_dn6 = assign47360_e60757_d_n6;
        locals.var_vsbxedge_dn7 = assign47360_e60757_d_n7;
        locals.var_vsbxedge_dn8 = assign47360_e60757_d_n8;

        let (assign47370_e60773, assign47370_e60773_d_n5, assign47370_e60773_d_n6, assign47370_e60773_d_n7, assign47370_e60773_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47370_e60763: f64 = (locals.var_pscededge_i * locals.var_vdsx);
        let assign47370_e60764: f64 = (1.0 + assign47370_e60763);
        let assign47370_e60765: f64 = (locals.var_psceedge_i * assign47370_e60764);
        let assign47370_e60769: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
        let assign47370_e60770: f64 = (1.0 + assign47370_e60769);
        let assign47370_e60771: f64 = (assign47370_e60765 * assign47370_e60770);
        (assign47370_e60771, (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn5)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn6)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8)),)
    } else {
        (locals.var_dphit1edge, locals.var_dphit1edge_dn5, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8,)
    }
};
        locals.var_dphit1edge = assign47370_e60773;
        locals.var_dphit1edge_dn5 = assign47370_e60773_d_n5;
        locals.var_dphit1edge_dn6 = assign47370_e60773_d_n6;
        locals.var_dphit1edge_dn7 = assign47370_e60773_d_n7;
        locals.var_dphit1edge_dn8 = assign47370_e60773_d_n8;

        let (assign47380_e60781, assign47380_e60781_d_n5, assign47380_e60781_d_n6, assign47380_e60781_d_n7, assign47380_e60781_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47380_e60778: f64 = (1.0 + locals.var_dphit1edge);
        let assign47380_e60779: f64 = (locals.var_phit0edge * assign47380_e60778);
        (assign47380_e60779, (locals.var_phit0edge * locals.var_dphit1edge_dn5), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8),)
    } else {
        (locals.var_phit1edge, locals.var_phit1edge_dn5, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8,)
    }
};
        locals.var_phit1edge = assign47380_e60781;
        locals.var_phit1edge_dn5 = assign47380_e60781_d_n5;
        locals.var_phit1edge_dn6 = assign47380_e60781_d_n6;
        locals.var_phit1edge_dn7 = assign47380_e60781_d_n7;
        locals.var_phit1edge_dn8 = assign47380_e60781_d_n8;

        let (assign47390_e60787, assign47390_e60787_d_n5, assign47390_e60787_d_n6, assign47390_e60787_d_n7, assign47390_e60787_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47390_e60785: f64 = (1.0 / locals.var_phit1edge);
        (assign47390_e60785, (-(locals.var_phit1edge_dn5 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))),)
    } else {
        (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn5, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8,)
    }
};
        locals.var_inv_phit1edge = assign47390_e60787;
        locals.var_inv_phit1edge_dn5 = assign47390_e60787_d_n5;
        locals.var_inv_phit1edge_dn6 = assign47390_e60787_d_n6;
        locals.var_inv_phit1edge_dn7 = assign47390_e60787_d_n7;
        locals.var_inv_phit1edge_dn8 = assign47390_e60787_d_n8;

        let (assign47400_e60802, assign47400_e60802_d_n6, assign47400_e60802_d_n7,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47400_e60791: f64 = (2.0 * locals.var_vdsx);
        let assign47400_e60796: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
        let assign47400_e60797: f64 = (1.0 + assign47400_e60796);
        let assign47400_e60798: f64 = (assign47400_e60797).sqrt();
        let assign47400_e60799: f64 = (1.0 + assign47400_e60798);
        let assign47400_e60800: f64 = (assign47400_e60791 / assign47400_e60799);
        (assign47400_e60800, ((((2.0 * locals.var_vdsx_dn6) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn6) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)), ((((2.0 * locals.var_vdsx_dn7) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)),)
    } else {
        (locals.var_vdspedge, locals.var_vdspedge_dn6, locals.var_vdspedge_dn7,)
    }
};
        locals.var_vdspedge = assign47400_e60802;
        locals.var_vdspedge_dn6 = assign47400_e60802_d_n6;
        locals.var_vdspedge_dn7 = assign47400_e60802_d_n7;

        let (assign47410_e60814, assign47410_e60814_d_n5, assign47410_e60814_d_n6, assign47410_e60814_d_n7, assign47410_e60814_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47410_e60806: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
        let assign47410_e60810: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
        let assign47410_e60811: f64 = (1.0 + assign47410_e60810);
        let assign47410_e60812: f64 = (assign47410_e60806 * assign47410_e60811);
        (assign47410_e60812, (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn5)), (((locals.var_cfedge_i * locals.var_vdspedge_dn6) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6))), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8)),)
    } else {
        (locals.var_delvgedge, locals.var_delvgedge_dn5, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8,)
    }
};
        locals.var_delvgedge = assign47410_e60814;
        locals.var_delvgedge_dn5 = assign47410_e60814_d_n5;
        locals.var_delvgedge_dn6 = assign47410_e60814_d_n6;
        locals.var_delvgedge_dn7 = assign47410_e60814_d_n7;
        locals.var_delvgedge_dn8 = assign47410_e60814_d_n8;

        let (assign47420_e60824, assign47420_e60824_d_n5, assign47420_e60824_d_n6, assign47420_e60824_d_n7, assign47420_e60824_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47420_e60819: f64 = (locals.var_vgb + locals.var_delvgedge);
        let assign47420_e60821: f64 = (assign47420_e60819 - locals.var_vfbedge_t);
        let assign47420_e60822: f64 = (locals.var_inv_phit1edge * assign47420_e60821);
        (assign47420_e60822, ((locals.var_inv_phit1edge_dn5 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn5 + locals.var_delvgedge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))),)
    } else {
        (locals.var_xgedge, locals.var_xgedge_dn5, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8,)
    }
};
        locals.var_xgedge = assign47420_e60824;
        locals.var_xgedge_dn5 = assign47420_e60824_d_n5;
        locals.var_xgedge_dn6 = assign47420_e60824_d_n6;
        locals.var_xgedge_dn7 = assign47420_e60824_d_n7;
        locals.var_xgedge_dn8 = assign47420_e60824_d_n8;

    }

    pub(super) fn stamp_transient_block_31(
        locals: &mut StampLocals,
    ) {
        let (assign47430_e60830, assign47430_e60830_d_n5, assign47430_e60830_d_n6, assign47430_e60830_d_n7, assign47430_e60830_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47430_e60828: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
        (assign47430_e60828, (locals.var_inv_phit1edge_dn5 * locals.var_phibedge), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge),)
    } else {
        (locals.var_xbedge, locals.var_xbedge_dn5, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8,)
    }
};
        locals.var_xbedge = assign47430_e60830;
        locals.var_xbedge_dn5 = assign47430_e60830_d_n5;
        locals.var_xbedge_dn6 = assign47430_e60830_d_n6;
        locals.var_xbedge_dn7 = assign47430_e60830_d_n7;
        locals.var_xbedge_dn8 = assign47430_e60830_d_n8;

        let (assign47440_e60842, assign47440_e60842_d_n5, assign47440_e60842_d_n6, assign47440_e60842_d_n7, assign47440_e60842_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47440_e60835: f64 = (locals.var_xbedge / locals.var_gfedge);
        let assign47440_e60837: f64 = (locals.var_xbedge).sqrt();
        let assign47440_e60838: f64 = (assign47440_e60835 + assign47440_e60837);
        let assign47440_e60839: f64 = (assign47440_e60838).ln();
        let assign47440_e60840: f64 = (2.0 * assign47440_e60839);
        (assign47440_e60840, (2.0 * (((locals.var_xbedge_dn5 / locals.var_gfedge) + (locals.var_xbedge_dn5 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47440_e60837))) / assign47440_e60838)),)
    } else {
        (locals.var_dxthedge, locals.var_dxthedge_dn5, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8,)
    }
};
        locals.var_dxthedge = assign47440_e60842;
        locals.var_dxthedge_dn5 = assign47440_e60842_d_n5;
        locals.var_dxthedge_dn6 = assign47440_e60842_d_n6;
        locals.var_dxthedge_dn7 = assign47440_e60842_d_n7;
        locals.var_dxthedge_dn8 = assign47440_e60842_d_n8;

        let (assign47450_e60848, assign47450_e60848_d_n5, assign47450_e60848_d_n6, assign47450_e60848_d_n7, assign47450_e60848_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47450_e60846: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
        (assign47450_e60846, ((locals.var_inv_phit1edge_dn5 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn5)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)),)
    } else {
        (locals.var_xnedge_s, locals.var_xnedge_s_dn5, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8,)
    }
};
        locals.var_xnedge_s = assign47450_e60848;
        locals.var_xnedge_s_dn5 = assign47450_e60848_d_n5;
        locals.var_xnedge_s_dn6 = assign47450_e60848_d_n6;
        locals.var_xnedge_s_dn7 = assign47450_e60848_d_n7;
        locals.var_xnedge_s_dn8 = assign47450_e60848_d_n8;

        let (assign47460_e60854, assign47460_e60854_d_n5, assign47460_e60854_d_n6, assign47460_e60854_d_n7, assign47460_e60854_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47460_e60852: f64 = (locals.var_xbedge + locals.var_xnedge_s);
        (assign47460_e60852, (locals.var_xbedge_dn5 + locals.var_xnedge_s_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8,)
    }
};
        locals.var_q_edge_xsth = assign47460_e60854;
        locals.var_q_edge_xsth_dn5 = assign47460_e60854_d_n5;
        locals.var_q_edge_xsth_dn6 = assign47460_e60854_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47460_e60854_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47460_e60854_d_n8;

        let (assign47470_e60863, assign47470_e60863_d_n5, assign47470_e60863_d_n6, assign47470_e60863_d_n7, assign47470_e60863_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47470_e60859: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47470_e60860: f64 = (locals.var_gfedge * assign47470_e60859);
        let assign47470_e60861: f64 = (locals.var_q_edge_xsth + assign47470_e60860);
        (assign47470_e60861, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47470_e60859)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8,)
    }
};
        locals.var_q_edge_xth0 = assign47470_e60863;
        locals.var_q_edge_xth0_dn5 = assign47470_e60863_d_n5;
        locals.var_q_edge_xth0_dn6 = assign47470_e60863_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47470_e60863_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47470_e60863_d_n8;

        let (assign47480_e60869, assign47480_e60869_d_n5, assign47480_e60869_d_n6, assign47480_e60869_d_n7, assign47480_e60869_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47480_e60867: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47480_e60867, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8,)
    }
};
        locals.var_q_edge_xth = assign47480_e60869;
        locals.var_q_edge_xth_dn5 = assign47480_e60869_d_n5;
        locals.var_q_edge_xth_dn6 = assign47480_e60869_d_n6;
        locals.var_q_edge_xth_dn7 = assign47480_e60869_d_n7;
        locals.var_q_edge_xth_dn8 = assign47480_e60869_d_n8;

        let (assign47490_e60880, assign47490_e60880_d_n5, assign47490_e60880_d_n6, assign47490_e60880_d_n7, assign47490_e60880_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47490_e60875: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47490_e60876: f64 = (2.0 * assign47490_e60875);
        let assign47490_e60877: f64 = (locals.var_gfedge / assign47490_e60876);
        let assign47490_e60878: f64 = (1.0 + assign47490_e60877);
        (assign47490_e60878, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8,)
    }
};
        locals.var_q_edge_n = assign47490_e60880;
        locals.var_q_edge_n_dn5 = assign47490_e60880_d_n5;
        locals.var_q_edge_n_dn6 = assign47490_e60880_d_n6;
        locals.var_q_edge_n_dn7 = assign47490_e60880_d_n7;
        locals.var_q_edge_n_dn8 = assign47490_e60880_d_n8;

        let (assign47500_e60886, assign47500_e60886_d_n5, assign47500_e60886_d_n6, assign47500_e60886_d_n7, assign47500_e60886_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47500_e60884: f64 = (1.0 / locals.var_q_edge_n);
        (assign47500_e60884, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8,)
    }
};
        locals.var_q_edge_n_inv = assign47500_e60886;
        locals.var_q_edge_n_inv_dn5 = assign47500_e60886_d_n5;
        locals.var_q_edge_n_inv_dn6 = assign47500_e60886_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47500_e60886_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47500_e60886_d_n8;

        let (assign47510_e60892, assign47510_e60892_d_n5, assign47510_e60892_d_n6, assign47510_e60892_d_n7, assign47510_e60892_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47510_e60890: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47510_e60890, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    }
};
        locals.var_q_edge_xgt = assign47510_e60892;
        locals.var_q_edge_xgt_dn5 = assign47510_e60892_d_n5;
        locals.var_q_edge_xgt_dn6 = assign47510_e60892_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47510_e60892_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47510_e60892_d_n8;

        let assign47520_e60895: f64 = (-12.0);
        let assign47520_e60896: f64 = if locals.var_q_edge_xgt > assign47520_e60895 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign47520_e60896;

        let (assign47530_e60906, assign47530_e60906_d_n5, assign47530_e60906_d_n6, assign47530_e60906_d_n7, assign47530_e60906_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47530_e60902: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47530_e60904: f64 = (assign47530_e60902 - 1.0);
        (assign47530_e60904, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8,)
    }
};
        locals.var_q_edge_xgt0 = assign47530_e60906;
        locals.var_q_edge_xgt0_dn5 = assign47530_e60906_d_n5;
        locals.var_q_edge_xgt0_dn6 = assign47530_e60906_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47530_e60906_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47530_e60906_d_n8;

        let (assign47540_e60921, assign47540_e60921_d_n5, assign47540_e60921_d_n6, assign47540_e60921_d_n7, assign47540_e60921_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47540_e60914: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47540_e60916: f64 = (assign47540_e60914 + 10.0);
        let assign47540_e60917: f64 = (assign47540_e60916).sqrt();
        let assign47540_e60918: f64 = (locals.var_q_edge_xgt0 + assign47540_e60917);
        let assign47540_e60919: f64 = (0.5 * assign47540_e60918);
        (assign47540_e60919, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47540_e60917)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8,)
    }
};
        locals.var_q_edge_xgt0e = assign47540_e60921;
        locals.var_q_edge_xgt0e_dn5 = assign47540_e60921_d_n5;
        locals.var_q_edge_xgt0e_dn6 = assign47540_e60921_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47540_e60921_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47540_e60921_d_n8;

        let (assign47550_e60934, assign47550_e60934_d_n5, assign47550_e60934_d_n6, assign47550_e60934_d_n7, assign47550_e60934_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47550_e60928: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47550_e60929: f64 = (locals.var_q_edge_n * assign47550_e60928);
        let assign47550_e60930: f64 = (locals.var_q_edge_xgt - assign47550_e60929);
        let assign47550_e60932: f64 = (assign47550_e60930 + locals.var_lngfedge2);
        (assign47550_e60932, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8,)
    }
};
        locals.var_q_edge_qi0si = assign47550_e60934;
        locals.var_q_edge_qi0si_dn5 = assign47550_e60934_d_n5;
        locals.var_q_edge_qi0si_dn6 = assign47550_e60934_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47550_e60934_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47550_e60934_d_n8;

        let (assign47560_e60949, assign47560_e60949_d_n5, assign47560_e60949_d_n6, assign47560_e60949_d_n7, assign47560_e60949_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47560_e60942: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47560_e60944: f64 = (assign47560_e60942 + 2.0);
        let assign47560_e60945: f64 = (assign47560_e60944).sqrt();
        let assign47560_e60946: f64 = (locals.var_q_edge_qi0si + assign47560_e60945);
        let assign47560_e60947: f64 = (0.5 * assign47560_e60946);
        (assign47560_e60947, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47560_e60945)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8,)
    }
};
        locals.var_q_edge_qi0 = assign47560_e60949;
        locals.var_q_edge_qi0_dn5 = assign47560_e60949_d_n5;
        locals.var_q_edge_qi0_dn6 = assign47560_e60949_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47560_e60949_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47560_e60949_d_n8;

        let assign47570_e60952: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47570_e60954: f64 = if assign47570_e60952 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign47570_e60954;

        let (assign47580_e60965, assign47580_e60965_d_n5, assign47580_e60965_d_n6, assign47580_e60965_d_n7, assign47580_e60965_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 != 0.0)) {
        let assign47580_e60962: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47580_e60963: f64 = (assign47580_e60962).exp();
        (assign47580_e60963, (assign47580_e60963 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47580_e60965;
        locals.var_q_edge_exp_x_dn5 = assign47580_e60965_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47580_e60965_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47580_e60965_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47580_e60965_d_n8;

        let (assign47590_e61002, assign47590_e61002_d_n5, assign47590_e61002_d_n6, assign47590_e61002_d_n7, assign47590_e61002_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign47590_e60976: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60978: f64 = (assign47590_e60976 - 230.25850929940458);
        let assign47590_e60983: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60985: f64 = (assign47590_e60983 - 230.25850929940458);
        let assign47590_e60989: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47590_e60991: f64 = (assign47590_e60989 - 230.25850929940458);
        let assign47590_e60993: f64 = (assign47590_e60991 * 0.3333333333333333);
        let assign47590_e60994: f64 = (1.0 + assign47590_e60993);
        let assign47590_e60995: f64 = (assign47590_e60985 * assign47590_e60994);
        let assign47590_e60996: f64 = (0.5 * assign47590_e60995);
        let assign47590_e60997: f64 = (1.0 + assign47590_e60996);
        let assign47590_e60998: f64 = (assign47590_e60978 * assign47590_e60997);
        let assign47590_e60999: f64 = (1.0 + assign47590_e60998);
        let assign47590_e61000: f64 = (1e100 * assign47590_e60999);
        (assign47590_e61000, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47590_e61002;
        locals.var_q_edge_exp_x_dn5 = assign47590_e61002_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47590_e61002_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47590_e61002_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47590_e61002_d_n8;

        let (assign47600_e61010, assign47600_e61010_d_n5, assign47600_e61010_d_n6, assign47600_e61010_d_n7, assign47600_e61010_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47600_e61008: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47600_e61008, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8,)
    }
};
        locals.var_q_edge_d0 = assign47600_e61010;
        locals.var_q_edge_d0_dn5 = assign47600_e61010_d_n5;
        locals.var_q_edge_d0_dn6 = assign47600_e61010_d_n6;
        locals.var_q_edge_d0_dn7 = assign47600_e61010_d_n7;
        locals.var_q_edge_d0_dn8 = assign47600_e61010_d_n8;

        let (assign47610_e61018, assign47610_e61018_d_n5, assign47610_e61018_d_n6, assign47610_e61018_d_n7, assign47610_e61018_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47610_e61016: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47610_e61016, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8,)
    }
};
        locals.var_q_edge_d0p = assign47610_e61018;
        locals.var_q_edge_d0p_dn5 = assign47610_e61018_d_n5;
        locals.var_q_edge_d0p_dn6 = assign47610_e61018_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47610_e61018_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47610_e61018_d_n8;

        let (assign47620_e61036, assign47620_e61036_d_n5, assign47620_e61036_d_n6, assign47620_e61036_d_n7, assign47620_e61036_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47620_e61024: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47620_e61028: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47620_e61029: f64 = (2.0 * assign47620_e61028);
        let assign47620_e61031: f64 = (assign47620_e61029 - locals.var_q_edge_d0p);
        let assign47620_e61033: f64 = (assign47620_e61031 * locals.var_q_edge_d0p);
        let assign47620_e61034: f64 = (assign47620_e61024 + assign47620_e61033);
        (assign47620_e61034, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn8))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8,)
    }
};
        locals.var_q_edge_sqerr = assign47620_e61036;
        locals.var_q_edge_sqerr_dn5 = assign47620_e61036_d_n5;
        locals.var_q_edge_sqerr_dn6 = assign47620_e61036_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47620_e61036_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47620_e61036_d_n8;

        let (assign47630_e61051, assign47630_e61051_d_n5, assign47630_e61051_d_n6, assign47630_e61051_d_n7, assign47630_e61051_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47630_e61042: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47630_e61044: f64 = (assign47630_e61042 - locals.var_q_edge_n);
        let assign47630_e61046: f64 = (assign47630_e61044 / locals.var_q_edge_d0p);
        let assign47630_e61048: f64 = (assign47630_e61046 - 1.0);
        let assign47630_e61049: f64 = (locals.var_q_edge_n * assign47630_e61048);
        (assign47630_e61049, ((locals.var_q_edge_n_dn5 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8,)
    }
};
        locals.var_q_edge_errq = assign47630_e61051;
        locals.var_q_edge_errq_dn5 = assign47630_e61051_d_n5;
        locals.var_q_edge_errq_dn6 = assign47630_e61051_d_n6;
        locals.var_q_edge_errq_dn7 = assign47630_e61051_d_n7;
        locals.var_q_edge_errq_dn8 = assign47630_e61051_d_n8;

        let (assign47640_e61059, assign47640_e61059_d_n5, assign47640_e61059_d_n6, assign47640_e61059_d_n7, assign47640_e61059_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
        let assign47640_e61057: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47640_e61057, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47640_e61059;
        locals.var_qseffedge_dn5 = assign47640_e61059_d_n5;
        locals.var_qseffedge_dn6 = assign47640_e61059_d_n6;
        locals.var_qseffedge_dn7 = assign47640_e61059_d_n7;
        locals.var_qseffedge_dn8 = assign47640_e61059_d_n8;

        let assign47650_e61063: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47650_e61064: f64 = (locals.var_q_edge_n_inv * assign47650_e61063);
        let assign47650_e61066: f64 = (-230.25850929940458);
        let assign47650_e61067: f64 = if assign47650_e61064 > assign47650_e61066 { 1.0 } else { 0.0 };
        locals.var_guard1252 = assign47650_e61067;

        let (assign47660_e61081, assign47660_e61081_d_n5, assign47660_e61081_d_n6, assign47660_e61081_d_n7, assign47660_e61081_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 != 0.0)) {
        let assign47660_e61077: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47660_e61078: f64 = (locals.var_q_edge_n_inv * assign47660_e61077);
        let assign47660_e61079: f64 = (assign47660_e61078).exp();
        (assign47660_e61079, (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn5 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn6 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn7 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn8 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47660_e61081;
        locals.var_qseffedge_dn5 = assign47660_e61081_d_n5;
        locals.var_qseffedge_dn6 = assign47660_e61081_d_n6;
        locals.var_qseffedge_dn7 = assign47660_e61081_d_n7;
        locals.var_qseffedge_dn8 = assign47660_e61081_d_n8;

        let (assign47670_e61128, assign47670_e61128_d_n5, assign47670_e61128_d_n6, assign47670_e61128_d_n7, assign47670_e61128_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 == 0.0)) {
        let assign47670_e61092: f64 = (-230.25850929940458);
        let assign47670_e61096: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61097: f64 = (locals.var_q_edge_n_inv * assign47670_e61096);
        let assign47670_e61098: f64 = (assign47670_e61092 - assign47670_e61097);
        let assign47670_e61102: f64 = (-230.25850929940458);
        let assign47670_e61106: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61107: f64 = (locals.var_q_edge_n_inv * assign47670_e61106);
        let assign47670_e61108: f64 = (assign47670_e61102 - assign47670_e61107);
        let assign47670_e61111: f64 = (-230.25850929940458);
        let assign47670_e61115: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47670_e61116: f64 = (locals.var_q_edge_n_inv * assign47670_e61115);
        let assign47670_e61117: f64 = (assign47670_e61111 - assign47670_e61116);
        let assign47670_e61119: f64 = (assign47670_e61117 * 0.3333333333333333);
        let assign47670_e61120: f64 = (1.0 + assign47670_e61119);
        let assign47670_e61121: f64 = (assign47670_e61108 * assign47670_e61120);
        let assign47670_e61122: f64 = (0.5 * assign47670_e61121);
        let assign47670_e61123: f64 = (1.0 + assign47670_e61122);
        let assign47670_e61124: f64 = (assign47670_e61098 * assign47670_e61123);
        let assign47670_e61125: f64 = (1.0 + assign47670_e61124);
        let assign47670_e61126: f64 = (1e-100 / assign47670_e61125);
        (assign47670_e61126, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8,)
    }
};
        locals.var_qseffedge = assign47670_e61128;
        locals.var_qseffedge_dn5 = assign47670_e61128_d_n5;
        locals.var_qseffedge_dn6 = assign47670_e61128_d_n6;
        locals.var_qseffedge_dn7 = assign47670_e61128_d_n7;
        locals.var_qseffedge_dn8 = assign47670_e61128_d_n8;

        let (assign47680_e61136, assign47680_e61136_d_n5, assign47680_e61136_d_n6, assign47680_e61136_d_n7, assign47680_e61136_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47680_e61133: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
        let assign47680_e61134: f64 = (locals.var_inv_phit1edge * assign47680_e61133);
        (assign47680_e61134, ((locals.var_inv_phit1edge_dn5 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn5 + locals.var_vsbstaredge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))),)
    } else {
        (locals.var_xnedge_d, locals.var_xnedge_d_dn5, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8,)
    }
};
        locals.var_xnedge_d = assign47680_e61136;
        locals.var_xnedge_d_dn5 = assign47680_e61136_d_n5;
        locals.var_xnedge_d_dn6 = assign47680_e61136_d_n6;
        locals.var_xnedge_d_dn7 = assign47680_e61136_d_n7;
        locals.var_xnedge_d_dn8 = assign47680_e61136_d_n8;

        let assign47690_e61143: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign47690_e61143;

        let assign47700_e61145: f64 = (-locals.var_xnedge_d);
        let assign47700_e61147: f64 = (assign47700_e61145 + locals.var_xnedge_s);
        let assign47700_e61149: f64 = (-230.25850929940458);
        let assign47700_e61150: f64 = if assign47700_e61147 > assign47700_e61149 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign47700_e61150;

        let (assign47710_e61162, assign47710_e61162_d_n5, assign47710_e61162_d_n6, assign47710_e61162_d_n7, assign47710_e61162_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 != 0.0)) {
        let assign47710_e61157: f64 = (-locals.var_xnedge_d);
        let assign47710_e61159: f64 = (assign47710_e61157 + locals.var_xnedge_s);
        let assign47710_e61160: f64 = (assign47710_e61159).exp();
        (assign47710_e61160, (assign47710_e61160 * ((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47710_e61162;
        locals.var_temp__blk936_dn5 = assign47710_e61162_d_n5;
        locals.var_temp__blk936_dn6 = assign47710_e61162_d_n6;
        locals.var_temp__blk936_dn7 = assign47710_e61162_d_n7;
        locals.var_temp__blk936_dn8 = assign47710_e61162_d_n8;

        let (assign47720_e61205, assign47720_e61205_d_n5, assign47720_e61205_d_n6, assign47720_e61205_d_n7, assign47720_e61205_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 == 0.0)) {
        let assign47720_e61172: f64 = (-230.25850929940458);
        let assign47720_e61174: f64 = (-locals.var_xnedge_d);
        let assign47720_e61176: f64 = (assign47720_e61174 + locals.var_xnedge_s);
        let assign47720_e61177: f64 = (assign47720_e61172 - assign47720_e61176);
        let assign47720_e61181: f64 = (-230.25850929940458);
        let assign47720_e61183: f64 = (-locals.var_xnedge_d);
        let assign47720_e61185: f64 = (assign47720_e61183 + locals.var_xnedge_s);
        let assign47720_e61186: f64 = (assign47720_e61181 - assign47720_e61185);
        let assign47720_e61189: f64 = (-230.25850929940458);
        let assign47720_e61191: f64 = (-locals.var_xnedge_d);
        let assign47720_e61193: f64 = (assign47720_e61191 + locals.var_xnedge_s);
        let assign47720_e61194: f64 = (assign47720_e61189 - assign47720_e61193);
        let assign47720_e61196: f64 = (assign47720_e61194 * 0.3333333333333333);
        let assign47720_e61197: f64 = (1.0 + assign47720_e61196);
        let assign47720_e61198: f64 = (assign47720_e61186 * assign47720_e61197);
        let assign47720_e61199: f64 = (0.5 * assign47720_e61198);
        let assign47720_e61200: f64 = (1.0 + assign47720_e61199);
        let assign47720_e61201: f64 = (assign47720_e61177 * assign47720_e61200);
        let assign47720_e61202: f64 = (1.0 + assign47720_e61201);
        let assign47720_e61203: f64 = (1e-100 / assign47720_e61202);
        (assign47720_e61203, (-((1e-100 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign47720_e61205;
        locals.var_temp__blk936_dn5 = assign47720_e61205_d_n5;
        locals.var_temp__blk936_dn6 = assign47720_e61205_d_n6;
        locals.var_temp__blk936_dn7 = assign47720_e61205_d_n7;
        locals.var_temp__blk936_dn8 = assign47720_e61205_d_n8;

        let (assign47730_e61215, assign47730_e61215_d_n5, assign47730_e61215_d_n6, assign47730_e61215_d_n7, assign47730_e61215_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
        let assign47730_e61212: f64 = (locals.var_temp__blk936 - 1.0);
        let assign47730_e61213: f64 = (locals.var_qseffedge * assign47730_e61212);
        (assign47730_e61213, ((locals.var_qseffedge_dn5 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn5)), ((locals.var_qseffedge_dn6 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn6)), ((locals.var_qseffedge_dn7 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn7)), ((locals.var_qseffedge_dn8 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8,)
    }
};
        locals.var_qdseffedge = assign47730_e61215;
        locals.var_qdseffedge_dn5 = assign47730_e61215_d_n5;
        locals.var_qdseffedge_dn6 = assign47730_e61215_d_n6;
        locals.var_qdseffedge_dn7 = assign47730_e61215_d_n7;
        locals.var_qdseffedge_dn8 = assign47730_e61215_d_n8;

        let (assign47740_e61223, assign47740_e61223_d_n5, assign47740_e61223_d_n6, assign47740_e61223_d_n7, assign47740_e61223_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
        let assign47740_e61221: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
        (assign47740_e61221, (locals.var_qdseffedge_dn5 + locals.var_qseffedge_dn5), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47740_e61223;
        locals.var_qdeffedge_dn5 = assign47740_e61223_d_n5;
        locals.var_qdeffedge_dn6 = assign47740_e61223_d_n6;
        locals.var_qdeffedge_dn7 = assign47740_e61223_d_n7;
        locals.var_qdeffedge_dn8 = assign47740_e61223_d_n8;

        let (assign47750_e61232, assign47750_e61232_d_n5, assign47750_e61232_d_n6, assign47750_e61232_d_n7, assign47750_e61232_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47750_e61230: f64 = (locals.var_xbedge + locals.var_xnedge_d);
        (assign47750_e61230, (locals.var_xbedge_dn5 + locals.var_xnedge_d_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8,)
    }
};
        locals.var_q_edge_xsth = assign47750_e61232;
        locals.var_q_edge_xsth_dn5 = assign47750_e61232_d_n5;
        locals.var_q_edge_xsth_dn6 = assign47750_e61232_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47750_e61232_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47750_e61232_d_n8;

    }
}
