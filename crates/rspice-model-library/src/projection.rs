//! Projection of parsed source into model catalogs and exact callable interfaces.

use crate::{
    DeviceModel, ModelLevel, ModelLibrary, ModelSubcircuitInterface, ModelType, ProcessCorner,
    subcircuit_interface_key,
};
use std::{collections::HashMap, path::Path};

impl DeviceModel {
    /// Project one parsed card with its exact file and optional section provenance.
    pub fn from_parsed(
        model: &rspice_core::library::ParsedModel,
        file_path: &Path,
        section: Option<&str>,
    ) -> DeviceModel {
        let model_type = ModelType::from_name(&model.spice_type);

        DeviceModel {
            name: model.name.clone(),
            section: section.map(str::to_owned),
            model_type,
            spice_type: Some(model.spice_type.clone()),
            level: ModelLevel::from_spice_card(model.level, &model.spice_type),
            spice_level: model.level,
            model_version: model.version,
            description: model.description.clone().unwrap_or_default(),
            l_min: model.lmin,
            l_max: model.lmax,
            w_min: model.wmin,
            w_max: model.wmax,
            vdd: None,
            vth0: None,
            file_path: Some(
                model
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            parameters: model.parameters.clone(),
            string_parameters: model.string_params.clone(),
            source_line: model.source_line,
        }
    }
}

impl ModelLevel {
    /// Classify what a model card *claims to be* for browsing/filtering.
    /// The card's type keyword wins over the LEVEL number because several
    /// LEVEL values are overloaded across device families (e.g. 8 is
    /// BSIM3v3 on a MOS card but HICUM/L2 on a BJT card, and 2002 is MVSG
    /// on MOS but DIODE_CMC on a diode). This is not a statement of native
    /// engine support.
    pub fn from_spice_card(level: Option<u32>, spice_type: &str) -> ModelLevel {
        let type_token = spice_type.trim().to_ascii_uppercase();

        // Family-named cards classify regardless of LEVEL.
        let by_name = match type_token.as_str() {
            t if t.starts_with("BSIMSOI") || t.starts_with("BSIM-SOI") => Some(ModelLevel::BsimSoi),
            t if t.starts_with("BSIMCMG") => Some(ModelLevel::BsimCmg),
            t if t.starts_with("BSIMBULK") => Some(ModelLevel::BsimBulk),
            t if t.starts_with("BSIMIMG") => Some(ModelLevel::BsimImg),
            t if t.starts_with("PSP") => Some(ModelLevel::Psp),
            t if t.starts_with("EKV") => Some(ModelLevel::Ekv),
            t if t.starts_with("HISIM") => Some(ModelLevel::HiSim),
            t if t.starts_with("L_UTSOI") || t.starts_with("LUTSOI") => Some(ModelLevel::LUtsoi),
            "MOSVAR" => Some(ModelLevel::Mosvar),
            t if t.starts_with("MVSG") => Some(ModelLevel::Mvsg),
            t if t.starts_with("VDMOS") || t == "NVDMOS" || t == "PVDMOS" => {
                Some(ModelLevel::Vdmos)
            }
            t if t.starts_with("VBIC") => Some(ModelLevel::Vbic),
            t if t.starts_with("MEXTRAM")
                || t.starts_with("BJT505")
                || t.starts_with("BJTD505") =>
            {
                Some(ModelLevel::Mextram)
            }
            t if t.starts_with("HICUM") => Some(ModelLevel::Hicum),
            t if t.starts_with("ASMHEMT")
                || t.starts_with("ANGELOV")
                || t.starts_with("EPFL_HEMT")
                || t.starts_with("EPFLHEMT") =>
            {
                Some(ModelLevel::Hemt)
            }
            "JUNCAP200" => Some(ModelLevel::Juncap),
            "DIODE_CMC" => Some(ModelLevel::DiodeCmc),
            t if t.starts_with("R2_CMC")
                || t.starts_with("R3_CMC")
                || t == "R2"
                || t == "R3"
                || t == "R2_ET" =>
            {
                Some(ModelLevel::RCmc)
            }
            _ => None,
        };
        if let Some(family) = by_name {
            return family;
        }

        let is_bjt = matches!(type_token.as_str(), "NPN" | "PNP" | "LPNP");
        let is_mos = matches!(type_token.as_str(), "NMOS" | "PMOS");
        let is_diode = matches!(type_token.as_str(), "D" | "DIODE");
        let is_resistor = matches!(type_token.as_str(), "R" | "RES" | "RESISTOR");

        match level {
            Some(4 | 9 | 11 | 12 | 13) if is_bjt => ModelLevel::Vbic,
            Some(8 | 230 | 234) if is_bjt => ModelLevel::Hicum,
            Some(504 | 505) if is_bjt => ModelLevel::Mextram,
            Some(200) if is_diode => ModelLevel::Juncap,
            Some(2002) if is_diode => ModelLevel::DiodeCmc,
            Some(1002 | 1003) if is_resistor => ModelLevel::RCmc,
            Some(1) => ModelLevel::SpiceLevel1,
            Some(3) => ModelLevel::SpiceLevel3,
            Some(8 | 49) if is_mos => ModelLevel::Bsim3v3,
            Some(14 | 54) if is_mos => ModelLevel::Bsim4,
            Some(10 | 55..=57 | 70470) if is_mos => ModelLevel::BsimSoi,
            Some(107 | 108 | 110 | 111) if is_mos => ModelLevel::BsimCmg,
            Some(104) if is_mos => ModelLevel::Psp,
            Some(260 | 301) if is_mos => ModelLevel::Ekv,
            Some(10240) if is_mos => ModelLevel::LUtsoi,
            Some(1000) if is_mos => ModelLevel::Mosvar,
            Some(2002) if is_mos => ModelLevel::Mvsg,
            Some(18) if is_mos => ModelLevel::Vdmos,
            // Preserve the historical level-only classification for cards
            // whose type keyword was not recognized.
            Some(8 | 49) => ModelLevel::Bsim3v3,
            Some(14 | 54) => ModelLevel::Bsim4,
            _ => ModelLevel::Unknown,
        }
    }
}

impl ModelSubcircuitInterface {
    /// Project the callable interface using the included source's own provenance.
    pub fn from_parsed(
        subcircuit: &rspice_core::library::ParsedSubcircuit,
        file_path: &Path,
        section: Option<&str>,
    ) -> Self {
        Self {
            name: subcircuit.name.clone(),
            ports: subcircuit.pins.clone(),
            parameter_defaults: subcircuit
                .parameter_defaults
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
            description: subcircuit.description.clone(),
            file_path: Some(
                subcircuit
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            source_line: subcircuit.source_line,
            section: section.map(str::to_owned),
        }
    }
}

impl ModelLibrary {
    /// Replace the parsed catalog while retaining the caller's source authority.
    /// The caller receives a complete value only after section, interface, and
    /// executable-content validation succeeds; failed imports cannot publish it.
    pub fn with_parsed_catalog(
        mut self,
        result: &rspice_core::library::LibParseResult,
        root: &Path,
        section: Option<&str>,
        diagnostic_name: &str,
    ) -> Result<Self, String> {
        self.models.clear();
        self.top_level_models.clear();
        self.section_models.clear();
        self.subcircuits.clear();
        self.model_definition_metadata.clear();
        self.model_qualification.clear();
        self.model_correlation.clear();
        self.corners.clear();
        self.selected_corner = None;

        for section_name in result.section_names() {
            // Build the corner through its section contract rather than by
            // field assignment: a corner with no section binding materializes
            // to nothing, so a bare name would seal an empty corner.
            let mut corner =
                ProcessCorner::from_composite_section(section_name, root.to_path_buf(), false);
            corner.description = format!("Process corner from {}", self.name);
            self.corners.insert(corner.name.clone(), corner);
        }

        let section_names = result.section_names();
        let selected_section = if let Some(section_name) = section {
            Some(section_name.to_owned())
        } else {
            section_names
                .iter()
                .find(|name| name.eq_ignore_ascii_case("tt"))
                .or_else(|| section_names.first())
                .map(|name| (*name).to_owned())
        };

        for model in &result.top_level_models {
            let device_model = DeviceModel::from_parsed(model, root, None);
            self.top_level_models
                .insert(device_model.name.clone(), device_model.clone());
            self.models.insert(device_model.name.clone(), device_model);
        }
        // Every section's interfaces are retained, not just the selected one:
        // a subcircuit is addressable by section-qualified identity, and a
        // library that declares only `.subckt` definitions is still a library.
        insert_parsed_subcircuits(&mut self, &result.top_level_subcircuits, root, None)?;
        for lib_section in &result.sections {
            insert_parsed_subcircuits(
                &mut self,
                &lib_section.subcircuits,
                root,
                Some(&lib_section.name),
            )?;
        }

        for lib_section in &result.sections {
            let section_models = self
                .section_models
                .entry(lib_section.name.clone())
                .or_default();
            for model in &lib_section.models {
                let device_model = DeviceModel::from_parsed(model, root, Some(&lib_section.name));
                section_models.insert(device_model.name.clone(), device_model);
            }
        }

        if let Some(section_name) = selected_section.as_deref() {
            if let Some(lib_section) = result.get_section(section_name) {
                self.selected_corner = Some(lib_section.name.clone());
                if let Some(corner) = self.corners.get_mut(&lib_section.name) {
                    corner.is_default = true;
                }
            } else {
                return Err(format!(
                    "Section '{}' not found. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        }
        self.refresh_effective_model_projection();
        if self.top_level_models.is_empty()
            && self.section_models.values().all(HashMap::is_empty)
            && self.subcircuits.is_empty()
            && !has_authenticated_veriloga_sources(result)?
        {
            return Err(format!(
                "Model library '{}' contains no supported device models, addressable subcircuits or authenticated Verilog-A/AMS sources",
                diagnostic_name
            ));
        }
        Ok(self)
    }
}

/// Project one parsed subcircuit onto its callable interface. A subcircuit
/// carries its own source file when it was reached through an include, so
/// that path wins over the root being scanned.
fn insert_parsed_subcircuits(
    library: &mut ModelLibrary,
    parsed: &[rspice_core::library::ParsedSubcircuit],
    file_path: &Path,
    section: Option<&str>,
) -> Result<(), String> {
    for subcircuit in parsed {
        let interface = ModelSubcircuitInterface::from_parsed(subcircuit, file_path, section);
        let key = subcircuit_interface_key(interface.section.as_deref(), &interface.name);
        if let Some(existing) = library
            .subcircuits
            .keys()
            .find(|existing| existing.eq_ignore_ascii_case(&key))
        {
            return Err(format!(
                "Subcircuit '{}' is defined more than once in the same library section (first identity '{}')",
                interface.name, existing
            ));
        }
        library.subcircuits.insert(key, interface);
    }
    Ok(())
}

/// Recognize HDL imports through the parser's captured resolution edges.
/// A suffix or an unrelated uploaded HDL file is not a library declaration.
fn has_authenticated_veriloga_sources(
    result: &rspice_core::library::LibParseResult,
) -> Result<bool, String> {
    for source in &result.resolved_sources {
        let projected =
            rspice_core::library::adapt_spectre_model_library(&source.path, &source.content)
                .map_err(|error| {
                    format!(
                        "{}:{}: {}",
                        source.path.display(),
                        error.line,
                        error.message
                    )
                })?;
        for line in projected.lines() {
            let Some(include) = rspice_core::netlist::parse_veriloga_source_directive(line) else {
                continue;
            };
            let requested = rspice_core::netlist::normalize_source_path_literal(
                &include.file_path.to_string_lossy(),
            )
            .map_err(|error| error.to_string())?;
            if result.resolved_dependencies.iter().any(|edge| {
                edge.owner == source.path
                    && rspice_core::netlist::normalize_source_path_literal(&edge.requested_path)
                        .is_ok_and(|path| path == requested)
                    && result
                        .resolved_sources
                        .iter()
                        .any(|target| target.path == edge.target)
            }) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
