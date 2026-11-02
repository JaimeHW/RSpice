//! Check for updates: the release manifest rspice.app publishes, fetched once
//! when the reader asks, read, and compared with this build's version.
//!
//! The fetch is one HTTPS GET of [`MANIFEST_URL`] with the User-Agent
//! `RSpice/<version>` and nothing else: no cookies, no identifiers, no body.
//! It gives up after ten seconds. What comes back is believed only so far:
//! a version that is not SemVer, a date that is not a calendar day, and a
//! link that is not an https page of rspice.app each refuse the manifest,
//! so the application never opens a page the network chose.

use std::cmp::Ordering;

/// Where rspice.app publishes the latest release.
pub const MANIFEST_URL: &str = "https://rspice.app/releases/latest.json";

/// The one host the manifest, and every page it names, is on.
const SITE_HOST: &str = "rspice.app";

/// This build's version, as the manifest's is compared with.
pub const INSTALLED_VERSION: &str = env!("CARGO_PKG_VERSION");

/// How long the check waits for rspice.app, connecting and reading alike.
#[cfg(not(target_arch = "wasm32"))]
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

/// The one header the check sets.
#[cfg(not(target_arch = "wasm32"))]
const USER_AGENT: &str = concat!("RSpice/", env!("CARGO_PKG_VERSION"));

/// The most the check reads of an answer. The manifest is a few hundred
/// bytes; anything near this is not one.
const MAX_MANIFEST_BYTES: usize = 64 * 1024;

/// A release the manifest announces, every field checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    /// The day it was released, `YYYY-MM-DD`.
    pub date: String,
    /// An https page of rspice.app to download it from.
    pub download_url: String,
    /// An https page of rspice.app with its release notes.
    pub notes_url: String,
}

/// Why a check could not say whether a newer version is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckFailure {
    /// No connection to rspice.app could be made, or it broke off.
    Unreachable,
    /// rspice.app did not answer in time.
    TimedOut,
    /// rspice.app answered with this HTTP status instead of the manifest.
    Status(u16),
    /// The answer is not a manifest: not JSON, a field missing or of the
    /// wrong kind, a date that is not a day, or far too long.
    Unreadable,
    /// The manifest's version is not a SemVer version.
    Version,
    /// The manifest names a page that is not an https page of rspice.app.
    ForeignPage,
}

impl CheckFailure {
    /// The failure as a sentence for the reader.
    pub fn sentence(self) -> String {
        match self {
            Self::Unreachable => {
                "rspice.app could not be reached. Check the network connection and try again."
                    .to_owned()
            }
            Self::TimedOut => "rspice.app did not answer within 10 seconds.".to_owned(),
            Self::Status(status) => {
                format!("rspice.app could not provide the latest release (HTTP {status}).")
            }
            Self::Unreadable => {
                "The release information from rspice.app could not be read.".to_owned()
            }
            Self::Version => {
                "The release information from rspice.app has no valid version number.".to_owned()
            }
            Self::ForeignPage => {
                "The release information named a page outside rspice.app, so it was not used."
                    .to_owned()
            }
        }
    }
}

/// The manifest as rspice.app writes it. Keys it adds later are ignored.
#[derive(serde::Deserialize)]
struct Manifest {
    version: String,
    date: String,
    download_url: String,
    notes_url: String,
}

/// Read a manifest's bytes into the release it announces, or say why they
/// are not one.
pub fn read_manifest(body: &[u8]) -> Result<Release, CheckFailure> {
    if body.len() > MAX_MANIFEST_BYTES {
        return Err(CheckFailure::Unreadable);
    }
    let manifest: Manifest = serde_json::from_slice(body).map_err(|_| CheckFailure::Unreadable)?;
    if SemVer::parse(&manifest.version).is_none() {
        return Err(CheckFailure::Version);
    }
    if !is_calendar_day(&manifest.date) {
        return Err(CheckFailure::Unreadable);
    }
    if !is_site_page(&manifest.download_url) || !is_site_page(&manifest.notes_url) {
        return Err(CheckFailure::ForeignPage);
    }
    Ok(Release {
        version: manifest.version,
        date: manifest.date,
        download_url: manifest.download_url,
        notes_url: manifest.notes_url,
    })
}

/// Whether `release` is strictly newer than `installed` by SemVer
/// precedence. An equal version, one that differs only in build metadata and
/// a pre-release of the installed version are not; a version that does not
/// parse never is.
pub fn is_newer(release: &str, installed: &str) -> bool {
    match (SemVer::parse(release), SemVer::parse(installed)) {
        (Some(release), Some(installed)) => release.cmp(&installed) == Ordering::Greater,
        _ => false,
    }
}

/// Whether `url` is an https page of rspice.app, with no credentials and
/// no port of its own.
pub fn is_site_page(url: &str) -> bool {
    url::Url::parse(url).is_ok_and(|url| is_site_url(&url))
}

fn is_site_url(url: &url::Url) -> bool {
    url.scheme() == "https"
        && url.host_str() == Some(SITE_HOST)
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
}

