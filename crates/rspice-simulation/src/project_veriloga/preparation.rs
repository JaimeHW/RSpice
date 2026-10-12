//! Prepare exact project, configured-cell and sealed-library Verilog-A runtimes.

use crate::netlist_preparation::executable_logical_lines;
use crate::preparation::{PreparationError, PreparationStage};
use crate::project_veriloga::receipt::ProjectCompileReceipt;
use rspice_app_types::product::ProjectId;
use rspice_design::project_sources::{
    ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceRegistry,
};
use rspice_design::projection::ConfigurationExecutionProjection;
use std::collections::HashMap;

pub fn prepared_configuration_veriloga_runtimes(
    project_id: ProjectId,
    sources: &ProjectSourceRegistry,
    projection: &ConfigurationExecutionProjection,
) -> Result<crate::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let mut prepared = HashMap::<String, crate::veriloga::PreparedVerilogARuntime>::new();
    for execution in projection.plan().bindings() {
        let Some(binding) = execution.project_veriloga() else {
            continue;
        };
        let bundle = sources
            .get_bundle(binding.source_bundle_id())
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Configured Verilog-A source bundle {} at {} no longer exists",
                        binding.source_bundle_id(),
                        execution.instance_path()
                    ),
                )
            })?;
        if bundle.closure_digest() != binding.source_closure_digest() {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A source bundle {} changed after hierarchy resolution",
                    binding.source_bundle_id()
                ),
            ));
        }
        let runtime = crate::project_veriloga::compile_project_source_bundle_runtime(
            project_id,
            bundle,
            binding.selected_module(),
        )
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        if runtime.source_key() != binding.source_key()
            || !runtime
                .netlist_alias()
                .eq_ignore_ascii_case(binding.netlist_alias())
        {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A binding at {} changed while compiling its sealed source",
                    execution.instance_path()
                ),
            ));
        }
        let materialized = execution.materialized_binding().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A binding at {} has no materialized interface",
                    execution.instance_path()
                ),
            )
        })?;
        let compiled_terminals = runtime
            .terminal_names()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        if compiled_terminals.len() != materialized.terminal_order.len()
            || !compiled_terminals
                .iter()
                .zip(&materialized.terminal_order)
                .all(|(compiled, declared)| compiled.eq_ignore_ascii_case(declared))
        {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Compiled Verilog-A module '{}' at {} does not match the exact declared terminal order [{}]",
                    binding.selected_module(),
                    execution.instance_path(),
                    materialized.terminal_order.join(", ")
                ),
            ));
        }
        if let Some(existing) = prepared.get(runtime.source_key()) {
            if existing != &runtime {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Configured Verilog-A source key '{}' resolves to conflicting artifacts",
                        runtime.source_key()
                    ),
                ));
            }
        } else {
            prepared.insert(runtime.source_key().to_owned(), runtime);
        }
    }
    crate::veriloga::PreparedVerilogARuntimeSet::try_new(prepared.into_values().collect())
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

/// The runtime of one project Verilog-A bundle a deck names, from the
/// retained compile receipt when it is this bundle's current one, else rebuilt
/// from the validated bytes.
fn prepared_project_veriloga_runtime(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    retained: Option<&ProjectCompileReceipt>,
) -> Result<crate::veriloga::PreparedVerilogARuntime, PreparationError> {
    let document = bundle.root();
    if let Some(receipt) = retained
        && receipt.token().project_id == project_id
        && receipt.token().bundle_id == bundle.id()
        && receipt.token().revision == bundle.revision().get()
        && receipt.token().closure_digest == bundle.closure_digest()
    {
        return receipt
            .prepare_runtime(project_id, bundle)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error));
    }
    if !document.validation_is_current() {
        return Err(PreparationError::new(
            PreparationStage::ModelBindings,
            format!(
                "Compile the exact current project Verilog-A source '{}' before preparing execution",
                document.file_name()
            ),
        ));
    }
    // Persisted validation authenticates only the exact source bytes. Rebuild
    // transient executable artifacts rather than trusting serialized code or
    // requiring a redundant manual compile after project/session restore.
    let (receipt, _) =
        crate::project_veriloga::receipt::compile_project_bundle_receipt(project_id, bundle, None)
            .map_err(|diagnostics| {
                let detail = diagnostics
                    .first()
                    .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
                    .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Could not rebuild validated Verilog-A source '{}': {detail}",
                        document.file_name()
                    ),
                )
            })?;
    receipt
        .prepare_runtime(project_id, bundle)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

