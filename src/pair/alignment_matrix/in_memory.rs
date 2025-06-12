use std::{fmt, mem::MaybeUninit};

use crate::pair::cursor::Cursor;
use crate::pair::step_mask::StepMask;

use super::AlignmentMatrix as AlignmentMatrixTrait;

pub struct AlignmentMatrix {
    width: usize,
    height: usize,
    buffer: Vec<MaybeUninit<StepMask>>,
}

impl AlignmentMatrix {
    fn offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }
}

impl AlignmentMatrixTrait for AlignmentMatrix {
    // FIXME: use never type, once stabilized!
    type Error = ();

    fn new(width: usize, height: usize) -> Result<Self, Self::Error> {
        let buffer = vec![MaybeUninit::uninit(); width * height];
        Ok(Self {
            width,
            height,
            buffer,
        })
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn at(&self, cursor: &Cursor) -> StepMask {
        let offset = self.offset(cursor);
        // Safety: Assumes `set_at` was called for this cursor, initializing the value.
        unsafe { self.buffer[offset].assume_init_read() }
    }

    fn set_at(&mut self, cursor: &Cursor, step_mask: StepMask) {
        let offset = self.offset(cursor);
        // This is safe because we are writing to a `MaybeUninit` slot.
        self.buffer[offset] = MaybeUninit::new(step_mask);
    }
}

impl fmt::Debug for AlignmentMatrix {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cursor = Cursor { x, y };
                let _ = write!(form, "{:?}\t", self.at(&cursor));
            }
            let _ = writeln!(form);
        }
        writeln!(form)
    }
}
