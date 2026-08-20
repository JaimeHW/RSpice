//! Model and subcircuit cards.
//!
//! Emits the model definitions the instance cards reference, and the
//! subcircuit bodies for hierarchical blocks.
//!
//! Two kinds of card are written here, and the difference matters. A switch,
//! a transmission line, a coupled line, a memristor or a magnetic core carries
//! its own parameters on its own card: the sheet offers RON, ROFF, VT, the
//! RLGC entries, and a per-instance card is the only place those can go. A
//! semiconductor the user never bound a model to carries nothing — the card is
//! entirely a statement about what the device *is*, and that statement belongs
//! to `rspice_core::library`, not here. Those families resolve onto a shared
//! foundation card, so an unbound placement means the same thing in the
//! generated deck, in the flattener and at bind time.

use rspice_core::library::{FoundationDeviceFamily, foundation_card_source};

use super::*;

/// The foundation family a bare placement of this symbol belongs to.
///
/// The symbol says what was drawn; the core library decides what that means
/// when nobody bound a model. Polarity is read from the symbol because it is
/// part of what was drawn, and nothing else about the card is decided here —
/// no parameter value appears in this file.
fn foundation_family(kind: ComponentType) -> Option<FoundationDeviceFamily> {
    Some(match kind {
        ComponentType::Diode => FoundationDeviceFamily::Diode,
        ComponentType::NpnBjt | ComponentType::NpnBjt4 => FoundationDeviceFamily::NpnBjt,
        ComponentType::PnpBjt | ComponentType::PnpBjt4 => FoundationDeviceFamily::PnpBjt,
        ComponentType::NpnBjt5 => FoundationDeviceFamily::NpnBjtThermal,
        ComponentType::PnpBjt5 => FoundationDeviceFamily::PnpBjtThermal,
        ComponentType::Nmos => FoundationDeviceFamily::Nmos,
        ComponentType::Pmos => FoundationDeviceFamily::Pmos,
        ComponentType::NmosSoi => FoundationDeviceFamily::NmosSoi,
        ComponentType::PmosSoi => FoundationDeviceFamily::PmosSoi,
        ComponentType::NVdmos => FoundationDeviceFamily::NVdmos,
        ComponentType::PVdmos => FoundationDeviceFamily::PVdmos,
        ComponentType::Njfet => FoundationDeviceFamily::Njfet,
        ComponentType::Pjfet => FoundationDeviceFamily::Pjfet,
        ComponentType::Nmesfet => FoundationDeviceFamily::Nmesfet,
        ComponentType::Pmesfet => FoundationDeviceFamily::Pmesfet,
        _ => return None,
    })
}

/// A ` NAME=value` fragment for a switch model parameter the user left blank,
/// or nothing at all.
///
/// Omission is not the same as a written default here: the engine's own
/// default for SMOOTH differs between the voltage and current switches, so
/// writing one would silently retune every deck that never asked for it.
fn optional_model_param(
    params: &std::collections::HashMap<String, String>,
    key: &str,
    card_name: &str,
) -> String {
    params
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| format!(" {card_name}={value}"))
        .unwrap_or_default()
}

impl<'a> NetlistGenerator<'a> {
    /// The model an instance card names, for a semiconductor whose card is a
    /// statement about what it is rather than a carrier for its own values.
    ///
    /// An explicit binding is trusted verbatim and injects nothing: a generic
    /// card written alongside it could silently override the library model the
    /// user chose. Otherwise the placement resolves onto its family's
    /// foundation card, and that card's authored text is written into the deck
    /// so the deck stays self-contained — the same bytes the engine's embedded
    /// fallback would have applied, keyed by card name so a hundred bare
    /// transistors of one family share one card rather than minting a hundred.
    ///
    /// `None` when the symbol has no foundation family, which leaves the
    /// caller to emit no instance line rather than one naming a model that
    /// does not exist.
    pub(super) fn get_default_device_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> Option<String> {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return Some(model_name.to_string());
        }

