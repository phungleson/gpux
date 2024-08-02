use gpui::Styled;

/// Extends [`gpui::Styled`] with specific methods.
pub trait StackExt: Styled + Sized {
    /// Stacks elements horizontally.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn stack_h(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Stacks elements vertically.
    ///
    /// Sets `flex()`, `flex_col()`
    fn stack_v(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StackExt for E {}
