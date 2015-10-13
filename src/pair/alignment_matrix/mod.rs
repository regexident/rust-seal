use pair::cursor::Cursor;

use pair::step_mask::StepMask;

pub mod in_memory;
pub mod memory_mapped;

#[derive(Clone, Debug)]
pub struct AlignmentMatrixCell<T> {
    pub score: T,
    pub steps: StepMask,
}

impl<T> AlignmentMatrixCell<T>
where
    T: Clone + PartialOrd,
{
    pub fn new(score: T, steps: StepMask) -> Self {
        Self { score, steps }
    }

    pub fn from_scores(align: T, delete: T, insert: T) -> Self {
        let mut score = align.clone();
        let mut steps = StepMask::empty();
        if (align <= delete) && (align <= insert) {
            steps.insert(StepMask::ALIGN);
            score = align.clone();
        }
        if (delete <= align) && (delete <= insert) {
            steps.insert(StepMask::DELETE);
            score = delete.clone();
        }
        if (insert <= align) && (insert <= delete) {
            steps.insert(StepMask::INSERT);
            score = insert.clone();
        }
        Self { score, steps }
    }
}

pub trait AlignmentMatrix: Sized {
    type Score;
    type Error;

    fn new(width: usize, height: usize) -> Result<Self, Self::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn cell(&self, cursor: &Cursor) -> &AlignmentMatrixCell<Self::Score>;
    fn cell_mut(&mut self, cursor: &Cursor) -> &mut AlignmentMatrixCell<Self::Score>;
}
