//! Where a project finds a library folder, and the form it writes that in.
//!
//! A path is always written with `/` and names the library folder in one of
//! three forms: relative to the project file's folder (`../shared/analog`),
//! under a named root given only as its first part (`$RSPICE_LIBS/analog`),
//! or absolute (`C:/eda/analog`, `//server/share/analog`, `/opt/libs/analog`).
//! A root's value comes from Preferences first, then the environment; it
//! must be absolute and cannot name another root. There is no `~`: it means
//! nothing on Windows and ties a project to one account.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The kind of computer a path is looked up on. A drive-letter or share
/// path is never looked up on macOS or Linux, nor a `/` path on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPlatform {
    Windows,
    Unix,
}

impl HostPlatform {
    pub const fn current() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Unix
        }
    }

    const fn folds_case(self) -> bool {
        matches!(self, Self::Windows)
    }
}

/// Which of the three forms a path is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryPathForm {
    /// Relative to the project file's folder: it travels with the project.
    Relative,
    /// Under a named root: the team's place, which survives the project
    /// moving.
    Root,
    /// A full path on one kind of computer.
    Absolute,
}

impl LibraryPathForm {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Relative => "Relative to the project",
            Self::Root => "Under a named root",
            Self::Absolute => "Absolute",
        }
    }
}

/// Why a path names no folder on this computer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LibraryPathError {
    #[error("No folder is named")]
    Empty,
    #[error("A path cannot start with ~; use a named root")]
    HomeFolder,
    #[error("'{0}' is not a root name")]
    BadRootName(String),
    #[error("${0} is not set on this computer")]
    RootNotSet(String),
    #[error("${0} must name a full path")]
    RootNotAbsolute(String),
    #[error("${0} cannot name another root")]
    RootNamesRoot(String),
    #[error("Written on another kind of computer")]
    OtherPlatform,
    #[error("The project is not saved yet, so a relative path names no folder")]
    NoProjectFolder,
    #[error("The folder's name is not valid Unicode")]
    NotUtf8,
    #[error("A picked folder must be a full path")]
    NotAbsolute,
}

/// The value of each named root, as this computer defines it.
pub trait PathRoots {
    fn value(&self, name: &str) -> Option<String>;
}

/// The one table of path variables, the one model paths read: a Preferences
/// value beats an environment variable of the same name.
pub struct RootTable<'a> {
    pub preferences: &'a BTreeMap<String, String>,
    pub environment: &'a dyn Fn(&str) -> Option<String>,
}

impl PathRoots for RootTable<'_> {
    fn value(&self, name: &str) -> Option<String> {
        self.preferences
            .get(name)
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .or_else(|| (self.environment)(name))
    }
}

impl RootTable<'_> {
    /// Every root Preferences names, with the value it takes here.
    pub fn defined(&self) -> Vec<(String, String)> {
        self.preferences
            .keys()
            .filter_map(|name| Some((name.clone(), self.value(name)?)))
            .collect()
    }
}

/// A library folder's path as a project writes it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LibraryPath(String);

impl TryFrom<String> for LibraryPath {
    type Error = LibraryPathError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::new(&text)
    }
}

impl From<LibraryPath> for String {
    fn from(path: LibraryPath) -> Self {
        path.0
    }
}