/// Whether `date` is `YYYY-MM-DD` naming a real day.
fn is_calendar_day(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |range: std::ops::Range<usize>| -> Option<u16> {
        let text = date.get(range)?;
        if text.bytes().all(|byte| byte.is_ascii_digit()) {
            text.parse().ok()
        } else {
            None
        }
    };
    let (Some(year), Some(month), Some(day)) = (digits(0..4), digits(5..7), digits(8..10)) else {
        return false;
    };
    let (Ok(month), Ok(day)) = (u8::try_from(month), u8::try_from(day)) else {
        return false;
    };
    let Ok(month) = time::Month::try_from(month) else {
        return false;
    };
    time::Date::from_calendar_date(i32::from(year), month, day).is_ok()
}

/// A SemVer 2.0.0 version, for precedence: the core and the pre-release
/// identifiers. Build metadata is checked and then set aside, as SemVer
/// gives it no part in precedence.
#[derive(Debug, PartialEq, Eq)]
struct SemVer {
    core: [u64; 3],
    pre: Vec<Identifier>,
}

#[derive(Debug, PartialEq, Eq)]
enum Identifier {
    Numeric(u64),
    Alphanumeric(String),
}

impl SemVer {
    fn parse(text: &str) -> Option<Self> {
        let (rest, build) = match text.split_once('+') {
            Some((rest, build)) => (rest, Some(build)),
            None => (text, None),
        };
        if let Some(build) = build
            && !build.split('.').all(|part| {
                !part.is_empty()
                    && part
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            })
        {
            return None;
        }
        let (core, pre) = match rest.split_once('-') {
            Some((core, pre)) => (core, Some(pre)),
            None => (rest, None),
        };
        let mut numbers = core.split('.');
        let mut core = [0u64; 3];
        for slot in &mut core {
            *slot = numeric(numbers.next()?)?;
        }
        if numbers.next().is_some() {
            return None;
        }
        let pre = match pre {
            None => Vec::new(),
            Some(pre) => pre
                .split('.')
                .map(|part| {
                    if part.is_empty()
                        || !part
                            .bytes()
                            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                    {
                        None
                    } else if part.bytes().all(|byte| byte.is_ascii_digit()) {
                        numeric(part).map(Identifier::Numeric)
                    } else {
                        Some(Identifier::Alphanumeric(part.to_owned()))
                    }
                })
                .collect::<Option<Vec<_>>>()?,
        };
        Some(Self { core, pre })
    }
}

