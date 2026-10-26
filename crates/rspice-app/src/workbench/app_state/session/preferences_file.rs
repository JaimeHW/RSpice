//! The personal preferences file.
//!
//! Preferences used to live only inside the working session eframe writes on
//! its autosave cadence. A change was durable only at the next such save, a
//! session that would not decode took every setting down with it, and nothing
//! was saved at all while a live session was mirrored. They now have a file of
//! their own, the way a `.cdsenv` or an LTspice ini sits apart from a design:
//!
//! - desktop: `<config>/rspice/preferences.json`, published atomically;
//! - browser: a local-storage key of its own, apart from eframe's session.
//!
//! The file is read before anything consults a preference and wins over the
//! copy the session still carries. That copy is kept so an older build, and
//! the shortcut store's first-run seeding, still find what they read. The file
//! is written shortly after a change settles, whatever the session is doing,
//! because preferences belong to the device rather than to the session.
//!
//! The keyboard shortcut library is not in it: that has its own store, which
//! saves a change before the change goes live. A settings file, the one
//! Import… and Export… exchange, carries both, so a person moves their whole
//! setup to another machine in one step.
//!
//! A preferences file this build cannot parse is set aside under a dated name
//! rather than overwritten. A file from a newer build loads what this build
//! knows and writes back everything else exactly as it found it.

use std::collections::BTreeMap;
use std::hash::Hasher as _;
use std::time::Duration;

use serde_json::{Map, Value};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::time_compat::Instant;
use crate::ui::Theme;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::{UserPreferences, WorkbenchSession};

pub const FORMAT: &str = "rspice-preferences";
const VERSION: u64 = 1;
const SETTINGS_FORMAT: &str = "rspice-settings";
const SETTINGS_VERSION: u64 = 1;
#[cfg(not(target_arch = "wasm32"))]
pub const FILE_NAME: &str = "preferences.json";
#[cfg(target_arch = "wasm32")]
const BROWSER_KEY: &str = "rspice.preferences.v1";
const MAX_FILE_BYTES: usize = 4 * 1024 * 1024;

/// The ceiling on a settings file Import… will read.
pub(in crate::workbench) const MAX_SETTINGS_FILE_BYTES: usize = 8 * 1024 * 1024;

/// What the settings-file picker offers and how its messages name the file.
pub const SETTINGS_FILE: crate::io::file_exchange::FileKind = crate::io::file_exchange::FileKind {
    label: "RSpice settings",
    extensions: &["json"],
    subject: "The settings file",
    fallback_name: "rspice-settings.json",
};

/// How long a change has to settle before it is written. A dragged control
/// changes its value every frame; the file is written once, after it stops.
const WRITE_DELAY: Duration = Duration::from_millis(500);

/// How often the settings are looked at on frames that carried no input,
/// which is how a change made by something other than the reader is seen.
const IDLE_LOOK: Duration = Duration::from_secs(1);

pub const THEME: &str = "theme";
pub const AUTOSAVE: &str = "autosave-minutes";
const SPOKEN_FEEDBACK: &str = "browser-spoken-feedback";
pub const PREFERENCES: &str = "preferences";
pub const SHORTCUTS: &str = "shortcuts";

/// What a preferences file holds, decoded.
#[derive(Debug, Clone)]
pub(crate) struct PersonalSettings {
    theme: Theme,
    autosave_minutes: u8,
    browser_spoken_feedback: bool,
    /// Every preference but the shortcut library, which keeps its own store.
    preferences: UserPreferences,
    envelope: Envelope,
}

/// What a file carried beyond the values this build reads.
#[derive(Debug, Clone, Default)]
struct Envelope {
    /// The version the file declared. A newer one is written back as found.
    version: u64,
    /// Every stored key but `format` and `version`, exactly as read.
    stored: Map<String, Value>,
    /// Known keys whose stored value this build could not read, with the
    /// value it used instead. Each is written back as it was found until the
    /// setting is changed here.
    unread: BTreeMap<&'static str, Value>,
}

/// The settings the file holds, borrowed from the running session. The
/// change check hashes this same struct, so the two cannot disagree on what
/// the file holds.
#[derive(serde::Serialize)]
struct Values<'a> {
    theme: Theme,
    autosave_minutes: u8,
    browser_spoken_feedback: bool,
    preferences: &'a UserPreferences,
}

