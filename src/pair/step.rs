use pair::run::Run;
use pair::step_mask::StepMask;

#[derive(Clone, PartialEq)]
pub enum Step {
    Align { x: usize, y: usize },
    Delete { x: usize },
    Insert { y: usize },
}

impl Step {
    pub fn mask(&self) -> StepMask {
        match self {
            Step::Align { .. } => StepMask::ALIGN,
            Step::Delete { .. } => StepMask::DELETE,
            Step::Insert { .. } => StepMask::INSERT,
        }
    }

    pub fn to_run(&self) -> Run {
        match *self {
            Step::Align { x, y } => Run::Align {
                x: (x)..(x + 1),
                y: (y)..(y + 1),
            },
            Step::Delete { x } => Run::Delete { x: (x)..(x + 1) },
            Step::Insert { y } => Run::Insert { y: (y)..(y + 1) },
        }
    }
}
