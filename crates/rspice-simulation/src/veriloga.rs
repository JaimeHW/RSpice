//! Sealed Verilog-A/AMS artifacts, compilation and engine registration.

use crate::compilation::unified_runtime_compiler_options;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
use sha2::{Digest as _, Sha256};

mod connections;
pub use connections::PreparedVerilogAConnectionLibrary;
#[cfg(test)]
pub(crate) mod tests;

/// Why a prepared Verilog-A runtime could not be built. The variants are the
/// kinds this module distinguishes in prose; the sentences they carry are the
/// ones the UI shows.
#[derive(Debug, Clone, PartialEq)]
pub enum PreparedRuntimeError {
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
/// source identity. Project sources and PDK sources use disjoint,
/// content-addressed virtual namespaces, so ambient files can never satisfy a
/// prepared directive accidentally.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntime {
    source_key: String,
    source_digest: rspice_vocabulary::product::ContentDigest,
    artifact_digest: rspice_vocabulary::product::ContentDigest,
    module_name: String,
    netlist_alias: String,
    /// The compiled model and its canonical IR, sealed as one compact record
    /// ([`rspice_core::seal_veriloga_artifact`]): the same bytes on every
    /// architecture, so a runtime sealed natively has the digest the browser
    /// worker would seal it under.
    artifact: SealedArtifactBytes,
    /// The fingerprint the engine compares installed artifacts by
    /// ([`rspice_core::SealedVerilogAArtifact::fingerprint`]), taken once when
    /// the artifact is sealed and covered by the digest. An install takes it
    /// rather than computing it, and holds an artifact the engine already has
    /// under it without reading the payload.
    artifact_fingerprint: rspice_vocabulary::product::ContentDigest,
    /// The cell table the artifact was compiled against, when its design
    /// instantiates SPICE cells as leaves: part of its identity, and of the
    /// key the engine installs it under, so it builds only for a deck that
    /// offers the same cells.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    external_cells: Option<Vec<rspice_veriloga::ExternalCell>>,
    /// The value of each min:typ:max expression the artifact was compiled
    /// keeping, the deck's `.options hdl_delay_select`: part of its identity
    /// beside its source, and of the key the engine installs it under, so it
    /// builds only for a deck keeping the same values. A typical artifact
    /// carries none and keeps the bytes and the digest it always had.
    #[serde(default, skip_serializing_if = "is_typical")]
    delay_select: DigitalDelaySelect,
}

fn is_typical(delay_select: &DigitalDelaySelect) -> bool {
    *delay_select == DigitalDelaySelect::Typ
}

/// A sealed artifact's bytes, shared by every clone of the runtime that holds
/// them: a request, its snapshot and its dispatch all carry the same set.
///
/// They travel as one base64 string, the form both of a runtime's transports
/// carry compactly: the JSON wire a host writes to the worker, and the object
/// a page hands it (`serde_wasm_bindgen`), which would otherwise spell each
/// byte as a number.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SealedArtifactBytes(std::sync::Arc<[u8]>);

impl SealedArtifactBytes {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl serde::Serialize for SealedArtifactBytes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use base64::Engine as _;
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.0))
    }
}

impl<'de> serde::Deserialize<'de> for SealedArtifactBytes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Base64;
        impl serde::de::Visitor<'_> for Base64 {
            type Value = SealedArtifactBytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a sealed Verilog-A artifact as base64")
            }

            fn visit_str<E: serde::de::Error>(self, text: &str) -> Result<Self::Value, E> {
                use base64::Engine as _;
                base64::engine::general_purpose::STANDARD
                    .decode(text)
                    .map(|bytes| SealedArtifactBytes(bytes.into()))
                    .map_err(E::custom)
            }
        }
        deserializer.deserialize_str(Base64)
    }
}

