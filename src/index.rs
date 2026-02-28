//! Useful traits for structures that can provide different data or behavior depending on axes

// SPDX-License-Identifier: LGPL-3.0-only

use crate::{Axis, Direction, DirectionFlags, SignedAxis};

/// A trait for any object that has inner components corresponding to euclidean 3D axes
pub trait AxisIndex {
    /// The item this object can provide
    type Item;
    /// Get the item corresponding to the X axis
    fn axis_x(&self) -> Self::Item;
    /// Get the item corresponding to the Y axis
    fn axis_y(&self) -> Self::Item;
    /// Get the item corresponding to the Z axis
    fn axis_z(&self) -> Self::Item;
    /// Get an item corresponding to the provided axis.
    #[inline]
    fn axis_index(&self, axis: Axis) -> Self::Item {
        match axis {
            Axis::X => self.axis_x(),
            Axis::Y => self.axis_y(),
            Axis::Z => self.axis_z(),
        }
    }
}
/// A trait for any object that has inner components corresponding to euclidean 3D axes,
/// but whose outputs differ if a negative axis is requested.
///
/// # Todo
/// This seriously needs a refactor when `#![feature(specialization)]` comes to stable,
/// I hate this API.
pub trait SignedAxisIndex: AxisIndex {
    /// Get the item corresponding to the +X axis
    #[inline]
    fn axis_pos_x(&self) -> Self::Item {
        self.axis_x()
    }
    /// Get the item corresponding to the +Y axis
    #[inline]
    fn axis_pos_y(&self) -> Self::Item {
        self.axis_y()
    }
    /// Get the item corresponding to the +Z axis
    #[inline]
    fn axis_pos_z(&self) -> Self::Item {
        self.axis_z()
    }
    /// Get the item corresponding to the -X axis
    fn axis_neg_x(&self) -> Self::Item;
    /// Get the item corresponding to the -Y axis
    fn axis_neg_y(&self) -> Self::Item;
    /// Get the item corresponding to the -Z axis
    fn axis_neg_z(&self) -> Self::Item;
    /// Get an item corresponding to the provided axis.
    fn signed_axis_index(&self, axis: SignedAxis) -> Self::Item {
        match axis {
            SignedAxis::XPos => self.axis_pos_x(),
            SignedAxis::YPos => self.axis_pos_y(),
            SignedAxis::ZPos => self.axis_pos_z(),
            SignedAxis::XNeg => self.axis_neg_x(),
            SignedAxis::YNeg => self.axis_neg_y(),
            SignedAxis::ZNeg => self.axis_neg_z(),
        }
    }
}

/// Objects that can be indexed with directions
pub trait DirectionIndex {
    /// The item to get
    type Item;
    /// Get the item corresponding to [`Direction::Left`]
    fn direction_left(&self) -> Self::Item;
    /// Get the item corresponding to [`Direction::Right`]
    fn direction_right(&self) -> Self::Item;
    /// Get the item corresponding to [`Direction::Down`]
    fn direction_down(&self) -> Self::Item;
    /// Get the item corresponding to [`Direction::Up`]
    fn direction_up(&self) -> Self::Item;
    /// Get the item corresponding to [`Direction::Front`]
    fn direction_front(&self) -> Self::Item;
    /// Get the item corresponding to [`Direction::Back`]
    fn direction_back(&self) -> Self::Item;

    /// Get the item corresponding to an arbitrary [`Direction`]
    fn direction(&self, direction: Direction) -> Self::Item {
        match direction {
            Direction::Back => self.direction_back(),
            Direction::Down => self.direction_down(),
            Direction::Front => self.direction_front(),
            Direction::Left => self.direction_left(),
            Direction::Right => self.direction_right(),
            Direction::Up => self.direction_up(),
        }
    }
}

/// Objects that can be indexed by multiple directions at a time!
pub trait CombinedDirectionIndex: DirectionIndex {
    /// Get the combined output of a flag set
    fn direction_flags(&self, flags: DirectionFlags) -> Self::Item;
}
