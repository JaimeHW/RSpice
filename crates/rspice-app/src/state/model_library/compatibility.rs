//! Authoritative schematic-instance/model family compatibility.
//!
//! Catalog presentation and the binding transaction both use this module so a
//! model cannot appear bindable and then be accepted under a different rule.
//! Runtime parameter validation remains the circuit builder's responsibility;
//! this gate owns the structural device family and polarity contract.

use crate::state::ComponentType;

use super::{DeviceModel, ModelLevel, ModelType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelDeviceFamily {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    NpnBjt,
    PnpBjt,
    Nmos,
    Pmos,
    NVdmos,
    PVdmos,
    NmosSoi,
    PmosSoi,
    Njfet,
    Pjfet,
    Nmesfet,
    Pmesfet,
    MagneticCore,
    Memristor,
    VoltageSwitch,
    CurrentSwitch,
    GenericSwitch,
    LossyTransmissionLine,
    CoupledTransmissionLine,
}

impl ModelDeviceFamily {
    const fn label(self) -> &'static str {
        match self {
            Self::Resistor => "resistor",
            Self::Capacitor => "capacitor",
            Self::Inductor => "inductor",
            Self::Diode => "diode/varactor",
            Self::NpnBjt => "NPN BJT",
            Self::PnpBjt => "PNP BJT",
            Self::Nmos => "bulk NMOS",
            Self::Pmos => "bulk PMOS",
            Self::NVdmos => "N-channel VDMOS",
            Self::PVdmos => "P-channel VDMOS",
            Self::NmosSoi => "N-channel SOI MOS",
            Self::PmosSoi => "P-channel SOI MOS",
            Self::Njfet => "N-channel JFET",
            Self::Pjfet => "P-channel JFET",
            Self::Nmesfet => "N-channel MESFET/HFET",
            Self::Pmesfet => "P-channel MESFET/HFET",
            Self::MagneticCore => "nonlinear magnetic core",
            Self::Memristor => "memristor",
            Self::VoltageSwitch => "voltage-controlled switch",
            Self::CurrentSwitch => "current-controlled switch",
            Self::GenericSwitch => "generic switch",
            Self::LossyTransmissionLine => "lossy transmission line",
            Self::CoupledTransmissionLine => "coupled transmission line",
        }
    }
}

/// Reject a structurally invalid primitive binding before schematic mutation.
pub(crate) fn validate_component_model_compatibility(
    component: ComponentType,
    model: &DeviceModel,
) -> Result<(), String> {
    let Some(actual) = model_device_family(model) else {
        return Err(format!(
            "Model '{}' declares type '{}', which has no schematic binding contract.",
            model.name,
            model_type_label(model)
        ));
    };
    if component_accepts_family(component, actual) {
        return Ok(());
    }

    Err(format!(
        "Model '{}' is a {}, which is incompatible with the selected {} instance.",
        model.name,
        actual.label(),
        component.display_name()
    ))
}

/// The device a card of this family is placed as.
///
/// Derived from [`component_accepts_family`] rather than declared beside it. A
/// second table would be a second answer, and the two would disagree the first
/// time a family gained a symbol: this asks the acceptance contract itself, so
/// a card is only ever offered a device that would accept it.
///
/// The plain form wins because [`ComponentType::ALL`] lists it first — `NpnBjt`
/// ahead of its four- and five-terminal variants, `VSwitch` ahead of the
/// generic switch — which is the right default for a card that says nothing
/// about how many terminals its symbol should have.
pub(crate) fn placement_component_for_model(model: &DeviceModel) -> Option<ComponentType> {
    let family = model_device_family(model)?;
    ComponentType::ALL
        .into_iter()
        .find(|component| component_accepts_family(*component, family))
}

/// A model-bound library cell may move only within the same structural family.
pub(crate) fn models_have_compatible_device_family(
    current: &DeviceModel,
    candidate: &DeviceModel,
) -> bool {
    match (model_device_family(current), model_device_family(candidate)) {
        (Some(current), Some(candidate)) => current == candidate,
        (None, None) => {
            current.model_type == candidate.model_type
                && opaque_family_token(current) == opaque_family_token(candidate)
        }
        _ => false,
    }
}

fn opaque_family_token(model: &DeviceModel) -> Option<String> {
    model
        .spice_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_uppercase)
}

