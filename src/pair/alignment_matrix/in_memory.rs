use std::fmt;

use pair::cursor::Cursor;

use super::{AlignmentMatrix as AlignmentMatrixTrait, AlignmentMatrixCell};

pub struct AlignmentMatrix<T> {
    width: usize,
    height: usize,
    buffer: Vec<AlignmentMatrixCell<T>>,
}

impl<T> AlignmentMatrix<T> {
    fn offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }
}

impl<T> AlignmentMatrixTrait for AlignmentMatrix<T> {
    type Score = T;

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

    fn cell(&self, cursor: &Cursor) -> &AlignmentMatrixCell<Self::Score> {
        let offset = self.offset(&cursor);
        unsafe { self.buffer.get_unchecked(offset) }
    }

    fn cell_mut(&mut self, cursor: &Cursor) -> &mut AlignmentMatrixCell<Self::Score> {
        let offset = self.offset(&cursor);
        unsafe { self.buffer.get_unchecked_mut(offset) }
    }
}

impl<T> fmt::Debug for AlignmentMatrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cursor = Cursor::new(x, y);

                let cell = self.cell(&cursor);
                // let string = cell.steps.string();
                let string = format!("{:?}", cell.score);
                write!(fmt, "{}\t", string)?;

                // write!(fmt, "{:?}\t", self.cell(&cursor).score)?;
            }
            writeln!(fmt)?;
        }
        writeln!(fmt)
    }
}
