//! Versioned, multi-file Automation workspace contract.
//!
//! The product surface exposes Python as the imperative entry point, YAML as
//! the declarative run plan, a read-only environment lock, and a fail-closed
//! TOML capability manifest.  Execution consumes only this compiled closure;
//! it never infers authority from labels painted by the UI.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use super::parser::{ArtifactKind, AutomationPlan, AutomationPlanComponents};
use crate::state::AutomationStarterFile;

// Low-level diagnostic placeholders only. Product identity is always supplied
// by persisted bundle paths and role bindings; keeping these private prevents
// callers from treating starter-template names as policy.
const PYTHON_ENTRY_PATH: &str = AutomationStarterFile::PythonEntry.path();
const RUN_PLAN_PATH: &str = AutomationStarterFile::RunPlan.path();
const ENVIRONMENT_LOCK_PATH: &str = AutomationStarterFile::EnvironmentLock.path();
const PERMISSIONS_PATH: &str = AutomationStarterFile::Permissions.path();

#[derive(Clone, Copy, Debug)]
pub struct AutomationWorkspaceSources<'a> {
    pub python: &'a str,
    pub run_plan: &'a str,
    pub environment_lock: &'a str,
    pub permissions: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationWorkspaceManifest {
    pub entry_path: String,
    pub run_plan_path: String,
    pub environment_lock_path: String,
    pub permissions_path: String,
    pub schema: String,
    pub name: String,
    pub source: String,
    pub analyses: Vec<String>,
    pub corners: String,
    pub requirements: String,
    pub artifacts: Vec<ArtifactKind>,
    pub target: String,
    pub baseline: String,
    pub compare_waveforms: bool,
    pub python_version: String,
    pub api_version: String,
    pub browser_runtime_requirement: Option<String>,
    pub environment_digest: String,
    /// Complete, immutable package artifact closure selected by the lock.
    /// Empty means the signed RSpice base environment only; it never means
    /// "consult pip" or inherit packages from a system Python installation.
    pub locked_artifacts: Vec<AutomationLockedArtifact>,
    pub project_files: String,
    pub artifact_directory: String,
    pub network: String,
    pub process_spawn: String,
    pub environment: Vec<String>,
    pub secret_logging: String,
}

/// One content-addressed package artifact supplied by an RSpice release.
/// A package may have separate records for different targets, but each
/// package/target pair resolves exactly once inside a valid lock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationLockedArtifact {
    pub package: String,
    pub version: String,
    pub filename: String,
    pub sha256: String,
    pub targets: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationWorkspaceDiagnostic {
    pub code: &'static str,
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl AutomationWorkspaceDiagnostic {
    fn new(
        code: &'static str,
        path: impl Into<String>,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            path: path.into(),
            line: line.max(1),
            column: 1,
            message: message.into(),
        }
    }
}

/// One arbitrary project-owned document in an Automation source closure.
/// Logical paths are data; none of the mockup's sample names are identities.
#[derive(Clone, Copy, Debug)]
pub struct AutomationSourceDocument<'a> {
    pub path: &'a str,
    pub source: &'a str,
}

/// Semantic document role supplied by persisted project configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AutomationSourceRole {
    RunPlan,
    EnvironmentLock,
    PermissionManifest,
}

/// One role binding in an arbitrary Automation source closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutomationRoleBinding<'a> {
    pub path: &'a str,
    pub role: AutomationSourceRole,
}

/// Resolve persisted governed document roles inside an arbitrary Automation
/// bundle, then compile the complete closure. Additional Python modules,
/// configuration documents, data fixtures, and package files remain part of
/// the authenticated digest even when they do not own a governed role.
pub fn compile_automation_documents(
    entry_path: &str,
    documents: &[AutomationSourceDocument<'_>],
    roles: &[AutomationRoleBinding<'_>],
) -> Result<(AutomationPlan, AutomationWorkspaceManifest), Vec<AutomationWorkspaceDiagnostic>> {
    let mut diagnostics = Vec::new();
    let Some(entry) = document_by_path(documents, entry_path) else {
        return Err(vec![AutomationWorkspaceDiagnostic::new(
            "AUTWS001",
            entry_path,
            1,
            "the configured Python entry document is missing",
        )]);
    };
    if !entry_path.to_ascii_lowercase().ends_with(".py") {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTWS002",
            entry_path,
            1,
            "the Automation entry document must be a Python source file",
        ));
    }

    let mut role_documents = |role| {
        roles
            .iter()
            .filter(|binding| binding.role == role)
            .filter_map(|binding| {
                document_by_path(documents, binding.path).or_else(|| {
                    diagnostics.push(AutomationWorkspaceDiagnostic::new(
                        "AUTWS007",
                        binding.path,
                        1,
                        format!("the configured {role:?} document is missing"),
                    ));
                    None
                })
            })
            .collect::<Vec<_>>()
    };
    let run_plan_documents = role_documents(AutomationSourceRole::RunPlan);
    let lock_documents = role_documents(AutomationSourceRole::EnvironmentLock);
    let permission_documents = role_documents(AutomationSourceRole::PermissionManifest);

    let referenced_plans = static_literal_run_plan_references(entry.source);
    let run_plan = if referenced_plans.len() == 1 {
        run_plan_documents
            .iter()
            .copied()
            .find(|document| same_project_path(document.path, &referenced_plans[0]))
            .or_else(|| {
                diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTWS003",
                &referenced_plans[0],
                1,
                "the statically referenced run plan is missing or not bound as a run-plan document",
            ));
                None
            })
    } else {
        if referenced_plans.len() > 1 || run_plan_documents.len() != 1 {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTWS004",
                entry_path,
                1,
                "the execution entry must resolve one unambiguous run-plan document",
            ));
            None
        } else {
            run_plan_documents.first().copied()
        }
    };
    let lock = unique_bound_role_document(
        &lock_documents,
        "AUTWS005",
        "environment lock",
        entry_path,
        &mut diagnostics,
    );
    let permissions = unique_bound_role_document(
        &permission_documents,
        "AUTWS006",
        "permission manifest",
        entry_path,
        &mut diagnostics,
    );
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let (Some(run_plan), Some(lock), Some(permissions)) = (run_plan, lock, permissions) else {
        return Err(vec![AutomationWorkspaceDiagnostic::new(
            "AUTWS008",
            entry_path,
            1,
            "Automation role resolution did not produce every required governed document",
        )]);
    };
    let (compiled, mut manifest) = compile_automation_workspace(AutomationWorkspaceSources {
        python: entry.source,
        run_plan: run_plan.source,
        environment_lock: lock.source,
        permissions: permissions.source,
    })
    .map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|mut diagnostic| {
                diagnostic.path = match diagnostic.path.as_str() {
                    PYTHON_ENTRY_PATH => entry.path,
                    RUN_PLAN_PATH => run_plan.path,
                    ENVIRONMENT_LOCK_PATH => lock.path,
                    PERMISSIONS_PATH => permissions.path,
                    _ => diagnostic.path.as_str(),
                }
                .to_owned();
                diagnostic
            })
            .collect::<Vec<_>>()
    })?;
    manifest.entry_path = entry.path.to_owned();
    manifest.run_plan_path = run_plan.path.to_owned();
    manifest.environment_lock_path = lock.path.to_owned();
    manifest.permissions_path = permissions.path.to_owned();

    let digest = document_closure_digest(entry_path, documents, roles);
    let plan = AutomationPlan::from_workspace(AutomationPlanComponents {
        source_digest: digest,
        project_name: compiled.project_name().to_owned(),
        artifacts: compiled.artifacts().collect(),
        corners: compiled.corners().to_owned(),
        target: compiled.target().to_owned(),
        required_specs: compiled.required_specs().to_owned(),
        baseline: compiled.baseline().to_owned(),
        compare_waveforms: compiled.compare_waveforms(),
    });
    Ok((plan, manifest))
}

