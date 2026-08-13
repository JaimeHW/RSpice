//! Figure-hydration emission: the sealed viewer runtime, the authored page
//! loader, and the JSON island that binds them to the document's figures.
//!
//! The runtime ships inside every figure-bearing bundle (ADR 0085 in the
//! cloud repository): page, payloads, and the code that interprets them are
//! sealed together, so a renderer upgrade can never skew against an
//! already-published page. The island carries the digests the loader
//! verifies before instantiating anything; the loader `<script>` tag itself
//! is pinned with subresource integrity computed here.

use base64::Engine as _;
use rspice_publication_contract::ManifestEntry;
use sha2::{Digest as _, Sha384};

use crate::{RenderError, sha256_hex};

/// Bundle paths for the three runtime assets. Stable names: the bundle is
/// immutable per publication, so content-hashed names would add nothing.
pub(crate) const LOADER_PATH: &str = "assets/loader.js";
pub(crate) const VIEWER_JS_PATH: &str = "assets/viewer.js";
pub(crate) const VIEWER_WASM_PATH: &str = "assets/viewer.wasm";

/// The authored hydration loader, emitted verbatim into every
/// figure-bearing bundle.
pub(crate) const LOADER_JS: &str = include_str!("assets/loader.js");

/// The built viewer runtime sealed into every figure-bearing bundle: the
/// wasm module and its JS glue exactly as `wasm-bindgen` produced them from
/// the `rspice-viewer` crate at this workspace version.
pub struct ViewerRuntime {
    pub(crate) wasm: Vec<u8>,
    pub(crate) js_glue: Vec<u8>,
}

impl std::fmt::Debug for ViewerRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Byte dumps would drown any failure message; sizes identify a
        // runtime well enough for diagnostics.
        f.debug_struct("ViewerRuntime")
            .field("wasm_bytes", &self.wasm.len())
            .field("js_glue_bytes", &self.js_glue.len())
            .finish()
    }
}

impl ViewerRuntime {
    /// Accept a runtime only if it is plausibly what the component build
    /// staged: a wasm module (leading `\0asm` magic) and non-empty glue. A
    /// mis-staged build directory fails here, at render time, instead of
    /// publishing pages whose figures can never hydrate.
    pub fn new(wasm: Vec<u8>, js_glue: Vec<u8>) -> Result<Self, RenderError> {
        if !wasm.starts_with(b"\0asm") {
            return Err(RenderError::ViewerRuntime(
                "viewer wasm does not begin with the wasm module magic".to_string(),
            ));
        }
        if js_glue.is_empty() {
            return Err(RenderError::ViewerRuntime(
                "viewer JS glue is empty".to_string(),
            ));
        }
        Ok(Self { wasm, js_glue })
    }
}

/// The document-side pieces of hydration, computed against the exact bytes
/// the bundle carries.
pub(crate) struct HydrationEmission {
    /// Complete island JSON: runtime paths and digests plus every manifest
    /// entry, `<`-escaped so it can never terminate its own script element.
    pub island_json: String,
    /// `sha384-…` subresource integrity for the loader script tag.
    pub loader_integrity: String,
}

impl HydrationEmission {
    pub fn new(entries: &[ManifestEntry], viewer: &ViewerRuntime) -> Result<Self, RenderError> {
        let island = serde_json::json!({
            "runtime": {
                "js": VIEWER_JS_PATH,
                "wasm": VIEWER_WASM_PATH,
                "wasm_sha256": sha256_hex(&viewer.wasm),
                "wasm_byte_len": viewer.wasm.len() as u64,
            },
            "figures": entries,
        });
        let island_json = serde_json::to_string(&island)
            .map_err(|error| RenderError::Payload(error.to_string()))?
            .replace('<', "\\u003c");
        Ok(Self {
            island_json,
            loader_integrity: sri_sha384(LOADER_JS.as_bytes()),
        })
    }
}

pub(crate) fn sri_sha384(bytes: &[u8]) -> String {
    format!(
        "sha384-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha384::digest(bytes))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn viewer() -> ViewerRuntime {
        ViewerRuntime::new(b"\0asm-test-viewer".to_vec(), b"// glue\n".to_vec())
            .expect("test runtime")
    }

    #[test]
    fn runtime_rejects_bytes_that_are_not_a_wasm_module() {
        assert!(ViewerRuntime::new(b"asm".to_vec(), b"// glue\n".to_vec()).is_err());
        assert!(ViewerRuntime::new(b"\0asm".to_vec(), Vec::new()).is_err());
    }

    #[test]
    fn island_json_cannot_terminate_its_own_script_element() {
        let emission = HydrationEmission::new(&[], &viewer()).expect("emission");
        assert!(!emission.island_json.contains('<'));
        assert!(emission.island_json.contains("assets/viewer.wasm"));
        let parsed: serde_json::Value =
            serde_json::from_str(&emission.island_json).expect("island stays valid JSON");
        assert_eq!(
            parsed["runtime"]["wasm_byte_len"].as_u64(),
            Some(b"\0asm-test-viewer".len() as u64)
        );
    }

    #[test]
    fn loader_integrity_matches_the_authored_loader() {
        let emission = HydrationEmission::new(&[], &viewer()).expect("emission");
        assert!(emission.loader_integrity.starts_with("sha384-"));
        assert_eq!(emission.loader_integrity, sri_sha384(LOADER_JS.as_bytes()));
    }

    #[test]
    fn loader_asset_is_lf_normalized() {
        assert!(
            !LOADER_JS.contains('\r'),
            "the loader is emitted byte-for-byte into deterministic bundles"
        );
    }
}
