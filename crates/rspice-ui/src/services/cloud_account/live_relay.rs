//! Live-session relay socket (native).
//!
//! One thread per relay connection pumps binary frames both ways between the
//! WebSocket and the workbench's [`super::LiveRelayPort`]. The thread is
//! deliberately dumb: it never retries, re-mints, or interprets payloads —
//! the executor orchestrates reconnection because every reconnect needs a
//! freshly minted single-use ticket, and payload protocols belong to the
//! peers. The connect credential enters this module only inside the
//! subprotocol offer and is never logged or echoed.

use std::sync::mpsc::Sender;

use futures_util::{SinkExt, StreamExt};
use rspice_cloud_client::contract::LIVE_SESSION_PROTOCOL;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

use super::{CloudAccountCommand, LiveFrame, LiveRelayClosure, MAX_LIVE_FRAME_BYTES};

/// Everything one socket connection needs. Carries the bearer ticket inside
/// `protocols`, so this type must never derive or implement `Debug`.
pub(super) struct LiveRelayConnection {
    pub url: url::Url,
    /// Exact `Origin` header value; the relay enforces an allow-list and
    /// native user agents send none by default.
    pub origin: String,
    /// The full `Sec-WebSocket-Protocol` offer, ticket included.
    pub protocols: String,
    /// Monotonic connection identity echoed on every notification.
    pub generation: u64,
}

/// Derive the socket URL and the exact `Origin` header from the release API
/// origin. The endpoint path was already pinned by the client crate's ticket
/// validation, so this only maps the scheme onto its socket counterpart.
pub(super) fn relay_endpoint(
    api_origin: &str,
    websocket_endpoint: &str,
) -> Option<(url::Url, String)> {
    let mut url = url::Url::parse(api_origin).ok()?;
    let scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        _ => return None,
    };
    let origin = url.origin().ascii_serialization();
    url.set_scheme(scheme).ok()?;
    url.set_path(websocket_endpoint);
    Some((url, origin))
}

/// The single comma-joined protocol offer the connect contract pins: the
/// relay protocol first, then the never-selected single-use ticket.
pub(super) fn subprotocol_offer(ticket_protocol: &str) -> String {
    format!("{LIVE_SESSION_PROTOCOL}, {ticket_protocol}")
}

/// Spawn the socket thread. Every exit path reports a
/// [`CloudAccountCommand::LiveRelayClosed`] carrying this connection's
/// generation, so the executor can distinguish it from a successor's.
pub(super) fn spawn(
    connection: LiveRelayConnection,
    outbound: tokio::sync::mpsc::UnboundedReceiver<LiveFrame>,
    inbound: std::sync::mpsc::Sender<LiveFrame>,
    stop: tokio::sync::oneshot::Receiver<()>,
    commands: Sender<CloudAccountCommand>,
    repaint: Option<egui::Context>,
) {
    let generation = connection.generation;
    let closed_commands = commands.clone();
    let spawned = std::thread::Builder::new()
        .name("rspice-live-relay".to_owned())
        .spawn(move || {
            let closure = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime.block_on(pump(
                    connection, outbound, inbound, stop, &commands, repaint,
                )),
                Err(_) => LiveRelayClosure::Interrupted,
            };
            let _ = commands.send(CloudAccountCommand::LiveRelayClosed {
                generation,
                closure,
            });
        });
    if spawned.is_err() {
        let _ = closed_commands.send(CloudAccountCommand::LiveRelayClosed {
            generation,
            closure: LiveRelayClosure::Interrupted,
        });
    }
}