fn document_by_path<'a>(
    documents: &'a [AutomationSourceDocument<'a>],
    path: &str,
) -> Option<AutomationSourceDocument<'a>> {
    documents
        .iter()
        .copied()
        .find(|document| same_project_path(document.path, path))
}

fn same_project_path(left: &str, right: &str) -> bool {
    left.to_lowercase() == right.to_lowercase()
}

fn unique_bound_role_document<'a>(
    candidates: &[AutomationSourceDocument<'a>],
    code: &'static str,
    label: &str,
    entry_path: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<AutomationSourceDocument<'a>> {
    if candidates.len() == 1 {
        candidates.first().copied()
    } else {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            code,
            entry_path,
            1,
            format!(
                "the Automation bundle must resolve one {label}; found {} candidates",
                candidates.len()
            ),
        ));
        None
    }
}

/// Finds only direct string-literal `run_plans.load(...)` selections in Python
/// code. This is a conservative workspace-selection aid, never Python grammar
/// authority: comments and quoted content are skipped, and an unrecognized or
/// dynamic expression simply contributes no selection. The managed CPython
/// runtime remains the sole syntax/evaluation authority.
fn static_literal_run_plan_references(source: &str) -> Vec<String> {
    let mut references: Vec<String> = Vec::new();
    let bytes = source.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'#' => cursor = skip_python_line_comment(bytes, cursor),
            b'\'' | b'"' => cursor = skip_python_string(bytes, cursor),
            b'.' => {
                if let Some((path, next)) = parse_literal_run_plan_call(source, cursor) {
                    if !references
                        .iter()
                        .any(|current| same_project_path(current, &path))
                    {
                        references.push(path);
                    }
                    cursor = next;
                } else {
                    cursor += 1;
                }
            }
            byte if byte.is_ascii() => cursor += 1,
            _ => {
                cursor += source[cursor..].chars().next().map_or(1, char::len_utf8);
            }
        }
    }
    references
}

fn skip_python_line_comment(bytes: &[u8], start: usize) -> usize {
    bytes[start..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(bytes.len(), |offset| start + offset + 1)
}

fn skip_python_string(bytes: &[u8], start: usize) -> usize {
    let quote = bytes[start];
    let triple = bytes.get(start + 1) == Some(&quote) && bytes.get(start + 2) == Some(&quote);
    let delimiter_len = if triple { 3 } else { 1 };
    let mut cursor = start + delimiter_len;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor = (cursor + 2).min(bytes.len());
            continue;
        }
        if bytes[cursor] == quote {
            if !triple {
                return cursor + 1;
            }
            if bytes.get(cursor + 1) == Some(&quote) && bytes.get(cursor + 2) == Some(&quote) {
                return cursor + 3;
            }
        }
        cursor += 1;
    }
    bytes.len()
}

fn parse_literal_run_plan_call(source: &str, start: usize) -> Option<(String, usize)> {
    let bytes = source.as_bytes();
    let mut cursor = start;
    consume_byte(bytes, &mut cursor, b'.')?;
    skip_python_trivia(bytes, &mut cursor);
    consume_identifier(bytes, &mut cursor, b"run_plans")?;
    skip_python_trivia(bytes, &mut cursor);
    consume_byte(bytes, &mut cursor, b'.')?;
    skip_python_trivia(bytes, &mut cursor);
    consume_identifier(bytes, &mut cursor, b"load")?;
    skip_python_trivia(bytes, &mut cursor);
    consume_byte(bytes, &mut cursor, b'(')?;
    skip_python_trivia(bytes, &mut cursor);

    let quote = *bytes.get(cursor)?;
    if !matches!(quote, b'\'' | b'"') {
        return None;
    }
    let content_start = cursor + 1;
    cursor = content_start;
    while cursor < bytes.len() && bytes[cursor] != quote {
        // Portable project paths reject backslashes and quotes. Refuse escape
        // interpretation here rather than accidentally giving this selector
        // a second, incomplete Python-string grammar.
        if bytes[cursor] == b'\\' || matches!(bytes[cursor], b'\r' | b'\n') {
            return None;
        }
        cursor += if bytes[cursor].is_ascii() {
            1
        } else {
            source[cursor..].chars().next()?.len_utf8()
        };
    }
    if bytes.get(cursor) != Some(&quote) {
        return None;
    }
    let path = source.get(content_start..cursor)?.to_owned();
    cursor += 1;
    skip_python_trivia(bytes, &mut cursor);
    consume_byte(bytes, &mut cursor, b')')?;
    Some((path, cursor))
}

fn consume_byte(bytes: &[u8], cursor: &mut usize, expected: u8) -> Option<()> {
    if bytes.get(*cursor) != Some(&expected) {
        return None;
    }
    *cursor += 1;
    Some(())
}

fn consume_identifier(bytes: &[u8], cursor: &mut usize, expected: &[u8]) -> Option<()> {
    if !bytes.get(*cursor..)?.starts_with(expected) {
        return None;
    }
    let next = *cursor + expected.len();
    if bytes
        .get(next)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    {
        return None;
    }
    *cursor = next;
    Some(())
}

fn skip_python_trivia(bytes: &[u8], cursor: &mut usize) {
    loop {
        while bytes.get(*cursor).is_some_and(u8::is_ascii_whitespace) {
            *cursor += 1;
        }
        if bytes.get(*cursor) == Some(&b'#') {
            *cursor = skip_python_line_comment(bytes, *cursor);
            continue;
        }
        if bytes.get(*cursor) == Some(&b'\\') {
            let newline_bytes = match (bytes.get(*cursor + 1), bytes.get(*cursor + 2)) {
                (Some(b'\n'), _) => 2,
                (Some(b'\r'), Some(b'\n')) => 3,
                _ => 0,
            };
            if newline_bytes > 0 {
                *cursor += newline_bytes;
                continue;
            }
        }
        break;
    }
}

