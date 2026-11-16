//! The host's own notification for a run that ends while the reader is in
//! another window.
//!
//! The shell decides when an outcome reaches for a reader outside the
//! window: the window is not in front, the work took long enough that they
//! may have turned to something else, and Task attention keeps that kind of
//! outcome. This module only carries the outcome's title and body to the
//! notification surface of the host it runs on:
//!
//! - in a browser, the page's own notification, where the reader turned it on;
//! - on macOS, Notification Center, for the application bundle. A process
//!   outside a bundle has no identity notifications are shown under (the
//!   notification center refuses it outright), so a development binary posts
//!   nothing;
//! - on Windows, a toast under the packaged application's AppUserModelID,
//!   registered for the user the first time one is posted. A binary that is
//!   not the packaged one (no release manifest beside it) posts nothing;
//! - elsewhere on Unix, the desktop's notification service on the session
//!   bus, which needs no identity. With no session bus or no service, nothing
//!   is shown.
//!
//! A failure to notify is never the reader's problem: the window is still
//! marked in the taskbar or dock, and the outcome is kept in the bell.

use std::cell::RefCell;

/// Somewhere an outcome's notification goes.
pub trait Notifier {
    /// Show `title` and `body` to the reader.
    fn post(&self, title: &str, body: &str);
}

thread_local! {
    /// A stand-in for the host's notification on this thread, for a test
    /// that reads what would have been shown.
    static STAND_IN: RefCell<Option<Box<dyn Notifier>>> = const { RefCell::new(None) };
}

/// Show an outcome in the host's notification surface. Choosing the
/// notification brings RSpice to the front; nothing here waits for the
/// host, so neither a frame nor a quit is held up by it.
pub fn post(title: &str, body: &str) {
    let stood_in = STAND_IN.with(|stand_in| {
        stand_in
            .borrow()
            .as_ref()
            .map(|notifier| notifier.post(title, body))
            .is_some()
    });
    if !stood_in {
        host::post(title, body);
    }
}

/// While the returned guard lives, this thread's notifications go to
/// `notifier` instead of the host's.
#[cfg(any(test, feature = "test-support"))]
pub fn stand_in(notifier: impl Notifier + 'static) -> StandIn {
    STAND_IN.with(|stand_in| *stand_in.borrow_mut() = Some(Box::new(notifier)));
    StandIn(())
}

/// Puts the host's notification back when dropped.
#[cfg(any(test, feature = "test-support"))]
#[must_use = "the stand-in is removed when the guard drops"]
pub struct StandIn(());

#[cfg(any(test, feature = "test-support"))]
impl Drop for StandIn {
    fn drop(&mut self) {
        STAND_IN.with(|stand_in| *stand_in.borrow_mut() = None);
    }
}

/// Whether this process can show a notification at all: a browser page,
/// the macOS application bundle, the packaged Windows application, or a Unix
/// desktop session.
pub fn host_has_identity() -> bool {
    host::has_identity()
}

#[cfg(target_arch = "wasm32")]
mod host {
    pub(super) fn post(title: &str, body: &str) {
        crate::workbench::browser::background_alerts::alert(title, body);
    }

    pub(super) const fn has_identity() -> bool {
        true
    }
}

#[cfg(target_os = "macos")]
mod host {
    use std::sync::atomic::{AtomicU64, Ordering};

    use block2::RcBlock;
    use objc2::runtime::Bool;
    use objc2_foundation::{NSBundle, NSError, NSString};
    use objc2_user_notifications::{
        UNAuthorizationOptions, UNMutableNotificationContent, UNNotificationRequest,
        UNUserNotificationCenter,
    };

    /// Each notification its own request, so a second outcome does not
    /// replace the first in Notification Center.
    static POSTED: AtomicU64 = AtomicU64::new(0);

    /// An application bundle: the main bundle has an identifier and is an
    /// `.app`. Outside one, `currentNotificationCenter` raises an
    /// Objective-C exception, so it is never asked.
    pub(super) fn has_identity() -> bool {
        let bundle = NSBundle::mainBundle();
        bundle.bundleIdentifier().is_some() && bundle.bundlePath().to_string().ends_with(".app")
    }

    pub(super) fn post(title: &str, body: &str) {
        if !has_identity() {
            return;
        }
        let (title, body) = (title.to_owned(), body.to_owned());
        // Asked every time: macOS prompts the reader the first time and
        // answers from their choice after that. The handler runs on one of
        // the system's queues, so the frame never waits for it.
        let deliver = RcBlock::new(move |granted: Bool, _error: *mut NSError| {
            if !granted.as_bool() {
                return;
            }
            let content = UNMutableNotificationContent::new();
            content.setTitle(&NSString::from_str(&title));
            content.setBody(&NSString::from_str(&body));
            let sequence = POSTED.fetch_add(1, Ordering::Relaxed);
            let identifier = NSString::from_str(&format!("rspice.run-outcome.{sequence}"));
            let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
                &identifier,
                &content,
                None,
            );
            UNUserNotificationCenter::currentNotificationCenter()
                .addNotificationRequest_withCompletionHandler(&request, None);
        });
        UNUserNotificationCenter::currentNotificationCenter()
            .requestAuthorizationWithOptions_completionHandler(
                UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound,
                &deliver,
            );
    }
}

