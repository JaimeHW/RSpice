//! Signed PDK manifest schema and cross-reference validation.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use super::contracts::*;
use super::{PdkTechnologyError, validate_identifier, validate_text};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyManifest {
    pub schema_version: u32,
    pub package_id: String,
    pub technology_name: String,
    pub revision: String,
    pub publisher_id: String,
    pub signing_key_id: String,
    pub license_spdx: String,
    pub process_node_nm: u32,
    pub database_unit_meters: f64,
    pub stack_name: String,
    pub compatibility: PdkTechnologyCompatibility,
    /// Exact SPICE model roots and process-section selections supplied by
    /// this package. Model artifacts are executable only through this typed
    /// contract; an artifact label alone never grants simulator authority.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_sources: Vec<PdkModelProcessContract>,
    /// Exact Verilog-A roots compiled from signed, package-contained source
    /// closures. An artifact typed as Verilog-A source is never executable
    /// unless one of these contracts selects its module and netlist alias.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub veriloga_sources: Vec<PdkVerilogASourceContract>,
    /// Read-only symbol, pin, netlist, and typed parameter-form contracts
    /// supplied by this exact signed technology revision.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbol_definitions: Vec<crate::symbol::ModelBoundSymbolDefinition>,
    pub layers: Vec<PdkTechnologyLayer>,
    /// Portable alternate names for exact layer-purpose identities. Aliases
    /// are normalized case-insensitively and never replace the canonical
    /// layer or purpose stored in layout data.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layer_aliases: Vec<PdkLayerAlias>,
    pub stream_map: Vec<PdkStreamMapEntry>,
    pub connectivity: Vec<PdkConnectivityEdge>,
    /// Manufacturable via-generator limits for connectivity transitions.
    /// Legacy manifests may provide connectivity without generator geometry;
    /// consumers must not infer dimensions from a bare connectivity edge.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vias: Vec<PdkViaDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recognition: Vec<PdkRecognitionContract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extraction: Vec<PdkExtractionContract>,
    pub callbacks: Vec<PdkCallbackContract>,
    pub artifacts: Vec<PdkTechnologyArtifact>,
}

