//! Helper bodies behind `CompressedTransientResult`'s `#[pymethods]` block.
//!
//! PyO3 permits one `#[pymethods]` block per type, so every Python-facing
//! signature and its docstring stays in `compressed.rs` and everything longer
//! than a delegation lives here, as a second inherent impl -- the same split
//! `transient/internals.rs` and `engine/internals.rs` make for the same
//! reason.
//!
//! What collects here is one subject: how a caller's spelling becomes a
//! channel of the decimated inventory. A node index, an event-only net, a
//! channel's availability and a branch current are each decided once, and the
//! accessors above agree because they all ask these.

use super::*;

impl PyCompressedTransientResult {
    pub fn new(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self {
            inner,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Tran,
                (),
            )),
        }
    }

    /// A compressed trajectory rebuilt from pickled state, which carries no
    /// analysis identity.
    pub(super) fn restored(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self {
            inner,
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained samples.
    ///
    /// The document carries the compression certificate the run produced, so
    /// a reader always knows which grid the published history is on.
    pub(super) fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let (analysis, coordinate) = document::execution(&self.evidence, "compressed transient")?;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_compressed_transient(analysis, &self.inner, Vec::new())
        })
    }

    pub(super) fn node_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        let num_nodes = self.inner.num_nodes();
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) if *index <= num_nodes => Ok(Some(index - 1)),
            NodeIdentifier::Index(index) => Err(invalid_node_index_error(*index, num_nodes).into()),
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names()
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }

    /// The node's name and event domain when the container carries it as
    /// events, not as a voltage.
    ///
    /// The channel stays in the inventory because that inventory is this
    /// container's node namespace, so the pair "unretained voltage plus an
    /// event trace under the same name" is what identifies the net — the
    /// same test the uncompressed result uses, in both event domains.
    pub(super) fn event_only_node(
        &self,
        index: usize,
    ) -> Option<(String, rspice_core::analysis::transient::EventOnlyNetKind)> {
        let channel = self.inner.node_voltage_channel(index)?;
        if channel.availability != rspice_core::engine::TransientChannelAvailability::NotProjected {
            return None;
        }
        let name = channel.descriptor.owner_name();
        self.event_only_name_kind(name)
            .map(|kind| (name.to_string(), kind))
    }

    /// The event domain that owns a name outright, judged the same way for a
    /// channel accessor and for the two listing methods.
    pub(super) fn event_only_name_kind(
        &self,
        name: &str,
    ) -> Option<rspice_core::analysis::transient::EventOnlyNetKind> {
        use rspice_core::analysis::transient::EventOnlyNetKind;
        if self
            .inner
            .digital_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case(name))
        {
            Some(EventOnlyNetKind::Digital)
        } else if self
            .inner
            .real_traces
            .iter()
            .any(|trace| trace.node_name.eq_ignore_ascii_case(name))
        {
            Some(EventOnlyNetKind::Real)
        } else {
            None
        }
    }

    /// Whether one channel of this container is an event-only net's voltage.
    ///
    /// The document builder drops exactly these descriptors, so the listing
    /// methods have to apply the same test or the compressed Python surface
    /// advertises a channel the document says does not exist.
    pub(super) fn channel_is_event_only(
        &self,
        channel: &rspice_core::engine::TransientCompressedChannel,
    ) -> bool {
        matches!(
            channel.descriptor.role(),
            rspice_core::engine::TransientChannelRole::NodeVoltage { .. }
        ) && channel.availability == rspice_core::engine::TransientChannelAvailability::NotProjected
            && self
                .event_only_name_kind(channel.descriptor.owner_name())
                .is_some()
    }

    /// Dense retained samples of one channel, refusing to invent a number for
    /// a sample the producing run recorded as absent.
    pub(super) fn dense_channel_values(
        &self,
        channel: &rspice_core::engine::TransientCompressedChannel,
        label: &str,
    ) -> PyResult<Vec<f64>> {
        if channel.availability != rspice_core::engine::TransientChannelAvailability::Available {
            return Err(crate::errors::key_error(format!(
                "{label} was not recorded; add it to .SAVE"
            )));
        }
        channel.dense_values().ok_or_else(|| {
            crate::errors::value_error(format!(
                "{label} has samples the producing run did not record as numbers; read `channel_absence` for the reason at each retained point"
            ))
        })
    }

    pub(super) fn channel(
        &self,
        canonical_name: &str,
    ) -> PyResult<&rspice_core::engine::TransientCompressedChannel> {
        let channel = self.inner.channel_named(canonical_name).ok_or_else(|| {
            crate::errors::key_error(format!(
                "unknown compressed transient channel '{canonical_name}'"
            ))
        })?;
        // The container keeps an event-only net's channel so the inventory
        // stays the node namespace, but the net has no voltage channel to
        // describe. Answering "not-projected" here would say the run chose
        // not to keep something it never had, which is the opposite of what
        // the document says by omitting the descriptor entirely.
        if self.channel_is_event_only(channel)
            && let Some(kind) = self.event_only_name_kind(channel.descriptor.owner_name())
        {
            return Err(PyErr::from(event_only_node_error(
                channel.descriptor.owner_name(),
                kind,
                EventTraceSurface::Compressed,
            )));
        }
        Ok(channel)
    }

    pub(super) fn branch_current_values(&self, name: &str) -> PyResult<Vec<f64>> {
        let channel = self
            .inner
            .branch_current_channel(name)
            .ok_or_else(|| PyErr::from(unknown_branch_name_error(name)))?;
        self.dense_channel_values(channel, &format!("branch-current waveform '{name}'"))
    }
}