impl LibraryPath {
    /// Take a typed or picked path: `\` is stored as `/`, and a verbatim
    /// `\\?\` prefix is never stored.
    pub fn new(written: &str) -> Result<Self, LibraryPathError> {
        let text = normalize(written.trim());
        if text.is_empty() {
            return Err(LibraryPathError::Empty);
        }
        if text.starts_with('~') {
            return Err(LibraryPathError::HomeFolder);
        }
        if let Some(rest) = text.strip_prefix('$') {
            let name = rest.split('/').next().unwrap_or_default();
            if !is_root_name(name) {
                return Err(LibraryPathError::BadRootName(name.to_owned()));
            }
        }
        Ok(Self(text))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn form(&self) -> LibraryPathForm {
        if self.0.starts_with('$') {
            LibraryPathForm::Root
        } else if Absolute::parse(&self.0).is_some() {
            LibraryPathForm::Absolute
        } else {
            LibraryPathForm::Relative
        }
    }

    /// The root a path is written under, by name.
    pub fn root_name(&self) -> Option<&str> {
        self.0.strip_prefix('$')?.split('/').next()
    }

    /// The path in the host's own form, as a reader is shown it.
    pub fn shown(&self, host: HostPlatform) -> String {
        match host {
            HostPlatform::Windows => self.0.replace('/', "\\"),
            HostPlatform::Unix => self.0.clone(),
        }
    }

    /// The folder this path names on this computer.
    pub fn resolve(
        &self,
        project_folder: Option<&Path>,
        roots: &dyn PathRoots,
        host: HostPlatform,
    ) -> Result<PathBuf, LibraryPathError> {
        let absolute = match self.form() {
            LibraryPathForm::Absolute => Absolute::parse(&self.0).ok_or(LibraryPathError::Empty)?,
            LibraryPathForm::Root => {
                let (head, rest) = self.0[1..].split_once('/').unwrap_or((&self.0[1..], ""));
                let value = roots
                    .value(head)
                    .ok_or_else(|| LibraryPathError::RootNotSet(head.to_owned()))?;
                let value = normalize(value.trim());
                if value.starts_with('$') {
                    return Err(LibraryPathError::RootNamesRoot(head.to_owned()));
                }
                let base = Absolute::parse(&value)
                    .ok_or_else(|| LibraryPathError::RootNotAbsolute(head.to_owned()))?;
                base.joined(rest)
            }
            LibraryPathForm::Relative => {
                let folder = project_folder.ok_or(LibraryPathError::NoProjectFolder)?;
                let folder = folder.to_str().ok_or(LibraryPathError::NotUtf8)?;
                Absolute::parse(&normalize(folder))
                    .ok_or(LibraryPathError::NoProjectFolder)?
                    .joined(&self.0)
            }
        };
        if absolute.platform() != host {
            return Err(LibraryPathError::OtherPlatform);
        }
        Ok(absolute.path(host))
    }

    /// The same folder, written for a project file that moves from the
    /// folder `from` to the folder `to`. Only a relative path changes, and
    /// it stays relative where it can.
    pub fn rebased(
        &self,
        from: &Path,
        to: &Path,
        host: HostPlatform,
    ) -> Result<Self, LibraryPathError> {
        if self.form() != LibraryPathForm::Relative {
            return Ok(self.clone());
        }
        let resolved = self.resolve(Some(from), &NoRoots, host)?;
        let target = parse_picked(&resolved)?;
        let destination = parse_picked(to)?;
        let written = if destination.same_volume(&target, host) {
            destination.relative_to(&target, host)
        } else {
            target.text()
        };
        Self::new(&written)
    }
}

struct NoRoots;

impl PathRoots for NoRoots {
    fn value(&self, _: &str) -> Option<String> {
        None
    }
}

/// The form Attach stores for a picked folder, and the other forms that
/// name the same folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryPathChoice {
    pub stored: LibraryPath,
    pub others: Vec<LibraryPath>,
}

/// The path Attach writes for `folder`, the first that applies: relative
/// when it is inside the project folder, because it travels with the
/// project; the root when it is under a defined root, the team's place;
/// relative when it is on the same drive or share; absolute otherwise.
pub fn choose_library_path(
    folder: &Path,
    project_folder: Option<&Path>,
    roots: &[(String, String)],
    host: HostPlatform,
) -> Result<LibraryPathChoice, LibraryPathError> {
    let target = parse_picked(folder)?;
    let project = project_folder.and_then(|folder| parse_picked(folder).ok());
    let inside = project
        .as_ref()
        .filter(|project| target.strip_prefix(project, host).is_some())
        .map(|project| project.relative_to(&target, host));
    let under_root = roots
        .iter()
        .filter(|(name, _)| is_root_name(name))
        .filter_map(|(name, value)| {
            let value = normalize(value.trim());
            let base = Absolute::parse(&value)?;
            let rest = target.strip_prefix(&base, host)?;
            let mut written = format!("${name}");
            for segment in rest {
                written.push('/');
                written.push_str(segment);
            }
            Some((base.segments.len(), written))
        })
        .max_by_key(|(depth, _)| *depth)
        .map(|(_, written)| written);
    let same_volume = project
        .as_ref()
        .filter(|project| project.same_volume(&target, host))
        .map(|project| project.relative_to(&target, host));
    let absolute = Some(target.text());
    let mut forms = Vec::new();
    for written in [inside, under_root, same_volume, absolute]
        .into_iter()
        .flatten()
    {
        let path = LibraryPath::new(&written)?;
        if !forms.contains(&path) {
            forms.push(path);
        }
    }
    let stored = forms.remove(0);
    Ok(LibraryPathChoice {
        stored,
        others: forms,
    })
}

fn parse_picked(folder: &Path) -> Result<Absolute, LibraryPathError> {
    let text = folder.to_str().ok_or(LibraryPathError::NotUtf8)?;
    Absolute::parse(&normalize(text)).ok_or(LibraryPathError::NotAbsolute)
}

fn is_root_name(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|rest| rest.is_ascii_alphanumeric() || rest == '_')
}

