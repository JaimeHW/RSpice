//! Staging an import for review.
//!
//! Nothing observed in a candidate file — dialect, includes, encoding — is
//! adopted until the review commits it, so a rejected import leaves no trace.

use super::bundle::{decode_import_bytes, parse_generated_netlist_bundle};
use super::import::{
    NetlistImportMetadata, NetlistImportMode, apply_netlist_import_result,
    netlist_import_start_block_reason, sha256,
};
use super::*;

pub(super) fn detect_netlist_dialect(
    source: &str,
) -> (crate::state::NetlistSourceDialect, Vec<String>) {
    use crate::state::NetlistSourceDialect;

    let mut spectre = 0usize;
    let mut hspice = 0usize;
    let mut pspice = 0usize;
    let mut ngspice = 0usize;
    let mut ads = 0usize;
    let mut evidence = Vec::new();
    for (line_index, raw) in source.lines().take(500_000).enumerate() {
        let line = raw.trim().to_ascii_lowercase();
        let (score, description) = if line.starts_with("simulator lang=")
            || line.starts_with("ahdl_include")
            || line.starts_with("saveoptions ")
            || line.starts_with("parameters ")
        {
            spectre += 1;
            (true, "Spectre language directive")
        } else if line.starts_with(".option post")
            || line == ".protect"
            || line == ".unprotect"
            || line.starts_with(".alter")
        {
            hspice += 1;
            (true, "HSPICE compatibility directive")
        } else if line.starts_with(".probe")
            || line.starts_with(".distribution")
            || line.starts_with(".stimulus")
        {
            pspice += 1;
            (true, "PSpice compatibility directive")
        } else if line == ".control"
            || line == ".endc"
            || line.starts_with("wrdata ")
            || line.starts_with("setplot ")
        {
            ngspice += 1;
            (true, "ngspice control-language directive")
        } else if line.starts_with("#uselib")
            || line.starts_with("define ")
            || line.starts_with("simulatoroptions ")
            || line.starts_with("options resourceusage=")
        {
            ads += 1;
            (true, "ADS netlist directive")
        } else {
            (false, "")
        };
        if score && evidence.len() < 12 {
            evidence.push(format!("line {}: {description}", line_index + 1));
        }
    }

    let scores = [
        (spectre, NetlistSourceDialect::Spectre),
        (hspice, NetlistSourceDialect::Hspice),
        (pspice, NetlistSourceDialect::Pspice),
        (ngspice, NetlistSourceDialect::Spice3Ngspice),
        (ads, NetlistSourceDialect::Ads),
    ];
    let maximum = scores.iter().map(|(score, _)| *score).max().unwrap_or(0);
    if maximum == 0 {
        return (NetlistSourceDialect::RSpice, evidence);
    }
    let mut matches = scores
        .iter()
        .filter_map(|(score, dialect)| (*score == maximum).then_some(*dialect));
    let first = matches.next().unwrap_or(NetlistSourceDialect::Unknown);
    let dialect = if matches.next().is_some() {
        NetlistSourceDialect::Unknown
    } else {
        first
    };
    (dialect, evidence)
}

