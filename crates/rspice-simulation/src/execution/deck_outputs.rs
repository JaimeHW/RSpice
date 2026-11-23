//! The outputs a manual deck names, which are what its run plots.

use rspice_core::netlist::{Netlist, SaveSet, SaveSignal};

/// The outputs a manual deck names with `.SAVE`, `.PROBE`, `.PRINT` or `.PLOT`: what its run
/// plots, live and after it ends. A deck that names none plots nothing, and every signal it
/// writes waits in the data browser to be ticked. `.SAVE ALL` keeps every signal and names
/// none of them.
#[derive(Debug, Clone, Default)]
pub struct DeckOutputs {
    named: SaveSet,
}

impl DeckOutputs {
    /// The outputs `netlist` names, as its parser folded them from its output cards.
    pub fn of(netlist: &Netlist) -> Self {
        let mut named = netlist.saves.clone();
        named
            .signals
            .retain(|signal| !matches!(signal, SaveSignal::All));
        Self { named }
    }

    /// Whether the deck names no output, so its run plots nothing.
    pub fn is_empty(&self) -> bool {
        self.named.is_empty()
    }

    /// Whether the signal named `name` (`V(out)`, `I(V1)`) is one the deck names.
    pub fn plots(&self, name: &str) -> bool {
        !self.named.is_empty() && self.named.selects(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outputs(deck: &str) -> DeckOutputs {
        DeckOutputs::of(&Netlist::parse(deck).expect("the deck parses"))
    }

    #[test]
    fn a_deck_plots_the_outputs_its_cards_name_and_nothing_else() {
        let body = "* outputs\nV1 a 0 1\nR1 a b 1k\nR2 b 0 1k\n.tran 1n 10n\n";
        for card in [
            ".probe V(b)",
            ".save v(b)",
            ".print tran V(b)",
            ".plot tran v(b)",
        ] {
            let deck = outputs(&format!("{body}{card}\n.end\n"));
            assert!(deck.plots("V(b)"), "{card} plots V(b)");
            assert!(!deck.plots("V(a)"), "{card} plots only what it names");
            assert!(!deck.plots("I(V1)"), "{card} plots only what it names");
        }
        let none = outputs(&format!("{body}.end\n"));
        assert!(none.is_empty());
        assert!(
            !none.plots("V(b)"),
            "a deck that names nothing plots nothing"
        );
        let all = outputs(&format!("{body}.save all\n.end\n"));
        assert!(
            !all.plots("V(b)"),
            ".save all keeps every signal and names none"
        );
    }
}
