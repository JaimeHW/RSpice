//! A check's or a run's refusal for going past one of the engine's limits,
//! in the reader's words: what is too large and by how much, in the units a
//! person reads, never the engine's name for the limit or its raw counts.

use rspice_core::{ResourceKind, ResourceLimitError};

use super::{MessageCatalog, MessageId};

/// How a resource's amounts are said.
#[derive(Clone, Copy)]
enum Measure {
    /// Bytes, in KB, MB or GB.
    Bytes,
    /// A count, with its thousands grouped.
    Count,
}

/// The sentence a refusal for `resource` is said in, and how its amounts
/// are said.
const fn said_as(resource: ResourceKind) -> (MessageId, Measure) {
    use Measure::{Bytes, Count};
    match resource {
        ResourceKind::NetlistBytes => (MessageId::ResourceNetlistBytes, Bytes),
        ResourceKind::NetlistLines => (MessageId::ResourceNetlistLines, Count),
        ResourceKind::ExpandedSourceBytes => (MessageId::ResourceExpandedSourceBytes, Bytes),
        ResourceKind::DependencySourceBytes => (MessageId::ResourceDependencySourceBytes, Bytes),
        ResourceKind::ExternalDataBytes => (MessageId::ResourceExternalDataBytes, Bytes),
        ResourceKind::ExternalDataValues => (MessageId::ResourceExternalDataValues, Count),
        ResourceKind::SharedCacheBytes => (MessageId::ResourceSharedCacheBytes, Bytes),
        ResourceKind::IncludeDepth => (MessageId::ResourceIncludeDepth, Count),
        ResourceKind::HierarchyDepth => (MessageId::ResourceHierarchyDepth, Count),
        ResourceKind::FlattenedElements => (MessageId::ResourceFlattenedElements, Count),
        ResourceKind::CircuitNodes => (MessageId::ResourceCircuitNodes, Count),
        ResourceKind::MatrixUnknowns => (MessageId::ResourceMatrixUnknowns, Count),
        ResourceKind::ResultValues => (MessageId::ResourceResultValues, Count),
        ResourceKind::TransportHistoryBytes => (MessageId::ResourceTransportHistoryBytes, Bytes),
        ResourceKind::TransportHistoryRecords => {
            (MessageId::ResourceTransportHistoryRecords, Count)
        }
        ResourceKind::MixedIntervalEvents => (MessageId::ResourceMixedIntervalEvents, Count),
        ResourceKind::ParallelWorkers => (MessageId::ResourceParallelWorkers, Count),
        ResourceKind::BatchRuns => (MessageId::ResourceBatchRuns, Count),
    }
}

impl MessageCatalog {
    /// What `refusal` refused, in one sentence: what is too large, how large
    /// it is and how large it can be, as "This deck is 95.4 MB; a deck can
    /// be at most 64 MB." Every limit a check or a run can go past is said
    /// here, so none reaches the page in the engine's words.
    #[must_use]
    pub fn resource_refusal(self, refusal: &ResourceLimitError) -> String {
        let (sentence, measure) = said_as(refusal.resource);
        let limit = self.amount(refusal.limit, measure);
        let requested = self.amount(refusal.requested, measure);
        // The engine stops counting at the first one over, and an amount
        // can round to the limit: either is said as more than the limit,
        // never as an amount that reads as within it.
        let requested =
            if requested == limit || refusal.requested == refusal.limit.saturating_add(1) {
                self.format(MessageId::ResourceMoreThan, &[("amount", &limit)])
            } else {
                requested
            };
        self.format(sentence, &[("requested", &requested), ("limit", &limit)])
    }

    /// [`Self::resource_refusal`] for a limit one device went past, the
    /// device named first.
    #[must_use]
    pub fn device_resource_refusal(self, device: &str, refusal: &ResourceLimitError) -> String {
        self.format(
            MessageId::ResourceOfDevice,
            &[
                ("device", device),
                ("refusal", &self.resource_refusal(refusal)),
            ],
        )
    }

    fn amount(self, amount: usize, measure: Measure) -> String {
        const KILO: f64 = 1024.0;
        let Measure::Bytes = measure else {
            return grouped(amount);
        };
        #[allow(clippy::cast_precision_loss)]
        let bytes = amount as f64;
        let (unit, scaled) = if bytes >= KILO * KILO * KILO {
            (MessageId::ResourceGigabytes, bytes / (KILO * KILO * KILO))
        } else if bytes >= KILO * KILO {
            (MessageId::ResourceMegabytes, bytes / (KILO * KILO))
        } else if bytes >= KILO {
            (MessageId::ResourceKilobytes, bytes / KILO)
        } else {
            return self.format(MessageId::ResourceBytes, &[("amount", &grouped(amount))]);
        };
        let tenths = format!("{scaled:.1}");
        let amount = tenths.strip_suffix(".0").unwrap_or(&tenths);
        self.format(unit, &[("amount", amount)])
    }
}

