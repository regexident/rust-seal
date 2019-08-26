use pair::matrix::Matrix;
use pair::cursor::Cursor;

mod alignment;
mod iter;

pub use self::alignment::Alignment;
pub use self::iter::Iter;

#[derive(Clone, Copy, Debug)]
pub enum AlignmentScope {
    Local,
    Global,
}

#[derive(Debug)]
pub struct Alignments<T> {
    matrix: Matrix<T>,
    cursor: Cursor,
}

impl<T> Alignments<T> where T: Clone {
    pub fn new(matrix: Matrix<T>, cursor: Cursor) -> Self {
        Self { matrix, cursor }
    }

    pub fn matrix(&self) -> &Matrix<T> {
        &self.matrix
    }

    pub fn score(&self) -> &T {
        &self.matrix.cell(&self.cursor).score
    }

    pub fn alignment(&self) -> Option<Alignment<T>> {
        self.iter().next()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(&self.matrix, self.cursor, self.score().clone())
    }
}
