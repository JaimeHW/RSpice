use super::*;

mod current_bias;
mod current_transient;
mod ground;
mod shared;
mod voltage_bias;
mod voltage_transient;

impl PropertyRegistry {
    pub(in super::super) fn register_sources(&mut self) {
        self.register_vsource_dc();
        self.register_vsource_ac();
        self.register_vsource_pulse();
        self.register_vsource_sin();
        self.register_vsource_pwl();
        self.register_vsource_exp();
        self.register_vsource_sffm();

        self.register_isource_dc();
        self.register_isource_ac();
        self.register_isource_pulse();
        self.register_isource_sin();
        self.register_isource_pwl();
        self.register_isource_exp();
        self.register_isource_noise();

        self.register_ground();
    }
}