/// `count` with its thousands grouped: "2,000,000".
fn grouped(count: usize) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::localization::UiTextLocale;

    const MIB: usize = 1024 * 1024;

    #[test]
    fn every_limit_reads_as_its_catalogue_sentence() {
        let table: [(ResourceKind, usize, usize, &str); 18] = [
            (
                ResourceKind::NetlistBytes,
                100_000_091,
                64 * MIB,
                "This deck is 95.4 MB; a deck can be at most 64 MB.",
            ),
            (
                ResourceKind::NetlistLines,
                2_500_000,
                2_000_000,
                "This deck has 2,500,000 lines; a deck can have at most 2,000,000.",
            ),
            (
                ResourceKind::ExpandedSourceBytes,
                300 * MIB,
                256 * MIB,
                "This deck comes to 300 MB with the files it includes read in; it can come \
                 to at most 256 MB.",
            ),
            (
                ResourceKind::DependencySourceBytes,
                257 * MIB,
                256 * MIB,
                "The files this deck includes come to 257 MB; they can come to at most \
                 256 MB.",
            ),
            (
                ResourceKind::ExternalDataBytes,
                3 * 1024 * MIB,
                256 * MIB,
                "A data file a source reads is 3 GB; a data file can be at most 256 MB.",
            ),
            (
                ResourceKind::ExternalDataValues,
                25_000_001,
                25_000_000,
                "A data file a source reads holds more than 25,000,000 values; a data file \
                 can hold at most 25,000,000.",
            ),
            (
                ResourceKind::SharedCacheBytes,
                600 * MIB,
                512 * MIB,
                "The compiled devices kept between runs come to 600 MB; they can come to at \
                 most 512 MB.",
            ),
            (
                ResourceKind::IncludeDepth,
                65,
                64,
                "This deck's includes nest more than 64 deep; they can nest at most 64 deep.",
            ),
            (
                ResourceKind::HierarchyDepth,
                120,
                100,
                "This circuit's subcircuits nest 120 deep; they can nest at most 100 deep.",
            ),
            (
                ResourceKind::FlattenedElements,
                400_000,
                250_000,
                "This circuit has 400,000 devices with its subcircuits expanded; a circuit \
                 can have at most 250,000.",
            ),
            (
                ResourceKind::CircuitNodes,
                300_000,
                250_000,
                "This circuit has 300,000 nodes; a circuit can have at most 250,000.",
            ),
            (
                ResourceKind::MatrixUnknowns,
                250_001,
                250_000,
                "This circuit has more than 250,000 unknowns to solve for; a circuit can \
                 have at most 250,000.",
            ),
            (
                ResourceKind::ResultValues,
                30_000_000,
                25_000_000,
                "This result would hold 30,000,000 values; a result can hold at most \
                 25,000,000.",
            ),
            (
                ResourceKind::TransportHistoryBytes,
                300 * MIB,
                256 * MIB,
                "The delay lines' histories need 300 MB; they can take at most 256 MB.",
            ),
            (
                ResourceKind::TransportHistoryRecords,
                2_000_000,
                1_000_000,
                "A delay line's history holds 2,000,000 points; it can hold at most \
                 1,000,000.",
            ),
            (
                ResourceKind::MixedIntervalEvents,
                1_000_001,
                1_000_000,
                "One step of the mixed-signal run has more than 1,000,000 events; a step \
                 can have at most 1,000,000.",
            ),
            (
                ResourceKind::ParallelWorkers,
                128,
                64,
                "This run asks for 128 threads; a run can use at most 64.",
            ),
            (
                ResourceKind::BatchRuns,
                20_000,
                10_000,
                "This analysis asks for 20,000 runs; an analysis can make at most 10,000.",
            ),
        ];
        let messages = MessageCatalog::new(UiTextLocale::EnglishUnitedStates);
        for (resource, requested, limit, sentence) in table {
            let refusal = ResourceLimitError {
                resource,
                requested,
                limit,
            };
            let said = messages.resource_refusal(&refusal);
            assert_eq!(said, sentence, "{resource}");
            assert!(!said.contains(resource.as_str()), "{said}");
        }
    }

    #[test]
    fn an_amount_that_rounds_to_the_limit_reads_as_more_than_it() {
        let messages = MessageCatalog::new(UiTextLocale::EnglishUnitedStates);
        let refusal = ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested: 64 * MIB + 4096,
            limit: 64 * MIB,
        };
        assert_eq!(
            messages.resource_refusal(&refusal),
            "This deck is more than 64 MB; a deck can be at most 64 MB."
        );
        assert_eq!(
            messages.device_resource_refusal(
                "T1",
                &ResourceLimitError {
                    resource: ResourceKind::TransportHistoryBytes,
                    requested: 2048,
                    limit: 1000,
                },
            ),
            "T1: The delay lines' histories need 2 KB; they can take at most 1,000 bytes."
        );
    }
}
