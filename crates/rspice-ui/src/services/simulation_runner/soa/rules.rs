//! Explicit, scoped terminal-stress rules and their resolved device bindings.

use super::terminals::{TerminalLayout, TerminalLayouts};
use super::*;

/// An additional limit, or an override of a default voltage limit on its scope.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaRuleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_envelope: Option<crate::services::safety::SoaCurrentEnvelope>,
    #[serde(
        default,
        skip_serializing_if = "crate::services::safety::SoaDurationMode::is_default"
    )]
    pub duration_mode: crate::services::safety::SoaDurationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_duration_s: Option<f64>,
    #[serde(default)]
    pub power_derating: Option<crate::services::safety::SoaPowerDerating>,
    pub parameter: SoAParameter,
    #[serde(default)]
    pub voltage_basis: SoaVoltageBasis,
    /// Maximum stress in SI units; temperature is absolute kelvin.
    pub max_value: f64,
    #[serde(default)]
    pub devices: Vec<String>,
    #[serde(default)]
    pub models: Vec<String>,
}

impl SoaRuleConfig {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(curve) = &self.current_envelope {
            curve.validate()?;
            if self.max_value <= 0.0 {
                return Err(
                    "SOA current/voltage curves require a positive maximum-current cap".into(),
                );
            }
            if crate::services::safety::SoaCurrentEnvelope::voltage_parameter(self.parameter)
                .is_none()
            {
                return Err("SOA current/voltage curves apply to Id, Ic or Ia (including directional rules)".into());
            }
        }
        self.duration_mode.validate(self.minimum_duration_s)?;
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
                | SoAParameter::Ig
                | SoAParameter::Is
                | SoAParameter::Ib
                | SoAParameter::Ie
                | SoAParameter::Temp
                | SoAParameter::Pdiss
                | SoAParameter::Vcsub
                | SoAParameter::Vbsub
                | SoAParameter::Vesub
                | SoAParameter::Isub
                | SoAParameter::Vak
                | SoAParameter::Ia
                | SoAParameter::Vbs
                | SoAParameter::Vbd
                | SoAParameter::Vgb
                | SoAParameter::Ibulk
                | SoAParameter::Ves
                | SoAParameter::Ved
                | SoAParameter::Vge
                | SoAParameter::Ibackgate
                | SoAParameter::VbodyBackgate
        ) {
            return Err(
                "SOA rules support magnitudes and positive/negative limits for Vgs, Vds, Vgd, Vbe, Vce, Vbc, Id, Ig, Is, Ic, Ib and Ie, external body/back-gate/substrate voltage and current, diode anode voltage/current, plus conductive power and absolute operating temperature".into(),
            );
        }
        if !self.max_value.is_finite()
            || self.max_value < 0.0
            || (self.max_value == 0.0 && self.parameter.polarity().is_none())
        {
            return Err("SOA rule limit must be finite and positive (zero is allowed for directional rules)".into());
        }
        if self.voltage_basis == SoaVoltageBasis::IntrinsicNodes
            && self.parameter.intrinsic_voltage_parameter().is_none()
        {
            return Err("Intrinsic voltage basis applies only to voltage rules".into());
        }
        if let Some(curve) = self.power_derating {
            curve.validate()?;
            if self.parameter != SoAParameter::Pdiss {
                return Err("SOA temperature derating applies only to conductive power".into());
            }
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

    fn scope_matches(&self, element: &Element) -> bool {
        (self.devices.is_empty()
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

    fn matches(&self, element: &Element, layouts: &TerminalLayouts) -> bool {
        self.scope_matches(element)
            && if self.voltage_basis == SoaVoltageBasis::IntrinsicNodes {
                intrinsic_available(element, self.parameter, layouts)
            } else {
                applicable(element, self.parameter, layouts.get(&element.name).copied())
            }
    }
}

fn intrinsic_available(
    element: &Element,
    parameter: SoAParameter,
    layouts: &TerminalLayouts,
) -> bool {
    layouts.get(&element.name).is_some_and(|layout| {
        layout.intrinsic_voltages & (1u128 << parameter.base_parameter() as u32) != 0
    })
}

pub(super) fn applicable(
    element: &Element,
    parameter: SoAParameter,
    layout: Option<TerminalLayout>,
) -> bool {
    if parameter.requires_terminal_layout() {
        let eligible = (parameter.requires_mos_layout()
            && matches!(element.kind, ElementKind::Mosfet { .. }))
            || (parameter.requires_bjt_layout() && matches!(element.kind, ElementKind::Bjt { .. }));
        return eligible
            && (terminal_pair(parameter, layout).is_some()
                || current_terminal(parameter, layout).is_some());
    }
    let parameter = parameter.base_parameter();
    if matches!(parameter, SoAParameter::Temp | SoAParameter::Pdiss) {
        return matches!(
            element.kind,
            ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. }
                | ElementKind::Bjt { .. }
                | ElementKind::Diode { .. }
        );
    }
    match element.kind {
        ElementKind::Diode { .. } => matches!(parameter, SoAParameter::Vak | SoAParameter::Ia),
        ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
            matches!(
                parameter,
                SoAParameter::Vgs
                    | SoAParameter::Vds
                    | SoAParameter::Vgd
                    | SoAParameter::Id
                    | SoAParameter::Ig
                    | SoAParameter::Is
            )
        }
        ElementKind::Bjt { .. } => {
            matches!(
                parameter,
                SoAParameter::Vbe
                    | SoAParameter::Vce
                    | SoAParameter::Vbc
                    | SoAParameter::Ic
                    | SoAParameter::Ib
                    | SoAParameter::Ie
            )
        }
        _ => false,
    }
}

