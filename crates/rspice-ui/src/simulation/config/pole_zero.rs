// Pole-Zero Configuration
//=============================================================================

/// Pole-zero analysis type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PzAnalysisType {
    /// Find both poles and zeros
    #[default]
    PoleZero,
    /// Find poles only
    PolesOnly,
    /// Find zeros only
    ZerosOnly,
}

/// Pole-zero analysis configuration
#[derive(Debug, Clone)]
pub struct PoleZeroConfig {
    /// Input node
    pub input_node: String,
    /// Input reference node
    pub input_ref: String,
    /// Output node
    pub output_node: String,
    /// Output reference node
    pub output_ref: String,
    /// Transfer function type (VOL or CUR)
    pub transfer_type: String,
    /// Analysis type
    pub analysis_type: PzAnalysisType,
}

impl Default for PoleZeroConfig {
    fn default() -> Self {
        Self {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: PzAnalysisType::PoleZero,
        }
    }
}

impl PoleZeroConfig {
    /// Generate SPICE .pz command
    pub fn to_spice(&self) -> String {
        let pz_type = match self.analysis_type {
            PzAnalysisType::PoleZero => "PZ",
            PzAnalysisType::PolesOnly => "POL",
            PzAnalysisType::ZerosOnly => "ZER",
        };
        format!(
            ".pz {} {} {} {} {} {}",
            self.input_node,
            self.input_ref,
            self.output_node,
            self.output_ref,
            self.transfer_type,
            pz_type
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.input_node.is_empty() {
            errors.push("Input node is required".to_string());
        }
        if self.output_node.is_empty() {
            errors.push("Output node is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

//=============================================================================