impl PreparedVerilogARuntime {
    /// The runtime's browser JIT model, compiled once per worker session from
    /// the artifact `installed` holds for it: the set's install checked and
    /// parsed the payload, and the devices build on that same artifact.
    ///
    /// A run asks for every runtime's model before it starts, and the same
    /// project runs again and again: the model is kept by the runtime's sealed
    /// artifact digest, so a later run emits nothing. The first run's emission
    /// is the one its devices take
    /// ([`rspice_veriloga::wasm_jit::cached_browser_model`]). A refusal is kept
    /// the same way. As many models are kept as the worker's registry holds.
    #[cfg(feature = "wasm-jit")]
    pub fn compile_wasm_jit_artifact(
        &self,
        installed: &rspice_core::HeldVerilogASources,
    ) -> Result<std::sync::Arc<rspice_veriloga::wasm_jit::WasmJitModelArtifact>, String> {
        use std::cell::RefCell;
        use std::sync::Arc;

        type Compiled = Result<Arc<rspice_veriloga::wasm_jit::WasmJitModelArtifact>, String>;
        thread_local! {
            /// Most recently used first.
            static COMPILED: RefCell<Vec<(rspice_vocabulary::product::ContentDigest, Compiled)>> =
                const { RefCell::new(Vec::new()) };
        }

        let kept = COMPILED.with(|compiled| {
            let mut compiled = compiled.borrow_mut();
            let index = compiled
                .iter()
                .position(|(digest, _)| *digest == self.artifact_digest)?;
            let entry = compiled.remove(index);
            let result = entry.1.clone();
            compiled.insert(0, entry);
            Some(result)
        });
        if let Some(kept) = kept {
            return kept;
        }
        let (model, canonical_ir) = installed
            .runtime_artifact(std::path::Path::new(&self.source_key))
            .ok_or_else(|| {
                format!(
                    "Verilog-A runtime '{}' is not installed for this run",
                    self.netlist_alias
                )
            })?;
        let result = rspice_veriloga::wasm_jit::cached_browser_model(&model, &canonical_ir);
        COMPILED.with(|compiled| {
            let mut compiled = compiled.borrow_mut();
            compiled.insert(0, (self.artifact_digest, result.clone()));
            compiled.truncate(rspice_veriloga::wasm_jit::BROWSER_MODEL_CACHE_MODELS);
        });
        result
    }

    /// Seal compiled artifacts under a caller-supplied source identity.
    /// The host must validate live project or catalog authority before calling.
    pub fn try_from_virtual_compilation(
        source_key: String,
        source_digest: rspice_vocabulary::product::ContentDigest,
        netlist_alias: String,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, PreparedRuntimeError> {
        Self::try_from_virtual_compilation_against(
            source_key,
            source_digest,
            netlist_alias,
            compilation,
            None,
            DigitalDelaySelect::Typ,
        )
    }

    /// [`Self::try_from_virtual_compilation`] for a compile given `cells`
    /// (`rspice_veriloga::CompilerOptions::external_cells`) and keeping the
    /// `delay_select` value of each min:typ:max expression
    /// (`rspice_veriloga::CompilerOptions::delay_select`).
    pub fn try_from_virtual_compilation_against(
        source_key: String,
        source_digest: rspice_vocabulary::product::ContentDigest,
        netlist_alias: String,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
        cells: Option<&[rspice_veriloga::ExternalCell]>,
        delay_select: DigitalDelaySelect,
    ) -> Result<Self, PreparedRuntimeError> {
        compilation.validate_integrity().map_err(|error| {
            PreparedRuntimeError::Integrity(format!(
                "Compiled Verilog-A bundle is invalid: {error}"
            ))
        })?;
        Self::seal(
            source_key,
            source_digest,
            &compilation.selected_module,
            netlist_alias,
            &compilation.runtime,
            cells,
            delay_select,
        )
    }

    /// Seal a report compiled given `cells` and under `delay_select`. The
    /// table is the artifact's only when a leaf of its design was built from
    /// it: a design with no leaf compiles the same against any table, and is
    /// sealed as one compiled against none. The selection is always the
    /// artifact's, since whether the design had a min:typ:max expression to
    /// choose from is not recorded.
    pub(crate) fn seal(
        source_key: String,
        source_digest: rspice_vocabulary::product::ContentDigest,
        module_name: &str,
        netlist_alias: String,
        report: &rspice_veriloga::RuntimeCompileReport,
        cells: Option<&[rspice_veriloga::ExternalCell]>,
        delay_select: DigitalDelaySelect,
    ) -> Result<Self, PreparedRuntimeError> {
        let sealed = rspice_core::seal_veriloga_artifact(&report.model, &report.canonical_ir)
            .map_err(PreparedRuntimeError::Integrity)?;
        let artifact_fingerprint =
            rspice_vocabulary::product::ContentDigest::from_bytes(sealed.fingerprint);
        let artifact = SealedArtifactBytes(sealed.bytes.into());
        let external_cells = cells
            .filter(|_| !report.canonical_ir.hir.external_leaves.is_empty())
            .map(<[_]>::to_vec);
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            source_digest,
            module_name,
            &netlist_alias,
            artifact.as_bytes(),
            artifact_fingerprint,
            external_cells.as_deref(),
            delay_select,
        );
        let runtime = Self {
            source_key,
            source_digest,
            artifact_digest,
            module_name: module_name.to_owned(),
            netlist_alias,
            artifact,
            artifact_fingerprint,
            external_cells,
            delay_select,
        };
        runtime
            .validate()
            .map_err(PreparedRuntimeError::Integrity)?;
        Ok(runtime)
    }