impl<'a> Values<'a> {
    fn of(
        workbench: &WorkbenchSession,
        platform: &PlatformState,
        browser_spoken_feedback: bool,
        preferences: &'a UserPreferences,
    ) -> Self {
        Self {
            theme: workbench.theme,
            autosave_minutes: platform.autosave_minutes,
            browser_spoken_feedback,
            preferences,
        }
    }

    fn encoded(&self) -> Result<[(&'static str, Value); 4], String> {
        let mut preferences = serde_json::to_value(self.preferences)
            .map_err(|error| format!("the preferences could not be encoded: {error}"))?;
        if let Some(object) = preferences.as_object_mut() {
            object.remove(SHORTCUTS);
        }
        let theme = serde_json::to_value(self.theme)
            .map_err(|error| format!("the theme could not be encoded: {error}"))?;
        Ok([
            (THEME, theme),
            (AUTOSAVE, Value::from(self.autosave_minutes)),
            (SPOKEN_FEEDBACK, Value::Bool(self.browser_spoken_feedback)),
            (PREFERENCES, preferences),
        ])
    }
}

fn encode(values: &Values<'_>, envelope: &Envelope) -> Result<Value, String> {
    let mut object = envelope.stored.clone();
    for (key, value) in values.encoded()? {
        if envelope.unread.get(key) == Some(&value) {
            continue;
        }
        let value = match (key, object.remove(key), value) {
            // A newer build's theme fields survive a theme change made here.
            (THEME, Some(Value::Object(mut stored)), Value::Object(current)) => {
                stored.extend(current);
                Value::Object(stored)
            }
            (_, _, value) => value,
        };
        object.insert(key.to_owned(), value);
    }
    object.insert("format".to_owned(), Value::from(FORMAT));
    object.insert(
        "version".to_owned(),
        Value::from(envelope.version.max(VERSION)),
    );
    Ok(Value::Object(object))
}

fn encode_bytes(values: &Values<'_>, envelope: &Envelope) -> Result<Vec<u8>, String> {
    crate::workbench::shortcuts::artifacts::canonical_json_bytes(encode(values, envelope)?)
        .map_err(|error| format!("the preferences could not be encoded: {error}"))
}

fn parse(bytes: &[u8], limit: usize) -> Result<Value, String> {
    if bytes.len() > limit {
        return Err(format!(
            "it is larger than the {} MiB limit",
            limit / (1024 * 1024)
        ));
    }
    serde_json::from_slice(bytes).map_err(|error| {
        format!(
            "it is not valid JSON at line {}, column {}",
            error.line(),
            error.column()
        )
    })
}

/// Decode a preferences object.
///
/// `strict` refuses a known setting this build cannot read; otherwise that
/// setting falls back to its default and its stored value is kept for writing
/// back. The file a newer build left on this machine is read leniently, so it
/// is never lost; a settings file being imported is read strictly, so an
/// import never half-applies.
fn decode(value: Value, strict: bool) -> Result<PersonalSettings, String> {
    let Value::Object(stored) = value else {
        return Err("it is not a JSON object".to_owned());
    };
    let mut stored = stored;
    if stored.remove("format").as_ref().and_then(Value::as_str) != Some(FORMAT) {
        return Err("it is not an RSpice preferences file".to_owned());
    }
    let version = stored
        .remove("version")
        .as_ref()
        .and_then(Value::as_u64)
        .filter(|version| *version >= 1)
        .ok_or_else(|| "its version is missing or not a whole number".to_owned())?;

    let mut unread = Vec::new();
    let theme = read_setting(&stored, THEME, "theme", strict, &mut unread, |raw| {
        serde_json::from_value::<Theme>(raw).map_err(|error| error.to_string())
    })?;
    let autosave_minutes = read_setting(
        &stored,
        AUTOSAVE,
        "autosave interval",
        strict,
        &mut unread,
        |raw| {
            serde_json::from_value::<u8>(raw)
                .map(crate::workbench::lifecycle::session::normalize_autosave_minutes)
                .map_err(|error| error.to_string())
        },
    )?
    .unwrap_or(crate::services::session::DEFAULT_AUTOSAVE_MINUTES);
    let browser_spoken_feedback = read_setting(
        &stored,
        SPOKEN_FEEDBACK,
        "spoken feedback",
        strict,
        &mut unread,
        |raw| serde_json::from_value::<bool>(raw).map_err(|error| error.to_string()),
    )?
    .unwrap_or_default();
    let preferences = read_setting(
        &stored,
        PREFERENCES,
        "preferences",
        strict,
        &mut unread,
        |mut raw| {
            if let Some(object) = raw.as_object_mut() {
                object.remove(SHORTCUTS);
            }
            let mut preferences = serde_json::from_value::<UserPreferences>(raw)
                .map_err(|error| error.to_string())?;
            preferences.normalize();
            Ok(preferences)
        },
    )?
    .unwrap_or_default();

    let mut settings = PersonalSettings {
        theme: theme.unwrap_or_default(),
        autosave_minutes,
        browser_spoken_feedback,
        preferences,
        envelope: Envelope {
            version,
            stored,
            unread: BTreeMap::new(),
        },
    };
    if !unread.is_empty() {
        // The value written for an unread key is the default it fell back
        // to, so that encoding is what marks the setting as still unchanged.
        let fallbacks = Values {
            theme: settings.theme,
            autosave_minutes: settings.autosave_minutes,
            browser_spoken_feedback: settings.browser_spoken_feedback,
            preferences: &settings.preferences,
        }
        .encoded()?;
        settings.envelope.unread = fallbacks
            .into_iter()
            .filter(|(key, _)| unread.contains(key))
            .collect();
    }
    Ok(settings)
}

/// One stored setting: `None` when the file does not have it, or when it
/// cannot be read and `strict` is off, in which case `key` joins `unread`.
fn read_setting<T>(
    stored: &Map<String, Value>,
    key: &'static str,
    label: &str,
    strict: bool,
    unread: &mut Vec<&'static str>,
    decode: impl FnOnce(Value) -> Result<T, String>,
) -> Result<Option<T>, String> {
    let Some(raw) = stored.get(key) else {
        return Ok(None);
    };
    match decode(raw.clone()) {
        Ok(value) => Ok(Some(value)),
        Err(error) if strict => Err(format!("its {label} could not be read: {error}")),
        Err(_) => {
            unread.push(key);
            Ok(None)
        }
    }
}

/// Whether a frame's input could have changed a setting: a key, text, a
/// press or release, a wheel, a paste, the end of a touch, or an action from
/// assistive technology. Pointer movement alone cannot, and a canvas drag or a
/// hover is nothing but that, frame after frame; the once-a-second look
/// catches anything that changes a setting some other way.
pub fn input_can_change_a_setting(events: &[egui::Event]) -> bool {
    events.iter().any(|event| match event {
        egui::Event::Key { .. }
        | egui::Event::Text(_)
        | egui::Event::Ime(_)
        | egui::Event::Paste(_)
        | egui::Event::Cut
        | egui::Event::PointerButton { .. }
        | egui::Event::MouseWheel { .. }
        | egui::Event::AccessKitActionRequest(_) => true,
        egui::Event::Touch { phase, .. } => *phase == egui::TouchPhase::End,
        _ => false,
    })
}

/// A hash of every value the file holds, taken on a frame whose input could
/// have changed one. The shortcut library rides inside the preferences but is
/// not in the file, and its own store owns its durability, so it is swapped
/// out for the duration rather than serialized every time.
pub fn fingerprint(
    workbench: &WorkbenchSession,
    platform: &PlatformState,
    browser_spoken_feedback: bool,
    preferences: &mut UserPreferences,
) -> u64 {
    struct HashWriter(std::hash::DefaultHasher);

    impl std::io::Write for HashWriter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.write(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let shortcuts = std::mem::take(preferences.shortcut_profiles_mut());
    let mut writer = HashWriter(std::hash::DefaultHasher::new());
    // A value that will not serialize cannot be written either, so whatever
    // was hashed before the failure is as good an observation as any.
    let _ = serde_json::to_writer(
        &mut writer,
        &Values::of(workbench, platform, browser_spoken_feedback, preferences),
    );
    *preferences.shortcut_profiles_mut() = shortcuts;
    writer.0.finish()
}

/// Where the preferences file lives.
#[derive(Debug, Clone)]
pub enum PreferencesStore {
    #[cfg(not(target_arch = "wasm32"))]
    File(std::path::PathBuf),
    #[cfg(target_arch = "wasm32")]
    Browser,
}

impl PreferencesStore {
    pub(in crate::workbench) fn production() -> Option<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            dirs::config_dir().map(|folder| Self::File(folder.join("rspice").join(FILE_NAME)))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Some(Self::Browser)
        }
    }

