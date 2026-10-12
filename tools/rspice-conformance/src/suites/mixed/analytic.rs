//! O6: analytic expectations, one module per design.
//!
//! A quantity names its expectation as `cNN::name` (or `p06x::name` for a
//! P0.6 probe, registered in [`super::probes`]). Each function receives the
//! quantity and, for expectations that depend on what the run itself sampled
//! (a code against the input it converted, a trial against its own drawn
//! parameter), RSpice's observation. Each design's lane owns its module.

use super::case::Observation;
use super::compare::Expected;
use super::design::Quantity;

pub mod c01;
pub mod c02;
pub mod c03;
pub mod c04;
pub mod c05;
pub mod c06;
pub mod c07;
pub mod c08;
pub mod c09;
pub mod c10;
pub mod c11;
pub mod c12;

/// What an expectation may read.
pub struct Context<'a> {
    pub quantity: &'a Quantity,
    pub variant: &'a str,
    /// RSpice's observation of this cell (one trial's, for per-trial use).
    pub rspice: &'a Observation,
    /// The trial index for per-trial expectations.
    pub trial: Option<usize>,
}

pub type AnalyticFn = fn(&Context<'_>) -> Result<Expected, String>;
pub type Registry = &'static [(&'static str, AnalyticFn)];

/// Resolve `cNN::name` / `p06x::name`.
pub fn lookup(name: &str) -> Option<AnalyticFn> {
    let (module, function) = name.split_once("::")?;
    let registry: Registry = match module {
        "c01" => c01::REGISTRY,
        "c02" => c02::REGISTRY,
        "c03" => c03::REGISTRY,
        "c04" => c04::REGISTRY,
        "c05" => c05::REGISTRY,
        "c06" => c06::REGISTRY,
        "c07" => c07::REGISTRY,
        "c08" => c08::REGISTRY,
        "c09" => c09::REGISTRY,
        "c10" => c10::REGISTRY,
        "c11" => c11::REGISTRY,
        "c12" => c12::REGISTRY,
        "p06a" | "p06b" | "p06c" => super::probes::REGISTRY,
        _ => return None,
    };
    let qualified = format!("{module}::{function}");
    registry
        .iter()
        .find(|(entry, _)| *entry == function || *entry == qualified)
        .map(|(_, function)| *function)
}

/// The quantity's sample points (frequencies, sweep values) for expectations
/// that are functions of them.
pub fn points(quantity: &Quantity) -> Result<Vec<f64>, String> {
    match (&quantity.measure.points, &quantity.measure.times) {
        (Some(points), _) => Ok(points.clone()),
        (None, Some(grid)) => Ok(grid.times()),
        (None, None) => Err(format!("{}: the expectation needs points", quantity.id)),
    }
}

pub fn values(values: Vec<f64>) -> Expected {
    Expected::Measured(super::measure::Measured::Values(values))
}

pub fn complex(values: Vec<num_complex::Complex64>) -> Expected {
    Expected::Measured(super::measure::Measured::Complex(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_names_resolve_and_unknown_ones_do_not() {
        assert!(lookup("c01::code").is_some());
        assert!(lookup("c02::vlock").is_some());
        assert!(lookup("c01::nonexistent").is_none());
        assert!(lookup("c13::code").is_none());
        assert!(lookup("code").is_none());
    }
}
