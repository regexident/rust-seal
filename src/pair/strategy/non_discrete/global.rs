use std::ops::RangeInclusive;

use num_traits::{NumAssign, One, Signed};

use pair::{cursor::Cursor, penalty::Penalty, strategy::Strategy as StrategyTrait};

// Pair-wise sequence alignment algorithm for optimal global alignment.
#[derive(Clone, Debug)]
pub struct Strategy<T> {
    penalty: Penalty<T>,
    window: usize,
    bounds: RangeInclusive<T>,
}

impl<T> Strategy<T> {
    pub fn new(penalty: Penalty<T>, window: usize, bounds: RangeInclusive<T>) -> Self {
        Self {
            penalty,
            window,
            bounds,
        }
    }
}

macro_rules! impl_default {
    ($t:ident) => {
        impl Default for Strategy<$t> {
            fn default() -> Self {
                let penalty = Penalty {
                    r#match: -$t::one(),
                    mismatch: $t::one(),
                    gap: $t::one(),
                };
                let window = usize::max_value();
                let min = std::$t::MIN;
                let max = std::$t::MAX;
                let bounds = min..=max;
                Self::new(penalty, window, bounds)
            }
        }
    };
}

impl_default!(f32);
impl_default!(f64);

impl<T, U> StrategyTrait<U> for Strategy<T>
where
    T: NumAssign + Signed + PartialOrd + Clone,
    U: PartialEq,
{
    type Score = T;

    fn penalty(&self) -> &Penalty<Self::Score> {
        &self.penalty
    }

    fn window(&self) -> usize {
        self.window
    }

    fn bounds(&self) -> &RangeInclusive<Self::Score> {
        &self.bounds
    }

    fn boundary_score(&self, prev_score: Self::Score) -> Self::Score {
        prev_score + self.penalty.gap.clone()
    }

    fn pick_optimum(
        &self,
        lhs: (Self::Score, Cursor),
        rhs: (Self::Score, Cursor),
    ) -> (Self::Score, Cursor) {
        // Pick whatever cursor is closer to bottom-right corner of matrix:
        if (rhs.1.x >= lhs.1.x) && (rhs.1.y >= lhs.1.y) {
            rhs
        } else {
            lhs
        }
    }
}
