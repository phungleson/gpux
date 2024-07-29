/// A trait for elements that can be selected.
///
/// Generally used to enable "toggle" or "active" behavior and styles on an element through the [`Selection`] status.
pub trait Selectable {
    /// Sets whether the element is selected.
    fn selected(self, selected: bool) -> Self;
}
