//! Process corners.
//!
//! The named corner a model section is selected by, and the default when a
//! design does not choose one.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Functional domain supplied by one named PDK library section.
///
/// A composite section is the conventional SPICE `.lib TT` shape: it owns
/// every device class needed by the point. The remaining domains support PDKs
/// that publish independently selectable device, statistical, and aging
/// sections. They are explicit so an absent BJT or aging binding cannot be
/// silently treated as "typical".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CornerSectionDomain {
    Composite,
    Mos,
    Bjt,
    Passives,
    MacroModels,
    StatisticalGlobal,
    StatisticalLocal,
    Aging,
}

impl CornerSectionDomain {
    pub const ALL: [Self; 8] = [
        Self::Composite,
        Self::Mos,
        Self::Bjt,
        Self::Passives,
        Self::MacroModels,
        Self::StatisticalGlobal,
        Self::StatisticalLocal,
        Self::Aging,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Composite => "Composite",
            Self::Mos => "MOS",
            Self::Bjt => "BJT",
            Self::Passives => "Passives",
            Self::MacroModels => "Macro",
            Self::StatisticalGlobal => "Statistical (global)",
            Self::StatisticalLocal => "Statistical (local)",
            Self::Aging => "Aging",
        }
    }
}

/// One explicit domain-to-section binding inside the corner's authenticated
/// source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CornerSectionBinding {
    pub domain: CornerSectionDomain,
    pub section: String,
}

impl CornerSectionBinding {
    #[must_use]
    pub fn new(domain: CornerSectionDomain, section: impl Into<String>) -> Self {
        Self {
            domain,
            section: section.into(),
        }
    }
}