async fn pump(
    connection: LiveRelayConnection,
    mut outbound: tokio::sync::mpsc::UnboundedReceiver<LiveFrame>,
    inbound: std::sync::mpsc::Sender<LiveFrame>,
    mut stop: tokio::sync::oneshot::Receiver<()>,
    commands: &Sender<CloudAccountCommand>,
    repaint: Option<egui::Context>,
) -> LiveRelayClosure {
    let mut request = match connection.url.as_str().into_client_request() {
        Ok(request) => request,
        Err(_) => return LiveRelayClosure::Rejected,
    };
    let (Ok(origin), Ok(protocols)) = (connection.origin.parse(), connection.protocols.parse())
    else {
        return LiveRelayClosure::Rejected;
    };
    request.headers_mut().insert(http::header::ORIGIN, origin);
    request
        .headers_mut()
        .insert(http::header::SEC_WEBSOCKET_PROTOCOL, protocols);

    // Handshake refusals are reported as interruptions on purpose: the
    // ticket is single-use and short-lived, so a refusal here is most often
    // a consumed or expired credential, and the executor's re-mint path is
    // the correct response. A genuinely dead session surfaces through the
    // roster poll instead.
    let (mut socket, response) = match tokio_tungstenite::connect_async(request).await {
        Ok(connected) => connected,
        Err(_) => return LiveRelayClosure::Interrupted,
    };
    let selected = response
        .headers()
        .get(http::header::SEC_WEBSOCKET_PROTOCOL)
        .and_then(|value| value.to_str().ok());
    if selected != Some(LIVE_SESSION_PROTOCOL) {
        // The service always selects the relay protocol; streaming into
        // anything else would be undefined.
        let _ = socket.close(None).await;
        return LiveRelayClosure::Rejected;
    }
    let _ = commands.send(CloudAccountCommand::LiveRelayAttached {
        generation: connection.generation,
    });
    if let Some(repaint) = &repaint {
        repaint.request_repaint();
    }

    loop {
        tokio::select! {
            _ = &mut stop => {
                let _ = socket.close(None).await;
                return LiveRelayClosure::Local;
            }
            queued = outbound.recv() => match queued {
                Some(frame) => {
                    let encoded = frame.encode();
                    if encoded.len() > MAX_LIVE_FRAME_BYTES {
                        // The relay would end the connection over it (1009).
                        log::error!(
                            "live relay frame of {} bytes dropped before send",
                            encoded.len()
                        );
                        continue;
                    }
                    if socket.send(Message::Binary(encoded.into())).await.is_err() {
                        return LiveRelayClosure::Interrupted;
                    }
                }
                // Every port clone is gone; nothing can stream anymore.
                None => {
                    let _ = socket.close(None).await;
                    return LiveRelayClosure::Local;
                }
            },
            received = socket.next() => match received {
                Some(Ok(Message::Binary(bytes))) => {
                    // The relay validates every class byte before relaying,
                    // so an undecodable frame is dropped, not fatal.
                    if let Some(frame) = LiveFrame::decode(&bytes) {
                        if inbound.send(frame).is_err() {
                            // The workbench dropped its port.
                            let _ = socket.close(None).await;
                            return LiveRelayClosure::Local;
                        }
                        if let Some(repaint) = &repaint {
                            repaint.request_repaint();
                        }
                    }
                }
                Some(Ok(Message::Close(frame))) => return closure_from_close(frame.as_ref()),
                // Pings are answered by the protocol layer while this loop
                // keeps polling; the relay never sends text.
                Some(Ok(_)) => {}
                Some(Err(_)) | None => return LiveRelayClosure::Interrupted,
            },
        }
    }
}

/// Read the relay's close into the executor's vocabulary. Deliberate session
/// endings arrive as normal closures with a fixed reason phrase (ADR 0082);
/// policy-class codes mean this client broke the frame contract.
fn closure_from_close(frame: Option<&CloseFrame>) -> LiveRelayClosure {
    let Some(frame) = frame else {
        return LiveRelayClosure::Interrupted;
    };
    match frame.code {
        CloseCode::Normal => LiveRelayClosure::SessionOver {
            message: session_over_message(frame.reason.as_str()),
        },
        CloseCode::Policy | CloseCode::Unsupported | CloseCode::Size => LiveRelayClosure::Rejected,
        _ => LiveRelayClosure::Interrupted,
    }
}

