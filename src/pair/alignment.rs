use pair::cursor::Cursor;
use pair::runs::Runs;
use pair::step_mask::StepMask;
use pair::steps::Steps;

#[derive(Debug)]
pub struct Alignment {
    origin: Cursor,
    steps: Vec<StepMask>,
    score: isize,
}

impl Alignment {
    pub fn new(origin: Cursor, steps: Vec<StepMask>, score: isize) -> Alignment {
        Alignment {
            origin,
            steps,
            score,
        }
    }

    pub fn origin(&self) -> &Cursor {
        &self.origin
    }

    pub fn score(&self) -> isize {
        self.score
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn steps(&self) -> Steps {
        Steps::new(self.steps.iter(), self.origin)
    }

    pub fn runs(&self) -> Runs<'_> {
        Runs::new(self.steps().peekable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pair::cursor::Cursor;
    use pair::step_mask::StepMask;

    fn origin() -> Cursor {
        Cursor { x: 3, y: 3 }
    }

    fn steps() -> Vec<StepMask> {
        vec![StepMask::ALIGN, StepMask::ALIGN, StepMask::ALIGN]
    }

    fn score() -> isize {
        42
    }

    fn alignment() -> Alignment {
        Alignment::new(origin(), steps(), score())
    }

    #[test]
    fn origin_works() {
        assert_eq!(alignment().origin(), &origin());
    }

    #[test]
    fn steps_works() {
        for (subject, expected) in alignment().steps().zip(steps()) {
            assert_eq!(subject.mask(), expected);
        }
    }

    #[test]
    fn score_works() {
        assert_eq!(alignment().score(), score());
    }
}
