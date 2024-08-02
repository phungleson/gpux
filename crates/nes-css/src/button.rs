use gpui::{div, ElementId, IntoElement, ParentElement, px, RenderOnce, rgb, Styled, WindowContext};
use smallvec::smallvec;

use gpux_css::box_shadow::box_shadow;

#[derive(IntoElement)]
pub struct Button {
    pub id: ElementId,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().relative()
            .child("Normal")
            .child(
                div()
                    .absolute()
                    .top(px(-4.))
                    .left(px(-4.))
                    .right(px(-4.))
                    .bottom(px(-4.))
                    .shadow(smallvec![box_shadow(-4., -4., 0., 0., rgb(0xadafbc), true)]),
            )
    }
}