    fn describe(&self) -> String {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(path) => format!("'{}'", path.display()),
            #[cfg(target_arch = "wasm32")]
            Self::Browser => "this browser's storage".to_owned(),
        }
    }

    fn read(&self) -> Result<Option<Vec<u8>>, String> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(path) => {
                use std::io::Read as _;

                crate::io::durable_file::reconcile_publication(path)
                    .map_err(|error| error.to_string())?;
                let file = match std::fs::File::open(path) {
                    Ok(file) => file,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        return Ok(None);
                    }
                    Err(error) => return Err(error.to_string()),
                };
                // One byte past the ceiling is enough to know it was passed.
                let mut bytes = Vec::new();
                file.take(MAX_FILE_BYTES as u64 + 1)
                    .read_to_end(&mut bytes)
                    .map_err(|error| error.to_string())?;
                Ok(Some(bytes))
            }
            #[cfg(target_arch = "wasm32")]
            Self::Browser => Ok(browser_storage()?
                .get_item(BROWSER_KEY)
                .map_err(|error| format!("the browser refused the read ({error:?})"))?
                .map(String::into_bytes)),
        }
    }

    fn write(&self, bytes: &[u8]) -> Result<(), String> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(path) => crate::io::durable_file::atomic_write_bytes(path, bytes)
                .map_err(|error| error.to_string()),
            #[cfg(target_arch = "wasm32")]
            Self::Browser => {
                let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
                browser_storage()?
                    .set_item(BROWSER_KEY, text)
                    .map_err(|error| format!("the browser refused the write ({error:?})"))
            }
        }
    }

    /// Move a file that will not parse out of the way, and say where it went.
    fn set_aside(&self, bytes: &[u8]) -> Result<String, String> {
        let stamp = crate::time_compat::unix_epoch().as_secs();
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(path) => {
                let _ = bytes;
                let name = path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or(FILE_NAME);
                let mut backup = path.with_file_name(format!("{name}.bad-{stamp}"));
                let mut attempt = 1;
                while backup.exists() {
                    attempt += 1;
                    backup = path.with_file_name(format!("{name}.bad-{stamp}-{attempt}"));
                }
                std::fs::rename(path, &backup).map_err(|error| error.to_string())?;
                Ok(backup.display().to_string())
            }
            #[cfg(target_arch = "wasm32")]
            Self::Browser => {
                let storage = browser_storage()?;
                let backup = format!("{BROWSER_KEY}.bad-{stamp}");
                storage
                    .set_item(&backup, &String::from_utf8_lossy(bytes))
                    .and_then(|()| storage.remove_item(BROWSER_KEY))
                    .map_err(|error| format!("the browser refused the change ({error:?})"))?;
                Ok(format!("browser storage key '{backup}'"))
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> Result<web_sys::Storage, String> {
    web_sys::window()
        .ok_or_else(|| "the browser window is unavailable".to_owned())?
        .local_storage()
        .map_err(|error| format!("the browser refused access to storage ({error:?})"))?
        .ok_or_else(|| "this browser offers no local storage".to_owned())
}

/// Write-on-change bookkeeping for the preferences file.
#[derive(Debug, Clone, Default)]
pub(crate) struct PreferencesFileRuntime {
    /// Where the file lives. `None` keeps preferences with the session alone,
    /// which is what every test state and a machine with no settings folder
    /// get.
    store: Option<PreferencesStore>,
    /// Set when the file exists but could not be read, so that a passing read
    /// failure never replaces it.
    write_withheld: bool,
    envelope: Envelope,
    /// The bytes the store holds, as far as this session knows.
    written: Option<Vec<u8>>,
    observed: u64,
    observed_at: Option<Instant>,
    /// When a settled change is due to be written.
    write_due: Option<Instant>,
    /// Set by a failed write and cleared by a successful one, so that a
    /// failure is reported once rather than once per attempt.
    failing: bool,
    #[cfg(test)]
    pub writes: usize,
    /// How many times the settings were hashed.
    #[cfg(test)]
    pub looks: usize,
}

/// A settings file that passed every check, waiting for its shortcut library
/// to be saved before its preferences go live.
#[derive(Debug, Clone)]
pub struct StagedSettingsImport {
    source_name: String,
    settings: PersonalSettings,
}

/// What the preferences file reads and writes, borrowed from the session:
/// the theme, the autosave interval, spoken feedback and the preferences, with
/// the file's own bookkeeping. Its failures go to the model's log, and its
/// answers to the gesture to the session's prompt.
pub struct PreferencesFileParts<'a> {
    pub(crate) file: &'a mut PreferencesFileRuntime,
    pub(crate) browser_spoken_feedback: &'a mut bool,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) platform: &'a mut PlatformState,
}

impl PreferencesFileParts<'_> {
    /// Read the preferences file over whatever the session restored, and keep
    /// it current from here on.
    ///
    /// With no file yet, what the session carried, or the shipped defaults
    /// when it carried nothing, is written as the first one.
    pub fn bind_preferences_file(&mut self, store: PreferencesStore) {
        let read = store.read();
        self.file.store = Some(store.clone());
        match read {
            Ok(None) => self.write_preferences_file(),
            Ok(Some(bytes)) => {
                match parse(&bytes, MAX_FILE_BYTES).and_then(|value| decode(value, false)) {
                    Ok(settings) => {
                        self.apply_personal_settings(settings);
                        self.file.written = encode_bytes(
                            &Values::of(
                                self.workbench,
                                self.platform,
                                *self.browser_spoken_feedback,
                                &self.model.preferences,
                            ),
                            &self.file.envelope,
                        )
                        .ok();
                    }
                    Err(reason) => self.set_aside_preferences_file(&store, &bytes, &reason),
                }
            }
            Err(error) => {
                self.file.write_withheld = true;
                self.model.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "Preferences could not be read from {}: {error}. This session uses the \
                         settings it restored and will not write that file.",
                        store.describe()
                    ),
                );
            }
        }
        self.file.observed = fingerprint(
            self.workbench,
            self.platform,
            *self.browser_spoken_feedback,
            &mut self.model.preferences,
        );
    }

    fn set_aside_preferences_file(&mut self, store: &PreferencesStore, bytes: &[u8], reason: &str) {
        match store.set_aside(bytes) {
            Ok(backup) => {
                self.apply_personal_settings(PersonalSettings {
                    theme: Theme::default(),
                    autosave_minutes: crate::services::session::DEFAULT_AUTOSAVE_MINUTES,
                    browser_spoken_feedback: false,
                    preferences: UserPreferences::default(),
                    envelope: Envelope::default(),
                });
                self.model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    format!(
                        "The preferences file could not be read: {reason}. Default settings are \
                         in use, and the unreadable file was kept as {backup}."
                    ),
                );
                self.write_preferences_file();
            }
            Err(error) => {
                self.file.write_withheld = true;
                self.model.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "The preferences file {} could not be read: {reason}. It could not be \
                         moved aside either: {error}. This session uses the settings it \
                         restored and will not write that file.",
                        store.describe()
                    ),
                );
            }
        }
    }

    /// Put decoded settings in force. The shortcut library in use stays: it
    /// is not part of the preferences file.
    fn apply_personal_settings(&mut self, settings: PersonalSettings) {
        let shortcuts = std::mem::take(self.model.preferences.shortcut_profiles_mut());
        self.workbench.theme = settings.theme;
        self.platform.autosave_minutes = settings.autosave_minutes;
        *self.browser_spoken_feedback = settings.browser_spoken_feedback;
        *self.model.preferences = settings.preferences;
        *self.model.preferences.shortcut_profiles_mut() = shortcuts;
        self.file.envelope = settings.envelope;
    }

    /// Notice a preference change and write it once it settles.
    ///
    /// Runs every frame. The settings are hashed only on a frame whose input
    /// could have changed one, while a write is pending, or once a second
    /// otherwise.
    pub(in crate::workbench) fn persist_preferences_file(&mut self, ctx: &egui::Context) {
        let had_input = ctx.input(|input| input_can_change_a_setting(&input.events));
        if let Some(wait) = self.persist_preferences_file_at(Instant::now(), had_input) {
            ctx.request_repaint_after(wait);
        }
    }

    /// [`Self::persist_preferences_file`] at `now`. Returns how long until a
    /// pending write is due.
    pub fn persist_preferences_file_at(
        &mut self,
        now: Instant,
        had_input: bool,
    ) -> Option<Duration> {
        let runtime = &mut *self.file;
        if runtime.store.is_none() || runtime.write_withheld {
            return None;
        }
        let look = had_input
            || runtime.write_due.is_some()
            || runtime
                .observed_at
                .is_none_or(|at| now.saturating_duration_since(at) >= IDLE_LOOK);
        if look {
            runtime.observed_at = Some(now);
            #[cfg(test)]
            {
                runtime.looks += 1;
            }
            let observed = fingerprint(
                self.workbench,
                self.platform,
                *self.browser_spoken_feedback,
                &mut self.model.preferences,
            );
            if observed != runtime.observed {
                runtime.observed = observed;
                runtime.write_due = Some(now + WRITE_DELAY);
            }
        }
        let due = runtime.write_due?;
        if now < due {
            return Some(due - now);
        }
        runtime.write_due = None;
        self.write_preferences_file();
        None
    }

    /// Write any change not yet in the file now, rather than when it settles.
    pub fn flush_preferences_file(&mut self) {
        self.file.write_due = None;
        self.file.observed = fingerprint(
            self.workbench,
            self.platform,
            *self.browser_spoken_feedback,
            &mut self.model.preferences,
        );
        self.write_preferences_file();
    }

    fn write_preferences_file(&mut self) {
        let runtime = &mut *self.file;
        let Some(store) = runtime.store.as_ref() else {
            return;
        };
        if runtime.write_withheld {
            return;
        }
        let outcome = encode_bytes(
            &Values::of(
                self.workbench,
                self.platform,
                *self.browser_spoken_feedback,
                &self.model.preferences,
            ),
            &runtime.envelope,
        )
        .and_then(|bytes| {
            if runtime.written.as_deref() == Some(bytes.as_slice()) {
                return Ok(());
            }
            store.write(&bytes)?;
            runtime.written = Some(bytes);
            #[cfg(test)]
            {
                runtime.writes += 1;
            }
            Ok(())
        });
        let first_failure = outcome.is_err() && !runtime.failing;
        runtime.failing = outcome.is_err();
        let location = store.describe();
        if let (Err(error), true) = (outcome, first_failure) {
            self.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "Preferences could not be saved to {location}: {error}. RSpice will try again \
                     after the next change."
                ),
            );
        }
    }

    /// The settings file Export… writes: the preferences file's contents and
    /// the shortcut library in use, together.
    pub fn settings_file_bytes(&self) -> Result<Vec<u8>, String> {
        let preferences = encode(
            &Values::of(
                self.workbench,
                self.platform,
                *self.browser_spoken_feedback,
                &self.model.preferences,
            ),
            &self.file.envelope,
        )?;
        let shortcuts = serde_json::to_value(self.model.preferences.shortcut_profiles())
            .map_err(|error| format!("the keyboard shortcuts could not be encoded: {error}"))?;
        crate::workbench::shortcuts::artifacts::canonical_json_bytes(serde_json::json!({
            "format": SETTINGS_FORMAT,
            "version": SETTINGS_VERSION,
            PREFERENCES: preferences,
            SHORTCUTS: shortcuts,
        }))
        .map_err(|error| format!("the settings could not be encoded: {error}"))
    }

    /// Put an imported settings file in force once its shortcut library has
    /// been saved, or say why nothing changed.
    pub fn finish_settings_import(
        &mut self,
        staged: StagedSettingsImport,
        shortcuts_saved: Result<(), String>,
    ) {
        if let Err(error) = shortcuts_saved {
            self.workbench.prompt(format!(
                "Settings were not imported from {}: {error}. Nothing was changed.",
                staged.source_name
            ));
            return;
        }
        self.apply_personal_settings(staged.settings);
        let preset = self
            .model
            .preferences
            .workspace()
            .map(crate::workbench::WorkspacePreferences::preset)
            .unwrap_or_default();
        self.workbench.set_workspace_preset(preset);
        self.flush_preferences_file();
        let message = format!("Settings imported from {}.", staged.source_name);
        self.model
            .record(LogSeverity::Info, LogSource::Studio, message.clone());
        self.workbench.prompt(message);
    }

    /// Open the folder that holds the preferences file.
    pub fn show_preferences_file(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let folder = match self.file.store.as_ref() {
                Some(PreferencesStore::File(path)) => {
                    path.parent().map(std::path::Path::to_path_buf)
                }
                None => None,
            };
            let Some(folder) = folder else {
                self.workbench
                    .prompt("Preferences are not kept in a file on this machine.");
                return;
            };
            if let Err(error) = show_folder(&folder) {
                self.workbench
                    .prompt(format!("The settings folder could not be shown: {error}"));
            }
        }
    }
}

