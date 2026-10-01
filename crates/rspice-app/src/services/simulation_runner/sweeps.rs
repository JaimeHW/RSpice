//! Parametric sweep runner.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

mod parametric;
mod types;

pub use parametric::{
    run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
