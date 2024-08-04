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

/// Represents the different modes that a theme can have.
#[derive(Debug, PartialEq, PartialOrd, Eq, Default, Copy, Clone)]
pub enum ThemeMode {
    /// Light theme mode.
    #[default]
    Light,
    /// Dark theme mode.
    Dark,
}

impl ThemeMode {
    /// Checks if the theme mode is dark.
    ///
    /// Returns true if the theme mode is dark, false otherwise.
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}
