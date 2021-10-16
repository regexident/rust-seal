use std::fmt;

use num_traits::Zero;

use pair::cursor::Cursor;
use pair::step_mask::StepMask;

use colored::*;

mod cell;

pub use self::cell::MatrixCell;

pub trait AlignmentMatrix: Sized {
    type Score;
    type Error;

    fn new(width: usize, height: usize) -> Result<Self, Self::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn cell(&self, cursor: &Cursor) -> &MatrixCell<Self::Score>;
    fn cell_mut(&mut self, cursor: &Cursor) -> &mut MatrixCell<Self::Score>;
}

pub struct MatrixRows<'a, T> {
    pub previous: &'a mut [MatrixCell<T>],
    pub current: &'a mut [MatrixCell<T>],
}

pub struct Matrix<T> {
    width: usize,
    height: usize,
    buffer: Vec<MatrixCell<T>>,
}

impl<T> Matrix<T>
where
    T: Zero + Copy,
{
    pub fn new(width: usize, height: usize) -> Self {
        let cell = unsafe { MatrixCell::new_unchecked(T::zero(), StepMask::empty()) };
        let buffer = vec![cell; width * height];
        Self {
            width,
            height,
            buffer,
        }
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows_mut(&mut self, index: usize) -> MatrixRows<'_, T> {
        let width = self.width();
        let mid = self.row_offset(index);

        let start = mid - width;
        let end = mid + width;
        let slice = &mut self.buffer[start..end];
        let (previous, current) = slice.split_at_mut(width);

        println!("{:?}, {:?}", previous.len(), current.len());

        MatrixRows { previous, current }
    }

    pub fn row(&self, index: usize) -> &[MatrixCell<T>] {
        let start = self.row_offset(index);
        let end = start + self.width;
        &self.buffer[start..end]
    }

    pub fn row_mut(&mut self, index: usize) -> &mut [MatrixCell<T>] {
        let start = self.row_offset(index);
        let end = start + self.width;
        println!("{:?}..{:?} ({:?})", start, end, self.buffer.len());
        &mut self.buffer[start..end]
    }

    fn row_offset(&self, index: usize) -> usize {
        index * self.width
    }

    fn print_row<F>(&self, row: usize, fmt: &mut fmt::Formatter, f: F) -> fmt::Result
    where
        F: Fn(&MatrixCell<T>) -> String,
    {
        for cell in self.row(row) {
            let string = f(cell);
            if cell.steps() == StepMask::STOP {
                write!(fmt, "{}\t", string.red())?;
            } else {
                write!(fmt, "{}\t", string)?;
            }
        }
        writeln!(fmt)
    }

    pub fn cell(&self, cursor: &Cursor) -> &MatrixCell<T> {
        let offset = self.cell_offset(cursor);
        unsafe { self.buffer.get_unchecked(offset) }
    }

    fn cell_offset(&self, cursor: &Cursor) -> usize {
        cursor.x + (cursor.y * self.width)
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: Copy + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            self.print_row(y, fmt, |cell| cell.steps().string())?;
            self.print_row(y, fmt, |cell| format!("{:2.2?}", cell.score()))?;
            writeln!(fmt)?;
        }
        Ok(())
    }
}
