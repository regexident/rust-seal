use std::{fmt, ops::RangeInclusive};

use pair::step_mask::StepMask;

#[derive(Clone, Copy)]
pub struct MatrixCell<T> {
    score: T,
    steps: StepMask,
}

impl<T> MatrixCell<T> {
    pub unsafe fn new_unchecked(score: T, steps: StepMask) -> Self {
        Self { score, steps }
    }
}

impl<T> MatrixCell<T>
where
    T: Copy + PartialOrd,
{
    pub fn with_bounds(score: T, steps: StepMask, bounds: &RangeInclusive<T>) -> Self {
        if score <= *bounds.start() {
            Self {
                score: *bounds.start(),
                steps: StepMask::STOP,
            }
        } else if score >= *bounds.end() {
            Self {
                score: *bounds.end(),
                steps: StepMask::STOP,
            }
        } else {
            Self { score, steps }
        }
    }

    pub fn from_steps(align: T, delete: T, insert: T, bounds: &RangeInclusive<T>) -> Self {
        let mut steps = StepMask::empty();
        let mut score = align;

        if (align <= delete) && (align <= insert) {
            steps.insert(StepMask::ALIGN);
            score = align;
        }
        if (delete <= align) && (delete <= insert) {
            steps.insert(StepMask::DELETE);
            score = delete;
        }
        if (insert <= align) && (insert <= delete) {
            steps.insert(StepMask::INSERT);
            score = insert;
        }
        Self::with_bounds(score, steps, bounds)
    }
}

impl<T> MatrixCell<T>
where
    T: Copy,
{
    pub fn score(&self) -> T {
        self.score
    }

    pub fn steps(&self) -> StepMask {
        self.steps
    }
}

impl<T> fmt::Debug for MatrixCell<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({:?}, {:?})", self.steps, self.score)
    }
}
