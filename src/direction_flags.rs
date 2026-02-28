//! A bitflags macro with directions

// SPDX-License-Identifier: LGPL-3.0-only

use {
    crate::{SignedAxis, axis::Axis, direction::Direction},
    ::core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

#[inline]
const fn dir2flag(direction: Direction) -> u8 {
    1 << (direction as usize)
}

bitflags::bitflags! {
    /// A bitflag structure that allows multiple directions to be stored in a single integer efficiently.
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
    // #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    // #[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
    #[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
    #[cfg_attr(
        feature = "bevy-inspector-egui",
        derive(bevy_inspector_egui::InspectorOptions)
    )]
    #[cfg_attr(feature = "bytemuck", derive(bytemuck::TransparentWrapper, bytemuck::Zeroable, bytemuck::Pod))]
    #[repr(transparent)]
    pub struct DirectionFlags : u8 {
        /// Directly corresponds to [`Direction::Up`]
        const UP = dir2flag(Direction::Up);
        /// Directly corresponds to [`Direction::Right`]
        const RIGHT = dir2flag(Direction::Right);
        /// Directly corresponds to [`Direction::Front`]
        const FRONT = dir2flag(Direction::Front);
        /// Directly corresponds to [`Direction::Down`]
        const DOWN = dir2flag(Direction::Down);
        /// Directly corresponds to [`Direction::Left`]
        const LEFT = dir2flag(Direction::Left);
        /// Directly corresponds to [`Direction::Back`]
        const BACK = dir2flag(Direction::Back);

        /// Both left and right together
        const MASK_X = Self::LEFT.union(Self::RIGHT).bits();
        /// Both up and down together
        const MASK_Y = Self::UP.union(Self::DOWN).bits();
        /// Both front and back together
        const MASK_Z = Self::FRONT.union(Self::BACK).bits();

        /// All horizontal directions
        const MASK_XZ = Self::MASK_X.union(Self::MASK_Z).bits();
    }
}
impl ::core::fmt::Display for DirectionFlags {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:02X}", self))
    }
}
impl DirectionFlags {
    /// Create a new set from a direction
    #[inline(always)]
    #[must_use]
    pub const fn new(direction: Direction) -> Self {
        Self::from_bits_retain(dir2flag(direction))
    }
    /// Convert from an axis (left-handed coordinate system)
    #[inline(always)]
    #[must_use]
    pub const fn from_signed_axis_lh(axis: SignedAxis) -> Self {
        Self::new(axis.to_direction_lh())
    }
    /// Convert from an axis (right-handed coordinate system)
    #[inline(always)]
    #[must_use]
    pub const fn from_signed_axis_rh(axis: SignedAxis) -> Self {
        Self::new(axis.to_direction_rh())
    }
    /// Convert from an axis to the corresponding direction flags mask.
    #[inline(always)]
    #[must_use]
    pub const fn from_axis_mask(axis: Axis) -> Self {
        axis.to_direction_flags_mask()
    }
    /// add a direction to this set, if and only if its reverse is not in the set.
    ///
    /// If the reverse is in the set, both corresponding flags will be cleared.
    /// ```
    /// use directionlib::{DirectionFlags, Direction};
    ///
    /// let mut direction = DirectionFlags::new(Direction::Left);
    /// direction |= direction.with_exclusive_axis_direction(Direction::Right);
    /// assert_eq!(direction, Direction::Left.to_flags());
    ///
    /// direction |= direction.with_exclusive_axis_direction(Direction::Up);
    /// assert_eq!(direction, Direction::Left.to_flags() | Direction::Up);
    /// ```
    #[must_use]
    pub const fn with_exclusive_axis_direction(self, direction: Direction) -> Self {
        if self.contains_direction(direction) {
            self
        } else {
            let dir_op = direction.reverse();

            if self.contains_direction(dir_op) {
                // holding opposite movement keys, do nothing
                self.difference_direction(dir_op)
            } else {
                self.union_direction(direction)
            }
        }
    }
    /// Returns whether the set contains any bits that correspond to the provided [`Axis`]
    #[inline(always)]
    #[must_use]
    pub const fn intersects_axis(self, axis: Axis) -> bool {
        self.intersects(axis.to_direction_flags_mask())
    }
    /// Returns the bits this set contains that correspond to the provided [`Axis`]
    #[inline(always)]
    #[must_use]
    pub const fn intersection_axis(self, axis: Axis) -> Self {
        self.intersection(axis.to_direction_flags_mask())
    }
    /// Add the flag corresponding to the provided direction if the opposite direction is not already
    /// present.
    ///
    /// If the opposite direction flag is already present, this clears both of them, canceling the operation entirely.
    pub const fn push_exclusive_axis_direction(&mut self, direction: Direction) {
        *self = self.with_exclusive_axis_direction(direction)
    }
    /// Whether this flag set contains the specified direction
    pub const fn contains_direction(self, direction: Direction) -> bool {
        self.contains(direction.to_flags())
    }
    /// Whether this flag set contains the specified direction, but NOT the reverse direction
    pub const fn contains_direction_exclusive_axis(self, direction: Direction) -> bool {
        self.contains_direction(direction) && !self.contains_direction(direction.reverse())
    }
    /// bitwise `|` a direction, adding the direction to the flag set
    #[inline(always)]
    #[must_use]
    pub const fn union_direction(self, direction: Direction) -> Self {
        self.union(direction.to_flags())
    }
    /// bitwise `&` a direction, adding the direction to the flag set
    #[inline(always)]
    #[must_use]
    pub const fn intersection_direction(self, direction: Direction) -> Self {
        self.intersection(direction.to_flags())
    }
    /// bitwise `&!` with a direction, removing the direction from the flag set
    #[inline(always)]
    #[must_use]
    pub const fn difference_direction(self, direction: Direction) -> Self {
        self.difference(direction.to_flags())
    }
    /// bitwise `^` with a single direction
    #[inline(always)]
    #[must_use]
    pub const fn symmetric_difference_direction(self, direction: Direction) -> Self {
        self.symmetric_difference(direction.to_flags())
    }
    /// bitwise `&!` with a direction, removing the direction from the flag set
    pub const fn remove_direction(&mut self, direction: Direction) {
        *self = self.difference_direction(direction);
    }
    /// Returns `true` if any bits are set that do NOT pertain to a valid direction.
    pub const fn contains_unknown_bits_const(self) -> bool {
        self.bits() & !(Self::all().bits()) != 0
    }
    /// Create an iterator of all the direction flags currently set as enum values.
    #[must_use]
    pub fn into_directions_iter(self) -> DirectionFlagsIter {
        DirectionFlagsIter {
            inner: self.into_iter(),
        }
    }
    /// Tries to get the lowest flag in the set, returns `None` if this set is empty.
    /// ```
    /// use directionlib::{Direction, DirectionFlags};
    ///
    /// let my_set = DirectionFlags::empty() | Direction::Left | Direction::Front;
    ///
    /// assert_eq!(my_set.lowest_flag(), Some(Direction::Front));
    /// ```
    #[must_use]
    pub const fn lowest_flag(self) -> Option<Direction> {
        debug_assert!(!self.contains_unknown_bits_const());
        Direction::try_from_repr(self.bits().trailing_zeros() as _)
    }

