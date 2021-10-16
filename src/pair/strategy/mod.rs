use std::ops::RangeInclusive;

use num_traits::{NumAssign, Signed, Zero};

use pair::{
    alignments::{Alignment, Alignments},
    cursor::Cursor,
    matrix::{Matrix, MatrixCell, MatrixRows},
    penalty::Penalty,
    step_mask::StepMask,
};

pub mod discrete;
pub mod non_discrete;

pub trait Strategy<T> {
    type Score: NumAssign + Signed + PartialOrd + Copy;

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

        println!("c: {}, r: {}", columns, rows);

        let mut matrix = Matrix::new(columns, rows);

        let zero_row_scores = (1..columns).scan(Self::Score::zero(), |score, column| {
            *score = self.boundary_score(*score);
            Some((column, *score))
        });
        let row = matrix.row_mut(0);
        for (column, score) in zero_row_scores {
            row[column] = MatrixCell::with_bounds(score, StepMask::DELETE, bounds);
        }

        let score = Self::Score::zero();
        row[0] = MatrixCell::with_bounds(score, StepMask::STOP, bounds);
        let upper_bound = (*bounds.end(), Cursor::new(1, 1));
        let optimum = (1..rows).fold(upper_bound, |optimum, row| {
            let y_val = &y[row - 1];

            let MatrixRows {
                previous: previous_row,
                current: current_row,
            } = matrix.rows_mut(row);

            let previous_score = previous_row[0].score();
            let score = self.boundary_score(previous_score);
            current_row[0] = MatrixCell::with_bounds(score, StepMask::INSERT, bounds);

            let min_column = 1.max(window.max(row) - window);
            let max_column = columns.min(row + window + 1);
            (min_column..max_column).fold(optimum, |optimum, column| {
                let x_val = &x[column - 1];

                let align_cell = &previous_row[column - 1];
                let insert_cell = &previous_row[column];
                let delete_cell = &current_row[column - 1];

                let cost = f(x_val, y_val);
                let is_match = cost <= Self::Score::zero();

                let align_penalty = if is_match {
                    penalty.r#match
                } else {
                    penalty.mismatch
                };
                let insert_penalty = penalty.gap;
                let delete_penalty = penalty.gap;

                let align = align_cell.score() + (cost.abs() * align_penalty);
                let insert = insert_cell.score() + (cost.abs() * insert_penalty);
                let delete = delete_cell.score() + (cost.abs() * delete_penalty);

                let cursor = Cursor::new(column, row);
                let cell = MatrixCell::from_steps(align, delete, insert, bounds);
                let score = cell.score();
                current_row[column] = cell;

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
