//! The name the deck writes a component's element under.
//!
//! The engine knows an element by that name and no other, so every probe,
//! output request and duplicate judgement built from a component asks
//! [`Component::written_element_name`], and the netlister writes each card it
//! heads from the same answer.

use super::{Component, ComponentType};
use rspice_app_types::hierarchy_path::{InstancePath, ProbeTarget};

/// What makes the deck write a controlled source as a behavioural `B` card: a
/// polynomial or an output limit, neither of which a linear E/G/F/H card can
/// carry.
///
/// The netlister builds the behavioural expression from exactly these values,
/// so whether a source is lowered and what the lowered card says are one
/// decision.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BehavioralLowering {
    /// The `poly` coefficients, as authored.
    pub polynomial: Option<String>,
    /// The lower output limit: `vmin` for a voltage output, `imin` for a
    /// current one.
    pub lower: Option<String>,
    /// The upper output limit: `vmax` or `imax`.
    pub upper: Option<String>,
}

impl Component {
    /// How the deck lowers this component to a behavioural source, or `None`
    /// when it writes the component's own card.
    ///
    /// A VCVS, VCCS, CCVS or CCCS is lowered by a polynomial or by either
    /// output limit. An op-amp is written as a VCVS and is lowered by either
    /// rail; it has no polynomial, and a blank rail is no rail.
    pub fn behavioral_lowering(&self) -> Option<BehavioralLowering> {
        let (upper_key, lower_key) = match self.kind {
            ComponentType::OpAmp | ComponentType::Vcvs | ComponentType::Ccvs => ("vmax", "vmin"),
            ComponentType::Vccs | ComponentType::Cccs => ("imax", "imin"),
            _ => return None,
        };
        let params = crate::parameters::parse_params_string(&self.params);
        let lowering = if self.kind == ComponentType::OpAmp {
            let rail = |key: &str| {
                params
                    .get(key)
                    .filter(|value| !value.trim().is_empty())
                    .cloned()
            };
            BehavioralLowering {
                polynomial: None,
                lower: rail(lower_key),
                upper: rail(upper_key),
            }
        } else {
            BehavioralLowering {
                polynomial: params
                    .get("poly")
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned),
                lower: params.get(lower_key).cloned(),
                upper: params.get(upper_key).cloned(),
            }
        };
        (lowering.polynomial.is_some() || lowering.lower.is_some() || lowering.upper.is_some())
            .then_some(lowering)
    }

    /// The name the deck writes this component's element under, which is the
    /// one name the engine knows it by.
    ///
    /// That is [`Component::emitted_instance_name`] — the drawn name under its
    /// device prefix — unless the deck writes the component as another
    /// element: a controlled source with a polynomial or an output limit is
    /// the behavioural source named for it, `B` in place of its own prefix, so
    /// a limited `E1` is `B1`. The prefix is matched as the reference spells
    /// it, which is how every deck already written spells the lowered card.
    pub fn written_element_name(&self) -> String {
        let emitted = self.emitted_instance_name();
        if self.behavioral_lowering().is_none() {
            return emitted;
        }
        let suffix = emitted
            .strip_prefix(self.kind.spice_prefix())
            .unwrap_or(&emitted);
        format!("{}{suffix}", ComponentType::BehavioralSource.spice_prefix())
    }
}