fn component_accepts_family(component: ComponentType, family: ModelDeviceFamily) -> bool {
    use ComponentType as Component;
    use ModelDeviceFamily as Family;

    match component {
        Component::Resistor => family == Family::Resistor,
        Component::Capacitor => family == Family::Capacitor,
        Component::Inductor => family == Family::Inductor,
        Component::Diode => family == Family::Diode,
        Component::NpnBjt | Component::NpnBjt4 | Component::NpnBjt5 => family == Family::NpnBjt,
        Component::PnpBjt | Component::PnpBjt4 | Component::PnpBjt5 => family == Family::PnpBjt,
        Component::Nmos => family == Family::Nmos,
        Component::Pmos => family == Family::Pmos,
        Component::NVdmos => family == Family::NVdmos,
        Component::PVdmos => family == Family::PVdmos,
        Component::NmosSoi => family == Family::NmosSoi,
        Component::PmosSoi => family == Family::PmosSoi,
        Component::Njfet => family == Family::Njfet,
        Component::Pjfet => family == Family::Pjfet,
        Component::Nmesfet => family == Family::Nmesfet,
        Component::Pmesfet => family == Family::Pmesfet,
        Component::SaturableInductor => family == Family::MagneticCore,
        Component::Memristor => family == Family::Memristor,
        Component::VSwitch => family == Family::VoltageSwitch,
        Component::ISwitch => family == Family::CurrentSwitch,
        Component::GenericSwitch => matches!(
            family,
            Family::GenericSwitch | Family::VoltageSwitch | Family::CurrentSwitch
        ),
        Component::LossyTransmissionLine => family == Family::LossyTransmissionLine,
        Component::CoupledTransmissionLine => family == Family::CoupledTransmissionLine,
        _ => false,
    }
}

fn model_device_family(model: &DeviceModel) -> Option<ModelDeviceFamily> {
    use ModelDeviceFamily as Family;

    let token = model
        .spice_type
        .as_deref()
        .map(str::trim)
        .unwrap_or_default()
        .to_ascii_uppercase();
    let exact = match token.as_str() {
        "R" | "RES" | "RESISTOR" => Some(Family::Resistor),
        "C" | "CAP" | "CAPACITOR" => Some(Family::Capacitor),
        "L" | "IND" | "INDUCTOR" => Some(Family::Inductor),
        "D" | "DIODE" | "JUNCAP200" | "DIODE_CMC" => Some(Family::Diode),
        "NPN" => Some(Family::NpnBjt),
        "PNP" | "LPNP" => Some(Family::PnpBjt),
        "NJF" => Some(Family::Njfet),
        "PJF" => Some(Family::Pjfet),
        "NMF" | "NHFET" => Some(Family::Nmesfet),
        "PMF" | "PHFET" => Some(Family::Pmesfet),
        "NVDMOS" | "VDMOSN" => Some(Family::NVdmos),
        "PVDMOS" | "VDMOSP" => Some(Family::PVdmos),
        "CORE" | "JA" | "JILES" | "JILESATHERTON" => Some(Family::MagneticCore),
        "MEMRISTOR" => Some(Family::Memristor),
        "VSWITCH" | "VSW" => Some(Family::VoltageSwitch),
        "CSW" | "ISWITCH" | "ISW" => Some(Family::CurrentSwitch),
        "SW" | "SWITCH" => Some(Family::GenericSwitch),
        "LTRA" | "TXL" => Some(Family::LossyTransmissionLine),
        "CPL" => Some(Family::CoupledTransmissionLine),
        _ => None,
    };
    if exact.is_some() {
        return exact;
    }

    // Exhaustive rather than defaulted: a family this build learns to name is
    // a binding contract someone has to decide, and a catch-all would decide
    // it silently as "nothing may bind to this".
    match (model.model_type, model.level) {
        (ModelType::Nmos, ModelLevel::Vdmos) => Some(Family::NVdmos),
        (ModelType::Pmos, ModelLevel::Vdmos) => Some(Family::PVdmos),
        (ModelType::Nmos, ModelLevel::BsimSoi) => Some(Family::NmosSoi),
        (ModelType::Pmos, ModelLevel::BsimSoi) => Some(Family::PmosSoi),
        (ModelType::Nmos, _) => Some(Family::Nmos),
        (ModelType::Pmos, _) => Some(Family::Pmos),
        (ModelType::NVdmos, _) => Some(Family::NVdmos),
        (ModelType::PVdmos, _) => Some(Family::PVdmos),
        (ModelType::Npn, _) => Some(Family::NpnBjt),
        (ModelType::Pnp, _) => Some(Family::PnpBjt),
        (ModelType::Njfet, _) => Some(Family::Njfet),
        (ModelType::Pjfet, _) => Some(Family::Pjfet),
        (ModelType::Nmesfet, _) => Some(Family::Nmesfet),
        (ModelType::Pmesfet, _) => Some(Family::Pmesfet),
        (ModelType::Resistor, _) => Some(Family::Resistor),
        (ModelType::Capacitor, _) => Some(Family::Capacitor),
        (ModelType::Inductor, _) => Some(Family::Inductor),
        (ModelType::Diode | ModelType::Varactor | ModelType::Esd, _) => Some(Family::Diode),
        // Not device families: an RF macro and an unclassified card have no
        // structural contract to check a placement against, so neither binds.
        (ModelType::Rf | ModelType::Other, _) => None,
    }
}

