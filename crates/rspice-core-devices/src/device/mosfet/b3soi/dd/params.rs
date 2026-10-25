//! B3SOIDD model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b3soidd.c` (`B3SOIDDmPTable`),
//! defaults from `b3soiddset.c` (`B3SOIDDsetup`, lines 52-873).

pub use super::super::common::Binned;
use super::super::common::params::{SoiCardEntries, SoiModelCore};
use super::super::common::{B3SoiDialect, binned, get, model_selector, val};
use crate::Value;
use std::collections::HashMap;

/// Full B3SOIDD `.model` card (one per model statement, shared by instances).
#[derive(Debug, Clone)]
pub struct B3SoiDdModel {
    core: SoiModelCore,

    pub rgate_mod: i32,
    pub dialect: B3SoiDialect,

    pub k1b: Value,
    pub k2b: Value,
    pub dk2b: Value,
    pub nofffd: Value,
    pub vofffd: Value,
    pub moin_fd: Value,
    pub ketas: Binned,
    pub beta1: Binned,
    pub beta2: Binned,
    pub vdsatii0: Binned,
    pub esatii: Binned,
    pub sii0: Binned,
    pub sii1: Binned,
    pub sii2: Binned,
    pub siid: Binned,
    pub fbjtii: Binned,
    pub lii: Value,
    pub tii: Value,
    pub nrecf0: Binned,
    pub nrecr0: Binned,
    pub ahli: Binned,
    pub lbjt0: Binned,
    pub vabjt: Binned,
    pub aely: Binned,
    pub vrec0: Value,
    pub vtun0: Value,
    pub ntrecf: Value,
    pub ntrecr: Value,
    pub ln: Value,
    pub nbjt: Value,
    pub ndif: Value,
    pub ldif0: Value,
    pub xrcrg1: Binned,
    pub xrcrg2: Binned,

    pub rhalo: Value,
    pub rshg: Value,
    pub ngcon: Value,
    pub xgw: Value,
    pub xgl: Value,
    pub wth0: Value,

    pub dlcb: Value,
    pub dlbg: Value,
    pub fbody: Value,
    pub noff: Binned,
    pub delvt: Binned,
    /// Xyce CAPMOD 3 (charge-thickness) parameters.
    pub dtoxcv: Value,
    pub acde: Binned,
    pub moin: Binned,

    pub tpbswg: Value,
    pub tcjswg: Value,
}

super::super::common::deref_core!(B3SoiDdModel => SoiModelCore);

impl B3SoiDdModel {
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
            .expect("B3SOIDD model selectors are valid")
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

        // ngspice B3SOIDD defaults the DVBD L/W/P bins to 1.0, while
        // Xyce LEVEL=10 exposes only the nominal DVBD0/DVBD1 defaults.
        let dvbd_bin_default = match dialect {
            B3SoiDialect::Ngspice => 1.0,
            B3SoiDialect::Xyce => 0.0,
        };
        let beta0_default = match dialect {
            B3SoiDialect::Ngspice => 30.0,
            B3SoiDialect::Xyce => 0.0,
        };

        let xbjt = val(p, "XBJT", 1.0);
        let xdif = get(p, "XDIF").unwrap_or(xbjt);

