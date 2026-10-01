//! Project identity, metadata and authenticated audit contracts shared by project and execution.

mod descriptor;
mod library_publication;

pub use descriptor::*;
pub use library_publication::*;

mod libraries;
pub use libraries::ProjectLibraries;
