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
        match self {
            Run::Align { .. } => StepMask::ALIGN,
            Run::Delete { .. } => StepMask::DELETE,
            Run::Insert { .. } => StepMask::INSERT,
        }
    }
}