/// A numeric identifier: digits, without a leading zero unless it is zero.
fn numeric(text: &str) -> Option<u64> {
    if text.is_empty()
        || !text.bytes().all(|byte| byte.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return None;
    }
    text.parse().ok()
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.core.cmp(&other.core).then_with(|| {
            match (self.pre.is_empty(), other.pre.is_empty()) {
                (true, true) => Ordering::Equal,
                // A pre-release comes before the release it leads to.
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => self.pre.cmp(&other.pre),
            }
        })
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Numeric(left), Self::Numeric(right)) => left.cmp(right),
            (Self::Numeric(_), Self::Alphanumeric(_)) => Ordering::Less,
            (Self::Alphanumeric(_), Self::Numeric(_)) => Ordering::Greater,
            (Self::Alphanumeric(left), Self::Alphanumeric(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Fetch the manifest's bytes: one GET of [`MANIFEST_URL`], on a runtime of
/// its own, so the caller runs it off the frame's thread.
///
/// A redirect is followed only to another https page of rspice.app, at most
/// twice; anything else ends the check with the redirect's own status.
#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_manifest() -> Result<Vec<u8>, CheckFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| CheckFailure::Unreachable)?;
    runtime.block_on(async {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(TIMEOUT)
            .redirect(reqwest::redirect::Policy::custom(|attempt| {
                if attempt.previous().len() <= 2 && is_site_url(attempt.url()) {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            }))
            .build()
            .map_err(|_| CheckFailure::Unreachable)?;
        let response = client
            .get(MANIFEST_URL)
            .send()
            .await
            .map_err(transport_failure)?;
        let status = response.status();
        if !status.is_success() {
            return Err(CheckFailure::Status(status.as_u16()));
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_MANIFEST_BYTES as u64)
        {
            return Err(CheckFailure::Unreadable);
        }
        let body = response.bytes().await.map_err(transport_failure)?;
        Ok(body.to_vec())
    })
}

/// The browser is always served the current release, so it never checks.
#[cfg(target_arch = "wasm32")]
pub fn fetch_manifest() -> Result<Vec<u8>, CheckFailure> {
    Err(CheckFailure::Unreachable)
}

#[cfg(not(target_arch = "wasm32"))]
fn transport_failure(error: reqwest::Error) -> CheckFailure {
    if error.is_timeout() {
        CheckFailure::TimedOut
    } else {
        CheckFailure::Unreachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"{"version": "0.2.0", "date": "2026-11-01", "download_url": "https://rspice.app/download", "notes_url": "https://rspice.app/changelog#v0.2.0"}"#;

    #[test]
    fn a_manifest_reads_into_the_release_it_announces_and_ignores_keys_it_does_not_know() {
        assert_eq!(
            read_manifest(MANIFEST.as_bytes()),
            Ok(Release {
                version: "0.2.0".to_owned(),
                date: "2026-11-01".to_owned(),
                download_url: "https://rspice.app/download".to_owned(),
                notes_url: "https://rspice.app/changelog#v0.2.0".to_owned(),
            })
        );
        let extra = MANIFEST.replace('}', r#", "sha256": "ab12", "channel": "stable"}"#);
        assert!(read_manifest(extra.as_bytes()).is_ok());
    }

    #[test]
    fn a_body_that_is_not_a_manifest_is_unreadable() {
        for body in [
            "",
            "not json",
            "<!doctype html><html></html>",
            "[]",
            r#"{"version": "0.2.0"}"#,
            r#"{"version": 2, "date": "2026-11-01", "download_url": "https://rspice.app/download", "notes_url": "https://rspice.app/changelog"}"#,
            r#"{"version": "0.2.0", "date": "2026-02-30", "download_url": "https://rspice.app/download", "notes_url": "https://rspice.app/changelog"}"#,
            r#"{"version": "0.2.0", "date": "1 November 2026", "download_url": "https://rspice.app/download", "notes_url": "https://rspice.app/changelog"}"#,
        ] {
            assert_eq!(
                read_manifest(body.as_bytes()),
                Err(CheckFailure::Unreadable),
                "{body}"
            );
        }
        let padded = format!("{MANIFEST}{}", " ".repeat(MAX_MANIFEST_BYTES));
        assert_eq!(
            read_manifest(padded.as_bytes()),
            Err(CheckFailure::Unreadable)
        );
    }

    #[test]
    fn a_version_that_is_not_semver_refuses_the_manifest() {
        for version in [
            "0.2",
            "v0.2.0",
            "0.2.0.1",
            "01.2.0",
            "0.2.0-",
            "0.2.0-beta..1",
            "latest",
        ] {
            let body = MANIFEST.replace("\"0.2.0\"", &format!("\"{version}\""));
            assert_eq!(
                read_manifest(body.as_bytes()),
                Err(CheckFailure::Version),
                "{version}"
            );
        }
    }

    #[test]
    fn a_manifest_naming_any_page_but_an_https_page_of_rspice_app_is_refused() {
        for url in [
            "http://rspice.app/download",
            "https://rspice.app.example.com/download",
            "https://example.com/rspice.app",
            "https://www.rspice.app/download",
            "https://rspice.app:8443/download",
            "https://user@rspice.app/download",
            "javascript:alert(1)",
            "file:///C:/rspice.exe",
            "/download",
        ] {
            for key in ["download_url", "notes_url"] {
                let body = match key {
                    "download_url" => MANIFEST.replace("https://rspice.app/download", url),
                    _ => MANIFEST.replace("https://rspice.app/changelog#v0.2.0", url),
                };
                assert_eq!(
                    read_manifest(body.as_bytes()),
                    Err(CheckFailure::ForeignPage),
                    "{key}: {url}"
                );
            }
        }
        assert!(is_site_page("https://rspice.app/changelog#v0.1.0"));
        assert!(is_site_page("https://RSPICE.APP/download"));
    }

    /// Only a strictly newer version counts: equal versions, build metadata
    /// and pre-releases of what is installed are up to date.
    #[test]
    fn only_a_strictly_newer_version_is_newer() {
        let table = [
            ("0.1.0", "0.1.0", false),
            ("0.1.0+build.7", "0.1.0", false),
            ("0.1.0-rc.1", "0.1.0", false),
            ("0.0.9", "0.1.0", false),
            ("0.1.1", "0.1.0", true),
            ("0.2.0", "0.1.0", true),
            ("1.0.0", "0.9.9", true),
            ("0.10.0", "0.9.0", true),
            ("0.2.0-beta.1", "0.1.0", true),
            ("0.2.0-beta.2", "0.2.0-beta.1", true),
            ("0.2.0-beta.11", "0.2.0-beta.2", true),
            ("0.2.0-beta", "0.2.0-alpha.9", true),
            ("0.2.0-alpha.1", "0.2.0-alpha", true),
            ("0.2.0-alpha", "0.2.0-1", true),
            ("0.2.0", "0.2.0-rc.1", true),
            ("0.2.0-rc.1", "0.2.0", false),
            ("garbage", "0.1.0", false),
            ("0.2.0", "garbage", false),
        ];
        for (release, installed, newer) in table {
            assert_eq!(
                is_newer(release, installed),
                newer,
                "{release} against {installed}"
            );
        }
    }

    #[test]
    fn every_failure_reads_as_a_sentence() {
        for failure in [
            CheckFailure::Unreachable,
            CheckFailure::TimedOut,
            CheckFailure::Status(503),
            CheckFailure::Unreadable,
            CheckFailure::Version,
            CheckFailure::ForeignPage,
        ] {
            let sentence = failure.sentence();
            assert!(
                sentence.ends_with('.') && !sentence.contains("Error") && !sentence.contains("::"),
                "{sentence}"
            );
        }
        assert_eq!(
            CheckFailure::Status(503).sentence(),
            "rspice.app could not provide the latest release (HTTP 503)."
        );
    }
}
