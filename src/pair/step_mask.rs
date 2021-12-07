#![allow(dead_code)]
use std::cmp;

use bitflags::bitflags;

bitflags! {
    pub struct StepMask: u8 {
        const STOP   = 0b00000000;
        const ALIGN  = 0b00000010;
        const DELETE = 0b00000100;
        const INSERT = 0b00001000;
    }
}

impl StepMask {
    pub fn from_scores(align: isize, delete: isize, insert: isize) -> StepMask {
        let mut step_mask = StepMask::empty();
        if align >= cmp::max(delete, insert) {
            step_mask.insert(StepMask::ALIGN);
        }
        if delete >= cmp::max(align, insert) {
            step_mask.insert(StepMask::DELETE);
        }
        if insert >= cmp::max(align, delete) {
            step_mask.insert(StepMask::INSERT);
        }
        step_mask
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
