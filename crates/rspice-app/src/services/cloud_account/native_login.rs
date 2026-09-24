//! Loopback redirect listener for the native sign-in flow (RFC 8252).
//!
//! Sign-in happens in the system browser; the identity provider redirects to
//! `http://127.0.0.1:{port}/oauth/callback`, which this listener answers once
//! with a plain confirmation page. Only the registered ports are attempted —
//! the provider matches redirect URIs exactly — and the listener accepts
//! nothing but the expected callback path carrying the expected `state`.
//! The authorization code is useless without this process's in-memory PKCE
//! verifier, so a hostile local page cannot redeem an intercepted redirect.

use std::io::{Read as _, Write as _};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::CloudAccountCommand;

/// How long the browser gets before an unanswered sign-in attempt expires.
const SIGN_IN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(5 * 60);
/// Poll cadence for cancellation while waiting on connections.
const ACCEPT_POLL: std::time::Duration = std::time::Duration::from_millis(50);
/// Upper bound on a callback request head; anything larger is hostile.
const MAX_REQUEST_BYTES: usize = 8 * 1024;

pub(super) struct LoopbackListener {
    listener: std::net::TcpListener,
    port: u16,
}

impl LoopbackListener {
    /// Bind the first free registered port on 127.0.0.1.
    pub(super) fn bind(ports: &[u16]) -> std::io::Result<Self> {
        let mut last_error =
            std::io::Error::new(std::io::ErrorKind::AddrInUse, "no loopback port configured");
        for &port in ports {
            match std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)) {
                Ok(listener) => {
                    listener.set_nonblocking(true)?;
                    return Ok(Self { listener, port });
                }
                Err(error) => last_error = error,
            }
        }
        Err(last_error)
    }

    /// The redirect URI registered for the bound port.
    pub(super) fn redirect_uri(&self) -> String {
        format!("http://127.0.0.1:{}/oauth/callback", self.port)
    }

    /// Wait for the provider redirect and deliver the outcome as a command.
    ///
    /// Runs on its own thread. Ends after one delivered outcome, on `cancel`,
    /// or at the deadline (delivering a failure so the session leaves the
    /// waiting state).
    pub(super) fn run(
        self,
        expected_state: String,
        cancel: Arc<AtomicBool>,
        commands: std::sync::mpsc::SyncSender<CloudAccountCommand>,
    ) {
        let deadline = std::time::Instant::now() + SIGN_IN_DEADLINE;
        loop {
            if cancel.load(Ordering::Relaxed) {
                return;
            }
            if std::time::Instant::now() >= deadline {
                let _ = commands.send(CloudAccountCommand::SignInFailed {
                    reason: "Sign-in timed out before the browser completed it.".to_owned(),
                });
                return;
            }
            let mut stream = match self.listener.accept() {
                Ok((stream, _)) => stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(ACCEPT_POLL);
                    continue;
                }
                Err(_) => {
                    let _ = commands.send(CloudAccountCommand::SignInFailed {
                        reason: "The sign-in listener failed.".to_owned(),
                    });
                    return;
                }
            };
            // `TcpListener::set_nonblocking(true)` is required so this thread
            // can observe cancellation. Whether an accepted socket inherits
            // that mode is platform-specific, so make the synchronous HTTP
            // parser's contract explicit before its first read. Otherwise a
            // valid callback can race the first read and be discarded after a
            // transient `WouldBlock` on Windows.
            if stream.set_nonblocking(false).is_err() {
                let _ = commands.send(CloudAccountCommand::SignInFailed {
                    reason: "The sign-in callback connection could not be configured.".to_owned(),
                });
                return;
            }
            match handle_connection(&mut stream, &expected_state) {
                CallbackOutcome::Unrelated => {}
                CallbackOutcome::Delivered(command) => {
                    let _ = commands.send(command);
                    return;
                }
            }
        }
    }
}

enum CallbackOutcome {
    /// Not the callback (favicon probe, wrong path); keep listening.
    Unrelated,
    /// A terminal outcome was produced and answered.
    Delivered(CloudAccountCommand),
}

fn handle_connection(stream: &mut std::net::TcpStream, expected_state: &str) -> CallbackOutcome {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(5)));
    let Some(target) = read_request_target(stream) else {
        respond(stream, 400, "The sign-in callback request was malformed.");
        return CallbackOutcome::Unrelated;
    };
    let Ok(parsed) = url::Url::parse(&format!("http://127.0.0.1{target}")) else {
        respond(stream, 400, "The sign-in callback request was malformed.");
        return CallbackOutcome::Unrelated;
    };
    if parsed.path() != "/oauth/callback" {
        respond(stream, 404, "Not the RSpice sign-in callback.");
        return CallbackOutcome::Unrelated;
    }

    let mut code = None;
    let mut state = None;
    let mut provider_error = None;
    for (key, value) in parsed.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.into_owned()),
            "state" => state = Some(value.into_owned()),
            "error" => provider_error = Some(value.into_owned()),
            _ => {}
        }
    }

    if state.as_deref() != Some(expected_state) {
        // Without the right state this request is not our flow; refuse it
        // and keep waiting for the genuine redirect.
        respond(
            stream,
            400,
            "This sign-in response does not match the pending attempt.",
        );
        return CallbackOutcome::Unrelated;
    }
    if let Some(error) = provider_error {
        respond(
            stream,
            200,
            "Sign-in was not completed. You can close this tab and return to RSpice.",
        );
        let reason = if error == "access_denied" {
            "Sign-in was cancelled in the browser.".to_owned()
        } else {
            "The sign-in service reported an error.".to_owned()
        };
        return CallbackOutcome::Delivered(CloudAccountCommand::SignInFailed { reason });
    }
    let Some(code) = code.filter(|value| !value.is_empty()) else {
        respond(
            stream,
            400,
            "The sign-in response carried no authorization code.",
        );
        return CallbackOutcome::Delivered(CloudAccountCommand::SignInFailed {
            reason: "The sign-in response carried no authorization code.".to_owned(),
        });
    };
    respond(
        stream,
        200,
        "You are signed in. You can close this tab and return to RSpice.",
    );
    CallbackOutcome::Delivered(CloudAccountCommand::CompleteSignIn {
        code,
        state: expected_state.to_owned(),
    })
}