#[cfg(windows)]
mod host {
    use windows::Data::Xml::Dom::XmlDocument;
    use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};
    use windows::core::HSTRING;

    /// The packaged application's AppUserModelID, the identity Windows shows
    /// its toasts under.
    const APP_ID: &str = "app.rspice.RSpice";

    /// The packaged application: the release manifest the archive carries
    /// stands beside the executable. A development build has none.
    pub(super) fn has_identity() -> bool {
        std::env::current_exe()
            .ok()
            .and_then(|executable| {
                executable
                    .parent()
                    .map(|directory| directory.join("RELEASE-MANIFEST.json"))
            })
            .is_some_and(|manifest| manifest.is_file())
    }

    pub(super) fn post(title: &str, body: &str) {
        if !has_identity() {
            return;
        }
        let toast = toast_xml(title, body);
        // Never joined: a toast that is slow to show holds up neither a frame
        // nor the process's exit.
        let _ = std::thread::Builder::new()
            .name("rspice-notification".to_owned())
            .spawn(move || {
                register();
                let _ = show(&toast);
            });
    }

    /// Name the AppUserModelID for this user, as an unpackaged application
    /// registers itself: without it Windows shows no toast under the id.
    fn register() {
        use windows_sys::Win32::System::Registry::{
            HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey,
            RegCreateKeyExW, RegSetValueExW,
        };
        let wide = |text: &str| text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        let path = wide(&format!(r"Software\Classes\AppUserModelId\{APP_ID}"));
        let mut key: HKEY = std::ptr::null_mut();
        // SAFETY: every string is NUL-terminated UTF-16 that outlives the
        // call, and `key` is closed below once it is open.
        unsafe {
            if RegCreateKeyExW(
                HKEY_CURRENT_USER,
                path.as_ptr(),
                0,
                std::ptr::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_SET_VALUE,
                std::ptr::null(),
                &mut key,
                std::ptr::null_mut(),
            ) != 0
            {
                return;
            }
            let name = wide("DisplayName");
            let value = wide("RSpice");
            let bytes = u32::try_from(value.len() * 2).unwrap_or(0);
            RegSetValueExW(key, name.as_ptr(), 0, REG_SZ, value.as_ptr().cast(), bytes);
            RegCloseKey(key);
        }
    }

    fn show(toast: &str) -> windows::core::Result<()> {
        let document = XmlDocument::new()?;
        document.LoadXml(&HSTRING::from(toast))?;
        let notification = ToastNotification::CreateToastNotification(&document)?;
        ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(APP_ID))?
            .Show(&notification)
    }

    /// The toast's generic two-line template, its texts escaped as XML.
    fn toast_xml(title: &str, body: &str) -> String {
        let escape = |text: &str| {
            text.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
        };
        format!(
            "<toast><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
            escape(title),
            escape(body)
        )
    }
}

#[cfg(all(unix, not(target_os = "macos"), not(target_arch = "wasm32")))]
mod host {
    use std::collections::HashMap;

    use zbus::zvariant::Value;

    /// The notification service needs no identity; whether a session bus
    /// and a service answer is found out by posting.
    pub(super) const fn has_identity() -> bool {
        true
    }

    pub(super) fn post(title: &str, body: &str) {
        let (title, body) = (title.to_owned(), body.to_owned());
        // Never joined: a session bus that is slow to answer holds up neither
        // a frame nor the process's exit.
        let _ = std::thread::Builder::new()
            .name("rspice-notification".to_owned())
            .spawn(move || {
                let _ = notify(&title, &body);
            });
    }

    /// The Desktop Notifications specification's `Notify`, as RSpice: no
    /// notification to replace, no actions, and the service's own timeout.
    fn notify(title: &str, body: &str) -> zbus::Result<()> {
        let connection = zbus::blocking::Connection::session()?;
        let hints = HashMap::from([("desktop-entry", Value::from("rspice"))]);
        connection.call_method(
            Some("org.freedesktop.Notifications"),
            "/org/freedesktop/Notifications",
            Some("org.freedesktop.Notifications"),
            "Notify",
            &(
                "RSpice",
                0_u32,
                "rspice",
                title,
                body,
                Vec::<&str>::new(),
                hints,
                -1_i32,
            ),
        )?;
        Ok(())
    }
}

#[cfg(not(any(target_arch = "wasm32", unix, windows)))]
mod host {
    pub(super) const fn has_identity() -> bool {
        false
    }

    pub(super) fn post(_title: &str, _body: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct Recorder(Rc<RefCell<Vec<(String, String)>>>);

    impl Notifier for Recorder {
        fn post(&self, title: &str, body: &str) {
            self.0
                .borrow_mut()
                .push((title.to_owned(), body.to_owned()));
        }
    }

    /// A stand-in takes this thread's notifications while it lives, and the
    /// host's surface takes them again once it is dropped.
    #[test]
    fn a_stand_in_takes_the_threads_notifications_while_it_lives() {
        let posted = Rc::new(RefCell::new(Vec::new()));
        {
            let _guard = stand_in(Recorder(Rc::clone(&posted)));
            post("Run 3 complete", "Default plan \u{b7} 1 analysis saved");
        }
        assert_eq!(
            *posted.borrow(),
            [(
                "Run 3 complete".to_owned(),
                "Default plan \u{b7} 1 analysis saved".to_owned()
            )]
        );
        assert!(STAND_IN.with(|stand_in| stand_in.borrow().is_none()));
    }

    /// A test binary is no application bundle: on macOS it has no identity
    /// to notify under, and posting is a quiet no-op rather than the
    /// notification center's exception.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_development_binary_on_macos_posts_nothing_and_raises_nothing() {
        assert!(!host_has_identity());
        post("Run 1 complete", "no analyses saved");
    }

    /// A development build on Windows has no release manifest beside it, so
    /// it registers nothing and shows no toast.
    #[cfg(windows)]
    #[test]
    fn a_development_binary_on_windows_posts_nothing() {
        assert!(!host_has_identity());
        post("Run 1 complete", "no analyses saved");
    }
}
