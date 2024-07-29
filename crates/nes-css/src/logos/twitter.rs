use gpui::{
    div, rgb, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Styled, WindowContext,
};
use gpux_css::box_shadow::box_shadow;
use smallvec::smallvec;

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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let color = rgb(0x2c9ceb);
        let white = rgb(0xffffff);

        div()
            .size_4()
            // .shadow(smallvec![box_shadow(2.0, 2.0, 0.0, 2.0, rgb(0x00ff00))])
            .child(div().size(Pixels(10.0)).shadow(smallvec![
                // 2px 1px
                // 3px 1px
                // 4px 1px
                // 5px 1px
                // 6px 1px
                // 7px 1px
                // 8px 1px
                // 9px 1px
                // 10px 1px
                // 11px 1px
                // 12px 1px
                // 13px 1px
                // 14px 1px
                // 15px 1px
                box_shadow(2.0, 1.0, 0.0, 0.0, color),
                box_shadow(3.0, 1.0, 0.0, 0.0, color),
                box_shadow(4.0, 1.0, 0.0, 0.0, color),
                box_shadow(5.0, 1.0, 0.0, 0.0, color),
                box_shadow(6.0, 1.0, 0.0, 0.0, color),
                box_shadow(7.0, 1.0, 0.0, 0.0, color),
                box_shadow(8.0, 1.0, 0.0, 0.0, color),
                box_shadow(9.0, 1.0, 0.0, 0.0, color),
                box_shadow(10.0, 1.0, 0.0, 0.0, color),
                box_shadow(11.0, 1.0, 0.0, 0.0, color),
                box_shadow(12.0, 1.0, 0.0, 0.0, color),
                box_shadow(13.0, 1.0, 0.0, 0.0, color),
                box_shadow(14.0, 1.0, 0.0, 0.0, color),
                box_shadow(15.0, 1.0, 0.0, 0.0, color),
                // 1px 2px
                // 2px 2px
                // 3px 2px
                // 4px 2px
                // 5px 2px
                // 6px 2px
                // 7px 2px
                // 8px 2px
                // 9px 2px
                // 10px 2px
                // 11px 2px
                // 12px 2px
                // 13px 2px
                // 14px 2px
                // 15px 2px
                // 16px 2px
                box_shadow(1.0, 2.0, 0.0, 0.0, color),
                box_shadow(2.0, 2.0, 0.0, 0.0, color),
                box_shadow(3.0, 2.0, 0.0, 0.0, color),
                box_shadow(4.0, 2.0, 0.0, 0.0, color),
                box_shadow(5.0, 2.0, 0.0, 0.0, color),
                box_shadow(6.0, 2.0, 0.0, 0.0, color),
                box_shadow(7.0, 2.0, 0.0, 0.0, color),
                box_shadow(8.0, 2.0, 0.0, 0.0, color),
                box_shadow(9.0, 2.0, 0.0, 0.0, color),
                box_shadow(10.0, 2.0, 0.0, 0.0, color),
                box_shadow(11.0, 2.0, 0.0, 0.0, color),
                box_shadow(12.0, 2.0, 0.0, 0.0, color),
                box_shadow(13.0, 2.0, 0.0, 0.0, color),
                box_shadow(14.0, 2.0, 0.0, 0.0, color),
                box_shadow(15.0, 2.0, 0.0, 0.0, color),
                box_shadow(16.0, 2.0, 0.0, 0.0, color),
                // 1px 3px
                // 2px 3px #fff
                // 3px 3px
                // 4px 3px
                // 5px 3px
                // 6px 3px
                // 7px 3px
                // 8px 3px
                // 9px 3px
                // 10px 3px #fff
                // 11px 3px #fff
                // 12px 3px #fff
                // 13px 3px
                // 14px 3px
                // 15px 3px
                // 16px 3px
                box_shadow(1.0, 3.0, 0.0, 0.0, color),
                box_shadow(2.0, 3.0, 0.0, 0.0, white),
                box_shadow(3.0, 3.0, 0.0, 0.0, color),
                box_shadow(4.0, 3.0, 0.0, 0.0, color),
                box_shadow(5.0, 3.0, 0.0, 0.0, color),
                box_shadow(6.0, 3.0, 0.0, 0.0, color),
                box_shadow(7.0, 3.0, 0.0, 0.0, color),
                box_shadow(8.0, 3.0, 0.0, 0.0, color),
                box_shadow(9.0, 3.0, 0.0, 0.0, color),
                box_shadow(10.0, 3.0, 0.0, 0.0, white),
                box_shadow(11.0, 3.0, 0.0, 0.0, white),
                box_shadow(12.0, 3.0, 0.0, 0.0, white),
                box_shadow(13.0, 3.0, 0.0, 0.0, color),
                box_shadow(14.0, 3.0, 0.0, 0.0, color),
                box_shadow(15.0, 3.0, 0.0, 0.0, color),
                box_shadow(16.0, 3.0, 0.0, 0.0, color),
                // 1px 4px
                // 2px 4px #fff
                // 3px 4px #fff
                // 4px 4px #fff
                // 5px 4px
                // 6px 4px
                // 7px 4px
                // 8px 4px
                // 9px 4px #fff
                // 10px 4px #fff
                // 11px 4px #fff
                // 12px 4px #fff
                // 13px 4px #fff
                // 14px 4px
                // 15px 4px
                // 16px 4px
                box_shadow(1.0, 4.0, 0.0, 0.0, color),
                box_shadow(2.0, 4.0, 0.0, 0.0, white),
                box_shadow(3.0, 4.0, 0.0, 0.0, white),
                box_shadow(4.0, 4.0, 0.0, 0.0, white),
                box_shadow(5.0, 4.0, 0.0, 0.0, color),
                box_shadow(6.0, 4.0, 0.0, 0.0, color),
                box_shadow(7.0, 4.0, 0.0, 0.0, color),
                box_shadow(8.0, 4.0, 0.0, 0.0, color),
                box_shadow(9.0, 4.0, 0.0, 0.0, white),
                box_shadow(10.0, 4.0, 0.0, 0.0, white),
                box_shadow(11.0, 4.0, 0.0, 0.0, white),
                box_shadow(12.0, 4.0, 0.0, 0.0, white),
                box_shadow(13.0, 4.0, 0.0, 0.0, white),
                box_shadow(14.0, 4.0, 0.0, 0.0, color),
                box_shadow(15.0, 4.0, 0.0, 0.0, color),
                box_shadow(16.0, 4.0, 0.0, 0.0, color),
                // 1px 5px
                // 2px 5px #fff
                // 3px 5px #fff
                // 4px 5px #fff
                // 5px 5px #fff
                // 6px 5px #fff
                // 7px 5px
                // 8px 5px #fff
                // 9px 5px #fff
                // 10px 5px #fff
                // 11px 5px #fff
                // 12px 5px #fff
                // 13px 5px #fff
                // 14px 5px #fff
                // 15px 5px #fff
                // 16px 5px
                box_shadow(1.0, 5.0, 0.0, 0.0, color),
                box_shadow(2.0, 5.0, 0.0, 0.0, white),
                box_shadow(3.0, 5.0, 0.0, 0.0, white),
                box_shadow(4.0, 5.0, 0.0, 0.0, white),
                box_shadow(5.0, 5.0, 0.0, 0.0, white),
                box_shadow(6.0, 5.0, 0.0, 0.0, white),
                box_shadow(7.0, 5.0, 0.0, 0.0, color),
                box_shadow(8.0, 5.0, 0.0, 0.0, white),
                box_shadow(9.0, 5.0, 0.0, 0.0, white),
                box_shadow(10.0, 5.0, 0.0, 0.0, white),
                box_shadow(11.0, 5.0, 0.0, 0.0, white),
                box_shadow(12.0, 5.0, 0.0, 0.0, white),
                box_shadow(13.0, 5.0, 0.0, 0.0, white),
                box_shadow(14.0, 5.0, 0.0, 0.0, white),
                box_shadow(15.0, 5.0, 0.0, 0.0, white),
                box_shadow(16.0, 5.0, 0.0, 0.0, color),
                // 1px 6px
                // 2px 6px
                // 3px 6px #fff
                // 4px 6px #fff
                // 5px 6px #fff
                // 6px 6px #fff
                // 7px 6px #fff
                // 8px 6px #fff
                // 9px 6px #fff
                // 10px 6px #fff
                // 11px 6px #fff
                // 12px 6px #fff
                // 13px 6px #fff
                // 14px 6px #fff
                // 15px 6px
                // 16px 6px
                box_shadow(1.0, 6.0, 0.0, 0.0, color),
                box_shadow(2.0, 6.0, 0.0, 0.0, color),
                box_shadow(3.0, 6.0, 0.0, 0.0, white),
                box_shadow(4.0, 6.0, 0.0, 0.0, white),
                box_shadow(5.0, 6.0, 0.0, 0.0, white),
                box_shadow(6.0, 6.0, 0.0, 0.0, white),
                box_shadow(7.0, 6.0, 0.0, 0.0, white),
                box_shadow(8.0, 6.0, 0.0, 0.0, white),
                box_shadow(9.0, 6.0, 0.0, 0.0, white),
                box_shadow(10.0, 6.0, 0.0, 0.0, white),
                box_shadow(11.0, 6.0, 0.0, 0.0, white),
                box_shadow(12.0, 6.0, 0.0, 0.0, white),
                box_shadow(13.0, 6.0, 0.0, 0.0, white),
                box_shadow(14.0, 6.0, 0.0, 0.0, white),
                box_shadow(15.0, 6.0, 0.0, 0.0, color),
                box_shadow(16.0, 6.0, 0.0, 0.0, color),
                // 1px 7px
                // 2px 7px
                // 3px 7px #fff
                // 4px 7px #fff
                // 5px 7px #fff
                // 6px 7px #fff
                // 7px 7px #fff
                // 8px 7px #fff
                // 9px 7px #fff
                // 10px 7px #fff
                // 11px 7px #fff
                // 12px 7px #fff
                // 13px 7px #fff
                // 14px 7px #fff
                // 15px 7px
                // 16px 7px
                box_shadow(1.0, 7.0, 0.0, 0.0, color),
                box_shadow(2.0, 7.0, 0.0, 0.0, color),
                box_shadow(3.0, 7.0, 0.0, 0.0, white),
                box_shadow(4.0, 7.0, 0.0, 0.0, white),
                box_shadow(5.0, 7.0, 0.0, 0.0, white),
                box_shadow(6.0, 7.0, 0.0, 0.0, white),
                box_shadow(7.0, 7.0, 0.0, 0.0, white),
                box_shadow(8.0, 7.0, 0.0, 0.0, white),
                box_shadow(9.0, 7.0, 0.0, 0.0, white),
                box_shadow(10.0, 7.0, 0.0, 0.0, white),
                box_shadow(11.0, 7.0, 0.0, 0.0, white),
                box_shadow(12.0, 7.0, 0.0, 0.0, white),
                box_shadow(13.0, 7.0, 0.0, 0.0, white),
                box_shadow(14.0, 7.0, 0.0, 0.0, white),
                box_shadow(15.0, 7.0, 0.0, 0.0, color),
                box_shadow(16.0, 7.0, 0.0, 0.0, color),
                // 1px 8px
                // 2px 8px
                // 3px 8px
                // 4px 8px #fff
                // 5px 8px #fff
                // 6px 8px #fff
                // 7px 8px #fff
                // 8px 8px #fff
                // 9px 8px #fff
                // 10px 8px #fff
                // 11px 8px #fff
                // 12px 8px #fff
                // 13px 8px #fff
                // 14px 8px
                // 15px 8px
                // 16px 8px
                box_shadow(1.0, 8.0, 0.0, 0.0, color),
                box_shadow(2.0, 8.0, 0.0, 0.0, color),
                box_shadow(3.0, 8.0, 0.0, 0.0, color),
                box_shadow(4.0, 8.0, 0.0, 0.0, white),
                box_shadow(5.0, 8.0, 0.0, 0.0, white),
                box_shadow(6.0, 8.0, 0.0, 0.0, white),
                box_shadow(7.0, 8.0, 0.0, 0.0, white),
                box_shadow(8.0, 8.0, 0.0, 0.0, white),
                box_shadow(9.0, 8.0, 0.0, 0.0, white),
                box_shadow(10.0, 8.0, 0.0, 0.0, white),
                box_shadow(11.0, 8.0, 0.0, 0.0, white),
                box_shadow(12.0, 8.0, 0.0, 0.0, white),
                box_shadow(13.0, 8.0, 0.0, 0.0, white),
                box_shadow(14.0, 8.0, 0.0, 0.0, color),
                box_shadow(15.0, 8.0, 0.0, 0.0, color),
                box_shadow(16.0, 8.0, 0.0, 0.0, color),
                // 1px 9px
                // 2px 9px
                // 3px 9px
                // 4px 9px #fff
                // 5px 9px #fff
                // 6px 9px #fff
                // 7px 9px #fff
                // 8px 9px #fff
                // 9px 9px #fff
                // 10px 9px #fff
                // 11px 9px #fff
                // 12px 9px #fff
                // 13px 9px
                // 14px 9px
                // 15px 9px
                // 16px 9px
                box_shadow(1.0, 9.0, 0.0, 0.0, color),
                box_shadow(2.0, 9.0, 0.0, 0.0, color),
                box_shadow(3.0, 9.0, 0.0, 0.0, color),
                box_shadow(4.0, 9.0, 0.0, 0.0, white),
                box_shadow(5.0, 9.0, 0.0, 0.0, white),
                box_shadow(6.0, 9.0, 0.0, 0.0, white),
                box_shadow(7.0, 9.0, 0.0, 0.0, white),
                box_shadow(8.0, 9.0, 0.0, 0.0, white),
                box_shadow(9.0, 9.0, 0.0, 0.0, white),
                box_shadow(10.0, 9.0, 0.0, 0.0, white),
                box_shadow(11.0, 9.0, 0.0, 0.0, white),
                box_shadow(12.0, 9.0, 0.0, 0.0, white),
                box_shadow(13.0, 9.0, 0.0, 0.0, color),
                box_shadow(14.0, 9.0, 0.0, 0.0, color),
                box_shadow(15.0, 9.0, 0.0, 0.0, color),
                box_shadow(16.0, 9.0, 0.0, 0.0, color),
                // 1px 10px
                // 2px 10px
                // 3px 10px
                // 4px 10px
                // 5px 10px #fff
                // 6px 10px #fff
                // 7px 10px #fff
                // 8px 10px #fff
                // 9px 10px #fff
                // 10px 10px #fff
                // 11px 10px #fff
                // 12px 10px #fff
                // 13px 10px
                // 14px 10px
                // 15px 10px
                // 16px 10px
                box_shadow(1.0, 10.0, 0.0, 0.0, color),
                box_shadow(2.0, 10.0, 0.0, 0.0, color),
                box_shadow(3.0, 10.0, 0.0, 0.0, color),
                box_shadow(4.0, 10.0, 0.0, 0.0, color),
                box_shadow(5.0, 10.0, 0.0, 0.0, white),
                box_shadow(6.0, 10.0, 0.0, 0.0, white),
                box_shadow(7.0, 10.0, 0.0, 0.0, white),
                box_shadow(8.0, 10.0, 0.0, 0.0, white),
                box_shadow(9.0, 10.0, 0.0, 0.0, white),
                box_shadow(10.0, 10.0, 0.0, 0.0, white),
                box_shadow(11.0, 10.0, 0.0, 0.0, white),
                box_shadow(12.0, 10.0, 0.0, 0.0, white),
                box_shadow(13.0, 10.0, 0.0, 0.0, color),
                box_shadow(14.0, 10.0, 0.0, 0.0, color),
                box_shadow(15.0, 10.0, 0.0, 0.0, color),
                box_shadow(16.0, 10.0, 0.0, 0.0, color),
                // 1px 11px
                // 2px 11px
                // 3px 11px
                // 4px 11px
                // 5px 11px
                // 6px 11px #fff
                // 7px 11px #fff
                // 8px 11px #fff
                // 9px 11px #fff
                // 10px 11px #fff
                // 11px 11px #fff
                // 12px 11px #fff
                // 13px 11px
                // 14px 11px
                // 15px 11px
                // 16px 11px
                box_shadow(1.0, 11.0, 0.0, 0.0, color),
                box_shadow(2.0, 11.0, 0.0, 0.0, color),
                box_shadow(3.0, 11.0, 0.0, 0.0, color),
                box_shadow(4.0, 11.0, 0.0, 0.0, color),
                box_shadow(5.0, 11.0, 0.0, 0.0, color),
                box_shadow(6.0, 11.0, 0.0, 0.0, white),
                box_shadow(7.0, 11.0, 0.0, 0.0, white),
                box_shadow(8.0, 11.0, 0.0, 0.0, white),
                box_shadow(9.0, 11.0, 0.0, 0.0, white),
                box_shadow(10.0, 11.0, 0.0, 0.0, white),
                box_shadow(11.0, 11.0, 0.0, 0.0, white),
                box_shadow(12.0, 11.0, 0.0, 0.0, white),
                box_shadow(13.0, 11.0, 0.0, 0.0, color),
                box_shadow(14.0, 11.0, 0.0, 0.0, color),
                box_shadow(15.0, 11.0, 0.0, 0.0, color),
                box_shadow(16.0, 11.0, 0.0, 0.0, color),
                // 1px 12px
                // 2px 12px
                // 3px 12px
                // 4px 12px
                // 5px 12px #fff
                // 6px 12px #fff
                // 7px 12px #fff
                // 8px 12px #fff
                // 9px 12px #fff
                // 10px 12px #fff
                // 11px 12px #fff
                // 12px 12px
                // 13px 12px
                // 14px 12px
                // 15px 12px
                // 16px 12px
                box_shadow(1.0, 12.0, 0.0, 0.0, color),
                box_shadow(2.0, 12.0, 0.0, 0.0, color),
                box_shadow(3.0, 12.0, 0.0, 0.0, color),
                box_shadow(4.0, 12.0, 0.0, 0.0, color),
                box_shadow(5.0, 12.0, 0.0, 0.0, white),
                box_shadow(6.0, 12.0, 0.0, 0.0, white),
                box_shadow(7.0, 12.0, 0.0, 0.0, white),
                box_shadow(8.0, 12.0, 0.0, 0.0, white),
                box_shadow(9.0, 12.0, 0.0, 0.0, white),
                box_shadow(10.0, 12.0, 0.0, 0.0, white),
                box_shadow(11.0, 12.0, 0.0, 0.0, white),
                box_shadow(12.0, 12.0, 0.0, 0.0, color),
                box_shadow(13.0, 12.0, 0.0, 0.0, color),
                box_shadow(14.0, 12.0, 0.0, 0.0, color),
                box_shadow(15.0, 12.0, 0.0, 0.0, color),
                box_shadow(16.0, 12.0, 0.0, 0.0, color),
                // 1px 13px
                // 2px 13px
                // 3px 13px #fff
                // 4px 13px #fff
                // 5px 13px #fff
                // 6px 13px #fff
                // 7px 13px #fff
                // 8px 13px #fff
                // 9px 13px #fff
                // 10px 13px
                // 11px 13px
                // 12px 13px
                // 13px 13px
                // 14px 13px
                // 15px 13px
                // 16px 13px
                box_shadow(1.0, 13.0, 0.0, 0.0, color),
                box_shadow(2.0, 13.0, 0.0, 0.0, color),
                box_shadow(3.0, 13.0, 0.0, 0.0, white),
                box_shadow(4.0, 13.0, 0.0, 0.0, white),
                box_shadow(5.0, 13.0, 0.0, 0.0, white),
                box_shadow(6.0, 13.0, 0.0, 0.0, white),
                box_shadow(7.0, 13.0, 0.0, 0.0, white),
                box_shadow(8.0, 13.0, 0.0, 0.0, white),
                box_shadow(9.0, 13.0, 0.0, 0.0, white),
                box_shadow(10.0, 13.0, 0.0, 0.0, color),
                box_shadow(11.0, 13.0, 0.0, 0.0, color),
                box_shadow(12.0, 13.0, 0.0, 0.0, color),
                box_shadow(13.0, 13.0, 0.0, 0.0, color),
                box_shadow(14.0, 13.0, 0.0, 0.0, color),
                box_shadow(15.0, 13.0, 0.0, 0.0, color),
                box_shadow(16.0, 13.0, 0.0, 0.0, color),
                // 1px 14px
                // 2px 14px
                // 3px 14px
                // 4px 14px #fff
                // 5px 14px #fff
                // 6px 14px #fff
                // 7px 14px
                // 8px 14px
                // 9px 14px
                // 10px 14px
                // 11px 14px
                // 12px 14px
                // 13px 14px
                // 14px 14px
                // 15px 14px
                // 16px 14px
                box_shadow(1.0, 14.0, 0.0, 0.0, color),
                box_shadow(2.0, 14.0, 0.0, 0.0, color),
                box_shadow(3.0, 14.0, 0.0, 0.0, color),
                box_shadow(4.0, 14.0, 0.0, 0.0, white),
                box_shadow(5.0, 14.0, 0.0, 0.0, white),
                box_shadow(6.0, 14.0, 0.0, 0.0, white),
                box_shadow(7.0, 14.0, 0.0, 0.0, color),
                box_shadow(8.0, 14.0, 0.0, 0.0, color),
                box_shadow(9.0, 14.0, 0.0, 0.0, color),
                box_shadow(10.0, 14.0, 0.0, 0.0, color),
                box_shadow(11.0, 14.0, 0.0, 0.0, color),
                box_shadow(12.0, 14.0, 0.0, 0.0, color),
                box_shadow(13.0, 14.0, 0.0, 0.0, color),
                box_shadow(14.0, 14.0, 0.0, 0.0, color),
                box_shadow(15.0, 14.0, 0.0, 0.0, color),
                box_shadow(16.0, 14.0, 0.0, 0.0, color),
                // 1px 15px
                // 2px 15px
                // 3px 15px
                // 4px 15px
                // 5px 15px
                // 6px 15px
                // 7px 15px
                // 8px 15px
                // 9px 15px
                // 10px 15px
                // 11px 15px
                // 12px 15px
                // 13px 15px
                // 14px 15px
                // 15px 15px
                // 16px 15px
                box_shadow(1.0, 15.0, 0.0, 0.0, color),
                box_shadow(2.0, 15.0, 0.0, 0.0, color),
                box_shadow(3.0, 15.0, 0.0, 0.0, color),
                box_shadow(4.0, 15.0, 0.0, 0.0, color),
                box_shadow(5.0, 15.0, 0.0, 0.0, color),
                box_shadow(6.0, 15.0, 0.0, 0.0, color),
                box_shadow(7.0, 15.0, 0.0, 0.0, color),
                box_shadow(8.0, 15.0, 0.0, 0.0, color),
                box_shadow(9.0, 15.0, 0.0, 0.0, color),
                box_shadow(10.0, 15.0, 0.0, 0.0, color),
                box_shadow(11.0, 15.0, 0.0, 0.0, color),
                box_shadow(12.0, 15.0, 0.0, 0.0, color),
                box_shadow(13.0, 15.0, 0.0, 0.0, color),
                box_shadow(14.0, 15.0, 0.0, 0.0, color),
                box_shadow(15.0, 15.0, 0.0, 0.0, color),
                box_shadow(16.0, 15.0, 0.0, 0.0, color),
                // 2px 16px
                // 3px 16px
                // 4px 16px
                // 5px 16px
                // 6px 16px
                // 7px 16px
                // 8px 16px
                // 9px 16px
                // 10px 16px
                // 11px 16px
                // 12px 16px
                // 13px 16px
                // 14px 16px
                // 15px 16px;
                box_shadow(2.0, 15.0, 0.0, 0.0, color),
                box_shadow(3.0, 15.0, 0.0, 0.0, color),
                box_shadow(4.0, 15.0, 0.0, 0.0, color),
                box_shadow(5.0, 15.0, 0.0, 0.0, color),
                box_shadow(6.0, 15.0, 0.0, 0.0, color),
                box_shadow(7.0, 15.0, 0.0, 0.0, color),
                box_shadow(8.0, 15.0, 0.0, 0.0, color),
                box_shadow(9.0, 15.0, 0.0, 0.0, color),
                box_shadow(10.0, 15.0, 0.0, 0.0, color),
                box_shadow(11.0, 15.0, 0.0, 0.0, color),
                box_shadow(12.0, 15.0, 0.0, 0.0, color),
                box_shadow(13.0, 15.0, 0.0, 0.0, color),
                box_shadow(14.0, 15.0, 0.0, 0.0, color),
                box_shadow(15.0, 15.0, 0.0, 0.0, color),
            ]))
    }
}