/// `/` for `\`, no verbatim prefix, no doubled or trailing separators and
/// no `.` parts.
fn normalize(text: &str) -> String {
    let text = text.replace('\\', "/");
    let text = if let Some(share) = text.strip_prefix("//?/UNC/") {
        format!("//{share}")
    } else if let Some(rest) = text.strip_prefix("//?/") {
        rest.to_owned()
    } else {
        text
    };
    let (lead, rest) = if let Some(rest) = text.strip_prefix("//") {
        ("//", rest)
    } else if let Some(rest) = text.strip_prefix('/') {
        ("/", rest)
    } else {
        ("", text.as_str())
    };
    let parts = rest
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>();
    if parts.is_empty() && lead.is_empty() && rest.split('/').any(|part| part == ".") {
        return ".".to_owned();
    }
    format!("{lead}{}", parts.join("/"))
}

/// Where an absolute path starts.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Anchor {
    Drive(char),
    Share(String, String),
    Root,
}

/// An absolute path: its anchor and the folders below it.
#[derive(Debug, Clone)]
struct Absolute {
    anchor: Anchor,
    segments: Vec<String>,
}

impl Absolute {
    fn parse(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        let segments = |rest: &str| {
            rest.split('/')
                .filter(|part| !part.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        if bytes.len() >= 2
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && (bytes.len() == 2 || bytes[2] == b'/')
        {
            return Some(Self {
                anchor: Anchor::Drive(char::from(bytes[0]).to_ascii_uppercase()),
                segments: segments(&text[2..]),
            });
        }
        if let Some(rest) = text.strip_prefix("//") {
            let mut parts = segments(rest).into_iter();
            let server = parts.next()?;
            let share = parts.next()?;
            return Some(Self {
                anchor: Anchor::Share(server, share),
                segments: parts.collect(),
            });
        }
        let rest = text.strip_prefix('/')?;
        Some(Self {
            anchor: Anchor::Root,
            segments: segments(rest),
        })
    }

    fn platform(&self) -> HostPlatform {
        match self.anchor {
            Anchor::Drive(_) | Anchor::Share(..) => HostPlatform::Windows,
            Anchor::Root => HostPlatform::Unix,
        }
    }

    /// This path with `relative`'s parts applied, `..` taking one off.
    fn joined(mut self, relative: &str) -> Self {
        for part in relative.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    self.segments.pop();
                }
                part => self.segments.push(part.to_owned()),
            }
        }
        self
    }

    fn text(&self) -> String {
        let anchor = match &self.anchor {
            Anchor::Drive(letter) => format!("{letter}:"),
            Anchor::Share(server, share) => format!("//{server}/{share}"),
            Anchor::Root => String::new(),
        };
        if self.segments.is_empty() {
            return match self.anchor {
                Anchor::Root => "/".to_owned(),
                Anchor::Drive(_) => format!("{anchor}/"),
                Anchor::Share(..) => anchor,
            };
        }
        format!("{anchor}/{}", self.segments.join("/"))
    }

    fn path(&self, host: HostPlatform) -> PathBuf {
        match host {
            HostPlatform::Windows => PathBuf::from(self.text().replace('/', "\\")),
            HostPlatform::Unix => PathBuf::from(self.text()),
        }
    }

    fn same_volume(&self, other: &Self, host: HostPlatform) -> bool {
        match (&self.anchor, &other.anchor) {
            (Anchor::Drive(left), Anchor::Drive(right)) => left == right,
            (Anchor::Share(left_server, left_share), Anchor::Share(right_server, right_share)) => {
                same_part(left_server, right_server, host)
                    && same_part(left_share, right_share, host)
            }
            (Anchor::Root, Anchor::Root) => true,
            _ => false,
        }
    }

    /// The parts of `self` below `base`, when `self` is `base` or inside it.
    fn strip_prefix<'a>(&'a self, base: &Self, host: HostPlatform) -> Option<&'a [String]> {
        if !self.same_volume(base, host) || base.segments.len() > self.segments.len() {
            return None;
        }
        base.segments
            .iter()
            .zip(&self.segments)
            .all(|(left, right)| same_part(left, right, host))
            .then(|| &self.segments[base.segments.len()..])
    }

    /// `target` written relative to this folder, on the same volume.
    fn relative_to(&self, target: &Self, host: HostPlatform) -> String {
        let common = self
            .segments
            .iter()
            .zip(&target.segments)
            .take_while(|(left, right)| same_part(left, right, host))
            .count();
        let mut parts = vec![".."; self.segments.len() - common];
        parts.extend(target.segments[common..].iter().map(String::as_str));
        if parts.is_empty() {
            ".".to_owned()
        } else {
            parts.join("/")
        }
    }
}

fn same_part(left: &str, right: &str, host: HostPlatform) -> bool {
    if host.folds_case() {
        left.eq_ignore_ascii_case(right)
    } else {
        left == right
    }
}