pub(super) fn validate_import_candidate(
    source: &str,
    source_path: Option<&std::path::Path>,
    execution_profile: Option<crate::state::NetlistExecutionProfile>,
) -> Vec<crate::workbench::documents::netlist_document::NetlistImportIssue> {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity,
    };

    let mut issues = Vec::new();
    if source.trim().is_empty() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: "The selected deck is empty.".to_owned(),
        });
        return issues;
    }
    if let Some((character_index, character)) = source
        .chars()
        .enumerate()
        .find(|(_, character)| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: format!(
                "Unsupported control character U+{:04X} at decoded character {}.",
                u32::from(character),
                character_index + 1
            ),
        });
        return issues;
    }

    let adapted = match execution_profile
        .map(|profile| profile.adapt_source(source))
        .transpose()
    {
        Ok(Some(adapted)) => adapted,
        Ok(None) => std::borrow::Cow::Borrowed(source),
        Err(error) => {
            issues.push(NetlistImportIssue {
                severity: NetlistImportIssueSeverity::Blocking,
                message: format!("Execution-profile adaptation failed: {error}"),
            });
            return issues;
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let parsed = source_path.map_or_else(
        || rspice_core::Netlist::parse(&adapted),
        |path| rspice_core::Netlist::parse_with_path(&adapted, path),
    );
    #[cfg(target_arch = "wasm32")]
    let parsed = {
        let _ = source_path;
        rspice_core::Netlist::parse(&adapted)
    };

    match parsed {
        Ok(netlist) => {
            if let Some(profile) = execution_profile
                && let Err(error) = profile.validate_parsed_netlist(&netlist)
            {
                issues.push(NetlistImportIssue {
                    severity: NetlistImportIssueSeverity::Blocking,
                    message: format!("Execution-profile validation failed: {error}"),
                });
            }
            for diagnostic in &netlist.diagnostics {
                let semantic_loss = matches!(
                    diagnostic.code.as_str(),
                    "unknown-option"
                        | "unsupported-dot-command"
                        | "control-block-ignored"
                        | "invalid-option-defaulted"
                );
                issues.push(NetlistImportIssue {
                    severity: if semantic_loss {
                        NetlistImportIssueSeverity::Blocking
                    } else {
                        NetlistImportIssueSeverity::Advisory
                    },
                    message: format!(
                        "Parser diagnostic {} at line {}: {}",
                        diagnostic.code, diagnostic.line, diagnostic.message
                    ),
                });
            }
            if let Err(error) = rspice_core::netlist::validate_output_symbols(&netlist) {
                issues.push(NetlistImportIssue {
                    severity: NetlistImportIssueSeverity::Blocking,
                    message: format!("Output-symbol validation failed: {error}"),
                });
            }
        }
        Err(error) => issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: format!("Canonical parse/include validation failed: {error}"),
        }),
    }
    issues
}

