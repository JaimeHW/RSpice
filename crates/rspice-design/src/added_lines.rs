//! Lines an engineer adds to a schematic design's generated netlist.
//!
//! The one place for cards the schematic and the simulation plan have no
//! control for: a `.nodeset`, a coupling card, an option the Simulate pages do
//! not offer. A design has at most one block, kept with the project, and every
//! deck generated for that design ends with it.

use rspice_design_model::cell_view::CellViewRef;
use serde::{Deserialize, Serialize};

/// The block of lines added to one design's generated netlist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddedLines {
    /// The schematic design (the root cell view) the lines belong to.
    pub design: CellViewRef,
    pub text: String,
}

/// The lines added to `design`, if it has any.
pub fn added_lines_of<'a>(blocks: &'a [AddedLines], design: &CellViewRef) -> Option<&'a str> {
    blocks
        .iter()
        .find(|block| &block.design == design)
        .map(|block| block.text.as_str())
}

/// Make `text` the lines added to `design`. Text that is only white space
/// removes the design's block. Returns whether the blocks changed.
pub fn set_added_lines(blocks: &mut Vec<AddedLines>, design: &CellViewRef, text: String) -> bool {
    let position = blocks.iter().position(|block| &block.design == design);
    match (position, text.trim().is_empty()) {
        (None, true) => false,
        (Some(index), true) => {
            blocks.remove(index);
            true
        }
        (Some(index), false) => {
            let changed = blocks[index].text != text;
            blocks[index].text = text;
            changed
        }
        (None, false) => {
            blocks.push(AddedLines {
                design: design.clone(),
                text,
            });
            true
        }
    }
}

/// Move each block to the design `remap` moves its design to, as a library,
/// cell or view rename does, so a renamed design keeps its lines.
pub fn remap_added_lines(blocks: &mut [AddedLines], mut remap: impl FnMut(&mut CellViewRef)) {
    for block in blocks {
        remap(&mut block.design);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn design(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    #[test]
    fn a_design_holds_one_block_and_blank_text_removes_it() {
        let mut blocks = Vec::new();
        assert!(set_added_lines(
            &mut blocks,
            &design("amp"),
            ".nodeset v(out)=1".to_owned()
        ));
        assert!(set_added_lines(
            &mut blocks,
            &design("amp"),
            ".ic v(out)=0".to_owned()
        ));
        assert!(!set_added_lines(
            &mut blocks,
            &design("amp"),
            ".ic v(out)=0".to_owned()
        ));
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            added_lines_of(&blocks, &design("amp")),
            Some(".ic v(out)=0")
        );
        assert_eq!(added_lines_of(&blocks, &design("bias")), None);

        assert!(set_added_lines(
            &mut blocks,
            &design("amp"),
            " \n\t".to_owned()
        ));
        assert!(blocks.is_empty());
        assert!(!set_added_lines(&mut blocks, &design("amp"), String::new()));
    }

    #[test]
    fn a_stored_block_names_its_design_and_refuses_unknown_fields() {
        let block = AddedLines {
            design: design("amp"),
            text: ".nodeset v(out)=1\n".to_owned(),
        };
        let json = serde_json::to_string(&block).expect("a block serializes");
        assert_eq!(
            serde_json::from_str::<AddedLines>(&json).expect("a block reads back"),
            block
        );
        let unknown = json.replacen('{', r#"{"kind":"override","#, 1);
        assert!(serde_json::from_str::<AddedLines>(&unknown).is_err());
    }
}