fn model_type_label(model: &DeviceModel) -> &str {
    model
        .spice_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| model.model_type.display_name())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(name: &str, model_type: ModelType, spice_type: &str) -> DeviceModel {
        let mut model = DeviceModel::new(name, model_type);
        model.spice_type = Some(spice_type.to_owned());
        model
    }

    #[test]
    fn primitive_polarity_and_family_mismatches_fail_closed() {
        let pmos = model("pch", ModelType::Pmos, "PMOS");
        let diode = model("clamp", ModelType::Diode, "D");

        assert!(validate_component_model_compatibility(ComponentType::Pmos, &pmos).is_ok());
        assert!(validate_component_model_compatibility(ComponentType::Nmos, &pmos).is_err());
        assert!(validate_component_model_compatibility(ComponentType::Resistor, &diode).is_err());
    }

    #[test]
    fn specialized_mos_symbols_do_not_accept_bulk_cards() {
        let bulk = model("nch", ModelType::Nmos, "NMOS");
        let mut soi = model("nsoi", ModelType::Nmos, "NMOS");
        soi.level = ModelLevel::BsimSoi;

        assert!(validate_component_model_compatibility(ComponentType::Nmos, &bulk).is_ok());
        assert!(validate_component_model_compatibility(ComponentType::NmosSoi, &bulk).is_err());
        assert!(validate_component_model_compatibility(ComponentType::Nmos, &soi).is_err());
        assert!(validate_component_model_compatibility(ComponentType::NmosSoi, &soi).is_ok());
    }

    #[test]
    fn exact_other_family_tokens_remain_bindable() {
        let njf = model("jmod", ModelType::Other, "NJF");
        let ltra = model("line", ModelType::Other, "LTRA");

        assert!(validate_component_model_compatibility(ComponentType::Njfet, &njf).is_ok());
        assert!(validate_component_model_compatibility(ComponentType::Pjfet, &njf).is_err());
        assert!(
            validate_component_model_compatibility(ComponentType::LossyTransmissionLine, &ltra)
                .is_ok()
        );
    }

    #[test]
    fn arbitrary_other_models_and_non_model_components_are_rejected() {
        let unknown = model("mystery", ModelType::Other, "VENDOR_PRIVATE");
        let diode = model("d", ModelType::Diode, "D");

        assert!(validate_component_model_compatibility(ComponentType::Nmos, &unknown).is_err());
        assert!(
            validate_component_model_compatibility(ComponentType::VoltageSource, &diode).is_err()
        );
    }

    #[test]
    fn opaque_cell_models_match_only_the_same_declared_family() {
        let generic_a = model("opa_a", ModelType::Other, "");
        let generic_b = model("opa_b", ModelType::Other, "");
        let vendor_a = model("vendor_a", ModelType::Other, "VENDOR_PRIVATE");
        let vendor_b = model("vendor_b", ModelType::Other, "vendor_private");
        let unrelated = model("unrelated", ModelType::Other, "OTHER_PRIVATE");

        assert!(models_have_compatible_device_family(&generic_a, &generic_b));
        assert!(models_have_compatible_device_family(&vendor_a, &vendor_b));
        assert!(!models_have_compatible_device_family(&vendor_a, &unrelated));
        assert!(!models_have_compatible_device_family(&generic_a, &vendor_a));
    }
}
