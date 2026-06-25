#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[693] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(200, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(197), scratch.values[95]), scratch.ad_value(193)), scratch.values[125]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(201, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(200)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(202, &AdValue::square(scratch.ad_value(201)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(203, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(202)), AdValue::offset(AdValue::square(scratch.ad_value(202)), 1.0))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(204, &AdValue::sqrt(AdValue::abs(scratch.ad_value(203))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(205, &AdValue::mul(scratch.ad_value(203), scratch.ad_value(204)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(208, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(200), scratch.ad_value(204)), 0.375)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(209, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(201), scratch.ad_value(204)), 2.0), scratch.ad_value(203)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(210, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(201), scratch.values[122]), scratch.ad_value(204)), AdValue::scale(scratch.ad_value(203), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(211, &AdValue::mul(AdValue::offset(scratch.ad_value(209), (-1.0)), scratch.ad_value(208)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(172, &AdValue::square(scratch.ad_value(211)));
        }

        scratch.values[696] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (scratch.values[696] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[696] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[697] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[698] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[697] != 0.0))) && (scratch.values[698] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[697] != 0.0))) && (!(scratch.values[698] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[699] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[700] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (scratch.values[700] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[116])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[700] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[116]), scratch.values[13]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[113]), scratch.ad_value(190)), scratch.values[98]));
        }

        scratch.values[701] = if (((((-scratch.values[128]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (scratch.values[701] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))));
        }

        scratch.values[702] = if (((-scratch.values[128]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[701] != 0.0))) && (scratch.values[702] != 0.0)) {
            let assign18640_ad_e24642: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign18640_ad_e24642));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[701] != 0.0))) && (!(scratch.values[702] != 0.0))) {
            let assign18650_ad_e24691: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign18650_ad_e24691);
        }

        scratch.values[703] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[704] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[705] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (scratch.values[704] != 0.0)) && (scratch.values[705] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[135]), AdValue::scale(scratch.ad_value(189), scratch.values[135])), AdValue::scale(scratch.ad_value(189), scratch.values[135])), AdValue::scale(scratch.ad_value(189), scratch.values[135])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (scratch.values[704] != 0.0)) && (!(scratch.values[705] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[135])), scratch.values[45]));
        }

        scratch.store_ad(634, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(628), scratch.values[217]), AdValue::scale(scratch.ad_value(630), scratch.values[218])), AdValue::scale(scratch.ad_value(632), scratch.values[219])));

    }
}
