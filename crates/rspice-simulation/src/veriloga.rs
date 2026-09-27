//! Sealed Verilog-A/AMS artifacts, compilation and engine registration.

use crate::compilation::unified_runtime_compiler_options;
use sha2::{Digest as _, Sha256};

mod connections;
use connections::PreparedVerilogAConnectionLibrary;
#[cfg(test)]
mod tests;

/// Why a prepared Verilog-A runtime could not be built.
///
/// Every construction path in this module used to answer `String`, which put
/// one failure a caller can act on — a payload carrying a float JSON cannot
/// encode — in the same shape as every failure it cannot. The variants are the
/// kinds this module already distinguished in prose; the sentences they carry
/// are the ones it already wrote, unchanged, because the UI shows them and
/// tests pin the text.
///
/// [`Self::NonFinite`] is the reason the enum exists. It keeps
/// [`rspice_veriloga::json_float::NonFiniteFloatError`] typed all the way to
/// the caller, which can then tell "this artifact cannot be written as JSON"
/// from "this source is wrong" without reading a message.
#[derive(Debug, Clone, PartialEq)]
pub enum PreparedRuntimeError {
    /// A sealed payload carries a non-finite float no annotation encodes. The
    /// artifact is well formed; JSON is what cannot carry it.
    NonFinite {
        /// What was being sealed, for the message.
        description: &'static str,
        source: rspice_veriloga::json_float::NonFiniteFloatError,
    },
    /// The source identity offered is not the one the caller asked to seal.
    SourceIdentity(String),
    /// The source closure could not be assembled into a compilable bundle.
    SourceBundle(String),
    /// The compiler refused the source.
    Compile(String),
    /// A runtime failed the integrity check it is defined by.
    Integrity(String),
}

impl std::fmt::Display for PreparedRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite {
                description,
                source,
            } => write!(formatter, "Could not seal {description}: {source}"),
            Self::SourceIdentity(message)
            | Self::SourceBundle(message)
            | Self::Compile(message)
            | Self::Integrity(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for PreparedRuntimeError {}

/// Every caller that still wants a sentence gets exactly the sentence this
/// module used to return, so typing the error moved no message and no call
/// site.
impl From<PreparedRuntimeError> for String {
    fn from(error: PreparedRuntimeError) -> Self {
        error.to_string()
    }
}

/// Immutable, worker-transferable Verilog-A runtime bound to one exact sealed
/// source identity. Project sources and signed PDK sources use disjoint,
/// content-addressed virtual namespaces, so ambient files can never satisfy a
/// prepared directive accidentally.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntime {
    source_key: String,
    source_digest: rspice_app_types::product::ContentDigest,
    artifact_digest: rspice_app_types::product::ContentDigest,
    module_name: String,
    netlist_alias: String,
    model_json: String,
    canonical_ir_json: String,
}

