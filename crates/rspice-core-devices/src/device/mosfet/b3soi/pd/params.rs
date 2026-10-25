//! B3SOIPD model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b3soipd.c` (`B3SOIPDmPTable`),
//! defaults from `b3soipdset.c` (`B3SOIPDsetup`, lines 52-873).

pub use super::super::common::Binned;
use super::super::common::params::{SoiCardEntries, SoiModelCore};
use super::super::common::{binned, get, val};
use crate::Value;
use std::collections::HashMap;

/// Full B3SOIPD `.model` card (one per model statement, shared by instances).
#[derive(Debug, Clone)]
pub struct B3SoiPdModel {
    core: SoiModelCore,

    pub dtoxcv: Value,

    /// PD bulk-charge body-bias coefficient for Abulk (`ketas`, b3soipdld.c:1207).
    pub ketas: Binned,
    // PD impact-ionization (b3soipdld.c:2534-2620) binned families.
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
    // PD body-diode / parasitic-BJT (b3soipdld.c:1862-2200) binned families.
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

    // Non-binned DC/temperature parameters.
    /// PD body-tie width correction coefficients for k1eff (b3soipdtemp.c:829).
    pub k1w1: Value,
    pub k1w2: Value,

    pub rhalo: Value,
    pub wth0: Value,

    /// Channel-length reduction for the body CV length `leffCVb`
    /// (b3soipdset.c:420-421, default 0).
    pub dlcb: Value,
    /// Back-gate CV length extension for `leffCVbg` (b3soipdset.c:436-437,
    /// default 0).
    pub dlbg: Value,
    /// Body-charge scaling factor (b3soipdset.c:422-423, default 1).
    pub fbody: Value,
    pub acde: Binned,
    pub moin: Binned,
    pub delvt: Binned,

    /// `TPBSWG`: temperature coefficient of the gate-sidewall potential.
    pub tpbswg: Value,
    /// `TCJSWG`: temperature coefficient of the gate-sidewall capacitance.
    pub tcjswg: Value,

    pub llc: Value,
    pub lwc: Value,
    pub lwlc: Value,
    pub wlc: Value,
    pub wwc: Value,
    pub wwlc: Value,
}

super::super::common::deref_core!(B3SoiPdModel => SoiModelCore);

impl B3SoiPdModel {
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
            .expect("B3SOIPD model selectors are valid")
    }

    pub fn try_from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Result<Self, String> {
        let p = params;

        let dtoxcv = val(p, "DTOXCV", 0.0);

        let xbjt = val(p, "XBJT", 1.0);
        let xdif = get(p, "XDIF").unwrap_or(xbjt);

        let mut model = Self {
            core: SoiModelCore::from_card(
                p,
                is_pmos,
                nominal_temp_k,
                SoiCardEntries {
                    // NOTE: ldvbd0/ldvbd1 (and w/p variants) default to 1.0.
                    dvbd0: binned(p, "DVBD0", 0.0, 1.0, 1.0, 1.0),
                    dvbd1: binned(p, "DVBD1", 0.0, 1.0, 1.0, 1.0),
                    // PD impact ionization: beta0=0 (unlike DD's 30).
                    beta0: binned(p, "BETA0", 0.0, 0.0, 0.0, 0.0),
                    cth0: val(p, "CTH0", 0.0),
                    xbjt,
                    xdif,
                    xrec: val(p, "XREC", 1.0),
                },
            )?,
            dtoxcv,

            ketas: binned(p, "KETAS", 0.0, 0.0, 0.0, 0.0),
            // PD impact-ionization defaults (b3soipdset.c): beta2=0.1,
            // vdsatii0=0.9, sii0=0.5, sii1=0.1, esatii=1e7.
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
            // PD body-diode / BJT defaults (b3soipdset.c).
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

            k1w1: val(p, "K1W1", 0.0),
            k1w2: val(p, "K1W2", 0.0),

            rhalo: val(p, "RHALO", 1.0e15),
            wth0: val(p, "WTH0", 0.0),

            dlcb: val(p, "DLCB", 0.0),
            dlbg: val(p, "DLBG", 0.0),
            fbody: val(p, "FBODY", 1.0),
            acde: binned(p, "ACDE", 1.0, 0.0, 0.0, 0.0),
            moin: binned(p, "MOIN", 15.0, 0.0, 0.0, 0.0),
            delvt: binned(p, "DELVT", 0.0, 0.0, 0.0, 0.0),

            tpbswg: val(p, "TPBSWG", 0.0),
            tcjswg: val(p, "TCJSWG", 0.0),

            llc: val(p, "LLC", 0.0),
            lwc: val(p, "LWC", 0.0),
            lwlc: val(p, "LWLC", 0.0),
            wlc: val(p, "WLC", 0.0),
            wwc: val(p, "WWC", 0.0),
            wwlc: val(p, "WWLC", 0.0),
        };

        model.finish_setup("B3SOIPD", |vbsa, tsi| {
            crate::log_scope::warn_line!(
                "B3SOIPD: vbsa = {:.3} is too large for tsi = {:.3e}; treated as zero",
                vbsa,
                tsi
            )
        });
        Ok(model)
    }
}
