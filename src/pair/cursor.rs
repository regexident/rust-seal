use pair::step_mask::StepMask;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn apply_forwards_step(&mut self, step_mask: StepMask) {
        self.apply_step(step_mask, true)
    }

    pub fn apply_backwards_step(&mut self, step_mask: StepMask) {
        self.apply_step(step_mask, false)
    }

    fn apply_step(&mut self, step_mask: StepMask, forward: bool) {
        let delta = match step_mask {
            StepMask::ALIGN => (1, 1),
            StepMask::INSERT => (0, 1),
            StepMask::DELETE => (1, 0),
            StepMask::STOP => (0, 0),
            _ => {
                panic!("Invalid step_mask.");
            }
        };
        if forward {
            self.x += delta.0;
            self.y += delta.1;
        } else {
            self.x -= delta.0;
            self.y -= delta.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_works() {
        {
            let mut cursor = Cursor { x: 10, y: 10 };
            cursor.apply_backwards_step(StepMask::STOP);
            assert_eq!(cursor, Cursor { x: 10, y: 10 });
        }
        {
            let mut cursor = Cursor { x: 10, y: 10 };
            cursor.apply_backwards_step(StepMask::ALIGN);
            assert_eq!(cursor, Cursor { x: 9, y: 9 });
        }
        {
            let mut cursor = Cursor { x: 10, y: 10 };
            cursor.apply_backwards_step(StepMask::INSERT);
            assert_eq!(cursor, Cursor { x: 10, y: 9 });
        }
        {
            let mut cursor = Cursor { x: 10, y: 10 };
            cursor.apply_backwards_step(StepMask::DELETE);
            assert_eq!(cursor, Cursor { x: 9, y: 10 });
        }
    }
}
