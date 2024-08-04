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

/// A collection of colors that are used to style the UI.
///
/// Each step has a semantic meaning, and is used to style different parts of the UI.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct NineScaleStep(usize);

impl NineScaleStep {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
    pub const SIX: Self = Self(6);
    pub const SEVEN: Self = Self(7);
    pub const EIGHT: Self = Self(8);
    pub const NINE: Self = Self(9);

    /// All the steps in a [`create::theme::Scale`].
    pub const ALL: [NineScaleStep; 9] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
        Self::SEVEN,
        Self::EIGHT,
        Self::NINE,
    ];
}

/// A scale of variables.
#[derive(Default, Clone)]
pub(crate) struct NineScale<T>(pub [T; 9]);

impl<T: Copy + Clone> NineScale<T> {
    /// Returns the specified step in the [`crate::theme::NineScale`].
    #[inline]
    pub fn step(&self, step: NineScaleStep) -> T {
        // Steps are one-based, so we need convert to the zero-based vec index.
        self.0[step.0 - 1]
    }

    #[inline]
    pub fn step_1(&self) -> T {
        self.step(NineScaleStep::ONE)
    }

    #[inline]
    pub fn step_2(&self) -> T {
        self.step(NineScaleStep::TWO)
    }

    #[inline]
    pub fn step_3(&self) -> T {
        self.step(NineScaleStep::THREE)
    }

    #[inline]
    pub fn step_4(&self) -> T {
        self.step(NineScaleStep::FOUR)
    }

    #[inline]
    pub fn step_5(&self) -> T {
        self.step(NineScaleStep::FIVE)
    }

    #[inline]
    pub fn step_6(&self) -> T {
        self.step(NineScaleStep::SIX)
    }

    #[inline]
    pub fn step_7(&self) -> T {
        self.step(NineScaleStep::SEVEN)
    }

    #[inline]
    pub fn step_8(&self) -> T {
        self.step(NineScaleStep::EIGHT)
    }

    #[inline]
    pub fn step_9(&self) -> T {
        self.step(NineScaleStep::NINE)
    }
}