fn document_closure_digest(
    entry_path: &str,
    documents: &[AutomationSourceDocument<'_>],
    roles: &[AutomationRoleBinding<'_>],
) -> [u8; 32] {
    let mut ordered = documents.to_vec();
    ordered.sort_by_key(|document| document.path.to_lowercase());
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.automation-document-closure/v1");
    hash_dynamic_frame(&mut hasher, entry_path.as_bytes());
    for document in ordered {
        hash_dynamic_frame(&mut hasher, document.path.as_bytes());
        hash_dynamic_frame(&mut hasher, document.source.as_bytes());
    }
    let mut ordered_roles = roles.to_vec();
    ordered_roles.sort_by_key(|binding| {
        (
            binding.role,
            binding.path.to_lowercase(),
            binding.path.to_owned(),
        )
    });
    for binding in ordered_roles {
        let role = match binding.role {
            AutomationSourceRole::RunPlan => "run-plan",
            AutomationSourceRole::EnvironmentLock => "environment-lock",
            AutomationSourceRole::PermissionManifest => "permission-manifest",
        };
        hash_dynamic_frame(&mut hasher, role.as_bytes());
        hash_dynamic_frame(&mut hasher, binding.path.as_bytes());
    }
    hasher.finalize().into()
}

fn hash_dynamic_frame(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

/// Compile one resolved set of Automation document roles into an immutable
/// execution plan. This accepts general project content; the constants in
/// this module provide only the new-project starter template.
pub fn compile_automation_workspace(
    sources: AutomationWorkspaceSources<'_>,
) -> Result<(AutomationPlan, AutomationWorkspaceManifest), Vec<AutomationWorkspaceDiagnostic>> {
    let mut diagnostics = Vec::new();
    validate_python_entry(sources.python, &mut diagnostics);
    let run_plan = parse_run_plan(sources.run_plan, &mut diagnostics);
    let lock = parse_environment_lock(sources.environment_lock, &mut diagnostics);
    let permissions = parse_permissions(sources.permissions, &mut diagnostics);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let (Some(run_plan), Some(lock), Some(permissions)) = (run_plan, lock, permissions) else {
        return Err(vec![AutomationWorkspaceDiagnostic::new(
            "AUTWS008",
            PYTHON_ENTRY_PATH,
            1,
            "Automation workspace parsing did not resolve every required governed document",
        )]);
    };
    let artifact_set = run_plan.artifacts.iter().copied().collect::<BTreeSet<_>>();
    let digest = workspace_digest(sources);
    let plan = AutomationPlan::from_workspace(AutomationPlanComponents {
        source_digest: digest,
        project_name: run_plan.name.clone(),
        artifacts: artifact_set,
        corners: run_plan.corners.clone(),
        target: run_plan.target.clone(),
        required_specs: run_plan.requirements.clone(),
        baseline: run_plan.baseline.clone(),
        compare_waveforms: run_plan.compare_waveforms,
    });
    let manifest = AutomationWorkspaceManifest {
        entry_path: PYTHON_ENTRY_PATH.to_owned(),
        run_plan_path: RUN_PLAN_PATH.to_owned(),
        environment_lock_path: ENVIRONMENT_LOCK_PATH.to_owned(),
        permissions_path: PERMISSIONS_PATH.to_owned(),
        schema: run_plan.schema,
        name: run_plan.name,
        source: run_plan.source,
        analyses: run_plan.analyses,
        corners: run_plan.corners,
        requirements: run_plan.requirements,
        artifacts: run_plan.artifacts,
        target: run_plan.target,
        baseline: run_plan.baseline,
        compare_waveforms: run_plan.compare_waveforms,
        python_version: lock.python_version,
        api_version: lock.api_version,
        browser_runtime_requirement: lock.browser_runtime_requirement,
        environment_digest: lock.environment_digest,
        locked_artifacts: lock.locked_artifacts,
        project_files: permissions.project_files,
        artifact_directory: permissions.artifact_directory,
        network: permissions.network,
        process_spawn: permissions.process_spawn,
        environment: permissions.environment,
        secret_logging: permissions.secret_logging,
    };
    Ok((plan, manifest))
}

fn workspace_digest(sources: AutomationWorkspaceSources<'_>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.automation-workspace/v1");
    for (path, source) in [
        (PYTHON_ENTRY_PATH, sources.python),
        (RUN_PLAN_PATH, sources.run_plan),
        (ENVIRONMENT_LOCK_PATH, sources.environment_lock),
        (PERMISSIONS_PATH, sources.permissions),
    ] {
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update((source.len() as u64).to_be_bytes());
        hasher.update(source.as_bytes());
    }
    hasher.finalize().into()
}

fn validate_python_entry(source: &str, diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>) {
    if source.len() > 2 * 1024 * 1024 {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTPY000",
            PYTHON_ENTRY_PATH,
            1,
            "Python source exceeds the 2 MiB per-document safety limit",
        ));
        return;
    }
    if source.trim().is_empty() {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTPY001",
            PYTHON_ENTRY_PATH,
            1,
            "Python entry source is empty",
        ));
        return;
    }
    if let Some(offset) = source.as_bytes().iter().position(|byte| *byte == 0) {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTPY002",
            PYTHON_ENTRY_PATH,
            source[..offset]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count()
                + 1,
            "Python source contains a NUL byte",
        ));
    }
    // Do not make the editor's bundled Rust parser an execution gate. The
    // shipping language is CPython 3.14 and the authoritative managed native
    // or Pyodide worker compiles every source document before execution. A
    // lagging convenience parser must never reject valid new CPython syntax.
}

#[derive(Clone, Debug)]
struct ParsedRunPlan {
    schema: String,
    name: String,
    source: String,
    analyses: Vec<String>,
    corners: String,
    requirements: String,
    artifacts: Vec<ArtifactKind>,
    target: String,
    baseline: String,
    compare_waveforms: bool,
}