/// Check a settings file completely, changing nothing.
///
/// Returns the shortcut library to save, which has to be in the store
/// before anything goes live, and the preferences that follow it.
pub fn stage_settings_import(
    model: &Model,
    source_name: &str,
    bytes: &[u8],
) -> Result<(ShortcutProfileLibrary, StagedSettingsImport), String> {
    let Value::Object(mut file) = parse(bytes, MAX_SETTINGS_FILE_BYTES)? else {
        return Err("it is not a JSON object".to_owned());
    };
    if file.get("format").and_then(Value::as_str) != Some(SETTINGS_FORMAT) {
        return Err("it is not an RSpice settings file".to_owned());
    }
    if file
        .get("version")
        .and_then(Value::as_u64)
        .is_none_or(|version| version < 1)
    {
        return Err("its version is missing or not a whole number".to_owned());
    }
    let settings = decode(
        file.remove(PREFERENCES)
            .ok_or_else(|| "it holds no preferences".to_owned())?,
        true,
    )
    .map_err(|reason| format!("its preferences are not usable: {reason}"))?;
    let imported: ShortcutProfileLibrary = serde_json::from_value(
        file.remove(SHORTCUTS)
            .ok_or_else(|| "it holds no keyboard shortcuts".to_owned())?,
    )
    .map_err(|error| format!("its keyboard shortcuts could not be read: {error}"))?;
    crate::workbench::shortcuts::library_persistence::validate_library(&imported)
        .map_err(|reason| format!("its keyboard shortcuts are not usable: {reason}"))?;
    let mut shortcuts = model.preferences.shortcut_profiles().clone();
    shortcuts
        .replace_content_from(&imported)
        .map_err(|error| format!("the shortcuts in use cannot be replaced: {error}"))?;
    Ok((
        shortcuts,
        StagedSettingsImport {
            source_name: source_name.to_owned(),
            settings,
        },
    ))
}

#[cfg(all(
    not(test),
    not(target_arch = "wasm32"),
    any(target_os = "windows", target_os = "macos", target_os = "linux")
))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    crate::workbench::lifecycle::recovery::reveal_directory(folder)
}

#[cfg(all(
    not(test),
    not(target_arch = "wasm32"),
    not(any(target_os = "windows", target_os = "macos", target_os = "linux"))
))]
fn show_folder(_folder: &std::path::Path) -> Result<(), String> {
    Err("this system has no file manager RSpice can open".to_owned())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
thread_local! {
    /// The folders a test build was asked to show, in place of a file manager.
    pub static SHOWN_FOLDERS: std::cell::RefCell<Vec<std::path::PathBuf>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(all(test, not(target_arch = "wasm32")))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    SHOWN_FOLDERS.with_borrow_mut(|shown| shown.push(folder.to_path_buf()));
    Ok(())
}
