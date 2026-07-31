//! Sealed Verilog-A runtimes prepared for an executable deck.
//!
//! A prepared runtime is engine state: an immutable, worker-transferable
//! model bound to one exact project-source or signed-PDK identity. Project
//! compilation is triggered from the Code & Automation workspace; PDK sources
//! are compiled only from the authenticated package closure.
//!
//! Construction stays with the type, because the validation *is* the type's
//! invariant. What stays in the workspace is only the adapter that unpacks an
//! editor compile receipt into the three facts this module checks — the
//! source token, the selected module, and the compile report.

use sha2::{Digest as _, Sha256};

/// Exact project-owned Verilog-A closure captured when compilation starts.
///
/// Unlike automation's single-document token, this identity includes the
/// stable bundle owner and the digest of every file in its sealed dependency
/// closure. A result can therefore never cross-publish between two cell views
/// that happen to contain identical root text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerilogASourceOperationToken {
    pub project_id: crate::product::ProjectId,
    pub bundle_id: crate::state::ProjectSourceId,
    pub revision: u64,
    pub closure_digest: crate::product::ContentDigest,
    /// Exact explicit module selection requested by a cell-view contract.
    /// `None` preserves the Code Workspace's compiler-selected module mode.
    pub requested_module_digest: Option<crate::product::ContentDigest>,
}

/// Immutable, worker-transferable Verilog-A runtime bound to one exact sealed
/// source identity. Project sources and signed PDK sources use disjoint,
/// content-addressed virtual namespaces, so ambient files can never satisfy a
/// prepared directive accidentally.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntime {
    source_key: String,
    source_digest: crate::product::ContentDigest,
    artifact_digest: crate::product::ContentDigest,
    module_name: String,
    netlist_alias: String,
    model_json: String,
    canonical_ir_json: String,
}

