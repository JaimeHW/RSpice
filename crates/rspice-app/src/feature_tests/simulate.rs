//! The simulate feature observed through a whole application: the cases
//! that drive a frame or the shell, or that read what another part of the
//! application carries out for a simulate command (the notice centre
//! draining what a plan command announced), or that read the whole
//! application's state (a project snapshot, a session round trip) or reach
//! an action the application ranks above the feature (tuning staging).

mod checkpoint_exchange;
mod clicks;
mod excitations;
mod frame_cost;
mod geometry;
mod inspector;
mod page_models;
mod page_runset_toolbar;
mod plan_exchange;
mod receipt_log;
mod registry_honesty;
mod run_host;
mod statement;
mod transient_checkpoint_exchange;
mod tuning_staging;
mod value_size;
