use gpui::{div, ElementId, IntoElement, RenderOnce, WindowContext};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
    }
}