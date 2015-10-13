use std::slice::Iter;

use pair::cursor::Cursor;
use pair::step::Step;
use pair::step_mask::StepMask;

pub struct Steps<'a> {
    pub inner: Iter<'a, StepMask>,
    pub cursor: Cursor,
}

impl<'a> Steps<'a> {
    pub fn new(iter: Iter<'a, StepMask>, cursor: Cursor) -> Steps {
        Steps {
            inner: iter,
            cursor: cursor,
        }
    }
}

impl<'a> Iterator for Steps<'a> {
    type Item = Step;

    fn next(&mut self) -> Option<Step> {
        self.inner.next().and_then(|mask| {
            let cursor = self.cursor;
            self.cursor.apply_forwards_step(*mask);
            match mask {
                &StepMask::ALIGN => Some(Step::Align {
                    x: cursor.x,
                    y: cursor.y,
                }),
                &StepMask::DELETE => Some(Step::Delete { x: cursor.x }),
                &StepMask::INSERT => Some(Step::Insert { y: cursor.y }),
                _ => None,
            }
        })
    }
}