/// Read the request head and return the target of `GET <target> HTTP/1.1`.
fn read_request_target(stream: &mut std::net::TcpStream) -> Option<String> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !buffer.windows(4).any(|window| window == b"\r\n\r\n") {
        if buffer.len() > MAX_REQUEST_BYTES {
            return None;
        }
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => buffer.extend_from_slice(&chunk[..read]),
            Err(_) => return None,
        }
    }
    let head = std::str::from_utf8(&buffer).ok()?;
    let request_line = head.lines().next()?;
    let mut parts = request_line.split(' ');
    if parts.next() != Some("GET") {
        return None;
    }
    let target = parts.next()?;
    if parts.next()?.starts_with("HTTP/") {
        Some(target.to_owned())
    } else {
        None
    }
}

fn respond(stream: &mut std::net::TcpStream, status: u16, message: &str) {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Bad Request",
    };
    let body = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <title>RSpice sign-in</title></head>\
         <body style=\"font-family:system-ui,sans-serif;margin:4rem auto;max-width:36rem\">\
         <h1 style=\"font-size:1.2rem\">RSpice</h1><p>{message}</p></body></html>"
    );
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_listener_against(request: &str) -> Option<CloudAccountCommand> {
        // Bind an OS-assigned ephemeral port directly for the test.
        let raw = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        raw.set_nonblocking(true).unwrap();
        let port = raw.local_addr().unwrap().port();
        let listener = LoopbackListener {
            listener: raw,
            port,
        };

        let (sender, receiver) = std::sync::mpsc::sync_channel(4);
        let cancel = Arc::new(AtomicBool::new(false));
        let request = request.replace("{port}", &port.to_string());
        let handle = {
            let cancel = Arc::clone(&cancel);
            std::thread::spawn(move || listener.run("expected-state".to_owned(), cancel, sender))
        };
        {
            let mut stream =
                std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)).unwrap();
            stream.write_all(request.as_bytes()).unwrap();
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response);
        }
        let outcome = receiver
            .recv_timeout(std::time::Duration::from_secs(5))
            .ok();
        cancel.store(true, Ordering::Relaxed);
        let _ = handle.join();
        outcome
    }

    #[test]
    fn a_valid_callback_delivers_the_code() {
        let outcome = run_listener_against(
            "GET /oauth/callback?code=the-code&state=expected-state HTTP/1.1\r\n\
             Host: 127.0.0.1:{port}\r\n\r\n",
        );
        assert_eq!(
            outcome,
            Some(CloudAccountCommand::CompleteSignIn {
                code: "the-code".to_owned(),
                state: "expected-state".to_owned(),
            })
        );
    }

    #[test]
    fn valid_callback_delivery_is_stable_across_ephemeral_listeners() {
        for attempt in 0..32 {
            let outcome = run_listener_against(
                "GET /oauth/callback?code=the-code&state=expected-state HTTP/1.1\r\n\
                 Host: 127.0.0.1:{port}\r\n\r\n",
            );
            assert_eq!(
                outcome,
                Some(CloudAccountCommand::CompleteSignIn {
                    code: "the-code".to_owned(),
                    state: "expected-state".to_owned(),
                }),
                "valid callback attempt {attempt} was not delivered"
            );
        }
    }

    #[test]
    fn a_denied_grant_reports_cancellation() {
        let outcome = run_listener_against(
            "GET /oauth/callback?error=access_denied&state=expected-state HTTP/1.1\r\n\
             Host: 127.0.0.1:{port}\r\n\r\n",
        );
        assert_eq!(
            outcome,
            Some(CloudAccountCommand::SignInFailed {
                reason: "Sign-in was cancelled in the browser.".to_owned(),
            })
        );
    }

    #[test]
    fn a_wrong_state_is_ignored_and_the_flow_keeps_waiting() {
        let raw = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        raw.set_nonblocking(true).unwrap();
        let port = raw.local_addr().unwrap().port();
        let listener = LoopbackListener {
            listener: raw,
            port,
        };
        let (sender, receiver) = std::sync::mpsc::sync_channel(4);
        let cancel = Arc::new(AtomicBool::new(false));
        let handle = {
            let cancel = Arc::clone(&cancel);
            std::thread::spawn(move || listener.run("expected-state".to_owned(), cancel, sender))
        };
        {
            let mut stream =
                std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)).unwrap();
            stream
                .write_all(b"GET /oauth/callback?code=x&state=forged HTTP/1.1\r\nHost: h\r\n\r\n")
                .unwrap();
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response);
            assert!(response.starts_with("HTTP/1.1 400"));
        }
        assert!(
            receiver
                .recv_timeout(std::time::Duration::from_millis(400))
                .is_err(),
            "a forged state must not end the pending sign-in"
        );
        cancel.store(true, Ordering::Relaxed);
        let _ = handle.join();
    }
}
