//! Application-modal tools that float above the active surface.
//!
//! None of these owns a route or a document. They open over whatever the user
//! is working on, do one job, and close: the project launcher, the background
//! jobs list and the notification centre. Grouping them keeps that
//! distinction from the canonical surfaces, which do own the route.

pub(crate) use rspice_app_project::workbench::tools::examples_browser;
pub(crate) use rspice_app_workbench::workbench::tools::notification_center;
