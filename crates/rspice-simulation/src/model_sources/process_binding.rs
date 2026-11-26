//! Which corner each source a run reads binds for a reference process.
//!
//! One decision, read by the run that binds the models and by the editor
//! that offers the reference process, so the editor never offers a process
//! the run refuses, nor hides one that changes what the run binds.

use rspice_model_library::pdk::contracts::PdkModelProcess;
use rspice_model_library::{ModelLibrary, ProcessCorner};
use rspice_vocabulary::product::ProcessCorner as CornerProcess;

/// The five processes a reference names.
const PROCESSES: [CornerProcess; 5] = [
    CornerProcess::TT,
    CornerProcess::SS,
    CornerProcess::FF,
    CornerProcess::SF,
    CornerProcess::FS,
];

/// Why a run binds no models for a process.
pub(super) enum ProcessRefusal<'c> {
    /// No library and no PDK: only TT binds, and it binds nothing.
    NoProcessSource,
    /// The library at this index, in the order given, has no such corner.
    MissingCorner { library: usize, corner: &'c str },
    /// The PDK contracts no model source for the process.
    NoPdkContract,
}

/// Whether sealing keeps `corner`. A corner with no source and no section
/// materializes nothing, so a run never binds it.
pub(super) fn seals(corner: &ProcessCorner) -> bool {
    corner.file_path.is_some() || !corner.section_bindings.is_empty()
}

pub(super) const fn pdk_model_process(process: CornerProcess) -> PdkModelProcess {
    match process {
        CornerProcess::TT => PdkModelProcess::Tt,
        CornerProcess::SS => PdkModelProcess::Ss,
        CornerProcess::FF => PdkModelProcess::Ff,
        CornerProcess::SF => PdkModelProcess::Sf,
        CornerProcess::FS => PdkModelProcess::Fs,
    }
}

/// The corner each library binds for `process`, in library order; `None` is
/// a library's unsectioned root.
///
/// A library binds the corner its plan binding selects where
/// `honor_nominal_selection` holds, and the corner the process names
/// otherwise. Only TT, over a library with no corners and no selection,
/// binds the root. A PDK must contract a model source for the process.
pub(super) fn library_corners<'c, C>(
    libraries: impl IntoIterator<Item = (C, Option<&'c str>)>,
    pdk: &[PdkModelProcess],
    process: CornerProcess,
    honor_nominal_selection: bool,
) -> Result<Vec<Option<&'c ProcessCorner>>, ProcessRefusal<'c>>
where
    C: IntoIterator<Item = &'c ProcessCorner>,
    C::IntoIter: Clone,
{
    let mut libraries = libraries.into_iter().peekable();
    if libraries.peek().is_none() && pdk.is_empty() {
        return if process == CornerProcess::TT {
            Ok(Vec::new())
        } else {
            Err(ProcessRefusal::NoProcessSource)
        };
    }
    let mut bound = Vec::new();
    for (index, (corners, selected)) in libraries.enumerate() {
        let mut corners = corners.into_iter();
        let requested = honor_nominal_selection
            .then_some(selected)
            .flatten()
            .unwrap_or(process.short_name());
        match corners
            .clone()
            .find(|corner| corner.name.eq_ignore_ascii_case(requested))
        {
            Some(corner) => bound.push(Some(corner)),
            None if selected.is_none()
                && process == CornerProcess::TT
                && corners.next().is_none() =>
            {
                bound.push(None);
            }
            None => {
                return Err(ProcessRefusal::MissingCorner {
                    library: index,
                    corner: requested,
                });
            }
        }
    }
    if !pdk.is_empty() && !pdk.contains(&pdk_model_process(process)) {
        return Err(ProcessRefusal::NoPdkContract);
    }
    Ok(bound)
}

/// What the reference process decides in a run: whether any source the run
/// reads binds by it, and which processes the run binds at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceProcessChoice {
    selects: bool,
    binds: Vec<CornerProcess>,
}

impl ReferenceProcessChoice {
    /// What the reference process decides in a reference run over
    /// `libraries`, each with the corner its plan binding selects, and a PDK
    /// that contracts model sources for the processes in `pdk`.
    #[must_use]
    pub fn of(libraries: &[(&ModelLibrary, Option<&str>)], pdk: &[PdkModelProcess]) -> Self {
        let selects = !pdk.is_empty()
            || libraries.iter().any(|(library, selected)| {
                selected.is_none() && library.corners.values().any(seals)
            });
        let binds = PROCESSES
            .into_iter()
            .filter(|process| {
                library_corners(
                    libraries.iter().map(|(library, selected)| {
                        (
                            library.corners.values().filter(|corner| seals(corner)),
                            *selected,
                        )
                    }),
                    pdk,
                    *process,
                    true,
                )
                .is_ok()
            })
            .collect();
        Self { selects, binds }
    }

    /// Whether some source the run reads binds by the reference process: a
    /// library its plan binding leaves to the process, with corners to bind,
    /// or a PDK's process contracts.
    #[must_use]
    pub fn selects(&self) -> bool {
        self.selects
    }

    /// Whether the run binds its models at `process`.
    #[must_use]
    pub fn binds(&self, process: CornerProcess) -> bool {
        self.binds.contains(&process)
    }
}