pub(super) fn resolve(
    elements: &[Element],
    config: &SoaRunConfig,
    layouts: &TerminalLayouts,
    model_limits: &super::model_ratings::ModelLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(usize, SoADefinition)>> {
    for (index, rule) in config.rules.iter().enumerate() {
        poll_periodically(abort, index)?;
        rule.scope()
            .validate_selection(elements)
            .map_err(ServiceRunError::Failure)?;
        if rule.voltage_basis == SoaVoltageBasis::IntrinsicNodes {
            for (element_index, element) in elements.iter().enumerate() {
                poll_periodically(abort, element_index)?;
                // A model's internal body exists independently of an authored
                // bulk contact (for example a floating-body SOI instance).
                let body_voltage = matches!(element.kind, ElementKind::Mosfet { .. })
                    && matches!(
                        rule.parameter.base_parameter(),
                        SoAParameter::Vbs | SoAParameter::Vbd | SoAParameter::Vgb
                    );
                if config.observation.includes(element)
                    && rule.scope_matches(element)
                    && (body_voltage
                        || applicable(element, rule.parameter, layouts.get(&element.name).copied()))
                    && !intrinsic_available(element, rule.parameter, layouts)
                {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA device '{}' does not expose the requested intrinsic {} voltage",
                        element.name,
                        rule.parameter.stress_code()
                    )));
                }
            }
        }
        if !elements
            .iter()
            .any(|element| config.observation.includes(element) && rule.matches(element, layouts))
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
            if enabled && applicable(element, parameter, layouts.get(&element.name).copied()) {
                limits.insert(
                    parameter,
                    explicit_limit(parameter, max_value, SoaVoltageBasis::ExternalTerminals),
                );
            }
        }
        if let Some(imported) = model_limits.get(&element.name) {
            for parameter in imported.keys() {
                let base = parameter.base_parameter();
                if let Some(default) = limits.remove(&base)
                    && parameter.polarity().is_some()
                    && !imported.contains_key(&base)
                    && let Some((positive, negative)) = base.directional_pair()
                {
                    // A card that rates only one polarity must not disable
                    // the user's configured default for the unrated side.
                    for half in [positive, negative] {
                        limits.insert(
                            half,
                            SoALimit {
                                duration_mode: Default::default(),
                                minimum_duration_s: None,
                                current_envelope: None,
                                power_derating: None,
                                parameter: half,
                                ..default.clone()
                            },
                        );
                    }
                }
            }
            limits.extend(
                imported
                    .iter()
                    .map(|(parameter, limit)| (*parameter, limit.clone())),
            );
        }
        // A directional override replaces that half of an inherited symmetric
        // default; the other half retains its default protection. An explicit
        // magnitude rule remains an independently authored constraint.
        for rule in &config.rules {
            let base = rule.parameter.base_parameter();
            if rule.matches(element, layouts)
                && rule.parameter.polarity().is_some()
                && !config
                    .rules
                    .iter()
                    .any(|r| r.matches(element, layouts) && r.parameter == base)
                && let Some(maximum) = limits.remove(&base)
                && let Some((positive, negative)) = base.directional_pair()
            {
                for half in [positive, negative] {
                    // A BSIM card can supply both a symmetric fallback and a
                    // tighter directional limit. Splitting the fallback must
                    // preserve that tighter protection on the untouched side.
                    if limits
                        .get(&half)
                        .is_none_or(|old| maximum.max_value < old.max_value)
                    {
                        limits.insert(
                            half,
                            SoALimit {
                                duration_mode: Default::default(),
                                minimum_duration_s: None,
                                current_envelope: None,
                                power_derating: None,
                                parameter: half,
                                ..maximum.clone()
                            },
                        );
                    }
                }
            }
        }
        let mut overridden = std::collections::HashSet::new();
        for rule in &config.rules {
            if rule.matches(element, layouts) {
                if !overridden.insert(rule.parameter) {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA has overlapping explicit {} rules for '{}'",
                        rule.parameter.stress_code(),
                        element.name
                    )));
                }
                // A magnitude override replaces inherited directional limits
                // from an asymmetric model rating. Explicit directional rules
                // remain independent regardless of their declaration order.
                if rule.parameter.polarity().is_none()
                    && let Some((positive, negative)) = rule.parameter.directional_pair()
                {
                    for half in [positive, negative] {
                        if !config
                            .rules
                            .iter()
                            .any(|r| r.matches(element, layouts) && r.parameter == half)
                        {
                            limits.remove(&half);
                        }
                    }
                }
                let mut limit = explicit_limit(rule.parameter, rule.max_value, rule.voltage_basis);
                limit.minimum_duration_s = rule.minimum_duration_s;
                limit.duration_mode = rule.duration_mode;
                limit.power_derating = rule.power_derating;
                limit.current_envelope = rule.current_envelope.clone();
                if let Some(curve) = &rule.current_envelope {
                    limit.description.push_str(&format!(
                        "; current/voltage curve: {}; conditions: {}; {}",
                        curve.source,
                        curve.conditions,
                        curve
                            .pulse_width_s
                            .map_or_else(|| "DC".into(), |width| format!("pulse {width} s"))
                    ));
                }
                if let Some(curve) = rule.power_derating {
                    limit.description.push_str(&format!("; rated {} W through {} K, derated by {} W/K above that temperature, clamped to zero", rule.max_value, curve.reference_temperature_kelvin, curve.watts_per_kelvin));
                }
                limits.insert(rule.parameter, limit);
            }
        }
        if !limits.is_empty() {
            let mut definition = SoADefinition::new();
            for (parameter, limit) in limits {
                if limit.voltage_basis == SoaVoltageBasis::IntrinsicNodes
                    && !intrinsic_available(element, parameter, layouts)
                {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA device '{}' does not expose the requested intrinsic {} voltage",
                        element.name,
                        parameter.stress_code()
                    )));
                }
                if limit.voltage_basis == SoaVoltageBasis::ExternalTerminals
                    && !applicable(element, parameter, layouts.get(&element.name).copied())
                {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA device '{}' does not expose the requested external electrical {} terminals; substrate rules require an explicit electrical substrate pin, not a thermal pin",
                        element.name,
                        parameter.stress_code()
                    )));
                }
                definition.add_limit(limit);
            }
            resolved.push((index, definition));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(resolved)
}

