use pair::{matrix::Matrix, alignments::Alignment, cursor::Cursor, step_mask::StepMask};

pub struct Iter<'a, T> where T: 'a {
    matrix: &'a Matrix<T>,
    stack: Vec<(Option<StepMask>, Cursor)>,
    steps: Vec<StepMask>,
    score: T,
}

impl<'a, T> Iter<'a, T> where T: 'a + Copy {
    pub fn new(matrix: &'a Matrix<T>, cursor: Cursor, score: T) -> Self {
        let stack = vec![(None, cursor)];
        let steps = vec![];
        Iter {
            matrix,
            stack,
            steps,
            score,
        }
    }

    fn branches(&self, cursor: Cursor) -> Vec<(StepMask, Cursor)> {
        let row = self.matrix.row(cursor.y);
        let steps = row[cursor.x].steps();
        if steps == StepMask::STOP {
            return vec![];
        }
        let masks = [StepMask::ALIGN, StepMask::INSERT, StepMask::DELETE];
        masks
            .iter()
            .cloned()
            .filter(|mask| steps.contains(*mask))
            .filter_map(|mask| cursor.backwards_step(mask).map(|cursor| (mask, cursor)))
            .collect()
    }
}

impl<'a, T> Iterator for Iter<'a, T> where T: 'a + Copy {
    type Item = Alignment<T>;

    fn next(&mut self) -> Option<Alignment<T>> {
        let mut start_cursor: Option<Cursor> = None;

        while let Some((step_mask, cursor)) = self.stack.pop() {
            if let Some(step_mask) = step_mask {
                if step_mask == StepMask::STOP {
                    break;
                }
                start_cursor = Some(cursor);
                self.steps.push(step_mask);
            }

            let branches = self.branches(cursor);
            if branches.is_empty() {
                break;
            }
            for (step_mask, cursor) in branches {
                self.stack.push((Some(step_mask), cursor));
            }
        }

        start_cursor.map(|cursor| {
            let mut steps = self.steps.to_owned();
            steps.reverse();
            Alignment::new(cursor, steps, self.score)
        })
    }
}