pub fn prepared_signed_pdk_veriloga_runtimes(
    sealed_models: &crate::model_sources::SealedModelExecutionSources,
) -> Result<crate::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some((package, archive_digest, artifacts, bindings)) =
        sealed_models.pdk_veriloga_authority()
    else {
        return Ok(Default::default());
    };
    let runtimes = bindings
        .iter()
        .map(|binding| {
            crate::veriloga::compile_signed_pdk_source_runtime(
                package,
                archive_digest,
                artifacts,
                binding,
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    crate::veriloga::PreparedVerilogARuntimeSet::try_new(runtimes)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

pub fn prepared_model_library_veriloga_runtimes(
    sealed_models: &crate::model_sources::SealedModelExecutionSources,
) -> Result<crate::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some(authority) = sealed_models
        .model_library_veriloga_authority()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
    else {
        return Ok(Default::default());
    };
    crate::project_veriloga::compile_model_library_source_runtimes(&authority)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

/// The project Verilog-A runtimes a deck's `.veriloga` directives name
/// exactly, one per bundle, whichever cell view owns it.
pub fn project_veriloga_runtimes_referenced_by(
    project_id: ProjectId,
    sources: &ProjectSourceRegistry,
    retained: Option<&ProjectCompileReceipt>,
    source: &str,
) -> Result<crate::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let lines = executable_logical_lines(source);
    let mut runtimes = Vec::new();
    for bundle in sources
        .iter_bundles()
        .filter(|bundle| bundle.language() == ProjectSourceLanguage::VerilogA)
    {
        let key_prefix = format!(
            "__rspice_project__/{}/{}/{}/",
            project_id,
            bundle.id(),
            bundle.closure_digest()
        );
        let key_suffix = format!("/{}", bundle.root().logical_path());
        let references_bundle = lines
            .iter()
            .filter_map(|(_, line)| parse_veriloga_directive_identity(line))
            .any(|(path, _)| path.starts_with(&key_prefix) && path.ends_with(&key_suffix));
        if !references_bundle {
            continue;
        }
        let runtime = prepared_project_veriloga_runtime(project_id, bundle, retained)?;
        let exact_reference = lines.iter().any(|(_, line)| {
            project_veriloga_directive_matches_exact_identity(
                line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        });
        if exact_reference {
            runtimes.push(runtime);
        }
    }
    crate::veriloga::PreparedVerilogARuntimeSet::try_new(runtimes)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

/// Rewrite each `.veriloga` key that names a project bundle's current bytes
/// under an earlier identity to that bundle's current key.
///
/// A key frames the bundle's owner into its digest, so a deck written against
/// the project-wide Verilog-A workspace names a key its source no longer
/// carries once that source became a cell view's. The bytes are the same,
/// which [`ProjectSourceRegistry::current_veriloga_source_key`] proves before
/// anything is rewritten; a key naming bytes the bundle no longer holds is
/// left exactly as written, to be refused as the stale reference it is.
#[must_use]
pub fn rebind_project_veriloga_source_keys(
    project_id: ProjectId,
    sources: &ProjectSourceRegistry,
    source: String,
) -> String {
    source
        .split_inclusive('\n')
        .map(|line| {
            let Some((key, _)) = parse_veriloga_directive_identity(line.trim()) else {
                return std::borrow::Cow::Borrowed(line);
            };
            match sources.current_veriloga_source_key(project_id, key) {
                Some(current) if current != key => {
                    std::borrow::Cow::Owned(line.replacen(key, &current, 1))
                }
                _ => std::borrow::Cow::Borrowed(line),
            }
        })
        .collect()
}

/// Parse the identity-bearing fields from the one project Verilog-A directive
/// shape emitted by RSpice. SPICE command and model identifiers are
/// case-insensitive, but the project virtual path is an authenticated key and
/// must remain byte-for-byte exact.
fn parse_veriloga_directive_identity(line: &str) -> Option<(&str, Option<&str>)> {
    let (command, remainder) = take_spice_token(line)?;
    if !command.eq_ignore_ascii_case(".veriloga") {
        return None;
    }
    let (path, remainder) = take_spice_token(remainder)?;
    let remainder = remainder.trim();
    if remainder.is_empty() {
        return Some((path, None));
    }
    let (model_name, trailing) = take_spice_token(remainder)?;
    trailing
        .trim()
        .is_empty()
        .then_some((path, Some(model_name)))
}

fn take_spice_token(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    let first = input.chars().next()?;
    if matches!(first, '\'' | '"') {
        let quoted = &input[first.len_utf8()..];
        let mut escaped = false;
        for (index, character) in quoted.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character == first {
                return Some((&quoted[..index], &quoted[index + character.len_utf8()..]));
            }
        }
        return None;
    }

    let end = input.find(char::is_whitespace).unwrap_or(input.len());
    (end > 0).then_some((&input[..end], &input[end..]))
}

pub(crate) fn project_veriloga_directive_matches_exact_identity(
    line: &str,
    source_key: &str,
    module_name: &str,
) -> bool {
    parse_veriloga_directive_identity(line).is_some_and(|(path, model_name)| {
        path == source_key
            && model_name.is_some_and(|model| model.eq_ignore_ascii_case(module_name))
    })
}
