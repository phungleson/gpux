use gpui::{
    div, px, rgb, size, App, Bounds, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};
use helpers::{interactivity::Disableable, interactivity::Selection};
use radix_themes::{assets::Assets, checkbox::Checkbox, theme::Theme};

struct Main {
    checked: Selection,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let checkbox = Checkbox::new("test")
            .label("Checkbox")
            .checked(self.checked)
            .on_click(cx.listener(|view, _, _| {
                view.checked = view.checked.inverse();
            }));

        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .border_1()
            .text_color(rgb(0xffffff))
            .child(checkbox)
            .child(
                div()
                    .size_16()
                    .shadow_xl()
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(0xffffff)),
            )
            .child(Checkbox::new("test1").disabled(true).label("Disabled"))
    }
}

fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        cx.set_global(Theme::indigo());
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Main {
                    checked: Selection::Selected,
                })
            },
        )
        .unwrap();
    });
}
