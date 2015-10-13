use std::ops::RangeInclusive;

use pair::cursor::Cursor;
use pair::step_mask::StepMask;
use pair::strategy::Strategy as StrategyTrait;
use pair::{
    alignment_matrix::{AlignmentMatrix, AlignmentMatrixCell},
    alignment_set::AlignmentSet,
};

#[derive(Clone, Debug)]
pub struct Strategy {
    equal: isize,
    align: isize,
    insert: isize,
    delete: isize,
    bounds: RangeInclusive<isize>,
}

impl Strategy {
    pub fn new(
        equal: isize,
        align: isize,
        insert: isize,
        delete: isize,
        bounds: RangeInclusive<isize>,
    ) -> Self {
        Self {
            equal,
            align,
            insert,
            delete,
            bounds,
        }
    }

    // Levenshtein algorithm calculating optimal global alignment:
    pub fn levenshtein() -> Self {
        let max = isize::max_value();
        Self::new(0, 1, 1, 1, 0..=max)
    }

    // Needleman-Wunsch algorithm calculating optimal global alignment:
    pub fn needleman_wunsch() -> Self {
        let min = isize::min_value();
        let max = isize::max_value();
        Self::new(-1, 1, 1, 1, min..=max)
    }

    // Needleman-Wunsch algorithm calculating optimal local alignment:
    pub fn smith_waterman() -> Self {
        let min = isize::min_value();
        Self::new(-2, 1, 1, 1, min..=0)
    }

    fn total_score(&self, score: isize) -> isize {
        if score < *self.bounds.start() {
            self.bounds.start().clone()
        } else if score > *self.bounds.end() {
            self.bounds.end().clone()
        } else {
            score
        }
    }

    fn calculate_cell<T: Eq>(
        &self,
        lhs: &T,
        rhs: &T,
        previous_scores: [isize; 3],
    ) -> AlignmentMatrixCell<isize> {
        let [mut insert, mut align, mut delete] = previous_scores;
        align += if lhs == rhs { self.equal } else { self.align };
        delete += self.delete;
        insert += self.insert;
        let mut cell = AlignmentMatrixCell::from_scores(align, delete, insert);
        cell.score = self.total_score(cell.score.clone());
        cell
    }
}

impl<T> StrategyTrait<T> for Strategy
where
    T: Eq,
{
    type Score = isize;

    fn alignment_set<M, E>(&self, x: &[T], y: &[T]) -> Result<AlignmentSet<isize, M>, E>
    where
        M: AlignmentMatrix<Score = Self::Score, Error = E>,
        M: ::std::fmt::Debug,
    {
        let width = x.len() + 1;
        let height = y.len() + 1;

        let mut matrix = M::new(width, height)?;

        for y in 1..height {
            let cell = matrix.cell_mut(&Cursor::new(0, y));
            cell.steps = StepMask::INSERT;
            // matrix.cell_mut(&Cursor::new(0, y)).steps = StepMask::INSERT;
        }
        for x in 1..width {
            matrix.cell_mut(&Cursor::new(x, 0)).steps = StepMask::DELETE;
        }

        let mut row: Vec<Self::Score> = (0..width)
            .map(|i| self.total_score(self.delete * (i as isize)))
            .collect();

        let mut score: isize = isize::max_value();
        let mut cursor = Cursor::new(1, 1);

        for (y_idx, y_val) in y.iter().enumerate() {
            let mut last_diagonal = self.total_score(self.align * (y_idx as isize));
            row[0] = self.total_score(self.insert * ((y_idx + 1) as isize));
            for (x_idx, x_val) in x.iter().enumerate() {
                let previous = [row[x_idx + 1], last_diagonal, row[x_idx]];
                let cell = self.calculate_cell(&x_val, &y_val, previous);
                let current_cursor = Cursor {
                    x: x_idx + 1,
                    y: y_idx + 1,
                };
                let cell_score = cell.score.clone();
                if cell_score < score {
                    score = cell_score;
                    cursor = current_cursor;
                }
                *matrix.cell_mut(&current_cursor) = cell;
                let old_diagonal = row[x_idx + 1];
                row[x_idx + 1] = cell_score;
                last_diagonal = old_diagonal;
            }
        }

        Ok(AlignmentSet::new(matrix, score, cursor))
    }
}
