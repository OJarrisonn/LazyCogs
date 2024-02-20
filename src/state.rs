#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Used to describe the current state of the lazy structure for optimizations.
/// It's higly recommended that every lazy structure has a field with a Cell<MutState> 
/// for help optimizing the cloning of the structure
pub enum MutState {
    /// The structure can be mutated with no side effects
    Mutable,
    /// The structure can't be mutated
    Immutable
}