pub fn validate_manifest(manifest: &PdkTechnologyManifest) -> Result<(), PdkTechnologyError> {
    if !(MINIMUM_PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION..=PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION)
        .contains(&manifest.schema_version)
    {
        return Err(PdkTechnologyError::UnsupportedSchema {
            object: "manifest",
            actual: manifest.schema_version,
            supported: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
        });
    }
    validate_identifier("manifest.package_id", &manifest.package_id)?;
    validate_text("manifest.technology_name", &manifest.technology_name, 256)?;
    validate_version("manifest.revision", &manifest.revision)?;
    validate_identifier("manifest.publisher_id", &manifest.publisher_id)?;
    validate_identifier("manifest.signing_key_id", &manifest.signing_key_id)?;
    validate_text("manifest.license_spdx", &manifest.license_spdx, 128)?;
    validate_text("manifest.stack_name", &manifest.stack_name, 256)?;
    if manifest.process_node_nm == 0 || manifest.process_node_nm > 1_000_000 {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.process_node_nm must be in 1..=1000000".to_owned(),
        ));
    }
    if !manifest.database_unit_meters.is_finite()
        || !(1.0e-12..=1.0e-3).contains(&manifest.database_unit_meters)
    {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.database_unit_meters must be finite and in 1e-12..=1e-3".to_owned(),
        ));
    }
    validate_version(
        "manifest.compatibility.minimum_engine_version",
        &manifest.compatibility.minimum_engine_version,
    )?;
    validate_version(
        "manifest.compatibility.minimum_viewer_version",
        &manifest.compatibility.minimum_viewer_version,
    )?;
    let targets = manifest
        .compatibility
        .targets
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if targets.len() != manifest.compatibility.targets.len() {
        return Err(PdkTechnologyError::Duplicate(
            "manifest.compatibility.targets contains duplicates".to_owned(),
        ));
    }
    if targets.is_empty() {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.compatibility.targets must declare at least one permitted execution target"
                .to_owned(),
        ));
    }

    if manifest.layers.is_empty() || manifest.layers.len() > MAX_PDK_LAYERS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.layers must contain 1..={MAX_PDK_LAYERS} entries"
        )));
    }
    let mut layers = BTreeMap::<String, BTreeSet<String>>::new();
    let mut layer_kinds = BTreeMap::<String, PdkLayerKind>::new();
    let mut orders = BTreeSet::new();
    for (index, layer) in manifest.layers.iter().enumerate() {
        validate_identifier(&format!("manifest.layers[{index}].name"), &layer.name)?;
        validate_text(&format!("manifest.layers[{index}].role"), &layer.role, 256)?;
        if !orders.insert(layer.order) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layers[{index}].order {} is repeated",
                layer.order
            )));
        }
        if layer.purposes.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.layers[{index}].purposes is empty"
            )));
        }
        let key = layer.name.to_ascii_lowercase();
        if layers.contains_key(&key) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layers repeats case-insensitive layer '{}'",
                layer.name
            )));
        }
        let mut purposes = BTreeSet::new();
        for (purpose_index, purpose) in layer.purposes.iter().enumerate() {
            validate_identifier(
                &format!("manifest.layers[{index}].purposes[{purpose_index}]"),
                purpose,
            )?;
            if !purposes.insert(purpose.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.layers[{index}] repeats purpose '{purpose}'"
                )));
            }
        }
        layer_kinds.insert(key.clone(), layer.kind);
        layers.insert(key, purposes);
    }

    if manifest.schema_version < 5
        && (!manifest.layer_aliases.is_empty() || !manifest.vias.is_empty())
    {
        return Err(PdkTechnologyError::InvalidField(
            "layer aliases and via definitions require manifest schema 5 or newer".to_owned(),
        ));
    }
    if manifest.layer_aliases.len() > MAX_PDK_LAYER_ALIASES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.layer_aliases exceeds {MAX_PDK_LAYER_ALIASES} entries"
        )));
    }
    let mut layer_aliases = BTreeSet::new();
    for (index, alias) in manifest.layer_aliases.iter().enumerate() {
        validate_identifier(
            &format!("manifest.layer_aliases[{index}].alias"),
            &alias.alias,
        )?;
        validate_layer_purpose_reference(
            &format!("manifest.layer_aliases[{index}]"),
            &PdkLayerPurposeRef {
                layer: alias.layer.clone(),
                purpose: alias.purpose.clone(),
            },
            &layers,
        )?;
        let identity = alias.alias.to_ascii_lowercase();
        if layers.contains_key(&identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layer_aliases[{index}] alias '{}' collides with a canonical layer name",
                alias.alias
            )));
        }
        if !layer_aliases.insert(identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layer_aliases repeats case-insensitive alias '{}'",
                alias.alias
            )));
        }
    }

    if manifest.stream_map.is_empty() || manifest.stream_map.len() > MAX_PDK_STREAM_MAP_ENTRIES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.stream_map must contain 1..={MAX_PDK_STREAM_MAP_ENTRIES} entries"
        )));
    }
    let mut logical_mappings = BTreeSet::new();
    let mut stream_mappings = BTreeSet::new();
    for (index, mapping) in manifest.stream_map.iter().enumerate() {
        let layer = mapping.layer.to_ascii_lowercase();
        let purpose = mapping.purpose.to_ascii_lowercase();
        let Some(purposes) = layers.get(&layer) else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.stream_map[{index}].layer '{}' is not declared",
                mapping.layer
            )));
        };
        if !purposes.contains(&purpose) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.stream_map[{index}] references undeclared purpose '{}:{}'",
                mapping.layer, mapping.purpose
            )));
        }
        if !logical_mappings.insert((layer, purpose)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.stream_map[{index}] repeats a logical layer/purpose"
            )));
        }
        if !stream_mappings.insert((mapping.stream_layer, mapping.stream_datatype)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.stream_map[{index}] repeats stream pair {}/{}",
                mapping.stream_layer, mapping.stream_datatype
            )));
        }
    }
    for (layer, purposes) in &layers {
        for purpose in purposes {
            if !logical_mappings.contains(&(layer.clone(), purpose.clone())) {
                return Err(PdkTechnologyError::MissingMapping(format!(
                    "{layer}:{purpose}"
                )));
            }
        }
    }

    if manifest.connectivity.len() > MAX_PDK_CONNECTIVITY_EDGES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.connectivity exceeds {MAX_PDK_CONNECTIVITY_EDGES} edges"
        )));
    }
    let mut edges = BTreeSet::new();
    for (index, edge) in manifest.connectivity.iter().enumerate() {
        let from = edge.from_layer.to_ascii_lowercase();
        let through = edge.through_layer.to_ascii_lowercase();
        let to = edge.to_layer.to_ascii_lowercase();
        for (field, value) in [
            ("from_layer", &from),
            ("through_layer", &through),
            ("to_layer", &to),
        ] {
            if !layers.contains_key(value) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.connectivity[{index}].{field} '{value}' is not declared"
                )));
            }
        }
        if from == through || through == to || from == to {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.connectivity[{index}] must reference three distinct layers"
            )));
        }
        if !matches!(
            layer_kinds.get(&through),
            Some(PdkLayerKind::Via | PdkLayerKind::Cut)
        ) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.connectivity[{index}].through_layer '{}' is not typed as a via or cut layer",
                edge.through_layer
            )));
        }
        for (field, identity, display) in [
            ("from_layer", &from, &edge.from_layer),
            ("to_layer", &to, &edge.to_layer),
        ] {
            if matches!(
                layer_kinds.get(identity),
                Some(PdkLayerKind::Via | PdkLayerKind::Cut | PdkLayerKind::Marker)
            ) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.connectivity[{index}].{field} '{display}' is not a conductor layer"
                )));
            }
        }
        if !edges.insert((from, through, to)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.connectivity[{index}] repeats an edge"
            )));
        }
    }

    if manifest.vias.len() > MAX_PDK_VIA_DEFINITIONS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.vias exceeds {MAX_PDK_VIA_DEFINITIONS} entries"
        )));
    }
    let mut via_ids = BTreeSet::new();
    let mut via_transitions = BTreeSet::new();
    for (index, via) in manifest.vias.iter().enumerate() {
        validate_identifier(&format!("manifest.vias[{index}].via_id"), &via.via_id)?;
        if !via_ids.insert(via.via_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.vias repeats case-insensitive via ID '{}'",
                via.via_id
            )));
        }
        let lower = via.lower_layer.to_ascii_lowercase();
        let cut = via.cut_layer.to_ascii_lowercase();
        let upper = via.upper_layer.to_ascii_lowercase();
        for (field, value) in [
            ("lower_layer", &lower),
            ("cut_layer", &cut),
            ("upper_layer", &upper),
        ] {
            if !layers.contains_key(value) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.vias[{index}].{field} '{value}' is not declared"
                )));
            }
        }
        if lower == cut || cut == upper || lower == upper {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}] must reference three distinct layers"
            )));
        }
        if !matches!(
            layer_kinds.get(&cut),
            Some(PdkLayerKind::Via | PdkLayerKind::Cut)
        ) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.vias[{index}].cut_layer '{}' is not typed as a via or cut layer",
                via.cut_layer
            )));
        }
        for (field, identity, display) in [
            ("lower_layer", &lower, &via.lower_layer),
            ("upper_layer", &upper, &via.upper_layer),
        ] {
            if matches!(
                layer_kinds.get(identity),
                Some(PdkLayerKind::Via | PdkLayerKind::Cut | PdkLayerKind::Marker)
            ) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.vias[{index}].{field} '{display}' is not a conductor layer"
                )));
            }
        }
        if !edges.contains(&(lower.clone(), cut.clone(), upper.clone())) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.vias[{index}] has no matching connectivity edge '{} -> {} -> {}'",
                via.lower_layer, via.cut_layer, via.upper_layer
            )));
        }
        if !via_transitions.insert((lower, cut, upper)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.vias[{index}] repeats a layer transition"
            )));
        }
        for (field, value) in [
            ("cut_width_meters", via.cut_width_meters),
            ("cut_height_meters", via.cut_height_meters),
            ("lower_enclosure_meters", via.lower_enclosure_meters),
            ("upper_enclosure_meters", via.upper_enclosure_meters),
        ] {
            if !value.is_finite() || !(1.0e-12..=1.0e-3).contains(&value) {
                return Err(PdkTechnologyError::InvalidField(format!(
                    "manifest.vias[{index}].{field} must be finite and in 1e-12..=1e-3"
                )));
            }
        }
        if via.maximum_rows == 0 || via.maximum_columns == 0 {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}] maximum rows and columns must be nonzero"
            )));
        }
        if via
            .maximum_rms_current_per_cut_amperes
            .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}].maximum_rms_current_per_cut_amperes must be finite and positive"
            )));
        }
    }

    if manifest.artifacts.is_empty() || manifest.artifacts.len() > MAX_PDK_ARTIFACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.artifacts must contain 1..={MAX_PDK_ARTIFACTS} entries"
        )));
    }
    let mut artifact_paths = BTreeMap::new();
    for (index, artifact) in manifest.artifacts.iter().enumerate() {
        validate_package_path(&format!("manifest.artifacts[{index}].path"), &artifact.path)?;
        if artifact.size_bytes > u64::try_from(MAX_PDK_ARTIFACT_BYTES).unwrap_or(u64::MAX) {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.artifacts[{index}] exceeds {MAX_PDK_ARTIFACT_BYTES} bytes"
            )));
        }
        if artifact_paths
            .insert(
                artifact.path.to_ascii_lowercase(),
                (artifact.path.as_str(), artifact.kind),
            )
            .is_some()
        {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.artifacts repeats case-insensitive path '{}'",
                artifact.path
            )));
        }
    }

    if manifest.model_sources.len() > PdkModelProcess::ALL.len() {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.model_sources exceeds {MAX_PDK_MODEL_PROCESS_CONTRACTS} process contracts"
        )));
    }
    let model_artifact_count = artifact_paths
        .values()
        .filter(|(_, kind)| *kind == PdkTechnologyArtifactKind::Model)
        .count();
    if model_artifact_count == 0 && !manifest.model_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "manifest.model_sources declares executable roots but the package has no model artifacts"
                .to_owned(),
        ));
    }
    if model_artifact_count > 0 && manifest.model_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "model artifacts require typed manifest.model_sources process contracts".to_owned(),
        ));
    }
    let mut process_contracts = BTreeSet::new();
    let mut total_model_sources = 0usize;
    for (process_index, contract) in manifest.model_sources.iter().enumerate() {
        if !process_contracts.insert(contract.process) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.model_sources repeats {}",
                contract.process.keyword()
            )));
        }
        if contract.sources.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.model_sources[{process_index}].sources is empty"
            )));
        }
        total_model_sources = total_model_sources
            .checked_add(contract.sources.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("model section source count overflow".to_owned())
            })?;
        if total_model_sources > MAX_PDK_MODEL_SECTION_SOURCES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.model_sources contains more than {MAX_PDK_MODEL_SECTION_SOURCES} section sources"
            )));
        }
        let mut source_ids = BTreeSet::new();
        let mut source_selections = BTreeSet::new();
        let mut supplied_domains = BTreeSet::new();
        for (source_index, source) in contract.sources.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.model_sources[{process_index}].sources[{source_index}].source_id"
                ),
                &source.source_id,
            )?;
            if !source_ids.insert(source.source_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats source ID '{}'",
                    source.source_id
                )));
            }
            validate_package_path(
                &format!(
                    "manifest.model_sources[{process_index}].sources[{source_index}].artifact_path"
                ),
                &source.artifact_path,
            )?;
            let Some((_, kind)) = artifact_paths.get(&source.artifact_path.to_ascii_lowercase())
            else {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] source '{}' references missing artifact '{}'",
                    source.source_id, source.artifact_path
                )));
            };
            if *kind != PdkTechnologyArtifactKind::Model {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] source '{}' references an artifact not typed as model",
                    source.source_id
                )));
            }
            if let Some(section) = source.section.as_deref() {
                validate_model_section_name(
                    &format!(
                        "manifest.model_sources[{process_index}].sources[{source_index}].section"
                    ),
                    section,
                )?;
            }
            let selection = (
                source.artifact_path.to_ascii_lowercase(),
                source.section.as_deref().map(str::to_ascii_lowercase),
            );
            if !source_selections.insert(selection) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats artifact/section selection '{}{}'",
                    source.artifact_path,
                    source
                        .section
                        .as_deref()
                        .map(|section| format!(" [{section}]"))
                        .unwrap_or_default()
                )));
            }
            supplied_domains.insert(source.domain);
        }
        if contract.required_domains.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.model_sources[{process_index}].required_domains is empty"
            )));
        }
        let mut required_domains = BTreeSet::new();
        for domain in &contract.required_domains {
            if !required_domains.insert(*domain) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats required domain {domain:?}"
                )));
            }
            if !supplied_domains.contains(domain) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] requires {domain:?} but supplies no matching source"
                )));
            }
        }
    }
    if model_artifact_count > 0 && !process_contracts.contains(&PdkModelProcess::Tt) {
        return Err(PdkTechnologyError::InvalidReference(
            "model-source packages must explicitly supply the TT reference process".to_owned(),
        ));
    }

    if manifest.schema_version < 4 && !manifest.symbol_definitions.is_empty() {
        return Err(PdkTechnologyError::InvalidField(
            "signed symbol definitions require manifest schema 4 or newer".to_owned(),
        ));
    }
    if manifest.symbol_definitions.len() > MAX_PDK_SYMBOL_DEFINITIONS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.symbol_definitions exceeds {MAX_PDK_SYMBOL_DEFINITIONS} definitions"
        )));
    }
    let model_artifact_paths = artifact_paths
        .iter()
        .filter_map(|(path, (_, kind))| {
            (*kind == PdkTechnologyArtifactKind::Model).then_some(path.as_str())
        })
        .collect::<BTreeSet<_>>();
    let mut symbol_providers_by_artifact = BTreeMap::<String, BTreeSet<String>>::new();
    for contract in &manifest.model_sources {
        for source in &contract.sources {
            symbol_providers_by_artifact
                .entry(source.artifact_path.to_ascii_lowercase())
                .or_default()
                .insert(format!("signed-pdk:{}", source.source_id));
        }
    }
    let mut symbol_identities = BTreeSet::new();
    for (index, definition) in manifest.symbol_definitions.iter().enumerate() {
        if !definition
            .identity
            .library
            .eq_ignore_ascii_case(&manifest.package_id)
        {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] identity library must equal package ID '{}'",
                manifest.package_id
            )));
        }
        let identity = (
            definition.identity.library.to_ascii_lowercase(),
            definition.identity.cell.to_ascii_lowercase(),
        );
        if !symbol_identities.insert(identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.symbol_definitions repeats case-insensitive identity '{}/{}'",
                definition.identity.library, definition.identity.cell
            )));
        }
        let crate::symbol::SymbolSourceContract::Model { model, .. } = &definition.source else {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] must use an executable model source contract"
            )));
        };
        if definition.netlist.model.as_ref() != Some(model) {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] source and netlist model identities differ"
            )));
        }
        if model.implementation_view != crate::symbol::SymbolImplementationView::Spice {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] must bind a signed SPICE model source; use veriloga_sources for Verilog-A authority"
            )));
        }
        let source_path = model.source_path.as_deref().ok_or_else(|| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] has no model source path"
            ))
        })?;
        validate_package_path(
            &format!("manifest.symbol_definitions[{index}].source_path"),
            source_path,
        )?;
        if !model_artifact_paths.contains(source_path.to_ascii_lowercase().as_str()) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.symbol_definitions[{index}] source '{}' is not a model artifact in this package",
                source_path
            )));
        }
        let providers = symbol_providers_by_artifact
            .get(&source_path.to_ascii_lowercase())
            .ok_or_else(|| {
                PdkTechnologyError::InvalidReference(format!(
                    "manifest.symbol_definitions[{index}] source '{}' is not reachable from a model-source contract",
                    source_path
                ))
            })?;
        if !providers
            .iter()
            .any(|provider| provider.eq_ignore_ascii_case(&model.library))
        {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.symbol_definitions[{index}] model library '{}' must name one signed provider for '{}': {}",
                model.library,
                source_path,
                providers.iter().cloned().collect::<Vec<_>>().join(", ")
            )));
        }
        if model.revision.as_deref() != Some(manifest.revision.as_str()) {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] model revision must equal signed package revision '{}'",
                manifest.revision
            )));
        }
        let mut executable = definition.clone();
        let virtual_source = signed_model_virtual_root("manifest-validation")
            .join(package_path_to_host_path(source_path))
            .to_string_lossy()
            .into_owned();
        let crate::symbol::SymbolSourceContract::Model { model, .. } = &mut executable.source
        else {
            unreachable!("model source was required above")
        };
        model.source_path = Some(virtual_source.clone());
        executable
            .netlist
            .model
            .as_mut()
            .expect("matching netlist model was required above")
            .source_path = Some(virtual_source);
        executable.validate().map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is invalid: {error}"
            ))
        })?;
    }

    let veriloga_artifact_count = artifact_paths
        .values()
        .filter(|(_, kind)| *kind == PdkTechnologyArtifactKind::VerilogASource)
        .count();
    if manifest.schema_version < 2
        && (!manifest.veriloga_sources.is_empty() || veriloga_artifact_count > 0)
    {
        return Err(PdkTechnologyError::InvalidField(
            "signed Verilog-A artifacts require manifest schema 2 or newer".to_owned(),
        ));
    }
    if manifest.veriloga_sources.len() > MAX_PDK_VERILOGA_SOURCE_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.veriloga_sources exceeds {MAX_PDK_VERILOGA_SOURCE_CONTRACTS} contracts"
        )));
    }
    if veriloga_artifact_count == 0 && !manifest.veriloga_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "manifest.veriloga_sources declares executable roots but the package has no Verilog-A source artifacts"
                .to_owned(),
        ));
    }
    if veriloga_artifact_count > 0 && manifest.veriloga_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "Verilog-A source artifacts require typed manifest.veriloga_sources contracts"
                .to_owned(),
        ));
    }
    let mut veriloga_source_ids = BTreeSet::new();
    let mut veriloga_aliases = BTreeSet::new();
    let mut veriloga_selections = BTreeSet::new();
    for (index, contract) in manifest.veriloga_sources.iter().enumerate() {
        validate_identifier(
            &format!("manifest.veriloga_sources[{index}].source_id"),
            &contract.source_id,
        )?;
        if !veriloga_source_ids.insert(contract.source_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats source ID '{}'",
                contract.source_id
            )));
        }
        validate_package_path(
            &format!("manifest.veriloga_sources[{index}].root_artifact_path"),
            &contract.root_artifact_path,
        )?;
        let Some((_, kind)) = artifact_paths.get(&contract.root_artifact_path.to_ascii_lowercase())
        else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.veriloga_sources[{index}] references missing artifact '{}'",
                contract.root_artifact_path
            )));
        };
        if *kind != PdkTechnologyArtifactKind::VerilogASource {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.veriloga_sources[{index}] root '{}' is not typed as Verilog-A source",
                contract.root_artifact_path
            )));
        }
        validate_veriloga_identifier(
            &format!("manifest.veriloga_sources[{index}].module_name"),
            &contract.module_name,
        )?;
        validate_veriloga_identifier(
            &format!("manifest.veriloga_sources[{index}].netlist_alias"),
            &contract.netlist_alias,
        )?;
        if !veriloga_aliases.insert(contract.netlist_alias.to_ascii_uppercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats case-insensitive netlist alias '{}'",
                contract.netlist_alias
            )));
        }
        let selection = (
            contract.root_artifact_path.to_ascii_lowercase(),
            contract.module_name.to_ascii_lowercase(),
        );
        if !veriloga_selections.insert(selection) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats root/module selection '{}' / '{}'",
                contract.root_artifact_path, contract.module_name
            )));
        }
    }

    if manifest.recognition.len() > MAX_PDK_RECOGNITION_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.recognition exceeds {MAX_PDK_RECOGNITION_CONTRACTS} contracts"
        )));
    }
    if manifest.extraction.len() > MAX_PDK_EXTRACTION_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.extraction exceeds {MAX_PDK_EXTRACTION_CONTRACTS} contracts"
        )));
    }
    let mut recognition_ids = BTreeSet::new();
    let mut extraction_ids = BTreeSet::new();
    let mut vector_ids = BTreeSet::new();
    let mut referenced_special_artifacts = BTreeSet::new();
    let mut vector_count = 0usize;
    for (index, contract) in manifest.recognition.iter().enumerate() {
        validate_identifier(
            &format!("manifest.recognition[{index}].contract_id"),
            &contract.contract_id,
        )?;
        validate_identifier(
            &format!("manifest.recognition[{index}].device_class"),
            &contract.device_class,
        )?;
        if !recognition_ids.insert(contract.contract_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.recognition repeats contract '{}'",
                contract.contract_id
            )));
        }
        require_special_artifact(
            &format!("manifest.recognition[{index}].rule_artifact_path"),
            &contract.rule_artifact_path,
            PdkTechnologyArtifactKind::RecognitionMap,
            &artifact_paths,
            &mut referenced_special_artifacts,
        )?;
        if contract.terminals.is_empty() || contract.terminals.len() > MAX_PDK_RECOGNITION_TERMINALS
        {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.recognition[{index}].terminals must contain 1..={MAX_PDK_RECOGNITION_TERMINALS} entries"
            )));
        }
        let mut terminals = BTreeSet::new();
        for (terminal_index, terminal) in contract.terminals.iter().enumerate() {
            validate_identifier(
                &format!("manifest.recognition[{index}].terminals[{terminal_index}].terminal_name"),
                &terminal.terminal_name,
            )?;
            if !terminals.insert(terminal.terminal_name.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.recognition[{index}] repeats terminal '{}'",
                    terminal.terminal_name
                )));
            }
            validate_layer_purpose_reference(
                &format!("manifest.recognition[{index}].terminals[{terminal_index}]"),
                &PdkLayerPurposeRef {
                    layer: terminal.layer.clone(),
                    purpose: terminal.purpose.clone(),
                },
                &layers,
            )?;
        }
        if contract.qualification_vectors.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.recognition[{index}].qualification_vectors is empty"
            )));
        }
        vector_count = vector_count
            .checked_add(contract.qualification_vectors.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("qualification vector count overflow".to_owned())
            })?;
        for (vector_index, vector) in contract.qualification_vectors.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].vector_id"
                ),
                &vector.vector_id,
            )?;
            if !vector_ids.insert(vector.vector_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest qualification vectors repeat '{}'",
                    vector.vector_id
                )));
            }
            if vector.expected_instance_count > 1_000_000 {
                return Err(PdkTechnologyError::LimitExceeded(format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].expected_instance_count exceeds 1000000"
                )));
            }
            require_special_artifact(
                &format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].layout_artifact_path"
                ),
                &vector.layout_artifact_path,
                PdkTechnologyArtifactKind::QualificationVector,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
        }
    }
    for (index, contract) in manifest.extraction.iter().enumerate() {
        validate_identifier(
            &format!("manifest.extraction[{index}].contract_id"),
            &contract.contract_id,
        )?;
        if !extraction_ids.insert(contract.contract_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.extraction repeats contract '{}'",
                contract.contract_id
            )));
        }
        require_special_artifact(
            &format!("manifest.extraction[{index}].rule_artifact_path"),
            &contract.rule_artifact_path,
            PdkTechnologyArtifactKind::ExtractionRule,
            &artifact_paths,
            &mut referenced_special_artifacts,
        )?;
        if contract.quantities.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.extraction[{index}].quantities is empty"
            )));
        }
        let quantities = contract.quantities.iter().copied().collect::<BTreeSet<_>>();
        if quantities.len() != contract.quantities.len() {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.extraction[{index}].quantities contains duplicates"
            )));
        }
        if contract.layer_purposes.is_empty() || contract.layer_purposes.len() > MAX_PDK_LAYERS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.extraction[{index}].layer_purposes must contain 1..={MAX_PDK_LAYERS} entries"
            )));
        }
        let mut layer_purposes = BTreeSet::new();
        for (reference_index, reference) in contract.layer_purposes.iter().enumerate() {
            validate_layer_purpose_reference(
                &format!("manifest.extraction[{index}].layer_purposes[{reference_index}]"),
                reference,
                &layers,
            )?;
            if !layer_purposes.insert((
                reference.layer.to_ascii_lowercase(),
                reference.purpose.to_ascii_lowercase(),
            )) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.extraction[{index}] repeats layer/purpose '{}:{}'",
                    reference.layer, reference.purpose
                )));
            }
        }
        if contract.qualification_vectors.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.extraction[{index}].qualification_vectors is empty"
            )));
        }
        vector_count = vector_count
            .checked_add(contract.qualification_vectors.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("qualification vector count overflow".to_owned())
            })?;
        for (vector_index, vector) in contract.qualification_vectors.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].vector_id"
                ),
                &vector.vector_id,
            )?;
            if !vector_ids.insert(vector.vector_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest qualification vectors repeat '{}'",
                    vector.vector_id
                )));
            }
            require_special_artifact(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].layout_artifact_path"
                ),
                &vector.layout_artifact_path,
                PdkTechnologyArtifactKind::QualificationVector,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
            require_special_artifact(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].reference_artifact_path"
                ),
                &vector.reference_artifact_path,
                PdkTechnologyArtifactKind::QualificationReference,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
        }
    }
    if vector_count > MAX_PDK_QUALIFICATION_VECTORS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest recognition and extraction contracts exceed {MAX_PDK_QUALIFICATION_VECTORS} qualification vectors"
        )));
    }
    for (path, (_, kind)) in &artifact_paths {
        if matches!(
            kind,
            PdkTechnologyArtifactKind::RecognitionMap
                | PdkTechnologyArtifactKind::ExtractionRule
                | PdkTechnologyArtifactKind::QualificationVector
                | PdkTechnologyArtifactKind::QualificationReference
        ) && !referenced_special_artifacts.contains(path)
        {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "specialized artifact '{path}' is not owned by a recognition or extraction contract"
            )));
        }
    }

    let mut callback_ids = BTreeSet::new();
    if manifest.callbacks.len() > MAX_PDK_CALLBACK_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.callbacks exceeds {MAX_PDK_CALLBACK_CONTRACTS} contracts"
        )));
    }
    if !manifest.callbacks.is_empty() && manifest.schema_version < 3 {
        return Err(PdkTechnologyError::InvalidField(
            "executable callbacks require manifest schema 3 or newer".to_owned(),
        ));
    }
    for (index, callback) in manifest.callbacks.iter().enumerate() {
        validate_identifier(
            &format!("manifest.callbacks[{index}].callback_id"),
            &callback.callback_id,
        )?;
        validate_package_path(
            &format!("manifest.callbacks[{index}].artifact_path"),
            &callback.artifact_path,
        )?;
        if callback.abi_version != PDK_CALLBACK_ABI_VERSION {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.callbacks[{index}].abi_version is {}; supported ABI is {PDK_CALLBACK_ABI_VERSION}",
                callback.abi_version
            )));
        }
        validate_identifier(
            &format!("manifest.callbacks[{index}].entrypoint"),
            &callback.entrypoint,
        )?;
        if !callback_ids.insert(callback.callback_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.callbacks repeats callback '{}'",
                callback.callback_id
            )));
        }
        let Some((_, kind)) = artifact_paths.get(&callback.artifact_path.to_ascii_lowercase())
        else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.callbacks[{index}] references missing artifact '{}'",
                callback.artifact_path
            )));
        };
        if *kind != PdkTechnologyArtifactKind::Callback {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.callbacks[{index}] artifact is not typed as callback"
            )));
        }
        let artifact = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.path.eq_ignore_ascii_case(&callback.artifact_path))
            .expect("callback artifact path was resolved above");
        if usize::try_from(artifact.size_bytes)
            .ok()
            .is_none_or(|size| size > MAX_PDK_CALLBACK_ARTIFACT_BYTES)
        {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.callbacks[{index}] artifact '{}' exceeds {MAX_PDK_CALLBACK_ARTIFACT_BYTES} bytes",
                callback.artifact_path
            )));
        }
        let capabilities = callback
            .capabilities
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if capabilities.len() != callback.capabilities.len() {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.callbacks[{index}].capabilities contains duplicates"
            )));
        }
        if capabilities.contains(&PdkCallbackCapability::Network) {
            return Err(PdkTechnologyError::ForbiddenCapability(format!(
                "manifest.callbacks[{index}] requests network access"
            )));
        }
    }
    Ok(())
}

