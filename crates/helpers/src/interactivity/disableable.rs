/// A trait for objects that can be enabled or disabled.
pub trait Disableable {
    /// Returns a new object with the specified disabled status.
    fn disabled(self, disabled: bool) -> Self;
}
