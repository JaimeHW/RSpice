//! Reference-output coverage checks and validation-contract enforcement.

use super::*;

impl TestRunner {
    #[inline]
    pub(super) fn has_control_block(source: &str) -> bool {
        source.lines().any(|line| {
            Self::strip_netlist_comment(line)
                .trim()
                .eq_ignore_ascii_case(".control")
        })
    }

    #[inline]
    pub(super) fn source_has_test_gold_nodes(source: &str) -> bool {
        let normalized = source.to_ascii_lowercase();
        normalized.contains("_t") && normalized.contains("_g")
    }

    pub(super) fn has_reference_table_for_axis(
        &self,
        cir_path: &Path,
        axis_candidates: &[&str],
    ) -> Result<bool, String> {
        Ok(self
            .load_reference_table_for_axis(cir_path, axis_candidates)?
            .is_some())
    }

    pub(super) fn reference_table_unknown_voltage_nodes(
        &self,
        cir_path: &Path,
        table: &ReferenceTable,
    ) -> Result<Vec<String>, String> {
        let source_cir_path = self.authoritative_circuit_path(cir_path)?;
        let source = fs::read_to_string(&source_cir_path).map_err(|e| {
            format!(
                "Failed to read circuit '{}' while validating reference output: {e}",
                source_cir_path.display()
            )
        })?;
        let preprocessed =
            Netlist::preprocess_includes(&source, &source_cir_path).unwrap_or(source);
        let Ok(stripped) = Netlist::strip_control_blocks(&preprocessed) else {
            return Ok(Vec::new());
        };
        let Ok(netlist) = Self::parse_regression_source(&stripped, &source_cir_path) else {
            return Ok(Vec::new());
        };
        let Ok(circuit) = self.create_dynamic_engine().build_circuit(&netlist) else {
            return Ok(Vec::new());
        };

        let mut node_to_idx = HashMap::new();
        node_to_idx.insert("0".to_string(), 0usize);
        node_to_idx.insert("gnd".to_string(), 0usize);
        for (idx, name) in circuit.node_names_sorted().iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        let mut unknown = BTreeSet::new();
        for var in table.variables.keys() {
            let Some((node_pos, node_neg)) = Self::parse_voltage_probe(var) else {
                continue;
            };

            if !Self::reference_node_exists(&node_to_idx, &node_pos) {
                unknown.insert(node_pos);
            }
            if let Some(node_neg) = node_neg
                && !Self::reference_node_exists(&node_to_idx, &node_neg)
            {
                unknown.insert(node_neg);
            }
        }

        Ok(unknown.into_iter().collect())
    }

    pub fn has_valid_reference_output(&self, cir_path: &Path) -> bool {
        self.load_dc_op_reference(cir_path).ok().flatten().is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["time"])
                .ok()
                .flatten()
                .is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["frequency"])
                .ok()
                .flatten()
                .is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["v-sweep"])
                .ok()
                .flatten()
                .is_some()
            || self.load_pz_reference(cir_path).ok().flatten().is_some()
    }

    #[inline]
    pub(super) fn is_internal_device_op_probe(name: &str) -> bool {
        let normalized = name.trim().to_ascii_lowercase();
        normalized.contains('#') && !normalized.ends_with("#branch")
    }

    #[inline]
    pub(super) fn is_non_data_op_section_header(normalized: &str) -> bool {
        normalized.starts_with("resistor")
            || normalized.starts_with("capacitor")
            || normalized.starts_with("inductor")
            || normalized.starts_with("vsource")
            || normalized.starts_with("isource")
            || normalized.starts_with("diode")
            || normalized.starts_with("bjt")
            || normalized.starts_with("mosfet")
            || normalized.starts_with("jfet")
            || normalized.starts_with("mesfet")
            || normalized.starts_with("mesa")
            || normalized.starts_with("vbic")
            || normalized.starts_with("hfet")
            || normalized.starts_with("model ")
            || normalized.starts_with("device ")
            || normalized.starts_with("warning")
            || normalized.starts_with("circuit:")
            || normalized.starts_with("doing analysis")
            || normalized.starts_with("no. of data rows")
    }

    pub(super) fn analysis_name(analysis: &AnalysisSpec) -> &'static str {
        match analysis {
            AnalysisSpec::DcOp => "DC operating point",
            AnalysisSpec::DcSweep { .. } | AnalysisSpec::DcSweep2 { .. } => "DC sweep",
            AnalysisSpec::Transient { .. } => "transient analysis",
            AnalysisSpec::Ac { .. } => "AC analysis",
            AnalysisSpec::PoleZero { .. } => "pole-zero analysis",
            AnalysisSpec::Noise { .. } => "noise analysis",
            AnalysisSpec::Sensitivity { .. } => "sensitivity analysis",
            AnalysisSpec::TransferFunction { .. } => "transfer-function analysis",
            AnalysisSpec::Unsupported { .. } => "unsupported analysis",
        }
    }

    pub(super) fn analysis_has_direct_validation(
        &self,
        cir_path: &Path,
        original_source: &str,
        analysis: &AnalysisSpec,
    ) -> Result<bool, String> {
        Ok(match analysis {
            AnalysisSpec::DcOp => {
                Self::source_has_test_gold_nodes(original_source)
                    || self.load_dc_op_reference(cir_path)?.is_some()
            }
            AnalysisSpec::DcSweep { .. } | AnalysisSpec::DcSweep2 { .. } => {
                self.has_reference_table_for_axis(cir_path, &["v-sweep"])?
            }
            AnalysisSpec::Transient { .. } => {
                self.has_reference_table_for_axis(cir_path, &["time"])?
            }
            AnalysisSpec::Ac { .. } | AnalysisSpec::Noise { .. } => {
                self.has_reference_table_for_axis(cir_path, &["frequency"])?
            }
            AnalysisSpec::PoleZero { .. } => self.load_pz_reference(cir_path)?.is_some(),
            AnalysisSpec::Sensitivity { .. } => false,
            AnalysisSpec::TransferFunction {
                output,
                input_source,
            } => self
                .load_transfer_function_reference(cir_path, output, input_source)?
                .is_some(),
            AnalysisSpec::Unsupported { .. } => true,
        })
    }

    pub fn has_direct_validation_coverage(
        &self,
        cir_path: &Path,
        original_source: &str,
    ) -> Result<bool, String> {
        let analyses = self.parse_analyses(original_source);
        if analyses.is_empty() {
            return Ok(false);
        }

        for analysis in &analyses {
            if !self.analysis_has_direct_validation(cir_path, original_source, analysis)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub(super) fn enforce_validation_coverage(
        &self,
        cir_path: &Path,
        original_source: &str,
        analysis: &AnalysisSpec,
    ) -> Result<(), String> {
        let contract = self.validation_contract_for(cir_path);
        if matches!(contract, Some(ValidationContract::ScriptedControl))
            && !Self::has_control_block(original_source)
        {
            return Err(format!(
                "Validation manifest marks '{}' as scripted_control, but no .control block was found",
                cir_path.display()
            ));
        }

        let automatically_validated =
            self.analysis_has_direct_validation(cir_path, original_source, analysis)?;

        if automatically_validated || contract.is_some() {
            return Ok(());
        }

        Err(format!(
            "No validation oracle for {} in '{}'. Add _t/_g assertions, checked-in reference data, or an explicit validation-manifest entry.",
            Self::analysis_name(analysis),
            cir_path.display()
        ))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Analysis Execution
    // ═══════════════════════════════════════════════════════════════════════════
}