fn explicit_limit(
    parameter: SoAParameter,
    max_value: f64,
    voltage_basis: SoaVoltageBasis,
) -> SoALimit {
    SoALimit {
        duration_mode: Default::default(),
        minimum_duration_s: None,
        current_envelope: None,
        power_derating: None,
        voltage_basis,
        parameter,
        max_value,
        unit: match parameter.base_parameter() {
            p if p.is_current() => "A",
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
                "Maximum {} {} at {}",
                parameter.base_parameter().stress_code(),
                match parameter.polarity() {
                    Some(true) => "positive part",
                    Some(false) => "negative part magnitude",
                    None => "magnitude",
                },
                match voltage_basis {
                    SoaVoltageBasis::ExternalTerminals => "authored terminals",
                    SoaVoltageBasis::IntrinsicNodes => "intrinsic electrical model nodes",
                }
            )
        },
    }
}

pub(super) fn observation_parameter(limit: &SoALimit) -> Option<&'static str> {
    if limit.voltage_basis == SoaVoltageBasis::IntrinsicNodes {
        return limit.parameter.intrinsic_voltage_parameter();
    }
    device_parameter(limit.parameter)
}

pub(super) fn device_parameter(parameter: SoAParameter) -> Option<&'static str> {
    match parameter.base_parameter() {
        SoAParameter::Temp => Some("temp"),
        SoAParameter::Pdiss => Some("power"),
        _ => None,
    }
}

pub(super) fn terminal_pair(
    parameter: SoAParameter,
    layout: Option<TerminalLayout>,
) -> Option<(usize, usize)> {
    match parameter.base_parameter() {
        SoAParameter::Vgs | SoAParameter::Vbe => Some((1, 2)),
        SoAParameter::Vds | SoAParameter::Vce => Some((0, 2)),
        SoAParameter::Vgd | SoAParameter::Vbc => Some((1, 0)),
        SoAParameter::Vbs => Some((layout?.body?, 2)),
        SoAParameter::Vbd => Some((layout?.body?, 0)),
        SoAParameter::Vgb => Some((1, layout?.body?)),
        SoAParameter::Ves => Some((layout?.back_gate?, 2)),
        SoAParameter::Ved => Some((layout?.back_gate?, 0)),
        SoAParameter::Vge => Some((1, layout?.back_gate?)),
        SoAParameter::VbodyBackgate => Some((layout?.body?, layout?.back_gate?)),
        SoAParameter::Vcsub => Some((0, layout?.substrate?)),
        SoAParameter::Vbsub => Some((1, layout?.substrate?)),
        SoAParameter::Vesub => Some((2, layout?.substrate?)),
        SoAParameter::Vak => Some((0, 1)),
        _ => None,
    }
}

/// Authored terminal index, with current positive into the device.
pub(super) fn current_terminal(
    parameter: SoAParameter,
    layout: Option<TerminalLayout>,
) -> Option<usize> {
    match parameter.base_parameter() {
        SoAParameter::Id | SoAParameter::Ic => Some(0),
        SoAParameter::Ig | SoAParameter::Ib => Some(1),
        SoAParameter::Is | SoAParameter::Ie => Some(2),
        SoAParameter::Ibulk => layout?.body,
        SoAParameter::Ibackgate => layout?.back_gate,
        SoAParameter::Isub => layout?.substrate,
        SoAParameter::Ia => Some(0),
        _ => None,
    }
}
