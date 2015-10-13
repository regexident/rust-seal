use pair::alignment::Alignment;
use pair::alignment_matrix::AlignmentMatrix;
use pair::cursor::Cursor;
use pair::step_mask::StepMask;

pub struct Alignments<'a, T, M: 'a> {
    matrix: &'a M,
    stack: Vec<(StepMask, Cursor, usize)>,
    steps: Vec<StepMask>,
    score: T,
}

impl<'a, T, M> Alignments<'a, T, M> {
    pub fn new(
        matrix: &'a M,
        stack: Vec<(StepMask, Cursor, usize)>,
        steps: Vec<StepMask>,
        score: T,
    ) -> Self {
        Alignments {
            matrix,
            stack,
            steps,
            score,
        }
    }
}

impl<'a, T, M> Alignments<'a, T, M>
where
    M: AlignmentMatrix<Score = T>,
{
    fn branches(&self, cursor: Cursor) -> Vec<(StepMask, Cursor)> {
        let steps = self.matrix.cell(&cursor).steps;
        let mut branches = vec![];
        if steps == StepMask::STOP {
            return branches;
        }
        for mask in [StepMask::ALIGN, StepMask::INSERT, StepMask::DELETE].iter() {
            if steps.contains(*mask) {
                let mut branch = cursor;
                branch.apply_backwards_step(*mask);
                branches.push((*mask, branch));
            }
        }
        branches
    }
}

impl<'a, T, M> Iterator for Alignments<'a, T, M>
where
    T: Clone,
    M: AlignmentMatrix<Score = T>,
{
    type Item = Alignment<T>;

    fn next(&mut self) -> Option<Alignment<T>> {
        let zero = Cursor::new(0, 0);
        while let Some((step_mask, cursor, depth)) = self.stack.pop() {
            if step_mask != StepMask::STOP {
                self.steps.truncate(depth - 1);
                self.steps.push(step_mask);
            }
            let branches = self.branches(cursor);
            for (step_mask, cursor) in branches {
                self.stack.push((step_mask, cursor, depth + 1));
            }
            if ((step_mask == StepMask::STOP) && (depth > 0)) || (cursor == zero) {
                let mut steps: Vec<StepMask> = self.steps.clone();
                steps.reverse();
                return Some(Alignment::new(cursor, steps, self.score.clone()));
            }
        }
        None
    }
}
