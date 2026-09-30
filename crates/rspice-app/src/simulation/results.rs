//! Application integration fixtures for runtime results and retained documents.

mod qpac;
pub(crate) use qpac::{qpac_retained_test_fixture, qpac_test_fixture};
mod qpnoise;
pub(crate) use qpnoise::{qpnoise_retained_test_fixture, qpnoise_test_fixture};
mod qpxf;
pub(crate) use qpxf::{qpxf_retained_test_fixture, qpxf_test_fixture};

mod units_tests;
