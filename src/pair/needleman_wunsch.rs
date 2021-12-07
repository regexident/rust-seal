use crate::pair::step_mask::StepMask;
use crate::pair::strategy::Strategy;

#[derive(Clone, Debug)]
pub struct NeedlemanWunsch {
    pub equal: isize,
    pub align: isize,
    pub insert: isize,
    pub delete: isize,
}

impl NeedlemanWunsch {
    pub fn new(equal: isize, align: isize, insert: isize, delete: isize) -> NeedlemanWunsch {
        NeedlemanWunsch {
            equal,
            align,
            insert,
            delete,
        }
    }
}

impl Strategy for NeedlemanWunsch {
    fn match_score(&self) -> isize {
        self.equal
    }

    fn mismatch_score(&self) -> isize {
        self.align
    }

    fn insert_score(&self) -> isize {
        self.insert
    }

    fn delete_score(&self) -> isize {
        self.delete
    }

    fn total_score(&self, score: isize) -> isize {
        score
    }

    fn step_mask(&self, align: isize, insert: isize, delete: isize) -> StepMask {
        StepMask::from_scores(align, insert, delete)
    }
}