    fn try_from_pdk_compilation(
        package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
        archive_digest: rspice_vocabulary::product::ContentDigest,
        binding: &crate::pdk::SealedPdkVerilogABinding,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, PreparedRuntimeError> {
        let VerilogASourceName {
            source_key,
            netlist_alias,
        } = pdk_source_name(package, archive_digest, binding);
        Self::try_from_virtual_compilation(source_key, archive_digest, netlist_alias, compilation)
    }

    fn registration(&self) -> Result<rspice_core::ProjectVerilogARuntimeRegistration, String> {
        let (model, canonical_ir) = self.parsed()?;
        Ok(rspice_core::ProjectVerilogARuntimeRegistration {
            source_key: self.source_key.clone().into(),
            aliases: vec![self.netlist_alias.clone()],
            model,
            canonical_ir,
            external_cells: self.external_cells.clone(),
            delay_select: self.delay_select,
        })
    }

    /// The registration an install hands the engine: the payload checked
    /// against the digest and opened once, with the fingerprint it was sealed
    /// with.
    fn sealed_registration(
        &self,
    ) -> Result<rspice_core::ProjectVerilogASourceRegistration, String> {
        Ok(rspice_core::ProjectVerilogASourceRegistration::Sealed {
            runtime: self.registration()?,
            artifact_fingerprint: *self.artifact_fingerprint.as_bytes(),
        })
    }

    /// What the engine knows an installed artifact by: its key, its cell table
    /// and selection, and its fingerprint.
    fn installed_identity(&self) -> rspice_core::InstalledProjectVerilogARuntime<'_> {
        rspice_core::InstalledProjectVerilogARuntime {
            source_key: std::path::Path::new(&self.source_key),
            external_cells: self.external_cells.as_deref(),
            delay_select: self.delay_select,
            artifact_fingerprint: *self.artifact_fingerprint.as_bytes(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        self.parsed().map(|_| ())
    }

    /// The source key, module and alias are well formed: the checks that do
    /// not need the payload.
    fn check_identity(&self) -> Result<(), String> {
        if !valid_sealed_source_key(&self.source_key) {
            return Err("Verilog-A runtime has an invalid sealed virtual source key".to_owned());
        }
        if self.module_name.trim().is_empty() || self.module_name.chars().any(char::is_control) {
            return Err("Verilog-A runtime has an invalid module identity".to_owned());
        }
        if !valid_veriloga_netlist_identifier(&self.netlist_alias) {
            return Err("Verilog-A runtime has an invalid netlist alias".to_owned());
        }
        Ok(())
    }

    /// The identity checks and the payload digest, without opening the
    /// payload.
    fn verify_payload_digest(&self) -> Result<(), String> {
        self.check_identity()?;
        let expected = runtime_artifact_digest(
            &self.source_key,
            self.source_digest,
            &self.module_name,
            &self.netlist_alias,
            self.artifact.as_bytes(),
            self.artifact_fingerprint,
            self.external_cells.as_deref(),
            self.delay_select,
        );
        if expected != self.artifact_digest {
            return Err("Verilog-A runtime artifact digest does not match its payload".to_owned());
        }
        Ok(())
    }

    /// The model and canonical IR, checked against the digest and against
    /// each other, opened once.
    fn parsed(
        &self,
    ) -> Result<
        (
            rspice_veriloga::CompiledModel,
            rspice_veriloga::canonical_ir::CanonicalIrArtifact,
        ),
        String,
    > {
        self.verify_payload_digest()?;
        let (model, canonical_ir) =
            rspice_core::open_sealed_veriloga_artifact(self.artifact.as_bytes())?;
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
        Ok((model, canonical_ir))
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub const fn source_digest(&self) -> rspice_vocabulary::product::ContentDigest {
        self.source_digest
    }

    pub const fn artifact_digest(&self) -> rspice_vocabulary::product::ContentDigest {
        self.artifact_digest
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }

    /// The min:typ:max selection the artifact was compiled under.
    pub const fn delay_select(&self) -> DigitalDelaySelect {
        self.delay_select
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
        let (model, _) = self.parsed()?;
        Ok(model
            .terminal_names
            .iter()
            .map(ToString::to_string)
            .collect())
    }

    /// The nodes an instance card binds, one name per node, in card order.
    ///
    /// Each terminal is one node, except a logic vector's: that one terminal
    /// carries the whole signal, and the engine's boundary lays it out as one
    /// net per bit, most significant first as declared, so it reads
    /// `mode[1]`, `mode[0]` here as a bus pin's members do. The layout is the
    /// engine's own, [`rspice_core::boundary_port_layout`].
    pub fn card_terminal_names(&self) -> Result<Vec<String>, String> {
        let (_, canonical_ir) = self.parsed()?;
        Ok(rspice_core::boundary_port_layout(&canonical_ir)
            .into_iter()
            .flat_map(|port| match port.range {
                Some(range) if port.conductors > 1 => range
                    .indices_msb_first()
                    .map(|bit| format!("{}[{bit}]", port.name))
                    .collect(),
                _ => vec![port.name],
            })
            .collect())
    }
}

/// Common binding inventory for executable models and connection libraries.
/// Device-only consumers use `PreparedVerilogARuntimeSet::device_runtimes`; deck and
/// provenance consumers must use `PreparedVerilogARuntimeSet::sources`.
pub struct PreparedVerilogASourceBinding<'a> {
    source_key: &'a str,
    netlist_alias: &'a str,
    artifact_digest: rspice_vocabulary::product::ContentDigest,
    is_connection_library: bool,
}

impl<'a> PreparedVerilogASourceBinding<'a> {
    pub fn source_key(&self) -> &'a str {
        self.source_key
    }
    pub fn netlist_alias(&self) -> &'a str {
        self.netlist_alias
    }
    pub fn artifact_digest(&self) -> rspice_vocabulary::product::ContentDigest {
        self.artifact_digest
    }
    pub fn provenance_label(&self) -> String {
        let authority = if self.source_key.starts_with("__rspice_pdk__/") {
            "pdk-veriloga"
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

    /// Device runtimes and connection libraries, the second for a source that
    /// declares connect rules and no device module.
    pub fn try_with_connections(
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
        self.check_structure()
    }

    /// Everything [`Self::validate`] checks but the runtimes' payloads: each
    /// source's identity, the keys and aliases across the set, and its order.
    ///
    /// A runtime's payload is checked against its digest and parsed by the
    /// install that first hands it to the engine ([`Self::install`]), and by
    /// nothing before: a worker decodes the same project's request run after
    /// run, and reading every payload there was a fixed cost of each run.
    pub(crate) fn check_structure(&self) -> Result<(), String> {
        for runtime in &self.runtimes {
            runtime.check_identity()?;
        }
        for library in &self.connections {
            library.validate()?;
        }
        let mut keys = std::collections::HashSet::new();
        let mut aliases =
            std::collections::HashMap::<String, rspice_vocabulary::product::ContentDigest>::new();
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

    /// Install every source for this process session, held until the
    /// returned value is dropped.
    ///
    /// Hold it for as long as anything builds a deck naming these sources: a
    /// sealed source the shared cache evicted is not compiled again, so a
    /// build after the eviction fails. While held, another execution's
    /// install that could only fit by evicting one is refused instead.
    ///
    /// A session runs one project again and again. When the engine already
    /// holds every runtime under its key with the fingerprint it was sealed
    /// with, as the first run's install left it, they are held as they are,
    /// and no payload is hashed or parsed. Otherwise each payload is checked
    /// against its digest and parsed once, and the engine takes the sealed
    /// fingerprint rather than computing it.
    pub fn install(&self) -> Result<rspice_core::HeldVerilogASources, String> {
        self.check_structure()?;
        if self.connections.is_empty()
            && let Some(held) = rspice_core::hold_installed_project_veriloga_runtimes(
                self.runtimes
                    .iter()
                    .map(PreparedVerilogARuntime::installed_identity),
            )?
        {
            return Ok(held);
        }
        let registrations = self
            .runtimes
            .iter()
            .map(PreparedVerilogARuntime::sealed_registration)
            .chain(
                self.connections
                    .iter()
                    .map(PreparedVerilogAConnectionLibrary::registration),
            )
            .collect::<Result<Vec<_>, _>>()?;
        rspice_core::hold_project_veriloga_sources_for_session(
            registrations,
            rspice_core::ResourceLimits::default(),
        )
    }

    /// The sources of these sets that `deck` names on an exact sealed
    /// directive line, as one set.
    ///
    /// A deck's directives say which runtimes it installs. A run whose
    /// process corners bind different Verilog-A gives each corner's deck that
    /// corner's directives, and every task installs exactly the set its own
    /// deck names, whichever deck it was handed. The result is validated as
    /// one set, so two sources one deck names under one alias are refused.
    pub(crate) fn named_by<'a>(
        inventory: impl IntoIterator<Item = &'a Self>,
        deck: &str,
    ) -> Result<Self, String> {
        let lines = deck
            .lines()
            .map(|line| line.trim().to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        let named = |binding: PreparedVerilogASourceBinding<'_>| {
            lines.contains(
                &crate::netlist_preparation::project_veriloga_directive(
                    binding.source_key,
                    binding.netlist_alias,
                )
                .to_ascii_lowercase(),
            )
        };
        let mut runtimes = Vec::<PreparedVerilogARuntime>::new();
        let mut connections = Vec::<PreparedVerilogAConnectionLibrary>::new();
        for set in inventory {
            for runtime in &set.runtimes {
                if named(runtime.binding()) && !runtimes.contains(runtime) {
                    runtimes.push(runtime.clone());
                }
            }
            for library in &set.connections {
                if named(library.binding()) && !connections.contains(library) {
                    connections.push(library.clone());
                }
            }
        }
        let directives = lines
            .iter()
            .filter(|line| line.starts_with(".veriloga "))
            .count();
        if directives != runtimes.len() + connections.len() {
            return Err(format!(
                "The deck names {directives} Verilog-A sources and {} of them are prepared",
                runtimes.len() + connections.len()
            ));
        }
        Self::try_with_connections(runtimes, connections)
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

/// The name [`compile_pdk_source_runtime`] installs `binding`'s runtime
/// under, without compiling it.
pub fn pdk_source_name(
    package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
    archive_digest: rspice_vocabulary::product::ContentDigest,
    binding: &crate::pdk::SealedPdkVerilogABinding,
) -> VerilogASourceName {
    VerilogASourceName {
        source_key: format!(
            "__rspice_pdk__/{}/{}/{}/{}.va",
            package.manifest_digest,
            archive_digest,
            binding.source_id,
            binding.root_artifact_digest
        ),
        netlist_alias: binding.netlist_alias.clone(),
    }
}

pub fn compile_pdk_source_runtime(
    package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
    archive_digest: rspice_vocabulary::product::ContentDigest,
    artifacts: &[crate::pdk::SealedPdkVerilogAArtifact],
    binding: &crate::pdk::SealedPdkVerilogABinding,
) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
    if artifacts.is_empty() {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "PDK Verilog-A source '{}' has no authenticated artifacts",
            binding.source_id
        )));
    }
    let mut root_seen = false;
    let mut files = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let actual = rspice_vocabulary::product::ContentDigest::from_bytes(
            Sha256::digest(artifact.source.as_bytes()).into(),
        );
        if actual != artifact.digest {
            return Err(PreparedRuntimeError::SourceIdentity(format!(
                "PDK Verilog-A artifact '{}' no longer matches digest {}",
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
                    "PDK Verilog-A root '{}' no longer matches its exact manifest identity",
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
            "PDK Verilog-A root '{}' is absent from the authenticated closure",
            binding.root_artifact_path
        )));
    }
    let bundle = rspice_veriloga::VirtualSourceBundle::new(&binding.root_artifact_path, files)
        .map_err(|error| {
            PreparedRuntimeError::SourceBundle(format!(
                "PDK Verilog-A bundle '{}' is invalid: {error}",
                binding.source_id
            ))
        })?;
    thread_compiles::record(&binding.module_name);
    let compilation = rspice_veriloga::VerilogACompiler::new(unified_runtime_compiler_options())
        .compile_virtual_runtime(&bundle, &binding.module_name, pdk_virtual_compile_limits())
        .map_err(|error| {
            PreparedRuntimeError::Compile(format!(
                "Could not compile PDK Verilog-A source '{}' module '{}': {error}",
                binding.source_id, binding.module_name
            ))
        })?;
    PreparedVerilogARuntime::try_from_pdk_compilation(
        package,
        archive_digest,
        binding,
        &compilation,
    )
}

