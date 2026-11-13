//! Engine contracts: configuration, errors, resources, results and documents.

#[path = "../common/family_src.rs"]
mod family_src;

mod ascii_numeric_precision;
mod current_output_units;
mod decoder_robustness;
mod determinism;
mod doc_examples;
mod engine_health;
mod engine_notices;
mod execution_properties;
mod frequency_table_documents;
mod macro_benchmark_contract;
mod model_construction_abort;
mod module_reachability;
mod planned_post_process;
mod public_paths_snapshot;
mod raw_ascii_admission;
mod raw_empty_plots;
mod raw_metadata_admission;
mod resource_and_commit_faults;
mod resource_limits;
mod result_document_json_precision;
mod result_document_properties;
mod save_directives;
mod signal_projection_matrix;
mod simulation_config_validation;
mod simulation_error_contract;
mod step_count_precision;
mod version_metadata;
