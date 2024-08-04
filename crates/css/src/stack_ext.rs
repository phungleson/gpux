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