/// One model-library Verilog-A root as a deck written outside the project
/// needs it: the deck-key prefix the run installs the root's runtimes under,
/// and the sealed sources, each by its path relative to the folder holding
/// them all, so an `` `include `` between them still resolves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelLibraryVerilogAFiles {
    /// Every deck key the root's runtimes install starts with this.
    pub key_prefix: String,
    /// The root source, relative to the folder.
    pub root: String,
    /// `(relative path, source)`, the root among them.
    pub files: Vec<(String, String)>,
}

/// The sealed sources behind each model-library Verilog-A root, keyed the way
/// [`compile_model_library_source_runtimes`] keys the runtimes it installs.
pub fn model_library_veriloga_files(
    authority: &crate::model_sources::SealedModelLibraryVerilogAAuthority,
) -> Result<Vec<ModelLibraryVerilogAFiles>, PreparedRuntimeError> {
    // The sealed closure also holds the library's SPICE files, which the
    // deck reads as its own model cards and no Verilog-A source includes.
    let verilog = |path: &std::path::Path| {
        path.extension()
            .and_then(std::ffi::OsStr::to_str)
            .is_some_and(|extension| {
                ["va", "vams", "vh", "vah", "v", "sv", "svh", "h", "inc"]
                    .iter()
                    .any(|known| extension.eq_ignore_ascii_case(known))
            })
    };
    let mut sources = Vec::with_capacity(authority.sources().len());
    for (path, source) in authority.sources() {
        if verilog(path) {
            sources.push((model_library_virtual_path(path)?, source.clone()));
        }
    }
    let folder = common_folder(sources.iter().map(|(logical, _)| logical.as_str()));
    let relative = |logical: &str| logical[folder.len()..].to_owned();
    let mut roots = std::collections::BTreeSet::new();
    for root in authority.roots() {
        roots.insert(model_library_virtual_path(&root.path)?);
    }
    Ok(roots
        .into_iter()
        .map(|root_path| ModelLibraryVerilogAFiles {
            key_prefix: format!(
                "__rspice_model_library__/{}/{}/",
                authority.closure_digest(),
                model_library_root_identity(&root_path)
            ),
            root: relative(&root_path),
            files: sources
                .iter()
                .map(|(logical, source)| (relative(logical), source.clone()))
                .collect(),
        })
        .collect())
}

