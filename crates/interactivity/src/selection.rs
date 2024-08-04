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

use std::fmt::Display;

/// Represents the selection status of an element.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Selection {
    /// The element is not selected.
    #[default]
    Unselected,
    /// The selection state of the element is indeterminate.
    Indeterminate,
    /// The element is selected.
    Selected,
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unselected => write!(f, "Unselected"),
            Self::Indeterminate => write!(f, "Indeterminate"),
            Self::Selected => write!(f, "Selected"),
        }
    }
}

impl Selection {
    /// Returns the inverse of the current selection status.
    ///
    /// Indeterminate states become selected if inverted.
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }
}