fn parse_run_plan(
    source: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<ParsedRunPlan> {
    let fields = parse_closed_fields(
        source,
        RUN_PLAN_PATH,
        ':',
        &[
            "schema",
            "name",
            "source",
            "analyses",
            "corners",
            "requirements",
            "artifacts",
            "target",
            "baseline",
            "compare-waveforms",
        ],
        &["schema", "name", "source", "analyses"],
        false,
        diagnostics,
    );
    let get = |key: &str| fields.get(key).map(|(_, value)| value.clone());
    let schema = get("schema")?;
    let name = get("name")?;
    let source_path = get("source")?;
    let corners = get("corners").unwrap_or_else(|| "nominal".to_owned());
    let requirements = get("requirements").unwrap_or_else(|| "none".to_owned());
    let target = get("target").unwrap_or_else(|| "local".to_owned());
    let baseline = get("baseline").unwrap_or_default();
    let compare_waveforms = get("compare-waveforms")
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(!baseline.is_empty());
    if schema != "rspice.run-plan/v1" {
        field_error(
            &fields,
            "schema",
            RUN_PLAN_PATH,
            "AUTYAML010",
            "schema must be rspice.run-plan/v1",
            diagnostics,
        );
    }
    if name.is_empty() || name.chars().count() > 256 || name.chars().any(char::is_control) {
        field_error(
            &fields,
            "name",
            RUN_PLAN_PATH,
            "AUTYAML011",
            "name must contain 1 to 256 non-control characters",
            diagnostics,
        );
    }
    if source_path.is_empty()
        || source_path
            .chars()
            .any(|character| matches!(character, '\\' | '\0'))
        || source_path.starts_with('/')
        || source_path.split('/').any(|part| part == "..")
    {
        field_error(
            &fields,
            "source",
            RUN_PLAN_PATH,
            "AUTYAML012",
            "source must be a portable project-relative netlist path",
            diagnostics,
        );
    }
    if corners.is_empty() || corners.chars().count() > 256 || corners.chars().any(char::is_control)
    {
        field_error(
            &fields,
            "corners",
            RUN_PLAN_PATH,
            "AUTYAML013",
            "corners must be a non-empty named corner selector",
            diagnostics,
        );
    }
    if requirements.is_empty()
        || requirements.chars().count() > 256
        || requirements.chars().any(char::is_control)
    {
        field_error(
            &fields,
            "requirements",
            RUN_PLAN_PATH,
            "AUTYAML014",
            "requirements must be a non-empty named requirement profile",
            diagnostics,
        );
    }
    let analyses = parse_list_field(
        &fields,
        "analyses",
        RUN_PLAN_PATH,
        "AUTYAML015",
        diagnostics,
    );
    let allowed = [
        "op",
        "tran",
        "ac",
        "dc",
        "noise",
        "pz",
        "sens",
        "mc",
        "pss",
        "stb",
        "temp",
        "hb",
        "sp",
        "pac",
        "pnoise",
        "pxf",
        "pstb",
        "xf",
        "corner",
        "envelope",
        "fourier",
        "reliability",
        "opt",
        "soa",
        "disto",
        "qpss",
        "hbsp",
        "hbnoise",
        "psp",
        "qpac",
        "qpnoise",
        "qpxf",
        "tnoise",
        "dcmatch",
    ];
    if analyses.is_empty()
        || analyses
            .iter()
            .any(|analysis| !allowed.contains(&analysis.as_str()))
        || unique_len(&analyses) != analyses.len()
    {
        field_error(
            &fields,
            "analyses",
            RUN_PLAN_PATH,
            "AUTYAML016",
            "analyses must be a non-empty unique list of supported analysis IDs",
            diagnostics,
        );
    }
    let artifact_names = parse_list_field(
        &fields,
        "artifacts",
        RUN_PLAN_PATH,
        "AUTYAML017",
        diagnostics,
    );
    let mut artifacts = Vec::new();
    for name in &artifact_names {
        let kind = match name.as_str() {
            "junit" => Some(ArtifactKind::JunitXml),
            "summary-json" => Some(ArtifactKind::SummaryJson),
            "verification-pdf" => Some(ArtifactKind::VerificationPdf),
            _ => None,
        };
        if let Some(kind) = kind {
            if !artifacts.contains(&kind) {
                artifacts.push(kind);
            }
        } else {
            field_error(
                &fields,
                "artifacts",
                RUN_PLAN_PATH,
                "AUTYAML018",
                "unsupported artifact; use junit, summary-json, or verification-pdf",
                diagnostics,
            );
        }
    }
    if artifacts.len() != artifact_names.len() {
        field_error(
            &fields,
            "artifacts",
            RUN_PLAN_PATH,
            "AUTYAML019",
            "artifacts must be unique",
            diagnostics,
        );
    }
    Some(ParsedRunPlan {
        schema,
        name,
        source: source_path,
        analyses,
        corners,
        requirements,
        artifacts,
        target,
        baseline,
        compare_waveforms,
    })
}

#[derive(Clone, Debug)]
struct ParsedEnvironmentLock {
    python_version: String,
    api_version: String,
    browser_runtime_requirement: Option<String>,
    environment_digest: String,
    locked_artifacts: Vec<AutomationLockedArtifact>,
}

fn parse_environment_lock(
    source: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<ParsedEnvironmentLock> {
    if source.len() > 256 * 1024 {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTFILE000",
            ENVIRONMENT_LOCK_PATH,
            1,
            "source exceeds the 256 KiB governed-file limit",
        ));
        return None;
    }
    if source
        .lines()
        .any(|line| line.trim() == "format = \"rspice-python-lock/v3\"")
    {
        return parse_environment_lock_v3(source, diagnostics);
    }
    parse_legacy_environment_lock(source, diagnostics)
}

fn parse_legacy_environment_lock(
    source: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<ParsedEnvironmentLock> {
    let fields = parse_closed_fields(
        source,
        ENVIRONMENT_LOCK_PATH,
        '=',
        &[
            "format",
            "python",
            "rspice-api",
            "numpy",
            "browser-runtime",
            "environment_digest",
        ],
        &["format", "python", "rspice-api", "environment_digest"],
        true,
        diagnostics,
    );
    let format = field(&fields, "format")?;
    let python = field(&fields, "python")?;
    let api = field(&fields, "rspice-api")?;
    let digest = field(&fields, "environment_digest")?;
    if !matches!(
        format.as_str(),
        "rspice-python-lock/v1" | "rspice-python-lock/v2"
    ) {
        field_error(
            &fields,
            "format",
            ENVIRONMENT_LOCK_PATH,
            "AUTLOCK010",
            "format must be rspice-python-lock/v1 or rspice-python-lock/v2",
            diagnostics,
        );
    }
    let version_requirements_are_valid =
        semver::VersionReq::parse(&python).is_ok() && semver::VersionReq::parse(&api).is_ok();
    if !version_requirements_are_valid {
        field_error(
            &fields,
            "python",
            ENVIRONMENT_LOCK_PATH,
            "AUTLOCK011",
            "Python and RSpice API must use valid semantic-version requirements",
            diagnostics,
        );
    }
    if semver::VersionReq::parse(&api).is_err() {
        field_error(
            &fields,
            "rspice-api",
            ENVIRONMENT_LOCK_PATH,
            "AUTLOCK012",
            "RSpice API must use a valid semantic-version requirement",
            diagnostics,
        );
    }
    if !valid_sha256_literal(&digest) {
        field_error(
            &fields,
            "environment_digest",
            ENVIRONMENT_LOCK_PATH,
            "AUTLOCK013",
            "environment_digest must be sha256 followed by 64 lowercase hexadecimal digits",
            diagnostics,
        );
    }
    let browser_runtime_requirement = field(&fields, "browser-runtime");
    if browser_runtime_requirement
        .as_ref()
        .is_some_and(|requirement| semver::VersionReq::parse(requirement).is_err())
    {
        field_error(
            &fields,
            "browser-runtime",
            ENVIRONMENT_LOCK_PATH,
            "AUTLOCK014",
            "browser-runtime must use a valid semantic-version requirement",
            diagnostics,
        );
    }
    Some(ParsedEnvironmentLock {
        python_version: if format == "rspice-python-lock/v1"
            && !python.starts_with(['=', '<', '>', '^', '~'])
        {
            format!("={python}")
        } else {
            python
        },
        api_version: if format == "rspice-python-lock/v1"
            && !api.starts_with(['=', '<', '>', '^', '~'])
        {
            format!("={api}")
        } else {
            api
        },
        browser_runtime_requirement,
        environment_digest: digest,
        locked_artifacts: Vec::new(),
    })
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentLockV3 {
    format: String,
    python: String,
    #[serde(rename = "rspice-api")]
    rspice_api: String,
    #[serde(rename = "browser-runtime")]
    browser_runtime: Option<String>,
    environment_digest: String,
    #[serde(default)]
    artifact: Vec<EnvironmentArtifactV3>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentArtifactV3 {
    package: String,
    version: String,
    filename: String,
    sha256: String,
    targets: Vec<String>,
    #[serde(default)]
    dependencies: Vec<String>,
}

fn parse_environment_lock_v3(
    source: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<ParsedEnvironmentLock> {
    let parsed = match toml::from_str::<EnvironmentLockV3>(source) {
        Ok(parsed) => parsed,
        Err(error) => {
            let line = error
                .span()
                .map(|span| source[..span.start.min(source.len())].lines().count())
                .unwrap_or(1)
                .max(1);
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK020",
                ENVIRONMENT_LOCK_PATH,
                line,
                format!("invalid environment-lock TOML: {error}"),
            ));
            return None;
        }
    };
    if parsed.format != "rspice-python-lock/v3" {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK021",
            ENVIRONMENT_LOCK_PATH,
            1,
            "format must be rspice-python-lock/v3",
        ));
    }
    if semver::VersionReq::parse(&parsed.python).is_err() {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK022",
            ENVIRONMENT_LOCK_PATH,
            1,
            "python must use a valid semantic-version requirement",
        ));
    }
    if semver::VersionReq::parse(&parsed.rspice_api).is_err() {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK023",
            ENVIRONMENT_LOCK_PATH,
            1,
            "rspice-api must use a valid semantic-version requirement",
        ));
    }
    if parsed
        .browser_runtime
        .as_ref()
        .is_some_and(|requirement| semver::VersionReq::parse(requirement).is_err())
    {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK024",
            ENVIRONMENT_LOCK_PATH,
            1,
            "browser-runtime must use a valid semantic-version requirement",
        ));
    }
    if !valid_sha256_literal(&parsed.environment_digest) {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK025",
            ENVIRONMENT_LOCK_PATH,
            1,
            "environment_digest must be sha256 followed by 64 lowercase hexadecimal digits",
        ));
    }
    if parsed.artifact.len() > 4096 {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTLOCK026",
            ENVIRONMENT_LOCK_PATH,
            1,
            "an environment lock may contain at most 4096 artifact records",
        ));
    }

    let artifacts = parsed
        .artifact
        .into_iter()
        .map(|artifact| AutomationLockedArtifact {
            package: artifact.package,
            version: artifact.version,
            filename: artifact.filename,
            sha256: artifact.sha256,
            targets: artifact.targets,
            dependencies: artifact.dependencies,
        })
        .collect::<Vec<_>>();
    validate_locked_artifacts(&artifacts, diagnostics);

    if valid_sha256_literal(&parsed.environment_digest) {
        let actual = environment_lock_digest(
            &parsed.python,
            &parsed.rspice_api,
            parsed.browser_runtime.as_deref(),
            &artifacts,
        );
        if parsed.environment_digest != actual {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK027",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "environment_digest does not authenticate the canonical resolved environment; expected {actual}"
                ),
            ));
        }
    }

    Some(ParsedEnvironmentLock {
        python_version: parsed.python,
        api_version: parsed.rspice_api,
        browser_runtime_requirement: parsed.browser_runtime,
        environment_digest: parsed.environment_digest,
        locked_artifacts: artifacts,
    })
}