/// The longest `/`-ended prefix every path shares.
fn common_folder<'a>(mut paths: impl Iterator<Item = &'a str>) -> String {
    let Some(first) = paths.next() else {
        return String::new();
    };
    let mut folder = first.rfind('/').map_or("", |end| &first[..=end]);
    for path in paths {
        while !path.starts_with(folder) {
            folder = folder[..folder.len() - 1]
                .rfind('/')
                .map_or("", |end| &folder[..=end]);
        }
    }
    folder.to_owned()
}

/// What a model-library root's deck keys name it by.
fn model_library_root_identity(root_path: &str) -> rspice_vocabulary::product::ContentDigest {
    rspice_vocabulary::product::ContentDigest::from_bytes(
        Sha256::digest(root_path.as_bytes()).into(),
    )
}

pub fn compile_model_library_source_runtimes(
    authority: &crate::model_sources::SealedModelLibraryVerilogAAuthority,
    limits: rspice_veriloga::VirtualCompileLimits,
) -> Result<PreparedVerilogARuntimeSet, PreparedRuntimeError> {
    // Every retained model includes its canonical digital plan and is installed
    // through the unified engine. Mixed reports are safe on this host path.
    let compiler = rspice_veriloga::VerilogACompiler::new(unified_runtime_compiler_options());
    let mut runtimes = Vec::new();
    let mut connections = Vec::new();
    for root in ModelLibraryRootPath::of(authority)? {
        let prepared = root.prepare(&compiler, limits)?;
        let module_names = prepared.module_names().collect::<Vec<_>>();
        for source in root.sources(authority, Some(&module_names))? {
            match source {
                ModelLibraryRootSource::Connections(name) => {
                    let artifact = prepared.connection_artifact().ok_or_else(|| {
                        PreparedRuntimeError::SourceIdentity(format!(
                            "Model-library Verilog-A source '{}' declares neither device modules nor connection rules",
                            root.display()
                        ))
                    })?;
                    connections.push(
                        PreparedVerilogAConnectionLibrary::try_new(
                            name.source_key,
                            authority.closure_digest(),
                            name.netlist_alias,
                            artifact,
                        )
                        .map_err(PreparedRuntimeError::Integrity)?,
                    );
                }
                ModelLibraryRootSource::Device { module_name, name } => {
                    thread_compiles::record(&module_name);
                    let compilation = prepared
                        .compile_runtime(&module_name)
                        .map_err(|error| {
                            PreparedRuntimeError::Compile(format!(
                                "Could not compile module '{}' from sealed model-library Verilog-A root '{}': {error}",
                                module_name,
                                root.display()
                            ))
                        })?;
                    runtimes.push(PreparedVerilogARuntime::try_from_virtual_compilation(
                        name.source_key,
                        authority.closure_digest(),
                        name.netlist_alias,
                        &compilation,
                    )?);
                }
            }
        }
    }
    PreparedVerilogARuntimeSet::try_with_connections(runtimes, connections)
        .map_err(PreparedRuntimeError::Integrity)
}

