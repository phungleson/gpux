use gpui::{
    div, px, rgb, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use gpux_css::box_shadow::box_shadow;
use smallvec::smallvec;

#[derive(IntoElement)]
pub struct Pokeball {
    pub id: ElementId,
}

impl Pokeball {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for Pokeball {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let red = rgb(0xff001d);
        let gray = rgb(0x9fa1a1);
        let black = rgb(0x212529);
        let white = rgb(0xffffff);

        div().size_4().relative().child(
            div()
                .absolute()
                .top(px(-6.))
                .left(px(-6.))
                .size(px(6.))
                .shadow(smallvec![
                    // 36px 6px
                    // 42px 6px
                    // 48px 6px
                    // 54px 6px
                    box_shadow(36.0, 6.0, 0.1, 0.0, black),
                    box_shadow(42.0, 6.0, 0.1, 0.0, black),
                    box_shadow(48.0, 6.0, 0.1, 0.0, black),
                    box_shadow(54.0, 6.0, 0.1, 0.0, black),
                    // 24px 12px
                    // 30px 12px
                    // 36px 12px #ff001d
                    // 42px 12px #ff001d
                    // 48px 12px #ff001d
                    // 54px 12px #ff001d
                    // 60px 12px
                    // 66px 12px
                    box_shadow(24.0, 12.0, 0.1, 0.0, black),
                    box_shadow(30.0, 12.0, 0.1, 0.0, black),
                    box_shadow(36.0, 12.0, 0.1, 0.0, red),
                    box_shadow(42.0, 12.0, 0.1, 0.0, red),
                    box_shadow(48.0, 12.0, 0.1, 0.0, red),
                    box_shadow(54.0, 12.0, 0.1, 0.0, red),
                    box_shadow(60.0, 12.0, 0.1, 0.0, black),
                    box_shadow(66.0, 12.0, 0.1, 0.0, black),
                    // 18px 18px
                    // 24px 18px #fff
                    // 30px 18px #fff
                    // 36px 18px #ff001d
                    // 42px 18px #ff001d
                    // 48px 18px #ff001d
                    // 54px 18px #ff001d
                    // 60px 18px #ff001d
                    // 66px 18px #ff001d
                    // 72px 18px
                    box_shadow(18.0, 18.0, 0.1, 0.0, black),
                    box_shadow(24.0, 18.0, 0.1, 0.0, white),
                    box_shadow(30.0, 18.0, 0.1, 0.0, white),
                    box_shadow(36.0, 18.0, 0.1, 0.0, red),
                    box_shadow(42.0, 18.0, 0.1, 0.0, red),
                    box_shadow(48.0, 18.0, 0.1, 0.0, red),
                    box_shadow(54.0, 18.0, 0.1, 0.0, red),
                    box_shadow(60.0, 18.0, 0.1, 0.0, red),
                    box_shadow(66.0, 18.0, 0.1, 0.0, red),
                    box_shadow(72.0, 18.0, 0.1, 0.0, black),
                    // 12px 24px
                    // 18px 24px #fff
                    // 24px 24px #fff
                    // 30px 24px #ff001d
                    // 36px 24px #ff001d
                    // 42px 24px #ff001d
                    // 48px 24px #ff001d
                    // 54px 24px #ff001d
                    // 60px 24px #ff001d
                    // 66px 24px #ff001d
                    // 72px 24px
                    // 78px 24px
                    box_shadow(12.0, 24.0, 0.1, 0.0, black),
                    box_shadow(18.0, 24.0, 0.1, 0.0, white),
                    box_shadow(24.0, 24.0, 0.1, 0.0, white),
                    box_shadow(30.0, 24.0, 0.1, 0.0, red),
                    box_shadow(36.0, 24.0, 0.1, 0.0, red),
                    box_shadow(42.0, 24.0, 0.1, 0.0, red),
                    box_shadow(48.0, 24.0, 0.1, 0.0, red),
                    box_shadow(54.0, 24.0, 0.1, 0.0, red),
                    box_shadow(60.0, 24.0, 0.1, 0.0, red),
                    box_shadow(66.0, 24.0, 0.1, 0.0, red),
                    box_shadow(72.0, 24.0, 0.1, 0.0, black),
                    box_shadow(78.0, 24.0, 0.1, 0.0, black),
                    // 12px 30px
                    // 18px 30px #fff
                    // 24px 30px #ff001d
                    // 30px 30px #ff001d
                    // 36px 30px #ff001d
                    // 42px 30px #ff001d
                    // 48px 30px #ff001d
                    // 54px 30px #ff001d
                    // 60px 30px #ff001d
                    // 66px 30px #ff001d
                    // 72px 30px #ff001d
                    // 78px 30px
                    box_shadow(12.0, 30.0, 0.1, 0.0, black),
                    box_shadow(18.0, 30.0, 0.1, 0.0, white),
                    box_shadow(24.0, 30.0, 0.1, 0.0, red),
                    box_shadow(30.0, 30.0, 0.1, 0.0, red),
                    box_shadow(36.0, 30.0, 0.1, 0.0, red),
                    box_shadow(42.0, 30.0, 0.1, 0.0, red),
                    box_shadow(48.0, 30.0, 0.1, 0.0, red),
                    box_shadow(54.0, 30.0, 0.1, 0.0, red),
                    box_shadow(60.0, 30.0, 0.1, 0.0, red),
                    box_shadow(66.0, 30.0, 0.1, 0.0, red),
                    box_shadow(72.0, 30.0, 0.1, 0.0, red),
                    box_shadow(78.0, 30.0, 0.1, 0.0, black),
                    // 6px 36px
                    // 12px 36px #fff
                    // 18px 36px #ff001d
                    // 24px 36px #ff001d
                    // 30px 36px #ff001d
                    // 36px 36px #ff001d
                    // 42px 36px #ff001d
                    // 48px 36px #ff001d
                    // 54px 36px #ff001d
                    // 60px 36px #ff001d
                    // 66px 36px #ff001d
                    // 72px 36px #ff001d
                    // 78px 36px #ff001d
                    // 84px 36px
                    box_shadow(6.0, 36.0, 0.1, 0.0, black),
                    box_shadow(12.0, 36.0, 0.1, 0.0, white),
                    box_shadow(18.0, 36.0, 0.1, 0.0, red),
                    box_shadow(24.0, 36.0, 0.1, 0.0, red),
                    box_shadow(30.0, 36.0, 0.1, 0.0, red),
                    box_shadow(36.0, 36.0, 0.1, 0.0, red),
                    box_shadow(42.0, 36.0, 0.1, 0.0, red),
                    box_shadow(48.0, 36.0, 0.1, 0.0, red),
                    box_shadow(54.0, 36.0, 0.1, 0.0, red),
                    box_shadow(60.0, 36.0, 0.1, 0.0, red),
                    box_shadow(66.0, 36.0, 0.1, 0.0, red),
                    box_shadow(72.0, 36.0, 0.1, 0.0, red),
                    box_shadow(78.0, 36.0, 0.1, 0.0, red),
                    box_shadow(84.0, 36.0, 0.1, 0.0, black),
                    // 6px 42px
                    // 12px 42px
                    // 18px 42px
                    // 24px 42px #ff001d
                    // 30px 42px #ff001d
                    // 36px 42px #ff001d
                    // 42px 42px #ff001d
                    // 48px 42px
                    // 54px 42px
                    // 60px 42px
                    // 66px 42px #ff001d
                    // 72px 42px #ff001d
                    // 78px 42px #ff001d
                    // 84px 42px
                    box_shadow(6.0, 42.0, 0.1, 0.0, black),
                    box_shadow(12.0, 42.0, 0.1, 0.0, black),
                    box_shadow(18.0, 42.0, 0.1, 0.0, black),
                    box_shadow(24.0, 42.0, 0.1, 0.0, red),
                    box_shadow(30.0, 42.0, 0.1, 0.0, red),
                    box_shadow(36.0, 42.0, 0.1, 0.0, red),
                    box_shadow(42.0, 42.0, 0.1, 0.0, red),
                    box_shadow(48.0, 42.0, 0.1, 0.0, black),
                    box_shadow(54.0, 42.0, 0.1, 0.0, black),
                    box_shadow(60.0, 42.0, 0.1, 0.0, black),
                    box_shadow(66.0, 42.0, 0.1, 0.0, red),
                    box_shadow(72.0, 42.0, 0.1, 0.0, red),
                    box_shadow(78.0, 42.0, 0.1, 0.0, red),
                    box_shadow(84.0, 42.0, 0.1, 0.0, black),
                    // 6px 48px
                    // 12px 48px
                    // 18px 48px
                    // 24px 48px
                    // 30px 48px #ff001d
                    // 36px 48px #ff001d
                    // 42px 48px
                    // 48px 48px #fff
                    // 54px 48px #fff
                    // 60px 48px #fff
                    // 66px 48px
                    // 72px 48px #ff001d
                    // 78px 48px #ff001d
                    // 84px 48px
                    box_shadow(6.0, 48.0, 0.1, 0.0, black),
                    box_shadow(12.0, 48.0, 0.1, 0.0, black),
                    box_shadow(18.0, 48.0, 0.1, 0.0, black),
                    box_shadow(24.0, 48.0, 0.1, 0.0, black),
                    box_shadow(30.0, 48.0, 0.1, 0.0, red),
                    box_shadow(36.0, 48.0, 0.1, 0.0, red),
                    box_shadow(42.0, 48.0, 0.1, 0.0, black),
                    box_shadow(48.0, 48.0, 0.1, 0.0, white),
                    box_shadow(54.0, 48.0, 0.1, 0.0, white),
                    box_shadow(60.0, 48.0, 0.1, 0.0, white),
                    box_shadow(66.0, 48.0, 0.1, 0.0, black),
                    box_shadow(72.0, 48.0, 0.1, 0.0, red),
                    box_shadow(78.0, 48.0, 0.1, 0.0, red),
                    box_shadow(84.0, 48.0, 0.1, 0.0, black),
                    // 6px 54px
                    // 12px 54px #9fa1a1
                    // 18px 54px #fff
                    // 24px 54px
                    // 30px 54px
                    // 36px 54px
                    // 42px 54px
                    // 48px 54px #fff
                    // 54px 54px #fff
                    // 60px 54px #fff
                    // 66px 54px
                    // 72px 54px
                    // 78px 54px
                    // 84px 54px
                    box_shadow(6.0, 54.0, 0.1, 0.0, black),
                    box_shadow(12.0, 54.0, 0.1, 0.0, gray),
                    box_shadow(18.0, 54.0, 0.1, 0.0, white),
                    box_shadow(24.0, 54.0, 0.1, 0.0, black),
                    box_shadow(30.0, 54.0, 0.1, 0.0, black),
                    box_shadow(36.0, 54.0, 0.1, 0.0, black),
                    box_shadow(42.0, 54.0, 0.1, 0.0, black),
                    box_shadow(48.0, 54.0, 0.1, 0.0, white),
                    box_shadow(54.0, 54.0, 0.1, 0.0, white),
                    box_shadow(60.0, 54.0, 0.1, 0.0, white),
                    box_shadow(66.0, 54.0, 0.1, 0.0, black),
                    box_shadow(72.0, 54.0, 0.1, 0.0, black),
                    box_shadow(78.0, 54.0, 0.1, 0.0, black),
                    box_shadow(84.0, 54.0, 0.1, 0.0, black),
                    // 12px 60px
                    // 18px 60px #fff
                    // 24px 60px #fff
                    // 30px 60px #fff
                    // 36px 60px
                    // 42px 60px
                    // 48px 60px #fff
                    // 54px 60px #fff
                    // 60px 60px #fff
                    // 66px 60px
                    // 72px 60px #fff
                    // 78px 60px
                    box_shadow(12.0, 60.0, 0.1, 0.0, black),
                    box_shadow(18.0, 60.0, 0.1, 0.0, white),
                    box_shadow(24.0, 60.0, 0.1, 0.0, white),
                    box_shadow(30.0, 60.0, 0.1, 0.0, white),
                    box_shadow(36.0, 60.0, 0.1, 0.0, black),
                    box_shadow(42.0, 60.0, 0.1, 0.0, black),
                    box_shadow(48.0, 60.0, 0.1, 0.0, white),
                    box_shadow(54.0, 60.0, 0.1, 0.0, white),
                    box_shadow(60.0, 60.0, 0.1, 0.0, white),
                    box_shadow(66.0, 60.0, 0.1, 0.0, black),
                    box_shadow(72.0, 60.0, 0.1, 0.0, white),
                    box_shadow(78.0, 60.0, 0.1, 0.0, black),
                    // 12px 66px
                    // 18px 66px #fff
                    // 24px 66px #fff
                    // 30px 66px #fff
                    // 36px 66px #fff
                    // 42px 66px #fff
                    // 48px 66px
                    // 54px 66px
                    // 60px 66px
                    // 66px 66px #fff
                    // 72px 66px #fff
                    // 78px 66px
                    box_shadow(12.0, 66.0, 0.1, 0.0, black),
                    box_shadow(18.0, 66.0, 0.1, 0.0, white),
                    box_shadow(24.0, 66.0, 0.1, 0.0, white),
                    box_shadow(30.0, 66.0, 0.1, 0.0, white),
                    box_shadow(36.0, 66.0, 0.1, 0.0, white),
                    box_shadow(42.0, 66.0, 0.1, 0.0, white),
                    box_shadow(48.0, 66.0, 0.1, 0.0, black),
                    box_shadow(54.0, 66.0, 0.1, 0.0, black),
                    box_shadow(60.0, 66.0, 0.1, 0.0, black),
                    box_shadow(66.0, 66.0, 0.1, 0.0, white),
                    box_shadow(72.0, 66.0, 0.1, 0.0, white),
                    box_shadow(78.0, 66.0, 0.1, 0.0, black),
                    // 18px 72px
                    // 24px 72px #9fa1a1
                    // 30px 72px #9fa1a1
                    // 36px 72px #fff
                    // 42px 72px #fff
                    // 48px 72px #fff
                    // 54px 72px #fff
                    // 60px 72px #fff
                    // 66px 72px #fff
                    // 72px 72px
                    box_shadow(18.0, 72.0, 0.1, 0.0, black),
                    box_shadow(24.0, 72.0, 0.1, 0.0, gray),
                    box_shadow(30.0, 72.0, 0.1, 0.0, gray),
                    box_shadow(36.0, 72.0, 0.1, 0.0, white),
                    box_shadow(42.0, 72.0, 0.1, 0.0, white),
                    box_shadow(48.0, 72.0, 0.1, 0.0, white),
                    box_shadow(54.0, 72.0, 0.1, 0.0, white),
                    box_shadow(60.0, 72.0, 0.1, 0.0, white),
                    box_shadow(66.0, 72.0, 0.1, 0.0, white),
                    box_shadow(72.0, 72.0, 0.1, 0.0, black),
                    // 24px 78px
                    // 30px 78px
                    // 36px 78px #9fa1a1
                    // 42px 78px #9fa1a1
                    // 48px 78px #9fa1a1
                    // 54px 78px #9fa1a1
                    // 60px 78px
                    // 66px 78px
                    box_shadow(24.0, 78.0, 0.1, 0.0, black),
                    box_shadow(30.0, 78.0, 0.1, 0.0, black),
                    box_shadow(36.0, 78.0, 0.1, 0.0, gray),
                    box_shadow(42.0, 78.0, 0.1, 0.0, gray),
                    box_shadow(48.0, 78.0, 0.1, 0.0, gray),
                    box_shadow(54.0, 78.0, 0.1, 0.0, gray),
                    box_shadow(60.0, 78.0, 0.1, 0.0, black),
                    box_shadow(66.0, 78.0, 0.1, 0.0, black),
                    // 36px 84px
                    // 42px 84px
                    // 48px 84px
                    // 54px 84px
                    box_shadow(36.0, 84.0, 0.1, 0.0, black),
                    box_shadow(42.0, 84.0, 0.1, 0.0, black),
                    box_shadow(48.0, 84.0, 0.1, 0.0, black),
                    box_shadow(54.0, 84.0, 0.1, 0.0, black),
                ]),
        )
    }
}