    /// Get the sign of the local X axis
    pub const fn signum_x_rh(self) -> f32 {
        match self.intersection_axis(Axis::X) {
            Self::LEFT => -1.,
            Self::RIGHT => 1.,
            _ => 0.,
        }
    }
    /// Get the sign of the local X axis
    #[inline(always)]
    pub const fn signum_x_lh(self) -> f32 {
        self.signum_x_rh()
    }
    /// Get the sign of the local Y axis
    pub const fn signum_y_rh(self) -> f32 {
        match self.intersection_axis(Axis::Y) {
            Self::DOWN => -1.,
            Self::UP => 1.,
            _ => 0.,
        }
    }
    /// Get the sign of the local Y axis
    #[inline(always)]
    pub const fn signum_y_lh(self) -> f32 {
        self.signum_y_rh()
    }
    /// Get the sign of the local Z axis
    pub const fn signum_z_rh(self) -> f32 {
        match self.intersection_axis(Axis::Z) {
            Self::FRONT => -1.,
            Self::BACK => 1.,
            _ => 0.,
        }
    }
    /// Get the sign of the local Z axis
    pub const fn signum_z_lh(self) -> f32 {
        match self.intersection_axis(Axis::Z) {
            Self::BACK => -1.,
            Self::FRONT => 1.,
            _ => 0.,
        }
    }
}
impl From<Direction> for DirectionFlags {
    #[inline(always)]
    fn from(value: Direction) -> Self {
        Self::new(value)
    }
}
impl BitAnd<Direction> for DirectionFlags {
    type Output = Self;
    fn bitand(self, rhs: Direction) -> Self::Output {
        self.intersection_direction(rhs)
    }
}
impl BitOr<Direction> for DirectionFlags {
    type Output = Self;
    fn bitor(self, rhs: Direction) -> Self::Output {
        self.union_direction(rhs)
    }
}
impl BitXor<Direction> for DirectionFlags {
    type Output = Self;
    fn bitxor(self, rhs: Direction) -> Self::Output {
        self.symmetric_difference_direction(rhs)
    }
}
impl BitAndAssign<Direction> for DirectionFlags {
    fn bitand_assign(&mut self, rhs: Direction) {
        *self = self.bitand(rhs)
    }
}
impl BitOrAssign<Direction> for DirectionFlags {
    fn bitor_assign(&mut self, rhs: Direction) {
        *self = self.bitor(rhs)
    }
}
impl BitXorAssign<Direction> for DirectionFlags {
    fn bitxor_assign(&mut self, rhs: Direction) {
        *self = self.bitxor(rhs)
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for DirectionFlags {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(Self::from_bits_truncate(arbitrary::Arbitrary::arbitrary(
            u,
        )?))
    }
}

/// An iterator that enumerates over all direction flags
#[cfg_attr(feature = "bytemuck", derive(bytemuck_derive::TransparentWrapper))]
#[repr(transparent)]
pub struct DirectionFlagsIter {
    inner: <DirectionFlags as IntoIterator>::IntoIter,
}
impl Iterator for DirectionFlagsIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(DirectionFlags::lowest_flag)
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    //! Internal utils for glam

