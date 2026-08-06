//! Browser and service-worker Fetch transport with pre-follow redirect rejection.
//!
//! Reqwest's WebAssembly backend currently leaves Fetch at its default
//! redirect-following policy. Keep this narrow adapter until that backend can
//! express `RequestRedirect::Error`; checking only the final URL is too late
//! for a credential-bearing request.

use std::{cell::Cell, rc::Rc, time::Duration};

use futures_util::StreamExt as _;
use gloo_timers::callback::Timeout;
use http::{
    HeaderMap, HeaderName, HeaderValue, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE, LOCATION, RETRY_AFTER},
};
use reqwest::RequestBuilder;
use url::Url;
use wasm_bindgen::{JsCast as _, prelude::wasm_bindgen};
use wasm_bindgen_futures::JsFuture;

use crate::{
    ClientConfig, CloudError, ProtocolFailure, ResponseMetadata, TransportFailure,
    client::{IDEMPOTENCY_REPLAYED, Received, X_REQUEST_ID, parse_metadata},
    error::map_transport,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn browser_fetch_with_request(input: &web_sys::Request) -> js_sys::Promise;
}

pub(super) async fn send(
    config: &ClientConfig,
    request: RequestBuilder,
    requested_url: &Url,
) -> Result<Received, CloudError> {
    let request = request.build().map_err(|error| CloudError::Transport {
        failure: map_transport(&error),
        status: None,
        metadata: None,
    })?;
    let js_headers = web_sys::Headers::new().map_err(|_| request_failure())?;
    for (name, value) in request.headers() {
        let value = value.to_str().map_err(|_| request_failure())?;
        js_headers
            .append(name.as_str(), value)
            .map_err(|_| request_failure())?;
    }

    let abort_controller = web_sys::AbortController::new().map_err(|_| request_failure())?;
    let timed_out = Rc::new(Cell::new(false));
    let timeout_flag = Rc::clone(&timed_out);
    let timeout_controller = abort_controller.clone();
    let timeout = Timeout::new(timeout_millis(config.request_timeout), move || {
        timeout_flag.set(true);
        timeout_controller.abort();
    });

    let init = web_sys::RequestInit::new();
    init.set_method(request.method().as_str());
    init.set_headers_headers(&js_headers);
    init.set_mode(web_sys::RequestMode::Cors);
    init.set_credentials(web_sys::RequestCredentials::Omit);
    init.set_cache(web_sys::RequestCache::NoStore);
    init.set_redirect(web_sys::RequestRedirect::Error);
    init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    init.set_signal(Some(&abort_controller.signal()));

    let request_body = request
        .body()
        .and_then(reqwest::Body::as_bytes)
        .filter(|body| !body.is_empty())
        .map(js_sys::Uint8Array::from);
    if let Some(body) = &request_body {
        init.set_body_opt_u8_array(Some(body));
    }

    let js_request = web_sys::Request::new_with_str_and_init(requested_url.as_str(), &init)
        .map_err(|_| request_failure())?;
    let response_value = JsFuture::from(fetch_with_request(&js_request))
        .await
        .map_err(|_| CloudError::Transport {
            failure: if timed_out.get() {
                TransportFailure::Timeout
            } else {
                TransportFailure::Request
            },
            status: None,
            metadata: None,
        })?;
    let response = response_value
        .dyn_into::<web_sys::Response>()
        .map_err(|_| CloudError::Transport {
            failure: TransportFailure::Other,
            status: None,
            metadata: None,
        })?;
    let status = StatusCode::from_u16(response.status()).map_err(|_| CloudError::Protocol {
        failure: ProtocolFailure::UnexpectedStatus,
        status: None,
        metadata: ResponseMetadata::default(),
    })?;
    if response.redirected() || response.url() != requested_url.as_str() {
        abort_controller.abort();
        return Err(CloudError::Protocol {
            failure: ProtocolFailure::RedirectedResponse,
            status: Some(status.as_u16()),
            metadata: ResponseMetadata::default(),
        });
    }

    let headers = response_headers(&response).map_err(|()| {
        abort_controller.abort();
        CloudError::Protocol {
            failure: ProtocolFailure::InvalidMetadata,
            status: Some(status.as_u16()),
            metadata: ResponseMetadata::default(),
        }
    })?;
    let metadata = parse_metadata(&headers, status).map_err(|()| {
        abort_controller.abort();
        CloudError::Protocol {
            failure: ProtocolFailure::InvalidMetadata,
            status: Some(status.as_u16()),
            metadata: ResponseMetadata::default(),
        }
    })?;
    let declared_length = declared_content_length(&headers).map_err(|()| {
        abort_controller.abort();
        CloudError::Protocol {
            failure: ProtocolFailure::InvalidContentLength,
            status: Some(status.as_u16()),
            metadata: metadata.clone(),
        }
    })?;
    if declared_length.is_some_and(|length| {
        length > u64::try_from(config.max_response_bytes).expect("usize fits in u64")
    }) {
        abort_controller.abort();
        return Err(CloudError::ResponseTooLarge {
            limit: config.max_response_bytes,
            status: status.as_u16(),
            metadata,
        });
    }

    let body = read_body(
        response.body(),
        &abort_controller,
        &timed_out,
        status,
        &metadata,
        config.max_response_bytes,
    )
    .await?;
    drop(timeout);

    Ok(Received {
        status,
        headers,
        metadata,
        body,
    })
}

