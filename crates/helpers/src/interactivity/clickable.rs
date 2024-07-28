use gpui::{ClickEvent, WindowContext};

/// A trait for objects that can be clicked.
pub trait Clickable {
    /// Assigns a click event handler to the object and returns a new object.
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self;
}
