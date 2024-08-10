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

/// Step of a six-scaled collection.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SixScaleStep(usize);

impl SixScaleStep {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
    pub const SIX: Self = Self(6);

    /// All the steps in a [`create::theme::Scale`].
    pub const ALL: [SixScaleStep; 6] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
    ];
}

/// A scale of variables.
#[derive(Default, Clone)]
pub(crate) struct SixScale<T>(pub [T; 6]);

impl<T: Copy + Clone> SixScale<T> {
    /// Returns the specified step in the [`crate::theme::NineScale`].
    #[inline]
    pub fn step(&self, step: SixScaleStep) -> T {
        // Steps are one-based, so we need convert to the zero-based vec index.
        self.0[step.0 - 1]
    }

    #[inline]
    pub fn step_1(&self) -> T {
        self.step(SixScaleStep::ONE)
    }

    #[inline]
    pub fn step_2(&self) -> T {
        self.step(SixScaleStep::TWO)
    }

    #[inline]
    pub fn step_3(&self) -> T {
        self.step(SixScaleStep::THREE)
    }

    #[inline]
    pub fn step_4(&self) -> T {
        self.step(SixScaleStep::FOUR)
    }

    #[inline]
    pub fn step_5(&self) -> T {
        self.step(SixScaleStep::FIVE)
    }

    #[inline]
    pub fn step_6(&self) -> T {
        self.step(SixScaleStep::SIX)
    }
}