pub(super) fn stage_netlist_import(
    state: &mut AppState,
    transaction: crate::product::TransactionId,
    mode: NetlistImportMode,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: selected file exceeds the supported {}-byte size limit",
            crate::io::project_io::MAX_PROJECT_FILE_BYTES
        )));
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        transaction,
    ) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
        state.push_user_message(ConsoleMessage::error(format!(
            "{} was cancelled because the project changed: {error}",
            mode.dialog_title()
        )));
        return false;
    }
    if bytes.starts_with(b"PK\x03\x04") {
        return stage_netlist_bundle_import(
            state,
            transaction,
            mode,
            bytes,
            source_path,
            display_name,
        );
    }
    let (source, encoding) = match decode_import_bytes(&bytes) {
        Ok(decoded) => decoded,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            return false;
        }
    };
    let line_ending = crate::state::NetlistLineEnding::detect(&source);
    let (detected_dialect, detection_evidence) = detect_netlist_dialect(&source);
    let mut issues = validate_import_candidate(
        &source,
        source_path.as_deref(),
        detected_dialect.execution_profile(),
    );
    if line_ending == crate::state::NetlistLineEnding::Mixed {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The deck contains mixed line endings; RSpice will preserve them unless the source is explicitly formatted.".to_owned(),
        });
    }
    if detected_dialect.requires_compatibility_review() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: format!(
                "{} was detected. No source statement will be translated silently; accepting records an explicit compatibility profile.",
                detected_dialect.label()
            ),
        });
    }
    let mut transformations = vec![
        format!(
            "Losslessly decoded {} into the editor's Unicode representation; Save retains the original encoding.",
            encoding.label()
        ),
        format!(
            "Preserved {} line endings and source statement order.",
            line_ending.label()
        ),
        "Applied no model substitution, unit coercion, analysis deletion, or unsupported-statement deletion.".to_owned(),
    ];
    if detected_dialect == crate::state::NetlistSourceDialect::Spice3Ngspice {
        transformations.push(
            "At validation and execution only, spice3-ngspice/2 promotes the bounded declarative .control/.endc subset (op, dc, ac, sp, tran, save, and simple aggregate measurements) into line-preserving dot directives; retained project source bytes remain unchanged and imperative commands fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Pspice {
        transformations.push(
            "The pspice-declarative/2 profile requires at least one pre-.END .PROBE, .PROBE64, or .PROBE/CSDF source marker. Ordinary analyses plus qualified .TF, .STEP, .FOUR, model DEV/LOT, .DISTRIBUTION, selected-analysis .MC collation, and E/G CHEBYSHEV LP/HP/BP/BR sources retain their source form. CHEBYSHEV uses an exact minimum-order analog Type-I realization; typed .MC LIST/OUTPUT selection is retained as bounded immutable result data without automatic file writes. Missing evidence, unsupported .STIMULUS or FREQ sources, and unknown output-format commands fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Hspice {
        transformations.push(
            "At validation and execution only, hspice-declarative/1 requires at least one pre-.END .OPTION POST or .PROTECT/.UNPROTECT source marker and maps those qualified presentation directives to line-preserving comments; retained project source bytes remain unchanged, while every other HSPICE .OPTION fails closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Spectre {
        transformations.push(
            "At validation and execution only, spectre-spice/1 requires exactly one `simulator lang=spice` interoperability boundary before .END and maps it to a line-preserving comment; retained project source bytes remain unchanged, while missing/duplicate boundaries and native Spectre statements fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Ads {
        transformations.push(
            "At validation and execution only, ads-spice-export/1 requires exactly one qualified ADS ResourceUsage/UseNutmegFormat/TopDesignName export header before .END and maps it to a line-preserving comment; retained source bytes remain unchanged, while missing/duplicate headers and native ADS/preprocessor statements fail closed."
                .to_owned(),
        );
    }
    let dependencies = match resolve_plain_import_dependencies(&source, source_path.as_deref()) {
        Ok(dependencies) => {
            if !dependencies.is_empty() {
                transformations.push(format!(
                    "Resolved and retained {} native include member(s) as an authenticated project closure.",
                    dependencies.len()
                ));
            }
            dependencies
        }
        Err(error) => {
            issues.push(NetlistImportIssue {
                severity: NetlistImportIssueSeverity::Blocking,
                message: error,
            });
            Vec::new()
        }
    };
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: match mode {
            NetlistImportMode::OpenProject => NetlistImportOperation::OpenProject,
            NetlistImportMode::ImportIntoProject => NetlistImportOperation::ImportIntoProject,
        },
        display_name,
        selected_file_path: source_path.clone(),
        source_path,
        source,
        dependencies,
        archive_import: false,
        original_byte_count: bytes.len(),
        original_sha256: sha256(&bytes),
        encoding,
        line_ending,
        detected_dialect,
        selected_dialect: detected_dialect,
        detection_evidence,
        transformations,
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_plain_import_dependencies(
    root_source: &str,
    root_path: Option<&std::path::Path>,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    use std::collections::HashSet;

    let root_directives = crate::state::netlist_document::parse_include_directives(root_source);
    if root_directives.is_empty() {
        return Ok(Vec::new());
    }
    let root_path = root_path.ok_or_else(|| {
        "This deck has include directives but no reopenable native source origin. Select the source through Open, or import an authenticated .rspice-netlist.zip bundle."
            .to_owned()
    })?;
    let root_path = std::fs::canonicalize(root_path).map_err(|error| {
        format!(
            "Could not establish the selected root deck's native origin '{}': {error}",
            root_path.display()
        )
    })?;
    let root_directory = root_path
        .parent()
        .ok_or_else(|| "The selected root deck has no parent directory.".to_owned())?;
    let mut resolver = NativeDependencyResolver {
        root_directory: root_directory.to_path_buf(),
        dependencies: Vec::new(),
        active_paths: HashSet::new(),
        retained_paths: HashSet::new(),
        retained_bytes: 0,
    };
    resolver.resolve_source(None, root_directory, root_source, 0)?;
    crate::state::expand_retained_netlist_dependencies(
        crate::state::NetlistDocumentId::new(),
        root_source,
        &resolver.dependencies,
    )
    .map_err(|error| format!("Resolved include closure is invalid: {error}"))?;
    Ok(resolver.dependencies)
}

#[cfg(target_arch = "wasm32")]
fn resolve_plain_import_dependencies(
    root_source: &str,
    _root_path: Option<&std::path::Path>,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    if crate::state::netlist_document::parse_include_directives(root_source).is_empty() {
        Ok(Vec::new())
    } else {
        Err(
            "Browser import cannot acquire neighboring include files from one file grant. Import an authenticated .rspice-netlist.zip bundle containing the complete dependency closure."
                .to_owned(),
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeDependencyResolver {
    root_directory: std::path::PathBuf,
    dependencies: Vec<crate::state::DependencyMetadata>,
    active_paths: std::collections::HashSet<std::path::PathBuf>,
    retained_paths: std::collections::HashSet<std::path::PathBuf>,
    retained_bytes: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeDependencyResolver {
    fn resolve_source(
        &mut self,
        parent: Option<crate::state::SourceLocator>,
        source_directory: &std::path::Path,
        source: &str,
        depth: usize,
    ) -> Result<(), String> {
        if depth >= crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH {
            return Err(format!(
                "Include closure exceeds the supported depth of {}.",
                crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH
            ));
        }
        let directives = crate::state::netlist_document::parse_include_directives(source);
        for (include_index, directive) in directives.iter().enumerate() {
            if self.dependencies.len() >= crate::state::MAX_PROJECT_SOURCE_DEPENDENCIES
                || self.dependencies.len().saturating_add(1)
                    >= crate::state::MAX_PROJECT_SOURCE_FILES
            {
                return Err(
                    "Include closure exceeds the supported project source inventory.".to_owned(),
                );
            }
            let requested = directive.locator();
            let requested_path = std::path::PathBuf::from(requested);
            let candidate = if requested_path.is_absolute() {
                requested_path
            } else {
                source_directory.join(requested_path)
            };
            let canonical = std::fs::canonicalize(&candidate).map_err(|error| {
                format!(
                    "Could not resolve include {requested:?} from '{}': {error}",
                    source_directory.display()
                )
            })?;
            if self.active_paths.contains(&canonical) {
                return Err(format!(
                    "Include cycle reaches '{}' again.",
                    canonical.display()
                ));
            }
            if !self.retained_paths.insert(canonical.clone()) {
                return Err(format!(
                    "The physical include '{}' is referenced more than once. Use a single canonical include edge so the retained dependency graph is unambiguous.",
                    canonical.display()
                ));
            }
            let metadata = std::fs::metadata(&canonical).map_err(|error| {
                format!(
                    "Could not inspect include '{}': {error}",
                    canonical.display()
                )
            })?;
            if !metadata.is_file() {
                return Err(format!(
                    "Include '{}' is not a regular file.",
                    canonical.display()
                ));
            }
            let file_len = usize::try_from(metadata.len()).map_err(|_| {
                format!(
                    "Include '{}' is too large for this platform.",
                    canonical.display()
                )
            })?;
            if file_len > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
                return Err(format!(
                    "Include '{}' exceeds the per-source {} byte limit.",
                    canonical.display(),
                    crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
                ));
            }
            self.retained_bytes = self
                .retained_bytes
                .checked_add(file_len)
                .ok_or_else(|| "Include closure byte size overflowed.".to_owned())?;
            if self.retained_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
                return Err(format!(
                    "Include closure exceeds the supported {} byte retained-source limit.",
                    crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
                ));
            }
            let bytes = std::fs::read(&canonical).map_err(|error| {
                format!("Could not read include '{}': {error}", canonical.display())
            })?;
            let child_source = std::str::from_utf8(&bytes).map_err(|error| {
                format!(
                    "Include '{}' is not valid UTF-8 ({error}); convert it explicitly before importing the closure.",
                    canonical.display()
                )
            })?.to_owned();
            let logical_identity = canonical
                .strip_prefix(&self.root_directory)
                .unwrap_or(&canonical)
                .to_string_lossy()
                .replace('\\', "/");
            let display_name = canonical
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| logical_identity.clone());
            let locator = crate::state::SourceLocator::try_new(logical_identity, display_name)
                .and_then(|locator| {
                    locator.with_native_origin(canonical.to_string_lossy().into_owned())
                })
                .map_err(|error| error.to_string())?;
            let dependency = if let Some(parent) = parent.clone() {
                crate::state::DependencyMetadata::unresolved_transitive_to(
                    parent,
                    include_index,
                    requested,
                    locator.clone(),
                )
            } else {
                crate::state::DependencyMetadata::unresolved_direct_to(
                    include_index,
                    requested,
                    locator.clone(),
                )
            }
            .and_then(|dependency| dependency.resolve_utf8(bytes))
            .map_err(|error| error.to_string())?;
            self.dependencies.push(dependency);
            self.active_paths.insert(canonical.clone());
            let child_directory = canonical.parent().ok_or_else(|| {
                format!("Include '{}' has no parent directory.", canonical.display())
            })?;
            let nested = self.resolve_source(
                Some(locator),
                child_directory,
                &child_source,
                depth.saturating_add(1),
            );
            self.active_paths.remove(&canonical);
            nested?;
        }
        Ok(())
    }
}

/// Stage a desktop/browser drag-and-drop import through the same bounded,
/// revision-guarded transaction used by the explicit picker.
pub(crate) fn stage_dropped_netlist_import(
    state: &mut AppState,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    stage_dropped_netlist(
        state,
        NetlistImportMode::ImportIntoProject,
        bytes,
        source_path,
        display_name,
    )
}

/// Stage a deck dropped on the project launcher as a new netlist-first
/// project. This uses the same review transaction as the explicit Open deck
/// command rather than importing into whichever project happened to be open.
pub(crate) fn stage_dropped_netlist_project(
    state: &mut AppState,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    stage_dropped_netlist(
        state,
        NetlistImportMode::OpenProject,
        bytes,
        source_path,
        display_name,
    )
}

fn stage_dropped_netlist(
    state: &mut AppState,
    mode: NetlistImportMode,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "Dropped SPICE source is unavailable: {reason}"
        )));
        return false;
    }
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Dropped SPICE source is unavailable: {error}"
                )));
                return false;
            }
        };
    stage_netlist_import(state, transaction, mode, bytes, source_path, display_name)
}

