use pair::alignment::Alignment;
use pair::alignment_matrix::AlignmentMatrix;
use pair::alignments::Alignments;
use pair::cursor::Cursor;
use pair::step_mask::StepMask;

#[derive(Clone, Debug)]
pub enum AlignmentScope {
    Local,
    Global,
}

#[derive(Clone, Debug)]
pub struct AlignmentSet<T, M> {
    matrix: M,
    score: T,
    cursor: Cursor,
}

impl<T, M> AlignmentSet<T, M> {
    pub fn new(matrix: M, score: T, cursor: Cursor) -> Self {
        Self {
            matrix,
            score,
            cursor,
        }
    }

    pub fn matrix(&self) -> &M {
        &self.matrix
    }
}

impl<T, M, E> AlignmentSet<T, M>
where
    T: Clone,
    M: AlignmentMatrix<Score = T, Error = E>,
{
    pub fn score(&self) -> T {
        self.score.clone()
    }

    pub fn alignment(&self, scope: AlignmentScope) -> Alignment<T> {
        self.alignments(scope).next().unwrap()
    }

    pub fn alignments<'a>(&'a self, scope: AlignmentScope) -> Alignments<'a, T, M> {
        let cursor = match scope {
            AlignmentScope::Local => self.cursor,
            AlignmentScope::Global => {
                Cursor::new(self.matrix.width() - 1, self.matrix.height() - 1)
            }
        };
        let stack = vec![(StepMask::STOP, cursor, 0)];
        Alignments::new(&self.matrix, stack, vec![], self.score.clone())
    }
}