/// Test-only record of the Verilog-A modules compiled on the calling thread,
/// so a test can state that a frame compiled none.
pub mod thread_compiles {
    #[cfg(any(test, feature = "test-support"))]
    thread_local! {
        static COMPILED: std::cell::RefCell<Vec<String>> = const {
            std::cell::RefCell::new(Vec::new())
        };
    }

    #[cfg_attr(not(any(test, feature = "test-support")), allow(unused_variables))]
    pub(crate) fn record(module_name: &str) {
        #[cfg(any(test, feature = "test-support"))]
        COMPILED.with(|compiled| compiled.borrow_mut().push(module_name.to_owned()));
    }

    /// The modules this thread compiled since the last call, which forgets
    /// them.
    #[cfg(any(test, feature = "test-support"))]
    pub fn take() -> Vec<String> {
        COMPILED.with(|compiled| std::mem::take(&mut *compiled.borrow_mut()))
    }
}

/// The deck key and netlist alias one Verilog-A source installs under: what
/// a `.veriloga` directive names it by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogASourceName {
    pub source_key: String,
    pub netlist_alias: String,
}

/// The names [`compile_model_library_source_runtimes`] installs `authority`'s
/// runtimes under, device sources first and then connection libraries, each
/// in the order the compiled set holds them, without compiling any module.
///
/// A root that selects its module is named from the selection alone. A root
/// that installs every module it declares is parsed for their names: a
/// source's names are read by the front end, not by the compile.
pub fn model_library_source_names(
    authority: &crate::model_sources::SealedModelLibraryVerilogAAuthority,
    limits: rspice_veriloga::VirtualCompileLimits,
) -> Result<(Vec<VerilogASourceName>, Vec<VerilogASourceName>), PreparedRuntimeError> {
    let compiler = rspice_veriloga::VerilogACompiler::new(unified_runtime_compiler_options());
    let mut devices = Vec::new();
    let mut connections = Vec::new();
    for root in ModelLibraryRootPath::of(authority)? {
        let prepared = root
            .reads_module_names()
            .then(|| root.prepare(&compiler, limits))
            .transpose()?;
        let module_names = prepared
            .as_ref()
            .map(|prepared| prepared.module_names().collect::<Vec<_>>());
        for source in root.sources(authority, module_names.as_deref())? {
            match source {
                ModelLibraryRootSource::Connections(name) => connections.push(name),
                ModelLibraryRootSource::Device { name, .. } => devices.push(name),
            }
        }
    }
    devices.sort_by_cached_key(|name| name.source_key.to_ascii_lowercase());
    connections.sort_by_cached_key(|name| name.source_key.to_ascii_lowercase());
    Ok((devices, connections))
}