pub(super) fn stage_netlist_bundle_import(
    state: &mut AppState,
    transaction: crate::product::TransactionId,
    mode: NetlistImportMode,
    bytes: Vec<u8>,
    selected_file_path: Option<std::path::PathBuf>,
    archive_display_name: String,
) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    let bundle = match parse_generated_netlist_bundle(&bytes) {
        Ok(bundle) => bundle,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "RSpice netlist bundle import failed: {error}"
            )));
            return false;
        }
    };
    let line_ending = crate::state::NetlistLineEnding::detect(&bundle.source);
    let mut issues = validate_import_candidate(
        &bundle.expanded_source,
        None,
        Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
    );
    if line_ending == crate::state::NetlistLineEnding::Mixed {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The retained root deck contains mixed line endings; RSpice will preserve them unless the source is explicitly formatted.".to_owned(),
        });
    }
    if bundle.dependencies.is_empty() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The authenticated bundle contains no retained dependency members.".to_owned(),
        });
    }
    let artifact_stem = std::path::Path::new(&archive_display_name)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::trim)
        .filter(|stem| !stem.is_empty() && *stem != "." && *stem != "..")
        .unwrap_or("imported-netlist");
    let display_name = format!("{artifact_stem}.spice");
    let dependency_count = bundle.dependencies.len();
    let transformations = vec![
        "Verified the RSpice bundle schema, ZIP structure, member declarations, CRC-32 values, and SHA-256 content identities before review.".to_owned(),
        format!(
            "Reconstructed and authenticated the retained dependency closure ({dependency_count} member{}).",
            if dependency_count == 1 { "" } else { "s" }
        ),
        "Validated the fully expanded retained deck without consulting the host filesystem or network.".to_owned(),
        "Preserved the retained root source and dependency bytes; applied no syntax rewrite, model substitution, unit coercion, or unsupported-statement deletion.".to_owned(),
    ];
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: match mode {
            NetlistImportMode::OpenProject => NetlistImportOperation::OpenProject,
            NetlistImportMode::ImportIntoProject => NetlistImportOperation::ImportIntoProject,
        },
        display_name,
        selected_file_path,
        // Never associate a text Save with the archive selected for import.
        source_path: None,
        source: bundle.source,
        dependencies: bundle.dependencies,
        archive_import: true,
        original_byte_count: bytes.len(),
        original_sha256: sha256(&bytes),
        encoding: crate::state::NetlistTextEncoding::Utf8,
        line_ending,
        detected_dialect: crate::state::NetlistSourceDialect::RSpice,
        selected_dialect: crate::state::NetlistSourceDialect::RSpice,
        detection_evidence: vec![
            "Authenticated retained/generated.spice from rspice-generated-netlist-bundle/v1."
                .to_owned(),
        ],
        transformations,
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Stage the exact persisted owned source for an in-product execution-profile
/// review. No file picker, host interpreter, or external conversion is
/// involved; retained dependency bytes are reviewed and validated as part of
/// the same guarded project snapshot.
pub(crate) fn begin_owned_netlist_profile_review(state: &mut AppState) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    let candidate = (|| -> Result<_, String> {
        if !state.project_lifecycle.project_open {
            return Err("Open the project before reviewing its netlist profile.".to_owned());
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err("The project is read-only in the current safe mode.".to_owned());
        }
        let descriptor = state
            .workspace
            .netlist_descriptor
            .as_ref()
            .ok_or_else(|| "No owned netlist descriptor is available.".to_owned())?;
        if !descriptor.execution_profile_review_required() {
            return Err("The owned source already has an exact execution profile.".to_owned());
        }
        let document = state
            .workspace
            .netlist_document
            .as_ref()
            .ok_or_else(|| "No canonical owned netlist document is available.".to_owned())?;
        let source = state
            .workspace
            .netlist_source
            .as_ref()
            .filter(|source| source.as_str() == document.source())
            .cloned()
            .ok_or_else(|| {
                "Owned netlist bytes do not match their canonical document projection.".to_owned()
            })?;
        let dependencies = document.dependencies().to_vec();
        let validation_source = if dependencies.is_empty() {
            source.clone()
        } else {
            crate::state::expand_retained_netlist_dependencies(
                document.id(),
                &source,
                &dependencies,
            )
            .map_err(|error| format!("Retained dependency closure is invalid: {error}"))?
            .source
        };
        let selected_dialect = descriptor
            .imported_dialect
            .unwrap_or(crate::state::NetlistSourceDialect::RSpice);
        let mut issues = validate_import_candidate(
            &validation_source,
            None,
            selected_dialect.execution_profile(),
        );
        if selected_dialect.requires_compatibility_review() {
            issues.push(NetlistImportIssue {
                severity: NetlistImportIssueSeverity::Advisory,
                message: format!(
                    "{} is quarantined until the exact versioned execution profile is accepted.",
                    selected_dialect.label()
                ),
            });
        }
        let (detected_dialect, detection_evidence) = detect_netlist_dialect(&source);
        Ok((
            descriptor.artifact_name.clone(),
            descriptor.source_encoding,
            descriptor.source_line_ending,
            selected_dialect,
            detected_dialect,
            detection_evidence,
            source,
            dependencies,
            issues,
        ))
    })();
    let (
        display_name,
        encoding,
        line_ending,
        selected_dialect,
        detected_dialect,
        detection_evidence,
        source,
        dependencies,
        issues,
    ) = match candidate {
        Ok(candidate) => candidate,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Execution-profile review is unavailable: {error}"
            )));
            return false;
        }
    };
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Execution-profile review is unavailable: {error}"
                )));
                return false;
            }
        };
    let original_byte_count = source.len();
    let original_sha256 = sha256(source.as_bytes());
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: NetlistImportOperation::RequalifyOwnedSource,
        display_name,
        selected_file_path: None,
        source_path: None,
        source,
        dependencies,
        archive_import: false,
        original_byte_count,
        original_sha256,
        encoding,
        line_ending,
        detected_dialect,
        selected_dialect,
        detection_evidence,
        transformations: vec![
            "Retained the exact project-owned root source and authenticated dependency bytes."
                .to_owned(),
            "Any qualified foreign presentation directive is adapted only in the sealed executable copy; project source, models, units, and analysis statements remain unchanged."
                .to_owned(),
            "The commit updates only versioned execution-profile authority and leaves source history intact."
                .to_owned(),
        ],
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Commit the exact reviewed candidate as one project transaction. The live
/// state is replaced only after a complete clone validates and applies, so an
/// allocation, parse, origin, or domain error cannot partially import a deck.
pub(crate) fn commit_staged_netlist_import(state: &mut AppState) -> bool {
    let Some(review) = state.ui.netlist.import_review.clone() else {
        return false;
    };
    if let Err(error) = review.dialect_qualification() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(error);
        }
        return false;
    }
    if !review.can_commit() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "Resolve every blocking issue and accept the declared compatibility profile before importing."
                    .to_owned(),
            );
        }
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        review.transaction,
    ) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The project changed after this review opened. Cancel and import the source again: {error}"
            ));
        }
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(path) = review.selected_file_path.as_deref() {
        let current_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                if let Some(current) = state.ui.netlist.import_review.as_mut() {
                    current.error = Some(format!(
                        "The selected source can no longer be read. Cancel and import it again: {error}"
                    ));
                }
                return false;
            }
        };
        if current_bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES
            || sha256(&current_bytes) != review.original_sha256
        {
            if let Some(current) = state.ui.netlist.import_review.as_mut() {
                current.error = Some(
                    "The selected source changed after review began. Cancel and import the new bytes again."
                        .to_owned(),
                );
            }
            return false;
        }
    }

    let validation_source = if review.dependencies.is_empty() {
        review.source.clone()
    } else {
        match crate::state::expand_retained_netlist_dependencies(
            crate::state::NetlistDocumentId::new(),
            &review.source,
            &review.dependencies,
        ) {
            Ok(expanded) => expanded.source,
            Err(error) => {
                if let Some(current) = state.ui.netlist.import_review.as_mut() {
                    current.error = Some(format!(
                        "The retained dependency closure no longer validates: {error}"
                    ));
                }
                return false;
            }
        }
    };
    let current_issues = validate_import_candidate(
        &validation_source,
        review.source_path.as_deref(),
        review.selected_dialect.execution_profile(),
    );
    if let Some(blocking) = current_issues.iter().find(|issue| {
        issue.severity
            == crate::workbench::documents::netlist_document::NetlistImportIssueSeverity::Blocking
    }) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The candidate no longer passes canonical validation: {}",
                blocking.message
            ));
        }
        return false;
    }

    if review.operation
        == crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource
    {
        return commit_owned_netlist_profile_review(state, review);
    }

    let metadata = NetlistImportMetadata {
        encoding: review.encoding,
        line_ending: review.line_ending,
        dialect: review.selected_dialect,
        compatibility_reviewed: review.selected_dialect.requires_compatibility_review(),
        raw_sha256: review.original_sha256,
    };
    let mode = match review.operation {
        crate::workbench::documents::netlist_document::NetlistImportOperation::OpenProject => {
            NetlistImportMode::OpenProject
        }
        crate::workbench::documents::netlist_document::NetlistImportOperation::ImportIntoProject => {
            NetlistImportMode::ImportIntoProject
        }
        crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource => {
            if let Some(current) = state.ui.netlist.import_review.as_mut() {
                current.error = Some(
                    "The owned-source profile review reached the import dispatcher without committing. Review the source profile again."
                        .to_owned(),
                );
            }
            return false;
        }
    };
    let mut committed = state.clone();
    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut committed);
    if !apply_netlist_import_result(
        &mut committed,
        mode,
        review.source,
        review.source_path,
        &review.display_name,
        metadata,
        review.dependencies,
    ) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The reviewed import could not be committed; the current project remains unchanged."
                    .to_owned(),
            );
        }
        return false;
    }
    *state = committed;
    true
}

