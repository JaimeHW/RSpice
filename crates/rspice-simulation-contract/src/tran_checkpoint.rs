//! Authored checkpoint cadence and immutable images selected for continuation.
use rspice_app_types::product::ContentDigest;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranCheckpointDraft {
    /// Absolute simulation-time cadence. None requests no new captures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// One unambiguous compatible image must cover each requested Run Set point.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resume: Vec<ContentDigest>,
}

impl Default for TranCheckpointDraft {
    fn default() -> Self {
        Self {
            interval: Some("10u".into()),
            resume: Vec::new(),
        }
    }
}

impl TranCheckpointDraft {
    pub fn capture_interval(&self) -> Result<Option<f64>, String> {
        self.interval
            .as_ref()
            .map(|text| {
                let interval = crate::spice_value::parse_spice_value_checked(text)?;
                if !interval.is_finite() || interval <= 0.0 {
                    return Err("Transient checkpoint interval must be finite and positive".into());
                }
                Ok(interval)
            })
            .transpose()
    }
}
