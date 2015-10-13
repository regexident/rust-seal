use std::fmt;

use pair::step_mask::StepMask;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn apply_forwards_step(&mut self, step: StepMask) {
        self.apply_step(step, true)
    }

    pub fn apply_backwards_step(&mut self, step: StepMask) {
        self.apply_step(step, false)
    }

    fn apply_step(&mut self, step: StepMask, forward: bool) {
        let delta = match step {
            StepMask::ALIGN => (1, 1),
            StepMask::INSERT => (0, 1),
            StepMask::DELETE => (1, 0),
            StepMask::STOP => (0, 0),
            _ => {
                panic!("Invalid step.");
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

impl fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_works() {
        {
            let mut cursor = Cursor::new(10, 10);
            cursor.apply_backwards_step(StepMask::STOP);
            assert_eq!(cursor, Cursor::new(10, 10));
        }
        {
            let mut cursor = Cursor::new(10, 10);
            cursor.apply_backwards_step(StepMask::ALIGN);
            assert_eq!(cursor, Cursor::new(9, 9));
        }
        {
            let mut cursor = Cursor::new(10, 10);
            cursor.apply_backwards_step(StepMask::INSERT);
            assert_eq!(cursor, Cursor::new(10, 9));
        }
        {
            let mut cursor = Cursor::new(10, 10);
            cursor.apply_backwards_step(StepMask::DELETE);
            assert_eq!(cursor, Cursor::new(9, 10));
        }
    }
}