impl PreparedVerilogARuntime {
    /// Bind a compiled report to one exact project source identity.
    ///
    /// This takes the three facts it validates rather than the editor's
    /// compile receipt: the receipt also carries editor diagnostics, and
    /// depending on it would drag the code editor down into the engine layer.
    /// The workspace keeps a thin adapter that unpacks a receipt into these.
    pub fn try_new(
        project_id: crate::product::ProjectId,
        bundle: &crate::state::ProjectSourceBundle,
        token: &VerilogASourceOperationToken,
        module_name: &str,
        report: &rspice_veriloga::RuntimeCompileReport,
        netlist_alias: impl Into<String>,
    ) -> Result<Self, String> {
        if token.project_id != project_id
            || token.bundle_id != bundle.id()
            || token.revision != bundle.revision().get()
            || token.closure_digest != bundle.closure_digest()
            || token
                .requested_module_digest
                .is_some_and(|expected| expected != veriloga_selected_module_digest(module_name))
        {
            return Err(
                "The retained Verilog-A runtime does not identify the exact current project source"
                    .to_owned(),
            );
        }
        let source_key =
            crate::state::project_veriloga_bundle_source_key(project_id, bundle, module_name)
                .map_err(|error| error.to_string())?;
        let netlist_alias = netlist_alias.into();
        let model_json = serde_json::to_string(&report.model)
            .map_err(|error| format!("Could not serialize compiled Verilog-A model: {error}"))?;
        let canonical_ir_json = serde_json::to_string(&report.canonical_ir)
            .map_err(|error| format!("Could not serialize canonical Verilog-A IR: {error}"))?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            bundle.closure_digest(),
            module_name,
            &netlist_alias,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest: bundle.closure_digest(),
            artifact_digest,
            module_name: module_name.to_owned(),
            netlist_alias,
            model_json,
            canonical_ir_json,
        };
        runtime.validate()?;
        Ok(runtime)
    }

    fn try_from_virtual_compilation(
        source_key: String,
        source_digest: crate::product::ContentDigest,
        netlist_alias: String,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, String> {
        compilation
            .validate_integrity()
            .map_err(|error| format!("Compiled Verilog-A bundle is invalid: {error}"))?;
        let model_json = serde_json::to_string(&compilation.runtime.model)
            .map_err(|error| format!("Could not serialize compiled Verilog-A model: {error}"))?;
        let canonical_ir_json = serde_json::to_string(&compilation.runtime.canonical_ir)
            .map_err(|error| format!("Could not serialize canonical Verilog-A IR: {error}"))?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            source_digest,
            &compilation.selected_module,
            &netlist_alias,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest,
            artifact_digest,
            module_name: compilation.selected_module.clone(),
            netlist_alias,
            model_json,
            canonical_ir_json,
        };
        runtime.validate()?;
        Ok(runtime)
    }

    fn try_from_signed_pdk_compilation(
        package: &crate::state::pdk_config::PdkTechnologyBinding,
        archive_digest: crate::product::ContentDigest,
        binding: &crate::state::pdk_config::SealedPdkVerilogABinding,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, String> {
        let source_key = format!(
            "__rspice_pdk__/{}/{}/{}/{}.va",
            package.manifest_digest,
            archive_digest,
            binding.source_id,
            binding.root_artifact_digest
        );
        Self::try_from_virtual_compilation(
            source_key,
            archive_digest,
            binding.netlist_alias.clone(),
            compilation,
        )
    }

    pub fn install(&self) -> Result<(), String> {
        rspice_core::register_project_veriloga_runtimes_for_session([self.registration()?])
    }

    fn registration(&self) -> Result<rspice_core::ProjectVerilogARuntimeRegistration, String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        Ok(rspice_core::ProjectVerilogARuntimeRegistration {
            source_key: self.source_key.clone().into(),
            aliases: vec![self.netlist_alias.clone()],
            model,
            canonical_ir,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(self.source_key.starts_with("__rspice_project__/")
            || self.source_key.starts_with("__rspice_pdk__/"))
            || self.source_key.contains('\\')
            || self.source_key.chars().any(char::is_control)
            || self
                .source_key
                .split('/')
                .any(|component| component.is_empty() || matches!(component, "." | ".."))
        {
            return Err("Verilog-A runtime has an invalid sealed virtual source key".to_owned());
        }
        if self.module_name.trim().is_empty() || self.module_name.chars().any(char::is_control) {
            return Err("Verilog-A runtime has an invalid module identity".to_owned());
        }
        if !valid_veriloga_netlist_identifier(&self.netlist_alias) {
            return Err("Verilog-A runtime has an invalid netlist alias".to_owned());
        }
        let expected = runtime_artifact_digest(
            &self.source_key,
            self.source_digest,
            &self.module_name,
            &self.netlist_alias,
            &self.model_json,
            &self.canonical_ir_json,
        );
        if expected != self.artifact_digest {
            return Err("Verilog-A runtime artifact digest does not match its payload".to_owned());
        }
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        if model.name.as_str() != self.module_name {
            return Err(format!(
                "Verilog-A runtime module '{}' does not match compiled model '{}'",
                self.module_name, model.name
            ));
        }
        if canonical_ir.hir.module_name.as_str() != self.module_name {
            return Err(format!(
                "Verilog-A canonical IR module '{}' does not match runtime module '{}'",
                canonical_ir.hir.module_name, self.module_name
            ));
        }
        Ok(())
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub const fn source_digest(&self) -> crate::product::ContentDigest {
        self.source_digest
    }

    pub const fn artifact_digest(&self) -> crate::product::ContentDigest {
        self.artifact_digest
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }

    pub(crate) fn provenance_label(&self) -> String {
        let authority = if self.source_key.starts_with("__rspice_pdk__/") {
            "signed-pdk-veriloga"
        } else {
            "project-veriloga"
        };
        format!("{authority}:{}", self.source_key)
    }

    pub fn terminal_names(&self) -> Result<Vec<String>, String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        Ok(model
            .terminal_names
            .iter()
            .map(ToString::to_string)
            .collect())
    }
}

pub(crate) fn veriloga_selected_module_digest(module_name: &str) -> crate::product::ContentDigest {
    let mut hasher = Sha256::new();
    let domain = b"rspice.veriloga-selected-module/v1";
    hasher.update((domain.len() as u64).to_be_bytes());
    hasher.update(domain);
    hasher.update((module_name.len() as u64).to_be_bytes());
    hasher.update(module_name.as_bytes());
    crate::product::ContentDigest::from_bytes(hasher.finalize().into())
}

/// Canonically ordered set of every sealed Verilog-A runtime required by one
/// immutable executable deck. The set rejects case-folded key/alias
/// collisions before worker transfer so model selection cannot depend on
/// discovery order.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntimeSet {
    runtimes: Vec<PreparedVerilogARuntime>,
}

