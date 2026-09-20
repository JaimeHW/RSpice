//! Explicit, scoped terminal-stress rules and their resolved device bindings.

use super::*;

/// An additional limit, or an override of a default voltage limit on its scope.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaRuleConfig {
    pub parameter: SoAParameter,
    /// Maximum stress in SI units; temperature is absolute kelvin.
    pub max_value: f64,
    #[serde(default)]
    pub devices: Vec<String>,
    #[serde(default)]
    pub models: Vec<String>,
}

impl SoaRuleConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !matches!(
            self.parameter.base_parameter(),
            SoAParameter::Vgs
                | SoAParameter::Vds
                | SoAParameter::Vgd
                | SoAParameter::Vbe
                | SoAParameter::Vce
                | SoAParameter::Vbc
                | SoAParameter::Id
                | SoAParameter::Ic
                | SoAParameter::Temp
                | SoAParameter::Pdiss
        ) {
            return Err(
                "SOA rules support magnitudes and positive/negative limits for Vgs, Vds, Vgd, Vbe, Vce, Vbc, Id and Ic, plus conductive power and absolute operating temperature".into(),
            );
        }
        if !self.max_value.is_finite()
            || self.max_value < 0.0
            || (self.max_value == 0.0 && self.parameter.polarity().is_none())
        {
            return Err("SOA rule limit must be finite and positive (zero is allowed for directional rules)".into());
        }
        self.scope().validate(1.0)
    }

    fn scope(&self) -> SoaObservationConfig {
        SoaObservationConfig {
            devices: self.devices.clone(),
            models: self.models.clone(),
            ..Default::default()
        }
    }

    fn matches(&self, element: &Element) -> bool {
        applicable(element, self.parameter)
            && (self.devices.is_empty()
                || self
                    .devices
                    .iter()
                    .any(|name| observation::same_name(name, &element.name)))
            && (self.models.is_empty()
                || SoaObservationConfig::model(element).is_some_and(|model| {
                    self.models
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(model))
                }))
    }
}

pub(super) fn applicable(element: &Element, parameter: SoAParameter) -> bool {
    let parameter = parameter.base_parameter();
    if matches!(parameter, SoAParameter::Temp | SoAParameter::Pdiss) {
        return matches!(
            element.kind,
            ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. }
                | ElementKind::Bjt { .. }
        );
    }
    match element.kind {
        ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
            matches!(
                parameter,
                SoAParameter::Vgs | SoAParameter::Vds | SoAParameter::Vgd | SoAParameter::Id
            )
        }
        ElementKind::Bjt { .. } => {
            matches!(
                parameter,
                SoAParameter::Vbe | SoAParameter::Vce | SoAParameter::Vbc | SoAParameter::Ic
            )
        }
        _ => false,
    }
}

pub(super) fn resolve(
    elements: &[Element],
    config: &SoaRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(usize, SoADefinition)>> {
    for (index, rule) in config.rules.iter().enumerate() {
        poll_periodically(abort, index)?;
        rule.scope()
            .validate_selection(elements)
            .map_err(ServiceRunError::Failure)?;
        if !elements
            .iter()
            .any(|element| config.observation.includes(element) && rule.matches(element))
        {
            return Err(ServiceRunError::Failure(format!(
                "SOA rule {} ({}) matches no applicable device inside the observation selection",
                index + 1,
                rule.parameter.stress_code()
            )));
        }
    }
    let mut resolved = Vec::new();
    for (index, element) in elements.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !config.observation.includes(element) {
            continue;
        }
        let mut limits = std::collections::BTreeMap::new();
        for (enabled, parameter, max_value) in [
            (config.check_vgs_max, SoAParameter::Vgs, config.max_vgs),
            (config.check_vds_max, SoAParameter::Vds, config.max_vds),
            (config.check_vbe_max, SoAParameter::Vbe, config.max_vbe),
            (config.check_vce_max, SoAParameter::Vce, config.max_vce),
        ] {
            if enabled && applicable(element, parameter) {
                limits.insert(parameter, max_value);
            }
        }
        // A directional override replaces that half of an inherited symmetric
        // default; the other half retains its default protection. An explicit
        // magnitude rule remains an independently authored constraint.
        for rule in &config.rules {
            let base = rule.parameter.base_parameter();
            if rule.matches(element)
                && rule.parameter.polarity().is_some()
                && !config
                    .rules
                    .iter()
                    .any(|r| r.matches(element) && r.parameter == base)
                && let Some(maximum) = limits.remove(&base)
                && let Some((positive, negative)) = base.directional_pair()
            {
                limits.insert(positive, maximum);
                limits.insert(negative, maximum);
            }
        }
        let mut overridden = std::collections::HashSet::new();
        for rule in &config.rules {
            if rule.matches(element) {
                if !overridden.insert(rule.parameter) {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA has overlapping explicit {} rules for '{}'",
                        rule.parameter.stress_code(),
                        element.name
                    )));
                }
                limits.insert(rule.parameter, rule.max_value);
            }
        }
        if !limits.is_empty() {
            let mut definition = SoADefinition::new();
            for (parameter, max_value) in limits {
                definition.add_limit(SoALimit {
                    parameter,
                    max_value,
                    unit: match parameter.base_parameter() {
                        SoAParameter::Id | SoAParameter::Ic => "A",
                        SoAParameter::Temp => "K",
                        SoAParameter::Pdiss => "W",
                        _ => "V",
                    }
                    .into(),
                    description: if parameter == SoAParameter::Temp {
                        "Maximum absolute operating temperature used by the device model".into()
                    } else if parameter == SoAParameter::Pdiss {
                        "Maximum positive conductive device power, including series losses; excludes stored-energy exchange".into()
                    } else {
                        format!(
                            "Maximum {} {} at authored terminals",
                            parameter.base_parameter().stress_code(),
                            match parameter.polarity() {
                                Some(true) => "positive part",
                                Some(false) => "negative part magnitude",
                                None => "magnitude",
                            }
                        )
                    },
                });
            }
            resolved.push((index, definition));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(resolved)
}

pub(super) fn device_parameter(parameter: SoAParameter) -> Option<&'static str> {
    match parameter.base_parameter() {
        SoAParameter::Id => Some("id"),
        SoAParameter::Ic => Some("ic"),
        SoAParameter::Temp => Some("temp"),
        SoAParameter::Pdiss => Some("power"),
        _ => None,
    }
}

pub(super) fn terminal_pair(parameter: SoAParameter) -> Option<(usize, usize)> {
    match parameter.base_parameter() {
        SoAParameter::Vgs | SoAParameter::Vbe => Some((1, 2)),
        SoAParameter::Vds | SoAParameter::Vce => Some((0, 2)),
        SoAParameter::Vgd | SoAParameter::Vbc => Some((1, 0)),
        _ => None,
    }
}
