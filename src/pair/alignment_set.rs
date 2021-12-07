use std::cmp;

use crate::pair::alignment::Alignment;
use crate::pair::alignment_matrix::AlignmentMatrix;
use crate::pair::alignments::Alignments;
use crate::pair::cursor::Cursor;
use crate::pair::step_mask::StepMask;
use crate::pair::strategy::Strategy;

#[derive(Copy, Clone)]
struct Highscore {
    score: isize,
    cursor: Cursor,
}

struct Highscores {
    local: Highscore,
    global: Highscore,
}

impl Highscores {
    fn update(&mut self, highscore: Highscore) {
        if highscore.score >= self.local.score {
            self.local = highscore
        }
        self.global = highscore;
    }
}

pub struct AlignmentSet<T> {
    matrix: T,
    highscores: Highscores,
}

impl<T, E> AlignmentSet<T>
where
    T: AlignmentMatrix<Error = E>,
{
    pub fn new<S: Strategy, F>(
        x_len: usize,
        y_len: usize,
        strategy: S,
        f: F,
    ) -> Result<AlignmentSet<T>, E>
    where
        F: Fn(usize, usize) -> bool,
    {
        let width = x_len + 1;
        let height = y_len + 1;

        let mut matrix = T::new(width, height)?;

        Self::prepare_matrix(&mut matrix);
        let mut row = Self::prepared_row(width, &strategy);
        let mut highscores = Self::prepared_highscores();

        for y in 0..y_len {
            let mut last_diagonal = strategy.total_score(strategy.mismatch_score() * (y as isize));
            row[0] = strategy.total_score(strategy.insert_score() * ((y + 1) as isize));
            for x in 0..x_len {
                let previous = (last_diagonal, row[x], row[x + 1]);
                let equal = f(x, y);
                let (steps, score) = Self::calculate_cell(&strategy, previous, equal);
                let cursor = Cursor { x: x + 1, y: y + 1 };
                highscores.update(Highscore { cursor, score });
                matrix.set_at(&cursor, steps);
                let old_diagonal = row[x + 1];
                row[x + 1] = score;
                last_diagonal = old_diagonal;
            }
        }

        Ok(AlignmentSet { matrix, highscores })
    }

    fn calculate_cell<S: Strategy>(
        strategy: &S,
        previous_scores: (isize, isize, isize),
        equal: bool,
    ) -> (StepMask, isize) {
        let (mut align, mut delete, mut insert) = previous_scores;
        align += if equal {
            strategy.match_score()
        } else {
            strategy.mismatch_score()
        };
        delete += strategy.delete_score();
        insert += strategy.insert_score();
        let steps = StepMask::from_scores(align, delete, insert);
        let score = strategy.total_score(cmp::max(cmp::max(align, delete), insert));
        (steps, score)
    }

    fn prepare_matrix(matrix: &mut T) {
        for y in 1..matrix.height() {
            let cursor = Cursor { x: 0, y };
            matrix.set_at(&cursor, StepMask::INSERT);
        }
        for x in 1..matrix.width() {
            let cursor = Cursor { x, y: 0 };
            matrix.set_at(&cursor, StepMask::DELETE);
        }
    }

    fn prepared_row<S: Strategy>(width: usize, strategy: &S) -> Vec<isize> {
        (0..width)
            .map(|i| strategy.total_score(strategy.delete_score() * (i as isize)))
            .collect()
    }

    fn prepared_highscores() -> Highscores {
        Highscores {
            local: Highscore {
                score: 0,
                cursor: Cursor { x: 0, y: 0 },
            },
            global: Highscore {
                score: 0,
                cursor: Cursor { x: 0, y: 0 },
            },
        }
    }

    pub fn local_score(&self) -> isize {
        self.highscores.local.score
    }

    pub fn global_score(&self) -> isize {
        self.highscores.global.score
    }

    pub fn local_max(&self) -> &Cursor {
        &self.highscores.local.cursor
    }

    pub fn global_max(&self) -> &Cursor {
        &self.highscores.global.cursor
    }

    pub fn local_alignment(&self) -> Alignment {
        self.local_alignments().next().unwrap()
    }

    pub fn global_alignment(&self) -> Alignment {
        self.global_alignments().next().unwrap()
    }

    pub fn local_alignments(&self) -> Alignments<'_, T> {
        let stack = vec![(StepMask::STOP, self.highscores.local.cursor, 0)];
        Alignments::new(&self.matrix, stack, vec![], self.highscores.local.score)
    }

    pub fn global_alignments(&self) -> Alignments<'_, T> {
        let stack = vec![(StepMask::STOP, self.highscores.global.cursor, 0)];
        Alignments::new(&self.matrix, stack, vec![], self.highscores.global.score)
    }

    pub fn matrix(&self) -> &T {
        &self.matrix
    }
}
