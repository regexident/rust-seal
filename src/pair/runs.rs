use std::iter::Peekable;

use pair::run::Run;
use pair::step::Step;

use pair::steps::Steps;

pub struct Runs<'a> {
    inner: Peekable<Steps<'a>>,
}

impl<'a> Runs<'a> {
    pub fn new(inner: Peekable<Steps<'a>>) -> Runs {
        Runs { inner }
    }
}

impl<'a> Iterator for Runs<'a> {
    type Item = Run;

    fn next(&mut self) -> Option<Run> {
        let inner = &mut self.inner;
        let run = inner.peek().map(|step| step.to_run());
        run.map(|mut run| {
            let mask = run.mask();
            let mut peekable = inner.peekable();
            let mut cautious_take_while = || {
                let is_match = match peekable.peek() {
                    Some(step) => step.mask() == mask,
                    None => false,
                };
                if is_match {
                    peekable.next()
                } else {
                    None
                }
            };
            while let Some(step) = cautious_take_while() {
                run = match (run, step) {
                    (
                        Run::Align { x: run_x, y: run_y },
                        Step::Align {
                            x: step_x,
                            y: step_y,
                        },
                    ) => Run::Align {
                        x: (run_x.start)..(step_x + 1),
                        y: (run_y.start)..(step_y + 1),
                    },
                    (Run::Delete { x: run_x }, Step::Delete { x: step_x }) => Run::Delete {
                        x: (run_x.start)..(step_x + 1),
                    },
                    (Run::Insert { y: run_y }, Step::Insert { y: step_y }) => Run::Insert {
                        y: (run_y.start)..(step_y + 1),
                    },
                    _ => unreachable!(),
                }
            }
            run
        })
    }
}
