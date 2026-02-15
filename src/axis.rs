use {
    crate::{
        DirectionFlags,
        direction::Direction,
        macros::{enum_matcher_array, enum_ordered_array},
    },
    ::core::ops::Neg,
};

#[must_use]
const fn map_from_signedaxis_into_direction<const N: usize>(
    pairs: [(SignedAxis, Direction); N],
) -> ([SignedAxis; N], [Direction; N]) {
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

enum_ordered_array! {
    /// A simple direction selection enum, designed for matching, selecting, indexing, etc.
    /// For more complex 2D or 3D scenes, consider using a vector instead.
    pub enum SignedAxis {
        /// +X
        XPos = 0,
        /// +Y
        YPos = 1,
        /// +Z
        ZPos = 2,

        /// -X
        XNeg = 3,
        /// -Y
        YNeg = 4,
        /// -Z
        ZNeg = 5,
    }
}
impl SignedAxis {
    enum_matcher_array! {
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
        pub const VARIANTS_OPPOSITE = {
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
    #[must_use]
    pub const fn reverse(self) -> Self {
        Self::VARIANTS_OPPOSITE[self as usize]
    }

    /// Convert the axis into its unsigned counterpart.
    /// ```
    /// use directionlib::{SignedAxis, Axis};
    ///
    /// assert_eq!(SignedAxis::XPos.to_unsigned(), Axis::X);
    /// assert_eq!(SignedAxis::YNeg.to_unsigned(), Axis::Y);
    /// assert_eq!(SignedAxis::ZNeg.to_unsigned(), Axis::Z);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn to_unsigned(self) -> Axis {
        Axis::from_repr((self as u8) % 3)
    }
    /// Get the absolute value of this axis
    /// ```
    /// use directionlib::SignedAxis;
    ///
    /// assert_eq!(SignedAxis::XNeg.abs(), SignedAxis::XPos);
    /// assert_eq!(SignedAxis::YPos.abs(), SignedAxis::YPos);
    /// assert_eq!(SignedAxis::ZNeg.abs(), SignedAxis::ZPos);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn abs(self) -> Self {
        self.to_unsigned().cast_signed()
    }

    /// Returns whether this axis is positive or not
    /// ```
    /// use directionlib::SignedAxis;
    ///
    /// let z = SignedAxis::ZPos;
    ///
    /// assert!(z.is_positive());
    /// assert!(!z.is_negative());
    /// assert!((z as u8) < (SignedAxis::YNeg as u8));
    /// assert!((z as u8) < (SignedAxis::ZNeg as u8));
    /// ```
    #[inline]
    pub const fn is_positive(self) -> bool {
        (self as u8) < (Self::XNeg as u8)
    }
    /// The inverse of [`is_positive`](Self::is_positive)
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        !self.is_positive()
    }

    const RH_VIEW_MAPS_FROM_INTO: ([Self; Self::COUNT], [Direction; Direction::COUNT]) = {
        map_from_signedaxis_into_direction([
            (Self::XPos, Direction::Right),
            (Self::YPos, Direction::Up),
            (Self::ZPos, Direction::Back),
            (Self::XNeg, Direction::Left),
            (Self::YNeg, Direction::Down),
            (Self::ZNeg, Direction::Front),
        ])
    };
    const LH_VIEW_MAPS_FROM_INTO: ([Self; Self::COUNT], [Direction; Direction::COUNT]) = {
        map_from_signedaxis_into_direction([
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

enum_ordered_array! {
    /// A simple direction selection enum, designed for matching, selecting, indexing, etc.
    /// For more complex 2D or 3D scenes, consider using a vector instead.
    pub enum Axis {
        /// ±X
        X = SignedAxis::XPos as _,
        /// ±Y
        Y = SignedAxis::YPos as _,
        /// ±Z
        Z = SignedAxis::ZPos as _,
    }
}
impl Axis {
    /// Cast this axis into a [`SignedAxis`]
    #[inline(always)]
    #[must_use]
    pub const fn cast_signed(self) -> SignedAxis {
        SignedAxis::VARIANT_ARRAY[self as usize]
    }

    enum_matcher_array! {
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
        const DIRECTION_FLAG_MASKS: DirectionFlags = {
            X => DirectionFlags::MASK_X,
            Y => DirectionFlags::MASK_Y,
            Z => DirectionFlags::MASK_Z,
        }
    }
    /// Get a [`DirectionFlags`] mask pertaining to this axis
    #[inline(always)]
    #[must_use]
    pub const fn to_direction_flags_mask(self) -> DirectionFlags {
        Self::DIRECTION_FLAG_MASKS[self as usize]
    }
}
impl Neg for Axis {
    type Output = SignedAxis;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.cast_signed().reverse()
    }
}
