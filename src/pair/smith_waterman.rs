use std::cmp;

use pair::step_mask::StepMask;
use pair::strategy::Strategy;

#[derive(Clone, Debug)]
pub struct SmithWaterman {
    equal: isize,
    align: isize,
    insert: isize,
    delete: isize,
}

impl SmithWaterman {
    pub fn new(equal: isize, align: isize, insert: isize, delete: isize) -> SmithWaterman {
        SmithWaterman {
            equal,
            align,
            insert,
            delete,
        }
    }
}

impl Strategy for SmithWaterman {
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
        if score >= 0 {
            score
        } else {
            0
        }
    }

    fn step_mask(&self, align: isize, insert: isize, delete: isize) -> StepMask {
        if cmp::max(cmp::max(align, insert), delete) > 0 {
            StepMask::from_scores(align, insert, delete)
        } else {
            StepMask::STOP
        }
    }
}
