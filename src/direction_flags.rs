use {
    crate::direction::Direction,
    ::core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

#[inline]
const fn dir2flag(direction: Direction) -> u8 {
    1 << (direction as usize)
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct DirectionFlags : u8 {
        const UP = dir2flag(Direction::Up);
        const RIGHT = dir2flag(Direction::Right);
        const FRONT = dir2flag(Direction::Front);
        const DOWN = dir2flag(Direction::Down);
        const LEFT = dir2flag(Direction::Left);
        const BACK = dir2flag(Direction::Back);

        /// Both left and right together
        const MASK_X = Self::LEFT.union(Self::RIGHT).bits();
        /// Both up and down together
        const MASK_Y = Self::UP.union(Self::DOWN).bits();
        /// Both front and back together
        const MASK_Z = Self::FRONT.union(Self::BACK).bits();
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
    #[inline]
    pub const fn new(direction: Direction) -> Self {
        Self::from_bits_retain(dir2flag(direction))
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
    #[must_use]
    pub const fn union_direction(self, direction: Direction) -> Self {
        self.union(direction.to_flags())
    }
    /// bitwise `&` a direction, adding the direction to the flag set
    #[must_use]
    pub const fn intersection_direction(self, direction: Direction) -> Self {
        self.intersection(direction.to_flags())
    }
    /// bitwise `&!` with a direction, removing the direction from the flag set
    #[must_use]
    pub const fn difference_direction(self, direction: Direction) -> Self {
        self.difference(direction.to_flags())
    }
    /// bitwise `^` with a single direction
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

#[repr(transparent)]
pub struct DirectionFlagsIter {
    inner: <DirectionFlags as IntoIterator>::IntoIter,
}
impl Iterator for DirectionFlagsIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(DirectionFlags::lowest_flag).flatten()
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }
}
