//! Application-modal tools that float above the active surface.
//!
//! None of these owns a route or a document. They open over whatever the user
//! is working on, do one job, and close: the project launcher, the background
//! jobs list, the specialist-tool browser, the calculator, and the
//! notification centre. Grouping them keeps that distinction from the
//! canonical surfaces, which do own the route.

pub(crate) mod calculator_tool;
pub(crate) mod expression_diagnostics;
pub(crate) mod jobs_manager;
pub(crate) mod notification_center;
pub(crate) mod project_launcher;
pub(crate) mod specialist_tool_browser;
