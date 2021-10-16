use pair::cursor::Cursor;
use pair::matrix::Matrix;

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
pub struct Alignments<T>
where
    T: Copy,
{
    matrix: Matrix<T>,
    cursor: Cursor,
}

impl<T> Alignments<T>
where
    T: Copy,
{
    pub fn new(matrix: Matrix<T>, cursor: Cursor) -> Self {
        Self { matrix, cursor }
    }

    pub fn matrix(&self) -> &Matrix<T> {
        &self.matrix
    }

    pub fn score(&self) -> T {
        let Cursor { x, y } = self.cursor;
        self.matrix.row(y)[x].score()
    }

    pub fn alignment(&self) -> Option<Alignment<T>> {
        self.iter().next()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self.matrix, self.cursor, self.score())
    }
}