/// One source path of a model library that `.hdl`/`ahdl_include` lines name
/// as a root, with every root that names it.
struct ModelLibraryRootPath<'a> {
    logical_path: String,
    logical_sources: std::rc::Rc<[rspice_veriloga::VirtualSourceFile]>,
    roots: Vec<&'a crate::model_sources::SealedModelLibraryVerilogARoot>,
}

/// What one root installs.
enum ModelLibraryRootSource {
    Device {
        module_name: String,
        name: VerilogASourceName,
    },
    /// A source that declares connect rules and no device module.
    Connections(VerilogASourceName),
}

impl<'a> ModelLibraryRootPath<'a> {
    /// `authority`'s roots, grouped by the path they name, in path order.
    fn of(
        authority: &'a crate::model_sources::SealedModelLibraryVerilogAAuthority,
    ) -> Result<Vec<Self>, PreparedRuntimeError> {
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
        let logical_sources = std::rc::Rc::<[_]>::from(logical_sources);
        let mut roots_by_path = std::collections::BTreeMap::<_, Vec<_>>::new();
        for root in authority.roots() {
            let root_path = model_library_virtual_path(&root.path)?;
            roots_by_path.entry(root_path).or_default().push(root);
        }
        Ok(roots_by_path
            .into_iter()
            .map(|(logical_path, roots)| Self {
                logical_path,
                logical_sources: std::rc::Rc::clone(&logical_sources),
                roots,
            })
            .collect())
    }

