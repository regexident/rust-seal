use std::ops::RangeInclusive;

use num_traits::{NumAssign, Signed, Zero};

use pair::{
    alignments::{Alignment, Alignments},
    cursor::Cursor,
    matrix::{Matrix, MatrixCell},
    penalty::Penalty,
    step_mask::StepMask,
};

pub mod discrete;
pub mod non_discrete;

pub trait Strategy<T> {
    type Score: NumAssign + Signed + PartialOrd + Clone;

    fn penalty(&self) -> &Penalty<Self::Score>;
    fn window(&self) -> usize;
    fn bounds(&self) -> &RangeInclusive<Self::Score>;

    fn boundary_score(&self, prev_score: Self::Score) -> Self::Score;
    fn pick_optimum(
        &self,
        lhs: (Self::Score, Cursor),
        rhs: (Self::Score, Cursor),
    ) -> (Self::Score, Cursor);

    fn alignment<F>(&self, x: &[T], y: &[T], f: F) -> Option<Alignment<Self::Score>>
    where
        F: Fn(&T, &T) -> Self::Score,
    {
        self.alignments(x, y, f).alignment()
    }

    fn alignments<F>(&self, x: &[T], y: &[T], f: F) -> Alignments<Self::Score>
    where
        F: Fn(&T, &T) -> Self::Score,
    {
        let columns = x.len() + 1;
        let rows = y.len() + 1;

        let window = if columns <= rows {
            self.window().min(columns).max(rows - columns)
        } else {
            self.window().min(columns).max(columns - rows)
        };

        let penalty = self.penalty();
        let bounds = self.bounds();

        let mut matrix = Matrix::new(columns, rows);

        let zero_column_scores = (1..rows).scan(Self::Score::zero(), |score, row| {
            *score = self.boundary_score(score.clone());
            Some((row, score.clone()))
        });
        for (row, score) in zero_column_scores {
            let cell = MatrixCell::with_bounds(score, StepMask::INSERT, bounds);
            *matrix.cell_mut(&Cursor::new(0, row)) = cell;
        }
        let zero_row_scores = (1..columns).scan(Self::Score::zero(), |score, column| {
            *score = self.boundary_score(score.clone());
            Some((column, score.clone()))
        });
        for (column, score) in zero_row_scores {
            let cell = MatrixCell::with_bounds(score, StepMask::DELETE, bounds);
            *matrix.cell_mut(&Cursor::new(column, 0)) = cell;
        }

        let score = Self::Score::zero();
        let cell = MatrixCell::with_bounds(score, StepMask::STOP, bounds);
        *matrix.cell_mut(&Cursor::new(0, 0)) = cell;

        let upper_bound = (bounds.end().clone(), Cursor::new(1, 1));
        let optimum = (1..rows).fold(upper_bound, |optimum, row| {
            let y_val = &y[row - 1];
            let min_column = 1.max(window.max(row) - window);
            let max_column = columns.min(row + window + 1);
            (min_column..max_column).fold(optimum, |optimum, column| {
                let x_val = &x[column - 1];
                let align_cell = matrix.cell(&Cursor::new(column - 1, row - 1));
                let insert_cell = matrix.cell(&Cursor::new(column, row - 1));
                let delete_cell = matrix.cell(&Cursor::new(column - 1, row));

                let cost = f(x_val, y_val);
                let align_penalty = if cost <= Self::Score::zero() {
                    cost.abs() * penalty.r#match.clone()
                } else {
                    cost.abs() * penalty.mismatch.clone()
                };
                let insert_penalty = penalty.gap.clone();
                let delete_penalty = penalty.gap.clone();

                let align = align_cell.score.clone() + align_penalty;
                let insert = insert_cell.score.clone() + insert_penalty;
                let delete = delete_cell.score.clone() + delete_penalty;

                let cursor = Cursor::new(column, row);
                let cell = MatrixCell::from_steps(align, delete, insert, bounds);
                let score = cell.score.clone();
                *matrix.cell_mut(&cursor) = cell;

                self.pick_optimum(optimum, (score, cursor))
            })
        });

        let cursor = optimum.1;

        Alignments::new(matrix, cursor)
    }

    fn distance<F>(&self, x: &[T], y: &[T], f: F) -> Option<Self::Score>
    where
        F: Fn(&T, &T) -> Self::Score,
    {
        self.alignments(x, y, f)
            .alignment()
            .map(|alignment| alignment.score())
    }
}
