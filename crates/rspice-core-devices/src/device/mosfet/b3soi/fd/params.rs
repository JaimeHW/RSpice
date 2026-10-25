//! B3SOIFD model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b3soifd.c` (`B3SOIFDmPTable`),
//! defaults from `b3soifdset.c` (`B3SOIFDsetup`, lines 52-873).

pub use super::super::common::Binned;
use super::super::common::params::{SoiCardEntries, SoiModelCore};
use super::super::common::{B3SoiDialect, binned, get, val};
use crate::Value;
use std::collections::HashMap;

/// Full B3SOIFD `.model` card (one per model statement, shared by instances).
#[derive(Debug, Clone)]
pub struct B3SoiFdModel {
    core: SoiModelCore,

    pub dialect: B3SoiDialect,
}

super::super::common::deref_core!(B3SoiFdModel => SoiModelCore);

impl B3SoiFdModel {
    /// Build a model card from an uppercase-keyed parameter map.
    ///
    /// `is_pmos` selects the device polarity, `nominal_temp_k` supplies the
    /// circuit nominal temperature in Kelvin (ngspice `CKTnomTemp`) used when
    /// `TNOM` is absent (TNOM on the card is in Celsius).
    pub fn from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Self {
        Self::try_from_params(params, is_pmos, nominal_temp_k)
            .expect("B3SOIFD model selectors are valid")
    }

    pub fn try_from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Result<Self, String> {
        let p = params;

        let dialect =
            if get(p, "LEVEL").is_some_and(|level| (level.round() - 10.0).abs() <= 1.0e-12) {
                B3SoiDialect::Xyce
            } else {
                B3SoiDialect::Ngspice
            };

        // NOTE (faithful to ngspice): the IF-parameter table maps "xdif" onto
        // the XBJT slot (`IOPR("xdif", B3SOIFD_MOD_XBJT, ...)` in b3soifd.c),
        // so a model card XDIF actually overwrites xbjt and the internal xdif
        // keeps its default of 2.0.
        let xbjt = get(p, "XDIF").or_else(|| get(p, "XBJT")).unwrap_or(2.0);

        let mut model = Self {
            core: SoiModelCore::from_card(
                p,
                is_pmos,
                nominal_temp_k,
                SoiCardEntries {
                    // NOTE: ldvbd0/ldvbd1 (and w/p variants) default to 1.0.
                    dvbd0: binned(p, "DVBD0", 0.0, 1.0, 1.0, 1.0),
                    dvbd1: binned(p, "DVBD1", 0.0, 1.0, 1.0, 1.0),
                    beta0: binned(p, "BETA0", 30.0, 0.0, 0.0, 0.0),
                    cth0: val(p, "CTH0", 0.0),
                    xbjt,
                    xdif: 2.0,
                    xrec: val(p, "XREC", 20.0),
                },
            )?,
            dialect,
        };

        model.finish_setup("B3SOIFD", |vbsa, tsi| {
            crate::log_scope::warn_line!(
                "B3SOIFD: vbsa = {:.3} is too large for tsi = {:.3e}; treated as zero",
                vbsa,
                tsi
            )
        });
        Ok(model)
    }
}