const QUALIFIED_PYTHON_TARGETS: &[&str] = &[
    "native/windows-x86_64/cp314",
    "native/linux-x86_64/cp314",
    "native/linux-aarch64/cp314",
    "native/macos-x86_64/cp314",
    "native/macos-aarch64/cp314",
    "browser/wasm32/pyodide-314.0.2",
];

fn validate_locked_artifacts(
    artifacts: &[AutomationLockedArtifact],
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) {
    let mut resolved = BTreeMap::<(String, String), String>::new();
    for artifact in artifacts {
        let canonical_name = canonical_package_name(&artifact.package);
        if canonical_name.as_deref() != Some(artifact.package.as_str()) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK030",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package name {:?} is not a normalized PEP 503 name",
                    artifact.package
                ),
            ));
        }
        if !valid_python_package_version(&artifact.version) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK031",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package {} has an invalid exact version {:?}",
                    artifact.package, artifact.version
                ),
            ));
        }
        if !valid_python_artifact_filename(&artifact.filename) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK032",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package {} must identify one basename-only wheel artifact",
                    artifact.package
                ),
            ));
        }
        if !valid_sha256_literal(&artifact.sha256) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK033",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package {} artifact digest is not a lowercase SHA-256 literal",
                    artifact.package
                ),
            ));
        }
        let unique_targets = artifact.targets.iter().collect::<BTreeSet<_>>();
        if artifact.targets.is_empty()
            || artifact.targets.len() > QUALIFIED_PYTHON_TARGETS.len()
            || unique_targets.len() != artifact.targets.len()
            || artifact
                .targets
                .iter()
                .any(|target| !QUALIFIED_PYTHON_TARGETS.contains(&target.as_str()))
        {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK034",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package {} must contain a non-empty unique list of qualified target selectors",
                    artifact.package
                ),
            ));
        }
        if artifact.dependencies.len() > 256
            || artifact.dependencies.iter().collect::<BTreeSet<_>>().len()
                != artifact.dependencies.len()
            || artifact
                .dependencies
                .iter()
                .any(|dependency| parse_locked_dependency(dependency).is_none())
        {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTLOCK035",
                ENVIRONMENT_LOCK_PATH,
                1,
                format!(
                    "package {} dependencies must be unique normalized name==version pins",
                    artifact.package
                ),
            ));
        }
        for target in &artifact.targets {
            let key = (artifact.package.clone(), target.clone());
            if resolved.insert(key, artifact.version.clone()).is_some() {
                diagnostics.push(AutomationWorkspaceDiagnostic::new(
                    "AUTLOCK036",
                    ENVIRONMENT_LOCK_PATH,
                    1,
                    format!(
                        "package {} resolves more than once for target {target}",
                        artifact.package
                    ),
                ));
            }
        }
    }

    for artifact in artifacts {
        for dependency in &artifact.dependencies {
            let Some((name, version)) = parse_locked_dependency(dependency) else {
                continue;
            };
            for target in &artifact.targets {
                let key = (name.clone(), target.clone());
                if resolved.get(&key).is_none_or(|actual| actual != &version) {
                    diagnostics.push(AutomationWorkspaceDiagnostic::new(
                        "AUTLOCK037",
                        ENVIRONMENT_LOCK_PATH,
                        1,
                        format!(
                            "package {} requires {dependency}, but that exact artifact is not locked for target {target}",
                            artifact.package
                        ),
                    ));
                }
            }
        }
    }
}

fn canonical_package_name(value: &str) -> Option<String> {
    if value.is_empty() || value.len() > 128 || !value.is_ascii() {
        return None;
    }
    let mut normalized = String::with_capacity(value.len());
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            normalized.push(character.to_ascii_lowercase());
            separator = false;
        } else if matches!(character, '-' | '_' | '.') {
            if !separator && !normalized.is_empty() {
                normalized.push('-');
                separator = true;
            }
        } else {
            return None;
        }
    }
    if normalized.ends_with('-') {
        normalized.pop();
    }
    (!normalized.is_empty()).then_some(normalized)
}

fn valid_python_package_version(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.is_ascii()
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || ".!+-_".contains(character))
}

fn valid_python_artifact_filename(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value.is_ascii()
        && !value.contains(['/', '\\'])
        && value.ends_with(".whl")
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_')
        })
}