impl PreparedVerilogARuntime {
    #[cfg(feature = "wasm-jit")]
    pub fn compile_wasm_jit_artifact(
        &self,
    ) -> Result<rspice_veriloga::wasm_jit::WasmJitModelArtifact, String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        let artifact = rspice_veriloga::wasm_jit::compile_model_value_module(&model, &canonical_ir)
            .map_err(|error| error.to_string())?;
        Ok(artifact)
    }

    /// Seal compiled artifacts under a caller-supplied source identity.
    /// The host must validate live project or catalog authority before calling.
    pub fn try_from_runtime_report(
        source_key: String,
        source_digest: rspice_app_types::product::ContentDigest,
        module_name: &str,
        report: &rspice_veriloga::RuntimeCompileReport,
        netlist_alias: impl Into<String>,
    ) -> Result<Self, PreparedRuntimeError> {
        let netlist_alias = netlist_alias.into();
        let model_json = seal_compiled_model(&report.model)?;
        let canonical_ir_json = seal_canonical_ir(&report.canonical_ir)?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            source_digest,
            module_name,
            &netlist_alias,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest,
            artifact_digest,
            module_name: module_name.to_owned(),
            netlist_alias,
            model_json,
            canonical_ir_json,
        };
        runtime
            .validate()
            .map_err(PreparedRuntimeError::Integrity)?;
        Ok(runtime)
    }

    pub fn try_from_virtual_compilation(
        source_key: String,
        source_digest: rspice_app_types::product::ContentDigest,
        netlist_alias: String,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, PreparedRuntimeError> {
        compilation.validate_integrity().map_err(|error| {
            PreparedRuntimeError::Integrity(format!(
                "Compiled Verilog-A bundle is invalid: {error}"
            ))
        })?;
        let model_json = seal_compiled_model(&compilation.runtime.model)?;
        let canonical_ir_json = seal_canonical_ir(&compilation.runtime.canonical_ir)?;
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
        runtime
            .validate()
            .map_err(PreparedRuntimeError::Integrity)?;
        Ok(runtime)
    }

    fn try_from_signed_pdk_compilation(
        package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
        archive_digest: rspice_app_types::product::ContentDigest,
        binding: &crate::pdk::SealedPdkVerilogABinding,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, PreparedRuntimeError> {
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
        if !valid_sealed_source_key(&self.source_key) {
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

    pub const fn source_digest(&self) -> rspice_app_types::product::ContentDigest {
        self.source_digest
    }

    pub const fn artifact_digest(&self) -> rspice_app_types::product::ContentDigest {
        self.artifact_digest
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }

    pub fn provenance_label(&self) -> String {
        self.binding().provenance_label()
    }

    fn binding(&self) -> PreparedVerilogASourceBinding<'_> {
        PreparedVerilogASourceBinding {
            source_key: &self.source_key,
            netlist_alias: &self.netlist_alias,
            artifact_digest: self.artifact_digest(),
            is_connection_library: false,
        }
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

/// Common binding inventory for executable models and connection libraries.
/// Device-only consumers use `PreparedVerilogARuntimeSet::device_runtimes`; deck and
/// provenance consumers must use `PreparedVerilogARuntimeSet::sources`.
pub struct PreparedVerilogASourceBinding<'a> {
    source_key: &'a str,
    netlist_alias: &'a str,
    artifact_digest: rspice_app_types::product::ContentDigest,
    is_connection_library: bool,
}

impl<'a> PreparedVerilogASourceBinding<'a> {
    pub fn source_key(&self) -> &'a str {
        self.source_key
    }
    pub fn netlist_alias(&self) -> &'a str {
        self.netlist_alias
    }
    pub fn artifact_digest(&self) -> rspice_app_types::product::ContentDigest {
        self.artifact_digest
    }
    pub fn provenance_label(&self) -> String {
        let authority = if self.source_key.starts_with("__rspice_pdk__/") {
            "signed-pdk-veriloga"
        } else if self.source_key.starts_with("__rspice_model_library__/") {
            "model-library-veriloga"
        } else {
            "project-veriloga"
        };
        let kind = if self.is_connection_library {
            "-connections"
        } else {
            ""
        };
        format!("{authority}{kind}:{}", self.source_key)
    }
}

/// Canonically ordered device and connection sources required by one deck.
/// Both kinds share a key/alias namespace and are installed atomically.
/// The connection inventory is required on the wire: older worker requests
/// must be rebuilt, never interpreted as having no connection libraries.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedVerilogARuntimeSet {
    runtimes: Vec<PreparedVerilogARuntime>,
    connections: Vec<PreparedVerilogAConnectionLibrary>,
}

impl PreparedVerilogARuntimeSet {
    pub fn try_new(runtimes: Vec<PreparedVerilogARuntime>) -> Result<Self, String> {
        Self::try_with_connections(runtimes, Vec::new())
    }