        let mut model = Self {
            core: SoiModelCore::from_card(
                p,
                is_pmos,
                nominal_temp_k,
                SoiCardEntries {
                    dvbd0: binned(
                        p,
                        "DVBD0",
                        0.0,
                        dvbd_bin_default,
                        dvbd_bin_default,
                        dvbd_bin_default,
                    ),
                    dvbd1: binned(
                        p,
                        "DVBD1",
                        0.0,
                        dvbd_bin_default,
                        dvbd_bin_default,
                        dvbd_bin_default,
                    ),
                    beta0: binned(p, "BETA0", beta0_default, 0.0, 0.0, 0.0),
                    cth0: val(p, "CTH0", 1.0e-5),
                    xbjt,
                    xdif,
                    xrec: val(p, "XREC", 1.0),
                },
            )?,
            rgate_mod: model_selector(p, "RGATEMOD", 0, &[0, 1, 2])?,
            dialect,

            k1b: val(p, "K1B", 1.0),
            k2b: val(p, "K2B", 0.0),
            dk2b: val(p, "DK2B", 0.0),
            nofffd: val(p, "NOFFFD", 1.0),
            vofffd: val(p, "VOFFFD", 0.0),
            moin_fd: val(p, "MOINFD", 1.0e3),
            ketas: binned(p, "KETAS", 0.0, 0.0, 0.0, 0.0),
            beta1: binned(p, "BETA1", 0.0, 0.0, 0.0, 0.0),
            beta2: binned(p, "BETA2", 0.1, 0.0, 0.0, 0.0),
            vdsatii0: binned(p, "VDSATII0", 0.9, 0.0, 0.0, 0.0),
            esatii: binned(p, "ESATII", 1e7, 0.0, 0.0, 0.0),
            sii0: binned(p, "SII0", 0.5, 0.0, 0.0, 0.0),
            sii1: binned(p, "SII1", 0.1, 0.0, 0.0, 0.0),
            sii2: binned(p, "SII2", 0.0, 0.0, 0.0, 0.0),
            siid: binned(p, "SIID", 0.0, 0.0, 0.0, 0.0),
            fbjtii: binned(p, "FBJTII", 0.0, 0.0, 0.0, 0.0),
            lii: val(p, "LII", 0.0),
            tii: val(p, "TII", 0.0),
            nrecf0: binned(p, "NRECF0", 2.0, 0.0, 0.0, 0.0),
            nrecr0: binned(p, "NRECR0", 10.0, 0.0, 0.0, 0.0),
            ahli: binned(p, "AHLI", 0.0, 0.0, 0.0, 0.0),
            lbjt0: binned(p, "LBJT0", 0.20e-6, 0.0, 0.0, 0.0),
            vabjt: binned(p, "VABJT", 10.0, 0.0, 0.0, 0.0),
            aely: binned(p, "AELY", 0.0, 0.0, 0.0, 0.0),
            vrec0: val(p, "VREC0", 0.0),
            vtun0: val(p, "VTUN0", 0.0),
            ntrecf: val(p, "NTRECF", 0.0),
            ntrecr: val(p, "NTRECR", 0.0),
            ln: val(p, "LN", 2e-6),
            nbjt: val(p, "NBJT", 1.0),
            ndif: val(p, "NDIF", -1.0),
            ldif0: val(p, "LDIF0", 1.0),
            xrcrg1: binned(p, "XRCRG1", 12.0, 0.0, 0.0, 0.0),
            xrcrg2: binned(p, "XRCRG2", 1.0, 0.0, 0.0, 0.0),

            rhalo: val(p, "RHALO", 1.0e15),
            rshg: val(p, "RSHG", 0.1),
            ngcon: val(p, "NGCON", 1.0),
            xgw: val(p, "XGW", 0.0),
            xgl: val(p, "XGL", 0.0),
            wth0: val(p, "WTH0", 0.0),

            dlcb: val(p, "DLCB", 0.0),
            dlbg: val(p, "DLBG", 0.0),
            fbody: val(p, "FBODY", 1.0),
            noff: binned(p, "NOFF", 1.0, 0.0, 0.0, 0.0),
            delvt: binned(p, "DELVT", 0.0, 0.0, 0.0, 0.0),
            dtoxcv: val(p, "DTOXCV", 0.0),
            acde: binned(p, "ACDE", 1.0, 0.0, 0.0, 0.0),
            moin: binned(p, "MOIN", 15.0, 0.0, 0.0, 0.0),

            tpbswg: val(p, "TPBSWG", 0.0),
            tcjswg: val(p, "TCJSWG", 0.0),
        };

        model.finish_setup("B3SOIDD", |vbsa, tsi| {
            crate::log_scope::warn_line!(
                "B3SOIDD: vbsa = {:.3} is too large for tsi = {:.3e}; treated as zero",
                vbsa,
                tsi
            )
        });
        Ok(model)
    }
}
