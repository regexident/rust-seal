use std::{
    fmt,
    ops::{Add, RangeInclusive},
};

use pair::step_mask::StepMask;

pub struct MatrixCell<T> {
    pub score: T,
    pub steps: StepMask,
}

impl<T> MatrixCell<T>
where
    T: Clone + PartialOrd + Add<Output = T>,
{
    pub fn with_bounds(score: T, steps: StepMask, bounds: &RangeInclusive<T>) -> Self {
        if score <= *bounds.start() {
            Self {
                score: bounds.start().clone(),
                steps: StepMask::STOP,
            }
        } else if score >= *bounds.end() {
            Self {
                score: bounds.end().clone(),
                steps: StepMask::STOP,
            }
        } else {
            Self { score, steps }
        }
    }

    pub fn from_steps(align: T, delete: T, insert: T, bounds: &RangeInclusive<T>) -> Self {
        let mut steps = StepMask::empty();
        let mut score = align.clone();

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
        Self::with_bounds(score, steps, bounds)
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