    fn try_with_connections(
        mut runtimes: Vec<PreparedVerilogARuntime>,
        mut connections: Vec<PreparedVerilogAConnectionLibrary>,
    ) -> Result<Self, String> {
        runtimes.sort_by_cached_key(|runtime| runtime.source_key.to_ascii_lowercase());
        connections.sort_by_cached_key(|library| library.binding().source_key.to_ascii_lowercase());
        let set = Self {
            runtimes,
            connections,
        };
        set.validate()?;
        Ok(set)
    }

    pub fn validate(&self) -> Result<(), String> {
        for runtime in &self.runtimes {
            runtime.validate()?;
        }
        for library in &self.connections {
            library.validate()?;
        }
        let mut keys = std::collections::HashSet::new();
        let mut aliases =
            std::collections::HashMap::<String, rspice_app_types::product::ContentDigest>::new();
        for source in self.sources() {
            if !keys.insert(source.source_key.to_ascii_lowercase()) {
                return Err(format!(
                    "Verilog-A runtime source key '{}' is duplicated",
                    source.source_key
                ));
            }
            let alias = source.netlist_alias.to_ascii_uppercase();
            if let Some(existing) = aliases.insert(alias, source.artifact_digest)
                && existing != source.artifact_digest
            {
                return Err(format!(
                    "Verilog-A netlist alias '{}' identifies different prepared artifacts",
                    source.netlist_alias
                ));
            }
        }
        if !self
            .runtimes
            .iter()
            .map(|runtime| runtime.source_key.to_ascii_lowercase())
            .is_sorted()
            || !self
                .connections
                .iter()
                .map(|library| library.binding().source_key.to_ascii_lowercase())
                .is_sorted()
        {
            return Err("Verilog-A runtime set is not in canonical order".to_owned());
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.runtimes.is_empty() && self.connections.is_empty()
    }

    pub fn len(&self) -> usize {
        self.runtimes.len() + self.connections.len()
    }

    /// Executable devices only, for consumers such as the browser JIT.
    pub fn device_runtimes(&self) -> impl ExactSizeIterator<Item = &PreparedVerilogARuntime> {
        self.runtimes.iter()
    }

    pub fn sources(&self) -> impl Iterator<Item = PreparedVerilogASourceBinding<'_>> {
        self.runtimes
            .iter()
            .map(PreparedVerilogARuntime::binding)
            .chain(
                self.connections
                    .iter()
                    .map(PreparedVerilogAConnectionLibrary::binding),
            )
    }

    pub fn install(&self) -> Result<(), String> {
        self.validate()?;
        let registrations = self
            .runtimes
            .iter()
            .map(|runtime| {
                runtime
                    .registration()
                    .map(rspice_core::ProjectVerilogASourceRegistration::Runtime)
            })
            .chain(
                self.connections
                    .iter()
                    .map(PreparedVerilogAConnectionLibrary::registration),
            )
            .collect::<Result<Vec<_>, _>>()?;
        rspice_core::register_project_veriloga_sources_for_session(registrations)
    }

    pub fn try_merge(self, additional: Self) -> Result<Self, String> {
        Self::try_with_connections(
            self.runtimes
                .into_iter()
                .chain(additional.runtimes)
                .collect(),
            self.connections
                .into_iter()
                .chain(additional.connections)
                .collect(),
        )
    }
}