    use {
        super::*,
        ::glam::{Vec3, Vec3A},
    };

    impl DirectionFlags {
        /// Returns a vector that contains the signs of each axis in the flag set.
        ///
        /// The vector returned is NOT normalized
        #[must_use]
        pub const fn signum_vec3_lh(self) -> Vec3 {
            Vec3 {
                x: self.signum_x_lh(),
                y: self.signum_y_lh(),
                z: self.signum_z_lh(),
            }
        }
        /// Returns a vector that contains the signs of each axis in the flag set.
        ///
        /// The vector returned is NOT normalized
        #[must_use]
        pub const fn signum_vec3_rh(self) -> Vec3 {
            Vec3 {
                x: self.signum_x_rh(),
                y: self.signum_y_rh(),
                z: self.signum_z_rh(),
            }
        }
        /// Returns a vector that contains the signs of each axis in the flag set.
        ///
        /// The vector returned is NOT normalized
        #[must_use]
        pub const fn signum_vec3a_lh(self) -> Vec3A {
            Vec3A::new(self.signum_x_lh(), self.signum_y_lh(), self.signum_z_lh())
        }
        /// Returns a vector that contains the signs of each axis in the flag set.
        ///
        /// The vector returned is NOT normalized
        #[must_use]
        pub const fn signum_vec3a_rh(self) -> Vec3A {
            Vec3A::new(self.signum_x_rh(), self.signum_y_rh(), self.signum_z_rh())
        }
    }
}
#[cfg(feature = "bevy_reflect")]
impl bevy_reflect::TupleStruct for DirectionFlags {
    fn field(&self, index: usize) -> Option<&dyn bevy_reflect::PartialReflect> {
        if index == 0 { Some(&self.0.0) } else { None }
    }
    fn field_len(&self) -> usize {
        1
    }
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn bevy_reflect::PartialReflect> {
        if index == 0 {
            Some(&mut self.0.0)
        } else {
            None
        }
    }
    fn iter_fields(&self) -> bevy_reflect::TupleStructFieldIter<'_> {
        bevy_reflect::TupleStructFieldIter::new(self)
    }
}
#[cfg(feature = "bevy_reflect")]
impl bevy_reflect::PartialReflect for DirectionFlags {
    fn get_represented_type_info(&self) -> Option<&'static bevy_reflect::TypeInfo> {
        Some(<Self as bevy_reflect::Typed>::type_info())
    }
    fn as_partial_reflect(&self) -> &dyn bevy_reflect::PartialReflect {
        self
    }
    fn as_partial_reflect_mut(&mut self) -> &mut dyn bevy_reflect::PartialReflect {
        self
    }
    fn try_as_reflect(&self) -> Option<&dyn bevy_reflect::Reflect> {
        Some(self)
    }
    fn try_as_reflect_mut(&mut self) -> Option<&mut dyn bevy_reflect::Reflect> {
        Some(self)
    }
    fn try_into_reflect(
        self: std::prelude::v1::Box<Self>,
    ) -> Result<
        std::prelude::v1::Box<dyn bevy_reflect::Reflect>,
        std::prelude::v1::Box<dyn bevy_reflect::PartialReflect>,
    > {
        Ok(self)
    }
    fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut<'_> {
        bevy_reflect::ReflectMut::TupleStruct(self)
    }
    fn into_partial_reflect(
        self: std::prelude::v1::Box<Self>,
    ) -> std::prelude::v1::Box<dyn bevy_reflect::PartialReflect> {
        self
    }
    fn try_apply(
        &mut self,
        value: &dyn bevy_reflect::PartialReflect,
    ) -> Result<(), bevy_reflect::ApplyError> {
        *self =
            *value
                .try_downcast_ref()
                .ok_or_else(|| bevy_reflect::ApplyError::MismatchedTypes {
                    from_type: value.reflect_type_path().into(),
                    to_type: <Self as bevy_reflect::TypePath>::type_path().into(),
                })?;
        Ok(())
    }
    fn reflect_ref(&self) -> bevy_reflect::ReflectRef<'_> {
        bevy_reflect::ReflectRef::TupleStruct(self)
    }
    fn reflect_owned(self: std::prelude::v1::Box<Self>) -> bevy_reflect::ReflectOwned {
        bevy_reflect::ReflectOwned::TupleStruct(self)
    }
}

