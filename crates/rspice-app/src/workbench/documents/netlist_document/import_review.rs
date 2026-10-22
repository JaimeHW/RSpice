//! The review a netlist import passes before it may replace project state:
//! the decoded candidate, the issues found in it, and whether the dialect the
//! reader selected has a qualified execution profile.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NetlistImportOperation {
    OpenProject,
    ImportIntoProject,
    RequalifyOwnedSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NetlistImportIssueSeverity {
    Advisory,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NetlistImportIssue {
    pub severity: NetlistImportIssueSeverity,
    pub message: String,
}

/// Lossless, revision-guarded candidate presented before a netlist import is
/// allowed to replace project state. The candidate owns the decoded source
/// snapshot and raw-file identity; accepting the dialog never re-reads a
/// different file and cancellation never mutates project-owned documents.
#[derive(Debug, Clone)]
pub(crate) struct NetlistImportReviewState {
    pub transaction: crate::product::TransactionId,
    pub operation: NetlistImportOperation,
    pub display_name: String,
    /// Native picker origin re-read at commit to reject review races. Archive
    /// imports deliberately keep `source_path` empty so Save never overwrites
    /// a ZIP with plain deck bytes.
    pub selected_file_path: Option<std::path::PathBuf>,
    pub source_path: Option<std::path::PathBuf>,
    pub source: String,
    pub dependencies: Vec<crate::state::DependencyMetadata>,
    pub archive_import: bool,
    pub original_byte_count: usize,
    pub original_sha256: [u8; 32],
    pub encoding: crate::state::NetlistTextEncoding,
    pub line_ending: crate::state::NetlistLineEnding,
    pub detected_dialect: crate::state::NetlistSourceDialect,
    pub selected_dialect: crate::state::NetlistSourceDialect,
    pub detection_evidence: Vec<String>,
    pub transformations: Vec<String>,
    pub issues: Vec<NetlistImportIssue>,
    pub compatibility_accepted: bool,
    pub error: Option<String>,
}

impl NetlistImportReviewState {
    pub(crate) fn blocking_issue_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == NetlistImportIssueSeverity::Blocking)
            .count()
    }

    pub(crate) fn can_commit(&self) -> bool {
        self.blocking_issue_count() == 0
            && self.dialect_qualification().is_ok()
            && (!self.selected_dialect.requires_compatibility_review()
                || self.compatibility_accepted)
    }

    /// Prove that the selected import profile has an executable semantic
    /// implementation. A review checkbox acknowledges a qualified profile;
    /// it can never create an adapter or turn a vendor marker into canonical
    /// RSpice syntax. This check is deliberately repeated at commit.
    pub(crate) fn dialect_qualification(&self) -> Result<(), String> {
        use crate::state::NetlistSourceDialect;

        match self.selected_dialect {
            NetlistSourceDialect::RSpice => {
                if self.detected_dialect == NetlistSourceDialect::RSpice {
                    Ok(())
                } else {
                    Err(format!(
                        "Detected {} markers cannot be relabeled as RSpice SPICE. Remove or convert every foreign statement before importing.",
                        self.detected_dialect.label()
                    ))
                }
            }
            NetlistSourceDialect::Spice3Ngspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Spice3Ngspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as SPICE3/ngspice. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::Spice3NgspiceV2
                    .validate_source(&self.source)
                    .map_err(|error| format!(
                        "SPICE3/ngspice compatibility is not qualified: {error}. Only line-preserving declarative .control/.endc commands (op, dc, ac, sp, tran, save, and bounded meas/measure) are qualified; remove imperative commands such as wrdata and setplot before import."
                    ))?;
                Ok(())
            }
            NetlistSourceDialect::Pspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Pspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as PSpice. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::PspiceDeclarativeV2
                    .validate_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "PSpice declarative compatibility is not qualified: {error}. The v2 profile requires a .PROBE, .PROBE64, or .PROBE/CSDF line before .END; accepts DEV/LOT, .DISTRIBUTION, .MC on a selected analysis, .MC LIST/OUTPUT results, and minimum-order Type-I E/G CHEBYSHEV LP/HP/BP/BR sources; and rejects unsupported .STIMULUS or FREQ sources and unknown output-format commands."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Hspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Hspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as HSPICE. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::HspiceDeclarativeV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "HSPICE declarative compatibility is not qualified: {error}. The v1 adapter requires a pre-.END .OPTION POST or .PROTECT/.UNPROTECT source marker and accepts only ordinary SPICE analyses plus those simulation-neutral presentation directives."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Spectre => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Spectre
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as Spectre SPICE interoperability source. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::SpectreSpiceV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "Spectre SPICE interoperability is not qualified: {error}. The v1 adapter requires `simulator lang=spice` mode and rejects native Spectre parameters, AHDL includes, saveOptions, and language switches."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Ads => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Ads
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as an ADS SPICE export. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::AdsSpiceExportV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "ADS SPICE-export compatibility is not qualified: {error}. The v1 adapter accepts the bounded ResourceUsage/UseNutmegFormat/TopDesignName header only; #uselib, define, simulatorOptions, native ADS devices, and unresolved library semantics remain rejected."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Unknown => Err(
                "The dialect is ambiguous or unknown, so no compatibility profile can run this source. Resolve the dialect and convert unsupported constructs before importing."
                    .to_owned(),
            ),
        }
    }
}