fn parse_locked_dependency(value: &str) -> Option<(String, String)> {
    let (name, version) = value.split_once("==")?;
    if canonical_package_name(name).as_deref() != Some(name)
        || !valid_python_package_version(version)
    {
        return None;
    }
    Some((name.to_owned(), version.to_owned()))
}

/// Recompute the canonical content identity of one fully resolved Python
/// environment. This is shared by lock generation, release packaging, and
/// runtime admission; changing line order or comments does not change it.
pub fn environment_lock_digest(
    python_requirement: &str,
    api_requirement: &str,
    browser_runtime_requirement: Option<&str>,
    artifacts: &[AutomationLockedArtifact],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.python-environment/v1");
    hash_dynamic_frame(&mut hasher, python_requirement.as_bytes());
    hash_dynamic_frame(&mut hasher, api_requirement.as_bytes());
    hash_dynamic_frame(
        &mut hasher,
        browser_runtime_requirement.unwrap_or_default().as_bytes(),
    );
    let mut ordered = artifacts.to_vec();
    ordered.sort_by(|left, right| {
        (&left.package, &left.version, &left.filename, &left.sha256).cmp(&(
            &right.package,
            &right.version,
            &right.filename,
            &right.sha256,
        ))
    });
    for artifact in ordered {
        hash_dynamic_frame(&mut hasher, artifact.package.as_bytes());
        hash_dynamic_frame(&mut hasher, artifact.version.as_bytes());
        hash_dynamic_frame(&mut hasher, artifact.filename.as_bytes());
        hash_dynamic_frame(&mut hasher, artifact.sha256.as_bytes());
        let mut targets = artifact.targets;
        targets.sort();
        for target in targets {
            hash_dynamic_frame(&mut hasher, target.as_bytes());
        }
        let mut dependencies = artifact.dependencies;
        dependencies.sort();
        for dependency in dependencies {
            hash_dynamic_frame(&mut hasher, dependency.as_bytes());
        }
    }
    format!("sha256:{:x}", hasher.finalize())
}

#[derive(Clone, Debug)]
struct ParsedPermissions {
    project_files: String,
    artifact_directory: String,
    network: String,
    process_spawn: String,
    environment: Vec<String>,
    secret_logging: String,
}

fn parse_permissions(
    source: &str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Option<ParsedPermissions> {
    let fields = parse_closed_fields(
        source,
        PERMISSIONS_PATH,
        '=',
        &[
            "project_files",
            "artifact_directory",
            "network",
            "process_spawn",
            "environment",
            "secret_logging",
        ],
        &[
            "project_files",
            "artifact_directory",
            "network",
            "process_spawn",
            "secret_logging",
        ],
        false,
        diagnostics,
    );
    let project_files = field(&fields, "project_files")?;
    let artifact_directory = field(&fields, "artifact_directory")?;
    let network = field(&fields, "network")?;
    let process_spawn = field(&fields, "process_spawn")?;
    let secret_logging = field(&fields, "secret_logging")?;
    let environment = parse_list_field(
        &fields,
        "environment",
        PERMISSIONS_PATH,
        "AUTPERM010",
        diagnostics,
    );
    for (key, actual, allowed, code) in [
        (
            "project_files",
            project_files.as_str(),
            &["deny", "read", "write"] as &[_],
            "AUTPERM011",
        ),
        (
            "artifact_directory",
            artifact_directory.as_str(),
            &["deny", "write"],
            "AUTPERM012",
        ),
        (
            "network",
            network.as_str(),
            &["deny", "allow"],
            "AUTPERM013",
        ),
        (
            "process_spawn",
            process_spawn.as_str(),
            &["deny", "allow"],
            "AUTPERM014",
        ),
        (
            "secret_logging",
            secret_logging.as_str(),
            &["redact", "deny"],
            "AUTPERM015",
        ),
    ] {
        if !allowed.contains(&actual) {
            field_error(
                &fields,
                key,
                PERMISSIONS_PATH,
                code,
                format!("{key} must be one of {}", allowed.join(", ")),
                diagnostics,
            );
        }
    }
    if environment.len() > 32
        || unique_len(&environment) != environment.len()
        || environment.iter().any(|name| !valid_environment_name(name))
    {
        field_error(
            &fields,
            "environment",
            PERMISSIONS_PATH,
            "AUTPERM016",
            "environment must contain at most 32 unique portable variable names",
            diagnostics,
        );
    }
    Some(ParsedPermissions {
        project_files,
        artifact_directory,
        network,
        process_spawn,
        environment,
        secret_logging,
    })
}

type ParsedFields = BTreeMap<String, (usize, String)>;

fn parse_closed_fields(
    source: &str,
    path: &str,
    separator: char,
    allowed: &[&str],
    required: &[&str],
    allow_unknown: bool,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> ParsedFields {
    let mut fields = BTreeMap::new();
    if source.len() > 256 * 1024 {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            "AUTFILE000",
            path,
            1,
            "source exceeds the 256 KiB governed-file limit",
        ));
        return fields;
    }
    for (index, raw) in source.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(separator) else {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTFILE001",
                path,
                index + 1,
                format!("expected a `{separator}` field separator"),
            ));
            continue;
        };
        let key = key.trim();
        if !allowed.contains(&key) && !allow_unknown {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTFILE002",
                path,
                index + 1,
                format!("unknown field `{key}`"),
            ));
            continue;
        }
        if fields.contains_key(key) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTFILE003",
                path,
                index + 1,
                format!("duplicate field `{key}`"),
            ));
            continue;
        }
        fields.insert(key.to_owned(), (index + 1, unquote(value.trim())));
    }
    for key in required {
        if !fields.contains_key(*key) {
            diagnostics.push(AutomationWorkspaceDiagnostic::new(
                "AUTFILE004",
                path,
                1,
                format!("missing required field `{key}`"),
            ));
        }
    }
    fields
}

fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .trim()
        .to_owned()
}

fn field(fields: &ParsedFields, key: &str) -> Option<String> {
    fields.get(key).map(|(_, value)| value.clone())
}

fn parse_list_field(
    fields: &ParsedFields,
    key: &str,
    path: &str,
    code: &'static str,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) -> Vec<String> {
    let Some((line, value)) = fields.get(key) else {
        return Vec::new();
    };
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        diagnostics.push(AutomationWorkspaceDiagnostic::new(
            code,
            path,
            *line,
            format!("{key} must use a bracketed list"),
        ));
        return Vec::new();
    };
    inner
        .split(',')
        .map(|value| unquote(value.trim()))
        .filter(|value| !value.is_empty())
        .collect()
}

fn field_error(
    fields: &ParsedFields,
    key: &str,
    path: &str,
    code: &'static str,
    message: impl Into<String>,
    diagnostics: &mut Vec<AutomationWorkspaceDiagnostic>,
) {
    diagnostics.push(AutomationWorkspaceDiagnostic::new(
        code,
        path,
        fields.get(key).map_or(1, |(line, _)| *line),
        message,
    ));
}

fn unique_len(values: &[String]) -> usize {
    values.iter().collect::<BTreeSet<_>>().len()
}

fn valid_sha256_literal(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .chars()
                .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
    })
}