#[cfg(feature = "bevy_reflect")]
impl bevy_reflect::Typed for DirectionFlags {
    fn type_info() -> &'static bevy_reflect::TypeInfo {
        use {::bevy_reflect::TypeInfo, ::std::sync::LazyLock};

        static CELL: LazyLock<TypeInfo> = LazyLock::new(|| {
            TypeInfo::TupleStruct(bevy_reflect::TupleStructInfo::new::<DirectionFlags>(&[
                bevy_reflect::UnnamedField::new::<DirectionFlags>(0),
            ]))
        });

        &CELL
    }
}
#[cfg(feature = "bevy_reflect")]
impl bevy_reflect::TypePath for DirectionFlags {
    fn crate_name() -> Option<&'static str> {
        Some(env!("CARGO_PKG_NAME"))
    }
    fn module_path() -> Option<&'static str> {
        Some(module_path!())
    }
    fn short_type_path() -> &'static str {
        "DirectionFlags"
    }
    fn type_path() -> &'static str {
        concat!(module_path!(), "::", "DirectionFlags")
    }
}

#[cfg(feature = "bevy_reflect")]
impl bevy_reflect::Reflect for DirectionFlags {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
    fn as_reflect(&self) -> &dyn bevy_reflect::Reflect {
        self
    }
    fn as_reflect_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
        self
    }
    fn into_any(self: std::boxed::Box<Self>) -> std::boxed::Box<dyn core::any::Any> {
        self
    }
    fn into_reflect(self: std::boxed::Box<Self>) -> std::boxed::Box<dyn bevy_reflect::Reflect> {
        self
    }
    fn set(
        &mut self,
        value: std::boxed::Box<dyn bevy_reflect::Reflect>,
    ) -> Result<(), std::boxed::Box<dyn bevy_reflect::Reflect>> {
        *self = *value.downcast::<Self>()?;
        Ok(())
    }
}
