use gpui::{div, ElementId, IntoElement, RenderOnce, WindowContext};

#[derive(IntoElement)]
pub struct Twitter {
    pub id: ElementId,
}

impl Twitter {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for Twitter {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
    }
}
