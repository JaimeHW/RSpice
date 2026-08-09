//! Browser WebSocket transport for the authenticated live-session v2 relay.
//!
//! Browser sockets expose an internal send buffer and callback-based receive
//! API. Both are bounded here: the Rust queues have finite capacity and a
//! socket whose JavaScript buffer stops draining is closed for an
//! authoritative resynchronization. The one-use ticket exists only in the
//! WebSocket subprotocol constructor argument and is never rendered, logged,
//! placed in a URL, or persisted.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc::{SyncSender, TrySendError};

use rspice_cloud_client::contract::LIVE_SESSION_PROTOCOL;
use wasm_bindgen::JsCast as _;
use wasm_bindgen::closure::Closure;

use super::{
    CloudAccountCommand, LiveFrame, LiveFrameClass, LiveRelayClosure, MAX_LIVE_FRAME_BYTES,
    MAX_LIVE_RELAY_WIRE_BYTES,
};

/// Maximum bytes admitted into the browser-owned WebSocket send buffer.
const MAX_BROWSER_BUFFERED_BYTES: u32 = 2 * 1024 * 1024;
const BUFFER_DRAIN_POLL_MS: u32 = 10;
const BUFFER_DRAIN_ATTEMPTS: usize = 500;
const CONNECT_POLL_MS: u32 = 10;
const CONNECT_ATTEMPTS: usize = 1_000;

/// Monotonic socket identity plus the URL and secret protocol offer.
pub(super) struct BrowserRelayConnection {
    pub url: String,
    pub ticket_protocol: String,
    pub generation: u64,
}

/// The live browser socket and its retained JavaScript callbacks.
pub(super) struct BrowserRelayHandle {
    pub generation: u64,
    pub attached: bool,
    socket: web_sys::WebSocket,
    local_stop: Rc<Cell<bool>>,
    _on_open: Closure<dyn FnMut(web_sys::Event)>,
    _on_message: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _on_error: Closure<dyn FnMut(web_sys::Event)>,
    _on_close: Closure<dyn FnMut(web_sys::CloseEvent)>,
}

impl BrowserRelayHandle {
    pub(super) fn stop(&self) {
        self.local_stop.set(true);
        let _ = self.socket.close_with_code_and_reason(1000, "local close");
    }
}

impl Drop for BrowserRelayHandle {
    fn drop(&mut self) {
        self.stop();
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        self.socket.set_onerror(None);
        self.socket.set_onclose(None);
    }
}

pub(super) fn relay_endpoint(api_origin: &str, websocket_endpoint: &str) -> Option<String> {
    let mut url = url::Url::parse(api_origin).ok()?;
    let scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        _ => return None,
    };
    url.set_scheme(scheme).ok()?;
    url.set_path(websocket_endpoint);
    Some(url.into())
}

