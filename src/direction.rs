use {
    crate::macros::{enum_matcher_array, enum_ordered_array},
    ::core::ops::Neg,
};

enum_ordered_array! {
    /// A simple direction selection enum, designed for matching, selecting, indexing, etc.
    /// For more complex 2D or 3D scenes, consider using a vector instead.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
    #[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
    pub enum Direction {
        Up = 0,
        Right = 1,
        Front = 2,

        Down = 3,
        Left = 4,
        Back = 5,
    }
}

impl Direction {
    enum_matcher_array! {
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
        pub const VARIANTS_OPPOSITE: Self = {
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
}
impl Neg for Direction {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.reverse()
    }
}
