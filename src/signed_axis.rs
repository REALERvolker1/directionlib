use {
    crate::{
        direction::Direction,
        macros::{enum_matcher_array, enum_ordered_array},
    },
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
    pub enum SignedAxis {
        XPos = 0,
        YPos = 1,
        ZPos = 2,

        XNeg = 3,
        YNeg = 4,
        ZNeg = 5,
    }
}
impl SignedAxis {
    enum_matcher_array! {
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
        pub const VARIANTS_OPPOSITE: Self = {
            XPos => Self::XNeg,
            YPos => Self::YNeg,
            ZPos => Self::ZNeg,
            XNeg => Self::XPos,
            YNeg => Self::YPos,
            ZNeg => Self::ZPos,
        }
    }
    /// Get the opposite of the provided `Direction`
    /// ```
    /// use directionlib::SignedAxis;
    ///
    /// let current = SignedAxis::XPos;
    /// assert_eq!(current.reverse(), SignedAxis::XNeg);
    /// assert_eq!(current.reverse(), -current);
    ///
    /// let vertical = SignedAxis::ZNeg;
    /// assert_eq!(vertical.reverse(), SignedAxis::ZPos);
    /// ```
    #[inline(always)]
    pub const fn reverse(self) -> Self {
        Self::VARIANTS_OPPOSITE[self as usize]
    }

    const fn map_from_into<const N: usize>(
        pairs: [(Self, Direction); N],
    ) -> ([Self; N], [Direction; N]) {
        let mut from_arr = [pairs[0].0; _];
        let mut into_arr = [pairs[0].1; _];
        let mut i = 0;

        while i != N {
            let (from, to) = pairs[i];

            from_arr[to as usize] = from;
            into_arr[from as usize] = to;
            i += 1;
        }

        (from_arr, into_arr)
    }

    /// An array of axes corresponding to each direction in a right-handed view
    /// facing towards -z
    const RH_VIEW_MAPS_FROM_INTO: ([Self; Self::COUNT], [Direction; Direction::COUNT]) = {
        Self::map_from_into([
            (Self::XPos, Direction::Right),
            (Self::YPos, Direction::Up),
            (Self::ZPos, Direction::Back),
            (Self::XNeg, Direction::Left),
            (Self::YNeg, Direction::Down),
            (Self::ZNeg, Direction::Front),
        ])
    };
    // const RH_VIEW_MAPS_FROMDIR

    /// An array of axes corresponding to each direction in a left-handed view (OpenGL),
    /// facing towards +z
    const LH_VIEW_MAPS_FROM_INTO: ([Self; Self::COUNT], [Direction; Direction::COUNT]) = {
        Self::map_from_into([
            (Self::XPos, Direction::Right),
            (Self::YPos, Direction::Up),
            (Self::ZPos, Direction::Front),
            (Self::XNeg, Direction::Left),
            (Self::YNeg, Direction::Down),
            (Self::ZNeg, Direction::Back),
        ])
    };

    /// Convert into a right-handed (facing -z) [`Direction`],
    /// for use with WGPU, Vulkan, DX12, or Metal
    /// ```
    /// use directionlib::{SignedAxis, Direction};
    ///
    /// let down = SignedAxis::YNeg;
    /// assert_eq!(Direction::Down, down.to_direction_rh());
    /// let right = SignedAxis::XPos;
    /// assert_eq!(Direction::Right, right.to_direction_rh());
    /// let backwards = SignedAxis::ZPos;
    /// assert_eq!(Direction::Back, backwards.to_direction_rh());
    /// ```
    #[inline(always)]
    pub const fn to_direction_rh(self) -> Direction {
        Self::RH_VIEW_MAPS_FROM_INTO.1[self as usize]
    }
    /// Convert from a [`Direction`], assuming a right-handed (facing -z) coordinate system
    /// for use with WGPU, Vulkan, DX12, or Metal
    /// ```
    /// use directionlib::{SignedAxis, Direction};
    ///
    /// let up = Direction::Up;
    /// assert_eq!(SignedAxis::YPos, SignedAxis::from_direction_rh(up));
    /// let left = Direction::Left;
    /// assert_eq!(SignedAxis::XNeg, SignedAxis::from_direction_rh(left));
    /// let forwards = Direction::Front;
    /// assert_eq!(SignedAxis::ZNeg, SignedAxis::from_direction_rh(forwards));
    /// ```
    #[inline(always)]
    pub const fn from_direction_rh(value: Direction) -> Self {
        Self::RH_VIEW_MAPS_FROM_INTO.0[value as usize]
    }
    /// Convert into a left-handed (facing +z) [`Direction`],
    /// for use with OpenGL
    /// ```
    /// use directionlib::{SignedAxis, Direction};
    ///
    /// let down = SignedAxis::YNeg;
    /// assert_eq!(Direction::Down, down.to_direction_lh());
    /// let right = SignedAxis::XPos;
    /// assert_eq!(Direction::Right, right.to_direction_lh());
    /// let backwards = SignedAxis::ZNeg;
    /// assert_eq!(Direction::Back, backwards.to_direction_lh());
    /// ```
    #[inline(always)]
    pub const fn to_direction_lh(self) -> Direction {
        Self::LH_VIEW_MAPS_FROM_INTO.1[self as usize]
    }
    /// Convert from a [`Direction`], assuming a left-handed (facing +z) coordinate system
    /// for use with OpenGL
    /// ```
    /// use directionlib::{SignedAxis, Direction};
    ///
    /// let up = Direction::Up;
    /// assert_eq!(SignedAxis::YPos, SignedAxis::from_direction_lh(up));
    /// let left = Direction::Left;
    /// assert_eq!(SignedAxis::XNeg, SignedAxis::from_direction_lh(left));
    /// let forwards = Direction::Front;
    /// assert_eq!(SignedAxis::ZPos, SignedAxis::from_direction_lh(forwards));
    /// ```
    #[inline(always)]
    pub const fn from_direction_lh(value: Direction) -> Self {
        Self::LH_VIEW_MAPS_FROM_INTO.0[value as usize]
    }
}
impl Neg for SignedAxis {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.reverse()
    }
}
