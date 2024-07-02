#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use table::Table;

use crate::util::{Rect, CellContent};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Word {
    pub text: String,
    pub rect: Rect,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub words: Vec<Word>,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Run {
    pub lines: Vec<Line>,
    pub kind: RunType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Flow {
    pub lines: Vec<Line>,
    pub runs: Vec<Run>,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum RunType {
    ParagraphContinuation,
    Paragraph,
    Header,
    Cell,
}

impl Flow {
    pub fn new() -> Self {
        Flow { 
            lines: vec![],
            runs: vec![]
        }
    }
    pub fn add_line(&mut self, words: Vec<Word>, kind: RunType) {
        if words.len() > 0 {
            self.runs.push(Run {
                lines: vec![Line { words }], 
                kind
            });
        }
    }
    pub fn add_table(&mut self, table: Table<CellContent>) {
        unimplemented!()
    }
}
