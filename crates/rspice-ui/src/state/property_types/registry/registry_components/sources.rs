//! Property sheets for independent and controlled sources, and for ground.

use super::*;

mod current_bias;
mod current_transient;
mod ground;
mod loop_probe;
mod rf_port;
mod shared;
mod shared_waveforms;
mod voltage_bias;
mod voltage_transient;

use shared_waveforms::Driven;

impl PropertyRegistry {
    pub(in super::super) fn register_sources(&mut self) {
        self.register_vsource_dc();
        self.register_vsource_ac();
        self.register_vsource_pulse();
        self.register_vsource_sin();
        self.register_vsource_pwl();
        self.register_vsource_exp();

        self.register_isource_dc();
        self.register_isource_ac();
        self.register_isource_pulse();
        self.register_isource_sin();
        self.register_isource_pwl();
        self.register_isource_exp();

        // Families whose two sheets are stamped from one builder, so the
        // voltage and current forms cannot drift apart.
        for (driven, sffm, am, pat, noise, pwl_file) in [
            (
                Driven::Voltage,
                ComponentType::VoltageSourceSffm,
                ComponentType::VoltageSourceAm,
                ComponentType::VoltageSourcePat,
                ComponentType::VoltageSourceNoise,
                ComponentType::VoltageSourcePwlFile,
            ),
            (
                Driven::Current,
                ComponentType::CurrentSourceSffm,
                ComponentType::CurrentSourceAm,
                ComponentType::CurrentSourcePat,
                ComponentType::CurrentSourceNoise,
                ComponentType::CurrentSourcePwlFile,
            ),
        ] {
            self.sheets.insert(sffm, Self::create_sffm_sheet(driven));
            self.sheets.insert(am, Self::create_am_sheet(driven));
            self.sheets.insert(pat, Self::create_pat_sheet(driven));
            self.sheets
                .insert(noise, Self::create_trnoise_sheet(driven));
            self.sheets
                .insert(pwl_file, Self::create_pwl_file_sheet(driven));
        }

        self.register_rf_port();
        self.register_loop_probe();
        self.register_ground();
    }
}
