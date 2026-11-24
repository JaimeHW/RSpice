//! The reads that tell a run what its own deck gives it read the deck under
//! the limits the run reads it under, so a deck past the size this build
//! ships with still gets each one's advice on a machine with the memory for
//! it.

use std::collections::HashMap;
use std::path::PathBuf;

use rspice_core::ResourceLimits;
use rspice_results::specification::{SpecEntry, SpecPointScope, SpecificationDefinition};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
use rspice_vocabulary::product::AnalysisInstanceId;

use super::{
    MachineMemory, deck_limits, forget_machine_memory_here, machine_memory,
    pretend_machine_memory_here,
};
use crate::netlist_preparation::{IncludeSearchChain, measurements, seal_deck_hdl_sources};
use crate::project_veriloga::preparation::deck_hdl;

const GB: u64 = 1024 * 1024 * 1024;

/// A design over `half`, a cell its deck defines and no source does.
const TOP: &str = "`include \"disciplines.vams\"\nmodule top(p, n);\n  inout p, n;\n  electrical p, n;\n  half xu(p, n);\nendmodule\n";

/// `body` behind a comment that puts the deck past the bytes a deck may be
/// as this build ships.
fn past_shipped(body: &str) -> String {
    let shipped = ResourceLimits::default().max_netlist_bytes;
    let mut deck = String::with_capacity(shipped + body.len() + 32);
    deck.push_str("Past the shipped size\n* ");
    deck.push_str(&"x".repeat(shipped));
    deck.push('\n');
    deck.push_str(body);
    deck
}

/// A folder of its own for the files a deck includes, removed when the test
/// ends.
struct Folder(PathBuf);

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Each read a run's preparation makes of its deck for advice, on a deck
/// over the shipped 64 MB: the limits as shipped turn the deck away and
/// the read advises nothing, while the limits the run reads it under on a
/// 16 GB machine give its advice.
#[test]
fn a_deck_past_the_shipped_size_still_gets_each_reads_advice() {
    pretend_machine_memory_here(MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: 8 * GB,
    });
    let sized = deck_limits(machine_memory());
    forget_machine_memory_here();
    let shipped = ResourceLimits::default();

    let folder =
        Folder(std::env::temp_dir().join(format!("rspice-deck-reads-{}", std::process::id())));
    std::fs::create_dir_all(&folder.0).expect("create the folder");
    std::fs::write(folder.0.join("top.va"), TOP).expect("write the design");
    let origin = folder.0.join("deck.cir");

    let mut gain = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: "gain".into(),
        expression: ".MEAS AC gain FIND VM(out) AT=1k".into(),
        min: None,
        max: None,
        unit: "V".into(),
        scope: SpecPointScope::AllPoints,
    });
    gain.define_measurement = true;
    let ac = AnalysisDraft::for_kind(AnalysisKind::Ac);
    let drafts = HashMap::from([(AnalysisInstanceId::new(), &ac)]);

    type Advised<'a> = &'a dyn Fn(&str, ResourceLimits) -> bool;
    let reads: [(&str, &str, Advised); 3] = [
        (
            "the cells its Verilog-A sources are compiled against, sealed with the run",
            ".hdl \"top.va\"\n.subckt half a b\nR1 a b 1k\n.ends\nV1 in 0 1\nX1 in 0 top\n.end\n",
            &|deck, limits| {
                seal_deck_hdl_sources(deck, Some(&origin), &IncludeSearchChain::default(), limits)
                    .is_ok()
            },
        ),
        (
            "the cells it offers its designs as leaves and the min:typ:max value they keep",
            ".options hdl_delay_select=min\n.subckt half a b\nR1 a b 1k\n.ends\nV1 in 0 1\n\
             X1 in 0 half\n.end\n",
            &|deck, limits| {
                let (cells, delay_select) = deck_hdl(deck, limits);
                !cells.is_empty() && delay_select == DigitalDelaySelect::Min
            },
        ),
        (
            "the plan's measurements, checked against its parameters",
            "V1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1u\n.end\n",
            &|deck, limits| {
                measurements::materialize(deck, std::slice::from_ref(&gain), &drafts, limits)
                    .is_ok_and(|source| source.contains(".MEAS AC gain"))
            },
        ),
    ];
    for (read, body, advised) in reads {
        let deck = past_shipped(body);
        assert!(
            !advised(&deck, shipped),
            "{read}: the deck is past the limits as shipped"
        );
        assert!(
            advised(&deck, sized),
            "{read}: a deck its run reads gets the advice"
        );
    }
}
