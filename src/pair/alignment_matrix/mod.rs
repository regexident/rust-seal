use crate::pair::{cursor::Cursor, step_mask::StepMask};

pub mod in_memory;
pub mod memory_mapped;

pub trait AlignmentMatrix: Sized {
    type Error;

    fn new(width: usize, height: usize) -> Result<Self, Self::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn at(&self, cursor: &Cursor) -> StepMask;
    fn set_at(&mut self, cursor: &Cursor, step_mask: StepMask);
}
