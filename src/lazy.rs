/// Trait for creating data that can be lazily cloned.
/// 
/// This provides both an interface for lazy cloning when the data is known to not be mutated. 
/// And an eager clone for data that is likely to be mutated.
pub trait LazyClone {
    /// The O(1) lazy-clone method. 
    /// Useful for cloning data that doesn't necessarily need to be mutated.
    fn lazy(&self) -> Self;

    /// A non-lazy cloning method. Useful for cloning data that is known to modified
    fn eager(&self) -> Self;

    /// Checks if the structure can be mutated with no side effects
    fn is_mutable(&self) -> bool;
}