use super::*;

impl FftState {
    /// Get fundamental frequency
    pub fn fundamental_freq(&self) -> Option<f64> {
        self.analysis.as_ref()?.fundamental_frequency
    }

    /// Get THD
    pub fn thd_percent(&self) -> Option<f64> {
        self.analysis.as_ref()?.thd_percent
    }

    /// Get SFDR
    pub fn sfdr_db(&self) -> Option<f64> {
        self.analysis.as_ref()?.sfdr_db
    }

    /// Get SNR
    pub fn snr_db(&self) -> Option<f64> {
        self.analysis.as_ref()?.snr_db
    }
}
