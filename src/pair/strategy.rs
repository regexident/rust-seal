use crate::pair::StepMask;

pub trait Strategy {
    fn match_score(&self) -> isize;
    fn mismatch_score(&self) -> isize;
    fn insert_score(&self) -> isize;
    fn delete_score(&self) -> isize;
    fn total_score(&self, strategy: isize) -> isize;
    fn step_mask(&self, align: isize, insert: isize, delete: isize) -> StepMask;
}
