//! Project source compilation with typed failures for host diagnostic adapters.

use rspice_design::project_sources::{ProjectSourceBundle, ProjectSourceRole};
use rspice_veriloga::{RuntimeCompileReport, VerilogACompiler};

use crate::veriloga::{PreparedRuntimeError, PreparedVerilogARuntime};

pub mod build_profile;
pub mod worker;

/// Compilation failures before projection into editor or worker diagnostics.
#[derive(Debug)]
pub enum ProjectVerilogACompileError {
    BuildProfile(String),
    SourceClosure(String),
    RootModuleRequired,
    Compile(Box<rspice_veriloga::CompileError>),
    Virtual(Box<rspice_veriloga::VirtualRuntimeCompileFailure>),
}

/// Compile the selected source closure using its authored build policy.
pub fn compile_project_bundle_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<Box<RuntimeCompileReport>, ProjectVerilogACompileError> {
    let resolved = build_profile::resolve_veriloga_build_profile(bundle)
        .map_err(ProjectVerilogACompileError::BuildProfile)?;
    let compiler = VerilogACompiler::new(resolved.profile.compiler_options());
    let qualifications = resolved.profile.qualification_options();
    let source = bundle.root().content();
    let selected_module = selected_module.or(match resolved.profile.entry_modules.as_slice() {
        [module] => Some(module.as_str()),
        _ => None,
    });
    let has_source_dependencies = bundle.files().iter().any(|file| {
        bundle.role_for_path(file.logical_path()) != Some(ProjectSourceRole::VerilogABuildProfile)
    });
    if let Some(module_name) = selected_module {
        resolved
            .profile
            .validate_selected_module(module_name)
            .map_err(ProjectVerilogACompileError::BuildProfile)?;
        let bundle =
            build_profile::project_bundle_as_virtual_with_profile(bundle, &resolved.profile)
                .map_err(ProjectVerilogACompileError::SourceClosure)?;
        return match compiler.compile_virtual_runtime_diagnosed_with_qualifications(
            &bundle,
            module_name,
            project_virtual_compile_limits(),
            qualifications,
        ) {
            Ok(compilation) => successful_compile_outcome(compilation.runtime, &resolved.profile),
            Err(failure) => Err(ProjectVerilogACompileError::Virtual(Box::new(failure))),
        };
    }
    if has_source_dependencies || resolved.profile.entry_modules.len() > 1 {
        return Err(ProjectVerilogACompileError::RootModuleRequired);
    }
    match compiler.compile_runtime_with_qualifications(source, None, qualifications) {
        Ok(report) => successful_compile_outcome(report, &resolved.profile),
        Err(error) => Err(ProjectVerilogACompileError::Compile(Box::new(error))),
    }
}

fn successful_compile_outcome(
    report: RuntimeCompileReport,
    profile: &build_profile::VerilogABuildProfile,
) -> Result<Box<RuntimeCompileReport>, ProjectVerilogACompileError> {
    build_profile::validate_profile_cell_bindings(&report, profile)
        .map_err(ProjectVerilogACompileError::BuildProfile)?;
    Ok(Box::new(report))
}

pub fn compile_project_virtual_runtime(
    bundle: &rspice_design::project_sources::ProjectSourceBundle,
    module_name: &str,
) -> Result<rspice_veriloga::VirtualRuntimeCompilation, PreparedRuntimeError> {
    let resolved = build_profile::resolve_veriloga_build_profile(bundle)
        .map_err(PreparedRuntimeError::SourceIdentity)?;
    resolved
        .profile
        .validate_selected_module(module_name)
        .map_err(PreparedRuntimeError::SourceIdentity)?;
    let virtual_bundle =
        build_profile::project_bundle_as_virtual_with_profile(bundle, &resolved.profile)
            .map_err(PreparedRuntimeError::SourceBundle)?;
    let compilation = rspice_veriloga::VerilogACompiler::new(resolved.profile.compiler_options())
        .compile_virtual_runtime_diagnosed_with_qualifications(
            &virtual_bundle, module_name, project_virtual_compile_limits(), resolved.profile.qualification_options(),
        )
        .map_err(|error| PreparedRuntimeError::Compile(format!(
            "Could not compile Verilog-A module '{module_name}' from project bundle {}: {error}", bundle.id(),
        )))?;
    build_profile::validate_profile_cell_bindings(&compilation.runtime, &resolved.profile)
        .map_err(PreparedRuntimeError::SourceIdentity)?;
    Ok(compilation)
}

pub fn compile_project_source_bundle_runtime(
    project_id: rspice_app_types::product::ProjectId,
    bundle: &rspice_design::project_sources::ProjectSourceBundle,
    module_name: &str,
) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
    if bundle.language() != rspice_design::project_sources::ProjectSourceLanguage::VerilogA {
        return Err(PreparedRuntimeError::SourceIdentity(format!(
            "Project source bundle {} is {}, not Verilog-A",
            bundle.id(),
            bundle.language()
        )));
    }
    let source_key = rspice_design::project_sources::project_veriloga_bundle_source_key(
        project_id,
        bundle,
        module_name,
    )
    .map_err(|error| PreparedRuntimeError::SourceIdentity(error.to_string()))?;
    let netlist_alias =
        rspice_design::project_sources::project_veriloga_bundle_alias(bundle, module_name)
            .map_err(|error| PreparedRuntimeError::SourceIdentity(error.to_string()))?;
    let compilation = compile_project_virtual_runtime(bundle, module_name)?;
    PreparedVerilogARuntime::try_from_virtual_compilation(
        source_key,
        bundle.closure_digest(),
        netlist_alias,
        &compilation,
    )
}

pub fn project_virtual_compile_limits() -> rspice_veriloga::VirtualCompileLimits {
    rspice_veriloga::VirtualCompileLimits {
        max_files: rspice_design::project_sources::MAX_PROJECT_SOURCE_FILES,
        max_path_bytes: rspice_design::project_sources::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
        max_file_bytes: rspice_design::project_sources::MAX_PROJECT_CODE_SOURCE_BYTES,
        max_total_source_bytes: rspice_design::project_sources::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        max_include_depth: rspice_design::project_sources::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
        // Macro expansion is intentionally bounded separately from retained
        // source bytes. Keep this identical to the prepared-runtime path so a
        // bundle accepted by the editor cannot be rejected only at execution.
        max_expanded_bytes: rspice_design::project_sources::MAX_PROJECT_SOURCE_BUNDLE_BYTES
            .saturating_mul(2),
        ..rspice_veriloga::VirtualCompileLimits::default()
    }
}