fn valid_environment_name(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|ch| ch == '_' || ch.is_ascii_uppercase())
        && chars.all(|ch| ch == '_' || ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        DEFAULT_AUTOMATION_PERMISSIONS, DEFAULT_AUTOMATION_PYTHON, DEFAULT_AUTOMATION_RUN_PLAN,
        DEFAULT_ENVIRONMENT_LOCK,
    };

    #[test]
    fn workspace_formation_has_no_production_panic_shortcuts() {
        let production = crate::source_guard::production_half(include_str!("workspace.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!(", "unreachable!("] {
            assert!(
                !production.contains(forbidden),
                "Automation workspace formation contains {forbidden}"
            );
        }
    }

    fn sources() -> AutomationWorkspaceSources<'static> {
        AutomationWorkspaceSources {
            python: DEFAULT_AUTOMATION_PYTHON,
            run_plan: DEFAULT_AUTOMATION_RUN_PLAN,
            environment_lock: DEFAULT_ENVIRONMENT_LOCK,
            permissions: DEFAULT_AUTOMATION_PERMISSIONS,
        }
    }

    #[test]
    fn canonical_workspace_compiles_to_one_exact_plan_and_manifest() {
        let (plan, manifest) = compile_automation_workspace(sources()).unwrap();
        assert_eq!(plan.project_name(), "Lab characterization");
        assert_eq!(plan.target(), "local");
        assert_eq!(plan.required_specs(), "release");
        assert_eq!(plan.artifacts().count(), 3);
        assert_eq!(manifest.python_version, ">=3.14.0,<3.15.0");
        assert_eq!(manifest.api_version, ">=1.0.0,<2.0.0");
        assert_eq!(
            manifest.browser_runtime_requirement.as_deref(),
            Some("=314.0.2")
        );
        assert_eq!(manifest.network, "deny");
        assert_eq!(manifest.process_spawn, "deny");
        assert!(manifest.locked_artifacts.is_empty());
    }

    #[test]
    fn every_file_is_exactly_bound_into_the_plan_digest() {
        let (baseline, _) = compile_automation_workspace(sources()).unwrap();
        for kind in AutomationStarterFile::ALL {
            let mut changed = sources();
            let owned = match kind {
                AutomationStarterFile::PythonEntry => {
                    format!("{}# changed\n", changed.python)
                }
                AutomationStarterFile::RunPlan => changed
                    .run_plan
                    .replace("Lab characterization", "Changed characterization"),
                AutomationStarterFile::EnvironmentLock => {
                    format!("{}# changed\n", changed.environment_lock)
                }
                AutomationStarterFile::Permissions => changed
                    .permissions
                    .replace("RSPICE_LICENSE_TOKEN", "RSPICE_ALT_TOKEN"),
            };
            match kind {
                AutomationStarterFile::PythonEntry => changed.python = &owned,
                AutomationStarterFile::RunPlan => changed.run_plan = &owned,
                AutomationStarterFile::EnvironmentLock => changed.environment_lock = &owned,
                AutomationStarterFile::Permissions => changed.permissions = &owned,
            }
            let (candidate, _) = compile_automation_workspace(changed).unwrap();
            assert_ne!(
                candidate.source_digest(),
                baseline.source_digest(),
                "{}",
                kind.path()
            );
        }
    }

    #[test]
    fn arbitrary_paths_and_additional_documents_form_the_authenticated_closure() {
        let entry =
            DEFAULT_AUTOMATION_PYTHON.replace("runplan.rspice.yaml", "plans/nightly-signoff.yaml");
        let helper = "def corner_label(name):\n    return name.upper()\n";
        let documents = [
            AutomationSourceDocument {
                path: "flows/nightly.py",
                source: &entry,
            },
            AutomationSourceDocument {
                path: "plans/nightly-signoff.yaml",
                source: DEFAULT_AUTOMATION_RUN_PLAN,
            },
            AutomationSourceDocument {
                path: "environment/frozen-runtime.toml",
                source: DEFAULT_ENVIRONMENT_LOCK,
            },
            AutomationSourceDocument {
                path: "security/capabilities.data",
                source: DEFAULT_AUTOMATION_PERMISSIONS,
            },
            AutomationSourceDocument {
                path: "flows/helpers.py",
                source: helper,
            },
        ];
        let roles = [
            AutomationRoleBinding {
                path: "plans/nightly-signoff.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "environment/frozen-runtime.toml",
                role: AutomationSourceRole::EnvironmentLock,
            },
            AutomationRoleBinding {
                path: "security/capabilities.data",
                role: AutomationSourceRole::PermissionManifest,
            },
        ];
        let (baseline, manifest) =
            compile_automation_documents("flows/nightly.py", &documents, &roles).unwrap();
        assert_eq!(manifest.entry_path, "flows/nightly.py");
        assert_eq!(manifest.run_plan_path, "plans/nightly-signoff.yaml");
        assert_eq!(
            manifest.environment_lock_path,
            "environment/frozen-runtime.toml"
        );
        assert_eq!(manifest.permissions_path, "security/capabilities.data");

        let changed_helper = "def corner_label(name):\n    return name.casefold()\n";
        let mut changed = documents;
        changed[4].source = changed_helper;
        let (candidate, _) =
            compile_automation_documents("flows/nightly.py", &changed, &roles).unwrap();
        assert_ne!(baseline.source_digest(), candidate.source_digest());
    }

    #[test]
    fn run_plan_selection_ignores_python_comments_and_string_literals() {
        assert!(same_project_path(
            "plans/\u{00c9}t\u{00e9}.yaml",
            "PLANS/\u{00e9}t\u{00c9}.YAML"
        ));
        let entry = r#"from rspice import Project
project = Project.open(".")
# project.run_plans.load("plans/comment-decoy.yaml")
description = ".run_plans.load('plans/string-decoy.yaml')"
documentation = """.run_plans.load("plans/docstring-decoy.yaml")"""
plan = project . run_plans . load (
    # The selected logical path remains ordinary project data.
    'plans/selected.yaml'
)
"#;
        let decoy_plan =
            DEFAULT_AUTOMATION_RUN_PLAN.replace("Lab characterization", "Decoy characterization");
        let documents = [
            AutomationSourceDocument {
                path: "flows/qualified.py",
                source: entry,
            },
            AutomationSourceDocument {
                path: "plans/selected.yaml",
                source: DEFAULT_AUTOMATION_RUN_PLAN,
            },
            AutomationSourceDocument {
                path: "plans/decoy.yaml",
                source: &decoy_plan,
            },
            AutomationSourceDocument {
                path: "environment/exact.snapshot",
                source: DEFAULT_ENVIRONMENT_LOCK,
            },
            AutomationSourceDocument {
                path: "policy/closed.rules",
                source: DEFAULT_AUTOMATION_PERMISSIONS,
            },
        ];
        let roles = [
            AutomationRoleBinding {
                path: "plans/selected.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "plans/decoy.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "environment/exact.snapshot",
                role: AutomationSourceRole::EnvironmentLock,
            },
            AutomationRoleBinding {
                path: "policy/closed.rules",
                role: AutomationSourceRole::PermissionManifest,
            },
        ];

        assert_eq!(
            static_literal_run_plan_references(entry),
            vec!["plans/selected.yaml"]
        );
        let (_, manifest) =
            compile_automation_documents("flows/qualified.py", &documents, &roles).unwrap();
        assert_eq!(manifest.run_plan_path, "plans/selected.yaml");
        assert_eq!(manifest.name, "Lab characterization");
    }

    #[test]
    fn general_python_and_explicit_capabilities_are_not_demo_hardcoded() {
        let general_python = format!(
            "{DEFAULT_AUTOMATION_PYTHON}\nfor label in [\"tt\", \"ff\"]:\n    print(label)\n"
        );
        let explicit_permissions = DEFAULT_AUTOMATION_PERMISSIONS
            .replace("network = \"deny\"", "network = \"allow\"")
            .replace("process_spawn = \"deny\"", "process_spawn = \"allow\"");
        let mut candidate = sources();
        candidate.python = &general_python;
        candidate.permissions = &explicit_permissions;
        let (_, manifest) = compile_automation_workspace(candidate).unwrap();
        assert_eq!(manifest.network, "allow");
        assert_eq!(manifest.process_spawn, "allow");
    }

    #[test]
    fn workspace_closure_defers_python_type_parameter_syntax_to_managed_cpython() {
        let python_312 = "type Pair[T] = tuple[T, T]\n\ndef first[T](value: Pair[T]) -> T:\n    return value[0]\n";
        let mut candidate = sources();
        candidate.python = python_312;
        compile_automation_workspace(candidate)
            .expect("workspace formation must defer current Python grammar to managed CPython");
    }

    #[test]
    fn malformed_source_is_deferred_to_managed_cpython_and_capabilities_fail_closed() {
        let malformed_python = "project = Project.open([\n";
        let invalid_permissions =
            DEFAULT_AUTOMATION_PERMISSIONS.replace("network = \"deny\"", "network = \"sometimes\"");
        let mut candidate = sources();
        candidate.python = malformed_python;
        candidate.permissions = &invalid_permissions;
        let diagnostics = compile_automation_workspace(candidate).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTPERM013")
        );
    }

    #[test]
    fn python_syntax_authority_belongs_to_the_exact_managed_runtime() {
        let malformed_python = "project = Project.open([\n";
        let mut candidate = sources();
        candidate.python = malformed_python;
        compile_automation_workspace(candidate)
            .expect("workspace closure formation must defer syntax to managed CPython");
    }

    #[test]
    fn version_three_lock_authenticates_a_target_closed_package_graph() {
        let target = "browser/wasm32/pyodide-314.0.2".to_owned();
        let artifacts = vec![
            AutomationLockedArtifact {
                package: "numpy".to_owned(),
                version: "2.3.1".to_owned(),
                filename: "numpy-2.3.1-cp314-cp314-pyemscripten_2026_0_wasm32.whl".to_owned(),
                sha256: format!("sha256:{}", "11".repeat(32)),
                targets: vec![target.clone()],
                dependencies: Vec::new(),
            },
            AutomationLockedArtifact {
                package: "pandas".to_owned(),
                version: "2.3.2".to_owned(),
                filename: "pandas-2.3.2-cp314-cp314-pyemscripten_2026_0_wasm32.whl".to_owned(),
                sha256: format!("sha256:{}", "22".repeat(32)),
                targets: vec![target],
                dependencies: vec!["numpy==2.3.1".to_owned()],
            },
        ];
        let digest = environment_lock_digest(
            ">=3.14.0,<3.15.0",
            ">=1.0.0,<2.0.0",
            Some("=314.0.2"),
            &artifacts,
        );
        let lock = format!(
            r#"format = "rspice-python-lock/v3"
python = ">=3.14.0,<3.15.0"
rspice-api = ">=1.0.0,<2.0.0"
browser-runtime = "=314.0.2"
environment_digest = "{digest}"

[[artifact]]
package = "numpy"
version = "2.3.1"
filename = "numpy-2.3.1-cp314-cp314-pyemscripten_2026_0_wasm32.whl"
sha256 = "sha256:{numpy_digest}"
targets = ["browser/wasm32/pyodide-314.0.2"]

[[artifact]]
package = "pandas"
version = "2.3.2"
filename = "pandas-2.3.2-cp314-cp314-pyemscripten_2026_0_wasm32.whl"
sha256 = "sha256:{pandas_digest}"
targets = ["browser/wasm32/pyodide-314.0.2"]
dependencies = ["numpy==2.3.1"]
"#,
            numpy_digest = "11".repeat(32),
            pandas_digest = "22".repeat(32),
        );
        let mut candidate = sources();
        candidate.environment_lock = &lock;
        let (_, manifest) = compile_automation_workspace(candidate).unwrap();
        assert_eq!(manifest.environment_digest, digest);
        assert_eq!(manifest.locked_artifacts, artifacts);
    }

    #[test]
    fn version_three_lock_rejects_digest_tampering_and_open_dependencies() {
        let lock = DEFAULT_ENVIRONMENT_LOCK.replace(
            "environment_digest = \"sha256:d445",
            "environment_digest = \"sha256:a445",
        );
        let mut candidate = sources();
        candidate.environment_lock = &lock;
        let diagnostics = compile_automation_workspace(candidate).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTLOCK027")
        );

        let artifact = AutomationLockedArtifact {
            package: "pandas".to_owned(),
            version: "2.3.2".to_owned(),
            filename: "pandas-2.3.2-cp314-cp314-pyemscripten_2026_0_wasm32.whl".to_owned(),
            sha256: format!("sha256:{}", "22".repeat(32)),
            targets: vec!["browser/wasm32/pyodide-314.0.2".to_owned()],
            dependencies: vec!["numpy==2.3.1".to_owned()],
        };
        let digest = environment_lock_digest(
            ">=3.14.0,<3.15.0",
            ">=1.0.0,<2.0.0",
            Some("=314.0.2"),
            std::slice::from_ref(&artifact),
        );
        let open_lock = format!(
            r#"format = "rspice-python-lock/v3"
python = ">=3.14.0,<3.15.0"
rspice-api = ">=1.0.0,<2.0.0"
browser-runtime = "=314.0.2"
environment_digest = "{digest}"

[[artifact]]
package = "pandas"
version = "2.3.2"
filename = "pandas-2.3.2-cp314-cp314-pyemscripten_2026_0_wasm32.whl"
sha256 = "sha256:{artifact_digest}"
targets = ["browser/wasm32/pyodide-314.0.2"]
dependencies = ["numpy==2.3.1"]
"#,
            artifact_digest = "22".repeat(32),
        );
        candidate.environment_lock = &open_lock;
        let diagnostics = compile_automation_workspace(candidate).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTLOCK037")
        );
    }

    #[test]
    fn unknown_duplicate_or_incomplete_schema_fields_fail_closed() {
        let invalid = format!("{DEFAULT_AUTOMATION_RUN_PLAN}name: duplicate\nremote: true\n")
            .replace("schema: rspice.run-plan/v1\n", "");
        let mut candidate = sources();
        candidate.run_plan = &invalid;
        let diagnostics = compile_automation_workspace(candidate).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTFILE002")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTFILE003")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "AUTFILE004")
        );
    }
}
