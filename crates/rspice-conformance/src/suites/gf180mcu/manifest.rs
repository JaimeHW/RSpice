//! Per-case contract lookup from `device-manifest.tsv`.
//!
//! One tab-separated row per case:
//!
//! ```text
//! <case>\t<contract>\t<reference-agreement-pct>\t<note>
//! ```
//!
//! Cases absent from the file take the default contract, so the file records
//! adjudications rather than an inventory of everything vendored.

use super::*;

pub(super) const FILE_NAME: &str = "device-manifest.tsv";

/// What a device case is required to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceContract {
    /// The case must reproduce the vendored reference curve within tolerance.
    #[default]
    Ngspice,
    /// The case must run, but carries no comparable reference.
    ///
    /// Kept as a contract rather than dropping the deck, because a device
    /// RSpice cannot even evaluate is worth continuing to exercise: the case
    /// still has to build and complete, and the day a reference becomes
    /// available the row changes rather than the corpus.
    ReferenceOnly,
    /// The case exercises a device model RSpice does not implement, and must
    /// fail while doing so.
    ///
    /// A case under this contract that starts passing fails the suite. That
    /// is the point: these rows are a live inventory of what the PDK needs
    /// and RSpice does not have, and the moment one closes it should be
    /// promoted to a real comparison rather than left recording a gap that
    /// no longer exists.
    ExpectedUnsupported,
}

impl DeviceContract {
    fn parse(token: &str) -> Option<Self> {
        match token.trim() {
            "ngspice" => Some(Self::Ngspice),
            "reference_only" => Some(Self::ReferenceOnly),
            "expected_unsupported" => Some(Self::ExpectedUnsupported),
            _ => None,
        }
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::Ngspice => "ngspice",
            Self::ReferenceOnly => "reference_only",
            Self::ExpectedUnsupported => "expected_unsupported",
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct ManifestEntry {
    pub(super) contract: DeviceContract,
    /// Agreement the reference implementation itself achieves, as a
    /// percentage. Zero for a captured reference, which by construction
    /// agrees with itself.
    pub(super) reference_pct: f64,
}

pub(super) fn load(root: &Path) -> HashMap<String, ManifestEntry> {
    let Ok(content) = std::fs::read_to_string(root.join(FILE_NAME)) else {
        return HashMap::new();
    };
    parse(&content, root)
}

fn parse(content: &str, root: &Path) -> HashMap<String, ManifestEntry> {
    let mut manifest = HashMap::new();
    for (index, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut fields = line.split('\t');
        let (Some(case), Some(token)) = (fields.next(), fields.next()) else {
            continue;
        };

        let Some(contract) = DeviceContract::parse(token) else {
            eprintln!(
                "{}/{FILE_NAME}:{}: unknown contract '{token}'; case takes the default",
                root.display(),
                index + 1
            );
            continue;
        };

        manifest.insert(
            case.trim().to_string(),
            ManifestEntry {
                contract,
                reference_pct: fields
                    .next()
                    .and_then(|raw| raw.trim().parse().ok())
                    .unwrap_or(0.0),
            },
        );
    }
    manifest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_parses_contracts_and_reference_agreement() {
        let manifest = parse(
            "# comment\n\
             \n\
             np_3p3_typical_A100_P40_T25\tngspice\t0.00\t\n\
             sc_diode_ss_A50_P102_T175\treference_only\t12.50\tno converged reference\n\
             broken\tnot_a_contract\t0.00\t\n",
            Path::new("corpus"),
        );

        assert_eq!(manifest.len(), 2);
        assert_eq!(
            manifest["np_3p3_typical_A100_P40_T25"].contract,
            DeviceContract::Ngspice
        );
        let adjudicated = manifest["sc_diode_ss_A50_P102_T175"];
        assert_eq!(adjudicated.contract, DeviceContract::ReferenceOnly);
        assert!((adjudicated.reference_pct - 12.5).abs() < 1e-9);
        // An unparsable contract must not silently become an exemption.
        assert!(!manifest.contains_key("broken"));
    }
}
