// Copyright 2024 Phung Le Son.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/phungleson/gpux/blob/main/LICENSE-APACHE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::Hsla;

/// Makes a [gpui::Hsla] color.
///
/// h - 0 - 360.0
/// s - 0.0 - 100.0
/// l - 0.0 - 100.0
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    gpui::hsla(h / 360., s / 100.0, l / 100.0, 1.0)
}

/// Makes a [gpui::Hsla] color.
///
/// h - 0 - 360.0
/// s - 0.0 - 100.0
/// l - 0.0 - 100.0
/// a - 0.0 - 100.0
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    gpui::hsla(h / 360., s / 100.0, l / 100.0, a)
}

/// Makes a white color.
pub fn white() -> Hsla {
    hsl(0., 100., 100.)
}

/// Makes a transparent white color.
///
/// Returns an [gpui::Hsla] color with fully opaque white (h=0, s=0, l=100) and fully transparent alpha (a=0).
pub fn transparent_white() -> Hsla {
    hsla(0., 0., 100., 0.)
}
