use std::ops::Range;

use pair::step_mask::StepMask;

#[derive(Clone, Debug, PartialEq)]
pub enum Run {
    Align { x: Range<usize>, y: Range<usize> },
    Delete { x: Range<usize> },
    Insert { y: Range<usize> },
}

impl Run {
    pub fn mask(&self) -> StepMask {
        match *self {
            Run::Align { x: _, y: _ } => StepMask::ALIGN,
            Run::Delete { x: _ } => StepMask::DELETE,
            Run::Insert { y: _ } => StepMask::INSERT,
        }
    }
}