pub fn compile_signed_pdk_source_runtime(
    package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
    archive_digest: rspice_app_types::product::ContentDigest,
    artifacts: &[crate::pdk::SealedPdkVerilogAArtifact],
    binding: &crate::pdk::SealedPdkVerilogABinding,
) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
    if artifacts.is_empty() {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "Signed PDK Verilog-A source '{}' has no authenticated artifacts",
            binding.source_id
        )));
    }
    let mut root_seen = false;
    let mut files = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let actual = rspice_app_types::product::ContentDigest::from_bytes(
            Sha256::digest(artifact.source.as_bytes()).into(),
        );
        if actual != artifact.digest {
            return Err(PreparedRuntimeError::SourceIdentity(format!(
                "Signed PDK Verilog-A artifact '{}' no longer matches digest {}",
                artifact.path, artifact.digest
            )));
        }
        if artifact
            .path
            .eq_ignore_ascii_case(&binding.root_artifact_path)
        {
            if artifact.path != binding.root_artifact_path
                || artifact.digest != binding.root_artifact_digest
            {
                return Err(PreparedRuntimeError::SourceIdentity(format!(
                    "Signed PDK Verilog-A root '{}' no longer matches its exact manifest identity",
                    binding.root_artifact_path
                )));
            }
            root_seen = true;
        }
        files.push(rspice_veriloga::VirtualSourceFile::new(
            &artifact.path,
            &artifact.source,
        ));
    }
    if !root_seen {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "Signed PDK Verilog-A root '{}' is absent from the authenticated closure",
            binding.root_artifact_path
        )));
    }
    let bundle = rspice_veriloga::VirtualSourceBundle::new(&binding.root_artifact_path, files)
        .map_err(|error| {
            PreparedRuntimeError::SourceBundle(format!(
                "Signed PDK Verilog-A bundle '{}' is invalid: {error}",
                binding.source_id
            ))
        })?;
    let compilation = rspice_veriloga::VerilogACompiler::new(unified_runtime_compiler_options())
        .compile_virtual_runtime(&bundle, &binding.module_name, pdk_virtual_compile_limits())
        .map_err(|error| {
            PreparedRuntimeError::Compile(format!(
                "Could not compile signed PDK Verilog-A source '{}' module '{}': {error}",
                binding.source_id, binding.module_name
            ))
        })?;
    PreparedVerilogARuntime::try_from_signed_pdk_compilation(
        package,
        archive_digest,
        binding,
        &compilation,
    )
}

