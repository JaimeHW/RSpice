//! Authored Newton linear-solver policy and bounded Krylov workspace.
use super::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum QuasiPeriodicLinearMethod {
    #[default]
    Auto,
    Direct,
    Krylov,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuasiPeriodicLinearConfig {
    pub method: QuasiPeriodicLinearMethod,
    pub restart: usize,
    /// Maximum restart cycles, each containing up to `restart` Arnoldi steps.
    pub max_cycles: usize,
    pub relative_tolerance: f64,
}
impl Default for QuasiPeriodicLinearConfig {
    fn default() -> Self {
        Self {
            method: QuasiPeriodicLinearMethod::Auto,
            restart: 32,
            max_cycles: 20,
            relative_tolerance: 1e-10,
        }
    }
}
impl QuasiPeriodicLinearConfig {
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
    pub(crate) fn uses_krylov(&self, size: usize) -> bool {
        self.method == QuasiPeriodicLinearMethod::Krylov
            || (self.method == QuasiPeriodicLinearMethod::Auto && size > super::MAX_DENSE_UNKNOWNS)
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if !(8..=64).contains(&self.restart)
            || self.max_cycles == 0
            || !self.relative_tolerance.is_finite()
            || self.relative_tolerance <= 0.0
            || self.relative_tolerance >= 1.0
        {
            return Err(Error::InvalidConfig("Krylov requires 8..64 restart vectors, positive restart cycles, and a finite linear tolerance between zero and one".into()));
        }
        Ok(())
    }
}