/// The component drawn on the sheet of `occurrence` that the engine names
/// `device`, a device name read off a result.
///
/// The engine's name carries the written names on both sides of the drawing:
/// `xamp.rc4` is, on the sheet of `/amp`, the resistor drawn `C4`. So the scope
/// is matched by written instance names and the device by its written element
/// name. `None` when the device is read in another occurrence, or when no
/// component on the sheet writes it.
pub fn component_for_engine_device<'a>(
    components: &'a [Component],
    occurrence: &InstancePath,
    device: &str,
) -> Option<&'a Component> {
    let device = device.trim();
    let (scope, leaf) = match ProbeTarget::parse(device) {
        Ok(target) => (target.scope, target.leaf),
        Err(_) => (InstancePath::root(), device.to_owned()),
    };
    if !scope.same_engine_scope(occurrence) {
        return None;
    }
    components
        .iter()
        .filter(|component| !component.kind.spice_prefix().is_empty())
        .find(|component| component.written_element_name().eq_ignore_ascii_case(&leaf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design_model::Point;

    #[test]
    fn a_device_read_off_a_result_finds_the_component_that_writes_it() {
        let mut limited = drawn(ComponentType::Vcvs, "E1", "vmax=1");
        limited.id = 8;
        let components = [drawn(ComponentType::Resistor, "C4", ""), limited];
        let amp = InstancePath::parse("/amp").expect("drawn occurrence");
        let root = InstancePath::root();

        let found = |occurrence: &InstancePath, device: &str| {
            component_for_engine_device(&components, occurrence, device).map(|found| found.id)
        };
        assert_eq!(found(&amp, "xamp.rc4"), Some(7));
        assert_eq!(found(&amp, "XAMP.B1"), Some(8));
        assert_eq!(found(&root, "RC4"), Some(7));
        assert_eq!(
            found(&root, "C4"),
            None,
            "the drawn name is not the element"
        );
        assert_eq!(found(&root, "E1"), None, "a lowered source is its B card");
        assert_eq!(
            found(&amp, "rc4"),
            None,
            "a root device is not on this sheet"
        );
        assert_eq!(found(&root, "xamp.rc4"), None);
    }

    fn drawn(kind: ComponentType, name: &str, params: &str) -> Component {
        let mut component = Component::new(7, kind, Point::new(0, 0)).with_name_value(name, "1");
        component.params = params.to_owned();
        component
    }

    #[test]
    fn a_component_is_written_under_its_prefixed_name_unless_it_is_lowered() {
        for (kind, name, params, written) in [
            (ComponentType::Resistor, "C4", "", "RC4"),
            (ComponentType::Resistor, "", "", "R7"),
            (ComponentType::Vcvs, "E1", "", "E1"),
            (ComponentType::Vcvs, "E1", "vmax=1", "B1"),
            (ComponentType::Vcvs, "E1", "vmin=-1", "B1"),
            (ComponentType::Vcvs, "E1", "poly=\"0,1,0.1\"", "B1"),
            (ComponentType::Vcvs, "amp", "vmax=1", "Bamp"),
            (ComponentType::Vcvs, "e1", "vmax=1", "Be1"),
            (ComponentType::Vccs, "G2", "imax=1m", "B2"),
            (ComponentType::Vccs, "G2", "vmax=1", "G2"),
            (ComponentType::Ccvs, "H3", "poly=\"0,2\"", "B3"),
            (ComponentType::Cccs, "F4", "imin=-1m", "B4"),
            (ComponentType::OpAmp, "E5", "vmax=12 vmin=-12", "B5"),
            (ComponentType::OpAmp, "E5", "", "E5"),
            (ComponentType::OpAmp, "E5", "poly=\"0,1\"", "E5"),
            (ComponentType::BehavioralSource, "B1", "vmax=1", "B1"),
            (ComponentType::CellInstance, "amp", "", "Xamp"),
        ] {
            assert_eq!(
                drawn(kind, name, params).written_element_name(),
                written,
                "{kind:?} drawn {name:?} with {params:?}"
            );
        }
    }

    #[test]
    fn the_lowering_carries_the_values_the_card_is_written_from() {
        let lowering = drawn(
            ComponentType::Ccvs,
            "H1",
            "vmax=10 poly=\"0,2\" vref=Vsense",
        )
        .behavioral_lowering()
        .expect("a limit lowers the source");
        assert_eq!(lowering.polynomial.as_deref(), Some("0,2"));
        assert_eq!(lowering.upper.as_deref(), Some("10"));
        assert_eq!(lowering.lower, None);
        assert_eq!(
            drawn(ComponentType::OpAmp, "E1", "vmax=").behavioral_lowering(),
            None,
            "a blank rail is no rail"
        );
    }
}