fn validate_layer_purpose_reference(
    path: &str,
    reference: &PdkLayerPurposeRef,
    layers: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), PdkTechnologyError> {
    validate_identifier(&format!("{path}.layer"), &reference.layer)?;
    validate_identifier(&format!("{path}.purpose"), &reference.purpose)?;
    let Some(purposes) = layers.get(&reference.layer.to_ascii_lowercase()) else {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path}.layer '{}' is not declared",
            reference.layer
        )));
    };
    if !purposes.contains(&reference.purpose.to_ascii_lowercase()) {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} references undeclared purpose '{}:{}'",
            reference.layer, reference.purpose
        )));
    }
    Ok(())
}

fn require_special_artifact(
    path: &str,
    artifact_path: &str,
    required_kind: PdkTechnologyArtifactKind,
    artifacts: &BTreeMap<String, (&str, PdkTechnologyArtifactKind)>,
    referenced: &mut BTreeSet<String>,
) -> Result<(), PdkTechnologyError> {
    validate_package_path(path, artifact_path)?;
    let identity = artifact_path.to_ascii_lowercase();
    let Some((_, actual_kind)) = artifacts.get(&identity) else {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} references missing artifact '{artifact_path}'"
        )));
    };
    if *actual_kind != required_kind {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} artifact '{artifact_path}' has type {actual_kind:?}, expected {required_kind:?}"
        )));
    }
    if !referenced.insert(identity) {
        return Err(PdkTechnologyError::Duplicate(format!(
            "specialized artifact '{artifact_path}' is referenced more than once"
        )));
    }
    Ok(())
}

pub fn validate_version(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 64)?;
    semver::Version::parse(value).map(|_| ()).map_err(|error| {
        PdkTechnologyError::InvalidField(format!("{path} must be a semantic version: {error}"))
    })
}

pub fn validate_package_path(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 1_024)?;
    if value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.contains(':')
        || value
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must be a normalized relative forward-slash path"
        )));
    }
    Ok(())
}

fn validate_model_section_name(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 256)?;
    if value.chars().any(|character| {
        character.is_whitespace() || character == '"' || character == '\'' || character.is_control()
    }) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} contains whitespace, a quote, or a control character"
        )));
    }
    Ok(())
}

fn validate_veriloga_identifier(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 128)?;
    let mut characters = value.chars();
    if !characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        || !characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must be a portable Verilog-A/SPICE identifier"
        )));
    }
    Ok(())
}

pub fn package_path_to_host_path(path: &str) -> PathBuf {
    path.split('/').collect()
}

pub fn signed_model_virtual_root(identity: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(format!(r"C:\rspice-pdk\model-sources\{identity}"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from(format!("/rspice-pdk/model-sources/{identity}"))
    }
}