pub(super) fn commit_owned_netlist_profile_review(
    state: &mut AppState,
    review: crate::workbench::documents::netlist_document::NetlistImportReviewState,
) -> bool {
    let Some(profile) = review.selected_dialect.execution_profile() else {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The selected dialect has no versioned executable profile in this build."
                    .to_owned(),
            );
        }
        return false;
    };
    if state.workspace.netlist_source.as_deref() != Some(review.source.as_str())
        || sha256(review.source.as_bytes()) != review.original_sha256
    {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The owned source changed after review began. Cancel and review the current bytes again."
                    .to_owned(),
            );
        }
        return false;
    }

    let mut committed = state.clone();
    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut committed);
    let Some(descriptor) = committed.workspace.netlist_descriptor.as_mut() else {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some("The owned netlist descriptor is no longer available.".to_owned());
        }
        return false;
    };
    descriptor.imported_dialect = Some(review.selected_dialect);
    descriptor.compatibility_reviewed = review.selected_dialect.requires_compatibility_review();
    descriptor.execution_profile = Some(profile);
    committed.workspace.project_metadata_dirty = true;
    committed.ui.netlist.import_review = None;
    if let Err(error) = committed.workspace.validate_simulation_configuration() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The reviewed profile could not be recorded without invalidating project state: {error}"
            ));
        }
        return false;
    }
    *state = committed;
    state.push_user_message(ConsoleMessage::info(format!(
        "Recorded execution profile {} for the exact owned netlist source.",
        profile.id()
    )));
    true
}

