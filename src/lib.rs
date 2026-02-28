//! A helper library for applications that wish to describe directional behavior

// SPDX-License-Identifier: LGPL-3.0-only

#![no_std]
#![deny(missing_docs)]

// might as well at that point
#[cfg(feature = "bevy_reflect")]
extern crate std;

mod axis;
mod axis_set;
mod direction;
mod direction_flags;
mod index;
mod index_impls;
mod macros;

pub use axis::{Axis, SignedAxis};
pub use axis_set::{SignedAxisSet, SignedAxisSetIter};
pub use direction::Direction;
pub use direction_flags::{DirectionFlags, DirectionFlagsIter};
pub use index::{AxisIndex, CombinedDirectionIndex, DirectionIndex, SignedAxisIndex};
