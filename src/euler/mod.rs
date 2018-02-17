//! Utilities for solving project euler's problems.

/// The context in which a problem is solved.
pub struct EulerContext {
    name: String,
}
impl EulerContext {
    #[inline]
    pub fn new(name: String) -> EulerContext {
        EulerContext { name }
    }
    /// The name of the problem we're solving
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}