async fn read_body(
    raw_stream: Option<web_sys::ReadableStream>,
    abort_controller: &web_sys::AbortController,
    timed_out: &Cell<bool>,
    status: StatusCode,
    metadata: &ResponseMetadata,
    max_response_bytes: usize,
) -> Result<Vec<u8>, CloudError> {
    let mut body = Vec::new();
    if let Some(raw_stream) = raw_stream {
        let mut stream = wasm_streams::ReadableStream::from_raw(raw_stream).into_stream();
        while let Some(chunk) = stream.next().await {
            if timed_out.get() {
                return Err(response_transport_failure(
                    TransportFailure::Timeout,
                    status,
                    metadata,
                ));
            }
            let chunk = chunk
                .map_err(|_| {
                    response_transport_failure(
                        if timed_out.get() {
                            TransportFailure::Timeout
                        } else {
                            TransportFailure::Body
                        },
                        status,
                        metadata,
                    )
                })?
                .dyn_into::<js_sys::Uint8Array>()
                .map_err(|_| {
                    response_transport_failure(TransportFailure::Body, status, metadata)
                })?;
            let chunk_length = usize::try_from(chunk.length())
                .expect("WebAssembly Uint8Array lengths fit in usize");
            let next_length = body.len().checked_add(chunk_length).ok_or_else(|| {
                abort_controller.abort();
                response_too_large(max_response_bytes, status, metadata)
            })?;
            if next_length > max_response_bytes {
                abort_controller.abort();
                return Err(response_too_large(max_response_bytes, status, metadata));
            }
            let offset = body.len();
            body.resize(next_length, 0);
            chunk.copy_to(&mut body[offset..]);
        }
    }
    if timed_out.get() {
        return Err(response_transport_failure(
            TransportFailure::Timeout,
            status,
            metadata,
        ));
    }
    Ok(body)
}

pub(crate) fn fetch_with_request(request: &web_sys::Request) -> js_sys::Promise {
    let global = js_sys::global();
    if js_sys::Reflect::has(
        &global,
        &wasm_bindgen::JsValue::from_str("ServiceWorkerGlobalScope"),
    ) == Ok(true)
    {
        global
            .unchecked_into::<web_sys::ServiceWorkerGlobalScope>()
            .fetch_with_request(request)
    } else {
        browser_fetch_with_request(request)
    }
}

fn response_headers(response: &web_sys::Response) -> Result<HeaderMap, ()> {
    let source = response.headers();
    let mut headers = HeaderMap::new();
    for name in [
        CONTENT_TYPE,
        CONTENT_LENGTH,
        RETRY_AFTER,
        LOCATION,
        HeaderName::from_static(X_REQUEST_ID),
        HeaderName::from_static(IDEMPOTENCY_REPLAYED),
    ] {
        if let Some(value) = source.get(name.as_str()).map_err(|_| ())? {
            headers.insert(name, HeaderValue::from_str(&value).map_err(|_| ())?);
        }
    }
    Ok(headers)
}

fn declared_content_length(headers: &HeaderMap) -> Result<Option<u64>, ()> {
    headers
        .get(CONTENT_LENGTH)
        .map(|value| {
            value
                .to_str()
                .map_err(|_| ())?
                .parse::<u64>()
                .map_err(|_| ())
        })
        .transpose()
}

pub(crate) fn timeout_millis(timeout: Duration) -> u32 {
    let millis = timeout.as_millis().clamp(1, u128::from(u32::MAX));
    u32::try_from(millis).expect("clamped timeout fits in u32")
}

fn request_failure() -> CloudError {
    CloudError::Transport {
        failure: TransportFailure::Request,
        status: None,
        metadata: None,
    }
}

fn response_transport_failure(
    failure: TransportFailure,
    status: StatusCode,
    metadata: &ResponseMetadata,
) -> CloudError {
    CloudError::Transport {
        failure,
        status: Some(status.as_u16()),
        metadata: Some(metadata.clone()),
    }
}

fn response_too_large(limit: usize, status: StatusCode, metadata: &ResponseMetadata) -> CloudError {
    CloudError::ResponseTooLarge {
        limit,
        status: status.as_u16(),
        metadata: metadata.clone(),
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    fn blob(bytes: &[u8]) -> web_sys::Blob {
        let parts = js_sys::Array::new();
        parts.push(&js_sys::Uint8Array::from(bytes).into());
        web_sys::Blob::new_with_u8_array_sequence(&parts).expect("test Blob")
    }

    #[wasm_bindgen_test]
    async fn response_stream_is_executed_and_bounded_in_webassembly() {
        let controller = web_sys::AbortController::new().expect("test abort controller");
        let timed_out = Cell::new(false);
        let status = StatusCode::OK;
        let metadata = ResponseMetadata::default();
        let bytes = b"bounded-control-plane-response";

        let body = read_body(
            Some(blob(bytes).stream()),
            &controller,
            &timed_out,
            status,
            &metadata,
            bytes.len(),
        )
        .await
        .expect("read bounded JavaScript stream");
        assert_eq!(body, bytes);

        let error = read_body(
            Some(blob(bytes).stream()),
            &controller,
            &timed_out,
            status,
            &metadata,
            bytes.len() - 1,
        )
        .await
        .expect_err("stream larger than the configured bound must fail");
        assert!(matches!(
            error,
            CloudError::ResponseTooLarge {
                limit,
                status: 200,
                ..
            } if limit == bytes.len() - 1
        ));
    }

    #[wasm_bindgen_test]
    fn javascript_timer_conversion_is_nonzero_and_saturating() {
        assert_eq!(timeout_millis(Duration::ZERO), 1);
        assert_eq!(timeout_millis(Duration::from_millis(42)), 42);
        assert_eq!(timeout_millis(Duration::MAX), u32::MAX);
    }
}