pub fn compile_model_library_source_runtimes(
    authority: &crate::model_sources::SealedModelLibraryVerilogAAuthority,
    limits: rspice_veriloga::VirtualCompileLimits,
) -> Result<PreparedVerilogARuntimeSet, PreparedRuntimeError> {
    let mut logical_sources = Vec::with_capacity(authority.sources().len());
    let mut logical_paths = std::collections::HashMap::<String, String>::new();
    for (path, source) in authority.sources() {
        let logical = model_library_virtual_path(path)?;
        let folded = logical.to_ascii_lowercase();
        if let Some(existing) = logical_paths.insert(folded, logical.clone()) {
            return Err(PreparedRuntimeError::SourceIdentity(format!(
                "Model-library Verilog-A paths '{}' and '{}' collide in the portable compiler namespace",
                existing,
                path.display()
            )));
        }
        logical_sources.push(rspice_veriloga::VirtualSourceFile::new(logical, source));
    }

    // Every retained model includes its canonical digital plan and is installed
    // through the unified engine. Mixed reports are safe on this host path.
    let compiler = rspice_veriloga::VerilogACompiler::new(unified_runtime_compiler_options());
    let mut runtimes = Vec::new();
    let mut connections = Vec::new();
    let mut roots_by_path = std::collections::BTreeMap::<_, Vec<_>>::new();
    for root in authority.roots() {
        let root_path = model_library_virtual_path(&root.path)?;
        roots_by_path.entry(root_path).or_default().push(root);
    }
    for (root_path, roots) in roots_by_path {
        let bundle =
            rspice_veriloga::VirtualSourceBundle::new(&root_path, logical_sources.iter().cloned())
                .map_err(|error| {
                    PreparedRuntimeError::SourceBundle(format!(
                        "Sealed model-library Verilog-A root '{}' is invalid: {error}",
                        roots[0].path.display()
                    ))
                })?;
        let prepared = compiler
            .prepare_virtual_runtime_source(&bundle, limits)
            .map_err(|error| {
                PreparedRuntimeError::Compile(format!(
                    "Could not prepare sealed model-library Verilog-A root '{}': {error}",
                    roots[0].path.display()
                ))
            })?;
        let module_names = prepared.module_names().collect::<Vec<_>>();
        let root_identity = rspice_app_types::product::ContentDigest::from_bytes(
            Sha256::digest(root_path.as_bytes()).into(),
        );
        for root in roots {
            if module_names.is_empty() && root.selected_module.is_none() {
                let artifact = prepared.connection_artifact().ok_or_else(|| {
                    PreparedRuntimeError::SourceIdentity(format!(
                        "Model-library Verilog-A source '{}' declares neither device modules nor connection rules",
                        root.path.display()
                    ))
                })?;
                let source_key = format!(
                    "__rspice_model_library__/{}/{}/connections.vams",
                    authority.closure_digest(),
                    root_identity
                );
                let alias = root
                    .netlist_alias
                    .clone()
                    .unwrap_or_else(|| format!("__rspice_connections_{root_identity}"));
                connections.push(
                    PreparedVerilogAConnectionLibrary::try_new(
                        source_key,
                        authority.closure_digest(),
                        alias,
                        artifact,
                    )
                    .map_err(PreparedRuntimeError::Integrity)?,
                );
                continue;
            }
            let selected = if let Some(module) = root.selected_module.as_deref() {
                if !module_names.contains(&module) {
                    return Err(PreparedRuntimeError::SourceIdentity(format!(
                        "Model-library Verilog-A source '{}' does not declare module '{}'",
                        root.path.display(),
                        module
                    )));
                }
                vec![(
                    module.to_owned(),
                    root.netlist_alias.as_deref().unwrap_or(module).to_owned(),
                )]
            } else if let Some(alias) = root.netlist_alias.as_deref() {
                let [module] = module_names.as_slice() else {
                    return Err(PreparedRuntimeError::SourceIdentity(format!(
                        "Model-library .veriloga source '{}' declares {} modules, so alias '{}' is ambiguous",
                        root.path.display(),
                        module_names.len(),
                        alias
                    )));
                };
                vec![((*module).to_owned(), alias.to_owned())]
            } else {
                module_names
                    .iter()
                    .map(|module| ((*module).to_owned(), (*module).to_owned()))
                    .collect::<Vec<_>>()
            };
            for (module_name, netlist_alias) in selected {
                if !valid_veriloga_netlist_identifier(&module_name)
                    || !valid_veriloga_netlist_identifier(&netlist_alias)
                {
                    return Err(PreparedRuntimeError::SourceIdentity(format!(
                        "Model-library Verilog-A module '{}' or alias '{}' is not a portable SPICE model identifier",
                        module_name, netlist_alias
                    )));
                }
                let compilation = prepared
                    .compile_runtime(&module_name)
                    .map_err(|error| {
                        PreparedRuntimeError::Compile(format!(
                            "Could not compile module '{}' from sealed model-library Verilog-A root '{}': {error}",
                            module_name,
                            root.path.display()
                        ))
                    })?;
                let source_key = format!(
                    "__rspice_model_library__/{}/{}/{}.va",
                    authority.closure_digest(),
                    root_identity,
                    module_name
                );
                runtimes.push(PreparedVerilogARuntime::try_from_virtual_compilation(
                    source_key,
                    authority.closure_digest(),
                    netlist_alias,
                    &compilation,
                )?);
            }
        }
    }
    PreparedVerilogARuntimeSet::try_with_connections(runtimes, connections)
        .map_err(PreparedRuntimeError::Integrity)
}

fn model_library_virtual_path(path: &std::path::Path) -> Result<String, PreparedRuntimeError> {
    let portable = path.to_string_lossy().replace('\\', "/");
    if portable.is_empty() || portable.contains('\0') {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "Model-library Verilog-A path '{}' is not a valid portable identity",
            path.display()
        )));
    }
    let logical = if let Some(rest) = portable.strip_prefix("//") {
        format!("unc/{rest}")
    } else if let Some(rest) = portable.strip_prefix('/') {
        format!("posix/{rest}")
    } else if portable.as_bytes().get(1) == Some(&b':') {
        let drive = portable[..1].to_ascii_lowercase();
        let rest = portable[2..].trim_start_matches('/');
        format!("windows/{drive}/{rest}")
    } else {
        format!("relative/{portable}")
    };
    if logical
        .split('/')
        .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "Model-library Verilog-A path '{}' cannot be represented in the sealed compiler namespace",
            path.display()
        )));
    }
    Ok(logical)
}