/// "−40 °C", or "−40 °C and 150 °C", or "−40 °C, 150 °C and 175 °C".
///
/// One rendering, beside the rule that produces the list, because a set of
/// temperatures spelled two ways on two surfaces reads as two different sets —
/// the same reason a digest has exactly one shortening.
#[must_use]
pub fn stated_temperatures(values: &[f64]) -> String {
    let stated = values
        .iter()
        .map(|celsius| format!("{celsius} °C"))
        .collect::<Vec<_>>();
    match stated.as_slice() {
        [] => "nothing".to_owned(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// A process corner definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessCorner {
    /// Corner name (e.g., "tt", "ff", "ss")
    pub name: String,
    /// Description
    pub description: String,
    /// NMOS corner (typical, fast, slow)
    pub nmos_corner: String,
    /// PMOS corner
    pub pmos_corner: String,
    /// Temperature
    pub temperature: f64,
    /// Supply voltage adjustment factor
    pub vdd_factor: f64,
    /// Corner file path
    pub file_path: Option<PathBuf>,
    /// Whether this is the default/typical corner
    pub is_default: bool,
    /// Named sections that compose this corner. Imported conventional
    /// libraries contain one `Composite` binding. Advanced PDKs may bind
    /// independent domains, all of which are materialized for execution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section_bindings: Vec<CornerSectionBinding>,
    /// Domains the PDK declares mandatory for this corner. Validation fails
    /// closed when any required domain has no binding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_domains: Vec<CornerSectionDomain>,
    /// Optional qualified operating range. This is separate from the nominal
    /// point and from the run-set temperature axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_temperature_c: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_temperature_c: Option<f64>,
}

impl Default for ProcessCorner {
    fn default() -> Self {
        Self {
            name: "tt".to_string(),
            description: "Typical-Typical".to_string(),
            nmos_corner: "typical".to_string(),
            pmos_corner: "typical".to_string(),
            temperature: 27.0,
            vdd_factor: 1.0,
            file_path: None,
            is_default: true,
            section_bindings: Vec::new(),
            required_domains: Vec::new(),
            minimum_temperature_c: None,
            maximum_temperature_c: None,
        }
    }
}

impl ProcessCorner {
    /// Create a new corner
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_default: false,
            ..Default::default()
        }
    }

    /// Construct the conventional one-file, one-section corner contract used
    /// by ordinary SPICE `.lib` files.
    #[must_use]
    pub fn from_composite_section(
        name: impl Into<String>,
        file_path: PathBuf,
        is_default: bool,
    ) -> Self {
        let name = name.into();
        Self {
            description: format!("Composite process section {name}"),
            file_path: Some(file_path),
            is_default,
            section_bindings: vec![CornerSectionBinding::new(
                CornerSectionDomain::Composite,
                name.clone(),
            )],
            required_domains: vec![CornerSectionDomain::Composite],
            name,
            ..Default::default()
        }
    }

    /// Effective bindings for execution and validation.
    ///
    /// Projects saved before typed corner contracts existed used
    /// `file_path + name`. Preserve that meaning as one composite binding
    /// instead of migrating it to an unbound draft.
    #[must_use]
    pub fn effective_section_bindings(&self) -> Vec<CornerSectionBinding> {
        if self.section_bindings.is_empty()
            && self.required_domains.is_empty()
            && self.file_path.is_some()
        {
            vec![CornerSectionBinding::new(
                CornerSectionDomain::Composite,
                self.name.clone(),
            )]
        } else {
            self.section_bindings.clone()
        }
    }

    /// Whether a requested temperature lies inside this corner's qualified
    /// range.
    ///
    /// The edge rule lives here and only here, because the Corners page and
    /// the simulation preflight both ask it and two spellings of "inclusive"
    /// would eventually disagree about a run at exactly 125 °C.
    ///
    /// **Bounds are inclusive.** A corner qualified −40 to 125 °C is qualified
    /// *at* −40 °C and *at* 125 °C: a PDK that publishes a range publishes the
    /// endpoints as qualified, not as the first values outside. A bound the
    /// PDK did not declare does not constrain — a corner declaring only a
    /// maximum is unqualified above it and unconstrained below — and a corner
    /// declaring neither qualifies everything, which is the honest reading of
    /// "no range was published" rather than a range invented for it. A
    /// non-finite request qualifies nowhere; it is not a temperature.
    #[must_use]
    pub fn qualifies_temperature(&self, celsius: f64) -> bool {
        celsius.is_finite()
            && self
                .minimum_temperature_c
                .is_none_or(|minimum| celsius >= minimum)
            && self
                .maximum_temperature_c
                .is_none_or(|maximum| celsius <= maximum)
    }

    /// The requested temperatures this corner's qualified range excludes.
    ///
    /// Empty when the corner declares no range at all: a corner that published
    /// nothing excludes nothing, and reporting every temperature against it
    /// would turn an absent claim into a finding on every design.
    #[must_use]
    pub fn temperatures_outside_qualified_range(&self, requested: &[f64]) -> Vec<f64> {
        if self.minimum_temperature_c.is_none() && self.maximum_temperature_c.is_none() {
            return Vec::new();
        }
        requested
            .iter()
            .copied()
            .filter(|celsius| !self.qualifies_temperature(*celsius))
            .collect()
    }

    /// The qualified range in one phrasing, so no two surfaces invent a second.
    #[must_use]
    pub fn qualified_range_label(&self) -> String {
        match (self.minimum_temperature_c, self.maximum_temperature_c) {
            (Some(minimum), Some(maximum)) => format!("{minimum:.3} to {maximum:.3} °C"),
            (Some(minimum), None) => format!("{minimum:.3} °C and above"),
            (None, Some(maximum)) => format!("{maximum:.3} °C and below"),
            (None, None) => "not declared".to_owned(),
        }
    }

    /// Effective mandatory domains, including the legacy composite contract.
    #[must_use]
    pub fn effective_required_domains(&self) -> Vec<CornerSectionDomain> {
        if self.required_domains.is_empty()
            && self.section_bindings.is_empty()
            && self.file_path.is_some()
        {
            vec![CornerSectionDomain::Composite]
        } else {
            self.required_domains.clone()
        }
    }

    /// Validate a persistable authoring draft independently of source parsing.
    ///
    /// Drafts may intentionally leave a required domain unbound. That state is
    /// retained so a multi-step editor can save, reload, undo, and recover its
    /// work; [`Self::validate_contract`] still rejects the draft at every
    /// execution boundary until all required domains are bound.
    pub fn validate_draft_contract(&self) -> Result<(), Vec<String>> {
        self.validate_contract_completeness(false)
    }

    /// Validate an executable corner contract independently of source parsing.
    ///
    /// Source existence, content digests, and section presence are checked by
    /// the sealed model-execution plan. This method additionally requires each
    /// declared domain to have one explicit binding.
    pub fn validate_contract(&self) -> Result<(), Vec<String>> {
        self.validate_contract_completeness(true)
    }

    fn validate_contract_completeness(&self, require_complete: bool) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let name = self.name.trim();
        if name.is_empty() {
            errors.push("corner name cannot be empty".to_owned());
        } else if name.chars().any(|character| {
            character.is_whitespace()
                || character == '"'
                || character == '\''
                || character.is_control()
        }) {
            errors.push(format!(
                "corner name contains an unsupported character: {name}"
            ));
        }
        if !self.temperature.is_finite() {
            errors.push("nominal temperature must be finite".to_owned());
        } else if self.temperature < -273.15 {
            errors.push("nominal temperature cannot be below absolute zero".to_owned());
        }
        if !self.vdd_factor.is_finite() || self.vdd_factor <= 0.0 {
            errors.push("supply factor must be positive and finite".to_owned());
        }
        for (label, axis) in [("NMOS", &self.nmos_corner), ("PMOS", &self.pmos_corner)] {
            if axis.trim().is_empty() {
                errors.push(format!("{label} corner axis cannot be empty"));
            } else if axis.chars().any(char::is_control) {
                errors.push(format!("{label} corner axis contains a control character"));
            }
        }
        match (self.minimum_temperature_c, self.maximum_temperature_c) {
            (None, None) => {}
            (Some(minimum), Some(maximum)) => {
                if !minimum.is_finite() || !maximum.is_finite() {
                    errors.push("qualified temperature bounds must be finite".to_owned());
                } else {
                    if minimum < -273.15 {
                        errors.push(
                            "minimum qualified temperature cannot be below absolute zero"
                                .to_owned(),
                        );
                    }
                    if minimum > maximum {
                        errors.push("minimum qualified temperature exceeds the maximum".to_owned());
                    }
                    if self.temperature < minimum || self.temperature > maximum {
                        errors.push(
                            "nominal temperature lies outside the qualified range".to_owned(),
                        );
                    }
                }
            }
            _ => errors.push(
                "qualified temperature range must provide both minimum and maximum".to_owned(),
            ),
        }

        let bindings = self.effective_section_bindings();
        let mut seen_domains = std::collections::BTreeSet::new();
        for binding in &bindings {
            if !seen_domains.insert(binding.domain) {
                errors.push(format!(
                    "{} has more than one section binding",
                    binding.domain.label()
                ));
            }
            let section = binding.section.trim();
            if section.is_empty() {
                errors.push(format!(
                    "{} section binding cannot be empty",
                    binding.domain.label()
                ));
            } else if section.chars().any(|character| {
                character.is_whitespace()
                    || character == '"'
                    || character == '\''
                    || character.is_control()
            }) {
                errors.push(format!(
                    "{} section contains an unsupported character: {section}",
                    binding.domain.label()
                ));
            }
        }

        let mut seen_required = std::collections::BTreeSet::new();
        for required in self.effective_required_domains() {
            if !seen_required.insert(required) {
                errors.push(format!(
                    "{} is listed as required more than once",
                    required.label()
                ));
            }
            if require_complete && !bindings.iter().any(|binding| binding.domain == required) {
                errors.push(format!(
                    "{} section is required but not bound",
                    required.label()
                ));
            }
        }
        if !bindings.is_empty() && self.file_path.is_none() {
            errors.push("section bindings require an authenticated source file".to_owned());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Standard corners for a PDK
    pub fn standard_corners() -> Vec<ProcessCorner> {
        vec![
            ProcessCorner {
                name: "tt".to_string(),
                description: "Typical-Typical".to_string(),
                nmos_corner: "typical".to_string(),
                pmos_corner: "typical".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                is_default: true,
                ..Default::default()
            },
            ProcessCorner {
                name: "ff".to_string(),
                description: "Fast-Fast".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: -40.0,
                vdd_factor: 1.1,
                is_default: false,
                ..Default::default()
            },
            ProcessCorner {
                name: "ss".to_string(),
                description: "Slow-Slow".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 125.0,
                vdd_factor: 0.9,
                is_default: false,
                ..Default::default()
            },
            ProcessCorner {
                name: "sf".to_string(),
                description: "Slow-Fast".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                is_default: false,
                ..Default::default()
            },
            ProcessCorner {
                name: "fs".to_string(),
                description: "Fast-Slow".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                is_default: false,
                ..Default::default()
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_file_backed_corner_is_one_required_composite_binding() {
        let mut corner = ProcessCorner::new("FF");
        corner.file_path = Some(PathBuf::from("/pdk/corners.lib"));

        assert_eq!(
            corner.effective_section_bindings(),
            vec![CornerSectionBinding::new(
                CornerSectionDomain::Composite,
                "FF"
            )]
        );
        assert_eq!(
            corner.effective_required_domains(),
            vec![CornerSectionDomain::Composite]
        );
        assert!(corner.validate_contract().is_ok());
    }

    #[test]
    fn missing_required_domain_and_invalid_environment_fail_closed() {
        let mut corner =
            ProcessCorner::from_composite_section("HOT", "/pdk/corners.lib".into(), false);
        corner.required_domains = vec![CornerSectionDomain::Mos, CornerSectionDomain::Aging];
        corner.section_bindings = vec![CornerSectionBinding::new(
            CornerSectionDomain::Mos,
            "mos_hot",
        )];
        corner.vdd_factor = 0.0;
        corner.minimum_temperature_c = Some(150.0);
        corner.maximum_temperature_c = Some(85.0);

        let errors = corner.validate_contract().expect_err("contract is invalid");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("Aging section is required"))
        );
        assert!(errors.iter().any(|error| error.contains("supply factor")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("minimum qualified temperature"))
        );
    }

    #[test]
    fn incomplete_required_domain_is_a_persistable_but_non_executable_draft() {
        let mut corner =
            ProcessCorner::from_composite_section("HOT", "/pdk/corners.lib".into(), false);
        corner.section_bindings.clear();
        corner.required_domains = vec![CornerSectionDomain::Mos];

        assert!(corner.effective_section_bindings().is_empty());
        corner
            .validate_draft_contract()
            .expect("an incomplete binding is a recoverable authoring draft");
        let errors = corner
            .validate_contract()
            .expect_err("the same draft cannot become executable");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("MOS section is required but not bound"))
        );
    }

    /// The qualified range includes its endpoints, and an absent bound is not
    /// a bound. Both surfaces that ask read this one rule.
    #[test]
    fn a_qualified_range_includes_the_endpoints_it_publishes() {
        let mut corner = ProcessCorner::new("hot");
        assert!(
            corner.qualifies_temperature(1_000.0),
            "a corner that published no range constrains nothing"
        );
        assert!(
            corner
                .temperatures_outside_qualified_range(&[-273.0, 1_000.0])
                .is_empty()
        );
        assert_eq!(corner.qualified_range_label(), "not declared");

        corner.minimum_temperature_c = Some(-40.0);
        corner.maximum_temperature_c = Some(125.0);
        assert!(
            corner.qualifies_temperature(-40.0),
            "the low endpoint is inside"
        );
        assert!(
            corner.qualifies_temperature(125.0),
            "the high endpoint is inside"
        );
        assert!(!corner.qualifies_temperature(-40.001));
        assert!(!corner.qualifies_temperature(125.001));
        assert!(!corner.qualifies_temperature(f64::NAN));
        assert_eq!(
            corner.temperatures_outside_qualified_range(&[-40.0, 27.0, 125.0, 150.0]),
            vec![150.0]
        );
        assert_eq!(corner.qualified_range_label(), "-40.000 to 125.000 °C");

        // One declared bound constrains one side and leaves the other open.
        corner.maximum_temperature_c = None;
        assert!(corner.qualifies_temperature(1_000.0));
        assert!(!corner.qualifies_temperature(-41.0));
        assert_eq!(corner.qualified_range_label(), "-40.000 °C and above");
    }
}
