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

    pub fn forwards_step(&self, step: StepMask) -> Option<Cursor> {
        self.step(step, true)
    }

    pub fn backwards_step(&self, step: StepMask) -> Option<Cursor> {
        self.step(step, false)
    }

    fn step(&self, step: StepMask, forward: bool) -> Option<Cursor> {
        let delta = match step {
            StepMask::ALIGN => (1, 1),
            StepMask::INSERT => (0, 1),
            StepMask::DELETE => (1, 0),
            StepMask::STOP => return None,
            _ => {
                panic!("Invalid step.");
            }
        };
        if forward {
            Some(Self {
                x: self.x + delta.0,
                y: self.y + delta.1,
            })
        } else {
            Some(Self {
                x: self.x - delta.0,
                y: self.y - delta.1,
            })
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
            let cursor = Cursor::new(10, 10).backwards_step(StepMask::STOP);
            assert_eq!(cursor, None);
        }
        {
            let cursor = Cursor::new(10, 10).backwards_step(StepMask::STOP);
            assert_eq!(cursor, Some(Cursor::new(10, 10)));
        }
        {
            let cursor = Cursor::new(10, 10).backwards_step(StepMask::ALIGN);
            assert_eq!(cursor, Some(Cursor::new(9, 9)));
        }
        {
            let cursor = Cursor::new(10, 10).backwards_step(StepMask::INSERT);
            assert_eq!(cursor, Some(Cursor::new(10, 9)));
        }
        {
            let cursor = Cursor::new(10, 10).backwards_step(StepMask::DELETE);
            assert_eq!(cursor, Some(Cursor::new(9, 10)));
        }
    }
}