/// Presentation-safe reading of the relay's deliberate close reasons.
fn session_over_message(reason: &str) -> String {
    match reason {
        "removed from the live session" => "The host removed you from this live session.",
        "live session expired" => "This live session reached its maximum duration and ended.",
        _ => "This live session has ended.",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio_tungstenite::tungstenite::handshake::server::{
        Request as ServerRequest, Response as ServerResponse,
    };

    use super::super::LiveFrameClass;
    use super::*;

    #[test]
    fn relay_endpoints_map_release_origins_onto_socket_schemes() {
        let (url, origin) = relay_endpoint(
            "https://api.rspice.app",
            "/api/v1/live-sessions/00000000-0000-0000-0000-000000000001/connect",
        )
        .expect("production endpoint");
        assert_eq!(
            url.as_str(),
            "wss://api.rspice.app/api/v1/live-sessions/00000000-0000-0000-0000-000000000001/connect"
        );
        assert_eq!(origin, "https://api.rspice.app");

        let (url, origin) =
            relay_endpoint("http://127.0.0.1:8080", "/api/v1/live-sessions/x/connect")
                .expect("loopback endpoint");
        assert_eq!(
            url.as_str(),
            "ws://127.0.0.1:8080/api/v1/live-sessions/x/connect"
        );
        assert_eq!(origin, "http://127.0.0.1:8080");

        assert!(relay_endpoint("file:///rspice", "/connect").is_none());
        assert!(relay_endpoint("not a url", "/connect").is_none());
    }

    #[test]
    fn subprotocol_offers_are_one_comma_joined_value() {
        assert_eq!(
            subprotocol_offer("rspice.ticket.CREDENTIAL"),
            "rspice.live-session.v1, rspice.ticket.CREDENTIAL"
        );
    }

    #[test]
    fn close_frames_map_onto_the_executor_vocabulary() {
        let close = |code, reason: &str| {
            closure_from_close(Some(&CloseFrame {
                code,
                reason: reason.into(),
            }))
        };
        assert_eq!(
            close(CloseCode::Normal, "live session ended"),
            LiveRelayClosure::SessionOver {
                message: "This live session has ended.".to_owned()
            }
        );
        assert_eq!(
            close(CloseCode::Normal, "removed from the live session"),
            LiveRelayClosure::SessionOver {
                message: "The host removed you from this live session.".to_owned()
            }
        );
        assert_eq!(
            close(CloseCode::Normal, "live session expired"),
            LiveRelayClosure::SessionOver {
                message: "This live session reached its maximum duration and ended.".to_owned()
            }
        );
        assert_eq!(
            close(CloseCode::Normal, "unrecognized wording"),
            LiveRelayClosure::SessionOver {
                message: "This live session has ended.".to_owned()
            },
            "unknown deliberate closes still end the session"
        );
        assert_eq!(close(CloseCode::Policy, ""), LiveRelayClosure::Rejected);
        assert_eq!(close(CloseCode::Size, ""), LiveRelayClosure::Rejected);
        assert_eq!(close(CloseCode::Away, ""), LiveRelayClosure::Interrupted);
        assert_eq!(closure_from_close(None), LiveRelayClosure::Interrupted);
    }

    /// Full pump exercise against a local mock relay: handshake headers,
    /// protocol selection, both stream directions, and the deliberate close.
    #[test]
    fn relay_pump_streams_frames_and_reports_the_server_close() {
        let (port_sender, port_receiver) = std::sync::mpsc::channel();
        let server = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("server runtime");
            runtime.block_on(async move {
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                    .await
                    .expect("bind");
                let port = listener.local_addr().expect("address").port();
                port_sender.send(port).expect("port");
                let (stream, _) = listener.accept().await.expect("accept");
                let callback = |request: &ServerRequest, mut response: ServerResponse| {
                    let offer = request
                        .headers()
                        .get(http::header::SEC_WEBSOCKET_PROTOCOL)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or_default();
                    assert!(
                        offer.starts_with("rspice.live-session.v1, rspice.ticket."),
                        "protocol offer was {offer:?}"
                    );
                    assert!(
                        request.headers().contains_key(http::header::ORIGIN),
                        "native connects must carry an explicit Origin"
                    );
                    response.headers_mut().insert(
                        http::header::SEC_WEBSOCKET_PROTOCOL,
                        LIVE_SESSION_PROTOCOL.parse().expect("header value"),
                    );
                    Ok(response)
                };
                let mut socket = tokio_tungstenite::accept_hdr_async(stream, callback)
                    .await
                    .expect("server handshake");
                let received = loop {
                    match socket.next().await.expect("open stream").expect("frame") {
                        Message::Binary(bytes) => break bytes,
                        _ => continue,
                    }
                };
                assert_eq!(received.as_ref(), [0u8, b'h', b'i']);
                socket
                    .send(Message::Binary(vec![1u8, 42].into()))
                    .await
                    .expect("relay a frame back");
                socket
                    .close(Some(CloseFrame {
                        code: CloseCode::Normal,
                        reason: "live session ended".into(),
                    }))
                    .await
                    .expect("deliberate close");
                while let Some(Ok(_)) = socket.next().await {}
            });
        });

        let port = port_receiver
            .recv_timeout(Duration::from_secs(10))
            .expect("server came up");
        let (url, origin) = relay_endpoint(
            &format!("http://127.0.0.1:{port}"),
            "/api/v1/live-sessions/0/connect",
        )
        .expect("endpoint");
        let (outbound_sender, outbound_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (inbound_sender, inbound_receiver) = std::sync::mpsc::channel();
        let (stop_sender, stop_receiver) = tokio::sync::oneshot::channel();
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        outbound_sender
            .send(LiveFrame {
                class: LiveFrameClass::Presence,
                payload: b"hi".to_vec(),
            })
            .expect("queue before attach");
        spawn(
            LiveRelayConnection {
                url,
                origin,
                protocols: subprotocol_offer("rspice.ticket.TEST"),
                generation: 7,
            },
            outbound_receiver,
            inbound_sender,
            stop_receiver,
            command_sender,
            None,
        );

        match command_receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(CloudAccountCommand::LiveRelayAttached { generation: 7 }) => {}
            other => panic!("expected the attach notification, got {other:?}"),
        }
        let echoed = inbound_receiver
            .recv_timeout(Duration::from_secs(10))
            .expect("relayed frame");
        assert_eq!(echoed.class, LiveFrameClass::Cursor);
        assert_eq!(echoed.payload, vec![42]);
        match command_receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(CloudAccountCommand::LiveRelayClosed {
                generation: 7,
                closure,
            }) => assert_eq!(
                closure,
                LiveRelayClosure::SessionOver {
                    message: "This live session has ended.".to_owned()
                }
            ),
            other => panic!("expected the close notification, got {other:?}"),
        }
        drop(stop_sender);
        server.join().expect("mock relay");
    }
}