    fn display(&self) -> std::path::Display<'_> {
        self.roots[0].path.display()
    }

    /// Whether some root here installs every module the source declares,
    /// so naming its runtimes reads the source.
    fn reads_module_names(&self) -> bool {
        self.roots.iter().any(|root| root.selected_module.is_none())
    }

    fn prepare(
        &self,
        compiler: &rspice_veriloga::VerilogACompiler,
        limits: rspice_veriloga::VirtualCompileLimits,
    ) -> Result<rspice_veriloga::PreparedVirtualSource, PreparedRuntimeError> {
        let bundle = rspice_veriloga::VirtualSourceBundle::new(
            &self.logical_path,
            self.logical_sources.iter().cloned(),
        )
        .map_err(|error| {
            PreparedRuntimeError::SourceBundle(format!(
                "Sealed model-library Verilog-A root '{}' is invalid: {error}",
                self.display()
            ))
        })?;
        compiler
            .prepare_virtual_runtime_source(&bundle, limits)
            .map_err(|error| {
                PreparedRuntimeError::Compile(format!(
                    "Could not prepare sealed model-library Verilog-A root '{}': {error}",
                    self.display()
                ))
            })
    }

    /// What each root here installs, under the keys a run's deck names.
    /// `module_names` are the modules the source declares, in declaration
    /// order; `None` when it was not read, which only a root that selects
    /// its module allows, and then the selection is not checked against it.
    fn sources(
        &self,
        authority: &crate::model_sources::SealedModelLibraryVerilogAAuthority,
        module_names: Option<&[&str]>,
    ) -> Result<Vec<ModelLibraryRootSource>, PreparedRuntimeError> {
        let root_identity = model_library_root_identity(&self.logical_path);
        let mut sources = Vec::new();
        for root in &self.roots {
            let declared = module_names.unwrap_or_default();
            if module_names.is_some_and(<[_]>::is_empty) && root.selected_module.is_none() {
                sources.push(ModelLibraryRootSource::Connections(VerilogASourceName {
                    source_key: format!(
                        "__rspice_model_library__/{}/{}/connections.vams",
                        authority.closure_digest(),
                        root_identity
                    ),
                    netlist_alias: root
                        .netlist_alias
                        .clone()
                        .unwrap_or_else(|| format!("__rspice_connections_{root_identity}")),
                }));
                continue;
            }
            let selected = if let Some(module) = root.selected_module.as_deref() {
                if module_names.is_some() && !declared.contains(&module) {
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
                let [module] = declared else {
                    return Err(PreparedRuntimeError::SourceIdentity(format!(
                        "Model-library .veriloga source '{}' declares {} modules, so alias '{}' is ambiguous",
                        root.path.display(),
                        declared.len(),
                        alias
                    )));
                };
                vec![((*module).to_owned(), alias.to_owned())]
            } else {
                declared
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
                let source_key = format!(
                    "__rspice_model_library__/{}/{}/{}.va",
                    authority.closure_digest(),
                    root_identity,
                    module_name
                );
                sources.push(ModelLibraryRootSource::Device {
                    module_name,
                    name: VerilogASourceName {
                        source_key,
                        netlist_alias,
                    },
                });
            }
        }
        Ok(sources)
    }
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

#[expect(
    clippy::too_many_arguments,
    reason = "each argument is one field of the artifact's identity, hashed in this order"
)]
fn runtime_artifact_digest(
    source_key: &str,
    source_digest: rspice_vocabulary::product::ContentDigest,
    module_name: &str,
    netlist_alias: &str,
    artifact: &[u8],
    artifact_fingerprint: rspice_vocabulary::product::ContentDigest,
    external_cells: Option<&[rspice_veriloga::ExternalCell]>,
    delay_select: DigitalDelaySelect,
) -> rspice_vocabulary::product::ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.project-veriloga-runtime/v4\0");
    let mut field = |bytes: &[u8]| {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    };
    for bytes in [
        source_key.as_bytes(),
        source_digest.as_bytes(),
        module_name.as_bytes(),
        netlist_alias.as_bytes(),
        artifact,
        artifact_fingerprint.as_bytes(),
    ] {
        field(bytes);
    }
    // An artifact with no leaf keeps the identity it always had.
    if let Some(cells) = external_cells {
        field(b"external-cells");
        field(&(cells.len() as u64).to_le_bytes());
        for cell in cells {
            field(cell.name.as_bytes());
            for names in [&cell.ports, &cell.parameters] {
                field(&(names.len() as u64).to_le_bytes());
                names.iter().for_each(|name| field(name.as_bytes()));
            }
        }
    }
    // A typical artifact keeps the identity it always had.
    if delay_select != DigitalDelaySelect::Typ {
        field(b"delay-select");
        field(delay_select.keyword().as_bytes());
    }
    rspice_vocabulary::product::ContentDigest::from_bytes(hasher.finalize().into())
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
