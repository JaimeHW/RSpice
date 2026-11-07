//! A resistor, capacitor or inductor's tolerance: how far its value may sit
//! from nominal, and how a Monte Carlo spreads it across that band.

use std::collections::HashMap;

use super::component_type::ComponentType;

/// The parameter holding the tolerance in percent (`5` or `5%`).
pub const TOLERANCE_KEY: &str = "tol";
/// The parameter naming the tolerance's distribution.
pub const DISTRIBUTION_KEY: &str = "tol_dist";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToleranceDistribution {
    /// Normal, with the tolerance at 3σ.
    #[default]
    Gaussian,
    /// Flat across the whole band.
    Uniform,
}

impl ToleranceDistribution {
    pub const ALL: [Self; 2] = [Self::Gaussian, Self::Uniform];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Gaussian => "Gaussian, 3σ",
            Self::Uniform => "Uniform",
        }
    }

    /// The word a component's `tol_dist` stores.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Gaussian => "gauss",
            Self::Uniform => "uniform",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "" | "gauss" | "gaussian" | "normal" => Some(Self::Gaussian),
            "uniform" | "unif" | "flat" => Some(Self::Uniform),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PartTolerance {
    /// Half-width of the band in percent: 5 is ±5 %.
    pub percent: f64,
    pub distribution: ToleranceDistribution,
}

/// Whether a device type has a value a tolerance applies to.
pub const fn supports_tolerance(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor
    )
}

/// A tolerance as typed: `5`, `5%` or `0.5 %`, finite and non-negative.
pub fn parse_tolerance_percent(text: &str) -> Result<f64, String> {
    let number = text.trim();
    let number = number.strip_suffix('%').unwrap_or(number).trim();
    number
        .parse::<f64>()
        .ok()
        .filter(|percent| percent.is_finite() && *percent >= 0.0)
        .ok_or_else(|| "Tolerance must be a non-negative percentage.".to_owned())
}

/// The tolerance a component's parameters state, or `None` when they state
/// none.
pub fn part_tolerance(params: &HashMap<String, String>) -> Result<Option<PartTolerance>, String> {
    let Some(text) = params
        .get(TOLERANCE_KEY)
        .filter(|text| !text.trim().is_empty())
    else {
        return Ok(None);
    };
    let percent = parse_tolerance_percent(text)?;
    let distribution = params
        .get(DISTRIBUTION_KEY)
        .map_or(Some(ToleranceDistribution::Gaussian), |text| {
            ToleranceDistribution::parse(text)
        })
        .ok_or_else(|| "Tolerance distribution must be gauss or uniform.".to_owned())?;
    Ok(Some(PartTolerance {
        percent,
        distribution,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn a_tolerance_reads_its_percent_and_distribution() {
        assert_eq!(part_tolerance(&params(&[])), Ok(None));
        assert_eq!(
            part_tolerance(&params(&[("tol", "5%")])),
            Ok(Some(PartTolerance {
                percent: 5.0,
                distribution: ToleranceDistribution::Gaussian
            }))
        );
        assert_eq!(
            part_tolerance(&params(&[("tol", "0.1"), ("tol_dist", "Uniform")])),
            Ok(Some(PartTolerance {
                percent: 0.1,
                distribution: ToleranceDistribution::Uniform
            }))
        );
        assert!(part_tolerance(&params(&[("tol", "-1")])).is_err());
        assert!(part_tolerance(&params(&[("tol", "5"), ("tol_dist", "lognormal")])).is_err());
    }
}
