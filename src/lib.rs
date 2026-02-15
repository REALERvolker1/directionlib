//! A helper library for applications that wish to describe directional behavior

#![no_std]
#![deny(missing_docs)]

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
pub use index::{AxisIndex, SignedAxisIndex};
