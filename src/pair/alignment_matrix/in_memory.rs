use std::fmt;

use pair::cursor::Cursor;
use pair::step_mask::StepMask;

use super::AlignmentMatrix as AlignmentMatrixTrait;

pub struct AlignmentMatrix {
    width: usize,
    height: usize,
    buffer: Vec<StepMask>,
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
        let buffer = Vec::with_capacity(width * height);
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
        let offset = self.offset(&cursor);
        unsafe { self.buffer.get_unchecked(offset) }.clone()
    }

    fn set_at(&mut self, cursor: &Cursor, step_mask: StepMask) {
        let offset = self.offset(&cursor);
        let byte_ref = unsafe { self.buffer.get_unchecked_mut(offset) };
        *byte_ref = step_mask;
    }
}

impl fmt::Debug for AlignmentMatrix {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cursor = Cursor { x: x, y: y };
                let _ = write!(form, "{:?}\t", self.at(&cursor));
            }
            let _ = write!(form, "\n");
        }
        write!(form, "\n")
    }
}
