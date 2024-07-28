use gpui::Styled;

/// Extends [`gpui::Styled`] with specific flex methods.
pub trait StackExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn stack_h(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn stack_v(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StackExt for E {}