pub(crate) fn cancel_staged_netlist_import(state: &mut AppState) {
    if state.ui.netlist.import_review.take().is_some() {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
    }
}

#[cfg(test)]
mod tests {
    #![cfg(not(target_arch = "wasm32"))]

    use super::*;

    struct Fixture(std::path::PathBuf);

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn plain_native_import_seals_direct_and_transitive_include_bytes() {
        let directory = std::env::temp_dir().join(format!(
            "rspice-netlist-import-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let fixture = Fixture(directory);
        std::fs::create_dir_all(fixture.0.join("models/nested")).unwrap();
        let root = "root\n.include \"models/base.lib\"\nV1 out 0 1\n.end\n";
        let base = ".include \"nested/leaf.lib\"\n.model RMOD R\n";
        let leaf = ".param scale=2\n";
        let root_path = fixture.0.join("root.cir");
        std::fs::write(&root_path, root).unwrap();
        std::fs::write(fixture.0.join("models/base.lib"), base).unwrap();
        std::fs::write(fixture.0.join("models/nested/leaf.lib"), leaf).unwrap();

        let dependencies = resolve_plain_import_dependencies(root, Some(&root_path)).unwrap();

        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies[0].requested_locator(), "models/base.lib");
        assert!(dependencies[0].direct_include_index().is_some());
        assert_eq!(dependencies[0].source(), Some(base));
        assert_eq!(dependencies[1].requested_locator(), "nested/leaf.lib");
        assert_eq!(
            dependencies[1]
                .parent()
                .map(crate::state::SourceLocator::logical_identity),
            Some(dependencies[0].locator().logical_identity())
        );
        assert_eq!(dependencies[1].source(), Some(leaf));
    }
}
