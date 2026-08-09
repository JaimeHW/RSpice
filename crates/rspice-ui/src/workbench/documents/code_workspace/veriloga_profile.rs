//! Persisted, fail-closed Verilog-A build configuration.
//!
//! The profile is a project document selected by an explicit semantic role.
//! Its path is presentation only: compilation never infers policy from a
//! demonstration filename or from ambient host configuration.

use std::collections::{BTreeMap, BTreeSet};

use rspice_veriloga::{CompilerOptions, InterpreterFallbackPolicy, RuntimeQualificationOptions};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::ContentDigest;
use crate::state::{
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, ProjectSourceBundle, ProjectSourceLanguage,
    ProjectSourceRole,
};

pub(crate) const VERILOGA_BUILD_PROFILE_SCHEMA: &str = "rspice.veriloga-build/v1";
const MAX_BUILD_PROFILE_BYTES: usize = 1024 * 1024;
const MAX_ENTRY_MODULES: usize = 4096;
const MAX_INCLUDE_PATHS: usize = 4096;
const MAX_PREPROCESSOR_SYMBOLS: usize = 16_384;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogABuildProfile {
    pub schema: String,
    pub package: VerilogAPackage,
    #[serde(default)]
    pub entry_modules: Vec<String>,
    #[serde(default)]
    pub include_paths: Vec<String>,
    #[serde(default)]
    pub preprocessor: VerilogAPreprocessorProfile,
    #[serde(default)]
    pub targets: VerilogATargetProfile,
    #[serde(default)]
    pub checks: VerilogACheckProfile,
    #[serde(default)]
    pub cell_bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAPackage {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAPreprocessorProfile {
    #[serde(default)]
    pub defines: BTreeMap<String, String>,
    #[serde(default)]
    pub undefines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogATargetProfile {
    #[serde(default = "enabled")]
    pub portable_interpreter: bool,
    #[serde(default)]
    pub generated_rust: bool,
    #[serde(default)]
    pub native_x64_jit: bool,
    #[serde(default)]
    pub fallback: VerilogAFallbackPolicy,
}

impl Default for VerilogATargetProfile {
    fn default() -> Self {
        Self {
            portable_interpreter: true,
            generated_rust: false,
            native_x64_jit: false,
            fallback: VerilogAFallbackPolicy::Allow,
        }
    }
}

const fn enabled() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum VerilogAFallbackPolicy {
    #[default]
    Allow,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogACheckProfile {
    #[serde(default = "enabled")]
    pub hidden_state: bool,
    #[serde(default = "enabled")]
    pub discontinuities: bool,
    #[serde(default = "enabled")]
    pub units_and_ranges: bool,
    #[serde(default = "enabled")]
    pub convergence: bool,
    #[serde(default = "enabled")]
    pub portability: bool,
}

impl Default for VerilogACheckProfile {
    fn default() -> Self {
        Self {
            hidden_state: true,
            discontinuities: true,
            units_and_ranges: true,
            convergence: true,
            portability: true,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedVerilogABuildProfile {
    pub profile: VerilogABuildProfile,
    pub logical_path: Option<String>,
    pub digest: ContentDigest,
    pub legacy_builtin: bool,
}

impl VerilogABuildProfile {
    pub(crate) fn starter(module: &str) -> Self {
        Self {
            schema: VERILOGA_BUILD_PROFILE_SCHEMA.to_owned(),
            package: VerilogAPackage {
                name: module.to_owned(),
                version: "0.1.0".to_owned(),
            },
            entry_modules: vec![module.to_owned()],
            include_paths: Vec::new(),
            preprocessor: VerilogAPreprocessorProfile::default(),
            targets: VerilogATargetProfile::default(),
            checks: VerilogACheckProfile::default(),
            cell_bindings: BTreeMap::new(),
        }
    }

    pub(crate) fn parse(source: &str) -> Result<Self, String> {
        if source.len() > MAX_BUILD_PROFILE_BYTES {
            return Err(format!(
                "Verilog-A build profile exceeds the {MAX_BUILD_PROFILE_BYTES}-byte limit."
            ));
        }
        let profile: Self = toml::from_str(source)
            .map_err(|error| format!("Invalid Verilog-A build profile: {error}"))?;
        profile.validate()?;
        Ok(profile)
    }

    pub(crate) fn to_toml(&self) -> Result<String, String> {
        self.validate()?;
        toml::to_string_pretty(self)
            .map_err(|error| format!("Could not encode Verilog-A build profile: {error}"))
    }

    pub(crate) fn compiler_options(&self) -> CompilerOptions {
        CompilerOptions {
            defines: self
                .preprocessor
                .defines
                .iter()
                .map(|(name, value)| (name.clone(), Some(value.clone())))
                .collect(),
            undefines: self.preprocessor.undefines.clone(),
            ..CompilerOptions::default()
        }
    }

    pub(crate) const fn qualification_options(&self) -> RuntimeQualificationOptions {
        RuntimeQualificationOptions {
            generated_rust: self.targets.generated_rust,
            native_x64_jit: self.targets.native_x64_jit,
            // The build profile does not yet expose wasm qualification;
            // requesting none preserves the schema's exact prior behavior.
            wasm_jit: false,
            interpreter_fallback: match self.targets.fallback {
                VerilogAFallbackPolicy::Allow => InterpreterFallbackPolicy::Allow,
                VerilogAFallbackPolicy::Reject => InterpreterFallbackPolicy::Reject,
            },
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema != VERILOGA_BUILD_PROFILE_SCHEMA {
            return Err(format!(
                "Unsupported Verilog-A build-profile schema '{}'; expected '{VERILOGA_BUILD_PROFILE_SCHEMA}'.",
                self.schema
            ));
        }
        validate_identifier(&self.package.name, "package name")?;
        semver::Version::parse(&self.package.version)
            .map_err(|error| format!("Invalid Verilog-A package version: {error}"))?;
        if !self.targets.portable_interpreter {
            return Err(
                "The portable interpreter is mandatory for a cross-platform Verilog-A package."
                    .to_owned(),
            );
        }
        validate_unique_identifiers(&self.entry_modules, "entry module", MAX_ENTRY_MODULES)?;
        validate_unique_paths(&self.include_paths, "include path", MAX_INCLUDE_PATHS)?;
        if self.preprocessor.defines.len() + self.preprocessor.undefines.len()
            > MAX_PREPROCESSOR_SYMBOLS
        {
            return Err(format!(
                "Verilog-A build profile exceeds the {MAX_PREPROCESSOR_SYMBOLS}-symbol preprocessor limit."
            ));
        }
        for name in self.preprocessor.defines.keys() {
            validate_identifier(name, "preprocessor definition")?;
        }
        validate_unique_identifiers(
            &self.preprocessor.undefines,
            "preprocessor undefinition",
            MAX_PREPROCESSOR_SYMBOLS,
        )?;
        if let Some(name) = self
            .preprocessor
            .undefines
            .iter()
            .find(|name| self.preprocessor.defines.contains_key(*name))
        {
            return Err(format!(
                "Preprocessor symbol '{name}' cannot be both defined and undefined."
            ));
        }
        for (cell, module) in &self.cell_bindings {
            validate_binding_path(cell)?;
            validate_identifier(module, "cell-model binding")?;
        }
        Ok(())
    }
}

pub(crate) fn resolve_veriloga_build_profile(
    bundle: &ProjectSourceBundle,
) -> Result<ResolvedVerilogABuildProfile, String> {
    if bundle.language() != ProjectSourceLanguage::VerilogA {
        return Err("Cannot resolve a Verilog-A build profile for another source language.".into());
    }
    let paths = bundle
        .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
        .collect::<Vec<_>>();
    match paths.as_slice() {
        [] => {
            let module = module_name_from_path(bundle.root().logical_path());
            let mut profile = VerilogABuildProfile::starter(&module);
            // A legacy bundle has no authored module-selection contract. Its
            // file name is presentation metadata and must never be promoted
            // into executable semantics. With an empty entry list the
            // compiler selects the only declared module exactly, while a
            // multi-module source still fails closed until the project records
            // an explicit entry module.
            profile.entry_modules.clear();
            let encoded = profile.to_toml()?;
            Ok(ResolvedVerilogABuildProfile {
                profile,
                logical_path: None,
                digest: digest(encoded.as_bytes()),
                legacy_builtin: true,
            })
        }
        [path] => {
            let source = bundle.file_content(path).ok_or_else(|| {
                format!("Verilog-A build-profile role references missing document '{path}'.")
            })?;
            let profile = VerilogABuildProfile::parse(source)?;
            Ok(ResolvedVerilogABuildProfile {
                profile,
                logical_path: Some((*path).to_owned()),
                digest: digest(source.as_bytes()),
                legacy_builtin: false,
            })
        }
        _ => Err("A Verilog-A bundle may own only one build profile.".to_owned()),
    }
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn module_name_from_path(path: &str) -> String {
    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .split('.')
        .next()
        .unwrap_or_default();
    let mut module = stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if module.is_empty() || module.starts_with(|character: char| character.is_ascii_digit()) {
        module.insert_str(0, "model_");
    }
    module
}

fn validate_unique_identifiers(values: &[String], label: &str, limit: usize) -> Result<(), String> {
    if values.len() > limit {
        return Err(format!(
            "Verilog-A build profile exceeds the {limit}-{label} limit."
        ));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        validate_identifier(value, label)?;
        if !unique.insert(value) {
            return Err(format!(
                "Duplicate {label} '{value}' in Verilog-A build profile."
            ));
        }
    }
    Ok(())
}

fn validate_identifier(value: &str, label: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.len() <= MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
        && value
            .chars()
            .next()
            .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && value
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric());
    if !valid {
        return Err(format!("Invalid Verilog-A {label} '{value}'."));
    }
    Ok(())
}

fn validate_unique_paths(values: &[String], label: &str, limit: usize) -> Result<(), String> {
    if values.len() > limit {
        return Err(format!(
            "Verilog-A build profile exceeds the {limit}-{label} limit."
        ));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        validate_portable_path(value, label)?;
        if !unique.insert(value.to_ascii_lowercase()) {
            return Err(format!(
                "Duplicate {label} '{value}' in Verilog-A build profile."
            ));
        }
    }
    Ok(())
}

fn validate_portable_path(value: &str, label: &str) -> Result<(), String> {
    let invalid = value.is_empty()
        || value.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.contains('\0')
        || value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..");
    if invalid {
        return Err(format!("Invalid portable Verilog-A {label} '{value}'."));
    }
    Ok(())
}

fn validate_binding_path(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
        || value.contains('\0')
        || value.chars().any(char::is_control)
        || value
            .split('/')
            .any(|segment| validate_identifier(segment, "cell-binding path segment").is_err())
    {
        return Err(format!("Invalid Verilog-A cell binding '{value}'."));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ProjectSourceDependency, ProjectSourceFile, ProjectSourceOwner, ProjectSourceRoleBinding,
    };

    #[test]
    fn persisted_profile_is_selected_by_role_not_filename() {
        let profile = VerilogABuildProfile::starter("device").to_toml().unwrap();
        let bundle = ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "models/device.va",
            "module device; endmodule\n",
            [ProjectSourceFile::try_new("config/release.data", profile).unwrap()],
            [
                ProjectSourceDependency::try_new("models/device.va", "config/release.data")
                    .unwrap(),
            ],
            [ProjectSourceRoleBinding::try_new(
                "config/release.data",
                ProjectSourceRole::VerilogABuildProfile,
            )
            .unwrap()],
        )
        .unwrap();

        let resolved = resolve_veriloga_build_profile(&bundle).unwrap();
        assert_eq!(
            resolved.logical_path.as_deref(),
            Some("config/release.data")
        );
        assert_eq!(resolved.profile.entry_modules, ["device"]);
        assert!(!resolved.legacy_builtin);
    }

    #[test]
    fn legacy_profile_never_invents_an_entry_module_from_the_file_name() {
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "presentation_name.va",
            "module actual_model; endmodule\n",
            [],
            [],
        )
        .unwrap();

        let resolved = resolve_veriloga_build_profile(&bundle).unwrap();
        assert!(resolved.legacy_builtin);
        assert!(resolved.profile.entry_modules.is_empty());
        assert_eq!(resolved.profile.package.name, "presentation_name");
    }

    #[test]
    fn malformed_or_unsafe_profile_fails_closed() {
        let malformed = r#"
schema = "rspice.veriloga-build/v1"
entry_modules = ["device"]
include_paths = ["../ambient"]

[package]
name = "device"
version = "1.0.0"
"#;
        let error = VerilogABuildProfile::parse(malformed).unwrap_err();
        assert!(error.contains("Invalid portable Verilog-A include path"));
    }

    #[test]
    fn profile_round_trip_preserves_exact_policy() {
        let mut profile = VerilogABuildProfile::starter("device");
        profile.package.version = "2.3.4-rc.1".to_owned();
        profile.targets.generated_rust = true;
        profile.targets.fallback = VerilogAFallbackPolicy::Reject;
        profile
            .preprocessor
            .defines
            .insert("CORNER".to_owned(), "fast".to_owned());
        let encoded = profile.to_toml().unwrap();
        assert_eq!(VerilogABuildProfile::parse(&encoded).unwrap(), profile);
    }
}
