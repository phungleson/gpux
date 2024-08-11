use gpui::{
    App, Bounds, px, Render, size, ViewContext, VisualContext, WindowBounds, WindowOptions,
};

use gpux_radix_themes::assets::Assets;
use gpux_radix_themes::theme::{AccentColor, GrayColor, Theme};
use gpux_theme::theme_mode::ThemeMode;

pub fn run_app<V: Render>(new_view: impl FnOnce(&mut ViewContext<'_, V>) -> V + 'static) {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        cx.set_global(Theme::new(
            ThemeMode::Light,
            AccentColor::Indigo,
            GrayColor::Slate,
        ));
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(new_view),
        )
        .unwrap();
    });
}

fn main() {}