impl PreparedVerilogARuntimeSet {
    pub fn try_new(mut runtimes: Vec<PreparedVerilogARuntime>) -> Result<Self, String> {
        for runtime in &runtimes {
            runtime.validate()?;
        }
        runtimes.sort_by(|left, right| {
            left.source_key
                .to_ascii_lowercase()
                .cmp(&right.source_key.to_ascii_lowercase())
                .then_with(|| {
                    left.netlist_alias
                        .to_ascii_lowercase()
                        .cmp(&right.netlist_alias.to_ascii_lowercase())
                })
        });
        for pair in runtimes.windows(2) {
            if pair[0].source_key.eq_ignore_ascii_case(&pair[1].source_key) {
                return Err(format!(
                    "Verilog-A runtime source key '{}' is duplicated",
                    pair[1].source_key
                ));
            }
        }
        let mut aliases = std::collections::HashMap::<String, crate::product::ContentDigest>::new();
        for runtime in &runtimes {
            let alias = runtime.netlist_alias.to_ascii_uppercase();
            if let Some(existing) = aliases.insert(alias, runtime.artifact_digest)
                && existing != runtime.artifact_digest
            {
                return Err(format!(
                    "Verilog-A netlist alias '{}' identifies different prepared artifacts",
                    runtime.netlist_alias
                ));
            }
        }
        Ok(Self { runtimes })
    }

    pub fn validate(&self) -> Result<(), String> {
        Self::try_new(self.runtimes.clone()).and_then(|canonical| {
            if canonical == *self {
                Ok(())
            } else {
                Err("Verilog-A runtime set is not in canonical order".to_owned())
            }
        })
    }

    pub fn is_empty(&self) -> bool {
        self.runtimes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.runtimes.len()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &PreparedVerilogARuntime> {
        self.runtimes.iter()
    }

    pub fn install(&self) -> Result<(), String> {
        self.validate()?;
        let registrations = self
            .runtimes
            .iter()
            .map(PreparedVerilogARuntime::registration)
            .collect::<Result<Vec<_>, _>>()?;
        rspice_core::register_project_veriloga_runtimes_for_session(registrations)
    }

    pub(crate) fn try_extend(
        self,
        additional: impl IntoIterator<Item = PreparedVerilogARuntime>,
    ) -> Result<Self, String> {
        Self::try_new(self.runtimes.into_iter().chain(additional).collect())
    }
}

pub(crate) fn compile_project_source_bundle_runtime(
    project_id: crate::product::ProjectId,
    bundle: &crate::state::ProjectSourceBundle,
    module_name: &str,
) -> Result<PreparedVerilogARuntime, String> {
    if bundle.language() != crate::state::ProjectSourceLanguage::VerilogA {
        return Err(format!(
            "Project source bundle {} is {}, not Verilog-A",
            bundle.id(),
            bundle.language()
        ));
    }
    let source_key =
        crate::state::project_veriloga_bundle_source_key(project_id, bundle, module_name)
            .map_err(|error| error.to_string())?;
    let netlist_alias = crate::state::project_veriloga_bundle_alias(bundle, module_name)
        .map_err(|error| error.to_string())?;
    let files =
        std::iter::once(rspice_veriloga::VirtualSourceFile::new(
            bundle.root().logical_path(),
            bundle.root().content(),
        ))
        .chain(bundle.files().iter().map(|file| {
            rspice_veriloga::VirtualSourceFile::new(file.logical_path(), file.content())
        }));
    let virtual_bundle =
        rspice_veriloga::VirtualSourceBundle::new(bundle.root().logical_path(), files)
            .map_err(|error| format!("Project Verilog-A bundle is invalid: {error}"))?;
    let limits = project_virtual_compile_limits();
    let compilation = rspice_veriloga::VerilogACompiler::default()
        .compile_virtual_runtime(&virtual_bundle, module_name, limits)
        .map_err(|error| {
            format!(
                "Could not compile Verilog-A module '{module_name}' from project bundle {}: {error}",
                bundle.id()
            )
        })?;
    PreparedVerilogARuntime::try_from_virtual_compilation(
        source_key,
        bundle.closure_digest(),
        netlist_alias,
        &compilation,
    )
}

pub(crate) fn compile_signed_pdk_source_runtime(
    package: &crate::state::pdk_config::PdkTechnologyBinding,
    archive_digest: crate::product::ContentDigest,
    artifacts: &[crate::state::pdk_config::SealedPdkVerilogAArtifact],
    binding: &crate::state::pdk_config::SealedPdkVerilogABinding,
) -> Result<PreparedVerilogARuntime, String> {
    if artifacts.is_empty() {
        return Err(format!(
            "Signed PDK Verilog-A source '{}' has no authenticated artifacts",
            binding.source_id
        ));
    }
    let mut root_seen = false;
    let mut files = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let actual = crate::product::ContentDigest::from_bytes(
            Sha256::digest(artifact.source.as_bytes()).into(),
        );
        if actual != artifact.digest {
            return Err(format!(
                "Signed PDK Verilog-A artifact '{}' no longer matches digest {}",
                artifact.path, artifact.digest
            ));
        }
        if artifact
            .path
            .eq_ignore_ascii_case(&binding.root_artifact_path)
        {
            if artifact.path != binding.root_artifact_path
                || artifact.digest != binding.root_artifact_digest
            {
                return Err(format!(
                    "Signed PDK Verilog-A root '{}' no longer matches its exact manifest identity",
                    binding.root_artifact_path
                ));
            }
            root_seen = true;
        }
        files.push(rspice_veriloga::VirtualSourceFile::new(
            &artifact.path,
            &artifact.source,
        ));
    }
    if !root_seen {
        return Err(format!(
            "Signed PDK Verilog-A root '{}' is absent from the authenticated closure",
            binding.root_artifact_path
        ));
    }
    let bundle = rspice_veriloga::VirtualSourceBundle::new(&binding.root_artifact_path, files)
        .map_err(|error| {
            format!(
                "Signed PDK Verilog-A bundle '{}' is invalid: {error}",
                binding.source_id
            )
        })?;
    let compilation = rspice_veriloga::VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, &binding.module_name, pdk_virtual_compile_limits())
        .map_err(|error| {
            format!(
                "Could not compile signed PDK Verilog-A source '{}' module '{}': {error}",
                binding.source_id, binding.module_name
            )
        })?;
    PreparedVerilogARuntime::try_from_signed_pdk_compilation(
        package,
        archive_digest,
        binding,
        &compilation,
    )
}

