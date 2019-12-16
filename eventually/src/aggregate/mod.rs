pub mod optional;
pub mod referential;
pub mod util;
pub mod versioned;

pub use util::AggregateExt;

/// State type of the Aggregate specified.
pub type StateOf<A: Aggregate> = A::State;

/// Event type of the Aggregate specified.
pub type EventOf<A: Aggregate> = A::Event;

/// Error type of the Aggregate specified.
pub type ErrorOf<A: Aggregate> = A::Error;

/// An Aggregate is an entity which State is composed of one or more
/// Value-Objects, Entities or Aggregates.
///
/// State mutations are expressed through clear Domain Events which, if
/// applied in the same order as they happened chronologically, will yield
/// the same Aggregate State.
pub trait Aggregate {
    /// State of the Aggregate.
    ///
    /// Usually this associate type is either `Self`, `Option<Self>` or
    /// `Option<T>`, depending on whether the Aggregate state is defined
    /// in a separate data structure or using the same structure that
    /// implements this trait.
    type State;

    /// Domain events that express mutations of the Aggregate State.
    ///
    /// An `enum` containing all the possible domain events is the
    /// usual reccomendation.
    type Event;

    /// Error type returned in `apply` when mutating the Aggregate State
    /// to the next version fails.
    ///
    /// Usually, this error is a validation error type raised when the
    /// domain event that is being applied is invalid, based on the current State.
    ///
    /// Consider using `std::convert::Infallible` (or `!` type if using nightly)
    /// if the `apply` method doesn't fail.
    type Error;

    /// Applies the changes described by the domain event in `Self::Event`
    /// to the current `state` of the `Aggregate`.
    fn apply(state: Self::State, event: Self::Event) -> Result<Self::State, Self::Error>;
}