/// Seal the compiled model as the JSON string the runtime carries.
fn seal_compiled_model(
    model: &rspice_veriloga::CompiledModel,
) -> Result<String, PreparedRuntimeError> {
    seal_payload_json("model", "compiled Verilog-A model", model)
}

/// Seal the canonical IR the same way. It is a separate string with a separate
/// parse, and the browser JIT lowers this one.
fn seal_canonical_ir(
    canonical_ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<String, PreparedRuntimeError> {
    seal_payload_json("canonical_ir", "canonical Verilog-A IR", canonical_ir)
}

/// Write one payload as JSON, refusing first anything JSON cannot carry.
///
/// The scan runs before the write rather than after, because after the write
/// there is nothing left to see: `serde_json` spells a non-finite float
/// `null`, a bare `f64` then refuses to decode, and an `Option<f64>` decodes
/// it as `None` — a bound silently deleted, with the artifact digest over the
/// JSON text agreeing that nothing is wrong. Every field that can legitimately
/// hold a non-finite float encodes it as a string through
/// `rspice_veriloga::json_float`, so a path this names is a field that
/// acquired one without being annotated, and the seal refuses rather than
/// shipping a model the browser worker would build different physics from.
///
/// The refusal keeps its cause typed: `NonFiniteFloatError` reaches the caller
/// as [`PreparedRuntimeError::NonFinite`] rather than as a sentence, so a
/// caller can tell an unencodable artifact from a bad source without parsing
/// the message it renders.
fn seal_payload_json<T>(
    path_root: &str,
    description: &'static str,
    value: &T,
) -> Result<String, PreparedRuntimeError>
where
    T: serde::Serialize,
{
    rspice_veriloga::json_float::refuse_non_finite_floats(path_root, value).map_err(|source| {
        PreparedRuntimeError::NonFinite {
            description,
            source,
        }
    })?;
    serde_json::to_string(value).map_err(|error| {
        PreparedRuntimeError::Integrity(format!("Could not serialize {description}: {error}"))
    })
}

fn runtime_artifact_digest(
    source_key: &str,
    source_digest: rspice_app_types::product::ContentDigest,
    module_name: &str,
    netlist_alias: &str,
    model_json: &str,
    canonical_ir_json: &str,
) -> rspice_app_types::product::ContentDigest {
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
    rspice_app_types::product::ContentDigest::from_bytes(hasher.finalize().into())
}

fn valid_sealed_source_key(key: &str) -> bool {
    (key.starts_with("__rspice_project__/")
        || key.starts_with("__rspice_pdk__/")
        || key.starts_with("__rspice_model_library__/"))
        && !key.contains('\\')
        && !key.chars().any(char::is_control)
        && !key
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
}

fn valid_veriloga_netlist_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn pdk_virtual_compile_limits() -> rspice_veriloga::VirtualCompileLimits {
    rspice_veriloga::VirtualCompileLimits {
        max_files: rspice_model_library::pdk::contracts::MAX_PDK_ARTIFACTS,
        max_path_bytes: 1_024,
        max_file_bytes: rspice_model_library::pdk::contracts::MAX_PDK_ARTIFACT_BYTES,
        max_total_source_bytes: rspice_model_library::pdk::contracts::MAX_PDK_TOTAL_ARTIFACT_BYTES,
        max_include_depth: 64,
        max_expanded_bytes: rspice_model_library::pdk::contracts::MAX_PDK_TOTAL_ARTIFACT_BYTES
            .saturating_mul(2),
        max_module_name_bytes: 128,
    }
}