/// Insert one sealed Verilog-A directive before the terminal `.end` card.
/// The exact same helper is used by the retained generated artifact and the
/// immutable prepared-run source, preventing display/execution drift.
pub fn project_veriloga_directive(source_key: &str, module_name: &str) -> String {
    format!(".veriloga \"{source_key}\" {module_name}")
}

pub fn append_project_veriloga_directive(source: &mut String, source_key: &str, module_name: &str) {
    let directive = project_veriloga_directive(source_key, module_name);
    if source
        .lines()
        .any(|line| line.trim().eq_ignore_ascii_case(&directive))
    {
        return;
    }
    let end = source
        .lines()
        .enumerate()
        .find_map(|(index, line)| line.trim().eq_ignore_ascii_case(".end").then_some(index));
    let retained_trailing_newline = source.ends_with('\n');
    let mut lines = source.lines().map(str::to_owned).collect::<Vec<_>>();
    lines.insert(end.unwrap_or(lines.len()), directive);
    *source = lines.join("\n");
    if retained_trailing_newline || !source.is_empty() {
        source.push('\n');
    }
}

fn runtime_artifact_digest(
    source_key: &str,
    source_digest: crate::product::ContentDigest,
    module_name: &str,
    netlist_alias: &str,
    model_json: &str,
    canonical_ir_json: &str,
) -> crate::product::ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.project-veriloga-runtime/v2\0");
    for bytes in [
        source_key.as_bytes(),
        source_digest.as_bytes(),
        module_name.as_bytes(),
        netlist_alias.as_bytes(),
        model_json.as_bytes(),
        canonical_ir_json.as_bytes(),
    ] {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    crate::product::ContentDigest::from_bytes(hasher.finalize().into())
}

fn valid_veriloga_netlist_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

pub(crate) fn project_virtual_compile_limits() -> rspice_veriloga::VirtualCompileLimits {
    rspice_veriloga::VirtualCompileLimits {
        max_files: crate::state::MAX_PROJECT_SOURCE_FILES,
        max_path_bytes: crate::state::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
        max_file_bytes: crate::state::MAX_PROJECT_CODE_SOURCE_BYTES,
        max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
        // Macro expansion is intentionally bounded separately from retained
        // source bytes. Keep this identical to the prepared-runtime path so a
        // bundle accepted by the editor cannot be rejected only at execution.
        max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
        ..rspice_veriloga::VirtualCompileLimits::default()
    }
}

fn pdk_virtual_compile_limits() -> rspice_veriloga::VirtualCompileLimits {
    rspice_veriloga::VirtualCompileLimits {
        max_files: crate::state::pdk_config::MAX_PDK_ARTIFACTS,
        max_path_bytes: 1_024,
        max_file_bytes: crate::state::pdk_config::MAX_PDK_ARTIFACT_BYTES,
        max_total_source_bytes: crate::state::pdk_config::MAX_PDK_TOTAL_ARTIFACT_BYTES,
        max_include_depth: 64,
        max_expanded_bytes: crate::state::pdk_config::MAX_PDK_TOTAL_ARTIFACT_BYTES
            .saturating_mul(2),
        max_module_name_bytes: 128,
    }
}
