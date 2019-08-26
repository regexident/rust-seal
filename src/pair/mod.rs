pub mod cursor;
pub mod penalty;
pub mod run;
pub mod step;
pub mod step_mask;
pub mod strategy;

pub mod alignments;
pub mod runs;
pub mod steps;

pub mod matrix;

pub use self::matrix::{Matrix, MatrixCell};
pub use self::cursor::Cursor;
pub use self::run::Run;
pub use self::step::Step;
pub use self::step_mask::StepMask;
pub use self::strategy::Strategy;

pub use self::alignments::{Alignment, AlignmentScope, Alignments, Iter};
pub use self::runs::Runs;
pub use self::steps::Steps;
