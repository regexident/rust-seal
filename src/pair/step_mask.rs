bitflags! {
    pub struct StepMask: u8 {
        const STOP   = 0b_0000_0000;
        const ALIGN  = 0b_0000_0010;
        const DELETE = 0b_0000_0100;
        const INSERT = 0b_0000_1000;
    }
}

impl StepMask {
    pub fn from_scores<T: PartialOrd>(align: T, delete: T, insert: T) -> StepMask {
        let mut step_mask = StepMask::empty();
        if (align <= delete) && (align <= insert) {
            step_mask.insert(StepMask::ALIGN);
        }
        if (delete <= align) && (delete <= insert) {
            step_mask.insert(StepMask::DELETE);
        }
        if (insert <= align) && (insert <= delete) {
            step_mask.insert(StepMask::INSERT);
        }
        step_mask
    }

    pub fn string(&self) -> String {
        let mut string = String::default();
        string.push_str(if self.contains(StepMask::INSERT) {
            "I"
        } else {
            "-"
        });
        string.push_str(if self.contains(StepMask::ALIGN) {
            "A"
        } else {
            "-"
        });
        string.push_str(if self.contains(StepMask::DELETE) {
            "D"
        } else {
            "-"
        });
        string
    }
}