pub(super) fn spawn(
    connection: BrowserRelayConnection,
    mut outbound: tokio::sync::mpsc::Receiver<LiveFrame>,
    inbound: SyncSender<LiveFrame>,
    commands: tokio::sync::mpsc::Sender<CloudAccountCommand>,
    repaint: Option<egui::Context>,
) -> Result<BrowserRelayHandle, ()> {
    let protocols = js_sys::Array::new();
    protocols.push(&wasm_bindgen::JsValue::from_str(LIVE_SESSION_PROTOCOL));
    protocols.push(&wasm_bindgen::JsValue::from_str(
        &connection.ticket_protocol,
    ));
    let socket = web_sys::WebSocket::new_with_str_sequence(&connection.url, &protocols.into())
        .map_err(|_| ())?;
    socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let local_stop = Rc::new(Cell::new(false));
    let reported = Rc::new(Cell::new(false));

    let open_socket = socket.clone();
    let open_commands = commands.clone();
    let open_reported = Rc::clone(&reported);
    let open_repaint = repaint.clone();
    let generation = connection.generation;
    let on_open = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if open_socket.protocol() != LIVE_SESSION_PROTOCOL {
            if !open_reported.replace(true) {
                let _ = open_commands.try_send(CloudAccountCommand::LiveRelayClosed {
                    generation,
                    closure: LiveRelayClosure::Rejected,
                });
            }
            let _ = open_socket.close_with_code_and_reason(1002, "subprotocol mismatch");
            return;
        }
        let _ = open_commands.try_send(CloudAccountCommand::LiveRelayAttached { generation });
        if let Some(repaint) = &open_repaint {
            repaint.request_repaint();
        }
    }) as Box<dyn FnMut(_)>);
    socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));

    let message_socket = socket.clone();
    let message_commands = commands.clone();
    let message_reported = Rc::clone(&reported);
    let message_repaint = repaint.clone();
    let on_message = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        let data = event.data();
        if !data.is_instance_of::<js_sys::ArrayBuffer>() {
            report_and_close(
                &message_socket,
                &message_commands,
                &message_reported,
                generation,
                LiveRelayClosure::Rejected,
                1003,
                "binary frames required",
            );
            return;
        }
        let view = js_sys::Uint8Array::new(&data);
        let length = usize::try_from(view.length()).expect("ArrayBuffer length fits usize");
        if length > MAX_LIVE_RELAY_WIRE_BYTES {
            report_and_close(
                &message_socket,
                &message_commands,
                &message_reported,
                generation,
                LiveRelayClosure::Rejected,
                1009,
                "frame too large",
            );
            return;
        }
        let bytes = view.to_vec();
        let Some(frame) = LiveFrame::decode_authenticated(&bytes) else {
            report_and_close(
                &message_socket,
                &message_commands,
                &message_reported,
                generation,
                LiveRelayClosure::Rejected,
                1002,
                "malformed relay frame",
            );
            return;
        };
        let disposable = matches!(
            frame.class,
            LiveFrameClass::Presence | LiveFrameClass::Cursor
        );
        match inbound.try_send(frame) {
            Ok(()) => {
                if let Some(repaint) = &message_repaint {
                    repaint.request_repaint();
                }
            }
            Err(TrySendError::Full(_)) if disposable => {}
            Err(TrySendError::Full(_)) => report_and_close(
                &message_socket,
                &message_commands,
                &message_reported,
                generation,
                LiveRelayClosure::Interrupted,
                1013,
                "client backpressure",
            ),
            Err(TrySendError::Disconnected(_)) => {
                let _ = message_socket.close_with_code_and_reason(1000, "local close");
            }
        }
    }) as Box<dyn FnMut(_)>);
    socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    // Browsers report useful connection detail through the subsequent close
    // event. Keep onerror intentionally non-reflective and let onclose own the
    // single lifecycle notification.
    let on_error = Closure::wrap(Box::new(move |_event: web_sys::Event| {}) as Box<dyn FnMut(_)>);
    socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    let close_commands = commands.clone();
    let close_reported = Rc::clone(&reported);
    let close_local = Rc::clone(&local_stop);
    let close_repaint = repaint;
    let on_close = Closure::wrap(Box::new(move |event: web_sys::CloseEvent| {
        if close_reported.replace(true) {
            return;
        }
        let closure = if close_local.get() {
            LiveRelayClosure::Local
        } else {
            closure_from_close(event.code(), &event.reason())
        };
        let _ = close_commands.try_send(CloudAccountCommand::LiveRelayClosed {
            generation,
            closure,
        });
        if let Some(repaint) = &close_repaint {
            repaint.request_repaint();
        }
    }) as Box<dyn FnMut(_)>);
    socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));

    let send_socket = socket.clone();
    let send_commands = commands;
    let send_reported = Rc::clone(&reported);
    let send_local = Rc::clone(&local_stop);
    wasm_bindgen_futures::spawn_local(async move {
        while let Some(frame) = outbound.recv().await {
            if send_local.get() {
                return;
            }
            let mut connect_attempts = 0;
            while send_socket.ready_state() == web_sys::WebSocket::CONNECTING {
                if send_local.get() {
                    return;
                }
                if connect_attempts >= CONNECT_ATTEMPTS {
                    report_and_close(
                        &send_socket,
                        &send_commands,
                        &send_reported,
                        generation,
                        LiveRelayClosure::Interrupted,
                        1013,
                        "connection timed out",
                    );
                    return;
                }
                connect_attempts += 1;
                gloo_timers::future::TimeoutFuture::new(CONNECT_POLL_MS).await;
            }
            if send_socket.ready_state() != web_sys::WebSocket::OPEN {
                // The close callback owns the lifecycle notification and
                // reconnect classification for sockets that never opened.
                return;
            }
            let encoded = frame.encode();
            if encoded.len() > MAX_LIVE_FRAME_BYTES {
                report_and_close(
                    &send_socket,
                    &send_commands,
                    &send_reported,
                    generation,
                    LiveRelayClosure::Rejected,
                    1009,
                    "outbound frame too large",
                );
                return;
            }
            let encoded_length = u32::try_from(encoded.len()).unwrap_or(u32::MAX);
            let mut attempts = 0;
            while send_socket.buffered_amount().saturating_add(encoded_length)
                > MAX_BROWSER_BUFFERED_BYTES
            {
                if send_local.get() {
                    return;
                }
                if attempts >= BUFFER_DRAIN_ATTEMPTS {
                    report_and_close(
                        &send_socket,
                        &send_commands,
                        &send_reported,
                        generation,
                        LiveRelayClosure::Interrupted,
                        1013,
                        "socket backpressure",
                    );
                    return;
                }
                attempts += 1;
                gloo_timers::future::TimeoutFuture::new(BUFFER_DRAIN_POLL_MS).await;
            }
            if send_socket.send_with_u8_array(&encoded).is_err() {
                report_and_close(
                    &send_socket,
                    &send_commands,
                    &send_reported,
                    generation,
                    LiveRelayClosure::Interrupted,
                    1011,
                    "send failed",
                );
                return;
            }
        }
        send_local.set(true);
        let _ = send_socket.close_with_code_and_reason(1000, "local close");
    });

    Ok(BrowserRelayHandle {
        generation,
        attached: false,
        socket,
        local_stop,
        _on_open: on_open,
        _on_message: on_message,
        _on_error: on_error,
        _on_close: on_close,
    })
}

#[allow(clippy::too_many_arguments)]
fn report_and_close(
    socket: &web_sys::WebSocket,
    commands: &tokio::sync::mpsc::Sender<CloudAccountCommand>,
    reported: &Cell<bool>,
    generation: u64,
    closure: LiveRelayClosure,
    code: u16,
    reason: &str,
) {
    if !reported.replace(true) {
        let _ = commands.try_send(CloudAccountCommand::LiveRelayClosed {
            generation,
            closure,
        });
    }
    let _ = socket.close_with_code_and_reason(code, reason);
}

fn closure_from_close(code: u16, reason: &str) -> LiveRelayClosure {
    match code {
        1000 => LiveRelayClosure::SessionOver {
            message: session_over_message(reason),
        },
        1002 | 1003 | 1009 => LiveRelayClosure::Rejected,
        _ => LiveRelayClosure::Interrupted,
    }
}

fn session_over_message(reason: &str) -> String {
    match reason {
        "removed from the live session" => "The host removed you from this live session.",
        "live session expired" => "This live session reached its maximum duration and ended.",
        _ => "This live session has ended.",
    }
    .to_owned()
}
