mod discovered;
mod environment;
mod library_paths;
mod recent;

pub(super) use discovered::render_discovered_files_tab;
pub(super) use environment::render_environment_tab;
pub(super) use library_paths::render_library_paths_tab;
pub(super) use recent::render_recent_files_tab;
