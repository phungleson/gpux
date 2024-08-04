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

/// A collection of colors that are used to style the UI.
///
/// Each step has a semantic meaning, and is used to style different parts of the UI.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ColorScaleStep(usize);

impl ColorScaleStep {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
    pub const SIX: Self = Self(6);
    pub const SEVEN: Self = Self(7);
    pub const EIGHT: Self = Self(8);
    pub const NINE: Self = Self(9);
    pub const TEN: Self = Self(10);
    pub const ELEVEN: Self = Self(11);
    pub const TWELVE: Self = Self(12);

    /// All the steps in a [`ColorScale`].
    pub const ALL: [ColorScaleStep; 12] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
        Self::SEVEN,
        Self::EIGHT,
        Self::NINE,
        Self::TEN,
        Self::ELEVEN,
        Self::TWELVE,
    ];
}

/// A scale of colors for a given [`ColorScaleSet`].
///
/// Each [`ColorScale`] contains exactly 12 colors. Refer to [`ColorScaleStep`] for a reference of what each step is used for.
#[derive(Default, Clone)]
pub struct ColorScale(Vec<Hsla>);

impl FromIterator<Hsla> for ColorScale {
    fn from_iter<T: IntoIterator<Item = Hsla>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl ColorScale {
    /// Returns the specified step in the [`ColorScale`].
    #[inline]
    pub fn step(&self, step: ColorScaleStep) -> Hsla {
        // Steps are one-based, so we need convert to the zero-based vec index.
        self.0[step.0 - 1]
    }

    /// `Step 1` - Used for main application backgrounds.
    ///
    /// This step provides a neutral base for any overlaying components, ideal for applications' main backdrop or empty spaces such as canvas areas.
    #[inline]
    pub fn step_1(&self) -> Hsla {
        self.step(ColorScaleStep::ONE)
    }

    /// `Step 2` - Used for both main application backgrounds and subtle component backgrounds.
    ///
    /// Like `Step 1`, this step allows variations in background styles, from striped tables, sidebar backgrounds, to card backgrounds.
    #[inline]
    pub fn step_2(&self) -> Hsla {
        self.step(ColorScaleStep::TWO)
    }

    /// `Step 3` - Used for UI component backgrounds in their normal states.
    ///
    /// This step maintains accessibility by guaranteeing a contrast ratio of 4.5:1 with steps 11 and 12 for text. It could also suit hover states for transparent components.
    #[inline]
    pub fn step_3(&self) -> Hsla {
        self.step(ColorScaleStep::THREE)
    }

    /// `Step 4` - Used for UI component backgrounds in their hover states.
    ///
    /// Also suited for pressed or selected states of components with a transparent background.
    #[inline]
    pub fn step_4(&self) -> Hsla {
        self.step(ColorScaleStep::FOUR)
    }

    /// `Step 5` - Used for UI component backgrounds in their pressed or selected states.
    #[inline]
    pub fn step_5(&self) -> Hsla {
        self.step(ColorScaleStep::FIVE)
    }

    /// `Step 6` - Used for subtle borders on non-interactive components.
    ///
    /// Its usage spans from sidebars' borders, headers' dividers, cards' outlines, to alerts' edges and separators.
    #[inline]
    pub fn step_6(&self) -> Hsla {
        self.step(ColorScaleStep::SIX)
    }

    /// `Step 7` - Used for subtle borders on interactive components.
    ///
    /// This step subtly delineates the boundary of elements users interact with.
    #[inline]
    pub fn step_7(&self) -> Hsla {
        self.step(ColorScaleStep::SEVEN)
    }

    /// `Step 8` - Used for stronger borders on interactive components and focus rings.
    ///
    /// It strengthens the visibility and accessibility of active elements and their focus states.
    #[inline]
    pub fn step_8(&self) -> Hsla {
        self.step(ColorScaleStep::EIGHT)
    }

    /// `Step 9` - Used for solid backgrounds.
    ///
    /// `Step 9` is the most saturated step, having the least mix of white or black.
    ///
    /// Due to its high chroma, `Step 9` is versatile and particularly useful for semantic colors such as error, warning, and success indicators.
    #[inline]
    pub fn step_9(&self) -> Hsla {
        self.step(ColorScaleStep::NINE)
    }

    /// `Step 10` - Used for hovered or active solid backgrounds, particularly when `Step 9` is their normal state.
    ///
    /// May also be used for extremely low contrast text. This should be used sparingly, as it may be difficult to read.
    #[inline]
    pub fn step_10(&self) -> Hsla {
        self.step(ColorScaleStep::TEN)
    }

    /// `Step 11` - Used for text and icons requiring low contrast or less emphasis.
    #[inline]
    pub fn step_11(&self) -> Hsla {
        self.step(ColorScaleStep::ELEVEN)
    }

    /// `Step 12` - Used for text and icons requiring high contrast or prominence.
    #[inline]
    pub fn step_12(&self) -> Hsla {
        self.step(ColorScaleStep::TWELVE)
    }
}
