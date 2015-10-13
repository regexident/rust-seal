use pair::{alignment_matrix::AlignmentMatrix, alignment_set::AlignmentSet};

pub mod discrete;
pub mod non_discrete;

pub trait Strategy<T> {
        type Score: PartialOrd;

        fn alignment_set<M, E>(&self, x: &[T], y: &[T]) -> Result<AlignmentSet<Self::Score, M>, E>
        where
                M: AlignmentMatrix<Score = Self::Score, Error = E>,
                M: ::std::fmt::Debug;
}
