//! The models feature observed through a whole application: the device-model
//! commands' selection authority over the session the shell builds, and the
//! cases whose outcome another part carries out (the schematic arming what
//! the Model Hub placed, the project history undoing a PDK transaction, the
//! shell answering what the Models workspace raised in a frame).

mod commands;
#[cfg(not(target_arch = "wasm32"))]
mod model_hub;
mod pdk_workflow;
mod studio;