        let card = foundation_family(component.kind)?.default_model_name();
        if let Some(source) = foundation_card_source(card) {
            self.models
                .entry(card.to_owned())
                .or_insert_with(|| source.to_owned());
        }
        Some(card.to_owned())
    }

    /// Get a current-controlled switch model (CSW) and add to models.
    pub(super) fn get_iswitch_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("isw_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::state::parse_params_string(&component.params);
            let it = Self::get_param_owned(&params, "it", "", "1m");
            let ih = Self::get_param_owned(&params, "ih", "", "0");
            let ron = Self::get_param_owned(&params, "ron", "", "1");
            let roff = Self::get_param_owned(&params, "roff", "", "1meg");
            let smooth = optional_model_param(&params, "smooth", "SMOOTH");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CSW (IT={} IH={} RON={} ROFF={}{})",
                    model_name, it, ih, ron, roff, smooth
                ),
            );
        }

        model_name
    }

    /// Get an expression-controlled switch model card.
    ///
    /// Deliberately not the `SW` card the voltage switch synthesizes: the
    /// generic-switch builder only honours ON/OFF/ONH/OFFH/RON/ROFF and
    /// silently drops everything else, so a VT/VH card would leave the
    /// device on its default 0..1 control window whatever the user typed.
    pub(super) fn get_generic_switch_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("sw_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::state::parse_params_string(&component.params);
            let on = Self::get_param_owned(&params, "on", "", "1");
            let off = Self::get_param_owned(&params, "off", "", "0");
            let ron = Self::get_param_owned(&params, "ron", "", "1");
            let roff = Self::get_param_owned(&params, "roff", "", "1e12");
            let onh = optional_model_param(&params, "onh", "ONH");
            let offh = optional_model_param(&params, "offh", "OFFH");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} SW (ON={} OFF={} RON={} ROFF={}{}{})",
                    model_name, on, off, ron, roff, onh, offh
                ),
            );
        }

        model_name
    }

    /// Get a lossy transmission-line model card (LTRA by default, TXL when
    /// selected) carrying the per-length RLGC values.
    pub(super) fn get_lossy_tline_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let params = crate::state::parse_params_string(&component.params);
        let txl = params.get("kind").is_some_and(|kind| kind == "txl");
        let model_name = format!("{}_{}", if txl { "txl" } else { "ltra" }, component.name);

        if !self.models.contains_key(&model_name) {
            let r = Self::get_param_owned(&params, "r", "", "1");
            let l = Self::get_param_owned(&params, "l", "", "250n");
            let c = Self::get_param_owned(&params, "c", "", "100p");
            let len = Self::get_param_owned(&params, "len", "", "1");
            let card = if txl {
                let g = Self::get_param_owned(&params, "g", "", "0");
                format!(
                    ".MODEL {} TXL (R={} L={} G={} C={} LENGTH={})",
                    model_name, r, l, g, c, len
                )
            } else {
                // The native LTRA runtime requires G=0.
                format!(
                    ".MODEL {} LTRA (R={} L={} C={} G=0 LEN={})",
                    model_name, r, l, c, len
                )
            };
            self.models.insert(model_name.clone(), card);
        }

        model_name
    }

    /// Get a two-conductor CPL model card. R/L/C/G are upper-triangle
    /// (11, 12, 22) per-length entries; the core validates passivity.
    pub(super) fn get_cpl_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("cpl_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::state::parse_params_string(&component.params);
            let entry = |key: &str, default: &str| Self::get_param_owned(&params, key, "", default);
            // The CPL card body is re-read from raw source text by a
            // line-oriented parser: one matrix key per `+` continuation.
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CPL\n+ R = ({} {} {})\n+ L = ({} {} {})\n+ C = ({} {} {})\n+ G = ({} {} {})\n+ LENGTH = {}",
                    model_name,
                    entry("r11", "0.1"),
                    entry("r12", "0"),
                    entry("r22", "0.1"),
                    entry("l11", "380n"),
                    entry("l12", "60n"),
                    entry("l22", "380n"),
                    entry("c11", "120p"),
                    entry("c12", "-12p"),
                    entry("c22", "120p"),
                    entry("g11", "0"),
                    entry("g12", "0"),
                    entry("g22", "0"),
                    entry("length", "0.1"),
                ),
            );
        }

        model_name
    }

    /// Get a memristor model card (Xyce TEAM, LEVEL=2).
    pub(super) fn get_memristor_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("mem_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::state::parse_params_string(&component.params);
            // The engine rejects a TEAM card carrying any key outside its own
            // set, so only the keys it reads may be forwarded — and every one
            // it reads is offered on the sheet. Blank fields are omitted so the
            // engine's default stands rather than being restated here, where it
            // could drift.
            let mut card = format!(".MODEL {model_name} MEMRISTOR (LEVEL=2");
            for key in [
                "ron", "roff", "xon", "xoff", "xscaling", "kon", "koff", "alphaon", "alphaoff",
                "ion", "ioff", "wt", "d", "p", "j", "aon", "aoff", "wc",
            ] {
                card.push_str(&optional_model_param(&params, key, &key.to_uppercase()));
            }
            card.push(')');
            self.models.insert(model_name.clone(), card);
        }

        model_name
    }

    /// Get the Jiles-Atherton magnetic-core model for a saturable inductor.
    ///
    /// The core parameters (saturation magnetization, anhysteretic shape,
    /// pinning, geometry, turns) ride on a `.MODEL <name> CORE` card built
    /// from the component's property values. An explicit `model=` override
    /// binds a library core model instead.
    pub(super) fn get_saturable_core_model(&mut self, component: &Component) -> String {
        let mut params = crate::state::parse_params_string(&component.params);
        if let Some(model_name) = params
            .remove("model")
            .map(|m| m.trim().to_string())
            .filter(|m| !m.is_empty())
        {
            return model_name;
        }

        let model_name = format!("core_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let ms = Self::get_param_owned(&params, "ms", "", "0.4");
            let a = Self::get_param_owned(&params, "a", "", "200");
            let k = Self::get_param_owned(&params, "k", "", "150");
            let c = Self::get_param_owned(&params, "c", "", "0.1");
            let alpha = Self::get_param_owned(&params, "alpha", "", "1e-4");
            let area = Self::get_param_owned(&params, "area", "", "1e-4");
            let length = Self::get_param_owned(&params, "length", "", "0.1");
            let turns = Self::get_param_owned(&params, "n", "", "100");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CORE (MS={} A={} K={} C={} ALPHA={} AREA={} LENGTH={} N={})",
                    model_name, ms, a, k, c, alpha, area, length, turns
                ),
            );
        }

        model_name
    }

    /// Get voltage-controlled switch model name and add to models
    pub(super) fn get_switch_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

        let model_name = format!("sw_{}", component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            let params = crate::state::parse_params_string(&component.params);
            let vt = Self::get_param_owned(&params, "vt", "", "0");
            let vh = Self::get_param_owned(&params, "vh", "", "0");
            let ron = Self::get_param_owned(&params, "ron", "", "1");
            let roff = Self::get_param_owned(&params, "roff", "", "1e12");
            let smooth = optional_model_param(&params, "smooth", "SMOOTH");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} SW (VT={} VH={} RON={} ROFF={}{})",
                    model_name, vt, vh, ron, roff, smooth
                ),
            );
        }

        model_name
    }

    //-------------------------------------------------------------------------
    // Phase 5: Model Generation
    //-------------------------------------------------------------------------

    /// Generate model statements
    pub(super) fn generate_models(&mut self) {
        if !self.models.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Models".to_string());
            for model_line in self.models.values() {
                self.lines.push(model_line.clone());
            }
        }
    }
}
