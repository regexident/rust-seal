use std::fmt;

use pair::cursor::Cursor;
use pair::step_mask::StepMask;

use colored::*;

mod cell;

pub use self::cell::MatrixCell;

pub struct Matrix<T> {
    width: usize,
    height: usize,
    buffer: Vec<MatrixCell<T>>,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = Vec::with_capacity(width * height);
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cell(&self, cursor: &Cursor) -> &MatrixCell<T> {
        let offset = self.offset(&cursor);
        unsafe { self.buffer.get_unchecked(offset) }
    }

    pub fn cell_mut(&mut self, cursor: &Cursor) -> &mut MatrixCell<T> {
        let offset = self.offset(&cursor);
        unsafe { self.buffer.get_unchecked_mut(offset) }
    }

    fn offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }

    fn print_row<F>(&self, row: usize, fmt: &mut fmt::Formatter, f: F) -> fmt::Result
    where
        F: Fn(&MatrixCell<T>) -> String,
    {
        for column in 0..self.width() {
            let cursor = Cursor::new(column, row);
            let cell = self.cell(&cursor);
            let string = f(cell);
            if cell.steps == StepMask::STOP {
                write!(fmt, "{}\t", string.red())?;
            } else {
                write!(fmt, "{}\t", string)?;
            }
        }
        writeln!(fmt)
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(for y in 0..self.height() {
            self.print_row(y, fmt, |cell| cell.steps.string())?;
            self.print_row(y, fmt, |cell| format!("{:2.2?}", cell.score))?;
            writeln!(fmt)?
        })
    }
}
