use super::*;

mod capacitor;
mod inductor;
mod resistor;
mod transformer;

impl PropertyRegistry {
    pub(in super::super) fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
        self.register_transformer();
        self.register_coupled_inductor();
    }
}
