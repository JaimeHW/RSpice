//! The browser's own notification for a run that ends while the reader is
//! elsewhere.
//!
//! The desktop marks its window in the taskbar or dock when a long run ends
//! out of sight. A browser tab has no such mark, so a reader in another tab
//! learns nothing until they come back. The browser's notification is the
//! only signal that reaches them, and a page may only show one with the
//! site's permission, which it may only ask for when the reader acts. So it
//! is off until they turn it on from the notification panel, and the choice
//! is kept in this browser, beside the permission it depends on.

/// Work shorter than this was watched rather than waited for, so its end
/// reaches for the reader outside the window on neither host.
pub(crate) const AWAY_AFTER_SECONDS: f64 = 5.0;

/// Whether a run that ends out of sight raises the browser's notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BackgroundAlerts {
    /// There is no browser notification to raise: a desktop build, a
    /// browser without the API, or a phone's Chromium, which shows a page's
    /// notification only through a service worker.
    Unavailable,
    /// The site may not notify. Only the browser's site settings can allow
    /// it again; a page cannot ask twice.
    Blocked,
    Off,
    /// Turned on, and the browser's permission prompt is not answered yet.
    Asking,
    On,
}

impl BackgroundAlerts {
    /// What the panel's switch shows: the reader's choice, granted or not.
    pub(crate) const fn is_chosen(self) -> bool {
        matches!(self, Self::Asking | Self::On)
    }

    /// How the browser's answers combine. `permission` is the site's:
    /// granted, refused, or `None` while it has not been asked.
    const fn resolve(reachable: bool, permission: Option<bool>, chosen: bool) -> Self {
        match (reachable, permission, chosen) {
            (false, _, _) => Self::Unavailable,
            (true, Some(false), _) => Self::Blocked,
            (true, _, false) => Self::Off,
            (true, None, true) => Self::Asking,
            (true, Some(true), true) => Self::On,
        }
    }
}

/// Where this browser stands.
#[cfg(target_arch = "wasm32")]
pub(crate) fn background_alerts() -> BackgroundAlerts {
    let reachable = web::notifications_reach_the_reader();
    BackgroundAlerts::resolve(
        reachable,
        reachable.then(web::permission).flatten(),
        web::chosen(),
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const fn background_alerts() -> BackgroundAlerts {
    BackgroundAlerts::resolve(false, None, false)
}

/// Keep the reader's choice and, turning alerts on, ask the browser for the
/// permission they need. Called from the press that made the choice, which
/// is what lets the browser show its prompt.
#[cfg(target_arch = "wasm32")]
pub(crate) fn set_background_alerts(ctx: &egui::Context, on: bool) {
    web::choose(on);
    if on && background_alerts() == BackgroundAlerts::Asking {
        web::ask(ctx);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn set_background_alerts(_ctx: &egui::Context, _on: bool) {}

/// Raise the browser's notification for an outcome, if the reader turned
/// alerts on and the browser allows them. Choosing it brings the tab back.
#[cfg(target_arch = "wasm32")]
pub(crate) fn alert(title: &str, body: &str) {
    if background_alerts() == BackgroundAlerts::On {
        web::notify(title, body);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn alert(_title: &str, _body: &str) {}

/// Whether the reader is at the page: its tab is in front and the browser
/// window has the keyboard. The browser's frame loop says neither in the
/// window focus a desktop window reports, so the page is asked directly.
/// `None` off the browser, where the window reports its own focus.
#[cfg(target_arch = "wasm32")]
pub(crate) fn reader_present() -> Option<bool> {
    let document = web_sys::window()?.document()?;
    Some(!document.hidden() && document.has_focus().unwrap_or(true))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const fn reader_present() -> Option<bool> {
    None
}

#[cfg(target_arch = "wasm32")]
mod web {
    use wasm_bindgen::{JsCast as _, JsValue};
    use web_sys::{Notification, NotificationOptions, NotificationPermission};

    /// The reader's choice, in this browser's storage for the site.
    const CHOICE_KEY: &str = "rspice.background-run-alerts";

    /// Whether a page's notification can reach the reader here. Chromium on
    /// a phone has the API but refuses to construct one outside a service
    /// worker, and offering a switch that can never alert would be a lie.
    pub(super) fn notifications_reach_the_reader() -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let get = |target: &JsValue, name: &str| {
            js_sys::Reflect::get(target, &JsValue::from_str(name)).unwrap_or(JsValue::UNDEFINED)
        };
        let agent = get(&get(window.as_ref(), "navigator"), "userAgentData");
        let phone_chromium = agent.is_object() && get(&agent, "mobile").is_truthy();
        js_sys::Reflect::has(window.as_ref(), &JsValue::from_str("Notification")).unwrap_or(false)
            && !phone_chromium
    }

    /// Only called where `Notification` exists: reading it elsewhere throws.
    pub(super) fn permission() -> Option<bool> {
        match Notification::permission() {
            NotificationPermission::Granted => Some(true),
            NotificationPermission::Denied => Some(false),
            _ => None,
        }
    }

    pub(super) fn ask(ctx: &egui::Context) {
        match Notification::request_permission() {
            Ok(answer) => {
                // The switch reads the permission each frame; the answer only
                // has to bring the next one.
                let ctx = ctx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(answer).await;
                    ctx.request_repaint();
                });
            }
            Err(error) => log::warn!("The browser refused to ask for notifications: {error:?}"),
        }
    }

    pub(super) fn notify(title: &str, body: &str) {
        let options = NotificationOptions::new();
        options.set_body(body);
        let notification = match Notification::new_with_options(title, &options) {
            Ok(notification) => notification,
            Err(error) => {
                log::warn!("The browser refused a run notification: {error:?}");
                return;
            }
        };
        let shown = notification.clone();
        let bring_back = wasm_bindgen::closure::Closure::once_into_js(move || {
            if let Some(window) = web_sys::window() {
                let _ = window.focus();
            }
            shown.close();
        });
        notification.set_onclick(Some(bring_back.unchecked_ref()));
    }

    pub(super) fn chosen() -> bool {
        storage()
            .and_then(|storage| storage.get_item(CHOICE_KEY).ok().flatten())
            .is_some_and(|choice| choice == "on")
    }

    pub(super) fn choose(on: bool) {
        let Some(storage) = storage() else {
            return;
        };
        let kept = if on {
            storage.set_item(CHOICE_KEY, "on")
        } else {
            storage.remove_item(CHOICE_KEY)
        };
        if let Err(error) = kept {
            log::warn!("The browser did not keep the run alert choice: {error:?}");
        }
    }

    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::BackgroundAlerts;

    #[test]
    fn the_switch_follows_the_choice_and_the_site_permission() {
        use BackgroundAlerts::{Asking, Blocked, Off, On, Unavailable};
        for (reachable, permission, chosen, expected) in [
            (false, Some(true), true, Unavailable),
            (true, Some(false), true, Blocked),
            (true, Some(false), false, Blocked),
            (true, Some(true), false, Off),
            (true, None, false, Off),
            (true, None, true, Asking),
            (true, Some(true), true, On),
        ] {
            assert_eq!(
                BackgroundAlerts::resolve(reachable, permission, chosen),
                expected,
                "{reachable} {permission:?} {chosen}"
            );
        }
        assert!(On.is_chosen() && Asking.is_chosen());
        assert!(!Off.is_chosen() && !Blocked.is_chosen() && !Unavailable.is_chosen());
    }
}
