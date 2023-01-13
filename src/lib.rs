pub mod cigar;
pub mod cost;

use std::cmp::Ordering;

pub use cigar::*;
pub use cost::*;

/// A single base
// NOTE: This is also part of rust-bio-types.
pub type Base = u8;

/// A sequence
// NOTE: This is also part of rust-bio-types.
pub type Sequence = Vec<Base>;

/// A non-owning sequence
pub type Seq<'a> = &'a [Base];

/// A 0-based index into a sequence.
pub type I = i32;

/// A position in a pairwise matching.
///
/// A matching starts at `(0,0)` and ends at `(n, m)`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    derive_more::Add,
    derive_more::Sub,
    derive_more::AddAssign,
    derive_more::SubAssign,
)]
pub struct Pos(pub I, pub I);

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

/// Partial ordering by
/// (a,b) <= (c,d) when a<=c and b<=d.
/// (a,b) < (c,d) when a<=c and b<=d and a<c or b<d.
impl PartialOrd for Pos {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.0.cmp(&other.0);
        let b = self.1.cmp(&other.1);
        if a == b {
            return Some(a);
        }
        if a == Ordering::Equal {
            return Some(b);
        }
        if b == Ordering::Equal {
            return Some(a);
        }
        None
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 <= other.1
    }
}

/// The path corresponding to an alignment of two sequences.
pub type Path = Vec<Pos>;

impl Pos {
    /// The start of an alignment.
    pub fn start() -> Self {
        Pos(0, 0)
    }

    /// The target of an alignment.
    pub fn target(a: Seq, b: Seq) -> Self {
        Pos(a.len() as I, b.len() as I)
    }

    /// The diagonal of position `(i, j)` is `i-j`.
    pub fn diag(&self) -> I {
        self.0 - self.1
    }

    /// The anti diagonal of position `(i, j)` is `i+j`.
    pub fn anti_diag(&self) -> I {
        self.0 + self.1
    }

    /// Mirror this position: `(i, j) -> (j, i)`.
    pub fn mirror(&self) -> Pos {
        Pos(self.1, self.0)
    }

    /// Create a position from differently typed positions.
    pub fn from<T>(i: T, j: T) -> Self
    where
        T: TryInto<I>,
        <T as TryInto<i32>>::Error: std::fmt::Debug,
    {
        Pos(i.try_into().unwrap(), j.try_into().unwrap())
    }
}
