use {
    crate::{
        direction_flags::DirectionFlags,
        macros::{enum_matcher_array, enum_ordered_array},
    },
    ::core::ops::Neg,
};

enum_ordered_array! {
    /// A simple direction selection enum, designed for matching, selecting, indexing, etc.
    /// For more complex 2D or 3D scenes, consider using a vector instead.
    pub enum Direction {
        /// The direction usually corresponding to +Y
        Up = 0,
        /// The direction usually corresponding to +X
        Right = 1,
        /// The direction usually corresponding to -Z (Right-handed), or +Z (Left-handed)
        #[doc(alias = "Forward")]
        Front = 2,

        /// The direction usually corresponding to -Y
        Down = 3,
        /// The direction usually corresponding to -X
        Left = 4,
        /// The direction usually corresponding to +Z (Right-handed), or -Z (Left-handed)
        #[doc(alias = "Backward")]
        Back = 5,
    }
}

impl Direction {
    enum_matcher_array! {
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
        pub const VARIANTS_OPPOSITE = {
            Back => Self::Front,
            Front => Self::Back,
            Left => Self::Right,
            Right => Self::Left,
            Up => Self::Down,
            Down => Self::Up,
        }
    }

    /// Get the opposite of the provided `Direction`
    /// ```
    /// use directionlib::Direction;
    ///
    /// let current = Direction::Front;
    /// assert_eq!(current.reverse(), Direction::Back);
    ///
    /// let vertical = Direction::Down;
    /// assert_eq!(vertical.reverse(), Direction::Up);
    /// ```
    #[inline(always)]
    pub const fn reverse(self) -> Self {
        Self::VARIANTS_OPPOSITE[self as usize]
    }
    /// Convert the provided direction into a flagset.
    /// ```
    /// use directionlib::{Direction, DirectionFlags};
    ///
    /// let current = Direction::Left;
    /// assert_eq!(current.to_flags(), DirectionFlags::LEFT);
    /// let up = Direction::Up;
    /// let down = Direction::Down;
    /// // we also overload the operator
    /// let all_y = up.to_flags() | down;
    /// assert_eq!(all_y, DirectionFlags::MASK_Y);
    /// ```
    #[inline(always)]
    pub const fn to_flags(self) -> DirectionFlags {
        DirectionFlags::new(self)
    }
}
impl Neg for Direction {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.reverse()
    }
}
