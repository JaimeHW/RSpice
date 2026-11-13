//! Transient analysis: timestep control, integration, checkpoints and aborts.

mod abort_iteration_bounds;
mod abort_reader;
mod advanced_abort;
mod behavioral_checkpoint;
mod checkpoint_encoding;
mod differential_startup;
mod dynamic_state_topology;
mod monte_carlo_checkpoint_identity;
mod timestep_sequence;
mod tran_uic;
mod transient_attempt_budget;
mod transient_compression_container;
mod transient_convergence_contract;
mod transient_exact_endpoint;
mod transient_integration_order;
mod transient_newlte;
mod transient_noise;
mod transient_passive_currents;
mod transient_transmission_line;
mod voltage_impulse_observations;
